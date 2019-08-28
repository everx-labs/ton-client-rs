use crate::interop::{InteropContext, Interop};
use serde::{Serialize, Serializer};
use crate::KeyPair;

#[derive(Clone)]
pub struct HDPublic(pub [u8; 33]);
#[derive(Clone)]
pub struct NaclNonce(pub [u8; 24]);
#[derive(Clone)]
pub struct NaclSignSecret(pub [u8; 64]);
#[derive(Clone)]
pub struct Ed25519Public(pub [u8; 32]);
#[derive(Clone)]
pub struct Ed25519Secret(pub [u8; 32]);

#[derive(Default, Clone, Serialize)]
pub struct Ed25519KeyPair {
    pub public: Ed25519Public,
    pub secret: Ed25519Secret,
}

impl Ed25519KeyPair {
    pub fn zero() -> Ed25519KeyPair {
        Ed25519KeyPair { public: Ed25519Public([0u8; 32]), secret: Ed25519Secret([0u8; 32]) }
    }
}

#[derive(Clone, Serialize)]
pub enum Ed25519KeySource {
    Keys(Ed25519KeyPair),
    KeyStoreHandle(u32),
}

#[derive(Clone)]
pub struct NaclSignKeyPair {
    pub public: Ed25519Public,
    pub secret: NaclSignSecret,
}

impl NaclSignKeyPair {
    pub fn zero() -> NaclSignKeyPair {
        NaclSignKeyPair { public: Ed25519Public([0u8; 32]), secret: NaclSignSecret([0u8; 64]) }
    }
}

pub struct TonCrypto {
    context: u32,
}

impl TonCrypto {
    pub(crate) fn new(context: InteropContext) -> Self {
        Self { context }
    }
    pub fn generate_ed25519_keys(&self) -> KeyPair {
        Interop::json_request_no_args(self.context, "crypto.ed25519.keypair").unwrap()
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
