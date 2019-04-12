#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{DataFlag, ObjectHandle, ObjectInfo, ObjectStorageConstants, PersistentObject};
use optee_utee::{Error, ErrorKind, Parameters, Result};

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
    let obj_id_sz: u32 = unsafe { (*params.first().raw).memref.size };

    //TEE_Malloc
    let mut obj_id = vec![0; obj_id_sz as usize];
    //TEE_MemMove
    let id_slice: &[u8] = unsafe {
        std::slice::from_raw_parts(
            (*params.first().raw).memref.buffer as *mut u8,
            obj_id_sz as usize,
        )
    };
    obj_id.clone_from_slice(id_slice);

    match PersistentObject::open(
        ObjectStorageConstants::Private,
        &mut obj_id,
        DataFlag::ACCESS_READ | DataFlag::ACCESS_WRITE_META,
    ) {
        Err(e) => {
            return Err(e);
        }

        Ok(mut object) => {
            object.close_and_delete()?;
            std::mem::forget(object);
            return Ok(());
        }
    }
}

pub fn create_raw_object(params: &mut Parameters) -> Result<()> {
    let obj_id_sz: u32 = unsafe { (*params.first().raw).memref.size };
    let mut obj_id = vec![0; obj_id_sz as usize];
    let id_slice: &[u8] = unsafe {
        std::slice::from_raw_parts(
            (*params.first().raw).memref.buffer as *mut u8,
            obj_id_sz as usize,
        )
    };
    obj_id.clone_from_slice(id_slice);

    let data: &[u8] = unsafe {
        std::slice::from_raw_parts(
            (*params.second().raw).memref.buffer as *mut u8,
            (*params.second().raw).memref.size as usize,
        )
    };
    let obj_data_flag = DataFlag::ACCESS_READ
        | DataFlag::ACCESS_WRITE
        | DataFlag::ACCESS_WRITE_META
        | DataFlag::OVERWRITE;

    let mut init_data: [u8; 0] = [0; 0];
    match PersistentObject::create(
        ObjectStorageConstants::Private,
        &mut obj_id,
        obj_data_flag,
        ObjectHandle::new_empty(),
        &mut init_data,
    ) {
        Err(e) => {
            return Err(e);
        }

        Ok(mut object) => match object.write(data) {
            Ok(()) => {
                return Ok(());
            }
            Err(e_write) => {
                object.close_and_delete()?;
                std::mem::forget(object);
                return Err(e_write);
            }
        },
    }
}

pub fn read_raw_object(params: &mut Parameters) -> Result<()> {
    let obj_id_sz: u32 = unsafe { (*params.first().raw).memref.size };
    let mut obj_id = vec![0; obj_id_sz as usize];
    let id_slice: &[u8] = unsafe {
        std::slice::from_raw_parts(
            (*params.first().raw).memref.buffer as *mut u8,
            obj_id_sz as usize,
        )
    };
    obj_id.clone_from_slice(id_slice);

    let data: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(
            (*params.second().raw).memref.buffer as *mut u8,
            (*params.second().raw).memref.size as usize,
        )
    };
    match PersistentObject::open(
        ObjectStorageConstants::Private,
        &mut obj_id,
        DataFlag::ACCESS_READ | DataFlag::SHARE_READ,
    ) {
        Err(e) => return Err(e),

        Ok(object) => {
            let mut obj_info = ObjectInfo::new();
            object.get_info(&mut obj_info)?;

            if obj_info.raw.dataSize > data.len() as u32 {
                unsafe {
                    (*params.second().raw).memref.size = obj_info.raw.dataSize;
                }
                return Err(Error::new(ErrorKind::ShortBuffer));
            }

            let read_bytes = object.read(data).unwrap();

            if read_bytes != obj_info.raw.dataSize {
                return Err(Error::new(ErrorKind::ExcessData));
            }

            unsafe {
                (*params.second().raw).memref.size = read_bytes;
            }
            Ok(())
        }
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
