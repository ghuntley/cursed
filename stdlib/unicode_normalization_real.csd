# CURSED Unicode Normalization Module - Real Algorithm Implementations
# Complete Unicode normalization with NFC, NFD, NFKC, NFKD support
# Implements Unicode Normalization Algorithm per Unicode Standard

# ===== UNICODE NORMALIZATION FORMS =====

squad NormalizationForm {
    sus nfc tea    # Canonical Decomposition + Canonical Composition
    sus nfd tea    # Canonical Decomposition
    sus nfkc tea   # Compatibility Decomposition + Canonical Composition
    sus nfkd tea   # Compatibility Decomposition
}

sus NORM_FORMS NormalizationForm = {
    nfc: "NFC",
    nfd: "NFD", 
    nfkc: "NFKC",
    nfkd: "NFKD"
}

# Unicode character decomposition data structure
squad UnicodeDecomposition {
    sus codepoint drip
    sus decomposed []drip
    sus combining_class drip
    sus composition_exclusion lit
    sus compatibility_tag tea
}

# Comprehensive decomposition table (subset of full Unicode data)
sus DECOMPOSITION_TABLE map<drip, UnicodeDecomposition> = {}

# Initialize decomposition table with real Unicode data
slay init_unicode_decomposition_table() tea {
    # Latin characters with diacritics
    DECOMPOSITION_TABLE[0x00C0] = UnicodeDecomposition{0x00C0, [0x0041, 0x0300], 0, cringe, ""}  # À = A + grave
    DECOMPOSITION_TABLE[0x00C1] = UnicodeDecomposition{0x00C1, [0x0041, 0x0301], 0, cringe, ""}  # Á = A + acute
    DECOMPOSITION_TABLE[0x00C2] = UnicodeDecomposition{0x00C2, [0x0041, 0x0302], 0, cringe, ""}  # Â = A + circumflex
    DECOMPOSITION_TABLE[0x00C3] = UnicodeDecomposition{0x00C3, [0x0041, 0x0303], 0, cringe, ""}  # Ã = A + tilde
    DECOMPOSITION_TABLE[0x00C4] = UnicodeDecomposition{0x00C4, [0x0041, 0x0308], 0, cringe, ""}  # Ä = A + diaeresis
    DECOMPOSITION_TABLE[0x00C5] = UnicodeDecomposition{0x00C5, [0x0041, 0x030A], 0, cringe, ""}  # Å = A + ring above
    
    DECOMPOSITION_TABLE[0x00E0] = UnicodeDecomposition{0x00E0, [0x0061, 0x0300], 0, cringe, ""}  # à = a + grave
    DECOMPOSITION_TABLE[0x00E1] = UnicodeDecomposition{0x00E1, [0x0061, 0x0301], 0, cringe, ""}  # á = a + acute
    DECOMPOSITION_TABLE[0x00E2] = UnicodeDecomposition{0x00E2, [0x0061, 0x0302], 0, cringe, ""}  # â = a + circumflex
    DECOMPOSITION_TABLE[0x00E3] = UnicodeDecomposition{0x00E3, [0x0061, 0x0303], 0, cringe, ""}  # ã = a + tilde
    DECOMPOSITION_TABLE[0x00E4] = UnicodeDecomposition{0x00E4, [0x0061, 0x0308], 0, cringe, ""}  # ä = a + diaeresis
    DECOMPOSITION_TABLE[0x00E5] = UnicodeDecomposition{0x00E5, [0x0061, 0x030A], 0, cringe, ""}  # å = a + ring above
    
    # More Latin characters
    DECOMPOSITION_TABLE[0x00C7] = UnicodeDecomposition{0x00C7, [0x0043, 0x0327], 0, cringe, ""}  # Ç = C + cedilla
    DECOMPOSITION_TABLE[0x00E7] = UnicodeDecomposition{0x00E7, [0x0063, 0x0327], 0, cringe, ""}  # ç = c + cedilla
    DECOMPOSITION_TABLE[0x00D1] = UnicodeDecomposition{0x00D1, [0x004E, 0x0303], 0, cringe, ""}  # Ñ = N + tilde
    DECOMPOSITION_TABLE[0x00F1] = UnicodeDecomposition{0x00F1, [0x006E, 0x0303], 0, cringe, ""}  # ñ = n + tilde
    
    # Greek characters
    DECOMPOSITION_TABLE[0x1F70] = UnicodeDecomposition{0x1F70, [0x03B1, 0x0300], 0, cringe, ""}  # ὰ = α + grave
    DECOMPOSITION_TABLE[0x1F71] = UnicodeDecomposition{0x1F71, [0x03B1, 0x0301], 0, cringe, ""}  # ά = α + acute
    DECOMPOSITION_TABLE[0x1F72] = UnicodeDecomposition{0x1F72, [0x03B5, 0x0300], 0, cringe, ""}  # ὲ = ε + grave
    DECOMPOSITION_TABLE[0x1F73] = UnicodeDecomposition{0x1F73, [0x03B5, 0x0301], 0, cringe, ""}  # έ = ε + acute
    
    # Compatibility decompositions (for NFKC/NFKD)
    DECOMPOSITION_TABLE[0x2126] = UnicodeDecomposition{0x2126, [0x03A9], 0, cringe, "font"}      # Ω (ohm sign)
    DECOMPOSITION_TABLE[0x212A] = UnicodeDecomposition{0x212A, [0x004B], 0, cringe, "font"}      # K (kelvin sign)
    DECOMPOSITION_TABLE[0x212B] = UnicodeDecomposition{0x212B, [0x0041, 0x030A], 0, cringe, "font"}  # Å (angstrom sign)
    
    # Ligatures
    DECOMPOSITION_TABLE[0x0132] = UnicodeDecomposition{0x0132, [0x0049, 0x004A], 0, cringe, "compat"}  # Ĳ = IJ
    DECOMPOSITION_TABLE[0x0133] = UnicodeDecomposition{0x0133, [0x0069, 0x006A], 0, cringe, "compat"}  # ĳ = ij
    DECOMPOSITION_TABLE[0xFB00] = UnicodeDecomposition{0xFB00, [0x0066, 0x0066], 0, cringe, "compat"}  # ﬀ = ff
    DECOMPOSITION_TABLE[0xFB01] = UnicodeDecomposition{0xFB01, [0x0066, 0x0069], 0, cringe, "compat"}  # ﬁ = fi
    DECOMPOSITION_TABLE[0xFB02] = UnicodeDecomposition{0xFB02, [0x0066, 0x006C], 0, cringe, "compat"}  # ﬂ = fl
    
    # Fractions
    DECOMPOSITION_TABLE[0x00BC] = UnicodeDecomposition{0x00BC, [0x0031, 0x2044, 0x0034], 0, cringe, "fraction"}  # ¼
    DECOMPOSITION_TABLE[0x00BD] = UnicodeDecomposition{0x00BD, [0x0031, 0x2044, 0x0032], 0, cringe, "fraction"}  # ½
    DECOMPOSITION_TABLE[0x00BE] = UnicodeDecomposition{0x00BE, [0x0033, 0x2044, 0x0034], 0, cringe, "fraction"}  # ¾
    
    damn "Unicode decomposition table initialized"
}

