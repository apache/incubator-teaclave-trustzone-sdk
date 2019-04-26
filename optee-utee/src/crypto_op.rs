use crate::{Attribute, Error, ObjHandle, Result};
use optee_utee_sys as raw;
use std::{mem, ptr};

/// Specify one of the available cryptographic operations. 
pub enum OperationMode {
    /// Encryption mode
    Encrypt = 0,
    /// Decryption mode
    Decrypt = 1,
    /// Signature generation mode
    Sign = 2,
    /// Signature verification mode 
    Verify = 3,
    /// MAC mode
    Mac = 4,
    /// Digest mode
    Digest = 5,
    /// Key derivation mode
    Derive = 6,
    /// Reserve for testing and validation
    IllegalValue = 0x7fffffff,
}

/// Represent the information about a crypto information.
///
/// # Fields
///
/// 1) `algorithm`: parameters passed when the operation is created
/// 2) `mode`: parameters passed when the operation is created
/// 3) `maxKeySize`: parameters passed when the operation is created
/// 4) `operationClass`: One of the constants from [OperationConstant](../object/enum.OperationConstant.html)
/// 5) `keySize`: 
/// 5.1) For an operation that makes no use of keys, 0.
/// 5.2) For an operation that uses a single key, the actual size of this key.
/// 5.3) For an operation that uses multiple keys, 0. (The actual value of keySize can be 
/// obtained from [OperationInfoMultiple](OperationInfoMultiple).
/// 6) `requiredKeyUsage`:
/// 6.1) For an operation that makes no use of keys, 0.
/// 6.2) For an operation that uses a single key, a bit vector that describes the necessary bits 
/// in the object usage for `set_key` functions to succeed without panicking.
/// 6.3) For an operation that uses multiple keys, 0. (The actual value of keySize can be 
/// obtained from [OperationInfoMultiple](OperationInfoMultiple).
/// 7) `digestLength`: For a [Mac](Mac), [AE](AE), or [Digest](Digest), describes the number of
///    bytes in the digest or tag.
/// 8) `handleState`: A bit vector describing the current state of the operation. Contains one or 
/// more of the [HandleFlag](../object/struct.HandleFlag.html). 
pub struct OperationInfo {
    raw: raw::TEE_OperationInfo,
}

impl OperationInfo {
    /// Return the `OperationInfo` struct based on the raw struct `TEE_OperationInfo`.
    pub fn from_raw(raw: raw::TEE_OperationInfo) -> Self {
        Self { raw }
    }

    /// Return the `maxDataSize` field of the `OperationInfo`.
    pub fn max_key_size(&self) -> u32 {
        self.raw.maxKeySize
    }
}

/// Represent the information about a crypto information which uses multiple keys.
///
/// # Fields
///
/// 1) `algorithm`: parameters passed when the operation is created
/// 2) `mode`: parameters passed when the operation is created
/// 3) `maxKeySize`: parameters passed when the operation is created
/// 4) `operationClass`: One of the constants from [OperationConstant](../object/enum.OperationConstant.html)
/// 5) `digestLength`: For a [Mac](Mac), [AE](AE), or [Digest](Digest), describes the number of
///    bytes in the digest or tag.
/// 6) `handleState`: A bit vector describing the current state of the operation. Contains one or
/// more of the [HandleFlag](../object/struct.HandleFlag.html).
/// 7) `numberOfKeys`: This is set to the number of keys required by this operation. May be 0 for an operation 
/// which requires no keys.
/// 8) `keyInformation`: This array contains `numberOfKeys` entries, each of which defines the details for
/// one key used by the operation, in the order they are defined. If the buffer is larger than required to
/// support `numberOfKeys` entries, the additional space is not initialized or modified. For each element:
/// 8.1) `keySize`: If a key is programmed in the operation, the actual size of this key, otherwise 0.
/// 8.2) `requiredKeyUsage`: A bit vector that describes the necessary bits in the object usage for
/// `set_key` or `set_key_2` to succeed without panicking.
pub struct OperationInfoMultiple {
    raw: *mut raw::TEE_OperationInfoMultiple,
    size: usize,
}

