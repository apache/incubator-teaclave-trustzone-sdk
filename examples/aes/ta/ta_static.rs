const TRACE_LEVEL: c_int = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_VERSION: &[u8] = b"Undefined version\0";
const TA_DESCRIPTION: &[u8] = b"Undefined description\0";
const TA_FRAMEWORK_STACK_SIZE: uint32_t = 2048;

#[no_mangle]
pub static mut trace_level: c_int = TRACE_LEVEL;

#[no_mangle]
pub static trace_ext_prefix: &[u8] = TRACE_EXT_PREFIX;

extern "C" {
    fn __utee_entry(func: c_ulong, session_id: c_ulong, up: *mut utee_params, cmd_id: c_ulong);
}

#[no_mangle]
#[link_section = ".ta_head"]
pub static ta_head: ta_head = ta_head {
    uuid: TA_UUID,
    stack_size: TA_STACK_SIZE + TA_FRAMEWORK_STACK_SIZE,
    flags: TA_FLAGS,
    entry: __utee_entry as unsafe extern "C" fn(c_ulong, c_ulong, *mut utee_params, c_ulong),
};

#[no_mangle]
pub static ta_heap: &[u8; TA_DATA_SIZE as usize] = &['\0' as u8; TA_DATA_SIZE as usize];

#[no_mangle]
pub static ta_heap_size: size_t = mem::size_of::<u8>() * TA_DATA_SIZE as usize;
pub static flag_bool: bool = (TA_FLAGS & TA_FLAG_SINGLE_INSTANCE) != 0;
pub static flag_multi: bool = (TA_FLAGS & TA_FLAG_MULTI_SESSION) != 0;
pub static flag_instance: bool = (TA_FLAGS & TA_FLAG_INSTANCE_KEEP_ALIVE) != 0;

#[no_mangle]
pub static ta_num_props: size_t = 9;

#[no_mangle]
pub static ta_props: [user_ta_property; 9] = [
    user_ta_property {
        name: TA_PROP_STR_SINGLE_INSTANCE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_BOOL,
        value: &flag_bool as *const bool as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_MULTI_SESSION,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_BOOL,
        value: &flag_multi as *const bool as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_KEEP_ALIVE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_BOOL,
        value: &flag_instance as *const bool as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_DATA_SIZE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_U32,
        value: &TA_DATA_SIZE as *const uint32_t as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_STACK_SIZE,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_U32,
        value: &TA_STACK_SIZE as *const uint32_t as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_VERSION,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_STRING,
        value: TA_VERSION as *const [u8] as *mut _,
    },
    user_ta_property {
        name: TA_PROP_STR_DESCRIPTION,
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_STRING,
        value: TA_DESCRIPTION as *const [u8] as *mut _,
    },
    user_ta_property {
        name: "gp.ta.description\0".as_ptr(),
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_STRING,
        value: EXT_PROP_VALUE_1 as *const [u8] as *mut _,
    },
    user_ta_property {
        name: "gp.ta.version\0".as_ptr(),
        prop_type: user_ta_prop_type::USER_TA_PROP_TYPE_U32,
        value: &EXT_PROP_VALUE_2 as *const uint32_t as *mut _,
    },
];

#[no_mangle]
pub unsafe extern "C" fn tahead_get_trace_level() -> c_int {
    return trace_level;
}

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    match MESA_CreateEntryPoint() {
        Ok(_) => {
            trace_println!("[+] CreateEntryPoint.");//, ta_name);
            return TEE_SUCCESS;
        }
        Err(e) => {
            trace_println!("{:x?}", e);
            return e.raw_code();
        }
    }
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {
    match MESA_DestroyEntryPoint() {
        Ok(_) => {
            trace_println!("[+] DestroyEntryPoint.");//, ta_name);
        }
        Err(e) => {
            trace_println!("{:x?}", e);
        }
    }
}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(
    param_types: uint32_t,
    params: &mut [TEE_Param; 4],
    sess_ctx: *mut *mut c_void,
) -> TEE_Result {
    let mut parameters = Parameters::new(params, param_types);

    match MESA_OpenSessionEntryPoint(&mut parameters, sess_ctx) {
        Ok(_) => {
            trace_println!("[+] OpenSessionEntryPoint.");//, ta_name);
            return TEE_SUCCESS;
        }
        Err(e) => {
            trace_println!("{:x?}", e);
            return e.raw_code();
        }
    }
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(sess_ctx: *mut *mut c_void) {
    match MESA_CloseSessionEntryPoint(sess_ctx) {
        Ok(_) => {
            trace_println!("[+] CloseSessionEntryPoint.");//, ta_name);
        }
        Err(e) => {
            trace_println!("{:x?}", e);
        }
    }
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    sess_ctx: *mut c_void,
    cmd_id: u32,
    param_types: uint32_t,
    params: &mut [TEE_Param; 4],
) -> TEE_Result {
    let mut parameters = Parameters::new(params, param_types);

    match MESA_InvokeCommandEntryPoint(sess_ctx, cmd_id, &mut parameters) {
        Ok(_) => {
            trace_println!("[+] InvokeCommandEntryPoint.");//, ta_name);
            return TEE_SUCCESS;
        }
        Err(e) => {
            trace_println!("{:x?}", e);
            return e.raw_code();
        }
    }
}
