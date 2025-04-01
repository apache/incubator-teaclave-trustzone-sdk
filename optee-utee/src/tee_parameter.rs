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

use crate::{ParamType, ParamTypes};
use core::{marker::PhantomData, slice};
use optee_utee_sys as raw;

pub trait TeeParam {
    fn into_raw(&mut self) -> raw::TEE_Param;
    fn param_type(&self) -> ParamType;
    fn from_raw(raw: raw::TEE_Param, param_type: ParamType) -> Self;
}

pub struct TeeParameters<A, B, C, D> {
    raw: [raw::TEE_Param; 4],
    param_types: ParamTypes,
    phantom0: PhantomData<A>,
    phantom1: PhantomData<B>,
    phantom2: PhantomData<C>,
    phantom3: PhantomData<D>,
}

impl<A: TeeParam, B: TeeParam, C: TeeParam, D: TeeParam> TeeParameters<A, B, C, D> {
    pub fn new(mut p0: A, mut p1: B, mut p2: C, mut p3: D) -> Self {
        Self {
            raw: [p0.into_raw(), p1.into_raw(), p2.into_raw(), p3.into_raw()],
            param_types: ParamTypes::new(
                p0.param_type(),
                p1.param_type(),
                p2.param_type(),
                p3.param_type(),
            ),
            phantom0: PhantomData,
            phantom1: PhantomData,
            phantom2: PhantomData,
            phantom3: PhantomData,
        }
    }

    pub fn raw_param_types(&mut self) -> u32 {
        self.param_types.into()
    }

    pub fn raw(&mut self) -> &mut [raw::TEE_Param; 4] {
        &mut self.raw
    }

    pub fn parameters(&mut self) -> (A, B, C, D) {
        let (f0, f1, f2, f3) = self.param_types.into_flags();
        (
            A::from_raw(self.raw[0], f0),
            B::from_raw(self.raw[1], f1),
            C::from_raw(self.raw[2], f2),
            D::from_raw(self.raw[3], f3),
        )
    }
}

pub struct TeeParamValue<'parameter> {
    raw: raw::Value,
    param_type: ParamType,
    _marker: PhantomData<&'parameter mut u32>,
}

impl<'parameter> TeeParamValue<'parameter> {
    pub fn new_input(a: u32, b: u32) -> Self {
        let raw = raw::Value { a, b };
        Self {
            raw,
            param_type: ParamType::ValueInput,
            _marker: PhantomData,
        }
    }

    pub fn new_output(a: u32, b: u32) -> Self {
        let raw = raw::Value { a, b };
        Self {
            raw,
            param_type: ParamType::ValueOutput,
            _marker: PhantomData,
        }
    }

    pub fn new_inout(a: u32, b: u32) -> Self {
        let raw = raw::Value { a, b };
        Self {
            raw,
            param_type: ParamType::ValueInout,
            _marker: PhantomData,
        }
    }

    pub fn a(&self) -> u32 {
        self.raw.a
    }

    pub fn b(&self) -> u32 {
        self.raw.b
    }

    pub fn set_a(&mut self, a: u32) {
        (self.raw).a = a;
    }

    pub fn set_b(&mut self, b: u32) {
        (self.raw).b = b;
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }
}

impl TeeParam for TeeParamValue<'_> {
    fn into_raw(&mut self) -> raw::TEE_Param {
        raw::TEE_Param { value: self.raw }
    }

    fn param_type(&self) -> ParamType {
        self.param_type
    }

    fn from_raw(raw: raw::TEE_Param, param_type: ParamType) -> Self {
        Self {
            raw: unsafe { raw.value },
            param_type,
            _marker: PhantomData,
        }
    }
}

pub struct TeeParamMemref<'parameter> {
    raw: raw::Memref,
    param_type: ParamType,
    _marker: PhantomData<&'parameter mut [u8]>,
}

impl<'parameter> TeeParamMemref<'parameter> {
    pub fn new(buffer: &'parameter mut [u8], param_type: ParamType) -> Self {
        let raw = raw::Memref {
            buffer: buffer.as_mut_ptr() as *mut _,
            size: buffer.len() as usize,
        };
        Self {
            raw,
            param_type,
            _marker: PhantomData,
        }
    }

    pub fn new_input(buffer: &'parameter mut [u8]) -> Self {
        Self::new(buffer, ParamType::MemrefInput)
    }

    pub fn new_output(buffer: &'parameter mut [u8]) -> Self {
        Self::new(buffer, ParamType::MemrefOutput)
    }

    pub fn new_inout(buffer: &'parameter mut [u8]) -> Self {
        Self::new(buffer, ParamType::MemrefInout)
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.raw.buffer as *mut u8, self.raw.size as usize) }
    }
}

impl TeeParam for TeeParamMemref<'_> {
    fn into_raw(&mut self) -> raw::TEE_Param {
        raw::TEE_Param { memref: self.raw }
    }

    fn param_type(&self) -> ParamType {
        self.param_type
    }

    fn from_raw(raw: raw::TEE_Param, param_type: ParamType) -> Self {
        Self {
            raw: unsafe { raw.memref },
            param_type,
            _marker: PhantomData,
        }
    }
}

pub struct TeeParamNone;

impl TeeParam for TeeParamNone {
    fn from_raw(_raw: raw::TEE_Param, _param_type: ParamType) -> Self {
        TeeParamNone
    }

    fn into_raw(&mut self) -> raw::TEE_Param {
        raw::TEE_Param {
            memref: raw::Memref {
                buffer: core::ptr::null_mut(),
                size: 0,
            },
        }
    }

    fn param_type(&self) -> ParamType {
        ParamType::None
    }
}
