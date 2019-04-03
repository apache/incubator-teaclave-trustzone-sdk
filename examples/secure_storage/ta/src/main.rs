#![no_main]

use libc::{c_void, uint32_t};
use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use optee_utee_sys::*;
use std::ptr;

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Write => {
            return create_raw_object(params);
        }
        Command::Read => {
            return read_raw_object(params);
        }
        Command::Delete => {
            return delete_object(params);
        }
        _ => {
            return Err(Error::new(ErrorKind::NotSupported));
        }
    }
}

pub fn delete_object(params: &mut Parameters) -> Result<()> {
    unsafe {
        let obj_id_sz: u32 = (*params.first().raw).memref.size;
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
        let obj_id_sz: u32 = (*params.first().raw).memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        if obj_id.is_null() {
            return Err(Error::new(ErrorKind::OutOfMemory));
        }
        TEE_MemMove(obj_id, (*params.first().raw).memref.buffer, obj_id_sz);

        let data: *mut c_void = (*params.second().raw).memref.buffer as *mut c_void;
        let data_sz: u32 = (*params.second().raw).memref.size;
        let obj_data_flag: u32 = TEE_DATA_FLAG_ACCESS_READ
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
        let obj_id_sz: u32 = (*params.first().raw).memref.size;
        let obj_id: *mut c_void = TEE_Malloc(obj_id_sz, 0);
        if obj_id.is_null() {
            return Err(Error::new(ErrorKind::OutOfMemory));
        }
        TEE_MemMove(obj_id, (*params.first().raw).memref.buffer, obj_id_sz);

        let data: *mut c_void = (*params.second().raw).memref.buffer as *mut c_void;
        let data_sz: u32 = (*params.second().raw).memref.size;
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
        let mut read_bytes: u32 = 0;

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

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"This is a secure storage example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Secure Storage TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
