/// Core character classification and conversion functions
// use crate::stdlib::glyph_gang::error::{GlyphGangResult, GlyphGangError, unicode_error, unicode_error_with_code_point, name_lookup_error};
// use crate::stdlib::glyph_gang::ranges::{
    RangeTable, 
    LETTER, UPPERCASE_LETTER, LOWERCASE_LETTER, TITLECASE_LETTER, MODIFIER_LETTER, OTHER_LETTER,
    NUMBER, DECIMAL_NUMBER, LETTER_NUMBER, OTHER_NUMBER,
    PUNCT, SYMBOL, MARK, SPACE, CONTROL, FORMAT, 
    SURROGATE, PRIVATE, CURRENCY_SYMBOL, MATH_SYMBOL,
    EMOJI, EMOJI_MODIFIER, EMOJI_COMPONENT, EMOJI_MODIFIER_BASE,
    get_range_table
};
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Character name database (simplified for initial implementation)
static CHARACTER_NAMES: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut names = HashMap::new();
    
    // Basic ASCII characters
    for i in 0x20..=0x7E {
        let name = match i {
            0x20 => "SPACE",
            0x21 => "EXCLAMATION MARK", 
            0x22 => "QUOTATION MARK",
            0x23 => "NUMBER SIGN",
            0x24 => "DOLLAR SIGN",
            0x25 => "PERCENT SIGN",
            0x26 => "AMPERSAND",
            0x27 => "APOSTROPHE",
            0x28 => "LEFT PARENTHESIS",
            0x29 => "RIGHT PARENTHESIS",
            0x2A => "ASTERISK",
            0x2B => "PLUS SIGN",
            0x2C => "COMMA",
            0x2D => "HYPHEN-MINUS",
            0x2E => "FULL STOP",
            0x2F => "SOLIDUS",
            0x30..=0x39 => "DIGIT ZERO",  // Will be adjusted below for 1-9
            0x3A => "COLON",
            0x3B => "SEMICOLON",
            0x3C => "LESS-THAN SIGN",
            0x3D => "EQUALS SIGN",
            0x3E => "GREATER-THAN SIGN",
            0x3F => "QUESTION MARK",
            0x40 => "COMMERCIAL AT",
            0x41..=0x5A => "LATIN CAPITAL LETTER A", // Will be adjusted below
            0x5B => "LEFT SQUARE BRACKET",
            0x5C => "REVERSE SOLIDUS",
            0x5D => "RIGHT SQUARE BRACKET",
            0x5E => "CIRCUMFLEX ACCENT",
            0x5F => "LOW LINE",
            0x60 => "GRAVE ACCENT",
            0x61..=0x7A => "LATIN SMALL LETTER A", // Will be adjusted below
            0x7B => "LEFT CURLY BRACKET",
            0x7C => "VERTICAL LINE",
            0x7D => "RIGHT CURLY BRACKET",
            0x7E => "TILDE",
            _ => continue,
        };
        names.insert(i, name);
    }
    
    // Adjust digit names
    for i in 0x30..=0x39 {
        let digit = (i - 0x30) as u8;
        let name = match digit {
            0 => "DIGIT ZERO",
            1 => "DIGIT ONE", 
            2 => "DIGIT TWO",
            3 => "DIGIT THREE",
            4 => "DIGIT FOUR",
            5 => "DIGIT FIVE",
            6 => "DIGIT SIX",
            7 => "DIGIT SEVEN",
            8 => "DIGIT EIGHT",
            9 => "DIGIT NINE",
            _ => unreachable!(),
        };
        names.insert(i, name);
    }
    
    // Adjust capital letter names
    for i in 0x41..=0x5A {
        let letter = (i - 0x41) as u8 + b'A';
        let name = match letter as char {
            'A' => "LATIN CAPITAL LETTER A",
            'B' => "LATIN CAPITAL LETTER B",
            'C' => "LATIN CAPITAL LETTER C",
            'D' => "LATIN CAPITAL LETTER D",
            'E' => "LATIN CAPITAL LETTER E",
            'F' => "LATIN CAPITAL LETTER F",
            'G' => "LATIN CAPITAL LETTER G",
            'H' => "LATIN CAPITAL LETTER H",
            'I' => "LATIN CAPITAL LETTER I",
            'J' => "LATIN CAPITAL LETTER J",
            'K' => "LATIN CAPITAL LETTER K",
            'L' => "LATIN CAPITAL LETTER L",
            'M' => "LATIN CAPITAL LETTER M",
            'N' => "LATIN CAPITAL LETTER N",
            'O' => "LATIN CAPITAL LETTER O",
            'P' => "LATIN CAPITAL LETTER P",
            'Q' => "LATIN CAPITAL LETTER Q",
            'R' => "LATIN CAPITAL LETTER R",
            'S' => "LATIN CAPITAL LETTER S",
            'T' => "LATIN CAPITAL LETTER T",
            'U' => "LATIN CAPITAL LETTER U",
            'V' => "LATIN CAPITAL LETTER V",
            'W' => "LATIN CAPITAL LETTER W",
            'X' => "LATIN CAPITAL LETTER X",
            'Y' => "LATIN CAPITAL LETTER Y",
            'Z' => "LATIN CAPITAL LETTER Z",
            _ => "LATIN CAPITAL LETTER A",
        };
        names.insert(i, name);
    }
    
    // Adjust small letter names
    for i in 0x61..=0x7A {
        let letter = (i - 0x61) as u8 + b'a';
        let name = match letter as char {
            'a' => "LATIN SMALL LETTER A",
            'b' => "LATIN SMALL LETTER B",
            'c' => "LATIN SMALL LETTER C",
            'd' => "LATIN SMALL LETTER D",
            'e' => "LATIN SMALL LETTER E",
            'f' => "LATIN SMALL LETTER F",
            'g' => "LATIN SMALL LETTER G",
            'h' => "LATIN SMALL LETTER H",
            'i' => "LATIN SMALL LETTER I",
            'j' => "LATIN SMALL LETTER J",
            'k' => "LATIN SMALL LETTER K",
            'l' => "LATIN SMALL LETTER L",
            'm' => "LATIN SMALL LETTER M",
            'n' => "LATIN SMALL LETTER N",
            'o' => "LATIN SMALL LETTER O",
            'p' => "LATIN SMALL LETTER P",
            'q' => "LATIN SMALL LETTER Q",
            'r' => "LATIN SMALL LETTER R",
            's' => "LATIN SMALL LETTER S",
            't' => "LATIN SMALL LETTER T",
            'u' => "LATIN SMALL LETTER U",
            'v' => "LATIN SMALL LETTER V",
            'w' => "LATIN SMALL LETTER W",
            'x' => "LATIN SMALL LETTER X",
            'y' => "LATIN SMALL LETTER Y",
            'z' => "LATIN SMALL LETTER Z",
            _ => "LATIN SMALL LETTER A",
        };
        names.insert(i, name);
    }
    
    // Control characters
    names.insert(0x00, "NULL");
    names.insert(0x01, "START OF HEADING");
    names.insert(0x02, "START OF TEXT");
    names.insert(0x03, "END OF TEXT");
    names.insert(0x04, "END OF TRANSMISSION");
    names.insert(0x05, "ENQUIRY");
    names.insert(0x06, "ACKNOWLEDGE");
    names.insert(0x07, "BELL");
    names.insert(0x08, "BACKSPACE");
    names.insert(0x09, "CHARACTER TABULATION");
    names.insert(0x0A, "LINE FEED");
    names.insert(0x0B, "LINE TABULATION");
    names.insert(0x0C, "FORM FEED");
    names.insert(0x0D, "CARRIAGE RETURN");
    names.insert(0x0E, "SHIFT OUT");
    names.insert(0x0F, "SHIFT IN");
    names.insert(0x10, "DATA LINK ESCAPE");
    names.insert(0x11, "DEVICE CONTROL ONE");
    names.insert(0x12, "DEVICE CONTROL TWO");
    names.insert(0x13, "DEVICE CONTROL THREE");
    names.insert(0x14, "DEVICE CONTROL FOUR");
    names.insert(0x15, "NEGATIVE ACKNOWLEDGE");
    names.insert(0x16, "SYNCHRONOUS IDLE");
    names.insert(0x17, "END OF TRANSMISSION BLOCK");
    names.insert(0x18, "CANCEL");
    names.insert(0x19, "END OF MEDIUM");
    names.insert(0x1A, "SUBSTITUTE");
    names.insert(0x1B, "ESCAPE");
    names.insert(0x1C, "FILE SEPARATOR");
    names.insert(0x1D, "GROUP SEPARATOR");
    names.insert(0x1E, "RECORD SEPARATOR");
    names.insert(0x1F, "UNIT SEPARATOR");
    names.insert(0x7F, "DELETE");
    
    // Extended ASCII and common Unicode characters
    names.insert(0x00A0, "NO-BREAK SPACE");
    names.insert(0x00A1, "INVERTED EXCLAMATION MARK");
    names.insert(0x00A2, "CENT SIGN");
    names.insert(0x00A3, "POUND SIGN");
    names.insert(0x00A4, "CURRENCY SIGN");
    names.insert(0x00A5, "YEN SIGN");
    names.insert(0x00A6, "BROKEN BAR");
    names.insert(0x00A7, "SECTION SIGN");
    names.insert(0x00A8, "DIAERESIS");
    names.insert(0x00A9, "COPYRIGHT SIGN");
    names.insert(0x00AA, "FEMININE ORDINAL INDICATOR");
    names.insert(0x00AB, "LEFT-POINTING DOUBLE ANGLE QUOTATION MARK");
    names.insert(0x00AC, "NOT SIGN");
    names.insert(0x00AD, "SOFT HYPHEN");
    names.insert(0x00AE, "REGISTERED SIGN");
    names.insert(0x00AF, "MACRON");
    names.insert(0x00B0, "DEGREE SIGN");
    names.insert(0x00B1, "PLUS-MINUS SIGN");
    names.insert(0x00B2, "SUPERSCRIPT TWO");
    names.insert(0x00B3, "SUPERSCRIPT THREE");
    names.insert(0x00B4, "ACUTE ACCENT");
    names.insert(0x00B5, "MICRO SIGN");
    names.insert(0x00B6, "PILCROW SIGN");
    names.insert(0x00B7, "MIDDLE DOT");
    names.insert(0x00B8, "CEDILLA");
    names.insert(0x00B9, "SUPERSCRIPT ONE");
    names.insert(0x00BA, "MASCULINE ORDINAL INDICATOR");
    names.insert(0x00BB, "RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK");
    names.insert(0x00BC, "VULGAR FRACTION ONE QUARTER");
    names.insert(0x00BD, "VULGAR FRACTION ONE HALF");
    names.insert(0x00BE, "VULGAR FRACTION THREE QUARTERS");
    names.insert(0x00BF, "INVERTED QUESTION MARK");
    
    // Some common mathematical and Greek symbols
    names.insert(0x0391, "GREEK CAPITAL LETTER ALPHA");
    names.insert(0x0392, "GREEK CAPITAL LETTER BETA");
    names.insert(0x0393, "GREEK CAPITAL LETTER GAMMA");
    names.insert(0x0394, "GREEK CAPITAL LETTER DELTA");
    names.insert(0x03B1, "GREEK SMALL LETTER ALPHA");
    names.insert(0x03B2, "GREEK SMALL LETTER BETA");
    names.insert(0x03B3, "GREEK SMALL LETTER GAMMA");
    names.insert(0x03B4, "GREEK SMALL LETTER DELTA");
    names.insert(0x03C0, "GREEK SMALL LETTER PI");
    
    // Some common CJK characters
    names.insert(0x4E00, "CJK UNIFIED IDEOGRAPH-4E00");
    names.insert(0x4E01, "CJK UNIFIED IDEOGRAPH-4E01");
    names.insert(0x4E02, "CJK UNIFIED IDEOGRAPH-4E02");
    names.insert(0x4E03, "CJK UNIFIED IDEOGRAPH-4E03");
    
    // Some emoji (simplified)
    names.insert(0x1F600, "GRINNING FACE");
    names.insert(0x1F601, "GRINNING FACE WITH SMILING EYES");
    names.insert(0x1F602, "FACE WITH TEARS OF JOY");
    names.insert(0x1F603, "SMILING FACE WITH OPEN MOUTH");
    names.insert(0x1F604, "SMILING FACE WITH OPEN MOUTH AND SMILING EYES");
    names.insert(0x1F44D, "THUMBS UP SIGN");
    names.insert(0x1F44E, "THUMBS DOWN SIGN");
    names.insert(0x2764, "HEAVY BLACK HEART");
    names.insert(0x1F493, "BEATING HEART");
    names.insert(0x1F494, "BROKEN HEART");
    
    names
});

