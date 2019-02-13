#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use optee_teec_sys::*;
use std::ptr;

pub const TA_HELLO_WORLD_CMD_INC_VALUE: u32 = 0;
pub const TA_HELLO_WORLD_CMD_DEC_VALUE: u32 = 1;

pub fn main() {
    let mut res: TEEC_Result;
    let mut ctx: TEEC_Context = TEEC_Context {
        fd: 0,
        reg_mem: true,
    };
    let mut sess: TEEC_Session = TEEC_Session {
        ctx: &mut ctx,
        session_id: 0,
    };

    let param1: TEEC_Parameter = TEEC_Parameter {
        value: TEEC_Value { a: 0, b: 0 },
    };
    let param2: TEEC_Parameter = TEEC_Parameter {
        value: TEEC_Value { a: 0, b: 0 },
    };
    let param3: TEEC_Parameter = TEEC_Parameter {
        value: TEEC_Value { a: 0, b: 0 },
    };
    let param4: TEEC_Parameter = TEEC_Parameter {
        value: TEEC_Value { a: 0, b: 0 },
    };
    let param_g: [TEEC_Parameter; 4] = [param1, param2, param3, param4];

    let mut op = TEEC_Operation {
        started: 0,
        param_types: 0,
        params: param_g,
        session: &mut sess,
    };
    let mut err_origin: uint32_t = 0;
    let mut uuid = TEEC_UUID {
        time_low: 0x8abcf200,
        time_mid: 0x2450,
        time_hi_and_version: 0x11e4,
        clock_seq_and_node: [0xab, 0xe2, 0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b],
    };

    unsafe {
        res = TEEC_InitializeContext(ptr::null_mut() as *mut c_char, &mut ctx);
        if res != TEEC_SUCCESS {
            println!("Init error.");
            return;
        }

        res = TEEC_OpenSession(
            &mut ctx,
            &mut sess,
            &mut uuid,
            TEEC_LOGIN_PUBLIC,
            ptr::null() as *const c_void,
            ptr::null_mut() as *mut TEEC_Operation,
            &mut err_origin,
        );
        if res != TEEC_SUCCESS {
            println!("Open session error.");
            return;
        }

        op.param_types = TEEC_PARAM_TYPES(TEEC_VALUE_INOUT, TEEC_NONE, TEEC_NONE, TEEC_NONE);
        op.params[0].value.a = 29;
        println!("original value is {}", op.params[0].value.a);
        res = TEEC_InvokeCommand(
            &mut sess,
            TA_HELLO_WORLD_CMD_INC_VALUE,
            &mut op,
            &mut err_origin,
        );
        if res != TEEC_SUCCESS {
            println!("Execute command error.");
            return;
        }
        println!("update value is {}", op.params[0].value.a);

        TEEC_CloseSession(&mut sess);
        TEEC_FinalizeContext(&mut ctx);
    }
}
