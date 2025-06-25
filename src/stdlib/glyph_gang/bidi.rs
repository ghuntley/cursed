/// Bidirectional text support for Unicode
// use crate::stdlib::glyph_gang::error::{GlyphGangResult, bidi_error};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Text direction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Left-to-Right
    LTR,
    /// Right-to-Left
    RTL,
    /// Mixed directionality
    Mixed,
}

/// Constants for direction
pub const LTR: Direction = Direction::LTR;
pub const RTL: Direction = Direction::RTL;
pub const MIXED: Direction = Direction::Mixed;

/// Bidirectional character types (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidiType {
    /// Left-to-Right
    L,
    /// Right-to-Left
    R,
    /// Arabic Letter
    AL,
    /// European Number
    EN,
    /// European Separator
    ES,
    /// European Terminator
    ET,
    /// Arabic Number
    AN,
    /// Common Separator
    CS,
    /// Nonspacing Mark
    NSM,
    /// Boundary Neutral
    BN,
    /// Paragraph Separator
    B,
    /// Segment Separator
    S,
    /// Whitespace
    WS,
    /// Other Neutrals
    ON,
}

/// Character bidirectional type database (simplified)
static BIDI_TYPES: Lazy<HashMap<u32, BidiType>> = Lazy::new(|| {
    let mut types = HashMap::new();
    
    // ASCII Latin characters (L)
    for i in 0x0041..=0x005A { types.insert(i, BidiType::L); } // A-Z
    for i in 0x0061..=0x007A { types.insert(i, BidiType::L); } // a-z
    
    // ASCII digits (EN)
    for i in 0x0030..=0x0039 { types.insert(i, BidiType::EN); } // 0-9
    
    // ASCII punctuation and symbols (various)
    types.insert(0x0021, BidiType::ON); // !
    types.insert(0x0022, BidiType::ON); // "
    types.insert(0x0023, BidiType::ET); // #
    types.insert(0x0024, BidiType::ET); // $
    types.insert(0x0025, BidiType::ET); // %
    types.insert(0x0026, BidiType::ON); // &
    types.insert(0x0027, BidiType::ON); // '
    types.insert(0x0028, BidiType::ON); // (
    types.insert(0x0029, BidiType::ON); // )
    types.insert(0x002A, BidiType::ON); // *
    types.insert(0x002B, BidiType::ES); // +
    types.insert(0x002C, BidiType::CS); // ,
    types.insert(0x002D, BidiType::ES); // -
    types.insert(0x002E, BidiType::CS); // .
    types.insert(0x002F, BidiType::CS); // /
    types.insert(0x003A, BidiType::CS); // :
    types.insert(0x003B, BidiType::ON); // ;
    types.insert(0x003C, BidiType::ON); // <
    types.insert(0x003D, BidiType::ON); // =
    types.insert(0x003E, BidiType::ON); // >
    types.insert(0x003F, BidiType::ON); // ?
    types.insert(0x0040, BidiType::ON); // @
    types.insert(0x005B, BidiType::ON); // [
    types.insert(0x005C, BidiType::ON); // \
    types.insert(0x005D, BidiType::ON); // ]
    types.insert(0x005E, BidiType::ON); // ^
    types.insert(0x005F, BidiType::ON); // _
    types.insert(0x0060, BidiType::ON); // `
    types.insert(0x007B, BidiType::ON); // {
    types.insert(0x007C, BidiType::ON); // |
    types.insert(0x007D, BidiType::ON); // }
    types.insert(0x007E, BidiType::ON); // ~
    
    // Whitespace characters (WS)
    types.insert(0x0020, BidiType::WS); // Space
    types.insert(0x0009, BidiType::S);  // Tab (segment separator)
    types.insert(0x000A, BidiType::B);  // LF (paragraph separator)
    types.insert(0x000B, BidiType::S);  // VT (segment separator)
    types.insert(0x000C, BidiType::WS); // FF
    types.insert(0x000D, BidiType::B);  // CR (paragraph separator)
    types.insert(0x001C, BidiType::B);  // FS (paragraph separator)
    types.insert(0x001D, BidiType::B);  // GS (paragraph separator)
    types.insert(0x001E, BidiType::B);  // RS (paragraph separator)
    types.insert(0x001F, BidiType::S);  // US (segment separator)
    
    // Control characters (BN)
    for i in 0x0000..=0x0008 { types.insert(i, BidiType::BN); }
    for i in 0x000E..=0x001B { types.insert(i, BidiType::BN); }
    for i in 0x007F..=0x009F { types.insert(i, BidiType::BN); }
    
    // Extended Latin (L)
    for i in 0x00C0..=0x00D6 { types.insert(i, BidiType::L); }
    for i in 0x00D8..=0x00F6 { types.insert(i, BidiType::L); }
    for i in 0x00F8..=0x02AF { types.insert(i, BidiType::L); }
    
    // Hebrew (R)
    for i in 0x05D0..=0x05EA { types.insert(i, BidiType::R); }
    for i in 0x05F0..=0x05F4 { types.insert(i, BidiType::R); }
    types.insert(0x05BE, BidiType::R); // Hebrew punctuation maqaf
    types.insert(0x05C0, BidiType::R); // Hebrew punctuation paseq
    types.insert(0x05C3, BidiType::R); // Hebrew punctuation sof pasuq
    types.insert(0x05C6, BidiType::R); // Hebrew punctuation nun hafukha
    for i in 0x05C1..=0x05C2 { types.insert(i, BidiType::NSM); } // Hebrew points
    for i in 0x05C4..=0x05C5 { types.insert(i, BidiType::NSM); } // Hebrew points
    for i in 0x05C7..=0x05C7 { types.insert(i, BidiType::NSM); } // Hebrew points
    
    // Arabic (AL)
    for i in 0x0600..=0x06FF { 
        match i {
            // Arabic digits
            0x0660..=0x0669 => { types.insert(i, BidiType::AN); }
            // Arabic letters and most other characters
            _ => { types.insert(i, BidiType::AL); }
        }
    }
    
    // Arabic supplement (AL)
    for i in 0x0750..=0x077F { types.insert(i, BidiType::AL); }
    
    // Arabic extended (AL)
    for i in 0x08A0..=0x08FF { types.insert(i, BidiType::AL); }
    
    // Arabic presentation forms (AL)
    for i in 0xFB50..=0xFDFF { types.insert(i, BidiType::AL); }
    for i in 0xFE70..=0xFEFF { types.insert(i, BidiType::AL); }
    
    // Greek (L)
    for i in 0x0370..=0x03FF { types.insert(i, BidiType::L); }
    
    // Cyrillic (L)
    for i in 0x0400..=0x04FF { types.insert(i, BidiType::L); }
    
    // Armenian (L)
    for i in 0x0530..=0x058F { types.insert(i, BidiType::L); }
    
    // Georgian (L)
    for i in 0x10A0..=0x10FF { types.insert(i, BidiType::L); }
    for i in 0x2D00..=0x2D2F { types.insert(i, BidiType::L); }
    
    // Latin Extended (L)
    for i in 0x0100..=0x017F { types.insert(i, BidiType::L); }
    for i in 0x0180..=0x024F { types.insert(i, BidiType::L); }
    for i in 0x1E00..=0x1EFF { types.insert(i, BidiType::L); }
    
    // CJK (L)
    for i in 0x4E00..=0x9FFF { types.insert(i, BidiType::L); }
    for i in 0x3400..=0x4DBF { types.insert(i, BidiType::L); }
    
    // Hiragana and Katakana (L)
    for i in 0x3040..=0x309F { types.insert(i, BidiType::L); }
    for i in 0x30A0..=0x30FF { types.insert(i, BidiType::L); }
    
    // Hangul (L)
    for i in 0x1100..=0x11FF { types.insert(i, BidiType::L); }
    for i in 0x3130..=0x318F { types.insert(i, BidiType::L); }
    for i in 0xAC00..=0xD7AF { types.insert(i, BidiType::L); }
    
    // Thai (L)
    for i in 0x0E00..=0x0E7F { types.insert(i, BidiType::L); }
    
    // Common punctuation (ON)
    for i in 0x2000..=0x206F { 
        match i {
            // Directional marks have special types
            0x200E => { types.insert(i, BidiType::L); }  // LRM
            0x200F => { types.insert(i, BidiType::R); }  // RLM
            0x202A => { types.insert(i, BidiType::L); }  // LRE
            0x202B => { types.insert(i, BidiType::R); }  // RLE
            0x202C => { types.insert(i, BidiType::BN); } // PDF
            0x202D => { types.insert(i, BidiType::L); }  // LRO
            0x202E => { types.insert(i, BidiType::R); }  // RLO
            0x2061..=0x2064 => { types.insert(i, BidiType::BN); } // Invisible operators
            // Most punctuation is ON
            _ => { types.insert(i, BidiType::ON); }
        }
    }
    
    // Mathematical operators (ON)
    for i in 0x2200..=0x22FF { types.insert(i, BidiType::ON); }
    
    // Miscellaneous symbols (ON)
    for i in 0x2600..=0x26FF { types.insert(i, BidiType::ON); }
    
    // Currency symbols (ET)
    for i in 0x20A0..=0x20CF { types.insert(i, BidiType::ET); }
    
    // Non-spacing marks (NSM)
    for i in 0x0300..=0x036F { types.insert(i, BidiType::NSM); } // Combining diacritical marks
    for i in 0x1AB0..=0x1AFF { types.insert(i, BidiType::NSM); } // Combining diacritical marks extended
    for i in 0x1DC0..=0x1DFF { types.insert(i, BidiType::NSM); } // Combining diacritical marks supplement
    for i in 0x20D0..=0x20FF { types.insert(i, BidiType::NSM); } // Combining diacritical marks for symbols
    for i in 0xFE20..=0xFE2F { types.insert(i, BidiType::NSM); } // Combining half marks
    
    types
});

