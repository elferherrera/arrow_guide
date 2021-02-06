use std::{collections::HashMap, sync::Arc};

use arrow::{
    array::{Array, ArrayRef, BooleanArray, Float64Array, Int32Array, StringArray, StructArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};

fn main() {
    let schema = Schema::new(vec![
        Field::new("index", DataType::Int32, false),
        Field::new("word", DataType::Utf8, false),
    ]);

    let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let b = StringArray::from(vec!["one", "two", "three", "four", "five"]);

    let record_batch =
        RecordBatch::try_new(Arc::new(schema), vec![Arc::new(a), Arc::new(b)]).unwrap();

    println!("{:#?}", record_batch);

    // Creating a schema with metadata
    let field_a = Field::new("a", DataType::Int64, false);
    let field_b = Field::new("b", DataType::Boolean, false);

    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("row_count".to_string(), "100".to_string());
    metadata.insert("file".to_string(), "example.csv".to_string());

    let schema = Schema::new_with_metadata(vec![field_a, field_b], metadata);

    println!("{:#?}", schema);

    // Creating from struct
    let index = Arc::new(Int32Array::from(vec![0, 1, 2, 3]));
    let boolean_array = Arc::new(BooleanArray::from(vec![false, false, true, true]));
    let int_array = Arc::new(Int32Array::from(vec![42, 28, 19, 31]));
    let struct_array = StructArray::from(vec![
        (
            Field::new("index", DataType::Int32, false),
            index as ArrayRef,
        ),
        (
            Field::new("col_1", DataType::Int32, false),
            int_array as ArrayRef,
        ),
        (
            Field::new("col_2", DataType::Boolean, false),
            boolean_array as ArrayRef,
        ),
    ]);

    let record_batch = RecordBatch::from(&struct_array);
    println!("{:#?}", record_batch);

    // Creating nested struct
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new(
            "nested",
            DataType::Struct(vec![
                Field::new("a", DataType::Utf8, false),
                Field::new("b", DataType::Float64, false),
                Field::new("c", DataType::Float64, false),
            ]),
            false,
        ),
    ]);

    let id = Int32Array::from(vec![1, 2, 3, 4, 5]);

    let nested = StructArray::from(vec![
        (
            Field::new("a", DataType::Utf8, false),
            Arc::new(StringArray::from(vec!["a", "b", "c", "d", "e"])) as Arc<dyn Array>,
        ),
        (
            Field::new("b", DataType::Float64, false),
            Arc::new(Float64Array::from(vec![1.1, 2.2, 3.3, 4.4, 5.5])),
        ),
        (
            Field::new("c", DataType::Float64, false),
            Arc::new(Float64Array::from(vec![2.2, 3.3, 4.4, 5.5, 6.6])),
        ),
    ]);

    let record_batch =
        RecordBatch::try_new(Arc::new(schema), vec![Arc::new(id), Arc::new(nested)]).unwrap();
    println!("{:#?}", record_batch);
}