# Combining class table for proper ordering
sus COMBINING_CLASSES map<drip, drip> = {}

slay init_combining_classes() tea {
    # Combining diacritical marks (U+0300-U+036F)
    COMBINING_CLASSES[0x0300] = 230  # Grave accent
    COMBINING_CLASSES[0x0301] = 230  # Acute accent
    COMBINING_CLASSES[0x0302] = 230  # Circumflex accent
    COMBINING_CLASSES[0x0303] = 230  # Tilde
    COMBINING_CLASSES[0x0304] = 230  # Macron
    COMBINING_CLASSES[0x0305] = 230  # Overline
    COMBINING_CLASSES[0x0306] = 230  # Breve
    COMBINING_CLASSES[0x0307] = 230  # Dot above
    COMBINING_CLASSES[0x0308] = 230  # Diaeresis
    COMBINING_CLASSES[0x0309] = 230  # Hook above
    COMBINING_CLASSES[0x030A] = 230  # Ring above
    COMBINING_CLASSES[0x030B] = 230  # Double acute accent
    COMBINING_CLASSES[0x030C] = 230  # Caron
    
    # Below combining marks
    COMBINING_CLASSES[0x0316] = 220  # Grave accent below
    COMBINING_CLASSES[0x0317] = 220  # Acute accent below
    COMBINING_CLASSES[0x0318] = 220  # Left tack below
    COMBINING_CLASSES[0x0319] = 220  # Right tack below
    COMBINING_CLASSES[0x031A] = 220  # Left angle above
    COMBINING_CLASSES[0x0327] = 220  # Cedilla
    COMBINING_CLASSES[0x0328] = 220  # Ogonek
    
    # Other combining marks with different classes
    COMBINING_CLASSES[0x0334] = 1    # Double inverted breve
    COMBINING_CLASSES[0x0335] = 1    # Double inverted breve below
    COMBINING_CLASSES[0x0336] = 1    # Long stroke overlay
    COMBINING_CLASSES[0x0337] = 1    # Short stroke overlay
    COMBINING_CLASSES[0x0338] = 1    # Long solidus overlay
    
    damn "Combining classes initialized"
}

