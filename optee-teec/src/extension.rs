use optee_teec_sys as raw;
use libc::{c_char};
use crate::{Result};

#[repr(C)]
pub struct Plugin_Method {
    pub name: *mut c_char,
    pub uuid: raw::TEEC_UUID,
    pub init: fn() -> Result<()>,
    pub invoke: fn(
        cmd: u32,
        sub_cmd: u32,
        data: *mut c_char,
        in_len: u32,
        out_len: *mut u32,
    ) -> Result<()>,
}
