use crate::{Error, Result};
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
        let mut param_mask: u32 = 0xf;
        param_mask = param_mask & param_types;
        let param_0 = Parameter::new(
            &mut ((*tee_params)[0]),
            ParamTypeFlags::map_back(param_mask),
        );
        param_mask = 0xf0;
        param_mask = (param_mask & param_types) >> 4;
        let param_1 = Parameter::new(
            &mut ((*tee_params)[1]),
            ParamTypeFlags::map_back(param_mask),
        );
        param_mask = 0xf00;
        param_mask = (param_mask & param_types) >> 8;
        let param_2 = Parameter::new(
            &mut ((*tee_params)[2]),
            ParamTypeFlags::map_back(param_mask),
        );
        param_mask = 0xf000;
        param_mask = (param_mask & param_types) >> 12;
        let param_3 = Parameter::new(
            &mut ((*tee_params)[3]),
            ParamTypeFlags::map_back(param_mask),
        );

        Parameters {
            param_0,
            param_1,
            param_2,
            param_3,
        }
    }

    pub fn check_type(
        &mut self,
        flag_0: ParamTypeFlags,
        flag_1: ParamTypeFlags,
        flag_2: ParamTypeFlags,
        flag_3: ParamTypeFlags,
    ) -> Result<()> {
        if (flag_0 != self.param_0.param_type)
            || (flag_1 != self.param_1.param_type)
            || (flag_2 != self.param_2.param_type)
            || (flag_3 != self.param_3.param_type)
        {
            return Err(Error::from_raw_error(raw::TEE_ERROR_BAD_PARAMETERS));
        } else {
            Ok(())
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

impl ParamTypeFlags {
    pub fn map_back(value: u32) -> Self {
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

pub struct ParamTypes(u32);

impl ParamTypes {
    pub fn new(
        p0: ParamTypeFlags,
        p1: ParamTypeFlags,
        p2: ParamTypeFlags,
        p3: ParamTypeFlags,
    ) -> Self {
        ParamTypes((p0 as u32) | (p1 as u32) << 4 | (p2 as u32) << 8 | (p3 as u32) << 12)
    }
}

impl From<[u32; 4]> for ParamTypes {
    fn from(param_types: [u32; 4]) -> Self {
        ParamTypes(
            param_types[0] | param_types[1] << 4 | param_types[2] << 8 | param_types[3] << 12,
        )
    }
}

impl From<ParamTypes> for u32 {
    fn from(a: ParamTypes) -> u32 {
        a.0
    }
}
