use libc::*;
use super::*;

extern "C" {
    pub fn trace_ext_puts(str: *const c_char); 
}
