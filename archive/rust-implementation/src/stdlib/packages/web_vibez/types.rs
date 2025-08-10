use std::collections::HashMap;

/// HTTP headers
pub type Headers = HashMap<String, String>;

/// Query parameters
pub type QueryParams = HashMap<String, String>;

/// Form data
pub type FormData = HashMap<String, String>;

/// JSON wrapper
pub struct Json<T>(pub T);

/// Content type enumeration
#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
    Html,
    Text,
    Binary,
    Form,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Html => "text/html",
            ContentType::Text => "text/plain",
            ContentType::Binary => "application/octet-stream",
            ContentType::Form => "application/x-www-form-urlencoded",
        }
    }
}

/// Request body
pub enum RequestBody {
    Empty,
    Text(String),
    Binary(Vec<u8>),
    Json(String), // JSON as string for now
    Form(FormData),
}

/// Cookie
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

/// SameSite attribute
#[derive(Debug, Clone)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}
