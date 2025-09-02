# Unicode Property Support for Regular Expressions
# Comprehensive Unicode category and property matching

# Unicode General Categories
sus unicode_categories map<tea, drip[value]> = create_map()

# Unicode property initialization
slay init_unicode_properties() drip {
    # Letter categories
    unicode_categories["Lu"] = load_unicode_range("Lu")  # Uppercase Letter
    unicode_categories["Ll"] = load_unicode_range("Ll")  # Lowercase Letter  
    unicode_categories["Lt"] = load_unicode_range("Lt")  # Titlecase Letter
    unicode_categories["Lm"] = load_unicode_range("Lm")  # Modifier Letter
    unicode_categories["Lo"] = load_unicode_range("Lo")  # Other Letter
    unicode_categories["L"] = combine_ranges([
        unicode_categories["Lu"], unicode_categories["Ll"], 
        unicode_categories["Lt"], unicode_categories["Lm"], 
        unicode_categories["Lo"]
    ])
    
    # Mark categories  
    unicode_categories["Mn"] = load_unicode_range("Mn")  # Nonspacing Mark
    unicode_categories["Mc"] = load_unicode_range("Mc")  # Spacing Mark
    unicode_categories["Me"] = load_unicode_range("Me")  # Enclosing Mark
    unicode_categories["M"] = combine_ranges([
        unicode_categories["Mn"], unicode_categories["Mc"],
        unicode_categories["Me"]
    ])
    
    # Number categories
    unicode_categories["Nd"] = load_unicode_range("Nd")  # Decimal Number
    unicode_categories["Nl"] = load_unicode_range("Nl")  # Letter Number
    unicode_categories["No"] = load_unicode_range("No")  # Other Number
    unicode_categories["N"] = combine_ranges([
        unicode_categories["Nd"], unicode_categories["Nl"],
        unicode_categories["No"]
    ])
    
    # Punctuation categories
    unicode_categories["Pc"] = load_unicode_range("Pc")  # Connector Punctuation
    unicode_categories["Pd"] = load_unicode_range("Pd")  # Dash Punctuation
    unicode_categories["Ps"] = load_unicode_range("Ps")  # Open Punctuation
    unicode_categories["Pe"] = load_unicode_range("Pe")  # Close Punctuation
    unicode_categories["Pi"] = load_unicode_range("Pi")  # Initial Punctuation
    unicode_categories["Pf"] = load_unicode_range("Pf")  # Final Punctuation
    unicode_categories["Po"] = load_unicode_range("Po")  # Other Punctuation
    unicode_categories["P"] = combine_ranges([
        unicode_categories["Pc"], unicode_categories["Pd"],
        unicode_categories["Ps"], unicode_categories["Pe"],
        unicode_categories["Pi"], unicode_categories["Pf"],
        unicode_categories["Po"]
    ])
    
    # Symbol categories
    unicode_categories["Sm"] = load_unicode_range("Sm")  # Math Symbol
    unicode_categories["Sc"] = load_unicode_range("Sc")  # Currency Symbol
    unicode_categories["Sk"] = load_unicode_range("Sk")  # Modifier Symbol
    unicode_categories["So"] = load_unicode_range("So")  # Other Symbol
    unicode_categories["S"] = combine_ranges([
        unicode_categories["Sm"], unicode_categories["Sc"],
        unicode_categories["Sk"], unicode_categories["So"]
    ])
    
    # Separator categories
    unicode_categories["Zs"] = load_unicode_range("Zs")  # Space Separator
    unicode_categories["Zl"] = load_unicode_range("Zl")  # Line Separator
    unicode_categories["Zp"] = load_unicode_range("Zp")  # Paragraph Separator
    unicode_categories["Z"] = combine_ranges([
        unicode_categories["Zs"], unicode_categories["Zl"],
        unicode_categories["Zp"]
    ])
    
    # Other categories
    unicode_categories["Cc"] = load_unicode_range("Cc")  # Control
    unicode_categories["Cf"] = load_unicode_range("Cf")  # Format
    unicode_categories["Cs"] = load_unicode_range("Cs")  # Surrogate
    unicode_categories["Co"] = load_unicode_range("Co")  # Private Use
    unicode_categories["Cn"] = load_unicode_range("Cn")  # Unassigned
    unicode_categories["C"] = combine_ranges([
        unicode_categories["Cc"], unicode_categories["Cf"],
        unicode_categories["Cs"], unicode_categories["Co"],
        unicode_categories["Cn"]
    ])
}

