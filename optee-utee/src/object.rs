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

use crate::{Error, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use core::{marker, mem, ptr};
#[cfg(not(target_os = "optee"))]
use alloc::boxed::Box;
#[cfg(not(target_os = "optee"))]
use alloc::vec::Vec;

/// A general attribute (buffer or value) that can be used to populate an object or to specify
/// operation parameters.
pub struct Attribute {
    raw: raw::TEE_Attribute,
}

impl Attribute {
    /// Return the raw struct `TEE_Attribute`.
    pub fn raw(&self) -> raw::TEE_Attribute {
        self.raw
    }
}

/// Convert the buffer attribute [AttributeMemref](AttributeMemref) to the general attribute.
impl<'attrref> From<AttributeMemref<'attrref>> for Attribute {
    fn from(attr: AttributeMemref) -> Self {
        Self { raw: attr.raw() }
    }
}

/// Convert the value attribute [AttributeValue](AttributeValue) to the general attribute.
impl From<AttributeValue> for Attribute {
    fn from(attr: AttributeValue) -> Self {
        Self { raw: attr.raw() }
    }
}

/// A buffer attribute.
#[derive(Clone, Copy)]
pub struct AttributeMemref<'attrref> {
    raw: raw::TEE_Attribute,
    _marker: marker::PhantomData<&'attrref mut [u8]>,
}

impl<'attrref> AttributeMemref<'attrref> {
    /// Return the raw struct TEE_Attribute.
    pub fn raw(&self) -> raw::TEE_Attribute {
        self.raw
    }

    fn new_ref() -> Self {
        let raw = raw::TEE_Attribute {
            attributeID: 0,
            content: raw::content {
                memref: raw::Memref {
                    buffer: 0 as *mut _,
                    size: 0,
                },
            },
        };
        Self {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }

    /// Populate a single attribute with a reference to a buffer.
    ///
    /// # Parameters
    ///
    /// 1) `id`: The AttributeId[AttributeId] is an identifier of the attribute to populate.
    /// 2) `buffer`: Input buffer that holds the content of the attribute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut attr = AttributeMemref::from_ref(AttributeId::SecretValue, &mut [0u8;1]);
    /// ```
    pub fn from_ref(id: AttributeId, buffer: &'attrref [u8]) -> Self {
        let mut res = AttributeMemref::new_ref();
        unsafe {
            raw::TEE_InitRefAttribute(
                &mut res.raw,
                id as u32,
                buffer.as_ptr() as *mut _,
                buffer.len(),
            );
        }
        res
    }
}

/// A value attribute.
pub struct AttributeValue {
    raw: raw::TEE_Attribute,
}

impl AttributeValue {
    /// Return the raw struct TEE_Attribute.
    pub fn raw(&self) -> raw::TEE_Attribute {
        self.raw
    }

    fn new_value() -> Self {
        let raw = raw::TEE_Attribute {
            attributeID: 0,
            content: raw::content {
                value: raw::Value { a: 0, b: 0 },
            },
        };
        Self { raw }
    }

