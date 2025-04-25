use anyhow::{Context, Result};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use super::request::Request;
use super::response::HTTPCode;


pub async fn run(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address).await.unwrap();
    

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream));
                println!("[INFO] Accepted new connection")
            }
            Err(e) => {
                println!("[ERROR] {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {

    let request = read_stream(&mut stream)
        .await
        .context("Failed to read the request")?;

    let request= Request::parse(request)?;
    
    let response_code = if request.path == "/" { HTTPCode::Ok } else { HTTPCode::NotFound };
    let response = format!("HTTP/1.1 {}\r\n\r\n", response_code);

    stream
        .write_all(response.as_bytes())
        .await
        .expect("Failed to write response");

    Ok(())
}

async fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut request = Vec::<u8>::new();
    let mut stream_reader = BufReader::new(stream);

    loop {
        match stream_reader.read_until(b'\n', &mut request).await {
            Ok(0) => {
                println!("[INFO] Connection closed");
                break;
            },
            Ok(_) => {
                if request.ends_with(b"\r\n\r\n") { break };
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(request)
}

