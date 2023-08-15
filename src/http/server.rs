use super::request::{Request, Method, Version, CustomErrors};
use super::response::{Response, StatusCode};
use std::collections::HashMap;

pub struct Server { html_dir: String }
impl Server {
    pub fn new(html_dir: String) -> Self { Self { html_dir } }
    pub async fn run(&self, ip_addr: &str) -> std::io::Result<()> {
        println!("TCP Server listening on {ip_addr}");
        let listener = tokio::net::TcpListener::bind(ip_addr).await?;
        loop {
            let (mut buffer, (mut socket, _)) = ([0; 2048], listener.accept().await?);
            let parsed_request = Request::new_by_parsing(&mut socket, &mut buffer).await;
            if let Err(err) = &parsed_request { println!("{err}"); }
            if let Ok(req) = &parsed_request { println!("Request in obj: {req:?}"); }
            let response = match &parsed_request {
                Ok(req) => self.handle_request(&req.method, req.version.clone(), req.pure_path).await,
                Err(custom_err) => match *custom_err {
                    CustomErrors::StreamReadError => Response::new(Version::HTTP1, StatusCode::INTERNAL_SERVER_ERROR, None , None),
                    CustomErrors::InvalidUtf8 | CustomErrors::InvalidFormat => Response::new(Version::HTTP1, StatusCode::BAD_REQUEST, None , None),
                    CustomErrors::InvalidMethod => {
                        let headers = HashMap::from([("Allow".to_string(), Method::all())]);
                        Response::new(Version::HTTP1, StatusCode::METHOD_NOT_ALLOWED, Some(headers) , None)
                    },
                    CustomErrors::InvalidVersion => Response::new(Version::HTTP1, StatusCode::VERSION_NOT_SUPPORTED, None , None),
                }
            };
            response.server_reply(&mut socket).await.unwrap_or_else(|err| eprintln!("Error with TcpStream writing response: {err}"));
        }
    }
    pub async fn handle_request(&self, method: &Method, version: Version, pure_path: &str) -> Response {
        let mut path = format!("{}{pure_path}.html", self.html_dir);
        if !std::path::Path::new(&path).is_file() { path = format!("{}{pure_path}/index.html", self.html_dir); }
        match method {
            Method::GET => {
                if let Ok(body) = tokio::fs::read_to_string(path).await { Response::new(version, StatusCode::OK, None, Some(body)) }
                else { Response::new(version, StatusCode::NOT_FOUND, None, None) }
            },
            Method::POST => Response::new(version, StatusCode::OK, None , None),             //yet to process data, 201: create resource, 204: no response, 400
            Method::DELETE => Response::new(version, StatusCode::NO_CONTENT, None , None),   //yet to do deletion, 200, 404
            Method::PUT => Response::new(version, StatusCode::NO_CONTENT, None , None),      //yet to overwrite, 201: create resource, 200: overwrite, 404
            Method::PATCH => Response::new(version, StatusCode::NO_CONTENT, None , None),    //yet to update, 200, 304, 400
            _ => Response::new(version, StatusCode::OK, None, None)
        }
    }
}