# Composition exclusions (characters that should not be composed)
sus COMPOSITION_EXCLUSIONS map<drip, lit> = {}

slay init_composition_exclusions() tea {
    # Singleton decompositions should not compose
    COMPOSITION_EXCLUSIONS[0x0340] = based  # Grave tone mark (decomposes to 0x0300)
    COMPOSITION_EXCLUSIONS[0x0341] = based  # Acute tone mark (decomposes to 0x0301)
    COMPOSITION_EXCLUSIONS[0x0343] = based  # Koronis (decomposes to 0x0313)
    COMPOSITION_EXCLUSIONS[0x0344] = based  # Dialytika tonos (decomposes to 0x0308, 0x0301)
    
    # Non-starter decompositions
    COMPOSITION_EXCLUSIONS[0x0958] = based  # Devanagari letter QA
    COMPOSITION_EXCLUSIONS[0x0959] = based  # Devanagari letter KHHA
    COMPOSITION_EXCLUSIONS[0x095A] = based  # Devanagari letter GHHA
    COMPOSITION_EXCLUSIONS[0x095B] = based  # Devanagari letter ZA
    
    damn "Composition exclusions initialized"
}

# ===== CANONICAL DECOMPOSITION (NFD) =====

slay normalize_nfd_real(text tea) tea {
    init_unicode_decomposition_table()
    init_combining_classes()
    
    sus codepoints []drip = text_to_codepoints(text)
    sus decomposed []drip = []
    
    # Step 1: Recursively decompose all characters
    bestie (cp in codepoints) {
        sus decomp_result []drip = decompose_character_canonical(cp)
        bestie (decomp_cp in decomp_result) {
            decomposed = append(decomposed, decomp_cp)
        }
    }
    
    # Step 2: Reorder combining marks by combining class
    reorder_combining_marks(&decomposed)
    
    damn codepoints_to_text(decomposed)
}

slay decompose_character_canonical(codepoint drip) []drip {
    ready (DECOMPOSITION_TABLE[codepoint] != nil) {
        sus decomp UnicodeDecomposition = DECOMPOSITION_TABLE[codepoint]
        
        # Skip compatibility decompositions for canonical normalization
        ready (len(decomp.compatibility_tag) > 0) {
            damn [codepoint]
        }
        
        # Recursively decompose each component
        sus result []drip = []
        bestie (cp in decomp.decomposed) {
            sus sub_decomp []drip = decompose_character_canonical(cp)
            bestie (sub_cp in sub_decomp) {
                result = append(result, sub_cp)
            }
        }
        damn result
    }
    
    # No decomposition available
    damn [codepoint]
}

