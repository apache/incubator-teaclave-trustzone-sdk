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

use optee_utee::net::TcpStream;
use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;
use rustls::{OwnedTrustAnchor, RootCertStore};
use std::convert::TryInto;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;

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
fn invoke_command(cmd_id: u32, _params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Start => {
            tls_client();
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

// copied from https://github.com/rustls/rustls/blob/v/0.21.0/examples/src/bin/simpleclient.rs
fn tls_client() {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    trace_println!("[+] root_store added");

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    trace_println!("[+] config created");

    let server_name = "google.com".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
    let mut sock = TcpStream::connect("google.com", 443).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    tls.write_all(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    let ciphersuite = tls.conn.negotiated_cipher_suite().unwrap();
    trace_println!("Current ciphersuite: {:?}", ciphersuite.suite());
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    trace_println!("{}", String::from_utf8_lossy(&plaintext));
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
