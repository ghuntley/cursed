# CURSED GlyphGang Unicode Library
# Pure CURSED implementation for Unicode character and string processing

# ================================
# Constants
# ================================

# Direction constants for bidirectional text
sus LTR normie = 0
sus RTL normie = 1
sus Mixed normie = 2

# Script constants for script detection
sus ScriptUnknown normie = 0
sus ScriptLatin normie = 1
sus ScriptGreek normie = 2
sus ScriptCyrillic normie = 3
sus ScriptHebrew normie = 4
sus ScriptArabic normie = 5
sus ScriptHan normie = 6
sus ScriptHiragana normie = 7
sus ScriptKatakana normie = 8
sus ScriptHangul normie = 9

# Normalization form constants
sus NFC normie = 0
sus NFD normie = 1
sus NFKC normie = 2
sus NFKD normie = 3

# ================================
# Character Classification Functions
# ================================

slay IsLetter(r rune) lit {
    # Check if character is a letter (A-Z, a-z, and Unicode letters)
    code := normie(r)
    if code >= 65 && code <= 90 {
        damn based # A-Z
    }
    if code >= 97 && code <= 122 {
        damn based # a-z
    }
    if code >= 192 && code <= 214 {
        damn based # À-Ö
    }
    if code >= 216 && code <= 246 {
        damn based # Ø-ö
    }
    if code >= 248 && code <= 255 {
        damn based # ø-ÿ
    }
    damn cap
}

slay IsDigit(r rune) lit {
    # Check if character is a digit (0-9)
    code := normie(r)
    damn (code >= 48 && code <= 57)
}

slay IsNumber(r rune) lit {
    # Check if character is a number (includes digits and numeric symbols)
    damn IsDigit(r)
}

slay IsSpace(r rune) lit {
    # Check if character is whitespace
    code := normie(r)
    if code == 32 {
        damn based # space
    }
    if code == 9 {
        damn based # tab
    }
    if code == 10 {
        damn based # newline
    }
    if code == 13 {
        damn based # carriage return
    }
    if code == 11 {
        damn based # vertical tab
    }
    if code == 12 {
        damn based # form feed
    }
    damn cap
}

slay IsPunct(r rune) lit {
    # Check if character is punctuation
    code := normie(r)
    if code >= 33 && code <= 47 {
        damn based # !"#$%&'()*+,-./
    }
    if code >= 58 && code <= 64 {
        damn based # :;<=>?@
    }
    if code >= 91 && code <= 96 {
        damn based # [\]^_`
    }
    if code >= 123 && code <= 126 {
        damn based # {|}~
    }
    damn cap
}

slay IsSymbol(r rune) lit {
    # Check if character is a symbol
    code := normie(r)
    if code >= 35 && code <= 38 {
        damn based # #$%&
    }
    if code >= 42 && code <= 43 {
        damn based # *+
    }
    if code == 60 || code == 62 {
        damn based # <>
    }
    if code >= 124 && code <= 126 {
        damn based # |}~
    }
    damn cap
}

slay IsMark(r rune) lit {
    # Check if character is a combining mark
    code := normie(r)
    damn (code >= 768 && code <= 879) # Combining Diacritical Marks
}

slay IsControl(r rune) lit {
    # Check if character is a control character
    code := normie(r)
    damn (code >= 0 && code <= 31) || (code >= 127 && code <= 159)
}

slay IsGraphic(r rune) lit {
    # Check if character is graphic (visible)
    damn !IsControl(r) && !IsSpace(r)
}

slay IsPrint(r rune) lit {
    # Check if character is printable
    damn IsGraphic(r) || r == ' '
}

slay IsUpper(r rune) lit {
    # Check if character is uppercase
    code := normie(r)
    damn (code >= 65 && code <= 90)
}

slay IsLower(r rune) lit {
    # Check if character is lowercase
    code := normie(r)
    damn (code >= 97 && code <= 122)
}

slay IsTitle(r rune) lit {
    # Check if character is titlecase
    damn IsUpper(r) # For basic implementation
}

slay IsEmoji(r rune) lit {
    # Check if character is an emoji
    code := normie(r)
    if code >= 0x1F600 && code <= 0x1F64F {
        damn based # Emoticons
    }
    if code >= 0x1F300 && code <= 0x1F5FF {
        damn based # Misc Symbols and Pictographs
    }
    if code >= 0x1F680 && code <= 0x1F6FF {
        damn based # Transport and Map
    }
    if code >= 0x2600 && code <= 0x26FF {
        damn based # Miscellaneous Symbols
    }
    if code >= 0x2700 && code <= 0x27BF {
        damn based # Dingbats
    }
    damn cap
}

