//! Linux native bridge and explicit non-Linux fallback.

/// Configuration for the native FUSE dispatch runtime.
///
/// The queue capacity limits admitted requests waiting for a worker. Each
/// worker also has a fixed active-future cap to bound in-flight work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeConfig {
    /// Number of OS worker threads, in the inclusive range 1..=16.
    pub worker_threads: usize,
    /// Number of requests admitted in the shared queue, in the range
    /// 1..=1,048,576.
    pub queue_capacity: usize,
}

impl RuntimeConfig {
    /// Maximum number of worker threads.
    pub const MAX_WORKER_THREADS: usize = 16;
    /// Default number of worker threads.
    pub const DEFAULT_WORKER_THREADS: usize = 4;
    /// Default number of queued requests.
    pub const DEFAULT_QUEUE_CAPACITY: usize = 1024;
    /// Maximum number of queued requests.
    pub const MAX_QUEUE_CAPACITY: usize = 1 << 20;

    /// Creates a runtime configuration. Invalid zero or oversized values are
    /// rejected by [`crate::run_mount`].
    #[must_use]
    pub const fn new(worker_threads: usize, queue_capacity: usize) -> Self {
        Self {
            worker_threads,
            queue_capacity,
        }
    }

    /// Loads configuration from `BENCHFS_FUSE_*` environment variables.
    ///
    /// Missing variables use [`Default`]. Present variables must be unsigned
    /// decimal integers within the limits enforced by [`crate::run_mount`].
    pub fn from_env() -> benchfs_sdk::FsResult<Self> {
        let config = Self::new(
            Self::env_usize("BENCHFS_FUSE_WORKER_THREADS", Self::DEFAULT_WORKER_THREADS)?,
            Self::env_usize("BENCHFS_FUSE_QUEUE_CAPACITY", Self::DEFAULT_QUEUE_CAPACITY)?,
        );
        config.validate()?;
        Ok(config)
    }

    fn env_usize(name: &str, default: usize) -> benchfs_sdk::FsResult<usize> {
        match std::env::var(name) {
            Ok(value) => value
                .parse()
                .map_err(|_| benchfs_sdk::Errno::InvalidArgument.into()),
            Err(std::env::VarError::NotPresent) => Ok(default),
            Err(std::env::VarError::NotUnicode(_)) => {
                Err(benchfs_sdk::Errno::InvalidArgument.into())
            }
        }
    }

    fn validate(self) -> benchfs_sdk::FsResult<()> {
        if self.worker_threads == 0
            || self.worker_threads > Self::MAX_WORKER_THREADS
            || self.queue_capacity == 0
            || self.queue_capacity > Self::MAX_QUEUE_CAPACITY
        {
            return Err(benchfs_sdk::Errno::InvalidArgument.into());
        }
        Ok(())
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self::new(Self::DEFAULT_WORKER_THREADS, Self::DEFAULT_QUEUE_CAPACITY)
    }
}

#[cfg(not(all(target_os = "linux", not(benchfs_skip_native))))]
/// Returns `EOPNOTSUPP` when the native adapter is unavailable for this target.
pub fn run_mount(
    _: std::sync::Arc<dyn benchfs_sdk::FilesystemOperations>,
    _: &std::path::Path,
    _: RuntimeConfig,
) -> benchfs_sdk::FsResult<()> {
    Err(benchfs_sdk::Errno::NotSupported.into())
}

#[cfg(not(all(target_os = "linux", not(benchfs_skip_native))))]
/// Returns `EOPNOTSUPP` when the native adapter is unavailable for this target.
pub fn run_mount_cow<F: benchfs_sdk::CowFilesystem>(
    _: benchfs_sdk::Mounted<F>,
    _: crate::MountSource,
    _: &std::path::Path,
    _: RuntimeConfig,
) -> benchfs_sdk::FsResult<()> {
    Err(benchfs_sdk::Errno::NotSupported.into())
}

#[cfg(all(target_os = "linux", not(benchfs_skip_native)))]
mod linux {
    #![allow(unsafe_code)]

    use std::collections::HashMap;
    use std::ffi::{CString, c_char, c_int, c_void};
    use std::num::NonZeroU64;
    use std::os::unix::ffi::OsStrExt;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::path::Path;
    use std::ptr;
    use std::sync::{
        Arc, Weak,
        atomic::{AtomicBool, Ordering},
    };

    use crate::{
        Adapter, MountSource, RuntimeConfig, XattrReply, normalize_fallocate_mode,
        normalize_open_options, normalize_rename_mode, normalize_setattr_privilege_clear,
        normalize_xattr_mode,
    };
    use async_channel::Sender;
    use benchfs_sdk::{
        DeviceId, DirHandle, DirectoryCookie, Entry, Errno, FileAttr, FileHandle, FileKind,
        FileOffset, FileRange, FilesystemOperations, FsError, FsResult, MODE_PERMISSIONS,
        PrivilegeClearMask, RequestContext, SeekKind, SetAttr, TimeSelector, Timestamp,
        TruncateIntent,
    };
    use compio::runtime::Runtime;
    use futures::FutureExt;
    use futures::stream::{FuturesUnordered, StreamExt};

    const LOOKUP: u32 = 1;
    const FORGET: u32 = 2;
    const FORGET_MULTI: u32 = 3;
    const GETATTR: u32 = 4;
    const SETATTR: u32 = 5;
    const READLINK: u32 = 6;
    const MKNOD: u32 = 7;
    const MKDIR: u32 = 8;
    const UNLINK: u32 = 9;
    const RMDIR: u32 = 10;
    const SYMLINK: u32 = 11;
    const RENAME: u32 = 12;
    const LINK: u32 = 13;
    const OPEN: u32 = 14;
    const READ: u32 = 15;
    const WRITE: u32 = 16;
    const FLUSH: u32 = 17;
    const RELEASE: u32 = 18;
    const FSYNC: u32 = 19;
    const OPENDIR: u32 = 20;
    const READDIR: u32 = 21;
    const RELEASEDIR: u32 = 22;
    const MAX_ACTIVE_REQUESTS_PER_WORKER: usize = 32;
    const FSYNCDIR: u32 = 23;
    const STATFS: u32 = 24;
    const SETXATTR: u32 = 25;
    const GETXATTR: u32 = 26;
    const LISTXATTR: u32 = 27;
    const REMOVEXATTR: u32 = 28;
    const CREATE: u32 = 29;
    const FALLOCATE: u32 = 30;
    const LSEEK: u32 = 31;

    const SET_ATTR_MODE: u32 = 1 << 0;
    const SET_ATTR_UID: u32 = 1 << 1;
    const SET_ATTR_GID: u32 = 1 << 2;
    const SET_ATTR_SIZE: u32 = 1 << 3;
    const SET_ATTR_ATIME: u32 = 1 << 4;
    const SET_ATTR_MTIME: u32 = 1 << 5;
    const SET_ATTR_ATIME_NOW: u32 = 1 << 7;
    const SET_ATTR_MTIME_NOW: u32 = 1 << 8;
    const SEEK_DATA: u32 = 3;
    const SEEK_HOLE: u32 = 4;

    #[repr(C)]
    struct NativeForget {
        inode: u64,
        count: u64,
    }

