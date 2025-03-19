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

mod pool;

use clap::{Parser, Subcommand};

/// A program that demonstrates the TEEC_Session pool in both threading and
/// async scenarios.
#[derive(Parser)]
#[command(version, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Test an r2d2 connection pool with each task running in its own thread.
    #[command(long_about)]
    Thread(pool::Args),
    /// Test a mobc connection pool with each task running in a Tokio async
    /// task.
    #[command(long_about)]
    Async(pool::Args),
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Thread(args) => pool::r2d2_pool::run(args),
        Commands::Async(args) => pool::mobc_pool::run(args),
    }
}
