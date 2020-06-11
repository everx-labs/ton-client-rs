/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

use crate::tests::*;
use crate::tests::create_client;
   

#[test]
fn test_hello() {
    let ton_client = create_client();

    let keypair = ton_client.crypto.generate_ed25519_keys().expect("Couldn't create key pair");

    let prepared_address = ton_client.contracts.get_deploy_address(
        HELLO_ABI.to_string().into(),
        &HELLO_IMAGE,
        None,
        &keypair.public,
        0).expect("Couldn't create key pair");

    super::get_grams_from_giver(&ton_client, &prepared_address, None);

    let hello_address = ton_client.contracts.deploy(
        HELLO_ABI.to_string().into(),
        &HELLO_IMAGE,
        None,
        json!({}).to_string().into(),
        None,
        &keypair,
        0)
        .expect("Couldn't deploy contract")
    .address;

    ton_client.contracts.run(
    &hello_address,
    HELLO_ABI.to_string().into(),
    "touch",
    None,
    json!({}).to_string().into(),
    Some(&keypair))
    .expect("Couldn't run contract");

    let response = ton_client.contracts.run_local(
        &hello_address,
        None,
        HELLO_ABI.to_string().into(),
        "sayHello",
        None,
        json!({}).to_string().into(),
        None,
        None,
        false,
    ).expect("Couldn't runLocal sayHello");

    println!("Hello contract was responded to sayHello: {:#?}", response);
}
