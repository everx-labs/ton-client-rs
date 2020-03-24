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

lazy_static::lazy_static! {
    static ref GIVER_ADDRESS: TonAddress = TonAddress::from_str("0:841288ed3b55d9cdafa806807f02a0ae0c169aa5edfe88a789a6482429756a94").unwrap();
    static ref WALLET_ADDRESS: TonAddress = TonAddress::from_str("0:5b168970a9c63dd5c42a6afbcf706ef652476bb8960a22e1d8a2ad148e60c0ea").unwrap();
	static ref WALLET_KEYS: Option<Ed25519KeyPair> = get_wallet_keys();

	static ref ABI_VERSION: u8 = u8::from_str_radix(&env::var("ABI_VERSION").unwrap_or("1".to_owned()), 10).unwrap();
	static ref CONTRACTS_PATH: String = format!("src/tests/contracts/abi_v{}/", *ABI_VERSION);
	static ref NODE_ADDRESS: String = env::var("TON_NETWORK_ADDRESS")
		//.unwrap_or("cinet.tonlabs.io".to_owned());
		.unwrap_or("http://localhost".to_owned());
	static ref NODE_SE: bool = env::var("NODE_SE").unwrap_or("true".to_owned()) == "true".to_owned();

	pub static ref SUBSCRIBE_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Subscription.abi.json").unwrap();
	pub static ref PIGGY_BANK_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Piggy.abi.json").unwrap();
    pub static ref WALLET_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "LimitWallet.abi.json").unwrap();
    pub static ref SIMPLE_WALLET_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Wallet.abi.json").unwrap();
    pub static ref GIVER_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Giver.abi.json").unwrap();
    
    pub static ref SUBSCRIBE_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Subscription.tvc").unwrap();
	pub static ref PIGGY_BANK_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Piggy.tvc").unwrap();
	pub static ref WALLET_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "LimitWallet.tvc").unwrap();
	pub static ref SIMPLE_WALLET_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Wallet.tvc").unwrap();
}

fn get_wallet_keys() -> Option<Ed25519KeyPair> {
	if *NODE_SE {
		return None;
	}

    let mut keys_file = dirs::home_dir().unwrap();
    keys_file.push("giverKeys.json");
    let keys = std::fs::read_to_string(keys_file).unwrap();
    
    Some(serde_json::from_str(&keys).unwrap())
}

pub fn create_client() -> TonClient {
	TonClient::new_with_base_url(&NODE_ADDRESS).unwrap()
}

