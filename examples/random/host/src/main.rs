use libc::*;
use optee_teec::{Context, Operation, ParamTypeFlags, Parameter, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn main() -> Result<(), Box<std::error::Error>> {
    let random_uuid = [0u8; 16];

    let mut ctx = Context::new()?;

    let param0 = Parameter::tmpref(
        random_uuid.as_ptr() as *mut c_char,
        random_uuid.len(),
        ParamTypeFlags::MemrefTempOutput,
    );
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())?;
    let mut session = ctx.open_session(uuid)?;

    println!("Invoking TA to generate random UUID...");
    let _ = session.invoke_command(TA_RANDOM_CMD_GENERATE, &mut operation)?;
    println!("TA generated UUID value = {:x}{:x}{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}", 
        random_uuid[0], random_uuid[1], random_uuid[2], random_uuid[3], 
        random_uuid[4], random_uuid[5], 
        random_uuid[6], random_uuid[7], 
        random_uuid[8], random_uuid[9], 
        random_uuid[10], random_uuid[11], random_uuid[12], random_uuid[13], random_uuid[14], random_uuid[15]
    );
    Ok(())
}
