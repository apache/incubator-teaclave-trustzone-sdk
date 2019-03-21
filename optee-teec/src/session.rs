use libc;
use optee_teec_sys as raw;
use std::ptr;

use crate::{Context, Error, Operation, Result, Uuid};

#[derive(Copy, Clone)]
pub enum ConnectionMethods {
    LoginPublic,
    LoginUser,
    LoginGroup,
    LoginApplication,
    LoginUserApplication,
    LoginGroupApplication,
}

pub struct Session {
    raw: raw::TEEC_Session,
}

impl Session {
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

    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEEC_Session {
        &mut self.raw
    }

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
