use crate::{ParamTypes, Parameter};
use optee_teec_sys as raw;
use std::ptr;

pub struct Operation {
    pub raw: raw::TEEC_Operation,
}

impl Operation {
    pub fn new(started: u32, param0: Parameter, param1: Parameter, param2: Parameter, param3: Parameter) -> Self {
        let raw_op = raw::TEEC_Operation {
            started: started,
            paramTypes: ParamTypes::new(
                param0.param_type,
                param1.param_type,
                param2.param_type,
                param3.param_type,
            )
            .into(),
            params: [
                param0.into(),
                param1.into(),
                param2.into(),
                param3.into(),
            ],
            session: ptr::null_mut() as *mut raw::TEEC_Session,
        };
        Operation { raw: raw_op }
    }

    pub fn as_mut_ptr(&mut self) -> *mut raw::TEEC_Operation {
        &mut self.raw
    }
}
