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
use crate::tests::test_errors::check_error;

#[test]
fn test_local_run() {
    let config = crate::client::TonClientConfig {
        base_url: Some(NODE_ADDRESS.to_string()),
        message_retries_count: None,
        message_expiration_timeout: None,
        message_expiration_timeout_grow_factor: None,
        message_processing_timeout: None,
        message_processing_timeout_grow_factor: None,
        wait_for_timeout: Some(5_000),
        access_key: None,
    };
    let ton_client = TonClient::new(&config).unwrap();
    let std_ton_client = create_client();

    let keypair = ton_client.crypto.generate_ed25519_keys().expect("Couldn't create key pair");

    let abi: crate::JsonValue = HELLO_ABI.to_string().into();

    let address = ton_client.contracts.get_deploy_address(
        abi.clone(), &HELLO_IMAGE, None, &keypair.public, 0
    ).unwrap();

    super::get_grams_from_giver(&std_ton_client, &address, None);

    let msg = ton_client.contracts.create_deploy_message(
        abi.clone(), &HELLO_IMAGE, None, json!({}).into(), None, &keypair, 0, None
    ).expect("Couldn't create deploy message");

    // check full run of deploy - contract should become active
    let result = ton_client.contracts.run_local_msg(
        &address, None, msg.message.clone(), None, None, None, true).unwrap();
        
    assert!(result.fees.is_some());
    assert_eq!(result.updated_account.unwrap()["acc_type"], 1); // account active

    println!("{:#?}", result.fees.unwrap());

    let result_err = ton_client.contracts.run_local_msg(
        &address, None, msg.message.clone(), None, None, None, false).unwrap_err();

    check_error(&result_err, 1015, None); // code missing

    let result = ton_client.contracts.deploy(
        abi.clone(), &HELLO_IMAGE, None, json!({}).into(), None, &keypair, 0
    ).expect("Couldn't deploy contract");

    println!("{:#?}", result.fees);

    ton_client.contracts.run(
        &address, abi.clone(), "touch", None, json!({}).into(), Some(&keypair)
    ).expect("Couldn't run contract");

    ton_client.contracts.run_local(
        &address, None, abi.clone(), "sayHello", None, json!({}).into(), None, None, false,
    ).expect("Couldn't runLocal sayHello");

    // check full run of get method - should fail as contract don't accept
    let response_err = ton_client.contracts.run_local(
        &address, None, abi.clone(), "sayHello", None, json!({}).into(), None, None, true,
    ).unwrap_err();

    check_error(&response_err, 3025, None); // tvm execution failed: no accept

    // contract saves transaction time in `touch` and return it in `sayHello`
    let time = now() + 3;

    // emulate local transaction to recieve new account state
    let result = ton_client.contracts.run_local(
        &address, None, abi.clone(), "touch", None, json!({}).into(), Some(&keypair), Some(time), true
    ).unwrap();

    let local_response = ton_client.contracts.run_local(
        &address,
        Some(result.updated_account.unwrap().into()),
        abi.clone(),
        "sayHello",
        None,
        json!({}).into(),
        None, None, false,
    ).expect("Couldn't runLocal sayHello");

    assert_eq!(local_response.output["value0"], format!("0x{:x}", time));
    assert!(local_response.fees.is_none());
    assert!(local_response.updated_account.is_none());
}
