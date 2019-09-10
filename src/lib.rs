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

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate base64;


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

//mod queries;
//pub use queries::*;

#[derive(Debug, Deserialize)]
pub struct TonError {
    source: String,
    code: u32,
    message: String,
}



impl TonError {

    fn sdk(code: u32, message: &str) -> Self {
        Self {
            source: "sdk".to_string(),
            code,
            message: message.to_string(),
        }
    }

    pub fn invalid_params(method_name: &str) -> Self {
        Self::sdk(1, &format!("Can not serialize params for {}", method_name))
    }

    pub fn invalid_response_result(method_name: &str, result_json: &String) -> Self {
        Self::sdk(2, &format!("Can not deserialize result for {}\nresult JSON: {}", method_name, result_json))
    }

    pub fn invalid_response_error(method_name: &str, error_json: &String) -> Self {
        Self::sdk(3, &format!("Can not deserialize error for {}\nerror JSON: {}", method_name, error_json))
    }
}

pub type TonResult<R> = Result<R, TonError>;

