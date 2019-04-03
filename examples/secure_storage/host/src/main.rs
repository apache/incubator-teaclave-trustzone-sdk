use libc::c_char;
use optee_teec::{Context, ErrorKind, Operation, ParamType, Parameter, Session, Uuid};
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));
const TEST_OBJECT_SIZE: usize = 7000;

fn read_secure_object(
    session: &mut Session,
    obj_id: &mut CString,
    obj_data: &mut [c_char],
) -> optee_teec::Result<()> {
    let p0 = Parameter::from_tmpref(
        obj_id.as_ptr() as *mut c_char,
        obj_id.as_bytes_with_nul().len(),
        ParamType::MemrefTempInput,
    );
    let p1 = Parameter::from_tmpref(
        obj_data.as_mut_ptr(),
        TEST_OBJECT_SIZE,
        ParamType::MemrefTempOutput,
    );
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    session.invoke_command(Command::Read as u32, &mut operation)?;

    println!("- Read back the object");
    Ok(())
}

fn write_secure_object(
    session: &mut Session,
    obj_id: &mut CString,
    obj_data: &mut [c_char],
) -> optee_teec::Result<()> {
    let p0 = Parameter::from_tmpref(
        obj_id.as_ptr() as *mut c_char,
        obj_id.as_bytes_with_nul().len(),
        ParamType::MemrefTempInput,
    );
    let p1 = Parameter::from_tmpref(
        obj_data.as_mut_ptr(),
        TEST_OBJECT_SIZE,
        ParamType::MemrefTempInput,
    );
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    session.invoke_command(Command::Write as u32, &mut operation)?;

    println!("- Create and load object in the TA secure storage");
    Ok(())
}

fn delete_secure_object(session: &mut Session, obj_id: &mut CString) -> optee_teec::Result<()> {
    let p0 = Parameter::from_tmpref(
        obj_id.as_ptr() as *mut c_char,
        obj_id.as_bytes_with_nul().len(),
        ParamType::MemrefTempInput,
    );
    let p1 = Parameter::new();
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    session.invoke_command(Command::Delete as u32, &mut operation)?;

    println!("- Delete the object");
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())
            .unwrap();
    let mut session = ctx.open_session(uuid)?;

    let mut obj1_id = CString::new("object#1").unwrap();
    let mut obj1_data = [0xA1u8; TEST_OBJECT_SIZE];
    let mut read_data = [0x00u8; TEST_OBJECT_SIZE];

    println!("\nTest on object \"object#1\"");
    write_secure_object(&mut session, &mut obj1_id, &mut obj1_data)?;
    read_secure_object(&mut session, &mut obj1_id, &mut read_data)?;

    if obj1_data.iter().zip(read_data.iter()).all(|(a, b)| a == b) {
        println!("- Content read-out correctly");
    } else {
        println!("- Unexpected content found in secure storage");
    }
    delete_secure_object(&mut session, &mut obj1_id)?;

    let mut obj2_id = CString::new("object#2").unwrap();

    println!("\nTest on object \"object#2\"");
    match read_secure_object(&mut session, &mut obj2_id, &mut read_data) {
        Err(e) => {
            if e.kind() != ErrorKind::ItemNotFound {
                println!("{}", e);
                return Err(e);
            } else {
                println!("- Object not found in TA secure storage, create it");
                let mut obj2_data = [0xB1u8; TEST_OBJECT_SIZE];
                write_secure_object(&mut session, &mut obj2_id, &mut obj2_data)?;
            }
        }

        Ok(()) => {
            println!("- Object found in TA secure storage, delete it");
            delete_secure_object(&mut session, &mut obj2_id)?;
        }
    }

    println!("\nWe're done, close and release TEE resources");
    Ok(())
}
