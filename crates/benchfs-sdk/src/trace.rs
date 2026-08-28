use crate::{DeviceErrorKind, Errno, FsError, FsResult, MAX_DEVICE_BYTES};

/// Identity of one in-flight BlockDevice request inside an authoritative trace.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequestId(u64);

impl RequestId {
    /// Creates the identity assigned by the recording device to one request.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

/// Identity of the thread or task that issued a traced request.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskId(u64);

impl TaskId {
    /// Creates the identity assigned by the recording device to one thread/task.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

/// Operation kind of one traced BlockDevice request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestKind {
    /// An exact read request covering `offset..offset+length`.
    Read,
    /// An exact write request whose payload must be recorded losslessly.
    Write,
    /// A stable-storage flush request carrying no byte range.
    Flush,
}

/// Lifecycle phase of one traced request event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EventPhase {
    /// The device accepted and started the request.
    Start,
    /// The request finished with a stable outcome.
    Complete,
}

/// Write payload recorded so a constructed crash image can be rebuilt losslessly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WritePayload {
    /// Inline bytes for small writes.
    Inline(Vec<u8>),
    /// Content-addressed reference to an externally stored copy.
    Reference(WritePayloadRef),
}

/// Content-addressed reference that must losslessly reconstruct one write payload.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WritePayloadRef {
    sha256: [u8; 32],
    length: u64,
}

impl WritePayloadRef {
    /// Creates a reference whose store must yield exactly `length` bytes hashing to `sha256`.
    #[must_use]
    pub const fn new(sha256: [u8; 32], length: u64) -> Self {
        Self { sha256, length }
    }

    /// Returns the SHA-256 digest of the referenced payload.
    #[must_use]
    pub const fn sha256(self) -> [u8; 32] {
        self.sha256
    }

    /// Returns the byte length of the referenced payload.
    #[must_use]
    pub const fn length(self) -> u64 {
        self.length
    }
}

/// One authoritative BlockDevice trace event.
///
/// `benchmark-requirements.md` section 4.5 freezes this record: a monotonic
/// event sequence, request ID, operation kind (read/write/flush), phase
/// (start/complete), offset, length, result, thread/task identity, and the
/// write payload or a lossless content-addressed reference for writes. A
/// successful flush complete only guarantees writes that completed before its
/// start; in-flight requests at a crash point are resolved by the frozen
/// sector-persistence policy under 512-byte power-fail atomicity. The
/// recording virtual BlockDevice and deterministic replay constructor are
/// Phase 4 implementations of this vocabulary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestEvent {
    sequence: u64,
    request_id: RequestId,
    kind: RequestKind,
    phase: EventPhase,
    offset: u64,
    length: u64,
    task: TaskId,
    result: Option<Result<(), DeviceErrorKind>>,
    payload: Option<WritePayload>,
}

impl RequestEvent {
    /// Validates the frozen per-event invariants and creates an event.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sequence: u64,
        request_id: RequestId,
        kind: RequestKind,
        phase: EventPhase,
        offset: u64,
        length: u64,
        task: TaskId,
        result: Option<Result<(), DeviceErrorKind>>,
        payload: Option<WritePayload>,
    ) -> FsResult<Self> {
        match (phase, result.is_some()) {
            (EventPhase::Start, true) | (EventPhase::Complete, false) => {
                return Err(Errno::InvalidArgument.into());
            }
            _ => {}
        }
        if kind == RequestKind::Flush && (offset != 0 || length != 0) {
            return Err(Errno::InvalidArgument.into());
        }
        if payload.is_some() && kind != RequestKind::Write {
            return Err(Errno::InvalidArgument.into());
        }
        let end = offset
            .checked_add(length)
            .ok_or_else(|| FsError::new(Errno::Range))?;
        if end > MAX_DEVICE_BYTES {
            return Err(FsError::new(Errno::Range));
        }
        Ok(Self {
            sequence,
            request_id,
            kind,
            phase,
            offset,
            length,
            task,
            result,
            payload,
        })
    }

    /// Returns the monotonic event sequence within one trace.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the identity of the traced request.
    #[must_use]
    pub const fn request_id(&self) -> RequestId {
        self.request_id
    }

    /// Returns the operation kind.
    #[must_use]
    pub const fn kind(&self) -> RequestKind {
        self.kind
    }

    /// Returns the lifecycle phase.
    #[must_use]
    pub const fn phase(&self) -> EventPhase {
        self.phase
    }

    /// Returns the first byte of the request range; zero for flush events.
    #[must_use]
    pub const fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns the byte length of the request range; zero for flush events.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// Returns the identity of the issuing thread/task.
    #[must_use]
    pub const fn task(&self) -> TaskId {
        self.task
    }

    /// Returns the stable outcome: `None` on start events, `Some` on complete events.
    #[must_use]
    pub const fn result(&self) -> Option<Result<(), DeviceErrorKind>> {
        self.result
    }

    /// Returns the recorded write payload for a write request, if any.
    #[must_use]
    pub fn payload(&self) -> Option<&WritePayload> {
        self.payload.as_ref()
    }
}

/// Power-loss instant defined between two consecutive trace events.
///
/// A crash point applies every event with a sequence at or before its last
/// recorded sequence and no later event. A request whose start is recorded but
/// whose complete is not is in flight at the crash: flush coverage and the
/// frozen sector-persistence policy decide which of its bytes survive, subject
/// to 512-byte power-fail atomicity. Recording only "the Nth write/flush call"
/// cannot express concurrency or flush coverage, so a point must sit between
/// request events rather than between calls.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CrashPoint(Option<u64>);

impl CrashPoint {
    /// Power loss before any traced event: the pristine initial image.
    #[must_use]
    pub const fn before_all() -> Self {
        Self(None)
    }

    /// Power loss immediately after the event with `sequence` was recorded.
    #[must_use]
    pub const fn after(sequence: u64) -> Self {
        Self(Some(sequence))
    }

    /// Returns the sequence of the last fully recorded event, if any.
    #[must_use]
    pub const fn last_recorded_sequence(self) -> Option<u64> {
        self.0
    }
}
