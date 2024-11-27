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

use anyhow::{bail, Result};
use structopt::StructOpt;

// decode hex string to [u8; 20]
pub fn decode_hex_to_address(src: &str) -> Result<[u8; 20]> {
    // strip the 0x prefix
    let src = src.trim_start_matches("0x");
    let vec = hex::decode(src)?;
    if vec.len() < 20 {
        bail!("invalid address length: {}", vec.len());
    }
    let mut array = [0u8; 20];
    array.copy_from_slice(&vec[..20]);
    Ok(array)
}

// decode string to uuid
pub fn decode_str_to_uuid(s: &str) -> Result<uuid::Uuid> {
    uuid::Uuid::parse_str(s).map_err(|e| e.into())
}

#[derive(Debug, StructOpt)]
pub struct CreateWalletOpt {}

#[derive(Debug, StructOpt)]
pub struct RemoveWalletOpt {
    #[structopt(short, long, required = true)]
    pub wallet_id: uuid::Uuid,
}

#[derive(Debug, StructOpt)]
pub struct DeriveAddressOpt {
    #[structopt(short, long, required = true)]
    pub wallet_id: uuid::Uuid,
    #[structopt(short, long, required = true, default_value = "m/44'/60'/0'/0/0")]
    pub hd_path: String,
}

#[derive(Debug, StructOpt)]
pub struct SignTransactionOpt {
    #[structopt(short, long, required = true, parse(try_from_str = decode_str_to_uuid))]
    pub wallet_id: uuid::Uuid,
    #[structopt(short, long, default_value = "m/44'/60'/0'/0/0")]
    pub hd_path: String,
    #[structopt(short, long, default_value = "5")]
    pub chain_id: u64,
    #[structopt(short, long, default_value = "0")]
    pub nonce: u128,
    #[structopt(short, long, required = true, parse(try_from_str = decode_hex_to_address))]
    pub to: [u8; 20],
    #[structopt(short, long, required = true)]
    pub value: u128,
    #[structopt(short = "p", long, default_value = "1000000000")]
    pub gas_price: u128,
    #[structopt(short, long, default_value = "21000")]
    pub gas: u128,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Create a new wallet.
    #[structopt(name = "create-wallet")]
    CreateWallet(CreateWalletOpt),
    /// Remove a wallet.
    #[structopt(name = "remove-wallet")]
    RemoveWallet(RemoveWalletOpt),
    /// Derive an address from a wallet.
    #[structopt(name = "derive-address")]
    DeriveAddress(DeriveAddressOpt),
    /// Sign a transaction.
    #[structopt(name = "sign-transaction")]
    SignTransaction(SignTransactionOpt),
    /// Run tests
    #[structopt(name = "test")]
    Test,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "eth_wallet", about = "A simple Ethereum wallet based on TEE")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command,
}
