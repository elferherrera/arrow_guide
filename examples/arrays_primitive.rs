use arrow::array::{ArrayData, Int32Array, Int32Builder, PrimitiveArray};
use arrow::buffer::Buffer;
use arrow::datatypes::{DataType, Date64Type, Int32Type, Time64MicrosecondType, ToByteSlice};

use std::sync::Arc;

fn main() {
    println!("Using buffer to construct primitive array");
    let buffer = Buffer::from(&[0u32, 1, 2, 3, 4, 5].to_byte_slice());
    let data = ArrayData::new(DataType::Int32, 6, None, None, 0, vec![buffer], vec![]);

    let array: PrimitiveArray<Int32Type> = PrimitiveArray::<Int32Type>::from(Arc::new(data));
    println!("{:?}", array);

    // Using Array Builder
    println!("Using array builder to construct primitive array");
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

    // Using traits
    println!("Using traits to construct primitive array");
    let array = Int32Array::from(vec![Some(0), None, Some(2), None, Some(4)]);
    println!("{:?}", array);

    let date_array: PrimitiveArray<Date64Type> =
        vec![Some(1550902545147), None, Some(1550902545147)].into();
    println!("{:?}", date_array);

    let date_array: PrimitiveArray<Date64Type> =
        PrimitiveArray::<Date64Type>::from(vec![Some(1550902545147), None, Some(1550902545147)]);
    println!("{:?}", date_array);

    let time_array: PrimitiveArray<Time64MicrosecondType> = (0..100).collect::<Vec<i64>>().into();
    println!("{:?}", time_array);
}
