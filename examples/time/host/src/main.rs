use optee_teec::ParamNone;
use optee_teec::{Context, Operation, Session, Uuid};
use proto::{Command, UUID};

fn time(session: &mut Session) -> optee_teec::Result<()> {
    let mut operation = Operation::new(0, ParamNone, ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::Test as u32, &mut operation)?;

    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    time(&mut session)?;

    println!("Success");
    Ok(())
}
