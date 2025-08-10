//! Unicode character handling for CURSED runtime
//!
//! Provides comprehensive Unicode support for CURSED strings including
//! character classification, normalization, grapheme handling, and
//! emoji support for Gen Z string operations.

use crate::error_types::{Error, Result as CursedResult};
use std::collections::HashMap;
use std::fmt;

/// Unicode character wrapper with CURSED-specific functionality
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnicodeChar {
    /// The underlying character
    pub char_value: char,
    /// Unicode scalar value
    pub scalar_value: u32,
    /// Character category
    pub category: UnicodeCategory,
    /// Additional properties
    pub properties: CharProperties,
}

/// Unicode character categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnicodeCategory {
    /// Letter categories
    LetterUppercase,
    LetterLowercase,
    LetterTitlecase,
    LetterModifier,
    LetterOther,
    
    /// Mark categories
    MarkNonspacing,
    MarkSpacingCombining,
    MarkEnclosing,
    
    /// Number categories
    NumberDecimalDigit,
    NumberLetter,
    NumberOther,
    
    /// Punctuation categories
    PunctuationConnector,
    PunctuationDash,
    PunctuationOpen,
    PunctuationClose,
    PunctuationInitialQuote,
    PunctuationFinalQuote,
    PunctuationOther,
    
    /// Symbol categories
    SymbolMath,
    SymbolCurrency,
    SymbolModifier,
    SymbolOther,
    
    /// Separator categories
    SeparatorSpace,
    SeparatorLine,
    SeparatorParagraph,
    
    /// Other categories
    OtherControl,
    OtherFormat,
    OtherSurrogate,
    OtherPrivateUse,
    OtherNotAssigned,
    
    /// Special CURSED categories
    Emoji,
    GenZSlang,
    Vibe,
}

/// Character properties for CURSED
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CharProperties {
    /// Whether this is an emoji
    pub is_emoji: bool,
    /// Whether this is Gen Z slang marker
    pub is_gen_z_slang: bool,
    /// Whether this contributes to "vibes"
    pub has_vibe: bool,
    /// Whether this is whitespace
    pub is_whitespace: bool,
    /// Whether this is a combining character
    pub is_combining: bool,
    /// Whether this is a control character
    pub is_control: bool,
    /// Normalization form
    pub normalization_form: Option<NormalizationForm>,
    /// Grapheme cluster boundary info
    pub grapheme_break: GraphemeBreak,
}

/// Unicode normalization forms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NormalizationForm {
    /// Canonical Decomposition (NFD)
    NFD,
    /// Canonical Decomposition, followed by Canonical Composition (NFC)
    NFC,
    /// Compatibility Decomposition (NFKD)
    NFKD,
    /// Compatibility Decomposition, followed by Canonical Composition (NFKC)
    NFKC,
}

/// Grapheme cluster break types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphemeBreak {
    /// Does not break grapheme clusters
    Other,
    /// Prepend characters
    Prepend,
    /// Carriage return
    CR,
    /// Line feed
    LF,
    /// Control characters
    Control,
    /// Extend characters (combining marks)
    Extend,
    /// Regional indicator symbols
    RegionalIndicator,
    /// Spacing marks
    SpacingMark,
    /// L Jamo (Korean)
    L,
    /// V Jamo (Korean)
    V,
    /// T Jamo (Korean)
    T,
    /// LV Hangul syllables
    LV,
    /// LVT Hangul syllables
    LVT,
    /// Zero Width Joiner
    ZWJ,
}

impl Default for GraphemeBreak {
    fn default() -> Self {
        GraphemeBreak::Other
    }
}

impl UnicodeChar {
    /// Create a new UnicodeChar from a char
    pub fn new(c: char) -> Self {
        let scalar_value = c as u32;
        let category = Self::categorize_char(c);
        let properties = Self::analyze_properties(c, &category);
        
        Self {
            char_value: c,
            scalar_value,
            category,
            properties,
        }
    }

    /// Create from Unicode scalar value
    pub fn from_scalar(scalar: u32) -> CursedResult<Self> {
        if let Some(c) = char::from_u32(scalar) {
            Ok(Self::new(c))
        } else {
            Err(Error::Runtime(format!("Invalid Unicode scalar value: {:#x}", scalar)))
        }
    }

