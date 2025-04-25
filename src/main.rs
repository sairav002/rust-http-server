mod http_server;
use http_server::server::run;

#[tokio::main]
async fn main() {
    match run("127.0.0.1:4221").await {
        Ok(_) => println!("Server finished"),
        Err(err) => eprintln!("Server error: {}", err)
    }
}
