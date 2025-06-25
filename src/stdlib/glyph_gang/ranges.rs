/// Unicode range tables and property definitions
// use crate::stdlib::glyph_gang::error::{GlyphGangResult, range_error};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Unicode range table for 16-bit code points
#[derive(Debug, Clone, PartialEq)]
pub struct Range16 {
    pub lo: u16,
    pub hi: u16,
    pub stride: u16,
}

/// Unicode range table for 32-bit code points
#[derive(Debug, Clone, PartialEq)]
pub struct Range32 {
    pub lo: u32,
    pub hi: u32,
    pub stride: u32,
}

/// Unicode range table containing ranges for character classification
#[derive(Debug, Clone)]
pub struct RangeTable {
    pub r16: Vec<Range16>,
    pub r32: Vec<Range32>,
    pub latin_offset: usize,
}

impl RangeTable {
    /// Create a new empty range table
    pub fn new() -> Self {
        RangeTable {
            r16: Vec::new(),
            r32: Vec::new(),
            latin_offset: 0,
        }
    }
    
    /// Add a 16-bit range to the table
    pub fn add_range16(&mut self, lo: u16, hi: u16, stride: u16) {
        self.r16.push(Range16 { lo, hi, stride });
    }
    
    /// Add a 32-bit range to the table
    pub fn add_range32(&mut self, lo: u32, hi: u32, stride: u32) {
        self.r32.push(Range32 { lo, hi, stride });
    }
    
    /// Check if a character is in this range table
    pub fn contains(&self, ch: char) -> bool {
        let code_point = ch as u32;
        
        // Check 16-bit ranges first (more common)
        if code_point <= 0xFFFF {
            let cp16 = code_point as u16;
            for range in &self.r16 {
                if cp16 >= range.lo && cp16 <= range.hi {
                    if range.stride == 1 {
                        return true;
                    }
                    return (cp16 - range.lo) % range.stride == 0;
                }
            }
        }
        
        // Check 32-bit ranges
        for range in &self.r32 {
            if code_point >= range.lo && code_point <= range.hi {
                if range.stride == 1 {
                    return true;
                }
                return (code_point - range.lo) % range.stride == 0;
            }
        }
        
        false
    }
}

impl Default for RangeTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined Unicode range tables (simplified for initial implementation)
/// In a production implementation, these would be loaded from Unicode data files

// Letter categories
pub static LETTER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    // Basic Latin letters
    table.add_range16(0x0041, 0x005A, 1); // A-Z
    table.add_range16(0x0061, 0x007A, 1); // a-z
    // Extended Latin
    table.add_range16(0x00C0, 0x00D6, 1);
    table.add_range16(0x00D8, 0x00F6, 1);
    table.add_range16(0x00F8, 0x01FF, 1);
    // Additional letter ranges would be added here
    table
});

pub static UPPERCASE_LETTER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0041, 0x005A, 1); // A-Z
    table.add_range16(0x00C0, 0x00D6, 1);
    table.add_range16(0x00D8, 0x00DE, 1);
    table
});

pub static LOWERCASE_LETTER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0061, 0x007A, 1); // a-z
    table.add_range16(0x00DF, 0x00F6, 1);
    table.add_range16(0x00F8, 0x00FF, 1);
    table
});

pub static TITLECASE_LETTER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x01C5, 0x01C5, 1);
    table.add_range16(0x01C8, 0x01C8, 1);
    table.add_range16(0x01CB, 0x01CB, 1);
    table
});

pub static MODIFIER_LETTER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x02B0, 0x02C1, 1);
    table.add_range16(0x02C6, 0x02D1, 1);
    table.add_range16(0x02E0, 0x02E4, 1);
    table
});

pub static OTHER_LETTER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x00AA, 0x00AA, 1);
    table.add_range16(0x00BA, 0x00BA, 1);
    table.add_range16(0x01BB, 0x01BB, 1);
    table
});

// Number categories
pub static NUMBER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0030, 0x0039, 1); // 0-9
    table.add_range16(0x00B2, 0x00B3, 1); // ², ³
    table.add_range16(0x00B9, 0x00B9, 1); // ¹
    table
});

pub static DECIMAL_NUMBER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0030, 0x0039, 1); // 0-9
    table.add_range16(0x0660, 0x0669, 1); // Arabic-Indic digits
    table.add_range16(0x06F0, 0x06F9, 1); // Extended Arabic-Indic digits
    table
});

pub static LETTER_NUMBER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x16EE, 0x16F0, 1); // Runic numbers
    table.add_range16(0x2160, 0x2182, 1); // Roman numerals
    table
});

pub static OTHER_NUMBER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x00B2, 0x00B3, 1); // Superscript 2, 3
    table.add_range16(0x00B9, 0x00B9, 1); // Superscript 1
    table.add_range16(0x00BC, 0x00BE, 1); // Fractions
    table
});

// Punctuation categories
pub static PUNCT: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0021, 0x0023, 1); // !, ", #
    table.add_range16(0x0025, 0x002A, 1); // %, &, ', (, ), *
    table.add_range16(0x002C, 0x002F, 1); // ,, -, ., /
    table.add_range16(0x003A, 0x003B, 1); // :, ;
    table.add_range16(0x003F, 0x0040, 1); // ?, @
    table.add_range16(0x005B, 0x005D, 1); // [, \, ]
    table.add_range16(0x005F, 0x005F, 1); // _
    table.add_range16(0x007B, 0x007B, 1); // {
    table.add_range16(0x007D, 0x007D, 1); // }
    table
});

pub static CONNECTOR_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x005F, 0x005F, 1); // _
    table.add_range16(0x203F, 0x2040, 1); // Undertie, character tie
    table
});

pub static DASH_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x002D, 0x002D, 1); // Hyphen-minus
    table.add_range16(0x058A, 0x058A, 1); // Armenian hyphen
    table.add_range16(0x05BE, 0x05BE, 1); // Hebrew punctuation maqaf
    table
});

pub static OPEN_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0028, 0x0028, 1); // (
    table.add_range16(0x005B, 0x005B, 1); // [
    table.add_range16(0x007B, 0x007B, 1); // {
    table
});

pub static CLOSE_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0029, 0x0029, 1); // )
    table.add_range16(0x005D, 0x005D, 1); // ]
    table.add_range16(0x007D, 0x007D, 1); // }
    table
});

