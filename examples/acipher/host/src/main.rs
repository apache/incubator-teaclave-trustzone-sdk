use optee_teec::{Context, Operation, ParamType, Session, Uuid};
use optee_teec::{Error, ErrorKind, ParamNone, ParamTmpRef, ParamValue};
use proto::{Command, UUID};
use std::{env, str};

fn gen_key(session: &mut Session, key_size: u32) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(key_size, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::GenKey as u32, &mut operation)?;

    Ok(())
}

fn encrypt(session: &mut Session, plain_text: &mut [u8]) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(plain_text, ParamType::MemrefTempInput);
    let p1 = ParamValue::new(0, 0, ParamType::ValueOutput);
    let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::GetSize as u32, &mut operation)?;

    let mut cipher_text = vec![0u8; operation.parameters().1.a() as usize];
    let p0 = ParamTmpRef::new(plain_text, ParamType::MemrefTempInput);
    let p1 = ParamTmpRef::new(&mut cipher_text, ParamType::MemrefTempOutput);
    let mut operation2 = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::Encrypt as u32, &mut operation2)?;
    println!(
        "Success encrypt input text \"{}\" as {} bytes cipher text: {:?}",
        str::from_utf8(plain_text).unwrap(),
        cipher_text.len(),
        cipher_text
    );
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Receive {} arguments while 2 arguments are expected!",
            args.len()
        );
        println!("Correct usage: passed 2 arguments as <key_size> and <string to encrypt>");
        return Err(Error::new(ErrorKind::BadParameters));
    }

    let mut key_size = args[1].parse::<u32>().unwrap();
    if key_size < 256 {
        println!(
            "Wrong key size {} is received. Use default minimal key size 256 instead.",
            key_size
        );
        key_size = 256;
    }

    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    gen_key(&mut session, key_size)?;
    encrypt(&mut session, unsafe { args[2].as_bytes_mut() })?;

    Ok(())
}
