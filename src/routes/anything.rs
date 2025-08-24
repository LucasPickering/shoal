use axum::{
    Json,
    extract::{OriginalUri, Query},
    http::{HeaderMap, Method},
};
use bytes::Bytes;
use indexmap::{IndexMap, map::Entry};
use serde::Serialize;
use std::mem;

/// A route to capture anything at /anything/*. Accepts any request and returns
/// a JSON body detailing the request, similar to httpbin.
pub async fn anything(
    uri: OriginalUri,
    method: Method,
    Query(params): Query<Vec<(String, String)>>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<AnythingResponse> {
    let args = group_query_parameters(params);

    let headers = headers
        .into_iter()
        .map(|(name, value)| {
            // Stringify headers
            (
                name.map(|name| name.to_string()),
                String::from_utf8_lossy(value.as_bytes()).into_owned(),
            )
        })
        .collect();

    // Decode the body as UTF-8. Since our response is JSON, we can't represent
    // non-UTF-8 data exactly
    // TODO possible to remove the clone?
    let data = String::from_utf8_lossy(&body).into_owned();

    let json: serde_json::Value =
        serde_json::from_slice(&body).unwrap_or_default();

    Json(AnythingResponse {
        method: method.to_string(),
        url: uri.to_string(),
        args,
        headers,
        data,
        json,
    })
}

/// Details about the user's request
#[derive(Debug, Serialize)]
pub struct AnythingResponse {
    /// HTTP request method
    method: String,
    /// HTTP request URL
    url: String,
    /// Query parameters
    args: IndexMap<String, QueryParameterValue>,
    /// HTTP headers
    headers: IndexMap<Option<String>, String>,
    /// Full body. Non-UTF-8 data will be replaced with placeholders
    data: String,
    /// JSON request body
    json: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum QueryParameterValue {
    One(String),
    Many(Vec<String>),
}

/// Group query parameters by key
fn group_query_parameters(
    params: Vec<(String, String)>,
) -> IndexMap<String, QueryParameterValue> {
    params
        .into_iter()
        .fold(IndexMap::default(), |mut acc, (name, value)| {
            match acc.entry(name) {
                // We have 0 values - add an entry
                Entry::Vacant(entry) => {
                    entry.insert(QueryParameterValue::One(value));
                }
                Entry::Occupied(mut entry) => {
                    match entry.get_mut() {
                        // We have 1 value - convert it to a list and make it 2
                        QueryParameterValue::One(old_value) => {
                            let old_value = mem::take(old_value);
                            entry.insert(QueryParameterValue::Many(vec![
                                old_value, value,
                            ]));
                        }
                        // We have 2+ - just add to the list
                        QueryParameterValue::Many(items) => items.push(value),
                    }
                }
            }
            acc
        })
}
