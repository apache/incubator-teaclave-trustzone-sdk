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

use core::mem;

use optee_utee_sys as raw;

use super::{AttributeId, ObjectInfo, UsageFlag};
use crate::{Error, Result};

/// An opaque handle on an object.
pub struct ObjectHandle {
    pub(crate) raw: *mut raw::TEE_ObjectHandle,
}

impl ObjectHandle {
    pub fn handle(&self) -> raw::TEE_ObjectHandle {
        unsafe { *(self.raw) }
    }

    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    pub fn from_raw(raw: *mut raw::TEE_ObjectHandle) -> ObjectHandle {
        Self { raw }
    }

    pub fn info(&self) -> Result<ObjectInfo> {
        let mut raw_info: raw::TEE_ObjectInfo = unsafe { mem::zeroed() };
        match unsafe { raw::TEE_GetObjectInfo1(self.handle(), &mut raw_info) } {
            raw::TEE_SUCCESS => Ok(ObjectInfo::from_raw(raw_info)),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn restrict_usage(&mut self, obj_usage: UsageFlag) -> Result<()> {
        match unsafe { raw::TEE_RestrictObjectUsage1(self.handle(), obj_usage.bits()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn ref_attribute(&self, id: AttributeId, buffer: &mut [u8]) -> Result<usize> {
        let mut size = buffer.len();
        match unsafe {
            raw::TEE_GetObjectBufferAttribute(
                self.handle(),
                id as u32,
                buffer as *mut _ as _,
                &mut size,
            )
        } {
            raw::TEE_SUCCESS => Ok(size),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn value_attribute(&self, id: u32) -> Result<(u32, u32)> {
        let mut value_a: u32 = 0;
        let mut value_b: u32 = 0;
        match unsafe {
            raw::TEE_GetObjectValueAttribute(
                self.handle(),
                id,
                &mut value_a as *mut _,
                &mut value_b as *mut _,
            )
        } {
            raw::TEE_SUCCESS => Ok((value_a, value_b)),
            code => Err(Error::from_raw_error(code)),
        }
    }
}

/// A trait for an object (transient or persistent) to return its handle.
pub trait ObjHandle {
    /// Return the handle of an object.
    fn handle(&self) -> raw::TEE_ObjectHandle;
}
