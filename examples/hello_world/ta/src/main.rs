#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_int, c_ulong, c_void, size_t, uint32_t};
use optee_utee;
use optee_utee::trace_println;
use optee_utee_sys::*;
use std::mem;

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    trace_println!("[+] TA_CreateEntryPoint: Hello.");
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(
    _param_types: uint32_t,
    _params: &mut [TEE_Param; 4],
    _sess_ctx: *mut *mut c_void,
) -> TEE_Result {
    trace_println!("[+] TA_OpenSessionEntryPoint: Hello, World!");
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) {
    trace_println!("[+] TA_CloseSessionEntryPoint: Goodbye.");
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    _param_types: uint32_t,
    params: &mut [TEE_Param; 4],
) -> TEE_Result {
    trace_println!("[+] TA_InvokeCommandEntryPoint: Invoke.");
    match cmd_id {
        TA_HELLO_WORLD_CMD_INC_VALUE => unsafe {
            params[0].value.a += 100;
        },
        TA_HELLO_WORLD_CMD_DEC_VALUE => unsafe {
            params[0].value.a -= 100;
        },
        _ => {
            return TEE_ERROR_BAD_PARAMETERS;
        }
    }
    TEE_SUCCESS
}

const TA_FLAGS: uint32_t = 0;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 32 * 1024;
const EXT_PROP_VALUE_1: &[u8] = b"Hello World TA\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
