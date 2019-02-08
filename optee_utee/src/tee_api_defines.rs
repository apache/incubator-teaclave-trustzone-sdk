use libc::*;

//enum value for ta property entry
pub enum user_ta_prop_type {
        USER_TA_PROP_TYPE_BOOL,
        USER_TA_PROP_TYPE_U32,
        USER_TA_PROP_TYPE_UUID,
        USER_TA_PROP_TYPE_IDENTITY,
        USER_TA_PROP_TYPE_STRING,
        USER_TA_PROP_TYPE_BINARY_BLOCK,
}

//core/lib/libutee/include/user_ta_header.h
pub const TA_FLAG_SINGLE_INSTANCE:uint32_t=         (1 << 2);
pub const TA_FLAG_MULTI_SESSION:uint32_t=           (1 << 3);
pub const TA_FLAG_INSTANCE_KEEP_ALIVE:uint32_t=     (1 << 4); /* remains after last close */
pub const TA_FLAG_SECURE_DATA_PATH:uint32_t=        (1 << 5); /* accesses SDP memory */
pub const TA_FLAG_REMAP_SUPPORT:uint32_t=           (1 << 6); /* use map/unmap syscalls */
pub const TA_FLAG_CACHE_MAINTENANCE:uint32_t=       (1 << 7); /* use cache flush syscall */


//API TA PROPERTY STRING
pub const TA_PROP_STR_SINGLE_INSTANCE : &str = "gpd.ta.singleInstance";
pub const TA_PROP_STR_MULTI_SESSION : &str = "gpd.ta.multiSession";
pub const TA_PROP_STR_KEEP_ALIVE : &str = "gpd.ta.instanceKeepAlive";
pub const TA_PROP_STR_DATA_SIZE : &str = "gpd.ta.dataSize";
pub const TA_PROP_STR_STACK_SIZE : &str = "gpd.ta.stackSize";
pub const TA_PROP_STR_VERSION : &str = "gpd.ta.version";
pub const TA_PROP_STR_DESCRIPTION : &str = "gpd.ta.description";
pub const TA_PROP_STR_UNSAFE_PARAM : &str = "op-tee.unsafe_param";
pub const TA_PROP_STR_REMAP : &str = "op-tee.remap";
pub const TA_PROP_STR_CACHE_SYNC : &str = "op-tee.cache_sync";


// API Error Codes
pub const TEE_SUCCESS: uint32_t                       = 0x00000000;
pub const TEE_ERROR_CORRUPT_OBJECT: uint32_t          = 0xF0100001;
pub const TEE_ERROR_CORRUPT_OBJECT_2: uint32_t        = 0xF0100002;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE: uint32_t   = 0xF0100003;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE_2: uint32_t = 0xF0100004;
pub const TEE_ERROR_GENERIC: uint32_t                 = 0xFFFF0000;
pub const TEE_ERROR_ACCESS_DENIED: uint32_t           = 0xFFFF0001;
pub const TEE_ERROR_CANCEL: uint32_t                  = 0xFFFF0002;
pub const TEE_ERROR_ACCESS_CONFLICT: uint32_t         = 0xFFFF0003;
pub const TEE_ERROR_EXCESS_DATA: uint32_t             = 0xFFFF0004;
pub const TEE_ERROR_BAD_FORMAT: uint32_t              = 0xFFFF0005;
pub const TEE_ERROR_BAD_PARAMETERS: uint32_t          = 0xFFFF0006;
pub const TEE_ERROR_BAD_STATE: uint32_t               = 0xFFFF0007;
pub const TEE_ERROR_ITEM_NOT_FOUND: uint32_t          = 0xFFFF0008;
pub const TEE_ERROR_NOT_IMPLEMENTED: uint32_t         = 0xFFFF0009;
pub const TEE_ERROR_NOT_SUPPORTED: uint32_t           = 0xFFFF000A;
pub const TEE_ERROR_NO_DATA: uint32_t                 = 0xFFFF000B;
pub const TEE_ERROR_OUT_OF_MEMORY: uint32_t           = 0xFFFF000C;
pub const TEE_ERROR_BUSY: uint32_t                    = 0xFFFF000D;
pub const TEE_ERROR_COMMUNICATION: uint32_t           = 0xFFFF000E;
pub const TEE_ERROR_SECURITY: uint32_t                = 0xFFFF000F;
pub const TEE_ERROR_SHORT_BUFFER: uint32_t            = 0xFFFF0010;
pub const TEE_ERROR_EXTERNAL_CANCEL: uint32_t         = 0xFFFF0011;
pub const TEE_ERROR_OVERFLOW: uint32_t                = 0xFFFF300F;
pub const TEE_ERROR_TARGET_DEAD: uint32_t             = 0xFFFF3024;
pub const TEE_ERROR_STORAGE_NO_SPACE: uint32_t        = 0xFFFF3041;
pub const TEE_ERROR_MAC_INVALID: uint32_t             = 0xFFFF3071;
pub const TEE_ERROR_SIGNATURE_INVALID: uint32_t       = 0xFFFF3072;
pub const TEE_ERROR_TIME_NOT_SET: uint32_t            = 0xFFFF5000;
pub const TEE_ERROR_TIME_NEEDS_RESET: uint32_t        = 0xFFFF5001;

pub const TEE_NUM_PARAMS: usize = 4;
