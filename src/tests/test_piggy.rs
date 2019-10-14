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

use crate::{TonClient, OrderBy, SortDirection};
use crate::tests::{WALLET_ABI, WALLET_CODE_BASE64};
use futures::stream::Stream;


const ACCOUNT_FIELDS: &str = r#"
    id
    addr {
        ...on MsgAddressIntAddrStdVariant {
            AddrStd {
                workchain_id
                address
            }
        }
    }
"#;

#[test]
fn test_piggy() {
    let ton = TonClient::new_with_base_url("http://0.0.0.0").unwrap();
    let keypair = ton.crypto.generate_ed25519_keys().unwrap();

    let prepared_address = ton.contracts.get_deploy_address(
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        &keypair).unwrap();

    super::get_grams_from_giver(&ton, &prepared_address);

    let wallet_address = ton.contracts.deploy(
        WALLET_ABI,
        &base64::decode(WALLET_CODE_BASE64).unwrap(),
        json!({}).to_string().into(), &keypair).unwrap();

    let prepared_address = ton.contracts.get_deploy_address(
        &base64::decode(PIGGY_BANK_CODE_BASE64).unwrap(),
        &keypair).unwrap();

    super::get_grams_from_giver(&ton, &prepared_address);

    let piggy_bank_address = ton.contracts.deploy(
        PIGGY_BANK_ABI,
        &base64::decode(PIGGY_BANK_CODE_BASE64).unwrap(),
        json!({
	        "amount": 123,
	        "goal": [83, 111, 109, 101, 32, 103, 111, 97, 108]
        }).to_string().into(),
        &keypair,
    ).unwrap();

    println!("address {}", piggy_bank_address);

    // check queries on real data
    let query_result = ton.queries.accounts.query(
        &json!({
            "id": {
                "eq": piggy_bank_address.to_string()
            }
        }).to_string(),
        ACCOUNT_FIELDS,
        Some(OrderBy{ path: "id".to_owned(), direction: SortDirection::Ascending }),
        Some(5)).unwrap();

    assert_eq!(
        query_result[0],
        json!({
            "id": piggy_bank_address.to_string(),
            "addr": {
                "AddrStd": {
                    "address": piggy_bank_address.to_string(),
                    "workchain_id": 0
                }
            }
        }));

    let wait_for_result = ton.queries.accounts.wait_for(
        &json!({
            "id": {
                "eq": piggy_bank_address.to_string()
            }
        }).to_string(),
        ACCOUNT_FIELDS).unwrap();

    assert_eq!(
        wait_for_result,
        json!({
            "id": piggy_bank_address.to_string(),
            "addr": {
                "AddrStd": {
                    "address": piggy_bank_address.to_string(),
                    "workchain_id": 0
                }
            }
        }));


    let get_goal_answer = ton.contracts.run_local(
        &piggy_bank_address,
        None,
        PIGGY_BANK_ABI,
        "getGoal",
        json!({}).to_string().into(), None).unwrap();

    println!("getGoal answer {}", get_goal_answer);

    let prepared_address = ton.contracts.get_deploy_address(
        &base64::decode(SUBSCRIBE_CODE_BASE64).unwrap(),
        &keypair).unwrap();

    super::get_grams_from_giver(&ton, &prepared_address);

    let subscription_constructor_params = json!({ "wallet" : format!("0x{}", wallet_address)}).to_string().into();
    let subscripition_address = ton.contracts.deploy(
        SUBSCRIBE_ABI,
        &base64::decode(SUBSCRIBE_CODE_BASE64).unwrap(),
        subscription_constructor_params,
        &keypair,
    ).unwrap();
    let set_subscription_params = json!({ "addr": format!("0x{}", subscripition_address) }).to_string().into();

    // subscribe for updates 
    let subscribe_stream = ton.queries.accounts.subscribe(
        &json!({
            "id": {
                "eq": subscripition_address.to_string()
            }
        }).to_string(),
        ACCOUNT_FIELDS).unwrap();

    let _set_subscription_answer = ton.contracts.run(
        &wallet_address,
        WALLET_ABI,
        "setSubscriptionAccount",
        set_subscription_params,
        Some(&keypair));

    let subscr_id_str = hex::encode(&[0x11; 32]);
    let pubkey_str = keypair.public.clone();

    let _subscribe_answer = ton.contracts.run(
        &subscripition_address,
        SUBSCRIBE_ABI,
        "subscribe",
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
            "pubkey" : format!("0x{}", pubkey_str),
            "to": format!("0x{}", piggy_bank_address),
            "value" : 123,
            "period" : 456
        }).to_string().into(),
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
            "addr": {
                "AddrStd": {
                    "address": subscripition_address.to_string(),
                    "workchain_id": 0
                }
            }
        }));

    let subscr_id_str = hex::encode(&[0x22; 32]);
    let _subscribe_answer = ton.contracts.run(
        &subscripition_address,
        SUBSCRIBE_ABI,
        "subscribe",
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
            "pubkey" : format!("0x{}", pubkey_str),
            "to": format!("0x{}", piggy_bank_address),
            "value" : 5000000000 as i64,
            "period" : 86400
        }).to_string().into(),
        Some(&keypair)
    ).unwrap();

    let subscriptions = ton.contracts.run(
        &subscripition_address,
        SUBSCRIBE_ABI,
        "getSubscription",
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
        }).to_string().into(),
        Some(&keypair)
    ).unwrap();

    println!("getSubscription answer {}", subscriptions);
}

