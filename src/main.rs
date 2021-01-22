mod store;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    stream.write(&buffer).unwrap();
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_client(stream));
    }
}