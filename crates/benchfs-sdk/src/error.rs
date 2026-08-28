use core::fmt;

/// Linux errno values that can be observed through the BenchFS v1 contract.
///
/// The discriminants are the stable Linux UAPI values used by the fixed
/// x86-64 evaluation environment. They do not come from the build host libc.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum Errno {
    /// Operation not permitted.
    OperationNotPermitted = 1,
    /// No such file or directory.
    NoEntry = 2,
    /// Input/output error.
    Io = 5,
    /// No such device or address / no matching data or hole.
    NoDeviceOrAddress = 6,
    /// Bad file descriptor or invalid BenchFS handle.
    BadFileDescriptor = 9,
    /// Cannot allocate memory.
    OutOfMemory = 12,
    /// Permission denied.
    AccessDenied = 13,
    /// Bad address, produced by the syscall boundary rather than a filesystem.
    BadAddress = 14,
    /// Device or resource busy.
    Busy = 16,
    /// File exists.
    Exists = 17,
    /// Invalid cross-device operation.
    CrossDevice = 18,
    /// A path component or target is not a directory.
    NotDirectory = 20,
    /// Operation requires a non-directory.
    IsDirectory = 21,
    /// Invalid argument or flag combination.
    InvalidArgument = 22,
    /// Inappropriate ioctl for device.
    NotTty = 25,
    /// File too large for the BenchFS logical-size contract.
    FileTooLarge = 27,
    /// Illegal seek on this inode kind.
    IllegalSeek = 29,
    /// No space in the relevant filesystem resource.
    NoSpace = 28,
    /// Read-only filesystem or snapshot view.
    ReadOnly = 30,
    /// Link count would overflow.
    TooManyLinks = 31,
    /// Result or supplied buffer is out of range.
    Range = 34,
    /// Raw pathname component is too long.
    NameTooLong = 36,
    /// Directory is not empty.
    NotEmpty = 39,
    /// Too many symbolic links while resolving a pathname.
    TooManySymlinks = 40,
    /// Extended attribute does not exist.
    NoData = 61,
    /// A required numeric result cannot be represented.
    Overflow = 75,
    /// Operation or capability is not supported by this profile.
    NotSupported = 95,
    /// Node key names a stale inode lifetime.
    Stale = 116,
    /// Persistent structure or checksum is corrupt.
    Corrupt = 117,
    /// Quota exceeded; reserved for VFS use because BenchFS v1 has no quota.
    QuotaExceeded = 122,
}

impl Errno {
    /// Returns the Linux UAPI integer value.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Decodes an errno in the public BenchFS error vocabulary.
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::OperationNotPermitted),
            2 => Some(Self::NoEntry),
            5 => Some(Self::Io),
            6 => Some(Self::NoDeviceOrAddress),
            9 => Some(Self::BadFileDescriptor),
            12 => Some(Self::OutOfMemory),
            13 => Some(Self::AccessDenied),
            14 => Some(Self::BadAddress),
            16 => Some(Self::Busy),
            17 => Some(Self::Exists),
            18 => Some(Self::CrossDevice),
            20 => Some(Self::NotDirectory),
            21 => Some(Self::IsDirectory),
            22 => Some(Self::InvalidArgument),
            25 => Some(Self::NotTty),
            27 => Some(Self::FileTooLarge),
            28 => Some(Self::NoSpace),
            29 => Some(Self::IllegalSeek),
            30 => Some(Self::ReadOnly),
            31 => Some(Self::TooManyLinks),
            34 => Some(Self::Range),
            36 => Some(Self::NameTooLong),
            39 => Some(Self::NotEmpty),
            40 => Some(Self::TooManySymlinks),
            61 => Some(Self::NoData),
            75 => Some(Self::Overflow),
            95 => Some(Self::NotSupported),
            116 => Some(Self::Stale),
            117 => Some(Self::Corrupt),
            122 => Some(Self::QuotaExceeded),
            _ => None,
        }
    }

    /// Returns the conventional symbolic errno name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::OperationNotPermitted => "EPERM",
            Self::NoEntry => "ENOENT",
            Self::Io => "EIO",
            Self::NoDeviceOrAddress => "ENXIO",
            Self::BadFileDescriptor => "EBADF",
            Self::OutOfMemory => "ENOMEM",
            Self::AccessDenied => "EACCES",
            Self::BadAddress => "EFAULT",
            Self::Busy => "EBUSY",
            Self::Exists => "EEXIST",
            Self::CrossDevice => "EXDEV",
            Self::NotDirectory => "ENOTDIR",
            Self::IsDirectory => "EISDIR",
            Self::InvalidArgument => "EINVAL",
            Self::NotTty => "ENOTTY",
            Self::FileTooLarge => "EFBIG",
            Self::IllegalSeek => "ESPIPE",
            Self::NoSpace => "ENOSPC",
            Self::ReadOnly => "EROFS",
            Self::TooManyLinks => "EMLINK",
            Self::Range => "ERANGE",
            Self::NameTooLong => "ENAMETOOLONG",
            Self::NotEmpty => "ENOTEMPTY",
            Self::TooManySymlinks => "ELOOP",
            Self::NoData => "ENODATA",
            Self::Overflow => "EOVERFLOW",
            Self::NotSupported => "EOPNOTSUPP",
            Self::Stale => "ESTALE",
            Self::Corrupt => "EUCLEAN",
            Self::QuotaExceeded => "EDQUOT",
        }
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A filesystem failure containing exactly one Linux errno.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct FsError(Errno);

impl FsError {
    /// Creates an error from its exact externally visible errno.
    #[must_use]
    pub const fn new(errno: Errno) -> Self {
        Self(errno)
    }

    /// Returns the exact externally visible errno.
    #[must_use]
    pub const fn errno(self) -> Errno {
        self.0
    }

    /// Returns the Linux UAPI integer value.
    #[must_use]
    pub const fn raw_os_error(self) -> i32 {
        self.0.as_i32()
    }
}

impl From<Errno> for FsError {
    fn from(errno: Errno) -> Self {
        Self::new(errno)
    }
}

impl fmt::Display for FsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for FsError {}

/// Result returned by every fallible filesystem contract method.
pub type FsResult<T> = Result<T, FsError>;
