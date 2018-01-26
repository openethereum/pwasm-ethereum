//! Externs library to interact with Ethereum-like network

#![cfg_attr(not(feature="std"), no_std)]

extern crate pwasm_std;
extern crate parity_hash as hash;
extern crate bigint;

mod ext;
mod storage;

pub use ext::*;
pub use storage::*;
