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

#![cfg_attr(not(target_os = "optee"), no_std)]
#![no_main]

extern crate alloc;

use optee_utee::net::UdpSocket;
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
fn invoke_command(cmd_id: u32, _params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Start => udp_socket(),
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

#[cfg(not(target_os = "optee"))]
fn udp_socket() -> Result<()> {
    use alloc::string::String;
    use alloc::vec::Vec;
    use optee_utee::net::Setup;

    let setup = Setup::new_v4("127.0.0.1", 34254)?;
    let mut stream = UdpSocket::open(setup).map_err(|err| {
        trace_println!("failed to open due to {:?}", err);
        ErrorKind::Generic
    })?;

    stream.set_send_timeout_in_milli(10 * 1000);
    stream.send(b"[TA]: Hello, Teaclave!").map_err(|err| {
        trace_println!("failed to send due to {:?}", err);
        ErrorKind::Generic
    })?;

    let mut response = Vec::new();
    let mut chunk = [0u8; 1024];
    stream.set_recv_timeout_in_milli(10 * 1000);

    // Loop until read something.
    loop {
        match stream.recv(&mut chunk) {
            Ok(0) => continue,
            Ok(n) => {
                response.extend_from_slice(&chunk[..n]);
                break;
            }
            Err(err) => {
                trace_println!("failed to read due to {:?}", err);
                return Err(ErrorKind::Generic.into());
            }
        }
    }
    trace_println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}

#[cfg(target_os = "optee")]
// For STD version, developers can also use APIs similar to std::net::UdpSocket.
fn udp_socket() -> Result<()> {
    use std::io::{Read, Write};

    let mut stream = UdpSocket::connect("127.0.0.1", 34254).map_err(|err| {
        trace_println!("failed to connect due to {:?}", err);
        ErrorKind::Generic
    })?;
    stream.write_all(b"[TA]: Hello, Teaclave!").map_err(|err| {
        trace_println!("failed to write_all due to {:?}", err);
        ErrorKind::Generic
    })?;
    let mut response = Vec::new();
    let mut chunk = [0u8; 1024];

    // Loop until read something.
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => continue,
            Ok(n) => {
                response.extend_from_slice(&chunk[..n]);
                break;
            }
            Err(err) => {
                trace_println!("failed to read due to {:?}", err);
                return Err(ErrorKind::Generic.into());
            }
        }
    }
    trace_println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));

/// Workaround for those rustc bugs:
/// * https://github.com/rust-lang/rust/issues/47493
/// * https://github.com/rust-lang/rust/issues/56152
///
/// It shouldn't even be possible to reach this function, thanks to panic=abort,
/// but libcore is compiled with unwinding enabled and that ends up making
/// unreachable references to this.
#[cfg(not(target_os = "optee"))]
#[no_mangle]
extern "C" fn _Unwind_Resume() -> ! {
    unreachable!("Unwinding not supported");
}

#[cfg(not(target_os = "optee"))]
#[no_mangle]
extern "C" fn rust_eh_personality() -> ! {
    unreachable!("Unwinding not supported");
}
