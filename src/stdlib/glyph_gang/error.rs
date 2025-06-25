use crate::error::CursedError;
/// CursedError handling for GlyphGang Unicode operations
use std::fmt;

/// Result type for GlyphGang operations
pub type GlyphGangResult<T> = std::result::Result<T, GlyphGangError>;

/// Comprehensive error types for Unicode operations
#[derive(Debug, Clone, PartialEq)]
pub enum GlyphGangError {
    /// Unicode processing error
    UnicodeError {
    
    /// Normalization error
    NormalizationError {
    
    /// Text encoding error
    EncodingError {
    
    /// Range table error
    RangeError {
    
    /// Character name lookup error
    NameLookupError {
    
    /// Script detection error
    ScriptDetectionError {
    
    /// Emoji processing error
    EmojiError {
    
    /// Bidirectional text processing error
    BidiError {
    
    /// General Unicode operation error
// impl fmt::Display for GlyphGangError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             GlyphGangError::UnicodeError { message, code_point, position } => {
//                 write!(f, "Unicode error: {}", message)?;
//                 if let Some(cp) = code_point {
//                     write!(f, " (code point: U+{:04X})", cp)?;
//                 }
//                 if let Some(pos) = position {
//                     write!(f, " (position: {})", pos)?;
//                 }
//                 Ok(())
//             }
//             GlyphGangError::NormalizationError { message, form, input } => {
//                 write!(f, "Normalization error: {} (form: {}, input: {:?})", message, form, input)
//             }
//             GlyphGangError::EncodingError { message, encoding, position } => {
//                 write!(f, "Encoding error: {} (encoding: {})", message, encoding)?;
//                 if let Some(pos) = position {
//                     write!(f, " (position: {})", pos)?;
//                 }
//                 Ok(())
//             }
//             GlyphGangError::RangeError { message, range_name } => {
//                 write!(f, "Range error: {}", message)?;
//                 if let Some(name) = range_name {
//                     write!(f, " (range: {})", name)?;
//                 }
//                 Ok(())
//             }
//             GlyphGangError::NameLookupError { message, character, name } => {
//                 write!(f, "Name lookup error: {}", message)?;
//                 if let Some(ch) = character {
//                     write!(f, " (character: '{}')", ch)?;
//                 }
//                 if let Some(n) = name {
//                     write!(f, " (name: {})", n)?;
//                 }
//                 Ok(())
//             }
//             GlyphGangError::ScriptDetectionError { message, text } => {
//                 write!(f, "Script detection error: {} (text: {:?})", message, text)
//             }
//             GlyphGangError::EmojiError { message, emoji } => {
//                 write!(f, "Emoji error: {}", message)?;
//                 if let Some(e) = emoji {
//                     write!(f, " (emoji: {})", e)?;
//                 }
//                 Ok(())
//             }
//             GlyphGangError::BidiError { message, text } => {
//                 write!(f, "Bidirectional text error: {} (text: {:?})", message, text)
//             }
//             GlyphGangError::General(message) => {
//                 write!(f, "GlyphGang error: {}", message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for GlyphGangError {}
// 
/// Helper function to create a Unicode error
pub fn unicode_error(message: &str) -> GlyphGangError {
    GlyphGangError::UnicodeError {
    }
}

/// Helper function to create a Unicode error with code point
pub fn unicode_error_with_code_point(message: &str, code_point: u32) -> GlyphGangError {
    GlyphGangError::UnicodeError {
    }
}

/// Helper function to create a normalization error
pub fn normalization_error(message: &str, form: &str, input: &str) -> GlyphGangError {
    GlyphGangError::NormalizationError {
    }
}

/// Helper function to create an encoding error
pub fn encoding_error(message: &str, encoding: &str) -> GlyphGangError {
    GlyphGangError::EncodingError {
    }
}

/// Helper function to create a range error
pub fn range_error(message: &str) -> GlyphGangError {
    GlyphGangError::RangeError {
    }
}

/// Helper function to create a name lookup error
pub fn name_lookup_error(message: &str) -> GlyphGangError {
    GlyphGangError::NameLookupError {
    }
}

/// Helper function to create a script detection error
pub fn script_detection_error(message: &str, text: &str) -> GlyphGangError {
    GlyphGangError::ScriptDetectionError {
    }
}

/// Helper function to create an emoji error
pub fn emoji_error(message: &str) -> GlyphGangError {
    GlyphGangError::EmojiError {
    }
}

/// Helper function to create a bidirectional text error
pub fn bidi_error(message: &str, text: &str) -> GlyphGangError {
    GlyphGangError::BidiError {
    }
}

/// Helper function to create a general error
pub fn general_error(message: &str) -> GlyphGangError {
    GlyphGangError::General(message.to_string())
}
