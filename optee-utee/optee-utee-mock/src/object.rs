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

use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, RwLock};

use crate::raw::{self, TEE_ObjectHandle, TEE_ObjectType, TEE_Result};

static GLOBAL_OBJECT_MOCK: RwLock<Option<Box<dyn ObjectController + 'static>>> = RwLock::new(None);
pub static SERIAL_TEST_LOCK: Mutex<()> = Mutex::new(());

#[mockall::automock]
// currently we just add functions that we need
pub trait ObjectController: Send + Sync {
    // Data and Key Storage API  - Generic Object Functions
    fn TEE_CloseObject(&self, object: TEE_ObjectHandle);

    /* Data and Key Storage API  - Transient Object Functions */
    fn TEE_AllocateTransientObject(
        &self,
        objectType: TEE_ObjectType,
        maxObjectSize: u32,
        object: *mut TEE_ObjectHandle,
    ) -> TEE_Result;

    // Data and Key Storage API  - Persistent Object Functions
    fn TEE_OpenPersistentObject(
        &self,
        storageID: u32,
        objectID: *const c_void,
        objectIDLen: usize,
        flags: u32,
        object: *mut TEE_ObjectHandle,
    ) -> TEE_Result;
    fn TEE_CreatePersistentObject(
        &self,
        storageID: u32,
        objectID: *const c_void,
        objectIDLen: usize,
        flags: u32,
        attributes: TEE_ObjectHandle,
        initialData: *const c_void,
        initialDataLen: usize,
        object: *mut TEE_ObjectHandle,
    ) -> TEE_Result;
    fn TEE_CloseAndDeletePersistentObject1(&self, object: TEE_ObjectHandle) -> TEE_Result;
}

pub fn set_global_object_mock(mock: impl ObjectController + 'static) {
    let mut value = GLOBAL_OBJECT_MOCK.write().unwrap();
    value.replace(Box::new(mock));
}

fn with_global_object_mock<R, F: FnOnce(&dyn ObjectController) -> R>(f: F) -> R {
    let mock = GLOBAL_OBJECT_MOCK.read().unwrap();
    let borrow = mock.as_ref().expect("Global Object Mock Not Set");
    f(borrow.as_ref())
}

macro_rules! forward_to_mock {
    ($fn_name:ident($($param:ident: $ty:ty),*) -> $ret:ty) => {
        #[no_mangle]
        fn $fn_name($($param: $ty),*) -> $ret {
            with_global_object_mock(|mock: &dyn ObjectController| {
                mock.$fn_name($($param),*)
            })
        }
    };
}

forward_to_mock!(TEE_CloseObject(object: TEE_ObjectHandle) -> ());

forward_to_mock!(TEE_AllocateTransientObject(
    objectType: TEE_ObjectType,
    maxObjectSize: u32,
    object: *mut TEE_ObjectHandle
) -> TEE_Result);

forward_to_mock!(TEE_OpenPersistentObject(
    storageID: u32,
    objectID: *const c_void,
    objectIDLen: usize,
    flags: u32,
    object: *mut TEE_ObjectHandle
) -> TEE_Result);
forward_to_mock!(TEE_CreatePersistentObject(
    storageID: u32,
    objectID: *const c_void,
    objectIDLen: usize,
    flags: u32,
    attributes: TEE_ObjectHandle,
    initialData: *const c_void,
    initialDataLen: usize,
    object: *mut TEE_ObjectHandle
) -> TEE_Result);
forward_to_mock!(TEE_CloseAndDeletePersistentObject1(
    object: TEE_ObjectHandle
) -> TEE_Result);

type ValidTestHandle = Arc<UnsafeCell<raw::TEE_ObjectHandle>>;

impl MockObjectController {
    pub fn new_valid_test_handle_struct() -> raw::__TEE_ObjectHandle {
        unsafe { core::mem::zeroed() }
    }
    pub fn new_valid_test_handle(handle: &mut raw::__TEE_ObjectHandle) -> ValidTestHandle {
        Arc::new(UnsafeCell::new(handle))
    }

