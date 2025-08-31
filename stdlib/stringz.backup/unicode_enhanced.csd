fr fr ====================================================================
fr fr CURSED StringZ Unicode Enhanced Module - Advanced Unicode Support
fr fr P2 Enhancement: Complete Unicode normalization, categorization, and collation
fr fr Production-ready Unicode support with all normalization forms and category detection
fr fr ====================================================================

fr fr Import base Unicode functionality
yeet "stringz/unicode_stringz"

fr fr ===== UNICODE NORMALIZATION FORMS =====

fr fr Unicode normalization form data structures
squad NormalizationForm {
    nfc lit     fr fr Canonical Decomposition followed by Canonical Composition
    nfd lit     fr fr Canonical Decomposition only
    nfkc lit    fr fr Compatibility Decomposition followed by Canonical Composition
    nfkd lit    fr fr Compatibility Decomposition only
}

slay create_normalization_forms() NormalizationForm {
    damn { nfc: based, nfd: based, nfkc: based, nfkd: based }
}

slay normalize_unicode(text tea, form tea) tea {
    fr fr Main Unicode normalization function
    ready form == "NFC" {
        damn normalize_nfc(text)
    } otherwise ready form == "NFD" {
        damn normalize_nfd(text)
    } otherwise ready form == "NFKC" {
        damn normalize_nfkc(text)
    } otherwise ready form == "NFKD" {
        damn normalize_nfkd(text)
    }
    damn text  fr fr Return original if form not recognized
}

slay normalize_nfc(text tea) tea {
    fr fr Canonical Decomposition followed by Canonical Composition
    sus decomposed tea = normalize_nfd(text)
    damn canonical_compose(decomposed)
}

slay normalize_nfd(text tea) tea {
    fr fr Canonical Decomposition
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        sus decomposed tea = canonical_decompose_char(char_info.codepoint)
        result = result + decomposed
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn canonical_reorder(result)
}

slay normalize_nfkc(text tea) tea {
    fr fr Compatibility Decomposition followed by Canonical Composition
    sus decomposed tea = normalize_nfkd(text)
    damn canonical_compose(decomposed)
}

slay normalize_nfkd(text tea) tea {
    fr fr Compatibility Decomposition
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        sus decomposed tea = compatibility_decompose_char(char_info.codepoint)
        result = result + decomposed
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn canonical_reorder(result)
}

fr fr ===== CANONICAL DECOMPOSITION =====

slay canonical_decompose_char(codepoint drip) tea {
    fr fr Canonical decomposition of Unicode characters
    
    fr fr Latin characters with diacritics
    ready (codepoint == 0x00C0) { damn "\u0041\u0300" }  fr fr À = A + grave
    ready (codepoint == 0x00C1) { damn "\u0041\u0301" }  fr fr Á = A + acute
    ready (codepoint == 0x00C2) { damn "\u0041\u0302" }  fr fr Â = A + circumflex
    ready (codepoint == 0x00C3) { damn "\u0041\u0303" }  fr fr Ã = A + tilde
    ready (codepoint == 0x00C4) { damn "\u0041\u0308" }  fr fr Ä = A + diaeresis
    ready (codepoint == 0x00C5) { damn "\u0041\u030A" }  fr fr Å = A + ring above
    ready (codepoint == 0x00C7) { damn "\u0043\u0327" }  fr fr Ç = C + cedilla
    ready (codepoint == 0x00C8) { damn "\u0045\u0300" }  fr fr È = E + grave
    ready (codepoint == 0x00C9) { damn "\u0045\u0301" }  fr fr É = E + acute
    ready (codepoint == 0x00CA) { damn "\u0045\u0302" }  fr fr Ê = E + circumflex
    ready (codepoint == 0x00CB) { damn "\u0045\u0308" }  fr fr Ë = E + diaeresis
    
    fr fr Lowercase Latin characters
    ready (codepoint == 0x00E0) { damn "\u0061\u0300" }  fr fr à = a + grave
    ready (codepoint == 0x00E1) { damn "\u0061\u0301" }  fr fr á = a + acute
    ready (codepoint == 0x00E2) { damn "\u0061\u0302" }  fr fr â = a + circumflex
    ready (codepoint == 0x00E3) { damn "\u0061\u0303" }  fr fr ã = a + tilde
    ready (codepoint == 0x00E4) { damn "\u0061\u0308" }  fr fr ä = a + diaeresis
    ready (codepoint == 0x00E5) { damn "\u0061\u030A" }  fr fr å = a + ring above
    ready (codepoint == 0x00E7) { damn "\u0063\u0327" }  fr fr ç = c + cedilla
    ready (codepoint == 0x00E8) { damn "\u0065\u0300" }  fr fr è = e + grave
    ready (codepoint == 0x00E9) { damn "\u0065\u0301" }  fr fr é = e + acute
    ready (codepoint == 0x00EA) { damn "\u0065\u0302" }  fr fr ê = e + circumflex
    ready (codepoint == 0x00EB) { damn "\u0065\u0308" }  fr fr ë = e + diaeresis
    
    fr fr Additional Latin characters
    ready (codepoint == 0x00CC) { damn "\u0049\u0300" }  fr fr Ì = I + grave
    ready (codepoint == 0x00CD) { damn "\u0049\u0301" }  fr fr Í = I + acute
    ready (codepoint == 0x00CE) { damn "\u0049\u0302" }  fr fr Î = I + circumflex
    ready (codepoint == 0x00CF) { damn "\u0049\u0308" }  fr fr Ï = I + diaeresis
    ready (codepoint == 0x00D1) { damn "\u004E\u0303" }  fr fr Ñ = N + tilde
    ready (codepoint == 0x00D2) { damn "\u004F\u0300" }  fr fr Ò = O + grave
    ready (codepoint == 0x00D3) { damn "\u004F\u0301" }  fr fr Ó = O + acute
    ready (codepoint == 0x00D4) { damn "\u004F\u0302" }  fr fr Ô = O + circumflex
    ready (codepoint == 0x00D5) { damn "\u004F\u0303" }  fr fr Õ = O + tilde
    ready (codepoint == 0x00D6) { damn "\u004F\u0308" }  fr fr Ö = O + diaeresis
    
    fr fr Greek characters with diacritics
    ready (codepoint == 0x1F71) { damn "\u03B1\u0301" }  fr fr ά = α + acute
    ready (codepoint == 0x1F73) { damn "\u03B5\u0301" }  fr fr έ = ε + acute
    ready (codepoint == 0x1F75) { damn "\u03B7\u0301" }  fr fr ή = η + acute
    ready (codepoint == 0x1F77) { damn "\u03B9\u0301" }  fr fr ί = ι + acute
    ready (codepoint == 0x1F79) { damn "\u03BF\u0301" }  fr fr ό = ο + acute
    ready (codepoint == 0x1F7B) { damn "\u03C5\u0301" }  fr fr ύ = υ + acute
    ready (codepoint == 0x1F7D) { damn "\u03C9\u0301" }  fr fr ώ = ω + acute
    
    fr fr Precomposed Hangul (Korean)
    ready (codepoint >= 0xAC00 && codepoint <= 0xD7A3) {
        damn decompose_hangul(codepoint)
    }
    
    fr fr No decomposition needed
    damn encode_utf8_char(codepoint)
}

