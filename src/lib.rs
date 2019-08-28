#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate base64;


#[cfg(test)]
mod tests;

mod interop;

mod client;
pub use client::*;

mod crypto;
pub use crypto::*;

mod contracts;
pub use contracts::*;

mod queries;
pub use queries::*;

#[derive(Debug, Deserialize)]
pub struct TonError {
    category: String,
    code: u32,
    message: String,
}

impl TonError {
    pub(crate) fn check<R>(ok: bool, r: R) -> TonResult<R> {
        if ok {
            Ok(r)
        } else {
            Err(Self {
                category: "sdk".to_string(),
                code: 0,
                message: String::new(),
            })
        }
    }

    fn sdk(code: u32, message: &str) -> Self {
        Self {
            category: "sdk".to_string(),
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

