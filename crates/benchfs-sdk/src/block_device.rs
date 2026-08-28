//! Asynchronous block-device abstraction (async + `Send + Sync`).
//!
//! Three backends, all direct-IO, all async:
//!
//! | Type | Backing | Use |
//! |------|---------|-----|
//! | [`FileBlockDevice`] | Raw block device (`O_DIRECT` / `FILE_FLAG_NO_BUFFERING`) | Production TEST disk |
//! | [`FileSimBlockDevice`] | Regular file (`O_DIRECT` / `FILE_FLAG_NO_BUFFERING`) | Scratch / simulation |
//! | [`MemBlockDevice`] | Heap | Deterministic tests |
//!
//! All file-backed devices submit through compio (io_uring on Linux, IOCP on
//! Windows). Every request is copied through a 4 KiB-aligned owned buffer so
//! the kernel's direct-IO alignment requirement is met.
use async_trait::async_trait;
use compio::buf::{IoBuf, IoBufMut, IoVectoredBuf, IoVectoredBufMut, SetLen};
use compio::fs::{File, OpenOptions};
use compio::io::{AsyncReadAt, AsyncWriteAt};
use core::fmt;
use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicU64, Ordering},
};

/// Largest byte length representable by the BenchFS v1 on-disk block address.
pub const MAX_DEVICE_BYTES: u64 = (u64::MAX / crate::BLOCK_SIZE) * crate::BLOCK_SIZE;

/// Fixed 4KiB direct-I/O alignment.
pub const DIRECT_IO_ALIGNMENT: usize = crate::BLOCK_SIZE as usize;
/// Maximum scatter/gather segments accepted in one device request.
pub const MAX_DEVICE_SEGMENTS: usize = 1024;

// ---------------------------------------------------------------------------
// Owned aligned scatter/gather buffers
// ---------------------------------------------------------------------------

/// One owned 4 KiB-aligned device-I/O segment.
///
/// The segment is moved into a [`BlockDevice`] request and returned in its
/// [`DeviceCompletion`], allowing compio to submit the same allocation without
/// an intermediate bounce-buffer copy.
#[derive(Debug)]
pub struct DeviceSegment {
    ptr: NonNull<u8>,
    len: usize,
    capacity: usize,
}

impl DeviceSegment {
    /// Allocates a zero-filled aligned segment of exactly `len` bytes.
    pub fn zeroed(len: usize) -> DeviceResult<Self> {
        let layout = Layout::from_size_align(len.max(1), DIRECT_IO_ALIGNMENT)
            .map_err(|_| DeviceError::new(DeviceErrorKind::OutOfBounds))?;
        // SAFETY: layout has nonzero size and power-of-two alignment.
        let ptr = unsafe { alloc_zeroed(layout) };
        let ptr =
            NonNull::new(ptr).ok_or_else(|| DeviceError::new(DeviceErrorKind::Unavailable))?;
        Ok(Self {
            ptr,
            len,
            capacity: len,
        })
    }

    /// Allocates an aligned segment and copies `data` into it.
    ///
    /// Producers should prefer writing directly through
    /// [`Self::as_mut_slice`]. This convenience constructor performs one
    /// explicit ingress copy.
    pub fn from_slice(data: &[u8]) -> DeviceResult<Self> {
        let mut segment = Self::zeroed(data.len())?;
        segment.as_mut_slice().copy_from_slice(data);
        Ok(segment)
    }

    /// Returns initialized bytes.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: construction initializes the complete allocation and set_len
        // never exceeds capacity.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Returns initialized bytes mutably.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        // SAFETY: same ownership and initialization invariant as as_slice.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    /// Returns the initialized length.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether this segment is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the allocation capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    fn reset_len(&mut self) {
        self.len = self.capacity;
    }
}

impl Drop for DeviceSegment {
    fn drop(&mut self) {
        let Ok(layout) = Layout::from_size_align(self.capacity.max(1), DIRECT_IO_ALIGNMENT) else {
            return;
        };
        // SAFETY: ptr was allocated with exactly this layout.
        unsafe { dealloc(self.ptr.as_ptr(), layout) };
    }
}

// SAFETY: DeviceSegment uniquely owns its allocation.
unsafe impl Send for DeviceSegment {}
// SAFETY: immutable access only exposes initialized bytes.
unsafe impl Sync for DeviceSegment {}

