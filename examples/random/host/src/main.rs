use libc::c_char;
use optee_teec::{Context, Operation, ParamType, Parameter, Session, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn random(session: &mut Session) -> optee_teec::Result<()> {
    let random_uuid = [0u8; 16];

    let p0 = Parameter::from_tmpref(
        random_uuid.as_ptr() as *mut c_char,
        random_uuid.len(),
        ParamType::MemrefTempOutput,
    );
    let p1 = Parameter::new();
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    println!("Invoking TA to generate random UUID...");
    session.invoke_command(TA_RANDOM_CMD_GENERATE, &mut operation)?;
    let generate_uuid = Uuid::from_bytes(random_uuid);

    println!("Generate random UUID: {}", generate_uuid);
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;

    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())
            .unwrap();
    let mut session = ctx.open_session(uuid)?;

    random(&mut session)?;

    println!("Success");
    Ok(())
}
