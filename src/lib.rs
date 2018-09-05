#![no_std]
#![feature(alloc)]
#[macro_use]
extern crate std;
extern crate alloc;
extern crate wasm_rpc;
extern crate ellipticoin;
extern crate secp256k1;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::btree_map::BTreeMap;
use self::secp256k1::{Secp256k1};
use wasm_rpc::*;
use wasm_rpc::{Bytes, Dereferenceable, Referenceable};
use std::cell::RefCell;
use core::intrinsics::transmute;

thread_local!(static SENDER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static BLOCK_WINNER: RefCell<Vec<u8>> = RefCell::new(Vec::new()));
thread_local!(static STATE: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new()));


pub fn set_sender(sender: Vec<u8>) {
    SENDER.with(|sender_cell|{
        sender_cell.replace(sender)
    });
}

pub fn set_block_winner(block_winner: Vec<u8>) {
    BLOCK_WINNER.with(|block_winner_cell|{
        block_winner_cell.replace(block_winner)
    });
}

pub fn read<K: Into<Vec<u8>>>(key: K) -> Vec<u8> {
    STATE.with(|state_cell|{
        let state = state_cell.borrow_mut();
        match state.get(&key.into()) {
            Some(value) => value.to_vec(),
            None => vec![],
        }
    })
}

pub fn write<K: Into<Vec<u8>>>(key: K, value: Vec<u8>) {
    STATE.with(|state_cell|{
        let mut state = state_cell.borrow_mut();
        state.insert(key.into(), value);
    })
}

pub fn update<K: Into<Vec<u8>>>(key: K, update_function: &Fn(Vec<u8>) -> Vec<u8>) {
    STATE.with(|state_cell|{
        let mut state = state_cell.borrow_mut();
        let key_vec = key.into();
        let value_before = match state.get(&key_vec.clone()) {
            Some(value) => value.to_vec(),
            None => vec![],
        };
        let value_after = update_function(value_before.into());
        state.insert(key_vec, value_after);
    })
}

pub fn sender() -> Vec<u8>{
    SENDER.with(|sender_cell|{
        sender_cell.borrow().to_vec()
    })
}

pub fn block_winner() -> Vec<u8>{
    BLOCK_WINNER.with(|block_winner_cell|{
        block_winner_cell.borrow().to_vec()
    })
}

pub fn read_u32<K: Into<Vec<u8>>>(key: K) -> u32 {
    read(key.into()).value()
}

pub fn read_u64<K: Into<Vec<u8>>>(key: K) -> u64 {
    read(key.into()).value()
}

pub fn write_u32<K: Into<Vec<u8>>>(key: K, value: u32) {
    write(key.into(), unsafe{transmute::<u32, [u8; 4]>(value).to_vec()});
}

pub fn write_u64<K: Into<Vec<u8>>>(key: K, value: u64) {
    write(key.into(), unsafe{transmute::<u64, [u8; 8]>(value).to_vec()});
}

pub fn write_int<K: Into<Vec<u8>>>(key: K, value: u64) {
    write(key.into(), unsafe{transmute::<u64, [u8; 8]>(value).to_vec()});
}

pub fn read_int<K: Into<Vec<u8>>>(key: K) -> u64 {
    read_u64(key.into())
}

pub fn secp256k1_recover(message_vec: Vec<u8>, signature_vec: Vec<u8>, recovery_id: u8) -> Vec<u8> {
    let s = Secp256k1::new();
    let message = secp256k1::Message::from_slice(&message_vec).unwrap();
    let signature = secp256k1::RecoverableSignature::from_compact(&s, &signature_vec, secp256k1::RecoveryId::from_i32(recovery_id as i32).unwrap()).unwrap();
    println!("{:?}", message);
    s.recover(&message, &signature).unwrap().serialize().to_vec()
}

pub fn call(_code: Vec<u8>, _method: String, _params: Vec<u8>, _storage_context: Vec<u8>) -> Vec<u8> {
    unreachable!();
}
