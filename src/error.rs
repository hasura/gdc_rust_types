use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error details
    pub details: Option<IndexMap<String, serde_json::Value>>,
    /// Error message
    pub message: String,
    #[serde(rename = "type")]
    pub r#type: Option<ErrorResponseType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorResponseType {
    UncaughtError,
    MutationConstraintViolation,
    MutationPermissionCheckFailure,
}
