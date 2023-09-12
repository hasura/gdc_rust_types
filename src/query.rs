use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_enum_str::{
    Deserialize_enum_str as DeserializeEnumStr, Serialize_enum_str as SerializeEnumStr,
};
use serde_with::skip_serializing_none;

use crate::capabilities::{AggregateFunction, ColumnName, FunctionName, ScalarType, TableName};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    /// If present, a list of columns and values for the columns that the query must be repeated for, applying the column values as a filter for each query.
    pub foreach: Option<Vec<IndexMap<ColumnName, ScalarValue>>>,
    pub interpolated_queries: Option<IndexMap<String, InterpolatedQuery>>,
    pub query: Query,
    /// The target of the query.
    pub target: Target,
    /// The relationships between tables involved in the entire query request
    pub relationships: Vec<TableRelationships>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterpolatedQuery {
    /// An id associated with the interpolated query - Should be unique across the request
    pub id: String,
    /// Interpolated items in the query
    pub items: Vec<InterpolatedItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InterpolatedItem {
    Text {
        value: String,
    },
    Scalar {
        value: serde_json::Value,
        value_type: ScalarType,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Target {
    Table {
        /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
        name: TableName,
    },
    Interpolated {
        id: String,
    },
    Function {
        name: FunctionName,
        arguments: Vec<FunctionRequestArgument>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FunctionRequestArgument {
    Named { name: String, value: ArgumentValue },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ArgumentValue {
    Scalar {
        value: serde_json::Value,
        value_type: ScalarType,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarValue {
    pub value: serde_json::Value,
    pub value_type: ScalarType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableRelationships {
    /// A map of relationships from the source table to target tables. The key of the map is the relationship name
    pub relationships: IndexMap<String, Relationship>,
    /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
    pub source_table: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    /// A mapping between columns on the source table to columns on the target table
    pub column_mapping: IndexMap<ColumnName, ColumnName>,
    pub relationship_type: RelationshipType,
    /// The target of the relationship.
    pub target: Target,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    Object,
    Array,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Query {
    /// Aggregate fields of the query
    pub aggregates: Option<IndexMap<String, Aggregate>>,
    /// Optionally limit the maximum number of rows considered while applying aggregations. This limit does not apply to returned rows.
    pub aggregates_limit: Option<u64>,
    /// Fields of the query
    pub fields: Option<IndexMap<String, Field>>,
    /// Optionally limit the maximum number of returned rows. This limit does not apply to records considered while apply aggregations.
    pub limit: Option<u64>,
    /// Optionally offset from the Nth result. This applies to both row and aggregation results.
    pub offset: Option<u64>,
    pub order_by: Option<OrderBy>,
    #[serde(rename = "where")]
    pub r#where: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Aggregate {
    ColumnCount {
        /// The column to apply the count aggregate function to
        column: ColumnName,
        /// Whether or not only distinct items should be counted
        distinct: bool,
    },
    SingleColumn {
        /// The column to apply the aggregation function to
        column: ColumnName,
        /// Single column aggregate function name. A valid GraphQL name
        function: AggregateFunction,
        result_type: ScalarType,
    },
    StarCount {},
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Field {
    Column {
        column: ColumnName,
        column_type: ScalarType,
    },
    Object {
        column: ColumnName,
        query: Query,
    },
    Array {
        field: Box<Field>,
        limit: Option<i64>,
        offset: Option<i64>,
        #[serde(rename = "where")]
        r#where: Option<OrderBy>,
    },
    Relationship {
        query: Query,
        /// The name of the relationship to follow for the subquery
        relationship: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderBy {
    /// The elements to order by, in priority order
    pub elements: Vec<OrderByElement>,
    /// A map of relationships from the current query table to target tables. The key of the map is the relationship name. The relationships are used within the order by elements.
    pub relations: IndexMap<String, OrderByRelation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderByElement {
    pub order_direction: OrderDirection,
    pub target: OrderByTarget,
    /// The relationship path from the current query table to the table that contains the target to order by. This is always non-empty for aggregate order by targets
    pub target_path: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderByRelation {
    /// Further relationships to follow from the relationship's target table. The key of the map is the relationship name.
    #[serde(rename = "subrelations")]
    pub subrelations: IndexMap<String, OrderByRelation>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderByTarget {
    Column {
        column: ColumnSelector,
    },
    SingleColumnAggregate {
        /// The column to apply the aggregation function to
        column: ColumnName,
        /// Single column aggregate function name. A valid GraphQL name
        function: AggregateFunction,
        result_type: ScalarType,
    },
    StarCountAggregate {},
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColumnSelector {
    Compound(Vec<String>),
    Name(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Expression {
    And {
        expressions: Vec<Expression>,
    },
    Or {
        expressions: Vec<Expression>,
    },
    Not {
        expression: Box<Expression>,
    },
    #[serde(rename = "unary_op")]
    ApplyUnaryComparison {
        column: ComparisonColumn,
        operator: UnaryComparisonOperator,
    },
    #[serde(rename = "binary_op")]
    ApplyBinaryComparison {
        column: ComparisonColumn,
        operator: BinaryComparisonOperator,
        value: ComparisonValue,
    },
    #[serde(rename = "binary_arr_op")]
    ApplyBinaryArrayComparison {
        column: ComparisonColumn,
        operator: BinaryArrayComparisonOperator,
        value_type: String,
        values: Vec<serde_json::Value>,
    },
    Exists {
        in_table: ExistsInTable,
        #[serde(rename = "where")]
        r#where: Box<Expression>,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComparisonColumn {
    pub column_type: ScalarType,
    /// The name of the column
    pub name: ColumnSelector,
    /// The path to the table that contains the specified column. Missing or empty array means the current table. [\"$\"] means the query table. No other values are supported at this time.
    pub path: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, SerializeEnumStr, DeserializeEnumStr)]
#[serde(rename_all = "snake_case")]
pub enum UnaryComparisonOperator {
    IsNull,
    #[serde(other)]
    Other(String),
}

#[derive(Clone, Debug, PartialEq, SerializeEnumStr, DeserializeEnumStr)]
#[serde(rename_all = "snake_case")]
pub enum BinaryComparisonOperator {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    #[serde(other)]
    Other(String),
}

#[derive(Clone, Debug, PartialEq, SerializeEnumStr, DeserializeEnumStr)]
#[serde(rename_all = "snake_case")]
pub enum BinaryArrayComparisonOperator {
    In,
    #[serde(other)]
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComparisonValue {
    Column {
        column: ComparisonColumn,
    },
    Scalar {
        value: serde_json::Value,
        value_type: ScalarType,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExistsInTable {
    Related {
        relationship: String,
    },
    Unrelated {
        /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
        // TODO: this probably needs to be a target, eventually?
        table: Vec<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryResponse {
    /// multiple result sets, when foreach is specified
    ForEach { rows: Vec<ForEachRow> },
    /// Single result set, when foreach is not specified
    Single(ResponseRow),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForEachRow {
    pub query: ResponseRow,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseRow {
    /// The results of the aggregates returned by the query
    pub aggregates: Option<IndexMap<String, serde_json::Value>>,
    /// The rows returned by the query, corresponding to the query's fields
    pub rows: Option<Vec<IndexMap<String, ResponseFieldValue>>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseFieldValue {
    Relationship(Box<ResponseRow>),
    Column(serde_json::Value),
}
