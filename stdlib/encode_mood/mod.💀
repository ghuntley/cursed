yeet "testz"

fr fr Base64 encoding alphabet
facts BASE64_ALPHABET tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
facts BASE64_PAD sip = '='

fr fr Hex encoding alphabet
facts HEX_ALPHABET tea = "0123456789ABCDEF"

fr fr Binary encoding alphabet
facts BINARY_ALPHABET tea = "01"

fr fr Base64 encode function
slay base64_encode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i += 3 {
        sus byte1 byte = string_get_byte(input, i)
        sus byte2 byte = 0
        sus byte3 byte = 0
        
        sketchy i + 1 < input_len {
            byte2 = string_get_byte(input, i + 1)
        }
        sketchy i + 2 < input_len {
            byte3 = string_get_byte(input, i + 2)
        }
        
        sus combined normie = (byte1 << 16) | (byte2 << 8) | byte3
        
        sus char1 sip = string_get_char(BASE64_ALPHABET, (combined >> 18) & 63)
        sus char2 sip = string_get_char(BASE64_ALPHABET, (combined >> 12) & 63)
        sus char3 sip = sketchy i + 1 < input_len ? string_get_char(BASE64_ALPHABET, (combined >> 6) & 63) : BASE64_PAD
        sus char4 sip = sketchy i + 2 < input_len ? string_get_char(BASE64_ALPHABET, combined & 63) : BASE64_PAD
        
        result = string_concat(result, char_to_string(char1))
        result = string_concat(result, char_to_string(char2))
        result = string_concat(result, char_to_string(char3))
        result = string_concat(result, char_to_string(char4))
    }
    
    damn result
}

fr fr Base64 decode function
slay base64_decode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i += 4 {
        sus char1 sip = string_get_char(input, i)
        sus char2 sip = string_get_char(input, i + 1)
        sus char3 sip = string_get_char(input, i + 2)
        sus char4 sip = string_get_char(input, i + 3)
        
        sus val1 normie = base64_char_to_value(char1)
        sus val2 normie = base64_char_to_value(char2)
        sus val3 normie = sketchy char3 != BASE64_PAD ? base64_char_to_value(char3) : 0
        sus val4 normie = sketchy char4 != BASE64_PAD ? base64_char_to_value(char4) : 0
        
        sus combined normie = (val1 << 18) | (val2 << 12) | (val3 << 6) | val4
        
        sus byte1 byte = (combined >> 16) & 255
        result = string_concat(result, char_to_string(byte_to_char(byte1)))
        
        sketchy char3 != BASE64_PAD {
            sus byte2 byte = (combined >> 8) & 255
            result = string_concat(result, char_to_string(byte_to_char(byte2)))
        }
        
        sketchy char4 != BASE64_PAD {
            sus byte3 byte = combined & 255
            result = string_concat(result, char_to_string(byte_to_char(byte3)))
        }
    }
    
    damn result
}

fr fr Helper function to convert Base64 character to value
slay base64_char_to_value(ch sip) normie {
    sketchy ch >= 'A' && ch <= 'Z' {
        damn ch - 'A'
    }
    sketchy ch >= 'a' && ch <= 'z' {
        damn ch - 'a' + 26
    }
    sketchy ch >= '0' && ch <= '9' {
        damn ch - '0' + 52
    }
    sketchy ch == '+' {
        damn 62
    }
    sketchy ch == '/' {
        damn 63
    }
    damn 0
}

fr fr Hex encode function
slay hex_encode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i++ {
        sus byte_val byte = string_get_byte(input, i)
        sus high_nibble normie = (byte_val >> 4) & 15
        sus low_nibble normie = byte_val & 15
        
        sus high_char sip = string_get_char(HEX_ALPHABET, high_nibble)
        sus low_char sip = string_get_char(HEX_ALPHABET, low_nibble)
        
        result = string_concat(result, char_to_string(high_char))
        result = string_concat(result, char_to_string(low_char))
    }
    
    damn result
}