    #[repr(C)]
    struct NativeRequest {
        opcode: u32,
        uid: u32,
        gid: u32,
        pid: u32,
        umask: u32,
        attribute_uid: u32,
        attribute_gid: u32,
        flags: u32,
        flags2: u32,
        mode: u32,
        device_major: u32,
        device_minor: u32,
        has_handle: u32,
        inode: u64,
        inode2: u64,
        handle: u64,
        offset: i64,
        offset2: i64,
        size: u64,
        size2: u64,
        atime_seconds: i64,
        atime_nanoseconds: u32,
        mtime_seconds: i64,
        mtime_nanoseconds: u32,
        name: *const u8,
        name_length: usize,
        name2: *const u8,
        name2_length: usize,
        data: *const u8,
        data_length: usize,
    }

    #[repr(C)]
    struct NativeAttr {
        inode: u64,
        generation: u64,
        mode: u32,
        uid: u32,
        gid: u32,
        device_major: u32,
        device_minor: u32,
        link_count: u64,
        size: u64,
        allocated_bytes: u64,
        atime_seconds: i64,
        atime_nanoseconds: u32,
        mtime_seconds: i64,
        mtime_nanoseconds: u32,
        ctime_seconds: i64,
        ctime_nanoseconds: u32,
    }

    #[repr(C)]
    struct NativeStatFs {
        blocks: u64,
        blocks_free: u64,
        blocks_available: u64,
        files: u64,
        files_free: u64,
        block_size: u32,
        fragment_size: u32,
        name_max: u32,
        read_only: u32,
    }

    #[repr(C)]
    struct NativeDirent {
        name: *const u8,
        name_length: usize,
        inode: u64,
        mode: u32,
        next_cookie: u64,
    }

    struct OwnedRequest {
        opcode: u32,
        uid: u32,
        gid: u32,
        pid: u32,
        umask: u32,
        attribute_uid: u32,
        attribute_gid: u32,
        flags: u32,
        flags2: u32,
        mode: u32,
        device_major: u32,
        device_minor: u32,
        has_handle: u32,
        inode: u64,
        inode2: u64,
        handle: u64,
        offset: i64,
        offset2: i64,
        size: u64,
        size2: u64,
        atime_seconds: i64,
        atime_nanoseconds: u32,
        mtime_seconds: i64,
        mtime_nanoseconds: u32,
        name: Vec<u8>,
        name2: Vec<u8>,
        data: Vec<u8>,
    }

    impl OwnedRequest {
        unsafe fn from_raw(request: &NativeRequest) -> Result<Self, FsError> {
            let copy = |pointer: *const u8, length: usize| -> Result<Vec<u8>, FsError> {
                if length == 0 {
                    return Ok(Vec::new());
                }
                if pointer.is_null() {
                    return Err(FsError::from(Errno::BadAddress));
                }
                Ok(unsafe { std::slice::from_raw_parts(pointer, length) }.to_vec())
            };
            Ok(Self {
                opcode: request.opcode,
                uid: request.uid,
                gid: request.gid,
                pid: request.pid,
                umask: request.umask,
                attribute_uid: request.attribute_uid,
                attribute_gid: request.attribute_gid,
                flags: request.flags,
                flags2: request.flags2,
                mode: request.mode,
                device_major: request.device_major,
                device_minor: request.device_minor,
                has_handle: request.has_handle,
                inode: request.inode,
                inode2: request.inode2,
                handle: request.handle,
                offset: request.offset,
                offset2: request.offset2,
                size: request.size,
                size2: request.size2,
                atime_seconds: request.atime_seconds,
                atime_nanoseconds: request.atime_nanoseconds,
                mtime_seconds: request.mtime_seconds,
                mtime_nanoseconds: request.mtime_nanoseconds,
                name: copy(request.name, request.name_length)?,
                name2: copy(request.name2, request.name2_length)?,
                data: copy(request.data, request.data_length)?,
            })
        }

        fn as_native(&self) -> NativeRequest {
            NativeRequest {
                opcode: self.opcode,
                uid: self.uid,
                gid: self.gid,
                pid: self.pid,
                umask: self.umask,
                attribute_uid: self.attribute_uid,
                attribute_gid: self.attribute_gid,
                flags: self.flags,
                flags2: self.flags2,
                mode: self.mode,
                device_major: self.device_major,
                device_minor: self.device_minor,
                has_handle: self.has_handle,
                inode: self.inode,
                inode2: self.inode2,
                handle: self.handle,
                offset: self.offset,
                offset2: self.offset2,
                size: self.size,
                size2: self.size2,
                atime_seconds: self.atime_seconds,
                atime_nanoseconds: self.atime_nanoseconds,
                mtime_seconds: self.mtime_seconds,
                mtime_nanoseconds: self.mtime_nanoseconds,
                name: self.name.as_ptr(),
                name_length: self.name.len(),
                name2: self.name2.as_ptr(),
                name2_length: self.name2.len(),
                data: self.data.as_ptr(),
                data_length: self.data.len(),
            }
        }
    }

    #[derive(Clone)]
    struct ReplyOnce {
        state: Arc<ReplyState>,
    }

    struct ReplyState {
        handle: usize,
        replied: AtomicBool,
        completed: AtomicBool,
        drain: Option<Weak<DrainState>>,
    }

    impl ReplyOnce {
        fn new(handle: usize) -> Self {
            Self::with_drain(handle, None)
        }

        fn registered(handle: usize, drain: &Arc<DrainState>) -> Self {
            Self::with_drain(handle, Some(Arc::downgrade(drain)))
        }

        fn with_drain(handle: usize, drain: Option<Weak<DrainState>>) -> Self {
            Self {
                state: Arc::new(ReplyState {
                    handle,
                    replied: AtomicBool::new(false),
                    completed: AtomicBool::new(false),
                    drain,
                }),
            }
        }

        fn same_request(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.state, &other.state)
        }

        fn claim(&self) -> bool {
            let claimed = !self.state.replied.swap(true, Ordering::AcqRel);
            if claimed {
                if let Some(drain) = self.state.drain.as_ref().and_then(Weak::upgrade) {
                    drain.retire(self);
                }
            }
            claimed
        }

        fn raw(&self) -> *mut c_void {
            self.state.handle as *mut c_void
        }

        fn error(&self, error: FsError) {
            if self.claim() {
                unsafe { benchfs_reply_error(self.raw(), error.raw_os_error()) };
            }
        }

        fn none(&self) {
            if self.claim() {
                unsafe { benchfs_reply_none(self.raw()) };
            }
        }

        fn success(&self) {
            if self.claim() {
                // libfuse's error reply uses errno 0 for success.
                unsafe { benchfs_reply_error(self.raw(), 0) };
            }
        }

        fn entry(&self, attribute: &NativeAttr) {
            if self.claim() {
                unsafe { benchfs_reply_entry(self.raw(), attribute as *const NativeAttr) };
            }
        }

        fn attribute(&self, attribute: &NativeAttr) {
            if self.claim() {
                unsafe { benchfs_reply_attribute(self.raw(), attribute as *const NativeAttr) };
            }
        }

        fn open(&self, handle: u64, directory: c_int) {
            if self.claim() {
                unsafe { benchfs_reply_open(self.raw(), handle, directory) };
            }
        }

        fn create(&self, attribute: &NativeAttr, handle: u64) {
            if self.claim() {
                unsafe { benchfs_reply_create(self.raw(), attribute as *const NativeAttr, handle) };
            }
        }

        fn data(&self, data: &[u8]) {
            if self.claim() {
                unsafe { benchfs_reply_data(self.raw(), data.as_ptr(), data.len()) };
            }
        }

