use arrow::array::{
    Array, ArrayData, BooleanArray, Int32Array, Int32Builder, ListArray, ListBuilder, StringArray,
    StringBuilder, StructArray,
};
use arrow::buffer::Buffer;
use arrow::datatypes::{DataType, Field, ToByteSlice};

use std::sync::Arc;

fn main() {
    // ListArray
    let value_data = ArrayData::builder(DataType::Int32)
        .len(10)
        .add_buffer(Buffer::from(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_byte_slice(),
        ))
        .build();

    let value_offsets = Buffer::from(&[0, 2, 4, 7, 7, 8, 10].to_byte_slice());

    let list_data_type = DataType::List(Box::new(Field::new("item", DataType::Int32, false)));
    let list_data = ArrayData::builder(list_data_type.clone())
        .len(6)
        .add_buffer(value_offsets)
        .add_child_data(value_data)
        .null_bit_buffer(Buffer::from([0b00110111]))
        .build();

    let value_offsets = Buffer::from(&[0, 2, 5, 6].to_byte_slice());
    let list_data = ArrayData::builder(list_data_type)
        .len(3)
        .add_buffer(value_offsets)
        .add_child_data(list_data)
        .build();

    println!("{:?}", list_data);

    let list_array = ListArray::from(list_data);
    println!("{:?}", list_array);

    // List array with builder
    let values_builder = Int32Builder::new(10);
    let mut builder = ListBuilder::new(values_builder);

    //  [[0, 1, 2], [3, 4, 5], [6, 7]]
    builder.values().append_value(0).unwrap();
    builder.values().append_value(1).unwrap();
    builder.values().append_value(2).unwrap();
    builder.append(true).unwrap();
    builder.values().append_value(3).unwrap();
    builder.values().append_value(4).unwrap();
    builder.values().append_value(5).unwrap();
    builder.append(true).unwrap();
    builder.values().append_value(6).unwrap();
    builder.values().append_value(7).unwrap();
    builder.append(true).unwrap();
    let list_array = builder.finish();

    println!("{:?}", list_array);

    // StringArray
    let values: [u8; 20] = [
        b'h', b'e', b'l', b'l', b'o', b'f', b'r', b'o', b'm', b'A', b'p', b'a', b'c', b'h', b'e',
        b'A', b'r', b'r', b'o', b'w',
    ];
    let offsets: [i32; 6] = [0, 5, 9, 9, 15, 20];

    let array_data = ArrayData::builder(DataType::Utf8)
        .len(5)
        .add_buffer(Buffer::from(offsets.to_byte_slice()))
        .add_buffer(Buffer::from(&values[..]))
        .null_bit_buffer(Buffer::from([0b00011011]))
        .build();
    let string_array = StringArray::from(array_data);
    println!("{:?}", string_array);
    println!("Value: {:?}", string_array.value(0));
    println!("Value: {:?}", string_array.value(1));
    println!("Value: {:?}", string_array.value(2));

    println!("\nCreating an String Array using builder");
    let mut builder = StringBuilder::new(10);
    builder.append_value("one").unwrap();
    builder.append_value("two").unwrap();
    builder.append_value("three").unwrap();
    builder.append_null().unwrap();
    builder.append_value("four").unwrap();

    let string_array = builder.finish();
    println!("{:?}", string_array);

    // StructArray ArrayData
    let boolean_data = ArrayData::builder(DataType::Boolean)
        .len(5)
        .add_buffer(Buffer::from([0b00010000]))
        .null_bit_buffer(Buffer::from([0b00010001]))
        .build();

    let int_data_b = ArrayData::builder(DataType::Int32)
        .len(5)
        .add_buffer(Buffer::from([0, 28, 42, 0, 0].to_byte_slice()))
        .null_bit_buffer(Buffer::from([0b00000110]))
        .build();

    let int_data_c = ArrayData::builder(DataType::Int32)
        .len(5)
        .add_buffer(Buffer::from([1, 2, 3, 4, 5].to_byte_slice()))
        .null_bit_buffer(Buffer::from([0b00011111]))
        .build();

    let mut field_types = vec![];
    field_types.push(Field::new("a", DataType::Boolean, false));
    field_types.push(Field::new("b", DataType::Int32, false));
    field_types.push(Field::new("c", DataType::Int32, false));

    let struct_array_data = ArrayData::builder(DataType::Struct(field_types))
        .len(5)
        .add_child_data(boolean_data)
        .add_child_data(int_data_b)
        .add_child_data(int_data_c)
        .build();
    let struct_array = StructArray::from(struct_array_data);
    println!("{:?}", struct_array);

    // Constructing StructArray from vector
    let struct_array = StructArray::from(vec![
        (
            Field::new("b", DataType::Boolean, false),
            Arc::new(BooleanArray::from(vec![false, false, true, true])) as Arc<dyn Array>,
        ),
        (
            Field::new("c", DataType::Int32, false),
            Arc::new(Int32Array::from(vec![42, 28, 19, 31])),
        ),
    ]);
    println!("{:?}", struct_array);
}
