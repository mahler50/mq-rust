#![allow(unused_imports)]
use anyhow::{Ok, Result};
use bytes::{BufMut, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream}, stream,
};

#[repr(u16)]
enum ErrorCode {
    UnSupportAPIVersion = 35,
}

// struct Header {
//     api_key: u16,
//     api_version: u16,
//     correlation_id: u32,
//     _client_id: String
// }

async fn handle_api_version(resp: &mut BytesMut, api_version: u16) {
    if api_version > 4 {
        resp.put_u16(ErrorCode::UnSupportAPIVersion as u16);
    } else {
        resp.put_u16(0);
    }
    resp.put_u8(2);
    // api_key
    resp.put_u16(18);
    // min version
    resp.put_u16(0);
    // max version
    resp.put_u16(4);
    // throttle_time_ms
    resp.put_u32(0);
    resp.put_u8(0);
    resp.put_u8(0);
}

async fn handle_connections(mut stream: TcpStream) -> Result<()> {
    loop {
        let mut reader = BufReader::new(&mut stream);
        // parse request header
        let _header_len = reader.read_u32().await? as usize;
        let api_key = reader.read_u16().await?;
        let api_version = reader.read_u16().await?;
        let correlation_id = reader.read_u32().await?;

        // create response
        let mut resp_msg = BytesMut::new();
        resp_msg.put_u32(correlation_id);
        match api_key {
            18 => handle_api_version(&mut resp_msg, api_version).await,
            _ => {}
        }

        // write response to stream
        let mut resp = BytesMut::new();
        resp.put_u32(resp_msg.len() as u32);
        resp.put(resp_msg);
        let mut writer = BufWriter::new(&mut stream);
        writer.write_all(&resp).await?;
        writer.flush().await?;
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("0.0.0.0:9092").await?;
    
    loop {
        let stream = listener.accept().await;
        match stream {
            Result::Ok((stream, _)) => {
                tokio::spawn(async move { 
                    if let Err(e) = handle_connections(stream).await {
                        println!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
