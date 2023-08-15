use super::custom_errors::CustomErrors;

#[derive(Debug, Clone)]
pub enum Version { HTTP1, HTTP2, HTTP3 }
impl std::str::FromStr for Version {   //"HTTP/1.1".parse::<Version>()
    type Err = CustomErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Self::HTTP1),
            "HTTP/2" => Ok(Self::HTTP2),
            "HTTP/3" => Ok(Self::HTTP3),
            _ => Err(CustomErrors::InvalidVersion)
        }
    }
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::HTTP1 => write!(f, "HTTP/1.1"),
            Version::HTTP2 => write!(f, "HTTP/2"),
            Version::HTTP3 => write!(f, "HTTP/3"),
        }
    }
}