#[test]
fn test_contracts() {
    // Deploy Messages

    let ton = create_client();
	
    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();
	    
	let prepared_wallet_address = ton.contracts.get_deploy_address(
		&WALLET_ABI,
		&WALLET_IMAGE,
		None,
        &keys).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address);

    let deploy_result = ton.contracts.deploy(
		&WALLET_ABI,
		&WALLET_IMAGE,
		None,
		json!({}).to_string().into(),
		None,
        &keys).unwrap();

	assert_eq!(prepared_wallet_address, deploy_result.address);
	assert!(!deploy_result.alreadyDeployed);

	// check that second deploy returns `alreadyDeployed == true`
	let deploy_result = ton.contracts.deploy(
		&WALLET_ABI,
		&WALLET_IMAGE,
		None,
		json!({}).to_string().into(),
		None,
        &keys).unwrap();

	assert_eq!(prepared_wallet_address, deploy_result.address);
	assert!(deploy_result.alreadyDeployed);

	if *ABI_VERSION == 2 {
		// check header params passing
		let result = ton.contracts.run(
			&deploy_result.address,
			&WALLET_ABI,
			"createOperationLimit",
			Some(json!({
				"expire": 123
			}).to_string().into()),
			json!({
				"value": 123
			}).to_string().into(),
			Some(&keys));
		println!("{:?}", result);
		assert!(result.is_err());
	};

    let result = ton.contracts.run(
        &deploy_result.address,
        &WALLET_ABI,
		"createOperationLimit",
		None,
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
		&SIMPLE_WALLET_ABI,
		&SIMPLE_WALLET_IMAGE,
		None,
        &keys).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address);

    let address = ton.contracts.deploy(
		&SIMPLE_WALLET_ABI,
		&SIMPLE_WALLET_IMAGE,
		None,
		json!({}).to_string().into(),
		None,
		&keys)
	.unwrap()
	.address;

	assert_eq!(prepared_wallet_address, address);

    let result = ton.contracts.run(
        &address,
        &SIMPLE_WALLET_ABI,
		"sendTransaction",
		None,
        json!({
			"dest": "0:0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF",
			"value": 0,
			"bounce": false
		}).to_string().into(),
        Some(&keys)
	)
	.unwrap_err();

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
	if *NODE_SE {
		ton.contracts.run(
			&GIVER_ADDRESS,
			&GIVER_ABI,
			"sendGrams",
			None,
			json!({
				"dest": account.to_string(),
				"amount": 10_000_000_000u64
			}).to_string().into(),
			None).unwrap();
	} else {
		ton.contracts.run(
			&WALLET_ADDRESS,
			&SIMPLE_WALLET_ABI,
			"sendTransaction",
			None,
			json!({
				"dest": account.to_string(),
				"value": 500_000_000u64,
				"bounce": false
			}).to_string().into(),
			WALLET_KEYS.as_ref()).unwrap();
	}

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
	if *ABI_VERSION == 1 {
		return
	}

    let body = "te6ccgEBAgEAxAAB4cN+DrKSpIGXl1Hiw3nKO1DuMz+2bdqiDQs+ls0Hg4AoQ4mIlgG5zmZHta3KuhGVa9OzWNuOLg30kPt7jgHlcQUAAAC4hj72ui88/bSiH7zDERERERERERERERERERERERERERERERERERERERERERFAAQCb4x6KrJjwAyuS43YYq6ijEXqYPNlXuuEg3inm/Xzrjp2AHzifK5Wl2DCMGV4PhQHgA6ugk3JnpzNe9amIHu+pZgYAAAAAJUC+QAAAKjAQ";
	let body = base64::decode(body).unwrap();

	let ton = TonClient::default().unwrap();

	let result = ton.contracts.decode_input_message_body(&SUBSCRIBE_ABI, &body).expect("Couldn't parse body");

	assert_eq!(result.function, "subscribe");
	assert_eq!(result.output, json!({
        "period": "0x15180",
        "pubkey": "0xe31e8aac98f0032b92e37618aba8a3117a983cd957bae120de29e6fd7ceb8e9d",
        "subscriptionId": "0x2222222222222222222222222222222222222222222222222222222222222222",
        "to": "0:f9c4f95cad2ec18460caf07c280f001d5d049b933d399af7ad4c40f77d4b3030",
        "value": "0x12a05f200"
    }));
}

#[test]
fn test_init_state() {

	if *ABI_VERSION == 2 {
		return;
	}

    let subscription_address1 = "0:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let subscription_address2 = "0:fedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321";

	let ton = create_client();
	
    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();
	    
	let wallet_address1 = ton.contracts.get_deploy_address(
		&WALLET_ABI,
		&WALLET_IMAGE,
		Some(json!({
			"subscription": subscription_address1,
            "owner": "0x".to_owned() + &keys.public.to_string(),
		}).to_string().into()),
		&keys).unwrap();
		
	let wallet_address2 = ton.contracts.get_deploy_address(
		&WALLET_ABI,
		&WALLET_IMAGE,
		Some(json!({
			"subscription": subscription_address2,
			"owner": "0x".to_owned() + &keys.public.to_string(),
		}).to_string().into()),
		&keys).unwrap();

	assert_ne!(wallet_address1, wallet_address2);
}
