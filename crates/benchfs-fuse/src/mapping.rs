//! Exhaustive libfuse 3.18.2 low-level callback mapping.

use crate::UnsupportedPolicy;
use benchfs_sdk::Errno;

/// Low-level operation slots present in libfuse 3.18.2.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LowLevelOperation {
    /// Session initialization.
    Init,
    /// Session destruction.
    Destroy,
    /// Name lookup.
    Lookup,
    /// Lookup-reference release.
    Forget,
    /// Attribute read.
    Getattr,
    /// Combined attribute mutation.
    Setattr,
    /// Symbolic-link read.
    Readlink,
    /// Special inode creation.
    Mknod,
    /// Directory creation.
    Mkdir,
    /// Unlink.
    Unlink,
    /// Directory removal.
    Rmdir,
    /// Symbolic-link creation.
    Symlink,
    /// Rename.
    Rename,
    /// Hard link.
    Link,
    /// File open.
    Open,
    /// File read.
    Read,
    /// File write.
    Write,
    /// Close-time error report.
    Flush,
    /// File-handle release.
    Release,
    /// File durability fence.
    Fsync,
    /// Directory open.
    Opendir,
    /// Directory iteration.
    Readdir,
    /// Directory-handle release.
    Releasedir,
    /// Directory durability fence.
    Fsyncdir,
    /// Filesystem statistics.
    Statfs,
    /// Extended-attribute set.
    Setxattr,
    /// Extended-attribute get.
    Getxattr,
    /// Extended-attribute list.
    Listxattr,
    /// Extended-attribute remove.
    Removexattr,
    /// VFS permission callback.
    Access,
    /// Atomic create/open.
    Create,
    /// POSIX lock query.
    Getlk,
    /// POSIX lock update.
    Setlk,
    /// Block mapping query.
    Bmap,
    /// Generic ioctl.
    Ioctl,
    /// Poll registration.
    Poll,
    /// Scatter/gather write.
    WriteBuf,
    /// Notification retrieval reply.
    RetrieveReply,
    /// Batched lookup-reference release.
    ForgetMulti,
    /// BSD flock.
    Flock,
    /// Range allocation/mutation.
    Fallocate,
    /// Plus-mode directory iteration.
    Readdirplus,
    /// Server-side range copy.
    CopyFileRange,
    /// Sparse data/hole seek.
    Lseek,
    /// Anonymous temporary file creation.
    Tmpfile,
    /// Extended stat.
    Statx,
}

impl LowLevelOperation {
    /// Every libfuse 3.18.2 operation slot, in header order.
    pub const ALL: &'static [Self] = &[
        Self::Init,
        Self::Destroy,
        Self::Lookup,
        Self::Forget,
        Self::Getattr,
        Self::Setattr,
        Self::Readlink,
        Self::Mknod,
        Self::Mkdir,
        Self::Unlink,
        Self::Rmdir,
        Self::Symlink,
        Self::Rename,
        Self::Link,
        Self::Open,
        Self::Read,
        Self::Write,
        Self::Flush,
        Self::Release,
        Self::Fsync,
        Self::Opendir,
        Self::Readdir,
        Self::Releasedir,
        Self::Fsyncdir,
        Self::Statfs,
        Self::Setxattr,
        Self::Getxattr,
        Self::Listxattr,
        Self::Removexattr,
        Self::Access,
        Self::Create,
        Self::Getlk,
        Self::Setlk,
        Self::Bmap,
        Self::Ioctl,
        Self::Poll,
        Self::WriteBuf,
        Self::RetrieveReply,
        Self::ForgetMulti,
        Self::Flock,
        Self::Fallocate,
        Self::Readdirplus,
        Self::CopyFileRange,
        Self::Lseek,
        Self::Tmpfile,
        Self::Statx,
    ];
}

/// Destination for a low-level callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MappingTarget {
    /// Exact public SDK method name.
    Sdk(&'static str),
    /// Fixed adapter or VFS behavior.
    Fixed(UnsupportedPolicy),
    /// Session/mount glue rather than an inode operation.
    Session,
}

/// One immutable mapping-table row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LowLevelMapping {
    /// libfuse callback slot.
    pub operation: LowLevelOperation,
    /// Its one allowed destination.
    pub target: MappingTarget,
}

const fn sdk(operation: LowLevelOperation, method: &'static str) -> LowLevelMapping {
    LowLevelMapping {
        operation,
        target: MappingTarget::Sdk(method),
    }
}

const fn fixed(operation: LowLevelOperation, policy: UnsupportedPolicy) -> LowLevelMapping {
    LowLevelMapping {
        operation,
        target: MappingTarget::Fixed(policy),
    }
}

