#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate optee_utee;
pub use optee_utee::*;

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    return TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {
}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(_paramTypes: ParamTypes, _params: TEE_Param, _sessionContext: SessionP) -> TEE_Result {
    return TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sessionContext: SessionP) {
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(_sessionContext: SessionP, _commandID: u32, _paramTypes: ParamTypes, _params: &mut [TEE_Param; 4]) -> TEE_Result {
    match _commandID {
        0 => {
            unsafe { _params[0].value.a += 121; }
        },
        1 => {
            unsafe { _params[0].value.a -= 21; }
        },
        _ => {
            return TEE_ERROR_BAD_PARAMETERS;
        }
    }
    return TEE_SUCCESS;
}