    /// Populate a single attribute with two u32 values.
    ///
    /// # Parameters
    ///
    /// 1) `id`: The AttributeId[AttributeId] is an identifier of the attribute to populate.
    /// 2) `a`, `b`: u32 values to assign to the members of the value attribute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut attr = AttributeValue::from_value(AttributeId::SecretValue, 0, 0);
    /// ```
    pub fn from_value(id: AttributeId, a: u32, b: u32) -> Self {
        let mut res = AttributeValue::new_value();
        unsafe {
            raw::TEE_InitValueAttribute(&mut res.raw, id as u32, a, b);
        }
        res
    }
}

/// Represent the characteristics of an object.
/// This info can be returned by [TransientObject](TransientObject) function
/// [info](TransientObject::info)
/// or [PersistentObject](PersistentObject) function
/// [info](PersistentObject::info).
pub struct ObjectInfo {
    raw: raw::TEE_ObjectInfo,
}

// Since raw struct is not implemented Copy attribute yet, every item in raw struct needs a function to extract.
impl ObjectInfo {
    /// Return an [ObjectInfo](ObjectInfo) struct based on the raw structure `TEE_ObjectInfo`.
    /// The raw structure contains following fields:
    ///
    /// 1) `objectType`: The parameter represents one of the
    ///    [TransientObjectType](TransientObjectType).
    /// 2) `objectSize`: The current size in bits of the object as determined by its attributes.
    /// This will always be less than or equal to maxObjectSize. Set to 0 for uninitialized and data only objects.
    /// 3) `maxObjectSize`: The maximum objectSize which this object can represent.
    /// 3.1) For a [PersistentObject](PersistentObject), set to `objectSize`.
    /// 3.2) For a [TransientObject](TransientObject), set to the parameter `maxObjectSize` passed to
    /// [allocate](TransientObject::allocate).
    /// 4) `objectUsage`: A bit vector of UsageFlag.
    /// 5) `dataSize`:
    /// 5.1) For a [PersistentObject](PersistentObject), set to the current size of the data associated with the object.
    /// 5.2) For a [TransientObject](TransientObject), always set to 0.
    /// 6) `dataPosition`:
    /// 6.1) For a [PersistentObject](PersistentObject), set to the current position in the data for this handle.
    /// Data positions for different handles on the same object may differ.
    /// 6.2) For a [TransientObject](TransientObject), set to 0.
    /// 7) `handleFlags`: A bit vector containing one or more [HandleFlag](HandleFlag) or [DataFlag](DataFlag).
    pub fn from_raw(raw: raw::TEE_ObjectInfo) -> Self {
        Self { raw }
    }

    /// Return the `dataSize` field of the raw structure `TEE_ObjectInfo`.
    pub fn data_size(&self) -> usize {
        self.raw.dataSize as usize
    }

    /// Return the `objectSize` field of the raw structure `TEE_ObjectInfo`.
    pub fn object_size(&self) -> usize {
        self.raw.objectSize as usize
    }
}

/// Indicate the possible start offset when moving a data position in the data stream associated with a [PersistentObject](PersistentObject).
pub enum Whence {
    /// The data position is set to offset bytes from the beginning of the data stream.
    DataSeekSet,
    /// The data position is set to its current position plus offset.
    DataSeekCur,
    /// The data position is set to the size of the object data plus offset.
    DataSeekEnd,
}

impl Into<raw::TEE_Whence> for Whence {
    fn into(self) -> raw::TEE_Whence {
        match self {
            Whence::DataSeekSet => raw::TEE_Whence::TEE_DATA_SEEK_SET,
            Whence::DataSeekCur => raw::TEE_Whence::TEE_DATA_SEEK_CUR,
            Whence::DataSeekEnd => raw::TEE_Whence::TEE_DATA_SEEK_END,
        }
    }
}

/// An opaque handle on an object.
pub struct ObjectHandle {
    raw: *mut raw::TEE_ObjectHandle,
}

impl ObjectHandle {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        unsafe { *(self.raw) }
    }

    fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    fn from_raw(raw: *mut raw::TEE_ObjectHandle) -> ObjectHandle {
        Self { raw }
    }

    fn info(&self) -> Result<ObjectInfo> {
        let mut raw_info: raw::TEE_ObjectInfo = unsafe { mem::zeroed() };
        match unsafe { raw::TEE_GetObjectInfo1(self.handle(), &mut raw_info) } {
            raw::TEE_SUCCESS => Ok(ObjectInfo::from_raw(raw_info)),
            code => Err(Error::from_raw_error(code)),
        }
    }

