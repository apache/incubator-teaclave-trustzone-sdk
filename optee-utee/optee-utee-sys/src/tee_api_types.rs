use libc::*;

// Common Definitions

pub type TEE_Result = uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TEE_UUID {
    pub timeLow: uint32_t,
    pub timeMid: uint16_t,
    pub timeHiAndVersion: uint16_t,
    pub clockSeqAndNode: [uint8_t; 8],
}

#[repr(C)]
pub struct TEE_Identity {
    pub login: uint32_t,
    pub uuid: TEE_UUID,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Memref {
    pub buffer: *mut c_void,
    pub size: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Value {
    pub a: uint32_t,
    pub b: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union TEE_Param {
    pub memref: Memref,
    pub value: Value,
}

#[repr(C)]
pub struct __TEE_TASessionHandle {
    _unused: [u8; 0],
}
pub type TEE_TASessionHandle = *mut __TEE_TASessionHandle;

#[repr(C)]
pub struct __TEE_PropSetHandle {
    _unused: [u8; 0],
}
pub type TEE_PropSetHandle = *mut __TEE_PropSetHandle;

#[repr(C)]
pub struct __TEE_ObjectHandle {
    _unused: [u8; 0],
}
pub type TEE_ObjectHandle = *mut __TEE_ObjectHandle;

#[repr(C)]
pub struct __TEE_ObjectEnumHandle {
    _unused: [u8; 0],
}
pub type TEE_ObjectEnumHandle = *mut __TEE_ObjectEnumHandle;

#[repr(C)]
pub struct __TEE_OperationHandle {
    _unused: [u8; 0],
}
pub type TEE_OperationHandle = *mut __TEE_OperationHandle;

// Storage Definitions

pub type TEE_ObjectType = uint32_t;

#[repr(C)]
pub struct TEE_ObjectInfo {
    pub objectType: uint32_t,
    //remove to 2 unions here, only keep 1.1.1 spec version
    pub objectSize: uint32_t,
    pub maxObjectSize: uint32_t,
    pub objectUsage: uint32_t,
    pub dataSize: uint32_t,
    pub dataPosition: uint32_t,
    pub handleFlags: uint32_t,
}

#[repr(C)]
pub enum TEE_Whence {
    TEE_DATA_SEEK_SET,
    TEE_DATA_SEEK_CUR,
    TEE_DATA_SEEK_END,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union content {
    pub memref: Memref,
    pub value: Value,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEE_Attribute {
    pub attributeID: uint32_t,
    pub content: content,
}

// Cryptographic Operations API

#[repr(C)]
pub enum TEE_OperationMode {
    TEE_MODE_ENCRYPT,
    TEE_MODE_DECRYPT,
    TEE_MODE_SIGN,
    TEE_MODE_VERIFY,
    TEE_MODE_MAC,
    TEE_MODE_DIGEST,
    TEE_MODE_DERIVE,
}

#[repr(C)]
pub struct TEE_OperationInfo {
    pub algorithm: uint32_t,
    pub operationClass: uint32_t,
    pub mode: uint32_t,
    pub digestLength: uint32_t,
    pub maxKeySize: uint32_t,
    pub keySize: uint32_t,
    pub requiredKeyUsage: uint32_t,
    pub handleState: uint32_t,
}

#[repr(C)]
pub struct TEE_OperationInfoKey {
    pub keySize: uint32_t,
    pub requiredKeyUsage: uint32_t,
}

#[repr(C)]
pub struct TEE_OperationInfoMultiple {
    pub algorithm: uint32_t,
    pub operationClass: uint32_t,
    pub mode: uint32_t,
    pub digestLength: uint32_t,
    pub maxKeySize: uint32_t,
    pub handleState: uint32_t,
    pub operationState: uint32_t,
    pub numberOfKeys: uint32_t,
    pub keyInformation: *mut TEE_OperationInfoKey,
}

// Time & Date API

#[repr(C)]
pub struct TEE_Time {
    pub seconds: uint32_t,
    pub millis: uint32_t,
}

// TEE Arithmetical APIs

pub type TEE_BigInt = uint32_t;
pub type TEE_BigIntFMM = uint32_t;
pub type TEE_BigIntFMMContext = uint32_t;

// Tee Secure Element APIs

#[repr(C)]
pub struct __TEE_SEServiceHandle {
    _unused: [u8; 0],
}
pub type TEE_SEServiceHandle = *mut __TEE_SEServiceHandle;
#[repr(C)]
pub struct __TEE_SEReaderHandle {
    _unused: [u8; 0],
}
pub type TEE_SEReaderHandle = *mut __TEE_SEReaderHandle;
#[repr(C)]
pub struct __TEE_SESessionHandle {
    _unused: [u8; 0],
}
pub type TEE_SESessionHandle = *mut __TEE_SESessionHandle;
#[repr(C)]
pub struct __TEE_SEChannelHandle {
    _unused: [u8; 0],
}
pub type TEE_SEChannelHandle = *mut __TEE_SEChannelHandle;

#[repr(C)]
pub struct TEE_SEReaderProperties {
    pub sePresent: bool,
    pub teeOnly: bool,
    pub selectResponseEnable: bool,
}

#[repr(C)]
pub struct TEE_SEAID {
    pub buffer: *mut uint8_t,
    pub bufferLen: size_t,
}

// Other definitions
pub type TEE_ErrorOrigin = uint32_t;
pub type TEE_Session = *mut c_void;

pub const TEE_MEM_INPUT: uint32_t = 0x00000001;
pub const TEE_MEM_OUTPUT: uint32_t = 0x00000002;
pub const TEE_MEMREF_0_USED: uint32_t = 0x00000001;
pub const TEE_MEMREF_1_USED: uint32_t = 0x00000002;
pub const TEE_MEMREF_2_USED: uint32_t = 0x00000004;
pub const TEE_MEMREF_3_USED: uint32_t = 0x00000008;
pub const TEE_SE_READER_NAME_MAX: uint32_t = 20;
