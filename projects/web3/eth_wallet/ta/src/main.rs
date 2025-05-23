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

#![no_main]

mod hash;
mod wallet;

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters};
use proto::Command;
use secure_db::SecureStorageClient;

use anyhow::{anyhow, bail, Result};
use std::io::Write;
use wallet::Wallet;

const DB_NAME: &str = "eth_wallet_db";

/// Represents the session context for the Ethereum wallet Trusted Application (TA).
///
/// The `WalletSession` struct manages session-specific data and provides access
/// to the secure storage client (`SecureStorageClient`) for database operations.
pub struct WalletSession {
    db_client: SecureStorageClient,
}

impl WalletSession {
    pub fn new() -> Result<Self> {
        let db_client = SecureStorageClient::open(DB_NAME)
            .map_err(|e| anyhow!("Failed to create SecureStorageClient: {:?}", e))?;
        Ok(Self { db_client })
    }
}

impl Default for WalletSession {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            trace_println!("Error initializing WalletSession: {:?}", e);
            panic!("Failed to initialize WalletSession");
        })
    }
}

#[ta_create]
fn create() -> optee_utee::Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
// The _sess_ctx: &mut WalletSession is explicitly defined here to fit into the
// macros definition (optee-utee/macros/src/lib.rs), meaning that we have a
// context for the session which would be initialized in the open_session() method.
// The context is initialized automatically by calling WalletSession::default()
// in the macro.
fn open_session(_params: &mut Parameters, _sess_ctx: &mut WalletSession) -> optee_utee::Result<()> {
    trace_println!("[+] TA open session");

    Ok(())
}

#[ta_close_session]
fn close_session(_sess_ctx: &mut WalletSession) {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[cfg(debug_assertions)]
macro_rules! dbg_println {
    ($($arg:tt)*) => (trace_println!($($arg)*));
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_println {
    ($($arg:tt)*) => {};
}

fn create_wallet(
    db_client: &SecureStorageClient,
    _input: &proto::CreateWalletInput,
) -> Result<proto::CreateWalletOutput> {
    let wallet = Wallet::new()?;
    let wallet_id = wallet.get_id();
    let mnemonic = wallet.get_mnemonic()?;
    dbg_println!("[+] Wallet ID: {:?}", wallet_id);

    db_client.put(&wallet)?;
    dbg_println!("[+] Wallet saved in secure storage");

    Ok(proto::CreateWalletOutput {
        wallet_id,
        mnemonic,
    })
}

fn remove_wallet(
    db_client: &SecureStorageClient,
    input: &proto::RemoveWalletInput,
) -> Result<proto::RemoveWalletOutput> {
    dbg_println!("[+] Removing wallet: {:?}", input.wallet_id);

    db_client.delete_entry::<Wallet>(&input.wallet_id)?;
    dbg_println!("[+] Wallet removed");

    Ok(proto::RemoveWalletOutput {})
}

fn derive_address(
    db_client: &SecureStorageClient,
    input: &proto::DeriveAddressInput,
) -> Result<proto::DeriveAddressOutput> {
    let wallet = db_client
        .get::<Wallet>(&input.wallet_id)
        .map_err(|e| anyhow!("[+] Deriving address: error: wallet not found: {:?}", e))?;
    dbg_println!("[+] Deriving address: wallet loaded");

    let (address, public_key) = wallet.derive_address(&input.hd_path)?;
    dbg_println!("[+] Deriving address: address: {:?}", address);
    dbg_println!("[+] Deriving address: public key: {:?}", public_key);

    Ok(proto::DeriveAddressOutput {
        address,
        public_key,
    })
}

fn sign_transaction(
    db_client: &SecureStorageClient,
    input: &proto::SignTransactionInput,
) -> Result<proto::SignTransactionOutput> {
    let wallet = db_client
        .get::<Wallet>(&input.wallet_id)
        .map_err(|e| anyhow!("[+] Sign transaction: error: wallet not found: {:?}", e))?;
    dbg_println!("[+] Sign transaction: wallet loaded");

    let signature = wallet.sign_transaction(&input.hd_path, &input.transaction)?;
    dbg_println!("[+] Sign transaction: signature: {:?}", signature);

    Ok(proto::SignTransactionOutput { signature })
}

fn handle_invoke(
    db_client: &SecureStorageClient,
    command: Command,
    serialized_input: &[u8],
) -> Result<Vec<u8>> {
    fn process<
        T: serde::de::DeserializeOwned,
        U: serde::Serialize,
        F: Fn(&SecureStorageClient, &T) -> Result<U>,
    >(
        db_client: &SecureStorageClient,
        serialized_input: &[u8],
        handler: F,
    ) -> Result<Vec<u8>> {
        let input: T = bincode::deserialize(serialized_input)?;
        let output = handler(db_client, &input)?;
        let serialized_output = bincode::serialize(&output)?;
        Ok(serialized_output)
    }

    match command {
        Command::CreateWallet => process(db_client, serialized_input, create_wallet),
        Command::RemoveWallet => process(db_client, serialized_input, remove_wallet),
        Command::DeriveAddress => process(db_client, serialized_input, derive_address),
        Command::SignTransaction => process(db_client, serialized_input, sign_transaction),
        _ => bail!("Unsupported command"),
    }
}

#[ta_invoke_command]
fn invoke_command(
    sess_ctx: &mut WalletSession,
    cmd_id: u32,
    params: &mut Parameters,
) -> optee_utee::Result<()> {
    dbg_println!("[+] TA invoke command");
    let mut p0 = unsafe { params.0.as_memref()? };
    let mut p1 = unsafe { params.1.as_memref()? };
    let mut p2 = unsafe { params.2.as_value()? };

    let output_vec = match handle_invoke(&sess_ctx.db_client, Command::from(cmd_id), p0.buffer()) {
        Ok(output) => output,
        Err(e) => {
            let err_message = format!("{:?}", e).as_bytes().to_vec();
            p1.buffer()
                .write(&err_message)
                .map_err(|_| Error::new(ErrorKind::BadState))?;
            p2.set_a(err_message.len() as u32);
            return Err(Error::new(ErrorKind::BadParameters));
        }
    };
    p1.buffer()
        .write(&output_vec)
        .map_err(|_| Error::new(ErrorKind::BadState))?;
    p2.set_a(output_vec.len() as u32);

    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