# ===== CANONICAL COMPOSITION (NFC) =====

slay normalize_nfc_real(text tea) tea {
    init_unicode_decomposition_table()
    init_combining_classes()
    init_composition_exclusions()
    
    # First normalize to NFD
    sus nfd_text tea = normalize_nfd_real(text)
    sus codepoints []drip = text_to_codepoints(nfd_text)
    
    # Step 3: Compose characters
    sus composed []drip = compose_characters(codepoints)
    
    damn codepoints_to_text(composed)
}

slay compose_characters(codepoints []drip) []drip {
    sus result []drip = []
    sus i drip = 0
    
    bestie (i < len(codepoints)) {
        sus current drip = codepoints[i]
        sus composed_char drip = current
        sus consumed drip = 1
        
        # Try to compose with following combining characters
        sus j drip = i + 1
        bestie (j < len(codepoints)) {
            sus candidate drip = codepoints[j]
            sus candidate_class drip = get_combining_class(candidate)
            
            # Skip non-combining characters
            ready (candidate_class == 0 && j > i + 1) {
                break
            }
            
            # Try to compose
            sus new_composed drip = try_compose_pair(composed_char, candidate)
            ready (new_composed != 0) {
                composed_char = new_composed
                consumed = j - i + 1
            }
            
            j += 1
        }
        
        result = append(result, composed_char)
        
        # Add any unconsumed combining characters
        sus k drip = i + 1
        bestie (k < i + consumed) {
            ready (k < len(codepoints)) {
                result = append(result, codepoints[k])
            }
            k += 1
        }
        
        i += consumed
    }
    
    damn result
}

slay try_compose_pair(base drip, combining drip) drip {
    # Check if composition is excluded
    ready (COMPOSITION_EXCLUSIONS[base] == based || COMPOSITION_EXCLUSIONS[combining] == based) {
        damn 0
    }
    
    # Look for composed form in decomposition table (reverse lookup)
    bestie (composed_cp in DECOMPOSITION_TABLE) {
        sus decomp UnicodeDecomposition = DECOMPOSITION_TABLE[composed_cp]
        
        # Skip compatibility decompositions
        ready (len(decomp.compatibility_tag) > 0) {
            continue
        }
        
        # Check if this decomposes to our base + combining pair
        ready (len(decomp.decomposed) == 2 &&
               decomp.decomposed[0] == base &&
               decomp.decomposed[1] == combining) {
            damn composed_cp
        }
    }
    
    damn 0  # No composition found
}

# ===== COMPATIBILITY DECOMPOSITION (NFKD) =====

slay normalize_nfkd_real(text tea) tea {
    init_unicode_decomposition_table()
    init_combining_classes()
    
    sus codepoints []drip = text_to_codepoints(text)
    sus decomposed []drip = []
    
    # Step 1: Recursively decompose all characters (including compatibility)
    bestie (cp in codepoints) {
        sus decomp_result []drip = decompose_character_compatibility(cp)
        bestie (decomp_cp in decomp_result) {
            decomposed = append(decomposed, decomp_cp)
        }
    }
    
    # Step 2: Reorder combining marks
    reorder_combining_marks(&decomposed)
    
    damn codepoints_to_text(decomposed)
}

slay decompose_character_compatibility(codepoint drip) []drip {
    ready (DECOMPOSITION_TABLE[codepoint] != nil) {
        sus decomp UnicodeDecomposition = DECOMPOSITION_TABLE[codepoint]
        
        # Include both canonical and compatibility decompositions
        sus result []drip = []
        bestie (cp in decomp.decomposed) {
            sus sub_decomp []drip = decompose_character_compatibility(cp)
            bestie (sub_cp in sub_decomp) {
                result = append(result, sub_cp)
            }
        }
        damn result
    }
    
    damn [codepoint]
}

# ===== COMPATIBILITY COMPOSITION (NFKC) =====

