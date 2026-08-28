use core::num::NonZeroU64;

use crate::{
    Errno, FsResult, MAX_FILE_SIZE, MAX_NAME_LEN, MAX_SYMLINK_TARGET_LEN, MAX_XATTR_BYTES,
};

/// Identity of one allocated inode lifetime.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeKey {
    inode_id: NonZeroU64,
    generation: NonZeroU64,
}

impl NodeKey {
    /// Root inode identity in every valid BenchFS v1 image.
    pub const ROOT: Self = Self {
        inode_id: NonZeroU64::MIN,
        generation: NonZeroU64::MIN,
    };

    /// Creates a key, rejecting zero inode IDs or generations with `EINVAL`.
    pub fn new(inode_id: u64, generation: u64) -> FsResult<Self> {
        let inode_id = NonZeroU64::new(inode_id).ok_or(Errno::InvalidArgument)?;
        let generation = NonZeroU64::new(generation).ok_or(Errno::InvalidArgument)?;
        Ok(Self {
            inode_id,
            generation,
        })
    }

    /// Returns the stable inode number.
    #[must_use]
    pub const fn inode_id(self) -> u64 {
        self.inode_id.get()
    }

    /// Returns the inode lifetime generation.
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation.get()
    }
}

/// Raw, non-UTF-8 pathname component accepted by the inode API.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(Vec<u8>);

impl Name {
    /// Validates the v1 component rules and returns their exact contract errno.
    pub fn new(bytes: impl Into<Vec<u8>>) -> FsResult<Self> {
        let bytes = bytes.into();
        if bytes.len() > MAX_NAME_LEN {
            return Err(Errno::NameTooLong.into());
        }
        if bytes.is_empty()
            || bytes.contains(&0)
            || bytes.contains(&b'/')
            || bytes == b"."
            || bytes == b".."
        {
            return Err(Errno::InvalidArgument.into());
        }
        Ok(Self(bytes))
    }

    /// Returns the uninterpreted component bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes the component and returns its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

/// Raw symbolic-link target with the BenchFS v1 size restrictions applied.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SymlinkTarget(Vec<u8>);

impl SymlinkTarget {
    /// Validates a target, returning `ENOENT`, `ENAMETOOLONG`, or `EINVAL`.
    pub fn new(bytes: impl Into<Vec<u8>>) -> FsResult<Self> {
        let bytes = bytes.into();
        if bytes.is_empty() {
            return Err(Errno::NoEntry.into());
        }
        if bytes.len() > MAX_SYMLINK_TARGET_LEN {
            return Err(Errno::NameTooLong.into());
        }
        if bytes.contains(&0) {
            return Err(Errno::InvalidArgument.into());
        }
        Ok(Self(bytes))
    }

    /// Returns the uninterpreted target bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes the target and returns its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

/// Valid API-visible `user.*` extended-attribute name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XattrName(Vec<u8>);

impl XattrName {
    /// Validates namespace, framing, and length with their specified errno.
    pub fn new(bytes: impl Into<Vec<u8>>) -> FsResult<Self> {
        let bytes = bytes.into();
        if bytes.len() > MAX_NAME_LEN {
            return Err(Errno::Range.into());
        }
        if bytes.is_empty() || bytes.contains(&0) || bytes == b"user." {
            return Err(Errno::InvalidArgument.into());
        }
        if !bytes.starts_with(b"user.") {
            return Err(Errno::NotSupported.into());
        }
        Ok(Self(bytes))
    }

    /// Returns the raw attribute name.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// One complete binary extended-attribute value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XattrValue(Vec<u8>);

impl XattrValue {
    /// Creates a value, rejecting a value larger than 64 KiB with `ERANGE`.
    pub fn new(bytes: impl Into<Vec<u8>>) -> FsResult<Self> {
        let bytes = bytes.into();
        if bytes.len() > MAX_XATTR_BYTES {
            return Err(Errno::Range.into());
        }
        Ok(Self(bytes))
    }

    /// Returns all value bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes the value and returns all bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

/// Timestamp in the on-disk 16-byte timestamp domain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp {
    seconds: i64,
    nanoseconds: u32,
}

impl Timestamp {
    /// Creates a representable timestamp, rejecting invalid nanoseconds.
    pub fn new(seconds: i64, nanoseconds: u32) -> FsResult<Self> {
        if nanoseconds > 999_999_999 {
            return Err(Errno::InvalidArgument.into());
        }
        Ok(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Returns signed Unix seconds; negative values are valid.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }

    /// Returns nanoseconds in `0..=999_999_999`.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }
}

/// Single timestamp sampled by the adapter for one logical mutation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct OperationTime(Timestamp);

impl OperationTime {
    /// Creates an operation time with validated nanoseconds.
    pub fn new(seconds: i64, nanoseconds: u32) -> FsResult<Self> {
        Timestamp::new(seconds, nanoseconds).map(Self)
    }

