#![no_main]

use libc::{c_char, c_int, uint32_t};
use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use optee_utee_sys::*;
use std::boxed::Box;

pub const AES128_KEY_BIT_SIZE: u32 = 128;
pub const AES128_KEY_BYTE_SIZE: u32 = AES128_KEY_BIT_SIZE / 8;
pub const AES256_KEY_BIT_SIZE: u32 = 256;
pub const AES256_KEY_BYTE_SIZE: u32 = AES256_KEY_BIT_SIZE / 8;

pub struct AesCipher {
    pub algo: u32,
    pub mode: u32,
    pub key_size: u32,
    pub op_handle: TEE_OperationHandle,
    pub key_handle: TEE_ObjectHandle,
}

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters, sess_ctx: *mut *mut AesCipher) -> Result<()> {
    trace_println!("[+] TA open session");
    let ptr = Box::into_raw(Box::new(AesCipher {
        algo: 0,
        mode: 0,
        key_size: 0,
        op_handle: std::ptr::null_mut(),
        key_handle: std::ptr::null_mut()
    }));
    unsafe { *sess_ctx = ptr; }
    Ok(())
}

#[ta_close_session]
fn close_session(sess_ctx: &mut AesCipher) {
    trace_println!("[+] TA close session");
    unsafe {
        if sess_ctx.key_handle != TEE_HANDLE_NULL as *mut _ {
            TEE_FreeTransientObject(sess_ctx.key_handle);
        }
        if sess_ctx.op_handle != TEE_HANDLE_NULL as *mut _ {
            TEE_FreeOperation(sess_ctx.op_handle);
        }
    }
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destory");
}

