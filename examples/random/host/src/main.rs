use libc::*;
use optee_teec::{Context, Operation, ParamType, Parameter, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn main() -> Result<(), Box<std::error::Error>> {
    let random_uuid = [0u8; 16];

    let mut ctx = Context::new()?;

    let param0 = Parameter::from_tmpref(
        random_uuid.as_ptr() as *mut c_char,
        random_uuid.len(),
        ParamType::MemrefTempOutput,
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
    let generate_uuid = Uuid::from_bytes(random_uuid);
    println!("Generate random UUID: {}", generate_uuid);

    Ok(())
}
