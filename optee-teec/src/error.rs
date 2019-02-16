use libc;
use optee_teec_sys as raw;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    code: u32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorCode {
    Generic,
    AccessDenied,
    Cancel,
    AccessConflict,
    ExcessData,
    BadFormat,
    BadParameters,
    BadState,
    ItemNotFound,
    NotImplemented,
    NotSupported,
    NoData,
    OutOfMEmory,
    Busy,
    Communication,
    Security,
    ShortBuffer,
    ExternalCancel,
    TargetDead,
    Unknown,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorCode::Generic => "Non-specific cause.",
            ErrorCode::AccessDenied => "Access privileges are not sufficient.",
            ErrorCode::Cancel => "The operation was canceled.",
            ErrorCode::AccessConflict => "Concurrent accesses caused conflict.",
            ErrorCode::ExcessData => "Too much data for the requested operation was passed.",
            ErrorCode::BadFormat => "Input data was of invalid format.",
            ErrorCode::BadParameters => "Input parameters were invalid.",
            ErrorCode::BadState => "Operation is not valid in the current state.",
            ErrorCode::ItemNotFound => "The requested data item is not found.",
            ErrorCode::NotImplemented => {
                "The requested operation should exist but is not yet implemented."
            }
            ErrorCode::NotSupported => {
                "The requested operation is valid but is not supported in this implementation."
            }
            ErrorCode::NoData => "Expected data was missing.",
            ErrorCode::OutOfMEmory => "System ran out of resources.",
            ErrorCode::Busy => "The system is busy working on something else.",
            ErrorCode::Communication => "Communication with a remote party failed.",
            ErrorCode::Security => "A security fault was detected.",
            ErrorCode::ShortBuffer => "The supplied buffer is too short for the generated output.",
            ErrorCode::ExternalCancel => "Undocumented.",
            ErrorCode::TargetDead => "Trusted Application has panicked during the operation.",
            ErrorCode::Unknown => "Unknown error.",
        }
    }
}

impl Error {
    pub fn from_raw_error(code: u32) -> Error {
        Error { code: code }
    }

    pub fn code(&self) -> ErrorCode {
        match self.code as libc::uint32_t {
            raw::TEEC_ERROR_GENERIC => ErrorCode::Generic,
            raw::TEEC_ERROR_ACCESS_DENIED => ErrorCode::AccessDenied,
            raw::TEEC_ERROR_CANCEL => ErrorCode::Cancel,
            raw::TEEC_ERROR_ACCESS_CONFLICT => ErrorCode::AccessConflict,
            raw::TEEC_ERROR_EXCESS_DATA => ErrorCode::ExcessData,
            raw::TEEC_ERROR_BAD_FORMAT => ErrorCode::BadFormat,
            raw::TEEC_ERROR_BAD_PARAMETERS => ErrorCode::BadParameters,
            raw::TEEC_ERROR_BAD_STATE => ErrorCode::BadState,
            raw::TEEC_ERROR_ITEM_NOT_FOUND => ErrorCode::ItemNotFound,
            raw::TEEC_ERROR_NOT_IMPLEMENTED => ErrorCode::NotImplemented,
            raw::TEEC_ERROR_NOT_SUPPORTED => ErrorCode::NotSupported,
            raw::TEEC_ERROR_NO_DATA => ErrorCode::NoData,
            raw::TEEC_ERROR_OUT_OF_MEMORY => ErrorCode::OutOfMEmory,
            raw::TEEC_ERROR_BUSY => ErrorCode::Busy,
            raw::TEEC_ERROR_COMMUNICATION => ErrorCode::Communication,
            raw::TEEC_ERROR_SECURITY => ErrorCode::Security,
            raw::TEEC_ERROR_SHORT_BUFFER => ErrorCode::ShortBuffer,
            raw::TEEC_ERROR_EXTERNAL_CANCEL => ErrorCode::ExternalCancel,
            raw::TEEC_ERROR_TARGET_DEAD => ErrorCode::TargetDead,
            _ => ErrorCode::Unknown,
        }
    }

    pub fn raw_code(&self) -> libc::uint32_t {
        self.code as libc::uint32_t
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.code().as_str())
    }
}
