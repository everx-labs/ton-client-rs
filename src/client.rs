/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

use crate::interop::Interop;
use crate::{TonCrypto, TonContracts, TonQueries, TonResult};

/// `TonClient` configuration. Contains optional fields with configuration parameters.
/// 
/// `base_url` is URL of the Tonlabs node to connect
/// 
/// `message_retries_count` sets message sending retries count. 0 to disable retrying. Default is 5.
/// 
/// `message_expiration_timeout` sets time in ms used to calculate for message expiration time if `expire`
/// value is not set in contract function header params. Default is 10 seconds
/// 
/// `message_expiration_timeout_grow_factor` sets `message_expiration_timeout` multiplying coefficient
/// for retriyng messages. `message_expiration_timeout` for each retry is calculated by formula
/// `message_expiration_timeout * message_expiration_timeout_grow_factor^retry_index`. Default is 1.5
/// 
/// `message_processing_timeout` sets timeout in ms for processing messages to contracts which don't support
/// message expiration. It is also used for waiting blocks after message expiration time ends.assert_eq!
/// 
/// `message_processing_timeout_grow_factor` sets `message_processing_timeout` multiplying coefficient
/// for retriyng messages. `message_processing_timeout` for each retry is calculated by formula
/// `message_processing_timeout * message_processing_timeout_grow_factor^retry_index`. Default is 1.5
/// 
/// `wait_for_timeout` sets default timeout in ms for `wait_for` function
/// 
/// `access_key` is key for authenicating user to Tonlabs node
/// 
#[derive(Default, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TonClientConfig {
    pub base_url: Option<String>,
    pub message_retries_count: Option<u8>,
    pub message_expiration_timeout: Option<u32>,
    pub message_expiration_timeout_grow_factor: Option<f32>,
    pub message_processing_timeout: Option<u32>,
    pub message_processing_timeout_grow_factor: Option<f32>,
    pub wait_for_timeout: Option<u32>,
    pub access_key: Option<String>,
}

/// Entry point for TON blockchain interaction. Provides useful methods for TON clients
pub struct TonClient {
    context: u32,
    pub crypto: TonCrypto,
    pub contracts: TonContracts,
    pub queries: TonQueries

}

impl TonClient {
    /// Create `TonClient` instance with full configuration.
    pub fn new(config: &TonClientConfig) -> TonResult<TonClient> {
        let context = Interop::create_context();
        let client = TonClient {
            context,
            crypto: TonCrypto::new(context),
            contracts: TonContracts::new(context),
            queries: TonQueries::new(context)
        };
        client.setup(config)?;
        Ok(client)
    }

    /// Create `TonClient` instance with base URL only. Other URLs are derived from base URL.write!
    /// `default_workchain` is set to 0.
    pub fn new_with_base_url(base_url: &str) -> TonResult<TonClient> {
        Self::new(&TonClientConfig {
            base_url: Some(base_url.to_string()),
            ..TonClientConfig::default()
        })
    }

    /// Create `TonClient` instance with default parameters.
    pub fn default() -> TonResult<TonClient> {
        Self::new(&TonClientConfig::default())
    }

    /// Get version of the library
    pub fn get_client_version(&self) -> TonResult<String> {
        Interop::json_request_no_args(self.context, "version")
    }

    /// Set parameters for node interaction
    pub fn setup(&self, config: &TonClientConfig) -> TonResult<()> {
        Interop::json_request(self.context, "setup", config)
    }
}

impl Drop for TonClient {
    fn drop(&mut self) {
        Interop::destroy_context(self.context);
    }
}
