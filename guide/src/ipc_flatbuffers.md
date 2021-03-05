# Detour: Flatbuffers

Before we dive into the IPC and how data is shared between multiple process
using Arrow's IPC, it is a good idea to get some understanding of what
Flatbuffers is and how it is used to serialize and deserialize data.

If you are familiar with data serialization, then feel free to skip this chapter.
We won't be discussing anything related to Arrow's IPC. This chapter will work as
a basic introduction for those that are not familiar with the process and it will
work as a foundation for understanding how data sharing works in Arrow.

## What is data serialization?

Data serialization is the process of converting an object located in memory to a
representation that can be understood by other processes, and it is done by
converting the object to a series of bytes following some sort of contract that
all processes understand.

So, imagine that we have a process that needs to send data to another process,
it could be via disk or wire, it doesn't matter at the moment. What matters is
that the data has to be in a format that both understand. Now, if the data that
needs to be sent is an integer, like 20, then the producing server could send or
store "10100" and since the receiver knows that the bytes that is receiving
represent an integer then it can translate it back to 20. This processes was
trivial for an integer, but it can become cumbersome when there are complex
structures that need to be sent/receive.

This is why the serialization protocols where created. In Rust, we can use Serde
to serialize/deserialize almost any structure under the sun. However, Arrow IPC
uses Flatbuffers because it offers advantages that are beneficial to Arrow's IPC.

## Flatbuffers

Flatbuffers is an open source
[project](https://google.github.io/flatbuffers/index.html#flatbuffers_overview)
that was originally created for game development and other performance-critical
applications. It is fast, memory efficient, and flexible. However, one of its
best features is the fact that once an object has been serialized, the process
reading the data doesn't need to unpack it back, it can extract information
as soon as it has read the buffer.

Now, let us create a small example of data serialization by constructing a
generic struct and sharing the serialized data using a TCP connection using rust
std::net functions.

### Installing the flatc compiler

Before you continue with the example you will need to install the flatc compiler. 

If you are in OSX you can use:

```
brew install flatbuffers
```

In ubuntu:

```
sudo apt install -y flatbuffers-compiler
```

and in Windows

```
choco install flatbuffers
```

Or you can also install it from the source following these
[instructions](https://google.github.io/flatbuffers/flatbuffers_guide_building.html)


### Struct serialization

In this example we are going to define a very simple schema that could represent
a table, and that would be shared between different processes. For this example,
the table schema definition will have a list of fields and each of these fields
will be defined by a name and a type.

To start you will need to create the Flatbuffer objects that define our tables.
Write the next code in a file, lets call it Schema.fbs

```
namespace MyStruct.Schema;

table Field {
    name:string;
    dtype:string;
}

table Schema {
    rows:long;
    fields:[Field];
}

root_type Schema;
```

As you can see we are creating two tables, Field and Schema. In Flatbuffers, a
**table** is the way to define an object with multiple elements, and they can be
composed of elements declared within the definition. In this case we are saying
that a the table could have a certain number of rows and a list of Fields. 

> **Note**. The Flatbuffers specification has multiple types of labels that can
> be used to describe an object. For a detailed description of all the available
> types, you should have a lot at [how to write a
> schema](https://google.github.io/flatbuffers/flatbuffers_guide_writing_schema.html)
> from the FlatBuffers project.

Now it is time to create the Rust file that will help us serialize and
deserialize the data. Run the next command using the file you created previously.

```
flatc --rust Schema.fbs
```

This should generate a rust file with the same name as the file with the
Flatbuffer schema plus a `_generated.rs` postfix. Have a look at the file and
marvel at all the object definitions the `flatc` command created for us.

#### Creating a buffer

Now comes the interesting part, the serialization of the struct. Lets start by
creating a constructor using the flatbuffers crate:

```rust,ignore
let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
```

This builder will be in charge of creating the correct conversions between
the rust objects and the Flatbuffer bytes representation.

Using the builder we can create two Field objects that will be added to
the mini table schema object we want to create:

```rust,ignore
let field_1_name = builder.create_string("col_1");
let field_1_dtype = builder.create_string("int");
let field_1 = Field::create(
    &mut builder,
    &FieldArgs {
        name: Some(field_1_name),
        dtype: Some(field_1_dtype),
    },
);

let field_2_name = builder.create_string("col_2");
let field_2_dtype = builder.create_string("int");
let field_2 = Field::create(
    &mut builder,
    &FieldArgs {
        name: Some(field_2_name),
        dtype: Some(field_2_dtype),
    },
);

let fields = builder.create_vector(&[field_1, field_2]);
```

The builder has helped us to convert the strings that represent the name and
dtype for each of the fields that will be part of the main mini schema that
represents our table. Also, since the table schema is expecting these fields to
be stored in a Flatbuffer vector, we use the builder to create the required
object for the final table schema.

With all the information that defines the table schema converted we can create
the final object. The new schema objet will look like this:

```rust,ignore
let schema = Schema::create(
    &mut builder,
    &SchemaArgs {
        rows: 100,
        fields: Some(fields),
    },
);
```

The final stage of the serialization process is to create the bytes buffer
with the serialized object. This step is done in the next lines:

```rust,ignore
builder.finish(schema, None);
let buf = builder.finished_data();
```

Pat your self in the back, we are finally done. The object is serialized and it
is ready to be shared with other processes that require this information.

#### Deserializing the buffer

To finish this example we are going to recover the information in the buffer by
reading the buffer using the `root_as_schema` function that was generated with
the flatc compiler.

```rust,ignore
let recovered_schema = root_as_schema(buf).unwrap();
println!("{:?}", recovered_schema.rows());

let recovered_fields = recovered_schema.fields().unwrap();
for f in recovered_fields {
    println!("{:?}", f.name());
    println!("{:?}", f.dtype());
}
```

As you can see, the serialization process is quite straight forward using the
generated Flatbuffers objects, and the data deserialization is just as easy. The
bytes received in the buffer don't need to be unpacked, and their information
can be extracted soon as they are read. 

In conclusion, in this small example we created a buffer of bytes that could
define a table with information about its fields. The resulting buffer is easy
to share with other processes, as it can be stored to disc or it can be shared
via a tcp stream. A similar process is done in the IPC Arrow module, obviously
with larger and more complex data structures. The schema information from the
RecordBatch is serialized to be written to any available stream, e.g. disk
writer or tcp writer. In the next chapter we are going to describe in more
detail how a RecordBatch is serialized to be consumed by other processes.