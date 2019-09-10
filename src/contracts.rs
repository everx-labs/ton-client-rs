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

use crate::{Ed25519KeyPair, TonResult, TonError, TonAddress};
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

/// Result of `run` function running. Contains parameters returned by contract function
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ResultOfRun {
    pub output: Value
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
        constructor_params: Value,
        keys: &Ed25519KeyPair,
    ) -> TonResult<TonAddress> {
        let abi = serde_json::from_str(abi)
            .map_err(|_|TonError::invalid_params("deploy"))?;
        let result: ResultOfDeploy = Interop::json_request(self.context, "contracts.deploy", ParamsOfDeploy {
            abi,
            constructorParams: constructor_params,
            imageBase64: base64::encode(code),
            keyPair: keys.clone(),
        })?;
        Ok(result.address)
    }

    /// Run the contract function with given parameters
    pub fn run(
        &self,
        address: &TonAddress,
        abi: &str,
        function_name: &str,
        input: Value,
        keys: Option<&Ed25519KeyPair>,
    ) -> TonResult<Value> {
        let abi = serde_json::from_str(abi)
            .map_err(|_|TonError::invalid_params("run"))?;
        let result: ResultOfRun = Interop::json_request(self.context, "contracts.run", ParamsOfRun {
            address: address.clone(),
            abi,
            functionName: function_name.to_string(),
            input,
            keyPair: if let Some(keys) = keys { Some(keys.clone()) } else { None },
        })?;
        Ok(result.output)
    }
}
