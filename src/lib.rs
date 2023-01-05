#![no_std]

#[cfg(not(feature = "binary-paths"))]
mod contract;

#[cfg(feature = "binary-paths")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
