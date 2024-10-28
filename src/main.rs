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

struct RequestHeader {
    api_key: u16,
    api_version: u16,
    correlation_id: u32,
    _client_id: String
}

async fn parse_header(stream: &mut TcpStream) -> Result<RequestHeader> {
    let mut reader = BufReader::new(stream);
    let _header_len = reader.read_u32().await? as usize;
    let api_key = reader.read_u16().await?;
    let api_version = reader.read_u16().await?;
    let correlation_id = reader.read_u32().await?;

    Ok(RequestHeader {
        api_key,
        api_version,
        correlation_id,
        _client_id: "".to_string()
    })
}

async fn get_resp(header: RequestHeader) -> Result<BytesMut> {
    let mut resp_msg = BytesMut::new();
    resp_msg.put_u32(header.correlation_id);
    match header.api_key {
        18 => api_version(&mut resp_msg, header.api_version).await,
        _ => {}
    }
    let mut resp = BytesMut::new();
    resp.put_u32(resp_msg.len() as u32);
    resp.put(resp_msg);
    Ok(resp)
}

async fn api_version(resp: &mut BytesMut, api_version: u16) {
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

async fn handle_request(mut stream: TcpStream) -> Result<()> {
    let header = parse_header(&mut stream).await?;
    let mut writter = BufWriter::new(&mut stream);
    let resp = get_resp(header).await?;
    writter.write_all(&resp).await?;

    writter.flush().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("0.0.0.0:9092").await?;
    
    loop {
        let stream = listener.accept().await;
        match stream {
            Result::Ok((stream, _)) => {
                println!("accepted new connection");
                let response = handle_request(stream).await;
                if let Err(e) = response {
                    println!("error: {}", e);
                }
                println!("finished handling connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    
}
