use super::custom_errors::CustomErrors;

#[derive(Debug)]
pub enum Method { GET, POST, DELETE, PUT, PATCH, HEAD, TRACE, OPTIONS, CONNECT }
impl std::str::FromStr for Method {   //"GET".parse::<Method>()
    type Err = CustomErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "TRACE" => Ok(Self::TRACE),
            "OPTIONS" => Ok(Self::OPTIONS),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(CustomErrors::InvalidMethod)
        }
    }
}
impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:?}") }
}
impl Method {
    pub fn all() -> String {
        format!("{}, {}, {}, {}, {}, {}, {}, {}, {}", Self::GET, Self::POST, Self::DELETE, Self::PUT, Self::PATCH, Self::HEAD, Self::TRACE, Self::OPTIONS, Self::CONNECT)
    }
}