pub static INITIAL_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x00A1, 0x00A1, 1); // ¡
    table.add_range16(0x00BF, 0x00BF, 1); // ¿
    table.add_range16(0x2018, 0x2018, 1); // '
    table
});

pub static FINAL_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x00BB, 0x00BB, 1); // »
    table.add_range16(0x2019, 0x2019, 1); // '
    table.add_range16(0x201D, 0x201D, 1); // "
    table
});

pub static OTHER_PUNCTUATION: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0021, 0x0023, 1); // !, ", #
    table.add_range16(0x0025, 0x0027, 1); // %, &, '
    table.add_range16(0x002A, 0x002A, 1); // *
    table.add_range16(0x002C, 0x002C, 1); // ,
    table.add_range16(0x002E, 0x002F, 1); // ., /
    table.add_range16(0x003A, 0x003B, 1); // :, ;
    table.add_range16(0x003F, 0x0040, 1); // ?, @
    table.add_range16(0x005C, 0x005C, 1); // \
    table
});

// Symbol categories
pub static SYMBOL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0024, 0x0024, 1); // $
    table.add_range16(0x002B, 0x002B, 1); // +
    table.add_range16(0x003C, 0x003E, 1); // <, =, >
    table.add_range16(0x005E, 0x005E, 1); // ^
    table.add_range16(0x0060, 0x0060, 1); // `
    table.add_range16(0x007C, 0x007C, 1); // |
    table.add_range16(0x007E, 0x007E, 1); // ~
    table
});

pub static MATH_SYMBOL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x002B, 0x002B, 1); // +
    table.add_range16(0x003C, 0x003E, 1); // <, =, >
    table.add_range16(0x007C, 0x007C, 1); // |
    table.add_range16(0x007E, 0x007E, 1); // ~
    table.add_range16(0x00AC, 0x00AC, 1); // ¬
    table.add_range16(0x00B1, 0x00B1, 1); // ±
    table
});

pub static CURRENCY_SYMBOL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0024, 0x0024, 1); // $
    table.add_range16(0x00A2, 0x00A5, 1); // ¢, £, ¤, ¥
    table.add_range16(0x058F, 0x058F, 1); // ֏
    table.add_range16(0x060B, 0x060B, 1); // ؋
    table
});

pub static MODIFIER_SYMBOL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x005E, 0x005E, 1); // ^
    table.add_range16(0x0060, 0x0060, 1); // `
    table.add_range16(0x00A8, 0x00A8, 1); // ¨
    table.add_range16(0x00AF, 0x00AF, 1); // ¯
    table
});

pub static OTHER_SYMBOL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x00A6, 0x00A6, 1); // ¦
    table.add_range16(0x00A9, 0x00A9, 1); // ©
    table.add_range16(0x00AE, 0x00AE, 1); // ®
    table.add_range16(0x00B0, 0x00B0, 1); // °
    table
});

// Mark categories
pub static MARK: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0300, 0x036F, 1); // Combining diacritical marks
    table.add_range16(0x1AB0, 0x1AFF, 1); // Combining diacritical marks extended
    table
});

pub static NON_SPACING_MARK: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0300, 0x036F, 1); // Combining diacritical marks
    table
});

pub static SPACING_MARK: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0903, 0x0903, 1); // Devanagari sign visarga
    table.add_range16(0x093B, 0x093B, 1); // Devanagari vowel sign oe
    table
});

pub static ENCLOSING_MARK: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0488, 0x0489, 1); // Combining cyrillic letters
    table.add_range16(0x20DD, 0x20E0, 1); // Combining enclosing marks
    table
});

// Other categories
pub static SPACE: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0020, 0x0020, 1); // Space
    table.add_range16(0x00A0, 0x00A0, 1); // Non-breaking space
    table.add_range16(0x1680, 0x1680, 1); // Ogham space mark
    table.add_range16(0x2000, 0x200A, 1); // En quad to hair space
    table.add_range16(0x202F, 0x202F, 1); // Narrow no-break space
    table.add_range16(0x205F, 0x205F, 1); // Medium mathematical space
    table.add_range16(0x3000, 0x3000, 1); // Ideographic space
    table
});

pub static CONTROL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0000, 0x001F, 1); // C0 controls
    table.add_range16(0x007F, 0x009F, 1); // DEL and C1 controls
    table
});

pub static FORMAT: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x00AD, 0x00AD, 1); // Soft hyphen
    table.add_range16(0x0600, 0x0605, 1); // Arabic number sign etc.
    table.add_range16(0x061C, 0x061C, 1); // Arabic letter mark
    table.add_range16(0x06DD, 0x06DD, 1); // Arabic end of ayah
    table
});

pub static SURROGATE: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0xD800, 0xDFFF, 1); // High and low surrogate area
    table
});

pub static PRIVATE: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0xE000, 0xF8FF, 1); // Private use area
    table.add_range32(0xF0000, 0xFFFFD, 1); // Supplementary private use area A
    table.add_range32(0x100000, 0x10FFFD, 1); // Supplementary private use area B
    table
});

pub static UNASSIGNED: Lazy<RangeTable> = Lazy::new(|| {
    // This would contain ranges of unassigned code points
    // For simplicity, creating an empty table
    RangeTable::new()
});

// Script tables (simplified)
pub static LATIN: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0041, 0x005A, 1); // A-Z
    table.add_range16(0x0061, 0x007A, 1); // a-z
    table.add_range16(0x00C0, 0x00D6, 1);
    table.add_range16(0x00D8, 0x00F6, 1);
    table.add_range16(0x00F8, 0x02AF, 1);
    table
});

pub static GREEK: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0370, 0x0373, 1);
    table.add_range16(0x0375, 0x0377, 1);
    table.add_range16(0x037A, 0x037D, 1);
    table.add_range16(0x037F, 0x037F, 1);
    table.add_range16(0x0384, 0x0384, 1);
    table.add_range16(0x0386, 0x0386, 1);
    table.add_range16(0x0388, 0x038A, 1);
    table.add_range16(0x038C, 0x038C, 1);
    table.add_range16(0x038E, 0x03A1, 1);
    table.add_range16(0x03A3, 0x03FF, 1);
    table
});

pub static CYRILLIC: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0400, 0x0484, 1);
    table.add_range16(0x0487, 0x052F, 1);
    table.add_range16(0x2DE0, 0x2DFF, 1);
    table.add_range16(0xA640, 0xA69F, 1);
    table
});

