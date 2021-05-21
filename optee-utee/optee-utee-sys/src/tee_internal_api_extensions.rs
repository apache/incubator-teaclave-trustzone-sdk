use super::*;
use libc::*;

extern "C" {
    /// tee_invoke_supp_plugin() - invoke a tee-supplicant's plugin
    /// @uuid:       uuid of the plugin
    /// @cmd:        command for the plugin
    /// @sub_cmd:    subcommand for the plugin
    /// @buf:        data [for/from] the plugin [in/out]
    /// @len:        length of the input buf
    /// @outlen:     pointer to length of the output data (if they will be used)
    /// Return TEE_SUCCESS on success or TEE_ERRROR_* on failure.
    pub fn tee_invoke_supp_plugin(
        uuid: *const TEE_UUID,
        cmd: u32,
        sub_cmd: u32,
        buf: *mut c_char,
        len: u32,
        outlen: *mut u32,
    ) -> TEE_Result; 

}
