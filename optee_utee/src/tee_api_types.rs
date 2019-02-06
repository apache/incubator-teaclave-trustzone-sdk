use libc::*;

#[allow(non_camel_case_types)]
pub type TEE_Result = uint32_t;

#[repr(C)]
pub struct TEE_UUID {
    time_low: uint32_t,
    time_mid: uint16_t,
    time_hi_and_version: uint16_t,
    clock_seq_and_node: [uint8_t; 8],
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