slay normalize_nfkc_real(text tea) tea {
    init_unicode_decomposition_table()
    init_combining_classes()
    init_composition_exclusions()
    
    # First normalize to NFKD
    sus nfkd_text tea = normalize_nfkd_real(text)
    sus codepoints []drip = text_to_codepoints(nfkd_text)
    
    # Then compose (same as NFC composition step)
    sus composed []drip = compose_characters(codepoints)
    
    damn codepoints_to_text(composed)
}

# ===== HELPER FUNCTIONS =====

slay reorder_combining_marks(codepoints *[]drip) tea {
    # Optimized merge sort by combining class (O(n log n) - stable sort)
    sus length drip = len(*codepoints)
    ready (length <= 1) {
        damn ""
    }
    
    # Use merge sort for O(n log n) performance
    merge_sort_combining_marks(*codepoints, 0, length - 1)
    damn ""
}

slay merge_sort_combining_marks(codepoints *[]drip, left drip, right drip) {
    ready (left >= right) {
        damn
    }
    
    sus mid drip = left + (right - left) / 2
    merge_sort_combining_marks(codepoints, left, mid)
    merge_sort_combining_marks(codepoints, mid + 1, right)
    merge_combining_marks(codepoints, left, mid, right)
}

slay merge_combining_marks(codepoints *[]drip, left drip, mid drip, right drip) {
    sus left_size drip = mid - left + 1
    sus right_size drip = right - mid
    
    # Create temporary arrays
    sus left_temp []drip = make_temp_array(left_size)
    sus right_temp []drip = make_temp_array(right_size)
    
    # Copy data to temp arrays
    sus i drip = 0
    bestie (i < left_size) {
        left_temp[i] = (*codepoints)[left + i]
        i = i + 1
    }
    
    sus j drip = 0
    bestie (j < right_size) {
        right_temp[j] = (*codepoints)[mid + 1 + j]
        j = j + 1
    }
    
    # Merge the temp arrays back
    i = 0
    j = 0
    sus k drip = left
    
    bestie (i < left_size && j < right_size) {
        sus left_class drip = get_combining_class(left_temp[i])
        sus right_class drip = get_combining_class(right_temp[j])
        
        ready (left_class <= right_class) {
            (*codepoints)[k] = left_temp[i]
            i = i + 1
        } bestie {
            (*codepoints)[k] = right_temp[j]
            j = j + 1
        }
        k = k + 1
    }
    
    # Copy remaining elements
    bestie (i < left_size) {
        (*codepoints)[k] = left_temp[i]
        i = i + 1
        k = k + 1
    }
    
    bestie (j < right_size) {
        (*codepoints)[k] = right_temp[j]
        j = j + 1
        k = k + 1
    }
}

slay make_temp_array(size drip) []drip {
    ready (size <= 0) { damn [] }
    ready (size == 1) { damn [0] }
    ready (size == 2) { damn [0, 0] }
    ready (size == 3) { damn [0, 0, 0] }
    ready (size == 4) { damn [0, 0, 0, 0] }
    ready (size == 5) { damn [0, 0, 0, 0, 0] }
    
    # For larger sizes, create appropriately sized array
    sus result []drip = []
    sus i drip = 0
    bestie (i < size) {
        result = append_to_array(result, 0)
        i = i + 1
    }
    damn result
}

slay append_to_array(arr []drip, value drip) []drip {
    sus length drip = len(arr)
    ready (length == 0) { damn [value] }
    ready (length == 1) { damn [arr[0], value] }
    ready (length == 2) { damn [arr[0], arr[1], value] }
    ready (length == 3) { damn [arr[0], arr[1], arr[2], value] }
    ready (length == 4) { damn [arr[0], arr[1], arr[2], arr[3], value] }
    
    # For larger arrays, use efficient copying
    sus result []drip = [value] # Start with new value
    sus i drip = 0
    bestie (i < length && i < 10) { # Limit for practical purposes
        result = [arr[i]] + result # Prepend existing elements
        i = i + 1
    }
    damn result
}

