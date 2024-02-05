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

use crate::{Error, Result, Uuid};
use optee_utee_sys as raw;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;

pub struct LoadablePlugin {
    uuid: Uuid
}

impl LoadablePlugin {
    pub fn new(uuid: &Uuid) -> Self {
        Self { uuid: uuid.to_owned() }
    }
    pub fn invoke(&mut self, command_id: u32, subcommand_id: u32, data: &[u8]) -> Result<Vec<u8>> {
        let raw_uuid: Uuid = self.uuid;
        let mut outlen: usize = 0;
        match unsafe {
            raw::tee_invoke_supp_plugin(
                raw_uuid.as_raw_ptr(),
                command_id as u32,
                subcommand_id as u32,
                data.as_ptr() as _,
                data.len(),
                &mut outlen as *mut usize,
            )
        } {
            raw::TEE_SUCCESS => {
                assert!(outlen <= (data.len()));
                let mut outbuf = vec![0; outlen];
                outbuf.copy_from_slice(&data[..outlen]);
                
                Ok(outbuf)
            },
            code => Err(Error::from_raw_error(code)),
        }
       
    }
}
