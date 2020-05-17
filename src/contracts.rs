/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and
 * limitations under the License.
 */

use crate::{Ed25519KeyPair, Ed25519Public, TonAddress};
use crate::error::*;
use serde_json::Value;
use crate::interop::{InteropContext, Interop};

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfDeploy {
    pub abi: serde_json::Value,
    pub constructor_header: Option<serde_json::Value>,
    pub constructor_params: serde_json::Value,
    pub init_params: Option<serde_json::Value>,
    pub image_base64: String,
    pub key_pair: Ed25519KeyPair,
    pub workchain_id: i32,
    pub try_index: Option<u8>,
}

/// Result of `deploy` function running. Contains address of the contract
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResultOfDeploy {
    pub address: TonAddress,
    pub already_deployed: bool, 
}

/// Result of `create_deploy_message` function. Contains message and future address of the contract
#[derive(Debug, PartialEq)]
pub struct ResultOfCreateDeployMessage {
    pub address: TonAddress,
    pub message: EncodedMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct ResultOfCreateDeployMessageCore {
    pub address: TonAddress,
    #[serde(flatten)]
    pub message: EncodedMessageCore,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfGetDeployData {
    pub abi: Option<serde_json::Value>,
    pub image_base64: Option<String>,
    pub init_params: Option<serde_json::Value>,
    pub public_key_hex: Ed25519Public,
    pub workchain_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResultOfGetDeployDataCore {
    pub image_base64: Option<String>,
    pub address: Option<String>,
    pub data_base64: String,
}

#[derive(Debug, PartialEq)]
/// Result of `get_deploy_data` function call. Contains updated contract image, deploy address and
/// stored data
pub struct ResultOfGetDeployData {
    pub image: Option<Vec<u8>>,
    pub address: Option<TonAddress>,
    pub data: Vec<u8>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfRun {
    pub address: TonAddress,
    pub abi: serde_json::Value,
    pub function_name: String,
    pub header: Option<serde_json::Value>,
    pub input: serde_json::Value,
    pub key_pair: Option<Ed25519KeyPair>,
    pub try_index: Option<u8>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfLocalRun {
    pub address: TonAddress,
    pub account: Option<serde_json::Value>,
    pub abi: serde_json::Value,
    pub function_name: String,
    pub header: Option<serde_json::Value>,
    pub input: serde_json::Value,
    pub key_pair: Option<Ed25519KeyPair>,
}

/// Result of `run` function running. Contains parameters returned by contract function
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResultOfRun {
    pub output: Value
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EncodedMessageCore {
    pub message_id: String,
    pub message_body_base64: String,
    pub expire: Option<u32>,
}

/// Message ready for sending to node
#[derive(Debug, PartialEq)]
pub struct EncodedMessage {
    pub message_id: String,
    pub message_body: Vec<u8>,
    pub expire: Option<u32>,
}

impl Into<EncodedMessageCore> for EncodedMessage {
    fn into(self) -> EncodedMessageCore {
        EncodedMessageCore {
            message_id: self.message_id,
            message_body_base64: base64::encode(&self.message_body),
            expire: self.expire
        }
    }
}

impl EncodedMessage {
    pub(crate) fn from_core(core_message: EncodedMessageCore) -> TonResult<Self> {
        Ok(EncodedMessage {
            message_id: core_message.message_id,
            message_body: base64::decode(&core_message.message_body_base64)?,
            expire: core_message.expire
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfDecodeMessageBody {
    pub abi: serde_json::Value,
    pub body_base64: String,
    pub internal: bool,
}

/// Result of `decode_input_message_body` and `decode_output_message_body` functions calls.
/// Contains contract function name and decoded parameters
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResultOfDecodeMessageBody {
    pub function: String,
    pub output: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfProcessMessage{
    pub abi: Option<serde_json::Value>,
    pub function_name: Option<String>,
    pub message: EncodedMessageCore,
    pub try_index: Option<u8>,
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
        abi: &str,
        image: &[u8],
        init_params: Option<RunParameters>,
        public_key: &Ed25519Public,
        workchain_id: i32,
    ) -> TonResult<TonAddress> {
        self.get_deploy_data(Some(abi), Some(image), init_params, public_key, Some(workchain_id))?
            .address
            .ok_or(TonErrorKind::InternalError("No address in result".to_owned()).into())
    }

    /// Get contract deploy data: image (state init), storage data and deploying address
    pub fn get_deploy_data(
        &self,
        abi: Option<&str>,
        image: Option<&[u8]>,
        init_params: Option<RunParameters>,
        public_key: &Ed25519Public,
        workchain_id: Option<i32>,
    ) -> TonResult<ResultOfGetDeployData> {
        let abi = abi.map(|val| {
            serde_json::from_str(val).map_err(|_| TonErrorKind::InvalidArg(val.to_owned()))
        }).transpose()?;

        let core_result: ResultOfGetDeployDataCore = Interop::json_request(
            self.context,
            "contracts.deploy.data",
            ParamsOfGetDeployData {
                abi,
                image_base64: image.map(|val| base64::encode(val)),
                init_params: Self::option_params_to_value(init_params)?,
                public_key_hex: public_key.clone(),
                workchain_id: workchain_id,
            })?;

        Ok(ResultOfGetDeployData {
            address: core_result.address.map(|val| TonAddress::from_str(&val)).transpose()?,
            image: core_result.image_base64.map(|val| base64::decode(&val).into()).transpose()?,
            data: base64::decode(&core_result.data_base64)?
        })
    }

    fn params_to_value(params: RunParameters) -> TonResult<Value> {
        let str_params = match &params {
            RunParameters::Json(string) => string
        };
       serde_json::from_str(str_params)
            .map_err(|_| TonErrorKind::InvalidArg(str_params.to_owned()).into())
    }

    fn option_params_to_value(params: Option<RunParameters>) -> TonResult<Option<Value>> {
        params.map(|params|  Self::params_to_value(params)).transpose()
    }

    /// Deploy contract to TON blockchain
    pub fn deploy(
        &self,
        abi: &str,
        code: &[u8],
        constructor_header: Option<RunParameters>,
        constructor_params: RunParameters,
        init_params: Option<RunParameters>,
        keys: &Ed25519KeyPair,
        workchain_id: i32,
    ) -> TonResult<ResultOfDeploy> {
        let abi = serde_json::from_str(abi)
            .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        Interop::json_request(self.context, "contracts.deploy", ParamsOfDeploy {
            abi,
            init_params: Self::option_params_to_value(init_params)?,
            constructor_header: Self::option_params_to_value(constructor_header)?,
            constructor_params: Self::params_to_value(constructor_params)?,
            image_base64: base64::encode(code),
            key_pair: keys.clone(),
            workchain_id: workchain_id,
            try_index: None
        })
    }

    /// Run the contract function with given parameters
    pub fn run(
        &self,
        address: &TonAddress,
        abi: &str,
        function_name: &str,
        header: Option<RunParameters>,
        input: RunParameters,
        keys: Option<&Ed25519KeyPair>,
    ) -> TonResult<Value> {
        let abi = serde_json::from_str(abi)
            .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        Interop::json_request(self.context, "contracts.run", ParamsOfRun {
            address: address.clone(),
            abi,
            function_name: function_name.to_string(),
            header: Self::option_params_to_value(header)?,
            input: Self::params_to_value(input)?,
            key_pair: keys.cloned(),
            try_index: None
        })
    }

    /// Run the contract function with given parameters locally
    pub fn run_local(
        &self,
        address: &TonAddress,
        account: Option<&str>,
        abi: &str,
        function_name: &str,
        header: Option<RunParameters>,
        input: RunParameters,
        keys: Option<&Ed25519KeyPair>,
    ) -> TonResult<Value> {
        let abi = serde_json::from_str(abi)
           .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        let account = account.map(|acc_str| {
                serde_json::from_str(acc_str)
                    .map_err(|_| TonErrorKind::InvalidArg(acc_str.to_owned()))
        }).transpose()?;

        Interop::json_request(self.context, "contracts.run.local", ParamsOfLocalRun {
            address: address.clone(),
            account,
            abi,
            function_name: function_name.to_string(),
            header: Self::option_params_to_value(header)?,
            input: Self::params_to_value(input)?,
            key_pair: keys.cloned(),
        })
    }

    /// Decodes external inbound message body with encoded contract call parameters
    pub fn decode_input_message_body(
        &self,
        abi: &str,
        body: &[u8],
        internal: bool,
    ) -> TonResult<ResultOfDecodeMessageBody> {
        let abi = serde_json::from_str(abi)
           .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        Interop::json_request(
            self.context,
            "contracts.run.unknown.input",
            ParamsOfDecodeMessageBody {
                abi,
                body_base64: base64::encode(body),
                internal,
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
                body_base64: base64::encode(body),
                internal: false,
        })
    }

    /// Create message to run the contract function with given parameters
    pub fn create_run_message(
        &self,
        address: &TonAddress,
        abi: &str,
        function_name: &str,
        header: Option<RunParameters>,
        input: RunParameters,
        keys: Option<&Ed25519KeyPair>,
        try_index: Option<u8>
    ) -> TonResult<EncodedMessage> {
        let abi = serde_json::from_str(abi)
            .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        let result: EncodedMessageCore = Interop::json_request(self.context, "contracts.run.message", ParamsOfRun {
            address: address.clone(),
            abi,
            function_name: function_name.to_string(),
            header: Self::option_params_to_value(header)?,
            input: Self::params_to_value(input)?,
            key_pair: keys.cloned(),
            try_index
        })?;

        EncodedMessage::from_core(result)
    }

    /// Create message to deploy contract
    pub fn create_deploy_message(
        &self,
        abi: &str,
        code: &[u8],
        constructor_header: Option<RunParameters>,
        constructor_params: RunParameters,
        init_params: Option<RunParameters>,
        keys: &Ed25519KeyPair,
        workchain_id: i32,
        try_index: Option<u8>
    ) -> TonResult<ResultOfCreateDeployMessage> {
        let abi = serde_json::from_str(abi)
            .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))?;

        let result: ResultOfCreateDeployMessageCore = Interop::json_request(
            self.context,
            "contracts.deploy.message",
            ParamsOfDeploy {
                abi,
                init_params: Self::option_params_to_value(init_params)?,
                constructor_header: Self::option_params_to_value(constructor_header)?,
                constructor_params: Self::params_to_value(constructor_params)?,
                image_base64: base64::encode(code),
                key_pair: keys.clone(),
                workchain_id: workchain_id,
                try_index
        })?;

        Ok(ResultOfCreateDeployMessage {
            address: result.address,
            message: EncodedMessage::from_core(result.message)?
        })
    }

    /// Send message to node without waiting for processing result
    pub fn send_message(&self, message: EncodedMessage) -> TonResult<()> {
        Interop::json_request::<EncodedMessageCore, ()>(
            self.context,
            "contracts.send.message",
            message.into()
        )
    }

    /// Send message to waiting for processing result and (optionally) parse result
    pub fn process_message(
        &self,
        message: EncodedMessage,
        abi: Option<&str>,
        function_name: Option<&str>,
        try_index: Option<u8>
    ) -> TonResult<ResultOfRun> {
        let abi = abi.map(|abi| serde_json::from_str(abi)
                .map_err(|_| TonErrorKind::InvalidArg(abi.to_owned()))
        ).transpose()?;

        Interop::json_request(
            self.context,
            "contracts.process.message",
            ParamsOfProcessMessage {
                abi,
                function_name: function_name.map(|val| val.to_owned()),
                try_index,
                message: message.into()
            }
        )
    }
}