/// Exhaustive libfuse 3.18.2 operation map. No callback is left ambiguous.
pub const LOW_LEVEL_MAPPINGS: &[LowLevelMapping] = &[
    LowLevelMapping {
        operation: LowLevelOperation::Init,
        target: MappingTarget::Session,
    },
    LowLevelMapping {
        operation: LowLevelOperation::Destroy,
        target: MappingTarget::Session,
    },
    sdk(LowLevelOperation::Lookup, "FilesystemOperations::lookup"),
    fixed(LowLevelOperation::Forget, UnsupportedPolicy::NoReply),
    sdk(LowLevelOperation::Getattr, "FilesystemOperations::getattr"),
    sdk(LowLevelOperation::Setattr, "FilesystemOperations::setattr"),
    sdk(
        LowLevelOperation::Readlink,
        "FilesystemOperations::readlink",
    ),
    sdk(LowLevelOperation::Mknod, "FilesystemOperations::mknod"),
    sdk(LowLevelOperation::Mkdir, "FilesystemOperations::mkdir"),
    sdk(LowLevelOperation::Unlink, "FilesystemOperations::unlink"),
    sdk(LowLevelOperation::Rmdir, "FilesystemOperations::rmdir"),
    sdk(LowLevelOperation::Symlink, "FilesystemOperations::symlink"),
    sdk(LowLevelOperation::Rename, "FilesystemOperations::rename"),
    sdk(LowLevelOperation::Link, "FilesystemOperations::link"),
    sdk(LowLevelOperation::Open, "FilesystemOperations::open"),
    sdk(LowLevelOperation::Read, "FilesystemOperations::read"),
    sdk(LowLevelOperation::Write, "FilesystemOperations::write"),
    sdk(LowLevelOperation::Flush, "FilesystemOperations::flush"),
    sdk(LowLevelOperation::Release, "FilesystemOperations::release"),
    sdk(LowLevelOperation::Fsync, "FilesystemOperations::fsync"),
    sdk(LowLevelOperation::Opendir, "FilesystemOperations::opendir"),
    sdk(LowLevelOperation::Readdir, "FilesystemOperations::readdir"),
    sdk(
        LowLevelOperation::Releasedir,
        "FilesystemOperations::releasedir",
    ),
    sdk(
        LowLevelOperation::Fsyncdir,
        "FilesystemOperations::fsyncdir",
    ),
    sdk(LowLevelOperation::Statfs, "FilesystemOperations::statfs"),
    sdk(
        LowLevelOperation::Setxattr,
        "FilesystemOperations::setxattr",
    ),
    sdk(
        LowLevelOperation::Getxattr,
        "FilesystemOperations::getxattr",
    ),
    sdk(
        LowLevelOperation::Listxattr,
        "FilesystemOperations::listxattr",
    ),
    sdk(
        LowLevelOperation::Removexattr,
        "FilesystemOperations::removexattr",
    ),
    fixed(LowLevelOperation::Access, UnsupportedPolicy::KernelLocal),
    sdk(LowLevelOperation::Create, "FilesystemOperations::create"),
    fixed(LowLevelOperation::Getlk, UnsupportedPolicy::KernelLocal),
    fixed(LowLevelOperation::Setlk, UnsupportedPolicy::KernelLocal),
    fixed(LowLevelOperation::Bmap, UnsupportedPolicy::KernelFallback),
    fixed(
        LowLevelOperation::Ioctl,
        UnsupportedPolicy::Errno(Errno::NotTty),
    ),
    fixed(LowLevelOperation::Poll, UnsupportedPolicy::KernelFallback),
    fixed(
        LowLevelOperation::WriteBuf,
        UnsupportedPolicy::KernelFallback,
    ),
    fixed(
        LowLevelOperation::RetrieveReply,
        UnsupportedPolicy::KernelFallback,
    ),
    fixed(LowLevelOperation::ForgetMulti, UnsupportedPolicy::NoReply),
    fixed(LowLevelOperation::Flock, UnsupportedPolicy::KernelLocal),
    sdk(
        LowLevelOperation::Fallocate,
        "FilesystemOperations::fallocate",
    ),
    fixed(
        LowLevelOperation::Readdirplus,
        UnsupportedPolicy::KernelFallback,
    ),
    fixed(
        LowLevelOperation::CopyFileRange,
        UnsupportedPolicy::KernelFallback,
    ),
    sdk(LowLevelOperation::Lseek, "FilesystemOperations::seek"),
    fixed(
        LowLevelOperation::Tmpfile,
        UnsupportedPolicy::Errno(Errno::NotSupported),
    ),
    fixed(LowLevelOperation::Statx, UnsupportedPolicy::KernelFallback),
];
