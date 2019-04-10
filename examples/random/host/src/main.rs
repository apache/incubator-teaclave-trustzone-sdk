use optee_teec::{Context, Operation, ParamNone, ParamTmpRef, ParamType, Session, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn random(session: &mut Session) -> optee_teec::Result<()> {
    let mut random_uuid = [0u8; 16];

    let p0 = ParamTmpRef::new(&mut random_uuid, ParamType::MemrefTempOutput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    println!("Invoking TA to generate random UUID...");
    session.invoke_command(Command::RandomGenerator as u32, &mut operation)?;

    let generate_uuid = Uuid::from_slice(operation.parameters().0.buffer()).unwrap();

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