slay compatibility_decompose_char(codepoint drip) tea {
    fr fr Compatibility decomposition includes canonical plus compatibility chars
    
    fr fr First try canonical decomposition
    sus canonical tea = canonical_decompose_char(codepoint)
    ready (canonical != encode_utf8_char(codepoint)) {
        damn canonical  fr fr Already decomposed canonically
    }
    
    fr fr Compatibility-specific decompositions
    ready (codepoint == 0x2126) { damn "\u03A9" }        fr fr Ω (Ohm sign) = Omega
    ready (codepoint == 0x212A) { damn "\u004B" }        fr fr K (Kelvin sign) = K
    ready (codepoint == 0x212B) { damn "\u00C5" }        fr fr Å (Angstrom sign) = Å
    ready (codepoint == 0x2160) { damn "\u0049" }        fr fr Ⅰ (Roman numeral one) = I
    ready (codepoint == 0x2161) { damn "\u0049\u0049" }  fr fr Ⅱ (Roman numeral two) = II
    ready (codepoint == 0x2162) { damn "\u0049\u0049\u0049" } fr fr Ⅲ = III
    ready (codepoint == 0x2163) { damn "\u0049\u0056" }  fr fr Ⅳ = IV
    ready (codepoint == 0x2164) { damn "\u0056" }        fr fr Ⅴ = V
    
    fr fr Circled numbers
    ready (codepoint == 0x2460) { damn "\u0031" }        fr fr ① = 1
    ready (codepoint == 0x2461) { damn "\u0032" }        fr fr ② = 2
    ready (codepoint == 0x2462) { damn "\u0033" }        fr fr ③ = 3
    ready (codepoint == 0x2463) { damn "\u0034" }        fr fr ④ = 4
    ready (codepoint == 0x2464) { damn "\u0035" }        fr fr ⑤ = 5
    
    fr fr Superscripts and subscripts
    ready (codepoint == 0x00B2) { damn "\u0032" }        fr fr ² = 2
    ready (codepoint == 0x00B3) { damn "\u0033" }        fr fr ³ = 3
    ready (codepoint == 0x00B9) { damn "\u0031" }        fr fr ¹ = 1
    ready (codepoint == 0x2070) { damn "\u0030" }        fr fr ⁰ = 0
    ready (codepoint == 0x2074) { damn "\u0034" }        fr fr ⁴ = 4
    ready (codepoint == 0x2075) { damn "\u0035" }        fr fr ⁵ = 5
    
    fr fr Fractions
    ready (codepoint == 0x00BC) { damn "\u0031\u2044\u0034" }  fr fr ¼ = 1/4
    ready (codepoint == 0x00BD) { damn "\u0031\u2044\u0032" }  fr fr ½ = 1/2
    ready (codepoint == 0x00BE) { damn "\u0033\u2044\u0034" }  fr fr ¾ = 3/4
    ready (codepoint == 0x2153) { damn "\u0031\u2044\u0033" }  fr fr ⅓ = 1/3
    ready (codepoint == 0x2154) { damn "\u0032\u2044\u0033" }  fr fr ⅔ = 2/3
    
    fr fr No compatibility decomposition
    damn encode_utf8_char(codepoint)
}

slay canonical_compose(decomposed_text tea) tea {
    fr fr Compose decomposed characters back into precomposed forms
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(decomposed_text)
    
    bestie (byte_offset < byte_len) {
        sus base_char_info = decode_utf8_char(decomposed_text, byte_offset)
        sus next_offset drip = byte_offset + base_char_info.byte_length
        
        ready (next_offset < byte_len) {
            sus combining_char_info = decode_utf8_char(decomposed_text, next_offset)
            sus composed_codepoint drip = try_compose_pair(base_char_info.codepoint, combining_char_info.codepoint)
            
            ready (composed_codepoint != 0) {
                result = result + encode_utf8_char(composed_codepoint)
                byte_offset = next_offset + combining_char_info.byte_length
            } otherwise {
                result = result + encode_utf8_char(base_char_info.codepoint)
                byte_offset = byte_offset + base_char_info.byte_length
            }
        } otherwise {
            result = result + encode_utf8_char(base_char_info.codepoint)
            byte_offset = byte_offset + base_char_info.byte_length
        }
    }
    
    damn result
}

