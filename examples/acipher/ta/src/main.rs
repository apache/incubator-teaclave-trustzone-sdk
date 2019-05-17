#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{AlgorithmId, Asymmetric, OperationMode};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use optee_utee::{TransientObject, TransientObjectType};
use proto::Command;

pub struct RsaCipher {
    pub key: TransientObject,
}

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters, sess_ctx: *mut *mut RsaCipher) -> Result<()> {
    trace_println!("[+] TA open session");
    let ptr = Box::into_raw(Box::new(RsaCipher {
        key: TransientObject::null_object(),
    }));
    unsafe {
        *sess_ctx = ptr;
    }
    Ok(())
}

#[ta_close_session]
fn close_session(sess_ctx: *mut RsaCipher) {
    trace_println!("[+] TA close session");
    unsafe { Box::from_raw(sess_ctx) };
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

fn gen_key(rsa: &mut RsaCipher, params: &mut Parameters) -> Result<()> {
    let key_size = unsafe { params.0.as_value().unwrap().a() };
    rsa.key =
        TransientObject::allocate(TransientObjectType::RsaKeypair, key_size as usize).unwrap();
    rsa.key.generate_key(key_size as usize, &[])?;
    Ok(())
}

fn get_size(rsa: &mut RsaCipher, params: &mut Parameters) -> Result<()> {
    let key_info = rsa.key.info().unwrap();
    unsafe {
        params
            .0
            .as_value()
            .unwrap()
            .set_a((key_info.object_size() / 8) as u32)
    };
    Ok(())
}

fn encrypt(rsa: &mut RsaCipher, params: &mut Parameters) -> Result<()> {
    let key_info = rsa.key.info().unwrap();
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let plain_text = p0.buffer();
    let mut p1 = unsafe { params.1.as_memref().unwrap() };
    match Asymmetric::allocate(
        AlgorithmId::RsaesPkcs1V15,
        OperationMode::Encrypt,
        key_info.object_size(),
    ) {
        Err(e) => Err(e),
        Ok(cipher) => {
            cipher.set_key(&rsa.key)?;
            match cipher.encrypt(&[], &plain_text) {
                Err(e) => Err(e),
                Ok(cipher_text) => Ok(p1.buffer().clone_from_slice(&cipher_text)),
            }
        }
    }
}

fn decrypt(rsa: &mut RsaCipher, params: &mut Parameters) -> Result<()> {
    let key_info = rsa.key.info().unwrap();
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let mut cipher_text = p0.buffer();
    let mut p1 = unsafe { params.1.as_memref().unwrap() };
    match Asymmetric::allocate(
        AlgorithmId::RsaesPkcs1V15,
        OperationMode::Decrypt,
        key_info.object_size(),
    ) {
        Err(e) => Err(e),
        Ok(cipher) => {
            cipher.set_key(&rsa.key)?;
            match cipher.decrypt(&mut [], &mut cipher_text) {
                Err(e) => Err(e),
                Ok(plain_text) => Ok(p1.buffer().clone_from_slice(&plain_text)),
            }
        }
    }
}

#[ta_invoke_command]
fn invoke_command(sess_ctx: &mut RsaCipher, cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::GenKey => gen_key(sess_ctx, params),
        Command::GetSize => get_size(sess_ctx, params),
        Command::Encrypt => encrypt(sess_ctx, params),
        Command::Decrypt => decrypt(sess_ctx, params),
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"Example of TA using asymmetric cipher.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Acipher TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
