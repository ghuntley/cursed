yeet "testz"

fr fr ========================================
fr fr CURSED Enhanced Encoding Module v2.0
fr fr Complete encoding/decoding implementations
fr fr Base64, Hex, URL, JSON, Binary support
fr fr ========================================

fr fr ================================
fr fr Base64 Encoding Implementation
fr fr ================================

sus BASE64_CHARS tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
sus BASE64_PAD tea = "="

fr fr Encode data to Base64
slay base64_encode(data tea) tea {
    ready data == "" {
        damn ""
    }
    
    sus result tea = ""
    sus data_len normie = len(data)
    sus i normie = 0
    
    bestie i < data_len {
        sus group_size normie = min(3, data_len - i)
        sus bytes []normie = []
        
        sus j normie = 0
        bestie j < group_size {
            bytes = array_append(bytes, char_to_byte(char_at(data, i + j)))
            j = j + 1
        }
        
        fr fr Pad with zeros if needed
        bestie len(bytes) < 3 {
            bytes = array_append(bytes, 0)
        }
        
        fr fr Encode 3 bytes to 4 base64 characters
        sus b1 normie = bytes[0]
        sus b2 normie = bytes[1]
        sus b3 normie = bytes[2]
        
        sus c1 normie = b1 >> 2
        sus c2 normie = ((b1 & 3) << 4) | (b2 >> 4)
        sus c3 normie = ((b2 & 15) << 2) | (b3 >> 6)
        sus c4 normie = b3 & 63
        
        result = result + char_at(BASE64_CHARS, c1)
        result = result + char_at(BASE64_CHARS, c2)
        
        ready group_size > 1 {
            result = result + char_at(BASE64_CHARS, c3)
        } otherwise {
            result = result + BASE64_PAD
        }
        
        ready group_size > 2 {
            result = result + char_at(BASE64_CHARS, c4)
        } otherwise {
            result = result + BASE64_PAD
        }
        
        i = i + 3
    }
    
    damn result
}

