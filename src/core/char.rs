/// Character operations for CURSED language
use crate::object::Object;
use crate::error::CursedError;

/// Trait for character methods
pub trait CharMethods {
    fn is_uppercase(&self) -> crate::error::Result<()>;
    fn is_lowercase(&self) -> crate::error::Result<()>;
    fn is_alphabetic(&self) -> crate::error::Result<()>;
    fn is_numeric(&self) -> crate::error::Result<()>;
    fn is_whitespace(&self) -> crate::error::Result<()>;
    fn to_uppercase(&self) -> crate::error::Result<()>;
    fn to_lowercase(&self) -> crate::error::Result<()>;
    fn to_string(&self) -> crate::error::Result<()>;
/// Trait for character object operations
pub trait CharObject {
    fn from_char(c: char) -> Object;
    fn to_char(&self) -> crate::error::Result<()>;
impl CharMethods for Object {
    fn is_uppercase(&self) -> crate::error::Result<()> {
        match self {
        }
    }

    fn is_lowercase(&self) -> crate::error::Result<()> {
        match self {
        }
    }

    fn is_alphabetic(&self) -> crate::error::Result<()> {
        match self {
        }
    }

    fn is_numeric(&self) -> crate::error::Result<()> {
        match self {
        }
    }

    fn is_whitespace(&self) -> crate::error::Result<()> {
        match self {
        }
    }

    fn to_uppercase(&self) -> crate::error::Result<()> {
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
        }
    }

    fn to_lowercase(&self) -> crate::error::Result<()> {
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
        }
    }

    fn to_string(&self) -> crate::error::Result<()> {
        match self {
        }
    }
impl CharObject for Object {
    fn from_char(c: char) -> Object {
        Object::Char(c)
    fn to_char(&self) -> crate::error::Result<()> {
        match self {
        }
    }
}
