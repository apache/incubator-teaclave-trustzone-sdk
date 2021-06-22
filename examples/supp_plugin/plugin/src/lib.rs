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

use libc::{c_char};
use optee_teec::{Error, ErrorKind, Plugin_Method};
use proto::{PluginCommand};

#[no_mangle]
fn syslog_plugin_init() -> optee_teec::Result<()> {
    println!("*plugin*: init");

    Ok(())
}

#[no_mangle]
fn syslog_plugin_invoke(
    cmd: u32, 
    sub_cmd: u32, 
    data: *mut c_char, 
    in_len: u32, 
    out_len: *mut u32
) -> optee_teec::Result<()> {
    println!("*plugin*: invoke");
    match PluginCommand::from(cmd) {
        PluginCommand::Print => {
            let received_slice = unsafe { std::slice::from_raw_parts(data, in_len as usize) };
            println!("*plugin*: receive value: {:?} length {:?}", received_slice, in_len);

            let send_slice: [u8;10] = [0x40;10];
            unsafe { 
                *out_len = send_slice.len() as u32;
                std::ptr::copy(send_slice.as_ptr(), data, send_slice.len());
                println!("*plugin*: send value: {:?} length {:?} to ta", send_slice, *out_len);
            };
            
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::NotSupported)),
    }
}


include!(concat!(env!("OUT_DIR"), "/plugin_static.rs"));
