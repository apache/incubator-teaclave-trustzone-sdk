use crate::{Error, Result, Uuid};
use optee_utee_sys as raw;

pub struct LoadablePlugin<'a> {
    uuid: &'a Uuid
}

impl<'a> LoadablePlugin<'a> {
    pub fn new(uuid: &'a Uuid) -> Self {
        Self { uuid }
    }
    pub fn invoke(&mut self, command_id: u32, subcommand_id: u32, data: &[u8]) -> Result<Vec<u8>> {
        let raw_uuid: Uuid = (*self.uuid).clone();
        let mut outlen: u32 = 0;
        match unsafe {
            raw::tee_invoke_supp_plugin(
                raw_uuid.as_raw_ptr(),
                command_id as u32,
                subcommand_id as u32,
                data.as_ptr() as _,
                data.len() as u32,
                &mut outlen as *mut u32,
            )
        } {
            raw::TEE_SUCCESS => {
                assert!(outlen <= (data.len() as u32));
                let mut outbuf = vec![0; outlen as usize];
                outbuf.copy_from_slice(&data[..(outlen as usize)]);
                
                Ok(outbuf)
            },
            code => Err(Error::from_raw_error(code)),
        }
       
    }
}
