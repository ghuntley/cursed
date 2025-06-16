/// Error handling for GlyphGang Unicode operations
use std::fmt;

/// Result type for GlyphGang operations
pub type GlyphGangResult<T> = Result<T, GlyphGangError>;

/// Comprehensive error types for Unicode operations
#[derive(Debug, Clone, PartialEq)]
pub enum GlyphGangError {
    /// Unicode processing error
    UnicodeError {
        message: String,
        code_point: Option<u32>,
        position: Option<usize>,
    },
    
    /// Normalization error
    NormalizationError {
        message: String,
        form: String,
        input: String,
    },
    
    /// Text encoding error
    EncodingError {
        message: String,
        encoding: String,
        position: Option<usize>,
    },
    
    /// Range table error
    RangeError {
        message: String,
        range_name: Option<String>,
    },
    
    /// Character name lookup error
    NameLookupError {
        message: String,
        character: Option<char>,
        name: Option<String>,
    },
    
    /// Script detection error
    ScriptDetectionError {
        message: String,
        text: String,
    },
    
    /// Emoji processing error
    EmojiError {
        message: String,
        emoji: Option<String>,
    },
    
    /// Bidirectional text processing error
    BidiError {
        message: String,
        text: String,
    },
    
    /// General Unicode operation error
    General(String),
}

impl fmt::Display for GlyphGangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlyphGangError::UnicodeError { message, code_point, position } => {
                write!(f, "Unicode error: {}", message)?;
                if let Some(cp) = code_point {
                    write!(f, " (code point: U+{:04X})", cp)?;
                }
                if let Some(pos) = position {
                    write!(f, " (position: {})", pos)?;
                }
                Ok(())
            }
            GlyphGangError::NormalizationError { message, form, input } => {
                write!(f, "Normalization error: {} (form: {}, input: {:?})", message, form, input)
            }
            GlyphGangError::EncodingError { message, encoding, position } => {
                write!(f, "Encoding error: {} (encoding: {})", message, encoding)?;
                if let Some(pos) = position {
                    write!(f, " (position: {})", pos)?;
                }
                Ok(())
            }
            GlyphGangError::RangeError { message, range_name } => {
                write!(f, "Range error: {}", message)?;
                if let Some(name) = range_name {
                    write!(f, " (range: {})", name)?;
                }
                Ok(())
            }
            GlyphGangError::NameLookupError { message, character, name } => {
                write!(f, "Name lookup error: {}", message)?;
                if let Some(ch) = character {
                    write!(f, " (character: '{}')", ch)?;
                }
                if let Some(n) = name {
                    write!(f, " (name: {})", n)?;
                }
                Ok(())
            }
            GlyphGangError::ScriptDetectionError { message, text } => {
                write!(f, "Script detection error: {} (text: {:?})", message, text)
            }
            GlyphGangError::EmojiError { message, emoji } => {
                write!(f, "Emoji error: {}", message)?;
                if let Some(e) = emoji {
                    write!(f, " (emoji: {})", e)?;
                }
                Ok(())
            }
            GlyphGangError::BidiError { message, text } => {
                write!(f, "Bidirectional text error: {} (text: {:?})", message, text)
            }
            GlyphGangError::General(message) => {
                write!(f, "GlyphGang error: {}", message)
            }
        }
    }
}

impl std::error::Error for GlyphGangError {}

/// Helper function to create a Unicode error
pub fn unicode_error(message: &str) -> GlyphGangError {
    GlyphGangError::UnicodeError {
        message: message.to_string(),
        code_point: None,
        position: None,
    }
}

/// Helper function to create a Unicode error with code point
pub fn unicode_error_with_code_point(message: &str, code_point: u32) -> GlyphGangError {
    GlyphGangError::UnicodeError {
        message: message.to_string(),
        code_point: Some(code_point),
        position: None,
    }
}

/// Helper function to create a normalization error
pub fn normalization_error(message: &str, form: &str, input: &str) -> GlyphGangError {
    GlyphGangError::NormalizationError {
        message: message.to_string(),
        form: form.to_string(),
        input: input.to_string(),
    }
}

/// Helper function to create an encoding error
pub fn encoding_error(message: &str, encoding: &str) -> GlyphGangError {
    GlyphGangError::EncodingError {
        message: message.to_string(),
        encoding: encoding.to_string(),
        position: None,
    }
}

/// Helper function to create a range error
pub fn range_error(message: &str) -> GlyphGangError {
    GlyphGangError::RangeError {
        message: message.to_string(),
        range_name: None,
    }
}

/// Helper function to create a name lookup error
pub fn name_lookup_error(message: &str) -> GlyphGangError {
    GlyphGangError::NameLookupError {
        message: message.to_string(),
        character: None,
        name: None,
    }
}

/// Helper function to create a script detection error
pub fn script_detection_error(message: &str, text: &str) -> GlyphGangError {
    GlyphGangError::ScriptDetectionError {
        message: message.to_string(),
        text: text.to_string(),
    }
}

/// Helper function to create an emoji error
pub fn emoji_error(message: &str) -> GlyphGangError {
    GlyphGangError::EmojiError {
        message: message.to_string(),
        emoji: None,
    }
}

/// Helper function to create a bidirectional text error
pub fn bidi_error(message: &str, text: &str) -> GlyphGangError {
    GlyphGangError::BidiError {
        message: message.to_string(),
        text: text.to_string(),
    }
}

/// Helper function to create a general error
pub fn general_error(message: &str) -> GlyphGangError {
    GlyphGangError::General(message.to_string())
}
