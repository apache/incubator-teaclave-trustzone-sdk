use optee_teec::{Context, Operation, ParamTypeFlags, Parameter, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

fn main() -> Result<(), Box<std::error::Error>> {
    let mut ctx = Context::new()?;

    let param0 = Parameter::value(29, 0, ParamTypeFlags::ValueInout);
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())?;
    let mut session = ctx.open_session(uuid)?;

    println!("original value is {}", unsafe {
        operation.raw.params[0].value.a
    });
    let _ = session.invoke_command(TA_HELLO_WORLD_CMD_INC_VALUE, &mut operation)?;
    println!("inc value is {}", unsafe {
        operation.raw.params[0].value.a
    });
    let _ = session.invoke_command(TA_HELLO_WORLD_CMD_DEC_VALUE, &mut operation)?;
    println!("dec value is {}", unsafe {
        operation.raw.params[0].value.a
    });
    Ok(())
}
