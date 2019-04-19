#![allow(unused)]
use crate::{Attribute, Error, ErrorKind, ObjHandle, Result};
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

pub enum OperationMode {
    Encrypt = 0,
    Decrypt = 1,
    Sign = 2,
    Verify = 3,
    Mac = 4,
    Digest = 5,
    Derive = 6,
    IllegalValue = 0x7fffffff,
}

pub struct OperationInfo {
    raw: raw::TEE_OperationInfo,
}

impl OperationInfo {
    pub fn from_raw(raw: raw::TEE_OperationInfo) -> Self {
        Self { raw }
    }
}

pub struct OperationInfoMultiple {
    raw: *mut raw::TEE_OperationInfoMultiple,
    size: usize,
}

pub struct OperationHandle {
    raw: *mut raw::TEE_OperationHandle,
}

impl OperationHandle {
    pub fn from_raw(raw: *mut raw::TEE_OperationHandle) -> OperationHandle {
        Self { raw }
    }

    pub fn handle(&self) -> raw::TEE_OperationHandle {
        unsafe { *(self.raw) }
    }

    pub fn null() -> Self {
        OperationHandle::from_raw(ptr::null_mut())
    }

    pub fn allocate(algo: AlgorithmId, mode: OperationMode, max_key_size: usize) -> Result<Self> {
        let mut raw_handle: *mut raw::TEE_OperationHandle =
            Box::into_raw(Box::new(ptr::null_mut()));
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

    pub fn info(&self) -> OperationInfo {
        let mut raw_info: raw::TEE_OperationInfo = unsafe { mem::zeroed() };
        unsafe { raw::TEE_GetOperationInfo(self.handle(), &mut raw_info) };
        OperationInfo::from_raw(raw_info)
    }

    /// Here the multiple info total size is not sure
    /// Passed in array is supposed to provide enough size for this struct
    pub fn info_multiple(&self, info_buf: &mut [u8]) -> Result<OperationInfoMultiple> {
        let mut tmp_size: usize = 0;
        match unsafe {
            raw::TEE_GetOperationInfoMultiple(
                self.handle(),
                info_buf.as_ptr() as _,
                &mut (tmp_size as u32),
            )
        } {
            raw::TEE_SUCCESS => Ok(OperationInfoMultiple {
                raw: info_buf.as_ptr() as _,
                size: tmp_size,
            }),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::TEE_ResetOperation(self.handle());
        }
    }

    pub fn set_key<T: ObjHandle>(&self, object: &T) -> Result<()> {
        match unsafe { raw::TEE_SetOperationKey(self.handle(), object.handle()) } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn set_key_2<T: ObjHandle, D: ObjHandle>(&self, object1: &T, object2: &D) -> Result<()> {
        match unsafe {
            raw::TEE_SetOperationKey2(self.handle(), object1.handle(), object2.handle())
        } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn copy<T: OpHandle>(&mut self, src: &T) {
        unsafe {
            raw::TEE_CopyOperation(self.handle(), src.handle());
        }
    }
}

/// free before check it's not null
impl Drop for OperationHandle {
    fn drop(&mut self) {
        unsafe {
            if self.raw != ptr::null_mut() {
                raw::TEE_FreeOperation(self.handle());
            }
            Box::from_raw(self.raw);
        }
    }
}

pub trait OpHandle {
    fn handle(&self) -> raw::TEE_OperationHandle;
}

struct Digest(OperationHandle);

impl Digest {
    pub fn digest_update(&self, chunk: &[u8]) {
        unsafe {
            raw::TEE_DigestUpdate(self.handle(), chunk.as_ptr() as _, chunk.len() as u32);
        }
    }

    //hash size is dynamic changed so we returned it's updated size
    pub fn digest_do_final(&self, chunk: &[u8], hash: &mut [u8]) -> Result<usize> {
        let mut hash_size: usize = hash.len();
        match unsafe {
            raw::TEE_DigestDoFinal(
                self.handle(),
                chunk.as_ptr() as _,
                chunk.len() as u32,
                hash.as_mut_ptr() as _,
                &mut (hash_size as u32),
            )
        } {
            raw::TEE_SUCCESS => return Ok(hash_size),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn null() -> Self {
        Self(OperationHandle::null())
    }

    pub fn allocate(algo: AlgorithmId, max_key_size: usize) -> Result<Self> {
        match OperationHandle::allocate(algo, OperationMode::Digest, max_key_size) {
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

impl OpHandle for Digest {
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
        let mut mac_size: usize = mac.len();
        match unsafe {
            raw::TEE_MACComputeFinal(
                self.handle(),
                message.as_ptr() as _,
                message.len() as u32,
                mac.as_mut_ptr() as _,
                &mut (mac_size as u32),
            )
        } {
            raw::TEE_SUCCESS => Ok(mac_size),
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

    pub fn set_key_2<T: ObjHandle, D: ObjHandle>(&self, object1: &T, object2: &D) -> Result<()> {
        self.0.set_key_2(object1, object2)
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

pub struct Cipher(OperationHandle);

impl Cipher {
    pub fn init(&self, iv: &[u8]) {
        unsafe { raw::TEE_CipherInit(self.handle(), iv.as_ptr() as _, iv.len() as u32) };
    }

    pub fn update(&self, src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let mut dest_size = dest.len();
        match unsafe {
            raw::TEE_CipherUpdate(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn do_final(&self, src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let mut dest_size = dest.len();
        match unsafe {
            raw::TEE_CipherDoFinal(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
            )
        } {
            raw::TEE_SUCCESS => return Ok(dest_size),
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
        let mut dest_size = dest.len();
        match unsafe {
            raw::TEE_AEUpdate(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size);
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
        let mut dest_size = dest.len();
        let mut tag_size = tag.len();
        match unsafe {
            raw::TEE_AEEncryptFinal(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
                tag.as_mut_ptr() as _,
                &mut (tag_size as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok((dest_size, tag_size));
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn decrypt_final(&self, src: &[u8], dest: &mut [u8], tag: &[u8]) -> Result<usize> {
        let mut dest_size = dest.len();
        match unsafe {
            raw::TEE_AEDecryptFinal(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
                tag.as_ptr() as _,
                tag.len() as u32,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size);
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

    pub fn set_key_2<T: ObjHandle, D: ObjHandle>(&self, object1: &T, object2: &D) -> Result<()> {
        self.0.set_key_2(object1, object2)
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
    pub fn encrypt(&self, params: &[Attribute], src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        let mut dest_size = dest.len();
        match unsafe {
            raw::TEE_AsymmetricEncrypt(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn decrypt(&self, params: &[Attribute], src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let p: Vec<raw::TEE_Attribute> = params.iter().map(|p| p.raw()).collect();
        let mut dest_size = dest.len();
        match unsafe {
            raw::TEE_AsymmetricDecrypt(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut (dest_size as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(dest_size);
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
        let mut signature_size = signature.len();
        match unsafe {
            raw::TEE_AsymmetricSignDigest(
                self.handle(),
                p.as_ptr() as _,
                params.len() as u32,
                digest.as_ptr() as _,
                digest.len() as u32,
                signature.as_mut_ptr() as _,
                &mut (signature_size as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(signature_size);
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

pub enum AlgorithmId {
    AesEcbNopad = 0x10000010,
    AesCbcNopad = 0x10000110,
    AesCtr = 0x10000210,
    AesCts = 0x10000310,
    AesXts = 0x10000410,
    AesCbcMacNopad = 0x30000110,
    AesCbcMacPkcs5 = 0x30000510,
    AesCmac = 0x30000610,
    AesCcm = 0x40000710,
    AesGcm = 0x40000810,
    DesEcbNopad = 0x10000011,
    DesCbcNopad = 0x10000111,
    DesCbcMacNopad = 0x30000111,
    DesCbcMacPkcs5 = 0x30000511,
    Des3EcbNopad = 0x10000013,
    Des3CbcNopad = 0x10000113,
    Des3CbcMacNopad = 0x30000113,
    Des3CbcMacPkcs5 = 0x30000513,
    RsassaPkcs1V15MD5 = 0x70001830,
    RsassaPkcs1V15Sha1 = 0x70002830,
    RsassaPkcs1V15Sha224 = 0x70003830,
    RsassaPkcs1V15Sha256 = 0x70004830,
    RsassaPkcs1V15Sha384 = 0x70005830,
    RsassaPkcs1V15Sha512 = 0x70006830,
    RsassaPkcs1V15MD5Sha1 = 0x7000F830,
    RsassaPkcs1PssMgf1Sha1 = 0x70212930,
    RsassaPkcs1PssMgf1Sha224 = 0x70313930,
    RsassaPkcs1PssMgf1Sha256 = 0x70414930,
    RsassaPkcs1PssMgf1Sha384 = 0x70515930,
    RsassaPkcs1PssMgf1Sha512 = 0x70616930,
    RsaesPkcs1V1_5 = 0x60000130,
    RsaesPkcs1OAepMgf1Sha1 = 0x60210230,
    RsaesPkcs1OAepMgf1Sha224 = 0x60310230,
    RsaesPkcs1OAepMgf1Sha256 = 0x60410230,
    RsaesPkcs1OAepMgf1Sha384 = 0x60510230,
    RsaesPkcs1OAepMgf1Sha512 = 0x60610230,
    RsaNopad = 0x60000030,
    DSASha1 = 0x70002131,
    DSASha224 = 0x70003131,
    DSASha256 = 0x70004131,
    DhDeriveShaRedSecret = 0x80000032,
    Md5 = 0x50000001,
    Sha1 = 0x50000002,
    Sha224 = 0x50000003,
    Sha256 = 0x50000004,
    Sha384 = 0x50000005,
    Sha512 = 0x50000006,
    Md5Sha1 = 0x5000000F,
    HmacMd5 = 0x30000001,
    HmacSha1 = 0x30000002,
    HmacSha224 = 0x30000003,
    HmacSha256 = 0x30000004,
    HmacSha384 = 0x30000005,
    HmacSha512 = 0x30000006,
    IllegalValue = 0xefffffff,
}
pub enum ElementId {
    EccCurveNistP192 = 0x00000001,
    EccCurveNistP224 = 0x00000002,
    EccCurveNistP256 = 0x00000003,
    EccCurveNistP384 = 0x00000004,
    EccCurveNistP521 = 0x00000005,
}
//OP-TEE does not implement function: TEE_IsAlgorithmSuppddorted
