use anyhow::{Context, Result};
use std::{ops::Add, str::FromStr};

use revm::{
    db::{CacheDB, EmptyDB, EmptyDBTyped},
    primitives::{Address, BlockEnv, Bytes, EVMError, ExecutionResult, TransactTo, TxEnv, U256},
    Evm, InMemoryDB,
};

fn main() {
    let mut evm = create_evm();

    // TODO:
    evm.db().insert_account_info("0x1c70319052E9Cfc804E3a8F408C828768F0Fe40A", info)

    let example_transaction = TxEnv {
        caller: Address::from_str("0x1c70319052E9Cfc804E3a8F408C828768F0Fe40A").unwrap(),
        gas_limit: 999999999999,
        gas_price: U256::from(10),
        transact_to: TransactTo::Call(
            Address::from_str("0x961bdA3F1b384f3c1F8DBE26B5eF46bd5a9A80c3").unwrap(),
        ),
        value: U256::from(999_999),
        data: Bytes::default(),
        nonce: Some(0),
        // the chain id of out evm is 1 by default
        chain_id: Some(1),
        // TODO:
        access_list: vec![],
        // idc
        gas_priority_fee: None,
        // idc about blobs
        blob_hashes: vec![],
        max_fee_per_blob_gas: None,
    };

    let tx_result = process_tx(&mut evm, example_transaction).unwrap();

    match tx_result {
        ExecutionResult::Success {
            reason,
            gas_used,
            gas_refunded,
            logs,
            output,
        } => {
            println!("ok! <3 Yay!")
        }
        ExecutionResult::Halt { reason, gas_used } => {
            println!("ok")
        }
        ExecutionResult::Revert { gas_used, output } => {
            println!("naughty")
        }
    }

    // let chain_id = evm.cfg().chain_id;
    // println!("chain_id {}", chain_id);

    // let block = evm.block();
    // println!("original block num: {}", block.number);

    // increment_block(&mut evm);
    // let block = evm.block();
    // println!("new block num: {}", block.number);
}

// static lifetime is okay for this example because we want evm to live the entire duration
// of our program (execution of main)
fn create_evm() -> Evm<'static, (), InMemoryDB> {
    // use a db type that holds all state changes in memory
    let db = InMemoryDB::new(EmptyDB::new());
    // create our evm instance with this db
    let evm_builder = Evm::builder().with_db(db);
    evm_builder.build()
}

fn process_tx(evm: &mut Evm<'_, (), InMemoryDB>, tx: TxEnv) -> Result<ExecutionResult> {
    *evm.tx_mut() = tx;

    evm.transact_commit().context("commit transaction to evm")
}

// mutates current evm state in place
fn increment_block<'a>(evm: &mut Evm<'_, (), InMemoryDB>) {
    let old_block = evm.block();

    // change whatever fields you want
    let new_block = BlockEnv {
        number: old_block.number + U256::from(1),
        // have to clone because not all fields implement copy.
        // This clone is probably fine
        ..old_block.clone()
    };

    // set the evm's block to the new block
    *evm.block_mut() = new_block;
}