    /// Returns the timestamp that all affected inodes must share.
    #[must_use]
    pub const fn timestamp(self) -> Timestamp {
        self.0
    }
}

impl From<OperationTime> for Timestamp {
    fn from(value: OperationTime) -> Self {
        value.timestamp()
    }
}

/// Nonnegative logical file offset bounded by `i64::MAX`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileOffset(u64);

impl FileOffset {
    /// Validates an offset, returning `EFBIG` above the v1 logical limit.
    pub fn new(offset: u64) -> FsResult<Self> {
        if offset > MAX_FILE_SIZE {
            return Err(Errno::FileTooLarge.into());
        }
        Ok(Self(offset))
    }

    /// Returns the byte offset.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Forms a checked range starting at this offset.
    pub fn checked_range(self, length: u64) -> FsResult<FileRange> {
        FileRange::new(self.0, length)
    }
}

/// Checked half-open logical byte range.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FileRange {
    offset: FileOffset,
    length: u64,
    end: u64,
}

impl FileRange {
    /// Validates both the start and exclusive end against `i64::MAX`.
    pub fn new(offset: u64, length: u64) -> FsResult<Self> {
        let offset = FileOffset::new(offset)?;
        let end = offset
            .get()
            .checked_add(length)
            .filter(|end| *end <= MAX_FILE_SIZE)
            .ok_or(Errno::FileTooLarge)?;
        Ok(Self {
            offset,
            length,
            end,
        })
    }

    /// First byte of the range.
    #[must_use]
    pub const fn offset(self) -> FileOffset {
        self.offset
    }

    /// Number of bytes in the range.
    #[must_use]
    pub const fn length(self) -> u64 {
        self.length
    }

    /// Exclusive end of the range.
    #[must_use]
    pub const fn end(self) -> u64 {
        self.end
    }

    /// Whether the range contains no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.length == 0
    }
}

/// Linux inode kind with frozen UAPI mode bits.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum FileKind {
    /// Named pipe.
    Fifo = 0o010_000,
    /// Character device inode.
    CharacterDevice = 0o020_000,
    /// Directory.
    Directory = 0o040_000,
    /// Block device inode.
    BlockDevice = 0o060_000,
    /// Regular file.
    Regular = 0o100_000,
    /// Symbolic link.
    Symlink = 0o120_000,
    /// Unix-domain socket inode.
    Socket = 0o140_000,
}

impl FileKind {
    /// Returns the frozen Linux `S_IF*` bits.
    #[must_use]
    pub const fn mode_bits(self) -> u32 {
        self as u32
    }

    /// Decodes the frozen Linux file-type bits from a complete mode.
    #[must_use]
    pub const fn from_mode(mode: u32) -> Option<Self> {
        match mode & crate::S_IFMT {
            crate::S_IFIFO => Some(Self::Fifo),
            crate::S_IFCHR => Some(Self::CharacterDevice),
            crate::S_IFDIR => Some(Self::Directory),
            crate::S_IFBLK => Some(Self::BlockDevice),
            crate::S_IFREG => Some(Self::Regular),
            crate::S_IFLNK => Some(Self::Symlink),
            crate::S_IFSOCK => Some(Self::Socket),
            _ => None,
        }
    }
}

/// Device major/minor pair stored for character and block device inodes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DeviceId {
    /// Linux device major number.
    pub major: u32,
    /// Linux device minor number.
    pub minor: u32,
}

/// Caller identity supplied by the fixed adapter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RequestContext {
    /// FUSE request user ID.
    pub uid: u32,
    /// FUSE request primary group ID.
    pub gid: u32,
    /// FUSE request process ID.
    pub pid: u32,
    /// Creation mask associated with the request.
    pub umask: u32,
}

/// Complete stable attributes returned by lookup/getattr/create operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileAttr {
    /// Allocated inode lifetime.
    pub node: NodeKey,
    /// Inode kind.
    pub kind: FileKind,
    /// Linux type and permission bits.
    pub mode: u32,
    /// Owner user ID.
    pub uid: u32,
    /// Owner group ID.
    pub gid: u32,
    /// Device identity; zero for non-device inodes.
    pub device: DeviceId,
    /// Exact persistent link count.
    pub link_count: u64,
    /// Logical file size, symlink bytes, or stored directory-entry count.
    pub size: u64,
    /// Allocated bytes, including unwritten extents.
    pub allocated_bytes: u64,
    /// Last access time.
    pub accessed_at: Timestamp,
    /// Last data modification time.
    pub modified_at: Timestamp,
    /// Last inode-status change time.
    pub changed_at: Timestamp,
}

