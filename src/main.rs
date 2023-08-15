mod http;
use http::server::Server;

#[tokio::main]
async fn main() {
    let html_dir = std::env::var("DIR_PATH").unwrap_or(format!("{}/src/html", env!("CARGO_MANIFEST_DIR")));
    let server = Server::new(html_dir).run("127.0.0.1:8080").await;
    server.unwrap_or_else(|err| eprintln!("Error with tokio TcpListener before connection: {err}"));
}