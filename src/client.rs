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

use crate::interop::Interop;
use crate::{TonCrypto, TonContracts, TonQueries, TonResult};

/// `TonClient` configuration. Contains optional fields with configuration parameters.
/// 
/// `default_workchain` sets target workchain for deploying and running contracts
/// 
/// `base_url` is used for deriving `requests_url`, `queries_url` and `subscriptions_url`
///  values with default suffixes if ones are not set.
/// 
/// `requests_url` points address for sending requests to node via http REST API
/// 
/// `queries_url` points address of GraphQL server for quering blockchain data
/// 
/// `subscriptions_url` points address of GraphQL server for subscripitions on blockchain data updates
#[derive(Default, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TonClientConfig {
    pub base_url: Option<String>,
    pub requests_url: Option<String>,
    pub queries_url: Option<String>,
    pub subscriptions_url: Option<String>,
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
            requests_url: None,
            queries_url: None,
            subscriptions_url: None
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
