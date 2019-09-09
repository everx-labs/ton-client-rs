use crate::{TonResult, TonError};
use std::ptr::null;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[link(name = "ton_client")]
extern "C" {
    fn tc_create_context() -> InteropContext;
    fn tc_destroy_context(context: InteropContext);
    fn tc_json_request(
        context: InteropContext,
        method_name: InteropString,
        params_json: InteropString,
    ) -> *const JsonResponse;
    fn tc_destroy_json_response(
        response: *const JsonResponse
    );

    fn tc_read_json_response(
        response: *const JsonResponse
    ) -> InteropJsonResponse;
}

// Types

pub type InteropContext = u32;

#[repr(C)]
pub struct InteropString {
    pub content: *const u8,
    pub len: u32,
}


#[repr(C)]
pub struct InteropJsonResponse {
    pub result_json: InteropString,
    pub error_json: InteropString,
}

pub struct JsonResponse {
    pub result_json: String,
    pub error_json: String,
}

// Helpers

impl InteropString {
    pub(crate) fn default() -> Self {
        Self {
            content: null(),
            len: 0
        }
    }

    pub(crate) fn from(s: &String) -> Self {
        Self {
            content: s.as_ptr(),
            len: s.len() as u32,
        }
    }

    pub(crate) fn to_string(&self) -> String {
        unsafe {
            let utf8 = std::slice::from_raw_parts(self.content, self.len as usize);
            String::from_utf8(utf8.to_vec()).unwrap()
        }
    }

}

impl InteropJsonResponse {
    pub(crate) fn default() -> Self {
        Self {
            result_json: InteropString::default(),
            error_json: InteropString::default(),
        }
    }

    pub(crate) fn from(response: &JsonResponse) -> Self {
        Self {
            result_json: InteropString::from(&response.result_json),
            error_json: InteropString::from(&response.error_json),
        }
    }

    pub(crate) fn to_response(&self) -> JsonResponse {
        JsonResponse {
            result_json: self.result_json.to_string(),
            error_json: self.error_json.to_string(),
        }
    }
}



pub(crate) struct Interop {}

impl Interop {
    pub fn create_context() -> InteropContext {
        unsafe {
            tc_create_context()
        }
    }

    pub fn destroy_context(context: InteropContext) {
        unsafe {
            tc_destroy_context(context)
        }
    }

    fn base_json_request<R: DeserializeOwned>(context: InteropContext, method_name: &str, params_json: String) -> TonResult<R> {
        let response = Self::interop_json_request(
            context,
            &method_name.to_string(),
            &params_json);
        if response.error_json.is_empty() {
            let result: Result<R, serde_json::Error> = serde_json::from_str(&response.result_json);
            result.map_err(|err| TonError::invalid_response_result(method_name, &response.result_json))
        } else {
            let result: Result<TonError, serde_json::Error> = serde_json::from_str(&response.error_json);
            match result {
                Ok(err) => Err(err),
                Err(err) => Err(TonError::invalid_response_error(method_name, &response.error_json))
            }
        }
    }

    pub fn json_request<P: Serialize, R: DeserializeOwned>(
        context: InteropContext,
        method_name: &str,
        params: P
    ) -> TonResult<R> {
        let params_json = serde_json::to_string(&params)
            .map_err(|err|TonError::invalid_params(method_name))?;
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
        params_json: &String
    ) -> JsonResponse {
        unsafe {
            let response_ptr = tc_json_request(
                context,
                InteropString::from(method_name),
                InteropString::from(params_json)
            );
            let interop_response = tc_read_json_response(response_ptr);
            let response = interop_response.to_response();
            tc_destroy_json_response(response_ptr);
            response
        }
    }

}
