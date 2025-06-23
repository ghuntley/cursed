use crate::web::StatusCode;
/// fr fr HTTP response handling for web_vibez - comprehensive response building
use std::collections::HashMap;
use std::fmt;

use crate::stdlib::packages::web_vibez::{
    status::StatusCode,
    crate::types::{Headers, ContentType, Cookie},
    error::{WebError, WebResult},
};

/// fr fr HTTP response representation - everything for outgoing responses
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// fr fr HTTP status code
    pub status: StatusCode,
    /// fr fr Response headers
    pub headers: Headers,
    /// fr fr Response body content
    pub body: Vec<u8>,
    /// fr fr HTTP version string
    pub version: String,
    /// fr fr Cookies to set in browser
    pub cookies: Vec<Cookie>,
}

impl HttpResponse {
    /// fr fr Create new HTTP response - basic setup
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body: Vec::new(),
            version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
        }
    }

    /// fr fr Create OK response - 200 status
    pub fn ok() -> Self {
        Self::new(StatusCode::Ok)
    }

    /// fr fr Create created response - 201 status
    pub fn created() -> Self {
        Self::new(StatusCode::Created)
    }

    /// fr fr Create no content response - 204 status
    pub fn no_content() -> Self {
        Self::new(StatusCode::NoContent)
    }

    /// fr fr Create bad request response - 400 status
    pub fn bad_request() -> Self {
        Self::new(StatusCode::BadRequest)
    }

    /// fr fr Create unauthorized response - 401 status
    pub fn unauthorized() -> Self {
        Self::new(StatusCode::Unauthorized)
    }

    /// fr fr Create forbidden response - 403 status
    pub fn forbidden() -> Self {
        Self::new(StatusCode::Forbidden)
    }

    /// fr fr Create not found response - 404 status
    pub fn not_found() -> Self {
        Self::new(StatusCode::NotFound)
    }

    /// fr fr Create internal server error response - 500 status
    pub fn internal_server_error() -> Self {
        Self::new(StatusCode::InternalServerError)
    }

    /// fr fr Create method not allowed response - 405 status
    pub fn method_not_allowed(message: &str) -> Self {
        Self::new(StatusCode::MethodNotAllowed)
            .with_text(message)
    }

    /// fr fr Set response status - change status code
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// fr fr Set response body from bytes - raw data
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self.set_content_length();
        self
    }

    /// fr fr Set response body from string - text content
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.body = text.into().into_bytes();
        self.set_content_length();
        self.set_content_type(ContentType::Text);
        self
    }

    /// fr fr Set response body as JSON - structured data
    pub fn with_json<T: serde::Serialize>(mut self, value: &T) -> WebResult<Self> {
        let json_str = serde_json::to_string(value).map_err(|e| {
            WebError::Json {
                message: format!("Failed to serialize JSON: {}", e),
                path: None,
            }
        })?;
        self.body = json_str.into_bytes();
        self.set_content_length();
        self.set_content_type(ContentType::Json);
        Ok(self)
    }

    /// fr fr Set response body as HTML - web page content
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.body = html.into().into_bytes();
        self.set_content_length();
        self.set_content_type(ContentType::Html);
        self
    }

    /// fr fr Add header to response - metadata setting
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// fr fr Add multiple headers - batch setting
    pub fn with_headers(mut self, headers: Headers) -> Self {
        for (name, value) in headers {
            self.headers.insert(name, value);
        }
        self
    }

    /// fr fr Set content type header - what kind of data
    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.set_content_type(content_type);
        self
    }

    /// fr fr Add cookie to response - browser storage
    pub fn with_cookie(mut self, cookie: Cookie) -> Self {
        self.cookies.push(cookie);
        self
    }

    /// fr fr Set redirect response - send elsewhere
    pub fn redirect(location: impl Into<String>, status: Option<StatusCode>) -> Self {
        let status = status.unwrap_or(StatusCode::Found);
        Self::new(status).with_header("location", location.into())
    }

    /// fr fr Set permanent redirect - 301 response
    pub fn permanent_redirect(location: impl Into<String>) -> Self {
        Self::redirect(location, Some(StatusCode::MovedPermanently))
    }

    /// fr fr Set temporary redirect - 302 response
    pub fn temporary_redirect(location: impl Into<String>) -> Self {
        Self::redirect(location, Some(StatusCode::Found))
    }

    /// fr fr Get header value by name - case insensitive lookup
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers
            .iter()
            .find(|(key, _)| key.to_lowercase() == name.to_lowercase())
            .map(|(_, value)| value)
    }

    /// fr fr Get response body as string - text representation
    pub fn body_text(&self) -> WebResult<String> {
        String::from_utf8(self.body.clone()).map_err(|e| {
            WebError::ResponseBuilding {
                message: format!("Failed to convert body to UTF-8: {}", e),
                component: "body".to_string(),
            }
        })
    }

    /// fr fr Get response body as JSON - structured data
    pub fn body_json<T>(&self) -> WebResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = self.body_text()?;
        serde_json::from_str(&text).map_err(|e| {
            WebError::Json {
                message: format!("Failed to parse JSON from response body: {}", e),
                path: None,
            }
        })
    }

    /// fr fr Get content length - size of response
    pub fn content_length(&self) -> usize {
        self.body.len()
    }

    /// fr fr Check if response is successful - 2xx status
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// fr fr Check if response is redirect - 3xx status
    pub fn is_redirect(&self) -> bool {
        self.status.is_redirection()
    }

    /// fr fr Check if response is client error - 4xx status
    pub fn is_client_error(&self) -> bool {
        self.status.is_client_error()
    }

    /// fr fr Check if response is server error - 5xx status
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error()
    }

    /// fr fr Set content length header - internal helper
    fn set_content_length(&mut self) {
        self.headers
            .insert("content-length".to_string(), self.body.len().to_string());
    }

    /// fr fr Set content type header - internal helper
    fn set_content_type(&mut self, content_type: ContentType) {
        self.headers
            .insert("content-type".to_string(), content_type.mime_type().to_string());
    }

    /// fr fr Convert to HTTP response string - wire format
    pub fn to_http_string(&self) -> String {
        let mut response = format!("{} {}\r\n", self.version, self.status);

        // Add headers
        for (name, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", name, value));
        }

        // Add cookies as Set-Cookie headers
        for cookie in &self.cookies {
            response.push_str(&format!("Set-Cookie: {}\r\n", cookie.to_header_value()));
        }

        // End headers section
        response.push_str("\r\n");

        // Add body
        if !self.body.is_empty() {
            response.push_str(&String::from_utf8_lossy(&self.body));
        }

        response
    }
}

