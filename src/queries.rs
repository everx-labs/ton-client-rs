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

use crate::interop::{Interop, InteropContext};
use crate::{TonError, TonResult};
use futures::stream::Stream;
use futures::{Async, Poll};
use serde_json::Value;

#[derive(Serialize)]
pub(crate) struct ParamsOfQuery {
    pub table: String,
    pub filter: String,
    pub result: String,
    pub order: Option<OrderBy>,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub(crate) struct ParamsOfSubscribe {
    pub table: String,
    pub filter: String,
    pub result: String,
}

#[derive(Deserialize)]
pub(crate) struct ResultOfQuery {
    pub result: Vec<Value>,
}

#[derive(Deserialize)]
pub(crate) struct SingleResult {
    pub result: Value,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SubscribeHandle {
    pub handle: StreamHandle,
}

type StreamHandle = u32;

/// GraphQL answers sorting direction
#[derive(Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "ASC")]
    Ascending,
    #[serde(rename = "DESC")]
    Descending,
}

/// Struct for specifying GraphQL answers sorting
#[derive(Serialize, Deserialize)]
pub struct OrderBy {
    pub path: String,
    pub direction: SortDirection,
}

struct SubscribeStream<'a> {
    collection: &'a TonQueriesCollection,
    handle: StreamHandle,
}

impl<'a> Stream for SubscribeStream<'a> {
    type Item = Value;
    type Error = TonError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(Some(self.collection.get_next(self.handle)?)))
    }
}

/// Struct for obtatining blockchain data through GraphQL queries
pub struct TonQueries {
    pub blocks: TonQueriesCollection,
    pub accounts: TonQueriesCollection,
    pub transactions: TonQueriesCollection,
    pub messages: TonQueriesCollection,
}

impl TonQueries {
    pub(crate) fn new(context: InteropContext) -> TonQueries {
        TonQueries {
            blocks: TonQueriesCollection::new(context, "blocks"),
            accounts: TonQueriesCollection::new(context, "accounts"),
            transactions: TonQueriesCollection::new(context, "transactions"),
            messages: TonQueriesCollection::new(context, "messages"),
        }
    }
}

/// Struct for quering particular GraphQL collection
pub struct TonQueriesCollection {
    context: InteropContext,
    pub name: String,
}

impl TonQueriesCollection {
    pub(crate) fn new(context: InteropContext, name: &str) -> TonQueriesCollection {
        TonQueriesCollection {
            context,
            name: name.to_string(),
        }
    }

    /// Query request. Returns set of GraphQL objects satisfying conditions described by `filter`
    pub fn query(
        &self,
        filter: &str,
        result: &str,
        order: Option<OrderBy>,
        limit: Option<usize>,
    ) -> TonResult<Vec<Value>> {
        let result: ResultOfQuery = Interop::json_request(
            self.context,
            "queries.query",
            ParamsOfQuery {
                table: self.name.to_owned(),
                filter: filter.to_owned(),
                result: result.to_owned(),
                order,
                limit,
            },
        )?;
        Ok(result.result)
    }

    /// Wait for appearance of an object satisfying conditions described by `filter`.
    /// If such an object already exists it is returned immediately.
    /// In case of several objects satisfying provided conditions exists first founded object returned.
    pub fn wait_for(&self, filter: &str, result: &str) -> TonResult<Value> {
        let result: SingleResult = Interop::json_request(
            self.context,
            "queries.wait.for",
            ParamsOfSubscribe {
                table: self.name.to_owned(),
                filter: filter.to_owned(),
                result: result.to_owned(),
            },
        )?;
        Ok(result.result)
    }

    /// Subscribe for object updates. Returns `Stream` containing objects states
    pub fn subscribe<'a>(
        &'a self,
        filter: &str,
        result: &str,
    ) -> TonResult<Box<dyn Stream<Item = Value, Error = TonError> + 'a>> {
        let result: SubscribeHandle = Interop::json_request(
            self.context,
            "queries.subscribe",
            ParamsOfSubscribe {
                table: self.name.to_owned(),
                filter: filter.to_owned(),
                result: result.to_owned(),
            },
        )?;
        Ok(Box::new(SubscribeStream {
            collection: self,
            handle: result.handle,
        }))
    }

    fn get_next(&self, handle: StreamHandle) -> TonResult<Value> {
        let result: SingleResult =
            Interop::json_request(self.context, "queries.get.next", SubscribeHandle { handle })?;
        Ok(result.result)
    }
}