impl OperationInfoMultiple {
    /// Return the `OperationInfoMultiple` struct based on the raw struct `TEE_OperationInfo`.
    pub fn from_raw(raw: *mut raw::TEE_OperationInfoMultiple, size: usize) -> Self {
        Self { raw, size }
    }

    /// Return the raw struct `TEE_OperationInfo`.
    pub fn raw(&self) -> *mut raw::TEE_OperationInfoMultiple {
        self.raw
    }

    /// Return the size field of the `ObjectInfoMultiple`.
    pub fn size(&self) -> usize {
        self.size
    }
}

/// An opaque reference that identifies a particular cryptographic operation.
pub struct OperationHandle {
    raw: *mut raw::TEE_OperationHandle,
}

impl OperationHandle {
    fn from_raw(raw: *mut raw::TEE_OperationHandle) -> OperationHandle {
        Self { raw }
    }

    fn handle(&self) -> raw::TEE_OperationHandle {
        unsafe { *(self.raw) }
    }

    fn null() -> Self {
        OperationHandle::from_raw(ptr::null_mut())
    }

    fn allocate(algo: AlgorithmId, mode: OperationMode, max_key_size: usize) -> Result<Self> {
        let raw_handle: *mut raw::TEE_OperationHandle = Box::into_raw(Box::new(ptr::null_mut()));
        match unsafe {
            raw::TEE_AllocateOperation(
                raw_handle as *mut _,
                algo as u32,
                mode as u32,
                max_key_size as u32,
            )
        } {
            raw::TEE_SUCCESS => Ok(Self::from_raw(raw_handle)),
            code => Err(Error::from_raw_error(code)),
        }
    }

    fn info(&self) -> OperationInfo {
        let mut raw_info: raw::TEE_OperationInfo = unsafe { mem::zeroed() };
        unsafe { raw::TEE_GetOperationInfo(self.handle(), &mut raw_info) };
        OperationInfo::from_raw(raw_info)
    }

    // Here the multiple info total size is not sure
    // Passed in array is supposed to provide enough size for this struct
    fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        let mut tmp_size: u32 = info_buf.len() as u32;
        match unsafe {
            raw::TEE_GetOperationInfoMultiple(self.handle(), info_buf.as_ptr() as _, &mut tmp_size)
        } {
            raw::TEE_SUCCESS => Ok(OperationInfoMultiple::from_raw(
                info_buf.as_ptr() as _,
                tmp_size as usize,
            )),
            code => Err(Error::from_raw_error(code)),
        }
    }

    fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetOperation(self.handle());
        }
    }

    fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        match unsafe { raw::TEE_SetOperationKey(self.handle(), object.handle()) } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    fn set_key_2<T: ObjHandle, D: ObjHandle>(&self, object1: &T, object2: &D) -> Result<()> {
        match unsafe {
            raw::TEE_SetOperationKey2(self.handle(), object1.handle(), object2.handle())
        } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    fn copy<T: OpHandle>(&mut self, src: &T) {
        unsafe {
            raw::TEE_CopyOperation(self.handle(), src.handle());
        }
    }
}

// free before check it's not null
impl Drop for OperationHandle {
    /// Deallocate all resources associated with an operation handle. After this function is called, 
    /// the operation handle is no longer valid. All cryptographic material in the operation is destroyed. 
    fn drop(&mut self) {
        unsafe {
            if self.raw != ptr::null_mut() {
                raw::TEE_FreeOperation(self.handle());
            }
            Box::from_raw(self.raw);
        }
    }
}

/// A trait for a crypto operation to return its handle.
pub trait OpHandle {
    /// Return the handle of an operation.
    fn handle(&self) -> raw::TEE_OperationHandle;
}

pub struct Digest(OperationHandle);

