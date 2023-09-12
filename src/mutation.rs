use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    capabilities::{ColumnName, ScalarType, TableName, UpdateOperator},
    query::{Expression, Field, TableRelationships},
    schema::ColumnType,
    ColumnValueGenerationStrategy, ResponseFieldValue,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutationRequest {
    /// The schema by which to interpret row data specified in any insert operations in this request
    pub insert_schema: Vec<TableInsertSchema>,
    /// The mutation operations to perform
    pub operations: Vec<MutationOperation>,
    /// The relationships between tables involved in the entire mutation request
    pub relationships: Vec<TableRelationships>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableInsertSchema {
    /// The fields that will be found in the insert row data for the table and the schema for each field
    pub fields: IndexMap<String, InsertFieldSchema>,
    /// The names of the columns that make up the table's primary key
    pub primary_key: Option<Vec<String>>,
    /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
    pub table: TableName,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InsertFieldSchema {
    ArrayRelation {
        /// The name of the array relationship over which the related rows must be inserted
        relationship: String,
    },
    Column {
        /// The name of the column that this field should be inserted into
        column: String,
        column_type: ColumnType,
        /// Is the column nullable
        nullable: bool,
        value_generated: Option<ColumnValueGenerationStrategy>,
    },
    ObjectRelation {
        insertion_order: ObjectRelationInsertionOrder,
        /// The name of the object relationship over which the related row must be inserted
        relationship: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectRelationInsertionOrder {
    BeforeParent,
    AfterParent,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MutationOperation {
    Delete {
        /// The fields to return for the rows affected by this delete operation
        returning_fields: Option<IndexMap<String, Field>>,
        /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
        table: TableName,
        #[serde(rename = "where")]
        r#where: Option<Expression>,
    },
    Insert {
        post_insert_check: Option<Expression>,
        /// The fields to return for the rows affected by this insert operation
        returning_fields: Option<IndexMap<String, Field>>,
        /// The rows to insert into the table
        rows: Vec<IndexMap<String, serde_json::Value>>,
        /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
        table: TableName,
    },
    Update {
        post_update_check: Option<Expression>,
        /// The fields to return for the rows affected by this update operation
        returning_fields: Option<IndexMap<String, Field>>,
        /// The fully qualified name of a table, where the last item in the array is the table name and any earlier items represent the namespacing of the table name
        table: TableName,
        /// The updates to make to the matched rows in the table
        updates: Vec<RowUpdate>,
        #[serde(rename = "where")]
        r#where: Option<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RowUpdate {
    CustomOperator {
        /// The name of the column in the row
        column: ColumnName,
        operator_name: UpdateOperator,
        /// The value to use with the column operator
        value: serde_json::Value,
        value_type: ScalarType,
    },
    Set {
        /// The name of the column in the row
        column: String,
        /// The value to use with the column operator
        value: IndexMap<String, serde_json::Value>,
        value_type: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutationResponse {
    /// The results of each mutation operation, in the same order as they were received
    pub operation_results: Vec<MutationOperationResults>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutationOperationResults {
    /// The number of rows affected by the mutation operation
    pub affected_rows: u64,
    /// The rows affected by the mutation operation
    pub returning: Option<Vec<IndexMap<String, ResponseFieldValue>>>,
}
