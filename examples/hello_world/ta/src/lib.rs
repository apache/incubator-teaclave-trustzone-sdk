#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate optee_utee;
extern crate libc;
use optee_utee::*;
use libc::{c_void, uint32_t};

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(
    _param_types: uint32_t,
    _params: TEE_Param,
    _sess_ctx: *mut *mut c_void,
) -> TEE_Result {
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) {}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    _param_types: uint32_t,
    params: &mut [TEE_Param; 4],
) -> TEE_Result {
    match cmd_id {
        0 => unsafe {
            params[0].value.a += 121;
        },
        1 => unsafe {
            params[0].value.a -= 21;
        },
        _ => {
            return TEE_ERROR_BAD_PARAMETERS;
        }
    }
    TEE_SUCCESS
}
