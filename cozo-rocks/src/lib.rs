#[cxx::bridge]
mod ffi {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Status {
        code: StatusCode,
        subcode: StatusSubCode,
        severity: StatusSeverity,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum StatusCode {
        kOk = 0,
        kNotFound = 1,
        kCorruption = 2,
        kNotSupported = 3,
        kInvalidArgument = 4,
        kIOError = 5,
        kMergeInProgress = 6,
        kIncomplete = 7,
        kShutdownInProgress = 8,
        kTimedOut = 9,
        kAborted = 10,
        kBusy = 11,
        kExpired = 12,
        kTryAgain = 13,
        kCompactionTooLarge = 14,
        kColumnFamilyDropped = 15,
        kMaxCode,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum StatusSubCode {
        kNone = 0,
        kMutexTimeout = 1,
        kLockTimeout = 2,
        kLockLimit = 3,
        kNoSpace = 4,
        kDeadlock = 5,
        kStaleFile = 6,
        kMemoryLimit = 7,
        kSpaceLimit = 8,
        kPathNotFound = 9,
        KMergeOperandsInsufficientCapacity = 10,
        kManualCompactionPaused = 11,
        kOverwritten = 12,
        kTxnNotPrepared = 13,
        kIOFenced = 14,
        kMaxSubCode,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum StatusSeverity {
        kNoError = 0,
        kSoftError = 1,
        kHardError = 2,
        kFatalError = 3,
        kUnrecoverableError = 4,
        kMaxSeverity,
    }

    unsafe extern "C++" {
        include!("cozorocks.h");

        type StatusCode;
        type StatusSubCode;
        type StatusSeverity;

        type PinnableSliceBridge;
        fn as_bytes(self: &PinnableSliceBridge) -> &[u8];

        type SliceBridge;
        fn as_bytes(self: &SliceBridge) -> &[u8];

        type ReadOptionsBridge;
        fn new_read_options() -> UniquePtr<ReadOptionsBridge>;
        fn set_verify_checksums(self: &ReadOptionsBridge, v: bool);
        fn set_total_order_seek(self: &ReadOptionsBridge, v: bool);

        type WriteOptionsBridge;
        fn new_write_options() -> UniquePtr<WriteOptionsBridge>;
        fn set_disable_wal(self: &WriteOptionsBridge, v: bool);

        type OptionsBridge;
        fn new_options() -> UniquePtr<OptionsBridge>;
        fn prepare_for_bulk_load(self: &OptionsBridge);
        fn increase_parallelism(self: &OptionsBridge);
        fn optimize_level_style_compaction(self: &OptionsBridge);
        fn set_create_if_missing(self: &OptionsBridge, v: bool);
        fn set_comparator(self: &OptionsBridge, name: &str, compare: fn(&[u8], &[u8]) -> i8);

        type DBBridge;
        fn list_column_families(options: &OptionsBridge, path: &[u8]) -> UniquePtr<CxxVector<CxxString>>;
        fn open_db(options: &OptionsBridge, path: &[u8], status: &mut Status) -> UniquePtr<DBBridge>;
        fn cf_names(self: &DBBridge) -> UniquePtr<CxxVector<CxxString>>;
        fn put(self: &DBBridge, options: &WriteOptionsBridge, cf_id: usize, key: &[u8], val: &[u8], status: &mut Status);
        fn get(self: &DBBridge, options: &ReadOptionsBridge, cf_id: usize, key: &[u8], status: &mut Status) -> UniquePtr<PinnableSliceBridge>;
        fn write_batch(self: &DBBridge) -> UniquePtr<WriteBatchBridge>;
        fn iterator(self: &DBBridge, options: &ReadOptionsBridge, cf_id: usize) -> UniquePtr<IteratorBridge>;

        type WriteBatchBridge;

        type IteratorBridge;
        fn seek_to_first(self: &IteratorBridge);
        fn seek_to_last(self: &IteratorBridge);
        fn next(self: &IteratorBridge);
        fn is_valid(self: &IteratorBridge) -> bool;
        fn seek(self: &IteratorBridge, key: &[u8]);
        fn seek_for_prev(self: &IteratorBridge, key: &[u8]);
        fn key(self: &IteratorBridge) -> UniquePtr<SliceBridge>;
        fn value(self: &IteratorBridge) -> UniquePtr<SliceBridge>;
        fn status(self: &IteratorBridge) -> Status;
    }
}


use std::collections::BTreeMap;
use cxx::UniquePtr;
pub use ffi::*;

pub struct Options {
    bridge: UniquePtr<OptionsBridge>,
}

impl Options {
    #[inline]
    pub fn prepare_for_bulk_load(self) -> Self {
        self.bridge.prepare_for_bulk_load();
        self
    }

    #[inline]
    pub fn increase_parallelism(self) -> Self {
        self.bridge.increase_parallelism();
        self
    }

    #[inline]
    pub fn optimize_level_style_compaction(self) -> Self {
        self.bridge.optimize_level_style_compaction();
        self
    }

    #[inline]
    pub fn set_create_if_missing(self, v: bool) -> Self {
        self.bridge.set_create_if_missing(v);
        self
    }

    #[inline]
    pub fn set_comparator(self, name: &str, compare: fn(&[u8], &[u8]) -> i8) -> Self {
        self.bridge.set_comparator(name, compare);
        self
    }
}

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Self { bridge: new_options() }
    }
}

pub struct ReadOptions {
    bridge: UniquePtr<ReadOptionsBridge>,
}

impl ReadOptions {
    pub fn set_total_order_seek(self, v: bool) -> Self {
        self.bridge.set_total_order_seek(v);
        self
    }
    pub fn set_verify_checksums(self, v: bool) -> Self {
        self.bridge.set_total_order_seek(v);
        self
    }
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self { bridge: new_read_options() }
    }
}

