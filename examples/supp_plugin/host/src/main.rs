use optee_teec::{Context, Operation, ParamTmpRef, Session, Uuid};
use optee_teec::{ParamNone};
use proto::{TA_UUID, Command};

fn ping_ta(session: &mut Session) -> optee_teec::Result<()> {
    let test_data = [0x36u8; 10];
    let p0 = ParamTmpRef::new_input(&test_data);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    println!("-*Work logic: host -> TA -> plugin*-");
    println!("*host*: send value {:?} to ta", test_data);
    session.invoke_command(Command::Ping as u32, &mut operation)?;
    println!("*host*: invoke commmand finished");

    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(TA_UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    ping_ta(&mut session)?;

    println!("Success");
    Ok(())
}