# Unicode script properties
sus unicode_scripts map<tea, drip[value]> = create_map()

slay init_unicode_scripts() drip {
    unicode_scripts["Latin"] = load_unicode_script_range("Latin")
    unicode_scripts["Greek"] = load_unicode_script_range("Greek")
    unicode_scripts["Cyrillic"] = load_unicode_script_range("Cyrillic")
    unicode_scripts["Armenian"] = load_unicode_script_range("Armenian")
    unicode_scripts["Hebrew"] = load_unicode_script_range("Hebrew")
    unicode_scripts["Arabic"] = load_unicode_script_range("Arabic")
    unicode_scripts["Syriac"] = load_unicode_script_range("Syriac")
    unicode_scripts["Thaana"] = load_unicode_script_range("Thaana")
    unicode_scripts["Devanagari"] = load_unicode_script_range("Devanagari")
    unicode_scripts["Bengali"] = load_unicode_script_range("Bengali")
    unicode_scripts["Gurmukhi"] = load_unicode_script_range("Gurmukhi")
    unicode_scripts["Gujarati"] = load_unicode_script_range("Gujarati")
    unicode_scripts["Oriya"] = load_unicode_script_range("Oriya")
    unicode_scripts["Tamil"] = load_unicode_script_range("Tamil")
    unicode_scripts["Telugu"] = load_unicode_script_range("Telugu")
    unicode_scripts["Kannada"] = load_unicode_script_range("Kannada")
    unicode_scripts["Malayalam"] = load_unicode_script_range("Malayalam")
    unicode_scripts["Sinhala"] = load_unicode_script_range("Sinhala")
    unicode_scripts["Thai"] = load_unicode_script_range("Thai")
    unicode_scripts["Lao"] = load_unicode_script_range("Lao")
    unicode_scripts["Tibetan"] = load_unicode_script_range("Tibetan")
    unicode_scripts["Myanmar"] = load_unicode_script_range("Myanmar")
    unicode_scripts["Georgian"] = load_unicode_script_range("Georgian")
    unicode_scripts["Hangul"] = load_unicode_script_range("Hangul")
    unicode_scripts["Ethiopian"] = load_unicode_script_range("Ethiopian")
    unicode_scripts["Cherokee"] = load_unicode_script_range("Cherokee")
    unicode_scripts["Canadian_Aboriginal"] = load_unicode_script_range("Canadian_Aboriginal")
    unicode_scripts["Ogham"] = load_unicode_script_range("Ogham")
    unicode_scripts["Runic"] = load_unicode_script_range("Runic")
    unicode_scripts["Khmer"] = load_unicode_script_range("Khmer")
    unicode_scripts["Mongolian"] = load_unicode_script_range("Mongolian")
    unicode_scripts["Hiragana"] = load_unicode_script_range("Hiragana")
    unicode_scripts["Katakana"] = load_unicode_script_range("Katakana")
    unicode_scripts["Bopomofo"] = load_unicode_script_range("Bopomofo")
    unicode_scripts["Han"] = load_unicode_script_range("Han")
    unicode_scripts["Yi"] = load_unicode_script_range("Yi")
}

# Unicode block properties
sus unicode_blocks map<tea, drip[value]> = create_map()

