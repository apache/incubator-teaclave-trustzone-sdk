use crate::{Param, ParamTypes};
use optee_teec_sys as raw;
use std::boxed::Box;
use std::marker::PhantomData;
use std::mem;

/// This type defines the payload of either an open session operation or an
/// invoke command operation. It is also used for cancellation of operations,
/// which may be desirable even if no payload is passed.
pub struct Operation<A, B, C, D> {
    pub raw: *mut raw::TEEC_Operation,
    phantom0: PhantomData<A>,
    phantom1: PhantomData<B>,
    phantom2: PhantomData<C>,
    phantom3: PhantomData<D>,
}

impl<A: Param, B: Param, C: Param, D: Param> Operation<A, B, C, D> {
    pub fn new(started: u32, mut p0: A, mut p1: B, mut p2: C, mut p3: D) -> Operation<A, B, C, D> {
        let mut raw_op: raw::TEEC_Operation = unsafe { mem::zeroed() };
        raw_op.started = started;
        raw_op.paramTypes = ParamTypes::new(
            p0.param_type(),
            p1.param_type(),
            p2.param_type(),
            p3.param_type(),
        )
        .into();
        raw_op.params = [p0.into_raw(), p1.into_raw(), p2.into_raw(), p3.into_raw()];
        Operation {
            raw: Box::into_raw(Box::new(raw_op)),
            phantom0: PhantomData,
            phantom1: PhantomData,
            phantom2: PhantomData,
            phantom3: PhantomData,
        }
    }

    pub fn as_mut_raw_ptr(&mut self) -> *mut raw::TEEC_Operation {
        self.raw
    }

    pub fn parameters(&self) -> (A, B, C, D) {
        unsafe {
            let (f0, f1, f2, f3) = ParamTypes::from((*self.raw).paramTypes).into_flags();
            (
                A::from_raw((*self.raw).params[0], f0),
                B::from_raw((*self.raw).params[1], f1),
                C::from_raw((*self.raw).params[2], f2),
                D::from_raw((*self.raw).params[3], f3),
            )
        }
    }
}

impl<A, B, C, D> Drop for Operation<A, B, C, D> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.raw);
        }
    }
}