impl Digest {
    pub fn digest_update(&self, chunk: &[u8]) {
        unsafe {
            raw::TEE_DigestUpdate(self.handle(), chunk.as_ptr() as _, chunk.len() as u32);
        }
    }

    //hash size is dynamic changed so we returned it's updated size
    pub fn digest_do_final(&self, chunk: &[u8], hash: &mut [u8]) -> Result<usize> {
        let mut hash_size: u32 = hash.len() as u32;
        match unsafe {
            raw::TEE_DigestDoFinal(
                self.handle(),
                chunk.as_ptr() as _,
                chunk.len() as u32,
                hash.as_mut_ptr() as _,
                &mut hash_size,
            )
        } {
            raw::TEE_SUCCESS => return Ok(hash_size as usize),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn null() -> Self {
        Self(OperationHandle::null())
    }

    /// Allocate a new cryptographic operation and sets the mode and algorithm type.
    ///
    /// # Parameters
    ///
    /// 1) `algo`: One of the cipher algorithms(`Md5`, `Sha1`,`Sha224`, `Sha256`, 
    /// `Sha384`, `Sha512`) listed in [AlgorithmId](AlgorithmId).
    /// 2) `max_key_size`: The maximum key sizes of different algorithms have been defined in
    ///    [TransientObjectType](../object/enum.TransientObjectType.html).
    ///
    /// # Example
    ///
    /// ```no_run
    /// match Digest::allocate(AlgorithmId::Md5, 128) {
    ///     Ok(operation) =>
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
    pub fn allocate(algo: AlgorithmId, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, OperationMode::Digest, max_key_size) {
            Ok(handle) => Ok(Self(handle)),
            Err(e) => Err(e),
        }
    }

    /// Return the characteristics of a Digest operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// match Digest::allocate(AlgorithmId::Md5, 128) {
    ///     Ok(operation) =>
    ///     {
    ///         let info = operation.info();
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// 1) If operation is not a valid opened object.
    /// 2) If the Implementation detects any other error.
    pub fn info(&self) -> OperationInfo {
        self.0.info()
    }

    /// Return the characteristics of a Digest operation with multiple keys.
    ///
    /// # Parameters
    /// `info_buf`: The buffer is supposed to save multiple keys, and its size should be large
    /// enough before passed in.
    ///
    /// The number of keys about this operation can be calculated as:
    /// `OperationInfoMultiple::size - size_of([OperationInfoMultiple](OperationInfoMultiple)) / size_of (
    /// raw::TEE_OperationInfoKey)+1`
    ///
    /// # Example
    ///
    /// ```no_run
    /// match Digest::allocate(AlgorithmId::Md5, 128) {
    ///     Ok(operation) =>
    ///     {
    ///         let mut buffer = [0u32, 10];
    ///         let info = operation.info_multiple(&mut buffer);
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    /// ```
    /// # Errors
    /// 
    /// 1) `ShortBuffer`: If the `info_buf` is not large enough to hold a
    ///    size_of([OperationInfoMultiple](OperationInfoMultiple) and the corresponding keys.
    ///
    /// # Panics
    ///
    /// 1) If operation is not a valid opened object.
    /// 2) If the Implementation detects any other error.
    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        self.0.info_multiple(info_buf)
    }

    /// For a multi-stage operation, this function reset the operation state after the initial `allocate` function
    /// with the addition of any keys which were configured subsequent to this so that the operation can be reused 
    /// with the same keys.
    /// When such a multi-stage operation is active, i.e. when it has been initialized but not yet successfully finalized,
    /// then the operation is reset to initial state. The operation key(s) are not cleared.
    /// 
    /// # Panics
    ///
    /// 1) If operation is not a valid opened object.
    /// 2) If the key has not been set yet.
    /// 3) Hardware or cryptographic algorithm failure
    /// 4) If the Implementation detects any other error.
    pub fn reset(&mut self) {
        self.0.reset()
    }

