#[allow(unused_imports)]
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    

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

async fn handle_connection(mut stream: TcpStream) {
    stream
        .write_all(b"HTTP/1.1 200 OK\r\n\r\n")
        .await
        .expect("[ERROR] Failed to write response");
}
