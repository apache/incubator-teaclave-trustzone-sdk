// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use optee_utee_sys as raw;
use std::io;
use std::io::ErrorKind;
use std::ptr;

pub struct TcpStream {
    pub handle: raw::TEE_iSocketHandle,
}

impl TcpStream {
    fn connect_with_ip_version(
        address: &str,
        port: u16,
        ip_version: raw::TEE_ipSocket_ipVersion,
    ) -> std::io::Result<Self> {
        use std::ffi::CString;
        unsafe {
            let addr = match CString::new(address) {
                Ok(addr) => addr,
                Err(_) => return Err(io::Error::new(ErrorKind::Other, "Invalid address")),
            };
            let mut handle: raw::TEE_iSocketHandle = ptr::null_mut();
            let mut protocol_error: u32 = 0;
            let mut setup = raw::TEE_tcpSocket_Setup {
                ipVersion: ip_version,
                server_addr: addr.as_ptr() as _,
                server_port: port,
            };
            let ret = ((*raw::TEE_tcpSocket).open)(
                &mut handle,
                &mut setup as *mut raw::TEE_tcpSocket_Setup as _,
                &mut protocol_error,
            );
            match ret {
                raw::TEE_SUCCESS => Ok(Self { handle }),
                raw::TEE_ERROR_CANCEL => {
                    Err(io::Error::new(ErrorKind::Interrupted, "TEE_ERROR_CANCEL"))
                }
                raw::TEE_ERROR_OUT_OF_MEMORY => {
                    Err(io::Error::new(ErrorKind::Other, "TEE_ERROR_OUT_OF_MEMORY"))
                }
                raw::TEE_ERROR_BAD_PARAMETERS => {
                    Err(io::Error::new(ErrorKind::Other, "TEE_ERROR_BAD_PARAMETERS"))
                }
                raw::TEE_ISOCKET_ERROR_TIMEOUT => Err(io::Error::new(
                    ErrorKind::TimedOut,
                    "TEE_ISOCKET_ERROR_TIMEOUT",
                )),
                raw::TEE_ERROR_COMMUNICATION => Err(io::Error::new(
                    ErrorKind::ConnectionAborted,
                    "TEE_ERROR_COMMUNICATION",
                )),
                raw::TEE_ISOCKET_ERROR_PROTOCOL => Err(io::Error::new(
                    ErrorKind::Other,
                    "TEE_ISOCKET_ERROR_PROTOCOL",
                )),
                raw::TEE_ISOCKET_WARNING_PROTOCOL => Err(io::Error::new(
                    ErrorKind::Other,
                    format!("TEE_ISOCKET_WARNING_PROTOCOL: {}", protocol_error),
                )),
                _ => panic!("Unexpected return value"),
            }
        }
    }

    pub fn connect_v4(address: &str, port: u16) -> std::io::Result<Self> {
        Self::connect_with_ip_version(address, port, raw::TEE_ipSocket_ipVersion::TEE_IP_VERSION_4)
    }

    pub fn connect_v6(address: &str, port: u16) -> std::io::Result<Self> {
        Self::connect_with_ip_version(address, port, raw::TEE_ipSocket_ipVersion::TEE_IP_VERSION_4)
    }

    pub fn connect(address: &str, port: u16) -> std::io::Result<Self> {
        Self::connect_v4(address, port)
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        // Ignore any errors on close.
        unsafe {
            ((*raw::TEE_tcpSocket).close)(self.handle);
        }
    }
}

