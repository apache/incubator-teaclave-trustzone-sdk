use crate::{Error, ErrorKind, Result};
use libc::c_void;
use optee_utee_sys as raw;

#[derive(Copy, Clone)]
pub struct Parameters {
    pub param_0: Parameter,
    pub param_1: Parameter,
    pub param_2: Parameter,
    pub param_3: Parameter,
}

impl Parameters {
    pub fn new(tee_params: &mut [raw::TEE_Param; 4], param_types: u32) -> Self {
        let param_0 = Parameter::new(&mut tee_params[0], (0x000fu32 & param_types).into());
        let param_1 = Parameter::new(&mut tee_params[1], ((0x00f0u32 & param_types) >> 4).into());
        let param_2 = Parameter::new(&mut tee_params[2], ((0x0f00u32 & param_types) >> 8).into());
        let param_3 = Parameter::new(&mut tee_params[3], ((0xf000u32 & param_types) >> 12).into());

        Parameters {
            param_0,
            param_1,
            param_2,
            param_3,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Parameter {
    pub raw: *mut raw::TEE_Param,
    pub param_type: ParamTypeFlags,
}

impl Parameter {
    pub fn new(ptr: *mut raw::TEE_Param, param_type: ParamTypeFlags) -> Self {
        Self {
            raw: ptr,
            param_type: param_type,
        }
    }

    pub fn get_value_a(&mut self) -> Result<u32> {
        match self.param_type {
            ParamTypeFlags::ValueInput | ParamTypeFlags::ValueInout => {
                let value = unsafe { (*self.raw).value.a };
                Ok(value)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn set_value_a(&mut self, value: u32) -> Result<()> {
        match self.param_type {
            ParamTypeFlags::ValueOutput | ParamTypeFlags::ValueInout => {
                unsafe { (*self.raw).value.a = value };
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn get_value_b(&mut self) -> Result<u32> {
        match self.param_type {
            ParamTypeFlags::ValueInput | ParamTypeFlags::ValueInout => {
                let value = unsafe { (*self.raw).value.b };
                Ok(value)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn set_value_b(&mut self, value: u32) -> Result<()> {
        match self.param_type {
            ParamTypeFlags::ValueOutput | ParamTypeFlags::ValueInout => {
                unsafe { (*self.raw).value.b = value };
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    // TODO: use memref type instead of separate ptr and size
    pub fn get_memref_ptr(&mut self) -> Result<*mut c_void> {
        match self.param_type {
            ParamTypeFlags::MemrefInput
            | ParamTypeFlags::MemrefOutput
            | ParamTypeFlags::MemrefInout => {
                let buffer = unsafe { (*self.raw).memref.buffer };
                Ok(buffer)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn get_memref_size(&mut self) -> Result<u32> {
        match self.param_type {
            ParamTypeFlags::MemrefInput
            | ParamTypeFlags::MemrefOutput
            | ParamTypeFlags::MemrefInout => {
                let size = unsafe { (*self.raw).memref.size };
                Ok(size)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ParamTypeFlags {
    None = 0,
    ValueInput = 1,
    ValueOutput = 2,
    ValueInout = 3,
    MemrefInput = 5,
    MemrefOutput = 6,
    MemrefInout = 7,
}

impl From<u32> for ParamTypeFlags {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamTypeFlags::None,
            1 => ParamTypeFlags::ValueInput,
            2 => ParamTypeFlags::ValueOutput,
            3 => ParamTypeFlags::ValueInout,
            5 => ParamTypeFlags::MemrefInput,
            6 => ParamTypeFlags::MemrefOutput,
            7 => ParamTypeFlags::MemrefInout,
            _ => ParamTypeFlags::None,
        }
    }
}
