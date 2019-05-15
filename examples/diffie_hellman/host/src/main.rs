use optee_teec::{Context, Operation, ParamType, Result, Session, Uuid};
use optee_teec::{ParamNone, ParamTmpRef, ParamValue};
use proto::{Command, KEY_SIZE, UUID};

fn generate_key(session: &mut Session) -> Result<(Vec<u8>, Vec<u8>)> {
    // Pass in the prime and base
    let p0 = ParamValue::new(23, 5, ParamType::ValueInput);
    // Save public and private key size
    let p1 = ParamValue::new(0, 0, ParamType::ValueOutput);
    // Vector for generated keys
    let mut public_key = [0u8; KEY_SIZE];
    let mut private_key = [0u8; KEY_SIZE];
    let p2 = ParamTmpRef::new_output(&mut public_key);
    let p3 = ParamTmpRef::new_output(&mut private_key);

    let mut operation = Operation::new(0, p0, p1, p2, p3);
    session.invoke_command(Command::GenerateKey as u32, &mut operation)?;

    let public_size = operation.parameters().1.a() as usize;
    let private_size = operation.parameters().1.b() as usize;
    let mut public_res = vec![0u8; public_size];
    let mut private_res = vec![0u8; private_size];
    public_res.copy_from_slice(&public_key[..public_size]);
    private_res.copy_from_slice(&private_key[..private_size]);

    Ok((public_res, private_res))
}

fn derive_key(key0_pub: &Vec<u8>, session: &mut Session) -> Result<()> {
    let p0 = ParamTmpRef::new_input(key0_pub.as_slice());
    let mut shared_key = [0u8; KEY_SIZE];
    let p1 = ParamTmpRef::new_output(&mut shared_key);
    let p2 = ParamValue::new(0, 0, ParamType::ValueOutput);
    let mut operation = Operation::new(0, p0, p1, p2, ParamNone);

    session.invoke_command(Command::DeriveKey as u32, &mut operation)?;

    let key_size = operation.parameters().2.a() as usize;
    let mut derive_res = vec![0u8; key_size];
    derive_res.copy_from_slice(&shared_key[..key_size]);
    println!("Derived share key as {:?}", derive_res);
    Ok(())
}

fn main() -> Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let (mut key0_public, key0_private) = generate_key(&mut session).unwrap();
    let (key1_public, key1_private) = generate_key(&mut session).unwrap();
    println!(
        "get key 0 pair as public: {:?}, private: {:?}",
        key0_public, key0_private
    );
    println!(
        "get key 1 pair as public: {:?}, private: {:?}",
        key1_public, key1_private
    );
    derive_key(&mut key0_public, &mut session)?;

    println!("Success");
    Ok(())
}
