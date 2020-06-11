/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

#![recursion_limit="128"] // needed for error_chain

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate base64;
#[macro_use]
extern crate error_chain;
extern crate crc16;


#[cfg(test)]
mod tests;

mod types;
pub use types::*;

mod interop;

mod client;
pub use client::*;

mod crypto;
pub use crypto::*;

mod contracts;
pub use contracts::*;

mod queries;
pub use queries::*;

mod error;
pub use error::*;

mod json_helper;