/// fr fr Response builder for creating responses - fluent API
#[derive(Debug)]
pub struct ResponseBuilder {
    response: HttpResponse,
}

impl ResponseBuilder {
    /// fr fr Create new response builder - start building
    pub fn new(status: StatusCode) -> Self {
        Self {
            response: HttpResponse::new(status),
        }
    }

    /// fr fr Create OK response builder - 200 status
    pub fn ok() -> Self {
        Self::new(StatusCode::Ok)
    }

    /// fr fr Create created response builder - 201 status
    pub fn created() -> Self {
        Self::new(StatusCode::Created)
    }

    /// fr fr Create bad request response builder - 400 status
    pub fn bad_request() -> Self {
        Self::new(StatusCode::BadRequest)
    }

    /// fr fr Create not found response builder - 404 status
    pub fn not_found() -> Self {
        Self::new(StatusCode::NotFound)
    }

    /// fr fr Create internal server error response builder - 500 status
    pub fn internal_server_error() -> Self {
        Self::new(StatusCode::InternalServerError)
    }

    /// fr fr Set status code - change response status
    pub fn status(mut self, status: StatusCode) -> Self {
        self.response.status = status;
        self
    }

    /// fr fr Add header - metadata setting
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.response.headers.insert(name.into(), value.into());
        self
    }

    /// fr fr Set body from bytes - raw data
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.response.body = body;
        self.response.set_content_length();
        self
    }

    /// fr fr Set body from string - text content
    pub fn text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.response.body = text.into_bytes();
        self.response.set_content_length();
        self.response.set_content_type(ContentType::Text);
        self
    }

    /// fr fr Set JSON body - structured data
    pub fn json<T: serde::Serialize>(mut self, value: &T) -> WebResult<Self> {
        let json_str = serde_json::to_string(value).map_err(|e| {
            WebError::Json {
                message: format!("Failed to serialize JSON: {}", e),
                path: None,
            }
        })?;
        self.response.body = json_str.into_bytes();
        self.response.set_content_length();
        self.response.set_content_type(ContentType::Json);
        Ok(self)
    }

    /// fr fr Set HTML body - web page content
    pub fn html(mut self, html: impl Into<String>) -> Self {
        let html = html.into();
        self.response.body = html.into_bytes();
        self.response.set_content_length();
        self.response.set_content_type(ContentType::Html);
        self
    }

    /// fr fr Set content type - what kind of data
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.response.set_content_type(content_type);
        self
    }

    /// fr fr Add cookie - browser storage
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.response.cookies.push(cookie);
        self
    }

    /// fr fr Build the final response - get result
    pub fn build(self) -> HttpResponse {
        self.response
    }
}

