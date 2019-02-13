#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate optee_teec;
pub use libc::*;
pub use optee_teec::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

pub const TA_SECURE_STORAGE_CMD_READ_RAW: u32 = 0;
pub const TA_SECURE_STORAGE_CMD_WRITE_RAW: u32 = 1;
pub const TA_SECURE_STORAGE_CMD_DELETE: u32 = 2;
pub const TEST_OBJECT_SIZE: usize = 7000;

pub fn read_secure_object(
    sess_ptr: *mut TEEC_Session,
    id: *mut c_char,
    data: *mut c_char,
    data_len: uint32_t,
) -> TEEC_Result {
    let mut res: TEEC_Result;
    let mut err_origin: uint32_t = 0;
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
        session: sess_ptr,
    };

    unsafe {
        op.param_types = TEEC_PARAM_TYPES(
            TEEC_MEMREF_TEMP_INPUT,
            TEEC_MEMREF_TEMP_OUTPUT,
            TEEC_NONE,
            TEEC_NONE,
        );
        op.params[0].tmpref.buffer = id as *mut c_void;
        op.params[0].tmpref.size = strlen(id) as size_t;
        op.params[1].tmpref.buffer = data as *mut c_void;
        op.params[1].tmpref.size = data_len as size_t;

        res = TEEC_InvokeCommand(
            sess_ptr,
            TA_SECURE_STORAGE_CMD_READ_RAW,
            &mut op,
            &mut err_origin,
        );
        return res;
    }
}

pub fn write_secure_object(
    sess_ptr: *mut TEEC_Session,
    id: *mut c_char,
    data: *mut c_char,
    data_len: uint32_t,
) -> TEEC_Result {
    let mut res: TEEC_Result;
    let mut err_origin: uint32_t = 0;
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
        session: sess_ptr,
    };

    unsafe {
        op.param_types = TEEC_PARAM_TYPES(
            TEEC_MEMREF_TEMP_INPUT,
            TEEC_MEMREF_TEMP_INPUT,
            TEEC_NONE,
            TEEC_NONE,
        );
        op.params[0].tmpref.buffer = id as *mut c_void;
        op.params[0].tmpref.size = strlen(id) as size_t;
        op.params[1].tmpref.buffer = data as *mut c_void;
        op.params[1].tmpref.size = data_len as size_t;

        res = TEEC_InvokeCommand(
            sess_ptr,
            TA_SECURE_STORAGE_CMD_WRITE_RAW,
            &mut op,
            &mut err_origin,
        );
        return res;
    }
}

pub fn delete_secure_object(sess_ptr: *mut TEEC_Session, id: *mut c_char) -> TEEC_Result {
    let mut res: TEEC_Result;
    let mut err_origin: uint32_t = 0;
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
        session: sess_ptr,
    };

    unsafe {
        op.param_types = TEEC_PARAM_TYPES(TEEC_MEMREF_TEMP_INPUT, TEEC_NONE, TEEC_NONE, TEEC_NONE);
        op.params[0].tmpref.buffer = id as *mut c_void;
        op.params[0].tmpref.size = strlen(id) as size_t;

        res = TEEC_InvokeCommand(
            sess_ptr,
            TA_SECURE_STORAGE_CMD_DELETE,
            &mut op,
            &mut err_origin,
        );
        return res;
    }
}

pub fn check_equal(array_1: [c_char; TEST_OBJECT_SIZE], array_2: [c_char; TEST_OBJECT_SIZE]) {
    for var in 0..TEST_OBJECT_SIZE {
        if array_1[var] != array_2[var] {
            println!(
                "Two arrays not equal, first: {}, second: {}",
                array_1[var], array_2[var]
            );
            return;
        }
    }
    println!("Arrays equal now!\0");
    return;
}

pub fn main() {
    let mut ctx: TEEC_Context = TEEC_Context {
        fd: 0,
        reg_mem: true,
    };
    let mut sess: TEEC_Session = TEEC_Session {
        ctx: &mut ctx,
        session_id: 0,
    };
    let mut err_origin: uint32_t = 0;
    let mut uuid = TEEC_UUID {
        time_low: 0xf4e750bb,
        time_mid: 0x1437,
        time_hi_and_version: 0x4fbf,
        clock_seq_and_node: [0x87, 0x85, 0x8d, 0x35, 0x80, 0xc3, 0x49, 0x94],
    };

    unsafe {
        let obj1_local = CString::new("object#1").expect("CString::new failed");
        let obj1_id: *mut c_char = obj1_local.as_ptr() as *mut c_char;
        let mut obj1_data: [c_char; TEST_OBJECT_SIZE] = [0xA1 as c_char; TEST_OBJECT_SIZE];
        let mut read_data: [c_char; TEST_OBJECT_SIZE] = ['\0' as c_char; TEST_OBJECT_SIZE];
        let mut res: TEEC_Result;

        res = TEEC_InitializeContext(ptr::null_mut() as *mut c_char, &mut ctx);
        if res != TEEC_SUCCESS {
            println!("Init error.\0");
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
            println!("Open session error.\0");
            return;
        }

        res = write_secure_object(
            &mut sess,
            obj1_id,
            &mut obj1_data[0],
            mem::size_of::<[c_char; TEST_OBJECT_SIZE]>() as uint32_t,
        );
        if res != TEEC_SUCCESS {
            println!("Write command error.\0");
            return;
        }
        println!("Create and load object in the TA secure storage success.\0");
        check_equal(obj1_data, read_data);

        res = read_secure_object(
            &mut sess,
            obj1_id,
            &mut read_data[0],
            mem::size_of::<[c_char; TEST_OBJECT_SIZE]>() as uint32_t,
        );
        if res != TEEC_SUCCESS {
            println!("Read command error.\0");
            return;
        }
        println!("Read back object success. \0");
        check_equal(obj1_data, read_data);

        res = delete_secure_object(&mut sess, obj1_id);
        if res != TEEC_SUCCESS {
            println!("Delete command error.\0");
            return;
        }
        println!("Delete object sucess.\0");

        TEEC_CloseSession(&mut sess);
        TEEC_FinalizeContext(&mut ctx);
    }
}
