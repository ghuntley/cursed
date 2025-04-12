use crate::error::Error;
use crate::object::Object;

/// Character type (sip) methods implementation
pub struct CharMethods;

impl CharMethods {
    /// Check if a character is uppercase
    pub fn is_uppercase(ch: char) -> bool {
        ch.is_uppercase()
    }

    /// Check if a character is lowercase
    pub fn is_lowercase(ch: char) -> bool {
        ch.is_lowercase()
    }

    /// Check if a character is a digit
    pub fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Check if a character is alphabetic
    pub fn is_alpha(ch: char) -> bool {
        ch.is_alphabetic()
    }

    /// Check if a character is alphanumeric
    pub fn is_alnum(ch: char) -> bool {
        ch.is_alphanumeric()
    }

    /// Convert a character to uppercase
    pub fn to_uppercase(ch: char) -> char {
        ch.to_uppercase().next().unwrap_or(ch)
    }

    /// Convert a character to lowercase
    pub fn to_lowercase(ch: char) -> char {
        ch.to_lowercase().next().unwrap_or(ch)
    }

    /// Convert a character to an integer
    pub fn to_int(ch: char) -> i64 {
        ch as i64
    }

    /// Create a character from an integer code point
    pub fn from_int(val: i64) -> Result<char, Error> {
        std::char::from_u32(val as u32)
            .ok_or_else(|| Error::Runtime(format!("Invalid character code point: {}", val)))
    }
}

pub trait CharObject {
    fn is_uppercase(&self) -> Result<Object, Error>;
    fn is_lowercase(&self) -> Result<Object, Error>;
    fn is_digit(&self) -> Result<Object, Error>;
    fn is_alpha(&self) -> Result<Object, Error>;
    fn is_alnum(&self) -> Result<Object, Error>;
    fn to_uppercase(&self) -> Result<Object, Error>;
    fn to_lowercase(&self) -> Result<Object, Error>;
}

impl CharObject for Object {
    fn is_uppercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_uppercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    fn is_lowercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_lowercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    fn is_digit(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_digit(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    fn is_alpha(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_alpha(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    fn is_alnum(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Boolean(CharMethods::is_alnum(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    fn to_uppercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Char(CharMethods::to_uppercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }

    fn to_lowercase(&self) -> Result<Object, Error> {
        match self {
            Object::Char(ch) => Ok(Object::Char(CharMethods::to_lowercase(*ch))),
            _ => Err(Error::Runtime(format!(
                "Expected character, got {}",
                self.type_name()
            ))),
        }
    }
}
