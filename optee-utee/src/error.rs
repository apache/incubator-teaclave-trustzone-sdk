use libc;
use optee_utee_sys as raw;
use std::convert::From;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    code: u32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorKind {
    CorruptObject = 0xF0100001,
    CorruptObject2 = 0xF0100002,
    StorageNotAvailable = 0xF0100003,
    StorageNotAvailable2 = 0xF0100004,
    Generic = 0xFFFF0000,
    AccessDenied = 0xFFFF0001,
    Cancel = 0xFFFF0002,
    AccessConflict = 0xFFFF0003,
    ExcessData = 0xFFFF0004,
    BadFormat = 0xFFFF0005,
    BadParameters = 0xFFFF0006,
    BadState = 0xFFFF0007,
    ItemNotFound = 0xFFFF0008,
    NotImplemented = 0xFFFF0009,
    NotSupported = 0xFFFF000A,
    NoData = 0xFFFF000B,
    OutOfMemory = 0xFFFF000C,
    Busy = 0xFFFF000D,
    Communication = 0xFFFF000E,
    Security = 0xFFFF000F,
    ShortBuffer = 0xFFFF0010,
    ExternalCancel = 0xFFFF0011,
    Overflow = 0xFFFF300F,
    TargetDead = 0xFFFF3024,
    StorageNoSpace = 0xFFFF3041,
    MacInvalid = 0xFFFF3071,
    SignatureInvalid = 0xFFFF3072,
    TimeNotSet = 0xFFFF5000,
    TimeNeedsReset = 0xFFFF5001,
    Unknown,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::CorruptObject => "Object corruption.",
            ErrorKind::CorruptObject2 => "Persistent object corruption.",
            ErrorKind::StorageNotAvailable => "Object storage is not available.",
            ErrorKind::StorageNotAvailable2 => "Persistent object storage is not available.",
            ErrorKind::Generic => "Non-specific cause.",
            ErrorKind::AccessDenied => "Access privileges are not sufficient.",
            ErrorKind::Cancel => "The operation was canceled.",
            ErrorKind::AccessConflict => "Concurrent accesses caused conflict.",
            ErrorKind::ExcessData => "Too much data for the requested operation was passed.",
            ErrorKind::BadFormat => "Input data was of invalid format.",
            ErrorKind::BadParameters => "Input parameters were invalid.",
            ErrorKind::BadState => "Operation is not valid in the current state.",
            ErrorKind::ItemNotFound => "The requested data item is not found.",
            ErrorKind::NotImplemented => {
                "The requested operation should exist but is not yet implemented."
            }
            ErrorKind::NotSupported => {
                "The requested operation is valid but is not supported in this implementation."
            }
            ErrorKind::NoData => "Expected data was missing.",
            ErrorKind::OutOfMemory => "System ran out of resources.",
            ErrorKind::Busy => "The system is busy working on something else.",
            ErrorKind::Communication => "Communication with a remote party failed.",
            ErrorKind::Security => "A security fault was detected.",
            ErrorKind::ShortBuffer => "The supplied buffer is too short for the generated output.",
            ErrorKind::ExternalCancel => "Undocumented.",
            ErrorKind::Overflow => "Data overflow",
            ErrorKind::TargetDead => "Trusted Application has panicked during the operation.",
            ErrorKind::StorageNoSpace => "Storage no space",
            ErrorKind::MacInvalid => "Mac is invalid.",
            ErrorKind::SignatureInvalid => "Signature is invalid.",
            ErrorKind::TimeNotSet => "Time is not set.",
            ErrorKind::TimeNeedsReset => "Time needs to be reset.",
            ErrorKind::Unknown => "Unknown error.",
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { code: kind as u32 }
    }

    pub fn from_raw_error(code: u32) -> Error {
        Error { code }
    }

    pub fn kind(&self) -> ErrorKind {
        match self.code as libc::uint32_t {
            raw::TEE_ERROR_CORRUPT_OBJECT => ErrorKind::CorruptObject,
            raw::TEE_ERROR_CORRUPT_OBJECT_2 => ErrorKind::CorruptObject2,
            raw::TEE_ERROR_STORAGE_NOT_AVAILABLE => ErrorKind::StorageNotAvailable,
            raw::TEE_ERROR_STORAGE_NOT_AVAILABLE_2 => ErrorKind::StorageNotAvailable2,
            raw::TEE_ERROR_GENERIC => ErrorKind::Generic,
            raw::TEE_ERROR_ACCESS_DENIED => ErrorKind::AccessDenied,
            raw::TEE_ERROR_CANCEL => ErrorKind::Cancel,
            raw::TEE_ERROR_ACCESS_CONFLICT => ErrorKind::AccessConflict,
            raw::TEE_ERROR_EXCESS_DATA => ErrorKind::ExcessData,
            raw::TEE_ERROR_BAD_FORMAT => ErrorKind::BadFormat,
            raw::TEE_ERROR_BAD_PARAMETERS => ErrorKind::BadParameters,
            raw::TEE_ERROR_BAD_STATE => ErrorKind::BadState,
            raw::TEE_ERROR_ITEM_NOT_FOUND => ErrorKind::ItemNotFound,
            raw::TEE_ERROR_NOT_IMPLEMENTED => ErrorKind::NotImplemented,
            raw::TEE_ERROR_NOT_SUPPORTED => ErrorKind::NotSupported,
            raw::TEE_ERROR_NO_DATA => ErrorKind::NoData,
            raw::TEE_ERROR_OUT_OF_MEMORY => ErrorKind::OutOfMemory,
            raw::TEE_ERROR_BUSY => ErrorKind::Busy,
            raw::TEE_ERROR_COMMUNICATION => ErrorKind::Communication,
            raw::TEE_ERROR_SECURITY => ErrorKind::Security,
            raw::TEE_ERROR_SHORT_BUFFER => ErrorKind::ShortBuffer,
            raw::TEE_ERROR_EXTERNAL_CANCEL => ErrorKind::ExternalCancel,
            raw::TEE_ERROR_OVERFLOW => ErrorKind::Overflow,
            raw::TEE_ERROR_TARGET_DEAD => ErrorKind::TargetDead,
            raw::TEE_ERROR_STORAGE_NO_SPACE => ErrorKind::StorageNoSpace,
            raw::TEE_ERROR_MAC_INVALID => ErrorKind::MacInvalid,
            raw::TEE_ERROR_SIGNATURE_INVALID => ErrorKind::SignatureInvalid,
            raw::TEE_ERROR_TIME_NOT_SET => ErrorKind::TimeNotSet,
            raw::TEE_ERROR_TIME_NEEDS_RESET => ErrorKind::TimeNeedsReset,
            _ => ErrorKind::Unknown,
        }
    }

    pub fn raw_code(&self) -> libc::uint32_t {
        self.code as libc::uint32_t
    }

    pub fn message(&self) -> &str {
        self.kind().as_str()
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.message())
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error { code: kind as u32 }
    }
}
