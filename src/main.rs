mod store;
mod command;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use store::Store;
use command::Command;
fn handle_client(mut stream: TcpStream, store: Arc<Mutex<Store>>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let req = String::from_utf8_lossy(&buffer[..]);
    let resp = match Command::parse(&req) {
        Command::Set(k, v, ttl) => {
            store.lock().unwrap().set(k, v, ttl);
            "OK".to_string()
        },
        Command::Get(k) => {
            match store.lock().unwrap().get(&k) {
                Some(v) => v,
                None => "(nil)".to_string(),
            }
        },
        Command::Unknown => "ERR".to_string(),
    };
    stream.write(resp.as_bytes()).unwrap();
}
fn main() {
    let store = Arc::new(Mutex::new(Store::new()));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let store = store.clone();
        thread::spawn(|| handle_client(stream, store));
    }
}