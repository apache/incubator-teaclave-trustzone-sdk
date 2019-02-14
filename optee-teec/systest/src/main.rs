#![allow(bad_style)]

extern crate optee_teec_sys;
extern crate libc;

use libc::*;
use optee_teec_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