pub static HEBREW: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0590, 0x05FF, 1);
    table.add_range16(0xFB1D, 0xFB4F, 1);
    table
});

pub static ARABIC: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0600, 0x06FF, 1);
    table.add_range16(0x0750, 0x077F, 1);
    table.add_range16(0x08A0, 0x08FF, 1);
    table.add_range16(0xFB50, 0xFDFF, 1);
    table.add_range16(0xFE70, 0xFEFF, 1);
    table
});

pub static DEVANAGARI: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0900, 0x097F, 1);
    table.add_range16(0xA8E0, 0xA8FF, 1);
    table
});

pub static THAI: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0E00, 0x0E7F, 1);
    table
});

pub static HAN: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x2E80, 0x2EFF, 1); // CJK Radicals Supplement
    table.add_range16(0x2F00, 0x2FDF, 1); // Kangxi Radicals
    table.add_range16(0x3400, 0x4DBF, 1); // CJK Extension A
    table.add_range16(0x4E00, 0x9FFF, 1); // CJK Unified Ideographs
    table.add_range32(0x20000, 0x2A6DF, 1); // CJK Extension B
    table.add_range32(0x2A700, 0x2B73F, 1); // CJK Extension C
    table.add_range32(0x2B740, 0x2B81F, 1); // CJK Extension D
    table.add_range32(0x2B820, 0x2CEAF, 1); // CJK Extension E
    table.add_range32(0x2CEB0, 0x2EBEF, 1); // CJK Extension F
    table
});

pub static HIRAGANA: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x3041, 0x3096, 1);
    table.add_range16(0x309D, 0x309F, 1);
    table.add_range32(0x1B001, 0x1B11E, 1); // Kana Supplement
    table.add_range32(0x1F200, 0x1F200, 1); // Square Hiragana Hoka
    table
});

pub static KATAKANA: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x30A1, 0x30FA, 1);
    table.add_range16(0x30FD, 0x30FF, 1);
    table.add_range16(0x31F0, 0x31FF, 1); // Katakana Phonetic Extensions
    table.add_range16(0x32D0, 0x32FE, 1); // Circled Katakana
    table.add_range16(0x3300, 0x3357, 1); // CJK Compatibility
    table.add_range32(0x1B000, 0x1B000, 1); // Katakana Letter Archaic E
    table
});

pub static HANGUL: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x1100, 0x11FF, 1); // Hangul Jamo
    table.add_range16(0x3130, 0x318F, 1); // Hangul Compatibility Jamo
    table.add_range16(0xA960, 0xA97F, 1); // Hangul Jamo Extended-A
    table.add_range16(0xAC00, 0xD7AF, 1); // Hangul Syllables
    table.add_range16(0xD7B0, 0xD7FF, 1); // Hangul Jamo Extended-B
    table
});

// Special categories (simplified)
pub static EMOJI: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x231A, 0x231B, 1); // Watch, hourglass
    table.add_range16(0x23E9, 0x23EC, 1); // Play buttons
    table.add_range16(0x23F0, 0x23F0, 1); // Alarm clock
    table.add_range16(0x23F3, 0x23F3, 1); // Hourglass flowing sand
    table.add_range16(0x25FD, 0x25FE, 1); // Small squares
    table.add_range16(0x2614, 0x2615, 1); // Umbrella, hot beverage
    table.add_range16(0x2648, 0x2653, 1); // Zodiac signs
    table.add_range16(0x267F, 0x267F, 1); // Wheelchair symbol
    table.add_range16(0x2693, 0x2693, 1); // Anchor
    table.add_range16(0x26A1, 0x26A1, 1); // High voltage
    table.add_range16(0x26AA, 0x26AB, 1); // Circles
    table.add_range16(0x26BD, 0x26BE, 1); // Soccer ball, baseball
    table.add_range16(0x26C4, 0x26C5, 1); // Snowman, sun
    table.add_range16(0x26CE, 0x26CE, 1); // Ophiuchus
    table.add_range16(0x26D4, 0x26D4, 1); // No entry
    table.add_range16(0x26EA, 0x26EA, 1); // Church
    table.add_range16(0x26F2, 0x26F3, 1); // Fountain, flag
    table.add_range16(0x26F5, 0x26F5, 1); // Sailboat
    table.add_range16(0x26FA, 0x26FA, 1); // Tent
    table.add_range16(0x26FD, 0x26FD, 1); // Fuel pump
    table.add_range32(0x1F004, 0x1F004, 1); // Mahjong tile
    table.add_range32(0x1F0CF, 0x1F0CF, 1); // Playing card
    table.add_range32(0x1F18E, 0x1F18E, 1); // AB button
    table.add_range32(0x1F191, 0x1F19A, 1); // CL to VS buttons
    table.add_range32(0x1F1E6, 0x1F1FF, 1); // Regional indicators
    table.add_range32(0x1F201, 0x1F202, 1); // Squared katakana
    table.add_range32(0x1F21A, 0x1F21A, 1); // Squared CJK
    table.add_range32(0x1F22F, 0x1F22F, 1); // Squared CJK
    table.add_range32(0x1F232, 0x1F236, 1); // Squared CJK
    table.add_range32(0x1F238, 0x1F23A, 1); // Squared CJK
    table.add_range32(0x1F250, 0x1F251, 1); // Circled CJK
    table.add_range32(0x1F300, 0x1F320, 1); // Miscellaneous symbols
    table.add_range32(0x1F32D, 0x1F335, 1); // Food and nature
    table.add_range32(0x1F337, 0x1F37C, 1); // Plants and drinks
    table.add_range32(0x1F37E, 0x1F393, 1); // Bottle to graduation cap
    table.add_range32(0x1F3A0, 0x1F3CA, 1); // Entertainment and sports
    table.add_range32(0x1F3CF, 0x1F3D3, 1); // Sports equipment
    table.add_range32(0x1F3E0, 0x1F3F0, 1); // Buildings
    table.add_range32(0x1F3F4, 0x1F3F4, 1); // Black flag
    table.add_range32(0x1F3F8, 0x1F43E, 1); // Badminton to paw prints
    table.add_range32(0x1F440, 0x1F440, 1); // Eyes
    table.add_range32(0x1F442, 0x1F4FC, 1); // Ear to videocassette
    table.add_range32(0x1F4FF, 0x1F53D, 1); // Prayer beads to down arrow
    table.add_range32(0x1F54B, 0x1F54E, 1); // Kaaba to menorah
    table.add_range32(0x1F550, 0x1F567, 1); // Clock faces
    table.add_range32(0x1F57A, 0x1F57A, 1); // Man dancing
    table.add_range32(0x1F595, 0x1F596, 1); // Hand gestures
    table.add_range32(0x1F5A4, 0x1F5A4, 1); // Black heart
    table.add_range32(0x1F5FB, 0x1F64F, 1); // Mount Fuji to folded hands
    table.add_range32(0x1F680, 0x1F6C5, 1); // Transportation
    table.add_range32(0x1F6CC, 0x1F6CC, 1); // Sleeping accommodation
    table.add_range32(0x1F6D0, 0x1F6D2, 1); // Place of worship to shopping cart
    table.add_range32(0x1F6D5, 0x1F6D7, 1); // Hindu temple to elevator
    table.add_range32(0x1F6EB, 0x1F6EC, 1); // Airplane departure/arrival
    table.add_range32(0x1F6F4, 0x1F6FC, 1); // Scooter to roller skate
    table.add_range32(0x1F7E0, 0x1F7EB, 1); // Colored circles and squares
    table.add_range32(0x1F90C, 0x1F93A, 1); // Various gestures and expressions
    table.add_range32(0x1F93C, 0x1F945, 1); // Wrestlers to goal net
    table.add_range32(0x1F947, 0x1F978, 1); // Medals to disguised face
    table.add_range32(0x1F97A, 0x1F9CB, 1); // Pleading face to bubble tea
    table.add_range32(0x1F9CD, 0x1F9FF, 1); // Standing person to nazar amulet
    table.add_range32(0x1FA70, 0x1FA74, 1); // Ballet shoes to thong sandal
    table.add_range32(0x1FA78, 0x1FA7A, 1); // Drop of blood to stethoscope
    table.add_range32(0x1FA80, 0x1FA86, 1); // Yo-yo to nesting dolls
    table.add_range32(0x1FA90, 0x1FAA8, 1); // Ringed planet to rock
    table.add_range32(0x1FAB0, 0x1FAB6, 1); // Fly to feather
    table.add_range32(0x1FAC0, 0x1FAC2, 1); // Anatomical heart to people hugging
    table.add_range32(0x1FAD0, 0x1FAD6, 1); // Blueberries to teapot
    table
});

