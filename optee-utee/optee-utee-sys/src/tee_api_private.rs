use libc::*;
use super::*;

extern "C" {
    pub fn __utee_entry(func: c_ulong, session_id: c_ulong, up: *mut utee_params, cmd_id: c_ulong);
}
