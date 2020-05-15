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

use crate::error::*;
use serde::Serialize;
use serde::de::DeserializeOwned;

// Types

pub type InteropContext = u32;

pub(crate) struct JsonResponse {
    pub result_json: String,
    pub error_json: String,
}

pub(crate) struct Interop {}

impl Interop {
    pub fn create_context() -> InteropContext {
        ton_client::create_context()
    }

    pub fn destroy_context(context: InteropContext) {
        ton_client::destroy_context(context)
    }

    fn base_json_request<R: DeserializeOwned>(context: InteropContext, method_name: &str, params_json: String) -> TonResult<R> {
        let response = Self::interop_json_request(
            context,
            &method_name.to_string(),
            &params_json);
        if response.error_json.is_empty() {
            serde_json::from_str(&response.result_json)
                .map_err(|err| TonError::from(TonErrorKind::InvalidFunctionResult(
                    method_name.to_owned(), response.result_json, err.to_string())))
        } else {
            let result: Result<InnerSdkError, serde_json::Error> = serde_json::from_str(&response.error_json);
            match result {
                Ok(err) => Err(TonErrorKind::InnerSdkError(err).into()),
                Err(err) => Err(TonErrorKind::InvalidFunctionError(
                    method_name.to_owned(), response.error_json, err.to_string()).into())
            }
        }
    }

    pub fn json_request<P: Serialize, R: DeserializeOwned>(
        context: InteropContext,
        method_name: &str,
        params: P,
    ) -> TonResult<R> {
        let params_json = serde_json::to_string(&params)
            .map_err(|err| TonError::from(TonErrorKind::InvalidFunctionParams(
                method_name.to_owned(), err.to_string())))?;
        Self::base_json_request(context, method_name, params_json)
    }

    pub fn json_request_no_args<R: DeserializeOwned>(
        context: InteropContext,
        method_name: &str,
    ) -> TonResult<R> {
        Self::base_json_request(context, method_name, String::new())
    }

    fn interop_json_request(
        context: InteropContext,
        method_name: &String,
        params_json: &String,
    ) -> JsonResponse {
        let response = ton_client::json_sync_request(
            context,
            method_name.clone(),
            params_json.clone(),
        );
        JsonResponse {
            error_json: response.error_json,
            result_json: response.result_json,
        }
    }
}
