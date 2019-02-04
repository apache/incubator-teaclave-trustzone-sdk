use libc::*;
use super::*;

#[repr(C)]
pub struct utee_params {
	  types: uint64_t,
	  vals: [uint64_t; TEE_NUM_PARAMS * 2],
}
