// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![no_main]

use ring::signature::KeyPair;
use ring::{rand, signature};

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;

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
        Command::Sign => {
            return sign(params);
        }
        Command::Verify => {
            return verify(params);
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

fn sign(params: &mut Parameters) -> Result<()> {
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let mut p1 = unsafe { params.1.as_memref().unwrap() };
    let mut p2 = unsafe { params.2.as_memref().unwrap() };
    let message = p0.buffer();
    trace_println!("[+] message: {:?}", &message);

    // Generate a key pair in PKCS#8 (v2) format.
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = match signature::Ed25519KeyPair::generate_pkcs8(&rng) {
        Ok(bytes) => bytes,
        Err(e) => {
            trace_println!("[+] error: {:?}", e);
            return Err(Error::new(ErrorKind::Generic));
        }
    };
    trace_println!("[+] pkcs8_bytes: {:?}", pkcs8_bytes.as_ref());

    let key_pair = match signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()) {
        Ok(key_pair) => key_pair,
        Err(e) => {
            trace_println!("[+] error: {:?}", e);
            return Err(Error::new(ErrorKind::Generic));
        }
    };
    let sig = key_pair.sign(message);
    trace_println!("[+] public key: {:?}", key_pair.public_key().as_ref());
    trace_println!("[+] signature: {:?}", sig.as_ref());

    p1.buffer().clone_from_slice(key_pair.public_key().as_ref());
    p2.buffer().clone_from_slice(sig.as_ref());

    Ok(())
}

fn verify(params: &mut Parameters) -> Result<()> {
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    let mut p1 = unsafe { params.1.as_memref().unwrap() };
    let mut p2 = unsafe { params.2.as_memref().unwrap() };

    let message = p0.buffer();
    let public_key_bytes = p1.buffer();
    let sig = p2.buffer();
    trace_println!("[+] message: {:?}", &message);
    trace_println!("[+] public_key: {:?}", &public_key_bytes);
    trace_println!("[+] signature: {:?}", &sig);

    // Verify the signature of the message using the public key.
    let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key_bytes);
    match public_key.verify(message, sig) {
        Ok(_) => {
            trace_println!("[+] verify ok");
            return Ok(());
        }
        Err(e) => {
            trace_println!("[+] error: {:?}", e);
            return Err(Error::new(ErrorKind::SignatureInvalid));
        }
    };
}

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 4 * 1024 * 1024;
const TA_STACK_SIZE: u32 = 4 * 1024;
const TA_VERSION: &[u8] = b"0.2\0";
const TA_DESCRIPTION: &[u8] = b"This is a signature verification example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Signature Verification TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
