//! Immutable BenchFS v1 adapter between Linux low-level FUSE and the safe SDK.
//!
//! The portable core owns all validation, state bookkeeping and request-to-
//! trait mapping. The Linux-only native boundary is deliberately kept outside
//! the public SDK so filesystem implementations never depend on C ABI types.

mod adapter;
mod flags;
mod mapping;
mod mount_source;
mod native;
mod policy;

pub use adapter::{Adapter, Clock, Invalidation, SystemClock, XattrReply};
pub use flags::{
    FALLOC_FL_COLLAPSE_RANGE, FALLOC_FL_INSERT_RANGE, FALLOC_FL_KEEP_SIZE, FALLOC_FL_PUNCH_HOLE,
    FALLOC_FL_UNSHARE_RANGE, FALLOC_FL_ZERO_RANGE, O_ACCMODE, O_APPEND, O_DIRECTORY, O_DSYNC,
    O_EXCL, O_NOFOLLOW, O_RDONLY, O_RDWR, O_SYNC, O_TRUNC, O_WRONLY, RENAME_EXCHANGE,
    RENAME_NOREPLACE, RENAME_WHITEOUT, XATTR_CREATE, XATTR_REPLACE, normalize_fallocate_mode,
    normalize_open_options, normalize_rename_mode, normalize_xattr_mode,
};
pub use mapping::{LOW_LEVEL_MAPPINGS, LowLevelMapping, LowLevelOperation, MappingTarget};
pub use mount_source::MountSource;
pub use native::{RuntimeConfig, run_mount, run_mount_cow};
pub use policy::{CachePolicy, MountPolicy, UnsupportedPolicy};
