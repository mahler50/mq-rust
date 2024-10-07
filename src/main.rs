#![allow(unused_imports)]
use std::{net::TcpListener, io::Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 7];
                stream.write(&buf).unwrap();
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
