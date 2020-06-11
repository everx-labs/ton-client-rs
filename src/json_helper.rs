/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 */

 use crate::contracts::{EncodedMessage, ResultOfGetDeployData, TransactionFees};
 use crate::TonAddress;
 use crate::error::*;
 use std::convert::TryFrom;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransactionFeesCore {
    pub in_msg_fwd_fee: String,
    pub storage_fee: String,
    pub gas_fee: String,
    pub out_msgs_fwd_fee: String,
    pub total_account_fees: String,
    pub total_output: String
}

impl TryFrom<TransactionFeesCore> for TransactionFees {
    type Error = TonError;

    fn try_from(value: TransactionFeesCore) -> Result<Self, Self::Error> {
        Ok(TransactionFees {
            in_msg_fwd_fee: u64_from_str(value.in_msg_fwd_fee)?,
            storage_fee: u64_from_str(value.storage_fee)?,
            gas_fee: u64_from_str(value.gas_fee)?,
            out_msgs_fwd_fee: u64_from_str(value.out_msgs_fwd_fee)?,
            total_account_fees: u64_from_str(value.total_account_fees)?,
            total_output: u64_from_str(value.total_output)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EncodedMessageCore {
    pub message_id: String,
    pub message_body_base64: String,
    pub expire: Option<u32>,
}

impl Into<EncodedMessageCore> for EncodedMessage {
    fn into(self) -> EncodedMessageCore {
        EncodedMessageCore {
            message_id: self.message_id,
            message_body_base64: base64::encode(&self.message_body),
            expire: self.expire
        }
    }
}

impl TryFrom<EncodedMessageCore> for EncodedMessage {
    type Error = TonError;
    
    fn try_from(value: EncodedMessageCore) -> Result<Self, Self::Error> {
        Ok(EncodedMessage {
            message_id: value.message_id,
            message_body: base64::decode(&value.message_body_base64)?,
            expire: value.expire
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResultOfGetDeployDataCore {
    pub image_base64: Option<String>,
    pub address: Option<String>,
    pub data_base64: String,
}

impl TryFrom<ResultOfGetDeployDataCore> for ResultOfGetDeployData {
    type Error = TonError;
    
    fn try_from(value: ResultOfGetDeployDataCore) -> Result<Self, Self::Error> {
        Ok(ResultOfGetDeployData {
            address: value.address.map(|val| TonAddress::from_str(&val)).transpose()?,
            image: value.image_base64.map(|val| base64::decode(&val).into()).transpose()?,
            data: base64::decode(&value.data_base64)?
        })
    }
}

fn u64_from_str(string: String) -> TonResult<u64> {
    if string.starts_with("0x") {
        u64::from_str_radix(&string[2..], 16)
    } else {
        u64::from_str_radix(&string, 10)
    }.map_err(|err| 
        TonErrorKind::InvalidData(format!("Error parsing number: {} ({})", string, err)).into())
}

