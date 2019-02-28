use libc::*;
use optee_teec::{Context, Operation, ParamTypeFlags, Parameter, Session, Uuid};

include!(concat!(env!("OUT_DIR"), "/host_header.rs"));

const AES_TEST_BUFFER_SIZE: usize = 4096;
const AES_TEST_KEY_SIZE: usize = 16;
const AES_BLOCK_SIZE: usize = 16;

const DECODE: i8 = 0;
const ENCODE: i8 = 1;

fn prepare_aes(session: &mut Session, encode: i8) {
    let param2_value = if encode == ENCODE {
        TA_AES_MODE_ENCODE
    } else {
        TA_AES_MODE_DECODE
    };
    let param0 = Parameter::value(TA_AES_ALGO_CTR, 0, ParamTypeFlags::ValueInput);
    let param1 = Parameter::value(TA_AES_SIZE_128BIT, 0, ParamTypeFlags::ValueInput);
    let param2 = Parameter::value(param2_value, 0, ParamTypeFlags::ValueInput);
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    match session.invoke_command(TA_AES_CMD_PREPARE, &mut operation) {
        Ok(_) => println!("AES operation prepare success.\n"),
        Err(e) => println!("{:?}", e),
    }
}

fn set_key(session: &mut Session, key: &mut [c_char], key_sz: size_t) {
    let param0 = Parameter::tmpref(
        key.as_ptr() as *mut c_char,
        key_sz,
        ParamTypeFlags::MemrefTempInput,
    );
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    match session.invoke_command(TA_AES_CMD_SET_KEY, &mut operation) {
        Ok(_) => println!("AES key has been set.\n"),
        Err(e) => println!("{:?}", e),
    }
}

fn set_iv(session: &mut Session, iv: &mut [c_char], iv_sz: size_t) {
    let param0 = Parameter::tmpref(
        iv.as_ptr() as *mut c_char,
        iv_sz,
        ParamTypeFlags::MemrefTempInput,
    );
    let param1 = Parameter::none();
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    match session.invoke_command(TA_AES_CMD_SET_IV, &mut operation) {
        Ok(_) => println!("AES IV has been set.\n"),
        Err(e) => println!("{:?}", e),
    }
}

fn cipher_buffer(session: &mut Session, intext: &mut [c_char], outtext: &mut [c_char], sz: size_t) {
    let param0 = Parameter::tmpref(
        intext.as_ptr() as *mut c_char,
        sz,
        ParamTypeFlags::MemrefTempInput,
    );
    let param1 = Parameter::tmpref(
        outtext.as_ptr() as *mut c_char,
        sz,
        ParamTypeFlags::MemrefTempOutput,
    );
    let param2 = Parameter::none();
    let param3 = Parameter::none();
    let mut operation = Operation::new(0, param0, param1, param2, param3);

    match session.invoke_command(TA_AES_CMD_CIPHER, &mut operation) {
        Ok(_) => println!("AES Encode/Decode done.\n"),
        Err(e) => println!("{:?}", e),
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())?;
    let mut session = ctx.open_session(uuid)?;

    let mut key = [0xa5u8; AES_TEST_KEY_SIZE];
    let mut iv = [0x00u8; AES_BLOCK_SIZE];
    let mut clear = [0x5au8; AES_TEST_BUFFER_SIZE];
    let mut ciph = [0x00u8; AES_TEST_BUFFER_SIZE];
    let mut tmp = [0x00u8; AES_TEST_BUFFER_SIZE];

    println!("Prepare encode operation...");
    prepare_aes(&mut session, ENCODE);

    println!("Load key in TA...");
    set_key(&mut session, &mut key, AES_TEST_KEY_SIZE as size_t);

    println!("Reset ciphering operation in TA (provides the initial vector)...");
    set_iv(&mut session, &mut iv, AES_BLOCK_SIZE as size_t);

    println!("Encode buffer from TA...");
    cipher_buffer(
        &mut session,
        &mut clear,
        &mut ciph,
        AES_TEST_BUFFER_SIZE as size_t,
    );

    println!("\nPrepare decode operation...");
    prepare_aes(&mut session, DECODE);

    let mut key = [0xa5 as c_char; AES_TEST_KEY_SIZE];
    println!("Load key in TA...");
    set_key(&mut session, &mut key, AES_TEST_KEY_SIZE as size_t);

    let mut iv = [0x00 as c_char; AES_BLOCK_SIZE];
    println!("Reset ciphering operation in TA (provides the initial vector)...");
    set_iv(&mut session, &mut iv, AES_BLOCK_SIZE as size_t);

    println!("Decode buffer from TA...");
    cipher_buffer(
        &mut session,
        &mut ciph,
        &mut tmp,
        AES_TEST_BUFFER_SIZE as size_t,
    );

    if clear.iter().zip(tmp.iter()).all(|(a, b)| a == b) {
        println!("\nDecode the original clear text correctly.");
    } else {
        println!("AES function runs wrong!i");
    }
    Ok(())
}