    pub fn expect_TEE_AllocateTransientObject_success_once(&mut self, handle: ValidTestHandle) {
        self.expect_TEE_AllocateTransientObject()
            .return_once_st(move |_, _, obj| {
                unsafe {
                    *obj = *handle.get();
                }
                raw::TEE_SUCCESS
            });
    }
    pub fn expect_TEE_AllocateTransientObject_fail_once(&mut self, code: raw::TEE_Result) {
        self.expect_TEE_AllocateTransientObject()
            .return_once_st(move |_, _, _| code);
    }

    pub fn expect_TEE_CreatePersistentObject_success_once(&mut self, handle: ValidTestHandle) {
        self.expect_TEE_CreatePersistentObject()
            .return_once_st(move |_, _, _, _, _, _, _, obj| {
                unsafe {
                    *obj = *handle.get();
                }
                raw::TEE_SUCCESS
            });
    }
    pub fn expect_TEE_CreatePersistentObject_fail_once(&mut self, code: raw::TEE_Result) {
        self.expect_TEE_CreatePersistentObject()
            .return_once_st(move |_, _, _, _, _, _, _, _| code);
    }

    pub fn expect_TEE_OpenPersistentObject_success_once(&mut self, handle: ValidTestHandle) {
        self.expect_TEE_OpenPersistentObject()
            .return_once_st(move |_, _, _, _, obj| {
                unsafe {
                    *obj = *handle.get();
                }
                raw::TEE_SUCCESS
            });
    }
    pub fn expect_TEE_OpenPersistentObject_fail_once(&mut self, code: raw::TEE_Result) {
        self.expect_TEE_OpenPersistentObject()
            .return_once_st(move |_, _, _, _, _| code);
    }

    pub fn expect_TEE_CloseAndDeletePersistentObject1_success_once(
        &mut self,
        exp_handle: ValidTestHandle,
    ) {
        self.expect_TEE_CloseAndDeletePersistentObject1()
            .return_once_st(move |obj| {
                assert_eq!(obj, unsafe { *exp_handle.get() });
                raw::TEE_SUCCESS
            });
    }
    pub fn expect_TEE_CloseAndDeletePersistentObject1_fail_once(&mut self, code: raw::TEE_Result) {
        self.expect_TEE_CloseAndDeletePersistentObject1()
            .return_once_st(move |_| code);
    }

    pub fn expect_TEE_CloseObject_once(&mut self, exp_handle: ValidTestHandle) {
        self.expect_TEE_CloseObject().return_once_st(move |obj| {
            assert_eq!(obj, unsafe { *exp_handle.get() });
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::raw;

    #[test]
    fn test_new_handle() {
        let mut handle_struct1 = MockObjectController::new_valid_test_handle_struct();
        let mut handle_struct2 = MockObjectController::new_valid_test_handle_struct();

        let handle1 = MockObjectController::new_valid_test_handle(&mut handle_struct1);
        let handle2 = MockObjectController::new_valid_test_handle(&mut handle_struct2);

        assert_ne!(unsafe { *handle1.get() }, unsafe { *handle2.get() });

        let handle3: ValidTestHandle = Arc::new(UnsafeCell::new(core::ptr::null_mut()));
        let handle4: ValidTestHandle = Arc::new(UnsafeCell::new(core::ptr::null_mut()));
        assert_eq!(unsafe { *handle3.get() }, unsafe { *handle4.get() });
    }

    #[test]
    fn test_mock_usage() {
        let mut handle_struct = MockObjectController::new_valid_test_handle_struct();
        let handle = MockObjectController::new_valid_test_handle(&mut handle_struct);
        let mut mock = MockObjectController::new();

        mock.expect_TEE_OpenPersistentObject_success_once(handle.clone());
        mock.expect_TEE_CloseAndDeletePersistentObject1_success_once(handle.clone());
        mock.expect_TEE_CloseObject_once(handle.clone());

        set_global_object_mock(mock);

        let mut handle = core::ptr::null_mut();
        let result =
            unsafe { raw::TEE_OpenPersistentObject(0, core::ptr::null(), 0, 0, &mut handle) };
        assert_eq!(result, raw::TEE_SUCCESS);

        let result = unsafe { raw::TEE_CloseAndDeletePersistentObject1(handle) };
        assert_eq!(result, raw::TEE_SUCCESS);

        unsafe { raw::TEE_CloseObject(handle) };
    }
}