/// Get the bidirectional type of a character
fn get_bidi_type(ch: char) -> BidiType {
    let code_point = ch as u32;
    
    if let Some(&bidi_type) = BIDI_TYPES.get(&code_point) {
        bidi_type
    } else {
        // Default classification for unknown characters
        if code_point < 0x80 {
            // ASCII defaults
            if ch.is_ascii_alphabetic() {
                BidiType::L
            } else if ch.is_ascii_digit() {
                BidiType::EN
            } else if ch.is_ascii_whitespace() {
                BidiType::WS
            } else {
                BidiType::ON
            }
        } else if code_point >= 0x4E00 && code_point <= 0x9FFF {
            // CJK Unified Ideographs
            BidiType::L
        } else if code_point >= 0x3400 && code_point <= 0x4DBF {
            // CJK Extension A
            BidiType::L
        } else if code_point >= 0xAC00 && code_point <= 0xD7AF {
            // Hangul Syllables
            BidiType::L
        } else {
            // Default for unknown characters
            BidiType::L
        }
    }
}

/// Get the bidirectional direction of a single character
pub fn get_direction(ch: char) -> Direction {
    match get_bidi_type(ch) {
        BidiType::L | BidiType::EN | BidiType::ES | BidiType::ET => Direction::LTR,
        BidiType::R | BidiType::AL | BidiType::AN => Direction::RTL,
        _ => Direction::LTR, // Default to LTR for neutral characters
    }
}