impl IoBuf for DeviceSegment {
    fn as_init(&self) -> &[u8] {
        self.as_slice()
    }
}

impl IoBufMut for DeviceSegment {
    fn as_uninit(&mut self) -> &mut [MaybeUninit<u8>] {
        // SAFETY: the allocation covers capacity bytes.
        unsafe {
            std::slice::from_raw_parts_mut(
                self.ptr.as_ptr().cast::<MaybeUninit<u8>>(),
                self.capacity,
            )
        }
    }
}

impl SetLen for DeviceSegment {
    unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity);
        self.len = len;
    }
}

/// Owned scatter/gather request buffer with an inline single-segment fast path.
#[derive(Debug)]
pub struct DeviceIoVec {
    first: DeviceSegment,
    rest: Vec<DeviceSegment>,
    total_capacity: usize,
}

impl DeviceIoVec {
    /// Creates a single-segment request without allocating a segment vector.
    #[must_use]
    pub const fn single(segment: DeviceSegment) -> Self {
        let total_capacity = segment.capacity();
        Self {
            first: segment,
            rest: Vec::new(),
            total_capacity,
        }
    }

    /// Creates a request from one or more owned aligned segments.
    pub fn new(mut segments: Vec<DeviceSegment>) -> DeviceResult<Self> {
        if segments.is_empty() || segments.len() > MAX_DEVICE_SEGMENTS {
            return Err(DeviceError::new(DeviceErrorKind::OutOfBounds));
        }
        if segments.iter().any(DeviceSegment::is_empty) {
            return Err(DeviceError::new(DeviceErrorKind::Misaligned));
        }
        let total_capacity = segments.iter().try_fold(0usize, |total, segment| {
            total.checked_add(segment.capacity())
        });
        let total_capacity =
            total_capacity.ok_or_else(|| DeviceError::new(DeviceErrorKind::OutOfBounds))?;
        let first = segments.remove(0);
        Ok(Self {
            first,
            rest: segments,
            total_capacity,
        })
    }

    /// Allocates one zero-filled segment.
    pub fn zeroed(len: usize) -> DeviceResult<Self> {
        DeviceSegment::zeroed(len).map(Self::single)
    }

    /// Allocates one aligned segment and copies `data` into it.
    pub fn from_slice(data: &[u8]) -> DeviceResult<Self> {
        DeviceSegment::from_slice(data).map(Self::single)
    }

    /// Returns the initialized byte count across all segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments().map(DeviceSegment::len).sum()
    }

    /// Returns the total allocation capacity across all segments.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.total_capacity
    }

    /// Returns the segment count.
    #[must_use]
    pub const fn segment_count(&self) -> usize {
        1 + self.rest.len()
    }

    /// Returns immutable segments in device order.
    pub fn segments(&self) -> impl Iterator<Item = &DeviceSegment> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    /// Returns mutable segments in device order.
    pub fn segments_mut(&mut self) -> impl Iterator<Item = &mut DeviceSegment> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }

    /// Returns the sole segment, or the unchanged request when vectored.
    pub fn into_single(self) -> Result<DeviceSegment, Self> {
        if self.rest.is_empty() {
            Ok(self.first)
        } else {
            Err(self)
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn into_parts(self) -> (DeviceSegment, Vec<DeviceSegment>) {
        (self.first, self.rest)
    }

    #[cfg(not(target_os = "linux"))]
    fn from_parts(first: DeviceSegment, rest: Vec<DeviceSegment>) -> Self {
        let total_capacity =
            first.capacity() + rest.iter().map(DeviceSegment::capacity).sum::<usize>();
        Self {
            first,
            rest,
            total_capacity,
        }
    }

    /// Returns the complete initialized bytes across all segments.
    #[must_use]
    pub fn as_slice(&self) -> Vec<u8> {
        self.segments()
            .flat_map(|segment| segment.as_slice().to_vec())
            .collect()
    }

    fn reset_lengths(&mut self) {
        for segment in self.segments_mut() {
            segment.reset_len();
        }
    }
}

impl IoVectoredBuf for DeviceIoVec {
    fn iter_slice(&self) -> impl Iterator<Item = &[u8]> {
        self.segments().map(DeviceSegment::as_slice)
    }
}

impl IoVectoredBufMut for DeviceIoVec {
    fn iter_uninit_slice(&mut self) -> impl Iterator<Item = &mut [MaybeUninit<u8>]> {
        self.segments_mut().map(IoBufMut::as_uninit)
    }
}

impl SetLen for DeviceIoVec {
    unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.total_capacity);
        let mut remaining = len;
        for segment in self.segments_mut() {
            let segment_len = remaining.min(segment.capacity());
            // SAFETY: segment_len never exceeds this segment's capacity.
            unsafe { segment.set_len(segment_len) };
            remaining -= segment_len;
        }
        debug_assert_eq!(remaining, 0);
    }
}

