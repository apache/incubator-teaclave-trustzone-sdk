#![allow(unused)]
use crate::{Error, ErrorKind, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

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
        const EXTRACTABLE = 0x00000001;
        const ENCRYPT = 0x00000002;
        const DECRYPT = 0x00000004;
        const MAC = 0x00000008;
        const SIGN = 0x00000010;
        const VERIFY = 0x00000020;
        const DERIVE = 0x00000040;
    }
}

pub struct ObjectInfo {
    raw: raw::TEE_ObjectInfo,
}

impl ObjectInfo {
    pub fn new() -> Self {
        let raw = raw::TEE_ObjectInfo {
            objectType: 0u32,
            objectSize: 0u32,
            maxObjectSize: 0u32,
            objectUsage: 0u32,
            dataSize: 0u32,
            dataPosition: 0u32,
            handleFlags: 0u32,
        };
        Self { raw }
    }
}

pub struct ObjectHandle {
    raw: *mut raw::TEE_ObjectHandle,
}

impl ObjectHandle {
    pub fn from_raw(raw: *mut raw::TEE_ObjectHandle) -> ObjectHandle {
        Self { raw }
    }

    pub fn raw(&self) -> *mut raw::TEE_ObjectHandle {
        self.raw
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    pub fn write(&self, buf: &[u8]) -> Result<()> {
        unimplemented!()
    }

    pub fn info(&self, info: &mut ObjectInfo) -> Result<()> {
        unimplemented!()
    }

    pub fn ref_attribute(&self, id: u32, ref_attribute: &mut Attribute) -> Result<()> {
        unimplemented!()
    }

    pub fn value_attribute(&self, id: u32, value_attribute: &mut Attribute) -> Result<()> {
        unimplemented!()
    }

    pub fn restrict(&self, usage: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn copy_attribute_from(&mut self, handle: &mut ObjectHandle) -> Result<()> {
        unimplemented!()
    }

    pub fn generate_key(&self, key_size: u32, params: &[Attribute]) -> Result<()> {
        unimplemented!()
    }
}

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        unimplemented!()
    }
}

pub enum Whence {
    DataSeekSet,
    DataSeekCur,
    DataseekEnd,
}

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
    /// Diffie-Hellman sub prime number: `q`
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
    RsaPssSaltLength = 0xF0000A30,
    EccPublicValueX = 0xD0000141,
    EccPublicValueY = 0xD0000241,
    /// ECC private value: `d`
    EccPrivateValue = 0xC0000341,
    /// ECC Curve algorithm
    EccCurve = 0xF0000441,
    BitProtected = (1 << 28),
    BitValue = (1 << 29),
}

pub struct Attribute {
    raw: raw::TEE_Attribute,
}

impl Attribute {
    pub fn from_ref<T>(id: u32, buffer: &mut T) -> Self {
        let mut raw = raw::TEE_Attribute {
            attributeID: 0,
            content: raw::content {
                memref: raw::Memref {
                    buffer: 0 as *mut _,
                    size: 0,
                },
            },
        };
        unsafe {
            raw::TEE_InitRefAttribute(
                &mut raw,
                id,
                buffer as *mut T as *mut _,
                mem::size_of::<T>() as u32,
            );
        }

        Self { raw }
    }

    pub fn from_value(id: u32, a: u32, b: u32) -> Self {
        let mut raw = raw::TEE_Attribute {
            attributeID: 0,
            content: raw::content {
                value: raw::Value { a: 0, b: 0 },
            },
        };
        unsafe {
            raw::TEE_InitValueAttribute(&mut raw, id, a, b);
        }

        Self { raw }
    }
}

pub struct TransientObject(ObjectHandle);

impl TransientObject {
    pub fn new(object_type: u32, max_object_size: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn reset() {
        unimplemented!()
    }

    pub fn populate() -> Result<()> {
        unimplemented!()
    }
}

impl Drop for TransientObject {
    fn drop(&mut self) {
        unsafe {
            raw::TEE_FreeTransientObject(*self.0.raw);
        }
    }
}

pub struct PersistentObject(ObjectHandle);

impl PersistentObject {
    pub fn open<T>(storage_id: u32, object_id: &mut T, flags: u32) -> Result<PersistentObject> {
        let raw_object_handle = ptr::null_mut();
        unsafe {
            match raw::TEE_OpenPersistentObject(
                storage_id,
                object_id as *mut T as *mut _,
                mem::size_of::<T>() as u32,
                flags,
                raw_object_handle,
            ) {
                raw::TEE_SUCCESS => {
                    let handle = ObjectHandle::from_raw(raw_object_handle);
                    Ok(PersistentObject(handle))
                }
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    pub fn create<T, D>(
        storage_id: u32,
        object_id: &mut T,
        flags: u32,
        initial_data: &mut D,
    ) -> Result<PersistentObject> {
        unimplemented!()
    }

    pub fn rename<T>(&mut self, new_object_id: &mut T) -> Result<()> {
        unimplemented!()
    }
}

impl Drop for PersistentObject {
    fn drop(&mut self) {
        unimplemented!()
    }
}