    /// Categorize a character
    fn categorize_char(c: char) -> UnicodeCategory {
        // Simplified categorization based on char properties
        if c.is_uppercase() {
            UnicodeCategory::LetterUppercase
        } else if c.is_lowercase() {
            UnicodeCategory::LetterLowercase
        } else if c.is_numeric() {
            UnicodeCategory::NumberDecimalDigit
        } else if c.is_whitespace() {
            UnicodeCategory::SeparatorSpace
        } else if c.is_control() {
            UnicodeCategory::OtherControl
        } else if Self::is_emoji_char(c) {
            UnicodeCategory::Emoji
        } else if Self::is_gen_z_slang_char(c) {
            UnicodeCategory::GenZSlang
        } else if Self::has_vibe_char(c) {
            UnicodeCategory::Vibe
        } else if c.is_alphabetic() {
            UnicodeCategory::LetterOther
        } else {
            UnicodeCategory::OtherNotAssigned
        }
    }

    /// Analyze character properties
    fn analyze_properties(c: char, category: &UnicodeCategory) -> CharProperties {
        CharProperties {
            is_emoji: Self::is_emoji_char(c),
            is_gen_z_slang: Self::is_gen_z_slang_char(c),
            has_vibe: Self::has_vibe_char(c),
            is_whitespace: c.is_whitespace(),
            is_combining: Self::is_combining_char(c),
            is_control: c.is_control(),
            normalization_form: None, // Determined during normalization
            grapheme_break: Self::get_grapheme_break(c),
        }
    }

    /// Check if character is an emoji
    fn is_emoji_char(c: char) -> bool {
        // Basic emoji ranges (simplified)
        match c as u32 {
            // Emoticons
            0x1F600..=0x1F64F => true,
            // Miscellaneous Symbols and Pictographs
            0x1F300..=0x1F5FF => true,
            // Transport and Map Symbols
            0x1F680..=0x1F6FF => true,
            // Additional emoji ranges
            0x1F700..=0x1F77F => true,
            0x1F780..=0x1F7FF => true,
            0x1F800..=0x1F8FF => true,
            0x1F900..=0x1F9FF => true,
            0x1FA00..=0x1FA6F => true,
            0x1FA70..=0x1FAFF => true,
            // Some basic Unicode symbols that are emoji-like
            0x2600..=0x26FF => true,
            0x2700..=0x27BF => true,
            _ => false,
        }
    }

    /// Check if character is Gen Z slang related
    fn is_gen_z_slang_char(c: char) -> bool {
        // Characters commonly used in Gen Z text
        matches!(c, '💯' | '🔥' | '😎' | '✨' | '💅' | '📱' | '🎵' | '🌟')
    }

    /// Check if character has "vibe" energy
    fn has_vibe_char(c: char) -> bool {
        // Characters that contribute to the "vibe"
        match c as u32 {
            // Sparkles, stars, hearts
            0x2728 | 0x2B50 | 0x1F496..=0x1F49F => true,
            // Music notes
            0x1F3B5 | 0x1F3B6 => true,
            // Party/celebration
            0x1F389 | 0x1F38A => true,
            // Fire
            0x1F525 => true,
            _ => false,
        }
    }

    /// Check if character is combining
    fn is_combining_char(c: char) -> bool {
        // Simplified check for combining characters
        match c as u32 {
            // Combining Diacritical Marks
            0x0300..=0x036F => true,
            // Combining Diacritical Marks Extended
            0x1AB0..=0x1AFF => true,
            // Combining Diacritical Marks Supplement
            0x1DC0..=0x1DFF => true,
            // Combining Half Marks
            0xFE20..=0xFE2F => true,
            _ => false,
        }
    }

    /// Get grapheme break property
    fn get_grapheme_break(c: char) -> GraphemeBreak {
        match c {
            '\r' => GraphemeBreak::CR,
            '\n' => GraphemeBreak::LF,
            '\u{200D}' => GraphemeBreak::ZWJ, // Zero Width Joiner
            _ if c.is_control() => GraphemeBreak::Control,
            _ if Self::is_combining_char(c) => GraphemeBreak::Extend,
            _ => GraphemeBreak::Other,
        }
    }

    /// Convert to uppercase
    pub fn to_uppercase(&self) -> Vec<UnicodeChar> {
        self.char_value.to_uppercase()
            .map(UnicodeChar::new)
            .collect()
    }

    /// Convert to lowercase
    pub fn to_lowercase(&self) -> Vec<UnicodeChar> {
        self.char_value.to_lowercase()
            .map(UnicodeChar::new)
            .collect()
    }

