extern crate ellipticoin;
extern crate wasm_rpc;
extern crate wasm_rpc_macros;

use std::cell::RefCell;
use std::collections::BTreeMap;
pub use wasm_rpc::{Bytes, FromBytes, ToBytes, Value};
pub use wasm_rpc_macros::export;

thread_local!(static SENDER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static BLOCK_WINNER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static BLOCK_NUMBER: RefCell<u64> = RefCell::new(0));
thread_local!(static MEMORY: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new()));
thread_local!(static STORAGE: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new()));

pub mod error;

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

pub fn sender() -> Vec<u8> {
    SENDER.with(|sender_cell| sender_cell.borrow().to_vec())
}

pub fn block_winner() -> Vec<u8> {
    BLOCK_WINNER.with(|block_winner_cell| block_winner_cell.borrow().to_vec())
}

pub fn block_number() -> u64 {
    BLOCK_NUMBER.with(|block_number_cell| *block_number_cell.borrow())
}

pub fn call(
    _code: Vec<u8>,
    _method: String,
    _params: Vec<u8>,
    _storage_context: Vec<u8>,
) -> Vec<u8> {
    unreachable!();
}
