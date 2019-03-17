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
    pub param_type: ParamTypeFlags,
}

impl Parameter {
    pub fn from_raw(ptr: *mut raw::TEE_Param, param_type: ParamTypeFlags) -> Self {
        Self {
            raw: ptr,
            param_type: param_type,
        }
    }

    pub fn raw(&self) -> *mut raw::TEE_Param { self.raw }

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

pub struct ParamTypes(u32);

impl ParamTypes {
    pub fn into_flags(
        &self,
    ) -> (
        ParamTypeFlags,
        ParamTypeFlags,
        ParamTypeFlags,
        ParamTypeFlags,
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
