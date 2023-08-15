#![allow(unused)]
#[derive(Debug)]
pub struct StatusCode { status: u16, text: &'static str }
impl StatusCode {
    pub const OK: Self = Self { status: 200, text: "OK" };
    pub const CREATED: Self = Self { status: 201, text: "Created" };
    pub const NO_CONTENT: Self = Self { status: 204, text: "No Content" };
    pub const BAD_REQUEST: Self = Self { status: 400, text: "Bad Request" };
    pub const UNAUTHORIZED: Self = Self { status: 401, text: "Unauthorized" };
    pub const FORBIDDEN: Self = Self { status: 403, text: "Forbidden" };
    pub const NOT_FOUND: Self = Self { status: 404, text: "Not Found" };
    pub const METHOD_NOT_ALLOWED: Self = Self { status: 405, text: "Method Not Allowed" };
    pub const INTERNAL_SERVER_ERROR: Self = Self { status: 500, text: "Internal Server Error" };
    pub const VERSION_NOT_SUPPORTED: Self = Self { status: 505, text: "HTTP Version Not Supported" };
}
impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{} {}", self.status, self.text) }
}