    /// Program the key of a Digest operartion; that is, it associates an oepration with a key.
    ///
    /// # Parameters
    ///
    /// `object`: The passed in key can be either a transient object or a persistent object.
    /// After the key is copied from the key ojbect, there is no longer any link betweeen the
    /// operation and the key object. The object handle can be closed or reset and this will 
    /// not affect the operation. This copied material exists until the operation is freed or
    /// another key is set.
    ///
    /// # Example
    /// match Digest::allocate(AlgorithmId::Md5, 128) {
    ///     Ok(operation) =>
    ///     {
    ///         match TransientObject::allocate(TransientObjectType::Aes, 128) {
    ///         Ok(object) =>
    ///         {
    ///             
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    ///         Ok(())
    ///     }
    ///     Err(e) => Err(e),
    /// }
    ///
    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        self.0.set_key(object)
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        self.0.copy(src)
    }
}

impl OpHandle for Digest {
    fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }
}

pub struct Cipher(OperationHandle);

impl Cipher {
    pub fn init(&self, iv: &[u8]) {
        unsafe { raw::TEE_CipherInit(self.handle(), iv.as_ptr() as _, iv.len() as u32) };
    }

    pub fn update(&self, src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let mut dest_size: u32 = dest.len() as u32;
        match unsafe {
            raw::TEE_CipherUpdate(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size as usize);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn do_final(&self, src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let mut dest_size: u32 = dest.len() as u32;
        match unsafe {
            raw::TEE_CipherDoFinal(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
            )
        } {
            raw::TEE_SUCCESS => return Ok(dest_size as usize),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn null() -> Self {
        Self(OperationHandle::null())
    }

    pub fn allocate(algo: AlgorithmId, mode: OperationMode, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, mode, max_key_size) {
            Ok(handle) => Ok(Self(handle)),
            Err(e) => Err(e),
        }
    }

    pub fn info(&self) -> OperationInfo {
        self.0.info()
    }

    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        self.0.info_multiple(info_buf)
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        self.0.set_key(object)
    }

    pub fn set_key_2<T: ObjHandle, D: ObjHandle>(&self, object1: &T, object2: &D) -> Result<()> {
        self.0.set_key_2(object1, object2)
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        self.0.copy(src)
    }
}

impl OpHandle for Cipher {
    fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }
}

pub struct Mac(OperationHandle);

impl Mac {
    pub fn init(&self, iv: &[u8]) {
        unsafe { raw::TEE_MACInit(self.handle(), iv.as_ptr() as _, iv.len() as u32) };
    }

    pub fn update(&self, chunk: &[u8]) {
        unsafe { raw::TEE_MACUpdate(self.handle(), chunk.as_ptr() as _, chunk.len() as u32) };
    }

