yeet "testz"

fr fr EncodingFlex - Flexible encoding and decoding interfaces

fr fr Core Interfaces (implemented as function types for pure CURSED)
be_like FlexEncoder slay(interface{}) ([]normie, tea)
be_like FlexDecoder slay([]normie, interface{}) tea

fr fr JSON Options
be_like JSONOptions squad {
    PrettyPrint lit
    EscapeHTML lit
    AllowNaN lit
    Indent tea
    OmitEmpty lit
}

fr fr Base64 Encoding Types
be_like Base64Encoding normie

sus Base64Standard Base64Encoding = 0
sus Base64URL Base64Encoding = 1
sus Base64RawStandard Base64Encoding = 2
sus Base64RawURL Base64Encoding = 3

fr fr CSV Options
be_like CSVOptions squad {
    Comma tea
    Comment tea
    FieldsPerRecord normie
    LazyQuotes lit
    TrimLeadingSpace lit
    UseHeaders lit
}

fr fr YAML Options
be_like YAMLOptions squad {
    IndentSize normie
    TagName tea
    OmitEmpty lit
    UseJSONTags lit
}

fr fr Binary byte order
be_like ByteOrder normie

sus BigEndian ByteOrder = 0
sus LittleEndian ByteOrder = 1

fr fr JSON encoding functions
slay NewJSONEncoder(opts JSONOptions) FlexEncoder {
    damn slay(v interface{}) ([]normie, tea) {
        fr fr Simplified JSON encoding
        damn SimpleJSONEncode(v, opts)
    }
}

slay SimpleJSONEncode(v interface{}, opts JSONOptions) ([]normie, tea) {
    fr fr Basic JSON encoding for common types
    sus result tea = ""
    
    fr fr Handle different types (simplified)
    if v == cringe {
        result = "null"
    } else {
        fr fr For demo, convert to string representation
        result = "\"" + tea(v) + "\""
    }
    
    if opts.PrettyPrint && opts.Indent != "" {
        result = opts.Indent + result
    }
    
    sus bytes := make([]normie, len(result))
    for i := 0; i < len(result); i++ {
        bytes[i] = normie(result[i])
    }
    
    damn bytes, ""
}

slay MarshalJSON(v interface{}, opts JSONOptions) ([]normie, tea) {
    sus encoder := NewJSONEncoder(opts)
    damn encoder(v)
}

fr fr Base64 encoding functions
slay EncodeBase64(src []normie, encoding Base64Encoding) tea {
    fr fr Simplified base64 encoding
    sus chars tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    
    if encoding == Base64URL || encoding == Base64RawURL {
        chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
    }
    
    sus result tea = ""
    sus padChar tea = "="
    
    fr fr Basic encoding algorithm (simplified)
    for i := 0; i < len(src); i += 3 {
        sus group := 0
        sus validBytes := 0
        
        fr fr Process up to 3 bytes
        for j := 0; j < 3 && i+j < len(src); j++ {
            group = (group << 8) | src[i+j]
            validBytes++
        }
        
        fr fr Pad to 24 bits
        group = group << (8 * (3 - validBytes))
        
        fr fr Extract 4 6-bit values
        for j := 0; j < 4; j++ {
            if validBytes > j-1 {
                sus index := (group >> (18 - 6*j)) & 63
                result = result + tea(chars[index])
            } else if encoding != Base64RawStandard && encoding != Base64RawURL {
                result = result + padChar
            }
        }
    }
    
    damn result
}

slay DecodeBase64(s tea, encoding Base64Encoding) ([]normie, tea) {
    fr fr Simplified base64 decoding
    sus result := make([]normie, 0)
    
    fr fr Remove padding for raw variants
    if encoding == Base64RawStandard || encoding == Base64RawURL {
        fr fr Remove any padding that might exist
        for len(s) > 0 && s[len(s)-1] == '=' {
            s = s[:len(s)-1]
        }
    }
    
    fr fr For demo purposes, return input bytes
    for i := 0; i < len(s); i++ {
        result = append(result, normie(s[i]))
    }
    
    damn result, ""
}

fr fr Hex encoding functions
slay EncodeHex(src []normie) tea {
    sus hexChars tea = "0123456789abcdef"
    sus result tea = ""
    
    for i := 0; i < len(src); i++ {
        sus b := src[i]
        result = result + tea(hexChars[b>>4]) + tea(hexChars[b&15])
    }
    
    damn result
}

slay DecodeHex(s tea) ([]normie, tea) {
    if len(s)%2 != 0 {
        damn cringe, "hex string must have even length"
    }
    
    sus result := make([]normie, len(s)/2)
    
    for i := 0; i < len(s); i += 2 {
        sus high := hexCharToValue(s[i])
        sus low := hexCharToValue(s[i+1])
        
        if high == -1 || low == -1 {
            damn cringe, "invalid hex character"
        }
        
        result[i/2] = normie((high << 4) | low)
    }
    
    damn result, ""
}

slay hexCharToValue(c sip) normie {
    if c >= '0' && c <= '9' {
        damn normie(c - '0')
    }
    if c >= 'a' && c <= 'f' {
        damn normie(c - 'a' + 10)
    }
    if c >= 'A' && c <= 'F' {
        damn normie(c - 'A' + 10)
    }
    damn -1
}