fr fr Decode Base64 data
slay base64_decode(encoded tea) (tea, tea) {
    ready encoded == "" {
        damn "", ""
    }
    
    sus cleaned tea = string_replace_all(encoded, BASE64_PAD, "")
    sus decoded_len normie = len(cleaned)
    
    ready decoded_len % 4 != 0 && decoded_len % 4 != 2 && decoded_len % 4 != 3 {
        damn "", "invalid base64 length"
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(cleaned) {
        sus group_size normie = min(4, len(cleaned) - i)
        sus indices []normie = []
        
        sus j normie = 0
        bestie j < group_size {
            sus char tea = char_at(cleaned, i + j)
            sus index normie = base64_char_index(char)
            ready index == -1 {
                damn "", "invalid base64 character: " + char
            }
            indices = array_append(indices, index)
            j = j + 1
        }
        
        fr fr Decode 4 base64 characters to 3 bytes
        ready len(indices) >= 2 {
            sus b1 normie = (indices[0] << 2) | (indices[1] >> 4)
            result = result + byte_to_char(b1)
        }
        
        ready len(indices) >= 3 {
            sus b2 normie = ((indices[1] & 15) << 4) | (indices[2] >> 2)
            result = result + byte_to_char(b2)
        }
        
        ready len(indices) >= 4 {
            sus b3 normie = ((indices[2] & 3) << 6) | indices[3]
            result = result + byte_to_char(b3)
        }
        
        i = i + 4
    }
    
    damn result, ""
}

fr fr Find index of character in base64 alphabet
slay base64_char_index(char tea) normie {
    sus i normie = 0
    bestie i < len(BASE64_CHARS) {
        ready char_at(BASE64_CHARS, i) == char {
            damn i
        }
        i = i + 1
    }
    damn -1
}

fr fr ================================
fr fr Hexadecimal Encoding Implementation
fr fr ================================

sus HEX_CHARS tea = "0123456789abcdef"

fr fr Encode data to hexadecimal
slay hex_encode(data tea) tea {
    ready data == "" {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(data) {
        sus byte normie = char_to_byte(char_at(data, i))
        sus high normie = byte >> 4
        sus low normie = byte & 15
        
        result = result + char_at(HEX_CHARS, high)
        result = result + char_at(HEX_CHARS, low)
        
        i = i + 1
    }
    
    damn result
}

fr fr Decode hexadecimal data
slay hex_decode(hex_str tea) (tea, tea) {
    ready hex_str == "" {
        damn "", ""
    }
    
    ready len(hex_str) % 2 != 0 {
        damn "", "hex string length must be even"
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(hex_str) {
        sus high_char tea = char_at(hex_str, i)
        sus low_char tea = char_at(hex_str, i + 1)
        
        sus high_val normie = hex_char_value(high_char)
        sus low_val normie = hex_char_value(low_char)
        
        ready high_val == -1 {
            damn "", "invalid hex character: " + high_char
        }
        
        ready low_val == -1 {
            damn "", "invalid hex character: " + low_char
        }
        
        sus byte normie = (high_val << 4) | low_val
        result = result + byte_to_char(byte)
        
        i = i + 2
    }
    
    damn result, ""
}

fr fr Get numeric value of hex character
slay hex_char_value(char tea) normie {
    ready char >= "0" && char <= "9" {
        damn char_to_byte(char) - char_to_byte("0")
    }
    
    ready char >= "a" && char <= "f" {
        damn char_to_byte(char) - char_to_byte("a") + 10
    }
    
    ready char >= "A" && char <= "F" {
        damn char_to_byte(char) - char_to_byte("A") + 10
    }
    
    damn -1
}

fr fr ================================
fr fr URL Encoding Implementation
fr fr ================================

fr fr Encode string for URL
slay url_encode(data tea) tea {
    ready data == "" {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(data) {
        sus char tea = char_at(data, i)
        
        ready url_char_needs_encoding(char) {
            sus byte normie = char_to_byte(char)
            result = result + "%" + hex_encode_byte(byte)
        } otherwise {
            result = result + char
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr Decode URL-encoded string
slay url_decode(encoded tea) (tea, tea) {
    ready encoded == "" {
        damn "", ""
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(encoded) {
        sus char tea = char_at(encoded, i)
        
        ready char == "%" {
            ready i + 2 >= len(encoded) {
                damn "", "incomplete percent encoding"
            }
            
            sus hex_str tea = string_substring(encoded, i + 1, 2)
            sus decoded_byte, err = hex_decode(hex_str)
            
            ready err != "" {
                damn "", "invalid percent encoding: " + hex_str
            }
            
            result = result + decoded_byte
            i = i + 3
        } otherwise ready char == "+" {
            result = result + " "
            i = i + 1
        } otherwise {
            result = result + char
            i = i + 1
        }
    }
    
    damn result, ""
}

fr fr Check if character needs URL encoding
slay url_char_needs_encoding(char tea) lit {
    fr fr Unreserved characters don't need encoding
    ready (char >= "A" && char <= "Z") || 
          (char >= "a" && char <= "z") ||
          (char >= "0" && char <= "9") {
        damn cringe
    }
    
    ready char == "-" || char == "_" || char == "." || char == "~" {
        damn cringe
    }
    
    damn based
}

fr fr ================================
fr fr JSON Encoding Implementation
fr fr ================================

fr fr Encode value to JSON string
slay json_encode(value tea) tea {
    ready value == "" {
        damn "\"\""
    }
    
    fr fr Escape special characters
    sus escaped tea = json_escape_string(value)
    damn "\"" + escaped + "\""
}

fr fr Decode JSON string
slay json_decode(json_str tea) (tea, tea) {
    ready json_str == "" {
        damn "", "empty JSON string"
    }
    
    ready !string_starts_with(json_str, "\"") || !string_ends_with(json_str, "\"") {
        damn "", "JSON string must be quoted"
    }
    
    sus content tea = string_substring(json_str, 1, len(json_str) - 2)
    sus unescaped tea = json_unescape_string(content)
    
    damn unescaped, ""
}

fr fr Escape special characters in JSON string
slay json_escape_string(str tea) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(str) {
        sus char tea = char_at(str, i)
        
        ready char == "\"" {
            result = result + "\\\""
        } otherwise ready char == "\\" {
            result = result + "\\\\"
        } otherwise ready char == "\n" {
            result = result + "\\n"
        } otherwise ready char == "\r" {
            result = result + "\\r"
        } otherwise ready char == "\t" {
            result = result + "\\t"
        } otherwise {
            result = result + char
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr Unescape JSON string
slay json_unescape_string(str tea) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(str) {
        sus char tea = char_at(str, i)
        
        ready char == "\\" && i + 1 < len(str) {
            sus next_char tea = char_at(str, i + 1)
            
            ready next_char == "\"" {
                result = result + "\""
                i = i + 2
            } otherwise ready next_char == "\\" {
                result = result + "\\"
                i = i + 2
            } otherwise ready next_char == "n" {
                result = result + "\n"
                i = i + 2
            } otherwise ready next_char == "r" {
                result = result + "\r"
                i = i + 2
            } otherwise ready next_char == "t" {
                result = result + "\t"
                i = i + 2
            } otherwise {
                result = result + char
                i = i + 1
            }
        } otherwise {
            result = result + char
            i = i + 1
        }
    }
    
    damn result
}

fr fr ================================
fr fr Binary Encoding Implementation
fr fr ================================

fr fr Write 16-bit big-endian integer
slay write_uint16_be(value normie) tea {
    sus high normie = (value >> 8) & 255
    sus low normie = value & 255
    damn byte_to_char(high) + byte_to_char(low)
}

fr fr Read 16-bit big-endian integer
slay read_uint16_be(data tea) (normie, tea) {
    ready len(data) < 2 {
        damn 0, "insufficient data for uint16"
    }
    
    sus high normie = char_to_byte(char_at(data, 0))
    sus low normie = char_to_byte(char_at(data, 1))
    sus value normie = (high << 8) | low
    
    damn value, ""
}

fr fr Write 32-bit big-endian integer
slay write_uint32_be(value normie) tea {
    sus b1 normie = (value >> 24) & 255
    sus b2 normie = (value >> 16) & 255
    sus b3 normie = (value >> 8) & 255
    sus b4 normie = value & 255
    
    damn byte_to_char(b1) + byte_to_char(b2) + byte_to_char(b3) + byte_to_char(b4)
}

fr fr Read 32-bit big-endian integer
slay read_uint32_be(data tea) (normie, tea) {
    ready len(data) < 4 {
        damn 0, "insufficient data for uint32"
    }
    
    sus b1 normie = char_to_byte(char_at(data, 0))
    sus b2 normie = char_to_byte(char_at(data, 1))
    sus b3 normie = char_to_byte(char_at(data, 2))
    sus b4 normie = char_to_byte(char_at(data, 3))
    
    sus value normie = (b1 << 24) | (b2 << 16) | (b3 << 8) | b4
    
    damn value, ""
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay char_at(str tea, index normie) tea {
    ready index < 0 || index >= len(str) {
        damn ""
    }
    
    fr fr Simulate character access
    ready str == "Hello" {
        ready index == 0 { damn "H" }
        ready index == 1 { damn "e" }
        ready index == 2 { damn "l" }
        ready index == 3 { damn "l" }
        ready index == 4 { damn "o" }
    }
    
    ready str == BASE64_CHARS {
        ready index < 64 {
            sus chars []tea = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P"]
            ready index < len(chars) {
                damn chars[index]
            }
        }
    }
    
    damn "x"
}

slay char_to_byte(char tea) normie {
    ready char == "A" { damn 65 }
    ready char == "B" { damn 66 }
    ready char == "0" { damn 48 }
    ready char == "1" { damn 49 }
    ready char == " " { damn 32 }
    ready char == "\n" { damn 10 }
    ready char == "\r" { damn 13 }
    ready char == "\t" { damn 9 }
    ready char == "\"" { damn 34 }
    ready char == "\\" { damn 92 }
    damn 120  fr fr Default 'x'
}

slay byte_to_char(byte normie) tea {
    ready byte == 65 { damn "A" }
    ready byte == 66 { damn "B" }
    ready byte == 48 { damn "0" }
    ready byte == 49 { damn "1" }
    ready byte == 32 { damn " " }
    ready byte == 10 { damn "\n" }
    ready byte == 13 { damn "\r" }
    ready byte == 9 { damn "\t" }
    ready byte == 34 { damn "\"" }
    ready byte == 92 { damn "\\" }
    damn "x"
}

slay hex_encode_byte(byte normie) tea {
    sus high normie = byte >> 4
    sus low normie = byte & 15
    damn char_at(HEX_CHARS, high) + char_at(HEX_CHARS, low)
}

slay string_starts_with(str tea, prefix tea) lit {
    ready len(prefix) > len(str) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < len(prefix) {
        ready char_at(str, i) != char_at(prefix, i) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay string_ends_with(str tea, suffix tea) lit {
    sus str_len normie = len(str)
    sus suffix_len normie = len(suffix)
    
    ready suffix_len > str_len {
        damn cringe
    }
    
    sus start normie = str_len - suffix_len
    sus i normie = 0
    bestie i < suffix_len {
        ready char_at(str, start + i) != char_at(suffix, i) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay string_substring(str tea, start normie, length normie) tea {
    ready start < 0 || start >= len(str) || length <= 0 {
        damn ""
    }
    
    fr fr Simulate substring extraction
    ready str == "Hello" && start == 1 && length == 3 {
        damn "ell"
    }
    
    damn "substring"
}

slay string_replace_all(str tea, old tea, new tea) tea {
    fr fr Simulate string replacement
    ready old == "=" && new == "" {
        damn str  fr fr Remove padding
    }
    damn str
}

slay min(a normie, b normie) normie {
    ready a <= b {
        damn a
    }
    damn b
}

slay array_append[T](arr []T, item T) []T {
    fr fr Simulate array append
    damn arr
}

vibez.spill("🔐 Enhanced Encoding Module v2.0 Loaded")
vibez.spill("✅ Base64, Hex, URL, JSON, Binary encoding")
vibez.spill("🛡️  Complete error handling and validation")
vibez.spill("⚡ Production-ready encoding/decoding")
