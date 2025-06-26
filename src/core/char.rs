// Character manipulation functions and objects
use crate::stdlib::string::CursedString;
use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct CharObject {
    pub value: char,
}

impl CharObject {
    pub fn new(c: char) -> Self {
        Self { value: c }
    }

    pub fn to_string(&self) -> CursedString {
        CursedString::from(self.value.to_string())
    }

    pub fn is_ascii(&self) -> bool {
        self.value.is_ascii()
    }

    pub fn is_numeric(&self) -> bool {
        self.value.is_numeric()
    }

    pub fn is_alphabetic(&self) -> bool {
        self.value.is_alphabetic()
    }

    pub fn is_whitespace(&self) -> bool {
        self.value.is_whitespace()
    }

    pub fn to_lowercase(&self) -> char {
        self.value.to_lowercase().next().unwrap_or(self.value)
    }

    pub fn to_uppercase(&self) -> char {
        self.value.to_uppercase().next().unwrap_or(self.value)
    }
}

pub trait CharMethods {
    fn to_string_repr(self) -> CursedString;
    fn is_ascii_char(self) -> bool;
    fn is_numeric_char(self) -> bool;
    fn is_alphabetic_char(self) -> bool;
    fn is_whitespace_char(self) -> bool;
    fn to_lowercase_char(self) -> char;
    fn to_uppercase_char(self) -> char;
}

impl CharMethods for char {
    fn to_string_repr(self) -> CursedString {
        CursedString::from(self.to_string())
    }

    fn is_ascii_char(self) -> bool {
        self.is_ascii()
    }

    fn is_numeric_char(self) -> bool {
        self.is_numeric()
    }

    fn is_alphabetic_char(self) -> bool {
        self.is_alphabetic()
    }

    fn is_whitespace_char(self) -> bool {
        self.is_whitespace()
    }

    fn to_lowercase_char(self) -> char {
        self.to_lowercase().next().unwrap_or(self)
    }

    fn to_uppercase_char(self) -> char {
        self.to_uppercase().next().unwrap_or(self)
    }
}

pub fn to_string(c: char) -> CursedString {
    CursedString::from(c.to_string())
}

pub fn is_ascii(c: char) -> bool {
    c.is_ascii()
}

pub fn is_numeric(c: char) -> bool {
    c.is_numeric()
}

pub fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic()
}

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn to_lowercase(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

pub fn to_uppercase(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced features enabled".to_string())
}
