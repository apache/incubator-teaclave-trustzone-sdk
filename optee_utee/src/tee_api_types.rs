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

pub type ParamTypes = u32;
pub type SessionP = *mut *mut c_void;
