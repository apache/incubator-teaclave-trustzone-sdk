use libc::{c_char, c_int};
use optee_teec_sys as raw;
use std::ptr;

use crate::{Error, Result};

pub struct TEECContext {
    raw: raw::TEEC_Context,
}

impl TEECContext {
    pub fn new() -> Result<TEECContext> {
        TEECContext::new_raw(0, true)
    }

    pub fn new_raw(fd: c_int, reg_mem: bool) -> Result<TEECContext> {
        let mut raw_ctx = raw::TEEC_Context {
            fd: fd,
            reg_mem: reg_mem,
        };
        unsafe {
            let res = raw::TEEC_InitializeContext(ptr::null_mut() as *mut c_char, &mut raw_ctx);
            if res != raw::TEEC_SUCCESS {
                Err(Error::from_raw_error(res))
            } else {
                Ok(TEECContext { raw: raw_ctx })
            }
        }
    }

    pub fn borrow_raw(&mut self) -> *mut raw::TEEC_Context {
        &mut self.raw
    }
}

impl Drop for TEECContext {
    fn drop(&mut self) {
        unsafe {
            raw::TEEC_FinalizeContext(&mut self.raw);
        }
    }
}
