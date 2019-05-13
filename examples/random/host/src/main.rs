use optee_teec::{Context, Operation, ParamNone, ParamTmpRef, ParamType, Session, Uuid};
use proto::{Command, UUID};

fn random(session: &mut Session) -> optee_teec::Result<()> {
    let mut random_uuid = [0u8; 16];

    let p0 = ParamTmpRef::new_output(&mut random_uuid, ParamType::MemrefTempOutput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    println!("Invoking TA to generate random UUID...");
    session.invoke_command(Command::RandomGenerator as u32, &mut operation)?;

    let generate_uuid = Uuid::from_slice(&random_uuid).unwrap();

    println!("Generate random UUID: {}", generate_uuid);
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;

    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    random(&mut session)?;

    println!("Success");
    Ok(())
}