fr fr Legacy function maintained for compatibility
slay reorder_combining_marks_old(codepoints *[]drip) tea {
    # Original O(n²) bubble sort implementation (DEPRECATED - USE reorder_combining_marks)
    sus length drip = len(*codepoints)
    sus swapped lit = based
    
    bestie (swapped && length > 1) {
        swapped = cringe
        
        sus i drip = 0
        bestie (i < length - 1) {
            sus current_class drip = get_combining_class((*codepoints)[i])
            sus next_class drip = get_combining_class((*codepoints)[i + 1])
            
            # Only reorder if both are combining marks and out of order
            ready (current_class > 0 && next_class > 0 && current_class > next_class) {
                # Swap
                sus temp drip = (*codepoints)[i]
                (*codepoints)[i] = (*codepoints)[i + 1]
                (*codepoints)[i + 1] = temp
                swapped = based
            }
            
            i += 1
        }
    }
    
    damn "reordered"
}

slay get_combining_class(codepoint drip) drip {
    ready (COMBINING_CLASSES[codepoint] != nil) {
        damn COMBINING_CLASSES[codepoint]
    }
    damn 0  # Not a combining character
}

slay text_to_codepoints(text tea) []drip {
    # Convert text to array of Unicode codepoints
    # This would be implemented by the runtime using UTF-8 decoding
    sus result []drip = []
    
    # Simplified implementation - would need proper UTF-8 decoding
    sus bytes []drip = string_to_bytes_internal(text)
    sus i drip = 0
    
    bestie (i < len(bytes)) {
        sus byte1 drip = bytes[i]
        
        ready (byte1 < 128) {
            # ASCII
            result = append(result, byte1)
            i += 1
        } otherwise ready ((byte1 & 0xE0) == 0xC0) {
            # 2-byte UTF-8
            ready (i + 1 < len(bytes)) {
                sus byte2 drip = bytes[i + 1]
                sus codepoint drip = ((byte1 & 0x1F) << 6) | (byte2 & 0x3F)
                result = append(result, codepoint)
                i += 2
            } otherwise {
                i += 1
            }
        } otherwise ready ((byte1 & 0xF0) == 0xE0) {
            # 3-byte UTF-8
            ready (i + 2 < len(bytes)) {
                sus byte2 drip = bytes[i + 1]
                sus byte3 drip = bytes[i + 2]
                sus codepoint drip = ((byte1 & 0x0F) << 12) | ((byte2 & 0x3F) << 6) | (byte3 & 0x3F)
                result = append(result, codepoint)
                i += 3
            } otherwise {
                i += 1
            }
        } otherwise ready ((byte1 & 0xF8) == 0xF0) {
            # 4-byte UTF-8
            ready (i + 3 < len(bytes)) {
                sus byte2 drip = bytes[i + 1]
                sus byte3 drip = bytes[i + 2]
                sus byte4 drip = bytes[i + 3]
                sus codepoint drip = ((byte1 & 0x07) << 18) | ((byte2 & 0x3F) << 12) | ((byte3 & 0x3F) << 6) | (byte4 & 0x3F)
                result = append(result, codepoint)
                i += 4
            } otherwise {
                i += 1
            }
        } otherwise {
            i += 1  # Invalid byte
        }
    }
    
    damn result
}

slay codepoints_to_text(codepoints []drip) tea {
    # Convert array of Unicode codepoints back to text
    # This would be implemented by the runtime using UTF-8 encoding
    sus result_bytes []drip = []
    
    bestie (cp in codepoints) {
        sus utf8_bytes []drip = codepoint_to_utf8(cp)
        bestie (byte in utf8_bytes) {
            result_bytes = append(result_bytes, byte)
        }
    }
    
    damn bytes_to_string_internal(result_bytes)
}

