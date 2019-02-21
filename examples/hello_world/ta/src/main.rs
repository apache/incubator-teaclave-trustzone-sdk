#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use optee_utee_sys::*;
use optee_utee;
use libc::{c_void, uint32_t};
use libc::*;
use std::mem;

pub const TA_HELLO_WORLD_CMD_INC_VALUE: u32 = 0;
pub const TA_HELLO_WORLD_CMD_DEC_VALUE: u32 = 1;

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    optee_utee::trace_println!("[+] TA_CreateEntryPoint: Hello.");
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
    optee_utee::trace_println!("[+] TA_OpenSessionEntryPoint: Hello, World!");
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) {
    optee_utee::trace_println!("[+] TA_CloseSessionEntryPoint: Goodbye.");
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    _param_types: uint32_t,
    params: &mut [TEE_Param; 4],
) -> TEE_Result {
    optee_utee::trace_println!("[+] TA_InvokeCommandEntryPoint: Invoke.");
    match cmd_id {
        TA_HELLO_WORLD_CMD_INC_VALUE => unsafe {
            params[0].value.a += 121;
        },
        TA_HELLO_WORLD_CMD_DEC_VALUE => unsafe {
            params[0].value.a -= 21;
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
const TRACE_LEVEL: c_int = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_VERSION: &[u8] = b"Undefined version\0";
const TA_DESCRIPTION: &[u8] = b"Undefined description\0";
const TA_FRAMEWORK_STACK_SIZE: uint32_t = 2048;

const TA_UUID: TEE_UUID = TEE_UUID {
    timeLow: 0x8abcf200,
    timeMid: 0x2450,
    timeHiAndVersion: 0x11e4,
    clockSeqAndNode: [0xab, 0xe2, 0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b],
};

#[no_mangle]
pub static mut trace_level: c_int = TRACE_LEVEL;

#[no_mangle]
pub static trace_ext_prefix: &[u8] = TRACE_EXT_PREFIX;

extern "C" {
    fn __utee_entry(func: c_ulong, session_id: c_ulong, up: *mut utee_params, cmd_id: c_ulong);
}

#[no_mangle]
#[link_section = ".ta_head"]
pub static ta_head: ta_head = ta_head {
    uuid: TA_UUID,
    stack_size: TA_STACK_SIZE + TA_FRAMEWORK_STACK_SIZE,
    flags: TA_FLAGS,
    entry: __utee_entry as unsafe extern "C" fn(c_ulong, c_ulong, *mut utee_params, c_ulong)
};

#[no_mangle]
pub static ta_heap: &[u8; TA_DATA_SIZE as usize] = &['\0' as u8; TA_DATA_SIZE as usize];

#[no_mangle]
pub static ta_heap_size: size_t = mem::size_of::<u8>() * TA_DATA_SIZE as usize;
pub static flag_bool: bool = (TA_FLAGS & TA_FLAG_SINGLE_INSTANCE) != 0;
pub static flag_multi: bool = (TA_FLAGS & TA_FLAG_MULTI_SESSION) != 0;
pub static flag_instance: bool = (TA_FLAGS & TA_FLAG_INSTANCE_KEEP_ALIVE) != 0;

#[no_mangle]
pub static ta_num_props: size_t = 9;

#[no_mangle]
pub static ta_props: [user_ta_property; 9] = [
    user_ta_property {
        name: TA_PROP_STR_SINGLE_INSTANCE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_BOOL,
        value: &flag_bool as *const bool as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_MULTI_SESSION,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_BOOL,
        value: &flag_multi as *const bool as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_KEEP_ALIVE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_BOOL,
        value: &flag_instance as *const bool as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_DATA_SIZE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_U32,
        value: &TA_DATA_SIZE as *const uint32_t as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_STACK_SIZE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_U32,
        value: &TA_STACK_SIZE as *const uint32_t as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_VERSION,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_STRING,
        value: TA_VERSION as *const [u8] as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_DESCRIPTION,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_STRING,
        value: TA_DESCRIPTION as *const [u8] as *mut _,
    },
    user_ta_property {
        name: "gp.ta.description\0".as_ptr(),
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_STRING,
        value: EXT_PROP_VALUE_1 as *const [u8] as *mut _,
    },
    user_ta_property {
        name: "gp.ta.version\0".as_ptr(),
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_U32,
        value: &EXT_PROP_VALUE_2 as *const uint32_t as *mut _,
    },
];

#[no_mangle]
pub unsafe extern "C" fn tahead_get_trace_level() -> c_int {
    return trace_level;
}
