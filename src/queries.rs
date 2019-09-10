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
#![allow(dead_code)]

use crate::interop::InteropContext;
use serde_json::Value;
use crate::TonResult;

pub struct TonQueries {
    context: InteropContext,
    pub blocks: TonQueriesCollection,
    pub accounts: TonQueriesCollection,
    pub transactions: TonQueriesCollection,
    pub messages: TonQueriesCollection,
}

impl TonQueries {
    pub(crate) fn new(context: InteropContext) -> TonQueries {
        TonQueries {
            context,
            blocks: TonQueriesCollection::new(context, "blocks"),
            accounts: TonQueriesCollection::new(context, "accounts"),
            transactions: TonQueriesCollection::new(context, "transactions"),
            messages: TonQueriesCollection::new(context, "messages"),
        }
    }
}

pub struct TonQueriesSubscription {
}

impl TonQueriesSubscription {
    pub fn cancel() {
    }
}

pub struct TonQueriesCollection {
    context: InteropContext,
    pub name: String
}

impl TonQueriesCollection {
    pub(crate) fn new(context: InteropContext, name: &str) -> TonQueriesCollection {
        TonQueriesCollection {
            context,
            name: name.to_string()
        }
    }

    pub fn query(filter: Value, order_by: Value, limit: usize) -> TonResult<Vec<Value>> {
        panic!("Not Implemented")
    }

    pub fn wait_for(filter: Value, order_by: Value, limit: usize) -> TonResult<Vec<Value>> {
        panic!("Not Implemented")
    }

    pub fn subscribe(filter: Value) -> TonResult<TonQueriesSubscription> {
        panic!("Not Implemented")
    }
}
