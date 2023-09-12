use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::capabilities::{ColumnName, FunctionName, ScalarType, TableName};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaRequest {
    /// How much information to return about the schema. Values:\n- 'everything': All information about the schema.\n- 'basic_info': For tables, only the table name and table type, for functions, only the function name and function type.
    pub detail_level: Option<DetailLevel>,
    pub filters: Option<SchemaFilters>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailLevel {
    Everything,
    BasicInfo,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaFilters {
    /// Only get the schemas for these functions
    pub only_functions: Option<Vec<FunctionName>>,
    /// Only get the schemas for these tables
    pub only_tables: Option<Vec<TableName>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaResponse {
    /// Object type definitions referenced in this schema
    pub object_types: Option<Vec<ObjectTypeDefinition>>,
    /// Available tables
    pub tables: Vec<TableInfo>,
    /// Available functions
    pub functions: Option<Vec<FunctionInfo>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// argument info - name/types
    pub args: Option<Vec<FunctionInformationArgument>>,
    /// Description of the table
    pub description: Option<String>,
    pub name: FunctionName,
    pub response_cardinality: Option<FunctionResponseCardinality>,
    pub returns: Option<FunctionReturnType>,
    #[serde(rename = "type")]
    pub r#type: FunctionType,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionInformationArgument {
    /// The name of the argument
    pub name: String,
    /// If the argument can be omitted
    pub optional: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: ScalarType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionResponseCardinality {
    One,
    Many,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionType {
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FunctionReturnType {
    Table { table: TableName },
    Unknown {},
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectTypeDefinition {
    /// The columns of the type
    pub columns: Vec<ColumnInfo>,
    /// The description of the type
    pub description: Option<String>,
    /// The name of the type
    pub name: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnInfo {
    /// Column description
    pub description: Option<String>,
    /// Whether or not the column can be inserted into
    pub insertable: Option<bool>,
    /// Column name
    pub name: ColumnName,
    /// Is column nullable
    pub nullable: bool,
    #[serde(rename = "type")]
    pub r#type: ColumnType,
    /// Whether or not the column can be updated
    pub updatable: Option<bool>,
    pub value_generated: Option<ColumnValueGenerationStrategy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColumnType {
    ColumnTypeNonScalar(ColumnTypeNonScalar),
    Scalar(ScalarType),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ColumnTypeNonScalar {
    Object {
        name: String,
    },
    Array {
        element_type: Box<ColumnType>,
        nullable: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ColumnValueGenerationStrategy {
    AutoIncrement {},
    DefaultValue {},
    UniqueIdentifier {},
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableInfo {
    /// The columns of the table
    pub columns: Option<Vec<ColumnInfo>>,
    /// Whether or not existing rows can be deleted in the table
    pub deletable: Option<bool>,
    /// Description of the table
    pub description: Option<String>,
    /// Foreign key constraints
    pub foreign_keys: Option<IndexMap<String, Constraint>>,
    /// Whether or not new rows can be inserted into the table
    pub insertable: Option<bool>,
    /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
    pub name: TableName,
    /// The primary key of the table
    pub primary_key: Option<Vec<ColumnName>>,
    #[serde(rename = "type")]
    pub r#type: Option<TableType>,
    /// Whether or not existing rows can be updated in the table
    pub updatable: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    /// The columns on which you want want to define the foreign key.
    pub column_mapping: IndexMap<ColumnName, ColumnName>,
    /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
    pub foreign_table: TableName,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableType {
    Table,
    View,
}