/// Lookup or creation result tying kind and attributes to the same lifetime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    /// Allocated inode lifetime.
    pub node: NodeKey,
    /// Directory-entry kind.
    pub kind: FileKind,
    /// Current attributes for `node`.
    pub attributes: FileAttr,
}

/// Which timestamp action a normalized setattr request carries.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TimeSelector {
    /// Persist the supplied value.
    Explicit(Timestamp),
    /// Use this request's [`OperationTime`].
    Now,
    /// Preserve the existing value.
    Omit,
}

/// Adapter-decided privilege bits to clear atomically with a mutation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct PrivilegeClearMask(u32);

impl PrivilegeClearMask {
    /// Clear no privilege bits.
    pub const NONE: Self = Self(0);
    /// Clear `S_ISGID`.
    pub const SET_GID: Self = Self(0o002_000);
    /// Clear `S_ISUID`.
    pub const SET_UID: Self = Self(0o004_000);
    /// Clear both `S_ISUID` and `S_ISGID`.
    pub const BOTH: Self = Self(Self::SET_UID.0 | Self::SET_GID.0);

    /// Validates a raw mask supplied by an adapter.
    pub fn from_bits(bits: u32) -> FsResult<Self> {
        if bits & !Self::BOTH.0 != 0 {
            return Err(Errno::InvalidArgument.into());
        }
        Ok(Self(bits))
    }

    /// Returns Linux mode bits to clear.
    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Atomic, combined setattr payload.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SetAttr {
    /// Replacement permission/special bits, excluding inode type.
    pub mode: Option<u32>,
    /// Replacement owner user ID.
    pub uid: Option<u32>,
    /// Replacement owner group ID.
    pub gid: Option<u32>,
    /// Replacement logical size.
    pub size: Option<u64>,
    /// Optional access-time action.
    pub accessed_at: Option<TimeSelector>,
    /// Optional modification-time action.
    pub modified_at: Option<TimeSelector>,
    /// Privilege bits normalized by the adapter for this combined mutation.
    pub privilege_clear: PrivilegeClearMask,
    /// Writable handle required by ftruncate-like requests, when applicable.
    pub handle: Option<FileHandle>,
}

/// Opaque identity of one open file description.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileHandle(NonZeroU64);

impl FileHandle {
    /// Creates a handle identity; zero is rejected with `EINVAL`.
    pub fn new(value: u64) -> FsResult<Self> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| Errno::InvalidArgument.into())
    }

    /// Returns the opaque numeric identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// Opaque identity of one open directory description.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DirHandle(NonZeroU64);

impl DirHandle {
    /// Creates a handle identity; zero is rejected with `EINVAL`.
    pub fn new(value: u64) -> FsResult<Self> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| Errno::InvalidArgument.into())
    }

    /// Returns the opaque numeric identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// Read/write access mode already checked by the adapter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AccessMode {
    /// Read operations only.
    ReadOnly,
    /// Write operations only.
    WriteOnly,
    /// Both read and write operations.
    ReadWrite,
}

impl AccessMode {
    /// Whether the open description permits reads.
    #[must_use]
    pub const fn readable(self) -> bool {
        matches!(self, Self::ReadOnly | Self::ReadWrite)
    }

    /// Whether the open description permits writes.
    #[must_use]
    pub const fn writable(self) -> bool {
        matches!(self, Self::WriteOnly | Self::ReadWrite)
    }
}

/// Durability mode attached to an open description or write.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SyncMode {
    /// No implicit durability fence.
    #[default]
    None,
    /// `O_DSYNC` / fdatasync scope.
    Data,
    /// `O_SYNC` / fsync scope.
    Full,
}

/// Normalized mutation performed by `O_TRUNC` during open.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TruncateIntent {
    /// Time shared by the size/timestamp mutation.
    pub operation_time: OperationTime,
    /// Privilege bits to clear with truncation.
    pub privilege_clear: PrivilegeClearMask,
}

/// Adapter-normalized regular-file open options.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpenOptions {
    /// Read/write mode.
    pub access: AccessMode,
    /// Whether writes atomically select the current EOF.
    pub append: bool,
    /// Implicit durability fence for successful writes.
    pub sync: SyncMode,
    /// Atomic open-time truncate, if requested.
    pub truncate: Option<TruncateIntent>,
}

