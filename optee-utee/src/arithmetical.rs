use crate::{Error, Result};
use optee_utee_sys as raw;
use std::cmp::max;

/// Test BigInt case first. After reviewed, expand BigInt to other two types.
pub type BigIntUnit = u32;

pub struct BigInt(Vec<BigIntUnit>);

impl BigInt {
    /// size represents BigInt bits
    pub fn size_in_u32(size: u32) -> u32 {
        return ((size + 31) / 32) + 2;
    }

    pub fn init(bits: u32) -> Self {
        let size: usize = Self::size_in_u32(bits) as usize;
        let mut tmp_vec: Vec<BigIntUnit> = Vec::with_capacity(size);
        unsafe { raw::TEE_BigIntInit(tmp_vec.as_mut_ptr(), size as u32) };
        Self(tmp_vec)
    }

    pub fn convert_from_octet_string(&mut self, buffer: &[u8], sign: i32) -> Result<()> {
        match unsafe {
            raw::TEE_BigIntConvertFromOctetString(
                self.0.as_mut_ptr(),
                buffer.as_ptr(),
                buffer.len() as u32,
                sign,
            )
        } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn convert_to_octet_string(&self, buffer: &mut [u8]) -> Result<usize> {
        let mut buffer_size: u32 = buffer.len() as u32;
        match unsafe {
            raw::TEE_BigIntConvertToOctetString(
                buffer.as_mut_ptr(),
                &mut buffer_size as _,
                self.0.as_ptr(),
            )
        } {
            raw::TEE_SUCCESS => Ok(buffer_size as usize),
            code => Err(Error::from_raw_error(code)),
        }
    }

    pub fn convert_from_s32(&mut self, short_val: i32) {
        unsafe { raw::TEE_BigIntConvertFromS32(self.0.as_mut_ptr(), short_val) };
    }

    pub fn convert_to_s32(&mut self, short_val: &mut i32) -> Result<()> {
        match unsafe { raw::TEE_BigIntConvertToS32(short_val as _, self.0.as_mut_ptr()) } {
            raw::TEE_SUCCESS => Ok(()),
            code => Err(Error::from_raw_error(code)),
        }
    }

    /// return negative number if self < target,
    /// 0 if self == target
    /// positive number if self > target
    pub fn compare_big_int(&self, target: &Self) -> i32 {
        unsafe { raw::TEE_BigIntCmp(self.0.as_ptr(), target.0.as_ptr()) }
    }

    pub fn compare_s32(&self, target: i32) -> i32 {
        unsafe { raw::TEE_BigIntCmpS32(self.0.as_ptr(), target) }
    }

    pub fn shift_right(&mut self, op: &Self, bits: usize) {
        unsafe { raw::TEE_BigIntShiftRight(self.0.as_mut_ptr(), op.0.as_ptr(), bits) };
    }

    pub fn get_bit(&self, bit_index: u32) -> bool {
        unsafe { raw::TEE_BigIntGetBit(self.0.as_ptr(), bit_index) }
    }

    pub fn get_bit_count(&self) -> u32 {
        unsafe { raw::TEE_BigIntGetBitCount(self.0.as_ptr()) }
    }

    pub fn add(op1: &Self, op2: &Self) -> Self {
        let bits = max(Self::get_bit_count(op1), Self::get_bit_count(op2)) + 1;
        let mut res = Self::init(bits);
        unsafe { raw::TEE_BigIntAdd(res.0.as_mut_ptr(), op1.0.as_ptr(), op2.0.as_ptr()) };
        res
    }

    pub fn sub(op1: &Self, op2: &Self) -> Self {
        let bits = max(Self::get_bit_count(op1), Self::get_bit_count(op2)) + 1;
        let mut res = Self::init(bits);
        unsafe { raw::TEE_BigIntSub(res.0.as_mut_ptr(), op1.0.as_ptr(), op2.0.as_ptr()) };
        res
    }

    pub fn neg(op: &Self) -> Self {
        let mut res = Self::init(Self::get_bit_count(op));
        unsafe { raw::TEE_BigIntNeg(res.0.as_mut_ptr(), op.0.as_ptr()) };
        res
    }

    pub fn multiply(op1: &Self, op2: &Self) -> Self {
        let bits = Self::get_bit_count(op1) + Self::get_bit_count(op2);
        let mut res = Self::init(bits);
        unsafe { raw::TEE_BigIntMul(res.0.as_mut_ptr(), op1.0.as_ptr(), op2.0.as_ptr()) };
        res
    }

    pub fn square(op: &Self) -> Self {
        let mut res = Self::init(2 * Self::get_bit_count(op));
        unsafe { raw::TEE_BigIntSquare(res.0.as_mut_ptr(), op.0.as_ptr()) };
        res
    }

    /// document defines wrong size for result quotient
    pub fn divide(op1: &Self, op2: &Self) -> (Self, Self) {
        let q_bits = match op1.compare_big_int(op2) {
            d if d >= 0 => max(1, Self::get_bit_count(op1) - Self::get_bit_count(op2)),
            _ => 0,
        };
        let r_bits = Self::get_bit_count(op2);
        let mut quotient = Self::init(q_bits);
        let mut remainder = Self::init(r_bits);

        unsafe {
            raw::TEE_BigIntDiv(
                quotient.0.as_mut_ptr(),
                remainder.0.as_mut_ptr(),
                op1.0.as_ptr(),
                op2.0.as_ptr(),
            )
        };
        (quotient, remainder)
    }

    pub fn module(op: &Self, n: &Self) -> Self {
        let mut res = Self::init(Self::get_bit_count(n));
        unsafe { raw::TEE_BigIntMod(res.0.as_mut_ptr(), op.0.as_ptr(), n.0.as_ptr()) };
        res
    }
    // test after develop other big int functions after BigIntMod
}

//pub type BigIntFMM = u32;
//pub type BigIntFMMContext = u32;
//OP-TEE does not implement function:
//TEE_BigIntSetBit
//TEE_BigIntAssign
//TEE_BigIntAbs
