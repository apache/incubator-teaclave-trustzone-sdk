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

mod cli;

use optee_teec::{Context, Operation, ParamType, Uuid};
use optee_teec::{ParamNone, ParamTmpRef, ParamValue};

use anyhow::{bail, Result};
use structopt::StructOpt;

const OUTPUT_MAX_SIZE: usize = 1024;

fn invoke_command(command: proto::Command, input: &[u8]) -> optee_teec::Result<Vec<u8>> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(proto::UUID)
        .map_err(|_| optee_teec::Error::new(optee_teec::ErrorKind::ItemNotFound))?;
    let mut session = ctx.open_session(uuid)?;

    println!("CA: command: {:?}", command);
    // input buffer
    let p0 = ParamTmpRef::new_input(input);
    // output buffer
    let mut output = vec![0u8; OUTPUT_MAX_SIZE];
    let p1 = ParamTmpRef::new_output(output.as_mut_slice());
    // output buffer size
    let p2 = ParamValue::new(0, 0, ParamType::ValueInout);

    let mut operation = Operation::new(0, p0, p1, p2, ParamNone);
    match session.invoke_command(command as u32, &mut operation) {
        Ok(()) => {
            println!("CA: invoke_command success");
            let output_len = operation.parameters().2.a() as usize;
            Ok(output[..output_len].to_vec())
        }
        Err(e) => {
            let output_len = operation.parameters().2.a() as usize;
            let err_message = String::from_utf8_lossy(&output[..output_len]);
            println!("CA: invoke_command failed: {:?}", err_message);
            Err(e)
        }
    }
}

fn main() -> Result<()> {
    let args = cli::Opt::from_args();
    match args.command {
        cli::Command::CreateWallet(_opt) => {
            let serialized_output = invoke_command(proto::Command::CreateWallet, &[])?;
            let output: proto::CreateWalletOutput = bincode::deserialize(&serialized_output)?;
            println!("Wallet ID: {:?}", output.wallet_id);
        }
        cli::Command::RemoveWallet(opt) => {
            let input = proto::RemoveWalletInput {
                wallet_id: opt.wallet_id,
            };
            let _output =
                invoke_command(proto::Command::RemoveWallet, &bincode::serialize(&input)?)?;
            println!("Wallet removed");
        }
        cli::Command::DeriveAddress(opt) => {
            let input = proto::DeriveAddressInput {
                wallet_id: opt.wallet_id,
                hd_path: opt.hd_path,
            };
            let serialized_output =
                invoke_command(proto::Command::DeriveAddress, &bincode::serialize(&input)?)?;
            let output: proto::DeriveAddressOutput = bincode::deserialize(&serialized_output)?;
            println!("Address: 0x{:?}", hex::encode(&output.address));
            println!("Public key: {:?}", hex::encode(&output.public_key));
        }
        cli::Command::SignTransaction(opt) => {
            let transaction = proto::EthTransaction {
                chain_id: opt.chain_id,
                nonce: opt.nonce,
                to: Some(opt.to),
                value: opt.value,
                gas_price: opt.gas_price,
                gas: opt.gas,
                data: vec![],
            };
            let input = proto::SignTransactionInput {
                wallet_id: opt.wallet_id,
                hd_path: opt.hd_path,
                transaction,
            };
            let serialized_output = invoke_command(
                proto::Command::SignTransaction,
                &bincode::serialize(&input)?,
            )?;
            let output: proto::SignTransactionOutput = bincode::deserialize(&serialized_output)?;
            println!("Signature: {:?}", hex::encode(&output.signature));
        }
        _ => {
            bail!("Unsupported command");
        }
    }
    Ok(())
}