/// Adapter-normalized write behavior.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct WriteOptions {
    /// Whether the filesystem must atomically choose current EOF.
    pub append: bool,
    /// Required implicit durability fence.
    pub sync: SyncMode,
    /// Privilege bits to clear with the successful prefix.
    pub privilege_clear: PrivilegeClearMask,
}

/// Atomic create-and-open result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateResult {
    /// Newly created regular-file entry.
    pub entry: Entry,
    /// Open description reserved before the namespace mutation publishes.
    pub handle: FileHandle,
}

/// Byte count committed by one write request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WriteResult {
    /// Length of the nonzero contiguous committed prefix.
    pub bytes_written: u64,
}

/// Rename behavior after the adapter validates Linux flag combinations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RenameMode {
    /// Replace a compatible destination if it exists.
    Replace,
    /// Fail with `EEXIST` if the destination exists.
    NoReplace,
    /// Atomically exchange two existing bindings.
    Exchange,
}

/// Range query supported by the regular-file extent map.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SeekKind {
    /// `SEEK_DATA`.
    Data,
    /// `SEEK_HOLE`.
    Hole,
}

/// Supported normalized fallocate operations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FallocateMode {
    /// Allocate unwritten backing and extend size as needed.
    Allocate,
    /// Allocate unwritten backing without changing size.
    AllocateKeepSize,
    /// Punch a hole without changing size.
    PunchHole,
    /// Zero the range and extend size as needed.
    ZeroRange,
    /// Zero the range without changing size.
    ZeroRangeKeepSize,
}

/// Start or resume cookie supplied to readdir.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct DirectoryCookie(u64);

impl DirectoryCookie {
    /// Starts a new directory continuation segment.
    pub const START: Self = Self(0);

    /// Wraps a cookie previously returned by this handle.
    #[must_use]
    pub const fn from_returned(value: NonZeroU64) -> Self {
        Self(value.get())
    }

    /// Returns the opaque adapter-facing value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Nonzero continuation cookie emitted after one complete directory entry.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct NextCookie(NonZeroU64);

impl NextCookie {
    /// Creates a generated cookie; zero is rejected with `EINVAL`.
    pub fn new(value: u64) -> FsResult<Self> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| Errno::InvalidArgument.into())
    }

    /// Returns the opaque adapter-facing value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// One complete directory entry returned by the filesystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirEntry {
    /// Raw entry name; `.` and `..` are synthesized and therefore represented
    /// as bytes rather than [`Name`].
    pub name: Vec<u8>,
    /// Target inode lifetime.
    pub node: NodeKey,
    /// Target kind stored in the directory item.
    pub kind: FileKind,
    /// Position immediately after this entry.
    pub next_cookie: NextCookie,
}

/// Complete-entry prefix returned by one readdir call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirBatch {
    /// Entries that fit the requested adapter budget.
    pub entries: Vec<DirEntry>,
    /// Position to use for the next call, unchanged when no entry fit.
    pub continuation: DirectoryCookie,
    /// Whether the handle was at end-of-directory at the linearization point.
    pub end_of_directory: bool,
}

/// Xattr create/replace behavior after flag normalization.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum XattrSetMode {
    /// Create or replace.
    Upsert,
    /// Require absence.
    Create,
    /// Require an existing value.
    Replace,
}

/// One coherent allocator snapshot returned by statfs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StatFs {
    /// Total filesystem blocks.
    pub blocks: u64,
    /// Currently free blocks.
    pub blocks_free: u64,
    /// Blocks available to ordinary foreground allocation.
    pub blocks_available: u64,
    /// Total inode slots or COW inode-ID contract count.
    pub files: u64,
    /// Free reusable inode slots or COW inode-ID contract count.
    pub files_free: u64,
    /// Preferred I/O block size; exactly 4096 in v1.
    pub block_size: u32,
    /// Fundamental allocation size; exactly 4096 in v1.
    pub fragment_size: u32,
    /// Maximum pathname-component length; exactly 255 in v1.
    pub name_max: u32,
    /// View-specific mount flags.
    pub flags: MountFlags,
}

/// Mount flags reported through statfs.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct MountFlags {
    /// Whether mutations are rejected with `EROFS`.
    pub read_only: bool,
}

/// On-disk update architecture requested at mount.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Variant {
    /// In-place home updates without cross-block crash atomicity.
    Core = 1,
    /// Redo metadata journal with ordered data.
    Journal = 2,
    /// Immutable rooted generations with snapshots and reflink.
    Cow = 3,
}

