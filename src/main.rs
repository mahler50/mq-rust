#![allow(unused_imports)]
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

#[repr(u16)]
enum ErrorCode {
    UnSupportAPIVersion = 35,
}

struct Header {
    api_key: u16,
    api_version: u16,
    correlation_id: u32
}

async fn parse_header(stream: &mut TcpStream) -> Result<Header> {
    let mut reader = BufReader::new(stream);
    let _header_len = reader.read_u32().await? as usize;
    let api_key = reader.read_u16().await?;
    let api_version = reader.read_u16().await?;
    let correlation_id = reader.read_u32().await?;

    Ok(Header {
        api_key,
        api_version,
        correlation_id,
    })
}

async fn handle_request(mut stream: TcpStream) -> Result<()> {
    let header = parse_header(&mut stream).await?;
    let mut writter = BufWriter::new(&mut stream);
    writter.write_u32(0).await?;
    writter.write_u32(header.correlation_id).await?;
    if header.api_version > 4 {
        writter.write_u16(ErrorCode::UnSupportAPIVersion as u16).await?;
    }

    writter.flush().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("0.0.0.0:9092").await?;
    
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _)) => {
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
