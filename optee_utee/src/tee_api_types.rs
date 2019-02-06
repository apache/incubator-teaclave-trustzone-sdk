use libc::*;

pub type TEE_Result = uint32_t;

#[repr(C)]
pub struct TEE_UUID {
    timeLow: uint32_t,
    timeMid: uint16_t,
    timeHiAndVersion: uint16_t,
    clockSeqAndNode: [uint8_t; 8],
}

#[repr(C)]
pub struct TEE_Identity {
    login: uint32_t,
    uuid: TEE_UUID,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Memref {
    buffer: *mut c_void,
    size: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Value {
    a: uint32_t,
    b: uint32_t,
}

#[repr(C)]
pub union TEE_Param {
    memref: Memref,
    value: Value
}