slay IsEmojiModifier(r rune) lit {
    # Check if character is an emoji modifier
    code := normie(r)
    damn (code >= 0x1F3FB && code <= 0x1F3FF)
}

slay IsEmojiComponent(r rune) lit {
    # Check if character is an emoji component
    code := normie(r)
    damn (code == 0x200D) # Zero Width Joiner
}

slay IsCurrency(r rune) lit {
    # Check if character is a currency symbol
    code := normie(r)
    if code == 36 {
        damn based # $
    }
    if code == 162 {
        damn based # ¢
    }
    if code == 163 {
        damn based # £
    }
    if code == 164 {
        damn based # ¤
    }
    if code == 165 {
        damn based # ¥
    }
    if code == 8364 {
        damn based # €
    }
    damn cap
}

slay IsMath(r rune) lit {
    # Check if character is a mathematical symbol
    code := normie(r)
    if code >= 43 && code <= 43 {
        damn based # +
    }
    if code == 45 {
        damn based # -
    }
    if code == 42 {
        damn based # *
    }
    if code == 47 {
        damn based # /
    }
    if code == 61 {
        damn based # =
    }
    if code >= 0x2200 && code <= 0x22FF {
        damn based # Mathematical Operators
    }
    damn cap
}

slay IsFormat(r rune) lit {
    # Check if character is a format character
    code := normie(r)
    damn (code >= 0x200B && code <= 0x200F) || (code >= 0x202A && code <= 0x202E)
}

slay IsPrivateUse(r rune) lit {
    # Check if character is in private use area
    code := normie(r)
    damn (code >= 0xE000 && code <= 0xF8FF)
}

slay IsSurrogate(r rune) lit {
    # Check if character is a surrogate
    code := normie(r)
    damn (code >= 0xD800 && code <= 0xDFFF)
}

slay IsASCII(r rune) lit {
    # Check if character is ASCII
    code := normie(r)
    damn (code >= 0 && code <= 127)
}

# ================================
# Character Conversion Functions
# ================================

slay ToUpper(r rune) rune {
    # Convert character to uppercase
    code := normie(r)
    if code >= 97 && code <= 122 {
        damn rune(code - 32)
    }
    damn r
}

slay ToLower(r rune) rune {
    # Convert character to lowercase
    code := normie(r)
    if code >= 65 && code <= 90 {
        damn rune(code + 32)
    }
    damn r
}

slay ToTitle(r rune) rune {
    # Convert character to titlecase
    damn ToUpper(r)
}

slay ToASCII(r rune) rune {
    # Convert character to ASCII equivalent if possible
    code := normie(r)
    if code <= 127 {
        damn r
    }
    # Basic diacritic removal
    if code >= 192 && code <= 198 {
        damn rune(65) # À-Æ → A
    }
    if code >= 224 && code <= 230 {
        damn rune(97) # à-æ → a
    }
    if code >= 200 && code <= 203 {
        damn rune(69) # È-Ë → E
    }
    if code >= 232 && code <= 235 {
        damn rune(101) # è-ë → e
    }
    damn r
}

slay SimpleFold(r rune) rune {
    # Simple case folding
    damn ToLower(r)
}

# ================================
# String Operations
# ================================

slay ToUpperString(s tea) tea {
    # Convert string to uppercase
    result := ""
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        result = result + tea(ToUpper(ch))
    }
    damn result
}

slay ToLowerString(s tea) tea {
    # Convert string to lowercase
    result := ""
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        result = result + tea(ToLower(ch))
    }
    damn result
}

slay ToTitleString(s tea) tea {
    # Convert string to title case
    result := ""
    make_upper := based
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        if make_upper && IsLetter(ch) {
            result = result + tea(ToUpper(ch))
            make_upper = cap
        } else {
            result = result + tea(ToLower(ch))
            if IsSpace(ch) {
                make_upper = based
            }
        }
    }
    damn result
}

slay NormalizeString(s tea, form normie) tea {
    # Basic normalization - just return the string for now
    damn s
}

# ================================
# String Analysis Functions
# ================================

slay RuneCount(s tea) normie {
    # Count runes in string
    damn len(s)
}

slay FirstRune(s tea) (rune, normie) {
    # Get first rune and its byte length
    if len(s) == 0 {
        damn (rune(0), 0)
    }
    damn (rune(s[0]), 1)
}

