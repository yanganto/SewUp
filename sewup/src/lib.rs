//! Sewup - Second state eWasm Utility Program
//! A library to help you sew up your Ethereum project with Rust and just like develop in a common backend.
//!
//! Use the crate with different feature to use the high level api just enable the features you
//! want to use.
//! - KV feature helps you develop contract as key value database
//! ```toml
//! sewup = { version = "*", features = ['kv'] }
//! sewup-derive = { version = "*", features = ['kv']  }
//! ```
//! - RDB feature helps you develop contract as rdb database
//! ```toml
//! sewup = { version = "*", features = ['rdb'] }
//! sewup-derive = { version = "*", features = ['rdb']  }
//! ```

#[cfg(feature = "token")]
pub mod token;
#[cfg(feature = "token")]
pub use token::*;

#[cfg(feature = "kv")]
pub mod kv;
#[cfg(feature = "kv")]
pub use kv::*;

#[cfg(feature = "rdb")]
pub mod rdb;
#[cfg(feature = "rdb")]
pub use rdb::*;

pub mod errors;

#[allow(dead_code)]
#[cfg(not(test))]
pub mod primitives;

#[allow(dead_code)]
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod runtimes;

#[allow(unused_variables)]
pub mod types;

pub use bincode;

/// Re export the ewasm_api
/// these api is low level apis, it is better to keep in a library not in the contract file
#[cfg(target_arch = "wasm32")]
pub use ewasm_api;