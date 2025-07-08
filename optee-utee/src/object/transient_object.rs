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

use alloc::{boxed::Box, vec::Vec};
use core::ptr;

use optee_utee_sys as raw;

use super::{Attribute, AttributeId, ObjHandle, ObjectHandle, ObjectInfo, UsageFlag};
use crate::{Error, Result};

/// Define types of [TransientObject](TransientObject) with predefined maximum sizes.
#[repr(u32)]
pub enum TransientObjectType {
    /// 128, 192, or 256 bits
    Aes = 0xA0000010,
    /// Always 64 bits including the parity bits. This gives an effective key size of 56 bits
    Des = 0xA0000011,
    /// 128 or 192 bits including the parity bits. This gives effective key sizes of 112 or 168 bits
    Des3 = 0xA0000013,
    /// Between 64 and 512 bits, multiple of 8 bits
    HmacMd5 = 0xA0000001,
    /// Between 80 and 512 bits, multiple of 8 bits
    HmacSha1 = 0xA0000002,
    /// Between 112 and 512 bits, multiple of 8 bits
    HmacSha224 = 0xA0000003,
    /// Between 192 and 1024 bits, multiple of 8 bits
    HmacSha256 = 0xA0000004,
    /// Between 256 and 1024 bits, multiple of 8 bits
    HmacSha384 = 0xA0000005,
    /// Between 256 and 1024 bits, multiple of 8 bits
    HmacSha512 = 0xA0000006,
    /// The number of bits in the modulus. 256, 512, 768, 1024, 1536 and 2048-bit keys SHALL be supported.
    /// Support for other key sizes including bigger key sizes is
    /// implementation-dependent. Minimum key size is 256 bits
    RsaPublicKey = 0xA0000030,
    /// Same as [RsaPublicKey](TransientObjectType::RsaPublicKey) key size.
    RsaKeypair = 0xA1000030,
    /// Depends on Algorithm:
    /// 1) [DsaSha1](../crypto_op/enum.AlgorithmId.html#variant.DsaSha1):
    /// Between 512 and 1024 bits, multiple of 64 bits
    /// 2) [DsaSha224](../crypto_op/enum.AlgorithmId.html#variant.DsaSha224): 2048 bits
    /// 3) [DsaSha256](../crypto_op/enum.AlgorithmId.html#variant.DsaSha256): 2048 or 3072 bits
    DsaPublicKey = 0xA0000031,
    /// Same as [DsaPublicKey](TransientObjectType::DsaPublicKey) key size.
    DsaKeypair = 0xA1000031,
    /// From 256 to 2048 bits, multiple of 8 bits.
    DhKeypair = 0xA1000032,
    /// Between 160 and 521 bits. Conditional: Available only if at least
    /// one of the ECC the curves defined in Table 6-14 with "generic"
    /// equal to "Y" is supported.
    EcdsaPublicKey = 0xA0000041,
    /// Between 160 and 521 bits. Conditional: Available only if at least
    /// one of the ECC curves defined in Table 6-14 with "generic" equal to
    /// "Y" is supported. SHALL be same value as for ECDSA public key size.
    EcdsaKeypair = 0xA1000041,
    /// Between 160 and 521 bits. Conditional: Available only if at least
    /// one of the ECC curves defined in Table 6-14 with "generic" equal to
    /// "Y" is supported.
    EcdhPublicKey = 0xA0000042,
    /// Between 160 and 521 bits. Conditional: Available only if at least
    /// one of the ECC curves defined in Table 6-14 with "generic" equal to
    /// "Y" is supported. SHALL be same value as for ECDH public key size
    EcdhKeypair = 0xA1000042,
    /// 256 bits. Conditional: Available only if TEE_ECC_CURVE_25519
    /// defined in Table 6-14 is supported.
    Ed25519PublicKey = 0xA0000043,
    /// 256 bits. Conditional: Available only if TEE_ECC_CURVE_25519
    /// defined in Table 6-14 is supported.
    Ed25519Keypair = 0xA1000043,
    /// 256 bits. Conditional: Available only if TEE_ECC_CURVE_25519
    /// defined in Table 6-14 is supported.
    X25519PublicKey = 0xA0000044,
    /// 256 bits. Conditional: Available only if TEE_ECC_CURVE_25519
    /// defined in Table 6-14 is supported.
    X25519Keypair = 0xA1000044,
    /// Multiple of 8 bits, up to 4096 bits. This type is intended for secret
    /// data that has been derived from a key derivation scheme.
    GenericSecret = 0xA0000000,
    /// Object is corrupted.
    CorruptedObject = 0xA00000BE,
    /// 0 – All data is in the associated data stream.
    Data = 0xA00000BF,
}

/// An object containing attributes but no data stream, which is reclaimed
/// when closed or when the TA instance is destroyed.
/// Transient objects are used to hold a cryptographic object (key or key-pair).
///
/// Contrast [PersistentObject](PersistentObject).
pub struct TransientObject(ObjectHandle);

