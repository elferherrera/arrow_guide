# Primitive arrays

A primitive array
([PrimitiveArray](https://docs.rs/arrow/2.0.0/arrow/array/struct.PrimitiveArray.html))
is a type of array used to store a list of elements of the same type. It
includes fixed bit-width, variable-size, binary and null arrays.

Lets begin with an example of a primitive array and how the data looks like
when printed.

```rust
use arrow::buffer::Buffer;
use arrow::array::{ArrayData, PrimitiveArray};
use arrow::datatypes::{DataType, Int32Type, ToByteSlice};

use std::sync::Arc;

fn main() {
    let buffer = Buffer::from(&[0u32, 1, 2, 3, 4, 5].to_byte_slice());
    let data = ArrayData::new(DataType::Int32, 6, None, None, 0, vec![buffer], vec![]);

    let array: PrimitiveArray<Int32Type> = PrimitiveArray::<Int32Type>::from(Arc::new(data));
    println!("{:?}", array);
}
```

This time you should see in your console output something like this

```json
PrimitiveArray<Int32>
    [ 0,
      1,
      2,
      3,
      4,
      5,
    ]
```

It seems that this time the primitive array knows how to represent the data
that is stored in the buffer using the correct datatype. When we print the
array we no longer see zeros padding the data (easier for us humans to
understand, isn't it).

You may have noticed that we used the **From** trait in order to create the
array from the ArrayData. Lucky for us, in the Arrow crate there are several
ways to create arrays. 

## The array builders

Lets make our life simpler by using the constructors defined within the crate.
This constructors will do all the job of defining the buffers, data arrays and
datatypes. They will even help us define the validity buffer used to mark the
presence of null values.

For this example we will use an Int32Builder which is a type definition created
from
[PrimitiveBuilder](https://docs.rs/arrow/2.0.0/arrow/array/struct.PrimitiveBuilder.html)

```rust
use arrow::array::Int32Builder;

fn main() {
    let mut primitive_array_builder = Int32Builder::new(20);

    primitive_array_builder.append_value(5).unwrap();
    primitive_array_builder.append_value(10000).unwrap();
    primitive_array_builder.append_value(2000000).unwrap();
    primitive_array_builder.append_null().unwrap();
    primitive_array_builder.append_slice(&[1, 2, 3]).unwrap();
    primitive_array_builder.append_null().unwrap();
    primitive_array_builder
        .append_slice(&(0..10).collect::<Vec<i32>>())
        .unwrap();

    let primitive_array = primitive_array_builder.finish();
    println!("{:?}", primitive_array);

}
```

As you can see, now the array was created in a more organic way. We didn't
need to define all the elements that compose the array. This builder will let
us add as many values as we need (thanks to the
[MutableBuffer](https://docs.rs/arrow/2.0.0/arrow/buffer/struct.MutableBuffer.html)
that is used by the constructor). We can add values, slices and nulls in one
go. When there are no more values to add, the builder will create a primitive
array that represents all the data stored within the data buffer. 

It should be mentioned that once the builder finishes the array, it will clear
its memory and the builder can be used again to create another primitive array.

> **Note**. The Arrow create also has
> [BufferBuilders](https://docs.rs/arrow/2.0.0/arrow/array/struct.BufferBuilder.html)
> that behave like the array builders. They can be used to create buffers in a
> dynamic way by adding values as needed. The finish buffer can be used to
> create arrays of different types.

# Using traits

We can also create arrays by using vectors of elements. This is thanks to the
**From** trait implemented in the crate.

```rust
use arrow::array::{PrimitiveArray, Int32Array};
use arrow::datatypes::{Date64Type, Time64MicrosecondType};

fn main() {
    // Creating an array from a vector of options
    let array = Int32Array::from(vec![Some(0), None, Some(2), None, Some(4)]);
    println!("{:?}", array);

    // Creating an array from a vector of Date64Types using the into method
    let date_array: PrimitiveArray<Date64Type> =
        vec![Some(1550902545147), None, Some(1550902545147)].into();
    println!("{:?}", date_array);

    // Creating an array from a vector of Date64Types using the from method
    let date_array: PrimitiveArray<Date64Type> = 
        PrimitiveArray::<Date64Type>::from(vec![Some(1550902545147), None, Some(1550902545147)]);
    println!("{:?}", date_array);

    let time_array: PrimitiveArray<Time64MicrosecondType> = 
        (0..100).collect::<Vec<i64>>().into();
    println!("{:?}", time_array);
}
```

As you can see from these examples, it is relatively easy to create primitive
arrays to store data in memory. The create has a variety of methods to store
data in memory that follows the Arrow specification; all data is padded and
aligned. 

Also, since all the arrays store an atomic reference to the buffers, it can be
shared between processes without copying the data. However, before we venture
into data sharing is important to see what operations are defined for the arrays
and also review the available nested structures and how they can be created
using primitive arrays.
