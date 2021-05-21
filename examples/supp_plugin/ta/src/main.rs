#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result, Uuid};
use optee_utee::{LoadablePlugin};
use proto::{Command, PluginCommand, PLUGIN_SUBCMD_NULL, PLUGIN_UUID};

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    let mut p0 = unsafe { params.0.as_memref().unwrap() };
    trace_println!("[+] TA received value {:?} then send to plugin", p0.buffer());
    let uuid = Uuid::parse_str(PLUGIN_UUID).unwrap();

    match Command::from(cmd_id) {
        Command::Ping => {
            let mut plugin = LoadablePlugin::new(&uuid);
            let outbuf = plugin.invoke(
                PluginCommand::Print as u32, 
                PLUGIN_SUBCMD_NULL, 
                p0.buffer()
            ).unwrap();

            trace_println!("[+] TA received out value {:?} outlen {:?}", outbuf, outbuf.len());
            trace_println!("[+] TA call invoke_supp_plugin finished");

            Ok(())
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"This is a plugin example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Plugin TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
