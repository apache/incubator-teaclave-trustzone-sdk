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
    trace_println!("[+] TA_CreateEntryPoint: Random generator.");
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
    trace_println!("[+] TA_OpenSessionEntryPoint: Random, generator!");
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) {
    trace_println!("[+] TA_CloseSessionEntryPoint: Goodbye.");
}

pub fn random_number_generate(param_types: uint32_t, params: &mut [TEE_Param; 4]) -> TEE_Result {
    let exp_param_types: uint32_t = TEE_PARAM_TYPES(
        TEE_PARAM_TYPE_MEMREF_OUTPUT,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
    );
    if param_types != exp_param_types {
        return TEE_ERROR_BAD_PARAMETERS;
    }

    unsafe {
        TEE_GenerateRandom(params[0].memref.buffer, params[0].memref.size);
    }
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    param_types: uint32_t,
    params: &mut [TEE_Param; 4],
) -> TEE_Result {
    trace_println!("[+] TA_InvokeCommandEntryPoint: Invoke.");
    match cmd_id {
        TA_RANDOM_CMD_GENERATE => {
            return random_number_generate(param_types, params);
        }
        _ => {
            return TEE_ERROR_BAD_PARAMETERS;
        }
    }
}

const TA_FLAGS: uint32_t = TA_FLAG_EXEC_DDR;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 32 * 1024;
const EXT_PROP_VALUE_1: &[u8] =
    b"Example of a TA that returns the output from TEE_GenerateRandom\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