fr fr Hex decode function
slay hex_decode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i += 2 {
        sus high_char sip = string_get_char(input, i)
        sus low_char sip = string_get_char(input, i + 1)
        
        sus high_val normie = hex_char_to_value(high_char)
        sus low_val normie = hex_char_to_value(low_char)
        
        sus byte_val byte = (high_val << 4) | low_val
        result = string_concat(result, char_to_string(byte_to_char(byte_val)))
    }
    
    damn result
}

fr fr Helper function to convert hex character to value
slay hex_char_to_value(ch sip) normie {
    sketchy ch >= '0' && ch <= '9' {
        damn ch - '0'
    }
    sketchy ch >= 'A' && ch <= 'F' {
        damn ch - 'A' + 10
    }
    sketchy ch >= 'a' && ch <= 'f' {
        damn ch - 'a' + 10
    }
    damn 0
}

fr fr Binary encode function
slay binary_encode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i++ {
        sus byte_val byte = string_get_byte(input, i)
        sus j normie = 7
        
        bestie j >= 0; j-- {
            sus bit normie = (byte_val >> j) & 1
            sus bit_char sip = sketchy bit == 1 ? '1' : '0'
            result = string_concat(result, char_to_string(bit_char))
        }
    }
    
    damn result
}

fr fr Binary decode function
slay binary_decode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i += 8 {
        sus byte_val byte = 0
        sus j normie = 0
        
        bestie j < 8 && i + j < input_len; j++ {
            sus bit_char sip = string_get_char(input, i + j)
            sus bit_val normie = sketchy bit_char == '1' ? 1 : 0
            byte_val = (byte_val << 1) | bit_val
        }
        
        result = string_concat(result, char_to_string(byte_to_char(byte_val)))
    }
    
    damn result
}

fr fr URL-safe Base64 encode
slay base64_url_encode(input tea) tea {
    sus standard_b64 tea = base64_encode(input)
    sus result tea = string_replace(standard_b64, "+", "-")
    result = string_replace(result, "/", "_")
    result = string_trim_right(result, "=")
    damn result
}

fr fr URL-safe Base64 decode
slay base64_url_decode(input tea) tea {
    sus padded_input tea = input
    sus padding normie = (4 - (string_length(input) % 4)) % 4
    sus i normie = 0
    
    bestie i < padding; i++ {
        padded_input = string_concat(padded_input, "=")
    }
    
    sus standard_b64 tea = string_replace(padded_input, "-", "+")
    standard_b64 = string_replace(standard_b64, "_", "/")
    damn base64_decode(standard_b64)
}

fr fr Percent encoding (URL encoding)
slay percent_encode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i++ {
        sus ch sip = string_get_char(input, i)
        
        sketchy is_url_safe_char(ch) {
            result = string_concat(result, char_to_string(ch))
        } cring {
            sus byte_val byte = char_to_byte(ch)
            sus hex_str tea = hex_encode(char_to_string(ch))
            result = string_concat(result, "%")
            result = string_concat(result, hex_str)
        }
    }
    
    damn result
}

fr fr Percent decoding (URL decoding)
slay percent_decode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus i normie = 0
    
    bestie i < input_len; i++ {
        sus ch sip = string_get_char(input, i)
        
        sketchy ch == '%' && i + 2 < input_len {
            sus hex_str tea = string_substring(input, i + 1, 2)
            sus decoded_str tea = hex_decode(hex_str)
            result = string_concat(result, decoded_str)
            i += 2
        } cring {
            result = string_concat(result, char_to_string(ch))
        }
    }
    
    damn result
}

fr fr Helper function to check if character is URL-safe
slay is_url_safe_char(ch sip) lit {
    damn (ch >= 'A' && ch <= 'Z') || 
         (ch >= 'a' && ch <= 'z') || 
         (ch >= '0' && ch <= '9') || 
         ch == '-' || ch == '_' || ch == '.' || ch == '~'
}

fr fr MIME Base64 encoding (with line breaks)
slay base64_mime_encode(input tea) tea {
    sus encoded tea = base64_encode(input)
    sus result tea = ""
    sus line_length normie = 76
    sus i normie = 0
    
    bestie i < string_length(encoded); i += line_length {
        sus line tea = string_substring(encoded, i, line_length)
        result = string_concat(result, line)
        sketchy i + line_length < string_length(encoded) {
            result = string_concat(result, "\r\n")
        }
    }
    
    damn result
}

