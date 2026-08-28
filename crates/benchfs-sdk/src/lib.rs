//! Public implementation contract and standard block-device adapters for
//! BenchFS v1.
//!
//! This crate contains no filesystem or FUSE implementation. It defines the
//! filesystem traits and provides the real/file/memory BlockDevice adapters
//! used by implementations and tests. Observable filesystem behavior is
//! normative in `agent-docs/SPEC.md`.

mod block_device;
mod checksum;
mod error;
mod filesystem;
mod operation;
mod snapshot_registry;
mod trace;
mod types;

pub use block_device::{
    BlockDevice, DIRECT_IO_ALIGNMENT, DeviceCompletion, DeviceError, DeviceErrorKind, DeviceIoVec,
    DeviceRange, DeviceResult, DeviceSegment, FileBlockDevice, FileSimBlockDevice,
    MAX_DEVICE_BYTES, MAX_DEVICE_SEGMENTS, MemBlockDevice,
};
pub use checksum::crc32c;
pub use error::{Errno, FsError, FsResult};
pub use filesystem::{
    CoreFilesystem, CowFilesystem, Filesystem, FilesystemOperations, JournalFilesystem,
};
pub use operation::{
    OPERATION_BINDINGS, OperationBinding, OperationId, OperationProfile, OperationSurface,
};
pub use snapshot_registry::{SnapshotIdKey, SnapshotMountRegistry};
pub use trace::{
    CrashPoint, EventPhase, RequestEvent, RequestId, RequestKind, TaskId, WritePayload,
    WritePayloadRef,
};
pub use types::*;

/// BenchFS v1 filesystem block size in bytes.
pub const BLOCK_SIZE: u64 = 4096;

/// Crash-model atomic sector size in bytes.
pub const SECTOR_SIZE: u64 = 512;

/// Largest logical file size exposed by BenchFS v1.
pub const MAX_FILE_SIZE: u64 = i64::MAX as u64;

/// Smallest block-device size accepted by the v1 mount contract (64 MiB).
pub const MIN_DEVICE_BYTES: u64 = 64 * 1024 * 1024;

/// Largest block-device size required by qualification/evaluation (1 TiB).
pub const MAX_QUALIFIED_DEVICE_BYTES: u64 = 1024 * 1024 * 1024 * 1024;

/// Largest physical block count representable by the v1 on-disk format.
pub const MAX_FORMAT_BLOCKS: u64 = u64::MAX / BLOCK_SIZE;

/// Maximum raw pathname-component length.
pub const MAX_NAME_LEN: usize = 255;

/// Maximum complete syscall path length enforced by the adapter/VFS boundary.
pub const MAX_PATH_LEN: usize = 4095;

/// Maximum raw symbolic-link target length.
pub const MAX_SYMLINK_TARGET_LEN: usize = 4095;

/// Maximum API-visible per-inode extended-attribute budget.
pub const MAX_XATTR_BYTES: usize = 65_536;

/// Linux file-type mask (`S_IFMT`).
pub const S_IFMT: u32 = 0o170_000;
/// Linux socket type bits (`S_IFSOCK`).
pub const S_IFSOCK: u32 = 0o140_000;
/// Linux symbolic-link type bits (`S_IFLNK`).
pub const S_IFLNK: u32 = 0o120_000;
/// Linux regular-file type bits (`S_IFREG`).
pub const S_IFREG: u32 = 0o100_000;
/// Linux block-device type bits (`S_IFBLK`).
pub const S_IFBLK: u32 = 0o060_000;
/// Linux directory type bits (`S_IFDIR`).
pub const S_IFDIR: u32 = 0o040_000;
/// Linux character-device type bits (`S_IFCHR`).
pub const S_IFCHR: u32 = 0o020_000;
/// Linux FIFO type bits (`S_IFIFO`).
pub const S_IFIFO: u32 = 0o010_000;
/// All Linux permission and special bits legal in BenchFS v1.
pub const MODE_PERMISSIONS: u32 = 0o007_777;
/// Linux set-user-ID bit (`S_ISUID`).
pub const S_ISUID: u32 = 0o004_000;
/// Linux set-group-ID bit (`S_ISGID`).
pub const S_ISGID: u32 = 0o002_000;
/// Linux sticky bit (`S_ISVTX`).
pub const S_ISVTX: u32 = 0o001_000;
/// Linux owner read/write/execute mask (`S_IRWXU`).
pub const S_IRWXU: u32 = 0o000_700;
/// Linux group read/write/execute mask (`S_IRWXG`).
pub const S_IRWXG: u32 = 0o000_070;
/// Linux other read/write/execute mask (`S_IRWXO`).
pub const S_IRWXO: u32 = 0o000_007;
