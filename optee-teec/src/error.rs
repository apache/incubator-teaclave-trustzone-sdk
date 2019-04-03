use optee_teec_sys as raw;
use std::fmt;

/// A specialized [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
/// type for TEE operations.
///
/// # Examples
///
/// ``` no_run
/// fn main() -> optee_teec::Result<()> {
///     let mut ctx = Context::new()?;
/// }
/// ````
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for TEE operations of [`Context`] and [`Session`].
///
/// [`Context`]: struct.Context.html
/// [`Session`]: struct.Session.html
pub struct Error {
    code: u32,
}

/// A list specifying general categories of TEE client error and its
/// corresponding code in OP-TEE client library.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// Non-specific cause.
    Generic = 0xFFFF0000,
    /// Access privileges are not sufficient.
    AccessDenied = 0xFFFF0001,
    /// The operation was canceled.
    Cancel = 0xFFFF0002,
    /// Concurrent accesses caused conflict.
    AccessConflict = 0xFFFF0003,
    /// Too much data for the requested operation was passed.
    ExcessData = 0xFFFF0004,
    /// Input data was of invalid format.
    BadFormat = 0xFFFF0005,
    /// Input parameters were invalid.
    BadParameters = 0xFFFF0006,
    /// Operation is not valid in the current state.
    BadState = 0xFFFF0007,
    /// The requested data item is not found.
    ItemNotFound = 0xFFFF0008,
    /// The requested operation should exist but is not yet implemented.
    NotImplemented = 0xFFFF0009,
    /// The requested operation is valid but is not supported in this implementation.
    NotSupported = 0xFFFF000A,
    /// Expected data was missing.
    NoData = 0xFFFF000B,
    /// System ran out of resources.
    OutOfMEmory = 0xFFFF000C,
    /// The system is busy working on something else.
    Busy = 0xFFFF000D,
    /// Communication with a remote party failed.
    Communication = 0xFFFF000E,
    /// A security fault was detected.
    Security = 0xFFFF000F,
    /// The supplied buffer is too short for the generated output.
    ShortBuffer = 0xFFFF0010,
    /// Implementation defined error code.
    ExternalCancel = 0xFFFF0011,
    /// Implementation defined error code: trusted Application has panicked during the operation.
    TargetDead = 0xFFFF3024,
    /// Unknown error.
    Unknown,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
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
            ErrorKind::OutOfMEmory => "System ran out of resources.",
            ErrorKind::Busy => "The system is busy working on something else.",
            ErrorKind::Communication => "Communication with a remote party failed.",
            ErrorKind::Security => "A security fault was detected.",
            ErrorKind::ShortBuffer => "The supplied buffer is too short for the generated output.",
            ErrorKind::ExternalCancel => "Undocumented.",
            ErrorKind::TargetDead => "Trusted Application has panicked during the operation.",
            ErrorKind::Unknown => "Unknown error.",
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { code: kind as u32 }
    }
    /// Creates a new instance of an `Error` from a particular TEE error code.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use optee_teec;
    ///
    /// let error = optee_teec::Error::from_raw_error(0xFFFF000F);
    /// assert_eq!(error.kind(), optee_teec::ErrorKind::Security);
    /// ```
    pub fn from_raw_error(code: u32) -> Error {
        Error { code }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use optee_teec;
    ///
    /// let error = optee_teec::Error::new(optee_teec::ErrorKind::Security);
    /// ```
    pub fn kind(&self) -> ErrorKind {
        match self.code {
            raw::TEEC_ERROR_GENERIC => ErrorKind::Generic,
            raw::TEEC_ERROR_ACCESS_DENIED => ErrorKind::AccessDenied,
            raw::TEEC_ERROR_CANCEL => ErrorKind::Cancel,
            raw::TEEC_ERROR_ACCESS_CONFLICT => ErrorKind::AccessConflict,
            raw::TEEC_ERROR_EXCESS_DATA => ErrorKind::ExcessData,
            raw::TEEC_ERROR_BAD_FORMAT => ErrorKind::BadFormat,
            raw::TEEC_ERROR_BAD_PARAMETERS => ErrorKind::BadParameters,
            raw::TEEC_ERROR_BAD_STATE => ErrorKind::BadState,
            raw::TEEC_ERROR_ITEM_NOT_FOUND => ErrorKind::ItemNotFound,
            raw::TEEC_ERROR_NOT_IMPLEMENTED => ErrorKind::NotImplemented,
            raw::TEEC_ERROR_NOT_SUPPORTED => ErrorKind::NotSupported,
            raw::TEEC_ERROR_NO_DATA => ErrorKind::NoData,
            raw::TEEC_ERROR_OUT_OF_MEMORY => ErrorKind::OutOfMEmory,
            raw::TEEC_ERROR_BUSY => ErrorKind::Busy,
            raw::TEEC_ERROR_COMMUNICATION => ErrorKind::Communication,
            raw::TEEC_ERROR_SECURITY => ErrorKind::Security,
            raw::TEEC_ERROR_SHORT_BUFFER => ErrorKind::ShortBuffer,
            raw::TEEC_ERROR_EXTERNAL_CANCEL => ErrorKind::ExternalCancel,
            raw::TEEC_ERROR_TARGET_DEAD => ErrorKind::TargetDead,
            _ => ErrorKind::Unknown,
        }
    }

    /// Returns raw code of this error.
    pub fn raw_code(&self) -> u32 {
        self.code
    }

    /// Returns corresponding error message of this error.
    pub fn message(&self) -> &str {
        self.kind().as_str()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} (error code 0x{:x})", self.message(), self.code)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} (error code 0x{:x})", self.message(), self.code)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error { code: kind as u32 }
    }
}
