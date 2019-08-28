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