impl std::io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut length: u32 = buf.len() as _;
        let ret = unsafe {
            ((*raw::TEE_tcpSocket).recv)(
                self.handle,
                buf.as_mut_ptr() as _,
                &mut length,
                raw::TEE_TIMEOUT_INFINITE,
            )
        };

        match ret {
            raw::TEE_SUCCESS => Ok(length as _),
            raw::TEE_ERROR_CANCEL => {
                Err(io::Error::new(ErrorKind::Interrupted, "TEE_ERROR_CANCEL"))
            }
            raw::TEE_ISOCKET_ERROR_TIMEOUT => Err(io::Error::new(
                ErrorKind::TimedOut,
                "TEE_ISOCKET_ERROR_TIMEOUT",
            )),
            raw::TEE_ERROR_COMMUNICATION => Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "TEE_ERROR_COMMUNICATION",
            )),
            raw::TEE_ISOCKET_ERROR_REMOTE_CLOSED => Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "TEE_ISOCKET_ERROR_REMOTE_CLOSED",
            )),
            raw::TEE_ISOCKET_ERROR_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_ERROR_PROTOCOL",
            )),
            raw::TEE_ISOCKET_WARNING_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_WARNING_PROTOCOL",
            )),
            _ => panic!("Unexpected return value"),
        }
    }
}

impl std::io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut length: u32 = buf.len() as _;
        let ret = unsafe {
            ((*raw::TEE_tcpSocket).send)(
                self.handle,
                buf.as_ptr() as *const u8 as _,
                &mut length,
                raw::TEE_TIMEOUT_INFINITE,
            )
        };

        match ret {
            raw::TEE_SUCCESS => Ok(length as _),
            raw::TEE_ERROR_CANCEL => {
                Err(io::Error::new(ErrorKind::Interrupted, "TEE_ERROR_CANCEL"))
            }
            raw::TEE_ISOCKET_ERROR_TIMEOUT => Err(io::Error::new(
                ErrorKind::TimedOut,
                "TEE_ISOCKET_ERROR_TIMEOUT",
            )),
            raw::TEE_ISOCKET_ERROR_REMOTE_CLOSED => Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "TEE_ISOCKET_ERROR_REMOTE_CLOSED",
            )),
            raw::TEE_ISOCKET_ERROR_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_ERROR_PROTOCOL",
            )),
            raw::TEE_ISOCKET_WARNING_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_WARNING_PROTOCOL",
            )),
            raw::TEE_ISOCKET_ERROR_LARGE_BUFFER => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_ERROR_LARGE_BUFFER",
            )),
            _ => panic!("Unexpected return value"),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct UdpSocket {
    pub handle: raw::TEE_iSocketHandle,
}

impl UdpSocket {
    fn connect_with_ip_version(
        address: &str,
        port: u16,
        ip_version: raw::TEE_ipSocket_ipVersion,
    ) -> std::io::Result<Self> {
        use std::ffi::CString;
        unsafe {
            let addr = match CString::new(address) {
                Ok(addr) => addr,
                Err(_) => return Err(io::Error::new(ErrorKind::Other, "Invalid address")),
            };
            let mut handle: raw::TEE_iSocketHandle = ptr::null_mut();
            let mut protocol_error: u32 = 0;
            let mut setup = raw::TEE_udpSocket_Setup {
                ipVersion: ip_version,
                server_addr: addr.as_ptr() as _,
                server_port: port,
            };
            let ret = ((*raw::TEE_udpSocket).open)(
                &mut handle,
                &mut setup as *mut raw::TEE_udpSocket_Setup as _,
                &mut protocol_error,
            );
            match ret {
                raw::TEE_SUCCESS => Ok(Self { handle }),
                raw::TEE_ERROR_CANCEL => {
                    Err(io::Error::new(ErrorKind::Interrupted, "TEE_ERROR_CANCEL"))
                }
                raw::TEE_ERROR_OUT_OF_MEMORY => {
                    Err(io::Error::new(ErrorKind::Other, "TEE_ERROR_OUT_OF_MEMORY"))
                }
                raw::TEE_ERROR_BAD_PARAMETERS => {
                    Err(io::Error::new(ErrorKind::Other, "TEE_ERROR_BAD_PARAMETERS"))
                }
                raw::TEE_ISOCKET_ERROR_TIMEOUT => Err(io::Error::new(
                    ErrorKind::TimedOut,
                    "TEE_ISOCKET_ERROR_TIMEOUT",
                )),
                raw::TEE_ERROR_COMMUNICATION => Err(io::Error::new(
                    ErrorKind::ConnectionAborted,
                    "TEE_ERROR_COMMUNICATION",
                )),
                raw::TEE_ISOCKET_ERROR_PROTOCOL => Err(io::Error::new(
                    ErrorKind::Other,
                    "TEE_ISOCKET_ERROR_PROTOCOL",
                )),
                raw::TEE_ISOCKET_WARNING_PROTOCOL => Err(io::Error::new(
                    ErrorKind::Other,
                    format!("TEE_ISOCKET_WARNING_PROTOCOL: {}", protocol_error),
                )),
                _ => panic!("Unexpected return value"),
            }
        }
    }