/// Get the overall bidirectional direction of a string
pub fn get_string_direction(s: &str) -> Direction {
    if s.is_empty() {
        return Direction::LTR;
    }
    
    let mut ltr_count = 0;
    let mut rtl_count = 0;
    
    for ch in s.chars() {
        match get_bidi_type(ch) {
            BidiType::L | BidiType::EN | BidiType::ES | BidiType::ET => {
                ltr_count += 1;
            }
            BidiType::R | BidiType::AL | BidiType::AN => {
                rtl_count += 1;
            }
            _ => {
                // Neutral characters don't affect directionality
            }
        }
    }
    
    if ltr_count > 0 && rtl_count > 0 {
        Direction::Mixed
    } else if rtl_count > 0 {
        Direction::RTL
    } else {
        Direction::LTR
    }
}

/// Check if a string is primarily right-to-left
pub fn is_rtl(s: &str) -> bool {
    get_string_direction(s) == Direction::RTL
}

/// Check if a string is primarily left-to-right
pub fn is_ltr(s: &str) -> bool {
    get_string_direction(s) == Direction::LTR
}

/// Check if a string has mixed directionality
pub fn is_mixed(s: &str) -> bool {
    get_string_direction(s) == Direction::Mixed
}

/// Simple bidirectional text reordering (simplified implementation)
pub fn reorder_text(s: &str) -> GlyphGangResult<String> {
    // This is a very simplified implementation
    // A full implementation would need the complete Unicode Bidirectional Algorithm
    
    let direction = get_string_direction(s);
    
    match direction {
        Direction::LTR => Ok(s.to_string()),
        Direction::RTL => {
            // Simple reversal for RTL text (not correct for mixed content)
            Ok(s.chars().rev().collect())
        }
        Direction::Mixed => {
            // For mixed text, this would require complex reordering
            // For now, just return as-is
            Ok(s.to_string())
        }
    }
}

