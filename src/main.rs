mod store;
use std::net::TcpListener;
use std::io::prelude::*;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write(b"Hello").unwrap();
    }
}