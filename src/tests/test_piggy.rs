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

use crate::{OrderBy, SortDirection};
use crate::tests::{WALLET_ABI, WALLET_CODE_BASE64};
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
	        "goal": "536f6d6520676f616c"
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

    let subscription_constructor_params = json!({
        "wallet" : wallet_address.to_string()
    }).to_string().into();

    let subscripition_address = ton.contracts.deploy(
        SUBSCRIBE_ABI,
        &base64::decode(SUBSCRIBE_CODE_BASE64).unwrap(),
        subscription_constructor_params,
        &keypair,
    ).unwrap();
    let set_subscription_params = json!({
            "addr": subscripition_address.to_string()
        }).to_string().into();

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
            "to": piggy_bank_address.to_string(),
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
        }));

    let subscr_id_str = hex::encode(&[0x22; 32]);
    let _subscribe_answer = ton.contracts.run(
        &subscripition_address,
        SUBSCRIBE_ABI,
        "subscribe",
        json!({
            "subscriptionId" : format!("0x{}", subscr_id_str),
            "pubkey" : format!("0x{}", pubkey_str),
            "to": piggy_bank_address.to_string(),
            "value" : 5000000000 as i64,
            "period" : 86400
        }).to_string().into(),
        Some(&keypair)
    ).unwrap();

    let subscriptions = ton.contracts.run_local(
        &subscripition_address,
        None,
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
                {"name":"goal","type":"bytes"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "transfer",
            "inputs": [
                {"name":"to","type":"address"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "getGoal",
            "inputs": [
            ],
            "outputs": [
                {"name":"value0","type":"bytes"}
            ]
        },
        {
            "name": "getTargetAmount",
            "inputs": [
            ],
            "outputs": [
                {"name":"value0","type":"uint64"}
            ]
        }
    ],
    "events": [
    ],
    "data": [
        {"key":100,"name":"targetGoal","type":"bytes"},
        {"key":101,"name":"targetAmount","type":"uint64"}
    ]
}"#;


const SUBSCRIBE_CODE_BASE64: &str = r#"te6ccgECPwEAClUAAgE0BgEBAcACAgPPIAUDAQHeBAAD0CAAQdgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAIo/wAgwAH0pCBYkvSg4YrtU1gw9KAlBwEK9KQg9KEIAgPNQBsJAgFiFAoCASAQCwIBIA0MAAcMNswgAcsIIBl7UdvEoBA9GuBAQD0a+1HbxFvEHAiePQOk9P/0ZFw4rry4Gb4AHUhePQOk9MH0ZFw4nC98uBl+CN0Inj0DpPTH9GRcOJzI3j0DpPTH9GRcOKgvJ8gdAH4I7UfyMsfWXj0QzGAOAf6OIXUhePQOk9MH0ZFw4oBo7UdvEoBA9A6T0wfRkXDivfLgZ+JyIXj0DpPTP9GRcOJxInj0DpVw8AnJ0N/I8BIgdQGAaO1HbxKAQPQOk9MH0ZFw4sjLB1l49EMxgGXtR28SgED0ayIBIlmBAQD0bzCAZe1HbxKAQPRvMO1HAW9SDwAI7VdfAgIBIBIRAJEdSGAZe1HbxKAQPRrgQEA9Gt49A6T0wfRkXDicL3y4GWAZe1HbxKAQPRrIQEhAYEBAPRbMDGAZe1HbxKAQPRvMO1HAW9S7VcwgAf0JHC9InC8sCFwvLDy4GWAZe1HbxKAQPRrJQElJSUlcIBn7UdvEoBA9A6T0wfRkXDibQHIywcBdQF49EMByMsfAXQBePRDAcjLHwFzAXj0QwHIyz8BcgF49EMByM4BcQF49EMByMv/AXABePRDWYEBAPRvMIBl7UdvEoBA9G8wgEwAS7UcBb1LtV18FAgEgGBUCASAXFgAnCCAZe1HbxKAQPRrgQEA9Gsx2zCAAJyAZO1HbxKAQPQOlXDwCcnQ39swgAgEgGhkAsxxyMsHgGftR28SgED0Q+1HAW9S7VdyyMsHgGjtR28SgED0Q+1HAW9S7VftR28RbxDIy/+AZu1HbxKAQPRD7UcBb1LtVyDIzoBk7UdvEoBA9EPtRwFvUu1XMIADVP77AWRlY29kZV9hZGRyIPpAMvpCIG8QIHK6IXO6sfLgfSFvEW7y4H3IdM8LAiJvEs8KByJvEyJyupYjbxMizjKfIYEBACLXSaHPQDIgIs4y4v78AWRlY29kZV9hZGRyMCHJ0CVVQV8F2zCACASAhHAIBSB4dADXX9+ALmytzIvsrw6L7a5s5B8EvwUeAg4fYAYQCASAgHwApr++gFzZW5kX2dyYW1zICIk8AhfA4AI2v79AWJ1aWxkX2V4dF9tc2fIc88LASHPFnDPCwEizws/cM8LH3DPCwAgzzUkzzFxoLyWcc9AI88XlXHPQSPN4iDJBF8E2zCAICnCMiAGM/v0BbWFrZV9hZGRyX3N0ZMiBBADPCwohzwv//v4BbWFrZV9hZGRyX3N0ZDAgMTHbMIAHLP78AXNlbmRfaW50X21zZ8ghI3+OR/74AWJ1aWxkbXNnyHLPQCHPCgBxz0D4KM8WIs8WI/oCcc9AcPoCcPoCgEDPQPgjzwsf/vwBYnVpbGRtc2dfZW5kIARfBNsw2M8XcM8LACMhgJACOjjz+/AFzdG9yZV9laXRoZXIgzzUizzFxoLyWcc9AIc8XlXHPQSHN4v7+AXN0b3JlX2VpdGhlcl8wIDEx2zDYMSDJcPsAXwQCASAsJgHg//79AW1haW5fZXh0ZXJuYWwhjln+/AFnZXRfc3JjX2FkZHIg0CDTADJwvY4a/v0BZ2V0X3NyY19hZGRyMHDIydBVEV8C2zDgIHLXITEg0wAyIfpAM/79AWdldF9zcmNfYWRkcjEhIVUxXwTbMNgxIScB+I51/v4BZ2V0X21zZ19wdWJrZXkgxwKOFv7/AWdldF9tc2dfcHVia2V5MXAx2zDg1SDHAY4X/v8BZ2V0X21zZ19wdWJrZXkycDEx2zDgIIECANch1wv/IvkBIiL5EPKo/v8BZ2V0X21zZ19wdWJrZXkzIANfA9sw2CLHArMoAcyUItQxM94kIiKOOP75AXN0b3JlX3NpZ28AIW+MIm+MI2+M7Uchb4ztRND0BW+MIO1X/v0Bc3RvcmVfc2lnX2VuZF8F2CLHAY4T/vwBbXNnX2lzX2VtcHR5XwbbMOAi0x80I9M/NSApAXaOgNiOL/7+AW1haW5fZXh0ZXJuYWwyJCJVcV8I8UAB/v4BbWFpbl9leHRlcm5hbDNfCNsw4IB88vBfCCoB/v77AXJlcGxheV9wcm90cHBw7UTQIPQEMjQggQCA10WaINM/MjMg0z8yMpaCCBt3QDLiIiW5JfgjgQPoqCSgubCOKcgkAfQAJc8LPyLPCz8hzxYgye1U/vwBcmVwbGF5X3Byb3QyfwZfBtsw4P78AXJlcGxheV9wcm90M3AFXwUrAATbMAIBIDQtAgFmLy4AD7cfuIHMNswgAgEgMTAAu7SLARZ2o7eIt4hAM3ajt4lAIHoHSen/6Mi4cV15cDJ8AGn/mHgS/34AuDq5tDgyMZu6N7GadqJoegDkdqO3iQD6ABDnixBk9qp/foC4Orm0ODIxm7o3sZoYL4FtmEABCbQ5VD3AMgH8/v0BY29uc3RyX3Byb3RfMHBwgggbd0DtRNAg9AQyNCCBAIDXRY4UINI/MjMg0j8yMiBx10WUgHvy8N7eyCQB9AAjzws/Is8LP3HPQSHPFiDJ7VT+/QFjb25zdHJfcHJvdF8xXwX4APAgMPAh/vwBcHVzaHBkYzd0b2M07UTQMwBK9AHI7UdvEgH0ACHPFiDJ7VT+/QFwdXNocGRjN3RvYzQwXwLbMAIBIDo1AgEgNzYAP7l1JtomHgRZEEIHdSbaMEIQAAAAFjnhY+Q54t4Cm2YQAgFYOTgAy7WeQ672o7eIt4hAM3ajt4lAIHoHSen/6Mi4cV15cDJ8AGn/6f/4EGmf6Y+YeBJ/fgC4Orm0ODIxm7o3sZp2omh6AOR2o7eJAPoAEOeLEGT2qn9+gLg6ubQ4MjGbujexmhgvgW2YQAC7tDG4VOn/mHgR5EEIFDG4VMEIQAAAAFjnhY+QgLgRPHoHeXAxZ4s4kTx6B3lwMWeLORE8egd5cDFnizmRPHoHeXAxZ4s6ETx6B3lwMWeLOpE8egd5cDFnixj4Cm2YQAIBSDw7AIG2QnD99P/MPAm/vwBcHVzaHBkYzd0b2M07UTQ9AHI7UdvEgH0ACHPFiDJ7VT+/QFwdXNocGRjN3RvYzQwXwLbMIAHi2/79AW1haW5faW50ZXJuYWwhjln+/AFnZXRfc3JjX2FkZHIg0CDTADJwvY4a/v0BZ2V0X3NyY19hZGRyMHDIydBVEV8C2zDgIHLXITEg0wAyIfpAM/79AWdldF9zcmNfYWRkcjEhIVUxXwTbMNgkIXA9AeqOOP75AXN0b3JlX3NpZ28AIW+MIm+MI2+M7Uchb4ztRND0BW+MIO1X/v0Bc3RvcmVfc2lnX2VuZF8F2CLHAI4cIXC6jhIighBcfuIHVVFfBvFAAV8G2zDgXwbbMOD+/gFtYWluX2ludGVybmFsMSLTHzQicbo+ADaeIIAnVWFfB/FAAV8H2zDgIyFVYV8H8UABXwc="#;
pub const SUBSCRIBE_ABI: &str = r#"
{
    "ABI version": 1,
    "functions": [
        {
            "name": "constructor",
            "inputs": [
                {"name":"wallet","type":"address"}
            ],
            "outputs": [
            ]
        },
        {
            "name": "getWallet",
            "inputs": [
            ],
            "outputs": [
                {"name":"value0","type":"address"}
            ]
        },
        {
            "name": "getSubscription",
            "inputs": [
                {"name":"subscriptionId","type":"uint256"}
            ],
            "outputs": [
                {"components":[{"name":"pubkey","type":"uint256"},{"name":"to","type":"address"},{"name":"value","type":"uint64"},{"name":"period","type":"uint32"},{"name":"start","type":"uint32"},{"name":"status","type":"uint8"}],"name":"value0","type":"tuple"}
            ]
        },
        {
            "name": "subscribe",
            "inputs": [
                {"name":"subscriptionId","type":"uint256"},
                {"name":"pubkey","type":"uint256"},
                {"name":"to","type":"address"},
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
        {"key":100,"name":"mywallet","type":"address"}
    ]
}"#;

