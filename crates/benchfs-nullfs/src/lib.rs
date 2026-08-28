//! Intentionally non-functional starting point for BenchFS implementations.
//!
//! Every filesystem and lifecycle operation returns `EOPNOTSUPP`. This crate
//! contains no inode table, namespace, allocator, block format, or persistence
//! behavior that could seed a contestant implementation.

use async_trait::async_trait;

use benchfs_sdk::{
    BlockDevice, Capabilities, CreateResult, DeviceId, DirBatch, DirHandle, DirectoryCookie, Entry,
    Errno, FallocateMode, FileAttr, FileHandle, FileOffset, FileRange, FileSyncMode, Filesystem,
    FilesystemOperations, FsResult, MountOptions, Mounted, Name, NodeKey, OpenOptions,
    OperationTime, RenameMode, RequestContext, SeekKind, SetAttr, StatFs, SymlinkTarget, Variant,
    WriteOptions, WriteResult, XattrName, XattrSetMode, XattrValue,
};

/// Stateless, deliberately unimplemented BenchFS scaffold.
#[derive(Clone, Copy, Debug, Default)]
pub struct NullFs;

fn unsupported<T>() -> FsResult<T> {
    Err(Errno::NotSupported.into())
}

#[async_trait(?Send)]
impl FilesystemOperations for NullFs {
    fn variant(&self) -> Variant {
        Variant::Core
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities::BASE
    }

    async fn lookup(&self, _: &RequestContext, _: NodeKey, _: &Name) -> FsResult<Entry> {
        unsupported()
    }

    async fn getattr(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: Option<FileHandle>,
    ) -> FsResult<FileAttr> {
        unsupported()
    }

    async fn setattr(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &SetAttr,
        _: OperationTime,
    ) -> FsResult<FileAttr> {
        unsupported()
    }

    async fn create(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: u32,
        _: OpenOptions,
        _: OperationTime,
    ) -> FsResult<CreateResult> {
        unsupported()
    }

    async fn mknod(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: benchfs_sdk::FileKind,
        _: u32,
        _: DeviceId,
        _: OperationTime,
    ) -> FsResult<Entry> {
        unsupported()
    }

    async fn mkdir(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: u32,
        _: OperationTime,
    ) -> FsResult<Entry> {
        unsupported()
    }

    async fn symlink(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: &SymlinkTarget,
        _: OperationTime,
    ) -> FsResult<Entry> {
        unsupported()
    }

    async fn link(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: NodeKey,
        _: &Name,
        _: OperationTime,
    ) -> FsResult<Entry> {
        unsupported()
    }

    async fn unlink(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: OperationTime,
    ) -> FsResult<()> {
        unsupported()
    }

    async fn rmdir(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: OperationTime,
    ) -> FsResult<()> {
        unsupported()
    }

    async fn rename(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &Name,
        _: NodeKey,
        _: &Name,
        _: RenameMode,
        _: OperationTime,
    ) -> FsResult<()> {
        unsupported()
    }

    async fn readlink(&self, _: &RequestContext, _: NodeKey) -> FsResult<Vec<u8>> {
        unsupported()
    }

    async fn open(&self, _: &RequestContext, _: NodeKey, _: OpenOptions) -> FsResult<FileHandle> {
        unsupported()
    }

    async fn release(&self, _: FileHandle) -> FsResult<()> {
        unsupported()
    }

    async fn opendir(&self, _: &RequestContext, _: NodeKey) -> FsResult<DirHandle> {
        unsupported()
    }

    async fn readdir(
        &self,
        _: &RequestContext,
        _: DirHandle,
        _: DirectoryCookie,
        _: u32,
    ) -> FsResult<DirBatch> {
        unsupported()
    }

    async fn releasedir(&self, _: DirHandle) -> FsResult<()> {
        unsupported()
    }

    async fn read(
        &self,
        _: &RequestContext,
        _: FileHandle,
        _: FileOffset,
        _: u64,
    ) -> FsResult<Vec<u8>> {
        unsupported()
    }

    async fn write(
        &self,
        _: &RequestContext,
        _: FileHandle,
        _: FileOffset,
        _: &[u8],
        _: WriteOptions,
        _: OperationTime,
    ) -> FsResult<WriteResult> {
        unsupported()
    }

    async fn seek(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: FileOffset,
        _: SeekKind,
    ) -> FsResult<FileOffset> {
        unsupported()
    }

    async fn fallocate(
        &self,
        _: &RequestContext,
        _: FileHandle,
        _: FallocateMode,
        _: FileRange,
        _: benchfs_sdk::PrivilegeClearMask,
        _: OperationTime,
    ) -> FsResult<()> {
        unsupported()
    }

    async fn setxattr(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &XattrName,
        _: &XattrValue,
        _: XattrSetMode,
        _: OperationTime,
    ) -> FsResult<()> {
        unsupported()
    }

    async fn getxattr(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &XattrName,
    ) -> FsResult<XattrValue> {
        unsupported()
    }

    async fn listxattr(&self, _: &RequestContext, _: NodeKey) -> FsResult<Vec<XattrName>> {
        unsupported()
    }

    async fn removexattr(
        &self,
        _: &RequestContext,
        _: NodeKey,
        _: &XattrName,
        _: OperationTime,
    ) -> FsResult<()> {
        unsupported()
    }

    async fn statfs(&self) -> FsResult<StatFs> {
        unsupported()
    }

    async fn flush(&self, _: FileHandle) -> FsResult<()> {
        unsupported()
    }

    async fn fsync(&self, _: FileHandle, _: FileSyncMode) -> FsResult<()> {
        unsupported()
    }

    async fn fsyncdir(&self, _: DirHandle) -> FsResult<()> {
        unsupported()
    }
}

#[async_trait(?Send)]
impl Filesystem for NullFs {
    async fn mount(_: Box<dyn BlockDevice>, _: MountOptions) -> FsResult<Mounted<Self>> {
        unsupported()
    }

    async fn unmount(self) -> FsResult<()> {
        unsupported()
    }
}