// ---------------------------------------------------------------------------
// Platform-specific direct-IO open helper
// ---------------------------------------------------------------------------

/// Opens `path` read/write with direct-IO flags for the current platform.
///
/// - **Unix**: `O_DIRECT`
/// - **Windows**: `FILE_FLAG_NO_BUFFERING`
async fn open_direct(path: &Path) -> std::io::Result<File> {
    let mut opts = OpenOptions::new();
    opts.read(true).write(true);
    #[cfg(unix)]
    opts.custom_flags(libc::O_DIRECT);
    #[cfg(windows)]
    opts.custom_flags(windows_sys::Win32::Storage::FileSystem::FILE_FLAG_NO_BUFFERING);
    opts.open(path).await
}
// ---------------------------------------------------------------------------
// Per-thread file caches — one open fd per backend instance per thread
// ---------------------------------------------------------------------------
//
// compio is thread-per-core: each worker runs its own io_uring/IOCP instance,
// and `File` is `!Send`. Each backend receives a process-unique identity so
// replacing a file at the same path cannot reuse the previous instance's open
// handle. Dead instances are pruned lazily through a weak lifetime token.

static NEXT_CACHE_ID: AtomicU64 = AtomicU64::new(1);

struct CacheEntry {
    lifetime: Weak<()>,
    file: File,
}

fn new_cache_identity() -> DeviceResult<(u64, Arc<()>)> {
    let id = NEXT_CACHE_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .map_err(|_| DeviceError::new(DeviceErrorKind::Unavailable))?;
    Ok((id, Arc::new(())))
}

async fn cached_direct_open(
    cache: &'static std::thread::LocalKey<RefCell<HashMap<u64, CacheEntry>>>,
    id: u64,
    lifetime: &Arc<()>,
    path: &Path,
) -> std::io::Result<File> {
    if let Some(file) = cache.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.retain(|_, entry| entry.lifetime.strong_count() > 0);
        cache.get(&id).map(|entry| entry.file.clone())
    }) {
        return Ok(file);
    }
    let file = open_direct(path).await?;
    cache.with(|cache| {
        cache.borrow_mut().insert(
            id,
            CacheEntry {
                lifetime: Arc::downgrade(lifetime),
                file: file.clone(),
            },
        );
    });
    Ok(file)
}

thread_local! {
    static BLOCK_DEV_CACHE: RefCell<HashMap<u64, CacheEntry>> = RefCell::new(HashMap::new());
    static SIM_DEV_CACHE: RefCell<HashMap<u64, CacheEntry>> = RefCell::new(HashMap::new());
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Categories exposed by the injected block-device fault model.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeviceErrorKind {
    /// The requested byte range is outside the device.
    OutOfBounds,
    /// A request violates device-imposed alignment.
    Misaligned,
    /// An exact read could not be completed.
    Read,
    /// An exact write could not be completed.
    Write,
    /// A stable-storage flush failed.
    Flush,
    /// The controller or backing device is unavailable.
    Unavailable,
    /// Discard/trim is not supported by this device.
    Unsupported,
}

/// A block-device failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeviceError {
    kind: DeviceErrorKind,
}

impl DeviceError {
    /// Creates a device error of `kind`.
    #[must_use]
    pub const fn new(kind: DeviceErrorKind) -> Self {
        Self { kind }
    }
    /// Returns the stable fault category.
    #[must_use]
    pub const fn kind(self) -> DeviceErrorKind {
        self.kind
    }
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block device {:?} failure", self.kind)
    }
}

impl std::error::Error for DeviceError {}

impl From<DeviceError> for crate::FsError {
    fn from(_: DeviceError) -> Self {
        crate::Errno::Io.into()
    }
}

/// Result returned by block-device operations.
pub type DeviceResult<T> = Result<T, DeviceError>;

