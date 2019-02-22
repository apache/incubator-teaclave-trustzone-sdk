use super::*;
use libc::*;

#[repr(C)]
pub enum utee_time_category {
    UTEE_TIME_CAT_SYSTEM,
    UTEE_TIME_CAT_TA_PERSISTENT,
    UTEE_TIME_CAT_REE,
}

#[repr(C)]
pub enum utee_entry_func {
    UTEE_ENTRY_FUNC_OPEN_SESSION,
    UTEE_ENTRY_FUNC_CLOSE_SESSION,
    UTEE_ENTRY_FUNC_INVOKE_COMMAND,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum utee_cache_operation {
    TEE_CACHECLEAN,
    TEE_CACHEFLUSH,
    TEE_CACHEINVALIDATE,
}

#[repr(C)]
pub struct utee_params {
    types: uint64_t,
    vals: [uint64_t; TEE_NUM_PARAMS as usize * 2],
}

#[repr(C)]
pub struct utee_attribute {
    a: uint64_t,
    b: uint64_t,
    attribute_id: uint32_t,
}
