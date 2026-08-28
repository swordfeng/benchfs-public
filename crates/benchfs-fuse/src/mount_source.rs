//! Snapshot mount-source selection for daemon FUSE sessions.

use benchfs_sdk::{FsError, FsResult, Name, SnapshotId, SnapshotSelector};

/// Which view a daemon FUSE session serves.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MountSource {
    /// The writable COW current view.
    Current,
    /// One captured read-only snapshot view, selected by ID or exact name.
    Snapshot(SnapshotSelector),
}

impl MountSource {
    /// Builds a mount source from `BENCHFS_SNAPSHOT`.
    ///
    /// A missing or empty variable serves the current view. A pure decimal
    /// value selects by snapshot ID; any other value selects by exact raw
    /// catalog name.
    pub fn from_env() -> FsResult<Option<Self>> {
        let Some(raw) = std::env::var_os("BENCHFS_SNAPSHOT") else {
            return Ok(None);
        };
        let raw = raw
            .to_str()
            .ok_or_else(|| FsError::from(benchfs_sdk::Errno::InvalidArgument))?;
        if raw.is_empty() {
            return Ok(None);
        }
        if let Ok(id) = raw.parse::<u64>() {
            return Ok(Some(Self::Snapshot(SnapshotSelector::Id(SnapshotId::new(
                id,
            )?))));
        }
        Ok(Some(Self::Snapshot(SnapshotSelector::Name(Name::new(
            raw.as_bytes().to_vec(),
        )?))))
    }
}
