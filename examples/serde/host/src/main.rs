use optee_teec::{Context, Operation, ParamNone, Session, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn serde(session: &mut Session) -> optee_teec::Result<()> {
    let mut operation = Operation::new(0, ParamNone, ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::DefaultOp as u32, &mut operation)?;

    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())
            .unwrap();
    let mut session = ctx.open_session(uuid)?;

    serde(&mut session)?;

    println!("Success");
    Ok(())
}
