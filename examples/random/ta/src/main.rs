#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_int, c_ulong, c_void, size_t, uint32_t};
use optee_utee;
use optee_utee::{trace_println, Error, Parameters, Result};
use optee_utee_sys::*;
use std::{mem, str};

fn MESA_CreateEntryPoint() -> Result<()> {
    Ok(())
}

fn MESA_OpenSessionEntryPoint(_params: &mut Parameters, _sess_ctx: *mut *mut c_void) -> Result<()> {
    Ok(())
}

fn MESA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) -> Result<()> {
    Ok(())
}

fn MESA_DestroyEntryPoint() -> Result<()> {
    Ok(())
}

fn MESA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    params: &mut Parameters,
) -> Result<()> {
    match cmd_id {
        TA_RANDOM_CMD_GENERATE => {
            return random_number_generate(params);
        }
        _ => {
            return Err(Error::from_raw_error(TEE_ERROR_BAD_PARAMETERS));
        }
    }
}

pub fn random_number_generate(params: &mut Parameters) -> Result<()> {
    unsafe {
        TEE_GenerateRandom(
            (*params.param_0.raw).memref.buffer,
            (*params.param_0.raw).memref.size,
        );
    }
    Ok(())
}

const ta_name: &str = "Random Generator";

const TA_FLAGS: uint32_t = TA_FLAG_EXEC_DDR;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 32 * 1024;
const EXT_PROP_VALUE_1: &[u8] =
    b"Example of a TA that returns the output from TEE_GenerateRandom\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
