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

use optee_teec::{Context, ErrorKind, Operation, Param, Session, Uuid};
use optee_teec::{ParamNone, ParamTmpRef, ParamType, ParamValue};

pub struct Connection {
    session: Session,
    identity: [u8; proto::IDENTITY_SIZE],
    last_err: Option<ErrorKind>,
}

impl Connection {
    pub fn new(ctx: &mut Context, uuid: Uuid) -> optee_teec::Result<Self> {
        let mut identity = [0_u8; proto::IDENTITY_SIZE];
        let session = {
            let mut operation = Operation::new(
                0,
                ParamTmpRef::new_output(&mut identity),
                ParamNone,
                ParamNone,
                ParamNone,
            );
            ctx.open_session_with_operation(uuid, &mut operation)?
        };
        Ok(Connection {
            session,
            identity,
            last_err: None,
        })
    }

    pub fn invoke_command<A: Param, B: Param, C: Param, D: Param>(
        &mut self,
        command_id: u32,
        operation: &mut Operation<A, B, C, D>,
    ) -> optee_teec::Result<()> {
        let result = self.session.invoke_command(command_id, operation);
        self.last_err = match result.as_ref() {
            Ok(()) => None,
            Err(err) => Some(err.kind()),
        };
        result
    }

    pub fn identity(&self) -> &[u8] {
        &self.identity
    }

    pub fn valid(&self) -> optee_teec::Result<()> {
        match self.last_err {
            Some(ErrorKind::TargetDead) => Err(ErrorKind::TargetDead.into()),
            _ => Ok(()),
        }
    }
}

pub fn tee_wait(session: &mut Connection, milliseconds: u32) -> optee_teec::Result<()> {
    let mut operation = Operation::new(
        0,
        ParamValue::new(milliseconds, 0, ParamType::ValueInput),
        ParamNone,
        ParamNone,
        ParamNone,
    );
    session.invoke_command(0, &mut operation)
}