/// Validated mount request supplied with an exclusive device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MountOptions {
    /// Variant the selected superblock must contain.
    pub variant: Variant,
    /// Whether the exposed current view is forced read-only.
    pub read_only: bool,
}

/// Discoverable optional operation capabilities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Capabilities {
    /// Fixed-control whole-file/range reflink support.
    pub reflink: bool,
    /// Snapshot create/list/mount/delete support.
    pub snapshots: bool,
    /// Explicit crash-safe garbage collection support.
    pub garbage_collection: bool,
}

impl Capabilities {
    /// Mandatory Core/Journal capability set.
    pub const BASE: Self = Self {
        reflink: false,
        snapshots: false,
        garbage_collection: false,
    };

    /// Mandatory COW capability set.
    pub const COW: Self = Self {
        reflink: true,
        snapshots: true,
        garbage_collection: true,
    };
}

/// Result of mounting a newly constructed filesystem implementation.
#[derive(Debug)]
pub struct Mounted<F> {
    /// Mounted implementation instance that owns the device capability.
    pub filesystem: F,
    /// Validated root entry, always `NodeKey::ROOT` for v1.
    pub root: Entry,
}

/// Whether a file durability fence has full or data-only scope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileSyncMode {
    /// fdatasync scope.
    Data,
    /// fsync scope.
    Full,
}

/// Checked offsets and nominal length for a range reflink.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CloneRange {
    source_offset: FileOffset,
    length: u64,
    destination_offset: FileOffset,
}

impl CloneRange {
    /// Checks both range ends against `i64::MAX` when length is nonzero.
    ///
    /// A zero length retains the Linux meaning "through source EOF"; mount
    /// state is needed to validate that effective range and partial-block rules.
    pub fn new(source_offset: u64, length: u64, destination_offset: u64) -> FsResult<Self> {
        let source_offset = FileOffset::new(source_offset)?;
        let destination_offset = FileOffset::new(destination_offset)?;
        if source_offset.get() % crate::BLOCK_SIZE != 0
            || destination_offset.get() % crate::BLOCK_SIZE != 0
        {
            return Err(Errno::InvalidArgument.into());
        }
        if length != 0 {
            source_offset.checked_range(length)?;
            destination_offset.checked_range(length)?;
        }
        Ok(Self {
            source_offset,
            length,
            destination_offset,
        })
    }

    /// Returns the source start.
    #[must_use]
    pub const fn source_offset(self) -> FileOffset {
        self.source_offset
    }

    /// Returns requested bytes, or zero for "through source EOF".
    #[must_use]
    pub const fn length(self) -> u64 {
        self.length
    }

    /// Returns the destination start.
    #[must_use]
    pub const fn destination_offset(self) -> FileOffset {
        self.destination_offset
    }
}

/// COW clone semantics that preserve whole-file/range distinction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CloneExtent {
    /// Replace the destination as an exact whole-file clone.
    WholeFile,
    /// Replace one normalized range; a zero length means through source EOF.
    Range(CloneRange),
}

/// Atomic COW clone request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CloneRequest {
    /// Readable source open description.
    pub source: FileHandle,
    /// Writable, non-append destination open description.
    pub destination: FileHandle,
    /// Whole-file or range semantics.
    pub extent: CloneExtent,
    /// Destination privilege bits to clear if its data changes.
    pub privilege_clear: PrivilegeClearMask,
}

/// Never-reused COW snapshot identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SnapshotId(NonZeroU64);

impl SnapshotId {
    /// Creates an ID, rejecting zero with `EINVAL`.
    pub fn new(value: u64) -> FsResult<Self> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| Errno::InvalidArgument.into())
    }

    /// Returns the persistent numeric ID.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// Persistent snapshot catalog item exposed by the control API.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotInfo {
    /// Never-reused catalog ID.
    pub id: SnapshotId,
    /// Raw catalog name.
    pub name: Name,
    /// Captured COW generation.
    pub generation: u64,
    /// Creation time supplied by the adapter.
    pub created_at: Timestamp,
}

/// Snapshot lookup key accepted by mount and delete operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnapshotSelector {
    /// Select by persistent ID.
    Id(SnapshotId),
    /// Select by exact raw catalog name.
    Name(Name),
}

/// A mounted, independently handled read-only snapshot view.
#[derive(Debug)]
pub struct MountedSnapshot<V> {
    /// Snapshot catalog item captured by the view.
    pub info: SnapshotInfo,
    /// Root entry from the captured inode root.
    pub root: Entry,
    /// Implementation-defined view object used for normal inode operations.
    pub view: V,
}
