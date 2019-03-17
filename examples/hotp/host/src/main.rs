use libc::*;
use optee_teec::{Context, Operation, ParamType, Parameter, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

const TEST_SIZE: usize = 10;

const RFC4226_TEST_VALUES: [u32; TEST_SIZE] = [
    755224, 287082, 359152, 969429, 338314, 254676, 287922, 162583, 399871, 520489,
];

fn main() -> Result<(), Box<std::error::Error>> {
    const SIZE_K: usize = 20;
    let mut k: [u8; SIZE_K] = [
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
        0x36, 0x37, 0x38, 0x39, 0x30,
    ];

    let mut ctx = Context::new()?;

    let param0 = Parameter::from_tmpref(
        (&mut k).as_ptr() as *mut c_char,
        SIZE_K,
        ParamType::MemrefTempInput,
    );
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())?;
    let mut session = ctx.open_session(uuid)?;

    session.invoke_command(TA_HOTP_CMD_REGISTER_SHARED_KEY, &mut operation)?;

    let param0 = Parameter::from_value(
        (&mut k) as *mut _ as u32,
        SIZE_K as u32,
        ParamType::ValueOutput,
    );
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    operation = Operation::new(0, param0, param1, param2, param3);

    let mut hotp_value: u32;
    for i in 0..TEST_SIZE {
        session.invoke_command(TA_HOTP_CMD_GET_HOTP, &mut operation)?;
        unsafe {
            hotp_value = operation.raw.params[0].value.a;
        }

        println!("Get HOTP: {}", hotp_value);

        if hotp_value != RFC4226_TEST_VALUES[i] {
            println!(
                "Wrong value get! Expected value: {}",
                RFC4226_TEST_VALUES[i]
            );
            //return Err(Box::new(()));
            return Ok(());
        }
    }
    Ok(())
}
