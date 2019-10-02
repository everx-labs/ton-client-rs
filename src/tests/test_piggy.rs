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

use crate::tests::{WALLET_ABI, WALLET_CODE_BASE64};
use crate::{OrderBy, SortDirection, TonClient};
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

    let wallet_address = ton
        .contracts
        .deploy(
            WALLET_ABI,
            &base64::decode(WALLET_CODE_BASE64).unwrap(),
            json!({}).to_string().into(),
            &keypair,
        )
        .unwrap();

    let piggy_bank_address = ton
        .contracts
        .deploy(
            PIGGY_BANK_ABI,
            &base64::decode(PIGGY_BANK_CODE_BASE64).unwrap(),
            json!({
                "amount": 123,
                "goal": [83, 111, 109, 101, 32, 103, 111, 97, 108]
            })
            .to_string()
            .into(),
            &keypair,
        )
        .unwrap();

    // check queries on real data
    let query_result = ton
        .queries
        .accounts
        .query(
            &json!({
                "id": {
                    "eq": piggy_bank_address.to_string()
                }
            })
            .to_string(),
            ACCOUNT_FIELDS,
            Some(OrderBy {
                path: "id".to_owned(),
                direction: SortDirection::Ascending,
            }),
            Some(5),
        )
        .unwrap();

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
        })
    );

    let wait_for_result = ton
        .queries
        .accounts
        .wait_for(
            &json!({
                "id": {
                    "eq": piggy_bank_address.to_string()
                }
            })
            .to_string(),
            ACCOUNT_FIELDS,
        )
        .unwrap();

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
        })
    );

    let get_goal_answer = ton
        .contracts
        .run_local(
            &piggy_bank_address,
            None,
            PIGGY_BANK_ABI,
            "getGoal",
            json!({}).to_string().into(),
            None,
        )
        .unwrap();

    println!("getGoal answer {}", get_goal_answer);

    let subscription_constructor_params = json!({ "wallet": format!("x{}", wallet_address) })
        .to_string()
        .into();
    let subscripition_address = ton
        .contracts
        .deploy(
            SUBSCRIBE_ABI,
            &base64::decode(SUBSCRIBE_CODE_BASE64).unwrap(),
            subscription_constructor_params,
            &keypair,
        )
        .unwrap();
    let set_subscription_params = json!({ "address": format!("x{}", subscripition_address) })
        .to_string()
        .into();

    // subscribe for updates
    let subscribe_stream = ton
        .queries
        .accounts
        .subscribe(
            &json!({
                "id": {
                    "eq": subscripition_address.to_string()
                }
            })
            .to_string(),
            ACCOUNT_FIELDS,
        )
        .unwrap();

    let _set_subscription_answer = ton.contracts.run(
        &wallet_address,
        WALLET_ABI,
        "setSubscriptionAccount",
        set_subscription_params,
        Some(&keypair),
    );

    let subscr_id_str = hex::encode(&[0x11; 32]);
    let pubkey_str = keypair.public.clone();

    let _subscribe_answer = ton
        .contracts
        .run(
            &subscripition_address,
            SUBSCRIBE_ABI,
            "subscribe",
            json!({
                "subscriptionId" : format!("x{}", subscr_id_str),
                "pubkey" : format!("x{}", pubkey_str),
                "to": format!("x{}", piggy_bank_address),
                "value" : 123,
                "period" : 456
            })
            .to_string()
            .into(),
            Some(&keypair),
        )
        .unwrap();

    // check updates
    let subscribe_result = subscribe_stream.wait().next().unwrap().unwrap();

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
        })
    );

    let subscr_id_str = hex::encode(&[0x22; 32]);
    let _subscribe_answer = ton
        .contracts
        .run(
            &subscripition_address,
            SUBSCRIBE_ABI,
            "subscribe",
            json!({
                "subscriptionId" : format!("x{}", subscr_id_str),
                "pubkey" : format!("x{}", pubkey_str),
                "to": format!("x{}", piggy_bank_address),
                "value" : 5000000000 as i64,
                "period" : 86400
            })
            .to_string()
            .into(),
            Some(&keypair),
        )
        .unwrap();

    let subscriptions = ton
        .contracts
        .run(
            &subscripition_address,
            SUBSCRIBE_ABI,
            "getSubscription",
            json!({
                "subscriptionId" : format!("x{}", subscr_id_str),
            })
            .to_string()
            .into(),
            Some(&keypair),
        )
        .unwrap();

    println!("getSubscription answer {}", subscriptions);
}

