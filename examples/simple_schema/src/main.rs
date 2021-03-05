mod ipc_schema_generated;
use ipc_schema_generated::my_struct::schema::{
    root_as_schema, Field, FieldArgs, Schema, SchemaArgs,
};

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

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
    let schema = Schema::create(
        &mut builder,
        &SchemaArgs {
            rows: 100,
            fields: Some(fields),
        },
    );

    builder.finish(schema, None);
    let buf = builder.finished_data();

    println!("{:?}", buf);

    // Reading the data
    let recovered_schema = root_as_schema(buf).unwrap();
    println!("{:?}", recovered_schema.rows());

    let recovered_fields = recovered_schema.fields().unwrap();
    for f in recovered_fields {
        println!("{:?}", f.name());
        println!("{:?}", f.dtype());
    }
}
