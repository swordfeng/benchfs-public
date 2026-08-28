//! In-memory snapshot mount-reference registry.
//!
//! Implements the [`crate::CowFilesystem`] cross-session coordination
//! contract: one reference per live read-only snapshot mount, checked by
//! `snapshot_delete` to return `EBUSY`, released by `snapshot_unmount`. The
//! registry is memory-only; daemon crashes drop it, matching the SPEC rule
//! that mount references are not persistent GC roots.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use crate::{Errno, FsError, FsResult};

/// Tracks live read-only snapshot mounts by snapshot ID.
#[derive(Debug, Default)]
pub struct SnapshotMountRegistry {
    references: Mutex<HashMap<u64, u64>>,
}

impl SnapshotMountRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Locks the registry, recovering from poisoning by discarding a panicking
    /// writer's partial state (each entry is a single counter update).
    fn lock(&self) -> FsResult<MutexGuard<'_, HashMap<u64, u64>>> {
        self.references.lock().map_err(|_| FsError::from(Errno::Io))
    }

    /// Registers one live mount reference for `id`.
    ///
    /// `delete` and `mount` serialize on this lock, so a concurrent
    /// delete/mount pair cannot both observe an unregistered snapshot.
    pub fn acquire(&self, id: SnapshotIdKey) -> FsResult<()> {
        let mut references = self.lock()?;
        let count = references.entry(id.raw()).or_insert(0);
        *count = count
            .checked_add(1)
            .ok_or_else(|| FsError::from(Errno::Overflow))?;
        Ok(())
    }

    /// Releases one mount reference for `id`.
    ///
    /// Returns `EBADF` when the ID has no live reference (unmount twice).
    pub fn release(&self, id: SnapshotIdKey) -> FsResult<()> {
        let mut references = self.lock()?;
        match references.get_mut(&id.raw()) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    references.remove(&id.raw());
                }
                Ok(())
            }
            None => Err(Errno::BadFileDescriptor.into()),
        }
    }

    /// Returns `EBUSY` when `id` has at least one live mount reference.
    pub fn ensure_unmounted(&self, id: SnapshotIdKey) -> FsResult<()> {
        let references = self.lock()?;
        if references.contains_key(&id.raw()) {
            return Err(Errno::Busy.into());
        }
        Ok(())
    }

    /// Returns the live reference count for `id` (zero when unmounted).
    #[must_use]
    pub fn references(&self, id: SnapshotIdKey) -> u64 {
        self.lock()
            .map(|references| references.get(&id.raw()).copied().unwrap_or(0))
            .unwrap_or(0)
    }

    /// Returns whether any snapshot currently has live mounts.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lock().is_ok_and(|references| references.is_empty())
    }
}

/// Raw snapshot identity accepted by the registry.
///
/// Wraps the persistent numeric snapshot ID so callers cannot confuse it with
/// other numeric identities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SnapshotIdKey(u64);

impl SnapshotIdKey {
    /// Wraps a persistent snapshot ID.
    #[must_use]
    pub const fn new(id: crate::SnapshotId) -> Self {
        Self(id.get())
    }

    /// Returns the raw persistent ID.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(raw: u64) -> SnapshotIdKey {
        SnapshotIdKey::new(crate::SnapshotId::new(raw).unwrap())
    }

    #[test]
    fn acquire_delete_block_and_release_cycle() {
        let registry = SnapshotMountRegistry::new();
        let id = key(7);

        registry.ensure_unmounted(id).expect("initially unmounted");
        registry.acquire(id).expect("acquire");
        assert_eq!(registry.references(id), 1);
        assert!(registry.ensure_unmounted(id).is_err());

        registry.acquire(id).expect("second mount");
        assert_eq!(registry.references(id), 2);

        registry.release(id).expect("first release");
        registry.release(id).expect("second release");
        assert!(registry.ensure_unmounted(id).is_ok());
        assert!(registry.is_empty());
    }

    #[test]
    fn release_without_mount_is_ebadf() {
        let registry = SnapshotMountRegistry::new();
        let error = registry.release(key(9)).unwrap_err();
        assert_eq!(error.errno(), Errno::BadFileDescriptor);
    }

    #[test]
    fn concurrent_acquires_serialize_on_registry_lock() {
        let registry = std::sync::Arc::new(SnapshotMountRegistry::new());
        let id = key(42);
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let registry = std::sync::Arc::clone(&registry);
                std::thread::spawn(move || registry.acquire(id))
            })
            .collect();
        for handle in handles {
            handle.join().expect("acquire thread").expect("acquire");
        }
        assert_eq!(registry.references(id), 8);

        for _ in 0..8 {
            registry.release(id).expect("release");
        }
        assert!(registry.is_empty());
    }
}