impl TransientObject {
    /// Create a [TransientObject](TransientObject) with a null handle which points to nothing.
    pub fn null_object() -> Self {
        Self(ObjectHandle::from_raw(ptr::null_mut()))
    }

    /// Check if current object is created with null handle.
    ///
    /// # See Also
    ///
    /// - [Self::null_object](Self::null_object).
    pub fn is_null_object(&self) -> bool {
        self.0.is_null()
    }

    /// Allocate an uninitialized [TransientObject](TransientObject), i.e. a container for attributes.
    ///
    /// As allocated, the object is uninitialized.
    /// It can be initialized by subsequently importing the object material, generating an object,
    /// deriving an object, or loading an object from the Trusted Storage.
    ///
    /// # Parameters
    ///
    /// 1) `object_type`: Type of uninitialized object container to be created as defined in
    /// [TransientObjectType](TransientObjectType).
    /// 2) `max_object_size`: Key Size of the object. Valid values depend on the object type and are
    ///    defined in [TransientObjectType](TransientObjectType).
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType};
    /// # fn main() -> optee_utee::Result<()> {
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
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
    /// 1) `OutOfMemory`: If not enough resources are available to allocate the object handle.
    /// 2) `NotSupported`: If the key size is not supported or the object type is not supported.
    ///
    /// # Panics
    ///
    /// 1) If the Implementation detects any error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn allocate(object_type: TransientObjectType, max_object_size: usize) -> Result<Self> {
        let raw_handle: *mut raw::TEE_ObjectHandle = Box::into_raw(Box::new(ptr::null_mut()));
        match unsafe {
            raw::TEE_AllocateTransientObject(object_type as u32, max_object_size as u32, raw_handle)
        } {
            raw::TEE_SUCCESS => {
                let handle = ObjectHandle::from_raw(raw_handle);
                Ok(Self(handle))
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    ///Reset a [TransientObject](TransientObject) to its initial state after allocation.
    ///If the object is currently initialized, the function clears the object of all its material.
    ///The object is then uninitialized again.
    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetTransientObject(self.handle());
        }
    }

    /// Populate an uninitialized object container with object attributes passed by the TA in the `attrs` parameter.
    /// When this function is called, the object SHALL be uninitialized.
    /// If the object is initialized, the caller SHALL first clear it using the function reset.
    /// Note that if the object type is a key-pair, then this function sets both the private and public attributes of the keypair.
    ///
    /// # Parameters
    ///
    /// 1) `attrs`: Array of object attributes.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{
    /// #     TransientObject,
    /// #     TransientObjectType,
    /// #     AttributeMemref,
    /// #     AttributeId,
    /// # };
    /// # fn main() -> optee_utee::Result<()> {
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(mut object) =>
    ///     {
    ///         let attrs = [AttributeMemref::from_ref(AttributeId::SecretValue, &[0u8;1]).into()];
    ///         object.populate(&attrs);
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `BadParameters`: If an incorrect or inconsistent attribute value is detected. In this case,
    /// the content of the object SHALL remain uninitialized.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object that is transient and uninitialized.
    /// 2) If some mandatory attribute is missing.
    /// 3) If an attribute which is not defined for the object’s type is present in attrs.
    /// 4) If an attribute value is too big to fit within the maximum object size specified when the object was created.
    /// 5) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn populate(&mut self, attrs: &[Attribute]) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = attrs.iter().map(|p| p.raw()).collect();
        match unsafe {
            raw::TEE_PopulateTransientObject(self.0.handle(), p.as_ptr() as _, attrs.len() as u32)
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => return Err(Error::from_raw_error(code)),
        }
    }

    /// Return the characteristics of an object.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType};
    /// # fn main() -> optee_utee::Result<()> {
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) => {
    ///         match object.info() {
    ///             Ok(info) => {
    ///                 // ...
    ///                 Ok(())
    ///             }
    ///             Err(e) => Err(e),
    ///         }
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn info(&self) -> Result<ObjectInfo> {
        self.0.info()
    }

    /// Restrict the object usage flags of an object handle to contain at most the flags passed in the obj_usage parameter.
    ///
    /// The initial value of the key usage contains all usage flags.
    ///
    /// # Parameters
    ///
    /// 1) `obj_usage`: New object usage, an OR combination of one or more of the [UsageFlag](UsageFlag).
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType, UsageFlag};
    /// # fn main() -> optee_utee::Result<()> {
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(mut object) =>
    ///     {
    ///         object.restrict_usage(UsageFlag::ENCRYPT)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn restrict_usage(&mut self, obj_usage: UsageFlag) -> Result<()> {
        self.0.restrict_usage(obj_usage)
    }

    /// Extract one buffer attribute from an object. The attribute is identified by the argument id.
    ///
    /// # Parameters
    ///
    /// 1) `id`: Identifier of the attribute to retrieve.
    /// 2) `ref_attr`: Output buffer to get the content of the attribute.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType, AttributeId};
    /// # fn main() -> optee_utee::Result<()> {
    /// # let id = AttributeId::SecretValue;
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) => {
    ///         let mut attr = [0u8; 16];
    ///         match object.ref_attribute(id, &mut attr) {
    ///             Ok(size) => {
    ///                 // ...
    ///                 Ok(())
    ///             }
    ///             Err(e) => Err(e),
    ///         }
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `ItemNotFound`: If the attribute is not found on this object.
    /// 2) `ShortBuffer`: If buffer is NULL or too small to contain the key part.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the object is not initialized.
    /// 3) If the Attribute is not a buffer attribute.
    pub fn ref_attribute(&self, id: AttributeId, buffer: &mut [u8]) -> Result<usize> {
        self.0.ref_attribute(id, buffer)
    }

    /// Extract one value attribute from an object. The attribute is identified by the argument id.
    ///
    /// # Parameters
    ///
    /// 1) `id`: Identifier of the attribute to retrieve.
    /// 2) `value_attr`: Two value placeholders to get the content of the attribute.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType};
    /// # fn main() -> optee_utee::Result<()> {
    /// # let id = 0_u32;
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) => {
    ///         match object.value_attribute(id) {
    ///             Ok((a,b)) => {
    ///                 // ...
    ///                 Ok(())
    ///             }
    ///             Err(e) => Err(e),
    ///         }
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `ItemNotFound`: If the attribute is not found on this object.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the object is not initialized.
    /// 3) If the Attribute is not a value attribute.
    pub fn value_attribute(&self, id: u32) -> Result<(u32, u32)> {
        self.0.value_attribute(id)
    }

    /// Populates an uninitialized object handle with the attributes of another object handle;
    /// that is, it populates the attributes of this handle with the attributes of src_handle.
    /// It is most useful in the following situations:
    /// 1) To extract the public key attributes from a key-pair object.
    /// 2) To copy the attributes from a [PersistentObject](PersistentObject) into a [TransientObject](TransientObject).
    ///
    /// # Parameters
    ///
    /// 1) `src_object`: Can be either a [TransientObject](TransientObject) or [PersistentObject](PersistentObject).
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType};
    /// # fn main() -> optee_utee::Result<()> {
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(mut object1) =>
    ///     {
    ///         match TransientObject::allocate(TransientObjectType::Aes, 256) {
    ///             Ok(object2) => {
    ///                 object1.copy_attribute_from(&object2);
    ///                 Ok(())
    ///             }
    ///             Err(e) => Err(e),
    ///         }
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `CorruptObject`: If the persistent` object is corrupt. The object handle is closed.
    /// 2) `StorageNotAvailable`: If the [PersistentObject](PersistentObject) is stored in a storage area which is
    ///    currently inaccessible.
    ///
    /// # Panics
    ///
    /// 1) If src_object is not initialized.
    /// 2) If self is initialized.
    /// 3) If the type and size of src_object and self are not compatible.
    /// 4) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn copy_attribute_from<T: ObjHandle>(&mut self, src_object: &T) -> Result<()> {
        match unsafe { raw::TEE_CopyObjectAttributes1(self.handle(), src_object.handle()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Generates a random key or a key-pair and populates a transient key object with the generated key material.
    ///
    /// # Parameters
    ///
    /// 1) `key_size`: the size of the desired key. It SHALL be less than or equal to
    /// the maximum key size specified when the [TransientObject](TransientObject) was created.
    ///
    /// # Example
    ///
    /// ``` rust,no_run
    /// # use optee_utee::{TransientObject, TransientObjectType};
    /// # fn main() -> optee_utee::Result<()> {
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) =>
    ///     {
    ///         object.generate_key(128, &[])?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// 1) `BadParameters`: If an incorrect or inconsistent attribute value is detected. In this
    ///    case, the content of the object SHALL remain uninitialized.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If some mandatory attribute is missing.
    /// 3) If an attribute which is not defined for the object’s type is present in attrs.
    /// 4) If an attribute value is too big to fit within the maximum object size specified when
    /// the object was created.
    /// 5) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    pub fn generate_key(&self, key_size: usize, params: &[Attribute]) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        unsafe {
            match raw::TEE_GenerateKey(
                self.handle(),
                key_size as u32,
                p.as_slice().as_ptr() as _,
                p.len() as u32,
            ) {
                raw::TEE_SUCCESS => Ok(()),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }
}

impl ObjHandle for TransientObject {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        self.0.handle()
    }
}

impl Drop for TransientObject {
    /// Deallocates a [TransientObject](TransientObject) previously allocated.
    /// After this function has been called, the object handle is no longer valid and all resources
    /// associated with the [TransientObject](TransientObject) SHALL have been reclaimed.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error associated with this function which is not
    ///    explicitly associated with a defined return code for this function.
    fn drop(&mut self) {
        unsafe {
            if self.0.raw != ptr::null_mut() {
                raw::TEE_FreeTransientObject(self.0.handle());
            }
            drop(Box::from_raw(self.0.raw));
        }
    }
}
