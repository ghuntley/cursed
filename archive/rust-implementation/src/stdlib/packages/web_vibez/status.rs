/// Status code wrapper for web_vibez
pub use crate::stdlib::net::http::response::StatusCode as HttpStatusCode;

/// Status class enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusClass {
    Informational, // 1xx
    Success,       // 2xx
    Redirection,   // 3xx
    ClientError,   // 4xx
    ServerError,   // 5xx
}

impl From<u16> for StatusClass {
    fn from(code: u16) -> Self {
        match code {
            100..=199 => StatusClass::Informational,
            200..=299 => StatusClass::Success,
            300..=399 => StatusClass::Redirection,
            400..=499 => StatusClass::ClientError,
            500..=599 => StatusClass::ServerError,
            _ => StatusClass::ServerError,
        }
    }
}