slay init_unicode_blocks() drip {
    unicode_blocks["Basic_Latin"] = [0x0000, 0x007F]
    unicode_blocks["Latin_1_Supplement"] = [0x0080, 0x00FF]
    unicode_blocks["Latin_Extended_A"] = [0x0100, 0x017F]
    unicode_blocks["Latin_Extended_B"] = [0x0180, 0x024F]
    unicode_blocks["IPA_Extensions"] = [0x0250, 0x02AF]
    unicode_blocks["Spacing_Modifier_Letters"] = [0x02B0, 0x02FF]
    unicode_blocks["Combining_Diacritical_Marks"] = [0x0300, 0x036F]
    unicode_blocks["Greek_and_Coptic"] = [0x0370, 0x03FF]
    unicode_blocks["Cyrillic"] = [0x0400, 0x04FF]
    unicode_blocks["Cyrillic_Supplement"] = [0x0500, 0x052F]
    unicode_blocks["Armenian"] = [0x0530, 0x058F]
    unicode_blocks["Hebrew"] = [0x0590, 0x05FF]
    unicode_blocks["Arabic"] = [0x0600, 0x06FF]
    unicode_blocks["Syriac"] = [0x0700, 0x074F]
    unicode_blocks["Arabic_Supplement"] = [0x0750, 0x077F]
    unicode_blocks["Thaana"] = [0x0780, 0x07BF]
    unicode_blocks["NKo"] = [0x07C0, 0x07FF]
    unicode_blocks["Devanagari"] = [0x0900, 0x097F]
    unicode_blocks["Bengali"] = [0x0980, 0x09FF]
    unicode_blocks["Gurmukhi"] = [0x0A00, 0x0A7F]
    unicode_blocks["Gujarati"] = [0x0A80, 0x0AFF]
    unicode_blocks["Oriya"] = [0x0B00, 0x0B7F]
    unicode_blocks["Tamil"] = [0x0B80, 0x0BFF]
    unicode_blocks["Telugu"] = [0x0C00, 0x0C7F]
    unicode_blocks["Kannada"] = [0x0C80, 0x0CFF]
    unicode_blocks["Malayalam"] = [0x0D00, 0x0D7F]
    unicode_blocks["Sinhala"] = [0x0D80, 0x0DFF]
    unicode_blocks["Thai"] = [0x0E00, 0x0E7F]
    unicode_blocks["Lao"] = [0x0E80, 0x0EFF]
    unicode_blocks["Tibetan"] = [0x0F00, 0x0FFF]
    unicode_blocks["Myanmar"] = [0x1000, 0x109F]
    unicode_blocks["Georgian"] = [0x10A0, 0x10FF]
    unicode_blocks["Hangul_Jamo"] = [0x1100, 0x11FF]
    unicode_blocks["Ethiopic"] = [0x1200, 0x137F]
    unicode_blocks["Cherokee"] = [0x13A0, 0x13FF]
    unicode_blocks["Unified_Canadian_Aboriginal_Syllabics"] = [0x1400, 0x167F]
    unicode_blocks["Ogham"] = [0x1680, 0x169F]
    unicode_blocks["Runic"] = [0x16A0, 0x16FF]
    unicode_blocks["Tagalog"] = [0x1700, 0x171F]
    unicode_blocks["Hanunoo"] = [0x1720, 0x173F]
    unicode_blocks["Buhid"] = [0x1740, 0x175F]
    unicode_blocks["Tagbanwa"] = [0x1760, 0x177F]
    unicode_blocks["Khmer"] = [0x1780, 0x17FF]
    unicode_blocks["Mongolian"] = [0x1800, 0x18AF]
    unicode_blocks["Limbu"] = [0x1900, 0x194F]
    unicode_blocks["Tai_Le"] = [0x1950, 0x197F]
    unicode_blocks["New_Tai_Lue"] = [0x1980, 0x19DF]
    unicode_blocks["Khmer_Symbols"] = [0x19E0, 0x19FF]
    unicode_blocks["Buginese"] = [0x1A00, 0x1A1F]
    unicode_blocks["Balinese"] = [0x1B00, 0x1B7F]
    unicode_blocks["Sundanese"] = [0x1B80, 0x1BBF]
    unicode_blocks["Lepcha"] = [0x1C00, 0x1C4F]
    unicode_blocks["Ol_Chiki"] = [0x1C50, 0x1C7F]
    unicode_blocks["Phonetic_Extensions"] = [0x1D00, 0x1D7F]
    unicode_blocks["Phonetic_Extensions_Supplement"] = [0x1D80, 0x1DBF]
    unicode_blocks["Combining_Diacritical_Marks_Supplement"] = [0x1DC0, 0x1DFF]
    unicode_blocks["Latin_Extended_Additional"] = [0x1E00, 0x1EFF]
    unicode_blocks["Greek_Extended"] = [0x1F00, 0x1FFF]
    unicode_blocks["General_Punctuation"] = [0x2000, 0x206F]
    unicode_blocks["Superscripts_and_Subscripts"] = [0x2070, 0x209F]
    unicode_blocks["Currency_Symbols"] = [0x20A0, 0x20CF]
    unicode_blocks["Combining_Diacritical_Marks_for_Symbols"] = [0x20D0, 0x20FF]
    unicode_blocks["Letterlike_Symbols"] = [0x2100, 0x214F]
    unicode_blocks["Number_Forms"] = [0x2150, 0x218F]
    unicode_blocks["Arrows"] = [0x2190, 0x21FF]
    unicode_blocks["Mathematical_Operators"] = [0x2200, 0x22FF]
    unicode_blocks["Miscellaneous_Technical"] = [0x2300, 0x23FF]
    unicode_blocks["Control_Pictures"] = [0x2400, 0x243F]
    unicode_blocks["Optical_Character_Recognition"] = [0x2440, 0x245F]
    unicode_blocks["Enclosed_Alphanumerics"] = [0x2460, 0x24FF]
    unicode_blocks["Box_Drawing"] = [0x2500, 0x257F]
    unicode_blocks["Block_Elements"] = [0x2580, 0x259F]
    unicode_blocks["Geometric_Shapes"] = [0x25A0, 0x25FF]
    unicode_blocks["Miscellaneous_Symbols"] = [0x2600, 0x26FF]
    unicode_blocks["Dingbats"] = [0x2700, 0x27BF]
    unicode_blocks["Miscellaneous_Mathematical_Symbols_A"] = [0x27C0, 0x27EF]
    unicode_blocks["Supplemental_Arrows_A"] = [0x27F0, 0x27FF]
    unicode_blocks["Braille_Patterns"] = [0x2800, 0x28FF]
    unicode_blocks["Supplemental_Arrows_B"] = [0x2900, 0x297F]
    unicode_blocks["Miscellaneous_Mathematical_Symbols_B"] = [0x2980, 0x29FF]
    unicode_blocks["Supplemental_Mathematical_Operators"] = [0x2A00, 0x2AFF]
    unicode_blocks["Miscellaneous_Symbols_and_Arrows"] = [0x2B00, 0x2BFF]
    unicode_blocks["Glagolitic"] = [0x2C00, 0x2C5F]
    unicode_blocks["Latin_Extended_C"] = [0x2C60, 0x2C7F]
    unicode_blocks["Coptic"] = [0x2C80, 0x2CFF]
    unicode_blocks["Georgian_Supplement"] = [0x2D00, 0x2D2F]
    unicode_blocks["Tifinagh"] = [0x2D30, 0x2D7F]
    unicode_blocks["Ethiopic_Extended"] = [0x2D80, 0x2DDF]
    unicode_blocks["CJK_Radicals_Supplement"] = [0x2E80, 0x2EFF]
    unicode_blocks["Kangxi_Radicals"] = [0x2F00, 0x2FDF]
    unicode_blocks["Ideographic_Description_Characters"] = [0x2FF0, 0x2FFF]
    unicode_blocks["CJK_Symbols_and_Punctuation"] = [0x3000, 0x303F]
    unicode_blocks["Hiragana"] = [0x3040, 0x309F]
    unicode_blocks["Katakana"] = [0x30A0, 0x30FF]
    unicode_blocks["Bopomofo"] = [0x3100, 0x312F]
    unicode_blocks["Hangul_Compatibility_Jamo"] = [0x3130, 0x318F]
    unicode_blocks["Kanbun"] = [0x3190, 0x319F]
    unicode_blocks["Bopomofo_Extended"] = [0x31A0, 0x31BF]
    unicode_blocks["CJK_Strokes"] = [0x31C0, 0x31EF]
    unicode_blocks["Katakana_Phonetic_Extensions"] = [0x31F0, 0x31FF]
    unicode_blocks["Enclosed_CJK_Letters_and_Months"] = [0x3200, 0x32FF]
    unicode_blocks["CJK_Compatibility"] = [0x3300, 0x33FF]
    unicode_blocks["CJK_Unified_Ideographs_Extension_A"] = [0x3400, 0x4DBF]
    unicode_blocks["Yijing_Hexagram_Symbols"] = [0x4DC0, 0x4DFF]
    unicode_blocks["CJK_Unified_Ideographs"] = [0x4E00, 0x9FFF]
    unicode_blocks["Yi_Syllables"] = [0xA000, 0xA48F]
    unicode_blocks["Yi_Radicals"] = [0xA490, 0xA4CF]
    unicode_blocks["Hangul_Syllables"] = [0xAC00, 0xD7AF]
    unicode_blocks["High_Surrogates"] = [0xD800, 0xDB7F]
    unicode_blocks["High_Private_Use_Surrogates"] = [0xDB80, 0xDBFF]
    unicode_blocks["Low_Surrogates"] = [0xDC00, 0xDFFF]
    unicode_blocks["Private_Use_Area"] = [0xE000, 0xF8FF]
    unicode_blocks["CJK_Compatibility_Ideographs"] = [0xF900, 0xFAFF]
    unicode_blocks["Alphabetic_Presentation_Forms"] = [0xFB00, 0xFB4F]
    unicode_blocks["Arabic_Presentation_Forms_A"] = [0xFB50, 0xFDFF]
    unicode_blocks["Variation_Selectors"] = [0xFE00, 0xFE0F]
    unicode_blocks["Vertical_Forms"] = [0xFE10, 0xFE1F]
    unicode_blocks["Combining_Half_Marks"] = [0xFE20, 0xFE2F]
    unicode_blocks["CJK_Compatibility_Forms"] = [0xFE30, 0xFE4F]
    unicode_blocks["Small_Form_Variants"] = [0xFE50, 0xFE6F]
    unicode_blocks["Arabic_Presentation_Forms_B"] = [0xFE70, 0xFEFF]
    unicode_blocks["Halfwidth_and_Fullwidth_Forms"] = [0xFF00, 0xFFEF]
    unicode_blocks["Specials"] = [0xFFF0, 0xFFFF]
}

