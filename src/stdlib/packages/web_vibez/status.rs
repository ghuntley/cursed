use crate::web::StatusCode;
use crate::error::Error;
/// fr fr HTTP status codes for web_vibez - all the statuses you need bestie
use std::fmt;

/// fr fr HTTP status codes enum - comprehensive coverage no cap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusCode {
    // 1xx Informational - keep going vibes
    /// fr fr 100 Continue - keep sending that data
    Continue = 100,
    /// fr fr 101 Switching Protocols - upgrade time
    SwitchingProtocols = 101,
    /// fr fr 102 Processing - still working on it
    Processing = 102,

    // 2xx Success - things went right periodt
    /// fr fr 200 OK - everything's good bestie
    Ok = 200,
    /// fr fr 201 Created - new resource made successfully
    Created = 201,
    /// fr fr 202 Accepted - request received, processing later
    Accepted = 202,
    /// fr fr 204 No Content - success but nothing to return
    NoContent = 204,
    /// fr fr 206 Partial Content - sending part of the data
    PartialContent = 206,

    // 3xx Redirection - gotta go somewhere else
    /// fr fr 301 Moved Permanently - resource moved for good
    MovedPermanently = 301,
    /// fr fr 302 Found - temporary redirect vibes
    Found = 302,
    /// fr fr 304 Not Modified - use your cached version
    NotModified = 304,
    /// fr fr 307 Temporary Redirect - try this other place
    TemporaryRedirect = 307,
    /// fr fr 308 Permanent Redirect - moved for real this time
    PermanentRedirect = 308,

    // 4xx Client Error - user messed up
    /// fr fr 400 Bad Request - request is sus bestie
    BadRequest = 400,
    /// fr fr 401 Unauthorized - need to log in first
    Unauthorized = 401,
    /// fr fr 403 Forbidden - you can't access this no cap
    Forbidden = 403,
    /// fr fr 404 Not Found - this ain't it chief
    NotFound = 404,
    /// fr fr 405 Method Not Allowed - wrong method for this endpoint
    MethodNotAllowed = 405,
    /// fr fr 406 Not Acceptable - can't provide requested format
    NotAcceptable = 406,
    /// fr fr 408 Request Timeout - took too long bestie
    RequestTimeout = 408,
    /// fr fr 409 Conflict - resource conflict situation
    Conflict = 409,
    /// fr fr 410 Gone - used to exist but not anymore
    Gone = 410,
    /// fr fr 411 Length Required - need Content-Length header
    LengthRequired = 411,
    /// fr fr 413 Payload Too Large - that's too much data
    PayloadTooLarge = 413,
    /// fr fr 415 Unsupported Media Type - can't handle this format
    UnsupportedMediaType = 415,
    /// fr fr 422 Unprocessable Entity - format's right but content's wrong
    UnprocessableEntity = 422,
    /// fr fr 429 Too Many Requests - slow down there bestie
    TooManyRequests = 429,

    // 5xx Server Error - we messed up
    /// fr fr 500 Internal Server Error - something went wrong on our end
    InternalServerError = 500,
    /// fr fr 501 Not Implemented - we don't support that yet
    NotImplemented = 501,
    /// fr fr 502 Bad Gateway - upstream server issues
    BadGateway = 502,
    /// fr fr 503 Service Unavailable - server's down right now
    ServiceUnavailable = 503,
    /// fr fr 504 Gateway Timeout - upstream server too slow
    GatewayTimeout = 504,
    /// fr fr 505 HTTP Version Not Supported - old protocol version
    HttpVersionNotSupported = 505,
}

impl StatusCode {
    /// fr fr Get the numeric status code - what gets sent in response
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    /// fr fr Get status class (1xx, 2xx, etc.) - quick categorization
    pub fn class(&self) -> StatusClass {
        match self.as_u16() {
            100..=199 => StatusClass::Informational,
            200..=299 => StatusClass::Success,
            300..=399 => StatusClass::Redirection,
            400..=499 => StatusClass::ClientError,
            500..=599 => StatusClass::ServerError,
            _ => StatusClass::Unknown,
        }
    }

