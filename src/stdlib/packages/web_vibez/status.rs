pub use crate::web::StatusCode;
use crate::error::CursedError;
// Use the main StatusCode from crate::web instead of defining our own
use std::fmt;

/// Status class helper for grouping status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusClass {
    /// 1xx Informational responses
    /// 2xx Success responses  
    /// 3xx Redirection responses
    /// 4xx Client error responses
    /// 5xx Server error responses
impl StatusClass {
    /// Get the status class for a given status code
    pub fn from_status_code(status: StatusCode) -> Self {
        let code = status.as_u16();
        match code {
            _ => StatusClass::ServerError, // Default to server error for unknown codes
        }
    }

    /// Check if this is an error class (4xx or 5xx)
    pub fn is_error(&self) -> bool {
        matches!(self, StatusClass::ClientError | StatusClass::ServerError)
    /// Check if this is a success class (2xx)
    pub fn is_success(&self) -> bool {
        matches!(self, StatusClass::Success)
    /// Check if this is a redirection class (3xx)
    pub fn is_redirection(&self) -> bool {
        matches!(self, StatusClass::Redirection)
    }
}

impl fmt::Display for StatusClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
