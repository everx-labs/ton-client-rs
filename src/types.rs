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

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Enum representing possible TON blockchain internal account addresses.
/// For now only `StdShort` address is supported by core library so all variants are
/// come down to `StdShort` variant while calling core library
#[derive(Clone, PartialEq, Debug)]
pub enum TonAddress {
    StdShort([u8; 32]),
    StdFull(i8, [u8; 32]),
    Var(i32, Vec<u8>),
    AnycastStd(u8, u32, i8, [u8; 32]),
    AnycastVar(u8, u32, i32, Vec<u8>),
}

impl Serialize for TonAddress {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        //serializer.serialize_str(&format!("{}", self))
        // for now only StdShort address is supported
        serializer.serialize_str(&self.get_account_hex_string())
    }
}

struct AddressVisitor;

impl<'de> Visitor<'de> for AddressVisitor {
    type Value = TonAddress;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("32 bytes written into string like a hex values without spaces")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // for now only StdShort address is supported
        let mut result = [0u8; 32];
        let vec = hex::decode(v)
            .map_err(|err| serde::de::Error::custom(format!("error decode hex: {}", err)))?;
        if vec.len() != 32 {
            return Err(serde::de::Error::custom(format!("Wrong data length")));
        }

        result.copy_from_slice(&vec);
        Ok(TonAddress::StdShort(result))
    }
}

impl<'de> Deserialize<'de> for TonAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AddressVisitor)
    }
}

impl TonAddress {
    /// Returns hex-string representation of account ID (not fully qualified address)
    pub fn get_account_hex_string(&self) -> String {
        match self {
            TonAddress::StdShort(a) => hex::encode(a),
            TonAddress::StdFull(_, a) => hex::encode(a),
            TonAddress::Var(_, a) => hex::encode(a),
            TonAddress::AnycastStd(_, _, _, a) => hex::encode(a),
            TonAddress::AnycastVar(_, _, _, a) => hex::encode(a),
        }
    }
}

fn fmt_addr(
    f: &mut std::fmt::Formatter,
    anycast: Option<(u8, u32)>,
    workchain: Option<i32>,
    address: Option<&[u8]>,
) -> Result<(), std::fmt::Error> {
    if let Some((d, p)) = anycast {
        write!(f, "{}:{}:", d, p)?
    }
    if let Some(w) = workchain {
        write!(f, "{}:", w)?
    }
    if let Some(a) = address {
        write!(f, "{}", hex::encode(a))?
    }
    Ok(())
}

impl std::fmt::Display for TonAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TonAddress::StdShort(a) => fmt_addr(f, None, None, Some(a)),
            TonAddress::StdFull(w, a) => fmt_addr(f, None, Some(*w as i32), Some(a)),
            TonAddress::Var(w, a) => fmt_addr(f, None, Some(*w), Some(a)),
            TonAddress::AnycastStd(d, p, w, a) => {
                fmt_addr(f, Some((*d, *p)), Some(*w as i32), Some(a))
            }
            TonAddress::AnycastVar(d, p, w, a) => fmt_addr(f, Some((*d, *p)), Some(*w), Some(a)),
        }
    }
}
