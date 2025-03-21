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

mod connection;
pub mod mobc_pool;
pub mod r2d2_pool;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = None, long_about)]
pub struct Args {
    /// The duration of the REE wait time, which the REE waits for all tasks to
    /// complete.
    #[arg(short, long, default_value_t = 1500)]
    execution_timeout: u32,

    /// The duration of the TEE wait time, which a TEEC_Session is holding.
    #[arg(short, long, default_value_t = 1000)]
    ta_wait_timeout: u32,

    /// The capacity of the TEEC_Session pool.
    #[arg(short, long, default_value_t = 10)]
    pool_capacity: u32,

    /// The total number of tasks running concurrently, with each task holding a
    /// TEEC_Session for approximately ${ta_wait_timeout}.
    #[arg(short, long, default_value_t = 10)]
    concurrency: u64,
}
