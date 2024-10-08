#![allow(unused_imports)]
use std::{io::{Cursor, Read, Seek, Write}, net::TcpListener};

use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");

                let mut buf = [0_u8; 4];
                _stream.read_exact(buf.as_mut_slice()).unwrap();
                let mut render = Cursor::new(buf);
                let len = render.read_u32().await.unwrap();

                let mut request = vec![0u8; len.try_into().unwrap()];
                _stream.read(request.as_mut_slice()).unwrap();

                let mut render = Cursor::new(request);
                render.read_u16().await.unwrap(); // request_api_key
                render.read_u16().await.unwrap(); // request_api_version
                let correlation_id = render.read_u32().await.unwrap();
                println!("correlation id: {}", correlation_id);
                _stream.write_all([0, 0, 0, 4].as_slice()).unwrap();
                _stream.write_all(&correlation_id.to_be_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
