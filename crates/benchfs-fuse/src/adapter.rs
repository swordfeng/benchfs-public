//! Safe request normalization and runtime bookkeeping.

use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU64;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use benchfs_sdk::{
    CloneRequest, CowFilesystem, CreateResult, DeviceId, DirBatch, DirHandle, DirectoryCookie,
    Entry, Errno, FallocateMode, FileAttr, FileHandle, FileKind, FileOffset, FileRange,
    FileSyncMode, FilesystemOperations, FsResult, MountedSnapshot, Name, NodeKey, OpenOptions,
    OperationTime, PrivilegeClearMask, RenameMode, RequestContext, SeekKind, SetAttr, SnapshotInfo,
    SnapshotSelector, StatFs, SymlinkTarget, WriteOptions, WriteResult, XattrName, XattrSetMode,
    XattrValue,
};

/// Clock sampled exactly once for every logical mutation.
pub trait Clock: Send + Sync + 'static {
    /// Returns a representable BenchFS operation timestamp.
    fn now(&self) -> FsResult<OperationTime>;
}

/// Wall-clock implementation used by the production adapter.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> FsResult<OperationTime> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = i64::try_from(duration.as_secs()).map_err(|_| Errno::Overflow)?;
                OperationTime::new(seconds, duration.subsec_nanos())
            }
            Err(error) => {
                let duration = error.duration();
                let seconds = i64::try_from(duration.as_secs()).map_err(|_| Errno::Overflow)?;
                let nanos = duration.subsec_nanos();
                if nanos == 0 {
                    OperationTime::new(-seconds, 0)
                } else {
                    OperationTime::new(
                        seconds
                            .checked_add(1)
                            .and_then(i64::checked_neg)
                            .ok_or(Errno::Overflow)?,
                        1_000_000_000 - nanos,
                    )
                }
            }
        }
    }
}

/// Kernel cache invalidation required after a successful mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Invalidation {
    /// Expire attributes and optionally a byte range for one inode.
    Inode {
        /// Stable inode number.
        inode: u64,
        /// First invalidated byte; zero for all attributes/data.
        offset: i64,
        /// Length, or zero for the complete inode.
        length: i64,
    },
    /// Expire one parent/name dentry.
    Entry {
        /// Parent inode number.
        parent: u64,
        /// Raw component bytes.
        name: Vec<u8>,
    },
}

#[derive(Clone, Copy, Debug)]
struct LookupRecord {
    key: NodeKey,
    references: u64,
}

#[derive(Clone, Copy, Debug)]
struct FileRecord {
    node: NodeKey,
    options: OpenOptions,
}

#[derive(Clone, Copy, Debug)]
struct DirectoryRecord {
    node: NodeKey,
}

#[derive(Debug)]
struct RuntimeState {
    lookups: BTreeMap<u64, LookupRecord>,
    files: BTreeMap<u64, FileRecord>,
    directories: BTreeMap<u64, DirectoryRecord>,
}

impl RuntimeState {
    fn new() -> Self {
        let mut lookups = BTreeMap::new();
        lookups.insert(
            NodeKey::ROOT.inode_id(),
            LookupRecord {
                key: NodeKey::ROOT,
                references: u64::MAX,
            },
        );
        Self {
            lookups,
            files: BTreeMap::new(),
            directories: BTreeMap::new(),
        }
    }
}

/// Safe, concurrent low-level request adapter for one mounted filesystem view.
pub struct Adapter<F: FilesystemOperations + ?Sized, C: Clock = SystemClock> {
    filesystem: Arc<F>,
    clock: C,
    state: Mutex<RuntimeState>,
}

impl<F: FilesystemOperations + ?Sized> Adapter<F, SystemClock> {
    /// Wraps a filesystem using the system wall clock.
    #[must_use]
    pub fn new(filesystem: Arc<F>) -> Self {
        Self::with_clock(filesystem, SystemClock)
    }
}