slay LastRune(s tea) (rune, normie) {
    # Get last rune and its byte length
    if len(s) == 0 {
        damn (rune(0), 0)
    }
    damn (rune(s[len(s)-1]), 1)
}

slay RuneAt(s tea, index normie) rune {
    # Get rune at index
    if index >= 0 && index < len(s) {
        damn rune(s[index])
    }
    damn rune(0)
}

slay StringWidth(s tea) normie {
    # Calculate display width of string
    width := 0
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        if IsASCII(ch) {
            width = width + 1
        } else {
            width = width + 2 # Assume wide characters
        }
    }
    damn width
}

slay TruncateString(s tea, width normie) tea {
    # Truncate string to specified width
    if StringWidth(s) <= width {
        damn s
    }
    result := ""
    current_width := 0
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        char_width := 1
        if !IsASCII(ch) {
            char_width = 2
        }
        if current_width + char_width > width {
            break
        }
        result = result + tea(ch)
        current_width = current_width + char_width
    }
    damn result
}

slay Reverse(s tea) tea {
    # Reverse string
    result := ""
    for i := len(s) - 1; i >= 0; i-- {
        result = result + tea(rune(s[i]))
    }
    damn result
}

# ================================
# Emoji Support Functions
# ================================

slay ContainsEmoji(s tea) lit {
    # Check if string contains any emoji
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        if IsEmoji(ch) {
            damn based
        }
    }
    damn cap
}

slay ReplaceEmojis(s tea, replacement tea) tea {
    # Replace all emojis with replacement string
    result := ""
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        if IsEmoji(ch) {
            result = result + replacement
        } else {
            result = result + tea(ch)
        }
    }
    damn result
}

slay GetEmojiName(emoji tea) tea {
    # Get name of emoji (simplified)
    if len(emoji) == 0 {
        damn "unknown"
    }
    ch := rune(emoji[0])
    code := normie(ch)
    if code == 0x1F60A {
        damn "smiling face with smiling eyes"
    }
    if code == 0x1F602 {
        damn "face with tears of joy"
    }
    if code == 0x1F44D {
        damn "thumbs up"
    }
    if code == 0x1F44E {
        damn "thumbs down"
    }
    damn "unknown emoji"
}

slay FindEmojiByName(name tea) tea {
    # Find emoji by name (simplified)
    if name == "smile" {
        damn "😊"
    }
    if name == "joy" {
        damn "😂"
    }
    if name == "thumbs up" {
        damn "👍"
    }
    if name == "thumbs down" {
        damn "👎"
    }
    damn ""
}

# ================================
# Bidirectional Text Support
# ================================

slay GetDirection(r rune) normie {
    # Get text direction of character
    code := normie(r)
    if code >= 0x0590 && code <= 0x08FF {
        damn RTL # Hebrew, Arabic ranges
    }
    damn LTR # Default to LTR
}

slay GetStringDirection(s tea) normie {
    # Get overall direction of string
    ltr_count := 0
    rtl_count := 0
    
    for i := 0; i < len(s); i++ {
        ch := rune(s[i])
        dir := GetDirection(ch)
        if dir == LTR {
            ltr_count = ltr_count + 1
        } else if dir == RTL {
            rtl_count = rtl_count + 1
        }
    }
    
    if ltr_count > 0 && rtl_count > 0 {
        damn Mixed
    }
    if rtl_count > ltr_count {
        damn RTL
    }
    damn LTR
}

slay IsRTL(s tea) lit {
    # Check if string is right-to-left
    damn GetStringDirection(s) == RTL
}

slay IsLTR(s tea) lit {
    # Check if string is left-to-right
    damn GetStringDirection(s) == LTR
}

slay IsMixed(s tea) lit {
    # Check if string has mixed directionality
    damn GetStringDirection(s) == Mixed
}

# ================================
# Script Detection
# ================================

slay DetectScript(s tea) normie {
    # Detect script of string
    if len(s) == 0 {
        damn ScriptUnknown
    }
    
    ch := rune(s[0])
    code := normie(ch)
    
    if code >= 0x0041 && code <= 0x007A {
        damn ScriptLatin
    }
    if code >= 0x0370 && code <= 0x03FF {
        damn ScriptGreek
    }
    if code >= 0x0400 && code <= 0x04FF {
        damn ScriptCyrillic
    }
    if code >= 0x0590 && code <= 0x05FF {
        damn ScriptHebrew
    }
    if code >= 0x0600 && code <= 0x06FF {
        damn ScriptArabic
    }
    if code >= 0x4E00 && code <= 0x9FFF {
        damn ScriptHan
    }
    if code >= 0x3040 && code <= 0x309F {
        damn ScriptHiragana
    }
    if code >= 0x30A0 && code <= 0x30FF {
        damn ScriptKatakana
    }
    if code >= 0xAC00 && code <= 0xD7AF {
        damn ScriptHangul
    }
    
    damn ScriptUnknown
}