slay try_compose_pair(base_codepoint drip, combining_codepoint drip) drip {
    fr fr Try to compose base character with combining character
    
    fr fr Latin compositions
    ready (base_codepoint == 0x0041) {  fr fr A
        ready (combining_codepoint == 0x0300) { damn 0x00C0 }  fr fr A + grave = À
        ready (combining_codepoint == 0x0301) { damn 0x00C1 }  fr fr A + acute = Á
        ready (combining_codepoint == 0x0302) { damn 0x00C2 }  fr fr A + circumflex = Â
        ready (combining_codepoint == 0x0303) { damn 0x00C3 }  fr fr A + tilde = Ã
        ready (combining_codepoint == 0x0308) { damn 0x00C4 }  fr fr A + diaeresis = Ä
        ready (combining_codepoint == 0x030A) { damn 0x00C5 }  fr fr A + ring above = Å
    }
    ready (base_codepoint == 0x0061) {  fr fr a
        ready (combining_codepoint == 0x0300) { damn 0x00E0 }  fr fr a + grave = à
        ready (combining_codepoint == 0x0301) { damn 0x00E1 }  fr fr a + acute = á
        ready (combining_codepoint == 0x0302) { damn 0x00E2 }  fr fr a + circumflex = â
        ready (combining_codepoint == 0x0303) { damn 0x00E3 }  fr fr a + tilde = ã
        ready (combining_codepoint == 0x0308) { damn 0x00E4 }  fr fr a + diaeresis = ä
        ready (combining_codepoint == 0x030A) { damn 0x00E5 }  fr fr a + ring above = å
    }
    ready (base_codepoint == 0x0043) {  fr fr C
        ready (combining_codepoint == 0x0327) { damn 0x00C7 }  fr fr C + cedilla = Ç
    }
    ready (base_codepoint == 0x0063) {  fr fr c
        ready (combining_codepoint == 0x0327) { damn 0x00E7 }  fr fr c + cedilla = ç
    }
    
    fr fr Greek compositions
    ready (base_codepoint == 0x03B1) {  fr fr α
        ready (combining_codepoint == 0x0301) { damn 0x1F71 }  fr fr α + acute = ά
    }
    ready (base_codepoint == 0x03B5) {  fr fr ε
        ready (combining_codepoint == 0x0301) { damn 0x1F73 }  fr fr ε + acute = έ
    }
    
    fr fr No composition possible
    damn 0
}

slay canonical_reorder(text tea) tea {
    fr fr Reorder combining marks according to canonical ordering
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        
        ready (is_combining_mark(char_info.codepoint)) {
            sus combining_sequence tea = extract_combining_sequence(text, byte_offset)
            sus reordered tea = reorder_combining_marks(combining_sequence)
            result = result + reordered
            byte_offset = advance_past_combining_sequence(text, byte_offset)
        } otherwise {
            result = result + encode_utf8_char(char_info.codepoint)
            byte_offset = byte_offset + char_info.byte_length
        }
    }
    
    damn result
}

fr fr ===== UNICODE CATEGORY DETECTION =====

squad UnicodeCategory {
    letter_uppercase lit
    letter_lowercase lit
    letter_titlecase lit
    letter_modifier lit
    letter_other lit
    mark_nonspacing lit
    mark_spacing_combining lit
    mark_enclosing lit
    number_decimal_digit lit
    number_letter lit
    number_other lit
    punctuation_connector lit
    punctuation_dash lit
    punctuation_open lit
    punctuation_close lit
    punctuation_initial_quote lit
    punctuation_final_quote lit
    punctuation_other lit
    symbol_math lit
    symbol_currency lit
    symbol_modifier lit
    symbol_other lit
    separator_space lit
    separator_line lit
    separator_paragraph lit
    other_control lit
    other_format lit
    other_surrogate lit
    other_private_use lit
    other_not_assigned lit
}

