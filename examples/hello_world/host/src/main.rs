use optee_teec::{Context, Operation, ParamType, Parameter, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn hello_world() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;

    let p0 = Parameter::from_value(29, 0, ParamType::ValueInout);
    let p1 = Parameter::none();
    let p2 = Parameter::none();
    let p3 = Parameter::none();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim()).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let (p0, _, _, _)= operation.parameters();
    println!("original value is {}", p0.value().0);

    let _ = session.invoke_command(Command::IncValue as u32, &mut operation)?;
    let (p0, _, _, _)= operation.parameters();
    println!("inc value is {}", p0.value().0);

    let _ = session.invoke_command(Command::DecValue as u32, &mut operation)?;
    let (p0, _, _, _)= operation.parameters();
    println!("dec value is {}", p0.value().0);
    Ok(())
}

fn main() {
    match hello_world() {
        Ok(_) => println!("Success"),
        Err(e) => println!("{}", e)
    }
}
