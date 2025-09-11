//! Struct types for the Swagger/OpenAPI specification data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Swagger/OpenAPI spec type.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Spec {
    /// Map of URL part to object.
    pub paths: HashMap<String, HashMap<String, SpecPathMethod>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SpecPathMethod {
    /// The operation ID to use this endpoint.
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
