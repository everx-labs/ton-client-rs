/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

use crate::{OrderBy, SortDirection};
use crate::tests::*;
use futures::stream::Stream;
use crate::tests::create_client;


const ACCOUNT_FIELDS: &str = r#"
    id
"#;

#[test]
fn test_piggy() {
    let ton = create_client();

    let keypair = ton.crypto.generate_ed25519_keys().unwrap();

    let prepared_address = ton.contracts.get_deploy_address(
        WALLET_ABI.to_string().into(),
        &WALLET_IMAGE,
        None,
        &keypair.public,
        0).unwrap();

    super::get_grams_from_giver(&ton, &prepared_address, None);

    let wallet_address = ton.contracts.deploy(
        WALLET_ABI.to_string().into(),
        &WALLET_IMAGE,
        None,
        json!({}).to_string().into(),
        None,
        &keypair,
        0)
    .unwrap()
    .address;

    let prepared_address = ton.contracts.get_deploy_address(
        PIGGY_BANK_ABI.to_string().into(),
        &PIGGY_BANK_IMAGE,
        None,
        &keypair.public,
        0).unwrap();

    super::get_grams_from_giver(&ton, &prepared_address, None);

    let piggy_bank_address = ton.contracts.deploy(
        PIGGY_BANK_ABI.to_string().into(),
        &PIGGY_BANK_IMAGE,
        None,
        json!({
	        "amount": 123,
	        "goal": "536f6d6520676f616c"
        }).to_string().into(),
        None,
        &keypair,
        0)
    .unwrap()
    .address;

    println!("address {}", piggy_bank_address);

    // check queries on real data
    let query_result = ton.queries.accounts.query(
        json!({
            "id": {
                "eq": piggy_bank_address.to_string()
            }
        }).into(),
        ACCOUNT_FIELDS,
        Some(OrderBy{ path: "id".to_owned(), direction: SortDirection::Ascending }),
        Some(5)).unwrap();

    assert_eq!(
        query_result[0],
        json!({
            "id": piggy_bank_address.to_string(),
        }));

    let wait_for_result = ton.queries.accounts.wait_for(
        json!({
            "id": {
                "eq": piggy_bank_address.to_string()
            }
        }).into(),
        ACCOUNT_FIELDS).unwrap();

    assert_eq!(
        wait_for_result,
        json!({
            "id": piggy_bank_address.to_string(),
        }));


    let get_goal_answer = ton.contracts.run_local(
        &piggy_bank_address,
        None,
        PIGGY_BANK_ABI.to_string().into(),
        "getGoal",
        None,
        json!({}).to_string().into(),
        None,
        None,
        false,
    ).unwrap();

    assert!(get_goal_answer.fees.is_none());

    println!("getGoal answer {:#?}", get_goal_answer);

    let prepared_address = ton.contracts.get_deploy_address(
        SUBSCRIBE_ABI.to_string().into(),
        &SUBSCRIBE_IMAGE,
        None,
        &keypair.public,
        0).unwrap();

    super::get_grams_from_giver(&ton, &prepared_address, None);

    let subscription_constructor_params = json!({
        "wallet" : wallet_address.to_string()
    }).to_string().into();

    let subscripition_address = ton.contracts.deploy(
        SUBSCRIBE_ABI.to_string().into(),
        &SUBSCRIBE_IMAGE,
        None,
        subscription_constructor_params,
        None,
        &keypair,
        0)
    .unwrap()
    .address;
    
    let set_subscription_params = json!({
            "addr": subscripition_address.to_string()
        }).to_string().into();

    // subscribe for updates 
    let subscribe_stream = ton.queries.accounts.subscribe(
        json!({
            "id": {
                "eq": subscripition_address.to_string()
            }
        }).into(),
        ACCOUNT_FIELDS).unwrap();

    let _set_subscription_answer = ton.contracts.run(
        &wallet_address,
        WALLET_ABI.to_string().into(),
        "setSubscriptionAccount",
        None,
        set_subscription_params,
        Some(&keypair)).unwrap();

    let subscr_id_str = hex::encode(&[0x11; 32]);
    let pubkey_str = keypair.public.clone();

    let _subscribe_answer = ton.contracts.run(
        &subscripition_address,
        SUBSCRIBE_ABI.to_string().into(),
        "subscribe",
        None,
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
            "pubkey" : format!("0x{}", pubkey_str),
            "to": piggy_bank_address.to_string(),
            "value" : 123,
            "period" : 456
        }).into(),
        Some(&keypair)
    ).unwrap();

    // check updates
    let subscribe_result = subscribe_stream
        .wait()
        .next()
        .unwrap()
        .unwrap();

    assert_eq!(
        subscribe_result,
        json!({
            "id": subscripition_address.to_string(),
        }));

    let subscr_id_str = hex::encode(&[0x22; 32]);

    let _subscribe_answer = ton.contracts.run(
        &subscripition_address,
        SUBSCRIBE_ABI.to_string().into(),
        "subscribe",
        None,
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
            "pubkey" : format!("0x{}", pubkey_str),
            "to": piggy_bank_address.to_string(),
            "value" : 5000000000 as i64,
            "period" : 86400
        }).into(),
        Some(&keypair)
    ).unwrap();

    let subscriptions = ton.contracts.run_local(
        &subscripition_address,
        None,
        SUBSCRIBE_ABI.to_string().into(),
        "getSubscription",
        None,
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
        }).into(),
        Some(&keypair),
        None,
        false,
    ).unwrap();

    println!("getSubscription answer {:#?}", subscriptions);
}
