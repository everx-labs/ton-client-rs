use crate::{Ed25519KeySource, TonResult, Ed25519Public, KeyPair, TonError};
use serde_json::Value;
use crate::interop::{InteropContext, Interop};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct ParamsOfDeploy {
    pub abi: serde_json::Value,
    pub constructorParams: serde_json::Value,
    pub imageBase64: String,
    pub keyPair: KeyPair,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ResultOfDeploy {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ParamsOfRun {
    pub address: String,
    pub abi: serde_json::Value,
    pub functionName: String,
    pub input: serde_json::Value,
    pub keyPair: Option<KeyPair>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ResultOfRun {
    pub output: Value
}

pub struct TonContracts {
    context: InteropContext,
}

impl TonContracts {
    pub(crate) fn new(context: InteropContext) -> Self {
        Self { context }
    }

    pub fn deploy(
        &self,
        abi: &str,
        code_base64: &str,
        constructor_params: Value,
        keys: &KeyPair,
    ) -> TonResult<String> {
        let abi = serde_json::from_str(abi)
            .map_err(|err|TonError::invalid_params("deploy"))?;
        let result: ResultOfDeploy = Interop::json_request(self.context, "contracts.deploy", ParamsOfDeploy {
            abi,
            constructorParams: constructor_params,
            imageBase64: code_base64.to_string(),
            keyPair: keys.clone(),
        })?;
        Ok(result.address)
    }

    pub fn run(
        &self,
        address: &str,
        abi: &str,
        function_name: &str,
        input: Value,
        keys: Option<&KeyPair>,
    ) -> TonResult<Value> {
        let abi = serde_json::from_str(abi)
            .map_err(|err|TonError::invalid_params("deploy"))?;
        let result: ResultOfRun = Interop::json_request(self.context, "contracts.run", ParamsOfRun {
            address: address.to_string(),
            abi,
            functionName: function_name.to_string(),
            input,
            keyPair: if let Some(keys) = keys { Some(keys.clone()) } else { None },
        })?;
        Ok(result.output)
    }
}
