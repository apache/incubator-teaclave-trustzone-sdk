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

#![no_std]
#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result, Uuid};
use optee_utee::{TaSession, TeeParamMemref, TeeParamNone, TeeParamValue, TeeParameters};
use optee_utee_sys as raw;
use proto::Command;

const SYSTEM_PTA_UUID: &str = "3a2f8978-5dc0-11e8-9c2d-fa7ae01bbebc";
const SYSTEM_PTA_CMD_DERIVE_TA_UNIQUE_KEY: u32 = 1;
const HELLO_WORLD_USER_TA_UUID: &str = "133af0ca-bdab-11eb-9130-43bf7873bf67";
const HELLO_WORLD_USER_TA_CMD_INC_VALUE: u32 = 0;

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

fn test_invoke_system_pta() -> Result<()> {
    let system_pta_uuid =
        Uuid::parse_str(SYSTEM_PTA_UUID).map_err(|_| Error::from(ErrorKind::BadParameters))?;
    let timeout = raw::TEE_TIMEOUT_INFINITE;
    let mut session = TaSession::new(system_pta_uuid, timeout)?;
    trace_println!("[+] TA open PTA session success");

    let mut input: [u8; 32] = [0; 32];
    let mut output: [u8; 32] = [0; 32];

    let param_in = TeeParamMemref::new_input(&mut input);
    let param_out = TeeParamMemref::new_output(&mut output);

    let mut parameters = TeeParameters::new(param_in, param_out, TeeParamNone, TeeParamNone);

    session.invoke_command(
        timeout,
        SYSTEM_PTA_CMD_DERIVE_TA_UNIQUE_KEY,
        &mut parameters,
    )?;

    let output = parameters.parameters().1.buffer().to_vec();
    if output == [0; 32].to_vec() {
        return Err(Error::new(ErrorKind::Generic));
    }
    trace_println!("[+] TA invoke PTA command success, output: {:?}", output);

    Ok(())
}

fn test_invoke_hello_world_user_ta() -> Result<()> {
    let hello_world_user_ta_uuid = Uuid::parse_str(HELLO_WORLD_USER_TA_UUID)
        .map_err(|_| Error::from(ErrorKind::BadParameters))?;
    let timeout = raw::TEE_TIMEOUT_INFINITE;
    let mut session = TaSession::new(hello_world_user_ta_uuid, timeout)?;
    trace_println!("[+] TA open user TA session success");

    let param_inout = TeeParamValue::new_inout(29, 0);
    let mut parameters = TeeParameters::new(param_inout, TeeParamNone, TeeParamNone, TeeParamNone);

    session.invoke_command(timeout, HELLO_WORLD_USER_TA_CMD_INC_VALUE, &mut parameters)?;

    let output = parameters.parameters().0.a();
    if output != 129 {
        return Err(Error::new(ErrorKind::Generic));
    }
    trace_println!("[+] TA invoke user TA command success",);

    Ok(())
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, _params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Test => {
            test_invoke_system_pta()?;
            test_invoke_hello_world_user_ta()?;
            trace_println!("[+] Test passed");
            Ok(())
        }
        _ => {
            return Err(Error::new(ErrorKind::NotSupported));
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
