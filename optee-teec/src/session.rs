use libc;
use optee_teec_sys as raw;
use std::ptr;

use crate::{Context, Error, Operation, Result, Uuid};

/// Session login methods.
#[derive(Copy, Clone)]
pub enum ConnectionMethods {
    /// No login data is provided.
    LoginPublic,
    /// Login data about the user running the Client Application process is provided.
    LoginUser,
    /// Login data about the group running the Client Application process is provided.
    LoginGroup,
    /// Login data about the running Client Application itself is provided.
    LoginApplication,
    /// Login data about the user and the running Client Application itself is provided.
    LoginUserApplication,
    /// Login data about the group and the running Client Application itself is provided.
    LoginGroupApplication,
}

/// Represents a connection between a client application and a trusted application.
pub struct Session {
    raw: raw::TEEC_Session,
}

impl Session {
    /// Initializes a TEE session object with specified context and uuid.
    pub fn new(context: &mut Context, uuid: Uuid) -> Result<Self> {
        let mut raw_session = raw::TEEC_Session {
            ctx: context.as_mut_raw_ptr(),
            session_id: 0,
        };
        let mut err_origin: libc::uint32_t = 0;
        unsafe {
            match raw::TEEC_OpenSession(
                context.as_mut_raw_ptr(),
                &mut raw_session,
                uuid.as_raw_ptr(),
                ConnectionMethods::LoginPublic as u32,
                ptr::null() as *const libc::c_void,
                ptr::null_mut() as *mut raw::TEEC_Operation,
                &mut err_origin,
            ) {
                raw::TEEC_SUCCESS => Ok(Self {
                    raw: raw_session,
                }),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    /// Converts a TEE client context to a raw pointer.
    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEEC_Session {
        &mut self.raw
    }

    /// Invokes a command with an operation with this session.
    pub fn invoke_command(&mut self, command_id: u32, operation: &mut Operation) -> Result<()> {
        let mut err_origin: libc::uint32_t = 0;
        unsafe {
            match raw::TEEC_InvokeCommand(
                &mut self.raw,
                command_id,
                operation.as_mut_raw_ptr(),
                &mut err_origin,
            ) {
                raw::TEEC_SUCCESS => Ok(()),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            raw::TEEC_CloseSession(&mut self.raw);
        }
    }
}