/// Apply logical order to visual order conversion (simplified)
pub fn logical_to_visual(s: &str) -> GlyphGangResult<String> {
    // In a full implementation, this would apply the Unicode Bidirectional Algorithm
    // For now, just handle simple cases
    reorder_text(s)
}

/// Apply visual order to logical order conversion (simplified)
pub fn visual_to_logical(s: &str) -> GlyphGangResult<String> {
    // In a full implementation, this would reverse the bidirectional algorithm
    // For now, just handle simple cases
    reorder_text(s)
}

/// Get detailed bidirectional analysis of a string
pub fn analyze_bidi(s: &str) -> HashMap<String, String> {
    let mut analysis = HashMap::new();
    
    analysis.insert("length".to_string(), s.chars().count().to_string());
    analysis.insert("direction".to_string(), format!("{:?}", get_string_direction(s)));
    analysis.insert("is_ltr".to_string(), is_ltr(s).to_string());
    analysis.insert("is_rtl".to_string(), is_rtl(s).to_string());
    analysis.insert("is_mixed".to_string(), is_mixed(s).to_string());
    
    // Count characters by type
    let mut l_count = 0;
    let mut r_count = 0;
    let mut al_count = 0;
    let mut en_count = 0;
    let mut an_count = 0;
    let mut neutral_count = 0;
    
    for ch in s.chars() {
        match get_bidi_type(ch) {
            BidiType::L => l_count += 1,
            BidiType::R => r_count += 1,
            BidiType::AL => al_count += 1,
            BidiType::EN => en_count += 1,
            BidiType::AN => an_count += 1,
            _ => neutral_count += 1,
        }
    }
    
    analysis.insert("l_chars".to_string(), l_count.to_string());
    analysis.insert("r_chars".to_string(), r_count.to_string());
    analysis.insert("al_chars".to_string(), al_count.to_string());
    analysis.insert("en_chars".to_string(), en_count.to_string());
    analysis.insert("an_chars".to_string(), an_count.to_string());
    analysis.insert("neutral_chars".to_string(), neutral_count.to_string());
    
    analysis
}