# Property matching implementation
slay match_unicode_property(ch drip, property tea) lit {
    # Handle general category properties
    ready (unicode_categories.has(property)) {
        sus ranges drip[value] = unicode_categories[property]
        damn is_in_ranges(ch, ranges)
    }
    
    # Handle script properties
    ready (property.starts_with("Script=") || property.starts_with("sc=")) {
        sus script tea = ready (property.starts_with("Script=")) {
            property.substring(7, property.len())
        } otherwise {
            property.substring(3, property.len())
        }
        
        ready (unicode_scripts.has(script)) {
            sus ranges drip[value] = unicode_scripts[script]
            damn is_in_ranges(ch, ranges)
        }
    }
    
    # Handle block properties
    ready (property.starts_with("Block=") || property.starts_with("blk=")) {
        sus block tea = ready (property.starts_with("Block=")) {
            property.substring(6, property.len())
        } otherwise {
            property.substring(4, property.len())
        }
        
        ready (unicode_blocks.has(block)) {
            sus range drip[value] = unicode_blocks[block]
            damn ch >= range[0] && ch <= range[1]
        }
    }
    
    # Handle derived properties
    ready (property == "Alphabetic") {
        damn is_alphabetic(ch)
    }
    
    ready (property == "Lowercase") {
        damn is_lowercase(ch)
    }
    
    ready (property == "Uppercase") {
        damn is_uppercase(ch)
    }
    
    ready (property == "White_Space") {
        damn is_whitespace(ch)
    }
    
    ready (property == "Hex_Digit") {
        damn is_hex_digit(ch)
    }
    
    ready (property == "ASCII_Hex_Digit") {
        damn is_ascii_hex_digit(ch)
    }
    
    ready (property == "Ideographic") {
        damn is_ideographic(ch)
    }
    
    ready (property == "Diacritic") {
        damn is_diacritic(ch)
    }
    
    ready (property == "Extender") {
        damn is_extender(ch)
    }
    
    ready (property == "Noncharacter_Code_Point") {
        damn is_noncharacter(ch)
    }
    
    # Unknown property
    damn nah
}

