/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

use std::env;
use crate::{TonClient, Ed25519KeyPair, Ed25519Public, TonAddress, ResultOfGetDeployData, JsonValue};
mod test_piggy;
mod test_hello;
mod test_run_get;
mod test_errors;
mod test_local_run;

const ROOT_CONTRACTS_PATH: &str = "src/tests/contracts/";

lazy_static::lazy_static! {
    static ref GIVER_ADDRESS: TonAddress = TonAddress::from_str("0:841288ed3b55d9cdafa806807f02a0ae0c169aa5edfe88a789a6482429756a94").unwrap();
    static ref WALLET_ADDRESS: TonAddress = TonAddress::from_str("0:2bb4a0e8391e7ea8877f4825064924bd41ce110fce97e939d3323999e1efbb13").unwrap();
	static ref WALLET_KEYS: Option<Ed25519KeyPair> = get_wallet_keys();

	static ref ABI_VERSION: u8 = u8::from_str_radix(&env::var("ABI_VERSION").unwrap_or("2".to_owned()), 10).unwrap();
	static ref CONTRACTS_PATH: String = format!("{}abi_v{}/", ROOT_CONTRACTS_PATH, *ABI_VERSION);
	static ref NODE_ADDRESS: String = env::var("TON_NETWORK_ADDRESS")
		//.unwrap_or("cinet.tonlabs.io".to_owned());
		.unwrap_or("http://localhost".to_owned());
		//.unwrap_or("net.ton.dev".to_owned());
	static ref NODE_SE: bool = env::var("USE_NODE_SE").unwrap_or("true".to_owned()) == "true".to_owned();

	pub static ref SUBSCRIBE_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Subscription.abi.json").unwrap();
	pub static ref PIGGY_BANK_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Piggy.abi.json").unwrap();
    pub static ref WALLET_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "LimitWallet.abi.json").unwrap();
    pub static ref SIMPLE_WALLET_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Wallet.abi.json").unwrap();
	pub static ref GIVER_ABI: String = std::fs::read_to_string(ROOT_CONTRACTS_PATH.to_owned() + "Giver.abi.json").unwrap();
	pub static ref GIVER_WALLET_ABI: String = std::fs::read_to_string(ROOT_CONTRACTS_PATH.to_owned() + "GiverWallet.abi.json").unwrap();
	pub static ref HELLO_ABI: String = std::fs::read_to_string(CONTRACTS_PATH.clone() + "Hello.abi.json").unwrap();

    pub static ref SUBSCRIBE_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Subscription.tvc").unwrap();
	pub static ref PIGGY_BANK_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Piggy.tvc").unwrap();
	pub static ref WALLET_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "LimitWallet.tvc").unwrap();
	pub static ref SIMPLE_WALLET_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Wallet.tvc").unwrap();
	pub static ref HELLO_IMAGE: Vec<u8> = std::fs::read(CONTRACTS_PATH.clone() + "Hello.tvc").unwrap();
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

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
		match record.level() {
			log::Level::Error | log::Level::Warn => {
				eprintln!("{} - {}", record.level(), record.args());
			}
			_ => {
				println!("{} - {}", record.level(), record.args());
			}
		}
        
    }

    fn flush(&self) {}
}

pub fn create_client() -> TonClient {
	//log::set_boxed_logger(Box::new(SimpleLogger)).unwrap();
	//log::set_max_level(log::LevelFilter::Warn);

	println!("Network address {}", *NODE_ADDRESS);
	if *NODE_SE {
		println!("Node SE giver");
	} else {
		println!("Real net giver");
	}
	TonClient::new_with_base_url(&NODE_ADDRESS).unwrap()
}

