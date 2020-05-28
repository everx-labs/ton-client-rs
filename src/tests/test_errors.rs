/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and
 * limitations under the License.
 */

use crate::tests::*;
use crate::error::{TonError, TonErrorKind, InnerSdkError};

fn extract_inner_error(error: &TonError) -> InnerSdkError {
    match error {
		TonError(TonErrorKind::InnerSdkError(err), _) => {
            println!("{:#?}", err);
			err.clone()
		},
		_ => panic!(),
	}
}

fn check_error(error: &TonError, main_code: isize, extended_code: Option<isize>) {
    let err = extract_inner_error(error);
    
    assert_eq!(err.code, main_code);
    if let Some(code) = extended_code {
        assert_eq!(&err.data["extended_code"], code);
    }
}

#[test]
fn test_errors() {
    let config = crate::client::TonClientConfig {
        base_url: Some(NODE_ADDRESS.to_string()),
        message_retries_count: Some(0),
        message_expiration_timeout: Some(2_000),
        message_expiration_timeout_grow_factor: None,
        message_processing_timeout: None,
        message_processing_timeout_grow_factor: None,
        wait_for_timeout: None,
        access_key: None,
    };
    let ton_client = TonClient::new(&config).unwrap();
    let std_ton_client = create_client();

    let keypair = ton_client.crypto.generate_ed25519_keys().expect("Couldn't create key pair");

    let hello_address = ton_client.contracts.get_deploy_address(
        &HELLO_ABI, &HELLO_IMAGE, None, &keypair.public, 0
    ).expect("Couldn't calculate address");

    // deploy without balance
    let result = ton_client.contracts.deploy(
        &HELLO_ABI, &HELLO_IMAGE, None, json!({}).to_string().into(), None, &keypair, 0
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
        &HELLO_ABI, &HELLO_IMAGE, None, json!({}).to_string().into(), None, &keypair, 0, None
    ).unwrap();

    let main_code = if *ABI_VERSION == 2 {
        1006    // message expired
    } else {
        1003    // transaction wait timeout
    };

    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32;

    // process message with error resolving
    let result = ton_client.contracts.process_message(msg.message.clone(), None, None, None).unwrap_err();

    check_error(&result, main_code, Some(1016)); // low balance

    let account = ton_client.queries.accounts.query(
        &json!({"id": { "eq": hello_address.to_string() }}).to_string(),
        "id acc_type code data balance balance_other { currency value } last_paid",
        None, None
    ).unwrap()[0].clone();

    // manual resolving
    let error = extract_inner_error(&result);
    let result = ton_client.contracts.resolve_error(
        &hello_address, Some(&account.to_string()), msg.message, time, error,
    ).unwrap_err();

    check_error(&result, main_code, Some(1016)); // low balance

    // run before deploy
    super::get_grams_from_giver(&std_ton_client, &hello_address, None);
    let result = ton_client.contracts.run(
        &hello_address, &HELLO_ABI, "touch", None, json!({}).to_string().into(), Some(&keypair)
    ).unwrap_err();

    check_error(&result, main_code, Some(1015)); // code missing

    // normal deploy
    std_ton_client.contracts.deploy(
        &HELLO_ABI, &HELLO_IMAGE, None, json!({}).to_string().into(), None, &keypair, 0
    ).unwrap();

    // unsigned message
    let result = ton_client.contracts.run(
        &hello_address,
        &HELLO_ABI,
        "sendAllMoney",
        None,
        json!({
            "dest_addr": WALLET_ADDRESS.to_string()
        }).to_string().into(),
        None
    ).unwrap_err();

    check_error(&result, main_code, Some(3025)); // contract execution failed

    std_ton_client.contracts.run(
        &hello_address,
        &HELLO_ABI,
        "sendAllMoney",
        None,
        json!({
            "dest_addr": WALLET_ADDRESS.to_string()
        }).to_string().into(),
        Some(&keypair)
    ).unwrap();

    let result = ton_client.contracts.run(
        &hello_address, &HELLO_ABI, "touch", None, json!({}).to_string().into(), Some(&keypair)
    ).unwrap_err();

    check_error(&result, main_code, Some(1016)); // low balance
}