const PIGGY_BANK_CODE_BASE64: &str = r#"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"#;
const PIGGY_BANK_ABI: &str = r#"
{
    "ABI version": 1,
    "functions": [
        {
            "name": "constructor",
            "inputs": [
                {"name":"amount","type":"uint64"},
                {"name":"goal","type":"uint8[]"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "getGoal",
            "inputs": [
            ],
            "outputs": [
                {"name":"value0","type":"uint8[]"}
            ]
        },
        {
            "name": "getTargetAmount",
            "inputs": [
            ],
            "outputs": [
                {"name":"value0","type":"uint64"}
            ]
        },
        {
            "name": "transfer",
            "inputs": [
                {"name":"to","type":"uint256"}
            ],
            "outputs": [
            ]
        }
    ],
    "events": [
    ],
    "data": [
        {"key":100,"name":"targetGoal","type":"uint8[]"},
        {"key":101,"name":"targetAmount","type":"uint64"}
    ]
}"#;


const SUBSCRIBE_CODE_BASE64: &str = r#"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"#;
const SUBSCRIBE_ABI: &str = r#"
{
    "ABI version": 1,
    "functions": [
        {
            "name": "constructor",
            "inputs": [
                {"name":"wallet","type":"uint256"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "getWallet",
            "inputs": [
            ],
            "outputs": [
                {"name":"value0","type":"uint256"}
            ]
        },
        {
            "name": "getSubscription",
            "inputs": [
                {"name":"subscriptionId","type":"uint256"}
            ],
            "outputs": [
                {"components":[{"name":"pubkey","type":"uint256"},{"name":"to","type":"uint256"},{"name":"value","type":"uint64"},{"name":"period","type":"uint32"},{"name":"start","type":"uint32"},{"name":"status","type":"uint8"}],"name":"value0","type":"tuple"}
            ]
        },
        {
            "name": "subscribe",
            "inputs": [
                {"name":"subscriptionId","type":"uint256"},
                {"name":"pubkey","type":"uint256"},
                {"name":"to","type":"uint256"},
                {"name":"value","type":"uint64"},
                {"name":"period","type":"uint32"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "cancel",
            "inputs": [
                {"name":"subscriptionId","type":"uint256"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "executeSubscription",
            "inputs": [
                {"name":"subscriptionId","type":"uint256"}
            ],
            "outputs": [
            ]
        }
    ],
    "events": [
    ],
    "data": [
        {"key":100,"name":"mywallet","type":"uint256"}
    ]
}"#;

