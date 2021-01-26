use arrow::buffer::Buffer;
use arrow::datatypes::ToByteSlice;

fn main() {
    let buffer_u8 = Buffer::from(&[0u8, 1, 2, 3, 4, 5]);

    println!("{:?}", buffer_u8);

    unsafe {
        for i in 0..6 {
            println!("{}", *buffer_u8.as_ptr().add(i));
        }
    }

    let buffer_u32 = Buffer::from(&[0u32, 1, 2, 3, 4, 5].to_byte_slice());

    println!("{:?}", buffer_u32);
    let ptr_32 = buffer_u32.as_ptr() as *const u32;

    unsafe {
        for i in 0..6 {
            println!("{}", *ptr_32.add(i));
        }
    }
}
