/// Stable operation identifiers from the normative SPEC coverage matrix.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationId {
    /// Mount and recover.
    M01,
    /// Clean unmount.
    M02,
    /// Lookup.
    L01,
    /// Adapter-local forget.
    L02,
    /// Get attributes.
    A01,
    /// Create and open a regular file.
    N01,
    /// Create a regular or special inode.
    N02,
    /// Create a directory.
    N03,
    /// Create a symbolic link.
    N04,
    /// Create a hard link.
    N05,
    /// Unlink a non-directory.
    N06,
    /// Remove an empty directory.
    N07,
    /// Rename, no-replace, or exchange.
    N08,
    /// Read a symbolic link.
    N09,
    /// Open and release a regular file.
    H01,
    /// Open, read, and release a directory.
    H02,
    /// Change owner or mode.
    A02,
    /// Truncate a regular file.
    A03,
    /// Set inode timestamps.
    A04,
    /// Read regular-file data.
    D01,
    /// Write regular-file data.
    D02,
    /// Seek to data or a hole.
    D03,
    /// Allocate unwritten backing.
    D04,
    /// Punch or zero a range.
    D05,
    /// Adapter/filesystem mmap cache coherence.
    D06,
    /// Set an extended attribute.
    X01,
    /// Get or list extended attributes.
    X02,
    /// Remove an extended attribute.
    X03,
    /// Report filesystem capacity.
    S01,
    /// Flush close hint.
    S02,
    /// File durability fences and synchronous writes.
    S03,
    /// Directory durability fence.
    S04,
    /// COW reflink.
    C01,
    /// Create a snapshot.
    C02,
    /// List snapshots.
    C03,
    /// Mount or unmount a snapshot reference.
    C04,
    /// Delete a snapshot.
    C05,
    /// Explicit COW garbage collection.
    C06,
    /// Adapter/VFS locks.
    K01,
    /// Adapter/VFS path, access, and offset behavior.
    K02,
    /// Unsupported capability and flag policy.
    U01,
}

impl OperationId {
    /// All 41 normative operation identifiers in SPEC matrix order.
    pub const ALL: [Self; 41] = [
        Self::M01,
        Self::M02,
        Self::L01,
        Self::L02,
        Self::A01,
        Self::N01,
        Self::N02,
        Self::N03,
        Self::N04,
        Self::N05,
        Self::N06,
        Self::N07,
        Self::N08,
        Self::N09,
        Self::H01,
        Self::H02,
        Self::A02,
        Self::A03,
        Self::A04,
        Self::D01,
        Self::D02,
        Self::D03,
        Self::D04,
        Self::D05,
        Self::D06,
        Self::X01,
        Self::X02,
        Self::X03,
        Self::S01,
        Self::S02,
        Self::S03,
        Self::S04,
        Self::C01,
        Self::C02,
        Self::C03,
        Self::C04,
        Self::C05,
        Self::C06,
        Self::K01,
        Self::K02,
        Self::U01,
    ];

    /// Returns the exact SPEC identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M01 => "M01",
            Self::M02 => "M02",
            Self::L01 => "L01",
            Self::L02 => "L02",
            Self::A01 => "A01",
            Self::N01 => "N01",
            Self::N02 => "N02",
            Self::N03 => "N03",
            Self::N04 => "N04",
            Self::N05 => "N05",
            Self::N06 => "N06",
            Self::N07 => "N07",
            Self::N08 => "N08",
            Self::N09 => "N09",
            Self::H01 => "H01",
            Self::H02 => "H02",
            Self::A02 => "A02",
            Self::A03 => "A03",
            Self::A04 => "A04",
            Self::D01 => "D01",
            Self::D02 => "D02",
            Self::D03 => "D03",
            Self::D04 => "D04",
            Self::D05 => "D05",
            Self::D06 => "D06",
            Self::X01 => "X01",
            Self::X02 => "X02",
            Self::X03 => "X03",
            Self::S01 => "S01",
            Self::S02 => "S02",
            Self::S03 => "S03",
            Self::S04 => "S04",
            Self::C01 => "C01",
            Self::C02 => "C02",
            Self::C03 => "C03",
            Self::C04 => "C04",
            Self::C05 => "C05",
            Self::C06 => "C06",
            Self::K01 => "K01",
            Self::K02 => "K02",
            Self::U01 => "U01",
        }
    }
}

/// Variant/profile in which an operation is mandatory.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OperationProfile {
    /// Core, Journal, and the writable current COW view.
    AllWritable,
    /// COW only.
    CowOnly,
    /// Fixed adapter or Linux VFS only.
    AdapterVfs,
    /// Policy applies across adapter and all variants.
    All,
}

/// Component whose compiled API represents the operation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OperationSurface {
    /// [`crate::Filesystem`] lifecycle.
    FilesystemLifecycle,
    /// [`crate::FilesystemOperations`] inode API.
    FilesystemOperations,
    /// [`crate::CowFilesystem`] capability extension.
    CowFilesystem,
    /// Shared responsibility of adapter and filesystem operations.
    AdapterAndFilesystem,
    /// No persistent SDK method; implemented by the fixed adapter/VFS.
    AdapterVfs,
    /// Capability discovery and fixed unsupported-operation policy.
    CapabilityPolicy,
}