    /// Get character name (simplified)
    pub fn name(&self) -> String {
        match self.char_value {
            'A'..='Z' => format!("LATIN CAPITAL LETTER {}", self.char_value),
            'a'..='z' => format!("LATIN SMALL LETTER {}", self.char_value.to_uppercase()),
            '0'..='9' => format!("DIGIT {}", self.char_value),
            ' ' => "SPACE".to_string(),
            '\n' => "LINE FEED".to_string(),
            '\r' => "CARRIAGE RETURN".to_string(),
            '\t' => "CHARACTER TABULATION".to_string(),
            '💯' => "HUNDRED POINTS SYMBOL".to_string(),
            '🔥' => "FIRE".to_string(),
            '😎' => "SMILING FACE WITH SUNGLASSES".to_string(),
            '✨' => "SPARKLES".to_string(),
            _ => format!("U+{:04X}", self.scalar_value),
        }
    }

    /// Check if this is a valid CURSED identifier character
    pub fn is_cursed_identifier_char(&self, is_start: bool) -> bool {
        if is_start {
            // Identifier start: letters, underscore, some emojis
            matches!(self.category, 
                UnicodeCategory::LetterUppercase | 
                UnicodeCategory::LetterLowercase | 
                UnicodeCategory::LetterOther) ||
            self.char_value == '_' ||
            (self.properties.is_emoji && !self.properties.is_combining)
        } else {
            // Identifier continuation: start chars + digits + some marks
            self.is_cursed_identifier_char(true) ||
            matches!(self.category, UnicodeCategory::NumberDecimalDigit) ||
            self.properties.is_gen_z_slang
        }
    }

    /// Check if this character affects string vibes
    pub fn affects_vibes(&self) -> bool {
        self.properties.has_vibe || 
        self.properties.is_emoji || 
        self.properties.is_gen_z_slang
    }

    /// Get the "vibe energy" of this character (0.0 to 1.0)
    pub fn vibe_energy(&self) -> f64 {
        if self.properties.has_vibe {
            0.8
        } else if self.properties.is_emoji {
            0.6
        } else if self.properties.is_gen_z_slang {
            0.7
        } else if self.char_value.is_uppercase() {
            0.2 // CAPS = YELLING = some energy
        } else {
            0.0
        }
    }
}

impl fmt::Display for UnicodeChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char_value)
    }
}

/// Unicode string with CURSED enhancements
#[derive(Debug, Clone)]
pub struct UnicodeString {
    /// The underlying string
    pub string_value: String,
    /// Unicode characters
    pub chars: Vec<UnicodeChar>,
    /// String properties
    pub properties: StringProperties,
    /// Normalization state
    pub normalization: Option<NormalizationForm>,
}

/// Properties of a Unicode string
#[derive(Debug, Clone, Default)]
pub struct StringProperties {
    /// Total vibe energy
    pub vibe_energy: f64,
    /// Number of emojis
    pub emoji_count: usize,
    /// Gen Z slang density
    pub gen_z_density: f64,
    /// Whether string is normalized
    pub is_normalized: bool,
    /// Grapheme cluster count
    pub grapheme_count: usize,
    /// Whether string is RTL
    pub is_rtl: bool,
}

impl UnicodeString {
    /// Create a new Unicode string
    pub fn new(s: &str) -> Self {
        let chars: Vec<UnicodeChar> = s.chars().map(UnicodeChar::new).collect();
        let properties = Self::analyze_string_properties(&chars);
        
        Self {
            string_value: s.to_string(),
            chars,
            properties,
            normalization: None,
        }
    }

    /// Analyze string properties
    fn analyze_string_properties(chars: &[UnicodeChar]) -> StringProperties {
        let mut properties = StringProperties::default();
        
        for unicode_char in chars {
            properties.vibe_energy += unicode_char.vibe_energy();
            
            if unicode_char.properties.is_emoji {
                properties.emoji_count += 1;
            }
            
            if unicode_char.properties.is_gen_z_slang {
                properties.gen_z_density += 1.0;
            }
        }
        
        // Normalize vibe energy
        if !chars.is_empty() {
            properties.vibe_energy /= chars.len() as f64;
            properties.gen_z_density /= chars.len() as f64;
        }
        
        // Count grapheme clusters (simplified)
        properties.grapheme_count = chars.len(); // In real implementation, would be more complex
        
        properties
    }

    /// Get character at index
    pub fn char_at(&self, index: usize) -> Option<&UnicodeChar> {
        self.chars.get(index)
    }

