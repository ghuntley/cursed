use crate::error::CursedError;
use thiserror::Error;

/// Errors that can occur during embed operations
#[derive(Error, Debug, Clone)]
pub enum EmbedError {
    #[error("File not found: {file}")]
    FileNotFound { file: String },
    
    #[error("Invalid file format: {file} - {reason}")]
    InvalidFormat { file: String, reason: String },
    
    #[error("Compression error: {reason}")]
    CompressionError { reason: String },
    
    #[error("Decompression error: {reason}")]
    DecompressionError { reason: String },
    
    #[error("Template parsing error: {reason}")]
    TemplateParsingError { reason: String },
    
    #[error("Image loading error: {reason}")]
    ImageLoadingError { reason: String },
    
    #[error("JSON parsing error: {reason}")]
    JsonParsingError { reason: String },
    
    #[error("YAML parsing error: {reason}")]
    YamlParsingError { reason: String },
    
    #[error("TOML parsing error: {reason}")]
    TomlParsingError { reason: String },
    
    #[error("Config parsing error: {reason}")]
    ConfigParsingError { reason: String },
    
    #[error("Cache error: {reason}")]
    CacheError { reason: String },
    
    #[error("MIME type detection error: {reason}")]
    MimeTypeError { reason: String },
    
    #[error("I/O error: {reason}")]
    IoError { reason: String },
    
    #[error("UTF-8 conversion error: {reason}")]
    Utf8Error { reason: String },
    
    #[error("Resource limit exceeded: {limit}")]
    ResourceLimitExceeded { limit: String },
    
    #[error("Invalid pattern: {pattern}")]
    InvalidPattern { pattern: String },
    
    #[error("General embed error: {message}")]
    General { message: String },
}

impl From<EmbedError> for CursedError {
    fn from(err: EmbedError) -> Self {
        CursedError::Runtime { 
            message: err.to_string(),
            location: None 
        }
    }
}

impl From<std::io::Error> for EmbedError {
    fn from(err: std::io::Error) -> Self {
        EmbedError::IoError { 
            reason: err.to_string() 
        }
    }
}

impl From<std::string::FromUtf8Error> for EmbedError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        EmbedError::Utf8Error { 
            reason: err.to_string() 
        }
    }
}

impl From<serde_json::Error> for EmbedError {
    fn from(err: serde_json::Error) -> Self {
        EmbedError::JsonParsingError { 
            reason: err.to_string() 
        }
    }
}

impl From<serde_yaml::Error> for EmbedError {
    fn from(err: serde_yaml::Error) -> Self {
        EmbedError::YamlParsingError { 
            reason: err.to_string() 
        }
    }
}

impl From<toml::de::Error> for EmbedError {
    fn from(err: toml::de::Error) -> Self {
        EmbedError::TomlParsingError { 
            reason: err.to_string() 
        }
    }
}

/// Type alias for Results in embed operations
pub type EmbedResult<T> = Result<T, EmbedError>;

/// Helper functions for creating specific embed errors
pub fn file_not_found(file: &str) -> EmbedError {
    EmbedError::FileNotFound { 
        file: file.to_string() 
    }
}

pub fn invalid_format(file: &str, reason: &str) -> EmbedError {
    EmbedError::InvalidFormat { 
        file: file.to_string(), 
        reason: reason.to_string() 
    }
}

pub fn compression_error(reason: &str) -> EmbedError {
    EmbedError::CompressionError { 
        reason: reason.to_string() 
    }
}

pub fn decompression_error(reason: &str) -> EmbedError {
    EmbedError::DecompressionError { 
        reason: reason.to_string() 
    }
}

pub fn template_parsing_error(reason: &str) -> EmbedError {
    EmbedError::TemplateParsingError { 
        reason: reason.to_string() 
    }
}

pub fn image_loading_error(reason: &str) -> EmbedError {
    EmbedError::ImageLoadingError { 
        reason: reason.to_string() 
    }
}

pub fn cache_error(reason: &str) -> EmbedError {
    EmbedError::CacheError { 
        reason: reason.to_string() 
    }
}

pub fn mime_type_error(reason: &str) -> EmbedError {
    EmbedError::MimeTypeError { 
        reason: reason.to_string() 
    }
}

pub fn config_parsing_error(reason: &str) -> EmbedError {
    EmbedError::ConfigParsingError { 
        reason: reason.to_string() 
    }
}

pub fn resource_limit_exceeded(limit: &str) -> EmbedError {
    EmbedError::ResourceLimitExceeded { 
        limit: limit.to_string() 
    }
}

pub fn invalid_pattern(pattern: &str) -> EmbedError {
    EmbedError::InvalidPattern { 
        pattern: pattern.to_string() 
    }
}

pub fn general_error(message: &str) -> EmbedError {
    EmbedError::General { 
        message: message.to_string() 
    }
}
