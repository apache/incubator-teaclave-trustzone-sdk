#![allow(unused)]
use crate::{Error, ErrorKind, Handle, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

pub struct OperationHandle {
    raw: *mut raw::TEE_OperationHandle,
}

impl OperationHandle {
    pub fn from_raw(raw: *mut raw::TEE_OperationHandle) -> OperationHandle {
        Self { raw }
    }

    pub fn handle(&self) -> raw::TEE_OperationHandle {
        unsafe { *(self.raw) }
    }
}

pub struct Operation(OperationHandle);

impl Operation {
    pub fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }

    pub fn allocate(
        algo: u32, /*Algorithm*/
        mode: u32, /*Mode*/
        max_key_size: usize,
    ) -> Result<Self> {
        let mut raw_handle: *mut raw::TEE_OperationHandle =
            Box::into_raw(Box::new(ptr::null_mut()));
        match unsafe {
            raw::TEE_AllocateOperation(
                raw_handle as *mut _,
                algo as u32,
                mode as u32,
                max_key_size as u32,
            )
        } {
            raw::TEE_SUCCESS => {
                let handle = OperationHandle::from_raw(raw_handle);
                return Ok(Self(handle));
            }
            code => Err(Error::from_raw_error(code)),
        }
    }
    pub fn set_key<T: Handle>(&self, object: &T) -> Result<()> {
        match unsafe { raw::TEE_SetOperationKey(self.handle(), object.handle()) } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }
}
/// free before check it's not null
/// Objects need the checks too
/// Objects above print need to be removed
/// Add obj_handle function for persistent object
impl Drop for Operation {
    fn drop(&mut self) {
        unsafe {
            if self.0.raw != Box::into_raw(Box::new(ptr::null_mut())) {
                raw::TEE_FreeOperation(*self.0.raw);
            }
            Box::from_raw(self.0.raw);
        }
    }
}

pub struct MAC(Operation);

impl MAC {
    pub fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }

    pub fn init(op: Operation, iv: &[u8]) -> Self {
        unsafe { raw::TEE_MACInit(op.handle(), iv.as_ptr() as _, iv.len() as u32) };
        Self(op)
    }

    pub fn update(&self, chunk: &[u8]) {
        unsafe { raw::TEE_MACUpdate(self.handle(), chunk.as_ptr() as _, chunk.len() as u32) };
    }

    /// output mac size is unsure when passed in, so we return its result
    pub fn compute_final(&self, message: &[u8], mac: &mut [u8]) -> Result<usize> {
        let mut out_len: usize = mac.len();
        match unsafe {
            raw::TEE_MACComputeFinal(
                self.handle(),
                message.as_ptr() as _,
                message.len() as u32,
                mac.as_mut_ptr() as _,
                &mut (out_len as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(out_len);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }
}

impl Drop for MAC {
    fn drop(&mut self) {
        drop(Operation);
    }
}
