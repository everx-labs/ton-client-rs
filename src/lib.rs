/*
 * Copyright 2018-2019 TON DEV SOLUTIONS LTD.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.  You may obtain a copy of the
 * License at: https://ton.dev/licenses
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and
 * limitations under the License.
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

