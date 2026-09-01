//! Frozen Linux x86-64 UAPI flag normalization.

use benchfs_sdk::{
    AccessMode, Errno, FallocateMode, FsResult, OpenOptions, PrivilegeClearMask, RenameMode,
    SyncMode, TruncateIntent, XattrSetMode,
};

/// Linux `O_ACCMODE`.
pub const O_ACCMODE: u32 = 0o3;
/// Linux `O_RDONLY`.
pub const O_RDONLY: u32 = 0;
/// Linux `O_WRONLY`.
pub const O_WRONLY: u32 = 0o1;
/// Linux `O_RDWR`.
pub const O_RDWR: u32 = 0o2;
/// Linux `O_EXCL`.
pub const O_EXCL: u32 = 0o200;
/// Linux `O_TRUNC`.
pub const O_TRUNC: u32 = 0o1000;
/// Linux `O_APPEND`.
pub const O_APPEND: u32 = 0o2000;
/// Linux `O_DSYNC`.
pub const O_DSYNC: u32 = 0o10000;
/// Linux `O_DIRECTORY`.
pub const O_DIRECTORY: u32 = 0o200_000;
/// Linux `O_NOFOLLOW`.
pub const O_NOFOLLOW: u32 = 0o400_000;
/// Linux `O_SYNC` (includes the historical `O_DSYNC` bit).
pub const O_SYNC: u32 = 0o4_010_000;

/// Linux `RENAME_NOREPLACE`.
pub const RENAME_NOREPLACE: u32 = 1;
/// Linux `RENAME_EXCHANGE`.
pub const RENAME_EXCHANGE: u32 = 2;
/// Linux `RENAME_WHITEOUT`.
pub const RENAME_WHITEOUT: u32 = 4;

/// Linux `FALLOC_FL_KEEP_SIZE`.
pub const FALLOC_FL_KEEP_SIZE: u32 = 0x01;
/// Linux `FALLOC_FL_PUNCH_HOLE`.
pub const FALLOC_FL_PUNCH_HOLE: u32 = 0x02;
/// Linux `FALLOC_FL_COLLAPSE_RANGE` (well-formed but unsupported by BenchFS v1).
pub const FALLOC_FL_COLLAPSE_RANGE: u32 = 0x08;
/// Linux `FALLOC_FL_ZERO_RANGE`.
pub const FALLOC_FL_ZERO_RANGE: u32 = 0x10;
/// Linux `FALLOC_FL_INSERT_RANGE` (well-formed but unsupported by BenchFS v1).
pub const FALLOC_FL_INSERT_RANGE: u32 = 0x20;
/// Linux `FALLOC_FL_UNSHARE_RANGE` (well-formed but unsupported by BenchFS v1).
pub const FALLOC_FL_UNSHARE_RANGE: u32 = 0x40;

/// Linux `XATTR_CREATE`.
pub const XATTR_CREATE: u32 = 0x1;
/// Linux `XATTR_REPLACE`.
pub const XATTR_REPLACE: u32 = 0x2;

/// Linux `FUSE_SET_ATTR_MODE`.
pub const SET_ATTR_MODE: u32 = 1 << 0;
/// Linux `FUSE_SET_ATTR_UID`.
pub const SET_ATTR_UID: u32 = 1 << 1;
/// Linux `FUSE_SET_ATTR_GID`.
pub const SET_ATTR_GID: u32 = 1 << 2;
/// Linux `FUSE_SET_ATTR_SIZE`.
pub const SET_ATTR_SIZE: u32 = 1 << 3;
/// Linux `FUSE_SET_ATTR_KILL_SUID`.
pub const SET_ATTR_KILL_SUID: u32 = 1 << 11;
/// Linux `FUSE_SET_ATTR_KILL_SGID`.
pub const SET_ATTR_KILL_SGID: u32 = 1 << 12;
/// Linux `FUSE_SET_ATTR_KILL_PRIV`.
pub const SET_ATTR_KILL_PRIV: u32 = 1 << 14;