        fn readlink(&self, data: &[u8]) {
            if self.claim() {
                unsafe { benchfs_reply_readlink_bytes(self.raw(), data.as_ptr(), data.len()) };
            }
        }

        fn write(&self, count: u32) {
            if self.claim() {
                unsafe { benchfs_reply_write(self.raw(), count) };
            }
        }

        fn statfs(&self, value: &NativeStatFs) {
            if self.claim() {
                unsafe { benchfs_reply_statfs_value(self.raw(), value as *const NativeStatFs) };
            }
        }

        fn xattr_size(&self, length: usize) {
            if self.claim() {
                unsafe { benchfs_reply_xattr_size(self.raw(), length) };
            }
        }

        fn lseek(&self, offset: i64) {
            if self.claim() {
                unsafe { benchfs_reply_lseek_value(self.raw(), offset) };
            }
        }

        fn readdir(&self, buffer_size: usize, entries: &[NativeDirent]) {
            if self.claim() {
                unsafe {
                    benchfs_reply_readdir_entries(
                        self.raw(),
                        buffer_size,
                        entries.as_ptr(),
                        entries.len(),
                    )
                };
            }
        }
    }
    unsafe extern "C" {
        fn benchfs_fuse_run(
            mountpoint: *const c_char,
            userdata: *mut c_void,
            dispatch: unsafe extern "C" fn(*mut c_void, *mut c_void, *const NativeRequest),
            drain: unsafe extern "C" fn(*mut c_void) -> c_int,
        ) -> c_int;
        fn benchfs_reply_error(request: *mut c_void, error_number: c_int);
        fn benchfs_reply_none(request: *mut c_void);
        fn benchfs_reply_entry(request: *mut c_void, attribute: *const NativeAttr);
        fn benchfs_reply_attribute(request: *mut c_void, attribute: *const NativeAttr);
        fn benchfs_reply_open(request: *mut c_void, handle: u64, directory: c_int);
        fn benchfs_reply_create(request: *mut c_void, attribute: *const NativeAttr, handle: u64);
        fn benchfs_reply_data(request: *mut c_void, data: *const u8, length: usize);
        fn benchfs_reply_readlink_bytes(request: *mut c_void, data: *const u8, length: usize);
        fn benchfs_reply_write(request: *mut c_void, count: u32);
        fn benchfs_reply_statfs_value(request: *mut c_void, value: *const NativeStatFs);
        fn benchfs_reply_xattr_size(request: *mut c_void, length: usize);
        fn benchfs_reply_lseek_value(request: *mut c_void, offset: i64);
        fn benchfs_reply_readdir_entries(
            request: *mut c_void,
            buffer_size: usize,
            entries: *const NativeDirent,
            count: usize,
        );
    }

    // `inner` is the single synchronization boundary for stop-admission,
    // request registration/completion, and the condition-variable predicate.
    struct DrainState {
        inner: std::sync::Mutex<DrainInner>,
        wake: std::sync::Condvar,
    }

    struct DrainInner {
        stopping: bool,
        // Admission guard keyed by the reusable native fuse_req_t address.
        replies: HashMap<usize, ReplyOnce>,
        // Requests remain active until dispatch returns, even after replying.
        active: usize,
    }

    impl DrainState {
        fn new() -> Self {
            Self {
                inner: std::sync::Mutex::new(DrainInner {
                    stopping: false,
                    replies: HashMap::new(),
                    active: 0,
                }),
                wake: std::sync::Condvar::new(),
            }
        }

        fn register(&self, reply: ReplyOnce) -> FsResult<()> {
            let mut inner = self.inner.lock().unwrap_or_else(|error| error.into_inner());
            if inner.stopping {
                return Err(Errno::Io.into());
            }

            let handle = reply.state.handle;
            if inner.replies.contains_key(&handle) {
                return Err(Errno::Corrupt.into());
            }
            inner.active = inner.active.checked_add(1).ok_or(Errno::Overflow)?;
            inner.replies.insert(handle, reply);
            Ok(())
        }

        // A native request address may be reused as soon as fuse_reply_* returns.
        // Retire it before making that call; `active` still keeps drain waiting.
        fn retire(&self, reply: &ReplyOnce) {
            let mut inner = self.inner.lock().unwrap_or_else(|error| error.into_inner());
            let handle = reply.state.handle;
            if inner
                .replies
                .get(&handle)
                .is_some_and(|current| current.same_request(reply))
            {
                inner.replies.remove(&handle);
            }
        }

        fn complete(&self, reply: &ReplyOnce) {
            if reply.state.completed.swap(true, Ordering::AcqRel) {
                return;
            }
            let mut inner = self.inner.lock().unwrap_or_else(|error| error.into_inner());
            let handle = reply.state.handle;
            if inner
                .replies
                .get(&handle)
                .is_some_and(|current| current.same_request(reply))
            {
                inner.replies.remove(&handle);
            }
            inner.active -= 1;
            if inner.active == 0 {
                self.wake.notify_all();
            }
        }

        #[cfg(test)]
        fn is_stopping(&self) -> bool {
            self.inner
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .stopping
        }

        fn stop_admission(&self) {
            self.inner
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .stopping = true;
        }
    }

    struct PendingRequest {
        reply: ReplyOnce,
        request: OwnedRequest,
        state: Arc<DrainState>,
    }
    enum WorkerMessage {
        Request(PendingRequest),
        Drain(std::sync::mpsc::SyncSender<FsResult<()>>),
    }
    struct AsyncBridge {
        sender: Sender<WorkerMessage>,
        state: Arc<DrainState>,
    }

    async fn process_request(
        adapter: Arc<Adapter<dyn FilesystemOperations>>,
        pending: PendingRequest,
    ) {
        let reply = pending.reply;
        let state = pending.state;
        let request = pending.request;
        let native = request.as_native();
        let result = AssertUnwindSafe(dispatch_request(&adapter, &reply, &native))
            .catch_unwind()
            .await;
        if result.is_err() {
            reply.error(Errno::Io.into());
        }
        state.complete(&reply);
    }
    async fn process_message(
        adapter: Arc<Adapter<dyn FilesystemOperations>>,
        message: WorkerMessage,
    ) {
        match message {
            WorkerMessage::Request(pending) => process_request(adapter, pending).await,
            WorkerMessage::Drain(completion) => {
                let result = adapter.drain_handles().await;
                let _ = completion.send(result);
            }
        }
    }
    async fn worker_loop(
        adapter: Arc<Adapter<dyn FilesystemOperations>>,
        receiver: async_channel::Receiver<WorkerMessage>,
    ) {
        let mut active = FuturesUnordered::new();
        let mut receiver_open = true;
        while receiver_open || !active.is_empty() {
            while receiver_open && active.len() < MAX_ACTIVE_REQUESTS_PER_WORKER {
                match receiver.try_recv() {
                    Ok(pending) => {
                        active.push(process_message(Arc::clone(&adapter), pending));
                    }
                    Err(async_channel::TryRecvError::Empty) => break,
                    Err(async_channel::TryRecvError::Closed) => receiver_open = false,
                }
            }

            if active.is_empty() {
                if !receiver_open {
                    break;
                }
                match receiver.recv().await {
                    Ok(pending) => active.push(process_message(Arc::clone(&adapter), pending)),
                    Err(_) => receiver_open = false,
                }
                continue;
            }

            if !receiver_open {
                while active.next().await.is_some() {}
                break;
            }
            if active.len() >= MAX_ACTIVE_REQUESTS_PER_WORKER {
                let _ = active.next().await;
                continue;
            }

            let receive = receiver.recv().fuse();
            let complete = active.next().fuse();
            futures::pin_mut!(receive, complete);
            match futures::future::select(receive, complete).await {
                futures::future::Either::Left((pending, _)) => match pending {
                    Ok(pending) => active.push(process_message(Arc::clone(&adapter), pending)),
                    Err(_) => receiver_open = false,
                },
                futures::future::Either::Right((_completed, _)) => {}
            }
        }
    }

    /// Runs a foreground libfuse session while dispatching filesystem work on
    /// a bounded queue. Each worker owns one OS thread and one thread-local
    pub fn run_mount(
        filesystem: Arc<dyn FilesystemOperations>,
        mountpoint: &Path,
        config: RuntimeConfig,
    ) -> FsResult<()> {
        config.validate()?;
        let mountpoint = CString::new(mountpoint.as_os_str().as_bytes())
            .map_err(|_| FsError::from(Errno::InvalidArgument))?;
        let (sender, receiver) = async_channel::bounded::<WorkerMessage>(config.queue_capacity);
        let adapter = Arc::new(Adapter::new(filesystem));
        let state = Arc::new(DrainState::new());
        let bridge = Box::new(AsyncBridge { sender, state });
        let userdata = (&*bridge as *const AsyncBridge) as usize;

        let worker_count = config.worker_threads;
        let (ready_sender, ready_receiver) = std::sync::mpsc::sync_channel(worker_count);
        let mut workers = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            let receiver = receiver.clone();
            let adapter = Arc::clone(&adapter);
            let ready_sender = ready_sender.clone();
            workers.push(std::thread::spawn(move || {
                let runtime = match catch_unwind(AssertUnwindSafe(Runtime::new)) {
                    Ok(Ok(runtime)) => {
                        let _ = ready_sender.send(true);
                        runtime
                    }
                    _ => {
                        let _ = ready_sender.send(false);
                        return;
                    }
                };
                runtime.block_on(worker_loop(adapter, receiver));
            }));
        }
        drop(receiver);
        drop(ready_sender);
        let all_workers_ready = (0..worker_count).all(|_| ready_receiver.recv().unwrap_or(false));
        if !all_workers_ready {
            drop(bridge);
            for worker in workers {
                let _ = worker.join();
            }
            return Err(Errno::Io.into());
        }

        let fuse_thread = std::thread::spawn(move || {
            let result = unsafe {
                benchfs_fuse_run(
                    mountpoint.as_ptr(),
                    userdata as *mut c_void,
                    dispatch_callback,
                    drain_callback,
                )
            };
            drop(bridge);
            result
        });

        let result = fuse_thread.join().map_err(|_| FsError::from(Errno::Io));
        for worker in workers {
            let _ = worker.join();
        }
        let result = result?;
        if result == 0 {
            Ok(())
        } else {
            let errno = Errno::from_i32(result.saturating_abs()).unwrap_or(Errno::Io);
            Err(errno.into())
        }
    }

    /// Runs a FUSE session for one [`benchfs_sdk::Mounted`] COW filesystem view.
    ///
    /// `MountSource::Current` serves the writable current view.
    /// `MountSource::Snapshot` acquires an independently handled read-only
    /// snapshot view through [`benchfs_sdk::CowFilesystem::snapshot_mount`],
    /// serves that view for the session, and releases the in-memory mount
    /// reference through [`benchfs_sdk::CowFilesystem::snapshot_unmount`] on
    /// exit (success, error, or panic).
    pub fn run_mount_cow<F: benchfs_sdk::CowFilesystem>(
        mounted: benchfs_sdk::Mounted<F>,
        source: MountSource,
        mountpoint: &Path,
        config: RuntimeConfig,
    ) -> FsResult<()> {
        let snapshot = match source {
            MountSource::Current => None,
            MountSource::Snapshot(selector) => {
                // Acquire on a throwaway runtime; the session workers run
                // their own runtimes once the bridge is live. The filesystem
                // borrow ends before `mounted` is moved into SnapshotOwner.
                let runtime = Runtime::new().map_err(|_| FsError::from(Errno::Io))?;
                let context = RequestContext {
                    uid: 0,
                    gid: 0,
                    pid: 0,
                    umask: 0,
                };
                let outcome =
                    runtime.block_on(mounted.filesystem.snapshot_mount(&context, &selector));
                match outcome {
                    Ok(snapshot) => Some(snapshot),
                    Err(error) => return Err(error),
                }
            }
        };
        let filesystem: Arc<dyn FilesystemOperations> =
            Arc::new(SnapshotOwner::new(mounted, snapshot));
        run_mount(filesystem, mountpoint, config)
    }

    /// Serves a COW current or snapshot view and releases the in-memory
    /// snapshot mount reference exactly once when the session ends.
    struct SnapshotOwner<F: benchfs_sdk::CowFilesystem> {
        filesystem: F,
        snapshot: Option<benchfs_sdk::MountedSnapshot<F::SnapshotView>>,
        released: AtomicBool,
    }

    impl<F: benchfs_sdk::CowFilesystem> SnapshotOwner<F> {
        fn new(
            mounted: benchfs_sdk::Mounted<F>,
            snapshot: Option<benchfs_sdk::MountedSnapshot<F::SnapshotView>>,
        ) -> Self {
            Self {
                filesystem: mounted.filesystem,
                snapshot,
                released: AtomicBool::new(false),
            }
        }
    }

    impl<F: benchfs_sdk::CowFilesystem> Drop for SnapshotOwner<F> {
        fn drop(&mut self) {
            if self.released.swap(true, Ordering::SeqCst) {
                return;
            }
            if let Some(snapshot) = self.snapshot.take() {
                if let Ok(runtime) = Runtime::new() {
                    // Best effort: the session is ending and the only
                    // observable failure mode is a dropped mount reference.
                    let _ = runtime.block_on(self.filesystem.snapshot_unmount(snapshot));
                }
            }
        }
    }

    #[async_trait::async_trait(?Send)]
    impl<F: benchfs_sdk::CowFilesystem> FilesystemOperations for SnapshotOwner<F> {
        fn variant(&self) -> benchfs_sdk::Variant {
            self.filesystem.variant()
        }

        fn capabilities(&self) -> benchfs_sdk::Capabilities {
            self.filesystem.capabilities()
        }

        async fn lookup(
            &self,
            context: &RequestContext,
            parent: benchfs_sdk::NodeKey,
            name: &benchfs_sdk::Name,
        ) -> FsResult<benchfs_sdk::Entry> {
            self.snapshot_view()?.lookup(context, parent, name).await
        }

        async fn getattr(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
            handle: Option<benchfs_sdk::FileHandle>,
        ) -> FsResult<benchfs_sdk::FileAttr> {
            self.snapshot_view()?.getattr(context, node, handle).await
        }

        async fn setattr(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::SetAttr,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::FileAttr> {
            Err(Errno::ReadOnly.into())
        }

        async fn create(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: u32,
            _: benchfs_sdk::OpenOptions,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::CreateResult> {
            Err(Errno::ReadOnly.into())
        }

        async fn mknod(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: benchfs_sdk::FileKind,
            _: u32,
            _: benchfs_sdk::DeviceId,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::Entry> {
            Err(Errno::ReadOnly.into())
        }

        async fn mkdir(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: u32,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::Entry> {
            Err(Errno::ReadOnly.into())
        }

        async fn symlink(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: &benchfs_sdk::SymlinkTarget,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::Entry> {
            Err(Errno::ReadOnly.into())
        }

        async fn link(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::Entry> {
            Err(Errno::ReadOnly.into())
        }

        async fn unlink(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<()> {
            Err(Errno::ReadOnly.into())
        }

        async fn rmdir(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<()> {
            Err(Errno::ReadOnly.into())
        }

        async fn rename(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::Name,
            _: benchfs_sdk::RenameMode,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<()> {
            Err(Errno::ReadOnly.into())
        }

        async fn readlink(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
        ) -> FsResult<Vec<u8>> {
            self.snapshot_view()?.readlink(context, node).await
        }

        async fn open(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
            options: benchfs_sdk::OpenOptions,
        ) -> FsResult<benchfs_sdk::FileHandle> {
            self.snapshot_view()?.open(context, node, options).await
        }

        async fn release(&self, handle: benchfs_sdk::FileHandle) -> FsResult<()> {
            self.snapshot_view()?.release(handle).await
        }

        async fn opendir(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
        ) -> FsResult<benchfs_sdk::DirHandle> {
            self.snapshot_view()?.opendir(context, node).await
        }

        async fn readdir(
            &self,
            context: &RequestContext,
            handle: benchfs_sdk::DirHandle,
            cookie: benchfs_sdk::DirectoryCookie,
            max_entries: u32,
        ) -> FsResult<benchfs_sdk::DirBatch> {
            self.snapshot_view()?
                .readdir(context, handle, cookie, max_entries)
                .await
        }

        async fn releasedir(&self, handle: benchfs_sdk::DirHandle) -> FsResult<()> {
            self.snapshot_view()?.releasedir(handle).await
        }

        async fn read(
            &self,
            context: &RequestContext,
            handle: benchfs_sdk::FileHandle,
            offset: benchfs_sdk::FileOffset,
            length: u64,
        ) -> FsResult<Vec<u8>> {
            self.snapshot_view()?
                .read(context, handle, offset, length)
                .await
        }

        async fn write(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::FileHandle,
            _: benchfs_sdk::FileOffset,
            _: &[u8],
            _: benchfs_sdk::WriteOptions,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<benchfs_sdk::WriteResult> {
            Err(Errno::ReadOnly.into())
        }

        async fn seek(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
            offset: benchfs_sdk::FileOffset,
            kind: benchfs_sdk::SeekKind,
        ) -> FsResult<benchfs_sdk::FileOffset> {
            self.snapshot_view()?
                .seek(context, node, offset, kind)
                .await
        }

        async fn fallocate(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::FileHandle,
            _: benchfs_sdk::FallocateMode,
            _: benchfs_sdk::FileRange,
            _: benchfs_sdk::PrivilegeClearMask,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<()> {
            Err(Errno::ReadOnly.into())
        }

        async fn setxattr(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::XattrName,
            _: &benchfs_sdk::XattrValue,
            _: benchfs_sdk::XattrSetMode,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<()> {
            Err(Errno::ReadOnly.into())
        }

        async fn getxattr(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
            name: &benchfs_sdk::XattrName,
        ) -> FsResult<benchfs_sdk::XattrValue> {
            self.snapshot_view()?.getxattr(context, node, name).await
        }

        async fn listxattr(
            &self,
            context: &RequestContext,
            node: benchfs_sdk::NodeKey,
        ) -> FsResult<Vec<benchfs_sdk::XattrName>> {
            self.snapshot_view()?.listxattr(context, node).await
        }

        async fn removexattr(
            &self,
            _: &RequestContext,
            _: benchfs_sdk::NodeKey,
            _: &benchfs_sdk::XattrName,
            _: benchfs_sdk::OperationTime,
        ) -> FsResult<()> {
            Err(Errno::ReadOnly.into())
        }

        async fn statfs(&self) -> FsResult<benchfs_sdk::StatFs> {
            self.filesystem.statfs().await
        }

        async fn flush(&self, handle: benchfs_sdk::FileHandle) -> FsResult<()> {
            self.snapshot_view()?.flush(handle).await
        }

        async fn fsync(
            &self,
            handle: benchfs_sdk::FileHandle,
            mode: benchfs_sdk::FileSyncMode,
        ) -> FsResult<()> {
            self.snapshot_view()?.fsync(handle, mode).await
        }

        async fn fsyncdir(&self, handle: benchfs_sdk::DirHandle) -> FsResult<()> {
            self.snapshot_view()?.fsyncdir(handle).await
        }
    }

    impl<F: benchfs_sdk::CowFilesystem> SnapshotOwner<F> {
        /// Returns the captured read-only view, rejecting absent snapshots.
        fn snapshot_view(&self) -> FsResult<&F::SnapshotView> {
            self.snapshot
                .as_ref()
                .map(|mounted| &mounted.view)
                .ok_or_else(|| Errno::Stale.into())
        }
    }
    unsafe extern "C" fn dispatch_callback(
        userdata: *mut c_void,
        request_handle: *mut c_void,
        request: *const NativeRequest,
    ) {
        if userdata.is_null() || request_handle.is_null() || request.is_null() {
            if !request_handle.is_null() {
                ReplyOnce::new(request_handle as usize).error(Errno::BadAddress.into());
            }
            return;
        }
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            let bridge = unsafe { &*userdata.cast::<AsyncBridge>() };
            let request = unsafe { &*request };
            let request = unsafe { OwnedRequest::from_raw(request) }?;
            let reply = ReplyOnce::registered(request_handle as usize, &bridge.state);
            bridge.state.register(reply.clone())?;
            let cleanup = reply.clone();
            bridge
                .sender
                .try_send(WorkerMessage::Request(PendingRequest {
                    reply,
                    request,
                    state: Arc::clone(&bridge.state),
                }))
                .map_err(|_| {
                    bridge.state.complete(&cleanup);
                    FsError::from(Errno::Busy)
                })
        }));
        match outcome {
            Ok(Ok(())) => {}
            Ok(Err(error)) => ReplyOnce::new(request_handle as usize).error(error),
            Err(_) => ReplyOnce::new(request_handle as usize).error(Errno::Io.into()),
        }
    }

    unsafe extern "C" fn drain_callback(userdata: *mut c_void) -> c_int {
        if userdata.is_null() {
            return Errno::BadAddress.as_i32();
        }
        let bridge = unsafe { &*userdata.cast::<AsyncBridge>() };
        bridge.state.stop_admission();
        let mut inner = bridge
            .state
            .inner
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        while inner.active != 0 {
            inner = bridge
                .state
                .wake
                .wait(inner)
                .unwrap_or_else(|error| error.into_inner());
        }
        drop(inner);

        let (completion, completed) = std::sync::mpsc::sync_channel(1);
        if bridge
            .sender
            .try_send(WorkerMessage::Drain(completion))
            .is_err()
        {
            return Errno::Io.as_i32();
        }
        match completed.recv() {
            Ok(Ok(())) => 0,
            Ok(Err(_)) | Err(_) => Errno::Io.as_i32(),
        }
    }

    #[allow(clippy::too_many_lines)] // Exhaustive, one-screen dispatch table for the frozen ABI.
    async fn dispatch_request(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        request: &NativeRequest,
    ) {
        let context = RequestContext {
            uid: request.uid,
            gid: request.gid,
            pid: request.pid,
            umask: request.umask,
        };
        let result = match request.opcode {
            LOOKUP => reply_entry(
                handle,
                adapter
                    .lookup(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                    )
                    .await,
            ),
            FORGET => {
                adapter.forget(request.inode, request.size).await;
                handle.none();
                return;
            }
            FORGET_MULTI => {
                if request.data_length % size_of::<NativeForget>() != 0
                    || (request.data_length != 0 && request.data.is_null())
                {
                    reply_error(handle, Errno::Io.into());
                    return;
                }
                let count = request.data_length / size_of::<NativeForget>();
                for index in 0..count {
                    let record = unsafe {
                        ptr::read_unaligned(
                            request
                                .data
                                .add(index * size_of::<NativeForget>())
                                .cast::<NativeForget>(),
                        )
                    };
                    adapter.forget(record.inode, record.count).await;
                }
                handle.none();
                return;
            }
            GETATTR => {
                let result = match optional_file_handle(request) {
                    Ok(file) => adapter.getattr(&context, request.inode, file).await,
                    Err(error) => Err(error),
                };
                reply_attr(handle, result)
            }
            SETATTR => reply_setattr(adapter, handle, &context, request).await,
            READLINK => reply_bytes(
                handle,
                adapter.readlink(&context, request.inode).await,
                true,
            ),
            MKNOD => reply_mknod(adapter, handle, &context, request).await,
            MKDIR => reply_created_entry(
                handle,
                adapter
                    .mkdir(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                        request.mode & MODE_PERMISSIONS,
                    )
                    .await,
            ),
            UNLINK => reply_mutation(
                handle,
                adapter
                    .unlink(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                    )
                    .await,
            ),
            RMDIR => reply_mutation(
                handle,
                adapter
                    .rmdir(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                    )
                    .await,
            ),
            SYMLINK => reply_created_entry(
                handle,
                adapter
                    .symlink(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                        bytes(request.data, request.data_length),
                    )
                    .await,
            ),
            RENAME => reply_rename(adapter, handle, &context, request).await,
            LINK => reply_created_entry(
                handle,
                adapter
                    .link(
                        &context,
                        request.inode,
                        request.inode2,
                        bytes(request.name, request.name_length),
                    )
                    .await,
            ),
            OPEN => reply_open(adapter, handle, &context, request).await,
            READ => reply_read(adapter, handle, &context, request).await,
            WRITE => reply_write(adapter, handle, &context, request).await,
            FLUSH => {
                let result = match file_handle(request.handle) {
                    Ok(file) => adapter.flush(file).await,
                    Err(error) => Err(error),
                };
                reply_empty(handle, result)
            }
            RELEASE => {
                let result = match file_handle(request.handle) {
                    Ok(file) => adapter.release(file).await,
                    Err(error) => Err(error),
                };
                reply_empty(handle, result)
            }
            FSYNC => {
                let result = match file_handle(request.handle) {
                    Ok(file) => adapter.fsync(file, request.flags != 0).await,
                    Err(error) => Err(error),
                };
                reply_empty(handle, result)
            }
            OPENDIR => reply_directory_open(adapter, handle, &context, request).await,
            READDIR => reply_readdir(adapter, handle, &context, request).await,
            RELEASEDIR => {
                let result = match dir_handle(request.handle) {
                    Ok(directory) => adapter.releasedir(directory).await,
                    Err(error) => Err(error),
                };
                reply_empty(handle, result)
            }
            FSYNCDIR => {
                let result = match dir_handle(request.handle) {
                    Ok(directory) => adapter.fsyncdir(directory).await,
                    Err(error) => Err(error),
                };
                reply_empty(handle, result)
            }
            STATFS => reply_statfs(handle, adapter.statfs().await),
            SETXATTR => reply_setxattr(adapter, handle, &context, request).await,
            GETXATTR => reply_xattr(
                handle,
                adapter
                    .getxattr(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                        usize::try_from(request.size).unwrap_or(usize::MAX),
                    )
                    .await,
            ),
            LISTXATTR => reply_xattr(
                handle,
                adapter
                    .listxattr(
                        &context,
                        request.inode,
                        usize::try_from(request.size).unwrap_or(usize::MAX),
                    )
                    .await,
            ),
            REMOVEXATTR => reply_single_mutation(
                handle,
                adapter
                    .removexattr(
                        &context,
                        request.inode,
                        bytes(request.name, request.name_length),
                    )
                    .await,
            ),
            CREATE => reply_create(adapter, handle, &context, request).await,
            FALLOCATE => reply_fallocate(adapter, handle, &context, request).await,
            LSEEK => reply_lseek(adapter, handle, &context, request).await,
            _ => Err(Errno::Io.into()),
        };
        if let Err(error) = result {
            reply_error(handle, error);
        }
    }

    fn bytes<'a>(pointer: *const u8, length: usize) -> &'a [u8] {
        if length == 0 || pointer.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(pointer, length) }
        }
    }

    fn file_handle(value: u64) -> FsResult<FileHandle> {
        FileHandle::new(value)
    }

    fn dir_handle(value: u64) -> FsResult<DirHandle> {
        DirHandle::new(value)
    }

    fn optional_file_handle(request: &NativeRequest) -> FsResult<Option<FileHandle>> {
        if request.has_handle == 0 {
            Ok(None)
        } else {
            file_handle(request.handle).map(Some)
        }
    }

    fn file_offset(value: i64) -> FsResult<FileOffset> {
        let value = u64::try_from(value).map_err(|_| FsError::from(Errno::InvalidArgument))?;
        FileOffset::new(value)
    }

    fn native_attr(attribute: &FileAttr) -> NativeAttr {
        NativeAttr {
            inode: attribute.node.inode_id(),
            generation: attribute.node.generation(),
            mode: attribute.mode,
            uid: attribute.uid,
            gid: attribute.gid,
            device_major: attribute.device.major,
            device_minor: attribute.device.minor,
            link_count: attribute.link_count,
            size: attribute.size,
            allocated_bytes: attribute.allocated_bytes,
            atime_seconds: attribute.accessed_at.seconds(),
            atime_nanoseconds: attribute.accessed_at.nanoseconds(),
            mtime_seconds: attribute.modified_at.seconds(),
            mtime_nanoseconds: attribute.modified_at.nanoseconds(),
            ctime_seconds: attribute.changed_at.seconds(),
            ctime_nanoseconds: attribute.changed_at.nanoseconds(),
        }
    }

    fn reply_error(handle: &ReplyOnce, error: FsError) {
        handle.error(error);
    }

    fn reply_entry(handle: &ReplyOnce, result: FsResult<Entry>) -> FsResult<()> {
        let entry = result?;
        let attribute = native_attr(&entry.attributes);
        handle.entry(&attribute);
        Ok(())
    }

    fn reply_attr(handle: &ReplyOnce, result: FsResult<FileAttr>) -> FsResult<()> {
        let attribute = native_attr(&result?);
        handle.attribute(&attribute);
        Ok(())
    }

    async fn reply_setattr(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let flags = request.flags;
        let clear = normalize_setattr_privilege_clear(flags)?;
        let timestamp = |seconds, nanos| Timestamp::new(seconds, nanos);
        let attributes = SetAttr {
            mode: (flags & SET_ATTR_MODE != 0).then_some(request.mode & MODE_PERMISSIONS),
            uid: (flags & SET_ATTR_UID != 0).then_some(request.attribute_uid),
            gid: (flags & SET_ATTR_GID != 0).then_some(request.attribute_gid),
            size: (flags & SET_ATTR_SIZE != 0).then_some(request.size),
            accessed_at: if flags & SET_ATTR_ATIME_NOW != 0 {
                Some(TimeSelector::Now)
            } else if flags & SET_ATTR_ATIME != 0 {
                Some(TimeSelector::Explicit(timestamp(
                    request.atime_seconds,
                    request.atime_nanoseconds,
                )?))
            } else {
                None
            },
            modified_at: if flags & SET_ATTR_MTIME_NOW != 0 {
                Some(TimeSelector::Now)
            } else if flags & SET_ATTR_MTIME != 0 {
                Some(TimeSelector::Explicit(timestamp(
                    request.mtime_seconds,
                    request.mtime_nanoseconds,
                )?))
            } else {
                None
            },
            privilege_clear: clear,
            handle: optional_file_handle(request)?,
        };
        let (attribute, _invalidation) =
            adapter.setattr(context, request.inode, &attributes).await?;
        reply_attr(handle, Ok(attribute))
    }

    async fn reply_mknod(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let kind = if request.mode & benchfs_sdk::S_IFMT == 0 {
            FileKind::Regular
        } else {
            FileKind::from_mode(request.mode).ok_or(Errno::InvalidArgument)?
        };
        reply_created_entry(
            handle,
            adapter
                .mknod(
                    context,
                    request.inode,
                    bytes(request.name, request.name_length),
                    kind,
                    request.mode & MODE_PERMISSIONS,
                    DeviceId {
                        major: request.device_major,
                        minor: request.device_minor,
                    },
                )
                .await,
        )
    }

    fn reply_created_entry(
        handle: &ReplyOnce,
        result: FsResult<(Entry, Vec<crate::Invalidation>)>,
    ) -> FsResult<()> {
        let (entry, _invalidations) = result?;
        reply_entry(handle, Ok(entry))
    }

    fn reply_mutation(
        handle: &ReplyOnce,
        result: FsResult<Vec<crate::Invalidation>>,
    ) -> FsResult<()> {
        let _invalidations = result?;
        handle.success();
        Ok(())
    }

    fn reply_single_mutation(
        handle: &ReplyOnce,
        result: FsResult<crate::Invalidation>,
    ) -> FsResult<()> {
        let _invalidation = result?;
        handle.success();
        Ok(())
    }

    async fn reply_rename(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let mode = normalize_rename_mode(request.flags)?;
        reply_mutation(
            handle,
            adapter
                .rename(
                    context,
                    request.inode,
                    bytes(request.name, request.name_length),
                    request.inode2,
                    bytes(request.name2, request.name2_length),
                    mode,
                )
                .await,
        )
    }

    async fn reply_open(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let truncate = if request.flags & crate::O_TRUNC != 0 {
            Some(TruncateIntent {
                operation_time: adapter.sample_operation_time()?,
                privilege_clear: PrivilegeClearMask::BOTH,
            })
        } else {
            None
        };
        let options = normalize_open_options(request.flags, truncate)?;
        let (file, _invalidation) = adapter.open(context, request.inode, options).await?;
        handle.open(file.get(), 0);
        Ok(())
    }

    async fn reply_read(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let data = adapter
            .read(
                context,
                file_handle(request.handle)?,
                file_offset(request.offset)?,
                request.size,
            )
            .await?;
        handle.data(&data);
        Ok(())
    }

    async fn reply_write(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let (result, _invalidation) = adapter
            .write(
                context,
                file_handle(request.handle)?,
                file_offset(request.offset)?,
                bytes(request.data, request.data_length),
                if request.data_length == 0 {
                    PrivilegeClearMask::NONE
                } else {
                    PrivilegeClearMask::BOTH
                },
            )
            .await?;
        let count = u32::try_from(result.bytes_written).map_err(|_| Errno::Overflow)?;
        handle.write(count);
        Ok(())
    }

    fn reply_empty(handle: &ReplyOnce, result: FsResult<()>) -> FsResult<()> {
        result?;
        handle.success();
        Ok(())
    }

    async fn reply_directory_open(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let directory = adapter.opendir(context, request.inode).await?;
        handle.open(directory.get(), 1);
        Ok(())
    }

    async fn reply_readdir(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let raw_cookie = u64::try_from(request.offset).map_err(|_| Errno::InvalidArgument)?;
        let cookie = NonZeroU64::new(raw_cookie)
            .map_or(DirectoryCookie::START, DirectoryCookie::from_returned);
        let buffer_size = usize::try_from(request.size).map_err(|_| Errno::Overflow)?;
        let max_entries = u32::try_from(buffer_size / 24 + 1).unwrap_or(u32::MAX);
        let batch = adapter
            .readdir(context, dir_handle(request.handle)?, cookie, max_entries)
            .await?;
        let entries = batch
            .entries
            .iter()
            .map(|entry| {
                if entry.next_cookie.get() > i64::MAX as u64 {
                    return Err(FsError::from(Errno::Overflow));
                }
                Ok(NativeDirent {
                    name: entry.name.as_ptr(),
                    name_length: entry.name.len(),
                    inode: entry.node.inode_id(),
                    mode: entry.kind.mode_bits(),
                    next_cookie: entry.next_cookie.get(),
                })
            })
            .collect::<FsResult<Vec<_>>>()?;
        handle.readdir(buffer_size, &entries);
        Ok(())
    }

    fn reply_statfs(handle: &ReplyOnce, result: FsResult<benchfs_sdk::StatFs>) -> FsResult<()> {
        let value = result?;
        let native = NativeStatFs {
            blocks: value.blocks,
            blocks_free: value.blocks_free,
            blocks_available: value.blocks_available,
            files: value.files,
            files_free: value.files_free,
            block_size: value.block_size,
            fragment_size: value.fragment_size,
            name_max: value.name_max,
            read_only: u32::from(value.flags.read_only),
        };
        handle.statfs(&native);
        Ok(())
    }

    async fn reply_setxattr(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let mode = normalize_xattr_mode(request.flags)?;
        reply_single_mutation(
            handle,
            adapter
                .setxattr(
                    context,
                    request.inode,
                    bytes(request.name, request.name_length),
                    bytes(request.data, request.data_length),
                    mode,
                )
                .await,
        )
    }

    fn reply_xattr(handle: &ReplyOnce, result: FsResult<XattrReply>) -> FsResult<()> {
        match result? {
            XattrReply::Size(size) => handle.xattr_size(size),
            XattrReply::Data(data) => handle.data(&data),
        }
        Ok(())
    }

    async fn reply_create(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let options = normalize_open_options(request.flags & !crate::O_TRUNC, None)?;
        let (created, _invalidations) = adapter
            .create(
                context,
                request.inode,
                bytes(request.name, request.name_length),
                request.mode & MODE_PERMISSIONS,
                options,
            )
            .await?;
        let attribute = native_attr(&created.entry.attributes);
        handle.create(&attribute, created.handle.get());
        Ok(())
    }

    async fn reply_fallocate(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        if request.offset < 0 || request.offset2 <= 0 {
            return Err(Errno::InvalidArgument.into());
        }
        let mode = normalize_fallocate_mode(request.flags)?;
        let range = FileRange::new(
            u64::try_from(request.offset).map_err(|_| Errno::InvalidArgument)?,
            u64::try_from(request.offset2).map_err(|_| Errno::InvalidArgument)?,
        )?;
        reply_single_mutation(
            handle,
            adapter
                .fallocate(
                    context,
                    file_handle(request.handle)?,
                    mode,
                    range,
                    PrivilegeClearMask::BOTH,
                )
                .await,
        )
    }

    async fn reply_lseek(
        adapter: &Adapter<dyn FilesystemOperations>,
        handle: &ReplyOnce,
        context: &RequestContext,
        request: &NativeRequest,
    ) -> FsResult<()> {
        let file = file_handle(request.handle)?;
        let kind = match request.flags {
            SEEK_DATA => SeekKind::Data,
            SEEK_HOLE => SeekKind::Hole,
            _ => return Err(Errno::InvalidArgument.into()),
        };
        let offset = adapter
            .seek(
                context,
                request.inode,
                file,
                file_offset(request.offset)?,
                kind,
            )
            .await?;
        let offset = i64::try_from(offset.get()).map_err(|_| Errno::Overflow)?;
        handle.lseek(offset);
        Ok(())
    }

    fn reply_bytes(handle: &ReplyOnce, result: FsResult<Vec<u8>>, readlink: bool) -> FsResult<()> {
        let data = result?;
        if readlink {
            handle.readlink(&data);
        } else {
            handle.data(&data);
        }
        Ok(())
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        fn drain_state() -> Arc<DrainState> {
            Arc::new(DrainState::new())
        }

        #[test]
        fn reply_guard_claims_once() {
            let reply = ReplyOnce::new(1);
            assert!(reply.claim());
            assert!(!reply.claim());
        }

        #[test]
        fn owned_request_copies_input_buffers() {
            let name = b"name".to_vec();
            let data = b"payload".to_vec();
            let raw = NativeRequest {
                opcode: LOOKUP,
                uid: 1,
                gid: 2,
                pid: 3,
                umask: 4,
                attribute_uid: 5,
                attribute_gid: 6,
                flags: 7,
                flags2: 8,
                mode: 9,
                device_major: 10,
                device_minor: 11,
                has_handle: 0,
                inode: 12,
                inode2: 13,
                handle: 14,
                offset: 15,
                offset2: 16,
                size: 17,
                size2: 18,
                atime_seconds: 19,
                atime_nanoseconds: 20,
                mtime_seconds: 21,
                mtime_nanoseconds: 22,
                name: name.as_ptr(),
                name_length: name.len(),
                name2: std::ptr::null(),
                name2_length: 0,
                data: data.as_ptr(),
                data_length: data.len(),
            };
            let owned = unsafe { OwnedRequest::from_raw(&raw) }.unwrap();
            drop(name);
            drop(data);
            let rebuilt = owned.as_native();
            assert_eq!(
                unsafe { std::slice::from_raw_parts(rebuilt.name, rebuilt.name_length) },
                b"name"
            );
            assert_eq!(
                unsafe { std::slice::from_raw_parts(rebuilt.data, rebuilt.data_length) },
                b"payload"
            );
        }
        #[test]
        fn drain_state_tracks_pending_request_to_zero() {
            let state = drain_state();
            let reply = ReplyOnce::registered(7, &state);
            state.register(reply.clone()).unwrap();
            assert_eq!(state.inner.lock().unwrap().replies.len(), 1);
            state.stop_admission();
            assert!(state.is_stopping());
            state.complete(&reply);
            assert!(state.inner.lock().unwrap().replies.is_empty());
        }
        #[test]
        fn registration_cannot_cross_the_shutdown_transition() {
            let state = drain_state();
            let mut transition = state.inner.lock().unwrap();
            let (attempting_sender, attempting_receiver) = std::sync::mpsc::sync_channel(0);
            let (finished_sender, finished_receiver) = std::sync::mpsc::sync_channel(1);
            let worker_state = Arc::clone(&state);
            let worker = std::thread::spawn(move || {
                attempting_sender.send(()).unwrap();
                let reply = ReplyOnce::registered(8, &worker_state);
                let result = worker_state.register(reply);
                finished_sender.send(()).unwrap();
                result
            });

            attempting_receiver.recv().unwrap();
            assert!(
                finished_receiver
                    .recv_timeout(std::time::Duration::from_millis(50))
                    .is_err()
            );
            transition.stopping = true;
            drop(transition);

            assert!(worker.join().unwrap().is_err());
            assert!(state.inner.lock().unwrap().replies.is_empty());
        }

        #[test]
        fn final_completion_is_serialized_with_drain_waiting() {
            let state = drain_state();
            let reply = ReplyOnce::registered(9, &state);
            state.register(reply.clone()).unwrap();
            let transition = state.inner.lock().unwrap();
            let (attempting_sender, attempting_receiver) = std::sync::mpsc::sync_channel(0);
            let (finished_sender, finished_receiver) = std::sync::mpsc::sync_channel(1);
            let worker_state = Arc::clone(&state);
            let worker_reply = reply.clone();
            let worker = std::thread::spawn(move || {
                attempting_sender.send(()).unwrap();
                worker_state.complete(&worker_reply);
                finished_sender.send(()).unwrap();
            });

            attempting_receiver.recv().unwrap();
            assert!(
                finished_receiver
                    .recv_timeout(std::time::Duration::from_millis(50))
                    .is_err()
            );
            assert_eq!(transition.replies.len(), 1);
            drop(transition);

            worker.join().unwrap();
            assert!(state.inner.lock().unwrap().replies.is_empty());
        }

        #[test]
        fn reply_retirement_allows_immediate_handle_reuse() {
            let state = drain_state();
            let first = ReplyOnce::registered(10, &state);
            state.register(first.clone()).unwrap();

            assert!(first.claim());
            let inner = state.inner.lock().unwrap();
            assert!(inner.replies.is_empty());
            assert_eq!(inner.active, 1);
            drop(inner);

            let second = ReplyOnce::registered(10, &state);
            state.register(second.clone()).unwrap();

            state.complete(&first);
            let current = state
                .inner
                .lock()
                .unwrap()
                .replies
                .get(&10)
                .cloned()
                .unwrap();
            assert!(current.same_request(&second));
            state.complete(&second);
            let inner = state.inner.lock().unwrap();
            assert!(inner.replies.is_empty());
            assert_eq!(inner.active, 0);
        }

        #[test]
        fn bounded_admission_rejects_without_waiting() {
            let (sender, _receiver) = async_channel::bounded::<u8>(1);
            sender.try_send(1).unwrap();
            assert!(sender.try_send(2).is_err());
        }

        #[test]
        fn panic_is_caught_before_reply_fallback() {
            let result = std::panic::catch_unwind(|| panic!("injected task panic"));
            assert!(result.is_err());
        }
    }
}

#[cfg(all(target_os = "linux", not(benchfs_skip_native)))]
pub use linux::{run_mount, run_mount_cow};
#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn runtime_config_accepts_bounded_values() {
        let config = RuntimeConfig::new(2, 64);
        assert!(config.validate().is_ok());
        assert_eq!(config.worker_threads, 2);
        assert_eq!(config.queue_capacity, 64);
    }

    #[test]
    fn runtime_config_rejects_invalid_values() {
        for config in [
            RuntimeConfig::new(0, 64),
            RuntimeConfig::new(RuntimeConfig::MAX_WORKER_THREADS + 1, 64),
            RuntimeConfig::new(2, 0),
            RuntimeConfig::new(2, RuntimeConfig::MAX_QUEUE_CAPACITY + 1),
        ] {
            assert!(config.validate().is_err());
        }
    }
    #[test]
    fn runtime_config_default_is_bounded() {
        assert_eq!(
            RuntimeConfig::default(),
            RuntimeConfig::new(
                RuntimeConfig::DEFAULT_WORKER_THREADS,
                RuntimeConfig::DEFAULT_QUEUE_CAPACITY,
            )
        );
    }
}
