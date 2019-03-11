use crate::{Error, Result};
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
        let param_0 = Parameter::new(
            &mut tee_params[0],
            ParamTypeFlags::from(0xfu32 & param_types),
        );
        let param_1 = Parameter::new(
            &mut tee_params[1],
            ParamTypeFlags::from((0xf0u32 & param_types) >> 4),
        );
        let param_2 = Parameter::new(
            &mut tee_params[2],
            ParamTypeFlags::from((0xf00u32 & param_types) >> 8),
        );
        let param_3 = Parameter::new(
            &mut tee_params[3],
            ParamTypeFlags::from((0xf000u32 & param_types) >> 12),
        );

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

    pub fn get_value_a(&mut self) -> std::result::Result<u32, Error> {
        match self.param_type {
            ParamTypeFlags::ValueInput | ParamTypeFlags::ValueInout => {
                unsafe { return Ok((*self.raw).value.a) };
            }
            _ => {
                return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
            }
        }
    }

    pub fn set_value_a(&mut self, value: u32) -> Result<()> {
        match self.param_type {
            ParamTypeFlags::ValueOutput | ParamTypeFlags::ValueInout => {
                unsafe { (*self.raw).value.a = value };
                Ok(())
            }
            _ => {
                return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
            }
        }
    }

    pub fn get_value_b(&mut self) -> std::result::Result<u32, Error> {
        match self.param_type {
            ParamTypeFlags::ValueInput | ParamTypeFlags::ValueInout => {
                unsafe { return Ok((*self.raw).value.b) };
            }
            _ => {
                return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
            }
        }
    }

    pub fn set_value_b(&mut self, value: u32) -> Result<()> {
        match self.param_type {
            ParamTypeFlags::ValueOutput | ParamTypeFlags::ValueInout => {
                unsafe { (*self.raw).value.b = value };
                Ok(())
            }
            _ => {
                return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
            }
        }
    }

    //Todo: diffrentiate memref types when get the reference
    pub fn get_memref_ptr(&mut self) -> std::result::Result<*mut c_void, Error> {
        match self.param_type {
            ParamTypeFlags::MemrefInput
            | ParamTypeFlags::MemrefOutput
            | ParamTypeFlags::MemrefInout => {
                unsafe { return Ok((*self.raw).memref.buffer) };
            }
            _ => {
                return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
            }
        }
    }

    pub fn get_memref_size(&mut self) -> std::result::Result<u32, Error> {
        match self.param_type {
            ParamTypeFlags::MemrefInput
            | ParamTypeFlags::MemrefOutput
            | ParamTypeFlags::MemrefInout => {
                unsafe { return Ok((*self.raw).memref.size) };
            }
            _ => {
                return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
        return match value {
            0 => ParamTypeFlags::None,
            1 => ParamTypeFlags::ValueInput,
            2 => ParamTypeFlags::ValueOutput,
            3 => ParamTypeFlags::ValueInout,
            5 => ParamTypeFlags::MemrefInput,
            6 => ParamTypeFlags::MemrefOutput,
            7 => ParamTypeFlags::MemrefInout,
            _ => ParamTypeFlags::None,
        };
    }
}
