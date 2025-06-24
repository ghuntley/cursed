pub use crate::web::StatusCode;
use crate::error::Error;
// Use the main StatusCode from crate::web instead of defining our own
use std::fmt;

/// Status class helper for grouping status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusClass {
    /// 1xx Informational responses
    Informational,
    /// 2xx Success responses  
    Success,
    /// 3xx Redirection responses
    Redirection,
    /// 4xx Client error responses
    ClientError,
    /// 5xx Server error responses
    ServerError,
}

impl StatusClass {
    /// Get the status class for a given status code
    pub fn from_status_code(status: StatusCode) -> Self {
        let code = status.as_u16();
        match code {
            100..=199 => StatusClass::Informational,
            200..=299 => StatusClass::Success,
            300..=399 => StatusClass::Redirection,
            400..=499 => StatusClass::ClientError,
            500..=599 => StatusClass::ServerError,
            _ => StatusClass::ServerError, // Default to server error for unknown codes
        }
    }

    /// Check if this is an error class (4xx or 5xx)
    pub fn is_error(&self) -> bool {
        matches!(self, StatusClass::ClientError | StatusClass::ServerError)
    }

    /// Check if this is a success class (2xx)
    pub fn is_success(&self) -> bool {
        matches!(self, StatusClass::Success)
    }

    /// Check if this is a redirection class (3xx)
    pub fn is_redirection(&self) -> bool {
        matches!(self, StatusClass::Redirection)
    }
}

impl fmt::Display for StatusClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusClass::Informational => write!(f, "Informational"),
            StatusClass::Success => write!(f, "Success"),
            StatusClass::Redirection => write!(f, "Redirection"),
            StatusClass::ClientError => write!(f, "Client Error"),
            StatusClass::ServerError => write!(f, "Server Error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_class_classification() {
        assert_eq!(StatusClass::from_status_code(StatusCode::OK), StatusClass::Success);
        assert_eq!(StatusClass::from_status_code(StatusCode::NotFound), StatusClass::ClientError);
        assert_eq!(StatusClass::from_status_code(StatusCode::InternalServerError), StatusClass::ServerError);
        assert_eq!(StatusClass::from_status_code(StatusCode::MovedPermanently), StatusClass::Redirection);
    }

    #[test]
    fn test_status_class_helpers() {
        assert!(StatusClass::Success.is_success());
        assert!(!StatusClass::Success.is_error());
        assert!(StatusClass::ClientError.is_error());
        assert!(StatusClass::Redirection.is_redirection());
    }
}
