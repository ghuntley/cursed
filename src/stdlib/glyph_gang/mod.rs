use crate::error::CursedError;
/// GlyphGang Unicode package - Unicode character processing with CURSED flair
/// 
/// This module provides comprehensive Unicode support including:
/// - Character classification and property testing
/// - Character case conversion 
/// - Unicode range tables and property support
/// - Enhanced string operations with Unicode awareness
/// - Emoji detection and processing
/// - Bidirectional text support
/// - Script detection and analysis
/// - International text utilities

pub mod core;
pub mod ranges;
pub mod error;
pub mod string_ops;
pub mod emoji;
pub mod bidi;
pub mod script;

// Re-export core functionality
pub use core::{
    // Character classification functions
    is_letter, is_digit, is_number, is_space, is_punct, is_symbol, is_mark,
    is_control, is_graphic, is_print, is_upper, is_lower, is_title,
    
    // Advanced classifications
    is_emoji, is_emoji_modifier, is_emoji_component, is_currency, is_math,
    is_format, is_private_use, is_surrogate, is_ascii,
    
    // Character conversion functions
    to_upper, to_lower, to_title, to_ascii, simple_fold,
    
    // Range and character set functions
    is_in_range, is_in_ranges, is_one_of,
    
    // Character properties
    get_character_name, find_character_by_name, get_block_name,
    get_category, get_properties, get_code_point, get_canonical_equivalent,
};

// Re-export range tables and Unicode properties
pub use ranges::{
    RangeTable, Range16, Range32,
    
    // Predefined range tables
    LETTER, UPPERCASE_LETTER, LOWERCASE_LETTER, TITLECASE_LETTER,
    MODIFIER_LETTER, OTHER_LETTER, NUMBER, DECIMAL_NUMBER, LETTER_NUMBER,
    OTHER_NUMBER, PUNCT, CONNECTOR_PUNCTUATION, DASH_PUNCTUATION,
    OPEN_PUNCTUATION, CLOSE_PUNCTUATION, INITIAL_PUNCTUATION,
    FINAL_PUNCTUATION, OTHER_PUNCTUATION, SYMBOL, MATH_SYMBOL,
    CURRENCY_SYMBOL, MODIFIER_SYMBOL, OTHER_SYMBOL, MARK,
    NON_SPACING_MARK, SPACING_MARK, ENCLOSING_MARK, SPACE,
    CONTROL, FORMAT, SURROGATE, PRIVATE, UNASSIGNED,
    
    // Script tables
    LATIN, GREEK, CYRILLIC, HEBREW, ARABIC, DEVANAGARI, THAI,
    HAN, HIRAGANA, KATAKANA, HANGUL,
    
    // Special categories
    EMOJI, EMOJI_PRESENTATION, EMOJI_MODIFIER, EMOJI_MODIFIER_BASE,
    EMOJI_COMPONENT, EXTENDED_PICTOGRAPHIC,
};

// Re-export string operations
pub use string_ops::{
    to_upper_string, to_lower_string, to_title_string, normalize_string,
    NormalizationForm, NFC, NFD, NFKC, NFKD,
    rune_count, first_rune, last_rune, rune_at, rune_indices,
    string_width, truncate_string, wrap_text, reverse_string,
    word_boundaries, sentence_boundaries, line_break_opportunities,
    fold_string, equal_fold, get_char_width, get_string_width,
    truncate_with_ellipsis,
};

// Re-export emoji support
pub use emoji::{
    is_emoji_sequence, contains_emoji, extract_emojis, replace_emojis,
    get_emoji_name, find_emoji_by_name, emoji_categories, emojis_in_category,
};

// Re-export bidirectional text support
pub use bidi::{
    Direction, LTR, RTL, MIXED,
    get_direction, get_string_direction, is_rtl, is_ltr, is_mixed,
};

// Re-export script detection
pub use script::{
    Script, SCRIPT_UNKNOWN, SCRIPT_LATIN, SCRIPT_GREEK, SCRIPT_CYRILLIC,
    detect_script, get_script_name, get_languages_by_script,
};

// Re-export error types
pub use error::{
    GlyphGangError, GlyphGangResult,
    unicode_error, normalization_error, encoding_error, range_error,
};

/// Initialize the GlyphGang module
pub fn initialize() -> GlyphGangResult<()> {
    // Initialize Unicode data tables
    ranges::initialize_tables()?;
    
    // Initialize emoji data
    emoji::initialize_emoji_data()?;
    
    // Initialize script detection data
    script::initialize_script_data()?;
    
    Ok(())
}

/// Get module version and capabilities
pub fn version() -> &'static str {
    "1.0.0"
}

/// Get supported Unicode version
pub fn unicode_version() -> &'static str {
    "15.0.0"
}

/// Get module capabilities
pub fn capabilities() -> Vec<&'static str> {
    vec![
        "character_classification",
        "case_conversion", 
        "unicode_normalization",
        "emoji_support",
        "bidirectional_text",
        "script_detection",
        "text_segmentation",
        "international_support",
    ]
}
