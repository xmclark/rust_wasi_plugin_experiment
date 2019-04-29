#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Foo {
    pub foo: String,
}

pub fn deserialize<'i, D>(slice: &'i [u8]) -> D
    where D: Deserialize<'i> {
    match bincode::deserialize(slice) {
        Ok(ret) => ret,
        Err(e) => {
            panic!("error deserializing {}", e);
        },
    }
}

pub fn serialize<S>(ret: S) -> Vec<u8>
    where S: Serialize {
    match bincode::serialize(&ret) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("error serializing {}", e)
        }
    }
}
