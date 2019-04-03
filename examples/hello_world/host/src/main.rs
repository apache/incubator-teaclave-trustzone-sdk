use optee_teec::{Context, Operation, ParamType, Parameter, Session, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn hello_world(session: &mut Session) -> optee_teec::Result<()> {
    let p0 = Parameter::from_value(29, 0, ParamType::ValueInout);
    let p1 = Parameter::new();
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    let (p0, _, _, _) = operation.parameters();
    println!("original value is {}", p0.value().0);

    session.invoke_command(Command::IncValue as u32, &mut operation)?;
    let (p0, _, _, _) = operation.parameters();
    println!("inc value is {}", p0.value().0);

    session.invoke_command(Command::DecValue as u32, &mut operation)?;
    let (p0, _, _, _) = operation.parameters();
    println!("dec value is {}", p0.value().0);
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