/// Completion of one owned-buffer device request.
#[must_use]
#[derive(Debug)]
pub struct DeviceCompletion {
    /// Exact-transfer status.
    pub result: DeviceResult<()>,
    /// The same owned scatter/gather buffer supplied by the caller.
    pub buffer: DeviceIoVec,
}

impl DeviceCompletion {
    /// Creates a completion from a status and the returned buffer.
    pub fn new(result: DeviceResult<()>, buffer: DeviceIoVec) -> Self {
        Self { result, buffer }
    }

    /// Returns the buffer on success, or the error together with the buffer.
    pub fn into_result(self) -> Result<DeviceIoVec, (DeviceError, DeviceIoVec)> {
        match self.result {
            Ok(()) => Ok(self.buffer),
            Err(error) => Err((error, self.buffer)),
        }
    }
}

// ---------------------------------------------------------------------------
// DeviceRange
// ---------------------------------------------------------------------------

/// A checked half-open byte range on a particular device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeviceRange {
    offset: u64,
    length: u64,
}

impl DeviceRange {
    /// Validates that `offset..offset+length` is representable and in bounds.
    pub fn new(offset: u64, length: u64, device_size: u64) -> DeviceResult<Self> {
        let end = offset
            .checked_add(length)
            .ok_or_else(|| DeviceError::new(DeviceErrorKind::OutOfBounds))?;
        if device_size > MAX_DEVICE_BYTES || end > device_size {
            return Err(DeviceError::new(DeviceErrorKind::OutOfBounds));
        }
        Ok(Self { offset, length })
    }

    /// First byte of the range.
    #[must_use]
    pub const fn offset(self) -> u64 {
        self.offset
    }
    /// Number of bytes in the range.
    #[must_use]
    pub const fn length(self) -> u64 {
        self.length
    }
    /// Exclusive end of this already-validated range.
    #[must_use]
    pub const fn end(self) -> u64 {
        self.offset + self.length
    }
    /// True if the range lies entirely within `device_size`.
    #[must_use]
    pub fn in_bounds(self, device_size: u64) -> bool {
        self.offset
            .checked_add(self.length)
            .is_some_and(|end| device_size <= MAX_DEVICE_BYTES && end <= device_size)
    }
}

// ---------------------------------------------------------------------------
// BlockDevice trait
// ---------------------------------------------------------------------------

/// Exclusive storage capability supplied to a mounted BenchFS instance.
///
/// Reads and writes transfer ownership of an aligned scatter/gather buffer and
/// return that same buffer at completion. File backends submit its allocations
/// directly to completion I/O; they never allocate or copy a bounce buffer.
///
/// Requests are exact: `completion.result == Ok(())` means every byte was
/// transferred. Multiple calls may be driven concurrently. The device may
/// reorder them as real hardware does; only `flush` provides a durable ordering
/// boundary.
#[async_trait(?Send)]
pub trait BlockDevice: Send + Sync + 'static {
    /// Returns the immutable byte length of the device.
    fn size(&self) -> DeviceResult<u64>;

    /// Reads exactly `buffer.capacity()` bytes starting at `offset`.
    async fn read_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion;

    /// Writes exactly `buffer.len()` bytes starting at `offset`. Completion
    /// means the device accepted the write; durability requires `flush`.
    async fn write_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion;

    /// Makes all preceding accepted writes durable.
    async fn flush(&self) -> DeviceResult<()>;

    /// Deallocates/holes `len` bytes starting at `offset` (trim/discard).
    async fn discard(&self, _offset: u64, _len: u64) -> DeviceResult<()> {
        Err(DeviceError::new(DeviceErrorKind::Unsupported))
    }
}

// ---------------------------------------------------------------------------
// Shared direct-IO helpers (used by both file backends)
// ---------------------------------------------------------------------------

fn validate_direct_request(
    offset: u64,
    buffer: &DeviceIoVec,
    device_size: u64,
) -> DeviceResult<usize> {
    let len = buffer.len();
    DeviceRange::new(offset, len as u64, device_size)?;
    if offset % DIRECT_IO_ALIGNMENT as u64 != 0
        || buffer
            .segments()
            .any(|segment| segment.len() % DIRECT_IO_ALIGNMENT != 0)
    {
        return Err(DeviceError::new(DeviceErrorKind::Misaligned));
    }
    Ok(len)
}

