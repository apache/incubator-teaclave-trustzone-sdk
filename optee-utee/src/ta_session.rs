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

use core::ptr;
use optee_utee_sys as raw;

use crate::{Error, Result, TeeParam, TeeParameters, Uuid};

/// Represents a connection between a trusted application and another trusted application (can be user TA or pseudo TA).
pub struct TaSession {
    raw: raw::TEE_TASessionHandle,
}

impl TaSession {
    /// Initializes a new TA Session.
    pub fn new(uuid: Uuid, timeout: u32) -> Result<Self> {
        // let mut raw_session = raw::TEE_HANDLE_NULL;
        let mut raw_session: raw::TEE_TASessionHandle = ptr::null_mut();
        let mut err_origin: u32 = 0;
        unsafe {
            match raw::TEE_OpenTASession(
                uuid.as_raw_ptr(),
                timeout,
                0,
                core::ptr::null_mut(),
                &mut raw_session,
                &mut err_origin,
            ) {
                raw::TEE_SUCCESS => Ok(Self { raw: raw_session }),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    /// Initializes a new TA Session with parameters.
    pub fn new_with_params<A: TeeParam, B: TeeParam, C: TeeParam, D: TeeParam>(
        uuid: Uuid,
        timeout: u32,
        params: &mut TeeParameters<A, B, C, D>,
    ) -> Result<Self> {
        let mut raw_session: raw::TEE_TASessionHandle = ptr::null_mut();
        let mut err_origin: u32 = 0;
        unsafe {
            match raw::TEE_OpenTASession(
                uuid.as_raw_ptr(),
                timeout,
                params.raw_param_types(),
                params.raw().as_mut_ptr(),
                &mut raw_session,
                &mut err_origin,
            ) {
                raw::TEE_SUCCESS => Ok(Self { raw: raw_session }),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    /// Converts a TA Session to a raw pointer.
    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEE_TASessionHandle {
        &mut self.raw
    }

    /// Invokes a command with parameters.
    pub fn invoke_command<A: TeeParam, B: TeeParam, C: TeeParam, D: TeeParam>(
        &mut self,
        timeout: u32,
        command_id: u32,
        params: &mut TeeParameters<A, B, C, D>,
    ) -> Result<()> {
        let mut err_origin: u32 = 0;

        unsafe {
            match raw::TEE_InvokeTACommand(
                self.raw,
                timeout,
                command_id,
                params.raw_param_types(),
                params.raw().as_mut_ptr(),
                &mut err_origin,
            ) {
                raw::TEE_SUCCESS => Ok(()),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }
}

impl Drop for TaSession {
    fn drop(&mut self) {
        unsafe {
            raw::TEE_CloseTASession(self.raw);
        }
    }
}
