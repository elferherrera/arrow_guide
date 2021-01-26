use arrow::array::ArrayData;
use arrow::buffer::Buffer;
use arrow::datatypes::DataType;

fn main() {
    let buffer_u8 = Buffer::from(&[0u8, 1, 2, 3, 4, 5]);
    println!("{:?}", buffer_u8);

    let data = ArrayData::new(DataType::Int32, 6, None, None, 0, vec![buffer_u8], vec![]);
    println!("{:?}", data);
}