/// One normative SPEC-to-SDK traceability row.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OperationBinding {
    /// Stable SPEC identifier.
    pub id: OperationId,
    /// Required variant/profile.
    pub profile: OperationProfile,
    /// Compiled API surface carrying the contract.
    pub surface: OperationSurface,
    /// Rust trait methods, or an explicit adapter/policy marker.
    pub methods: &'static [&'static str],
}

const fn binding(
    id: OperationId,
    profile: OperationProfile,
    surface: OperationSurface,
    methods: &'static [&'static str],
) -> OperationBinding {
    OperationBinding {
        id,
        profile,
        surface,
        methods,
    }
}

/// Complete SPEC operation-ID to compiled SDK surface mapping.
pub const OPERATION_BINDINGS: [OperationBinding; 41] = [
    binding(
        OperationId::M01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemLifecycle,
        &["Filesystem::mount"],
    ),
    binding(
        OperationId::M02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemLifecycle,
        &["Filesystem::unmount"],
    ),
    binding(
        OperationId::L01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::lookup"],
    ),
    binding(
        OperationId::L02,
        OperationProfile::AdapterVfs,
        OperationSurface::AdapterVfs,
        &["adapter lookup-reference bookkeeping"],
    ),
    binding(
        OperationId::A01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::getattr"],
    ),
    binding(
        OperationId::N01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::create"],
    ),
    binding(
        OperationId::N02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::mknod"],
    ),
    binding(
        OperationId::N03,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::mkdir"],
    ),
    binding(
        OperationId::N04,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::symlink"],
    ),
    binding(
        OperationId::N05,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::link"],
    ),
    binding(
        OperationId::N06,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::unlink"],
    ),
    binding(
        OperationId::N07,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::rmdir"],
    ),
    binding(
        OperationId::N08,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::rename"],
    ),
    binding(
        OperationId::N09,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::readlink"],
    ),
    binding(
        OperationId::H01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &[
            "FilesystemOperations::open",
            "FilesystemOperations::release",
        ],
    ),
    binding(
        OperationId::H02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &[
            "FilesystemOperations::opendir",
            "FilesystemOperations::readdir",
            "FilesystemOperations::releasedir",
        ],
    ),
    binding(
        OperationId::A02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::setattr"],
    ),
    binding(
        OperationId::A03,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &[
            "FilesystemOperations::setattr",
            "FilesystemOperations::open",
        ],
    ),
    binding(
        OperationId::A04,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::setattr"],
    ),
    binding(
        OperationId::D01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::read"],
    ),
    binding(
        OperationId::D02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::write"],
    ),
    binding(
        OperationId::D03,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::seek"],
    ),
    binding(
        OperationId::D04,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::fallocate"],
    ),
    binding(
        OperationId::D05,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::fallocate"],
    ),
    binding(
        OperationId::D06,
        OperationProfile::AllWritable,
        OperationSurface::AdapterAndFilesystem,
        &[
            "adapter cache invalidation/writeback",
            "FilesystemOperations::write",
            "FilesystemOperations::fsync",
        ],
    ),
    binding(
        OperationId::X01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::setxattr"],
    ),
    binding(
        OperationId::X02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &[
            "FilesystemOperations::getxattr",
            "FilesystemOperations::listxattr",
        ],
    ),
    binding(
        OperationId::X03,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::removexattr"],
    ),
    binding(
        OperationId::S01,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::statfs"],
    ),
    binding(
        OperationId::S02,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::flush"],
    ),
    binding(
        OperationId::S03,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::fsync", "FilesystemOperations::write"],
    ),
    binding(
        OperationId::S04,
        OperationProfile::AllWritable,
        OperationSurface::FilesystemOperations,
        &["FilesystemOperations::fsyncdir"],
    ),
    binding(
        OperationId::C01,
        OperationProfile::CowOnly,
        OperationSurface::CowFilesystem,
        &["CowFilesystem::clone_range"],
    ),
    binding(
        OperationId::C02,
        OperationProfile::CowOnly,
        OperationSurface::CowFilesystem,
        &["CowFilesystem::snapshot_create"],
    ),
    binding(
        OperationId::C03,
        OperationProfile::CowOnly,
        OperationSurface::CowFilesystem,
        &["CowFilesystem::snapshot_list"],
    ),
    binding(
        OperationId::C04,
        OperationProfile::CowOnly,
        OperationSurface::CowFilesystem,
        &[
            "CowFilesystem::snapshot_mount",
            "CowFilesystem::snapshot_unmount",
        ],
    ),
    binding(
        OperationId::C05,
        OperationProfile::CowOnly,
        OperationSurface::CowFilesystem,
        &["CowFilesystem::snapshot_delete"],
    ),
    binding(
        OperationId::C06,
        OperationProfile::CowOnly,
        OperationSurface::CowFilesystem,
        &["CowFilesystem::gc"],
    ),
    binding(
        OperationId::K01,
        OperationProfile::AdapterVfs,
        OperationSurface::AdapterVfs,
        &["adapter/kernel-local lock manager"],
    ),
    binding(
        OperationId::K02,
        OperationProfile::AdapterVfs,
        OperationSurface::AdapterVfs,
        &["adapter/VFS path, access, fd-offset handling"],
    ),
    binding(
        OperationId::U01,
        OperationProfile::All,
        OperationSurface::CapabilityPolicy,
        &[
            "FilesystemOperations::capabilities",
            "fixed adapter error mapping",
        ],
    ),
];
