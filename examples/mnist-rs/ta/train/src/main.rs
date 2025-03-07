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
extern crate alloc;

use burn::backend::{ndarray::NdArrayDevice, Autodiff, NdArray};
use common::copy_to_output;
use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{ErrorKind, Parameters, Result};
use proto::train::Command;
use spin::Mutex;

mod trainer;

type NoStdTrainer = trainer::Trainer<Autodiff<NdArray>>;

const DEVICE: NdArrayDevice = NdArrayDevice::Cpu;
static TRAINER: Mutex<Option<NoStdTrainer>> = Mutex::new(Option::None);

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(params: &mut Parameters) -> Result<()> {
    let mut p0 = unsafe { params.0.as_memref()? };

    let learning_rate = f64::from_le_bytes(p0.buffer().try_into().map_err(|err| {
        trace_println!("bad parameter {:?}", err);
        ErrorKind::BadParameters
    })?);
    trace_println!("Initialize with learning_rate: {}", learning_rate);

    let mut trainer = TRAINER.lock();
    trainer.replace(NoStdTrainer::new(DEVICE, learning_rate));

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
    match Command::try_from(cmd_id) {
        Ok(Command::Train) => {
            let mut p0 = unsafe { params.0.as_memref()? };
            let mut p1 = unsafe { params.1.as_memref()? };

            let images = p0.buffer();
            let labels = p1.buffer();

            let mut trainer = TRAINER.lock();
            let result = trainer
                .as_mut()
                .ok_or(ErrorKind::CorruptObject)?
                .train(bytemuck::cast_slice(images), labels);
            let bytes = serde_json::to_vec(&result).map_err(|err| {
                trace_println!("unexpected error: {:?}", err);
                ErrorKind::BadState
            })?;

            copy_to_output(&mut params.2, &bytes)
        }
        Ok(Command::Valid) => {
            let mut p0 = unsafe { params.0.as_memref()? };
            let mut p1 = unsafe { params.1.as_memref()? };

            let images = p0.buffer();
            let labels = p1.buffer();

            let trainer = TRAINER.lock();
            let result = trainer
                .as_ref()
                .ok_or(ErrorKind::CorruptObject)?
                .valid(bytemuck::cast_slice(images), labels);

            let bytes = serde_json::to_vec(&result).map_err(|err| {
                trace_println!("unexpected error: {:?}", err);
                ErrorKind::BadState
            })?;
            copy_to_output(&mut params.2, &bytes)
        }
        Ok(Command::Export) => {
            let trainer = TRAINER.lock();
            let result = trainer
                .as_ref()
                .ok_or(ErrorKind::CorruptObject)?
                .export()
                .map_err(|err| {
                    trace_println!("unexpected error: {:?}", err);
                    ErrorKind::BadState
                })?;
            copy_to_output(&mut params.0, &result)
        }
        Err(_) => Err(ErrorKind::BadParameters.into()),
    }
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