/// Character block name database (simplified)
static BLOCK_NAMES: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut blocks = HashMap::new();
    
    // Define major Unicode blocks
    for i in 0x0000..=0x007F { blocks.insert(i, "Basic Latin"); }
    for i in 0x0080..=0x00FF { blocks.insert(i, "Latin-1 Supplement"); }
    for i in 0x0100..=0x017F { blocks.insert(i, "Latin Extended-A"); }
    for i in 0x0180..=0x024F { blocks.insert(i, "Latin Extended-B"); }
    for i in 0x0250..=0x02AF { blocks.insert(i, "IPA Extensions"); }
    for i in 0x02B0..=0x02FF { blocks.insert(i, "Spacing Modifier Letters"); }
    for i in 0x0300..=0x036F { blocks.insert(i, "Combining Diacritical Marks"); }
    for i in 0x0370..=0x03FF { blocks.insert(i, "Greek and Coptic"); }
    for i in 0x0400..=0x04FF { blocks.insert(i, "Cyrillic"); }
    for i in 0x0500..=0x052F { blocks.insert(i, "Cyrillic Supplement"); }
    for i in 0x0530..=0x058F { blocks.insert(i, "Armenian"); }
    for i in 0x0590..=0x05FF { blocks.insert(i, "Hebrew"); }
    for i in 0x0600..=0x06FF { blocks.insert(i, "Arabic"); }
    for i in 0x0700..=0x074F { blocks.insert(i, "Syriac"); }
    for i in 0x0750..=0x077F { blocks.insert(i, "Arabic Supplement"); }
    for i in 0x0780..=0x07BF { blocks.insert(i, "Thaana"); }
    for i in 0x07C0..=0x07FF { blocks.insert(i, "NKo"); }
    for i in 0x0800..=0x083F { blocks.insert(i, "Samaritan"); }
    for i in 0x0840..=0x085F { blocks.insert(i, "Mandaic"); }
    for i in 0x0860..=0x086F { blocks.insert(i, "Syriac Supplement"); }
    for i in 0x08A0..=0x08FF { blocks.insert(i, "Arabic Extended-A"); }
    for i in 0x0900..=0x097F { blocks.insert(i, "Devanagari"); }
    for i in 0x0980..=0x09FF { blocks.insert(i, "Bengali"); }
    for i in 0x0A00..=0x0A7F { blocks.insert(i, "Gurmukhi"); }
    for i in 0x0A80..=0x0AFF { blocks.insert(i, "Gujarati"); }
    for i in 0x0B00..=0x0B7F { blocks.insert(i, "Oriya"); }
    for i in 0x0B80..=0x0BFF { blocks.insert(i, "Tamil"); }
    for i in 0x0C00..=0x0C7F { blocks.insert(i, "Telugu"); }
    for i in 0x0C80..=0x0CFF { blocks.insert(i, "Kannada"); }
    for i in 0x0D00..=0x0D7F { blocks.insert(i, "Malayalam"); }
    for i in 0x0D80..=0x0DFF { blocks.insert(i, "Sinhala"); }
    for i in 0x0E00..=0x0E7F { blocks.insert(i, "Thai"); }
    for i in 0x0E80..=0x0EFF { blocks.insert(i, "Lao"); }
    for i in 0x0F00..=0x0FFF { blocks.insert(i, "Tibetan"); }
    for i in 0x1000..=0x109F { blocks.insert(i, "Myanmar"); }
    for i in 0x10A0..=0x10FF { blocks.insert(i, "Georgian"); }
    for i in 0x1100..=0x11FF { blocks.insert(i, "Hangul Jamo"); }
    for i in 0x1200..=0x137F { blocks.insert(i, "Ethiopic"); }
    for i in 0x1380..=0x139F { blocks.insert(i, "Ethiopic Supplement"); }
    for i in 0x13A0..=0x13FF { blocks.insert(i, "Cherokee"); }
    for i in 0x1400..=0x167F { blocks.insert(i, "Unified Canadian Aboriginal Syllabics"); }
    for i in 0x1680..=0x169F { blocks.insert(i, "Ogham"); }
    for i in 0x16A0..=0x16FF { blocks.insert(i, "Runic"); }
    for i in 0x1700..=0x171F { blocks.insert(i, "Tagalog"); }
    for i in 0x1720..=0x173F { blocks.insert(i, "Hanunoo"); }
    for i in 0x1740..=0x175F { blocks.insert(i, "Buhid"); }
    for i in 0x1760..=0x177F { blocks.insert(i, "Tagbanwa"); }
    for i in 0x1780..=0x17FF { blocks.insert(i, "Khmer"); }
    for i in 0x1800..=0x18AF { blocks.insert(i, "Mongolian"); }
    for i in 0x18B0..=0x18FF { blocks.insert(i, "Unified Canadian Aboriginal Syllabics Extended"); }
    for i in 0x1900..=0x194F { blocks.insert(i, "Limbu"); }
    for i in 0x1950..=0x197F { blocks.insert(i, "Tai Le"); }
    for i in 0x1980..=0x19DF { blocks.insert(i, "New Tai Lue"); }
    for i in 0x19E0..=0x19FF { blocks.insert(i, "Khmer Symbols"); }
    for i in 0x1A00..=0x1A1F { blocks.insert(i, "Buginese"); }
    for i in 0x1A20..=0x1AAF { blocks.insert(i, "Tai Tham"); }
    for i in 0x1AB0..=0x1AFF { blocks.insert(i, "Combining Diacritical Marks Extended"); }
    for i in 0x1B00..=0x1B7F { blocks.insert(i, "Balinese"); }
    for i in 0x1B80..=0x1BBF { blocks.insert(i, "Sundanese"); }
    for i in 0x1BC0..=0x1BFF { blocks.insert(i, "Batak"); }
    for i in 0x1C00..=0x1C4F { blocks.insert(i, "Lepcha"); }
    for i in 0x1C50..=0x1C7F { blocks.insert(i, "Ol Chiki"); }
    for i in 0x1C80..=0x1C8F { blocks.insert(i, "Cyrillic Extended-C"); }
    for i in 0x1C90..=0x1CBF { blocks.insert(i, "Georgian Extended"); }
    for i in 0x1CC0..=0x1CCF { blocks.insert(i, "Sundanese Supplement"); }
    for i in 0x1CD0..=0x1CFF { blocks.insert(i, "Vedic Extensions"); }
    for i in 0x1D00..=0x1D7F { blocks.insert(i, "Phonetic Extensions"); }
    for i in 0x1D80..=0x1DBF { blocks.insert(i, "Phonetic Extensions Supplement"); }
    for i in 0x1DC0..=0x1DFF { blocks.insert(i, "Combining Diacritical Marks Supplement"); }
    for i in 0x1E00..=0x1EFF { blocks.insert(i, "Latin Extended Additional"); }
    for i in 0x1F00..=0x1FFF { blocks.insert(i, "Greek Extended"); }
    for i in 0x2000..=0x206F { blocks.insert(i, "General Punctuation"); }
    for i in 0x2070..=0x209F { blocks.insert(i, "Superscripts and Subscripts"); }
    for i in 0x20A0..=0x20CF { blocks.insert(i, "Currency Symbols"); }
    for i in 0x20D0..=0x20FF { blocks.insert(i, "Combining Diacritical Marks for Symbols"); }
    for i in 0x2100..=0x214F { blocks.insert(i, "Letterlike Symbols"); }
    for i in 0x2150..=0x218F { blocks.insert(i, "Number Forms"); }
    for i in 0x2190..=0x21FF { blocks.insert(i, "Arrows"); }
    for i in 0x2200..=0x22FF { blocks.insert(i, "Mathematical Operators"); }
    for i in 0x2300..=0x23FF { blocks.insert(i, "Miscellaneous Technical"); }
    for i in 0x2400..=0x243F { blocks.insert(i, "Control Pictures"); }
    for i in 0x2440..=0x245F { blocks.insert(i, "Optical Character Recognition"); }
    for i in 0x2460..=0x24FF { blocks.insert(i, "Enclosed Alphanumerics"); }
    for i in 0x2500..=0x257F { blocks.insert(i, "Box Drawing"); }
    for i in 0x2580..=0x259F { blocks.insert(i, "Block Elements"); }
    for i in 0x25A0..=0x25FF { blocks.insert(i, "Geometric Shapes"); }
    for i in 0x2600..=0x26FF { blocks.insert(i, "Miscellaneous Symbols"); }
    for i in 0x2700..=0x27BF { blocks.insert(i, "Dingbats"); }
    for i in 0x27C0..=0x27EF { blocks.insert(i, "Miscellaneous Mathematical Symbols-A"); }
    for i in 0x27F0..=0x27FF { blocks.insert(i, "Supplemental Arrows-A"); }
    for i in 0x2800..=0x28FF { blocks.insert(i, "Braille Patterns"); }
    for i in 0x2900..=0x297F { blocks.insert(i, "Supplemental Arrows-B"); }
    for i in 0x2980..=0x29FF { blocks.insert(i, "Miscellaneous Mathematical Symbols-B"); }
    for i in 0x2A00..=0x2AFF { blocks.insert(i, "Supplemental Mathematical Operators"); }
    for i in 0x2B00..=0x2BFF { blocks.insert(i, "Miscellaneous Symbols and Arrows"); }
    for i in 0x2C00..=0x2C5F { blocks.insert(i, "Glagolitic"); }
    for i in 0x2C60..=0x2C7F { blocks.insert(i, "Latin Extended-C"); }
    for i in 0x2C80..=0x2CFF { blocks.insert(i, "Coptic"); }
    for i in 0x2D00..=0x2D2F { blocks.insert(i, "Georgian Supplement"); }
    for i in 0x2D30..=0x2D7F { blocks.insert(i, "Tifinagh"); }
    for i in 0x2D80..=0x2DDF { blocks.insert(i, "Ethiopic Extended"); }
    for i in 0x2DE0..=0x2DFF { blocks.insert(i, "Cyrillic Extended-A"); }
    for i in 0x2E00..=0x2E7F { blocks.insert(i, "Supplemental Punctuation"); }
    for i in 0x2E80..=0x2EFF { blocks.insert(i, "CJK Radicals Supplement"); }
    for i in 0x2F00..=0x2FDF { blocks.insert(i, "Kangxi Radicals"); }
    for i in 0x2FF0..=0x2FFF { blocks.insert(i, "Ideographic Description Characters"); }
    for i in 0x3000..=0x303F { blocks.insert(i, "CJK Symbols and Punctuation"); }
    for i in 0x3040..=0x309F { blocks.insert(i, "Hiragana"); }
    for i in 0x30A0..=0x30FF { blocks.insert(i, "Katakana"); }
    for i in 0x3100..=0x312F { blocks.insert(i, "Bopomofo"); }
    for i in 0x3130..=0x318F { blocks.insert(i, "Hangul Compatibility Jamo"); }
    for i in 0x3190..=0x319F { blocks.insert(i, "Kanbun"); }
    for i in 0x31A0..=0x31BF { blocks.insert(i, "Bopomofo Extended"); }
    for i in 0x31C0..=0x31EF { blocks.insert(i, "CJK Strokes"); }
    for i in 0x31F0..=0x31FF { blocks.insert(i, "Katakana Phonetic Extensions"); }
    for i in 0x3200..=0x32FF { blocks.insert(i, "Enclosed CJK Letters and Months"); }
    for i in 0x3300..=0x33FF { blocks.insert(i, "CJK Compatibility"); }
    for i in 0x3400..=0x4DBF { blocks.insert(i, "CJK Unified Ideographs Extension A"); }
    for i in 0x4DC0..=0x4DFF { blocks.insert(i, "Yijing Hexagram Symbols"); }
    for i in 0x4E00..=0x9FFF { blocks.insert(i, "CJK Unified Ideographs"); }
    for i in 0xA000..=0xA48F { blocks.insert(i, "Yi Syllables"); }
    for i in 0xA490..=0xA4CF { blocks.insert(i, "Yi Radicals"); }
    for i in 0xA4D0..=0xA4FF { blocks.insert(i, "Lisu"); }
    for i in 0xA500..=0xA63F { blocks.insert(i, "Vai"); }
    for i in 0xA640..=0xA69F { blocks.insert(i, "Cyrillic Extended-B"); }
    for i in 0xA6A0..=0xA6FF { blocks.insert(i, "Bamum"); }
    for i in 0xA700..=0xA71F { blocks.insert(i, "Modifier Tone Letters"); }
    for i in 0xA720..=0xA7FF { blocks.insert(i, "Latin Extended-D"); }
    for i in 0xA800..=0xA82F { blocks.insert(i, "Syloti Nagri"); }
    for i in 0xA830..=0xA83F { blocks.insert(i, "Common Indic Number Forms"); }
    for i in 0xA840..=0xA87F { blocks.insert(i, "Phags-pa"); }
    for i in 0xA880..=0xA8DF { blocks.insert(i, "Saurashtra"); }
    for i in 0xA8E0..=0xA8FF { blocks.insert(i, "Devanagari Extended"); }
    for i in 0xA900..=0xA92F { blocks.insert(i, "Kayah Li"); }
    for i in 0xA930..=0xA95F { blocks.insert(i, "Rejang"); }
    for i in 0xA960..=0xA97F { blocks.insert(i, "Hangul Jamo Extended-A"); }
    for i in 0xA980..=0xA9DF { blocks.insert(i, "Javanese"); }
    for i in 0xA9E0..=0xA9FF { blocks.insert(i, "Myanmar Extended-B"); }
    for i in 0xAA00..=0xAA5F { blocks.insert(i, "Cham"); }
    for i in 0xAA60..=0xAA7F { blocks.insert(i, "Myanmar Extended-A"); }
    for i in 0xAA80..=0xAADF { blocks.insert(i, "Tai Viet"); }
    for i in 0xAAE0..=0xAAFF { blocks.insert(i, "Meetei Mayek Extensions"); }
    for i in 0xAB00..=0xAB2F { blocks.insert(i, "Ethiopic Extended-A"); }
    for i in 0xAB30..=0xAB6F { blocks.insert(i, "Latin Extended-E"); }
    for i in 0xAB70..=0xABBF { blocks.insert(i, "Cherokee Supplement"); }
    for i in 0xABC0..=0xABFF { blocks.insert(i, "Meetei Mayek"); }
    for i in 0xAC00..=0xD7AF { blocks.insert(i, "Hangul Syllables"); }
    for i in 0xD7B0..=0xD7FF { blocks.insert(i, "Hangul Jamo Extended-B"); }
    for i in 0xD800..=0xDB7F { blocks.insert(i, "High Surrogates"); }
    for i in 0xDB80..=0xDBFF { blocks.insert(i, "High Private Use Surrogates"); }
    for i in 0xDC00..=0xDFFF { blocks.insert(i, "Low Surrogates"); }
    for i in 0xE000..=0xF8FF { blocks.insert(i, "Private Use Area"); }
    for i in 0xF900..=0xFAFF { blocks.insert(i, "CJK Compatibility Ideographs"); }
    for i in 0xFB00..=0xFB4F { blocks.insert(i, "Alphabetic Presentation Forms"); }
    for i in 0xFB50..=0xFDFF { blocks.insert(i, "Arabic Presentation Forms-A"); }
    for i in 0xFE00..=0xFE0F { blocks.insert(i, "Variation Selectors"); }
    for i in 0xFE10..=0xFE1F { blocks.insert(i, "Vertical Forms"); }
    for i in 0xFE20..=0xFE2F { blocks.insert(i, "Combining Half Marks"); }
    for i in 0xFE30..=0xFE4F { blocks.insert(i, "CJK Compatibility Forms"); }
    for i in 0xFE50..=0xFE6F { blocks.insert(i, "Small Form Variants"); }
    for i in 0xFE70..=0xFEFF { blocks.insert(i, "Arabic Presentation Forms-B"); }
    for i in 0xFF00..=0xFFEF { blocks.insert(i, "Halfwidth and Fullwidth Forms"); }
    for i in 0xFFF0..=0xFFFF { blocks.insert(i, "Specials"); }
    
    // Handle supplementary planes (simplified)
    for i in 0x10000..=0x1007F { blocks.insert(i, "Linear B Syllabary"); }
    for i in 0x10080..=0x100FF { blocks.insert(i, "Linear B Ideograms"); }
    for i in 0x10100..=0x1013F { blocks.insert(i, "Aegean Numbers"); }
    for i in 0x10140..=0x1018F { blocks.insert(i, "Ancient Greek Numbers"); }
    for i in 0x10190..=0x101CF { blocks.insert(i, "Ancient Symbols"); }
    for i in 0x101D0..=0x101FF { blocks.insert(i, "Phaistos Disc"); }
    for i in 0x10280..=0x1029F { blocks.insert(i, "Lycian"); }
    for i in 0x102A0..=0x102DF { blocks.insert(i, "Carian"); }
    for i in 0x102E0..=0x102FF { blocks.insert(i, "Coptic Epact Numbers"); }
    for i in 0x10300..=0x1032F { blocks.insert(i, "Old Italic"); }
    for i in 0x10330..=0x1034F { blocks.insert(i, "Gothic"); }
    for i in 0x10350..=0x1037F { blocks.insert(i, "Old Permic"); }
    for i in 0x10380..=0x1039F { blocks.insert(i, "Ugaritic"); }
    for i in 0x103A0..=0x103DF { blocks.insert(i, "Old Persian"); }
    for i in 0x10400..=0x1044F { blocks.insert(i, "Deseret"); }
    for i in 0x10450..=0x1047F { blocks.insert(i, "Shavian"); }
    for i in 0x10480..=0x104AF { blocks.insert(i, "Osmanya"); }
    for i in 0x104B0..=0x104FF { blocks.insert(i, "Osage"); }
    for i in 0x10500..=0x1052F { blocks.insert(i, "Elbasan"); }
    for i in 0x10530..=0x1056F { blocks.insert(i, "Caucasian Albanian"); }
    for i in 0x10600..=0x1077F { blocks.insert(i, "Linear A"); }
    for i in 0x10800..=0x1083F { blocks.insert(i, "Cypriot Syllabary"); }
    for i in 0x10840..=0x1085F { blocks.insert(i, "Imperial Aramaic"); }
    for i in 0x10860..=0x1087F { blocks.insert(i, "Palmyrene"); }
    for i in 0x10880..=0x108AF { blocks.insert(i, "Nabataean"); }
    for i in 0x108E0..=0x108FF { blocks.insert(i, "Hatran"); }
    for i in 0x10900..=0x1091F { blocks.insert(i, "Phoenician"); }
    for i in 0x10920..=0x1093F { blocks.insert(i, "Lydian"); }
    for i in 0x10980..=0x1099F { blocks.insert(i, "Meroitic Hieroglyphs"); }
    for i in 0x109A0..=0x109FF { blocks.insert(i, "Meroitic Cursive"); }
    for i in 0x10A00..=0x10A5F { blocks.insert(i, "Kharoshthi"); }
    for i in 0x10A60..=0x10A7F { blocks.insert(i, "Old South Arabian"); }
    for i in 0x10A80..=0x10A9F { blocks.insert(i, "Old North Arabian"); }
    for i in 0x10AC0..=0x10AFF { blocks.insert(i, "Manichaean"); }
    for i in 0x10B00..=0x10B3F { blocks.insert(i, "Avestan"); }
    for i in 0x10B40..=0x10B5F { blocks.insert(i, "Inscriptional Parthian"); }
    for i in 0x10B60..=0x10B7F { blocks.insert(i, "Inscriptional Pahlavi"); }
    for i in 0x10B80..=0x10BAF { blocks.insert(i, "Psalter Pahlavi"); }
    for i in 0x10C00..=0x10C4F { blocks.insert(i, "Old Turkic"); }
    for i in 0x10C80..=0x10CFF { blocks.insert(i, "Old Hungarian"); }
    for i in 0x10D00..=0x10D3F { blocks.insert(i, "Hanifi Rohingya"); }
    for i in 0x10E60..=0x10E7F { blocks.insert(i, "Rumi Numeral Symbols"); }
    for i in 0x10E80..=0x10EBF { blocks.insert(i, "Yezidi"); }
    for i in 0x10F00..=0x10F2F { blocks.insert(i, "Old Sogdian"); }
    for i in 0x10F30..=0x10F6F { blocks.insert(i, "Sogdian"); }
    for i in 0x10F70..=0x10FAF { blocks.insert(i, "Old Uyghur"); }
    for i in 0x10FB0..=0x10FDF { blocks.insert(i, "Chorasmian"); }
    for i in 0x10FE0..=0x10FFF { blocks.insert(i, "Elymaic"); }
    for i in 0x11000..=0x1107F { blocks.insert(i, "Brahmi"); }
    for i in 0x11080..=0x110CF { blocks.insert(i, "Kaithi"); }
    for i in 0x110D0..=0x110FF { blocks.insert(i, "Sora Sompeng"); }
    for i in 0x11100..=0x1114F { blocks.insert(i, "Chakma"); }
    for i in 0x11150..=0x1117F { blocks.insert(i, "Mahajani"); }
    for i in 0x11180..=0x111DF { blocks.insert(i, "Sharada"); }
    for i in 0x111E0..=0x111FF { blocks.insert(i, "Sinhala Archaic Numbers"); }
    for i in 0x11200..=0x1124F { blocks.insert(i, "Khojki"); }
    for i in 0x11280..=0x112AF { blocks.insert(i, "Multani"); }
    for i in 0x112B0..=0x112FF { blocks.insert(i, "Khudawadi"); }
    for i in 0x11300..=0x1137F { blocks.insert(i, "Grantha"); }
    for i in 0x11400..=0x1147F { blocks.insert(i, "Newa"); }
    for i in 0x11480..=0x114DF { blocks.insert(i, "Tirhuta"); }
    for i in 0x11580..=0x115FF { blocks.insert(i, "Siddham"); }
    for i in 0x11600..=0x1165F { blocks.insert(i, "Modi"); }
    for i in 0x11660..=0x1167F { blocks.insert(i, "Mongolian Supplement"); }
    for i in 0x11680..=0x116CF { blocks.insert(i, "Takri"); }
    for i in 0x11700..=0x1173F { blocks.insert(i, "Ahom"); }
    for i in 0x11800..=0x1184F { blocks.insert(i, "Dogra"); }
    for i in 0x118A0..=0x118FF { blocks.insert(i, "Warang Citi"); }
    for i in 0x11900..=0x1195F { blocks.insert(i, "Dives Akuru"); }
    for i in 0x119A0..=0x119FF { blocks.insert(i, "Nandinagari"); }
    for i in 0x11A00..=0x11A4F { blocks.insert(i, "Zanabazar Square"); }
    for i in 0x11A50..=0x11AAF { blocks.insert(i, "Soyombo"); }
    for i in 0x11AB0..=0x11ABF { blocks.insert(i, "Unified Canadian Aboriginal Syllabics Extended-A"); }
    for i in 0x11AC0..=0x11AFF { blocks.insert(i, "Pau Cin Hau"); }
    for i in 0x11C00..=0x11C6F { blocks.insert(i, "Bhaiksuki"); }
    for i in 0x11C70..=0x11CBF { blocks.insert(i, "Marchen"); }
    for i in 0x11D00..=0x11D5F { blocks.insert(i, "Masaram Gondi"); }
    for i in 0x11D60..=0x11DAF { blocks.insert(i, "Gunjala Gondi"); }
    for i in 0x11EE0..=0x11EFF { blocks.insert(i, "Makasar"); }
    for i in 0x11FB0..=0x11FBF { blocks.insert(i, "Lisu Supplement"); }
    for i in 0x11FC0..=0x11FFF { blocks.insert(i, "Tamil Supplement"); }
    for i in 0x12000..=0x123FF { blocks.insert(i, "Cuneiform"); }
    for i in 0x12400..=0x1247F { blocks.insert(i, "Cuneiform Numbers and Punctuation"); }
    for i in 0x12480..=0x1254F { blocks.insert(i, "Early Dynastic Cuneiform"); }
    for i in 0x13000..=0x1342F { blocks.insert(i, "Egyptian Hieroglyphs"); }
    for i in 0x13430..=0x1343F { blocks.insert(i, "Egyptian Hieroglyph Format Controls"); }
    for i in 0x14400..=0x1467F { blocks.insert(i, "Anatolian Hieroglyphs"); }
    for i in 0x16800..=0x16A3F { blocks.insert(i, "Bamum Supplement"); }
    for i in 0x16A40..=0x16A6F { blocks.insert(i, "Mro"); }
    for i in 0x16AD0..=0x16AFF { blocks.insert(i, "Bassa Vah"); }
    for i in 0x16B00..=0x16B8F { blocks.insert(i, "Pahawh Hmong"); }
    for i in 0x16E40..=0x16E9F { blocks.insert(i, "Medefaidrin"); }
    for i in 0x16F00..=0x16F9F { blocks.insert(i, "Miao"); }
    for i in 0x16FE0..=0x16FFF { blocks.insert(i, "Ideographic Symbols and Punctuation"); }
    for i in 0x17000..=0x187FF { blocks.insert(i, "Tangut"); }
    for i in 0x18800..=0x18AFF { blocks.insert(i, "Tangut Components"); }
    for i in 0x18B00..=0x18CFF { blocks.insert(i, "Khitan Small Script"); }
    for i in 0x18D00..=0x18D8F { blocks.insert(i, "Tangut Supplement"); }
    for i in 0x1B000..=0x1B0FF { blocks.insert(i, "Kana Supplement"); }
    for i in 0x1B100..=0x1B12F { blocks.insert(i, "Kana Extended-A"); }
    for i in 0x1B130..=0x1B16F { blocks.insert(i, "Small Kana Extension"); }
    for i in 0x1B170..=0x1B2FF { blocks.insert(i, "Nushu"); }
    for i in 0x1BC00..=0x1BC9F { blocks.insert(i, "Duployan"); }
    for i in 0x1BCA0..=0x1BCAF { blocks.insert(i, "Shorthand Format Controls"); }
    for i in 0x1D000..=0x1D0FF { blocks.insert(i, "Byzantine Musical Symbols"); }
    for i in 0x1D100..=0x1D1FF { blocks.insert(i, "Musical Symbols"); }
    for i in 0x1D200..=0x1D24F { blocks.insert(i, "Ancient Greek Musical Notation"); }
    for i in 0x1D2E0..=0x1D2FF { blocks.insert(i, "Mayan Numerals"); }
    for i in 0x1D300..=0x1D35F { blocks.insert(i, "Tai Xuan Jing Symbols"); }
    for i in 0x1D360..=0x1D37F { blocks.insert(i, "Counting Rod Numerals"); }
    for i in 0x1D400..=0x1D7FF { blocks.insert(i, "Mathematical Alphanumeric Symbols"); }
    for i in 0x1D800..=0x1DAAF { blocks.insert(i, "Sutton SignWriting"); }
    for i in 0x1E000..=0x1E02F { blocks.insert(i, "Glagolitic Supplement"); }
    for i in 0x1E100..=0x1E14F { blocks.insert(i, "Nyiakeng Puachue Hmong"); }
    for i in 0x1E2C0..=0x1E2FF { blocks.insert(i, "Wancho"); }
    for i in 0x1E800..=0x1E8DF { blocks.insert(i, "Mende Kikakui"); }
    for i in 0x1E900..=0x1E95F { blocks.insert(i, "Adlam"); }
    for i in 0x1EC70..=0x1ECBF { blocks.insert(i, "Indic Siyaq Numbers"); }
    for i in 0x1ED00..=0x1ED4F { blocks.insert(i, "Ottoman Siyaq Numbers"); }
    for i in 0x1EE00..=0x1EEFF { blocks.insert(i, "Arabic Mathematical Alphabetic Symbols"); }
    for i in 0x1F000..=0x1F02F { blocks.insert(i, "Mahjong Tiles"); }
    for i in 0x1F030..=0x1F09F { blocks.insert(i, "Domino Tiles"); }
    for i in 0x1F0A0..=0x1F0FF { blocks.insert(i, "Playing Cards"); }
    for i in 0x1F100..=0x1F1FF { blocks.insert(i, "Enclosed Alphanumeric Supplement"); }
    for i in 0x1F200..=0x1F2FF { blocks.insert(i, "Enclosed CJK Letters and Months Supplement"); }
    for i in 0x1F300..=0x1F5FF { blocks.insert(i, "Miscellaneous Symbols and Pictographs"); }
    for i in 0x1F600..=0x1F64F { blocks.insert(i, "Emoticons"); }
    for i in 0x1F650..=0x1F67F { blocks.insert(i, "Ornamental Dingbats"); }
    for i in 0x1F680..=0x1F6FF { blocks.insert(i, "Transport and Map Symbols"); }
    for i in 0x1F700..=0x1F77F { blocks.insert(i, "Alchemical Symbols"); }
    for i in 0x1F780..=0x1F7FF { blocks.insert(i, "Geometric Shapes Extended"); }
    for i in 0x1F800..=0x1F8FF { blocks.insert(i, "Supplemental Arrows-C"); }
    for i in 0x1F900..=0x1F9FF { blocks.insert(i, "Supplemental Symbols and Pictographs"); }
    for i in 0x1FA00..=0x1FA6F { blocks.insert(i, "Chess Symbols"); }
    for i in 0x1FA70..=0x1FAFF { blocks.insert(i, "Symbols and Pictographs Extended-A"); }
    for i in 0x1FB00..=0x1FBFF { blocks.insert(i, "Symbols for Legacy Computing"); }
    for i in 0x20000..=0x2A6DF { blocks.insert(i, "CJK Unified Ideographs Extension B"); }
    for i in 0x2A700..=0x2B73F { blocks.insert(i, "CJK Unified Ideographs Extension C"); }
    for i in 0x2B740..=0x2B81F { blocks.insert(i, "CJK Unified Ideographs Extension D"); }
    for i in 0x2B820..=0x2CEAF { blocks.insert(i, "CJK Unified Ideographs Extension E"); }
    for i in 0x2CEB0..=0x2EBEF { blocks.insert(i, "CJK Unified Ideographs Extension F"); }
    for i in 0x2F800..=0x2FA1F { blocks.insert(i, "CJK Compatibility Ideographs Supplement"); }
    for i in 0x30000..=0x3134F { blocks.insert(i, "CJK Unified Ideographs Extension G"); }
    for i in 0xE0000..=0xE007F { blocks.insert(i, "Tags"); }
    for i in 0xE0100..=0xE01EF { blocks.insert(i, "Variation Selectors Supplement"); }
    for i in 0xF0000..=0xFFFFF { blocks.insert(i, "Supplementary Private Use Area-A"); }
    for i in 0x100000..=0x10FFFF { blocks.insert(i, "Supplementary Private Use Area-B"); }
    
    blocks
});

