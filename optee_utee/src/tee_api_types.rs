use libc::*;

pub type TEE_Result = uint32_t;

#[repr(C)]
pub struct TEE_UUID {
    pub timeLow: uint32_t,
    pub timeMid: uint16_t,
    pub timeHiAndVersion: uint16_t,
    pub clockSeqAndNode: [uint8_t; 8],
}


/*#[repr(C)]
pub struct ta_head_func_ptr {
        ptr64 : uint64_t,
}*/


#[repr(C)]
pub struct ta_head {
        pub uuid: TEE_UUID,
        pub stack_size : c_int,
        pub flags : uint32_t, 
        pub entry : *mut c_void,//ta_head_func_ptr,
}

#[repr(C)]
pub struct user_ta_property {
        pub name : &'static str, //*mut c_char,
        pub ta_type : i8, //original define is type, with enum user_ta_prop_type,
        pub value : *mut c_void,
}

/**mut c_char*/
pub fn user_ta_prop_init(c_ptr:&'static str, number:i8, desc:*mut c_void) -> user_ta_property {
	user_ta_property {name:c_ptr, ta_type:number, value: desc,}
}


#[repr(C)]
pub struct TEE_Identity {
    pub login: uint32_t,
    pub uuid: TEE_UUID,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Memref {
    pub buffer: *mut c_void,
    pub size: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Value {
    pub a: uint32_t,
    pub b: uint32_t,
}

#[repr(C)]
pub union TEE_Param {
    pub memref: Memref,
    pub value: Value
}

pub type ParamTypes = u32;
pub type SessionP = *mut *mut c_void;