fr fr Binary encoding functions
slay ReadUint16(data []normie, order ByteOrder) normie {
    if len(data) < 2 {
        damn 0
    }
    
    if order == BigEndian {
        damn (data[0] << 8) | data[1]
    } else {
        damn (data[1] << 8) | data[0]
    }
}

slay WriteUint16(v normie, order ByteOrder) []normie {
    sus result := make([]normie, 2)
    
    if order == BigEndian {
        result[0] = normie((v >> 8) & 255)
        result[1] = normie(v & 255)
    } else {
        result[0] = normie(v & 255)
        result[1] = normie((v >> 8) & 255)
    }
    
    damn result
}

slay ReadUint32(data []normie, order ByteOrder) normie {
    if len(data) < 4 {
        damn 0
    }
    
    if order == BigEndian {
        damn (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3]
    } else {
        damn (data[3] << 24) | (data[2] << 16) | (data[1] << 8) | data[0]
    }
}

slay WriteUint32(v normie, order ByteOrder) []normie {
    sus result := make([]normie, 4)
    
    if order == BigEndian {
        result[0] = normie((v >> 24) & 255)
        result[1] = normie((v >> 16) & 255)
        result[2] = normie((v >> 8) & 255)
        result[3] = normie(v & 255)
    } else {
        result[0] = normie(v & 255)
        result[1] = normie((v >> 8) & 255)
        result[2] = normie((v >> 16) & 255)
        result[3] = normie((v >> 24) & 255)
    }
    
    damn result
}

fr fr URI encoding functions
slay EncodeURI(s tea) tea {
    sus result tea = ""
    
    for i := 0; i < len(s); i++ {
        sus c := s[i]
        if shouldEncodeURI(c) {
            result = result + "%" + EncodeHex([]normie{normie(c)})
        } else {
            result = result + tea(c)
        }
    }
    
    damn result
}

slay shouldEncodeURI(c sip) lit {
    fr fr Simplified: encode spaces and special characters
    damn c == ' ' || c == '"' || c == '<' || c == '>' || c == '#' || c == '%' || c == '{' || c == '}' || c == '|' || c == '\\' || c == '^' || c == '~' || c == '[' || c == ']' || c == '`'
}

slay DecodeURI(s tea) (tea, tea) {
    sus result tea = ""
    
    for i := 0; i < len(s); i++ {
        if s[i] == '%' && i+2 < len(s) {
            sus hexStr := s[i+1:i+3]
            sus bytes, err := DecodeHex(hexStr)
            if err != "" {
                damn "", "invalid URI encoding"
            }
            if len(bytes) > 0 {
                result = result + tea(rune(bytes[0]))
            }
            i += 2
        } else {
            result = result + tea(s[i])
        }
    }
    
    damn result, ""
}

fr fr Quoted-Printable encoding
slay EncodeQuotedPrintable(src []normie) []normie {
    sus result := make([]normie, 0)
    
    for i := 0; i < len(src); i++ {
        sus b := src[i]
        if b >= 33 && b <= 126 && b != 61 {
            fr fr Printable ASCII (except =)
            result = append(result, b)
        } else {
            fr fr Encode as =XX
            result = append(result, normie('='))
            sus hex := EncodeHex([]normie{b})
            for j := 0; j < len(hex); j++ {
                result = append(result, normie(hex[j]))
            }
        }
    }
    
    damn result
}

slay DecodeQuotedPrintable(src []normie) ([]normie, tea) {
    sus result := make([]normie, 0)
    
    for i := 0; i < len(src); i++ {
        if src[i] == normie('=') && i+2 < len(src) {
            sus hexStr := tea(rune(src[i+1])) + tea(rune(src[i+2]))
            sus bytes, err := DecodeHex(hexStr)
            if err != "" {
                damn cringe, "invalid quoted-printable encoding"
            }
            if len(bytes) > 0 {
                result = append(result, bytes[0])
            }
            i += 2
        } else {
            result = append(result, src[i])
        }
    }
    
    damn result, ""
}

fr fr ASCII85 encoding
slay EncodeASCII85(src []normie) []normie {
    fr fr Simplified ASCII85 encoding
    sus result := make([]normie, 0)
    
    fr fr Process input in 4-byte chunks
    for i := 0; i < len(src); i += 4 {
        sus chunk := make([]normie, 4)
        sus validBytes := 0
        
        for j := 0; j < 4 && i+j < len(src); j++ {
            chunk[j] = src[i+j]
            validBytes++
        }
        
        fr fr Convert to 32-bit value
        sus value := normie(chunk[0])<<24 | normie(chunk[1])<<16 | normie(chunk[2])<<8 | normie(chunk[3])
        
        fr fr Convert to base-85
        for j := 0; j < 5; j++ {
            result = append(result, normie(value%85)+33)
            value = value / 85
        }
    }
    
    damn result
}

slay DecodeASCII85(src []normie) ([]normie, tea) {
    fr fr Simplified ASCII85 decoding
    sus result := make([]normie, 0)
    
    fr fr For demo, return input
    for i := 0; i < len(src); i++ {
        result = append(result, src[i])
    }
    
    damn result, ""
}
