#[cfg(any(test, doctest))]
mod guide {
    doc_comment::doctest!("../guide/src/arrays_buffer.md");
    doc_comment::doctest!("../guide/src/arrays_data.md");
    doc_comment::doctest!("../guide/src/arrays_primitive.md");
    doc_comment::doctest!("../guide/src/arrays_nested.md");
}
