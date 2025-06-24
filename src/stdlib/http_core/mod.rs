//! CURSED HTTP Core - Comprehensive HTTP Request and Response Processing
//!
//! This package provides comprehensive HTTP request and response processing capabilities
//! for web applications built with the CURSED programming language.

use crate::web::StatusCode;

pub mod request;
pub mod response;
pub mod headers;
pub mod cookies;
pub mod url;
pub mod content_type;
pub mod form_data;
pub mod error;
pub mod validation;
pub mod integration;

// Re-export main types for convenience
pub use request::{Request, Method, RequestBuilder};
pub use response::{Response, StatusCode, ResponseBuilder};
pub use headers::{Headers, HeaderMap};
pub use cookies::{Cookie, CookieJar};
pub use url::{Url, QueryParams};
pub use content_type::{ContentType, MimeType};
pub use form_data::{FormData, FormField, MultipartData};
pub use error::{HttpError, HttpResult};
pub use validation::{HttpValidator, ValidationRules};
pub use integration::{HttpContext, RequestParser, ResponseWriter, SecureResponseBuilder};

/// HTTP processing version and capabilities
pub const VERSION: &str = "1.0.0";
pub const SUPPORTED_HTTP_VERSIONS: &[&str] = &["HTTP/1.0", "HTTP/1.1"];
pub const MAX_HEADER_SIZE: usize = 8192;
pub const MAX_REQUEST_SIZE: usize = 100 * 1024 * 1024; // 100MB
pub const DEFAULT_READ_TIMEOUT: u64 = 30; // seconds