slay GetScriptName(script normie) tea {
    # Get name of script
    if script == ScriptLatin {
        damn "Latin"
    }
    if script == ScriptGreek {
        damn "Greek"
    }
    if script == ScriptCyrillic {
        damn "Cyrillic"
    }
    if script == ScriptHebrew {
        damn "Hebrew"
    }
    if script == ScriptArabic {
        damn "Arabic"
    }
    if script == ScriptHan {
        damn "Han"
    }
    if script == ScriptHiragana {
        damn "Hiragana"
    }
    if script == ScriptKatakana {
        damn "Katakana"
    }
    if script == ScriptHangul {
        damn "Hangul"
    }
    damn "Unknown"
}

# ================================
# Character Width Functions
# ================================

slay GetCharWidth(r rune) normie {
    # Get display width of character
    if IsASCII(r) {
        damn 1
    }
    damn 2 # Assume wide characters
}

slay GetStringWidth(s tea) normie {
    # Get total display width of string
    damn StringWidth(s)
}

slay TruncateWithEllipsis(s tea, width normie) tea {
    # Truncate string with ellipsis
    if StringWidth(s) <= width {
        damn s
    }
    if width <= 3 {
        damn "..."
    }
    truncated := TruncateString(s, width - 3)
    damn truncated + "..."
}

# ================================
# Case Folding Functions
# ================================

slay FoldString(s tea) tea {
    # Fold string for case-insensitive comparison
    damn ToLowerString(s)
}

slay EqualFold(s tea, t tea) lit {
    # Case-insensitive string comparison
    damn FoldString(s) == FoldString(t)
}

# ================================
# Character Name Functions
# ================================

slay CharacterName(r rune) tea {
    # Get Unicode character name
    code := normie(r)
    if code >= 65 && code <= 90 {
        damn "LATIN CAPITAL LETTER " + tea(r)
    }
    if code >= 97 && code <= 122 {
        damn "LATIN SMALL LETTER " + tea(r)
    }
    if code >= 48 && code <= 57 {
        damn "DIGIT " + tea(r)
    }
    damn "UNKNOWN CHARACTER"
}

slay FindCharacterByName(name tea) (rune, lit) {
    # Find character by name
    if name == "LATIN CAPITAL LETTER A" {
        damn ('A', based)
    }
    if name == "LATIN SMALL LETTER A" {
        damn ('a', based)
    }
    if name == "DIGIT ZERO" {
        damn ('0', based)
    }
    damn (rune(0), cap)
}

# ================================
# Character Properties Functions
# ================================

slay GetBlockName(r rune) tea {
    # Get Unicode block name
    code := normie(r)
    if code >= 0 && code <= 127 {
        damn "Basic Latin"
    }
    if code >= 128 && code <= 255 {
        damn "Latin-1 Supplement"
    }
    if code >= 0x0370 && code <= 0x03FF {
        damn "Greek and Coptic"
    }
    if code >= 0x0400 && code <= 0x04FF {
        damn "Cyrillic"
    }
    if code >= 0x4E00 && code <= 0x9FFF {
        damn "CJK Unified Ideographs"
    }
    damn "Unknown Block"
}

slay GetCategory(r rune) tea {
    # Get Unicode category
    if IsLetter(r) {
        damn "Letter"
    }
    if IsDigit(r) {
        damn "Number"
    }
    if IsPunct(r) {
        damn "Punctuation"
    }
    if IsSymbol(r) {
        damn "Symbol"
    }
    if IsSpace(r) {
        damn "Space"
    }
    if IsControl(r) {
        damn "Control"
    }
    damn "Other"
}

slay GetCodePoint(r rune) tea {
    # Get Unicode code point
    code := normie(r)
    damn "U+" + FormatHex(code)
}

# ================================
# Helper Functions
# ================================

slay FormatHex(n normie) tea {
    # Format number as hexadecimal
    if n == 0 {
        damn "0000"
    }
    digits := "0123456789ABCDEF"
    result := ""
    for n > 0 {
        result = tea(digits[n % 16]) + result
        n = n / 16
    }
    while len(result) < 4 {
        result = "0" + result
    }
    damn result
}