const PIGGY_BANK_CODE_BASE64: &str = r#"te6ccgECHwEAApUAAgE0AgEAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATL/AIn0BSHBAZN49KCbePQN8rSAIPSh8jPiAwEBwAQCASAGBQAp/+ABw8AEhcfABAdMHAfJr0x8B8AKAgHWEQcBAawIAgFIDAkCAWILCgCjt9kOJRb7UTQ1DBxAXj0DjCCEE9kOJRwyMsHyx/OjjGOK8hyz0Fyz0Byz0Bwzws/cM8LH3HPQFzPNQHPMaS+lXHPQM8TlXHPQc8R4snYcPsA2IAA/t1nVP8x1j8BcG149BZxAXj0FsjM7UTQ1v8wzxbJ7VSACAUgQDQEJuWWEepAOAf4xgQEAmAGLCtcmAdcY2DDXC//tR28QbxdvEO1E0NQwcAF49A4w0z8wIbuOTyCAD4BkqYVcoTKLCHBYjj7++QBTbmRCZHlJbnQB7UdvEG8Y+kJvEsjPhkDKB8v/ydCOF8jPhSDPigBAgQEAz0DOAfoCgGvPQM7J2HD7ANjfAYsIDwBwcBIDAe1HbxBvGPpCbxLIz4ZAygfL/8nQjhfIz4Ugz4oAQIEBAM9AzgH6AoBrz0DOydiBAID7ADAAo7iKB9ILfaiaGoYOAC8egcYaZ+YZGfKAAigfSFln8cYxxXkOWeguWegOWegOGeFn7hnhY+456AuZ5qA55jSX0q456Bnicq456DniPFk7Dh9gGxACASAeEgEBMBMCA8/AFRQAGTQ1ygF+kD6QPoAbwSABbQg0wcB8muJ9AWAIPQK8qnXCwCOGSDHAvJt1SDHAPJtIfkBAe1E0NcL//kQ8qiXIMcCktQx3+KAWAQHAFwIBSBsYAgFiGhkACbfZDiUQAAm3WdU/0AIBSB0cAAm5ZYR6mAAJuIoH0ggABTbMIA=="#;
const PIGGY_BANK_ABI: &str = r#"
{
    "ABI version": 0,
    "functions": [{
        "name": "transfer",
        "signed": true,
        "inputs": [{"name": "to", "type": "bits256"}],
        "outputs": []
    }, {
        "name": "getTargetAmount",
        "inputs": [],
        "outputs": [{"name": "amount", "type": "uint64"}]
    }, {
        "name": "getGoal",
        "inputs": [],
        "outputs": [{"name": "goal", "type": "uint8[]"}]
    }, {
        "name": "constructor",
        "inputs": [
				    {"name": "amount","type": "uint64"},
            {"name": "goal","type": "uint8[]"}
        ],
        "outputs": []
    }]
}"#;

