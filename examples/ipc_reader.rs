use std::net::{TcpListener, TcpStream};

use arrow::ipc::reader::StreamReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let ipc_reader = StreamReader::try_new(stream).unwrap();
    println!("{:?}", ipc_reader.schema());

    for batch in ipc_reader {
        println!("{:?}", batch);
    }
}
