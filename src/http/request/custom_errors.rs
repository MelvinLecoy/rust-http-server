#[derive(Debug)]
pub enum CustomErrors { StreamReadError, InvalidUtf8, InvalidFormat, InvalidMethod, InvalidVersion }
impl From<std::io::Error> for CustomErrors {
    fn from(_: std::io::Error) -> Self { Self::StreamReadError }
}
impl From<std::str::Utf8Error> for CustomErrors {
    fn from(_: std::str::Utf8Error) -> Self { Self::InvalidUtf8 }
}
impl std::fmt::Display for CustomErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomErrors::StreamReadError => write!(f, "Error while reading TcpStream into buffer"),
            CustomErrors::InvalidUtf8 => write!(f, "Invalid request input for UTF-8 decoding"),
            CustomErrors::InvalidFormat => write!(f, "Invalid HTTP request format"),
            CustomErrors::InvalidMethod => write!(f, "Invalid HTTP method"),
            CustomErrors::InvalidVersion => write!(f, "Invalid HTTP version"),
        }
    }
}