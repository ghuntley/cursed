// CURSED Compression Module
// Pure CURSED implementation for data compression and decompression

yeet "string"

// Run-length encoding compression
slay rle_compress(data tea) tea {
    vibes string_len(data) == 0 {
        damn ""
    }
    
    sus result tea = ""
    sus current_char tea = string_char_at(data, 0)
    sus count normie = 1
    
    bestie i := 1; i < string_len(data); i++ {
        sus char tea = string_char_at(data, i)
        vibes char == current_char {
            count++
        } nah {
            result = result + int_to_string(count) + current_char
            current_char = char
            count = 1
        }
    }
    
    result = result + int_to_string(count) + current_char
    damn result
}

slay rle_decompress(compressed tea) tea {
    vibes string_len(compressed) == 0 {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < string_len(compressed) {
        // Read count
        sus count_str tea = ""
        bestie i < string_len(compressed) && is_digit(string_char_at(compressed, i)) {
            count_str = count_str + string_char_at(compressed, i)
            i++
        }
        
        vibes i >= string_len(compressed) {
            ghosted
        }
        
        sus count normie = string_to_int(count_str)
        sus char tea = string_char_at(compressed, i)
        i++
        
        bestie j := 0; j < count; j++ {
            result = result + char
        }
    }
    
    damn result
}

// LZ77-style compression (simplified)
slay lz77_compress(data tea) tea {
    vibes string_len(data) == 0 {
        damn ""
    }
    
    sus result tea = ""
    sus window_size normie = 32
    sus lookahead_size normie = 8
    sus i normie = 0
    
    bestie i < string_len(data) {
        sus best_match Match = find_best_match(data, i, window_size, lookahead_size)
        
        vibes best_match.length > 0 {
            // Encode as (distance, length, next_char)
            result = result + "[" + int_to_string(best_match.distance) + "," + int_to_string(best_match.length) + "]"
            i = i + best_match.length
        } nah {
            // Literal character
            result = result + string_char_at(data, i)
            i++
        }
    }
    
    damn result
}

slay lz77_decompress(compressed tea) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie i < string_len(compressed) {
        sus char tea = string_char_at(compressed, i)
        
        vibes char == "[" {
            // Parse compression token
            sus token_end normie = string_index_of(compressed, "]", i)
            vibes token_end == -1 {
                ghosted
            }
            
            sus token tea = string_substring(compressed, i + 1, token_end - i - 1)
            sus parts [tea] = string_split(token, ",")
            
            vibes len(parts) == 2 {
                sus distance normie = string_to_int(parts[0])
                sus length normie = string_to_int(parts[1])
                
                // Copy from result
                sus start normie = string_len(result) - distance
                bestie j := 0; j < length; j++ {
                    vibes start + j >= 0 && start + j < string_len(result) {
                        result = result + string_char_at(result, start + j)
                    }
                }
            }
            
            i = token_end + 1
        } nah {
            // Literal character
            result = result + char
            i++
        }
    }
    
    damn result
}

// Match structure for LZ77
be_like Match squad {
    distance normie
    length normie
    next_char tea
}

slay find_best_match(data tea, pos normie, window_size normie, lookahead_size normie) Match {
    sus best Match = Match{distance: 0, length: 0, next_char: ""}
    
    vibes pos == 0 {
        damn best
    }
    
    sus window_start normie = max(0, pos - window_size)
    sus lookahead_end normie = min(string_len(data), pos + lookahead_size)
    
    bestie i := window_start; i < pos; i++ {
        sus match_len normie = 0
        
        bestie j := 0; j < lookahead_end - pos && i + j < pos; j++ {
            vibes string_char_at(data, i + j) == string_char_at(data, pos + j) {
                match_len++
            } nah {
                ghosted
            }
        }
        
        vibes match_len > best.length && match_len > 2 {
            best.distance = pos - i
            best.length = match_len
            vibes pos + match_len < string_len(data) {
                best.next_char = string_char_at(data, pos + match_len)
            }
        }
    }
    
    damn best
}

// Huffman-style frequency compression
slay frequency_compress(data tea) tea {
    vibes string_len(data) == 0 {
        damn ""
    }
    
    sus freq_map map[tea]normie = build_frequency_map(data)
    sus encoding_map map[tea]tea = build_simple_encoding(freq_map)
    
    sus result tea = ""
    bestie i := 0; i < string_len(data); i++ {
        sus char tea = string_char_at(data, i)
        sus encoding tea = get_encoding(encoding_map, char)
        result = result + encoding
    }
    
    damn result
}

