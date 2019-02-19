use optee_teec_sys as raw;
use std::mem;

#[derive(Copy, Clone)]
pub struct Parameter {
    raw: raw::TEEC_Parameter,
    pub param_type: ParamTypeFlags,
}

impl Parameter {
    pub fn none() -> Self {
        let raw = unsafe { mem::zeroed() };
        Self {
            raw: raw,
            param_type: ParamTypeFlags::None,
        }
    }

    pub fn value(a: u32, b: u32, param_type: ParamTypeFlags) -> Self {
        let raw = raw::TEEC_Parameter {
            value: raw::TEEC_Value { a, b },
        };
        Self {
            raw: raw,
            param_type: param_type,
        }
    }

    pub fn tmpref<T>(buffer: *mut T, size: usize, param_type: ParamTypeFlags) -> Self {
        let raw = raw::TEEC_Parameter {
            tmpref: raw::TEEC_TempMemoryReference {
                buffer: buffer as *mut libc::c_void,
                size: size as libc::size_t,
            },
        };
        Self {
            raw: raw,
            param_type: param_type,
        }
    }
}

impl From<Parameter> for raw::TEEC_Parameter {
    fn from(a: Parameter) -> raw::TEEC_Parameter {
        a.raw
    }
}

#[derive(Copy, Clone)]
pub enum ParamTypeFlags {
    None = 0,
    ValueInput = 1,
    ValueOutput = 2,
    ValueInout = 3,
    MemrefTempInput = 5,
    MemrefTempOutput = 6,
    MemrefTempInout = 7,
    MemrefWhole = 0xC,
    MemrefPartialInput = 0xD,
    MemrefPartialOutput = 0xE,
    MemrefPartialInout = 0xF,
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
