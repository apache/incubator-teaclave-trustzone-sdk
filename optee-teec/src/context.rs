use crate::{Error, Operation, Result, Session, Uuid};
use crate::{Param, ParamNone};
use libc;
use optee_teec_sys as raw;
use std::boxed::Box;
use std::ptr;

/// An abstraction of the logical connection between a client application and a
/// TEE.
pub struct Context {
    raw: *mut raw::TEEC_Context,
}

impl Context {
    /// Creates a TEE client context object.
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = Context::new().unwrap();
    /// ```
    pub fn new() -> Result<Context> {
        Context::new_raw(0, true).map(|raw| Context { raw })
    }

    /// Creates a raw TEE client context with implementation defined parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// let raw_ctx: optee_teec_sys::TEEC_Context = Context::new_raw(0, true).unwrap();
    /// ```
    pub fn new_raw(fd: libc::c_int, reg_mem: bool) -> Result<*mut raw::TEEC_Context> {
        let raw_ctx = Box::into_raw(Box::new(raw::TEEC_Context {
            fd: fd,
            reg_mem: reg_mem,
        }));
        unsafe {
            match raw::TEEC_InitializeContext(ptr::null_mut() as *mut libc::c_char, raw_ctx) {
                raw::TEEC_SUCCESS => Ok(raw_ctx),
                code => Err(Error::from_raw_error(code)),
            }
        }
    }

    /// Converts a TEE client context to a raw pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = Context::new().unwrap();
    /// let mut raw_ptr: *mut optee_teec_sys::TEEC_Context = ctx.as_mut_raw_ptr();
    /// ```
    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEEC_Context {
        self.raw
    }

    /// Opens a new session with the specified trusted application.
    ///
    /// The target trusted application is specified by `uuid`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = Context::new().unwrap();
    /// let uuid = Uuid::parse_str("8abcf200-2450-11e4-abe2-0002a5d5c51b").unwrap();
    /// let session = ctx.open_session(uuid).unwrap();
    /// ```
    pub fn open_session(&mut self, uuid: Uuid) -> Result<Session> {
        Session::new(
            self,
            uuid,
            None::<Operation<ParamNone, ParamNone, ParamNone, ParamNone>>,
        )
    }

    /// Opens a new session with the specified trusted application, pass some
    /// parameters to TA by an operation.
    ///
    /// The target trusted application is specified by `uuid`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = Context::new().unwrap();
    /// let uuid = Uuid::parse_str("8abcf200-2450-11e4-abe2-0002a5d5c51b").unwrap();
    /// let p0 = Parameter::from_value(42, 0, ParamType::ValueInout);
    /// let p1 = Parameter::new();
    /// let p2 = Parameter::new();
    /// let p3 = Parameter::new();
    /// let mut operation = Operation::new(0, p0, p1, p2, p3);
    /// let session = ctx.open_session_with_operation(uuid, operation).unwrap();
    /// ```
    pub fn open_session_with_operation<A: Param, B: Param, C: Param, D: Param>(
        &mut self,
        uuid: Uuid,
        operation: Operation<A, B, C, D>,
    ) -> Result<Session> {
        Session::new(self, uuid, Some(operation))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            raw::TEEC_FinalizeContext(self.raw);
            Box::from_raw(self.raw);
        }
    }
}