slay build_frequency_map(data tea) map[tea]normie {
    sus freq_map map[tea]normie = {}
    
    bestie i := 0; i < string_len(data); i++ {
        sus char tea = string_char_at(data, i)
        sus current normie = get_frequency(freq_map, char)
        set_frequency(freq_map, char, current + 1)
    }
    
    damn freq_map
}

slay build_simple_encoding(freq_map map[tea]normie) map[tea]tea {
    // Simple binary encoding based on frequency
    sus encoding_map map[tea]tea = {}
    sus code normie = 0
    
    // For simplicity, assign binary codes based on order
    // In a real implementation, this would use Huffman algorithm
    
    damn encoding_map
}

slay get_encoding(encoding_map map[tea]tea, char tea) tea {
    // Get encoding for character
    // Placeholder implementation
    damn "0"
}

slay get_frequency(freq_map map[tea]normie, char tea) normie {
    // Get frequency for character
    // Placeholder implementation
    damn 0
}

slay set_frequency(freq_map map[tea]normie, char tea, freq normie) {
    // Set frequency for character
    // Placeholder implementation
}

// Dictionary compression
slay dictionary_compress(data tea) tea {
    sus dictionary [tea] = build_dictionary(data)
    sus result tea = ""
    
    bestie i := 0; i < string_len(data); {
        sus best_match tea = find_longest_dictionary_match(data, i, dictionary)
        
        vibes string_len(best_match) > 0 {
            sus dict_index normie = find_dictionary_index(dictionary, best_match)
            result = result + "#" + int_to_string(dict_index) + "#"
            i = i + string_len(best_match)
        } nah {
            result = result + string_char_at(data, i)
            i++
        }
    }
    
    damn result
}

slay dictionary_decompress(compressed tea, dictionary [tea]) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie i < string_len(compressed) {
        sus char tea = string_char_at(compressed, i)
        
        vibes char == "#" {
            // Parse dictionary reference
            sus end_pos normie = string_index_of(compressed, "#", i + 1)
            vibes end_pos > i + 1 {
                sus index_str tea = string_substring(compressed, i + 1, end_pos - i - 1)
                sus dict_index normie = string_to_int(index_str)
                
                vibes dict_index >= 0 && dict_index < len(dictionary) {
                    result = result + dictionary[dict_index]
                }
                
                i = end_pos + 1
            } nah {
                result = result + char
                i++
            }
        } nah {
            result = result + char
            i++
        }
    }
    
    damn result
}

slay build_dictionary(data tea) [tea] {
    sus dictionary [tea] = []
    sus window_size normie = 16
    
    bestie i := 0; i < string_len(data) - 1; i++ {
        bestie j := 2; j <= window_size && i + j <= string_len(data); j++ {
            sus phrase tea = string_substring(data, i, j)
            vibes !contains_phrase(dictionary, phrase) {
                dictionary = dictionary + [phrase]
            }
        }
    }
    
    damn dictionary
}

slay find_longest_dictionary_match(data tea, pos normie, dictionary [tea]) tea {
    sus best_match tea = ""
    
    bestie i := 0; i < len(dictionary); i++ {
        sus phrase tea = dictionary[i]
        
        vibes pos + string_len(phrase) <= string_len(data) {
            sus substr tea = string_substring(data, pos, string_len(phrase))
            vibes substr == phrase && string_len(phrase) > string_len(best_match) {
                best_match = phrase
            }
        }
    }
    
    damn best_match
}

slay find_dictionary_index(dictionary [tea], phrase tea) normie {
    bestie i := 0; i < len(dictionary); i++ {
        vibes dictionary[i] == phrase {
            damn i
        }
    }
    damn -1
}

slay contains_phrase(dictionary [tea], phrase tea) lit {
    bestie i := 0; i < len(dictionary); i++ {
        vibes dictionary[i] == phrase {
            damn based
        }
    }
    damn cap
}

// Compression utilities
slay compression_ratio(original tea, compressed tea) meal {
    vibes string_len(original) == 0 {
        damn 0.0
    }
    
    sus original_size meal = meal(string_len(original))
    sus compressed_size meal = meal(string_len(compressed))
    
    damn compressed_size / original_size
}

