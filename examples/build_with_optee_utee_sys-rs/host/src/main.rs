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

use optee_teec::{Context, ErrorKind, Operation, ParamType, Session, Uuid};
use optee_teec::{ParamNone, ParamValue};
use proto::{Command, UUID};

fn inc_value(session: &mut Session) -> optee_teec::Result<u32> {
    let p0 = ParamValue::new(0, 0, ParamType::ValueOutput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    session.invoke_command(Command::IncValue as u32, &mut operation)?;
    Ok(operation.parameters().0.a())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).map_err(|_|ErrorKind::BadParameters)?;
    // Ensure that multiple sessions can be opened concurrently.
    let mut session1= ctx.open_session(uuid.clone())?;
    let mut session2= ctx.open_session(uuid)?;
    // Ensure that each session can successfully perform a call.
    println!("result is: {}", inc_value(&mut session1)?);
    println!("result is: {}", inc_value(&mut session2)?);
    Ok(())
}
