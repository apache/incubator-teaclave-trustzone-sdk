use optee_teec_sys as raw;
use std::mem;

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

pub struct Parameter {
    raw: raw::TEEC_Parameter,
    pub param_type: ParamType,
}

impl Parameter {
    pub fn none() -> Self {
        let raw = unsafe { mem::zeroed() };
        Self {
            raw: raw,
            param_type: ParamType::None,
        }
    }

    pub fn from_value(a: u32, b: u32, param_type: ParamType) -> Self {
        let raw = raw::TEEC_Parameter {
            value: raw::TEEC_Value { a, b },
        };
        Self {
            raw: raw,
            param_type: param_type,
        }
    }

    pub fn from_tmpref<T>(buffer: *mut T, size: usize, param_type: ParamType) -> Self {
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

    pub fn from_raw(raw: raw::TEEC_Parameter, param_type: ParamType) -> Self {
        Self {
            raw: raw,
            param_type: param_type,
        }
    }

    pub fn value(&self) -> (u32, u32) {
        unsafe { (self.raw.value.a, self.raw.value.b) }
    }

    pub fn set_value(&mut self, a: u32, b: u32) {
        unsafe {
            self.raw.value.a = a;
            self.raw.value.b = b;
        }
    }
}

impl From<Parameter> for raw::TEEC_Parameter {
    fn from(a: Parameter) -> raw::TEEC_Parameter {
        a.raw
    }
}

#[derive(Copy, Clone)]
pub enum ParamType {
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

impl From<u32> for ParamType {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamType::None,
            1 => ParamType::ValueInput,
            2 => ParamType::ValueOutput,
            3 => ParamType::ValueInout,
            5 => ParamType::MemrefTempInput,
            6 => ParamType::MemrefTempOutput,
            7 => ParamType::MemrefTempInout,
            0xC => ParamType::MemrefWhole,
            0xD => ParamType::MemrefPartialInput,
            0xE => ParamType::MemrefPartialOutput,
            0xF => ParamType::MemrefPartialInout,
            _ => ParamType::None,
        }
    }
}

pub struct ParamTypes(u32);

impl ParamTypes {
    pub fn new(p0: ParamType, p1: ParamType, p2: ParamType, p3: ParamType) -> Self {
        ParamTypes((p0 as u32) | (p1 as u32) << 4 | (p2 as u32) << 8 | (p3 as u32) << 12)
    }

    pub fn into_flags(&self) -> (ParamType, ParamType, ParamType, ParamType) {
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
