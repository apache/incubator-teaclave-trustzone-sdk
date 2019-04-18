#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{AlgorithmId, Operation, OperationMode};
use optee_utee::{Attribute, AttributeId, TransientObject, TransientObjectType};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::*;
use std::boxed::Box;

pub const AES128_KEY_BIT_SIZE: u32 = 128;
pub const AES128_KEY_BYTE_SIZE: u32 = AES128_KEY_BIT_SIZE / 8;
pub const AES256_KEY_BIT_SIZE: u32 = 256;
pub const AES256_KEY_BYTE_SIZE: u32 = AES256_KEY_BIT_SIZE / 8;

pub struct AesCipher {
    pub key_size: usize,
    pub operation: Operation,
    pub key_object: TransientObject,
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
        key_size: 0,
        operation: Operation::null_operation(),
        key_object: TransientObject::null_object(),
    }));
    unsafe {
        *sess_ctx = ptr;
    }
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destory");
}

#[ta_invoke_command]
fn invoke_command(sess_ctx: &mut AesCipher, cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Prepare => {
            return alloc_resources(sess_ctx, params);
        }
        Command::SetKey => {
            return set_aes_key(sess_ctx, params);
        }
        Command::SetIV => {
            return reset_aes_iv(sess_ctx, params);
        }
        Command::Cipher => {
            return cipher_buffer(sess_ctx, params);
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn ta2tee_algo_id(algo_id: u32) -> Result<AlgorithmId> {
    match Algo::from(algo_id) {
        Algo::ECB => Ok(AlgorithmId::AesEcbNopad),
        Algo::CBC => Ok(AlgorithmId::AesCbcNopad),
        Algo::CTR => Ok(AlgorithmId::AesCtr),
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

pub fn ta2tee_key_size(key_sz: u32) -> Result<usize> {
    match key_sz {
        AES128_KEY_BYTE_SIZE | AES256_KEY_BYTE_SIZE => Ok(key_sz as usize),
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

pub fn ta2tee_mode_id(mode: u32) -> Result<OperationMode> {
    match Mode::from(mode) {
        Mode::Encode => Ok(OperationMode::Encrypt),
        Mode::Decode => Ok(OperationMode::Decrypt),
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

pub fn alloc_resources(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let algo_value = unsafe { params.0.as_value().unwrap().a() };
    let key_size_value = unsafe { params.1.as_value().unwrap().a() };
    let mode_id_value = unsafe { params.2.as_value().unwrap().a() };

    aes.key_size = ta2tee_key_size(key_size_value).unwrap();

    aes.operation = Operation::allocate(
        ta2tee_algo_id(algo_value).unwrap(),
        ta2tee_mode_id(mode_id_value).unwrap(),
        aes.key_size * 8,
    )
    .unwrap();
    aes.key_object = TransientObject::allocate(TransientObjectType::Aes, aes.key_size * 8).unwrap();
    let mut key = vec![0u8; aes.key_size as usize];
    let attr = Attribute::from_ref(AttributeId::SecretValue, &mut key);
    aes.key_object.populate(&mut [attr])?;
    aes.operation.set_key(&mut aes.key_object)?;
    Ok(())
}

pub fn set_aes_key(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let mut param0 = unsafe { params.0.as_memref().unwrap() };
    let mut key = param0.buffer();

    if key.len() != aes.key_size {
        trace_println!("[+] Get wrong key size !\n");
        return Err(Error::new(ErrorKind::BadParameters));
    }

    let attr = Attribute::from_ref(AttributeId::SecretValue, &mut key);

    aes.key_object.reset();
    aes.key_object.populate(&mut [attr])?;

    aes.operation.set_key(&mut aes.key_object)?;
    Ok(())
}

pub fn reset_aes_iv(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let mut param0 = unsafe { params.0.as_memref().unwrap() };
    let iv = param0.buffer();

    aes.operation.cipher_init(iv);

    trace_println!("[+] TA initial vectore reset done!");
    Ok(())
}

pub fn cipher_buffer(aes: &mut AesCipher, params: &mut Parameters) -> Result<()> {
    let mut param0 = unsafe { params.0.as_memref().unwrap() };
    let mut param1 = unsafe { params.1.as_memref().unwrap() };

    let input = param0.buffer();
    let output = param1.buffer();

    if output.len() < input.len() {
        return Err(Error::new(ErrorKind::BadParameters));
    }

    trace_println!("[+] TA tries to update ciphers!");

    unsafe { (*param1.raw()).size = aes.operation.cipher_update(input, output).unwrap() as u32 };
    Ok(())
}

const TA_FLAGS: u32 = 0;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_DATA_SIZE: u32 = 1 * 1024 * 1024;
const TA_VERSION: &[u8] = b"Undefined version\0";
const TA_DESCRIPTION: &[u8] = b"This is an AES example\0";
const EXT_PROP_VALUE_1: &[u8] = b"AES TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
