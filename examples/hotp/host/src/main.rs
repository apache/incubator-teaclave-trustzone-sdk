use libc::c_char;
use optee_teec::{Context, Error, ErrorKind, Operation, ParamType, Parameter, Session, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

const TEST_SIZE: usize = 10;
const SIZE_K: usize = 20;
const RFC4226_TEST_VALUES: [u32; TEST_SIZE] = [
    755224, 287082, 359152, 969429, 338314, 254676, 287922, 162583, 399871, 520489,
];

fn register_shared_key(session: &mut Session) -> optee_teec::Result<()> {
    let mut k: [u8; SIZE_K] = [
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
        0x36, 0x37, 0x38, 0x39, 0x30,
    ];

    let p0 = Parameter::from_tmpref(
        (&mut k).as_ptr() as *mut c_char,
        SIZE_K,
        ParamType::MemrefTempInput,
    );
    let p1 = Parameter::new();
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    session.invoke_command(Command::RegisterSharedKey as u32, &mut operation)?;
    Ok(())
}

fn get_hotp(session: &mut Session) -> optee_teec::Result<()> {
    let mut res = [0u8; SIZE_K];
    let p0 = Parameter::from_value(
        (&mut res) as *mut _ as u32,
        SIZE_K as u32,
        ParamType::ValueOutput,
    );
    let p1 = Parameter::new();
    let p2 = Parameter::new();
    let p3 = Parameter::new();
    let mut operation = Operation::new(0, p0, p1, p2, p3);

    for i in 0..TEST_SIZE {
        session.invoke_command(Command::GetHOTP as u32, &mut operation)?;
        let (p0, _, _, _) = operation.parameters();
        let (hotp_value, _) = p0.value();

        println!("Get HOTP: {}", hotp_value);

        if hotp_value != RFC4226_TEST_VALUES[i] {
            println!(
                "Wrong value get! Expected value: {}",
                RFC4226_TEST_VALUES[i]
            );
            return Err(Error::new(ErrorKind::Generic));
        }
    }
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())
            .unwrap();
    let mut session = ctx.open_session(uuid)?;

    register_shared_key(&mut session)?;
    get_hotp(&mut session)?;

    println!("Success");
    Ok(())
}