async fn direct_read(
    file: &File,
    offset: u64,
    mut buffer: DeviceIoVec,
    device_size: u64,
) -> DeviceCompletion {
    buffer.reset_lengths();
    let expected = match validate_direct_request(offset, &buffer, device_size) {
        Ok(expected) => expected,
        Err(error) => return DeviceCompletion::new(Err(error), buffer),
    };

    if buffer.segment_count() == 1 {
        let segment = buffer.into_single().expect("single segment checked");
        let compio::buf::BufResult(result, segment) = file.read_at(segment, offset).await;
        let result = result
            .map_err(|_| DeviceError::new(DeviceErrorKind::Read))
            .and_then(|read| {
                if read == expected {
                    Ok(())
                } else {
                    Err(DeviceError::new(DeviceErrorKind::Read))
                }
            });
        return DeviceCompletion::new(result, DeviceIoVec::single(segment));
    }

    #[cfg(target_os = "linux")]
    {
        let compio::buf::BufResult(result, buffer) = file.read_vectored_at(buffer, offset).await;
        let result = result
            .map_err(|_| DeviceError::new(DeviceErrorKind::Read))
            .and_then(|read| {
                if read == expected {
                    Ok(())
                } else {
                    Err(DeviceError::new(DeviceErrorKind::Read))
                }
            });
        DeviceCompletion::new(result, buffer)
    }

    #[cfg(not(target_os = "linux"))]
    {
        direct_read_segments(file, offset, buffer).await
    }
}

async fn direct_write(
    file: &File,
    offset: u64,
    buffer: DeviceIoVec,
    device_size: u64,
) -> DeviceCompletion {
    let expected = match validate_direct_request(offset, &buffer, device_size) {
        Ok(expected) => expected,
        Err(error) => return DeviceCompletion::new(Err(error), buffer),
    };

    if buffer.segment_count() == 1 {
        let segment = buffer.into_single().expect("single segment checked");
        let mut file = file;
        let compio::buf::BufResult(result, segment) = file.write_at(segment, offset).await;
        let result = result
            .map_err(|_| DeviceError::new(DeviceErrorKind::Write))
            .and_then(|written| {
                if written == expected {
                    Ok(())
                } else {
                    Err(DeviceError::new(DeviceErrorKind::Write))
                }
            });
        return DeviceCompletion::new(result, DeviceIoVec::single(segment));
    }

    #[cfg(target_os = "linux")]
    {
        let mut file = file;
        let compio::buf::BufResult(result, buffer) = file.write_vectored_at(buffer, offset).await;
        let result = result
            .map_err(|_| DeviceError::new(DeviceErrorKind::Write))
            .and_then(|written| {
                if written == expected {
                    Ok(())
                } else {
                    Err(DeviceError::new(DeviceErrorKind::Write))
                }
            });
        DeviceCompletion::new(result, buffer)
    }

    #[cfg(not(target_os = "linux"))]
    {
        direct_write_segments(file, offset, buffer).await
    }
}

#[cfg(not(target_os = "linux"))]
async fn direct_read_segments(file: &File, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
    let (first, rest) = buffer.into_parts();
    let mut pending = std::collections::VecDeque::from(rest);
    pending.push_front(first);
    let mut completed = Vec::with_capacity(pending.len());
    let mut current_offset = offset;
    let mut result = Ok(());
    while let Some(segment) = pending.pop_front() {
        let expected = segment.len();
        let compio::buf::BufResult(io_result, segment) =
            file.read_at(segment, current_offset).await;
        current_offset += expected as u64;
        completed.push(segment);
        match io_result {
            Ok(read) if read == expected => {}
            _ => {
                result = Err(DeviceError::new(DeviceErrorKind::Read));
                completed.extend(pending);
                break;
            }
        }
    }
    let first = completed.remove(0);
    DeviceCompletion::new(result, DeviceIoVec::from_parts(first, completed))
}