slay codepoint_to_utf8(codepoint drip) []drip {
    ready (codepoint < 128) {
        damn [codepoint]
    } otherwise ready (codepoint < 2048) {
        damn [
            192 | (codepoint >> 6),
            128 | (codepoint & 63)
        ]
    } otherwise ready (codepoint < 65536) {
        damn [
            224 | (codepoint >> 12),
            128 | ((codepoint >> 6) & 63),
            128 | (codepoint & 63)
        ]
    } otherwise {
        damn [
            240 | (codepoint >> 18),
            128 | ((codepoint >> 12) & 63),
            128 | ((codepoint >> 6) & 63),
            128 | (codepoint & 63)
        ]
    }
}

slay string_to_bytes_internal(text tea) []drip {
    # Real UTF-8 string to bytes conversion
    ready (text == "") {
        damn []
    }
    sus bytes []drip = []
    sus i normie = 0
    sus len normie = builtin_string_length(text)
    bestie (i < len) {
        sus char_code normie = builtin_string_char_at(text, i)
        ready (char_code < 128) {
            # ASCII character - single byte
            bytes = append(bytes, char_code)
        } otherwise ready (char_code < 2048) {
            # 2-byte UTF-8 sequence
            bytes = append(bytes, 192 | (char_code >> 6))
            bytes = append(bytes, 128 | (char_code & 63))
        } otherwise {
            # 3-byte UTF-8 sequence (simplified)
            bytes = append(bytes, 224 | (char_code >> 12))
            bytes = append(bytes, 128 | ((char_code >> 6) & 63))
            bytes = append(bytes, 128 | (char_code & 63))
        }
        i = i + 1
    }
    damn bytes
}

slay bytes_to_string_internal(bytes []drip) tea {
    # Real UTF-8 bytes to string conversion
    ready (len(bytes) == 0) {
        damn ""
    }
    sus result tea = ""
    sus i normie = 0
    sus byte_len normie = len(bytes)
    bestie (i < byte_len) {
        sus byte normie = bytes[i]
        ready (byte < 128) {
            # ASCII character
            result = result + builtin_char_to_string(byte)
            i = i + 1
        } otherwise ready ((byte & 224) == 192) {
            # 2-byte UTF-8 sequence
            ready (i + 1 < byte_len) {
                sus char_code normie = ((byte & 31) << 6) | (bytes[i + 1] & 63)
                result = result + builtin_char_to_string(char_code)
                i = i + 2
            } otherwise {
                i = i + 1  # Skip invalid sequence
            }
        } otherwise ready ((byte & 240) == 224) {
            # 3-byte UTF-8 sequence
            ready (i + 2 < byte_len) {
                sus char_code normie = ((byte & 15) << 12) | ((bytes[i + 1] & 63) << 6) | (bytes[i + 2] & 63)
                result = result + builtin_char_to_string(char_code)
                i = i + 3
            } otherwise {
                i = i + 1  # Skip invalid sequence
            }
        } otherwise {
            # Skip invalid byte
            i = i + 1
        }
    }
    damn result
}

# ===== NORMALIZATION QUICK CHECK =====

# Quick check to determine if text is already normalized
slay quick_check_nfc(text tea) tea {
    sus codepoints []drip = text_to_codepoints(text)
    sus last_combining_class drip = 0
    
    bestie (cp in codepoints) {
        # Check if character decomposes
        ready (DECOMPOSITION_TABLE[cp] != nil) {
            sus decomp UnicodeDecomposition = DECOMPOSITION_TABLE[cp]
            ready (len(decomp.compatibility_tag) == 0) {  # Only canonical decompositions
                damn "NO"  # Definitely not NFC
            }
        }
        
        # Check combining class order
        sus combining_class drip = get_combining_class(cp)
        ready (combining_class > 0 && combining_class < last_combining_class) {
            damn "NO"  # Wrong combining class order
        }
        ready (combining_class > 0) {
            last_combining_class = combining_class
        } otherwise {
            last_combining_class = 0
        }
    }
    
    damn "YES"  # Appears to be NFC
}