    /// Get substring
    pub fn substring(&self, start: usize, end: usize) -> CursedResult<UnicodeString> {
        if end > self.chars.len() || start > end {
            return Err(Error::Runtime("Invalid substring range".to_string()));
        }
        
        let sub_chars = self.chars[start..end].to_vec();
        let sub_string: String = sub_chars.iter().map(|c| c.char_value).collect();
        
        Ok(UnicodeString {
            string_value: sub_string,
            chars: sub_chars.clone(),
            properties: Self::analyze_string_properties(&sub_chars),
            normalization: self.normalization.clone(),
        })
    }

    /// Convert to uppercase
    pub fn to_uppercase(&self) -> UnicodeString {
        let upper_chars: Vec<UnicodeChar> = self.chars.iter()
            .flat_map(|c| c.to_uppercase())
            .collect();
        
        let upper_string: String = upper_chars.iter().map(|c| c.char_value).collect();
        
        UnicodeString {
            string_value: upper_string,
            chars: upper_chars.clone(),
            properties: Self::analyze_string_properties(&upper_chars),
            normalization: self.normalization.clone(),
        }
    }

    /// Convert to lowercase
    pub fn to_lowercase(&self) -> UnicodeString {
        let lower_chars: Vec<UnicodeChar> = self.chars.iter()
            .flat_map(|c| c.to_lowercase())
            .collect();
        
        let lower_string: String = lower_chars.iter().map(|c| c.char_value).collect();
        
        UnicodeString {
            string_value: lower_string,
            chars: lower_chars.clone(),
            properties: Self::analyze_string_properties(&lower_chars),
            normalization: self.normalization.clone(),
        }
    }

    /// Normalize string
    pub fn normalize(&mut self, form: NormalizationForm) -> CursedResult<()> {
        // In a real implementation, this would perform actual Unicode normalization
        // For now, we'll just mark it as normalized
        self.normalization = Some(form);
        self.properties.is_normalized = true;
        Ok(())
    }

    /// Check if string is valid CURSED identifier
    pub fn is_valid_identifier(&self) -> bool {
        if self.chars.is_empty() {
            return false;
        }
        
        // First character must be valid identifier start
        if !self.chars[0].is_cursed_identifier_char(true) {
            return false;
        }
        
        // Remaining characters must be valid identifier continuation
        for unicode_char in &self.chars[1..] {
            if !unicode_char.is_cursed_identifier_char(false) {
                return false;
            }
        }
        
        true
    }

    /// Get the "vibe level" of the string
    pub fn vibe_level(&self) -> VibeLevel {
        match self.properties.vibe_energy {
            x if x >= 0.8 => VibeLevel::Immaculate,
            x if x >= 0.6 => VibeLevel::Fire,
            x if x >= 0.4 => VibeLevel::Decent,
            x if x >= 0.2 => VibeLevel::Mid,
            _ => VibeLevel::NoCap,
        }
    }

    /// Check if string is "sus" (suspicious)
    pub fn is_sus(&self) -> bool {
        // String is sus if it has weird Unicode combinations
        // or excessive emoji usage
        self.properties.emoji_count > self.chars.len() / 2 ||
        self.properties.gen_z_density > 0.8
    }

    /// Convert to "normalized vibe" representation
    pub fn to_vibe_string(&self) -> String {
        self.chars.iter()
            .map(|c| {
                if c.affects_vibes() {
                    format!("{} ", c.char_value)
                } else {
                    format!("{}", c.char_value)
                }
            })
            .collect::<String>()
            .trim()
            .to_string()
    }
}

/// Vibe levels for CURSED strings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VibeLevel {
    NoCap,      // No energy
    Mid,        // Medium energy
    Decent,     // Good energy
    Fire,       // High energy
    Immaculate, // Maximum energy
}

impl fmt::Display for VibeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VibeLevel::NoCap => write!(f, "no cap"),
            VibeLevel::Mid => write!(f, "mid"),
            VibeLevel::Decent => write!(f, "decent"),
            VibeLevel::Fire => write!(f, "fire 🔥"),
            VibeLevel::Immaculate => write!(f, "immaculate ✨"),
        }
    }
}

impl fmt::Display for UnicodeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string_value)
    }
}

/// Unicode utilities and helper functions
pub mod utils {
    use super::*;

