#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use optee_utee;
use optee_utee::{trace_println, Error, Result};
use optee_utee_sys::*;
use std::mem;
use std::ptr;

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    trace_println!("[+] TA_CreateEntryPoint: Secure storage.");
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(
    _param_types: uint32_t,
    _params: &mut [TEE_Param; 4],
    _sess_ctx: *mut *mut c_void,
) -> TEE_Result {
    trace_println!("[+] TA_OpenSessionEntryPoint: Secure, storage!");
    TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) {
    trace_println!("[+] TA_CloseSessionEntryPoint: Goodbye.");
}

pub fn delete_object(param_types: uint32_t, params: &mut [TEE_Param; 4]) -> TEE_Result {
    let exp_param_types: uint32_t = TEE_PARAM_TYPES(
        TEE_PARAM_TYPE_MEMREF_INPUT,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
    );
    if param_types != exp_param_types {
        return TEE_ERROR_BAD_PARAMETERS;
    }

    unsafe {
        //original code: id_size: size_t, obj_id: *mut c_char;
        let obj_id_sz: uint32_t = params[0].memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        TEE_MemMove(obj_id, params[0].memref.buffer, obj_id_sz);

        let mut object: TEE_ObjectHandle = ptr::null_mut();
        let res = TEE_OpenPersistentObject(
            TEE_STORAGE_PRIVATE,
            obj_id,
            obj_id_sz,
            TEE_DATA_FLAG_ACCESS_READ | TEE_DATA_FLAG_ACCESS_WRITE_META,
            &mut object as *mut _,
        );
        if res != TEE_SUCCESS {
            //EMSG("Failed to open persistent object, res=0x%08x", res);
            TEE_Free(obj_id);
            return res;
        }
        TEE_CloseAndDeletePersistentObject1(object);
        TEE_Free(obj_id);
        return res;
    }
}

pub fn create_raw_object(param_types: uint32_t, params: &mut [TEE_Param; 4]) -> TEE_Result {
    let exp_param_types: uint32_t = TEE_PARAM_TYPES(
        TEE_PARAM_TYPE_MEMREF_INPUT,
        TEE_PARAM_TYPE_MEMREF_INPUT,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
    );
    if param_types != exp_param_types {
        return TEE_ERROR_BAD_PARAMETERS;
    }

    unsafe {
        let obj_id_sz: uint32_t = params[0].memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        if obj_id.is_null() {
            return TEE_ERROR_OUT_OF_MEMORY;
        }
        TEE_MemMove(obj_id, params[0].memref.buffer, obj_id_sz);

        let data: *mut c_void = params[1].memref.buffer as *mut c_void;
        let data_sz: uint32_t = params[1].memref.size;
        let obj_data_flag: uint32_t = TEE_DATA_FLAG_ACCESS_READ
            | TEE_DATA_FLAG_ACCESS_WRITE
            | TEE_DATA_FLAG_ACCESS_WRITE_META
            | TEE_DATA_FLAG_OVERWRITE;
        let mut object: TEE_ObjectHandle = ptr::null_mut();
        let mut res: TEE_Result = TEE_CreatePersistentObject(
            TEE_STORAGE_PRIVATE,
            obj_id,
            obj_id_sz,
            obj_data_flag,
            TEE_HANDLE_NULL as TEE_ObjectHandle,
            ptr::null(),
            0,
            &mut object as *mut _,
        );
        if res != TEE_SUCCESS {
            //EMSG("TEE_CreatePersistentObject failed 0x%08x", res);
            TEE_Free(obj_id);
            return res;
        }

        res = TEE_WriteObjectData(object, data as *const c_void, data_sz);
        if res != TEE_SUCCESS {
            //EMSG("TEE_WriteObjectData failed 0x%08x", res);
            TEE_CloseAndDeletePersistentObject1(object);
        } else {
            TEE_CloseObject(object);
        }
        TEE_Free(obj_id);
        return res;
    }
}

pub fn read_raw_object(param_types: uint32_t, params: &mut [TEE_Param; 4]) -> Result<()> {
    let exp_param_types: uint32_t = TEE_PARAM_TYPES(
        TEE_PARAM_TYPE_MEMREF_INPUT,
        TEE_PARAM_TYPE_MEMREF_OUTPUT,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
    );
    if param_types != exp_param_types {
       return Err(Error::from_raw_error(TEE_ERROR_BAD_PARAMETERS));
    }

    unsafe {
        let obj_id_sz: uint32_t = params[0].memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        if obj_id.is_null() {
            return Err(Error::from_raw_error(TEE_ERROR_OUT_OF_MEMORY));
        }
        TEE_MemMove(obj_id, params[0].memref.buffer, obj_id_sz);

        let data: *mut c_void = params[1].memref.buffer as *mut c_void;
        let data_sz: uint32_t = params[1].memref.size;
        let mut object: TEE_ObjectHandle = ptr::null_mut();
        let mut res = TEE_OpenPersistentObject(
            TEE_STORAGE_PRIVATE,
            obj_id,
            obj_id_sz,
            TEE_DATA_FLAG_ACCESS_READ | TEE_DATA_FLAG_SHARE_READ,
            &mut object as *mut _,
        );
        if res != TEE_SUCCESS {
            TEE_Free(obj_id);
            //return res;
            return Err(Error::from_raw_error(res));
        }

        let mut object_info: TEE_ObjectInfo = TEE_ObjectInfo {
            objectType: 0,
            objectSize: 0,
            maxObjectSize: 0,
            objectUsage: 0,
            dataSize: 0,
            dataPosition: 0,
            handleFlags: 0,
        };

        res = TEE_GetObjectInfo1(object, &mut object_info as *mut _);
        let mut read_bytes: uint32_t = 0; //original type: size_t

        'correct_handle: loop {
            if res != TEE_SUCCESS {
                break 'correct_handle;
            }

            if object_info.dataSize > data_sz {
                params[1].memref.size = object_info.dataSize;
                res = TEE_ERROR_SHORT_BUFFER;
                break 'correct_handle;
            }

            res = TEE_ReadObjectData(
                object,
                data,
                object_info.dataSize,
                &mut read_bytes as *mut _,
            );
            if res != TEE_SUCCESS || read_bytes != object_info.dataSize {
                if res == TEE_SUCCESS {
                    res = TEE_ERROR_EXCESS_DATA;
                }
                break 'correct_handle;
            }
            params[1].memref.size = read_bytes;
            return Ok(());
        }
        TEE_CloseObject(object);
        TEE_Free(obj_id);
        return Err(Error::from_raw_error(res));
    }
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    param_types: uint32_t,
    params: &mut [TEE_Param; 4],
) -> TEE_Result {
    trace_println!("[+] TA_InvokeCommandEntryPoint: Invoke.");
    match cmd_id {
        TA_SECURE_STORAGE_CMD_WRITE_RAW=> {
            return create_raw_object(param_types, params); 
        }
        TA_SECURE_STORAGE_CMD_READ_RAW => match read_raw_object(param_types, params) {
            Ok(_) => {
                trace_println!("Create object success!");
                return TEE_SUCCESS;
            }
            Err(e) => {
                trace_println!("{:?}", e);
                return e.raw_code();
            }
        }
        TA_SECURE_STORAGE_CMD_DELETE => {
            return delete_object(param_types, params);
        }
        _ => {
            return TEE_ERROR_NOT_SUPPORTED;
        }
    }
}

const TA_FLAGS: uint32_t = TA_FLAG_EXEC_DDR;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 32 * 1024;
const EXT_PROP_VALUE_1: &[u8] = b"Secure Storage TA\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
