use crate::error::Error;
/// Error handling for time and date operations
use std::fmt;
use crate::error::CursedError;

/// Result type for time operations
pub type TimeResult<T> = std::result::Result<T, TimeError>;

/// Comprehensive error type for time and date operations
#[derive(Debug, Clone, PartialEq)]
pub enum TimeError {
    /// Invalid date or time values
    InvalidDate {
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        message: String,
    },
    
    /// Invalid time values
    InvalidTime {
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
        message: String,
    },
    
    /// Parsing errors
    ParseError {
        input: String,
        expected_format: String,
        position: Option<usize>,
        message: String,
    },
    
    /// Timezone errors
    TimezoneError {
        timezone: String,
        message: String,
    },
    
    /// Arithmetic overflow/underflow
    ArithmeticOverflow {
        operation: String,
        message: String,
    },
    
    /// Format string errors
    FormatError {
        format_string: String,
        message: String,
    },
    
    /// System time errors
    SystemTimeError {
        message: String,
    },
    
    /// Duration errors
    DurationError {
        message: String,
    },
    
    /// General time error
    General {
        message: String,
    },
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeError::InvalidDate { year, month, day, message } => {
                write!(f, "Invalid date")?;
                if let (Some(y), Some(m), Some(d)) = (year, month, day) {
                    write!(f, " {}-{}-{}", y, m, d)?;
                }
                write!(f, ": {}", message)
            }
            
            TimeError::InvalidTime { hour, minute, second, message } => {
                write!(f, "Invalid time")?;
                if let (Some(h), Some(m), Some(s)) = (hour, minute, second) {
                    write!(f, " {}:{}:{}", h, m, s)?;
                }
                write!(f, ": {}", message)
            }
            
            TimeError::ParseError { input, expected_format, position, message } => {
                write!(f, "Parse error: {}", message)?;
                write!(f, " (input: '{}', expected format: '{}')", input, expected_format)?;
                if let Some(pos) = position {
                    write!(f, " at position {}", pos)?;
                }
                Ok(())
            }
            
            TimeError::TimezoneError { timezone, message } => {
                write!(f, "Timezone error for '{}': {}", timezone, message)
            }
            
            TimeError::ArithmeticOverflow { operation, message } => {
                write!(f, "Arithmetic overflow in {}: {}", operation, message)
            }
            
            TimeError::FormatError { format_string, message } => {
                write!(f, "Format error in '{}': {}", format_string, message)
            }
            
            TimeError::SystemTimeError { message } => {
                write!(f, "System time error: {}", message)
            }
            
            TimeError::DurationError { message } => {
                write!(f, "Duration error: {}", message)
            }
            
            TimeError::General { message } => {
                write!(f, "Time error: {}", message)
            }
        }
    }
}

impl std::error::Error for TimeError {}

impl From<TimeError> for CursedError {
    fn from(err: TimeError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

// Helper functions for creating specific error types
pub fn time_error(message: &str) -> TimeError {
    TimeError::General {
        message: message.to_string(),
    }
}

pub fn parse_error(input: &str, expected_format: &str, message: &str) -> TimeError {
    TimeError::ParseError {
        input: input.to_string(),
        expected_format: expected_format.to_string(),
        position: None,
        message: message.to_string(),
    }
}

pub fn parse_error_at_position(input: &str, expected_format: &str, position: usize, message: &str) -> TimeError {
    TimeError::ParseError {
        input: input.to_string(),
        expected_format: expected_format.to_string(),
        position: Some(position),
        message: message.to_string(),
    }
}

pub fn invalid_date_error(year: i32, month: u32, day: u32, message: &str) -> TimeError {
    TimeError::InvalidDate {
        year: Some(year),
        month: Some(month),
        day: Some(day),
        message: message.to_string(),
    }
}

pub fn invalid_time_error(hour: u32, minute: u32, second: u32, message: &str) -> TimeError {
    TimeError::InvalidTime {
        hour: Some(hour),
        minute: Some(minute),
        second: Some(second),
        message: message.to_string(),
    }
}

pub fn timezone_error(timezone: &str, message: &str) -> TimeError {
    TimeError::TimezoneError {
        timezone: timezone.to_string(),
        message: message.to_string(),
    }
}

pub fn arithmetic_overflow_error(operation: &str, message: &str) -> TimeError {
    TimeError::ArithmeticOverflow {
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

pub fn format_error(format_string: &str, message: &str) -> TimeError {
    TimeError::FormatError {
        format_string: format_string.to_string(),
        message: message.to_string(),
    }
}

pub fn system_time_error(message: &str) -> TimeError {
    TimeError::SystemTimeError {
        message: message.to_string(),
    }
}

pub fn duration_error(message: &str) -> TimeError {
    TimeError::DurationError {
        message: message.to_string(),
    }
}
