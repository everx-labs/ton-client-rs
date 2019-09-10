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

use crate::interop::{InteropContext, Interop};
use crate::TonResult;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Visitor;

#[derive(Clone)]
struct HDPublic(pub [u8; 33]);
#[derive(Clone)]
struct NaclNonce(pub [u8; 24]);
#[derive(Clone)]
struct NaclSignSecret(pub [u8; 64]);
/// Ed25519 public key
#[derive(Clone)]
pub struct Ed25519Public(pub [u8; 32]);
/// Ed25519 secret key
#[derive(Clone)]
pub struct Ed25519Secret(pub [u8; 32]);

/// Ed25519 key pair
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Ed25519KeyPair {
    pub public: Ed25519Public,
    pub secret: Ed25519Secret,
}

impl Ed25519KeyPair {
    pub fn zero() -> Ed25519KeyPair {
        Ed25519KeyPair { public: Ed25519Public([0u8; 32]), secret: Ed25519Secret([0u8; 32]) }
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        let mut result = [0u8; 64];
        result[..32].copy_from_slice(&self.secret.0);
        result[32..].copy_from_slice(&self.public.0);
        result
    }

    pub fn from_bytes(&self, bytes: [u8; 64]) -> Ed25519KeyPair {
        let mut secret = [0u8; 32];
        let mut public = [0u8; 32];

        secret.copy_from_slice(&bytes[..32]);
        public.copy_from_slice(&bytes[32..]);

        Ed25519KeyPair { public: Ed25519Public(public), secret: Ed25519Secret(secret) }
    }
}

#[allow(dead_code)]
#[derive(Clone, Serialize)]
enum Ed25519KeySource {
    Keys(Ed25519KeyPair),
    KeyStoreHandle(u32),
}

#[derive(Clone)]
struct NaclSignKeyPair {
    pub public: Ed25519Public,
    pub secret: NaclSignSecret,
}

#[allow(dead_code)]
impl NaclSignKeyPair {
    pub fn zero() -> NaclSignKeyPair {
        NaclSignKeyPair { public: Ed25519Public([0u8; 32]), secret: NaclSignSecret([0u8; 64]) }
    }
}

/// Crypto functions struct
pub struct TonCrypto {
    context: u32,
}

impl TonCrypto {
    pub(crate) fn new(context: InteropContext) -> Self {
        Self { context }
    }

    /// Generate Ed25519 key pair for using within TON blockchain
    pub fn generate_ed25519_keys(&self) -> TonResult<Ed25519KeyPair> {
        Interop::json_request_no_args(self.context, "crypto.ed25519.keypair")
    }
}

impl Default for HDPublic {
    fn default() -> Self {
        Self([0u8; 33])
    }
}

impl Default for NaclSignSecret {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

impl Default for Ed25519Public {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl Default for Ed25519Secret {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl Serialize for HDPublic {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&hex::encode(self.0.as_ref()))
    }
}

impl Serialize for NaclSignSecret {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&hex::encode(self.0.as_ref()))
    }
}

impl Serialize for Ed25519Public {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&hex::encode(&self.0.as_ref()))
    }
}

impl Serialize for Ed25519Secret {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&hex::encode(&self.0.as_ref()))
    }
}

struct KeysVisitor;

impl<'de> Visitor<'de> for KeysVisitor {
    type Value = [u8; 32];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("32 bytes written into string like a hex values without spaces")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        let mut result = [0u8; 32];
        let vec = hex::decode(v)
            .map_err(|err| serde::de::Error::custom(format!("error decode hex: {}", err)))?;
        if vec.len() != 32 {
            return Err(serde::de::Error::custom(format!("Wrong data length")));
        }

        result.copy_from_slice(&vec);
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for Ed25519Public {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, 
    {
        Ok(Ed25519Public(deserializer.deserialize_str(KeysVisitor)?))
    }
}

impl<'de> Deserialize<'de> for Ed25519Secret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, 
    {
        Ok(Ed25519Secret(deserializer.deserialize_str(KeysVisitor)?))
    }
}

impl std::fmt::Display for Ed25519Public {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl std::fmt::Display for Ed25519Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", hex::encode(self.0))
    }
}
