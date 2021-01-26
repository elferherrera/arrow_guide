# The Array Data

As we discussed before, an Arrow array is made out of several components and
the way these elements are stored will define the type of array that is being
created. In the Rust Arrow crate the
[ArrayData](https://docs.rs/arrow/3.0.0/arrow/array/struct.ArrayData.html)
struct is the generic representation of the data stored in memory. All types of
arrays are made or represented using an atomic reference to an ArrayData. 

Let us understand how this struct represents an Arrow array by creating one by
using the ArrayData::new implementation.


```rust
use arrow::buffer::Buffer;
use arrow::array::ArrayData;
use arrow::datatypes::DataType;

fn main() {
    let buffer_u8 = Buffer::from(&[0u8, 1, 2, 3, 4, 5]);
    let data = ArrayData::new(DataType::Int8, 6, None, None, 0, vec![buffer_u8], vec![]);

    println!("{:?}", data);
}
```

If you print the previous code you should see the next output

```json
ArrayData { 
    data_type: Int8,
    len: 6,
    null_count: 0,
    offset: 0, 
    buffers: [
        Buffer { 
            data: Bytes { 
                ptr: 0x20300849b00,
                len: 6, 
                data: [0, 1, 2, 3, 4, 5]
            },
            offset: 0
        }
    ],
    child_data: [],
    null_bitmap: None }
```

As you can see, to create the data (ArrayData::new) it was required to input the
datatype to be stored, the number of elements in the array, a validity null
buffer, an offset, a vector of buffers and child data. Each of these values is
used to define attributes and operations in the arrays. 

Lets begin with the type of data. Each Arrow Array can store different
datatypes in memory as mentioned before. The available datatypes are defined
using the enum
[DataType](https://docs.rs/arrow/3.0.0/arrow/datatypes/enum.DataType.html) and
it follows the Arrow specification on datatypes (see
[Scehma.fbs](https://github.com/apache/arrow/blob/master/format/Schema.fbs)).
The selection of the datatype is very important because, as we saw in the
buffer chapter, the implementation needs to know what type of pointer to use
in order to access the vales stored in memory. 

The next element is the length of values stored in the data. This value
indicates how many of the values available in the buffer will be considered in
the array.

> **Note**. Remember that the buffer doesn't store the values using their
> original datatype, instead it uses u8 types

Therefore, the data array "needs" to know how many of the values from the
buffer it has to read. The offset has a similar use; it indicates the array
offset to start reading the data.

> **Tip**. Change the len and offset values from the constructor from the
> previous example to see what happens to the data array.

The null bitmap and null count are used to indicate if there are null values
stored in the array and what their positions are. Have a read at this
[section](https://github.com/apache/arrow/blob/master/docs/source/format/Columnar.rst#null-count)
of the Apache columnar format to understand how a null value is represented and
stored in the data array.

Also, during the construction of the data array we introduce an vector of
buffers to the constructor.  As we mentioned before some arrays required more
than one buffer to represent the data.  For example, an array of strings
requires a data buffer and an offset buffer (We'll see an example later).

The child data is used for nested arrays, such as a list array or a struct
array.  Each of them represent data collections that are composed of one or
more primitive arrays.


 