pub static EMOJI_PRESENTATION: Lazy<RangeTable> = Lazy::new(|| {
    // Subset of emoji that have emoji presentation by default
    let mut table = RangeTable::new();
    table.add_range16(0x231A, 0x231B, 1);
    table.add_range16(0x23E9, 0x23EC, 1);
    table.add_range16(0x23F0, 0x23F0, 1);
    table.add_range16(0x23F3, 0x23F3, 1);
    table.add_range32(0x1F004, 0x1F004, 1);
    table.add_range32(0x1F0CF, 0x1F0CF, 1);
    table
});

pub static EMOJI_MODIFIER: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range32(0x1F3FB, 0x1F3FF, 1); // Skin tone modifiers
    table
});

pub static EMOJI_MODIFIER_BASE: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range32(0x261D, 0x261D, 1); // White up pointing index
    table.add_range32(0x26F9, 0x26F9, 1); // Person bouncing ball
    table.add_range32(0x270A, 0x270D, 1); // Various hand gestures
    table.add_range32(0x1F385, 0x1F385, 1); // Santa Claus
    table.add_range32(0x1F3C2, 0x1F3C4, 1); // Snowboarder to surfer
    table.add_range32(0x1F3C7, 0x1F3C7, 1); // Horse racing
    table.add_range32(0x1F3CA, 0x1F3CC, 1); // Swimmer to golfer
    table.add_range32(0x1F442, 0x1F443, 1); // Ear and nose
    table.add_range32(0x1F446, 0x1F450, 1); // Various pointing hands
    table.add_range32(0x1F466, 0x1F478, 1); // Various people
    table.add_range32(0x1F47C, 0x1F47C, 1); // Baby angel
    table.add_range32(0x1F481, 0x1F483, 1); // Information desk person to dancer
    table.add_range32(0x1F485, 0x1F487, 1); // Nail polish to haircut
    table.add_range32(0x1F48F, 0x1F48F, 1); // Kiss
    table.add_range32(0x1F491, 0x1F491, 1); // Couple with heart
    table.add_range32(0x1F4AA, 0x1F4AA, 1); // Flexed biceps
    table.add_range32(0x1F574, 0x1F575, 1); // Man in business suit levitating and detective
    table.add_range32(0x1F57A, 0x1F57A, 1); // Man dancing
    table.add_range32(0x1F590, 0x1F590, 1); // Raised hand with fingers splayed
    table.add_range32(0x1F595, 0x1F596, 1); // Middle finger and raised hand
    table.add_range32(0x1F645, 0x1F647, 1); // Face gestures
    table.add_range32(0x1F64B, 0x1F64F, 1); // Various gestures
    table.add_range32(0x1F6A3, 0x1F6A3, 1); // Rowboat
    table.add_range32(0x1F6B4, 0x1F6B6, 1); // Cyclist to pedestrian
    table.add_range32(0x1F6C0, 0x1F6C0, 1); // Bath
    table.add_range32(0x1F6CC, 0x1F6CC, 1); // Sleeping accommodation
    table.add_range32(0x1F90F, 0x1F90F, 1); // Pinching hand
    table.add_range32(0x1F918, 0x1F91F, 1); // Various hand gestures
    table.add_range32(0x1F926, 0x1F926, 1); // Face palm
    table.add_range32(0x1F930, 0x1F939, 1); // Various people and gestures
    table.add_range32(0x1F93C, 0x1F93E, 1); // Wrestlers and various activities
    table
});