    /// fr fr Get the standard reason phrase - what humans see
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::Processing => "Processing",
            
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NoContent => "No Content",
            StatusCode::PartialContent => "Partial Content",
            
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::NotModified => "Not Modified",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",
            
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PayloadTooLarge => "Payload Too Large",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::UnprocessableEntity => "Unprocessable Entity",
            StatusCode::TooManyRequests => "Too Many Requests",
            
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
        }
    }

    /// fr fr Get a casual description - Gen Z friendly explanation
    pub fn casual_description(&self) -> &'static str {
        match self {
            StatusCode::Continue => "Keep going bestie, send the rest",
            StatusCode::SwitchingProtocols => "Switching to new protocol vibes",
            StatusCode::Processing => "Still working on it, hold up",
            
            StatusCode::Ok => "Everything's good bestie",
            StatusCode::Created => "New thing created successfully",
            StatusCode::Accepted => "Got your request, processing later",
            StatusCode::NoContent => "Success but nothing to show",
            StatusCode::PartialContent => "Here's part of what you wanted",
            
            StatusCode::MovedPermanently => "This moved permanently, update your bookmarks",
            StatusCode::Found => "Found it but it's somewhere else now",
            StatusCode::NotModified => "You already have the latest version",
            StatusCode::TemporaryRedirect => "Try this other place instead",
            StatusCode::PermanentRedirect => "It moved for real, go here instead",
            
            StatusCode::BadRequest => "Your request is sus, check it again",
            StatusCode::Unauthorized => "You need to log in first bestie",
            StatusCode::Forbidden => "You can't access this no cap",
            StatusCode::NotFound => "This ain't it chief, doesn't exist",
            StatusCode::MethodNotAllowed => "Wrong method for this endpoint",
            StatusCode::NotAcceptable => "Can't give you the format you want",
            StatusCode::RequestTimeout => "You took too long to send data",
            StatusCode::Conflict => "There's a conflict with the current state",
            StatusCode::Gone => "This used to exist but it's gone now",
            StatusCode::LengthRequired => "Need to know how much data you're sending",
            StatusCode::PayloadTooLarge => "That's way too much data bestie",
            StatusCode::UnsupportedMediaType => "Can't handle that file type",
            StatusCode::UnprocessableEntity => "Format's right but content's wrong",
            StatusCode::TooManyRequests => "Slow down there, too many requests",
            
            StatusCode::InternalServerError => "We messed up on our end, sorry",
            StatusCode::NotImplemented => "We don't support that yet",
            StatusCode::BadGateway => "Upstream server is having issues",
            StatusCode::ServiceUnavailable => "Server's down right now",
            StatusCode::GatewayTimeout => "Upstream server is too slow",
            StatusCode::HttpVersionNotSupported => "Your HTTP version is too old",
        }
    }

    /// fr fr Check if status indicates success - 2xx range
    pub fn is_success(&self) -> bool {
        matches!(self.class(), StatusClass::Success)
    }

    /// fr fr Check if status indicates client error - 4xx range
    pub fn is_client_error(&self) -> bool {
        matches!(self.class(), StatusClass::ClientError)
    }

    /// fr fr Check if status indicates server error - 5xx range
    pub fn is_server_error(&self) -> bool {
        matches!(self.class(), StatusClass::ServerError)
    }

    /// fr fr Check if status indicates redirection - 3xx range
    pub fn is_redirection(&self) -> bool {
        matches!(self.class(), StatusClass::Redirection)
    }

    /// fr fr Check if status indicates informational - 1xx range
    pub fn is_informational(&self) -> bool {
        matches!(self.class(), StatusClass::Informational)
    }

    /// fr fr Create status code from u16 - parse from number
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            100 => Some(StatusCode::Continue),
            101 => Some(StatusCode::SwitchingProtocols),
            102 => Some(StatusCode::Processing),
            
            200 => Some(StatusCode::Ok),
            201 => Some(StatusCode::Created),
            202 => Some(StatusCode::Accepted),
            204 => Some(StatusCode::NoContent),
            206 => Some(StatusCode::PartialContent),
            
            301 => Some(StatusCode::MovedPermanently),
            302 => Some(StatusCode::Found),
            304 => Some(StatusCode::NotModified),
            307 => Some(StatusCode::TemporaryRedirect),
            308 => Some(StatusCode::PermanentRedirect),
            
            400 => Some(StatusCode::BadRequest),
            401 => Some(StatusCode::Unauthorized),
            403 => Some(StatusCode::Forbidden),
            404 => Some(StatusCode::NotFound),
            405 => Some(StatusCode::MethodNotAllowed),
            406 => Some(StatusCode::NotAcceptable),
            408 => Some(StatusCode::RequestTimeout),
            409 => Some(StatusCode::Conflict),
            410 => Some(StatusCode::Gone),
            411 => Some(StatusCode::LengthRequired),
            413 => Some(StatusCode::PayloadTooLarge),
            415 => Some(StatusCode::UnsupportedMediaType),
            422 => Some(StatusCode::UnprocessableEntity),
            429 => Some(StatusCode::TooManyRequests),
            
            500 => Some(StatusCode::InternalServerError),
            501 => Some(StatusCode::NotImplemented),
            502 => Some(StatusCode::BadGateway),
            503 => Some(StatusCode::ServiceUnavailable),
            504 => Some(StatusCode::GatewayTimeout),
            505 => Some(StatusCode::HttpVersionNotSupported),
            
            _ => None,
        }
    }
}

