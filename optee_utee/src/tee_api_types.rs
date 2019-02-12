use libc::*;

pub type TEE_Result = uint32_t;

#[repr(C)]
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

#[repr(C)]
pub union TEE_Param {
    pub memref: Memref,
    pub value: Value
}

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

#[repr(C)] pub struct __TEE_TASessionHandle { _unused: [u8; 0] }
pub type TEE_TASessionHandle = *mut __TEE_TASessionHandle;

#[repr(C)] pub struct __TEE_PropSetHandle { _unused: [u8; 0] }
pub type TEE_PropSetHandle = *mut __TEE_PropSetHandle;

#[repr(C)] pub struct __TEE_ObjectHandle { _unused: [u8; 0] }
pub type TEE_ObjectHandle = *mut __TEE_ObjectHandle;

#[repr(C)] pub struct __TEE_ObjectEnumHandle { _unused: [u8; 0] }
pub type TEE_ObjectEnumHandle = *mut __TEE_ObjectEnumHandle;

#[repr(C)] pub struct __TEE_OperationHandle { _unused: [u8; 0] }
pub type TEE_OperationHandle = *mut __TEE_OperationHandle;