    fn restrict_usage(&mut self, obj_usage: UsageFlag) -> Result<()> {
        match unsafe { raw::TEE_RestrictObjectUsage1(self.handle(), obj_usage.bits()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    fn ref_attribute(&self, id: AttributeId, buffer: &mut [u8]) -> Result<usize> {
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

    fn value_attribute(&self, id: u32) -> Result<(u32, u32)> {
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

#[repr(u32)]
pub enum ObjectStorageConstants {
    Private = 0x00000001,
    IllegalValue = 0x7FFFFFFF,
}

bitflags! {
    /// A set of flags that controls the access rights and sharing permissions
    /// with which the object handle is opened.
    pub struct DataFlag: u32 {
        /// The object is opened with the read access right. This allows the
        /// Trusted Application to call the function `TEE_ReadObjectData`.
        const ACCESS_READ = 0x00000001;
        /// The object is opened with the write access right. This allows the
        /// Trusted Application to call the functions `TEE_WriteObjectData` and
        /// `TEE_TruncateObjectData`.
        const ACCESS_WRITE = 0x00000002;
        /// The object is opened with the write-meta access right. This allows
        /// the Trusted Application to call the functions
        /// `TEE_CloseAndDeletePersistentObject1` and `TEE_RenamePersistentObject`.
        const ACCESS_WRITE_META = 0x00000004;
        /// The caller allows another handle on the object to be created with
        /// read access.
        const SHARE_READ = 0x00000010;
        /// The caller allows another handle on the object to be created with
        /// write access.
        const SHARE_WRITE = 0x00000020;
        /// * If this flag is present and the object exists, then the object is
        ///   deleted and re-created as an atomic operation: that is, the TA sees
        ///   either the old object or the new one.
        /// * If the flag is absent and the object exists, then the function
        ///   SHALL return `TEE_ERROR_ACCESS_CONFLICT`.
        const OVERWRITE = 0x00000400;
    }
}

bitflags! {
    /// A set of flags that defines usages of data in TEE secure storage.
    pub struct UsageFlag: u32 {
        /// The object [Attribute](Attribute) can be extracted.
        const EXTRACTABLE = 0x00000001;
        /// The object can be used for encryption.
        const ENCRYPT = 0x00000002;
        /// The object can be used for decryption.
        const DECRYPT = 0x00000004;
        /// The object can be used for mac operation.
        const MAC = 0x00000008;
        /// The object can be used for signature.
        const SIGN = 0x00000010;
        /// The object can be used for verification of a signature.
        const VERIFY = 0x00000020;
        /// The object can be used for deriving a key.
        const DERIVE = 0x00000040;
    }
}

/// Miscellaneous constants.
#[repr(u32)]
pub enum MiscellaneousConstants {
    /// Maximum offset of a data object.
    TeeDataMaxPosition = 0xFFFFFFFF,
    /// Maximum length of an object id.
    TeeObjectIdMaxLen = 64,
}

bitflags! {
    /// A set of flags that defines Handle features.
    pub struct HandleFlag: u32{
        /// Set for a [PersistentObject](PersistentObject).
        const PERSISTENT = 0x00010000;
        /// 1) For a [PersistentObject](PersistentObject), always set.
        /// 2) For a [TransientObject](TransientObject), initially cleared, then set when the object becomes initialized.
        const INITIALIZED = 0x00020000;
        /// Following two flags are for crypto operation handles:
        /// 1) Set if the required operation key has been set.
        /// 2) Always set for digest operations.
        const KEY_SET = 0x00040000;
        /// Set if the algorithm expects two keys to be set, using `TEE_SetOperationKey2`.
        /// This happens only if algorithm is set to [AesXts](../crypto_op/enum.AlgorithmId.html#variant.AesXts)
        /// or `TEE_ALG_SM2_KEP`(not supported now).
        const EXPECT_TWO_KEYS = 0x00080000;
    }
}

#[repr(u32)]
pub enum AttributeId {
    /// Used for all secret keys for symmetric ciphers, MACs, and HMACs
    SecretValue = 0xC0000000,
    /// RSA modulus: `n`
    RsaModulus = 0xD0000130,
    /// RSA public key exponent: `e`
    RsaPublicExponent = 0xD0000230,
    /// RSA private key exponent: `d`
    RsaPrivateExponent = 0xC0000330,
    /// RSA prime number: `p`
    RsaPrime1 = 0xC0000430,
    /// RSA prime number: `q`
    RsaPrime2 = 0xC0000530,
    /// RSA exponent: `dp`
    RsaExponent1 = 0xC0000630,
    /// RSA exponent: `dq`
    RsaExponent2 = 0xC0000730,
    /// RSA coefficient: `iq`
    RsaCoefficient = 0xC0000830,
    /// DSA prime number: `p`
    DsaPrime = 0xD0001031,
    /// DSA sub prime number: `q`
    DsaSubprime = 0xD0001131,
    /// DSA base: `g`
    DsaBase = 0xD0001231,
    /// DSA public value: `y`
    DsaPublicValue = 0xD0000131,
    /// DSA private value: `x`
    DsaPrivateValue = 0xC0000231,
    /// Diffie-Hellman prime number: `p`
    DhPrime = 0xD0001032,
    /// Diffie-Hellman subprime number: `q`
    DhSubprime = 0xD0001132,
    /// Diffie-Hellman base: `g`
    DhBase = 0xD0001232,
    /// Diffie-Hellman x bits: `l`
    DhXBits = 0xF0001332,
    /// Diffie-Hellman public value: `y`
    DhPublicValue = 0xD0000132,
    /// Diffie-Hellman public value: `x`
    DhPrivateValue = 0xC0000232,
    RsaOaepLabel = 0xD0000930,
    RsaOaepMgf1Hash = 0xD0000931,
    RsaPssSaltLength = 0xF0000A30,
    /// ECC public value: `x`
    EccPublicValueX = 0xD0000141,
    /// ECC public value: `y`
    EccPublicValueY = 0xD0000241,
    /// ECC private value: `d`
    EccPrivateValue = 0xC0000341,
    /// Ed25519 public value
    Ed25519PublicValue = 0xD0000743,
    /// Ed25519 private value
    Ed25519PrivateValue = 0xC0000843,
    /// X25519 public value
    X25519PublicValue = 0xD0000944,
    /// X25519 private value
    X25519PrivateValue = 0xC0000A44,
    /// ECC Curve algorithm
    EccCurve = 0xF0000441,
    BitProtected = (1 << 28),
    BitValue = (1 << 29),
}

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

/// A trait for an object (transient or persistent) to return its handle.
pub trait ObjHandle {
    /// Return the handle of an object.
    fn handle(&self) -> raw::TEE_ObjectHandle;
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) =>
    ///     {
    ///         // ...
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) =>
    ///     {
    ///         let attrs = [AttributeMemref::from_ref(AttributeId::SecretValue, &[0u8;1])];
    ///         object.populate(&attrs);
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) => {
    ///         match object.info() {
    ///             Ok(info) => {
    ///                 // ...
    ///                 Ok(())
    ///             }
    ///         Err(e) => Err(e),
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) =>
    ///     {
    ///         object.restrict_usage(UsageFlag::ENCRYPT)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) => {
    ///         match object.value_attribute(id) {
    ///             Ok(a,b) => {
    ///                 // ...
    ///                 Ok(())
    ///             }
    ///             Err(e) => Err(e),
    ///         }
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object1) =>
    ///     {
    ///         match TransientObject::allocate(TransientObjectType::Aes, 256) {
    ///             Ok(object2) => {
    ///                 object1.copy_attribute_from(object2);
    ///                 Ok(())
    ///             }
    ///             Err(e) => Err(e),
    ///         }
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///     Ok(object) =>
    ///     {
    ///         object.generate_key(128, &[])?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
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
    /// ```no_run
    /// let obj_id = [1u8;1];
    /// let mut init_data: [u8; 0] = [0; 0];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ | DataFlag::ACCESS_WRITE
    ///         None,
    ///         &mut init_data) {
    ///     Ok(object) =>
    ///     {
    ///         // ...
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ) {
    ///     Ok(object) =>
    ///     {
    ///         object.close_and_delete()?;
    ///         std::mem::forget(object);
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// let obj_id = [1u8;1];
    /// let new_obj_id = [2u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_WRITE_META) {
    ///     Ok(object) =>
    ///     {
    ///         object.rename(&new_obj_id)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
    /// ```no_run
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_READ) {
    ///     Ok(object) =>
    ///     {
    ///         let read_buf = [0u8;16];
    ///         object.read(&mut read_buf)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
            raw::TEE_ReadObjectData(
                self.handle(),
                buf.as_mut_ptr() as _,
                buf.len(),
                &mut count,
            )
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
    /// ```no_run
    /// let obj_id = [1u8;1];
    /// match PersistentObject::open (
    ///         ObjectStorageConstants::Private,
    ///         &obj_id,
    ///         DataFlag::ACCESS_WRITE) {
    ///     Ok(object) =>
    ///     {
    ///         let write_buf = [1u8;16];
    ///         object.write(& write_buf)?;
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
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
        match unsafe {
            raw::TEE_WriteObjectData(self.handle(), buf.as_ptr() as _, buf.len())
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Change the size of a data stream associate with the object.
    ///
    /// # Example
    ///
    /// ```no_run
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
    /// ```no_run
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

// The examples and detailed function explanation will be added after we test this struct and its
// functions.
/// An enumerator for [PersistentObject](PersistentObject)s.
pub struct ObjectEnumHandle {
    raw: *mut raw::TEE_ObjectEnumHandle,
}

impl ObjectEnumHandle {
    /// Allocate an object enumerator.
    /// Once an object enumerator has been allocated, it can be reused for multiple enumerations.
    pub fn allocate() -> Result<Self> {
        let raw_handle: *mut raw::TEE_ObjectEnumHandle = Box::into_raw(Box::new(ptr::null_mut()));
        match unsafe { raw::TEE_AllocatePersistentObjectEnumerator(raw_handle) } {
            raw::TEE_SUCCESS => Ok(Self { raw: raw_handle }),
            code => {
                unsafe {
                    drop(Box::from_raw(raw_handle));
                }
                Err(Error::from_raw_error(code))
            }
        }
    }

    /// Reset an object enumerator handle to its initial state after allocation.
    /// If an enumeration has been started, it is stopped.
    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetPersistentObjectEnumerator(*self.raw);
        }
    }

    /// Start the enumeration of all the [PersistentObject](PersistentObject)s in a given Trusted Storage.
    /// The object information can be retrieved by calling the function
    /// [ObjectEnumHandle::get_next](ObjectEnumHandle::get_next) repeatedly.
    pub fn start(&mut self, storage_id: u32) -> Result<()> {
        match unsafe { raw::TEE_StartPersistentObjectEnumerator(*self.raw, storage_id) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Get the next object in an enumeration and returns information about the object: type, size, identifier, etc.
    pub fn get_next<T>(
        &mut self,
        object_info: &mut ObjectInfo,
        object_id: &mut [u8],
    ) -> Result<u32> {
        let mut object_id_len: usize = 0;
        match unsafe {
            raw::TEE_GetNextPersistentObject(
                *self.raw,
                &mut object_info.raw,
                object_id.as_mut_ptr() as _,
                &mut object_id_len,
            )
        } {
            raw::TEE_SUCCESS => Ok(object_id_len as u32),
            code => Err(Error::from_raw_error(code)),
        }
    }
}

impl Drop for ObjectEnumHandle {
    /// Deallocates all resources associated with an object enumerator handle. After this function is called, the handle is no longer valid.
    ///
    /// # Panics
    ///
    /// 1) If object is not a valid opened object.
    /// 2) If the Implementation detects any other error.
    fn drop(&mut self) {
        unsafe {
            raw::TEE_FreePersistentObjectEnumerator(*self.raw);
            drop(Box::from_raw(self.raw));
        }
    }
}
