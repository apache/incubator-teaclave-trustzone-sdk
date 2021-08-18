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

use optee_teec::{Context, Operation, Session, Uuid};
use optee_teec::ParamNone;
use proto::{UUID, Command};
use std::thread;
use std::net::UdpSocket;
use std::str;

fn udp_socket(session: &mut Session) -> optee_teec::Result<()> {
    let mut operation = Operation::new(0, ParamNone, ParamNone, ParamNone, ParamNone);
    session.invoke_command(Command::Start as u32, &mut operation)?;
    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let child = thread::spawn(move || {
        let mut session = ctx.open_session(uuid).unwrap();
        udp_socket(&mut session).unwrap();
    });

    let mut buf = [0; 100];
    let (_, src_addr) = socket.recv_from(&mut buf).unwrap();
    socket.send_to(b"[Host] Hello, Teaclave!", src_addr).unwrap();
    println!("{}", str::from_utf8(&buf).unwrap());
    let _ = child.join();

    println!("Success");
    Ok(())
}
