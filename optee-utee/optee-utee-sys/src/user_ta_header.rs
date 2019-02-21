use libc::*;
use super::tee_api_types::*;
use super::utee_types::*;

pub const TA_FLAG_SINGLE_INSTANCE: uint32_t     = (1 << 2);
pub const TA_FLAG_MULTI_SESSION: uint32_t       = (1 << 3);
pub const TA_FLAG_INSTANCE_KEEP_ALIVE: uint32_t = (1 << 4);
pub const TA_FLAG_SECURE_DATA_PATH: uint32_t    = (1 << 5);
pub const TA_FLAG_REMAP_SUPPORT: uint32_t       = (1 << 6);
pub const TA_FLAG_CACHE_MAINTENANCE: uint32_t   = (1 << 7);

#[repr(C)]
pub struct ta_head {
    pub uuid: TEE_UUID,
    pub stack_size : uint32_t,
    pub flags : uint32_t,
    pub entry : unsafe extern "C" fn(c_ulong, c_ulong, *mut utee_params, c_ulong),
}

unsafe impl Sync for ta_head  {}

pub const TA_PROP_STR_SINGLE_INSTANCE: *const c_char = "gpd.ta.singleInstance\0".as_ptr();
pub const TA_PROP_STR_MULTI_SESSION: *const c_char   = "gpd.ta.multiSession\0".as_ptr();
pub const TA_PROP_STR_KEEP_ALIVE: *const c_char      = "gpd.ta.instanceKeepAlive\0".as_ptr();
pub const TA_PROP_STR_DATA_SIZE: *const c_char       = "gpd.ta.dataSize\0".as_ptr();
pub const TA_PROP_STR_STACK_SIZE: *const c_char      = "gpd.ta.stackSize\0".as_ptr();
pub const TA_PROP_STR_VERSION: *const c_char         = "gpd.ta.version\0".as_ptr();
pub const TA_PROP_STR_DESCRIPTION: *const c_char     = "gpd.ta.description\0".as_ptr();
pub const TA_PROP_STR_UNSAFE_PARAM: *const c_char    = "op-tee.unsafe_param\0".as_ptr();
pub const TA_PROP_STR_REMAP: *const c_char           = "op-tee.remap\0".as_ptr();
pub const TA_PROP_STR_CACHE_SYNC: *const c_char      = "op-tee.cache_sync\0".as_ptr();

#[repr(C)]
pub enum user_ta_prop_type {
    USER_TA_PROP_TYPE_BOOL,
    USER_TA_PROP_TYPE_U32,
    USER_TA_PROP_TYPE_UUID,
    USER_TA_PROP_TYPE_IDENTITY,
    USER_TA_PROP_TYPE_STRING,
    USER_TA_PROP_TYPE_BINARY_BLOCK,
}

#[repr(C)]
pub struct user_ta_property {
    pub name: *const c_char,
    pub prop_type: user_ta_prop_type,
    pub value: *mut c_void,
}

unsafe impl Sync for user_ta_property {}
