use libc;
use optee_teec_sys as raw;
use std::ptr;

use crate::{Error, Result, Session, Uuid};

pub struct Context {
    raw: raw::TEEC_Context,
}

impl Context {
    pub fn new() -> Result<Context> {
        Context::new_raw(0, true)
    }

    pub fn new_raw(fd: libc::c_int, reg_mem: bool) -> Result<Context> {
        let mut raw_ctx = raw::TEEC_Context { fd, reg_mem };
        unsafe {
            match raw::TEEC_InitializeContext(ptr::null_mut() as *mut libc::c_char, &mut raw_ctx) {
                raw::TEEC_SUCCESS => Ok(Context { raw: raw_ctx }),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEEC_Context {
        &mut self.raw
    }

    pub fn open_session(&mut self, uuid: Uuid) -> Result<Session> {
        Session::new(self, uuid)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            raw::TEEC_FinalizeContext(&mut self.raw);
        }
    }
}
