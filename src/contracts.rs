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

use crate::{Ed25519KeyPair, Ed25519Public, JsonValue, TonAddress};
use crate::types::option_params_to_value;
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
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResultOfDeploy {
    pub address: TonAddress,
    pub already_deployed: bool,
    pub fees: Option<TransactionFees>,
    pub transaction: serde_json::Value,
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

/// Result of `get_deploy_data` function call. Contains updated contract image, deploy address and
/// stored data
#[derive(Deserialize, Debug, PartialEq)]
#[serde(try_from = "crate::json_helper::ResultOfGetDeployDataCore")]
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
    pub full_run: bool,
    pub time: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfLocalRunWithMsg {
    pub address: TonAddress,
    pub account: Option<serde_json::Value>,
    pub abi: Option<serde_json::Value>,
    pub function_name: Option<String>,
    pub message_base64: String,
    pub full_run: bool,
    pub time: Option<u32>,
}

/// Result of `run` function running. Contains parameters returned by contract function
#[derive(Deserialize, Debug, PartialEq)]
pub struct ResultOfRun {
    pub output: Value,
    pub fees: TransactionFees,
    pub transaction: serde_json::Value,
}

/// Result of `run` function running. Contains parameters returned by contract function
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResultOfLocalRun {
    pub output: Value,
    pub fees: Option<TransactionFees>,
    pub account: Option<serde_json::Value>
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(try_from = "crate::json_helper::TransactionFeesCore")]
pub struct TransactionFees {
    pub in_msg_fwd_fee: u64,
    pub storage_fee: u64,
    pub gas_fee: u64,
    pub out_msgs_fwd_fee: u64,
    pub total_account_fees: u64,
    pub total_output: u64
}

/// Message ready for sending to node
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(try_from = "crate::json_helper::EncodedMessageCore")]
#[serde(into = "crate::json_helper::EncodedMessageCore")]
pub struct EncodedMessage {
    pub message_id: String,
    pub message_body: Vec<u8>,
    pub expire: Option<u32>,
    pub address: TonAddress,
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
    pub message: EncodedMessage,
    pub infinite_wait: bool
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub(crate) struct RunGetAccount {
    #[serde(rename(serialize = "codeBase64"))]
    pub code: Option<String>,
    #[serde(rename(serialize = "dataBase64"))]
    pub data: Option<String>,
    #[serde(rename(serialize = "address"))]
    pub id: Option<String>,
    pub balance: Option<String>,
    pub last_paid: Option<u32>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfRunGet {
    #[serde(flatten)]
    pub account: RunGetAccount,
    pub function_name: String,
    pub input: Option<Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfResolveError {
    pub address: TonAddress,
    pub account: Option<serde_json::Value>,
    pub message_base64: String,
    pub time: u32,
    pub main_error: InnerSdkError,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfProcessTransaction{
    pub transaction: serde_json::Value,
    pub abi: Option<serde_json::Value>,
    pub function_name: Option<String>,
    pub address: TonAddress,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageProcessingState {
    last_block_id: String,
    sent_time: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParamsOfWaitTransaction {
    pub abi: Option<serde_json::Value>,
    pub function_name: Option<String>,
    pub message: EncodedMessage,
    pub state: MessageProcessingState,
    pub infinite_wait: bool
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
        abi: JsonValue,
        image: &[u8],
        init_params: Option<JsonValue>,
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
        abi: Option<JsonValue>,
        image: Option<&[u8]>,
        init_params: Option<JsonValue>,
        public_key: &Ed25519Public,
        workchain_id: Option<i32>,
    ) -> TonResult<ResultOfGetDeployData> {
        Interop::json_request(
            self.context,
            "contracts.deploy.data",
            ParamsOfGetDeployData {
                abi: option_params_to_value(abi)?,
                image_base64: image.map(|val| base64::encode(val)),
                init_params: option_params_to_value(init_params)?,
                public_key_hex: public_key.clone(),
                workchain_id: workchain_id,
            })
    }

    /// Deploy contract to TON blockchain
    pub fn deploy(
        &self,
        abi: JsonValue,
        code: &[u8],
        constructor_header: Option<JsonValue>,
        constructor_params: JsonValue,
        init_params: Option<JsonValue>,
        keys: &Ed25519KeyPair,
        workchain_id: i32,
    ) -> TonResult<ResultOfDeploy> {
        Interop::json_request(self.context, "contracts.deploy", ParamsOfDeploy {
            abi: abi.to_value()?,
            init_params: option_params_to_value(init_params)?,
            constructor_header: option_params_to_value(constructor_header)?,
            constructor_params:constructor_params.to_value()?,
            image_base64: base64::encode(code),
            key_pair: keys.clone(),
            workchain_id: workchain_id,
            try_index: None,
        })
    }

    /// Run the contract function with given parameters
    pub fn run(
        &self,
        address: &TonAddress,
        abi: JsonValue,
        function_name: &str,
        header: Option<JsonValue>,
        input: JsonValue,
        keys: Option<&Ed25519KeyPair>,
    ) -> TonResult<ResultOfRun> {
        Interop::json_request(self.context, "contracts.run", ParamsOfRun {
            address: address.clone(),
            abi: abi.to_value()?,
            function_name: function_name.to_string(),
            header: option_params_to_value(header)?,
            input: input.to_value()?,
            key_pair: keys.cloned(),
            try_index: None,
        })
    }

    /// Run the contract function with given parameters locally
    pub fn run_local(
        &self,
        address: &TonAddress,
        account: Option<JsonValue>,
        abi: JsonValue,
        function_name: &str,
        header: Option<JsonValue>,
        input: JsonValue,
        keys: Option<&Ed25519KeyPair>,
        time: Option<u32>,
        emulate_transaction: bool
    ) -> TonResult<ResultOfLocalRun> {
        Interop::json_request(self.context, "contracts.run.local", ParamsOfLocalRun {
            address: address.clone(),
            account: option_params_to_value(account)?,
            abi: abi.to_value()?,
            function_name: function_name.to_string(),
            header: option_params_to_value(header)?,
            input: input.to_value()?,
            key_pair: keys.cloned(),
            time,
            full_run: emulate_transaction
        })
    }

        /// Run the contract function with given parameters locally
    pub fn run_local_msg(
        &self,
        address: &TonAddress,
        account: Option<JsonValue>,
        message: EncodedMessage,
        abi: Option<JsonValue>,
        function_name: Option<&str>,
        time: Option<u32>,
        emulate_transaction: bool,
    ) -> TonResult<ResultOfLocalRun> {
        Interop::json_request(self.context, "contracts.run.local.msg", ParamsOfLocalRunWithMsg {
            address: address.clone(),
            account: option_params_to_value(account)?,
            message_base64: base64::encode(&message.message_body),
            abi: option_params_to_value(abi)?,
            function_name: function_name.map(|val| val.to_string()),
            time,
            full_run: emulate_transaction
        })
    }

    /// Decodes input message body with encoded contract call parameters
    pub fn decode_input_message_body(
        &self,
        abi: JsonValue,
        body: &[u8],
        internal: bool,
    ) -> TonResult<ResultOfDecodeMessageBody> {
        Interop::json_request(
            self.context,
            "contracts.run.unknown.input",
            ParamsOfDecodeMessageBody {
                abi: abi.to_value()?,
                body_base64: base64::encode(body),
                internal,
        })
    }

    /// Decode external outbound message body with encoded contract function response or event
    pub fn decode_output_message_body(
        &self,
        abi: JsonValue,
        body: &[u8]
    ) -> TonResult<ResultOfDecodeMessageBody> {
        Interop::json_request(
            self.context,
            "contracts.run.unknown.output",
            ParamsOfDecodeMessageBody {
                abi: abi.to_value()?,
                body_base64: base64::encode(body),
                internal: false,
        })
    }

    /// Create message to run the contract function with given parameters
    pub fn create_run_message(
        &self,
        address: &TonAddress,
        abi: JsonValue,
        function_name: &str,
        header: Option<JsonValue>,
        input: JsonValue,
        keys: Option<&Ed25519KeyPair>,
        try_index: Option<u8>
    ) -> TonResult<EncodedMessage> {
        Interop::json_request(self.context, "contracts.run.message", ParamsOfRun {
            address: address.clone(),
            abi: abi.to_value()?,
            function_name: function_name.to_string(),
            header: option_params_to_value(header)?,
            input: input.to_value()?,
            key_pair: keys.cloned(),
            try_index,
        })
    }

    /// Create message to deploy contract
    pub fn create_deploy_message(
        &self,
        abi: JsonValue,
        code: &[u8],
        constructor_header: Option<JsonValue>,
        constructor_params: JsonValue,
        init_params: Option<JsonValue>,
        keys: &Ed25519KeyPair,
        workchain_id: i32,
        try_index: Option<u8>
    ) -> TonResult<EncodedMessage> {
        Interop::json_request(
            self.context,
            "contracts.deploy.message",
            ParamsOfDeploy {
                abi: abi.to_value()?,
                init_params: option_params_to_value(init_params)?,
                constructor_header: option_params_to_value(constructor_header)?,
                constructor_params: constructor_params.to_value()?,
                image_base64: base64::encode(code),
                key_pair: keys.clone(),
                workchain_id,
                try_index,
        })
    }

    /// Send message to node without waiting for processing result
    pub fn send_message(&self, message: EncodedMessage) -> TonResult<MessageProcessingState> {
        Interop::json_request(
            self.context,
            "contracts.send.message",
            message
        )
    }

    /// Send message to waiting for processing result and (optionally) parse result
    pub fn process_message(
        &self,
        message: EncodedMessage,
        abi: Option<JsonValue>,
        function_name: Option<&str>,
        infinite_wait: bool
    ) -> TonResult<ResultOfRun> {
        Interop::json_request(
            self.context,
            "contracts.process.message",
            ParamsOfProcessMessage {
                abi: option_params_to_value(abi)?,
                function_name: function_name.map(|val| val.to_owned()),
                infinite_wait,
                message: message
            }
        )
    }

    /// Wait for message processing result and (optionally) parse result
    pub fn wait_for_transaction(
        &self,
        message: EncodedMessage,
        abi: Option<JsonValue>,
        function_name: Option<&str>,
        state: MessageProcessingState,
        infinite_wait: bool
    ) -> TonResult<ResultOfRun> {
        Interop::json_request(
            self.context,
            "contracts.wait.transaction",
            ParamsOfWaitTransaction {
                abi: option_params_to_value(abi)?,
                function_name: function_name.map(|val| val.to_owned()),
                state,
                message,
                infinite_wait
            }
        )
    }

    /// Run the contract get method locally
    pub fn run_get(
        &self,
        address: Option<&TonAddress>,
        account: Option<JsonValue>,
        function_name: &str,
        input: Option<JsonValue>,
    ) -> TonResult<ResultOfLocalRun> {
        let mut account: RunGetAccount = account.map(|val| {
                serde_json::from_value(val.to_value()?)
                    .map_err(|_| TonErrorKind::InvalidArg("account".to_owned()))
            })
            .transpose()?
            .unwrap_or_default();
        if let Some(addr) = address {
            account.id = Some(addr.to_string());
        }
        Interop::json_request(self.context, "tvm.get", ParamsOfRunGet {
            account,
            function_name: function_name.to_string(),
            input: option_params_to_value(input)?,
        })
    }

    /// Convert list in `cons` representation to `Vec`
    pub fn cons_to_vec(&self, cons: Value) -> TonResult<Value> {
        let mut result = vec![];
        let mut item = cons;
        while !item.is_null() {
            if item.as_array().unwrap_or(&vec![]).len() != 2 {
                return Err(TonErrorKind::InvalidArg("Invalid cons".to_owned()).into());
            }
            result.push(item[0].take());
            item = item[1].take();
        }
        Ok(result.into())
    }

    /// Investigate message processing error
    pub fn resolve_error(
        &self,
        address: &TonAddress,
        account: Option<JsonValue>,
        message: EncodedMessage,
        send_time: u32,
        error: InnerSdkError
    ) -> TonResult<()> {
        Interop::json_request(self.context, "contracts.resolve.error", ParamsOfResolveError {
            address: address.clone(),
            account: option_params_to_value(account)?,
            message_base64: base64::encode(&message.message_body),
            time: send_time,
            main_error: error,
        })
    }

    /// Process recieved transaction to check errors and get output
    pub fn process_transaction(
        &self,
        address: &TonAddress,
        transaction: JsonValue,
        abi: Option<JsonValue>,
        function_name: Option<&str>,
    ) -> TonResult<ResultOfRun> {
        Interop::json_request(self.context, "contracts.process.transaction", ParamsOfProcessTransaction {
            address: address.clone(),
            abi: option_params_to_value(abi)?,
            transaction: transaction.to_value()?,
            function_name: function_name.map(|val| val.to_owned())
        })
    }
}
