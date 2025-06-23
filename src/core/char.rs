/// Character operations for CURSED language
use crate::object::Object;
use crate::error::Error;

/// Trait for character methods
pub trait CharMethods {
    fn is_uppercase(&self) -> Result<(), Error>;
    fn is_lowercase(&self) -> Result<(), Error>;
    fn is_alphabetic(&self) -> Result<(), Error>;
    fn is_numeric(&self) -> Result<(), Error>;
    fn is_whitespace(&self) -> Result<(), Error>;
    fn to_uppercase(&self) -> Result<(), Error>;
    fn to_lowercase(&self) -> Result<(), Error>;
    fn to_string(&self) -> Result<(), Error>;
}

/// Trait for character object operations
pub trait CharObject {
    fn from_char(c: char) -> Object;
    fn to_char(&self) -> Result<(), Error>;
}

impl CharMethods for Object {
    fn is_uppercase(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(Object::Boolean(c.is_uppercase())),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn is_lowercase(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(Object::Boolean(c.is_lowercase())),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn is_alphabetic(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(Object::Boolean(c.is_alphabetic())),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn is_numeric(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(Object::Boolean(c.is_numeric())),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn is_whitespace(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(Object::Boolean(c.is_whitespace())),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn to_uppercase(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => {
                let upper_chars: Vec<char> = c.to_uppercase().collect();
                if upper_chars.len() == 1 {
                    Ok(Object::Char(upper_chars[0]))
                } else {
                    // For multi-character uppercase results, return a string
                    Ok(Object::String(upper_chars.into_iter().collect()))
                }
            }
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn to_lowercase(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => {
                let lower_chars: Vec<char> = c.to_lowercase().collect();
                if lower_chars.len() == 1 {
                    Ok(Object::Char(lower_chars[0]))
                } else {
                    // For multi-character lowercase results, return a string
                    Ok(Object::String(lower_chars.into_iter().collect()))
                }
            }
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }

    fn to_string(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(Object::String(c.to_string())),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }
}

impl CharObject for Object {
    fn from_char(c: char) -> Object {
        Object::Char(c)
    }

    fn to_char(&self) -> Result<(), Error> {
        match self {
            Object::Char(c) => Ok(*c),
            _ => Err(Error::Runtime("Expected character".to_string())),
        }
    }
}
