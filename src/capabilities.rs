use indexmap::IndexMap;
use openapiv3::Schema as OpenApiSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Possibly qualified table name. Must be non-empty
pub type TableName = Vec<String>;
/// Possibly qualified function name. Must be non-empty
pub type FunctionName = Vec<String>;
/// The name of a column
pub type ColumnName = String;
pub type ScalarType = String;
pub type AggregateFunction = String;
pub type ComparisonOperator = String;
pub type UpdateOperator = String;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilitiesResponse {
    pub capabilities: Capabilities,
    pub config_schemas: ConfigSchemaResponse,
    pub display_name: Option<String>,
    pub release_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigSchemaResponse {
    pub config_schema: OpenApiSchema,
    pub other_schemas: IndexMap<String, OpenApiSchema>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Capabilities {
    pub comparisons: Option<ComparisonCapabilities>,
    pub data_schema: Option<DataSchemaCapabilities>,
    pub datasets: Option<serde_json::Value>,
    pub explain: Option<serde_json::Value>,
    pub interpolated_queries: Option<serde_json::Value>,
    pub licensing: Option<serde_json::Value>,
    pub metrics: Option<serde_json::Value>,
    pub mutations: Option<MutationCapabilities>,
    pub queries: Option<QueryCapabilities>,
    pub raw: Option<serde_json::Value>,
    pub relationships: Option<serde_json::Value>,
    /// A map from scalar type names to their capabilities. Keys must be valid GraphQL names and must be defined as scalar types in the `graphql_schema`
    pub scalar_types: Option<IndexMap<ScalarType, ScalarTypeCapabilities>>,
    pub subscriptions: Option<serde_json::Value>,
    pub user_defined_functions: Option<serde_json::Value>,
    pub post_schema_capabilities: Option<serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComparisonCapabilities {
    pub subquery: Option<SubqueryComparisonCapabilities>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubqueryComparisonCapabilities {
    /// Does the agent support comparisons that involve related tables (ie. joins)?
    pub supports_relations: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataSchemaCapabilities {
    pub column_nullability: Option<ColumnNullability>,
    /// Whether tables can have foreign keys
    pub supports_foreign_keys: Option<bool>,
    /// Whether tables can have primary keys
    pub supports_primary_keys: Option<bool>,
    pub supports_schemaless_tables: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnNullability {
    OnlyNullable,
    NullableAndNonNullable,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutationCapabilities {
    pub atomicity_support_level: Option<AtomicitySupportLevel>,
    pub delete: Option<serde_json::Value>,
    pub insert: Option<InsertCapabilities>,
    pub returning: Option<serde_json::Value>,
    pub update: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AtomicitySupportLevel {
    Row,
    SingleOperation,
    HomogeneousOperations,
    HeterogeneousOperations,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertCapabilities {
    /// Whether or not nested inserts to related tables are supported
    pub supports_nested_inserts: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryCapabilities {
    pub foreach: Option<serde_json::Value>,
}

/// ScalarTypeCapabilities : Capabilities of a scalar type. comparison_operators: The comparison operators supported by the scalar type. aggregate_functions: The aggregate functions supported by the scalar type. update_column_operators: The update column operators supported by the scalar type. graphql_type: Associates the custom scalar type with one of the built-in GraphQL scalar types.  If a `graphql_type` is specified then HGE will use the parser for that built-in type when parsing values of the custom type. If not given then any JSON value will be accepted.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarTypeCapabilities {
    /// A map from aggregate function names to their result types. Function and result type names must be valid GraphQL names. Result type names must be defined scalar types declared in ScalarTypesCapabilities.
    pub aggregate_functions: Option<IndexMap<AggregateFunction, ScalarType>>,
    /// A map from comparison operator names to their argument types. Operator and argument type names must be valid GraphQL names. Argument type names must be defined scalar types declared in ScalarTypesCapabilities.
    pub comparison_operators: Option<IndexMap<ComparisonOperator, ScalarType>>,
    pub graphql_type: Option<GraphQlType>,
    /// A map from update column operator names to their definitions. Operator names must be valid GraphQL names.
    pub update_column_operators: Option<IndexMap<UpdateOperator, UpdateColumnOperatorDefinition>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GraphQlType {
    Int,
    Float,
    String,
    Boolean,
    #[serde(rename = "ID")]
    Id,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColumnOperatorDefinition {
    pub argument_type: String,
}
