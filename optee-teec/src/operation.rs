use crate::{ParamTypes, Parameter};
use optee_teec_sys as raw;
use std::ptr;

pub struct Operation {
    pub raw: raw::TEEC_Operation,
}

impl Operation {
    pub fn new(params: [Parameter; 4]) -> Self {
        let raw_op = raw::TEEC_Operation {
            started: 0,
            paramTypes: ParamTypes::new(
                params[0].param_type,
                params[1].param_type,
                params[2].param_type,
                params[3].param_type,
            )
            .into(),
            params: [
                params[0].into(),
                params[1].into(),
                params[2].into(),
                params[3].into(),
            ],
            session: ptr::null_mut() as *mut raw::TEEC_Session,
        };
        Operation { raw: raw_op }
    }

    pub fn as_mut_ptr(&mut self) -> *mut raw::TEEC_Operation {
        &mut self.raw
    }
}