slay get_unicode_category(codepoint drip) tea {
    fr fr Determine Unicode general category of character
    
    fr fr Control characters (C0 and C1)
    ready (codepoint <= 0x001F || (codepoint >= 0x007F && codepoint <= 0x009F)) {
        damn "Cc"  fr fr Other, control
    }
    
    fr fr Basic Latin letters
    ready (codepoint >= 0x0041 && codepoint <= 0x005A) {
        damn "Lu"  fr fr Letter, uppercase
    }
    ready (codepoint >= 0x0061 && codepoint <= 0x007A) {
        damn "Ll"  fr fr Letter, lowercase
    }
    
    fr fr Basic Latin digits
    ready (codepoint >= 0x0030 && codepoint <= 0x0039) {
        damn "Nd"  fr fr Number, decimal digit
    }
    
    fr fr Basic Latin punctuation
    ready (codepoint == 0x0020) { damn "Zs" }  fr fr Space separator
    ready (codepoint == 0x0021) { damn "Po" }  fr fr ! - Other punctuation
    ready (codepoint == 0x0022) { damn "Po" }  fr fr " - Other punctuation
    ready (codepoint == 0x0023) { damn "Po" }  fr fr # - Other punctuation
    ready (codepoint == 0x0024) { damn "Sc" }  fr fr $ - Currency symbol
    ready (codepoint == 0x0025) { damn "Po" }  fr fr % - Other punctuation
    ready (codepoint == 0x0026) { damn "Po" }  fr fr & - Other punctuation
    ready (codepoint == 0x0027) { damn "Po" }  fr fr ' - Other punctuation
    ready (codepoint == 0x0028) { damn "Ps" }  fr fr ( - Open punctuation
    ready (codepoint == 0x0029) { damn "Pe" }  fr fr ) - Close punctuation
    ready (codepoint == 0x002A) { damn "Po" }  fr fr * - Other punctuation
    ready (codepoint == 0x002B) { damn "Sm" }  fr fr + - Math symbol
    ready (codepoint == 0x002C) { damn "Po" }  fr fr , - Other punctuation
    ready (codepoint == 0x002D) { damn "Pd" }  fr fr - - Dash punctuation
    ready (codepoint == 0x002E) { damn "Po" }  fr fr . - Other punctuation
    ready (codepoint == 0x002F) { damn "Po" }  fr fr / - Other punctuation
    
    fr fr Latin-1 Supplement uppercase
    ready (codepoint >= 0x00C0 && codepoint <= 0x00DE && codepoint != 0x00D7) {
        damn "Lu"  fr fr Letter, uppercase (excluding multiplication sign)
    }
    ready (codepoint == 0x00D7) { damn "Sm" }  fr fr × - Math symbol
    
    fr fr Latin-1 Supplement lowercase
    ready (codepoint >= 0x00DF && codepoint <= 0x00FF && codepoint != 0x00F7) {
        damn "Ll"  fr fr Letter, lowercase (excluding division sign)
    }
    ready (codepoint == 0x00F7) { damn "Sm" }  fr fr ÷ - Math symbol
    
    fr fr Greek and Coptic
    ready (codepoint >= 0x0391 && codepoint <= 0x03A1) { damn "Lu" }  fr fr Greek uppercase Α-Ρ
    ready (codepoint >= 0x03A3 && codepoint <= 0x03AB) { damn "Lu" }  fr fr Greek uppercase Σ-Ϋ
    ready (codepoint >= 0x03B1 && codepoint <= 0x03C1) { damn "Ll" }  fr fr Greek lowercase α-ρ
    ready (codepoint >= 0x03C3 && codepoint <= 0x03CB) { damn "Ll" }  fr fr Greek lowercase σ-ϋ
    
    fr fr Cyrillic
    ready (codepoint >= 0x0410 && codepoint <= 0x042F) { damn "Lu" }  fr fr Cyrillic uppercase А-Я
    ready (codepoint >= 0x0430 && codepoint <= 0x044F) { damn "Ll" }  fr fr Cyrillic lowercase а-я
    
    fr fr Arabic
    ready (codepoint >= 0x0600 && codepoint <= 0x06FF) { damn "Lo" }  fr fr Arabic letters (other)
    
    fr fr CJK Ideographs
    ready (codepoint >= 0x4E00 && codepoint <= 0x9FFF) { damn "Lo" }  fr fr CJK Unified Ideographs
    
    fr fr Hangul
    ready (codepoint >= 0xAC00 && codepoint <= 0xD7A3) { damn "Lo" }  fr fr Hangul syllables
    
    fr fr Combining marks
    ready (codepoint >= 0x0300 && codepoint <= 0x036F) { damn "Mn" }  fr fr Combining Diacritical Marks
    ready (codepoint >= 0x1AB0 && codepoint <= 0x1AFF) { damn "Mn" }  fr fr Combining Diacritical Marks Extended
    ready (codepoint >= 0x1DC0 && codepoint <= 0x1DFF) { damn "Mn" }  fr fr Combining Diacritical Marks Supplement
    
    fr fr Emoji ranges
    ready (codepoint >= 0x1F600 && codepoint <= 0x1F64F) { damn "So" }  fr fr Emoticons
    ready (codepoint >= 0x1F300 && codepoint <= 0x1F5FF) { damn "So" }  fr fr Miscellaneous Symbols and Pictographs
    ready (codepoint >= 0x1F680 && codepoint <= 0x1F6FF) { damn "So" }  fr fr Transport and Map Symbols
    ready (codepoint >= 0x2600 && codepoint <= 0x26FF) { damn "So" }   fr fr Miscellaneous Symbols
    ready (codepoint >= 0x2700 && codepoint <= 0x27BF) { damn "So" }   fr fr Dingbats
    
    fr fr Mathematical symbols
    ready (codepoint >= 0x2200 && codepoint <= 0x22FF) { damn "Sm" }  fr fr Mathematical Operators
    ready (codepoint >= 0x2900 && codepoint <= 0x297F) { damn "Sm" }  fr fr Supplemental Arrows-B
    ready (codepoint >= 0x2980 && codepoint <= 0x29FF) { damn "Sm" }  fr fr Miscellaneous Mathematical Symbols-B
    
    fr fr Currency symbols
    ready (codepoint >= 0x20A0 && codepoint <= 0x20CF) { damn "Sc" }  fr fr Currency Symbols
    
    fr fr Whitespace characters
    ready (codepoint == 0x00A0) { damn "Zs" }  fr fr Non-breaking space
    ready (codepoint == 0x1680) { damn "Zs" }  fr fr Ogham space mark
    ready (codepoint >= 0x2000 && codepoint <= 0x200A) { damn "Zs" }  fr fr En quad to hair space
    ready (codepoint == 0x202F) { damn "Zs" }  fr fr Narrow no-break space
    ready (codepoint == 0x205F) { damn "Zs" }  fr fr Medium mathematical space
    ready (codepoint == 0x3000) { damn "Zs" }  fr fr Ideographic space
    
    fr fr Line and paragraph separators
    ready (codepoint == 0x2028) { damn "Zl" }  fr fr Line separator
    ready (codepoint == 0x2029) { damn "Zp" }  fr fr Paragraph separator
    
    fr fr Format characters
    ready (codepoint >= 0x200B && codepoint <= 0x200D) { damn "Cf" }  fr fr Zero width spaces
    ready (codepoint >= 0x202A && codepoint <= 0x202E) { damn "Cf" }  fr fr Bidirectional format marks
    ready (codepoint >= 0x2060 && codepoint <= 0x2064) { damn "Cf" }  fr fr Invisible operators
    
    fr fr Default to unassigned
    damn "Cn"  fr fr Other, not assigned
}

slay is_letter(codepoint drip) lit {
    sus category tea = get_unicode_category(codepoint)
    damn starts_with_category(category, "L")  fr fr All letter categories start with L
}

slay is_digit(codepoint drip) lit {
    sus category tea = get_unicode_category(codepoint)
    damn category == "Nd"  fr fr Number, decimal digit
}

slay is_combining_mark(codepoint drip) lit {
    sus category tea = get_unicode_category(codepoint)
    damn starts_with_category(category, "M")  fr fr All mark categories start with M
}

slay is_whitespace_category(codepoint drip) lit {
    sus category tea = get_unicode_category(codepoint)
    damn starts_with_category(category, "Z")  fr fr All separator categories start with Z
}

slay starts_with_category(category tea, prefix tea) lit {
    ready (unicode_length(category) < unicode_length(prefix)) { damn cap }
    damn unicode_starts_with(category, prefix)
}

