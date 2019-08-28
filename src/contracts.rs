use crate::{Ed25519KeySource, TonResult, Ed25519Public};
use serde_json::Value;
use crate::interop::{InteropContext, Interop};
use serde::{Serialize, Serializer};

#[derive(Clone)]
pub enum TonAddress {
    ExternalNone,
    External(Vec<u8>),
    Std(i8, [u8; 32]),
    Var(i32, Vec<u8>),
    AnycastStd(u8, u32, i8, [u8; 32]),
    AnycastVar(u8, u32, i32, Vec<u8>),
}

impl Serialize for TonAddress {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl TonAddress {
    pub fn get_account_hex_string(&self) -> String {
        match self {
            TonAddress::ExternalNone =>
                String::new(),
            TonAddress::External(a) =>
                hex::encode(a),
            TonAddress::Std(_, a) =>
                hex::encode(a),
            TonAddress::Var(_, a) =>
                hex::encode(a),
            TonAddress::AnycastStd(_, _, _, a) =>
                hex::encode(a),
            TonAddress::AnycastVar(_, _, _, a) =>
                hex::encode(a),
        }
    }
}

fn fmt_addr(
    f: &mut std::fmt::Formatter,
    anycast: Option<(u8, u32)>,
    workchain: Option<i32>,
    address: Option<&[u8]>) -> Result<(), std::fmt::Error> {
    if let Some((d, p)) = anycast {
        write!(f, "{}:{}:", d, p);
    }
    if let Some(w) = workchain {
        write!(f, "{}:", w);
    }
    if let Some(a) = address {
        write!(f, "{}", hex::encode(a));
    }
    Ok(())
}

impl std::fmt::Display for TonAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TonAddress::ExternalNone =>
                fmt_addr(f, None, None, None),
            TonAddress::External(a) =>
                fmt_addr(f, None, None, Some(a)),
            TonAddress::Std(w, a) =>
                fmt_addr(f, None, Some(*w as i32), Some(a)),
            TonAddress::Var(w, a) =>
                fmt_addr(f, None, Some(*w), Some(a)),
            TonAddress::AnycastStd(d, p, w, a) =>
                fmt_addr(f, Some((*d, *p)), Some(*w as i32), Some(a)),
            TonAddress::AnycastVar(d, p, w, a) =>
                fmt_addr(f, Some((*d, *p)), Some(*w), Some(a))
        }
    }
}

#[derive(Clone, Default)]
pub struct NodeRequestId(pub [u8; 32]);

impl Serialize for NodeRequestId {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&hex::encode(self.0.as_ref()))
    }
}

#[derive(Clone, Default)]
pub struct TransactionId(pub [u8; 32]);

#[derive(Clone, Default)]
pub struct MessageId(pub [u8; 32]);

#[derive(Clone, Serialize)]
pub struct NodeRequest {
    pub body: Vec<u8>,
    pub id: NodeRequestId,
}

#[derive(Clone)]
pub struct UnsignedMessage {
    pub message: Vec<u8>,
    pub bytes_to_sign: Vec<u8>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct AbiRun {
    pub abi: String,
    pub name: String,
    pub input: Value,
    pub keys: Option<Ed25519KeySource>
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Anycast {
    depth: u8,
    rewrite_pfx: u32,
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct AbiDeployParams {
    anycast: Option<Anycast>,
    workchain: i32,
    code: String,
    public: Ed25519Public,
    constructor: Option<AbiRun>,
}

pub struct TonContracts {
    context: InteropContext,
}

impl TonContracts {
    pub(crate) fn new(context: InteropContext) -> Self {
        Self { context }
    }

    pub fn abi_deploy(
        &self,
        anycast: Option<Anycast>,
        workchain: i32,
        code: &[u8],
        public: String,
        constructor: Option<AbiRun>,
    ) -> TonResult<TonAddress> {
        Interop::json_request(self.context, "contracts.abi.deploy", AbiDeployParams {
            anycast,
            workchain,
            code: base64::encode(code),
            public,
            constructor,
        })
    }
}
