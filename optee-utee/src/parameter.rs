use crate::{Error, ErrorKind, Result};
use libc::c_void;
use optee_utee_sys as raw;

pub struct Parameters(pub Parameter, pub Parameter, pub Parameter, pub Parameter);

impl Parameters {
    pub fn from_raw(tee_params: &mut [raw::TEE_Param; 4], param_types: u32) -> Self {
        let (f0, f1, f2, f3) = ParamTypes::from(param_types).into_flags();
        let p0 = Parameter::from_raw(&mut tee_params[0], f0);
        let p1 = Parameter::from_raw(&mut tee_params[1], f1);
        let p2 = Parameter::from_raw(&mut tee_params[2], f2);
        let p3 = Parameter::from_raw(&mut tee_params[3], f3);

        Parameters(p0, p1, p2, p3)
    }

    pub fn first(&self) -> &Parameter {
        &self.0
    }

    pub fn second(&self) -> &Parameter {
        &self.1
    }

    pub fn third(&self) -> &Parameter {
        &self.2
    }

    pub fn fourth(&self) -> &Parameter {
        &self.3
    }
}

pub struct Parameter {
    pub raw: *mut raw::TEE_Param,
    pub param_type: ParamType,
}

impl Parameter {
    pub fn from_raw(ptr: *mut raw::TEE_Param, param_type: ParamType) -> Self {
        Self {
            raw: ptr,
            param_type: param_type,
        }
    }

    pub fn raw(&self) -> *mut raw::TEE_Param { self.raw }

    pub fn get_value_a(&mut self) -> Result<u32> {
        match self.param_type {
            ParamType::ValueInput | ParamType::ValueInout => {
                let value = unsafe { (*self.raw).value.a };
                Ok(value)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn set_value_a(&mut self, value: u32) -> Result<()> {
        match self.param_type {
            ParamType::ValueOutput | ParamType::ValueInout => {
                unsafe { (*self.raw).value.a = value };
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn get_value_b(&mut self) -> Result<u32> {
        match self.param_type {
            ParamType::ValueInput | ParamType::ValueInout => {
                let value = unsafe { (*self.raw).value.b };
                Ok(value)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn set_value_b(&mut self, value: u32) -> Result<()> {
        match self.param_type {
            ParamType::ValueOutput | ParamType::ValueInout => {
                unsafe { (*self.raw).value.b = value };
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    // TODO: use memref type instead of separate ptr and size
    pub fn get_memref_ptr(&mut self) -> Result<*mut c_void> {
        match self.param_type {
            ParamType::MemrefInput
            | ParamType::MemrefOutput
            | ParamType::MemrefInout => {
                let buffer = unsafe { (*self.raw).memref.buffer };
                Ok(buffer)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn get_memref_size(&mut self) -> Result<u32> {
        match self.param_type {
            ParamType::MemrefInput
            | ParamType::MemrefOutput
            | ParamType::MemrefInout => {
                let size = unsafe { (*self.raw).memref.size };
                Ok(size)
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }
}

pub struct ParamTypes(u32);

impl ParamTypes {
    pub fn into_flags(
        &self,
    ) -> (
        ParamType,
        ParamType,
        ParamType,
        ParamType,
    ) {
        (
            (0x000fu32 & self.0).into(),
            (0x00f0u32 & self.0).into(),
            (0x0f00u32 & self.0).into(),
            (0xf000u32 & self.0).into(),
        )
    }
}

impl From<u32> for ParamTypes {
    fn from(value: u32) -> Self {
        ParamTypes(value)
    }
}

#[derive(Copy, Clone)]
pub enum ParamType {
    None = 0,
    ValueInput = 1,
    ValueOutput = 2,
    ValueInout = 3,
    MemrefInput = 5,
    MemrefOutput = 6,
    MemrefInout = 7,
}

impl From<u32> for ParamType {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamType::None,
            1 => ParamType::ValueInput,
            2 => ParamType::ValueOutput,
            3 => ParamType::ValueInout,
            5 => ParamType::MemrefInput,
            6 => ParamType::MemrefOutput,
            7 => ParamType::MemrefInout,
            _ => ParamType::None,
        }
    }
}
