use optee_teec_sys as raw;
use std::mem;

#[derive(Copy, Clone)]
pub struct Parameters(pub Parameter, pub Parameter, pub Parameter, pub Parameter);

impl Parameters {
    pub fn new(teec_params: [raw::TEEC_Parameter; 4], param_types: u32) -> Self {
        let (f0, f1, f2, f3) = ParamTypes::from(param_types).into_flags();
        let p0 = Parameter::from_raw(teec_params[0], f0);
        let p1 = Parameter::from_raw(teec_params[1], f1);
        let p2 = Parameter::from_raw(teec_params[2], f2);
        let p3 = Parameter::from_raw(teec_params[3], f3);

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

    pub fn from_value(a: u32, b: u32, param_type: ParamTypeFlags) -> Self {
        let raw = raw::TEEC_Parameter {
            value: raw::TEEC_Value { a, b },
        };
        Self {
            raw: raw,
            param_type: param_type,
        }
    }

    pub fn from_tmpref<T>(buffer: *mut T, size: usize, param_type: ParamTypeFlags) -> Self {
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

    pub fn from_raw(raw: raw::TEEC_Parameter, param_type: ParamTypeFlags) -> Self {
        Self {
            raw: raw,
            param_type: param_type,
        }
    }

    pub fn value(&self) -> (u32, u32) {
        unsafe {
            (self.raw.value.a, self.raw.value.a)
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

impl From<u32> for ParamTypeFlags {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamTypeFlags::None,
            1 => ParamTypeFlags::ValueInput,
            2 => ParamTypeFlags::ValueOutput,
            3 => ParamTypeFlags::ValueInout,
            5 => ParamTypeFlags::MemrefTempInput,
            6 => ParamTypeFlags::MemrefTempOutput,
            7 => ParamTypeFlags::MemrefTempInout,
            0xC => ParamTypeFlags::MemrefWhole,
            0xD => ParamTypeFlags::MemrefPartialInput,
            0xE => ParamTypeFlags::MemrefPartialOutput,
            0xF => ParamTypeFlags::MemrefPartialInout,
            _ => ParamTypeFlags::None,
        }
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

    pub fn into_flags(&self) -> (ParamTypeFlags, ParamTypeFlags, ParamTypeFlags, ParamTypeFlags) {
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