impl<F: FilesystemOperations + ?Sized, C: Clock> Adapter<F, C> {
    /// Wraps a filesystem with an injectable clock for deterministic tests.
    #[must_use]
    pub fn with_clock(filesystem: Arc<F>, clock: C) -> Self {
        Self {
            filesystem,
            clock,
            state: Mutex::new(RuntimeState::new()),
        }
    }

    /// Returns the wrapped implementation.
    #[must_use]
    pub fn filesystem(&self) -> &Arc<F> {
        &self.filesystem
    }

    /// Samples the adapter clock for a normalized open-time truncate intent.
    pub fn sample_operation_time(&self) -> FsResult<OperationTime> {
        self.clock.now()
    }

    fn state(&self) -> FsResult<MutexGuard<'_, RuntimeState>> {
        self.state.lock().map_err(|_| Errno::Io.into())
    }

    fn resolve_node(&self, inode: u64) -> FsResult<NodeKey> {
        self.state()?
            .lookups
            .get(&inode)
            .map(|record| record.key)
            .ok_or_else(|| Errno::Stale.into())
    }

    fn validate_entry(entry: &Entry) -> FsResult<()> {
        if entry.node != entry.attributes.node || entry.kind != entry.attributes.kind {
            return Err(Errno::Corrupt.into());
        }
        Self::validate_attr(&entry.attributes)
    }

    fn validate_attr(attribute: &FileAttr) -> FsResult<()> {
        if FileKind::from_mode(attribute.mode) != Some(attribute.kind)
            || attribute.size > benchfs_sdk::MAX_FILE_SIZE
        {
            return Err(Errno::Corrupt.into());
        }
        Ok(())
    }

    fn require_entry(entry: &Entry, kind: FileKind, node: Option<NodeKey>) -> FsResult<()> {
        if entry.kind != kind || node.is_some_and(|expected| entry.node != expected) {
            return Err(Errno::Corrupt.into());
        }
        Ok(())
    }

    fn remember_entry(&self, entry: &Entry) -> FsResult<()> {
        Self::validate_entry(entry)?;
        let mut state = self.state()?;
        match state.lookups.get_mut(&entry.node.inode_id()) {
            Some(record) if record.key != entry.node => Err(Errno::Stale.into()),
            Some(record) if record.references == u64::MAX => Ok(()),
            Some(record) => {
                record.references = record.references.checked_add(1).ok_or(Errno::Overflow)?;
                Ok(())
            }
            None => {
                state.lookups.insert(
                    entry.node.inode_id(),
                    LookupRecord {
                        key: entry.node,
                        references: 1,
                    },
                );
                Ok(())
            }
        }
    }

    fn file_record(&self, handle: FileHandle) -> FsResult<FileRecord> {
        self.state()?
            .files
            .get(&handle.get())
            .copied()
            .ok_or_else(|| Errno::BadFileDescriptor.into())
    }

    fn directory_record(&self, handle: DirHandle) -> FsResult<DirectoryRecord> {
        self.state()?
            .directories
            .get(&handle.get())
            .copied()
            .ok_or_else(|| Errno::BadFileDescriptor.into())
    }

    fn remember_file(
        &self,
        handle: FileHandle,
        node: NodeKey,
        options: OpenOptions,
    ) -> FsResult<()> {
        let replaced = self
            .state()?
            .files
            .insert(handle.get(), FileRecord { node, options });
        if replaced.is_some() {
            return Err(Errno::Corrupt.into());
        }
        Ok(())
    }

    /// Maps lookup and acquires one kernel lookup reference.
    pub async fn lookup(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
    ) -> FsResult<Entry> {
        let parent = self.resolve_node(parent)?;
        let name = Name::new(name.to_vec())?;
        let entry = self.filesystem.lookup(context, parent, &name).await?;
        self.remember_entry(&entry)?;
        Ok(entry)
    }

    /// Releases lookup references. This notification has no FUSE reply.
    pub async fn forget(&self, inode: u64, count: u64) {
        if inode == NodeKey::ROOT.inode_id() {
            return;
        }
        let forgotten = {
            let Ok(mut state) = self.state.lock() else {
                return;
            };
            let has_handle = state
                .files
                .values()
                .any(|record| record.node.inode_id() == inode)
                || state
                    .directories
                    .values()
                    .any(|record| record.node.inode_id() == inode);
            if let Some(record) = state.lookups.get_mut(&inode) {
                let new_count = record.references.checked_sub(count);
                match new_count {
                    Some(remaining) => record.references = remaining,
                    None => {
                        debug_assert!(
                            false,
                            "forget({inode}, {count}) exceeds tracked references {}",
                            record.references
                        );
                        record.references = 0;
                    }
                }
                if record.references == 0 && !has_handle {
                    state.lookups.remove(&inode);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if forgotten {
            self.filesystem.forget(inode).await;
        }
    }

    /// Maps getattr, including handle-attributed fstat.
    pub async fn getattr(
        &self,
        context: &RequestContext,
        inode: u64,
        handle: Option<FileHandle>,
    ) -> FsResult<FileAttr> {
        let node = if let Some(handle) = handle {
            let node = self.file_record(handle)?.node;
            if node.inode_id() != inode {
                return Err(Errno::BadFileDescriptor.into());
            }
            node
        } else {
            self.resolve_node(inode)?
        };
        let attribute = self.filesystem.getattr(context, node, handle).await?;
        if attribute.node != node {
            return Err(Errno::Corrupt.into());
        }
        Self::validate_attr(&attribute)?;
        Ok(attribute)
    }

    /// Maps one combined setattr and returns its cache invalidation.
    pub async fn setattr(
        &self,
        context: &RequestContext,
        inode: u64,
        attributes: &SetAttr,
    ) -> FsResult<(FileAttr, Invalidation)> {
        let node = self.resolve_node(inode)?;
        if let Some(handle) = attributes.handle {
            if self.file_record(handle)?.node != node {
                return Err(Errno::BadFileDescriptor.into());
            }
        }
        let attr = self
            .filesystem
            .setattr(context, node, attributes, self.clock.now()?)
            .await?;
        if attr.node != node {
            return Err(Errno::Corrupt.into());
        }
        Self::validate_attr(&attr)?;
        Ok((attr, Self::inode_invalidation(node)))
    }

    /// Maps atomic create-and-open.
    pub async fn create(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
        mode: u32,
        options: OpenOptions,
    ) -> FsResult<(CreateResult, Vec<Invalidation>)> {
        let parent = self.resolve_node(parent)?;
        let name = Name::new(name.to_vec())?;
        let result = self
            .filesystem
            .create(context, parent, &name, mode, options, self.clock.now()?)
            .await?;
        Self::require_entry(&result.entry, FileKind::Regular, None)?;
        self.remember_entry(&result.entry)?;
        self.remember_file(result.handle, result.entry.node, options)?;
        Ok((
            result,
            vec![
                Self::entry_invalidation(parent, &name),
                Self::inode_invalidation(parent),
            ],
        ))
    }

    /// Maps mknod.
    pub async fn mknod(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
        kind: benchfs_sdk::FileKind,
        mode: u32,
        device: DeviceId,
    ) -> FsResult<(Entry, Vec<Invalidation>)> {
        let parent = self.resolve_node(parent)?;
        let name = Name::new(name.to_vec())?;
        let entry = self
            .filesystem
            .mknod(
                context,
                parent,
                &name,
                kind,
                mode,
                device,
                self.clock.now()?,
            )
            .await?;
        Self::require_entry(&entry, kind, None)?;
        self.remember_entry(&entry)?;
        Ok((
            entry,
            vec![
                Self::entry_invalidation(parent, &name),
                Self::inode_invalidation(parent),
            ],
        ))
    }

    /// Maps mkdir.
    pub async fn mkdir(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
        mode: u32,
    ) -> FsResult<(Entry, Vec<Invalidation>)> {
        let parent = self.resolve_node(parent)?;
        let name = Name::new(name.to_vec())?;
        let entry = self
            .filesystem
            .mkdir(context, parent, &name, mode, self.clock.now()?)
            .await?;
        Self::require_entry(&entry, FileKind::Directory, None)?;
        self.remember_entry(&entry)?;
        Ok((
            entry,
            vec![
                Self::entry_invalidation(parent, &name),
                Self::inode_invalidation(parent),
            ],
        ))
    }

    /// Maps symlink creation.
    pub async fn symlink(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
        target: &[u8],
    ) -> FsResult<(Entry, Vec<Invalidation>)> {
        let parent = self.resolve_node(parent)?;
        let name = Name::new(name.to_vec())?;
        let target = SymlinkTarget::new(target.to_vec())?;
        let entry = self
            .filesystem
            .symlink(context, parent, &name, &target, self.clock.now()?)
            .await?;
        Self::require_entry(&entry, FileKind::Symlink, None)?;
        self.remember_entry(&entry)?;
        Ok((
            entry,
            vec![
                Self::entry_invalidation(parent, &name),
                Self::inode_invalidation(parent),
            ],
        ))
    }

    /// Maps hard-link creation.
    pub async fn link(
        &self,
        context: &RequestContext,
        source: u64,
        new_parent: u64,
        new_name: &[u8],
    ) -> FsResult<(Entry, Vec<Invalidation>)> {
        let source = self.resolve_node(source)?;
        let parent = self.resolve_node(new_parent)?;
        let name = Name::new(new_name.to_vec())?;
        let entry = self
            .filesystem
            .link(context, source, parent, &name, self.clock.now()?)
            .await?;
        Self::require_entry(&entry, entry.kind, Some(source))?;
        self.remember_entry(&entry)?;
        Ok((
            entry,
            vec![
                Self::entry_invalidation(parent, &name),
                Self::inode_invalidation(parent),
                Self::inode_invalidation(source),
            ],
        ))
    }

    /// Maps unlink.
    pub async fn unlink(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
    ) -> FsResult<Vec<Invalidation>> {
        self.remove_name(context, parent, name, false).await
    }

    /// Maps rmdir.
    pub async fn rmdir(
        &self,
        context: &RequestContext,
        parent: u64,
        name: &[u8],
    ) -> FsResult<Vec<Invalidation>> {
        self.remove_name(context, parent, name, true).await
    }

    async fn remove_name(
        &self,
        context: &RequestContext,
        parent: u64,
        raw_name: &[u8],
        directory: bool,
    ) -> FsResult<Vec<Invalidation>> {
        let parent = self.resolve_node(parent)?;
        let name = Name::new(raw_name.to_vec())?;
        if directory {
            self.filesystem
                .rmdir(context, parent, &name, self.clock.now()?)
                .await?;
        } else {
            self.filesystem
                .unlink(context, parent, &name, self.clock.now()?)
                .await?;
        }
        Ok(vec![
            Self::entry_invalidation(parent, &name),
            Self::inode_invalidation(parent),
        ])
    }

    /// Maps rename/renameat2.
    pub async fn rename(
        &self,
        context: &RequestContext,
        old_parent: u64,
        old_name: &[u8],
        new_parent: u64,
        new_name: &[u8],
        mode: RenameMode,
    ) -> FsResult<Vec<Invalidation>> {
        let old_parent = self.resolve_node(old_parent)?;
        let new_parent = self.resolve_node(new_parent)?;
        let old_name = Name::new(old_name.to_vec())?;
        let new_name = Name::new(new_name.to_vec())?;
        self.filesystem
            .rename(
                context,
                old_parent,
                &old_name,
                new_parent,
                &new_name,
                mode,
                self.clock.now()?,
            )
            .await?;
        Ok(vec![
            Self::entry_invalidation(old_parent, &old_name),
            Self::entry_invalidation(new_parent, &new_name),
            Self::inode_invalidation(old_parent),
            Self::inode_invalidation(new_parent),
        ])
    }

    /// Maps readlink.
    pub async fn readlink(&self, context: &RequestContext, inode: u64) -> FsResult<Vec<u8>> {
        let target = self
            .filesystem
            .readlink(context, self.resolve_node(inode)?)
            .await?;
        SymlinkTarget::new(target.clone()).map_err(|_| Errno::Corrupt)?;
        Ok(target)
    }

    /// Maps regular-file open and records immutable open-description flags.
    pub async fn open(
        &self,
        context: &RequestContext,
        inode: u64,
        options: OpenOptions,
    ) -> FsResult<(FileHandle, Option<Invalidation>)> {
        let node = self.resolve_node(inode)?;
        let handle = self.filesystem.open(context, node, options).await?;
        self.remember_file(handle, node, options)?;
        Ok((
            handle,
            options.truncate.map(|_| Self::inode_invalidation(node)),
        ))
    }

    /// Maps release. The handle becomes invalid even if final cleanup fails.
    pub async fn release(&self, handle: FileHandle) -> FsResult<()> {
        let record = self
            .state()?
            .files
            .remove(&handle.get())
            .ok_or(Errno::BadFileDescriptor)?;
        let result = self.filesystem.release(handle).await;
        self.drop_unreferenced(record.node.inode_id()).await;
        result
    }

    /// Maps opendir.
    pub async fn opendir(&self, context: &RequestContext, inode: u64) -> FsResult<DirHandle> {
        let node = self.resolve_node(inode)?;
        let handle = self.filesystem.opendir(context, node).await?;
        if self
            .state()?
            .directories
            .insert(handle.get(), DirectoryRecord { node })
            .is_some()
        {
            return Err(Errno::Corrupt.into());
        }
        Ok(handle)
    }

    /// Maps handle-local readdir cookies.
    pub async fn readdir(
        &self,
        context: &RequestContext,
        handle: DirHandle,
        cookie: DirectoryCookie,
        max_entries: u32,
    ) -> FsResult<DirBatch> {
        let directory = self.directory_record(handle)?;
        let batch = self
            .filesystem
            .readdir(context, handle, cookie, max_entries)
            .await?;
        if batch.entries.len() > max_entries as usize
            || (!batch.end_of_directory && batch.entries.is_empty())
        {
            return Err(Errno::Corrupt.into());
        }
        let mut cookies = BTreeSet::new();
        for entry in &batch.entries {
            match entry.name.as_slice() {
                b"." if entry.node == directory.node && entry.kind == FileKind::Directory => {}
                b".." if entry.kind == FileKind::Directory => {}
                b"." | b".." => return Err(Errno::Corrupt.into()),
                name if Name::new(name.to_vec()).is_ok() => {}
                _ => return Err(Errno::Corrupt.into()),
            }
            if !cookies.insert(entry.next_cookie.get()) {
                return Err(Errno::Corrupt.into());
            }
        }
        let expected_continuation = if let Some(entry) = batch.entries.last() {
            let value = NonZeroU64::new(entry.next_cookie.get()).ok_or(Errno::Corrupt)?;
            DirectoryCookie::from_returned(value)
        } else {
            cookie
        };
        if batch.continuation != expected_continuation {
            return Err(Errno::Corrupt.into());
        }
        Ok(batch)
    }

    /// Maps releasedir. The handle and all its cookies become invalid.
    pub async fn releasedir(&self, handle: DirHandle) -> FsResult<()> {
        let record = self
            .state()?
            .directories
            .remove(&handle.get())
            .ok_or(Errno::BadFileDescriptor)?;
        let result = self.filesystem.releasedir(handle).await;
        self.drop_unreferenced(record.node.inode_id()).await;
        result
    }

    /// Releases every handle still owned by the adapter during shutdown.
    ///
    /// The caller must first stop admission and wait for active operations.
    /// All handles are invalidated before cleanup begins; cleanup continues
    /// after individual failures and returns `EIO` if any release failed.
    pub async fn drain_handles(&self) -> FsResult<()> {
        let (files, directories) = {
            let mut state = self.state()?;
            (
                std::mem::take(&mut state.files),
                std::mem::take(&mut state.directories),
            )
        };
        let mut failed = false;
        for (raw_handle, record) in files {
            let result = match FileHandle::new(raw_handle) {
                Ok(handle) => self.filesystem.release(handle).await,
                Err(_) => Err(Errno::Corrupt.into()),
            };
            failed |= result.is_err();
            self.drop_unreferenced(record.node.inode_id()).await;
        }
        for (raw_handle, record) in directories {
            let result = match DirHandle::new(raw_handle) {
                Ok(handle) => self.filesystem.releasedir(handle).await,
                Err(_) => Err(Errno::Corrupt.into()),
            };
            failed |= result.is_err();
            self.drop_unreferenced(record.node.inode_id()).await;
        }
        if failed {
            Err(Errno::Io.into())
        } else {
            Ok(())
        }
    }

    /// Maps regular-file read.
    pub async fn read(
        &self,
        context: &RequestContext,
        handle: FileHandle,
        offset: FileOffset,
        length: u64,
    ) -> FsResult<Vec<u8>> {
        let record = self.file_record(handle)?;
        if !record.options.access.readable() {
            return Err(Errno::BadFileDescriptor.into());
        }
        let data = self
            .filesystem
            .read(context, handle, offset, length)
            .await?;
        if u64::try_from(data.len()).map_or(true, |actual| actual > length) {
            return Err(Errno::Corrupt.into());
        }
        Ok(data)
    }

    /// Maps a regular-file write with handle flags and protocol clear-mask.
    pub async fn write(
        &self,
        context: &RequestContext,
        handle: FileHandle,
        offset: FileOffset,
        data: &[u8],
        privilege_clear: PrivilegeClearMask,
    ) -> FsResult<(WriteResult, Invalidation)> {
        let record = self.file_record(handle)?;
        if !record.options.access.writable() {
            return Err(Errno::BadFileDescriptor.into());
        }
        let result = self
            .filesystem
            .write(
                context,
                handle,
                offset,
                data,
                WriteOptions {
                    append: record.options.append,
                    sync: record.options.sync,
                    privilege_clear,
                },
                self.clock.now()?,
            )
            .await?;
        if u64::try_from(data.len()).map_or(true, |requested| result.bytes_written > requested) {
            return Err(Errno::Corrupt.into());
        }
        Ok((result, Self::inode_invalidation(record.node)))
    }

    /// Maps SEEK_DATA/SEEK_HOLE.
    pub async fn seek(
        &self,
        context: &RequestContext,
        inode: u64,
        handle: FileHandle,
        offset: FileOffset,
        kind: SeekKind,
    ) -> FsResult<FileOffset> {
        let record = self.file_record(handle)?;
        if record.node.inode_id() != inode {
            return Err(Errno::BadFileDescriptor.into());
        }
        self.filesystem
            .seek(context, record.node, offset, kind)
            .await
    }

    /// Maps fallocate range modes after rejecting a non-normalized empty range.
    pub async fn fallocate(
        &self,
        context: &RequestContext,
        handle: FileHandle,
        mode: FallocateMode,
        range: FileRange,
        privilege_clear: PrivilegeClearMask,
    ) -> FsResult<Invalidation> {
        if range.is_empty() {
            return Err(Errno::InvalidArgument.into());
        }
        let record = self.file_record(handle)?;
        if !record.options.access.writable() {
            return Err(Errno::BadFileDescriptor.into());
        }
        self.filesystem
            .fallocate(
                context,
                handle,
                mode,
                range,
                privilege_clear,
                self.clock.now()?,
            )
            .await?;
        Ok(Self::inode_invalidation(record.node))
    }

    /// Maps setxattr.
    pub async fn setxattr(
        &self,
        context: &RequestContext,
        inode: u64,
        name: &[u8],
        value: &[u8],
        mode: XattrSetMode,
    ) -> FsResult<Invalidation> {
        let node = self.resolve_node(inode)?;
        self.filesystem
            .setxattr(
                context,
                node,
                &XattrName::new(name.to_vec())?,
                &XattrValue::new(value.to_vec())?,
                mode,
                self.clock.now()?,
            )
            .await?;
        Ok(Self::inode_invalidation(node))
    }

    /// Maps getxattr and performs Linux size-probe/buffer negotiation.
    pub async fn getxattr(
        &self,
        context: &RequestContext,
        inode: u64,
        name: &[u8],
        buffer_size: usize,
    ) -> FsResult<XattrReply> {
        let value = self
            .filesystem
            .getxattr(
                context,
                self.resolve_node(inode)?,
                &XattrName::new(name.to_vec())?,
            )
            .await?;
        XattrReply::negotiate(value.into_bytes(), buffer_size)
    }

    /// Maps listxattr and serializes NUL-delimited Linux names.
    pub async fn listxattr(
        &self,
        context: &RequestContext,
        inode: u64,
        buffer_size: usize,
    ) -> FsResult<XattrReply> {
        let names = self
            .filesystem
            .listxattr(context, self.resolve_node(inode)?)
            .await?;
        let required = names.iter().try_fold(0_usize, |total, name| {
            total
                .checked_add(name.as_bytes().len() + 1)
                .ok_or(Errno::Overflow)
        })?;
        let mut bytes = Vec::with_capacity(required);
        for name in names {
            bytes.extend_from_slice(name.as_bytes());
            bytes.push(0);
        }
        XattrReply::negotiate(bytes, buffer_size)
    }

    /// Maps removexattr.
    pub async fn removexattr(
        &self,
        context: &RequestContext,
        inode: u64,
        name: &[u8],
    ) -> FsResult<Invalidation> {
        let node = self.resolve_node(inode)?;
        self.filesystem
            .removexattr(
                context,
                node,
                &XattrName::new(name.to_vec())?,
                self.clock.now()?,
            )
            .await?;
        Ok(Self::inode_invalidation(node))
    }

    /// Maps statfs.
    pub async fn statfs(&self) -> FsResult<StatFs> {
        let value = self.filesystem.statfs().await?;
        if value.block_size != 4096
            || value.fragment_size != 4096
            || value.name_max != 255
            || value.blocks_available > value.blocks_free
            || value.blocks_free > value.blocks
            || value.files_free > value.files
        {
            return Err(Errno::Corrupt.into());
        }
        Ok(value)
    }

    /// Maps close-time flush error reporting.
    pub async fn flush(&self, handle: FileHandle) -> FsResult<()> {
        self.file_record(handle)?;
        self.filesystem.flush(handle).await
    }

    /// Maps fsync/fdatasync after the kernel has written dirty cached pages.
    pub async fn fsync(&self, handle: FileHandle, data_only: bool) -> FsResult<()> {
        self.file_record(handle)?;
        self.filesystem
            .fsync(
                handle,
                if data_only {
                    FileSyncMode::Data
                } else {
                    FileSyncMode::Full
                },
            )
            .await
    }

    /// Maps directory fsync.
    pub async fn fsyncdir(&self, handle: DirHandle) -> FsResult<()> {
        self.directory_record(handle)?;
        self.filesystem.fsyncdir(handle).await
    }

    async fn drop_unreferenced(&self, inode: u64) {
        if inode == NodeKey::ROOT.inode_id() {
            return;
        }
        let forgotten = {
            let Ok(mut state) = self.state.lock() else {
                return;
            };
            let has_handle = state
                .files
                .values()
                .any(|record| record.node.inode_id() == inode)
                || state
                    .directories
                    .values()
                    .any(|record| record.node.inode_id() == inode);
            if !has_handle
                && state
                    .lookups
                    .get(&inode)
                    .is_some_and(|record| record.references == 0)
            {
                state.lookups.remove(&inode);
                true
            } else {
                false
            }
        };
        if forgotten {
            self.filesystem.forget(inode).await;
        }
    }

    fn inode_invalidation(node: NodeKey) -> Invalidation {
        Invalidation::Inode {
            inode: node.inode_id(),
            offset: 0,
            length: 0,
        }
    }

    fn entry_invalidation(parent: NodeKey, name: &Name) -> Invalidation {
        Invalidation::Entry {
            parent: parent.inode_id(),
            name: name.as_bytes().to_vec(),
        }
    }
}

impl<F: CowFilesystem, C: Clock> Adapter<F, C> {
    /// Maps fixed-control reflink and invalidates the destination cache.
    pub async fn clone_range(
        &self,
        context: &RequestContext,
        request: CloneRequest,
    ) -> FsResult<Invalidation> {
        let source = self.file_record(request.source)?;
        let destination = self.file_record(request.destination)?;
        if !source.options.access.readable()
            || !destination.options.access.writable()
            || destination.options.append
        {
            return Err(Errno::BadFileDescriptor.into());
        }
        self.filesystem
            .clone_range(context, request, self.clock.now()?)
            .await?;
        Ok(Self::inode_invalidation(destination.node))
    }

    /// Maps fixed-control snapshot creation and samples its operation time once.
    pub async fn snapshot_create(
        &self,
        context: &RequestContext,
        name: &[u8],
    ) -> FsResult<SnapshotInfo> {
        let name = Name::new(name.to_vec())?;
        self.filesystem
            .snapshot_create(context, &name, self.clock.now()?)
            .await
    }

    /// Maps fixed-control snapshot listing.
    pub async fn snapshot_list(&self, context: &RequestContext) -> FsResult<Vec<SnapshotInfo>> {
        self.filesystem.snapshot_list(context).await
    }

    /// Acquires an independently handled read-only snapshot view.
    pub async fn snapshot_mount(
        &self,
        context: &RequestContext,
        selector: &SnapshotSelector,
    ) -> FsResult<MountedSnapshot<F::SnapshotView>> {
        self.filesystem.snapshot_mount(context, selector).await
    }

    /// Releases a snapshot view previously returned by [`Self::snapshot_mount`].
    pub async fn snapshot_unmount(
        &self,
        snapshot: MountedSnapshot<F::SnapshotView>,
    ) -> FsResult<()> {
        self.filesystem.snapshot_unmount(snapshot).await
    }

    /// Maps fixed-control snapshot deletion.
    pub async fn snapshot_delete(
        &self,
        context: &RequestContext,
        selector: &SnapshotSelector,
    ) -> FsResult<()> {
        self.filesystem.snapshot_delete(context, selector).await
    }

    /// Maps fixed-control full garbage collection.
    pub async fn gc(&self, context: &RequestContext) -> FsResult<()> {
        self.filesystem.gc(context).await
    }
}

/// Result of Linux xattr size negotiation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum XattrReply {
    /// Size probe result.
    Size(usize),
    /// Complete value/list bytes.
    Data(Vec<u8>),
}

impl XattrReply {
    fn negotiate(bytes: Vec<u8>, buffer_size: usize) -> FsResult<Self> {
        if buffer_size == 0 {
            return Ok(Self::Size(bytes.len()));
        }
        if buffer_size < bytes.len() {
            return Err(Errno::Range.into());
        }
        Ok(Self::Data(bytes))
    }
}