#[cfg(not(target_os = "linux"))]
async fn direct_write_segments(file: &File, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
    let (first, rest) = buffer.into_parts();
    let mut pending = std::collections::VecDeque::from(rest);
    pending.push_front(first);
    let mut completed = Vec::with_capacity(pending.len());
    let mut current_offset = offset;
    let mut result = Ok(());
    while let Some(segment) = pending.pop_front() {
        let expected = segment.len();
        let mut file = file;
        let compio::buf::BufResult(io_result, segment) =
            file.write_at(segment, current_offset).await;
        current_offset += expected as u64;
        completed.push(segment);
        match io_result {
            Ok(written) if written == expected => {}
            _ => {
                result = Err(DeviceError::new(DeviceErrorKind::Write));
                completed.extend(pending);
                break;
            }
        }
    }
    let first = completed.remove(0);
    DeviceCompletion::new(result, DeviceIoVec::from_parts(first, completed))
}

async fn direct_flush(file: &File) -> DeviceResult<()> {
    file.sync_all()
        .await
        .map_err(|_| DeviceError::new(DeviceErrorKind::Flush))
}

// ---------------------------------------------------------------------------
// FileBlockDevice — raw block device, direct IO
// ---------------------------------------------------------------------------

/// A block device backed by a raw block device with direct IO.
///
/// Linux: `O_DIRECT` via compio io_uring. Windows: `FILE_FLAG_NO_BUFFERING`
/// via compio IOCP. All requests must be 4 KiB-aligned.
pub struct FileBlockDevice {
    path: PathBuf,
    size: u64,
    cache_id: u64,
    cache_lifetime: Arc<()>,
}

impl FileBlockDevice {
    /// Opens `path` with direct IO and an explicit logical byte size.
    pub async fn open(path: impl AsRef<Path>, size: u64) -> DeviceResult<Self> {
        if size == 0 || size > MAX_DEVICE_BYTES {
            return Err(DeviceError::new(DeviceErrorKind::OutOfBounds));
        }
        let path = path.as_ref().to_path_buf();
        // Validate the path is openable before returning.
        open_direct(&path)
            .await
            .map_err(|_| DeviceError::new(DeviceErrorKind::Unavailable))?;
        let (cache_id, cache_lifetime) = new_cache_identity()?;
        Ok(Self {
            path,
            size,
            cache_id,
            cache_lifetime,
        })
    }

    /// Returns the fixed direct-I/O alignment in bytes.
    #[must_use]
    pub const fn alignment(&self) -> usize {
        DIRECT_IO_ALIGNMENT
    }

    /// Opens the TEST device selected by `BENCHFS_DEVICE` / `BENCHFS_DEVICE_SIZE`.
    pub async fn open_device_from_env() -> DeviceResult<Self> {
        let path = std::env::var_os("BENCHFS_DEVICE")
            .ok_or_else(|| DeviceError::new(DeviceErrorKind::Unavailable))?;
        let size = std::env::var("BENCHFS_DEVICE_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| DeviceError::new(DeviceErrorKind::Unavailable))?;
        Self::open(path, size).await
    }
}

#[async_trait(?Send)]
impl BlockDevice for FileBlockDevice {
    fn size(&self) -> DeviceResult<u64> {
        Ok(self.size)
    }

    async fn read_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
        let file = match cached_direct_open(
            &BLOCK_DEV_CACHE,
            self.cache_id,
            &self.cache_lifetime,
            &self.path,
        )
        .await
        {
            Ok(file) => file,
            Err(_) => {
                return DeviceCompletion::new(Err(DeviceError::new(DeviceErrorKind::Read)), buffer);
            }
        };
        direct_read(&file, offset, buffer, self.size).await
    }

    async fn write_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
        let file = match cached_direct_open(
            &BLOCK_DEV_CACHE,
            self.cache_id,
            &self.cache_lifetime,
            &self.path,
        )
        .await
        {
            Ok(file) => file,
            Err(_) => {
                return DeviceCompletion::new(
                    Err(DeviceError::new(DeviceErrorKind::Write)),
                    buffer,
                );
            }
        };
        direct_write(&file, offset, buffer, self.size).await
    }

    async fn flush(&self) -> DeviceResult<()> {
        let file = cached_direct_open(
            &BLOCK_DEV_CACHE,
            self.cache_id,
            &self.cache_lifetime,
            &self.path,
        )
        .await
        .map_err(|_| DeviceError::new(DeviceErrorKind::Flush))?;
        direct_flush(&file).await
    }
}

// ---------------------------------------------------------------------------
// FileSimBlockDevice — regular file, direct IO
// ---------------------------------------------------------------------------

