//!
//! These types are compile-time models of the policy hardcoded in the C
//! bridge (`native/benchfs_fuse.c`). They exist so the contract tests can
//! assert the expected values. The actual enforcement is in the C bridge;
//! `run_mount` does not consult these structs at runtime.

use benchfs_sdk::Errno;

/// Kernel mount options owned by the immutable adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MountPolicy {
    /// Delegate ordinary mode/uid/gid checks to the VFS.
    pub default_permissions: bool,
    /// Allow non-root callers to reach the daemon.
    pub allow_other: bool,
    /// Prevent device inodes from being opened as devices.
    pub nodev: bool,
    /// Disable access-time mutations from ordinary reads.
    pub noatime: bool,
}

impl MountPolicy {
    /// BenchFS v1 fixed mount policy.
    pub const V1: Self = Self {
        default_permissions: true,
        allow_other: true,
        nodev: true,
        noatime: true,
    };

    /// Mount-option fragment passed to libfuse.
    #[must_use]
    pub const fn option_string(self) -> &'static str {
        "default_permissions,allow_other,nodev,noatime"
    }
}

/// Page, attribute and dentry cache choices owned by the adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct CachePolicy {
    /// Kernel buffered cache is enabled for regular I/O and mmap coherence.
    pub buffered_io: bool,
    /// Kernel writeback cache is deliberately disabled.
    pub writeback_cache: bool,
    /// Direct I/O is deliberately disabled.
    pub direct_io: bool,
    /// Adapter emits explicit invalidations after out-of-band mutations.
    pub explicit_invalidation: bool,
}

impl CachePolicy {
    /// BenchFS v1 fixed cache policy.
    pub const V1: Self = Self {
        buffered_io: true,
        writeback_cache: false,
        direct_io: false,
        explicit_invalidation: true,
    };
}

/// Error/fallback policy for low-level requests outside the SDK surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedPolicy {
    /// Return `ENOSYS` so the kernel may install its documented fallback.
    KernelFallback,
    /// Return a stable errno without invoking an SDK method.
    Errno(Errno),
    /// Omit the callback so the VFS supplies kernel-local behavior.
    KernelLocal,
    /// No reply is legal for this bookkeeping notification.
    NoReply,
}
