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

use std::env;
use crate::{TonClient, Ed25519KeyPair, TonAddress};
mod test_piggy;

pub fn create_client() -> TonClient {
	
	let node_se_addr = env::var("TON_NETWORK_ADDRESS")
		.unwrap_or("http://localhost".to_string());
	
	TonClient::new_with_base_url(&node_se_addr).unwrap()
}

#[test]
fn test_contracts() {
    // Deploy Messages

    let ton = create_client();
	
    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();
	    
	let prepared_wallet_address = ton.contracts.get_deploy_address(
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        &keys).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address);

    let deploy_result = ton.contracts.deploy(
        WALLET_ABI,
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        json!({}).to_string().into(),
        &keys).unwrap();

	assert_eq!(prepared_wallet_address, deploy_result.address);
	assert!(!deploy_result.alreadyDeployed);

	// check that second deploy returns `alreadyDeployed == true`
	let deploy_result = ton.contracts.deploy(
        WALLET_ABI,
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        json!({}).to_string().into(),
        &keys).unwrap();

	assert_eq!(prepared_wallet_address, deploy_result.address);
	assert!(deploy_result.alreadyDeployed);


    let result = ton.contracts.run(
        &deploy_result.address,
        WALLET_ABI,
        "createOperationLimit",
        json!({
			"value": 123
		}).to_string().into(),
        Some(&keys)).unwrap();
    println!("{}", result)
}

#[test]
fn test_call_aborted_transaction() {
	use crate::error::{TonError, TonErrorKind::InnerSdkError};

    let ton = create_client();
	
    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();
	    
	let prepared_wallet_address = ton.contracts.get_deploy_address(
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        &keys).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address);

    let address = ton.contracts.deploy(
        WALLET_ABI,
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        json!({}).to_string().into(),
		&keys)
	.unwrap()
	.address;

	assert_eq!(prepared_wallet_address, address);

    let result = ton.contracts.run(
        &address,
        WALLET_ABI,
        "sendTransaction",
        json!({
			"dest": "0:0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF",
			"value": 0,
			"bounce": false
		}).to_string().into(),
        Some(&keys)
	)
	.unwrap_err();

	println!("Error: {}", result);

	match result {
		TonError(InnerSdkError(err), _) => {
			assert_eq!(&err.source, "node");
			assert_eq!(err.code, 102);
			assert_eq!(err.data.is_some(), true);
			assert_eq!(&err.data.as_ref().unwrap().phase, "computeVm");
		},
		_ => panic!(),
	};
}

pub fn get_grams_from_giver(ton: &TonClient, account: &TonAddress) {
	ton.contracts.run(
        &TonAddress::from_str(GIVER_ADDRESS).unwrap(),
        GIVER_ABI,
        "sendGrams",
        json!({
           "dest": account.to_string(),
           "amount": 10_000_000_000u64
        }).to_string().into(),
       None).unwrap();

	// wait for grams recieving
	let wait_result = ton.queries.accounts.wait_for(&json!({
			"id": { "eq": account.to_string() },
			"balance": { "gt": "0" }
		}).to_string(),
		"id balance"
	).unwrap();

	println!("wait result {}", wait_result);
}


#[test]
fn test_decode_input() {
    let body = "te6ccgEBAgEA3wAB8y88h10AAAFuW6FWJBERERERERERERERERERERERERERERERERERERERERERIXxlwlrjEGJEDhx3dC3WlQeZKzuAYBDOJ8+g7AM+Ek6AF49G0+VDwIkQKBdIh7hi4J5F0T/g5OggwrHI4HGN1KHAAAAAAAAAD2AAADkQAQDADBiSeQ1t5j0LwYo9dx7wefpnCQ3KrYOeAhX9ZUux62yIxWdQdUHJGCXXcoLbrDDduL9sgKSZT3TzYpRKi8YqASF8ZcJa4xBiRA4cd3Qt1pUHmSs7gGAQzifPoOwDPhJO";
	let body = base64::decode(body).unwrap();

	let ton = TonClient::default().unwrap();

    let result = ton.contracts.decode_input_message_body(test_piggy::SUBSCRIBE_ABI, &body).expect("Couldn't parse body");

	assert_eq!(result.function, "subscribe");
	assert_eq!(result.output, json!({
        "period": "0x1c8",
        "pubkey": "0x217c65c25ae31062440e1c77742dd69507992b3b806010ce27cfa0ec033e124e",
        "subscriptionId": "0x1111111111111111111111111111111111111111111111111111111111111111",
        "to": "0:bc7a369f2a1e04488140ba443dc31704f22e89ff07274106158e47038c6ea50e",
        "value": "0x7b"
    }));
}

const GIVER_ADDRESS: &str = "0:841288ed3b55d9cdafa806807f02a0ae0c169aa5edfe88a789a6482429756a94";
const GIVER_ABI: &str = r#"
{
	"ABI version": 1,
	"functions": [
		{
			"name": "constructor",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "sendGrams",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"amount","type":"uint64"}
			],
			"outputs": [
			]
		}
	],
	"events": [
	],
	"data": [
	]
}"#;

pub const WALLET_CODE_BASE64: &str = r#"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"#;
pub const WALLET_ABI: &str = r#"
{
	"ABI version": 1,
	"functions": [
		{
			"name": "sendTransaction",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"value","type":"uint128"},
				{"name":"bounce","type":"bool"}
			],
			"outputs": [
			]
		},
		{
			"name": "setSubscriptionAccount",
			"inputs": [
				{"name":"addr","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "getSubscriptionAccount",
			"inputs": [
			],
			"outputs": [
				{"name":"value0","type":"address"}
			]
		},
		{
			"name": "createOperationLimit",
			"inputs": [
				{"name":"value","type":"uint256"}
			],
			"outputs": [
				{"name":"value0","type":"uint256"}
			]
		},
		{
			"name": "createArbitraryLimit",
			"inputs": [
				{"name":"value","type":"uint256"},
				{"name":"period","type":"uint32"}
			],
			"outputs": [
				{"name":"value0","type":"uint64"}
			]
		},
		{
			"name": "changeLimit",
			"inputs": [
				{"name":"limitId","type":"uint64"},
				{"name":"value","type":"uint256"},
				{"name":"period","type":"uint32"}
			],
			"outputs": [
			]
		},
		{
			"name": "deleteLimit",
			"inputs": [
				{"name":"limitId","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "getLimit",
			"inputs": [
				{"name":"limitId","type":"uint64"}
			],
			"outputs": [
				{"components":[{"name":"value","type":"uint256"},{"name":"period","type":"uint32"},{"name":"ltype","type":"uint8"},{"name":"spent","type":"uint256"},{"name":"start","type":"uint32"}],"name":"value0","type":"tuple"}
			]
		},
		{
			"name": "getLimitCount",
			"inputs": [
			],
			"outputs": [
				{"name":"value0","type":"uint64"}
			]
		},
		{
			"name": "getLimits",
			"inputs": [
			],
			"outputs": [
				{"name":"value0","type":"uint64[]"}
			]
		},
		{
			"name": "constructor",
			"inputs": [
			],
			"outputs": [
			]
		}
	],
	"events": [
	],
	"data": [
		{"key":101,"name":"subscription","type":"address"},
		{"key":100,"name":"owner","type":"uint256"}
	]
}
"#;