pub struct WriteOptions {
    bridge: UniquePtr<WriteOptionsBridge>,
}

impl WriteOptions {
    #[inline]
    pub fn set_disable_wal(&self, v: bool) {
        self.bridge.set_disable_wal(v);
    }
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self { bridge: new_write_options() }
    }
}

pub type PinnableSlice = UniquePtr<PinnableSliceBridge>;
pub type Slice = UniquePtr<SliceBridge>;
pub type Iterator = UniquePtr<IteratorBridge>;

pub struct DB {
    bridge: UniquePtr<DBBridge>,
    pub options: Options,
    pub default_read_options: ReadOptions,
    pub default_write_options: WriteOptions,
    pub column_families: BTreeMap<String, usize>,
}

fn get_path_bytes(path: &std::path::Path) -> &[u8] {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes()
    }

    #[cfg(not(unix))]
    { path.to_string_lossy().to_string().as_bytes() }
}

impl DB {
    #[inline]
    pub fn list_column_families(options: &Options, path: impl AsRef<std::path::Path>) -> Vec<String> {
        let results = list_column_families(&options.bridge, get_path_bytes(path.as_ref()));
        results.iter().map(|s| s.to_string_lossy().into_owned()).collect()
    }

    #[inline]
    pub fn open(options: Options, path: impl AsRef<std::path::Path>) -> Result<Self, Status> {
        let mut status = Status::default();
        let bridge = open_db(
            &options.bridge,
            get_path_bytes(path.as_ref()),
            &mut status,
        );

        if status.code == StatusCode::kOk {
            let column_families = bridge.cf_names().iter().enumerate().map(|(i, v)| (v.to_string_lossy().into_owned(), i)).collect();
            Ok(Self {
                bridge,
                default_read_options: ReadOptions::default(),
                default_write_options: WriteOptions::default(),
                options,
                column_families,
            })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn put(&self, key: impl AsRef<[u8]>, val: impl AsRef<[u8]>, cf: usize, options: Option<&WriteOptions>) -> Result<Status, Status> {
        let mut status = Status::default();
        self.bridge.put(&options.unwrap_or(&self.default_write_options).bridge, cf,
                        key.as_ref(), val.as_ref(),
                        &mut status);
        if status.code == StatusCode::kOk {
            Ok(status)
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get(&self, key: impl AsRef<[u8]>, cf: usize, options: Option<&ReadOptions>) -> Result<Option<PinnableSlice>, Status> {
        let mut status = Status::default();
        let slice = self.bridge.get(
            &options.unwrap_or(&self.default_read_options).bridge, cf,
            key.as_ref(), &mut status);
        match status.code {
            StatusCode::kOk => Ok(Some(slice)),
            StatusCode::kNotFound => Ok(None),
            _ => Err(status)
        }
    }

    #[inline]
    pub fn iterator(&self, cf: usize, options: Option<&ReadOptions>) -> Iterator {
        self.bridge.iterator(&options.unwrap_or(&self.default_read_options).bridge, cf)
    }
}

impl Default for Status {
    #[inline]
    fn default() -> Self {
        Self {
            code: StatusCode::kOk,
            subcode: StatusSubCode::kNone,
            severity: StatusSeverity::kNoError,
        }
    }
}