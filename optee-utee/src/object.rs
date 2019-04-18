use crate::{Error, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

pub struct Attribute {
    raw: raw::TEE_Attribute,
}

impl Attribute {
    pub fn new_ref() -> Self {
        let raw = raw::TEE_Attribute {
            attributeID: 0,
            content: raw::content {
                memref: raw::Memref {
                    buffer: 0 as *mut _,
                    size: 0,
                },
            },
        };
        Self { raw }
    }

    pub fn new_value() -> Self {
        let raw = raw::TEE_Attribute {
            attributeID: 0,
            content: raw::content {
                value: raw::Value { a: 0, b: 0 },
            },
        };
        Self { raw }
    }

    pub fn from_ref(id: AttributeId, buffer: &mut [u8]) -> Self {
        let mut res = Attribute::new_ref();
        unsafe {
            raw::TEE_InitRefAttribute(
                &mut res.raw,
                id as u32,
                buffer as *mut [u8] as *mut _,
                buffer.len() as u32,
            );
        }
        res
    }

    pub fn from_value(id: AttributeId, a: u32, b: u32) -> Self {
        let mut res = Attribute::new_value();
        unsafe {
            raw::TEE_InitValueAttribute(&mut res.raw, id as u32, a, b);
        }
        res
    }
}

pub struct ObjectInfo {
    pub raw: raw::TEE_ObjectInfo,
}

impl ObjectInfo {
    /// Raw struct is not implemented Copy attribute yet.
    /// Every item in raw struct needs a function to extract.
    pub fn data_size(&self) -> usize {
        self.raw.dataSize as usize
    }

    pub fn from_raw (raw: raw::TEE_ObjectInfo) -> Self {
        Self { raw }
    }
}

pub enum Whence {
    DataSeekSet,
    DataSeekCur,
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

pub struct ObjectHandle {
    raw: *mut raw::TEE_ObjectHandle,
}

impl ObjectHandle {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        unsafe { *(self.raw) }
    }

    pub fn from_raw(raw: *mut raw::TEE_ObjectHandle) -> ObjectHandle {
        Self { raw }
    }

    pub fn raw(&self) -> *mut raw::TEE_ObjectHandle {
        self.raw
    }

    pub fn info(&self) -> Result<ObjectInfo> {
        let mut raw_info: raw::TEE_ObjectInfo = unsafe { mem::zeroed() };
        match unsafe { raw::TEE_GetObjectInfo1(self.handle(), &mut raw_info) } {
            raw::TEE_SUCCESS => Ok(ObjectInfo::from_raw(raw_info)),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn restrict_usage(&mut self, obj_usage: u32) -> Result<()> {
        match unsafe { raw::TEE_RestrictObjectUsage1(self.handle(), obj_usage) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn ref_attribute(&self, id: u32, ref_attr: &mut Attribute) -> Result<()> {
        match unsafe {
            raw::TEE_GetObjectBufferAttribute(
                self.handle(),
                id,
                ref_attr.raw.content.memref.buffer as _,
                &mut ref_attr.raw.content.memref.size as _,
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn value_attribute(&self, id: u32, value_attr: &mut Attribute) -> Result<()> {
        match unsafe {
            raw::TEE_GetObjectValueAttribute(
                self.handle(),
                id,
                &mut value_attr.raw.content.value.a as *mut _,
                &mut value_attr.raw.content.value.b as *mut _,
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn copy_attribute_from(&mut self, src_handle: &ObjectHandle) -> Result<()> {
        match unsafe { raw::TEE_CopyObjectAttributes1(self.handle(), src_handle.handle()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn generate_key(&self, key_size: u32, params: &[Attribute]) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw).collect();
        unsafe {
            match raw::TEE_GenerateKey(
                self.handle(),
                key_size,
                p.as_slice().as_ptr() as _,
                params.len() as u32,
            ) {
                raw::TEE_SUCCESS => Ok(()),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<u32> {
        let mut count: u32 = 0;
        match unsafe {
            raw::TEE_ReadObjectData(
                self.handle(),
                buf.as_mut_ptr() as _,
                buf.len() as u32,
                &mut count,
            )
        } {
            raw::TEE_SUCCESS => Ok(count),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        match unsafe {
            raw::TEE_WriteObjectData(self.handle(), buf.as_ptr() as _, buf.len() as u32)
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn truncate(&self, size: u32) -> Result<()> {
        match unsafe { raw::TEE_TruncateObjectData(self.handle(), size) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn seek(&self, offset: i32, whence: Whence) -> Result<()> {
        match unsafe { raw::TEE_SeekObjectData(self.handle(), offset, whence.into()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }
}

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
        const EXTRACTABLE = 0x00000001;
        const ENCRYPT = 0x00000002;
        const DECRYPT = 0x00000004;
        const MAC = 0x00000008;
        const SIGN = 0x00000010;
        const VERIFY = 0x00000020;
        const DERIVE = 0x00000040;
    }
}

pub enum MiscellaneousConstants {
    TeeDataMaxPosition = 0xFFFFFFFF,
    TeeObjectIdMaxLen = 64,
}

bitflags! {
    /// A set of flags that defines Handle features.
    pub struct HandleFlag: u32{
        const PERSISTENT = 0x00010000;
        const INITIALIZED = 0x00020000;
        const KEY_SET = 0x00040000;
        const EXPECT_TWO_KEYS = 0x00080000;
    }
}

pub enum OperationStates {
    Initial = 0x00000000,
    Active = 0x00000001,
}

pub enum OperationConstant {
    Cipher = 1,
    Mac = 3,
    Ae = 4,
    Digest = 5,
    AsymmetricCipher = 6,
    AsymmetricSignature = 7,
    KeyDerivation = 8,
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

pub enum TransientObjectType {
    Aes = 0xA0000010,
    Des = 0xA0000011,
    Des3 = 0xA0000013,
    HmacMd5 = 0xA0000001,
    HmacSha1 = 0xA0000002,
    HmacSha224 = 0xA0000003,
    HmacSha256 = 0xA0000004,
    HmacSha384 = 0xA0000005,
    HmacSha512 = 0xA0000006,
    RsaPublicKey = 0xA0000030,
    RsaKeypair = 0xA1000030,
    DsaPublicKey = 0xA0000031,
    DsaKeypair = 0xA1000031,
    DhKeypair = 0xA1000032,
    EcdsaPublicKey = 0xA0000041,
    EcdsaKeypair = 0xA1000041,
    EcdhPublicKey = 0xA0000042,
    EcdhKeypair = 0xA1000042,
    GenericSecret = 0xA0000000,
    CorruptedObject = 0xA00000BE,
    Data = 0xA00000BF,
}

pub trait Handle {
    fn handle(&self) -> raw::TEE_ObjectHandle;
}

pub struct TransientObject(ObjectHandle);

impl TransientObject {
    pub fn null_object() -> Self {
        Self(ObjectHandle::from_raw(ptr::null_mut()))
    }

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

    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetTransientObject(self.handle());
        }
    }

    pub fn populate(&mut self, attrs: &mut [Attribute]) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = attrs.iter().map(|p| p.raw).collect();
        match unsafe {
            raw::TEE_PopulateTransientObject(self.0.handle(), p.as_ptr() as _, attrs.len() as u32)
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => return Err(Error::from_raw_error(code)),
        }
    }

    pub fn info(&self) -> Result<ObjectInfo> {
        self.0.info()
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<u32> {
        self.0.read(buf)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.0.write(buf)
    }

    pub fn truncate(&self, size: u32) -> Result<()> {
        self.0.truncate(size)
    }

    pub fn seek(&self, offset: i32, whence: Whence) -> Result<()> {
        self.0.seek(offset, whence)
    }
}

impl Handle for TransientObject {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        self.0.handle()
    }
}

impl Drop for TransientObject {
    fn drop(&mut self) {
        unsafe {
            if self.0.raw != ptr::null_mut() {
                raw::TEE_FreeTransientObject(self.0.handle());
            }
            Box::from_raw(self.0.raw);
        }
    }
}

pub struct PersistentObject(ObjectHandle);

impl PersistentObject {
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
                object_id.len() as u32,
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
                    Box::from_raw(raw_handle);
                }
                Err(Error::from_raw_error(code))
            }
        }
    }

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
                object_id.len() as u32,
                flags.bits(),
                attributes,
                initial_data.as_ptr() as _,
                initial_data.len() as u32,
                raw_handle as *mut _,
            )
        } {
            raw::TEE_SUCCESS => {
                let handle = ObjectHandle::from_raw(raw_handle);
                Ok(Self(handle))
            }
            code => {
                unsafe {
                    Box::from_raw(raw_handle);
                }
                Err(Error::from_raw_error(code))
            }
        }
    }

    pub fn rename(&mut self, new_object_id: &[u8]) -> Result<()> {
        match unsafe {
            raw::TEE_RenamePersistentObject(
                self.0.handle(),
                new_object_id.as_ptr() as _,
                new_object_id.len() as u32,
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    // this function is conflicted with Drop implementation, when use this one to avoid panic:
    // 1) call mem::forget for this structure to avoid double drop the object
    pub fn close_and_delete(&mut self) -> Result<()> {
        match unsafe { raw::TEE_CloseAndDeletePersistentObject1(self.0.handle()) } {
            raw::TEE_SUCCESS => {
                unsafe {
                    Box::from_raw(self.0.raw);
                }
                return Ok(());
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn info(&self) -> Result<ObjectInfo> {
        self.0.info()
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<u32> {
        self.0.read(buf)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.0.write(buf)
    }

    pub fn truncate(&self, size: u32) -> Result<()> {
        self.0.truncate(size)
    }

    pub fn seek(&self, offset: i32, whence: Whence) -> Result<()> {
        self.0.seek(offset, whence)
    }
}

impl Handle for PersistentObject {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        self.0.handle()
    }
}

impl Drop for PersistentObject {
    fn drop(&mut self) {
        unsafe {
            if self.0.raw != Box::into_raw(Box::new(ptr::null_mut())) {
                raw::TEE_CloseObject(self.0.handle());
            }
            Box::from_raw(self.0.raw);
        }
    }
}

pub struct ObjectEnumHandle {
    raw: *mut raw::TEE_ObjectEnumHandle,
}
impl ObjectEnumHandle {
    pub fn allocate() -> Result<Self> {
        let raw_handle: *mut raw::TEE_ObjectEnumHandle = Box::into_raw(Box::new(ptr::null_mut()));
        match unsafe { raw::TEE_AllocatePersistentObjectEnumerator(raw_handle) } {
            raw::TEE_SUCCESS => Ok(Self { raw: raw_handle }),
            code => {
                unsafe {
                    Box::from_raw(raw_handle);
                }
                Err(Error::from_raw_error(code))
            }
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetPersistentObjectEnumerator(*self.raw);
        }
    }

    pub fn start(&mut self, storage_id: u32) -> Result<()> {
        match unsafe { raw::TEE_StartPersistentObjectEnumerator(*self.raw, storage_id) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn get_next<T>(
        &mut self,
        object_info: &mut ObjectInfo,
        object_id: &mut [u8],
    ) -> Result<u32> {
        let mut object_id_len: u32 = 0;
        match unsafe {
            raw::TEE_GetNextPersistentObject(
                *self.raw,
                &mut object_info.raw,
                object_id.as_mut_ptr() as _,
                &mut object_id_len,
            )
        } {
            raw::TEE_SUCCESS => Ok(object_id_len),
            code => Err(Error::from_raw_error(code)),
        }
    }
}

impl Drop for ObjectEnumHandle {
    fn drop(&mut self) {
        unsafe {
            raw::TEE_FreePersistentObjectEnumerator(*self.raw);
            Box::from_raw(self.raw);
        }
    }
}
