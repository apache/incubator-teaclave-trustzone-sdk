use optee_teec::{Context, Operation, ParamType, Session, Uuid};
use optee_teec::{ParamNone, ParamValue};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn hello_world(session: &mut Session) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(29, 0, ParamType::ValueInout);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    println!("original value is {:?}", operation.parameters().0.a());

    session.invoke_command(Command::IncValue as u32, &mut operation)?;
    println!("inc value is {:?}", operation.parameters().0.a());

    session.invoke_command(Command::DecValue as u32, &mut operation)?;
    println!("dec value is {:?}", operation.parameters().0.a());
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())
            .unwrap();
    let mut session = ctx.open_session(uuid)?;

    hello_world(&mut session)?;

    println!("Success");
    Ok(())
}
