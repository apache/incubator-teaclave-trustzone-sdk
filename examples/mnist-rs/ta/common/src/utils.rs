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

use optee_utee::{trace_println, ErrorKind, Parameter, Result};

pub fn copy_to_output(param: &mut Parameter, data: &[u8]) -> Result<()> {
    let mut output = unsafe { param.as_memref()? };

    let buffer = output.buffer();
    if buffer.len() < data.len() {
        trace_println!(
            "expect output buffer size {}, got size {} instead",
            data.len(),
            buffer.len()
        );
        return Err(ErrorKind::ShortBuffer.into());
    }
    buffer[..data.len()].copy_from_slice(data);
    output.set_updated_size(data.len());
    Ok(())
}