pub static EMOJI_COMPONENT: Lazy<RangeTable> = Lazy::new(|| {
    let mut table = RangeTable::new();
    table.add_range16(0x0023, 0x0023, 1); // Number sign
    table.add_range16(0x002A, 0x002A, 1); // Asterisk
    table.add_range16(0x0030, 0x0039, 1); // Digits 0-9
    table.add_range16(0x00A9, 0x00A9, 1); // Copyright
    table.add_range16(0x00AE, 0x00AE, 1); // Registered
    table.add_range16(0x203C, 0x203C, 1); // Double exclamation mark
    table.add_range16(0x2049, 0x2049, 1); // Exclamation question mark
    table.add_range16(0x2122, 0x2122, 1); // Trade mark
    table.add_range16(0x2139, 0x2139, 1); // Information source
    table.add_range16(0x2194, 0x2199, 1); // Various arrows
    table.add_range16(0x21A9, 0x21AA, 1); // Curved arrows
    table.add_range16(0x231A, 0x231B, 1); // Watch and hourglass
    table.add_range16(0x2328, 0x2328, 1); // Keyboard
    table.add_range16(0x23CF, 0x23CF, 1); // Eject symbol
    table.add_range16(0x23E9, 0x23F3, 1); // Various media controls
    table.add_range16(0x23F8, 0x23FA, 1); // Pause, stop, record
    table.add_range16(0x24C2, 0x24C2, 1); // Circled M
    table.add_range16(0x25AA, 0x25AB, 1); // Small squares
    table.add_range16(0x25B6, 0x25B6, 1); // Play button
    table.add_range16(0x25C0, 0x25C0, 1); // Reverse button
    table.add_range16(0x25FB, 0x25FE, 1); // Various squares
    table.add_range16(0x2600, 0x2604, 1); // Weather symbols
    table.add_range16(0x260E, 0x260E, 1); // Telephone
    table.add_range16(0x2611, 0x2611, 1); // Ballot box with check
    table.add_range16(0x2614, 0x2615, 1); // Umbrella and hot beverage
    table.add_range16(0x2618, 0x2618, 1); // Shamrock
    table.add_range16(0x261D, 0x261D, 1); // White up pointing index
    table.add_range16(0x2620, 0x2620, 1); // Skull and crossbones
    table.add_range16(0x2622, 0x2623, 1); // Radioactive and biohazard
    table.add_range16(0x2626, 0x2626, 1); // Orthodox cross
    table.add_range16(0x262A, 0x262A, 1); // Star and crescent
    table.add_range16(0x262E, 0x262F, 1); // Peace symbols
    table.add_range16(0x2638, 0x263A, 1); // Wheel of dharma and smiley
    table.add_range16(0x2640, 0x2640, 1); // Female sign
    table.add_range16(0x2642, 0x2642, 1); // Male sign
    table.add_range16(0x2648, 0x2653, 1); // Zodiac signs
    table.add_range16(0x265F, 0x2660, 1); // Chess piece and spade suit
    table.add_range16(0x2663, 0x2663, 1); // Club suit
    table.add_range16(0x2665, 0x2666, 1); // Heart and diamond suits
    table.add_range16(0x2668, 0x2668, 1); // Hot springs
    table.add_range16(0x267B, 0x267B, 1); // Recycling symbol
    table.add_range16(0x267E, 0x267F, 1); // Infinity and wheelchair
    table.add_range16(0x2692, 0x2697, 1); // Various symbols
    table.add_range16(0x2699, 0x2699, 1); // Gear
    table.add_range16(0x269B, 0x269C, 1); // Atom and fleur-de-lis
    table.add_range16(0x26A0, 0x26A1, 1); // Warning and high voltage
    table.add_range16(0x26AA, 0x26AB, 1); // Circles
    table.add_range16(0x26B0, 0x26B1, 1); // Coffin and funeral urn
    table.add_range16(0x26BD, 0x26BE, 1); // Soccer and baseball
    table.add_range16(0x26C4, 0x26C5, 1); // Snowman and sun
    table.add_range16(0x26C8, 0x26C8, 1); // Thunder cloud and rain
    table.add_range16(0x26CE, 0x26CF, 1); // Ophiuchus and pick
    table.add_range16(0x26D1, 0x26D1, 1); // Helmet with cross
    table.add_range16(0x26D3, 0x26D4, 1); // Chains and no entry
    table.add_range16(0x26E9, 0x26EA, 1); // Shinto shrine and church
    table.add_range16(0x26F0, 0x26F5, 1); // Mountain to sailboat
    table.add_range16(0x26F7, 0x26FA, 1); // Skier to tent
    table.add_range16(0x26FD, 0x26FD, 1); // Fuel pump
    table.add_range32(0x1F004, 0x1F004, 1); // Mahjong red dragon
    table.add_range32(0x1F0CF, 0x1F0CF, 1); // Playing card black joker
    table.add_range32(0x1F170, 0x1F171, 1); // A and B buttons
    table.add_range32(0x1F17E, 0x1F17F, 1); // O and P buttons
    table.add_range32(0x1F18E, 0x1F18E, 1); // AB button
    table.add_range32(0x1F191, 0x1F19A, 1); // Various squared buttons
    table.add_range32(0x1F1E6, 0x1F1FF, 1); // Regional indicator symbols
    table.add_range32(0x1F201, 0x1F202, 1); // Squared katakana koko and sa
    table.add_range32(0x1F21A, 0x1F21A, 1); // Squared CJK unified ideograph
    table.add_range32(0x1F22F, 0x1F22F, 1); // Squared CJK unified ideograph
    table.add_range32(0x1F232, 0x1F236, 1); // Various squared CJK
    table.add_range32(0x1F238, 0x1F23A, 1); // Squared CJK
    table.add_range32(0x1F250, 0x1F251, 1); // Circled ideographs
    table.add_range32(0x1F300, 0x1F320, 1); // Cyclone to shooting star
    table.add_range32(0x1F321, 0x1F321, 1); // Thermometer
    table.add_range32(0x1F324, 0x1F32C, 1); // Weather symbols
    table.add_range32(0x1F32D, 0x1F32F, 1); // Hot dog to burrito
    table.add_range32(0x1F330, 0x1F335, 1); // Chestnut to cactus
    table.add_range32(0x1F336, 0x1F336, 1); // Hot pepper
    table.add_range32(0x1F337, 0x1F37C, 1); // Tulip to baby bottle
    table.add_range32(0x1F37D, 0x1F37D, 1); // Fork and knife with plate
    table.add_range32(0x1F37E, 0x1F37F, 1); // Bottle with popping cork and popcorn
    table.add_range32(0x1F380, 0x1F393, 1); // Ribbon to graduation cap
    table.add_range32(0x1F396, 0x1F397, 1); // Military medal and reminder ribbon
    table.add_range32(0x1F399, 0x1F39B, 1); // Studio microphone to control knobs
    table.add_range32(0x1F39E, 0x1F39F, 1); // Film frames and admission tickets
    table.add_range32(0x1F3A0, 0x1F3CA, 1); // Carousel horse to swimmer
    table.add_range32(0x1F3CB, 0x1F3CE, 1); // Weight lifter to racing car
    table.add_range32(0x1F3CF, 0x1F3D3, 1); // Cricket game to ping pong
    table.add_range32(0x1F3D4, 0x1F3DF, 1); // Snow capped mountain to stadium
    table.add_range32(0x1F3E0, 0x1F3F0, 1); // House to castle
    table.add_range32(0x1F3F3, 0x1F3F5, 1); // Waving flag to rosette
    table.add_range32(0x1F3F7, 0x1F3F7, 1); // Label
    table.add_range32(0x1F3F8, 0x1F3FF, 1); // Badminton to dark skin tone
    table.add_range32(0x1F400, 0x1F43E, 1); // Rat to paw prints
    table.add_range32(0x1F43F, 0x1F43F, 1); // Chipmunk
    table.add_range32(0x1F440, 0x1F440, 1); // Eyes
    table.add_range32(0x1F441, 0x1F441, 1); // Eye
    table.add_range32(0x1F442, 0x1F4F7, 1); // Ear to camera
    table.add_range32(0x1F4F8, 0x1F4F8, 1); // Camera with flash
    table.add_range32(0x1F4F9, 0x1F4FC, 1); // Video camera to videocassette
    table.add_range32(0x1F4FD, 0x1F4FE, 1); // Film projector and portable stereo
    table.add_range32(0x1F4FF, 0x1F4FF, 1); // Prayer beads
    table.add_range32(0x1F500, 0x1F53D, 1); // Twisted arrows to down button
    table.add_range32(0x1F549, 0x1F54A, 1); // Om symbol and dove
    table.add_range32(0x1F54B, 0x1F54E, 1); // Kaaba to menorah
    table.add_range32(0x1F550, 0x1F567, 1); // Clock faces
    table.add_range32(0x1F56F, 0x1F570, 1); // Candle and mantelpiece clock
    table.add_range32(0x1F573, 0x1F579, 1); // Hole to joystick
    table.add_range32(0x1F57A, 0x1F57A, 1); // Man dancing
    table.add_range32(0x1F587, 0x1F587, 1); // Linked paperclips
    table.add_range32(0x1F58A, 0x1F58D, 1); // Lower left ballpoint pen to lower left crayon
    table.add_range32(0x1F590, 0x1F590, 1); // Raised hand with fingers splayed
    table.add_range32(0x1F595, 0x1F596, 1); // Middle finger and raised hand with part
    table.add_range32(0x1F5A4, 0x1F5A4, 1); // Black heart
    table.add_range32(0x1F5A5, 0x1F5A5, 1); // Desktop computer
    table.add_range32(0x1F5A8, 0x1F5A8, 1); // Printer
    table.add_range32(0x1F5B1, 0x1F5B2, 1); // Computer mouse and trackball
    table.add_range32(0x1F5BC, 0x1F5BC, 1); // Frame with picture
    table.add_range32(0x1F5C2, 0x1F5C4, 1); // Card index dividers to file cabinet
    table.add_range32(0x1F5D1, 0x1F5D3, 1); // Wastebasket to spiral calendar
    table.add_range32(0x1F5DC, 0x1F5DE, 1); // Compression to rolled-up newspaper
    table.add_range32(0x1F5E1, 0x1F5E1, 1); // Dagger
    table.add_range32(0x1F5E3, 0x1F5E3, 1); // Speaking head
    table.add_range32(0x1F5E8, 0x1F5E8, 1); // Left speech bubble
    table.add_range32(0x1F5EF, 0x1F5EF, 1); // Right anger bubble
    table.add_range32(0x1F5F3, 0x1F5F3, 1); // Ballot box with ballot
    table.add_range32(0x1F5FA, 0x1F5FA, 1); // World map
    table.add_range32(0x1F5FB, 0x1F5FF, 1); // Mount Fuji to moyai
    table.add_range32(0x1F600, 0x1F64F, 1); // Grinning face to folded hands
    table.add_range32(0x1F680, 0x1F6C5, 1); // Rocket to left luggage
    table.add_range32(0x1F6CB, 0x1F6CF, 1); // Couch and bed to track and airplane
    table.add_range32(0x1F6D0, 0x1F6D2, 1); // Place of worship to shopping cart
    table.add_range32(0x1F6D5, 0x1F6D7, 1); // Hindu temple to elevator
    table.add_range32(0x1F6DC, 0x1F6DF, 1); // Wireless to ring buoy
    table.add_range32(0x1F6E0, 0x1F6EC, 1); // Hammer and wrench to airplane arrival
    table.add_range32(0x1F6F0, 0x1F6FC, 1); // Satellite to roller skate
    table.add_range32(0x1F700, 0x1F773, 1); // Alchemical symbols
    table.add_range32(0x1F780, 0x1F7D8, 1); // Black square and symbols
    table.add_range32(0x1F7E0, 0x1F7EB, 1); // Orange circle to brown square
    table.add_range32(0x1F7F0, 0x1F7F0, 1); // Heavy equals sign
    table.add_range32(0x1F800, 0x1F80B, 1); // Leftwards arrow with small triangle arrowhead to rightwards arrow
    table.add_range32(0x1F810, 0x1F847, 1); // Leftward arrow with small equilateral arrowhead to heavy rightward arrow
    table.add_range32(0x1F850, 0x1F859, 1); // Leftward sans-serif arrow to up down arrow
    table.add_range32(0x1F860, 0x1F887, 1); // Wide-headed leftward light arrow to wide-headed rightward heavy barb arrow
    table.add_range32(0x1F890, 0x1F8AD, 1); // Leftward triangle arrowhead to white arrow shaft
    table.add_range32(0x1F8B0, 0x1F8B1, 1); // Arrow pointing upwards then north west and arrow pointing right
    table.add_range32(0x1F900, 0x1F90B, 1); // Circled cross formee with four dots to white large square
    table.add_range32(0x1F90D, 0x1F90F, 1); // White heart to pinching hand
    table.add_range32(0x1F910, 0x1F93A, 1); // Zipper-mouth face to fencer
    table.add_range32(0x1F93C, 0x1F945, 1); // Wrestlers to goal net
    table.add_range32(0x1F947, 0x1F978, 1); // First place medal to disguised face
    table.add_range32(0x1F97A, 0x1F9CB, 1); // Pleading face to bubble tea
    table.add_range32(0x1F9CD, 0x1F9FF, 1); // Standing person to nazar amulet
    table.add_range32(0x1FA70, 0x1FA74, 1); // Ballet shoes to thong sandal
    table.add_range32(0x1FA78, 0x1FA7A, 1); // Drop of blood to stethoscope
    table.add_range32(0x1FA80, 0x1FA86, 1); // Yo-yo to nesting dolls
    table.add_range32(0x1FA90, 0x1FAA8, 1); // Ringed planet to rock
    table.add_range32(0x1FAB0, 0x1FAB6, 1); // Fly to feather
    table.add_range32(0x1FAC0, 0x1FAC2, 1); // Anatomical heart to people hugging
    table.add_range32(0x1FAD0, 0x1FAD6, 1); // Blueberries to teapot
    table.add_range32(0x1FAF0, 0x1FAF6, 1); // Hand with index finger and thumb crossed to heart hands
    // Zero-width joiner for emoji sequences
    table.add_range16(0x200D, 0x200D, 1);
    // Variation selectors for emoji presentation
    table.add_range16(0xFE0E, 0xFE0F, 1);
    table
});

