pub const TA_AES_CMD_PREPARE: u32 = 0;
pub const TA_AES_CMD_SET_KEY: u32 =  1;
pub const TA_AES_CMD_SET_IV: u32 = 2;
pub const TA_AES_CMD_CIPHER: u32 = 3;

pub const TA_AES_ALGO_ECB: u32 = 0;
pub const TA_AES_ALGO_CBC: u32 = 1;
pub const TA_AES_ALGO_CTR: u32 = 2;

pub const TA_AES_SIZE_128BIT: u32 = (128 / 8);
pub const TA_AES_SIZE_256BIT: u32 = (256 / 8);

pub const TA_AES_MODE_ENCODE: u32 = 1;
pub const TA_AES_MODE_DECODE: u32 = 0;
