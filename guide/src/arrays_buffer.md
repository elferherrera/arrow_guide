# The Arrow Buffer

The [Buffer](https://docs.rs/arrow/2.0.0/arrow/buffer/struct.Buffer.html) is the
main data container in the Arrow Array. Depending on the type of array that is
being created, it can have one or many buffers holding information. So, this
means that an array could include a combination of a values buffer, a validity
bitmap buffer and an offset buffer.

However, all buffers are the same. A buffer is the representation of a
continuous memory region that is used to store data in memory. According to the
Arrow specification a buffer should be aligned and padded using multiples of 8
or 64 bytes.

To see how a buffer looks in Rust lets create one.

```rust
use arrow::buffer::Buffer;

fn main() {
    let buffer_u8 = Buffer::from(&[0u8, 1, 2, 3, 4, 5]);
    println!("{:?}", buffer_u8);
}
```
> **Note**. Don't use the "Run this code" button. The Arrow crate is not loaded
> in the book and it will produce an error

> **Tip**. If you use **"{:#?}"** in the println! macro you should see a
> formated version of the struct in your screen

If you printed the previous code you should see something like this:

```json
Buffer { 
    data: Bytes { 
        ptr: 0x1dcab5b5400, 
        len: 6,
        data: [0, 1, 2, 3, 4, 5] 
    }, 
    offset: 0 
}
```

As it can be seen, a buffer is made out of a Bytes structure and an offset. The
Bytes structure is used to represent the data in memory by using a pointer, the
number of elements it has, and the data itself. The offset is used by the arrays
to indicate an offset for reading the stored values. By creating a buffer the
constructor has allocated in memory enough bytes to store the supplied values
and it has given a pointer to access the stored data. It should also be noted
that the resulting buffer is inmutable.

The normal usage of the Arrays don't require you to use pointer arithmetic to
access the data in the buffer, but as a learning experience lets use the pointer
to access the data in memory.

```rust
use arrow::buffer::Buffer;

fn main() {
    let buffer_u8 = Buffer::from(&[0u8, 1, 2, 3, 4, 5]);
    
    unsafe {
        for i in 0..5 {
            println!("{}", *buffer_u8.as_ptr().add(i));
        }
    }
}
```

If you are following the examples, you should see printed the values 0 to 5 in
you screen. 

Now lets change the type of elements the buffer is holding to u32 and see what
happens to the buffer.

```rust
use arrow::buffer::Buffer;
use arrow::datatypes::ToByteSlice;

fn main() {
    let buffer_u32 = Buffer::from(&[0u32, 1, 2, 3, 4, 5].to_byte_slice());
    
    println!("{:?}", buffer_u32);
}
```

In this case a new element is introduced to the code; the **ToByteSlice** trait.
The ToByteSlice trait exposes the method to_byte_slice for *[T]* and *T* which
allows us to allocate the required memory using u8 as the base unit. This means
that now each *u32* number will be represented by four *u8* numbers. That can be
seen better by printing the new buffer:

```json
Buffer { 
    data: Bytes { 
        ptr: 0x1ad7d5ffb00,
        len: 24,
        data: [0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0] 
    },
    offset: 0
}
```

Now the length of the buffer is 24, even though we stored only 6 digits, and
there are extra zeros in the data array. What happened is that each of the u32
numbers is represented using multiples of u8 numbers. Now each number in the
array is padded and aligned. Neat isn't it?. 

> **Tip**. Try increasing the number of values used to create the buffer to see
> what happens to the len. Also, try using numbers larger than 255 to see
> how the number representation changes in the data array.

Again, as a learning experience, you can use the raw pointer to access all the
elements from the buffer. However, since the buffer pointer is a `*const u8`
you need to cast it to a `*const u32`.

```rust
use arrow::buffer::Buffer;
use arrow::datatypes::ToByteSlice;

fn main() {
    let buffer_u32 = Buffer::from(&[0u32, 1, 2, 3, 4, 5].to_byte_slice());
    
    let ptr_32 = buffer_u32.as_ptr() as *const u32;
    unsafe {
        for i in 0..6 {
            println!("{}", *ptr_32.add(i));
        }
    }
}
```

With your newly earned understanding of how a buffers works, lets start creating
Arrow arrays.