pub static EXTENDED_PICTOGRAPHIC: Lazy<RangeTable> = Lazy::new(|| {
    // Extended pictographic property for emoji
    let mut table = RangeTable::new();
    table.add_range16(0x00A9, 0x00A9, 1); // Copyright
    table.add_range16(0x00AE, 0x00AE, 1); // Registered
    table.add_range16(0x203C, 0x203C, 1); // Double exclamation mark
    table.add_range16(0x2049, 0x2049, 1); // Exclamation question mark
    table.add_range16(0x2122, 0x2122, 1); // Trade mark
    table.add_range16(0x2139, 0x2139, 1); // Information source
    table.add_range16(0x2194, 0x2199, 1); // Arrows
    table.add_range16(0x21A9, 0x21AA, 1); // Curved arrows
    table.add_range16(0x231A, 0x231B, 1); // Watch and hourglass
    table.add_range16(0x2328, 0x2328, 1); // Keyboard
    table.add_range16(0x2388, 0x2388, 1); // Helm symbol
    table.add_range16(0x23CF, 0x23CF, 1); // Eject symbol
    table.add_range16(0x23E9, 0x23F3, 1); // Various media controls
    table.add_range16(0x23F8, 0x23FA, 1); // Pause, stop, record
    table.add_range16(0x24C2, 0x24C2, 1); // Circled M
    table.add_range16(0x25AA, 0x25AB, 1); // Small squares
    table.add_range16(0x25B6, 0x25B6, 1); // Play button
    table.add_range16(0x25C0, 0x25C0, 1); // Reverse button
    table.add_range16(0x25FB, 0x25FE, 1); // Various squares
    table.add_range16(0x2600, 0x2605, 1); // Weather and star symbols
    table.add_range16(0x2607, 0x2612, 1); // Various symbols
    table.add_range16(0x2614, 0x2685, 1); // Large range of symbols
    table.add_range16(0x2690, 0x2705, 1); // More symbols
    table.add_range16(0x2708, 0x2712, 1); // Airplane and writing symbols
    table.add_range16(0x2714, 0x2714, 1); // Heavy check mark
    table.add_range16(0x2716, 0x2716, 1); // Heavy multiplication X
    table.add_range16(0x271D, 0x271D, 1); // Latin cross
    table.add_range16(0x2721, 0x2721, 1); // Star of David
    table.add_range16(0x2728, 0x2728, 1); // Sparkles
    table.add_range16(0x2733, 0x2734, 1); // Eight spoked asterisk and eight pointed star
    table.add_range16(0x2744, 0x2744, 1); // Snowflake
    table.add_range16(0x2747, 0x2747, 1); // Sparkle
    table.add_range16(0x274C, 0x274C, 1); // Cross mark
    table.add_range16(0x274E, 0x274E, 1); // Squared cross mark
    table.add_range16(0x2753, 0x2755, 1); // Question marks
    table.add_range16(0x2757, 0x2757, 1); // Heavy exclamation mark
    table.add_range16(0x2763, 0x2767, 1); // Various symbols
    table.add_range16(0x2795, 0x2797, 1); // Plus, minus, division
    table.add_range16(0x27A1, 0x27A1, 1); // Black rightwards arrow
    table.add_range16(0x27B0, 0x27B0, 1); // Curly loop
    table.add_range16(0x27BF, 0x27BF, 1); // Double curly loop
    table.add_range16(0x2934, 0x2935, 1); // Curved arrows
    table.add_range16(0x2B05, 0x2B07, 1); // Arrows
    table.add_range16(0x2B1B, 0x2B1C, 1); // Squares
    table.add_range16(0x2B50, 0x2B50, 1); // White medium star
    table.add_range16(0x2B55, 0x2B55, 1); // Heavy large circle
    table.add_range16(0x3030, 0x3030, 1); // Wavy dash
    table.add_range16(0x303D, 0x303D, 1); // Part alternation mark
    table.add_range16(0x3297, 0x3297, 1); // Circled ideograph congratulation
    table.add_range16(0x3299, 0x3299, 1); // Circled ideograph secret
    table.add_range32(0x1F000, 0x1F0FF, 1); // Mahjong tiles and playing cards
    table.add_range32(0x1F10D, 0x1F10F, 1); // Circled zero with slash to circled dollar sign
    table.add_range32(0x1F12F, 0x1F12F, 1); // Copyleft symbol
    table.add_range32(0x1F16C, 0x1F171, 1); // Raised symbols
    table.add_range32(0x1F17E, 0x1F17F, 1); // O and P buttons
    table.add_range32(0x1F18E, 0x1F18E, 1); // AB button
    table.add_range32(0x1F191, 0x1F19A, 1); // Squared buttons
    table.add_range32(0x1F1AD, 0x1F1E5, 1); // Mask work symbol to regional indicators
    table.add_range32(0x1F201, 0x1F20F, 1); // Squared characters
    table.add_range32(0x1F21A, 0x1F21A, 1); // Squared CJK unified ideograph
    table.add_range32(0x1F22F, 0x1F22F, 1); // Squared CJK unified ideograph
    table.add_range32(0x1F232, 0x1F23A, 1); // Squared CJK ideographs
    table.add_range32(0x1F23C, 0x1F23F, 1); // Squared symbols
    table.add_range32(0x1F249, 0x1F3FA, 1); // Various symbols and pictographs
    table.add_range32(0x1F400, 0x1F53D, 1); // Animals and various symbols
    table.add_range32(0x1F546, 0x1F64F, 1); // Various symbols and emoji
    table.add_range32(0x1F680, 0x1F6FF, 1); // Transport and map symbols
    table.add_range32(0x1F774, 0x1F77F, 1); // Alchemical symbols
    table.add_range32(0x1F7D5, 0x1F7FF, 1); // Circled symbols
    table.add_range32(0x1F80C, 0x1F80F, 1); // Arrows
    table.add_range32(0x1F848, 0x1F84F, 1); // Arrows
    table.add_range32(0x1F85A, 0x1F85F, 1); // Arrows
    table.add_range32(0x1F888, 0x1F88F, 1); // Arrows
    table.add_range32(0x1F8AE, 0x1F8FF, 1); // Arrows and symbols
    table.add_range32(0x1F90C, 0x1F93A, 1); // Various emoji
    table.add_range32(0x1F93C, 0x1F945, 1); // Wrestlers and sports
    table.add_range32(0x1F947, 0x1FAFF, 1); // Medals and various emoji
    table
});

