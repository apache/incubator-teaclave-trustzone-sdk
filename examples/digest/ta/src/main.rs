#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{AlgorithmId, Digest};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;

pub struct DigestOp {
    pub op: Digest,
}

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters, sess_ctx: *mut *mut DigestOp) -> Result<()> {
    trace_println!("[+] TA open session");
    let ptr = Box::into_raw(Box::new(DigestOp {
        op: Digest::allocate(AlgorithmId::Sha256).unwrap(),
    }));
    unsafe {
        *sess_ctx = ptr;
    }
    Ok(())
}

#[ta_close_session]
fn close_session(sess_ctx: *mut DigestOp) {
    trace_println!("[+] TA close session");
    unsafe { Box::from_raw(sess_ctx) };
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(sess_ctx: &mut DigestOp, cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Update => {
            return update(sess_ctx, params);
        }
        Command::DoFinal => {
            return do_final(sess_ctx, params);
        }
        _ => {
            return Err(Error::new(ErrorKind::BadParameters));
        }
    }
}

pub fn update(digest: &mut DigestOp, params: &mut Parameters) -> Result<()> {
    let mut p = unsafe { params.0.as_memref().unwrap() };
    let buffer = p.buffer();
    digest.op.update(buffer);
    Ok(())
}

pub fn do_final(digest: &mut DigestOp, params: &mut Parameters) -> Result<()> {
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let mut p1 = unsafe { params.1.as_memref().unwrap() };
    let mut p2 = unsafe { params.2.as_value().unwrap() };
    let input = p0.buffer();
    let output = p1.buffer();
    match digest.op.do_final(input, output) {
        Err(e) => Err(e),
        Ok(hash_length) => {
            p2.set_a(hash_length as u32);
            Ok(())
        }
    }
}

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"This is a message digest example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Digest TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
