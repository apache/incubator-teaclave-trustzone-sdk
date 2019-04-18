#![allow(unused)]
use crate::{Error, ErrorKind, Handle, Result};
use bitflags::bitflags;
use optee_utee_sys as raw;
use std::mem;
use std::ptr;

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
}

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

pub struct Operation(OperationHandle);

impl Operation {
    pub fn null_operation() -> Self {
        Self(OperationHandle::from_raw(ptr::null_mut()))
    }

    pub fn handle(&self) -> raw::TEE_OperationHandle {
        self.0.handle()
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
            raw::TEE_SUCCESS => {
                let handle = OperationHandle::from_raw(raw_handle);
                return Ok(Self(handle));
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn set_key<T: Handle>(&self, object: &T) -> Result<()> {
        match unsafe { raw::TEE_SetOperationKey(self.handle(), object.handle()) } {
            raw::TEE_SUCCESS => return Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn cipher_init(&self, iv: &[u8]) {
        unsafe { raw::TEE_CipherInit(self.handle(), iv.as_ptr() as _, iv.len() as u32) };
    }

    pub fn cipher_update(&self, src: &[u8], dest: &mut [u8]) -> Result<usize> {
        let mut out_len: u32 = dest.len() as u32;
        match unsafe {
            raw::TEE_CipherUpdate(
                self.handle(),
                src.as_ptr() as _,
                src.len() as u32,
                dest.as_mut_ptr() as _,
                &mut out_len,
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(out_len as usize);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn mac_init(&self, iv: &[u8]) {
        unsafe { raw::TEE_MACInit(self.handle(), iv.as_ptr() as _, iv.len() as u32) };
    }

    pub fn mac_update(&self, chunk: &[u8]) {
        unsafe { raw::TEE_MACUpdate(self.handle(), chunk.as_ptr() as _, chunk.len() as u32) };
    }

    /// output mac size is unsure when passed in, so we return its result
    pub fn mac_compute_final(&self, message: &[u8], mac: &mut [u8]) -> Result<usize> {
        let mut out_len: usize = mac.len();
        match unsafe {
            raw::TEE_MACComputeFinal(
                self.handle(),
                message.as_ptr() as _,
                message.len() as u32,
                mac.as_mut_ptr() as _,
                &mut (out_len as u32),
            )
        } {
            raw::TEE_SUCCESS => {
                return Ok(out_len);
            }
            code => Err(Error::from_raw_error(code)),
        }
    }
}
/// free before check it's not null
impl Drop for Operation {
    fn drop(&mut self) {
        unsafe {
            if self.0.raw != ptr::null_mut() {
                raw::TEE_FreeOperation(self.0.handle());
            }
            Box::from_raw(self.0.raw);
        }
    }
}

pub struct Random();

impl Random {
    pub fn generate(res_buffer: &mut [u8]) {
        unsafe {
        raw::TEE_GenerateRandom(
            res_buffer.as_mut_ptr() as _,
            res_buffer.len() as _,
        );
        }
    }
}