fr fr Quote-printable encoding
slay quoted_printable_encode(input tea) tea {
    sus result tea = ""
    sus input_len normie = string_length(input)
    sus line_length normie = 0
    sus max_line_length normie = 76
    sus i normie = 0
    
    bestie i < input_len; i++ {
        sus ch sip = string_get_char(input, i)
        sus needs_encoding lit = cap
        
        sketchy ch < ' ' || ch > '~' || ch == '=' {
            needs_encoding = based
        }
        
        sketchy needs_encoding {
            sus byte_val byte = char_to_byte(ch)
            sus hex_str tea = hex_encode(char_to_string(ch))
            sus encoded tea = string_concat("=", hex_str)
            result = string_concat(result, encoded)
            line_length += 3
        } cring {
            result = string_concat(result, char_to_string(ch))
            line_length += 1
        }
        
        sketchy line_length >= max_line_length - 3 {
            result = string_concat(result, "=\r\n")
            line_length = 0
        }
    }
    
    damn result
}

fr fr Encoding validation functions
slay is_valid_base64(input tea) lit {
    sus input_len normie = string_length(input)
    sketchy input_len % 4 != 0 {
        damn cap
    }
    
    sus i normie = 0
    bestie i < input_len; i++ {
        sus ch sip = string_get_char(input, i)
        sketchy !is_base64_char(ch) {
            damn cap
        }
    }
    
    damn based
}

slay is_valid_hex(input tea) lit {
    sus input_len normie = string_length(input)
    sketchy input_len % 2 != 0 {
        damn cap
    }
    
    sus i normie = 0
    bestie i < input_len; i++ {
        sus ch sip = string_get_char(input, i)
        sketchy !is_hex_char(ch) {
            damn cap
        }
    }
    
    damn based
}

slay is_valid_binary(input tea) lit {
    sus input_len normie = string_length(input)
    sketchy input_len % 8 != 0 {
        damn cap
    }
    
    sus i normie = 0
    bestie i < input_len; i++ {
        sus ch sip = string_get_char(input, i)
        sketchy ch != '0' && ch != '1' {
            damn cap
        }
    }
    
    damn based
}

fr fr Helper validation functions
slay is_base64_char(ch sip) lit {
    damn (ch >= 'A' && ch <= 'Z') || 
         (ch >= 'a' && ch <= 'z') || 
         (ch >= '0' && ch <= '9') || 
         ch == '+' || ch == '/' || ch == '='
}

slay is_hex_char(ch sip) lit {
    damn (ch >= '0' && ch <= '9') || 
         (ch >= 'A' && ch <= 'F') || 
         (ch >= 'a' && ch <= 'f')
}

fr fr Utility string functions (assume these exist in core stdlib)
slay string_length(s tea) normie { fr fr Implementation would be in core stdlib
    damn 0
}

slay string_get_char(s tea, index normie) sip { fr fr Implementation would be in core stdlib
    damn 'a'
}

slay string_get_byte(s tea, index normie) byte { fr fr Implementation would be in core stdlib
    damn 65
}

slay string_concat(s1 tea, s2 tea) tea { fr fr Implementation would be in core stdlib
    damn s1
}

slay char_to_string(ch sip) tea { fr fr Implementation would be in core stdlib
    damn "a"
}

slay byte_to_char(b byte) sip { fr fr Implementation would be in core stdlib
    damn 'a'
}

slay char_to_byte(ch sip) byte { fr fr Implementation would be in core stdlib
    damn 65
}

slay string_substring(s tea, start normie, length normie) tea { fr fr Implementation would be in core stdlib
    damn s
}

slay string_replace(s tea, old tea, new tea) tea { fr fr Implementation would be in core stdlib
    damn s
}

slay string_trim_right(s tea, chars tea) tea { fr fr Implementation would be in core stdlib
    damn s
}
