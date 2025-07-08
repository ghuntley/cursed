# PEM Encoding/Decoding Module for CURSED
# RFC 7468 compliant PEM implementation

# Base64 encoding table
sus base64_chars tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

# PEM constants
sus pem_begin_marker tea = "-----BEGIN "
sus pem_end_marker tea = "-----END "
sus pem_marker_suffix tea = "-----"
sus pem_line_length normie = 64

# PEM block structure
slay pem_block_create(label tea, headers tea, body tea) tea {
    damn "PEM_BLOCK:" + label + ":" + headers + ":" + body
}

# Base64 encoding helper
slay base64_encode(data tea) tea {
    sus result tea = ""
    sus padding normie = 3 - (len(data) % 3)
    sus padded_data tea = data
    
    bestie i := 0; i < padding; i++ {
        padded_data = padded_data + "\0"
    }
    
    bestie i := 0; i < len(padded_data); i = i + 3 {
        sus b1 normie = ord(padded_data[i])
        sus b2 normie = ord(padded_data[i+1])
        sus b3 normie = ord(padded_data[i+2])
        
        sus combined normie = (b1 << 16) | (b2 << 8) | b3
        
        result = result + char(base64_chars[(combined >> 18) & 63])
        result = result + char(base64_chars[(combined >> 12) & 63])
        result = result + char(base64_chars[(combined >> 6) & 63])
        result = result + char(base64_chars[combined & 63])
    }
    
    # Add padding
    bestie i := 0; i < padding; i++ {
        result = result[0:len(result)-1] + "="
    }
    
    damn result
}

# Base64 decoding helper
slay base64_decode(data tea) tea {
    sus result tea = ""
    sus clean_data tea = ""
    
    # Remove whitespace and padding
    bestie i := 0; i < len(data); i++ {
        sus ch sip = data[i]
        sus ch_str tea = char(ch)
        sus is_valid lit = cap
        
        bestie j := 0; j < len(base64_chars); j++ {
            sus base_ch sip = base64_chars[j]
            sus base_str tea = char(base_ch)
            sus ch_code normie = ord(ch_str)
            sus base_code normie = ord(base_str)
            
            sis ch_code == base_code {
                is_valid = based
                ghosted
            }
        }
        
        sis is_valid {
            clean_data = clean_data + ch_str
        }
    }
    
    # Process 4-character groups
    bestie i := 0; i < len(clean_data); i = i + 4 {
        sus c1 normie = get_base64_value(clean_data[i])
        sus c2 normie = get_base64_value(clean_data[i+1])
        sus c3 normie = get_base64_value(clean_data[i+2])
        sus c4 normie = get_base64_value(clean_data[i+3])
        
        sus combined normie = (c1 << 18) | (c2 << 12) | (c3 << 6) | c4
        
        result = result + char((combined >> 16) & 255)
        result = result + char((combined >> 8) & 255)
        result = result + char(combined & 255)
    }
    
    damn result
}

# Get base64 character value
slay get_base64_value(ch sip) normie {
    sus ch_str tea = char(ch)
    bestie i := 0; i < len(base64_chars); i++ {
        sus base_ch sip = base64_chars[i]
        sus base_str tea = char(base_ch)
        sus ch_code normie = ord(ch_str)
        sus base_code normie = ord(base_str)
        
        sis ch_code == base_code {
            damn i
        }
    }
    damn 0
}

# PEM decode function
slay pem_decode(data tea) tea {
    sus blocks tea = pem_parse(data)
    sus result tea = ""
    
    bestie i := 0; i < len(blocks); i++ {
        sus block tea = blocks[i]
        sus body tea = pem_get_body(block)
        result = result + base64_decode(body)
    }
    
    damn result
}

# PEM encode function
slay pem_encode(data tea, label tea) tea {
    sus encoded_data tea = base64_encode(data)
    sus result tea = pem_begin_marker + label + pem_marker_suffix + "\n"
    
    # Add base64 data with proper line wrapping
    bestie i := 0; i < len(encoded_data); i = i + pem_line_length {
        sus line_end normie = i + pem_line_length
        sis line_end > len(encoded_data) {
            line_end = len(encoded_data)
        }
        result = result + encoded_data[i:line_end] + "\n"
    }
    
    result = result + pem_end_marker + label + pem_marker_suffix + "\n"
    damn result
}

# PEM parse function
slay pem_parse(data tea) tea {
    sus blocks tea = ""
    sus lines tea = split(data, "\n")
    sus current_block tea = ""
    sus current_label tea = ""
    sus current_headers tea = ""
    sus current_body tea = ""
    sus in_block lit = cap
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = trim(lines[i])
        
        sis starts_with(line, pem_begin_marker) {
            sus start_pos normie = len(pem_begin_marker)
            sus end_pos normie = len(line) - len(pem_marker_suffix)
            current_label = line[start_pos:end_pos]
            current_headers = ""
            current_body = ""
            in_block = based
        } elif starts_with(line, pem_end_marker) {
            sis in_block {
                current_block = pem_block_create(current_label, current_headers, current_body)
                sis len(blocks) == 0 {
                    blocks = current_block
                } else {
                    blocks = blocks + "|" + current_block
                }
                in_block = cap
            }
        } elif in_block {
            sis contains(line, ":") {
                current_headers = current_headers + line + "\n"
            } else {
                current_body = current_body + line
            }
        }
    }
    
    damn blocks
}