/// Character category database (simplified)
static CATEGORIES: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut categories = HashMap::new();
    
    // Letter categories
    for i in 0x0041..=0x005A { categories.insert(i, "Lu"); } // Uppercase letters
    for i in 0x0061..=0x007A { categories.insert(i, "Ll"); } // Lowercase letters
    
    // Number categories
    for i in 0x0030..=0x0039 { categories.insert(i, "Nd"); } // Decimal numbers
    
    // Punctuation categories
    categories.insert(0x0021, "Po"); // Exclamation mark
    categories.insert(0x0022, "Po"); // Quotation mark
    categories.insert(0x0023, "Po"); // Number sign
    categories.insert(0x0025, "Po"); // Percent sign
    categories.insert(0x0026, "Po"); // Ampersand
    categories.insert(0x0027, "Po"); // Apostrophe
    categories.insert(0x0028, "Ps"); // Left parenthesis
    categories.insert(0x0029, "Pe"); // Right parenthesis
    categories.insert(0x002A, "Po"); // Asterisk
    categories.insert(0x002C, "Po"); // Comma
    categories.insert(0x002D, "Pd"); // Hyphen-minus
    categories.insert(0x002E, "Po"); // Full stop
    categories.insert(0x002F, "Po"); // Solidus
    categories.insert(0x003A, "Po"); // Colon
    categories.insert(0x003B, "Po"); // Semicolon
    categories.insert(0x003F, "Po"); // Question mark
    categories.insert(0x0040, "Po"); // Commercial at
    categories.insert(0x005B, "Ps"); // Left square bracket
    categories.insert(0x005C, "Po"); // Reverse solidus
    categories.insert(0x005D, "Pe"); // Right square bracket
    categories.insert(0x005F, "Pc"); // Low line
    categories.insert(0x007B, "Ps"); // Left curly bracket
    categories.insert(0x007D, "Pe"); // Right curly bracket
    
    // Symbol categories
    categories.insert(0x0024, "Sc"); // Dollar sign
    categories.insert(0x002B, "Sm"); // Plus sign
    categories.insert(0x003C, "Sm"); // Less-than sign
    categories.insert(0x003D, "Sm"); // Equals sign
    categories.insert(0x003E, "Sm"); // Greater-than sign
    categories.insert(0x005E, "Sk"); // Circumflex accent
    categories.insert(0x0060, "Sk"); // Grave accent
    categories.insert(0x007C, "Sm"); // Vertical line
    categories.insert(0x007E, "Sm"); // Tilde
    
    // Space category
    categories.insert(0x0020, "Zs"); // Space
    categories.insert(0x00A0, "Zs"); // No-break space
    
    // Control categories
    for i in 0x0000..=0x001F { categories.insert(i, "Cc"); } // C0 controls
    categories.insert(0x007F, "Cc"); // Delete
    for i in 0x0080..=0x009F { categories.insert(i, "Cc"); } // C1 controls
    
    // Format category
    categories.insert(0x00AD, "Cf"); // Soft hyphen
    
    // Mark categories (combining diacritical marks)
    for i in 0x0300..=0x036F { categories.insert(i, "Mn"); } // Non-spacing marks
    
    // Extended Latin
    for i in 0x00C0..=0x00D6 { categories.insert(i, "Lu"); } // Latin capital letters with diacritics
    for i in 0x00D8..=0x00DE { categories.insert(i, "Lu"); } // More Latin capital letters
    for i in 0x00DF..=0x00F6 { categories.insert(i, "Ll"); } // Latin small letters with diacritics
    for i in 0x00F8..=0x00FF { categories.insert(i, "Ll"); } // More Latin small letters
    
    // Greek
    for i in 0x0391..=0x03A1 { categories.insert(i, "Lu"); } // Greek capital letters
    for i in 0x03A3..=0x03AB { categories.insert(i, "Lu"); } // More Greek capital letters
    for i in 0x03B1..=0x03C9 { categories.insert(i, "Ll"); } // Greek small letters
    
    // Cyrillic
    for i in 0x0410..=0x042F { categories.insert(i, "Lu"); } // Cyrillic capital letters
    for i in 0x0430..=0x044F { categories.insert(i, "Ll"); } // Cyrillic small letters
    
    // Hebrew
    for i in 0x05D0..=0x05EA { categories.insert(i, "Lo"); } // Hebrew letters
    
    // Arabic
    for i in 0x0627..=0x063A { categories.insert(i, "Lo"); } // Arabic letters
    for i in 0x0641..=0x064A { categories.insert(i, "Lo"); } // More Arabic letters
    
    // CJK
    for i in 0x4E00..=0x9FFF { categories.insert(i, "Lo"); } // CJK Unified Ideographs
    for i in 0x3400..=0x4DBF { categories.insert(i, "Lo"); } // CJK Extension A
    
    // Hiragana
    for i in 0x3041..=0x3096 { categories.insert(i, "Lo"); } // Hiragana letters
    
    // Katakana
    for i in 0x30A1..=0x30FA { categories.insert(i, "Lo"); } // Katakana letters
    
    // Hangul
    for i in 0xAC00..=0xD7AF { categories.insert(i, "Lo"); } // Hangul syllables
    
    categories
});