/// fr fr Status code classes for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusClass {
    /// fr fr 1xx - informational responses
    Informational,
    /// fr fr 2xx - successful responses
    Success,
    /// fr fr 3xx - redirection messages
    Redirection,
    /// fr fr 4xx - client error responses
    ClientError,
    /// fr fr 5xx - server error responses
    ServerError,
    /// fr fr Unknown status code class
    Unknown,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.as_u16(), self.reason_phrase())
    }
}

impl fmt::Display for StatusClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            StatusClass::Informational => "Informational",
            StatusClass::Success => "Success",
            StatusClass::Redirection => "Redirection",
            StatusClass::ClientError => "Client Error",
            StatusClass::ServerError => "Server Error",
            StatusClass::Unknown => "Unknown",
        };
        write!(f, "{}", desc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_basic() {
        assert_eq!(StatusCode::Ok.as_u16(), 200);
        assert_eq!(StatusCode::NotFound.as_u16(), 404);
        assert_eq!(StatusCode::InternalServerError.as_u16(), 500);
    }

    #[test]
    fn test_status_classes() {
        assert_eq!(StatusCode::Ok.class(), StatusClass::Success);
        assert_eq!(StatusCode::NotFound.class(), StatusClass::ClientError);
        assert_eq!(StatusCode::InternalServerError.class(), StatusClass::ServerError);
        assert_eq!(StatusCode::MovedPermanently.class(), StatusClass::Redirection);
        assert_eq!(StatusCode::Continue.class(), StatusClass::Informational);
    }

    #[test]
    fn test_status_predicates() {
        assert!(StatusCode::Ok.is_success());
        assert!(!StatusCode::NotFound.is_success());
        
        assert!(StatusCode::BadRequest.is_client_error());
        assert!(!StatusCode::Ok.is_client_error());
        
        assert!(StatusCode::InternalServerError.is_server_error());
        assert!(!StatusCode::BadRequest.is_server_error());
    }

    #[test]
    fn test_from_u16() {
        assert_eq!(StatusCode::from_u16(200), Some(StatusCode::Ok));
        assert_eq!(StatusCode::from_u16(404), Some(StatusCode::NotFound));
        assert_eq!(StatusCode::from_u16(999), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(StatusCode::Ok.to_string(), "200 OK");
        assert_eq!(StatusCode::NotFound.to_string(), "404 Not Found");
    }
}
