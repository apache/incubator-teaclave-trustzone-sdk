#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use optee_utee;
use optee_utee::{trace_println, Error, ErrorKind, Parameters, Result};
use optee_utee_sys::*;
use std::{mem, ptr, str};

fn MESA_CreateEntryPoint() -> Result<()> {
    Ok(())
}

fn MESA_OpenSessionEntryPoint(_params: &mut Parameters, _sess_ctx: *mut *mut c_void) -> Result<()> {
    Ok(())
}

fn MESA_CloseSessionEntryPoint(_sess_ctx: *mut *mut c_void) -> Result<()> {
    Ok(())
}

fn MESA_DestroyEntryPoint() -> Result<()> {
    Ok(())
}

fn MESA_InvokeCommandEntryPoint(
    _sess_ctx: *mut c_void,
    cmd_id: u32,
    params: &mut Parameters,
) -> Result<()> {
    match cmd_id {
        TA_SECURE_STORAGE_CMD_WRITE_RAW => {
            return create_raw_object(params);
        }
        TA_SECURE_STORAGE_CMD_READ_RAW => {
            return read_raw_object(params);
        }
        TA_SECURE_STORAGE_CMD_DELETE => {
            return delete_object(params);
        }
        _ => {
            return Err(Error::from_raw_error(TEE_ERROR_NOT_SUPPORTED));
        }
    }
}

pub fn delete_object(params: &mut Parameters) -> Result<()> {
    unsafe {
        let obj_id_sz: uint32_t = (*params.first().raw).memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        TEE_MemMove(obj_id, (*params.first().raw).memref.buffer, obj_id_sz);

        let mut object: TEE_ObjectHandle = TEE_HANDLE_NULL as *mut _;
        let res = TEE_OpenPersistentObject(
            TEE_STORAGE_PRIVATE,
            obj_id,
            obj_id_sz,
            TEE_DATA_FLAG_ACCESS_READ | TEE_DATA_FLAG_ACCESS_WRITE_META,
            &mut object as *mut _,
        );
        if res != TEE_SUCCESS {
            TEE_Free(obj_id);
            return Err(Error::from_raw_error(res));
        }
        TEE_CloseAndDeletePersistentObject1(object);
        TEE_Free(obj_id);
        Ok(())
    }
}

pub fn create_raw_object(params: &mut Parameters) -> Result<()> {
    unsafe {
        let obj_id_sz: uint32_t = (*params.first().raw).memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        if obj_id.is_null() {
            return Err(Error::from_raw_error(TEE_ERROR_OUT_OF_MEMORY));
        }
        TEE_MemMove(obj_id, (*params.first().raw).memref.buffer, obj_id_sz);

        let data: *mut c_void = (*params.second().raw).memref.buffer as *mut c_void;
        let data_sz: uint32_t = (*params.second().raw).memref.size;
        let obj_data_flag: uint32_t = TEE_DATA_FLAG_ACCESS_READ
            | TEE_DATA_FLAG_ACCESS_WRITE
            | TEE_DATA_FLAG_ACCESS_WRITE_META
            | TEE_DATA_FLAG_OVERWRITE;
        let mut object: TEE_ObjectHandle = TEE_HANDLE_NULL as *mut _;
        let mut res = TEE_CreatePersistentObject(
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
            TEE_Free(obj_id);
            return Err(Error::from_raw_error(res));
        }

        res = TEE_WriteObjectData(object, data as *const c_void, data_sz);
        if res != TEE_SUCCESS {
            TEE_CloseAndDeletePersistentObject1(object);
            return Err(Error::from_raw_error(res));
        } else {
            TEE_CloseObject(object);
        }
        TEE_Free(obj_id);
        Ok(())
    }
}

pub fn read_raw_object(params: &mut Parameters) -> Result<()> {
    unsafe {
        let obj_id_sz: uint32_t = (*params.first().raw).memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        if obj_id.is_null() {
            return Err(Error::from_raw_error(TEE_ERROR_OUT_OF_MEMORY));
        }
        TEE_MemMove(obj_id, (*params.first().raw).memref.buffer, obj_id_sz);

        let data: *mut c_void = (*params.second().raw).memref.buffer as *mut c_void;
        let data_sz: uint32_t = (*params.second().raw).memref.size;
        let mut object: TEE_ObjectHandle = TEE_HANDLE_NULL as *mut _;
        let res = TEE_OpenPersistentObject(
            TEE_STORAGE_PRIVATE,
            obj_id,
            obj_id_sz,
            TEE_DATA_FLAG_ACCESS_READ | TEE_DATA_FLAG_SHARE_READ,
            &mut object as *mut _,
        );
        if res != TEE_SUCCESS {
            TEE_Free(obj_id);
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
        let res = TEE_GetObjectInfo1(object, &mut object_info as *mut _);
        if res != TEE_SUCCESS {
            TEE_CloseObject(object);
            TEE_Free(obj_id);
            return Err(Error::from_raw_error(res));
        }
        let mut read_bytes: uint32_t = 0;

        if object_info.dataSize > data_sz {
            (*params.second().raw).memref.size = object_info.dataSize;
            TEE_CloseObject(object);
            TEE_Free(obj_id);
            return Err(Error::new(ErrorKind::ShortBuffer));
        }

        let res = TEE_ReadObjectData(
            object,
            data,
            object_info.dataSize,
            &mut read_bytes as *mut _,
        );
        if res != TEE_SUCCESS {
            TEE_CloseObject(object);
            TEE_Free(obj_id);
            return Err(Error::from_raw_error(res));
        } else if read_bytes != object_info.dataSize {
            TEE_CloseObject(object);
            TEE_Free(obj_id);
            return Err(Error::new(ErrorKind::ExcessData));
        }
        (*params.second().raw).memref.size = read_bytes;
        TEE_CloseObject(object);
        TEE_Free(obj_id);
        return Ok(());
    }
}

const ta_name: &str = "Secure Storage";

const TA_FLAGS: uint32_t = TA_FLAG_EXEC_DDR;
const TA_STACK_SIZE: uint32_t = 2 * 1024;
const TA_DATA_SIZE: uint32_t = 32 * 1024;
const EXT_PROP_VALUE_1: &[u8] = b"Secure Storage TA\0";
const EXT_PROP_VALUE_2: uint32_t = 0x0010;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
