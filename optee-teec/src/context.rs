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

use crate::{Error, Operation, Result, Session, Uuid};
use crate::{Param, ParamNone};
use optee_teec_sys as raw;
use std::{cell::RefCell, ptr, sync::Arc};

pub struct InnerContext(pub raw::TEEC_Context);

impl Drop for InnerContext {
    fn drop(&mut self) {
        unsafe {
            raw::TEEC_FinalizeContext(&mut self.0);
        }
    }
}

/// An abstraction of the logical connection between a client application and a
/// TEE.
pub struct Context {
    // Use Arc to share it with Session, eliminating the lifetime constraint.
    // Use RefCell to allow conversion into a raw mutable pointer.
    raw: Arc<RefCell<InnerContext>>,
}

// Since RefCell is used for Context, Rust does not automatically implement
// Send and Sync for it. We need to manually implement them and ensure that
// InnerContext is used correctly.
unsafe impl Send for Context{}
unsafe impl Sync for Context{}

impl Context {
    /// Creates a TEE client context object.
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = Context::new().unwrap();
    /// ```
    pub fn new() -> Result<Context> {
        // define an empty TEEC_Context
        let mut raw_ctx = raw::TEEC_Context {
            fd: 0,
            reg_mem: false,
            memref_null: false,
        };
        match unsafe { raw::TEEC_InitializeContext(ptr::null_mut(), &mut raw_ctx) } {
            raw::TEEC_SUCCESS => Ok(Self {
                raw: Arc::new(RefCell::new(InnerContext(raw_ctx))),
            }),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Opens a new session with the specified trusted application.
    ///
    /// The target trusted application is specified by `uuid`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = Context::new().unwrap();
    /// let uuid = Uuid::parse_str("8abcf200-2450-11e4-abe2-0002a5d5c51b").unwrap();
    /// let session = ctx.open_session(uuid).unwrap();
    /// ```
    pub fn open_session(&mut self, uuid: Uuid) -> Result<Session> {
        Session::new(
            self,
            uuid,
            None::<&mut Operation<ParamNone, ParamNone, ParamNone, ParamNone>>,
        )
    }

    /// Opens a new session with the specified trusted application, pass some
    /// parameters to TA by an operation.
    ///
    /// The target trusted application is specified by `uuid`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = Context::new().unwrap();
    /// let uuid = Uuid::parse_str("8abcf200-2450-11e4-abe2-0002a5d5c51b").unwrap();
    /// let p0 = ParamValue(42, 0, ParamType::ValueInout);
    /// let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    /// let session = ctx.open_session_with_operation(uuid, operation).unwrap();
    /// ```
    pub fn open_session_with_operation<A: Param, B: Param, C: Param, D: Param>(
        &mut self,
        uuid: Uuid,
        operation: &mut Operation<A, B, C, D>,
    ) -> Result<Session> {
        Session::new(self, uuid, Some(operation))
    }
}

// Intenal usage only
impl Context {
    // anyone who wants to access the inner_context must take this as mut.
    pub(crate) fn inner_context(&mut self) -> Arc<RefCell<InnerContext>> {
        self.raw.clone()
    }
}