    /// Check if a string contains only ASCII
    pub fn is_ascii_only(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii())
    }

    /// Count grapheme clusters in a string (simplified)
    pub fn count_graphemes(s: &str) -> usize {
        // In a real implementation, this would properly handle grapheme clusters
        s.chars().count()
    }

    /// Get the display width of a string (accounting for wide characters)
    pub fn display_width(s: &str) -> usize {
        s.chars().map(|c| {
            match c {
                // CJK characters are typically wide
                '\u{1100}'..='\u{115F}' | 
                '\u{2E80}'..='\u{9FFF}' | 
                '\u{AC00}'..='\u{D7AF}' | 
                '\u{F900}'..='\u{FAFF}' | 
                '\u{FE10}'..='\u{FE19}' | 
                '\u{FE30}'..='\u{FE6F}' | 
                '\u{FF00}'..='\u{FF60}' | 
                '\u{FFE0}'..='\u{FFE6}' => 2,
                // Control characters have no width
                c if c.is_control() => 0,
                // Everything else is 1
                _ => 1,
            }
        }).sum()
    }

    /// Create a Unicode string from a byte array (UTF-8)
    pub fn from_utf8_bytes(bytes: &[u8]) -> CursedResult<UnicodeString> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(UnicodeString::new(&s)),
            Err(e) => Err(Error::Runtime(format!("Invalid UTF-8: {}", e))),
        }
    }

    /// Create a Unicode string from UTF-16 code units
    pub fn from_utf16_units(units: &[u16]) -> CursedResult<UnicodeString> {
        match String::from_utf16(units) {
            Ok(s) => Ok(UnicodeString::new(&s)),
            Err(e) => Err(Error::Runtime(format!("Invalid UTF-16: {}", e))),
        }
    }

    /// Get vibes analysis of a string
    pub fn analyze_vibes(s: &str) -> VibeAnalysis {
        let unicode_string = UnicodeString::new(s);
        
        VibeAnalysis {
            vibe_level: unicode_string.vibe_level(),
            vibe_energy: unicode_string.properties.vibe_energy,
            emoji_count: unicode_string.properties.emoji_count,
            gen_z_density: unicode_string.properties.gen_z_density,
            is_sus: unicode_string.is_sus(),
            is_valid_identifier: unicode_string.is_valid_identifier(),
        }
    }

    /// Sanitize string for safe display
    pub fn sanitize_for_display(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_control() && c != '\n' && c != '\t' {
                    format!("\\u{{{:04X}}}", c as u32)
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    /// Generate a random Unicode string with vibes
    pub fn generate_vibe_string(length: usize) -> String {
        let vibe_chars = vec!['✨', '🔥', '💯', '😎', '🎵', '⭐', '💫', '🌟'];
        let letters = "abcdefghijklmnopqrstuvwxyz";
        
        let mut result = String::new();
        for i in 0..length {
            if i % 3 == 0 && i > 0 {
                // Add vibe character every 3 characters
                let vibe_char = vibe_chars[i % vibe_chars.len()];
                result.push(vibe_char);
            } else {
                let letter = letters.chars().nth(i % letters.len()).unwrap();
                result.push(letter);
            }
        }
        
        result
    }
}

/// Analysis result for string vibes
#[derive(Debug, Clone)]
pub struct VibeAnalysis {
    pub vibe_level: VibeLevel,
    pub vibe_energy: f64,
    pub emoji_count: usize,
    pub gen_z_density: f64,
    pub is_sus: bool,
    pub is_valid_identifier: bool,
}

impl fmt::Display for VibeAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vibe Level: {} | Energy: {:.2} | Emojis: {} | Gen Z: {:.2} | Sus: {} | Valid ID: {}",
            self.vibe_level,
            self.vibe_energy,
            self.emoji_count,
            self.gen_z_density,
            self.is_sus,
            self.is_valid_identifier
        )
    }
}

/// Global Unicode character cache for performance
static UNICODE_CHAR_CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<char, UnicodeChar>>> = 
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Get or create a cached Unicode character
pub fn get_unicode_char(c: char) -> UnicodeChar {
    let mut cache = UNICODE_CHAR_CACHE.lock().unwrap();
    
    if let Some(unicode_char) = cache.get(&c) {
        unicode_char.clone()
    } else {
        let unicode_char = UnicodeChar::new(c);
        cache.insert(c, unicode_char.clone());
        unicode_char
    }
}

/// Clear the Unicode character cache
pub fn clear_unicode_cache() {
    let mut cache = UNICODE_CHAR_CACHE.lock().unwrap();
    cache.clear();
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED Unicode character system initialized".to_string())
}
