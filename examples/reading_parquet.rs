use arrow::{
    array::{
        Array, ArrayRef, BooleanArray, Date32Array, Float32Array, Float64Array, Int16Array,
        Int32Array, Int64Array, Int8Array, LargeStringArray, ListArray, StringArray, UInt16Array,
        UInt32Array, UInt64Array, UInt8Array,
    },
    datatypes::{DataType, DateUnit, Schema},
    record_batch::RecordBatch,
};

use parquet::{
    arrow::{ArrowReader, ArrowWriter, ParquetFileArrowReader},
    file::reader::SerializedFileReader,
};

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

// Taken from DataFusion
// Represents a dynamically typed, nullable single value.
// This is the single-valued counter-part of arrow’s `Array`.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarValue {
    Boolean(Option<bool>),
    Float32(Option<f32>),
    Float64(Option<f64>),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    UInt8(Option<u8>),
    UInt16(Option<u16>),
    UInt32(Option<u32>),
    UInt64(Option<u64>),
    Utf8(Option<String>),
    LargeUtf8(Option<String>),
    List(Option<Vec<ScalarValue>>, DataType),
    Date32(Option<i32>),
    TimeMicrosecond(Option<i64>),
    TimeNanosecond(Option<i64>),
}

// Macro used to extract data from an specific array
macro_rules! typed_cast {
    ($array:expr, $index:expr, $ARRAYTYPE:ident, $SCALAR:ident) => {{
        let array = $array.as_any().downcast_ref::<$ARRAYTYPE>().unwrap();
        ScalarValue::$SCALAR(match array.is_null($index) {
            true => None,
            false => Some(array.value($index).into()),
        })
    }};
}

impl ScalarValue {
    /// Converts a value in `array` at `index` into a ScalarValue
    pub fn try_from_array(array: &ArrayRef, index: usize) -> Result<Self, String> {
        Ok(match array.data_type() {
            DataType::Boolean => typed_cast!(array, index, BooleanArray, Boolean),
            DataType::Float64 => typed_cast!(array, index, Float64Array, Float64),
            DataType::Float32 => typed_cast!(array, index, Float32Array, Float32),
            DataType::UInt64 => typed_cast!(array, index, UInt64Array, UInt64),
            DataType::UInt32 => typed_cast!(array, index, UInt32Array, UInt32),
            DataType::UInt16 => typed_cast!(array, index, UInt16Array, UInt16),
            DataType::UInt8 => typed_cast!(array, index, UInt8Array, UInt8),
            DataType::Int64 => typed_cast!(array, index, Int64Array, Int64),
            DataType::Int32 => typed_cast!(array, index, Int32Array, Int32),
            DataType::Int16 => typed_cast!(array, index, Int16Array, Int16),
            DataType::Int8 => typed_cast!(array, index, Int8Array, Int8),
            DataType::Utf8 => typed_cast!(array, index, StringArray, Utf8),
            DataType::LargeUtf8 => typed_cast!(array, index, LargeStringArray, LargeUtf8),
            DataType::List(nested_type) => {
                let list_array = array
                    .as_any()
                    .downcast_ref::<ListArray>()
                    .ok_or_else(|| "Failed to downcast ListArray".to_string())?;
                let value = match list_array.is_null(index) {
                    true => None,
                    false => {
                        let nested_array = list_array.value(index);
                        let scalar_vec = (0..nested_array.len())
                            .map(|i| ScalarValue::try_from_array(&nested_array, i))
                            .collect::<Result<Vec<ScalarValue>, String>>()?;
                        Some(scalar_vec)
                    }
                };
                ScalarValue::List(value, nested_type.data_type().clone())
            }
            DataType::Date32(DateUnit::Day) => {
                typed_cast!(array, index, Date32Array, Date32)
            }
            other => {
                return Err(format!("Downcast not available for type: {}", other));
            }
        })
    }
}

// The Table object will be used to store all the information collected
// from the parquet file
pub struct Table {
    schema: Schema,
    data: Vec<RecordBatch>,
    rows: usize,
    chunk_size: usize,
}

impl Table {
    // Reads the parquet file and stores the chunks in a vector
    // This will keep the data in memory
    pub fn read_parquet<T: AsRef<Path>>(path: T, chunk_size: usize) -> Self {
        let file = File::open(path).unwrap();
        let file_reader = SerializedFileReader::new(file).unwrap();
        let mut arrow_reader = ParquetFileArrowReader::new(Arc::new(file_reader));

        let schema = arrow_reader.get_schema().unwrap();
        let record_batch_reader = arrow_reader.get_record_reader(chunk_size).unwrap();
        let mut data: Vec<RecordBatch> = Vec::new();

        let mut rows = 0;
        for maybe_batch in record_batch_reader {
            let record_batch = maybe_batch.unwrap();
            rows += record_batch.num_rows();

            data.push(record_batch);
        }

        Self {
            schema,
            data,
            rows,
            chunk_size,
        }
    }

    // Simple writer to store the table data into a parquet file
    pub fn to_parquet<T: AsRef<Path>>(&self, path: T) {
        let file = File::create(path).unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(self.schema.clone()), None).unwrap();

        for batch in self.data.iter() {
            writer.write(&batch).unwrap();
        }

        writer.close().unwrap();
    }

    // From the schema we can extract all the information regarding
    // the data extracted from the parquet file. The schema contains
    // the name of the fields and the types of each column.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn data(&self) -> &Vec<RecordBatch> {
        &self.data
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    // Extracts the value from the selected column and index
    pub fn value(&self, column: usize, index: usize) -> Option<ScalarValue> {
        // If the selected column is larger than the available columns
        // in the schema then there is no value to collect
        if column >= self.schema.fields().len() {
            return None;
        }

        let batch = index / self.chunk_size;
        // If the index creates a batch index larger than all the available
        // batches in the data, then there is no value to collect, thus None
        if batch >= self.data.len() {
            return None;
        }

        // Selecting the array from the RecordBatch stored in the
        // data vector
        let array = self.data[batch].column(column);

        // The index argument refers to the position of the value within
        // all the rows in the table. A relative index in the batch is
        // required to access the data stored in the batch
        let index_in_batch = index % self.chunk_size;

        ScalarValue::try_from_array(array, index_in_batch).ok()
    }

    pub fn column_iterator(&self, column: usize) -> ColumnIterator {
        ColumnIterator::new(column, &self.data)
    }
}

pub struct ColumnIterator<'iter> {
    column: usize,
    data: &'iter [RecordBatch],
    index: usize,
    batch: usize,
}

impl<'iter> ColumnIterator<'iter> {
    pub fn new(column: usize, data: &'iter [RecordBatch]) -> Self {
        Self {
            column,
            data,
            index: 0,
            batch: 0,
        }
    }
}

impl<'iter> Iterator for ColumnIterator<'iter> {
    type Item = ScalarValue;

    fn next(&mut self) -> Option<Self::Item> {
        let records = self.data[self.batch].column(self.column).len();

        let (next_record, next_batch) = if self.index + 1 >= records {
            (0, self.batch + 1)
        } else {
            (self.index + 1, self.batch)
        };

        if next_batch >= self.data.len() {
            return None;
        }

        let array = self.data[self.batch].column(self.column);

        let value = ScalarValue::try_from_array(array, self.index).ok();

        self.index = next_record;
        self.batch = next_batch;

        value
    }
}

fn main() {
    let table = Table::read_parquet("data/olympics.parquet", 2000);

    let col_iter = table.column_iterator(0);

    for val in col_iter {
        if let ScalarValue::Int64(res) = val {
            println!("{:?}", res);
        }
    }
}