/// Global registry for Unicode property tables
static RANGE_TABLE_REGISTRY: Lazy<HashMap<&'static str, &'static RangeTable>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    
    // Letter categories
    registry.insert("Letter", &*LETTER);
    registry.insert("UppercaseLetter", &*UPPERCASE_LETTER);
    registry.insert("LowercaseLetter", &*LOWERCASE_LETTER);
    registry.insert("TitlecaseLetter", &*TITLECASE_LETTER);
    registry.insert("ModifierLetter", &*MODIFIER_LETTER);
    registry.insert("OtherLetter", &*OTHER_LETTER);
    
    // Number categories
    registry.insert("Number", &*NUMBER);
    registry.insert("DecimalNumber", &*DECIMAL_NUMBER);
    registry.insert("LetterNumber", &*LETTER_NUMBER);
    registry.insert("OtherNumber", &*OTHER_NUMBER);
    
    // Punctuation categories
    registry.insert("Punct", &*PUNCT);
    registry.insert("ConnectorPunctuation", &*CONNECTOR_PUNCTUATION);
    registry.insert("DashPunctuation", &*DASH_PUNCTUATION);
    registry.insert("OpenPunctuation", &*OPEN_PUNCTUATION);
    registry.insert("ClosePunctuation", &*CLOSE_PUNCTUATION);
    registry.insert("InitialPunctuation", &*INITIAL_PUNCTUATION);
    registry.insert("FinalPunctuation", &*FINAL_PUNCTUATION);
    registry.insert("OtherPunctuation", &*OTHER_PUNCTUATION);
    
    // Symbol categories
    registry.insert("Symbol", &*SYMBOL);
    registry.insert("MathSymbol", &*MATH_SYMBOL);
    registry.insert("CurrencySymbol", &*CURRENCY_SYMBOL);
    registry.insert("ModifierSymbol", &*MODIFIER_SYMBOL);
    registry.insert("OtherSymbol", &*OTHER_SYMBOL);
    
    // Mark categories
    registry.insert("Mark", &*MARK);
    registry.insert("NonSpacingMark", &*NON_SPACING_MARK);
    registry.insert("SpacingMark", &*SPACING_MARK);
    registry.insert("EnclosingMark", &*ENCLOSING_MARK);
    
    // Other categories
    registry.insert("Space", &*SPACE);
    registry.insert("Control", &*CONTROL);
    registry.insert("Format", &*FORMAT);
    registry.insert("Surrogate", &*SURROGATE);
    registry.insert("Private", &*PRIVATE);
    registry.insert("Unassigned", &*UNASSIGNED);
    
    // Scripts
    registry.insert("Latin", &*LATIN);
    registry.insert("Greek", &*GREEK);
    registry.insert("Cyrillic", &*CYRILLIC);
    registry.insert("Hebrew", &*HEBREW);
    registry.insert("Arabic", &*ARABIC);
    registry.insert("Devanagari", &*DEVANAGARI);
    registry.insert("Thai", &*THAI);
    registry.insert("Han", &*HAN);
    registry.insert("Hiragana", &*HIRAGANA);
    registry.insert("Katakana", &*KATAKANA);
    registry.insert("Hangul", &*HANGUL);
    
    // Special categories
    registry.insert("Emoji", &*EMOJI);
    registry.insert("EmojiPresentation", &*EMOJI_PRESENTATION);
    registry.insert("EmojiModifier", &*EMOJI_MODIFIER);
    registry.insert("EmojiModifierBase", &*EMOJI_MODIFIER_BASE);
    registry.insert("EmojiComponent", &*EMOJI_COMPONENT);
    registry.insert("ExtendedPictographic", &*EXTENDED_PICTOGRAPHIC);
    
    registry
});