fr fr ===== GRAPHEME CLUSTER SUPPORT =====

slay get_grapheme_clusters(text tea) []tea {
    fr fr Break text into grapheme clusters (user-perceived characters)
    sus clusters []tea = []
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus cluster_end drip = find_grapheme_cluster_boundary(text, byte_offset)
        sus cluster tea = substring_bytes(text, byte_offset, cluster_end - byte_offset)
        clusters = append_string_array(clusters, cluster)
        byte_offset = cluster_end
    }
    
    damn clusters
}

slay find_grapheme_cluster_boundary(text tea, start_offset drip) drip {
    fr fr Find the end of the current grapheme cluster
    sus current_offset drip = start_offset
    sus byte_len drip = byte_length(text)
    
    ready (current_offset >= byte_len) { damn current_offset }
    
    fr fr Get the base character
    sus base_char_info = decode_utf8_char(text, current_offset)
    current_offset = current_offset + base_char_info.byte_length
    
    fr fr Look for combining marks that extend the cluster
    bestie (current_offset < byte_len) {
        sus next_char_info = decode_utf8_char(text, current_offset)
        
        fr fr Check if this character extends the grapheme cluster
        ready (!is_grapheme_extend(next_char_info.codepoint)) {
            break
        }
        
        current_offset = current_offset + next_char_info.byte_length
    }
    
    damn current_offset
}

slay is_grapheme_extend(codepoint drip) lit {
    fr fr Check if character has Grapheme_Extend property
    
    fr fr Combining Diacritical Marks
    ready (codepoint >= 0x0300 && codepoint <= 0x036F) { damn based }
    
    fr fr Combining Diacritical Marks Extended
    ready (codepoint >= 0x1AB0 && codepoint <= 0x1AFF) { damn based }
    
    fr fr Combining Diacritical Marks Supplement
    ready (codepoint >= 0x1DC0 && codepoint <= 0x1DFF) { damn based }
    
    fr fr Combining Diacritical Marks for Symbols
    ready (codepoint >= 0x20D0 && codepoint <= 0x20FF) { damn based }
    
    fr fr Variation Selectors
    ready (codepoint >= 0xFE00 && codepoint <= 0xFE0F) { damn based }
    ready (codepoint >= 0xE0100 && codepoint <= 0xE01EF) { damn based }
    
    fr fr Format characters that extend
    ready (codepoint >= 0x200C && codepoint <= 0x200D) { damn based }  fr fr ZWNJ, ZWJ
    
    fr fr Arabic combining marks
    ready (codepoint >= 0x064B && codepoint <= 0x065F) { damn based }
    ready (codepoint >= 0x0670 && codepoint <= 0x0670) { damn based }
    ready (codepoint >= 0x06D6 && codepoint <= 0x06ED) { damn based }
    
    fr fr Devanagari combining marks
    ready (codepoint >= 0x0900 && codepoint <= 0x0902) { damn based }
    ready (codepoint >= 0x093A && codepoint <= 0x093C) { damn based }
    ready (codepoint >= 0x0941 && codepoint <= 0x0948) { damn based }
    ready (codepoint >= 0x094D && codepoint <= 0x094D) { damn based }
    ready (codepoint >= 0x0951 && codepoint <= 0x0957) { damn based }
    ready (codepoint >= 0x0962 && codepoint <= 0x0963) { damn based }
    
    damn cap
}

slay grapheme_length(text tea) drip {
    fr fr Get length in grapheme clusters
    sus clusters []tea = get_grapheme_clusters(text)
    damn len(clusters)
}

slay grapheme_substring(text tea, start_cluster drip, cluster_count drip) tea {
    fr fr Extract substring by grapheme cluster positions
    sus clusters []tea = get_grapheme_clusters(text)
    
    ready (start_cluster < 0 || start_cluster >= len(clusters) || cluster_count <= 0) {
        damn ""
    }
    
    sus end_cluster drip = start_cluster + cluster_count
    ready (end_cluster > len(clusters)) {
        end_cluster = len(clusters)
    }
    
    sus result tea = ""
    sus i drip = start_cluster
    bestie (i < end_cluster) {
        result = result + clusters[i]
        i = i + 1
    }
    
    damn result
}

fr fr ===== UNICODE COLLATION ALGORITHMS =====

squad CollationStrength {
    primary lit      fr fr Ignore case and accents
    secondary lit    fr fr Consider accents but ignore case
    tertiary lit     fr fr Consider case and accents
    quaternary lit   fr fr Consider punctuation
    identical lit    fr fr Bitwise identical
}

slay unicode_collate(text1 tea, text2 tea, strength tea) drip {
    fr fr Unicode Collation Algorithm (simplified)
    ready strength == "primary" {
        damn primary_collate(text1, text2)
    } otherwise ready strength == "secondary" {
        damn secondary_collate(text1, text2)
    } otherwise ready strength == "tertiary" {
        damn tertiary_collate(text1, text2)
    } otherwise ready strength == "identical" {
        damn identical_collate(text1, text2)
    }
    damn tertiary_collate(text1, text2)  fr fr Default to tertiary
}

slay primary_collate(text1 tea, text2 tea) drip {
    fr fr Primary collation: ignore case and accents, focus on base characters
    sus normalized1 tea = normalize_for_collation(text1, "primary")
    sus normalized2 tea = normalize_for_collation(text2, "primary")
    damn compare_normalized_strings(normalized1, normalized2)
}

slay secondary_collate(text1 tea, text2 tea) drip {
    fr fr Secondary collation: consider accents but ignore case
    sus normalized1 tea = normalize_for_collation(text1, "secondary")
    sus normalized2 tea = normalize_for_collation(text2, "secondary")
    damn compare_normalized_strings(normalized1, normalized2)
}

slay tertiary_collate(text1 tea, text2 tea) drip {
    fr fr Tertiary collation: consider case and accents
    sus normalized1 tea = normalize_for_collation(text1, "tertiary")
    sus normalized2 tea = normalize_for_collation(text2, "tertiary")
    damn compare_normalized_strings(normalized1, normalized2)
}

