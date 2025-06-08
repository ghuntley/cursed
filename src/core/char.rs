use crate::error::Error;
use crate::object::Object;
use tracing::instrument;

/// Character type (sip) methods implementation  
/// Follows Unicode standards for character classification and conversion
pub struct CharMethods;

impl CharMethods {
    /// Check if a character is uppercase
    /// Uses Unicode definition of uppercase
    #[instrument]
    pub fn is_uppercase(ch: char) -> bool {
        ch.is_uppercase()
    }

    /// Check if a character is lowercase
    /// Uses Unicode definition of lowercase
    #[instrument]
    pub fn is_lowercase(ch: char) -> bool {
        ch.is_lowercase()
    }

    /// Check if a character is alphabetic
    /// Uses Unicode definition of alphabetic character
    #[instrument]
    pub fn is_alphabetic(ch: char) -> bool {
        ch.is_alphabetic()
    }

    /// Check if a character is numeric (a digit)
    /// Uses Unicode definition of numeric character
    #[instrument]
    pub fn is_numeric(ch: char) -> bool {
        ch.is_numeric()
    }

    /// Check if a character is whitespace
    /// Uses Unicode definition of whitespace
    #[instrument]
    pub fn is_whitespace(ch: char) -> bool {
        ch.is_whitespace()
    }

    /// Check if a character is a digit (0-9)
    /// ASCII-only digit check for compatibility
    #[instrument]
    pub fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Check if a character is alphanumeric
    /// Uses Unicode definition of alphanumeric
    #[instrument]
    pub fn is_alnum(ch: char) -> bool {
        ch.is_alphanumeric()
    }

    /// Convert a character to uppercase
    /// Handles Unicode case conversion properly
    #[instrument]
    pub fn to_uppercase(ch: char) -> char {
        ch.to_uppercase().next().unwrap_or(ch)
    }

    /// Convert a character to lowercase  
    /// Handles Unicode case conversion properly
    #[instrument]
    pub fn to_lowercase(ch: char) -> char {
        ch.to_lowercase().next().unwrap_or(ch)
    }

    /// Convert a character to string representation
    #[instrument]
    pub fn to_string(ch: char) -> String {
        ch.to_string()
    }

    /// Convert a character to an integer code point
    #[instrument]
    pub fn to_int(ch: char) -> i64 {
        ch as i64
    }

    /// Create a character from an integer code point
    #[instrument]
    pub fn from_int(val: i64) -> Result<char, Error> {
        std::char::from_u32(val as u32)
            .ok_or_else(|| Error::Runtime(format!("Invalid character code point: {}", val)))
    }
}

pub trait CharObject {
    fn is_uppercase(&self) -> Result<Object, Error>;
    fn is_lowercase(&self) -> Result<Object, Error>;
    fn is_alphabetic(&self) -> Result<Object, Error>;
    fn is_numeric(&self) -> Result<Object, Error>;
    fn is_whitespace(&self) -> Result<Object, Error>;
    fn is_digit(&self) -> Result<Object, Error>;
    fn is_alnum(&self) -> Result<Object, Error>;
    fn to_uppercase(&self) -> Result<Object, Error>;
    fn to_lowercase(&self) -> Result<Object, Error>;
    fn to_string(&self) -> Result<Object, Error>;
}

impl CharObject for Object {
    #[instrument]
    fn is_uppercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_uppercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn is_lowercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_lowercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn is_alphabetic(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_alphabetic(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn is_numeric(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_numeric(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn is_whitespace(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_whitespace(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn is_digit(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_digit(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn is_alnum(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_alnum(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn to_uppercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Char(CharMethods::to_uppercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn to_lowercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Char(CharMethods::to_lowercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    #[instrument]
    fn to_string(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::String(CharMethods::to_string(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }
}
