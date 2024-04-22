// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use optee_utee_sys as raw;
use core::{fmt, result};
#[cfg(feature = "std")]
use std::error;
#[cfg(not(feature = "std"))]
use core::error;

/// A specialized [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
/// type for TEE operations.
///
/// # Examples
///
/// ``` no_run
/// fn open_session(params: &mut Parameters) -> Result<()> {
///     Ok(())
/// }
/// ````
pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    code: u32,
}

/// A list specifying general categories of TEE error and its corresponding code
/// in OP-TEE OS.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum ErrorKind {
    /// Object corruption.
    CorruptObject = 0xF0100001,
    /// Persistent object corruption.
    CorruptObject2 = 0xF0100002,
    /// Object storage is not available.
    StorageNotAvailable = 0xF0100003,
    /// Persistent object storage is not available.
    StorageNotAvailable2 = 0xF0100004,
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
    OutOfMemory = 0xFFFF000C,
    /// The system is busy working on something else.
    Busy = 0xFFFF000D,
    /// Communication with a remote party failed.
    Communication = 0xFFFF000E,
    /// A security fault was detected.
    Security = 0xFFFF000F,
    /// The supplied buffer is too short for the generated output.
    ShortBuffer = 0xFFFF0010,
    /// The operation has been cancelled by an external event which occurred in
    /// the REE while the function was in progress.
    ExternalCancel = 0xFFFF0011,
    /// Data overflow.
    Overflow = 0xFFFF300F,
    /// Trusted Application has panicked during the operation.
    TargetDead = 0xFFFF3024,
    /// Insufficient space is available.
    StorageNoSpace = 0xFFFF3041,
    /// MAC is invalid.
    MacInvalid = 0xFFFF3071,
    /// Signature is invalid.
    SignatureInvalid = 0xFFFF3072,
    /// The persistent time has not been set.
    TimeNotSet = 0xFFFF5000,
    /// The persistent time has been set but may have been corrupted and SHALL
    /// no longer be trusted.
    TimeNeedsReset = 0xFFFF5001,
    /// Unknown error.
    Unknown,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
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
            ErrorKind::Overflow => "Data overflow.",
            ErrorKind::TargetDead => "Trusted Application has panicked during the operation.",
            ErrorKind::StorageNoSpace => "Insufficient space is available.",
            ErrorKind::MacInvalid => "MAC is invalid.",
            ErrorKind::SignatureInvalid => "Signature is invalid.",
            ErrorKind::TimeNotSet => "The persistent time has not been set.",
            ErrorKind::TimeNeedsReset => {
                "The persistent time has been set but may have been corrupted and SHALL no longer be trusted."
            },
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
    /// use optee_utee;
    ///
    /// let error = optee_utee::Error::from_raw_error(0xFFFF000F);
    /// assert_eq!(error.kind(), optee_utee::ErrorKind::Security);
    /// ```
    pub fn from_raw_error(code: u32) -> Error {
        Error { code }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use optee_utee;
    ///
    /// let error = optee_utee::Error::new(optee_utee::ErrorKind::Security);
    /// ```
    pub fn kind(&self) -> ErrorKind {
        match self.code {
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

    pub fn raw_code(&self) -> u32 {
        self.code
    }

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

impl error::Error for Error {
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