/// A regular-file simulated block device using direct IO.
///
/// Same direct-IO path as [`FileBlockDevice`] but opens a regular file and
/// derives the device size from the file's current length.
pub struct FileSimBlockDevice {
    path: PathBuf,
    size: u64,
    cache_id: u64,
    cache_lifetime: Arc<()>,
}

impl FileSimBlockDevice {
    /// Opens a regular file with direct IO, using its current length as size.
    pub async fn open(path: impl AsRef<Path>) -> DeviceResult<Self> {
        let path = path.as_ref().to_path_buf();
        let file = open_direct(&path)
            .await
            .map_err(|_| DeviceError::new(DeviceErrorKind::Unavailable))?;
        let size = file
            .metadata()
            .await
            .map_err(|_| DeviceError::new(DeviceErrorKind::Unavailable))?
            .len();
        let (cache_id, cache_lifetime) = new_cache_identity()?;
        Ok(Self {
            path,
            size,
            cache_id,
            cache_lifetime,
        })
    }
}

#[async_trait(?Send)]
impl BlockDevice for FileSimBlockDevice {
    fn size(&self) -> DeviceResult<u64> {
        Ok(self.size)
    }

    async fn read_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
        let file = match cached_direct_open(
            &SIM_DEV_CACHE,
            self.cache_id,
            &self.cache_lifetime,
            &self.path,
        )
        .await
        {
            Ok(file) => file,
            Err(_) => {
                return DeviceCompletion::new(Err(DeviceError::new(DeviceErrorKind::Read)), buffer);
            }
        };
        direct_read(&file, offset, buffer, self.size).await
    }

    async fn write_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
        let file = match cached_direct_open(
            &SIM_DEV_CACHE,
            self.cache_id,
            &self.cache_lifetime,
            &self.path,
        )
        .await
        {
            Ok(file) => file,
            Err(_) => {
                return DeviceCompletion::new(
                    Err(DeviceError::new(DeviceErrorKind::Write)),
                    buffer,
                );
            }
        };
        direct_write(&file, offset, buffer, self.size).await
    }

    async fn flush(&self) -> DeviceResult<()> {
        let file = cached_direct_open(
            &SIM_DEV_CACHE,
            self.cache_id,
            &self.cache_lifetime,
            &self.path,
        )
        .await
        .map_err(|_| DeviceError::new(DeviceErrorKind::Flush))?;
        direct_flush(&file).await
    }
}

// ---------------------------------------------------------------------------
// MemBlockDevice — in-memory, deterministic
// ---------------------------------------------------------------------------

/// An in-memory block device for deterministic filesystem tests.
pub struct MemBlockDevice {
    data: Mutex<Vec<u8>>,
}

impl MemBlockDevice {
    /// Creates a zero-filled device of `size` bytes.
    pub fn new(size: u64) -> DeviceResult<Self> {
        let size =
            usize::try_from(size).map_err(|_| DeviceError::new(DeviceErrorKind::OutOfBounds))?;
        Ok(Self {
            data: Mutex::new(vec![0; size]),
        })
    }
}

#[async_trait(?Send)]
impl BlockDevice for MemBlockDevice {
    fn size(&self) -> DeviceResult<u64> {
        self.data
            .lock()
            .map(|d| d.len() as u64)
            .map_err(|_| DeviceError::new(DeviceErrorKind::Unavailable))
    }

    async fn read_at(&self, offset: u64, mut buffer: DeviceIoVec) -> DeviceCompletion {
        buffer.reset_lengths();
        let data = match self.data.lock() {
            Ok(data) => data,
            Err(_) => {
                return DeviceCompletion::new(
                    Err(DeviceError::new(DeviceErrorKind::Unavailable)),
                    buffer,
                );
            }
        };
        if let Err(error) = DeviceRange::new(offset, buffer.len() as u64, data.len() as u64) {
            return DeviceCompletion::new(Err(error), buffer);
        }
        let mut source_offset = offset as usize;
        for segment in buffer.segments_mut() {
            let end = source_offset + segment.len();
            segment
                .as_mut_slice()
                .copy_from_slice(&data[source_offset..end]);
            source_offset = end;
        }
        DeviceCompletion::new(Ok(()), buffer)
    }

