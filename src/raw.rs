use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawRequest {
    /// A string representing a raw query
    pub query: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawResponse {
    /// The rows returned by the raw query.
    pub rows: Vec<IndexMap<String, serde_json::Value>>,
}