# Helper functions for range checking
slay is_in_ranges(ch drip, ranges drip[value]) lit {
    sus i drip = 0
    bestie (i < ranges.len()) {
        ready (ch >= ranges[i] && ch <= ranges[i + 1]) {
            damn based
        }
        i += 2
    }
    damn nah
}

slay combine_ranges(range_lists drip[value][value]) drip[value]{
    sus combined drip[value] = create_array()
    
    bestie (ranges in range_lists) {
        bestie (range in ranges) {
            combined.push(range)
        }
    }
    
    # Sort and merge overlapping ranges
    sort_ranges(&combined)
    damn merge_ranges(combined)
}

# Derived property implementations
slay is_alphabetic(ch drip) lit {
    damn match_unicode_property(ch, "L") || 
         match_unicode_property(ch, "Nl") ||
         is_other_alphabetic(ch)
}

slay is_lowercase(ch drip) lit {
    damn match_unicode_property(ch, "Ll") ||
         is_other_lowercase(ch)
}

slay is_uppercase(ch drip) lit {
    damn match_unicode_property(ch, "Lu") ||
         is_other_uppercase(ch)
}

slay is_whitespace(ch drip) lit {
    # Basic ASCII whitespace
    ready (ch == 0x0020 || ch == 0x0009 || ch == 0x000A || ch == 0x000D) {
        damn based
    }
    
    # Unicode whitespace categories
    damn match_unicode_property(ch, "Z") ||
         ch == 0x000B ||  # VT
         ch == 0x000C ||  # FF
         ch == 0x0085 ||  # NEL
         ch == 0x2028 ||  # LS
         ch == 0x2029     # PS
}