/// Character classification functions

/// Test if a character is a letter
pub fn is_letter(ch: char) -> bool {
    LETTER.contains(ch)
}

/// Test if a character is a digit
pub fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit() || DECIMAL_NUMBER.contains(ch)
}

/// Test if a character is a number (including digits, superscripts, fractions)
pub fn is_number(ch: char) -> bool {
    NUMBER.contains(ch)
}

/// Test if a character is whitespace
pub fn is_space(ch: char) -> bool {
    SPACE.contains(ch) || ch.is_whitespace()
}

/// Test if a character is punctuation
pub fn is_punct(ch: char) -> bool {
    PUNCT.contains(ch)
}

/// Test if a character is a symbol
pub fn is_symbol(ch: char) -> bool {
    SYMBOL.contains(ch)
}

/// Test if a character is a mark (combining character)
pub fn is_mark(ch: char) -> bool {
    MARK.contains(ch)
}

/// Test if a character is a control character
pub fn is_control(ch: char) -> bool {
    CONTROL.contains(ch)
}

/// Test if a character is graphic (printable, excluding spaces)
pub fn is_graphic(ch: char) -> bool {
    !is_space(ch) && !is_control(ch) && !is_format(ch) && !is_surrogate(ch) && !is_private_use(ch)
}

/// Test if a character is printable (graphic or space)
pub fn is_print(ch: char) -> bool {
    is_graphic(ch) || is_space(ch)
}

