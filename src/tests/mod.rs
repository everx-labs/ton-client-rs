use crate::TonClient;

#[test]
fn test_contracts() {
    // Deploy Messages

    let keys = r#"{
        "public": "d59bdd49a40013f6335753eb19b34b37f42ca25df8a44bd7388882ab57019dd1",
        "secret": "4f255abd8da7dcf1fbc94ae2e2742d350621a99a4bd53592661f22ec25bf1d23"
    }"#;
}

#[test]
fn test_piggy() {
    let client = TonClient::default().unwrap();
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


pub const WALLET_CODE_BASE64: &str = r#"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"#;
pub const WALLET_ABI: &str = r#"{
	"ABI version": 0,
	"functions": [
		{
			"inputs": [
				{
					"name": "recipient",
					"type": "bits256"
				},
				{
					"name": "value",
					"type": "duint"
				}
			],
			"name": "sendTransaction",
			"outputs": [
				{
					"name": "transaction",
					"type": "uint64"
				},
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [
				{
					"name": "type",
					"type": "uint8"
				},
				{
					"name": "value",
					"type": "duint"
				},
				{
					"name": "meta",
					"type": "bitstring"
				}
			],
			"name": "createLimit",
			"outputs": [
				{
					"name": "limitId",
					"type": "uint8"
				},
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [
				{
					"name": "limitId",
					"type": "uint8"
				},
				{
					"name": "value",
					"type": "duint"
				},
				{
					"name": "meta",
					"type": "bitstring"
				}
			],
			"name": "changeLimitById",
			"outputs": [
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [
				{
					"name": "limitId",
					"type": "uint8"
				}
			],
			"name": "removeLimit",
			"outputs": [
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [
				{
					"name": "limitId",
					"type": "uint8"
				}
			],
			"name": "getLimitById",
			"outputs": [
				{
					"name": "limitInfo",
					"type": "tuple",
					"components": [
						{
							"name": "value",
							"type": "duint"
						},
						{
							"name": "type",
							"type": "uint8"
						},
						{
							"name": "meta",
							"type": "bitstring"
						}
					]
				},
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [],
			"name": "getLimits",
			"outputs": [
				{
					"name": "list",
					"type": "uint8[]"
				},
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [],
			"name": "getLimitsEx",
			"outputs": [
				{
					"name": "list",
					"type": "tuple[]",
					"components": [
						{
							"name": "id",
							"type": "uint8"
						},
						{
							"name": "type",
							"type": "uint8"
						},
						{
							"name": "value",
							"type": "duint"
						},
						{
							"name": "meta",
							"type": "bitstring"
						}
					]
				}
			]
		},
		{
			"inputs": [],
			"name": "getVersion",
			"outputs": [
				{
					"name": "version",
					"type": "tuple",
					"components": [
						{
							"name": "major",
							"type": "uint16"
						},
						{
							"name": "minor",
							"type": "uint16"
						}
					]
				},
				{
					"name": "error",
					"type": "int8"
				}
			]
		},
		{
			"inputs": [],
			"name": "getBalance",
			"outputs": [
				{
					"name": "balance",
					"type": "uint64"
				}
			]
		},
		{
			"inputs": [],
			"name": "constructor",
			"outputs": []
		},
		{
			"inputs": [
				{
					"name": "address",
					"type": "bits256"
				}
			],
			"name": "setSubscriptionAccount",
			"outputs": []
		},
		{
			"inputs": [],
			"name": "getSubscriptionAccount",
			"outputs": [
				{
					"name": "address",
					"type": "bits256"
				}
			]
		}
	]
}
"#;
