use libc::*;
use super::*;

extern "C" {
    pub fn trace_ext_puts(str: *const c_char);
    pub fn trace_ext_get_thread_id() -> c_int;
    pub fn trace_set_level(level: c_int);
    pub fn trace_get_level() -> c_int;
    pub fn trace_printf(func: *const c_char, line: c_int, level: c_int, level_ok: bool, fmt: *const c_char, ...);
}