/// Test if a character is uppercase
pub fn is_upper(ch: char) -> bool {
    UPPERCASE_LETTER.contains(ch)
}

/// Test if a character is lowercase
pub fn is_lower(ch: char) -> bool {
    LOWERCASE_LETTER.contains(ch)
}

/// Test if a character is titlecase
pub fn is_title(ch: char) -> bool {
    TITLECASE_LETTER.contains(ch)
}

/// Test if a character is an emoji
pub fn is_emoji(ch: char) -> bool {
    EMOJI.contains(ch)
}

/// Test if a character is an emoji modifier
pub fn is_emoji_modifier(ch: char) -> bool {
    EMOJI_MODIFIER.contains(ch)
}

/// Test if a character is an emoji component
pub fn is_emoji_component(ch: char) -> bool {
    EMOJI_COMPONENT.contains(ch)
}

/// Test if a character is a currency symbol
pub fn is_currency(ch: char) -> bool {
    CURRENCY_SYMBOL.contains(ch)
}

/// Test if a character is a mathematical symbol
pub fn is_math(ch: char) -> bool {
    MATH_SYMBOL.contains(ch)
}

/// Test if a character is a format character
pub fn is_format(ch: char) -> bool {
    FORMAT.contains(ch)
}

/// Test if a character is in the private use area
pub fn is_private_use(ch: char) -> bool {
    PRIVATE.contains(ch)
}