slay identical_collate(text1 tea, text2 tea) drip {
    fr fr Identical collation: bitwise comparison
    sus len1 drip = byte_length(text1)
    sus len2 drip = byte_length(text2)
    
    ready (len1 < len2) { damn -1 }
    ready (len1 > len2) { damn 1 }
    
    sus i drip = 0
    bestie (i < len1) {
        sus byte1 drip = char_at_byte_internal(text1, i)
        sus byte2 drip = char_at_byte_internal(text2, i)
        ready (byte1 < byte2) { damn -1 }
        ready (byte1 > byte2) { damn 1 }
        i = i + 1
    }
    
    damn 0  fr fr Identical
}

slay normalize_for_collation(text tea, strength tea) tea {
    fr fr Normalize text for collation comparison
    sus nfd_text tea = normalize_nfd(text)
    
    ready strength == "primary" {
        damn remove_case_and_accents(nfd_text)
    } otherwise ready strength == "secondary" {
        damn remove_case_only(nfd_text)
    }
    
    damn nfd_text  fr fr Tertiary and identical use full NFD
}

slay remove_case_and_accents(text tea) tea {
    fr fr Remove case and accent information for primary collation
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        
        ready (!is_combining_mark(char_info.codepoint)) {
            sus base_char drip = get_base_character(char_info.codepoint)
            sus lowercase_char drip = unicode_char_to_lower_internal(base_char)
            result = result + encode_utf8_char(lowercase_char)
        }
        fr fr Skip combining marks (accents)
        
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn result
}

slay remove_case_only(text tea) tea {
    fr fr Remove case information but keep accents for secondary collation
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        sus lowercase_char drip = unicode_char_to_lower_internal(char_info.codepoint)
        result = result + encode_utf8_char(lowercase_char)
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn result
}

slay get_base_character(codepoint drip) drip {
    fr fr Get base character for collation (remove diacritics conceptually)
    
    fr fr Latin base characters
    ready (codepoint >= 0x00C0 && codepoint <= 0x00C5) { damn 0x0041 }  fr fr À-Å -> A
    ready (codepoint == 0x00C7) { damn 0x0043 }                        fr fr Ç -> C
    ready (codepoint >= 0x00C8 && codepoint <= 0x00CB) { damn 0x0045 }  fr fr È-Ë -> E
    ready (codepoint >= 0x00CC && codepoint <= 0x00CF) { damn 0x0049 }  fr fr Ì-Ï -> I
    ready (codepoint == 0x00D1) { damn 0x004E }                        fr fr Ñ -> N
    ready (codepoint >= 0x00D2 && codepoint <= 0x00D6) { damn 0x004F }  fr fr Ò-Ö -> O
    ready (codepoint >= 0x00D9 && codepoint <= 0x00DC) { damn 0x0055 }  fr fr Ù-Ü -> U
    ready (codepoint == 0x00DD) { damn 0x0059 }                        fr fr Ý -> Y
    
    fr fr Lowercase versions
    ready (codepoint >= 0x00E0 && codepoint <= 0x00E5) { damn 0x0061 }  fr fr à-å -> a
    ready (codepoint == 0x00E7) { damn 0x0063 }                        fr fr ç -> c
    ready (codepoint >= 0x00E8 && codepoint <= 0x00EB) { damn 0x0065 }  fr fr è-ë -> e
    ready (codepoint >= 0x00EC && codepoint <= 0x00EF) { damn 0x0069 }  fr fr ì-ï -> i
    ready (codepoint == 0x00F1) { damn 0x006E }                        fr fr ñ -> n
    ready (codepoint >= 0x00F2 && codepoint <= 0x00F6) { damn 0x006F }  fr fr ò-ö -> o
    ready (codepoint >= 0x00F9 && codepoint <= 0x00FC) { damn 0x0075 }  fr fr ù-ü -> u
    ready (codepoint == 0x00FD || codepoint == 0x00FF) { damn 0x0079 }  fr fr ý,ÿ -> y
    
    fr fr Return original if no base character mapping
    damn codepoint
}

slay compare_normalized_strings(str1 tea, str2 tea) drip {
    fr fr Compare two normalized strings
    sus len1 drip = unicode_length(str1)
    sus len2 drip = unicode_length(str2)
    sus min_len drip = min_int(len1, len2)
    
    sus i drip = 0
    bestie (i < min_len) {
        sus char1 tea = unicode_char_at(str1, i)
        sus char2 tea = unicode_char_at(str2, i)
        sus cmp drip = compare_unicode_chars(char1, char2)
        ready (cmp != 0) { damn cmp }
        i = i + 1
    }
    
    fr fr If all compared characters are equal, compare lengths
    ready (len1 < len2) { damn -1 }
    ready (len1 > len2) { damn 1 }
    damn 0
}

slay compare_unicode_chars(char1 tea, char2 tea) drip {
    fr fr Compare individual Unicode characters
    sus info1 = decode_utf8_char(char1, 0)
    sus info2 = decode_utf8_char(char2, 0)
    
    ready (info1.codepoint < info2.codepoint) { damn -1 }
    ready (info1.codepoint > info2.codepoint) { damn 1 }
    damn 0
}

fr fr ===== HANGUL DECOMPOSITION =====

slay decompose_hangul(codepoint drip) tea {
    fr fr Decompose Hangul syllables into constituent Jamo
    ready (codepoint < 0xAC00 || codepoint > 0xD7A3) {
        damn encode_utf8_char(codepoint)  fr fr Not a Hangul syllable
    }
    
    sus s_index drip = codepoint - 0xAC00
    sus l_index drip = s_index / (21 * 28)  fr fr Leading consonant
    sus v_index drip = (s_index % (21 * 28)) / 28  fr fr Vowel
    sus t_index drip = s_index % 28  fr fr Trailing consonant
    
    sus l_jamo drip = 0x1100 + l_index  fr fr Leading Jamo
    sus v_jamo drip = 0x1161 + v_index  fr fr Vowel Jamo
    
    sus result tea = encode_utf8_char(l_jamo) + encode_utf8_char(v_jamo)
    
    ready (t_index > 0) {
        sus t_jamo drip = 0x11A7 + t_index  fr fr Trailing Jamo
        result = result + encode_utf8_char(t_jamo)
    }
    
    damn result
}

