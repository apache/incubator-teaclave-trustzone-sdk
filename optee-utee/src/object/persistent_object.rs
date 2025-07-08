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

use alloc::boxed::Box;
use core::ptr;

use optee_utee_sys as raw;

use super::{
    AttributeId, DataFlag, ObjHandle, ObjectHandle, ObjectInfo, ObjectStorageConstants, UsageFlag,
    Whence,
};
use crate::{Error, Result};

/// An object identified by an Object Identifier and including a Data Stream.
///
/// Contrast [TransientObject](TransientObject).
pub struct PersistentObject(ObjectHandle);

impl PersistentObject {
    /// Open an existing [PersistentObject](PersistentObject).
    ///
    /// # Parameters
    ///
    /// 1) `storage_id`: The storage to use which is defined in
    ///    [ObjectStorageConstants](ObjectStorageConstants).
    /// 2) `object_id`: The object identifier. Note that this buffer cannot reside in shared memory.
    /// 3) `flags`: The [DataFlag](DataFlag) which determine the settings under which the object is opened.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open(
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ) {
    ///     Ok(object) =>
    ///     {
    ///         // ...
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `ItemNotFound`: If the storage denoted by storage_id does not exist or if the object
    ///    identifier cannot be found in the storage.
    /// 2) `Access_Conflict`: If an access right conflict was detected while opening the object.
    /// 3) `OutOfMemory`: If there is not enough memory to complete the operation.
    /// 4) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 5) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object_id.len() >
    ///    [MiscellaneousConstants::TeeObjectIdMaxLen](MiscellaneousConstants::TeeObjectIdMaxLen)
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn open(
        storage_id: ObjectStorageConstants,
        object_id: &[u8],
        flags: DataFlag,
    ) -> Result<Self> {
        let raw_handle: *mut raw::TEE_ObjectHandle = Box::into_raw(Box::new(ptr::null_mut()));
        match unsafe {
            raw::TEE_OpenPersistentObject(
                storage_id as u32,
                object_id.as_ptr() as _,
                object_id.len(),
                flags.bits(),
                raw_handle as *mut _,
            )
        } {
            raw::TEE_SUCCESS => {
                let handle = ObjectHandle::from_raw(raw_handle);
                Ok(Self(handle))
            }
            code => {
                unsafe {
                    drop(Box::from_raw(raw_handle));
                }
                Err(Error::from_raw_error(code))
            }
        }
    }

    /// Create a [PersistentObject](PersistentObject) with initial attributes and an initial data stream content.
    ///
    /// # Parameters
    ///
    /// 1) `storage_id`: The storage to use which is defined in
    ///    [ObjectStorageConstants](ObjectStorageConstants).
    /// 2) `object_id`: The object identifier. Note that this buffer cannot reside in shared memory.
    /// 3) `flags`: The [DataFlag](DataFlag) which determine the settings under which the object is opened.
    /// 4) `attributes`: A handle on a [PersistentObject](PersistentObject) or an initialized [TransientObject](TransientObject)
    /// from which to take the [PersistentObject](PersistentObject) attributes.
    /// Can be NONE if the [PersistentObject](PersistentObject) contains no attribute.
    /// For example,if  it is a pure data object.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// let mut init_data: [u8; 0] = [0; 0];
    /// match PersistentObject::create(
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ | DataFlag::ACCESS_WRITE,
    ///         None,
    ///         &mut init_data) {
    ///     Ok(object) =>
    ///     {
    ///         // ...
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `ItemNotFound`: If the storage denoted by storage_id does not exist or if the object
    ///    identifier cannot be found in the storage.
    /// 2) `Access_Conflict`: If an access right conflict was detected while opening the object.
    /// 3) `OutOfMemory`: If there is not enough memory to complete the operation.
    /// 4) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 5) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object_id.len() >
    ///    [MiscellaneousConstants::TeeObjectIdMaxLen](MiscellaneousConstants::TeeObjectIdMaxLen).
    /// 2) If attributes is not NONE and is not a valid handle on an initialized object containing
    ///    the type and attributes of the [PersistentObject](PersistentObject) to create.
    /// 3) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn create(
        storage_id: ObjectStorageConstants,
        object_id: &[u8],
        flags: DataFlag,
        attributes: Option<ObjectHandle>,
        initial_data: &[u8],
    ) -> Result<Self> {
        let raw_handle: *mut raw::TEE_ObjectHandle = Box::into_raw(Box::new(ptr::null_mut()));
        let attributes = match attributes {
            Some(a) => a.handle(),
            None => ptr::null_mut(),
        };
        match unsafe {
            raw::TEE_CreatePersistentObject(
                storage_id as u32,
                object_id.as_ptr() as _,
                object_id.len(),
                flags.bits(),
                attributes,
                initial_data.as_ptr() as _,
                initial_data.len(),
                raw_handle as *mut _,
            )
        } {
            raw::TEE_SUCCESS => {
                let handle = ObjectHandle::from_raw(raw_handle);
                Ok(Self(handle))
            }
            code => {
                unsafe {
                    drop(Box::from_raw(raw_handle));
                }
                Err(Error::from_raw_error(code))
            }
        }
    }

    /// Marks an object for deletion and closes the object.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ) {
    ///     Ok(mut object) =>
    ///     {
    ///         object.close_and_delete()?;
    ///         std::mem::forget(object);
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    // this function is conflicted with Drop implementation, when use this one to avoid panic:
    // Call `mem::forget` for this structure to avoid double drop the object
    pub fn close_and_delete(&mut self) -> Result<()> {
        match unsafe { raw::TEE_CloseAndDeletePersistentObject1(self.0.handle()) } {
            raw::TEE_SUCCESS => {
                unsafe {
                    drop(Box::from_raw(self.0.raw));
                }
                return Ok(());
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Changes the identifier of an object.
    /// The object SHALL have been opened with the [DataFlag::ACCESS_WRITE_META](DataFlag::ACCESS_WRITE_META) right, which means access to the object is exclusive.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// let new_obj_id = [2u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_WRITE_META) {
    ///     Ok(mut object) =>
    ///     {
    ///         object.rename(&new_obj_id)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `Access_Conflict`: If an access right conflict was detected while opening the object.
    /// 2) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 3) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If new_object_id resides in shared memory.
    /// 3) If new_object_id.len() >
    ///    [MiscellaneousConstants::TeeObjectIdMaxLen](MiscellaneousConstants::TeeObjectIdMaxLen).
    /// 4) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn rename(&mut self, new_object_id: &[u8]) -> Result<()> {
        match unsafe {
            raw::TEE_RenamePersistentObject(
                self.0.handle(),
                new_object_id.as_ptr() as _,
                new_object_id.len(),
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }
    /// Return the characteristics of an object.
    /// Function is similar to [TransientObject::info](TransientObject::info) besides extra errors.
    ///
    /// # Errors
    ///
    /// 1) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 2) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    pub fn info(&self) -> Result<ObjectInfo> {
        self.0.info()
    }

    /// Restrict the object usage flags of an object handle to contain at most the flags passed in the obj_usage parameter.
    /// Function is similar to [TransientObject::restrict_usage](TransientObject::restrict_usage) besides extra errors.
    ///
    /// # Errors
    ///
    /// 1) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 2) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    pub fn restrict_usage(&mut self, obj_usage: UsageFlag) -> Result<()> {
        self.0.restrict_usage(obj_usage)
    }

    /// Extract one buffer attribute from an object. The attribute is identified by the argument id.
    /// Function is similar to [TransientObject::ref_attribute](TransientObject::ref_attribute) besides extra errors.
    ///
    /// # Errors
    ///
    /// 1) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 2) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    pub fn ref_attribute(&self, id: AttributeId, buffer: &mut [u8]) -> Result<usize> {
        self.0.ref_attribute(id, buffer)
    }

    /// Extract one value attribute from an object. The attribute is identified by the argument id.
    /// Function is similar to [TransientObject::value_attribute](TransientObject::value_attribute) besides extra errors.
    ///
    /// # Errors
    ///
    /// 1) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 2) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    pub fn value_attribute(&self, id: u32) -> Result<(u32, u32)> {
        self.0.value_attribute(id)
    }

    /// Read requested size from the data stream associate with the object into the buffer.
    ///
    /// # Parameters
    ///
    /// 1) `buffer`: A pre-allocated buffer for saving the object's data stream.
    /// 2) `count`: The returned value contains the number of bytes read.
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ) {
    ///     Ok(object) =>
    ///     {
    ///         let mut read_buf = [0u8;16];
    ///         object.read(&mut read_buf)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 2) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn read(&self, buf: &mut [u8]) -> Result<u32> {
        let mut count: usize = 0;
        match unsafe {
            raw::TEE_ReadObjectData(self.handle(), buf.as_mut_ptr() as _, buf.len(), &mut count)
        } {
            raw::TEE_SUCCESS => Ok(count as u32),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Write the passed in buffer data into from the data stream associate with the object.
    ///
    /// # Parameters
    ///
    /// 1) `buffer`: A pre-allocated buffer for saving the object's data stream.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_WRITE) {
    ///     Ok(mut object) =>
    ///     {
    ///         let write_buf = [1u8;16];
    ///         object.write(& write_buf)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `StorageNoSpace`: If insufficient storage space is available.
    /// 2) `Overflow`: If the value of the data position indicator resulting from this operation
    ///    would be greater than
    ///    [MiscellaneousConstants::TeeDataMaxPosition](MiscellaneousConstants::TeeDataMaxPosition).
    /// 3) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 4) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        match unsafe { raw::TEE_WriteObjectData(self.handle(), buf.as_ptr() as _, buf.len()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Change the size of a data stream associate with the object.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_WRITE) {
    ///     Ok(object) =>
    ///     {
    ///         object.truncate(1u32)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `StorageNoSpace`: If insufficient storage space is available.
    ///    would be greater than
    ///    [MiscellaneousConstants::TeeDataMaxPosition](MiscellaneousConstants::TeeDataMaxPosition).
    /// 2) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 3) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn truncate(&self, size: u32) -> Result<()> {
        match unsafe { raw::TEE_TruncateObjectData(self.handle(), size as usize) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Set the data position indicator associate with the object.
    ///
    /// # Parameters
    /// 1) `whence`: Defined in [Whence](Whence).
    /// 2) `offset`: The bytes shifted based on `whence`.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{PersistentObject, ObjectStorageConstants, DataFlag, Whence};
    /// # fn main() -> optee_utee::Result<()> {
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open(
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_WRITE) {
    ///     Ok(object) =>
    ///     {
    ///         object.seek(0i32, Whence::DataSeekSet)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `Overflow`: If data position indicator is greater than
    ///    [MiscellaneousConstants::TeeDataMaxPosition](MiscellaneousConstants::TeeDataMaxPosition).
    /// 2) `CorruptObject`: If the [PersistentObject](PersistentObject) is corrupt. The object handle is closed.
    /// 3) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn seek(&self, offset: i32, whence: Whence) -> Result<()> {
        match unsafe { raw::TEE_SeekObjectData(self.handle(), offset.into(), whence.into()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }
}

impl ObjHandle for PersistentObject {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        self.0.handle()
    }
}

impl Drop for PersistentObject {
    /// Close an opened [PersistentObject](PersistentObject).
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    fn drop(&mut self) {
        unsafe {
            if self.0.raw != Box::into_raw(Box::new(ptr::null_mut())) {
                raw::TEE_CloseObject(self.0.handle());
            }
            drop(Box::from_raw(self.0.raw));
        }
    }
}
