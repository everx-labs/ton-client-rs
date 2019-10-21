/*
 * Copyright 2018-2019 TON DEV SOLUTIONS LTD.
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

use crate::{TonClient, Ed25519KeyPair, TonAddress};
mod test_piggy;

#[test]
fn test_contracts() {
    // Deploy Messages

    let ton = TonClient::new_with_base_url("http://0.0.0.0").unwrap();
	
    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();
	    
	let prepared_wallet_address = ton.contracts.get_deploy_address(
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        &keys).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address);

    let address = ton.contracts.deploy(
        WALLET_ABI,
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        json!({}).to_string().into(),
        &keys).unwrap();

	assert_eq!(prepared_wallet_address, address);

    let version = ton.contracts.run(
        &address,
        WALLET_ABI,
        "getLimitCount",
        json!({}).to_string().into(),
        Some(&keys)).unwrap();
    println!("{}", version)
}

pub fn get_grams_from_giver(ton: &TonClient, account: &TonAddress) {
	ton.contracts.run(
        &TonAddress::from_str(GIVER_ADDRESS).unwrap(),
        GIVER_ABI,
        "sendGrams",
        json!({
           "dest": format!("0x{}", account.get_account_hex_string()),
           "amount": 10_000_000_000u64
        }).to_string().into(),
       None).unwrap();

	// wait for grams recieving
	let wait_result = ton.queries.accounts.wait_for(&json!({
			"id": { "eq": account.get_account_hex_string() },
			"storage": {
				"balance": {
					"Grams": { "gt": "0" }
				}
			}
		}).to_string(),
		"id storage {balance {Grams}}"
	).unwrap();

	println!("wait result {}", wait_result);
}


#[test]
#[ignore]
fn test_decode_input() {
    let body = "te6ccoEBAgEAcwARcwEbACfvUIcBgJTr3AOCAGABAMDr2GubWXYR6wuk6WFn4btjW3w+DbidhSrKArHbqCaunLGN9LwAbQFT9kyOpN6DR6DJbuKkvC94KwJgan7xeTUHS89H/vKbWZbzZEHu4euhqvQE2I9aW+PNdn2BKZJXlA4=";
	let body = base64::decode(body).unwrap();

	let ton = TonClient::default().unwrap();

    let result = ton.contracts.decode_input_message_body(WALLET_ABI, &body).expect("Couldn't parse body");

	assert_eq!(result.function, "createLimit");
	assert_eq!(result.output, json!({ "type": "0x1", "value": "0x3b9aca00", "meta": "x01"}));
}

const GIVER_ADDRESS: &str = "a46af093b38fcae390e9af5104a93e22e82c29bcb35bf88160e4478417028884";
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
				{"name":"dest","type":"uint256"},
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
			"name": "constructor",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "sendTransaction",
			"inputs": [
				{"name":"dest","type":"uint256"},
				{"name":"value","type":"uint128"},
				{"name":"bounce","type":"bool"}
			],
			"outputs": [
			]
		},
		{
			"name": "setSubscriptionAccount",
			"inputs": [
				{"name":"addr","type":"uint256"}
			],
			"outputs": [
			]
		},
		{
			"name": "getSubscriptionAccount",
			"inputs": [
			],
			"outputs": [
				{"name":"value0","type":"uint256"}
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
		}
	],
	"events": [
	],
	"data": [
		{"key":102,"name":"MAX_LIMIT_COUNT","type":"uint8"},
		{"key":103,"name":"SECONDS_IN_DAY","type":"uint32"},
		{"key":104,"name":"MAX_LIMIT_PERIOD","type":"uint32"}
	]
}
"#;
