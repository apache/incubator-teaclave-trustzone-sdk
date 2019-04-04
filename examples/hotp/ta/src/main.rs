#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use optee_utee_sys::*;
use std::ptr;

pub const SHA1_HASH_SIZE: u32 = 20;

pub const MAX_KEY_SIZE: u32 = 64;
pub const MIN_KEY_SIZE: u32 = 10;

pub const DBC2_MODULO: u32 = 1000000;

pub static mut K: [u8; MAX_KEY_SIZE as usize] = [0; MAX_KEY_SIZE as usize];
pub static mut K_LEN: u32 = 0;

pub static mut COUNTER: [u8; 8] = [0x0; 8];

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::RegisterSharedKey => {
            return register_shared_key(params);
        }
        Command::GetHOTP => {
            return get_hotp(params);
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn register_shared_key(params: &mut Parameters) -> Result<()> {
    unsafe {
        K_LEN = (*params.first().raw).memref.size;
        let tmp: *mut [u8; MAX_KEY_SIZE as usize] = (*params.first().raw).memref.buffer as *mut _;
        for i in 0..K_LEN {
            K[i as usize] = (*tmp)[i as usize];
        }
    }
    Ok(())
}

pub fn get_hotp(params: &mut Parameters) -> Result<()> {
    let mut mac: [u8; SHA1_HASH_SIZE as usize] = [0x0; SHA1_HASH_SIZE as usize];
    let mut mac_len: u32 = SHA1_HASH_SIZE;
    let mut hotp_val: u32 = 0;

    unsafe {
        hmac_sha1(&mut mac, &mut mac_len)?;
        for i in (0..COUNTER.len()).rev() {
            COUNTER[i] += 1;
            if COUNTER[i] > 0 {
                break;
            }
        }
        truncate(&mut mac, &mut hotp_val)?;
        (*params.first().raw).value.a = hotp_val;
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
            return Err(Error::new(ErrorKind::BadParameters));
        }

        //original code check COUNTER pointer which is useless here
        if out.is_null() || outlen.is_null() {
            return Err(Error::new(ErrorKind::BadParameters));
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
                &mut K as *mut [u8; MAX_KEY_SIZE as usize] as *mut _,
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
                &mut COUNTER as *mut [u8; 8] as *mut _,
                COUNTER.len() as u32,
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

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"This is an HOTP example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"HOTP TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
