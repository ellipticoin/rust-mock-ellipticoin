extern crate ellipticoin;
extern crate wasm_rpc;
extern crate wasm_rpc_macros;

use std::cell::RefCell;
use std::collections::HashMap;
pub use wasm_rpc::{abort, Bytes, FromBytes, ToBytes, Value, error, BTreeMap, value};
pub use wasm_rpc_macros::export;

thread_local!(static CALL_MOCKS: RefCell<HashMap<(Vec<u8>, &'static str),  &'static dyn Fn(Vec<Value>) -> (u32, Value)>> = RefCell::new(HashMap::new()));
thread_local!(static CONTRACT_ADDRESS: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static CALLER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static SENDER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static BLOCK_WINNER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static BLOCK_NUMBER: RefCell<u64> = RefCell::new(0));
thread_local!(static MEMORY: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new()));
thread_local!(static STORAGE: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new()));

pub fn set_contract_address(contract_address: Vec<u8>) {
    CONTRACT_ADDRESS.with(|contract_address_cell| contract_address_cell.replace(contract_address));
}
pub fn set_caller(caller: Vec<u8>) {
    CALLER.with(|caller_cell| caller_cell.replace(caller));
}


pub fn set_sender(sender: Vec<u8>) {
    SENDER.with(|sender_cell| sender_cell.replace(sender));
}

pub fn set_block_winner(block_winner: Vec<u8>) {
    BLOCK_WINNER.with(|block_winner_cell| block_winner_cell.replace(block_winner));
}

pub fn set_block_number(block_number: u64) {
    BLOCK_NUMBER.with(|block_number_cell| block_number_cell.replace(block_number));
}

pub fn get_memory<K: ToBytes, V: FromBytes>(key: K) -> V {
    let v: Vec<u8> = MEMORY.with(|state_cell| {
        let state = state_cell.borrow_mut();
        match state.get::<Vec<u8>>(&key.to_bytes()) {
            Some(value) => value.to_vec(),
            None => vec![],
        }
    });
    FromBytes::from_bytes(v)
}

pub fn set_memory<K: ToBytes, V: ToBytes>(key: K, value: V) {
    MEMORY.with(|state_cell| {
        let mut state = state_cell.borrow_mut();
        state.insert(key.to_bytes(), value.to_bytes());
    })
}

pub fn get_storage<K: ToBytes, V: FromBytes>(key: K) -> V {
    let v: Vec<u8> = STORAGE.with(|state_cell| {
        let state = state_cell.borrow_mut();
        match state.get::<Vec<u8>>(&key.to_bytes()) {
            Some(value) => value.to_vec(),
            None => vec![],
        }
    });
    FromBytes::from_bytes(v)
}

pub fn set_storage<K: ToBytes, V: ToBytes>(key: K, value: V) {
    STORAGE.with(|state_cell| {
        let mut state = state_cell.borrow_mut();
        state.insert(key.to_bytes(), value.to_bytes());
    })
}

pub fn contract_address() -> Vec<u8> {
    CONTRACT_ADDRESS.with(|contract_address_cell| contract_address_cell.borrow().to_vec())
}

pub fn caller() -> Vec<u8> {
    CALLER.with(|caller_cell| caller_cell.borrow().to_vec())
}

pub fn sender() -> Vec<u8> {
    SENDER.with(|sender_cell| sender_cell.borrow().to_vec())
}

pub fn block_winner() -> Vec<u8> {
    BLOCK_WINNER.with(|block_winner_cell| block_winner_cell.borrow().to_vec())
}

pub fn block_number() -> u64 {
    BLOCK_NUMBER.with(|block_number_cell| *block_number_cell.borrow())
}

pub fn set_mock_call(
    contract_address: Vec<u8>,
    function_name: &'static str,
    function: &'static dyn Fn(Vec<Value>) -> (u32, Value)
    ) {
    CALL_MOCKS.with(|state_cell| {
        let mut state = state_cell.borrow_mut();
        state.insert((contract_address, function_name), function);
    })
}

pub fn call(
    contract_address: Vec<u8>,
    function: &str,
    arguments: Vec<Value>,
) -> (u32, Value) {
    CALL_MOCKS.with(|call_mocks_cell| {
        let call_mocks = &*call_mocks_cell.borrow();
        let f = call_mocks.get(&(contract_address, function)).expect(&format!("{} not found", function));
        f(arguments)
    })
}
