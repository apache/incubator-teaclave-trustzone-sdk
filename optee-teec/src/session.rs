use libc;
use optee_teec_sys as raw;

use crate::{Error, Operation, Result};

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
    pub fn from_raw(raw: raw::TEEC_Session) -> Self {
        Session { raw }
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