    async fn write_at(&self, offset: u64, buffer: DeviceIoVec) -> DeviceCompletion {
        let mut data = match self.data.lock() {
            Ok(data) => data,
            Err(_) => {
                return DeviceCompletion::new(
                    Err(DeviceError::new(DeviceErrorKind::Unavailable)),
                    buffer,
                );
            }
        };
        if let Err(error) = DeviceRange::new(offset, buffer.len() as u64, data.len() as u64) {
            return DeviceCompletion::new(Err(error), buffer);
        }
        let mut destination_offset = offset as usize;
        for segment in buffer.segments() {
            let end = destination_offset + segment.len();
            data[destination_offset..end].copy_from_slice(segment.as_slice());
            destination_offset = end;
        }
        DeviceCompletion::new(Ok(()), buffer)
    }

    async fn flush(&self) -> DeviceResult<()> {
        Ok(())
    }

    async fn discard(&self, offset: u64, len: u64) -> DeviceResult<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|_| DeviceError::new(DeviceErrorKind::Unavailable))?;
        DeviceRange::new(offset, len, data.len() as u64)?;
        data[offset as usize..offset as usize + len as usize].fill(0);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_bounds_checked() {
        assert!(DeviceRange::new(0, 512, 4096).is_ok());
        assert!(DeviceRange::new(4096, 1, 4096).is_err());
        assert!(DeviceRange::new(0, 512, 4097).is_ok());
        assert!(DeviceRange::new(u64::MAX - 1, 1, MAX_DEVICE_BYTES).is_err());
    }

    #[test]
    fn direct_io_alignment_is_filesystem_block() {
        assert_eq!(DIRECT_IO_ALIGNMENT, 4096);
    }

    #[test]
    fn mem_device_reads_writes_and_discards() {
        let device = MemBlockDevice::new(4096).unwrap();
        futures::executor::block_on(async {
            let buffer = DeviceIoVec::from_slice(b"benchfs").unwrap();
            device.write_at(128, buffer).await.result.unwrap();
            let buffer = DeviceIoVec::zeroed(7).unwrap();
            let completion = device.read_at(128, buffer).await;
            completion.result.unwrap();
            assert_eq!(completion.buffer.as_slice(), b"benchfs");
            device.discard(128, 7).await.unwrap();
            let buffer = DeviceIoVec::zeroed(7).unwrap();
            let completion = device.read_at(128, buffer).await;
            completion.result.unwrap();
            assert_eq!(completion.buffer.as_slice(), [0; 7]);
        });
    }

    #[test]
    fn file_sim_device_reads_writes_and_flushes() {
        let path =
            std::env::temp_dir().join(format!("benchfs-sdk-sim-test-{}", std::process::id()));
        // Pre-size the file to 4096 so O_DIRECT alignment is satisfied.
        std::fs::File::create(&path).unwrap().set_len(4096).unwrap();
        let runtime = compio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let device = FileSimBlockDevice::open(&path).await.unwrap();
            let mut buffer = DeviceIoVec::zeroed(4096).unwrap();
            buffer
                .segments_mut()
                .next()
                .unwrap()
                .as_mut_slice()
                .fill(0xab);
            device.write_at(0, buffer).await.result.unwrap();
            device.flush().await.unwrap();
            let completion = device.read_at(0, DeviceIoVec::zeroed(4096).unwrap()).await;
            completion.result.unwrap();
            assert!(
                completion
                    .buffer
                    .segments()
                    .all(|s| s.as_slice() == [0xab; 4096])
            );
        });
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn file_block_device_direct_io_round_trip() {
        let path =
            std::env::temp_dir().join(format!("benchfs-sdk-direct-test-{}", std::process::id()));
        std::fs::File::create(&path).unwrap().set_len(4096).unwrap();
        let runtime = compio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let device = FileBlockDevice::open(&path, 4096).await.unwrap();
            let mut buffer = DeviceIoVec::zeroed(4096).unwrap();
            buffer
                .segments_mut()
                .next()
                .unwrap()
                .as_mut_slice()
                .fill(0x5a);
            device.write_at(0, buffer).await.result.unwrap();
            device.flush().await.unwrap();
            let completion = device.read_at(0, DeviceIoVec::zeroed(4096).unwrap()).await;
            completion.result.unwrap();
            assert!(
                completion
                    .buffer
                    .segments()
                    .all(|s| s.as_slice() == [0x5a; 4096])
            );
        });
        std::fs::remove_file(path).unwrap();
    }
}
