use crate::{ParamTypes, Parameter};
use optee_teec_sys as raw;
use std::ptr;

/// This type defines the payload of either an open session operation or an
/// invoke command operation. It is also used for cancellation of operations,
/// which may be desirable even if no payload is passed.
pub struct Operation {
    pub raw: raw::TEEC_Operation,
}

impl Operation {
    pub fn new(started: u32, p0: Parameter, p1: Parameter, p2: Parameter, p3: Parameter) -> Self {
        let raw_op = raw::TEEC_Operation {
            started: started,
            paramTypes: ParamTypes::new(p0.param_type, p1.param_type, p2.param_type, p3.param_type)
                .into(),
            params: [p0.into(), p1.into(), p2.into(), p3.into()],
            session: ptr::null_mut() as *mut raw::TEEC_Session,
        };
        Operation { raw: raw_op }
    }

    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEEC_Operation {
        &mut self.raw
    }

    pub fn parameters(&self) -> (Parameter, Parameter, Parameter, Parameter) {
        let (f0, f1, f2, f3) = ParamTypes::from(self.raw.paramTypes).into_flags();
        let p0 = Parameter::from_raw(self.raw.params[0], f0);
        let p1 = Parameter::from_raw(self.raw.params[1], f1);
        let p2 = Parameter::from_raw(self.raw.params[2], f2);
        let p3 = Parameter::from_raw(self.raw.params[3], f3);
        (p0, p1, p2, p3)
    }
}