/// Normalizes the open/create flags that have persistent SDK meaning.
pub fn normalize_open_options(
    flags: u32,
    truncate: Option<TruncateIntent>,
) -> FsResult<OpenOptions> {
    let access = match flags & O_ACCMODE {
        O_RDONLY => AccessMode::ReadOnly,
        O_WRONLY => AccessMode::WriteOnly,
        O_RDWR => AccessMode::ReadWrite,
        _ => return Err(Errno::InvalidArgument.into()),
    };
    if flags & O_TRUNC != 0 && !access.writable() {
        return Err(Errno::InvalidArgument.into());
    }
    let truncate = match (flags & O_TRUNC != 0, truncate) {
        (true, Some(intent)) => Some(intent),
        (false, None) => None,
        _ => return Err(Errno::InvalidArgument.into()),
    };
    let sync = if flags & O_SYNC == O_SYNC {
        SyncMode::Full
    } else if flags & O_DSYNC != 0 {
        SyncMode::Data
    } else {
        SyncMode::None
    };
    Ok(OpenOptions {
        access,
        append: flags & O_APPEND != 0,
        sync,
        truncate,
    })
}

/// Normalizes Linux `renameat2` flags.
pub fn normalize_rename_mode(flags: u32) -> FsResult<RenameMode> {
    match flags {
        0 => Ok(RenameMode::Replace),
        RENAME_NOREPLACE => Ok(RenameMode::NoReplace),
        RENAME_EXCHANGE => Ok(RenameMode::Exchange),
        _ => Err(Errno::InvalidArgument.into()),
    }
}

/// Normalizes the five fallocate combinations supported by BenchFS v1.
pub fn normalize_fallocate_mode(flags: u32) -> FsResult<FallocateMode> {
    const PUNCH_HOLE_KEEP_SIZE: u32 = FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE;
    const ZERO_RANGE_KEEP_SIZE: u32 = FALLOC_FL_ZERO_RANGE | FALLOC_FL_KEEP_SIZE;
    match flags {
        0 => Ok(FallocateMode::Allocate),
        FALLOC_FL_KEEP_SIZE => Ok(FallocateMode::AllocateKeepSize),
        PUNCH_HOLE_KEEP_SIZE => Ok(FallocateMode::PunchHole),
        FALLOC_FL_ZERO_RANGE => Ok(FallocateMode::ZeroRange),
        ZERO_RANGE_KEEP_SIZE => Ok(FallocateMode::ZeroRangeKeepSize),
        FALLOC_FL_COLLAPSE_RANGE | FALLOC_FL_INSERT_RANGE | FALLOC_FL_UNSHARE_RANGE => {
            Err(Errno::NotSupported.into())
        }
        _ => Err(Errno::InvalidArgument.into()),
    }
}

/// Normalizes Linux setxattr creation flags.
pub fn normalize_xattr_mode(flags: u32) -> FsResult<XattrSetMode> {
    match flags {
        0 => Ok(XattrSetMode::Upsert),
        XATTR_CREATE => Ok(XattrSetMode::Create),
        XATTR_REPLACE => Ok(XattrSetMode::Replace),
        _ => Err(Errno::InvalidArgument.into()),
    }
}

/// Normalizes privilege clearing for negotiated `HANDLE_KILLPRIV` v1.
///
/// Linux 6.8 only supplies the individual kill flags when killpriv v2 is
/// negotiated. Under v1 an ownership or size change is therefore the signal
/// to clear both privilege bits. An explicit mode change alone preserves the
/// requested mode, including any privilege bits it contains.
pub fn normalize_setattr_privilege_clear(flags: u32) -> FsResult<PrivilegeClearMask> {
    let explicit = if flags & SET_ATTR_KILL_PRIV != 0 {
        PrivilegeClearMask::BOTH.bits()
    } else {
        (if flags & SET_ATTR_KILL_SUID != 0 {
            PrivilegeClearMask::SET_UID.bits()
        } else {
            0
        }) | (if flags & SET_ATTR_KILL_SGID != 0 {
            PrivilegeClearMask::SET_GID.bits()
        } else {
            0
        })
    };
    let inferred = if explicit == 0 && flags & (SET_ATTR_UID | SET_ATTR_GID | SET_ATTR_SIZE) != 0 {
        PrivilegeClearMask::BOTH.bits()
    } else {
        explicit
    };
    PrivilegeClearMask::from_bits(inferred)
}
