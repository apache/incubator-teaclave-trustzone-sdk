use optee_teec::{Context, Operation, Parameter, Uuid};

fn hello_world() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;

    let p0 = Parameter::new();
    let p1 = Parameter::new();
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim()).unwrap();
    let mut session = ctx.open_session(uuid)?;
    session.invoke_command(0, &mut operation)?;

    Ok(())
}

fn main() {
    match hello_world() {
        Ok(_) => println!("Success"),
        Err(e) => println!("{}", e)
    }
}
