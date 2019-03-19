#![allow(unused)]
use crate::{Error, ErrorKind, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

bitflags! {
    pub struct DataFlag: u32 {
        const ACCESS_READ = 0x00000001;
        const ACCESS_WRITE = 0x00000002;
        const ACCESS_WRITE_META = 0x00000004;
        const SHARE_READ = 0x00000010;
        const SHARE_WRITE = 0x00000020;
        const OVERWRITE = 0x00000400;
    }
}

bitflags! {
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

    pub fn info(&self) -> Result<ObjectInfo> {
        unimplemented!()
    }

    pub fn attribute(&self, id: u32) -> Result<Attribute> {
        unimplemented!()
    }

    pub fn restrict(&self, usage: u32) -> Result<()> {
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
    SecretValue = 0xC0000000,
    RsaModulus = 0xD0000130,
    RsaPublicExponent = 0xD0000230,
    RsaPrivateExponent = 0xC0000330,
    RsaPrime1 = 0xC0000430,
    RsaPrime2 = 0xC0000530,
    RsaExponent1 = 0xC0000630,
    RsaExponent2 = 0xC0000730,
    RsaCoefficient = 0xC0000830,
    DsaPrime = 0xD0001031,
    DsaSubprime = 0xD0001131,
    DsaBase = 0xD0001231,
    DsaPublicValue = 0xD0000131,
    DsaPrivateValue = 0xC0000231,
    DhPrime = 0xD0001032,
    DhSubprime = 0xD0001132,
    DhBase = 0xD0001232,
    DhXBits = 0xF0001332,
    DhPublicValue = 0xD0000132,
    DhPrivateValue = 0xC0000232,
    RsaOaepLabel = 0xD0000930,
    RsaPssSaltLength = 0xF0000A30,
    EccPublicValueX = 0xD0000141,
    EccPublicValueY = 0xD0000241,
    EccPrivateValue = 0xC0000341,
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
        unimplemented!()
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
        data: &mut D,
    ) -> Result<PersistentObject> {
        unimplemented!()
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }

    pub fn write(&self, buf: &[u8]) -> Result<()> {
        self.0.write(buf)
    }

    pub fn rename<T>(&mut self, new_object_id: &mut T) -> Result<()> {
        unimplemented!()
    }

    pub fn handle(&self) -> &ObjectHandle {
        &self.0
    }

    pub fn into_handle(self) -> ObjectHandle {
        self.0
    }
}
