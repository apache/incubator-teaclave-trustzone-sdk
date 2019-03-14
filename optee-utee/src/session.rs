use crate::{Error, ErrorKind, Result};
use libc::c_void;
use optee_utee_sys as raw;

#[derive(Copy, Clone)]
pub struct Session {
    pub raw: *mut c_void,
}

impl Session {
    pub fn new(ptr: *mut c_void) -> Self {
        Self { raw: ptr }
    }

    //Session struct needs to be manually allocated in open session function
    pub fn alloc(ptr: *mut *mut c_void, size: u32) -> Result<Self> {
        unsafe {
            *ptr = raw::TEE_Malloc(size, 0);

            if (*ptr).is_null() {
                return Err(Error::new(ErrorKind::OutOfMemory));
            } else {
                Ok(Session::new(*ptr))
            }
        }
    }

    pub fn validate(&mut self) -> Result<()> {
        if self.raw.is_null() {
            return Err(Error::new(ErrorKind::ItemNotFound));
        } else {
            Ok(())
        }
    }

    //Session needs to collect the ownership everytime after borrow the pointer to a session
    //context struct
    pub fn collect(&mut self, box_ptr: *mut c_void) {
        self.raw = box_ptr;
    }
}
