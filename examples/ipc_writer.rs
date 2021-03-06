use std::collections::HashMap;
use std::{net::TcpStream, sync::Arc};

use arrow::{
    array::{Int32Array, StringArray},
    datatypes::{DataType, Field, Schema},
    ipc::writer::StreamWriter,
    record_batch::RecordBatch,
};

fn main() {
    let mut schema_metadata: HashMap<String, String> = HashMap::new();
    schema_metadata.insert("file_name".to_string(), "my_file.parquet".to_string());

    let schema = Schema::new_with_metadata(
        vec![
            Field::new("index", DataType::Int32, false),
            Field::new("word", DataType::Utf8, false),
        ],
        schema_metadata,
    );

    let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let b = StringArray::from(vec!["one", "two", "three", "four", "five"]);

    let batch =
        RecordBatch::try_new(Arc::new(schema.clone()), vec![Arc::new(a), Arc::new(b)]).unwrap();

    let stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    let mut writer = StreamWriter::try_new(stream, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.write(&batch).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
}