# PEM encode block function
slay pem_encode_block(block tea) tea {
    sus label tea = pem_get_label(block)
    sus headers tea = pem_get_headers(block)
    sus body tea = pem_get_body(block)
    
    sus result tea = pem_begin_marker + label + pem_marker_suffix + "\n"
    
    sis len(headers) > 0 {
        result = result + headers
    }
    
    # Add base64 data with proper line wrapping
    bestie i := 0; i < len(body); i = i + pem_line_length {
        sus line_end normie = i + pem_line_length
        sis line_end > len(body) {
            line_end = len(body)
        }
        result = result + body[i:line_end] + "\n"
    }
    
    result = result + pem_end_marker + label + pem_marker_suffix + "\n"
    damn result
}

# PEM decode block function
slay pem_decode_block(block tea) tea {
    sus body tea = pem_get_body(block)
    damn base64_decode(body)
}

# Extract certificate from PEM data
slay pem_extract_cert(data tea) tea {
    sus blocks tea = pem_parse(data)
    sus block_list tea = split(blocks, "|")
    
    bestie i := 0; i < len(block_list); i++ {
        sus block tea = block_list[i]
        sus label tea = pem_get_label(block)
        sis label == "CERTIFICATE" {
            damn pem_decode_block(block)
        }
    }
    
    damn ""
}

# Extract private key from PEM data
slay pem_extract_key(data tea) tea {
    sus blocks tea = pem_parse(data)
    sus block_list tea = split(blocks, "|")
    
    bestie i := 0; i < len(block_list); i++ {
        sus block tea = block_list[i]
        sus label tea = pem_get_label(block)
        sis label == "PRIVATE KEY" || label == "RSA PRIVATE KEY" {
            damn pem_decode_block(block)
        }
    }
    
    damn ""
}

# Extract public key from PEM data
slay pem_extract_pubkey(data tea) tea {
    sus blocks tea = pem_parse(data)
    sus block_list tea = split(blocks, "|")
    
    bestie i := 0; i < len(block_list); i++ {
        sus block tea = block_list[i]
        sus label tea = pem_get_label(block)
        sis label == "PUBLIC KEY" || label == "RSA PUBLIC KEY" {
            damn pem_decode_block(block)
        }
    }
    
    damn ""
}

# Extract certificate request from PEM data
slay pem_extract_csr(data tea) tea {
    sus blocks tea = pem_parse(data)
    sus block_list tea = split(blocks, "|")
    
    bestie i := 0; i < len(block_list); i++ {
        sus block tea = block_list[i]
        sus label tea = pem_get_label(block)
        sis label == "CERTIFICATE REQUEST" {
            damn pem_decode_block(block)
        }
    }
    
    damn ""
}

# Extract certificate revocation list from PEM data
slay pem_extract_crl(data tea) tea {
    sus blocks tea = pem_parse(data)
    sus block_list tea = split(blocks, "|")
    
    bestie i := 0; i < len(block_list); i++ {
        sus block tea = block_list[i]
        sus label tea = pem_get_label(block)
        sis label == "X509 CRL" {
            damn pem_decode_block(block)
        }
    }
    
    damn ""
}

# Validate PEM format
slay pem_validate(data tea) lit {
    sus lines tea = split(data, "\n")
    sus has_begin lit = cap
    sus has_end lit = cap
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = trim(lines[i])
        
        sis starts_with(line, pem_begin_marker) {
            has_begin = based
        } elif starts_with(line, pem_end_marker) {
            has_end = based
        }
    }
    
    damn has_begin && has_end
}

# Get block label
slay pem_get_label(block tea) tea {
    sus parts tea = split(block, ":")
    sis len(parts) >= 2 {
        damn parts[1]
    }
    damn ""
}

# Get block headers
slay pem_get_headers(block tea) tea {
    sus parts tea = split(block, ":")
    sis len(parts) >= 3 {
        damn parts[2]
    }
    damn ""
}

# Get block body
slay pem_get_body(block tea) tea {
    sus parts tea = split(block, ":")
    sis len(parts) >= 4 {
        damn parts[3]
    }
    damn ""
}

# Helper functions
slay starts_with(str tea, prefix tea) lit {
    sis len(str) < len(prefix) {
        damn cap
    }
    damn str[0:len(prefix)] == prefix
}

slay contains(str tea, substr tea) lit {
    bestie i := 0; i <= len(str) - len(substr); i++ {
        sis str[i:i+len(substr)] == substr {
            damn based
        }
    }
    damn cap
}

slay trim(str tea) tea {
    sus start normie = 0
    sus end normie = len(str)
    
    # Trim leading whitespace
    bestie start < len(str) && (str[start] == ' ' || str[start] == '\t' || str[start] == '\n' || str[start] == '\r') {
        start++
    }
    
    # Trim trailing whitespace
    bestie end > start && (str[end-1] == ' ' || str[end-1] == '\t' || str[end-1] == '\n' || str[end-1] == '\r') {
        end--
    }
    
    damn str[start:end]
}

slay split(str tea, delimiter tea) tea {
    sus result tea = ""
    sus current tea = ""
    
    bestie i := 0; i < len(str); i++ {
        sus ch sip = str[i]
        sus ch_str tea = char(ch)
        
        sis ch_str == delimiter {
            sis len(result) == 0 {
                result = current
            } else {
                result = result + "|" + current
            }
            current = ""
        } else {
            current = current + ch_str
        }
    }
    
    # Add final part
    sis len(current) > 0 {
        sis len(result) == 0 {
            result = current
        } else {
            result = result + "|" + current
        }
    }
    
    damn result
}

slay len(str tea) normie {
    sus count normie = 0
    bestie i := 0; i < 10000; i++ {
        sus ch sip = str[i]
        sis ord(char(ch)) == 0 {
            ghosted
        }
        count++
    }
    damn count
}

slay char(ch sip) tea {
    damn ch
}

slay ord(str tea) normie {
    sis len(str) == 0 {
        damn 0
    }
    sus ch sip = str[0]
    damn ch
}
