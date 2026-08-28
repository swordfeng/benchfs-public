use async_trait::async_trait;

use crate::{
    BlockDevice, Capabilities, CloneRequest, CreateResult, DeviceId, DirBatch, DirHandle,
    DirectoryCookie, Entry, FallocateMode, FileAttr, FileHandle, FileKind, FileOffset, FileRange,
    FileSyncMode, FsResult, MountOptions, Mounted, MountedSnapshot, Name, NodeKey, OpenOptions,
    OperationTime, RenameMode, RequestContext, SeekKind, SetAttr, SnapshotInfo, SnapshotSelector,
    StatFs, SymlinkTarget, Variant, WriteOptions, WriteResult, XattrName, XattrSetMode, XattrValue,
};

/// Concurrent inode-based operations shared by current and snapshot views.
///
/// Implementations may serialize internally, but every method taking `&self`
/// can be called concurrently. Each call has one live linearization point and
/// follows the error, cache-visibility, and durability rules in
/// `agent-docs/SPEC.md`.
#[allow(clippy::too_many_arguments)]
#[async_trait(?Send)]
pub trait FilesystemOperations: Send + Sync + 'static {
    /// Reports the mounted on-disk update architecture.
    fn variant(&self) -> Variant;

    /// Reports optional operation families exposed by this implementation.
    fn capabilities(&self) -> Capabilities;

    /// Looks up one raw name below an allocated directory (`L01`).
    async fn lookup(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
    ) -> FsResult<Entry>;

    /// Returns complete attributes for a node or its still-live handle (`A01`).
    async fn getattr(
        &self,
        context: &RequestContext,
        node: NodeKey,
        handle: Option<FileHandle>,
    ) -> FsResult<FileAttr>;

    /// Applies all present normalized fields as one metadata mutation
    /// (`A02`, `A03`, and `A04`).
    async fn setattr(
        &self,
        context: &RequestContext,
        node: NodeKey,
        attributes: &SetAttr,
        operation_time: OperationTime,
    ) -> FsResult<FileAttr>;

    /// Atomically creates an absent regular file and reserves its handle
    /// (`N01`).
    async fn create(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
        mode: u32,
        options: OpenOptions,
        operation_time: OperationTime,
    ) -> FsResult<CreateResult>;

    /// Creates an absent regular or special inode without opening it (`N02`).
    async fn mknod(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
        kind: FileKind,
        mode: u32,
        device: DeviceId,
        operation_time: OperationTime,
    ) -> FsResult<Entry>;

    /// Creates an empty directory and updates both link counts (`N03`).
    async fn mkdir(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
        mode: u32,
        operation_time: OperationTime,
    ) -> FsResult<Entry>;

    /// Creates a symbolic-link inode containing the complete raw target
    /// (`N04`).
    async fn symlink(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
        target: &SymlinkTarget,
        operation_time: OperationTime,
    ) -> FsResult<Entry>;

    /// Creates a new hard-link binding to a non-directory inode (`N05`).
    async fn link(
        &self,
        context: &RequestContext,
        source: NodeKey,
        new_parent: NodeKey,
        new_name: &Name,
        operation_time: OperationTime,
    ) -> FsResult<Entry>;

    /// Removes one non-directory binding and handles orphaning/finalization
    /// (`N06`).
    async fn unlink(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Removes one empty directory binding (`N07`).
    async fn rmdir(
        &self,
        context: &RequestContext,
        parent: NodeKey,
        name: &Name,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Performs replace, no-replace, or exchange as one namespace mutation
    /// (`N08`).
    async fn rename(
        &self,
        context: &RequestContext,
        old_parent: NodeKey,
        old_name: &Name,
        new_parent: NodeKey,
        new_name: &Name,
        mode: RenameMode,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Returns the complete raw symbolic-link target without a trailing NUL
    /// (`N09`).
    async fn readlink(&self, context: &RequestContext, node: NodeKey) -> FsResult<Vec<u8>>;

    /// Opens a regular inode, atomically applying any truncate intent (`H01`).
    async fn open(
        &self,
        context: &RequestContext,
        node: NodeKey,
        options: OpenOptions,
    ) -> FsResult<FileHandle>;

    /// Releases one open file description and finalizes its orphan if needed
    /// (`H01`).
    async fn release(&self, handle: FileHandle) -> FsResult<()>;

    /// Opens a directory and creates handle-local cookie state (`H02`).
    async fn opendir(&self, context: &RequestContext, node: NodeKey) -> FsResult<DirHandle>;

    /// Returns a complete-entry directory prefix and continuation (`H02`).
    async fn readdir(
        &self,
        context: &RequestContext,
        handle: DirHandle,
        cookie: DirectoryCookie,
        max_entries: u32,
    ) -> FsResult<DirBatch>;

    /// Releases a directory description and all of its cookies (`H02`).
    async fn releasedir(&self, handle: DirHandle) -> FsResult<()>;

    /// Reads at most `length` regular-file bytes from a validated offset
    /// (`D01`).
    async fn read(
        &self,
        context: &RequestContext,
        handle: FileHandle,
        offset: FileOffset,
        length: u64,
    ) -> FsResult<Vec<u8>>;

    /// Atomically commits one complete or nonzero contiguous write prefix
    /// (`D02`).
    async fn write(
        &self,
        context: &RequestContext,
        handle: FileHandle,
        offset: FileOffset,
        data: &[u8],
        options: WriteOptions,
        operation_time: OperationTime,
    ) -> FsResult<WriteResult>;

    /// Locates the next written-data or hole position (`D03`).
    async fn seek(
        &self,
        context: &RequestContext,
        node: NodeKey,
        offset: FileOffset,
        kind: SeekKind,
    ) -> FsResult<FileOffset>;

    /// Applies one supported allocation, punch, or zero range (`D04`, `D05`).
    ///
    /// The adapter guarantees that `range` is checked and nonempty. Implementations
    /// do not repeat syscall signedness or zero-length validation.
    async fn fallocate(
        &self,
        context: &RequestContext,
        handle: FileHandle,
        mode: FallocateMode,
        range: FileRange,
        privilege_clear: crate::PrivilegeClearMask,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Creates or replaces one complete API-visible xattr (`X01`).
    async fn setxattr(
        &self,
        context: &RequestContext,
        node: NodeKey,
        name: &XattrName,
        value: &XattrValue,
        mode: XattrSetMode,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Returns one complete API-visible xattr (`X02`).
    async fn getxattr(
        &self,
        context: &RequestContext,
        node: NodeKey,
        name: &XattrName,
    ) -> FsResult<XattrValue>;

    /// Returns all API-visible xattr names as a set-equivalent vector (`X02`).
    async fn listxattr(&self, context: &RequestContext, node: NodeKey) -> FsResult<Vec<XattrName>>;

    /// Removes one complete API-visible xattr (`X03`).
    async fn removexattr(
        &self,
        context: &RequestContext,
        node: NodeKey,
        name: &XattrName,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Returns one coherent allocator and inode-capacity snapshot (`S01`).
    async fn statfs(&self) -> FsResult<StatFs>;

    /// Reports a handle-attributable asynchronous writeback error without
    /// adding a durability fence (`S02`).
    async fn flush(&self, handle: FileHandle) -> FsResult<()>;

    /// Applies file fsync or fdatasync durability scope (`S03`).
    async fn fsync(&self, handle: FileHandle, mode: FileSyncMode) -> FsResult<()>;

    /// Applies directory namespace durability scope (`S04`).
    async fn fsyncdir(&self, handle: DirHandle) -> FsResult<()>;

    /// Releases per-inode resources after all kernel lookup references are
    /// forgotten. The adapter forwards FUSE FORGET once the cached lookup
    /// refcount reaches zero. The default is a no-op; implementations that
    /// cache per-inode state (e.g. open file descriptors) override this to
    /// reclaim them.
    async fn forget(&self, inode: u64) {
        let _ = inode;
    }
}

/// Construction and clean-unmount lifecycle for a writable/current filesystem.
#[async_trait(?Send)]
pub trait Filesystem: FilesystemOperations + Sized {
    /// Validates, recovers, and mounts an exclusive device (`M01`).
    async fn mount(device: Box<dyn BlockDevice>, options: MountOptions) -> FsResult<Mounted<Self>>;

    /// Drains pending state and publishes a clean unmount (`M02`).
    async fn unmount(self) -> FsResult<()>;
}

/// Marker declaring that an implementation can mount the in-place Core
/// variant and provide the complete base operation set.
///
/// A single implementation type may support multiple variants; `variant()` and
/// `capabilities()` describe the particular mounted instance.
pub trait CoreFilesystem: Filesystem {}

/// Marker declaring that an implementation can mount the redo-journal Journal
/// variant and provide the complete base operation set.
///
/// A single implementation type may also implement the Core and COW contracts.
pub trait JournalFilesystem: Filesystem {}

/// COW-only reflink, snapshot, and garbage-collection capability interface.
#[async_trait(?Send)]
pub trait CowFilesystem: Filesystem {
    /// Independently handled read-only view over captured snapshot roots.
    type SnapshotView: FilesystemOperations;

    /// Atomically applies whole-file or range reflink semantics (`C01`).
    async fn clone_range(
        &self,
        context: &RequestContext,
        request: CloneRequest,
        operation_time: OperationTime,
    ) -> FsResult<()>;

    /// Creates and durably publishes one snapshot catalog item (`C02`).
    async fn snapshot_create(
        &self,
        context: &RequestContext,
        name: &Name,
        operation_time: OperationTime,
    ) -> FsResult<SnapshotInfo>;

    /// Lists active snapshots in strictly increasing ID order (`C03`).
    async fn snapshot_list(&self, context: &RequestContext) -> FsResult<Vec<SnapshotInfo>>;

    /// Acquires an independently handled read-only snapshot view (`C04`).
    ///
    /// Implementations must register one in-memory mount reference per
    /// successful call. While that reference is live,
    /// [`Self::snapshot_delete`] must return `EBUSY`; the reference is not a
    /// persistent GC root, so crash recovery drops it without cleanup.
    async fn snapshot_mount(
        &self,
        context: &RequestContext,
        selector: &SnapshotSelector,
    ) -> FsResult<MountedSnapshot<Self::SnapshotView>>;

    /// Consumes a mounted snapshot view and releases its ephemeral GC root
    /// reference (`C04`).
    ///
    /// The release makes the snapshot eligible for
    /// [`Self::snapshot_delete`] again; it must not modify the catalog or
    /// force a checkpoint by itself.
    async fn snapshot_unmount(&self, snapshot: MountedSnapshot<Self::SnapshotView>)
    -> FsResult<()>;

    /// Durably removes one inactive snapshot catalog item (`C05`).
    ///
    /// While the selected snapshot has at least one active read-only mount
    /// reference, returns `EBUSY` without modifying the catalog. Deletion and
    /// mounting must serialize against each other so exactly one of a
    /// concurrent delete/mount pair wins.
    async fn snapshot_delete(
        &self,
        context: &RequestContext,
        selector: &SnapshotSelector,
    ) -> FsResult<()>;

    /// Runs and durably publishes crash-safe full garbage collection (`C06`).
    async fn gc(&self, context: &RequestContext) -> FsResult<()>;
}