/// Test if a character is a surrogate
pub fn is_surrogate(ch: char) -> bool {
    SURROGATE.contains(ch)
}

/// Test if a character is ASCII
pub fn is_ascii(ch: char) -> bool {
    ch.is_ascii()
}

/// Character conversion functions

/// Convert a character to uppercase
pub fn to_upper(ch: char) -> char {
    // Use Rust's built-in conversion for now
    // In a production implementation, this would use Unicode case mapping tables
    ch.to_uppercase().next().unwrap_or(ch)
}

/// Convert a character to lowercase
pub fn to_lower(ch: char) -> char {
    // Use Rust's built-in conversion for now
    // In a production implementation, this would use Unicode case mapping tables
    ch.to_lowercase().next().unwrap_or(ch)
}

/// Convert a character to titlecase
pub fn to_title(ch: char) -> char {
    // For most characters, titlecase is the same as uppercase
    // In a production implementation, this would handle special titlecase characters
    to_upper(ch)
}

/// Convert a character to ASCII equivalent if possible
pub fn to_ascii(ch: char) -> char {
    if ch.is_ascii() {
        ch
    } else {
        // Simple conversion for common characters
        match ch {
            'à'..='ä' | 'À'..='Ä' => if ch.is_lowercase() { 'a' } else { 'A' },
            'è'..='ë' | 'È'..='Ë' => if ch.is_lowercase() { 'e' } else { 'E' },
            'ì'..='ï' | 'Ì'..='Ï' => if ch.is_lowercase() { 'i' } else { 'I' },
            'ò'..='ö' | 'Ò'..='Ö' => if ch.is_lowercase() { 'o' } else { 'O' },
            'ù'..='ü' | 'Ù'..='Ü' => if ch.is_lowercase() { 'u' } else { 'U' },
            'ç' | 'Ç' => if ch.is_lowercase() { 'c' } else { 'C' },
            'ñ' | 'Ñ' => if ch.is_lowercase() { 'n' } else { 'N' },
            _ => ch, // Return unchanged if no ASCII equivalent
        }
    }
}

