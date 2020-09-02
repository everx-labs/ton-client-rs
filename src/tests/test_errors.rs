/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

use crate::tests::*;
use crate::error::{TonError, TonErrorKind, InnerSdkError};

pub fn extract_inner_error(error: &TonError) -> InnerSdkError {
    //println!("{:#}", error);
    match error {
		TonError(TonErrorKind::InnerSdkError(err), _) => {
			err.clone()
		},
		_ => panic!(),
	}
}

pub fn check_error(error: &TonError, main_code: isize, original_code: Option<isize>) {
    let err = extract_inner_error(error);
    
    assert_eq!(err.code, main_code);
    if let Some(code) = original_code {
        assert_eq!(&err.data["original_error"]["code"], code);
    } else {
        assert!(err.data["original_error"]["code"].is_null())
    }
}

#[test]
fn test_errors() {
    let config = crate::client::TonClientConfig {
        base_url: Some(NODE_ADDRESS.to_string()),
        message_retries_count: Some(0),
        message_expiration_timeout: Some(2_000),
        message_expiration_timeout_grow_factor: None,
        message_processing_timeout: if *ABI_VERSION == 1 { Some(10_000) } else { None },
        wait_for_timeout: None,
        access_key: None,
        out_of_sync_threshold: None,
    };
    let ton_client = TonClient::new(&config).unwrap();
    let std_ton_client = create_client();

    let keypair = ton_client.crypto.generate_ed25519_keys().expect("Couldn't create key pair");

    let hello_address = ton_client.contracts.get_deploy_address(
        HELLO_ABI.to_string().into(), &HELLO_IMAGE, None, &keypair.public, 0
    ).expect("Couldn't calculate address");

    // deploy without balance
    let result = ton_client.contracts.deploy(
        HELLO_ABI.to_string().into(), &HELLO_IMAGE, None, json!({}).into(), None, &keypair, 0
    ).unwrap_err();

    let main_code = if *NODE_SE {
		1015    // compute phase skipped with NoState reason 
	} else {
		1014    // account missing while trying to investigate transaction
    };
    
    check_error(&result, main_code, None);
    
    super::get_grams_from_giver(&std_ton_client, &hello_address, Some(1000));

    // deploy with low balance
    let msg = ton_client.contracts.create_deploy_message(
        HELLO_ABI.to_string().into(), &HELLO_IMAGE, None, json!({}).into(), None, &keypair, 0, None
    ).unwrap();

    let real_original_code = if *ABI_VERSION == 2 {
        1006    // message expired
    } else {
        1012    // transaction wait timeout
    };

    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32;

    // process message with error resolving
    let result = ton_client.contracts.process_message(msg.clone(), None, None, false).unwrap_err();

    if *NODE_SE {
        check_error(&result, 3025, None); // 3025 - tvm execution failed                 
    } else {
        check_error(&result, 1016, Some(real_original_code))    // 1016 - low balance
    };

    let account = ton_client.queries.accounts.query(
        json!({"id": { "eq": hello_address.to_string() }}).into(),
        "id acc_type code data balance balance_other { currency value } last_paid",
        None, None
    ).unwrap()[0].clone();

    // manual resolving
    let error = extract_inner_error(&result);
    let code = error.code;
    let result = ton_client.contracts.resolve_error(
        &hello_address,
        Some(account.into()),
        msg,
        time,
        error,
    ).unwrap_err();

    check_error(&result, 1016, Some(code));    // 1016 - low balance

    // ABI version 1 messages don't expire so previous deploy message can be processed after
    // increasing balance. Need to wait until message will be rejected by all validators
    if *ABI_VERSION == 1 && !*NODE_SE{
        std::thread::sleep(std::time::Duration::from_secs(40));
    }

    // run before deploy
    super::get_grams_from_giver(&std_ton_client, &hello_address, None);
    let result = ton_client.contracts.run(
        &hello_address, HELLO_ABI.to_string().into(), "touch", None, json!({}).to_string().into(), Some(&keypair)
    ).unwrap_err();

    if *NODE_SE {
        check_error(&result, 1015, None) // 1015 - code missing              
    } else {
        check_error(&result, 1015, Some(real_original_code)) // 1015 - code missing
    };

    // normal deploy
    std_ton_client.contracts.deploy(
        HELLO_ABI.to_string().into(), &HELLO_IMAGE, None, json!({}).to_string().into(), None, &keypair, 0
    ).unwrap();

    // unsigned message
    let result = ton_client.contracts.run(
        &hello_address,
        HELLO_ABI.to_string().into(),
        "sendAllMoney",
        None,
        json!({
            "dest_addr": WALLET_ADDRESS.to_string()
        }).to_string().into(),
        None
    ).unwrap_err();

    if *NODE_SE {
        check_error(&result, 3025, None) // 3025 - tvm execution failed
    } else {
        check_error(&result, 3025, Some(real_original_code)) // 3025 - tvm execution failed
    };

    std_ton_client.contracts.run(
        &hello_address,
        HELLO_ABI.to_string().into(),
        "sendAllMoney",
        None,
        json!({
            "dest_addr": WALLET_ADDRESS.to_string()
        }).to_string().into(),
        Some(&keypair)
    ).unwrap();

    let result = ton_client.contracts.run(
        &hello_address, HELLO_ABI.to_string().into(), "touch", None, json!({}).to_string().into(), Some(&keypair)
    ).unwrap_err();

    if *NODE_SE {
        check_error(&result, 1016, None) // 1016 - low balance
    } else {
        check_error(&result, 1016, Some(real_original_code)) // 1016 - low balance
    }
}
