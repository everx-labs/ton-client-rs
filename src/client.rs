use crate::interop::Interop;
use crate::{TonCrypto, TonContracts, TonResult, TonQueries};

#[derive(Default, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TonClientConfig {
    pub default_workchain: Option<i32>,
    pub base_url: Option<String>,
    pub requests_url: Option<String>,
    pub queries_url: Option<String>,
    pub subscriptions_url: Option<String>,
}

pub struct TonClient {
    context: u32,
    pub crypto: TonCrypto,
    pub contracts: TonContracts,
    pub queries: TonQueries,
}

impl TonClient {
    pub fn new(config: &TonClientConfig) -> TonResult<TonClient> {
        let context = Interop::create_context();
        let mut client = TonClient {
            context,
            crypto: TonCrypto::new(context),
            contracts: TonContracts::new(context),
            queries: TonQueries::new(context),
        };
        client.setup(config);
        Ok(client)
    }

    pub fn default() -> TonResult<TonClient> {
        Self::new(&TonClientConfig::default())
    }

    pub fn setup(&self, config: &TonClientConfig) -> TonResult<()> {
        Interop::json_request(self.context, "setup", config)
    }
}
