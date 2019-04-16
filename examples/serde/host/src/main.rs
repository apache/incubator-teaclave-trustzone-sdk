use optee_teec::{Context, Operation, ParamNone, ParamTmpRef, ParamType, Session, Uuid};
use serde::Deserialize;

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

#[derive(Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn serde(session: &mut Session) -> optee_teec::Result<()> {
    let mut buffer = [0u8; 128];
    let p0 = ParamTmpRef::new(&mut buffer, ParamType::MemrefTempOutput);
    let mut operation = Operation::new(0, p0 , ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::DefaultOp as u32, &mut operation)?;
    let updated_size = operation.parameters().0.updated_size();

    let p: Point = serde_json::from_slice(&buffer[..updated_size]).unwrap();
    println!("{:?}", p);

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