pub fn now() -> u32 {
	std::time::SystemTime::now()
		.duration_since(std::time::SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_secs() as u32
}

#[test]
fn test_contracts() {
    // Deploy Messages

    let ton = create_client();

    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();

	let prepared_wallet_address = ton.contracts.get_deploy_address(
		WALLET_ABI.to_string().into(),
		&WALLET_IMAGE,
		None,
		&keys.public,
		0).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address, None);

    let deploy_result = ton.contracts.deploy(
		WALLET_ABI.to_string().into(),
		&WALLET_IMAGE,
		None,
		json!({}).to_string().into(),
		None,
		&keys,
		0).unwrap();

	assert_eq!(prepared_wallet_address, deploy_result.address);
	assert!(!deploy_result.already_deployed);

	// check that second deploy returns `alreadyDeployed == true`
	let deploy_result = ton.contracts.deploy(
		WALLET_ABI.to_string().into(),
		&WALLET_IMAGE,
		None,
		json!({}).to_string().into(),
		None,
		&keys,
		0).unwrap();

	assert_eq!(prepared_wallet_address, deploy_result.address);
	assert!(deploy_result.already_deployed);

	if *ABI_VERSION == 2 {
		// check header params passing
		let mut message = ton.contracts.create_run_message(
			&deploy_result.address,
			WALLET_ABI.to_string().into(),
			"createOperationLimit",
			Some(json!({
				"expire": 123
			}).to_string().into()),
			json!({
				"value": 123
			}).to_string().into(),
			Some(&keys),
			None
		).unwrap();

		assert_eq!(message.expire, Some(123));
		// set valid expire value in order to send message (core checks that message is not expired yet)
		message.expire = Some(now() + 10);

		let result = ton.contracts.process_message(message, None, None, false);

		match result.unwrap_err().0 {
			crate::error::TonErrorKind::InnerSdkError(err) => {
				println!("{:#?}", err);
				assert_eq!(err.code, 3025); // 3025 - tvm execution failed
				assert_eq!(&err.data["original_error"]["code"], 1006); // 1006 - message expired
			}
			_ => panic!("InnerSdkError expected")
		}
	};

    let result = ton.contracts.run(
        &deploy_result.address,
        WALLET_ABI.to_string().into(),
		"createOperationLimit",
		None,
        json!({
			"value": 123
		}).to_string().into(),
        Some(&keys)).unwrap();
    println!("{:#?}", result)
}

