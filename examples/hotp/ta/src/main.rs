#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_int, c_ulong, c_void, size_t, uint32_t};
use optee_utee;
use optee_utee::{trace_println, Error, ParamTypeFlags, Parameters, Result};
use optee_utee_sys::*;
use std::{mem, ptr, str};

pub const SHA1_HASH_SIZE: u32 = 20;

pub const MAX_KEY_SIZE: u32 = 64;
pub const MIN_KEY_SIZE: u32 = 10;

pub const DBC2_MODULO: u32 = 1000000;

pub static mut k: [u8; MAX_KEY_SIZE as usize] = [0; MAX_KEY_SIZE as usize];
pub static mut K_LEN: u32 = 0;

pub static mut counter: [u8; 8] = [0x0; 8];

fn MESA_CreateEntryPoint() -> Result<()> {
    Ok(())
}

fn MESA_OpenSessionEntryPoint(params: &mut Parameters, _sess_ctx: *mut *mut c_void) -> Result<()> {
    params.check_type(
        ParamTypeFlags::None,
        ParamTypeFlags::None,
        ParamTypeFlags::None,
        ParamTypeFlags::None,
    )?;
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
        TA_HOTP_CMD_REGISTER_SHARED_KEY => {
            return register_shared_key(params);
        }
        TA_HOTP_CMD_GET_HOTP => {
            return get_hotp(params);
        }
        _ => {
            return Err(Error::from_raw_error(TEE_ERROR_BAD_PARAMETERS));
        }
    }
}

pub fn register_shared_key(params: &mut Parameters) -> Result<()> {
    params.check_type(
        ParamTypeFlags::MemrefInput,
        ParamTypeFlags::None,
        ParamTypeFlags::None,
        ParamTypeFlags::None,
    )?;

    unsafe {
        K_LEN = (*params.param_0.raw).memref.size;
        let tmp: *mut [u8; MAX_KEY_SIZE as usize] = (*params.param_0.raw).memref.buffer as *mut _;
        for i in 0..K_LEN {
            k[i as usize] = (*tmp)[i as usize];
        }
        //trace_println!("[+] Got shared key {}, whose size is {}.", k, K_LEN);
    }
    Ok(())
}

pub fn get_hotp(params: &mut Parameters) -> Result<()> {
    params.check_type(
        ParamTypeFlags::ValueOutput,
        ParamTypeFlags::None,
        ParamTypeFlags::None,
        ParamTypeFlags::None,
    )?;

    let mut mac: [u8; SHA1_HASH_SIZE as usize] = [0x0; SHA1_HASH_SIZE as usize];
    let mut mac_len: u32 = SHA1_HASH_SIZE;
    let mut hotp_val: u32 = 0;

    unsafe {
        hmac_sha1(&mut mac, &mut mac_len)?;
        for i in (0..counter.len()).rev() {
            counter[i] += 1;
            if counter[i] > 0 {
                break;
            }
        }
        truncate(&mut mac, &mut hotp_val)?;
        (*params.param_0.raw).value.a = hotp_val;
    }
    Ok(())
}

pub fn hmac_sha1(out: *mut [u8; SHA1_HASH_SIZE as usize], outlen: *mut u32) -> Result<()> {
    let mut attr = TEE_Attribute {
        attributeID: 0,
        content: content {
            memref: Memref {
                buffer: 0 as *mut _,
                size: 0,
            },
        },
    };

    let mut key_handle: TEE_ObjectHandle = TEE_HANDLE_NULL as *mut _;
    let mut op_handle: TEE_OperationHandle = TEE_HANDLE_NULL as *mut _;

    unsafe {
        if K_LEN < MIN_KEY_SIZE || K_LEN > MAX_KEY_SIZE {
            return Err(Error::from_raw_error(TEE_ERROR_BAD_PARAMETERS));
        }

        //original code check counter pointer which is useless here
        if out.is_null() || outlen.is_null() {
            return Err(Error::from_raw_error(TEE_ERROR_BAD_PARAMETERS));
        }

        let mut res = TEE_AllocateOperation(
            &mut op_handle,
            TEE_ALG_HMAC_SHA1,
            TEE_OperationMode::TEE_MODE_MAC as u32,
            K_LEN * 8,
        );

        'correct_handle: loop {
            if res != TEE_SUCCESS {
                break 'correct_handle;
            }

            res = TEE_AllocateTransientObject(TEE_TYPE_HMAC_SHA1, K_LEN * 8, &mut key_handle);
            if res != TEE_SUCCESS {
                break 'correct_handle;
            }

            TEE_InitRefAttribute(
                &mut attr,
                TEE_ATTR_SECRET_VALUE,
                &mut k as *mut [u8; MAX_KEY_SIZE as usize] as *mut _,
                K_LEN,
            );
            res = TEE_PopulateTransientObject(key_handle, &mut attr, 1);
            if res != TEE_SUCCESS {
                break 'correct_handle;
            }

            res = TEE_SetOperationKey(op_handle, key_handle);
            if res != TEE_SUCCESS {
                break 'correct_handle;
            }

            TEE_MACInit(op_handle, ptr::null() as *const _, 0);
            TEE_MACUpdate(
                op_handle,
                &mut counter as *mut [u8; 8] as *mut _,
                counter.len() as u32,
            );

            res = TEE_MACComputeFinal(op_handle, ptr::null() as *const _, 0, out as *mut _, outlen);
            break 'correct_handle;
        }

        if op_handle != TEE_HANDLE_NULL as *mut _ {
            TEE_FreeOperation(op_handle);
        }
        TEE_FreeTransientObject(key_handle);
        if res == TEE_SUCCESS {
            return Ok(());
        } else {
            return Err(Error::from_raw_error(res));
        }
    }
}

pub fn truncate(hmac_result: *mut [u8; SHA1_HASH_SIZE as usize], bin_code: *mut u32) -> Result<()> {
    unsafe {
        let offset: usize = ((*hmac_result)[19] & 0xf) as usize;

        *bin_code = (((*hmac_result)[offset] & 0x7f) as u32) << 24
            | (((*hmac_result)[offset + 1] & 0xff) as u32) << 16
            | (((*hmac_result)[offset + 2] & 0xff) as u32) << 8
            | (((*hmac_result)[offset + 3] & 0xff) as u32);

        *bin_code %= DBC2_MODULO;
    }
    Ok(())
}

const ta_name: &str = "HMAC OTP";

const TA_FLAGS: uint32_t = TA_FLAG_EXEC_DDR;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 32 * 1024;
const EXT_PROP_VALUE_1: &[u8] = b"HMAC OTP\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
