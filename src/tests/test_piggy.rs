use crate::TonClient;
use crate::tests::{WALLET_ABI, WALLET_CODE_BASE64};

#[test]
fn test_piggy() {
    let ton = TonClient::new_with_base_url("http://0.0.0.0");
    let keypair = ton.crypto.generate_ed25519_keys();
    ;
    let wallet_address = ton.contracts.deploy(WALLET_ABI, WALLET_CODE_BASE64,
        json!({}), &keypair).unwrap();

    let piggy_bank_address = ton.contracts.deploy(
        PIGGY_BANK_ABI,
        PIGGY_BANK_CODE_BASE64,
        json!({
	        "amount": 123,
	        "goal": [83, 111, 109, 101, 32, 103, 111, 97, 108]
        }),
        &keypair,
    ).unwrap();

    let get_goal_answer = ton.contracts.run(
        piggy_bank_address.as_str(),
        PIGGY_BANK_ABI,
        "getGoal",
        json!({}), None).unwrap();

    let subscription_constructor_params = json!({ "wallet" : format!("x{}", wallet_address)});
    let subscripition_address = ton.contracts.deploy(
        SUBSCRIBE_ABI,
        SUBSCRIBE_CODE_BASE64,
        subscription_constructor_params,
        &keypair,
    ).unwrap();
    let set_subscription_params = json!({ "address": format!("x{}", subscripition_address) });

    let _set_subscription_answer = ton.contracts.run(
        wallet_address.as_str(),
        WALLET_ABI,
        "setSubscriptionAccount",
        set_subscription_params,
        Some(&keypair));

    let subscr_id_str = hex::encode(&[0x11; 32]);
    let pubkey_str = keypair.public.clone();

    let _subscribe_answer = ton.contracts.run(
        subscripition_address.as_str(),
        SUBSCRIBE_ABI,
        "subscribe",
        json!({
            "subscriptionId" : format!("x{}", subscr_id_str),
            "pubkey" : format!("x{}", pubkey_str),
            "to": format!("x{}", piggy_bank_address),
            "value" : 123,
            "period" : 456
        }),
        Some(&keypair)
    ).unwrap();

    let subscr_id_str = hex::encode(&[0x22; 32]);
    let _subscribe_answer = ton.contracts.run(
        subscripition_address.as_str(),
        SUBSCRIBE_ABI,
        "subscribe",
        json!({
            "subscriptionId" : format!("x{}", subscr_id_str),
            "pubkey" : format!("x{}", pubkey_str),
            "to": format!("x{}", piggy_bank_address),
            "value" : 5000000000 as i64,
            "period" : 86400
        }),
        Some(&keypair)
    ).unwrap();

    let subscriptions = ton.contracts.run(
        subscripition_address.as_str(),
        SUBSCRIBE_ABI,
        "getSubscription",
        json!({
            "subscriptionId" : format!("x{}", subscr_id_str),
        }),
        Some(&keypair)
    ).unwrap();
}

const PIGGY_BANK_CODE_BASE64: &str = r#"te6ccgECHwEAApAAAgE0AgEAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATL/AIn0BSHBAZN49KCbePQN8rSAIPSh8jPiAwEBwAQCASAGBQAp/+ABw8AEhcfABAdMHAfJ00x8B8AKAgHWEQcBAawIAgFIDAkCAWILCgCXt9kOJRb7UTQ1DBxAXj0DjDIzsnQjjSOLshyz0Fyz0Byz0Bwzws/cM8LH3HPQFzPNQHXSaS+lHHPQM6Zcc9BAcjOyc8U4snYcPsA2IAA/t1nVP8x1j8BcG149BZxAXj0FsjM7UTQ1v8wzxbJ7VSACAUgQDQEJuWWEepAOAf4xgQEAmAGLCtcmAdcY2DDT/9HtR28QbxdvEO1E0NQwcAF49A4w0z8wIbuOTyCAD4BkqYVcoTKLCHBYjj7++QBTbmRCZHlJbnQB7UdvEG8Y+kJvEsjPhkDKB8v/ydCOF8jPhSDPigBAgQEAz0DOAfoCgGvPQM7J2HD7ANjfAYsIDwBwcBIDAe1HbxBvGPpCbxLIz4ZAygfL/8nQjhfIz4Ugz4oAQIEBAM9AzgH6AoBrz0DOydiBAID7ADAAn7iKB9ILfaiaGoYOAC8egcYaZ+YZGWf5OhHGkcXZDlnoLlnoDlnoDhnhZ+4Z4WPuOegLmeagOuk0l9KOOegZ0y456CA5Gdk54pxZOw4fYBsQAgEgHhIBATATAgPPwBUUABk0NcoBfpA+kD6AG8EgAXMINMH0x8wAfJ0ifQFgCD0DvKp1wsAjhkgxwLyaNUgxwDyaCH5AQHtRNDXC//5EPKolyDHApLUMd/igFgEBwBcCAUgbGAIBYhoZAAm32Q4lEAAJt1nVP9ACAUgdHAAJuWWEepgACbiKB9IIAAU2zCA="#;
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