    pub fn connect_v4(address: &str, port: u16) -> std::io::Result<Self> {
        Self::connect_with_ip_version(address, port, raw::TEE_ipSocket_ipVersion::TEE_IP_VERSION_4)
    }

    pub fn connect_v6(address: &str, port: u16) -> std::io::Result<Self> {
        Self::connect_with_ip_version(address, port, raw::TEE_ipSocket_ipVersion::TEE_IP_VERSION_4)
    }

    pub fn connect(address: &str, port: u16) -> std::io::Result<Self> {
        Self::connect_v4(address, port)
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        // Ignore any errors on close.
        unsafe {
            ((*raw::TEE_udpSocket).close)(self.handle);
        }
    }
}

impl std::io::Read for UdpSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut length: u32 = buf.len() as _;
        let ret = unsafe {
            ((*raw::TEE_udpSocket).recv)(
                self.handle,
                buf.as_mut_ptr() as _,
                &mut length,
                raw::TEE_TIMEOUT_INFINITE,
            )
        };

        match ret {
            raw::TEE_SUCCESS => Ok(length as _),
            raw::TEE_ERROR_CANCEL => {
                Err(io::Error::new(ErrorKind::Interrupted, "TEE_ERROR_CANCEL"))
            }
            raw::TEE_ISOCKET_ERROR_TIMEOUT => Err(io::Error::new(
                ErrorKind::TimedOut,
                "TEE_ISOCKET_ERROR_TIMEOUT",
            )),
            raw::TEE_ERROR_COMMUNICATION => Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "TEE_ERROR_COMMUNICATION",
            )),
            raw::TEE_ISOCKET_ERROR_REMOTE_CLOSED => Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "TEE_ISOCKET_ERROR_REMOTE_CLOSED",
            )),
            raw::TEE_ISOCKET_ERROR_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_ERROR_PROTOCOL",
            )),
            raw::TEE_ISOCKET_WARNING_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_WARNING_PROTOCOL",
            )),
            _ => panic!("Unexpected return value"),
        }
    }
}

impl std::io::Write for UdpSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut length: u32 = buf.len() as _;
        let ret = unsafe {
            ((*raw::TEE_udpSocket).send)(
                self.handle,
                buf.as_ptr() as *const u8 as _,
                &mut length,
                raw::TEE_TIMEOUT_INFINITE,
            )
        };

        match ret {
            raw::TEE_SUCCESS => Ok(length as _),
            raw::TEE_ERROR_CANCEL => {
                Err(io::Error::new(ErrorKind::Interrupted, "TEE_ERROR_CANCEL"))
            }
            raw::TEE_ISOCKET_ERROR_TIMEOUT => Err(io::Error::new(
                ErrorKind::TimedOut,
                "TEE_ISOCKET_ERROR_TIMEOUT",
            )),
            raw::TEE_ISOCKET_ERROR_REMOTE_CLOSED => Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "TEE_ISOCKET_ERROR_REMOTE_CLOSED",
            )),
            raw::TEE_ISOCKET_ERROR_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_ERROR_PROTOCOL",
            )),
            raw::TEE_ISOCKET_WARNING_PROTOCOL => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_WARNING_PROTOCOL",
            )),
            raw::TEE_ISOCKET_ERROR_LARGE_BUFFER => Err(io::Error::new(
                ErrorKind::Other,
                "TEE_ISOCKET_ERROR_LARGE_BUFFER",
            )),
            _ => panic!("Unexpected return value"),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