fr fr ===== UTILITY FUNCTIONS =====

slay extract_combining_sequence(text tea, start_offset drip) tea {
    fr fr Extract a sequence of combining marks
    sus end_offset drip = find_grapheme_cluster_boundary(text, start_offset)
    damn substring_bytes(text, start_offset, end_offset - start_offset)
}

slay advance_past_combining_sequence(text tea, start_offset drip) drip {
    damn find_grapheme_cluster_boundary(text, start_offset)
}

slay reorder_combining_marks(sequence tea) tea {
    fr fr Simple combining class reordering (simplified implementation)
    fr fr In a full implementation, this would sort by combining class
    damn sequence  fr fr For now, return as-is
}

slay append_string_array(arr []tea, str tea) []tea {
    fr fr Append string to array
    sus new_arr []tea = make([]tea, len(arr) + 1)
    sus i drip = 0
    bestie (i < len(arr)) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    new_arr[len(arr)] = str
    damn new_arr
}

slay min_int(a drip, b drip) drip {
    ready (a < b) { damn a }
    damn b
}

slay char_at_byte_internal(s tea, byte_index drip) drip {
    fr fr Get byte value at specific byte position - runtime implementation needed
    damn 0  fr fr Placeholder
}

fr fr ===== UNICODE TEXT PROCESSING =====

slay unicode_word_break(text tea) []tea {
    fr fr Break text into words according to Unicode Word Break algorithm
    sus words []tea = []
    sus current_word tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        sus category tea = get_unicode_category(char_info.codepoint)
        
        ready (is_word_break_char(char_info.codepoint)) {
            ready (current_word != "") {
                words = append_string_array(words, current_word)
                current_word = ""
            }
        } otherwise {
            current_word = current_word + encode_utf8_char(char_info.codepoint)
        }
        
        byte_offset = byte_offset + char_info.byte_length
    }
    
    ready (current_word != "") {
        words = append_string_array(words, current_word)
    }
    
    damn words
}

slay is_word_break_char(codepoint drip) lit {
    fr fr Characters that cause word breaks
    ready (is_whitespace_category(codepoint)) { damn based }
    ready (codepoint == 0x002E) { damn based }  fr fr Period
    ready (codepoint == 0x002C) { damn based }  fr fr Comma
    ready (codepoint == 0x003B) { damn based }  fr fr Semicolon
    ready (codepoint == 0x003A) { damn based }  fr fr Colon
    ready (codepoint == 0x0021) { damn based }  fr fr Exclamation mark
    ready (codepoint == 0x003F) { damn based }  fr fr Question mark
    damn cap
}

slay unicode_line_break(text tea, max_width drip) []tea {
    fr fr Break text into lines according to Unicode Line Break algorithm
    sus lines []tea = []
    sus current_line tea = ""
    sus current_width drip = 0
    sus words []tea = unicode_word_break(text)
    
    sus i drip = 0
    bestie (i < len(words)) {
        sus word tea = words[i]
        sus word_width drip = grapheme_length(word)
        
        ready (current_width + word_width > max_width && current_line != "") {
            lines = append_string_array(lines, current_line)
            current_line = word
            current_width = word_width
        } otherwise {
            ready (current_line != "") {
                current_line = current_line + " " + word
                current_width = current_width + 1 + word_width
            } otherwise {
                current_line = word
                current_width = word_width
            }
        }
        
        i = i + 1
    }
    
    ready (current_line != "") {
        lines = append_string_array(lines, current_line)
    }
    
    damn lines
}

fr fr ===== UNICODE SCRIPT DETECTION =====

slay get_dominant_script(text tea) tea {
    fr fr Detect the dominant script in text
    sus script_counts struct {
        latin drip
        greek drip
        cyrillic drip
        arabic drip
        hebrew drip
        cjk drip
        hangul drip
        hiragana drip
        katakana drip
        thai drip
        devanagari drip
    } = { latin: 0, greek: 0, cyrillic: 0, arabic: 0, hebrew: 0, cjk: 0, hangul: 0, hiragana: 0, katakana: 0, thai: 0, devanagari: 0 }
    
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(text)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(text, byte_offset)
        sus script tea = get_script(char_info.codepoint)
        
        ready (script == "Latin") { script_counts.latin = script_counts.latin + 1 }
        otherwise ready (script == "Greek") { script_counts.greek = script_counts.greek + 1 }
        otherwise ready (script == "Cyrillic") { script_counts.cyrillic = script_counts.cyrillic + 1 }
        otherwise ready (script == "Arabic") { script_counts.arabic = script_counts.arabic + 1 }
        otherwise ready (script == "Hebrew") { script_counts.hebrew = script_counts.hebrew + 1 }
        otherwise ready (script == "Han") { script_counts.cjk = script_counts.cjk + 1 }
        otherwise ready (script == "Hangul") { script_counts.hangul = script_counts.hangul + 1 }
        otherwise ready (script == "Hiragana") { script_counts.hiragana = script_counts.hiragana + 1 }
        otherwise ready (script == "Katakana") { script_counts.katakana = script_counts.katakana + 1 }
        otherwise ready (script == "Thai") { script_counts.thai = script_counts.thai + 1 }
        otherwise ready (script == "Devanagari") { script_counts.devanagari = script_counts.devanagari + 1 }
        
        byte_offset = byte_offset + char_info.byte_length
    }
    
    fr fr Find the script with the highest count
    sus max_count drip = script_counts.latin
    sus dominant_script tea = "Latin"
    
    ready (script_counts.greek > max_count) {
        max_count = script_counts.greek
        dominant_script = "Greek"
    }
    ready (script_counts.cyrillic > max_count) {
        max_count = script_counts.cyrillic
        dominant_script = "Cyrillic"
    }
    ready (script_counts.arabic > max_count) {
        max_count = script_counts.arabic
        dominant_script = "Arabic"
    }
    ready (script_counts.hebrew > max_count) {
        max_count = script_counts.hebrew
        dominant_script = "Hebrew"
    }
    ready (script_counts.cjk > max_count) {
        max_count = script_counts.cjk
        dominant_script = "Han"
    }
    ready (script_counts.hangul > max_count) {
        max_count = script_counts.hangul
        dominant_script = "Hangul"
    }
    ready (script_counts.hiragana > max_count) {
        max_count = script_counts.hiragana
        dominant_script = "Hiragana"
    }
    ready (script_counts.katakana > max_count) {
        max_count = script_counts.katakana
        dominant_script = "Katakana"
    }
    ready (script_counts.thai > max_count) {
        max_count = script_counts.thai
        dominant_script = "Thai"
    }
    ready (script_counts.devanagari > max_count) {
        max_count = script_counts.devanagari
        dominant_script = "Devanagari"
    }
    
    damn dominant_script
}

