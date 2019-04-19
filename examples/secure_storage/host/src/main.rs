use optee_teec::{Context, ErrorKind, Operation, ParamNone, ParamTmpRef, ParamType, Session, Uuid};
use std::ffi::CString;
use proto::{UUID, Command};

const TEST_OBJECT_SIZE: usize = 7000;

fn read_secure_object(
    session: &mut Session,
    obj_id: &mut [u8],
    obj_data: &mut [u8],
) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(obj_id, ParamType::MemrefTempInput);
    let p1 = ParamTmpRef::new(obj_data, ParamType::MemrefTempOutput);
    let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::Read as u32, &mut operation)?;

    println!("- Read back the object");
    Ok(())
}

fn write_secure_object(
    session: &mut Session,
    obj_id: &mut [u8],
    obj_data: &mut [u8],
) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(obj_id, ParamType::MemrefTempInput);
    let p1 = ParamTmpRef::new(obj_data, ParamType::MemrefTempInput);
    let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::Write as u32, &mut operation)?;

    println!("- Create and load object in the TA secure storage");
    Ok(())
}

fn delete_secure_object(session: &mut Session, obj_id: &mut [u8]) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(obj_id, ParamType::MemrefTempInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::Delete as u32, &mut operation)?;

    println!("- Delete the object");
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let mut obj1_id = CString::new("object#1").unwrap().into_bytes_with_nul();
    let mut obj1_data = [0xA1u8; TEST_OBJECT_SIZE];
    let mut read_data = [0x00u8; TEST_OBJECT_SIZE];

    println!("\nTest on object \"object#1\"");
    write_secure_object(&mut session, obj1_id.as_mut_slice(), &mut obj1_data)?;
    read_secure_object(&mut session, obj1_id.as_mut_slice(), &mut read_data)?;

    if obj1_data.iter().zip(read_data.iter()).all(|(a, b)| a == b) {
        println!("- Content read-out correctly");
    } else {
        println!("- Unexpected content found in secure storage");
    }
    delete_secure_object(&mut session, &mut obj1_id)?;

    let mut obj2_id = CString::new("object#2").unwrap().into_bytes_with_nul();

    println!("\nTest on object \"object#2\"");
    match read_secure_object(&mut session, obj2_id.as_mut_slice(), &mut read_data) {
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
