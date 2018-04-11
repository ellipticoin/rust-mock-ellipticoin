extern crate cask;
extern crate wasm_rpc;
extern crate ellipticoin;
use std::fs;
const TEST_DB_PATH: &str = "tmp/test.db";
use ellipticoin::BlockChain;
use wasm_rpc::*;
use self::cask::{CaskOptions, SyncStrategy, Cask};
use std::path::Path;

pub const SENDER: [u8; 20] = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ];
pub const ALICE: [u8; 20] = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2 ];
pub struct FakeBlockChain {
    pub db: Cask,
    pub sender: Vec<u8>,
}

impl BlockChain for FakeBlockChain {
    fn read(&self, key: Vec<u8>) -> Vec<u8> {
        match self.db.get(key).unwrap() {
            Some(x) => x,
            None => vec![],
        }
    }

    fn write(&self, key: Vec<u8>, value: Vec<u8>) {
        self.db
            .put(key, value)
            .expect("could not put value");
    }

    fn sender(&self) -> Vec<u8>{
        self.sender.to_vec()
    }

    fn call(&self, _code: Vec<u8>, _method: String, _params: Vec<u8>, _storage_context: Vec<u8>) -> Vec<u8> {
        unreachable!();
    }
}

impl Default for FakeBlockChain {
    fn default() -> FakeBlockChain {
        if Path::new(TEST_DB_PATH).is_dir() {
            fs::remove_dir_all(TEST_DB_PATH).expect("could not remove test db");
        }
        let cask = CaskOptions::default()
            .compaction_check_frequency(1200)
            .sync(SyncStrategy::Interval(5000))
            .max_file_size(1024 * 1024 * 1024)
            .open(TEST_DB_PATH).unwrap();
        FakeBlockChain {
            db: cask,
            sender: SENDER.to_vec(),
        }
    }
}
