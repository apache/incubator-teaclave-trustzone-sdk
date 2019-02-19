use libc::*;
use optee_teec::{Context, Operation, ParamTypeFlags, Parameter, Session, Uuid};
use std::ffi::CString;

const TA_SECURE_STORAGE_CMD_READ_RAW: u32 = 0;
const TA_SECURE_STORAGE_CMD_WRITE_RAW: u32 = 1;
const TA_SECURE_STORAGE_CMD_DELETE: u32 = 2;
const TEST_OBJECT_SIZE: usize = 7000;

fn write_secure_object(session: &mut Session, obj_id: &mut CString, obj_data: &mut [c_char]) {
    let param0 = Parameter::tmpref(
        obj_id.as_ptr() as *mut c_char,
        obj_id.as_bytes_with_nul().len(),
        ParamTypeFlags::MemrefTempInput,
    );
    let param1 = Parameter::tmpref(
        obj_data.as_mut_ptr(),
        TEST_OBJECT_SIZE,
        ParamTypeFlags::MemrefTempInput,
    );
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let params: [Parameter; 4] = [param0, param1, param2, param3];
    let mut operation = Operation::new(params);
    match session.invoke_command(TA_SECURE_STORAGE_CMD_WRITE_RAW, &mut operation) {
        Ok(_) => println!("Write object to secure storage success."),
        Err(e) => println!("{:?}", e),
    }
}

fn read_secure_object(session: &mut Session, obj_id: &mut CString, obj_data: &mut [c_char]) {
    let param0 = Parameter::tmpref(
        obj_id.as_ptr() as *mut c_char,
        obj_id.as_bytes_with_nul().len(),
        ParamTypeFlags::MemrefTempInput,
    );
    let param1 = Parameter::tmpref(
        obj_data.as_mut_ptr(),
        TEST_OBJECT_SIZE,
        ParamTypeFlags::MemrefTempOutput,
    );
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let params: [Parameter; 4] = [param0, param1, param2, param3];
    let mut operation = Operation::new(params);
    match session.invoke_command(TA_SECURE_STORAGE_CMD_READ_RAW, &mut operation) {
        Ok(_) => println!("Read back object success."),
        Err(e) => println!("{:?}", e),
    }
}

fn delete_secure_object(session: &mut Session, obj_id: &mut CString) {
    let param0 = Parameter::tmpref(
        obj_id.as_ptr() as *mut c_char,
        obj_id.as_bytes_with_nul().len(),
        ParamTypeFlags::MemrefTempInput,
    );
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let params: [Parameter; 4] = [param0, param1, param2, param3];
    let mut operation = Operation::new(params);
    match session.invoke_command(TA_SECURE_STORAGE_CMD_DELETE, &mut operation) {
        Ok(_) => println!("Delete object success."),
        Err(e) => println!("{:?}", e),
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut obj_id = CString::new("object#1")?;
    let mut obj_data = [0xA1u8; TEST_OBJECT_SIZE];
    let mut read_data = [0x00u8; TEST_OBJECT_SIZE];

    let uuid = Uuid::parse_str("f4e750bb-1437-4fbf-8785-8d3580c34994")?;
    let mut ctx = Context::new()?;
    let mut session = ctx.open_session(uuid)?;
    write_secure_object(&mut session, &mut obj_id, &mut obj_data);
    read_secure_object(&mut session, &mut obj_id, &mut read_data);

    if obj_data.iter().zip(read_data.iter()).all(|(a, b)| a == b) {
        println!("obj_data == read_data");
    }
    delete_secure_object(&mut session, &mut obj_id);
    Ok(())
}