#[ta_invoke_command]
fn invoke_command(sess_ctx: &mut AesCipher, cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match cmd_id {
        TA_AES_CMD_PREPARE => {
            return alloc_resources(sess_ctx, params);
        }
        TA_AES_CMD_SET_KEY => {
            return set_aes_key(sess_ctx, params);
        }
        TA_AES_CMD_SET_IV => {
            return reset_aes_iv(sess_ctx, params);
        }
        TA_AES_CMD_CIPHER => {
            return cipher_buffer(sess_ctx, params);
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn ta2tee_algo_id(param: uint32_t, aes: &mut AesCipher) -> Result<()> {
    match param {
        TA_AES_ALGO_ECB => {
            aes.algo = TEE_ALG_AES_ECB_NOPAD;
            return Ok(());
        }
        TA_AES_ALGO_CBC => {
            aes.algo = TEE_ALG_AES_CBC_NOPAD;
            return Ok(());
        }
        TA_AES_ALGO_CTR => {
            aes.algo = TEE_ALG_AES_CTR;
            return Ok(());
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn ta2tee_key_size(param: uint32_t, aes: &mut AesCipher) -> Result<()> {
    match param {
        AES128_KEY_BYTE_SIZE | AES256_KEY_BYTE_SIZE => {
            aes.key_size = param;
            return Ok(());
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn ta2tee_mode_id(param: uint32_t, aes: &mut AesCipher) -> Result<()> {
    match param {
        TA_AES_MODE_ENCODE => {
            aes.mode = TEE_OperationMode::TEE_MODE_ENCRYPT as uint32_t;
            return Ok(());
        }
        TA_AES_MODE_DECODE => {
            aes.mode = TEE_OperationMode::TEE_MODE_DECRYPT as uint32_t;
            return Ok(());
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn alloc_resources(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {

    let algo_value = params.0.get_value_a()?;
    let key_size_value = params.1.get_value_a()?;
    let mode_id_value = params.2.get_value_a()?;

    ta2tee_algo_id(algo_value, aes)?;
    ta2tee_key_size(key_size_value, aes)?;
    ta2tee_mode_id(mode_id_value, aes)?;

    if aes.op_handle != TEE_HANDLE_NULL as *mut _ {
        unsafe { TEE_FreeOperation(aes.op_handle) };
    }

    let mut res: TEE_Result =
        unsafe { TEE_AllocateOperation(&mut aes.op_handle, aes.algo, aes.mode, aes.key_size * 8) };

    'correct_handle: loop {
        if res != TEE_SUCCESS {
            trace_println!("[+] TA allocate operation failed.");
            aes.op_handle = TEE_HANDLE_NULL as *mut _;
            break 'correct_handle;
        }
        if aes.key_handle != TEE_HANDLE_NULL as *mut _ {
            unsafe { TEE_FreeTransientObject(aes.key_handle) };
        }

        res = unsafe {
            TEE_AllocateTransientObject(TEE_TYPE_AES, aes.key_size * 8, &mut aes.key_handle)
        };

        if res != TEE_SUCCESS {
            trace_println!("[+] TA allocate operation failed.");
            aes.key_handle = TEE_HANDLE_NULL as *mut _;
            break 'correct_handle;
        }

        let key: *mut c_char = unsafe { TEE_Malloc(aes.key_size, 0) as *mut _ };

        if key.is_null() {
            res = TEE_ERROR_OUT_OF_MEMORY;
            trace_println!("[+] TA allocate key failed.");
            break 'correct_handle;
        }

        let mut attr = TEE_Attribute {
            attributeID: 0,
            content: content {
                memref: Memref {
                    buffer: 0 as *mut _,
                    size: 0,
                },
            },
        };
        unsafe {
            TEE_InitRefAttribute(
                &mut attr,
                TEE_ATTR_SECRET_VALUE,
                key as *mut _,
                aes.key_size,
            )
        };

        res = unsafe { TEE_PopulateTransientObject(aes.key_handle, &mut attr, 1) };
        if res != TEE_SUCCESS {
            trace_println!("[+] TA populate transient object failed.");
            break 'correct_handle;
        }

        res = unsafe { TEE_SetOperationKey(aes.op_handle, aes.key_handle) };
        if res != TEE_SUCCESS {
            trace_println!("[+] TA set operation key failed.");
            break 'correct_handle;
        }
        return Ok(());
    }
    trace_println!("[+] Error id is {}.", res);
    if (aes.op_handle) != TEE_HANDLE_NULL as *mut _ {
        unsafe { TEE_FreeOperation(aes.op_handle) };
    }
    aes.op_handle = TEE_HANDLE_NULL as *mut _;

    if (aes.key_handle) != TEE_HANDLE_NULL as *mut _ {
        unsafe { TEE_FreeTransientObject(aes.key_handle) };
    }
    aes.key_handle = TEE_HANDLE_NULL as *mut _;

    return Err(Error::from_raw_error(res));
}

pub fn set_aes_key(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let mut attr = TEE_Attribute {
        attributeID: 0,
        content: content {
            value: Value { a: 0, b: 0 },
        },
    };

    let key = params.0.get_memref_ptr()?;
    let key_sz = params.0.get_memref_size()?;

    if key_sz != aes.key_size {
        trace_println!("[+] Get wrong key size !\n");
        return Err(Error::new(ErrorKind::BadParameters));
    }

    unsafe { TEE_InitRefAttribute(&mut attr, TEE_ATTR_SECRET_VALUE, key, key_sz) };
    unsafe { TEE_ResetTransientObject(aes.key_handle) };
    let res = unsafe { TEE_PopulateTransientObject(aes.key_handle, &mut attr, 1) };

    if res != TEE_SUCCESS {
        trace_println!("[+] TA set key failed!");
        return Err(Error::from_raw_error(res));
    } else {
        trace_println!("[+] TA set key success!");
        Ok(())
    }
}

pub fn reset_aes_iv(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let iv = params.0.get_memref_ptr()?;
    let iv_sz = params.0.get_memref_size()?;

    unsafe {
        TEE_CipherInit(aes.op_handle, iv, iv_sz);
    }

    trace_println!("[+] TA initial vectore reset done!");
    Ok(())
}

pub fn cipher_buffer(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let input_ptr = params.0.get_memref_ptr()?;
    let output_ptr = params.1.get_memref_ptr()?;
    let input_size = params.0.get_memref_size()?;
    let mut output_size = params.1.get_memref_size()?;

    if output_size < input_size {
        return Err(Error::new(ErrorKind::BadParameters));
    }

    trace_println!("[+] TA tries to update ciphers!");

    let res = unsafe {
        TEE_CipherUpdate(
            aes.op_handle,
            input_ptr,
            input_size,
            output_ptr,
            &mut output_size as *mut _,
        )
    };
    if res == TEE_SUCCESS {
        return Ok(());
    } else {
        return Err(Error::from_raw_error(res));
    }
}

const TA_FLAGS: uint32_t = TA_FLAG_EXEC_DDR;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 1 * 1024 * 1024;
const TA_VERSION: &[u8] = b"Undefined version\0";
const TA_DESCRIPTION: &[u8] = b"Undefined description\0";
const EXT_PROP_VALUE_1: &[u8] = b"Example of TA using an AES sequence\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;
const TRACE_LEVEL: c_int = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: uint32_t = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
