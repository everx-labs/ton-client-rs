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

 /// Error returned from SDK core
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InnerSdkError {
    pub source: String,
    pub code: isize,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Information about aborted transaction
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiErrorData {
    pub transaction_id: String,
    pub phase: String,
}

error_chain! {

    types {
        TonError, TonErrorKind, TonResultExt, TonResult;
    }

    foreign_links {
        Io(std::io::Error);
        SerdeJson(serde_json::Error);
        TryFromSliceError(std::array::TryFromSliceError);
        ParseIntError(std::num::ParseIntError);
        FromHexError(hex::FromHexError);
        Base64DecodeError(base64::DecodeError);
    }

    errors {
        NotFound {
            description("Requested item not found")
        }
        InvalidOperation(msg: String) {
             description("Invalid operation"),
             display("Invalid operation: {}", msg)
        }
        InvalidData(msg: String) {
            description("Invalid data"),
            display("Invalid data: {}", msg)
        }
        InvalidArg(msg: String) {
            description("Invalid argument"),
            display("Invalid argument: {}", msg)
        }
        InvalidFunctionParams(func: String, inner: String){
            description("Invalid function parameters"),
            display("Can not serialize params for {}. Error {}", func, inner)
        }
        InvalidFunctionResult(func: String, result: String, inner: String){
            description("Invalid function result"),
            display("Can not deserialize result for {}\nresult JSON: {}\ninner error {}", func, result, inner)
        }
        InvalidFunctionError(func: String, error: String, inner: String){
            description("Invalid function parameters"),
            display("Can not deserialize error for {}\nerror JSON: {}\ninner error {}", func, error, inner)
        }
        InternalError(msg: String) {
            description("Internal error"),
            display("Internal error: {}", msg)
        }
        InnerSdkError(inner: InnerSdkError) {
            description("Inner SDK error"),
            display(
                "Inner SDK error.\n source: {}\n code: {}\n message: {}\n data: {:#}\n",
                inner.source,
                inner.code,
                inner.message,
                inner.data.as_ref().unwrap_or(&"null".into()),
            )
        }
    }
}
