#[no_mangle]
pub static mut plugin_method: Plugin_Method = Plugin_Method {
    name: plugin_name.as_ptr() as *mut c_char,
    uuid: PLUGIN_UUID_STRUCT,
    init: syslog_plugin_init,
    invoke: syslog_plugin_invoke,
};

#[no_mangle]
pub static plugin_name: &[u8] = b"syslog\0";
