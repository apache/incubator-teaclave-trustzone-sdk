use libc::*;
use super::*;

extern "C" {
    pub fn TEE_Malloc(len: uint32_t, hint: uint32_t) -> *mut c_void;
    pub fn TEE_MemMove(dest: *mut c_void, src: *const c_void, fsize: uint32_t) -> *mut c_void;
    pub fn TEE_Free(buffer: *mut c_void);
}
