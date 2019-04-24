use crate::{Error, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

/// An attribute can be either a buffer attribute or a value attribute.
/// When an array of attributes is passed to a function, either to populate an object or to specify operation parameters,
/// and if an attribute identifier is present twice in the array, then only the first occurrence is used.
pub struct Attribute {
    raw: raw::TEE_Attribute,
}

impl Attribute {
    pub fn raw(&self) -> raw::TEE_Attribute {
        self.raw
    }

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

    /// from_ref and from_value are helper functions can be used to populate a single attribute either with a reference to a buffer or with integer values.
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
    /// an object info has following attributes:
    /// objectType: The parameter objectType passed when the object was created
    /// objectSize: The current size in bits of the object as determined by its attributes. This will always be less than or equal to maxObjectSize. Set to 0 for uninitialized and data only objects.
    /// maxObjectSize: The maximum objectSize which this object can represent.
    /// 1) For a persistent object, set to objectSize
    /// 2) For a transient object, set to the parameter maxObjectSize passed to TEE_AllocateTransientObject
    /// objectUsage: A bit vector of UsageFlag.
    /// dataSize:
    /// 1) For a persistent object, set to the current size of the data associated with the object
    /// 2) For a transient object, always set to 0
    /// dataPosition:
    /// 1) For a persistent object, set to the current position in the data for this handle. Data positions for different handles on the same object may differ.
    /// 2) For a transient object, set to 0.
    /// handleFlags: A bit vector containing one or more HandleFlag or DataFlag.
    ///
    /// Since raw struct is not implemented Copy attribute yet, every item in raw struct needs a function to extract.
    pub fn data_size(&self) -> usize {
        self.raw.dataSize as usize
    }

    pub fn object_size(&self) -> usize {
        self.raw.objectSize as usize
    }

    pub fn from_raw(raw: raw::TEE_ObjectInfo) -> Self {
        Self { raw }
    }
}

/// This structure indicates the possible start offset when moving a data position in the data stream associated with a persistent object.
/// DataSeekSet: the data position is set to offset bytes from the beginning of the data stream.
/// DataSeekCur: the data position is set to its current position plus offset.
/// DataSeekEnd: the data position is set to the size of the object data plus offset.
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

/// ObjectHandle is an opaque handle on an object.
/// These handles are returned by:
/// 1) TransientObject::allocate
/// 2) PersistentObject::{open, create}
///
/// Object handle is closed by:
/// 1) TransientObject::drop
/// 2) PersisdentObject::{close_and_delete, drop}
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

    /// Returns the characteristics of an object.
    /// It fills in the fields in the structure raw::TEE_ObjectInfo
    pub fn info(&self) -> Result<ObjectInfo> {
        let mut raw_info: raw::TEE_ObjectInfo = unsafe { mem::zeroed() };
        match unsafe { raw::TEE_GetObjectInfo1(self.handle(), &mut raw_info) } {
            raw::TEE_SUCCESS => Ok(ObjectInfo::from_raw(raw_info)),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Restricts the object usage flags of an object handle to contain at most the flags passed in the obj_usage parameter.
    pub fn restrict_usage(&mut self, obj_usage: UsageFlag) -> Result<()> {
        match unsafe { raw::TEE_RestrictObjectUsage1(self.handle(), obj_usage.bits()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Extracts one buffer attribute from an object. The attribute is identified by the argument id.
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

    /// Extracts one value attribute from an object. The attribute is identified by the argument id.
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

    /// Populates an uninitialized object handle with the attributes of another object handle;
    /// that is, it populates the attributes of this handle with the attributes of src_handle.
    /// It is most useful in the following situations:
    /// 1) To extract the public key attributes from a key-pair object
    /// 2) To copy the attributes from a persistent object into a transient object
    pub fn copy_attribute_from(&mut self, src_handle: &ObjectHandle) -> Result<()> {
        match unsafe { raw::TEE_CopyObjectAttributes1(self.handle(), src_handle.handle()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Generates a random key or a key-pair and populates a transient key object with the generated key material.
    /// The size of the desired key is passed in the keySize parameter and SHALL be less than or equal to
    /// the maximum key size specified when the transient object was created. The valid values for key size are pre-defined.
    pub fn generate_key(&self, key_size: usize, params: &[Attribute]) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        unsafe {
            match raw::TEE_GenerateKey(
                self.handle(),
                key_size as u32,
                p.as_slice().as_ptr() as _,
                params.len() as u32,
            ) {
                raw::TEE_SUCCESS => Ok(()),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    /// Attempts to read size bytes from the data stream associated with the object object into the buffer pointed to by buffer.
    /// The object handle SHALL have been opened with the read access right.
    /// The bytes are read starting at the position in the data stream currently stored in the object handle.
    /// The handle’s position is incremented by the number of bytes actually read.
    /// On completion the function sets the number of bytes actually read in the u32 pointed to by count.
    /// The value written to counter may be less than size if the number of bytes until the end-ofstream is less than size.
    /// It is set to 0 if the position at the start of the read operation is at or beyond the end-of-stream.
    /// These are the only cases where count may be less than size.
    /// No data transfer can occur past the current end of stream. If an attempt is made to read past the end-of-stream,
    /// the function stops reading data at the end-of-stream and returns the data read up to that point.
    /// This is still a success.
    /// The position indicator is then set at the end-of-stream.
    /// If the position is at, or past, the end of the data when this function is called, then no bytes are copied to buf and count is set to 0.
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

    /// Writes size bytes from the buffer pointed to by buffer to the data stream associated with the open object handle object.
    /// The object handle SHALL have been opened with the write access permission.
    /// If the current data position points before the end-of-stream, then size bytes are written to the data stream,
    /// overwriting bytes starting at the current data position. If the current data position points beyond the stream’s end,
    /// then the data stream is first extended with zero bytes until the length indicated by the data position indicator is reached,
    /// and then size bytes are written to the stream.
    /// Thus, the size of the data stream can be increased as a result of this operation.
    /// If the operation would move the data position indicator to beyond its maximum possible value, then ERROR Overflow is returned and the operation fails.
    /// The data position indicator is advanced by size. The data position indicators of other object handles opened on the same object are not changed.
    /// Writing in a data stream is atomic; either the entire operation completes successfully or no write is done.
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        match unsafe {
            raw::TEE_WriteObjectData(self.handle(), buf.as_ptr() as _, buf.len() as u32)
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Changes the size of a data stream.
    /// If size is less than the current size of the data stream then all bytes beyond size are removed.
    /// If size is greater than the current size of the data stream then the data stream is extended by adding zero bytes at the end of the stream.
    /// The object handle SHALL have been opened with the write access permission.
    /// This operation does not change the data position of any handle opened on the object.
    /// Note that if the current data position of such a handle is beyond size, the data position will point beyond the object data’s end after truncation.
    /// Truncating a data stream is atomic; either the data stream is successfully truncated or nothing happens
    pub fn truncate(&self, size: u32) -> Result<()> {
        match unsafe { raw::TEE_TruncateObjectData(self.handle(), size) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// ets the data position indicator associated with the object handle. The parameter whence controls the meaning of offset Whence.
    /// The function may be used to set the data position beyond the end of stream; this does not constitute an error.
    /// However, the data position indicator does have a maximum value which is MiscellaneousConstants::TeeDataMaxPosition.
    /// If the value of the data position indicator resulting from this operation would be greater than above value,
    /// the error Overflow is returned.
    /// If an attempt is made to move the data position before the beginning of the data stream,
    /// the data position is set at the beginning of the stream. This does not constitute an error.
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
        ///  Set for a persistent object
        const PERSISTENT = 0x00010000;
        /// For a persistent object, always set  For a transient object, initially cleared, then set when the object becomes initialized
        const INITIALIZED = 0x00020000;
        ///Following two flags are for crypto operation handles:
        /// Set if the required operation key has been set. Always set for digest operations
        const KEY_SET = 0x00040000;
        /// Set if the algorithm expects two keys to be set, using TEE_SetOperationKey2. This happens only if algorithm is set to TEE_ALG_AES_XTS or TEE_ALG_SM2_KEP.
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

pub trait ObjHandle {
    fn handle(&self) -> raw::TEE_ObjectHandle;
}

pub struct TransientObject(ObjectHandle);

impl TransientObject {
    pub fn null_object() -> Self {
        Self(ObjectHandle::from_raw(ptr::null_mut()))
    }

    /// Allocates an uninitialized transient object, i.e. a container for attributes.
    /// Transient objects are used to hold a cryptographic object (key or key-pair).
    /// The object type is defined in TransientObjectType with a predefined maximum size.
    /// As allocated, the container is uninitialized. It can be initialized by subsequently importing the object material, generating an object, deriving an object, or loading an object from the Trusted Storage. The initial value of the key usage contains all usage flags.
    /// You can use the function restrict_usage to restrict the usage of the container.
    /// The returned handle is used to refer to the newly-created container in all subsequent functions that require an object container: key management and operation functions.
    /// It is not necessary to provide the size of the object to allocate. In particular, for key objects.
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

    ///Resets a transient object to its initial state after allocation.
    ///If the object is currently initialized, the function clears the object of all its material. The object is then uninitialized again.
    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetTransientObject(self.handle());
        }
    }

    /// Populates an uninitialized object container with object attributes passed by the TA in the attrs parameter.
    /// When this function is called, the object SHALL be uninitialized. If the object is initialized, the caller SHALL first clear it using the function reset.
    /// Note that if the object type is a key-pair, then this function sets both the private and public attributes of the keypair.
    pub fn populate(&mut self, attrs: &mut [Attribute]) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = attrs.iter().map(|p| p.raw).collect();
        match unsafe {
            raw::TEE_PopulateTransientObject(self.0.handle(), p.as_ptr() as _, attrs.len() as u32)
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => return Err(Error::from_raw_error(code)),
        }
    }

    /// Wrap the functions of ObjectHandle so we do not expose the handle itself to TA.
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

    pub fn object_info(&self) -> Result<ObjectInfo> {
        self.0.info()
    }

    pub fn generate_key(&self, key_size: usize, params: &[Attribute]) -> Result<()> {
        self.0.generate_key(key_size, params)
    }
}

impl ObjHandle for TransientObject {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        self.0.handle()
    }
}

impl Drop for TransientObject {
    ///Deallocates a transient object previously allocated.
    ///After this function has been called, the object handle is no longer valid and all resources associated with the transient object SHALL have been reclaimed.
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
    /// opens a handle on an existing persistent object. It returns a handle that can be used to access the object’s attributes and data stream
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

    /// Creates a persistent object with initial attributes and an initial data stream content,
    /// and optionally returns either a handle on the created object, or NULL handle upon failure.
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

    /// Marks an object for deletion and closes the object handle.
    /// The object handle SHALL have been opened with the write-meta access right, which means access to the object is exclusive.
    /// Deleting an object is atomic; once this function returns, the object is definitely deleted and no more open
    /// handles for the object exist. This SHALL be the case even if the object or the storage containing it have become corrupted.
    /// The only reason this routine can fail is if the storage area containing the object becomes inaccessible (e.g. the
    /// user removes the media holding the object). In this case TEE_ERROR_STORAGE_NOT_AVAILABLE SHALL be returned.
    ///
    /// this function is conflicted with Drop implementation, when use this one to avoid panic:
    /// 1) call mem::forget for this structure to avoid double drop the object
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

    /// Changes the identifier of an object.
    /// The object handle SHALL have been opened with the write-meta access right, which means access to the object is exclusive.
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

    /// Wrap the functions of ObjectHandle so we do not expose the handle itself to TA.
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

impl ObjHandle for PersistentObject {
    fn handle(&self) -> raw::TEE_ObjectHandle {
        self.0.handle()
    }
}

impl Drop for PersistentObject {
    /// Closes an opened persistent object.
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
    /// Allocates a handle on an object enumerator. Once an object enumerator handle has been allocated, it can be reused for multiple enumerations.
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

    /// Resets an object enumerator handle to its initial state after allocation. If an enumeration has been started, it is stopped.
    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetPersistentObjectEnumerator(*self.raw);
        }
    }

    /// Starts the enumeration of all the persistent objects in a given Trusted Storage.
    /// The object information can be retrieved by calling the function get_next repeatedly.
    /// The enumeration does not necessarily reflect a given consistent state of the storage:
    /// During the enumeration, other TAs or other instances of the TA may create, delete, or rename objects.
    /// It is not guaranteed that all objects will be returned if objects are created or destroyed while the enumeration is in progress.
    /// To stop an enumeration, the TA can call the function TEE_ResetPersistentObjectEnumerator,
    /// which detaches the enumerator from the Trusted Storage.
    /// The TA can call the function drop to completely deallocate the object enumerator.
    /// If this function is called on an enumerator that has already been started, the enumeration is first reset then started.
    pub fn start(&mut self, storage_id: u32) -> Result<()> {
        match unsafe { raw::TEE_StartPersistentObjectEnumerator(*self.raw, storage_id) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// Gets the next object in an enumeration and returns information about the object: type, size, identifier, etc.
    /// If there are no more objects in the enumeration or if there is no enumeration started, then the function returns ERROR ItemNotFound.
    /// If while enumerating objects a corrupt object is detected, then its object ID SHALL be returned in
    /// objectID, objectInfo SHALL be zeroed, and the function SHALL return ERROR CorruptObject.
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
    /// Deallocates all resources associated with an object enumerator handle. After this function is called, the handle is no longer valid.
    fn drop(&mut self) {
        unsafe {
            raw::TEE_FreePersistentObjectEnumerator(*self.raw);
            Box::from_raw(self.raw);
        }
    }
}
