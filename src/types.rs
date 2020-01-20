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

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Visitor;
use std::convert::TryFrom;
use crate::error::*;

/// Enum representing possible TON blockchain internal account addresses.
#[derive(Clone, PartialEq, Debug)]
pub enum TonAddress {
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

struct AddressVisitor;

impl<'de> Visitor<'de> for AddressVisitor {
    type Value = TonAddress;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("32 bytes written into string like a hex values without spaces")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        TonAddress::from_str(v)
            .map_err(|err| serde::de::Error::custom(format!("error decode address: {}", err)))
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

    fn decode_std_short(data: &str) -> TonResult<Self> {
        let vec = hex::decode(data)?;

        Ok(TonAddress::Std(0, <[u8; 32]>::try_from(&vec[..])?))
    }
    
    fn decode_std_base64(data: &str) -> TonResult<Self> {
        // conversion from base64url
        let data = data.replace('_', "/").replace('-', "+");

        let vec = base64::decode(&data)?;

        // check CRC and address tag

        let mut orig_crc = [0u8; 2];
        orig_crc.copy_from_slice(&vec[34..36]);

        if crc16::State::<crc16::XMODEM>::calculate(&vec[..34]) != u16::from_be_bytes(orig_crc) {
            return Err(TonError::from(TonErrorKind::InvalidData(
                format!("base64 address invalid CRC \"{}\"", data))));
        };

        if vec[0] & 0x3f != 0x11 {
            return Err(TonError::from(TonErrorKind::InvalidData(
                format!("base64 address invalid tag \"{}\"", data))));
        }

        Ok(TonAddress::Std(
            i8::from_be_bytes(<[u8; 1]>::try_from(&vec[1..2])?),
            <[u8; 32]>::try_from(&vec[2..34])?))
    }

    fn decode_std_hex(data: &str) -> TonResult<Self> {
        let vec: Vec<&str> = data.split(':').collect();

        if vec.len() != 2 {
            return Err(TonError::from(TonErrorKind::InvalidData(
                format!("Malformed std hex address. No \":\" delimiter. \"{}\"", data))));
        }

        Ok(TonAddress::Std(
            i8::from_str_radix(vec[0], 10)?,
            <[u8; 32]>::try_from(&hex::decode(vec[1])?[..])?))
    }
    
    /// Retrieves account address from `str` in Telegram lite-client format
    pub fn from_str(data: &str) -> TonResult<Self> {
        if data.len() == 64 {
            Self::decode_std_short(data)
        } else if data.len() == 48 {
            Self::decode_std_base64(data)
        } else {
            Self::decode_std_hex(data)
        }
    }
}

fn fmt_addr(
    f: &mut std::fmt::Formatter,
    anycast: Option<(u8, u32)>,
    workchain: Option<i32>,
    address: Option<&[u8]>) -> Result<(), std::fmt::Error> {
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

#[test]
fn test_address_parsing() {
    let short = "fcb91a3a3816d0f7b8c2c76108b8a9bc5a6b7a55bd79f8ab101c52db29232260";
    let full_std = "-1:fcb91a3a3816d0f7b8c2c76108b8a9bc5a6b7a55bd79f8ab101c52db29232260";
    let base64 = "kf/8uRo6OBbQ97jCx2EIuKm8Wmt6Vb15+KsQHFLbKSMiYIny";
    let base64_url = "kf_8uRo6OBbQ97jCx2EIuKm8Wmt6Vb15-KsQHFLbKSMiYIny";

    let full_address = TonAddress::Std(-1, <[u8;32]>::try_from(&hex::decode(short).unwrap()[..]).unwrap());
    let short_address = TonAddress::Std(0, <[u8;32]>::try_from(&hex::decode(short).unwrap()[..]).unwrap());

    assert_eq!(short_address, TonAddress::from_str(short).expect("Couldn't parse short address"));
    assert_eq!(full_address, TonAddress::from_str(full_std).expect("Couldn't parse full_std address"));
    assert_eq!(full_address, TonAddress::from_str(base64).expect("Couldn't parse base64 address"));
    assert_eq!(full_address, TonAddress::from_str(base64_url).expect("Couldn't parse base64_url address"));
}
