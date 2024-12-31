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

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{BufReader, Cursor, Read, Write};
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref TLS_SESSIONS: RwLock<HashMap<u32, Mutex<rustls::ServerConnection>>> =
        RwLock::new(HashMap::new());
}

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
    let session_id = unsafe { params.0.as_value().unwrap().a() };
    trace_println!("[+] session id: {}", session_id);
    match Command::from(cmd_id) {
        Command::NewTlsSession => {
            trace_println!("[+] new_tls_session");
            new_tls_session(session_id);
            Ok(())
        }
        Command::DoTlsRead => {
            let mut p1 = unsafe { params.1.as_memref().unwrap() };
            let buffer = p1.buffer();
            trace_println!("[+] do_tls_read");
            do_tls_read(session_id, buffer);
            Ok(())
        }
        Command::DoTlsWrite => {
            trace_println!("[+] do_tls_write");
            let mut p1 = unsafe { params.1.as_memref().unwrap() };
            let mut p2 = unsafe { params.2.as_value().unwrap() };
            let mut buffer = p1.buffer();
            let n = do_tls_write(session_id, &mut buffer);
            p2.set_a(n as u32);
            Ok(())
        }
        Command::CloseTlsSession => {
            trace_println!("[+] close_tls_session");
            close_tls_session(session_id);
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

pub fn new_tls_session(session_id: u32) {
    let tls_config = make_config();
    let tls_session = rustls::ServerConnection::new(tls_config).unwrap();
    TLS_SESSIONS
        .write()
        .unwrap()
        .insert(session_id, Mutex::new(tls_session));
}

pub fn close_tls_session(session_id: u32) {
    TLS_SESSIONS.write().unwrap().remove(&session_id);
}

pub fn do_tls_read(session_id: u32, buf: &[u8]) {
    let mut rd = Cursor::new(buf);
    let ts_guard = TLS_SESSIONS.read().unwrap();
    let mut tls_session = ts_guard.get(&session_id).unwrap().lock().unwrap();
    let _rc = tls_session.read_tls(&mut rd).unwrap();
    let _processed = tls_session.process_new_packets().unwrap();

    // Read and process all available plaintext.
    let mut buf = Vec::new();
    let _rc = tls_session.reader().read_to_end(&mut buf);
    if !buf.is_empty() {
        tls_session.writer().write_all(&buf).unwrap();
    }
}

pub fn do_tls_write(session_id: u32, buf: &mut [u8]) -> usize {
    let ts_guard = TLS_SESSIONS.read().unwrap();
    let mut tls_session = ts_guard.get(&session_id).unwrap().lock().unwrap();
    let mut wr = Cursor::new(buf);
    let mut rc = 0;
    while tls_session.wants_write() {
        rc += tls_session.write_tls(&mut wr).unwrap();
    }

    rc
}

fn make_config() -> Arc<rustls::ServerConfig> {
    let certs = load_certs();
    let privkey = load_private_key();
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, privkey)
        .unwrap();

    Arc::new(config)
}

fn load_certs() -> Vec<rustls::Certificate> {
    let bytes = include_bytes!("../test-ca/ecdsa/end.fullchain").to_vec();
    let cursor = std::io::Cursor::new(bytes);
    let mut reader = BufReader::new(cursor);
    let certs = rustls_pemfile::certs(&mut reader).unwrap();
    certs
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

fn load_private_key() -> rustls::PrivateKey {
    let bytes = include_bytes!("../test-ca/ecdsa/end.key").to_vec();
    let cursor = std::io::Cursor::new(bytes);
    let mut reader = BufReader::new(cursor);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader).unwrap();
    assert_eq!(keys.len(), 1);
    rustls::PrivateKey(keys[0].clone())
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
