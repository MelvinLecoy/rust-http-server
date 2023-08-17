pub mod status_code;
pub use status_code::StatusCode;

use super::request::Version;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Response {
    pub version: Version,
    pub status: StatusCode,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}
impl Response {
    pub fn new(version: Version, status: StatusCode, headers: Option<HashMap<String, String>>, body: Option<String>) -> Self {
        Self { version, status, headers, body }
    }
    pub async fn server_reply(&self, socket: &mut tokio::net::TcpStream) -> std::io::Result<()> {
        let mut res = format!("{} {}\r\n", self.version, self.status);
        if let Some(headers) = &self.headers {
            for (k, v) in headers { res.push_str(&format!("{k}: {v}\r\n")) }
        }
        res.push_str("\r\n");
        if let Some(body) = &self.body { res.push_str(body); }
        println!("Response in full string: {res:?}\n");
        socket.write_all(res.as_bytes()).await
    }
}