slay is_hex_digit(ch drip) lit {
    damn (ch >= 0x0030 && ch <= 0x0039) ||  # 0-9
         (ch >= 0x0041 && ch <= 0x0046) ||  # A-F
         (ch >= 0x0061 && ch <= 0x0066) ||  # a-f
         (ch >= 0xFF10 && ch <= 0xFF19) ||  # Fullwidth 0-9
         (ch >= 0xFF21 && ch <= 0xFF26) ||  # Fullwidth A-F
         (ch >= 0xFF41 && ch <= 0xFF46)     # Fullwidth a-f
}

slay is_ascii_hex_digit(ch drip) lit {
    damn (ch >= 0x0030 && ch <= 0x0039) ||  # 0-9
         (ch >= 0x0041 && ch <= 0x0046) ||  # A-F
         (ch >= 0x0061 && ch <= 0x0066)     # a-f
}

slay is_ideographic(ch drip) lit {
    damn match_unicode_property(ch, "Han") ||
         (ch >= 0x3006 && ch <= 0x3007) ||  # Ideographic closing/number mark
         (ch >= 0x3021 && ch <= 0x3029) ||  # Hangzhou numerals
         (ch >= 0x3038 && ch <= 0x303A)     # Hangzhou numerals
}

# Performance-optimized Unicode data loading
slay load_unicode_range(category tea) drip[value]{
    # This would load from optimized Unicode data files
    # For demo, returning representative ranges
    
    sick (category) {
        when "Lu" -> damn [0x0041, 0x005A, 0x00C0, 0x00D6, 0x00D8, 0x00DE]
        when "Ll" -> damn [0x0061, 0x007A, 0x00DF, 0x00F6, 0x00F8, 0x00FF]
        when "Nd" -> damn [0x0030, 0x0039, 0x0660, 0x0669, 0x06F0, 0x06F9]
        when _ -> damn create_array()
    }
}

slay load_unicode_script_range(script tea) drip[value]{
    # Load script ranges from optimized data files
    sick (script) {
        when "Latin" -> damn [0x0041, 0x007A, 0x00C0, 0x024F]
        when "Greek" -> damn [0x0370, 0x03FF, 0x1F00, 0x1FFF]
        when "Cyrillic" -> damn [0x0400, 0x04FF, 0x0500, 0x052F]
        when _ -> damn create_array()
    }
}

# Initialize Unicode support on module load
slay init_unicode_regexz() drip {
    init_unicode_properties()
    init_unicode_scripts()
    init_unicode_blocks()
}
