# Reading Parquet Files

A [parquet](https://parquet.apache.org) file is a columnar storage format that
is designed to efficiently store data in disk. Storing data using this format
presents several [advantages](https://databricks.com/glossary/what-is-parquet)
and you are invited to have a look a them and conclude if these format could be of
use for your project. 

In this chapter we are going to see how to open a parquet file using the
[parquet](https://crates.io/crates/parquet) crate. The parquet crate is one of
several crates that forms part of the Rust arrow suite. The reader that we are
going to use from this crate reads a file in chunks and for each chunk it
creates a RecordBach that can be consumed. As you will see with the example,
these operations are fairly straight forward thanks to the parquet crate.

However, in order to make this chapter a bit more interesting, we are going to
create a Table struct that can be used to read and write parquet files. The
Table struct will implement some functions that will allow us to maintain
information in memory for further use and to extract specific values from them
either using an index or an iterator.

## The data

The data that was used to test this code can be found from this
[page](https://domohelp.domo.com/hc/en-us/articles/360043931814-Fun-Sample-DataSets).
If you want to use the same dataset you will need to download the "120 Years of
Olimpic History" and convert it to a parquet file. The easiest way to do it is
by loading the csv file using pandas
([pandas.read_parquet](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.read_parquet.html))
and save it with pandas
([df.to_parquet](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.to_parquet.html)).

> **Note**. Keep in mind that the code that we are going to create can be used
> to read any parquet file. So don't worry if you are unable to convert the
> previously mentioned file. As long as you have a parquet file you are good to
> go.

## The Table struct

The module that we are going to use to read the parquet is the
[parquet::arrow](https://docs.rs/parquet/3.0.0/parquet/arrow/index.html). This
module already defines a reader that can be used to extract the information in
chunks. However, since we want to keep the data in memory to use it for further
analysis, we will create and compose a struct called Table. 

The Table struct will maintain a vector with the information extracted from the
parquet file and this data will be used to extract specific values from the
columns.

```rust
use arrow::{
    record_batch::RecordBatch,
    datatypes::Schema,
};
use parquet::{
    arrow::{ArrowReader, ArrowWriter, ParquetFileArrowReader},
    file::reader::SerializedFileReader,
};
use std::sync::Arc;
use std::fs::File;
use std::path::Path;

// The Table struct. This object will represent the data read from the
// parquet files and it will be our entry point to any value in the file
pub struct Table {
    // We mantain a copy of the RecordBatch schema to keep handy the
    // file's metadata information.
    schema: Schema,
    data: Vec<RecordBatch>,
    rows: usize,
}

impl Table {
    pub fn read_parquet<T: AsRef<Path>>(path: T, chunk_size: usize) -> Self {
        // Using the parquet Arrow reader to extract batches of data
        // from the file to keep them in memory
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

        Self { schema, data, rows }
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

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn data(&self) -> &Vec<RecordBatch> {
        &self.data
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
}

fn main() {
    let table = Table::read_parquet("data/olympics.parquet", 2000);
    println!("Number of rows: {}", table.rows())
}
```

The most important functions from the struct are the **read_parquet** and
**to_parquet**. These represent the backbone used to manipulate parquet files.
In the **read_parquet** function we are reading the file in chunks or batches
using the
[ParquetFileArrowReader](https://docs.rs/parquet/3.0.0/parquet/arrow/arrow_reader/struct.ParquetFileArrowReader.html).
These batches, which are stored in a vector called data, will be our reference
for the next functions that we are going to implemented on Table. To write data
back to a parquet file we are using the
[ArrowWriter](https://docs.rs/parquet/3.0.0/parquet/arrow/arrow_writer/struct.ArrowWriter.html)
struct, which writes the data to the desired path. As you can see, the parquet
crate has everything we need to read from and store data in parquet files. That
is very convenient and helpful.

To make the Table struct a bit useful for further work, we are also keeping a
copy of the RecordBatch schema in the table. This will make our life easier
whenever we want to extract the file's metadata. We also added some helper
functions in order to make the Table object a bit more useful.

Go ahead and compile this struct together with the main function to read and
write a parquet file. 

Well, writing and reading data wasn't that hard. That's thanks to the great work
put into the parquet crate. Now, since we have created **Table** to read the
files, lets continue by giving it a bit more functionality to learn more about
the Arrow datatypes.

## Getting a value

Here comes the interesting part of the Table struct; to extract a value from the
RecordBatches. One could be tempted to simply use the vector holding the
RecordBatches and try to read the values from there. Let say we could use an
index to select a RecordBatch from the vector and then using the RecordBatch
[column](https://docs.rs/arrow/3.0.0/arrow/record_batch/struct.RecordBatch.html#method.column)
method we could select a column from the RecordBatch. With the desired column
available we could select any value from it. That sounds straight
forward, right?. If only Rust were that simple.

One thing that should be noted from the RecordBatch
[column](https://docs.rs/arrow/3.0.0/arrow/record_batch/struct.RecordBatch.html#method.column)
method is that the return signature is **&ArrayRef** which is an alias for and
**Arc\<dyn Array\>**. This means that the method returns a reference to an
object that implements dynamically the **Array** trait, not an explicit type of
Arrow array. This does make sense, since the return column can be of any Arrow
[datatype](https://docs.rs/arrow/3.0.0/arrow/datatypes/index.html), Rust needs
to know dynamically if the values read from the column are an integer, float,
string or a list. 

That's why the **Array** trait is so useful in this case. It lets us work with
any array that implements the **Array** trait without worrying about its
specific type. However, this complicates our life because now we don't have an
specific type of array and thus we can not extract a value with its type from
the column. So, how are we going to access the real value from the columns?.

## Enter the enums

One way in which we can access the data from any array that implements the
**Array** trait is by using the **as_any** method available to us via the
**AsAny** trait. The **AsAny** trait exposes the function **dowcast_ref** that,
as long as it is possible, downcasts this generic array to the specific array. 
We can do this for any column we would like to read data from. This approach
works but it is not the most flexible approach we can take.

Another thing we could do is to define an Enum that encapsulates all possible types
that could be found when reading the file. The advantage of using an enum this
way is that we can implement a unique function that converts or downcasts the
returned array into each of the possible Arrow arrays types.

In order to be able to downcast the Array to the desired array type we are going
to take advantage of the previously mentioned fact that the **Array** trait
implements the **AsAny** trait for all the Arrow array types. And to make it
even more flexible we are going to help ourselves by writing all the helper
functions required for each type we are going the use a macro to do the job for
us.

Have a look at the implementation of the enum ScalarValue

```rust
use arrow::{
    array::{
        Array, ArrayRef, BooleanArray, Date32Array, Float32Array, Float64Array, Int16Array,
        Int32Array, Int64Array, Int8Array, LargeStringArray, ListArray, StringArray, UInt16Array,
        UInt32Array, UInt64Array, UInt8Array,
    },
    datatypes::{DataType, DateUnit, Field, Schema, TimeUnit},
    record_batch::RecordBatch,
};

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

// Helper macro that is used to create the function that downcasts
// an array to the correct type of array. This is done thanks to all
// the defined Arrow data types.
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
```

The **try_from_array** function uses an array, which until this point we only
know that is an object that implements dynamically the **Array** trait, and
downcasts it to the correct Arrow array type. This is done thanks to the options
defined in the ScalarValue enum and the Arrow implementation. Now, with this
enum under our belt we can implement the missing functions in the Table struct.


## The complete implementation

With the ScalarValue enum defined, we can write the missing function from Table;
collect a value from a column. This missing function will extract a value from a
column using an index and returns it with the correct type. Also, since it may
be useful to have a way to loop though all the values in a column, we can
implement the iterator trait for a column.

The final implementation of all the code is presented next.

```rust
use arrow::{
    array::{
        Array, ArrayRef, BooleanArray, Date32Array, Float32Array, Float64Array, Int16Array,
        Int32Array, Int64Array, Int8Array, LargeStringArray, ListArray, StringArray, UInt16Array,
        UInt32Array, UInt64Array, UInt8Array,
    },
    datatypes::{DataType, DateUnit, Field, Schema, TimeUnit},
    record_batch::RecordBatch,
};

use parquet::{
    arrow::{ArrowReader, ArrowWriter, ParquetFileArrowReader},
    file::reader::SerializedFileReader,
};

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

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

pub struct Table {
    schema: Schema,
    data: Vec<RecordBatch>,
    rows: usize,
    // We keep the batch chunk size to calculate a relative index
    // to access the information from the data vector
    chunk_size: usize,
}

impl Table {
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

    pub fn to_parquet<T: AsRef<Path>>(&self, path: T) {
        let file = File::create(path).unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(self.schema.clone()), None).unwrap();

        for batch in self.data.iter() {
            writer.write(&batch).unwrap();
        }

        writer.close().unwrap();
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn data(&self) -> &Vec<RecordBatch> {
        &self.data
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    // Function to get a value from a column in the table
    // The function will search in the batches from the data
    // vector and returns the selected value with its correct
    // datatype
    pub fn value(&self, column: usize, index: usize) -> Option<ScalarValue> {
        if column >= self.schema.fields().len() {
            return None;
        }

        let batch = index / self.chunk_size;
        if batch >= self.data.len() {
            return None;
        }

        let array = self.data[batch].column(column);
        let index_in_batch = index % self.chunk_size;

        ScalarValue::try_from_array(array, index_in_batch).ok()
    }


    pub fn column_iterator(&self, column: usize) -> ColumnIterator {
        ColumnIterator::new(column, &self.data)
    }
}

// Iterator to loop through all the values in a column using
// as return value a ScalarValue
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
```

It should be noted that without the definition of the **ScalarValue** enum, it
would have been impossible to keep the return value generic for any data type
defined in the Arrow implementation. There was no way to create an specific
Arrow datatype return value for this function. Well, maybe we could have done it
with a new trait, but we would have had to do more work than the one we just
did. The enum has saved our day.

Have fun compiling the code and testing reading and writing different
parquet files. 

## Conclusion

From this and the previous examples that we saw in this section, we hope you get
and idea of how you could use the Arrow implementation to read data from
different type of files and use them for data analysis and calculations. As you
can see, the Rust Arrow suite already has several methods and structs that make
these operations simple to implement.

One thing that should be mention is that the implementation of the ScalarValue
enum is a simplification of the approach used in the
[Datafusion](https://docs.rs/datafusion/3.0.0/datafusion/) crate. The objective
of Datafusion is to create an interface for doing complex data operations using
Arrow as the data backbone. It implements a
[DataFrame](https://docs.rs/datafusion/3.0.0/datafusion/dataframe/trait.DataFrame.html)
which is a more advanced and complex version of **Table** struct we just created
in this example. It aims to become a Pandas analogue in Rust. We are
going to discuss Datafusion in future chapters but before that, we are going to
talk about IPC (interprocess communication) and how Arrow is used to share data
between processes. 