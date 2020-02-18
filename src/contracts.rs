/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
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

use crate::{Ed25519KeyPair, TonAddress};
use crate::error::*;
use serde_json::Value;
use crate::interop::{InteropContext, Interop};

#[derive(Serialize)]
#[allow(non_snake_case)]
pub(crate) struct ParamsOfDeploy {
    pub abi: serde_json::Value,
    pub constructorParams: serde_json::Value,
    pub imageBase64: String,
    pub keyPair: Ed25519KeyPair,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub(crate) struct ParamsOfGetDeployAddress {
    pub abi: serde_json::Value,
    pub imageBase64: String,
    pub keyPair: Ed25519KeyPair,
}

/// Result of `deploy` and `get_deploy_address` function running. Contains address of the contract
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ResultOfDeploy {
    pub address: TonAddress,
    pub alreadyDeployed: bool, 
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub(crate) struct ParamsOfRun {
    pub address: TonAddress,
    pub abi: serde_json::Value,
    pub functionName: String,
    pub input: serde_json::Value,
    pub keyPair: Option<Ed25519KeyPair>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub(crate) struct ParamsOfLocalRun {
    pub address: TonAddress,
    pub account: Option<serde_json::Value>,
    pub abi: serde_json::Value,
    pub functionName: String,
    pub input: serde_json::Value,
    pub keyPair: Option<Ed25519KeyPair>,
}

/// Result of `run` function running. Contains parameters returned by contract function
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ResultOfRun {
    pub output: Value
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ParamsOfDecodeMessageBody {
    pub abi: serde_json::Value,
    pub bodyBase64: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ResultOfDecodeMessageBody {
    pub function: String,
    pub output: serde_json::Value
}

/// Parameters to be passed into contract function
pub enum RunParameters {
     Json(String),
    // Values(...)
}

impl<T: Into<String>> From<T> for RunParameters {
    fn from(string: T) -> Self {
        RunParameters::Json(string.into())
    }
}

/// Contract management struct
pub struct TonContracts {
    context: InteropContext,
}

impl TonContracts {
    pub(crate) fn new(context: InteropContext) -> Self {
        Self { context }
    }

    /// Get address for contract deploying
    pub fn get_deploy_address(
        &self,
        code: &[u8],
        keys: &Ed25519KeyPair,
    ) -> TonResult<TonAddress> {
        let result: TonAddress = Interop::json_request(
            self.context,
            "contracts.deploy.address",
            ParamsOfGetDeployAddress {
                abi: Value::Null,
                imageBase64: base64::encode(code),
                keyPair: keys.clone(),
            })?;
        Ok(result)
    }

    /// Deploy contract to TON blockchain
    pub fn deploy(
        &self,
        abi: &str,
        code: &[u8],
        constructor_params: RunParameters,
        keys: &Ed25519KeyPair,
    ) -> TonResult<ResultOfDeploy> {
        let abi = serde_json::from_str(abi)
            .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        let str_params = match &constructor_params {
            RunParameters::Json(string) => string
        };
        let params_value = serde_json::from_str(str_params)
            .map_err(|_| TonErrorKind::InvalidArg(str_params.to_owned()))?;

        let result: ResultOfDeploy = Interop::json_request(self.context, "contracts.deploy", ParamsOfDeploy {
            abi,
            constructorParams: params_value,
            imageBase64: base64::encode(code),
            keyPair: keys.clone(),
        })?;
        Ok(result)
    }

    /// Run the contract function with given parameters
    pub fn run(
        &self,
        address: &TonAddress,
        abi: &str,
        function_name: &str,
        input: RunParameters,
        keys: Option<&Ed25519KeyPair>,
    ) -> TonResult<Value> {
        let abi = serde_json::from_str(abi)
            .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;
        
        let str_params = match &input {
            RunParameters::Json(string) => string
        };
        let params_value = serde_json::from_str(str_params)
            .map_err(|_| TonErrorKind::InvalidArg(str_params.to_owned()))?;

        let result: ResultOfRun = Interop::json_request(self.context, "contracts.run", ParamsOfRun {
            address: address.clone(),
            abi,
            functionName: function_name.to_string(),
            input: params_value,
            keyPair: if let Some(keys) = keys { Some(keys.clone()) } else { None },
        })?;
        Ok(result.output)
    }

    /// Run the contract function with given parameters locally
    pub fn run_local(
        &self,
        address: &TonAddress,
        account: Option<&str>,
        abi: &str,
        function_name: &str,
        input: RunParameters,
        keys: Option<&Ed25519KeyPair>,
    ) -> TonResult<Value> {
        let abi = serde_json::from_str(abi)
           .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;
        
        let str_params = match &input {
            RunParameters::Json(string) => string
        };
        let params_value = serde_json::from_str(str_params)
            .map_err(|_| TonErrorKind::InvalidArg(str_params.to_owned()))?;

        let account = match account {
            Some(acc_str) => {
                Some(serde_json::from_str(acc_str)
                   .map_err(|_| TonErrorKind::InvalidArg(acc_str.to_owned()))?)
            },
            None => None
        };

        let result: ResultOfRun = Interop::json_request(self.context, "contracts.run.local", ParamsOfLocalRun {
            address: address.clone(),
            account,
            abi,
            functionName: function_name.to_string(),
            input: params_value,
            keyPair: if let Some(keys) = keys { Some(keys.clone()) } else { None },
        })?;
        Ok(result.output)
    }

    /// Decodes external inbound message body with encoded contract call parameters
    pub fn decode_input_message_body(
        &self,
        abi: &str,
        body: &[u8]
    ) -> TonResult<ResultOfDecodeMessageBody> {
        let abi = serde_json::from_str(abi)
           .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        Interop::json_request(
            self.context,
            "contracts.run.unknown.input",
            ParamsOfDecodeMessageBody {
                abi,
                bodyBase64: base64::encode(body)
        })
    }

    /// Decode external outbound message body with encoded contract function response or event
    pub fn decode_output_message_body(
        &self,
        abi: &str,
        body: &[u8]
    ) -> TonResult<ResultOfDecodeMessageBody> {
        let abi = serde_json::from_str(abi)
           .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        Interop::json_request(
            self.context,
            "contracts.run.unknown.output",
            ParamsOfDecodeMessageBody {
                abi,
                bodyBase64: base64::encode(body)
        })
    }
}
