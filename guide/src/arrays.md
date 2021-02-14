# Arrays

The Array is the center piece of the Rust Apache Arrow implementation. An array
is defined by different pieces of data and metadata, as it can be seen in the
next image.

<p align="center">
  <img src="images/layout.png">
</p>

From the image it can be seen that an Array is composed of one or more buffers,
a validity bitmap and a datatype definition. By using an Arrow Array, you can
map complex or nested data structures into memory, and with the data ordered
and loaded you can shared it across several processes using a RecordBatch. 

In Rust, the Array trait is the building block for all the available types of 
data containers. These include:

- [BinaryArray](https://docs.rs/arrow/3.0.0/arrow/array/type.BinaryArray.html)
  An array where each element is a byte whose maximum length is represented by a
  i32. 

- [DictionaryArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.DictionaryArray.html)
  A dictionary array where each element is a single value indexed by an integer
  key

- [FixedSizeBinaryArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.FixedSizeBinaryArray.html)
  A type of FixedSizeListArray whose elements are binaries.

- [FixedSizeListArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.FixedSizeBinaryArray.html)
  A type of FixedSizeListArray whose elements are binaries.

- [LargeBinaryArray](https://docs.rs/arrow/3.0.0/arrow/array/type.LargeBinaryArray.html)
  An array where each element is a byte whose maximum length is represented by a
  i64.

- [LargeListArray](https://docs.rs/arrow/3.0.0/arrow/array/type.LargeListArray.html)
  A list array where each element is a variable-sized sequence of values with
  the same type whose memory offsets between elements are represented by a i64.

- [LargeStringArray](https://docs.rs/arrow/3.0.0/arrow/array/type.LargeStringArray.html)
  An array where each element is a variable-sized sequence of bytes representing
  a string whose maximum length (in bytes) is represented by a i64.

- [ListArray](https://docs.rs/arrow/3.0.0/arrow/array/type.ListArray.html) A
  list array where each element is a variable-sized sequence of values with the
  same type whose memory offsets between elements are represented by a i32.

- [PrimitiveArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.PrimitiveArray.html)
  Array whose elements are of primitive types.

- [StringArray](https://docs.rs/arrow/3.0.0/arrow/array/type.StringArray.html)
  An array where each element is a variable-sized sequence of bytes representing
  a string whose maximum length (in bytes) is represented by a i32.

- [StructArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.StructArray.html)
  A nested array type where each child (called field) is represented by a
  separate array.

- [NullArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.NullArray.html) An
  Array where all elements are nulls

- [UnionArray](https://docs.rs/arrow/3.0.0/arrow/array/struct.UnionArray.html)
  An Array that can represent slots of varying types.


Each of these containers follow a set of rules in order to define some sort of
behaviour. For example, a PrimitiveArray is made out of elements of the same
datatype and it contains one data buffer and one validity buffer. Or a
StructArray is a nested Array containing child fields that represent separate
PrimitiveArrays. By using a combination of these arrays the user
is capable of storing a variety of data in memory. 

> **Tip**. To have a better idea of the components that make each of the
> mentioned arrays and how they work together have a look at this
> [section](https://github.com/apache/arrow/blob/master/docs/source/format/Columnar.rst#physical-memory-layout)
> of the columnar format. 

Given the different components that define an array, it is important to
understand the basic unit that allocates the required memory used to hold data;
the buffer.