const SUBSCRIBE_CODE_BASE64: &str = r#"te6ccgECKAEABGkAAgE0AgEAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATL/AIn0BSHBAZN49KCbePQN8rSAIPSh8jPiAwEBwAQCASAGBQAp/+ABw8AEhcfABAdMHAfJr0x8B8AKAgHWGAcBAawIAgEgEgkCASALCgBdvBPJjSGMCAgEwAxYVrkwDrjGxo64X/9qJoa6ZAgIB6LflZZGZ2omhqGOeLZPaqQCAnUODAEJtJsMe0ANAfoxgQEAmAGLCtcmAdcY2NHXC//tRNDXTIEBAPQP8rJ1IXj0DvKx0wfRAXMhePQO8rHTH9EBciF49A7ysdN/0QFxIXj0DvKxMcjPlAK02GPaz4aAzgGVgwapDCGXgwagWMsHAegxzwsHAZWDBqkMIZeDBqBYywcB6DHPCwfLBxcBCbUQ68JADwH+MYEBAJgBiwrXJgHXGNgB1wv/AYECAJgBiwrXJgHXGNjRISDtRNDXTIEBAPQP8rJwIXj0DvKx1wv/E/kQ8uBmdCF49A7ysdMf0QFzIXj0DvKx0x/REqD4IyBYvJ91Inj0DvKx0wfRc7ry0GXfyMsfydB0WHj0FosQOHVYePQWcBABzO1E0NdMgQEA9A7ysgFyIXj0DvKxAXEhePQO8rEBJO1E0NdMgQEA9BfIzO1E0NQxzxbJ7VSCECT04VVwyMsHyx/PhoDOAdN/0ZWDBqkMIZeDBqBYywcB6DHPCwfJ0FgwAdcL/3BZcBEAgo4+/vkAU25kQmR5SW50Ae1HbxBvGPpCbxLIz4ZAygfL/8nQjhfIz4Ugz4oAQIEBAM9AzgH6AoBrz0DOydhw+wDYAgJwFBMAVbYbRXcMYEBAJgBiwrXJgHXGNjR7UTQINdKcbrcAXBtgQEA9BbIzM7J7VSABCbZr2EGgFQH+MYEBAJgBiwrXJgHXGNiBAQCYAYsK1yYB1xjYgQEAmAGLCtcmAdcY2NMA0wZYjhVxdwOSVSCc0wDTBgMgpgcFWayg6DHe0wDTBliOFXF3A5JVIJzTANMGAyCmBwVZrKDoMd7RXjAg10mBAQC68uBkIddJgQEAuvLgZCLXSYEBABYB5Lry4GQjwQHy0GQkwQHy0GRVMHBtePQWcQF49BYhyMt/ydAycgF49BYhyMsfydAycwF49Bb4I8jLH8nQdFh49BaLEAh1WHj0FiD5AALXC//tRNDXTIEBAPQXyMztRNDUMc8Wye1UyM+UAAa9hBrPhoDL/xcAaI4xjivIcs9Bcs9Acs9AcM8LP3DPCx9xz0BczzUBzzGkvpVxz0DPE5Vxz0HPEeLJ2HD7ANgCASAnGQEBMBoCA8/AHBsAGTQ1ygF+kD6QPoAbwSABbQg0wcB8muJ9AWAIPQK8qnXCwCOGSDHAvJt1SDHAPJtIfkBAe1E0NcL//kQ8qiXIMcCktQx3+KAdAQHAHgIBICQfAgEgISAACbwTyY0mAgJ1IyIACbSbDHsgAAm1EOvCIAICcCYlAAm2G0V3EAAJtmvYQbAABTbMIA=="#;
const SUBSCRIBE_ABI: &str = r#"
{
    "ABI version": 0,
    "functions": [{
        "name": "constructor",
        "inputs": [{"name": "wallet", "type": "bits256"}],
        "outputs": []
    }, {
        "name": "subscribe",
        "signed": true,
        "inputs": [
            {"name": "subscriptionId", "type": "bits256"},
            {"name": "pubkey", "type": "bits256"},
            {"name": "to",     "type": "bits256"},
            {"name": "value",  "type": "duint"},
            {"name": "period", "type": "duint"}
        ],
        "outputs": [{"name": "subscriptionHash", "type": "bits256"}]
    }, {
        "name": "cancel",
        "signed": true,
        "inputs": [{"name": "subscriptionId", "type": "bits256"}],
        "outputs": []
    }, {
        "name": "executeSubscription",
        "inputs": [
            {"name": "subscriptionId",  "type": "bits256"},
            {"name": "signature",       "type": "bits256"}
        ],
        "outputs": []
    }, {
        "name": "getSubscription",
        "inputs": [{"name": "subscriptionId","type": "bits256"}],
        "outputs": [
            {"name": "to", "type": "bits256"},
            {"name": "amount", "type": "duint"},
            {"name": "period", "type": "duint"},
            {"name": "status", "type": "uint8"}
        ]
    }]
}"#;