/// Simple case folding for case-insensitive comparison
pub fn simple_fold(ch: char) -> char {
    to_lower(ch)
}

/// Range and character set functions

/// Test if a character is in a range table
pub fn is_in_range(ch: char, range_table: &RangeTable) -> bool {
    range_table.contains(ch)
}

/// Test if a character is in any of the given range tables
pub fn is_in_ranges(ch: char, range_tables: &[&RangeTable]) -> bool {
    range_tables.iter().any(|table| table.contains(ch))
}

/// Test if a character is in one of the named range tables
pub fn is_one_of(range_table_names: &[&str], ch: char) -> GlyphGangResult<bool> {
    for name in range_table_names {
        if let Some(table) = get_range_table(name) {
            if table.contains(ch) {
                return Ok(true);
            }
        } else {
            return Err(unicode_error(&format!("Unknown range table: {}", name)));
        }
    }
    Ok(false)
}

/// Character properties and information

/// Get the Unicode name of a character
pub fn get_character_name(ch: char) -> String {
    let code_point = ch as u32;
    
    if let Some(&name) = CHARACTER_NAMES.get(&code_point) {
        name.to_string()
    } else {
        // Generate a generic name for characters not in our database
        if code_point <= 0x001F {
            format!("CONTROL CHARACTER U+{:04X}", code_point)
        } else if code_point >= 0x007F && code_point <= 0x009F {
            format!("CONTROL CHARACTER U+{:04X}", code_point)
        } else if code_point >= 0x4E00 && code_point <= 0x9FFF {
            format!("CJK UNIFIED IDEOGRAPH-{:04X}", code_point)
        } else if code_point >= 0x3400 && code_point <= 0x4DBF {
            format!("CJK UNIFIED IDEOGRAPH EXTENSION A-{:04X}", code_point)
        } else if code_point >= 0xAC00 && code_point <= 0xD7AF {
            format!("HANGUL SYLLABLE U+{:04X}", code_point)
        } else if code_point >= 0xE000 && code_point <= 0xF8FF {
            format!("PRIVATE USE CHARACTER-{:04X}", code_point)
        } else if code_point > 0xFFFF {
            format!("CHARACTER U+{:06X}", code_point)
        } else {
            format!("CHARACTER U+{:04X}", code_point)
        }
    }
}