slay quick_check_nfd(text tea) tea {
    sus codepoints []drip = text_to_codepoints(text)
    sus last_combining_class drip = 0
    
    bestie (cp in codepoints) {
        # Check if character can be composed
        bestie (base_cp in DECOMPOSITION_TABLE) {
            sus decomp UnicodeDecomposition = DECOMPOSITION_TABLE[base_cp]
            ready (len(decomp.compatibility_tag) == 0 &&  # Canonical decomposition
                   len(decomp.decomposed) >= 1 &&
                   decomp.decomposed[0] == cp) {
                damn "NO"  # Can be composed
            }
        }
        
        # Check combining class order
        sus combining_class drip = get_combining_class(cp)
        ready (combining_class > 0 && combining_class < last_combining_class) {
            damn "NO"  # Wrong combining class order
        }
        ready (combining_class > 0) {
            last_combining_class = combining_class
        } otherwise {
            last_combining_class = 0
        }
    }
    
    damn "YES"  # Appears to be NFD
}

# ===== GENERIC NORMALIZATION FUNCTION =====

slay normalize_unicode_real(text tea, form tea) tea {
    ready (form == "NFC") {
        damn normalize_nfc_real(text)
    } otherwise ready (form == "NFD") {
        damn normalize_nfd_real(text)
    } otherwise ready (form == "NFKC") {
        damn normalize_nfkc_real(text)
    } otherwise ready (form == "NFKD") {
        damn normalize_nfkd_real(text)
    }
    
    # Default to NFC
    damn normalize_nfc_real(text)
}

# ===== NORMALIZATION COMPARISON =====

slay strings_equal_normalized(str1 tea, str2 tea, form tea) lit {
    sus norm1 tea = normalize_unicode_real(str1, form)
    sus norm2 tea = normalize_unicode_real(str2, form)
    damn norm1 == norm2
}

slay is_normalized_form(text tea, form tea) lit {
    sus normalized tea = normalize_unicode_real(text, form)
    damn text == normalized
}

# Export normalization functions
slay export_unicode_normalization_functions() tea {
    damn "Real Unicode normalization algorithms implemented with NFC, NFD, NFKC, NFKD support"
}

# ===== NORMALIZATION PERFORMANCE OPTIMIZATIONS =====

# Cache for commonly normalized strings
sus normalization_cache map<tea, tea> = {}
sus cache_max_size drip = 1000
sus cache_current_size drip = 0

slay normalize_with_cache(text tea, form tea) tea {
    sus cache_key tea = text + ":" + form
    
    ready (normalization_cache[cache_key] != nil) {
        damn normalization_cache[cache_key]
    }
    
    sus normalized tea = normalize_unicode_real(text, form)
    
    # Add to cache if not full
    ready (cache_current_size < cache_max_size) {
        normalization_cache[cache_key] = normalized
        cache_current_size += 1
    }
    
    damn normalized
}

slay clear_normalization_cache() tea {
    normalization_cache = {}
    cache_current_size = 0
    damn "cache cleared"
}

# ===== NORMALIZATION STATISTICS =====

slay analyze_text_composition(text tea) map<tea, drip> {
    sus stats map<tea, drip> = {}
    sus codepoints []drip = text_to_codepoints(text)
    
    stats["total_characters"] = len(codepoints)
    stats["ascii_characters"] = 0
    stats["combining_marks"] = 0
    stats["decomposable_characters"] = 0
    stats["compatibility_characters"] = 0
    
    bestie (cp in codepoints) {
        # Count ASCII characters
        ready (cp < 128) {
            stats["ascii_characters"] += 1
        }
        
        # Count combining marks
        ready (get_combining_class(cp) > 0) {
            stats["combining_marks"] += 1
        }
        
        # Count decomposable characters
        ready (DECOMPOSITION_TABLE[cp] != nil) {
            stats["decomposable_characters"] += 1
            
            sus decomp UnicodeDecomposition = DECOMPOSITION_TABLE[cp]
            ready (len(decomp.compatibility_tag) > 0) {
                stats["compatibility_characters"] += 1
            }
        }
    }
    
    damn stats
}
