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

use optee_teec_sys as raw;
use libc::{c_char};
use crate::{Result, Error, ErrorKind};

#[repr(C)]
pub struct PluginMethod {
    pub name: *const c_char,
    pub uuid: raw::TEEC_UUID,
    pub init: fn() -> Result<()>,
    pub invoke: fn(
        cmd: u32,
        sub_cmd: u32,
        data: *mut c_char,
        in_len: u32,
        out_len: *mut u32,
    ) -> Result<()>,
}

/// struct PluginParameters {
/// @cmd: u32,          plugin cmd, defined in proto/
/// @sub_cmd: u32,      plugin subcmd, defined in proto/
/// @inbuf: &'a [u8],   input buffer sent from TA
/// @outbuf: Vec<u8>,   output buffer sent from plugin to TA,
///                     outlen SHOULD be less than or equal to inlen
/// }
pub struct PluginParameters<'a> {
    pub cmd: u32,
    pub sub_cmd: u32,
    pub inbuf: &'a [u8],
    outbuf: Vec<u8>,
}
impl<'a> PluginParameters<'a> {
    pub fn new(cmd: u32, sub_cmd: u32, inbuf: &'a [u8]) -> Self {
        let mut outbuf = vec![0u8; inbuf.len() as usize];
        Self {
            cmd,
            sub_cmd,
            inbuf,
            outbuf,
        }
    }
    pub fn set_outbuf_from_slice(&mut self, sendslice: &[u8]) -> Result<()> {
        if self.inbuf.len() < sendslice.len() {
            println!("Overflow: Input length is less than output length");
            return Err(Error::new(ErrorKind::Security));
        }
        self.outbuf[..sendslice.len()].copy_from_slice(&sendslice);
        Ok(())
    }
    pub fn get_outbuf_as_slice(&self) -> &[u8] {
        self.outbuf.as_slice()
    }
}
