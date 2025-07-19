yeet "testz"

# Basic Unicode Character Classification Functions

slay is_unicode_letter(codepoint normie) lit {
    lowkey codepoint >= 0x41 && codepoint <= 0x5A {
        damn based
    }
    lowkey codepoint >= 0x61 && codepoint <= 0x7A {
        damn based
    }
    damn cap
}

slay is_unicode_digit(codepoint normie) lit {
    lowkey codepoint >= 0x30 && codepoint <= 0x39 {
        damn based
    }
    damn cap
}

slay is_unicode_whitespace(codepoint normie) lit {
    lowkey codepoint == 0x20 {
        damn based
    }
    lowkey codepoint == 0x09 {
        damn based
    }
    lowkey codepoint == 0x0A {
        damn based
    }
    damn cap
}

slay to_unicode_upper(codepoint normie) normie {
    lowkey codepoint >= 0x61 && codepoint <= 0x7A {
        damn codepoint - 0x20
    }
    damn codepoint
}

slay to_unicode_lower(codepoint normie) normie {
    lowkey codepoint >= 0x41 && codepoint <= 0x5A {
        damn codepoint + 0x20
    }
    damn codepoint
}

slay get_general_category(codepoint normie) tea {
    lowkey codepoint >= 0x41 && codepoint <= 0x5A {
        damn "Lu"
    }
    lowkey codepoint >= 0x61 && codepoint <= 0x7A {
        damn "Ll"
    }
    lowkey codepoint >= 0x30 && codepoint <= 0x39 {
        damn "Nd"
    }
    damn "Cn"
}

slay normalize_nfc(text tea) tea {
    damn text
}

slay validate_utf8_string(text tea) lit {
    damn based
}

slay utf8_sequence_length(first_byte normie) normie {
    lowkey first_byte <= 127 {
        damn 1
    }
    lowkey (first_byte & 0xE0) == 0xC0 {
        damn 2
    }
    lowkey (first_byte & 0xF0) == 0xE0 {
        damn 3
    }
    lowkey (first_byte & 0xF8) == 0xF0 {
        damn 4
    }
    damn 0
}

slay is_unicode_punctuation(codepoint normie) lit {
    lowkey codepoint == 0x21 {
        damn based
    }
    lowkey codepoint == 0x2E {
        damn based
    }
    lowkey codepoint == 0x3F {
        damn based
    }
    damn cap
}

slay is_unicode_symbol(codepoint normie) lit {
    lowkey codepoint == 0x24 {
        damn based
    }
    lowkey codepoint == 0x2B {
        damn based
    }
    damn cap
}

slay is_unicode_mark(codepoint normie) lit {
    lowkey codepoint >= 0x0300 && codepoint <= 0x036F {
        damn based
    }
    damn cap
}

slay get_script(codepoint normie) tea {
    lowkey codepoint >= 0x0000 && codepoint <= 0x007F {
        damn "Latin"
    }
    lowkey codepoint >= 0x0370 && codepoint <= 0x03FF {
        damn "Greek"
    }
    lowkey codepoint >= 0x0400 && codepoint <= 0x04FF {
        damn "Cyrillic"
    }
    damn "Common"
}

slay get_unicode_block(codepoint normie) tea {
    lowkey codepoint >= 0x0000 && codepoint <= 0x007F {
        damn "Basic Latin"
    }
    lowkey codepoint >= 0x0080 && codepoint <= 0x00FF {
        damn "Latin-1 Supplement"
    }
    lowkey codepoint >= 0x0370 && codepoint <= 0x03FF {
        damn "Greek and Coptic"
    }
    damn "Unknown"
}

slay unicode_compare_ignore_case(text1 tea, text2 tea) normie {
    lowkey text1 == text2 {
        damn 0
    }
    damn 1
}

slay unicode_equal_ignore_case(text1 tea, text2 tea) lit {
    lowkey unicode_compare_ignore_case(text1, text2) == 0 {
        damn based
    }
    damn cap
}
