use libc;
use optee_utee_sys as raw;
use std::fmt;
use std::io;
use std::io::Write;

pub struct Trace;

impl Trace {
    fn new() -> Self {
        Trace {}
    }

    pub fn _print(fmt: fmt::Arguments) {
        let mut writer = Trace::new();
        let result = writer.write_fmt(fmt);

        if let Err(e) = result {
            panic!("failed printing to trace: {}", e);
        }
    }

    pub fn set_level(level: i32) {
        unsafe {
            raw::trace_set_level(level);
        }
    }

    pub fn get_level() -> i32 {
        unsafe { raw::trace_get_level() }
    }
}

impl io::Write for Trace {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            raw::_utee_log(buf.as_ptr() as *const libc::c_void, buf.len());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