    pub fn compute_final(&self, message: &[u8], mac: &mut [u8]) -> Result<usize> {
        let mut mac_size: u32 = mac.len() as u32;
        match unsafe {
            raw::TEE_MACComputeFinal(
                self.handle(),
                message.as_ptr() as _,
                message.len() as u32,
                mac.as_mut_ptr() as _,
                &mut mac_size,
            )
        } {
            raw::TEE_SUCCESS => Ok(mac_size as usize),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn compare_final(&self, message: &[u8], mac: &[u8]) -> Result<()> {
        match unsafe {
            raw::TEE_MACCompareFinal(
                self.handle(),
                message.as_ptr() as _,
                message.len() as u32,
                mac.as_ptr() as _,
                mac.len() as u32,
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

        pub fn null() -> Self {
        Self(OperationHandle::null())
    }

    pub fn allocate(algo: AlgorithmId, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, OperationMode::Mac, max_key_size) {
            Ok(handle) => Ok(Self(handle)),
            Err(e) => Err(e),
        }
    }

    pub fn info(&self) -> OperationInfo {
        self.0.info()
    }

    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        self.0.info_multiple(info_buf)
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        self.0.set_key(object)
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        self.0.copy(src)
    }
}

impl OpHandle for Mac {
    fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }
}

pub struct AE(OperationHandle);

impl AE {
    pub fn init(
        &self,
        nonce: &[u8],
        tag_len: usize,
        aad_len: usize,
        pay_load_len: usize,
    ) -> Result<()> {
        match unsafe {
            raw::TEE_AEInit(
                self.handle(),
                nonce.as_ptr() as _,
                nonce.len() as u32,
                tag_len as u32,
                aad_len as u32,
                pay_load_len as u32,
            )
        } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn update_add(&self, aad_data: &[u8]) {
        unsafe {
            raw::TEE_AEUpdateAAD(self.handle(), aad_data.as_ptr() as _, aad_data.len() as u32)
        };
    }

    pub fn update(&self, src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let mut dest_size: u32 = dest.len() as u32;
        match unsafe {
            raw::TEE_AEUpdate(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size as usize);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// both dest and tag are updated with different size
    pub fn encrypt_final(
        &self,
        src: &[u8],
        dest: &mut [u8],
        tag: &mut [u8],
    ) -> Result<(usize, usize)> {
        let mut dest_size: u32 = dest.len() as u32;
        let mut tag_size: u32 = tag.len() as u32;
        match unsafe {
            raw::TEE_AEEncryptFinal(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
                tag.as_mut_ptr() as _,
                &mut tag_size,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok((dest_size as usize, tag_size as usize));
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn decrypt_final(&self, src: &[u8], dest: &mut [u8], tag: &[u8]) -> Result<usize> {
        let mut dest_size: u32 = dest.len() as u32;
        match unsafe {
            raw::TEE_AEDecryptFinal(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
                tag.as_ptr() as _,
                tag.len() as u32,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size as usize);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn null() -> Self {
        Self(OperationHandle::null())
    }

    pub fn allocate(algo: AlgorithmId, mode: OperationMode, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, mode, max_key_size) {
            Ok(handle) => Ok(Self(handle)),
            Err(e) => Err(e),
        }
    }

    pub fn info(&self) -> OperationInfo {
        self.0.info()
    }

    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        self.0.info_multiple(info_buf)
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        self.0.set_key(object)
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        self.0.copy(src)
    }
}

impl OpHandle for AE {
    fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }
}

pub struct Asymmetric(OperationHandle);

impl Asymmetric {
    /// This function can update output size with short buffer error when buffer is too
    /// short, and example acipher utilizes this feature!
    /// Define this function as unsafe because we need to return Ok for short buffer error.
    pub unsafe fn encrypt(
        &self,
        params: &[Attribute],
        src: &[u8],
        dest: &mut [u8],
    ) -> Result<usize> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        let mut dest_size: u32 = dest.len() as u32;
        match {
            raw::TEE_AsymmetricEncrypt(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size as usize);
            }
            code => match code {
                raw::TEE_ERROR_SHORT_BUFFER => Ok(dest_size as usize),
                _ => Err(Error::from_raw_error(code)),
            },
        }
    }

    pub fn decrypt(&self, params: &[Attribute], src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        let mut dest_size: u32 = dest.len() as u32;
        match unsafe {
            raw::TEE_AsymmetricDecrypt(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut dest_size,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size as usize);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn sign_digest(
        &self,
        params: &[Attribute],
        digest: &[u8],
        signature: &mut [u8],
    ) -> Result<usize> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        let mut signature_size: u32 = signature.len() as u32;
        match unsafe {
            raw::TEE_AsymmetricSignDigest(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                digest.as_ptr() as _,
                digest.len() as u32,
                signature.as_mut_ptr() as _,
                &mut signature_size,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(signature_size as usize);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn verify_digest(
        &self,
        params: &[Attribute],
        digest: &[u8],
        signature: &[u8],
    ) -> Result<()> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        match unsafe {
            raw::TEE_AsymmetricVerifyDigest(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                digest.as_ptr() as _,
                digest.len() as u32,
                signature.as_ptr() as _,
                signature.len() as u32,
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn null() -> Self {
        Self(OperationHandle::null())
    }

    pub fn allocate(algo: AlgorithmId, mode: OperationMode, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, mode, max_key_size) {
            Ok(handle) => Ok(Self(handle)),
            Err(e) => Err(e),
        }
    }

    pub fn info(&self) -> OperationInfo {
        self.0.info()
    }

    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        self.0.info_multiple(info_buf)
    }

    /// Program the key of an Asymmetric operation. 
    /// Function usage is similar to [Digest::set_key](Digest::set_key) besides extra limitation on
    /// the parameter `object`.
    ///
    /// # Parameters:
    /// 
    /// 1) object: For algorithm [RsaNopad](AlgorithmId::RsaNopad), if the operation is
    ///   [Encrypt](OperationMode::Encrypt), then the object usage SHALL contain both flags
    ///   [ENCRYPT](../object/struct.UsageFlag.html#associatedconstant.ENCRYPT) and
    ///   [VERIFY](../object/struct.UsageFlag.html#associatedconstant.VERIFY); if the operation is
    ///   [Decrypt](OperationMode::Decrypt), then the object usage SHALL contain both flags
    ///   [DECRYPT](../object/struct.UsageFlag.html#associatedconstant.DECRYPT) and
    ///   [SIGN](../object/struct.UsageFlag.html#associatedconstant.SIGN).
    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        self.0.set_key(object)
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        self.0.copy(src)
    }
}

impl OpHandle for Asymmetric {
    fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }
}

pub struct DeriveKey(OperationHandle);

impl DeriveKey {
    pub fn allocate(algo: AlgorithmId, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, OperationMode::Derive, max_key_size) {
            Ok(handle) => Ok(Self(handle)),
            Err(e) => Err(e),
        }
    }

    pub fn update<T: ObjHandle>(&self, params: &mut [Attribute], object: &mut T) {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        unsafe {
            raw::TEE_DeriveKey(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                object.handle(),
            )
        };
    }

    pub fn info(&self) -> OperationInfo {
        self.0.info()
    }

    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        self.0.info_multiple(info_buf)
    }

    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        self.0.set_key(object)
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        self.0.copy(src)
    }
}

impl OpHandle for DeriveKey {
    fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
    }
}

pub struct Random();

impl Random {
    pub fn generate(res_buffer: &mut [u8]) {
        unsafe {
            raw::TEE_GenerateRandom(res_buffer.as_mut_ptr() as _, res_buffer.len() as _);
        }
    }
}

/// Algorithms that can be allocated as an crypto operation.
pub enum AlgorithmId {
    /// [Cipher](Cipher) supported algorithm 
    AesEcbNopad = 0x10000010,
    /// [Cipher](Cipher) supported algorithm
    AesCbcNopad = 0x10000110,
    /// [Cipher](Cipher) supported algorithm
    AesCtr = 0x10000210,
    /// [Cipher](Cipher) supported algorithm
    AesCts = 0x10000310,
    /// [Cipher](Cipher) supported algorithm
    AesXts = 0x10000410,
    /// [Mac](Mac) supported algorithm
    AesCbcMacNopad = 0x30000110,
    /// [Mac](Mac) supported algorithm
    AesCbcMacPkcs5 = 0x30000510,
    /// [Mac](Mac) supported algorithm
    AesCmac = 0x30000610,
    /// [Cipher](Cipher) supported algorithm
    AesCcm = 0x40000710,
    /// [Cipher](Cipher) supported algorithm
    AesGcm = 0x40000810,
    /// [Cipher](Cipher) supported algorithm
    DesEcbNopad = 0x10000011,
    /// [Cipher](Cipher) supported algorithm
    DesCbcNopad = 0x10000111,
    /// [Mac](Mac) supported algorithm
    DesCbcMacNopad = 0x30000111,
    /// [Mac](Mac) supported algorithm
    DesCbcMacPkcs5 = 0x30000511,
    /// [Cipher](Cipher) supported algorithm
    Des3EcbNopad = 0x10000013,
    /// [Cipher](Cipher) supported algorithm
    Des3CbcNopad = 0x10000113,
    /// [Mac](Mac) supported algorithm
    Des3CbcMacNopad = 0x30000113,
    /// [Mac](Mac) supported algorithm
    Des3CbcMacPkcs5 = 0x30000513,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15MD5 = 0x70001830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15Sha1 = 0x70002830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15Sha224 = 0x70003830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15Sha256 = 0x70004830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15Sha384 = 0x70005830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15Sha512 = 0x70006830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1V15MD5Sha1 = 0x7000F830,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1PssMgf1Sha1 = 0x70212930,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1PssMgf1Sha224 = 0x70313930,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1PssMgf1Sha256 = 0x70414930,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1PssMgf1Sha384 = 0x70515930,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    RsassaPkcs1PssMgf1Sha512 = 0x70616930,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaesPkcs1V15 = 0x60000130,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaesPkcs1OAepMgf1Sha1 = 0x60210230,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaesPkcs1OAepMgf1Sha224 = 0x60310230,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaesPkcs1OAepMgf1Sha256 = 0x60410230,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaesPkcs1OAepMgf1Sha384 = 0x60510230,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaesPkcs1OAepMgf1Sha512 = 0x60610230,
    /// [Asymmetric](Asymmetric) supported algorithm, [Encrypt](OperationMode::Encrypt) or
    /// [Decrypt](OperationMode::Decrypt) modes
    RsaNopad = 0x60000030,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    DSASha1 = 0x70002131,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    DSASha224 = 0x70003131,
    /// [Asymmetric](Asymmetric) supported algorithm, [Sign](OperationMode::Sign) or
    /// [Verify](OperationMode::Verify) modes
    DSASha256 = 0x70004131,
    /// [DeriveKey](DeriveKey) supported algorithm
    DhDeriveSharedSecret = 0x80000032,
    /// [Mac](Mac) supported algorithm
    Md5 = 0x50000001,
    /// [Mac](Mac) supported algorithm
    Sha1 = 0x50000002,
    /// [Mac](Mac) supported algorithm
    Sha224 = 0x50000003,
    /// [Mac](Mac) supported algorithm
    Sha256 = 0x50000004,
    /// [Mac](Mac) supported algorithm
    Sha384 = 0x50000005,
    /// [Mac](Mac) supported algorithm
    Sha512 = 0x50000006,
    /// [Mac](Mac) supported algorithm
    Md5Sha1 = 0x5000000F,
    /// [Mac](Mac) supported algorithm
    HmacMd5 = 0x30000001,
    /// [Mac](Mac) supported algorithm
    HmacSha1 = 0x30000002,
    /// [Mac](Mac) supported algorithm
    HmacSha224 = 0x30000003,
    /// [Mac](Mac) supported algorithm
    HmacSha256 = 0x30000004,
    /// [Mac](Mac) supported algorithm
    HmacSha384 = 0x30000005,
    /// [Mac](Mac) supported algorithm
    HmacSha512 = 0x30000006,
    /// Reserved for GlobalPlatform compliance test applications
    IllegalValue = 0xefffffff,
}

/// This specification defines support for optional cryptographic elements
pub enum ElementId {
    /// Source: `NIST`, Generic: Y, Size: 192 bits
    EccCurveNistP192 = 0x00000001,
    /// Source: `NIST`, Generic: Y, Size: 224 bits
    EccCurveNistP224 = 0x00000002,
    /// Source: `NIST`, Generic: Y, Size: 256 bits
    EccCurveNistP256 = 0x00000003,
    /// Source: `NIST`, Generic: Y, Size: 384 bits
    EccCurveNistP384 = 0x00000004,
    /// Source: `NIST`, Generic: Y, Size: 521 bits
    EccCurveNistP521 = 0x00000005,
}
// OP-TEE does not implement function: TEE_IsAlgorithmSuppddorted
// How to solve unused issue 
