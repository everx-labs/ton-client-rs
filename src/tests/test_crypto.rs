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

use crate::tests::create_client;
use crate::{ScryptParams, NaclSignSecret};
use std::convert::TryInto;


fn bytes(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string).unwrap()
}
fn hx(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[test]
fn test_crypto() {
    let client = create_client();
    let crypto = &client.crypto;

    let result = crypto.factorize(&bytes("17ed48941a08f981")).unwrap();
    assert_eq!(hx(&result.0), "494c553b");
    assert_eq!(hx(&result.1), "53911073");

    let result = crypto.modular_power(
        &bytes("0123456789abcdef"),
        &bytes("0123"),
        &bytes("01234567"),
    ).unwrap();
    assert_eq!(hx(&result), "63bfdf");

    // random

    let result = crypto.random_generate_bytes(32).unwrap();
    assert_eq!(result.len(), 32);

    // ed25519

    let result = crypto.generate_ed25519_keys().unwrap();
    assert_eq!(result.public.0.len(), 32);
    assert_eq!(result.secret.0.len(), 32);
    assert_ne!(result.public.0, result.secret.0);

    // sha

    let result = crypto.sha512("Message to hash with sha 512".as_bytes()).unwrap();
    assert_eq!(hx(&result), "2616a44e0da827f0244e93c2b0b914223737a6129bc938b8edf2780ac9482960baa9b7c7cdb11457c1cebd5ae77e295ed94577f32d4c963dc35482991442daa5");
    let result = crypto.sha256("Message to hash with sha 256".as_bytes()).unwrap();
    assert_eq!(hx(&result), "16fd057308dd358d5a9b3ba2de766b2dfd5e308478fc1f7ba5988db2493852f5");

    // scrypt

    let result = crypto.scrypt(ScryptParams {
        password: "Test Password".as_bytes(),
        salt: "Test Salt".as_bytes(),
        log_n: 10,
        r: 8,
        p: 16,
        dk_len: 64,
    }).unwrap();
    assert_eq!(hx(&result), "52e7fcf91356eca55fc5d52f16f5d777e3521f54e3c570c9bbb7df58fc15add73994e5db42be368de7ebed93c9d4f21f9be7cc453358d734b04a057d0ed3626d");

    // nacl keys

    let result = crypto.nacl_sign_keypair().unwrap();
    assert_eq!(result.public.0.len(), 32);
    assert_eq!(result.secret.0.len(), 64);

    let result = crypto.nacl_box_keypair_from_secret_key(&NaclSignSecret(
        bytes("e207b5966fb2c5be1b71ed94ea813202706ab84253bdf4dc55232f82a1caf0d4").as_slice().try_into().unwrap()
    )).unwrap();
    assert_eq!(hx(&result.public.0), "a53b003d3ffc1e159355cb37332d67fc235a7feb6381e36c803274074dc3933a");

    let result = crypto.nacl_sign_keypair().unwrap();
    assert_eq!(result.public.0.len(), 32);
    assert_eq!(result.secret.0.len(), 64);

    let result = crypto.nacl_sign_keypair_from_secret_key(&NaclSignSecret(
        bytes("8fb4f2d256e57138fb310b0a6dac5bbc4bee09eb4821223a720e5b8e1f3dd674").as_slice().try_into().unwrap()
    )).unwrap();
    assert_eq!(hx(&result.public.0), "aa5533618573860a7e1bf19f34bd292871710ed5b2eafa0dcdbb33405f2231c6");

}
