use libc::*;

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