/// Get a range table by name
pub fn get_range_table(name: &str) -> Option<&'static RangeTable> {
    RANGE_TABLE_REGISTRY.get(name).copied()
}

/// Initialize Unicode range tables
pub fn initialize_tables() -> GlyphGangResult<()> {
    // Force initialization of lazy statics
    Lazy::force(&RANGE_TABLE_REGISTRY);
    
    // Validate that tables loaded correctly
    if RANGE_TABLE_REGISTRY.is_empty() {
        return Err(range_error("Failed to initialize Unicode range tables"));
    }
    
    Ok(())
}

/// Get list of all available range table names
pub fn get_available_tables() -> Vec<&'static str> {
    RANGE_TABLE_REGISTRY.keys().copied().collect()
}

/// Validate that a range table contains expected characters for basic testing
pub fn validate_basic_tables() -> GlyphGangResult<()> {
    // Test basic ASCII letter classification
    if !LETTER.contains('A') || !LETTER.contains('a') {
        return Err(range_error("Letter table validation failed for basic ASCII"));
    }
    
    if !UPPERCASE_LETTER.contains('A') || UPPERCASE_LETTER.contains('a') {
        return Err(range_error("Uppercase letter table validation failed"));
    }
    
    if !LOWERCASE_LETTER.contains('a') || LOWERCASE_LETTER.contains('A') {
        return Err(range_error("Lowercase letter table validation failed"));
    }
    
    if !NUMBER.contains('5') || NUMBER.contains('A') {
        return Err(range_error("Number table validation failed"));
    }
    
    if !SPACE.contains(' ') || SPACE.contains('A') {
        return Err(range_error("Space table validation failed"));
    }
    
    Ok(())
}