#[test]
fn test_call_aborted_transaction() {
	use crate::error::{TonError, TonErrorKind::InnerSdkError};

    let ton = create_client();

    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();

	let prepared_wallet_address = ton.contracts.get_deploy_address(
		SIMPLE_WALLET_ABI.to_string().into(),
		&SIMPLE_WALLET_IMAGE,
		None,
		&keys.public,
		0).unwrap();

	get_grams_from_giver(&ton, &prepared_wallet_address, None);

    let address = ton.contracts.deploy(
		SIMPLE_WALLET_ABI.to_string().into(),
		&SIMPLE_WALLET_IMAGE,
		None,
		json!({}).to_string().into(),
		None,
		&keys,
		0)
	.unwrap()
	.address;

	assert_eq!(prepared_wallet_address, address);

    let result = ton.contracts.run(
        &address,
        SIMPLE_WALLET_ABI.to_string().into(),
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

	println!("{:#}", result);

	match result {
		TonError(InnerSdkError(err), _) => {
			assert_eq!(&err.source, "node");
			assert_eq!(err.code, 3025);
			assert_eq!(&err.data["phase"], "computeVm");
			assert_eq!(&err.data["exit_code"], 101);
		},
		_ => panic!(),
	};
}

pub fn get_grams_from_giver(ton: &TonClient, account: &TonAddress, value: Option<u64>) {
	if *NODE_SE {
		ton.contracts.run(
			&GIVER_ADDRESS,
			GIVER_ABI.to_string().into(),
			"sendGrams",
			None,
			json!({
				"dest": account.to_string(),
				"amount": value.unwrap_or(500_000_000u64)
			}).to_string().into(),
			None).unwrap();
	} else {
		ton.contracts.run(
			&WALLET_ADDRESS,
			GIVER_WALLET_ABI.to_string().into(),
			"sendTransaction",
			None,
			json!({
				"dest": account.to_string(),
				"value": value.unwrap_or(500_000_000u64),
				"bounce": false
			}).to_string().into(),
			WALLET_KEYS.as_ref()).unwrap();
	}

	// wait for grams recieving
	let wait_result = ton.queries.accounts.wait_for(
		json!({
			"id": { "eq": account.to_string() },
			"balance": { "gt": "0" }
		}).into(),
		"id balance"
	).unwrap();

	println!("wait result {}", wait_result);
}

pub fn deploy_with_giver(
	client: &TonClient, abi: JsonValue, image: &[u8], params: JsonValue, keypair: &Ed25519KeyPair
) -> TonAddress {
    let prepared_address = client.contracts.get_deploy_address(
        abi.clone(),
        image,
        None,
        &keypair.public,
        0).expect("Couldn't create key pair");

    get_grams_from_giver(client, &prepared_address, None);

    client.contracts.deploy(
        abi,
        image,
        None,
        params,
        None,
        keypair,
        0)
        .expect("Couldn't deploy contract")
    .address
}

#[test]
fn test_decode_input() {
	if *ABI_VERSION == 1 {
		return
	}

    let body = "te6ccgEBAgEAxAAB4cN+DrKSpIGXl1Hiw3nKO1DuMz+2bdqiDQs+ls0Hg4AoQ4mIlgG5zmZHta3KuhGVa9OzWNuOLg30kPt7jgHlcQUAAAC4hj72ui88/bSiH7zDERERERERERERERERERERERERERERERERERERERERERFAAQCb4x6KrJjwAyuS43YYq6ijEXqYPNlXuuEg3inm/Xzrjp2AHzifK5Wl2DCMGV4PhQHgA6ugk3JnpzNe9amIHu+pZgYAAAAAJUC+QAAAKjAQ";
	let body = base64::decode(body).unwrap();

	let ton = TonClient::default().unwrap();

	let result = ton.contracts.decode_input_message_body(
		SUBSCRIBE_ABI.to_string().into(), &body, false
	).expect("Couldn't parse body");

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

	let ton = TonClient::default().unwrap();

    let keys: Ed25519KeyPair = ton.crypto.generate_ed25519_keys().unwrap();

	let wallet_address1 = ton.contracts.get_deploy_address(
		WALLET_ABI.to_string().into(),
		&WALLET_IMAGE,
		Some(json!({
			"subscription": subscription_address1,
            "owner": "0x".to_owned() + &keys.public.to_string(),
		}).to_string().into()),
		&keys.public,
		0).unwrap();

	let wallet_address2 = ton.contracts.get_deploy_address(
		WALLET_ABI.to_string().into(),
		&WALLET_IMAGE,
		Some(json!({
			"subscription": subscription_address2,
			"owner": "0x".to_owned() + &keys.public.to_string(),
		}).to_string().into()),
		&keys.public,
		0).unwrap();

	assert_ne!(wallet_address1, wallet_address2);
}

#[test]
fn test_deploy_data() {
	if *ABI_VERSION == 2 {
		return;
	}

	let ton = TonClient::default().unwrap();

	let key: Ed25519Public = serde_json::from_value(serde_json::Value::from("1111111111111111111111111111111111111111111111111111111111111111")).unwrap();
	let subscription_addess = "0:2222222222222222222222222222222222222222222222222222222222222222";

	// only key
    let result = ton.contracts.get_deploy_data(
		None,
		None,
		None,
		&key,
		None
	).unwrap();

	//println!("data {}", base64::encode(&result.data));

	assert_eq!(result, ResultOfGetDeployData{
		address: None,
		image: None,
		data: base64::decode("te6ccgEBAgEAKAABAcABAEPQBERERERERERERERERERERERERERERERERERERERERERg").unwrap()
	});

	// image and key
	let result = ton.contracts.get_deploy_data(
		None,
		Some(&WALLET_IMAGE),
		None,
		&key,
		None
	).unwrap();

	// println!("data {}", base64::encode(&result.data));
	// println!("image {}", base64::encode(&result.image.as_ref().unwrap()));
	// println!("address {}", result.address.as_ref().unwrap());

	assert_eq!(result, ResultOfGetDeployData{
		image: Some(base64::decode("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").unwrap()),
		address: Some(TonAddress::from_str("0:16c81b0bc7d7773e02a5baed5e217459b896b066fb8f95aae1fd669ce72f36c5").unwrap()),
        data: base64::decode("te6ccgEBBQEANQABAcABAgPPIAQCAQHeAwAD0CAAQdiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIjA==").unwrap(),
	});

	// init data and key
	let result = ton.contracts.get_deploy_data(
		Some(WALLET_ABI.to_string().into()),
		None,
		Some(json!({
			"subscription": subscription_addess,
            "owner": format!("0x{}", serde_json::to_value(&key).unwrap().as_str().unwrap()),
		}).to_string().into()),
		&key,
		None
	).unwrap();

	//println!("data {}", base64::encode(&result.data));

	assert_eq!(result, ResultOfGetDeployData{
		image: None,
		address: None,
		data: base64::decode("te6ccgEBBgEAegABAcABAgPOYAUCAgOsoAQDAEMgAREREREREREREREREREREREREREREREREREREREREREUAEEERERERERERERERERERERERERERERERERERERERERERGAAQdhERERERERERERERERERERERERERERERERERERERERERg==").unwrap(),
	});

	// all
	let result = ton.contracts.get_deploy_data(
		Some(WALLET_ABI.to_string().into()),
		Some(&WALLET_IMAGE),
		Some(json!({
			"subscription": subscription_addess,
            "owner": format!("0x{}", serde_json::to_value(&key).unwrap().as_str().unwrap()),
		}).to_string().into()),
		&key,
		Some(-1)
	).unwrap();

	// println!("data {}", base64::encode(&result.data));
	// println!("image {}", base64::encode(&result.image.as_ref().unwrap()));
	// println!("address {}", result.address.as_ref().unwrap());

	assert_eq!(result, ResultOfGetDeployData{
		image: Some(base64::decode("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").unwrap()),
		address: Some(TonAddress::from_str("-1:6195d78a0aae01af3584df743d3b2b08ceeff2a4e624a39d5b67fe1da8f5eb26").unwrap()),
		data: base64::decode("te6ccgEBCQEAhwABAcABAgPOYAUCAgOsoAQDAEMgAREREREREREREREREREREREREREREREREREREREREREUAEEERERERERERERERERERERERERERERERERERERERERERGACAWIIBgEB3gcAA9AgAEHYiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIw=").unwrap(),
	});
}

#[test]
fn test_retries() {
	if *ABI_VERSION == 1 {
		return;
	}
	let config = crate::client::TonClientConfig {
        base_url: Some(NODE_ADDRESS.to_string()),
        message_retries_count: Some(10),
        message_expiration_timeout: None,
        message_expiration_timeout_grow_factor: Some(1.1),
        message_processing_timeout: None,
        wait_for_timeout: None,
		access_key: None,
		out_of_sync_threshold: None,
    };
	let ton_client = TonClient::new(&config).unwrap();
	
    let keypair = ton_client.crypto.generate_ed25519_keys().expect("Couldn't create key pair");

    let address = deploy_with_giver(
		&ton_client,
        WALLET_ABI.to_string().into(),
        &WALLET_IMAGE,
        json!({}).to_string().into(),
        &keypair);

    let mut threads = vec![];
    for i in 0..10 {
		let address = address.clone();
		let config = config.clone();
		let keypair = keypair.clone();
        let str_address = address.to_string();
        let str_address = str_address[..str_address.len() - 2].to_owned() + &format!("{:02x}", i);
        let call = std::thread::spawn(move || {
			let client = TonClient::new(&config).unwrap();
			client.contracts.run(
				&address,
				WALLET_ABI.to_string().into(),
				"setSubscriptionAccount",
				None,
				json!({
					"addr": str_address
				}).into(),
				Some(&keypair)).unwrap();
		});

		threads.push(call);
    }

    for thread in threads {
		thread.join().unwrap();
	}
}

#[test]
fn test_messages() {
	let ton = create_client();

    let keypair = ton.crypto.generate_ed25519_keys().unwrap();

    let address = ton.contracts.get_deploy_address(
        WALLET_ABI.to_string().into(),
        &WALLET_IMAGE,
        None,
        &keypair.public,
		0).unwrap();

	get_grams_from_giver(&ton, &address, None);

    let message = ton.contracts.create_deploy_message(
        WALLET_ABI.to_string().into(),
        &WALLET_IMAGE,
        None,
        json!({}).into(),
        None,
		&keypair,
		0,
		None
	).unwrap();

	ton.contracts.process_message(
		message, None, None, false).unwrap();

	// check processing with result decoding
	let run_message = ton.contracts.create_run_message(
		&address,
		WALLET_ABI.to_string().into(),
		"createOperationLimit",
		None,
		json!({
			"value": 100_000_000
		}).to_string().into(),
		Some(&keypair),
		None
	).unwrap();

	let state = ton.contracts.send_message(run_message.clone()).unwrap();

	// println!("network silent test");

	// let ton1 = TonClient::new_with_base_url("cinet.tonlabs.io").unwrap();

	// let wait_transaction_result = ton1.contracts.wait_for_transaction(
	// 	run_message.clone(), 
	// 	Some(WALLET_ABI.to_string().into()),
	// 	Some("createOperationLimit"),
	// 	state,
	// 	false
	// ).unwrap_err();

	// println!("Error returned {}", wait_transaction_result);

	// let error = test_errors::extract_inner_error(&wait_transaction_result);

	// let state = error.message_processing_state.unwrap();

	let wait_transaction_result = ton.contracts.wait_for_transaction(
		run_message, 
		Some(WALLET_ABI.to_string().into()),
		Some("createOperationLimit"),
		state,
		false
	).unwrap();

	let run_result = ton.contracts.process_transaction(
		&address,
		wait_transaction_result.transaction.clone().into(),
		Some(WALLET_ABI.to_string().into()),
		Some("createOperationLimit")
	).unwrap();

	assert_eq!(run_result.output, json!({"value0": "0x0"}));
	assert_eq!(wait_transaction_result, run_result);

	// check processing without result decoding
	let run_message = ton.contracts.create_run_message(
		&address,
		WALLET_ABI.to_string().into(),
		"createArbitraryLimit",
		None,
		json!({
			"value": 100_000_000,
			"period": 1
		}).to_string().into(),
		Some(&keypair),
		None
	).unwrap();

	let run_result = ton.contracts.process_message(run_message, None, None, true).unwrap();

	assert_eq!(run_result.output, json!(null));

	// check processing transaction without output messages
	let run_message = ton.contracts.create_run_message(
		&address,
		WALLET_ABI.to_string().into(),
		"sendTransaction",
		None,
		json!({
			"dest": WALLET_ADDRESS.to_string(),
			"value": 100_000_000,
			"bounce": false
		}).to_string().into(),
		Some(&keypair),
		None
	).unwrap();

	let run_result = ton.contracts.process_message(
		run_message, Some(WALLET_ABI.to_string().into()), Some("sendTransaction"), false).unwrap();

	assert_eq!(run_result.output, json!(null));
}
