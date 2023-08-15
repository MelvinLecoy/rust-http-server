pub mod method;
pub use method::Method;
pub mod version;
pub use version::Version;
pub mod custom_errors;
pub use custom_errors::CustomErrors;

use std::collections::HashMap;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub pure_path: &'a str,
    pub version: Version,
    pub headers: HashMap<&'a str, &'a str>,
    pub search_params: Option<HashMap<&'a str, &'a str>>,
    // pub params: Option<HashMap<&'a str, &'a str>>,
}
impl<'a> Request<'a> {
    pub async fn new_by_parsing(socket_stream: &mut tokio::net::TcpStream, buffer: &'a mut [u8]) -> Result<Request<'a>, CustomErrors> {
        socket_stream.read(buffer).await?;
        let whole_req = std::str::from_utf8(buffer)?.trim_end_matches(char::from(0));
        println!("Request in string form: {whole_req:?}");
        let mut rlh_mb_itr = whole_req.split("\r\n\r\n");
        let reqline_headers = rlh_mb_itr.next().ok_or(CustomErrors::InvalidFormat)?;
        let _msg_body = rlh_mb_itr.next().ok_or(CustomErrors::InvalidFormat)?;
        assert!(rlh_mb_itr.next().is_none());
        let (request_line, headers) = reqline_headers.split_once("\r\n").ok_or(CustomErrors::InvalidFormat)?;
        if let &[meth, uri, ver] = request_line.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
            let (method, mut pure_path, version, mut headers_hm, mut search_params) = (meth.parse::<Method>()?, uri, ver.parse::<Version>()?, HashMap::new(), None);
            for header in headers.split("\r\n") {
                if let &[k, v] = header.split(": ").collect::<Vec<_>>().as_slice() { headers_hm.insert(k, v); }
                else { return Err(CustomErrors::InvalidFormat); }
            }
            if let Some((path, sps)) = uri.split_once("?") {
                pure_path = path;
                let mut sp_hm = HashMap::new();
                for sp in sps.split("&") {
                    if let &[k, v] = sp.split("=").collect::<Vec<_>>().as_slice() { sp_hm.insert(k, v); }
                    else { return Err(CustomErrors::InvalidFormat); }
                }
                search_params = Some(sp_hm);
            }
            return Ok(Self { method, pure_path, version, headers: headers_hm, search_params });
        }
        else { return Err(CustomErrors::InvalidFormat); }
    }
}