impl Default for HttpResponse {
    /// fr fr Default response is 200 OK with no body
    fn default() -> Self {
        Self::ok()
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({}B)", self.version, self.status, self.body.len())
    }
}

/// fr fr Error response helper functions - common error responses
impl HttpResponse {
    /// fr fr Create error response from WebError - automatic conversion
    pub fn from_error(error: &WebError) -> Self {
        let status = error.status_code();
        let message = error.message();

        let error_body = serde_json::json!({
            "error": {
                "status": status.as_u16(),
                "message": message,
                "category": error.category(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });

        Self::new(status)
            .with_json(&error_body)
            .unwrap_or_else(|_| {
                // Fallback if JSON serialization fails
                Self::new(status).with_text(format!("Error: {}", message))
            })
    }

    /// fr fr Create validation error response - 422 with details
    pub fn validation_error(errors: &HashMap<String, Vec<String>>) -> Self {
        let error_body = serde_json::json!({
            "error": {
                "status": 422,
                "message": "Validation failed",
                "validation_errors": errors,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });

        Self::new(StatusCode::UnprocessableEntity)
            .with_json(&error_body)
            .unwrap_or_else(|_| {
                Self::new(StatusCode::UnprocessableEntity).with_text("Validation failed")
            })
    }

    /// fr fr Create rate limit response - 429 with retry info
    pub fn rate_limited(retry_after_seconds: Option<u64>) -> Self {
        let mut response = Self::new(StatusCode::TooManyRequests);

        if let Some(retry_after) = retry_after_seconds {
            response = response.with_header("retry-after", retry_after.to_string());
        }

        let error_body = serde_json::json!({
            "error": {
                "status": 429,
                "message": "Rate limit exceeded",
                "retry_after": retry_after_seconds,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });

        response.with_json(&error_body).unwrap_or_else(|_| {
            Self::new(StatusCode::TooManyRequests).with_text("Rate limit exceeded")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_creation() {
        let resp = HttpResponse::ok();
        assert_eq!(resp.status, StatusCode::Ok);
        assert!(resp.body.is_empty());
        assert_eq!(resp.version, "HTTP/1.1");
    }

    #[test]
    fn test_response_with_text() {
        let resp = HttpResponse::ok().with_text("Hello world");
        assert_eq!(resp.body_text().unwrap(), "Hello world");
        assert_eq!(resp.header("content-type"), Some(&"text/plain".to_string()));
        assert_eq!(resp.header("content-length"), Some(&"11".to_string()));
    }

    #[test]
    fn test_response_with_json() {
        let data = serde_json::json!({ "message": "hello" });
        let resp = HttpResponse::ok().with_json(&data).unwrap();
        
        assert_eq!(resp.header("content-type"), Some(&"application/json".to_string()));
        assert!(resp.body_text().unwrap().contains("hello"));
    }

    #[test]
    fn test_response_builder() {
        let resp = ResponseBuilder::ok()
            .header("x-custom", "value")
            .text("Hello world")
            .build();

        assert_eq!(resp.status, StatusCode::Ok);
        assert_eq!(resp.header("x-custom"), Some(&"value".to_string()));
        assert_eq!(resp.body_text().unwrap(), "Hello world");
    }

    #[test]
    fn test_redirect_responses() {
        let resp = HttpResponse::permanent_redirect("/new-location");
        assert_eq!(resp.status, StatusCode::MovedPermanently);
        assert_eq!(resp.header("location"), Some(&"/new-location".to_string()));

        let resp = HttpResponse::temporary_redirect("/temp-location");
        assert_eq!(resp.status, StatusCode::Found);
        assert_eq!(resp.header("location"), Some(&"/temp-location".to_string()));
    }

    #[test]
    fn test_response_predicates() {
        assert!(HttpResponse::ok().is_success());
        assert!(!HttpResponse::not_found().is_success());
        
        assert!(HttpResponse::not_found().is_client_error());
        assert!(!HttpResponse::ok().is_client_error());
        
        assert!(HttpResponse::internal_server_error().is_server_error());
        assert!(!HttpResponse::ok().is_server_error());
    }

    #[test]
    fn test_http_string_format() {
        let resp = HttpResponse::ok()
            .with_text("Hello world")
            .with_header("x-custom", "value");

        let http_string = resp.to_http_string();
        assert!(http_string.contains("HTTP/1.1 200 OK"));
        assert!(http_string.contains("content-type: text/plain"));
        assert!(http_string.contains("x-custom: value"));
        assert!(http_string.contains("Hello world"));
    }
}