/// Find a character by its Unicode name (simplified lookup)
pub fn find_character_by_name(name: &str) -> GlyphGangResult<char> {
    let name_upper = name.to_uppercase();
    
    // Search in our character names database
    for (&code_point, &char_name) in CHARACTER_NAMES.iter() {
        if char_name.to_uppercase() == name_upper {
            if let Some(ch) = std::char::from_u32(code_point) {
                return Ok(ch);
            }
        }
    }
    
    // Try to parse as a Unicode code point
    if name_upper.starts_with("U+") {
        let hex_part = &name_upper[2..];
        if let Ok(code_point) = u32::from_str_radix(hex_part, 16) {
            if let Some(ch) = std::char::from_u32(code_point) {
                return Ok(ch);
            }
        }
    }
    
    Err(name_lookup_error(&format!("Character not found: {}", name)))
}

/// Get the Unicode block name for a character
pub fn get_block_name(ch: char) -> String {
    let code_point = ch as u32;
    
    if let Some(&block_name) = BLOCK_NAMES.get(&code_point) {
        block_name.to_string()
    } else {
        // Generate a generic block name for characters not in our database
        if code_point <= 0x007F {
            "Basic Latin".to_string()
        } else if code_point <= 0xFFFF {
            format!("Block U+{:04X}00-U+{:04X}FF", code_point >> 8, code_point >> 8)
        } else {
            format!("Supplementary Block U+{:06X}", code_point & 0xFFF000)
        }
    }
}

/// Get the Unicode general category for a character
pub fn get_category(ch: char) -> String {
    let code_point = ch as u32;
    
    if let Some(&category) = CATEGORIES.get(&code_point) {
        category.to_string()
    } else {
        // Determine category by testing with range tables
        if is_upper(ch) {
            "Lu".to_string() // Letter, uppercase
        } else if is_lower(ch) {
            "Ll".to_string() // Letter, lowercase
        } else if is_letter(ch) {
            "Lo".to_string() // Letter, other
        } else if is_digit(ch) {
            "Nd".to_string() // Number, decimal digit
        } else if is_number(ch) {
            "No".to_string() // Number, other
        } else if is_space(ch) {
            "Zs".to_string() // Separator, space
        } else if is_control(ch) {
            "Cc".to_string() // Other, control
        } else if is_format(ch) {
            "Cf".to_string() // Other, format
        } else if is_mark(ch) {
            "Mn".to_string() // Mark, nonspacing
        } else if is_punct(ch) {
            "Po".to_string() // Punctuation, other
        } else if is_symbol(ch) {
            "So".to_string() // Symbol, other
        } else if is_private_use(ch) {
            "Co".to_string() // Other, private use
        } else if is_surrogate(ch) {
            "Cs".to_string() // Other, surrogate
        } else {
            "Cn".to_string() // Other, not assigned
        }
    }
}

/// Get various Unicode properties for a character
pub fn get_properties(ch: char) -> std::collections::HashMap<String, String> {
    let mut properties = std::collections::HashMap::new();
    
    properties.insert("name".to_string(), get_character_name(ch));
    properties.insert("block".to_string(), get_block_name(ch));
    properties.insert("category".to_string(), get_category(ch));
    properties.insert("code_point".to_string(), get_code_point(ch));
    
    // Boolean properties
    properties.insert("is_letter".to_string(), is_letter(ch).to_string());
    properties.insert("is_digit".to_string(), is_digit(ch).to_string());
    properties.insert("is_number".to_string(), is_number(ch).to_string());
    properties.insert("is_space".to_string(), is_space(ch).to_string());
    properties.insert("is_punct".to_string(), is_punct(ch).to_string());
    properties.insert("is_symbol".to_string(), is_symbol(ch).to_string());
    properties.insert("is_mark".to_string(), is_mark(ch).to_string());
    properties.insert("is_control".to_string(), is_control(ch).to_string());
    properties.insert("is_graphic".to_string(), is_graphic(ch).to_string());
    properties.insert("is_print".to_string(), is_print(ch).to_string());
    properties.insert("is_upper".to_string(), is_upper(ch).to_string());
    properties.insert("is_lower".to_string(), is_lower(ch).to_string());
    properties.insert("is_title".to_string(), is_title(ch).to_string());
    properties.insert("is_ascii".to_string(), is_ascii(ch).to_string());
    properties.insert("is_emoji".to_string(), is_emoji(ch).to_string());
    properties.insert("is_currency".to_string(), is_currency(ch).to_string());
    properties.insert("is_math".to_string(), is_math(ch).to_string());
    properties.insert("is_format".to_string(), is_format(ch).to_string());
    properties.insert("is_private_use".to_string(), is_private_use(ch).to_string());
    properties.insert("is_surrogate".to_string(), is_surrogate(ch).to_string());
    
    // Case conversion
    properties.insert("to_upper".to_string(), to_upper(ch).to_string());
    properties.insert("to_lower".to_string(), to_lower(ch).to_string());
    properties.insert("to_title".to_string(), to_title(ch).to_string());
    
    properties
}

/// Get the Unicode code point representation
pub fn get_code_point(ch: char) -> String {
    let code_point = ch as u32;
    if code_point <= 0xFFFF {
        format!("U+{:04X}", code_point)
    } else {
        format!("U+{:06X}", code_point)
    }
}

/// Get canonical equivalent characters (simplified - returns single character for now)
pub fn get_canonical_equivalent(ch: char) -> Vec<char> {
    // In a full implementation, this would decompose characters into their canonical forms
    // For now, just return the character itself
    vec![ch]
}