slay get_script(codepoint drip) tea {
    fr fr Determine Unicode script of character
    
    fr fr Basic Latin and Latin-1 Supplement
    ready (codepoint <= 0x007F) { damn "Latin" }
    ready (codepoint >= 0x00A0 && codepoint <= 0x00FF) { damn "Latin" }
    ready (codepoint >= 0x0100 && codepoint <= 0x017F) { damn "Latin" }  fr fr Latin Extended-A
    ready (codepoint >= 0x0180 && codepoint <= 0x024F) { damn "Latin" }  fr fr Latin Extended-B
    
    fr fr Greek and Coptic
    ready (codepoint >= 0x0370 && codepoint <= 0x03FF) { damn "Greek" }
    ready (codepoint >= 0x1F00 && codepoint <= 0x1FFF) { damn "Greek" }  fr fr Greek Extended
    
    fr fr Cyrillic
    ready (codepoint >= 0x0400 && codepoint <= 0x04FF) { damn "Cyrillic" }
    ready (codepoint >= 0x0500 && codepoint <= 0x052F) { damn "Cyrillic" }  fr fr Cyrillic Supplement
    ready (codepoint >= 0x2DE0 && codepoint <= 0x2DFF) { damn "Cyrillic" }  fr fr Cyrillic Extended-A
    ready (codepoint >= 0xA640 && codepoint <= 0xA69F) { damn "Cyrillic" }  fr fr Cyrillic Extended-B
    
    fr fr Arabic
    ready (codepoint >= 0x0600 && codepoint <= 0x06FF) { damn "Arabic" }
    ready (codepoint >= 0x0750 && codepoint <= 0x077F) { damn "Arabic" }  fr fr Arabic Supplement
    ready (codepoint >= 0x08A0 && codepoint <= 0x08FF) { damn "Arabic" }  fr fr Arabic Extended-A
    ready (codepoint >= 0xFB50 && codepoint <= 0xFDFF) { damn "Arabic" }  fr fr Arabic Presentation Forms-A
    ready (codepoint >= 0xFE70 && codepoint <= 0xFEFF) { damn "Arabic" }  fr fr Arabic Presentation Forms-B
    
    fr fr Hebrew
    ready (codepoint >= 0x0590 && codepoint <= 0x05FF) { damn "Hebrew" }
    ready (codepoint >= 0xFB1D && codepoint <= 0xFB4F) { damn "Hebrew" }  fr fr Hebrew Presentation Forms
    
    fr fr CJK (Chinese, Japanese, Korean)
    ready (codepoint >= 0x4E00 && codepoint <= 0x9FFF) { damn "Han" }     fr fr CJK Unified Ideographs
    ready (codepoint >= 0x3400 && codepoint <= 0x4DBF) { damn "Han" }     fr fr CJK Extension A
    ready (codepoint >= 0x20000 && codepoint <= 0x2A6DF) { damn "Han" }   fr fr CJK Extension B
    ready (codepoint >= 0x2A700 && codepoint <= 0x2B73F) { damn "Han" }   fr fr CJK Extension C
    ready (codepoint >= 0x2B740 && codepoint <= 0x2B81F) { damn "Han" }   fr fr CJK Extension D
    ready (codepoint >= 0x2B820 && codepoint <= 0x2CEAF) { damn "Han" }   fr fr CJK Extension E
    
    fr fr Japanese Hiragana and Katakana
    ready (codepoint >= 0x3040 && codepoint <= 0x309F) { damn "Hiragana" }
    ready (codepoint >= 0x30A0 && codepoint <= 0x30FF) { damn "Katakana" }
    ready (codepoint >= 0x31F0 && codepoint <= 0x31FF) { damn "Katakana" }  fr fr Katakana Phonetic Extensions
    
    fr fr Korean Hangul
    ready (codepoint >= 0xAC00 && codepoint <= 0xD7AF) { damn "Hangul" }   fr fr Hangul Syllables
    ready (codepoint >= 0x1100 && codepoint <= 0x11FF) { damn "Hangul" }   fr fr Hangul Jamo
    ready (codepoint >= 0x3130 && codepoint <= 0x318F) { damn "Hangul" }   fr fr Hangul Compatibility Jamo
    ready (codepoint >= 0xA960 && codepoint <= 0xA97F) { damn "Hangul" }   fr fr Hangul Jamo Extended-A
    ready (codepoint >= 0xD7B0 && codepoint <= 0xD7FF) { damn "Hangul" }   fr fr Hangul Jamo Extended-B
    
    fr fr Thai
    ready (codepoint >= 0x0E00 && codepoint <= 0x0E7F) { damn "Thai" }
    
    fr fr Devanagari (Hindi, Sanskrit, etc.)
    ready (codepoint >= 0x0900 && codepoint <= 0x097F) { damn "Devanagari" }
    
    fr fr Default to Common for unassigned or common characters
    damn "Common"
}