const SUBSCRIBE_CODE_BASE64: &str = r#"te6ccgECJwEABFMAAgE0AgEAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATL/AIn0BSHBAZN49KCbePQN8rSAIPSh8jPiAwEBwAQCASAGBQAp/+ABw8AEhcfABAdMHAfJ00x8B8AKAgHWFwcBAawIAgEgEQkCASALCgBdvBPJjSGMCAgEwAxYVrkwDrjGxo6f/o9qJoa6ZAgIB6LflU5GZ2omhqGOeLZPaqQCAnUNDAH5tJsMexjAgIBMAMWFa5MA64xsaOn/6PaiaGumQICAegf5VLqQvHoHeVnpg+iAuZC8egd5WemP6IC5ELx6B3lZ6b/ogLiQvHoHeVmY5GfDQGcAysGDVIYQy8GDUCxlg4D0GOeFg4DKwYNUhhDLwYNQLGWDgPQY54WD5YPk6EAWAQm1EOvCQA4B/jGBAQCYAYsK1yYB1xjYAdP/0QGBAgCYAYsK1yYB1xjY0SEg7UTQ10yBAQD0D/KpcCF49A7ys9P/0RP5EPKodCF49A7ys9Mf0QFzIXj0DvKz0x/REqD4IyBYvJ51Inj0DvKz0wfRc7ryfN/Iyx/J0HRYePQWixA4dVh49BZw7UQPAcjQ10yBAQD0DvKpAXIhePQO8rMBcSF49A7yswEk7UTQ10yBAQD0F8jM7UTQ1DHPFsntVIIQJPThVXDIywfLH8+GgM4B03/RlYMGqQwhl4MGoFjLBwHoMc8LB8nQWDAB0//RcFlwEACCjj7++QBTbmRCZHlJbnQB7UdvEG8Y+kJvEsjPhkDKB8v/ydCOF8jPhSDPigBAgQEAz0DOAfoCgGvPQM7J2HD7ANgCAnATEgBVthtFdwxgQEAmAGLCtcmAdcY2NHtRNAg10pxutwBcG2BAQD0FsjMzsntVIAEJtmvYQaAUAfwxgQEAmAGLCtcmAdcY2IEBAJgBiwrXJgHXGNiBAQCYAYsK1yYB1xjY0wDTBliOFXF3A5JVIJzTANMGAyCmBwVZrKDoMd7TANMGWI4VcXcDklUgnNMA0wYDIKYHBVmsoOgx3tFeMCDXSYEBALrytiHXSYEBALrytiLXSYEBALoVAdLytiPBAfJ2JMEB8nZVMHBtePQWcQF49BYhyMt/ydAycgF49BYhyMsfydAycwF49Bb4I8jLH8nQdFh49BaLEAh1WHj0FiD5AALT/9HtRNDXTIEBAPQXyMztRNDUMc8Wye1UyM+GgMv/ydAWAG6ONI4uyHLPQXLPQHLPQHDPCz9wzwsfcc9AXM81AddJpL6Ucc9Azplxz0EByM7JzxTiydhw+wDYAgEgJhgBATAZAgPPwBsaABk0NcoBfpA+kD6AG8EgAWMINMH0x8wAfJ0ifQFgCD0DvKp1wsAjhkgxwLyaNUgxwDyaCH5AQHtRNDXC//5EPKo3oBwBAcAdAgEgIx4CASAgHwAJvBPJjSYCAnUiIQAJtJsMeyAACbUQ68IgAgJwJSQACbYbRXcQAAm2a9hBsAAFNswg"#;
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