slay calculate_savings(original tea, compressed tea) meal {
    sus ratio meal = compression_ratio(original, compressed)
    damn (1.0 - ratio) * 100.0
}

// Auto-detect best compression method
slay auto_compress(data tea) tea {
    sus rle_result tea = rle_compress(data)
    sus lz77_result tea = lz77_compress(data)
    sus dict_result tea = dictionary_compress(data)
    
    // Return the best compression
    vibes string_len(rle_result) <= string_len(lz77_result) && string_len(rle_result) <= string_len(dict_result) {
        damn "RLE:" + rle_result
    } nah vibes string_len(lz77_result) <= string_len(dict_result) {
        damn "LZ77:" + lz77_result
    } nah {
        damn "DICT:" + dict_result
    }
}

slay auto_decompress(compressed tea) tea {
    vibes string_starts_with(compressed, "RLE:") {
        damn rle_decompress(string_substring(compressed, 4, string_len(compressed) - 4))
    } nah vibes string_starts_with(compressed, "LZ77:") {
        damn lz77_decompress(string_substring(compressed, 5, string_len(compressed) - 5))
    } nah vibes string_starts_with(compressed, "DICT:") {
        // Dictionary decompression requires the dictionary
        damn compressed  // Placeholder
    }
    
    damn compressed
}

// Utility functions
slay max(a normie, b normie) normie {
    vibes a > b {
        damn a
    }
    damn b
}

slay min(a normie, b normie) normie {
    vibes a < b {
        damn a
    }
    damn b
}

slay int_to_string(num normie) tea {
    // Convert integer to string
    vibes num == 0 {
        damn "0"
    }
    
    sus result tea = ""
    sus n normie = num
    vibes n < 0 {
        result = "-"
        n = -n
    }
    
    sus digits tea = ""
    bestie n > 0 {
        sus digit normie = n % 10
        digits = char_from_digit(digit) + digits
        n = n / 10
    }
    
    damn result + digits
}

slay char_from_digit(digit normie) tea {
    // Convert digit to character
    vibes digit >= 0 && digit <= 9 {
        damn string_char_from_code(48 + digit)  // '0' + digit
    }
    damn "0"
}

slay string_char_from_code(code normie) tea {
    // Convert character code to string
    // Placeholder implementation
    damn "0"
}

slay string_to_int(str tea) normie {
    // Convert string to integer
    vibes string_len(str) == 0 {
        damn 0
    }
    
    sus result normie = 0
    sus negative lit = cap
    sus start normie = 0
    
    vibes string_char_at(str, 0) == "-" {
        negative = based
        start = 1
    }
    
    bestie i := start; i < string_len(str); i++ {
        sus char tea = string_char_at(str, i)
        vibes is_digit(char) {
            sus digit normie = char_to_digit(char)
            result = result * 10 + digit
        }
    }
    
    vibes negative {
        result = -result
    }
    
    damn result
}

slay char_to_digit(char tea) normie {
    // Convert character to digit
    vibes string_len(char) == 1 {
        sus code normie = string_char_code(char)
        vibes code >= 48 && code <= 57 {
            damn code - 48
        }
    }
    damn 0
}

slay is_digit(char tea) lit {
    vibes string_len(char) != 1 {
        damn cap
    }
    
    sus code normie = string_char_code(char)
    damn code >= 48 && code <= 57  // '0' to '9'
}

slay string_char_code(char tea) normie {
    // Get character code for single character
    // Placeholder implementation
    damn 65
}

slay string_index_of(text tea, substring tea, start normie) normie {
    // Find index of substring starting from position
    sus text_len normie = string_len(text)
    sus sub_len normie = string_len(substring)
    
    bestie i := start; i <= text_len - sub_len; i++ {
        sus match lit = based
        bestie j := 0; j < sub_len; j++ {
            vibes string_char_at(text, i + j) != string_char_at(substring, j) {
                match = cap
                ghosted
            }
        }
        
        vibes match {
            damn i
        }
    }
    
    damn -1
}

slay string_starts_with(text tea, prefix tea) lit {
    vibes string_len(prefix) > string_len(text) {
        damn cap
    }
    
    sus prefix_part tea = string_substring(text, 0, string_len(prefix))
    damn prefix_part == prefix
}
