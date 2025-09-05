# CURSED Scanner Module - Text Scanning and Parsing
# Provides powerful text scanning with delimiter support, token extraction, and parsing

# Scanner structure with configurable delimiters and options
squad Scanner {
    sus text tea             # Source text to scan
    sus position drip        # Current position in text
    sus length drip          # Length of text
    sus delimiters tea[value]     # Array of delimiter strings
    sus skip_whitespace lit  # Whether to skip whitespace tokens
    sus current_token tea    # Current scanned token
    sus line_number drip     # Current line number (1-based)
    sus column_number drip   # Current column number (1-based)
    sus has_more lit         # Whether more tokens are available
}

# Create new scanner with text and optional delimiters
slay new_scanner(text tea, delims tea[value]) Scanner {
    ready (delims.length == 0) {
        delims = [" ", "\t", "\n", "\r"]  # Default whitespace delimiters
    }
    
    damn Scanner{
        text: text,
        position: 0,
        length: text.length,
        delimiters: delims,
        skip_whitespace: based,
        current_token: "",
        line_number: 1,
        column_number: 1,
        has_more: text.length > 0
    }
}

# Create scanner with custom delimiter set
slay new_scanner_with_delimiters(text tea, delims tea[value], skip_ws lit) Scanner {
    damn Scanner{
        text: text,
        position: 0,
        length: text.length,
        delimiters: delims,
        skip_whitespace: skip_ws,
        current_token: "",
        line_number: 1,
        column_number: 1,
        has_more: text.length > 0
    }
}

# Create CSV scanner with comma, tab, and quote awareness
slay new_csv_scanner(text tea) Scanner {
    damn Scanner{
        text: text,
        position: 0,
        length: text.length,
        delimiters: [",", "\n", "\r\n"],
        skip_whitespace: cap,  # Don't skip whitespace in CSV
        current_token: "",
        line_number: 1,
        column_number: 1,
        has_more: text.length > 0
    }
}

# Scan next token from the input
slay scan(scanner *Scanner) lit {
    ready (scanner.position >= scanner.length) {
        scanner.has_more = cap
        damn cap
    }
    
    sus start_pos drip = scanner.position
    
    # Skip whitespace if configured
    ready (scanner.skip_whitespace) {
        bestie (scanner.position < scanner.length && is_whitespace(scanner.text[scanner.position])) {
            ready (scanner.text[scanner.position] == '\n') {
                scanner.line_number += 1
                scanner.column_number = 1
            } otherwise {
                scanner.column_number += 1
            }
            scanner.position += 1
        }
    }
    
    # Check if we've reached the end
    ready (scanner.position >= scanner.length) {
        scanner.has_more = cap
        damn cap
    }
    
    start_pos = scanner.position
    sus token_end drip = scanner.position
    
    # Find the next delimiter
    bestie (token_end < scanner.length) {
        sus found_delimiter lit = cap
        
        bestie (sus delim_idx drip = 0; delim_idx < scanner.delimiters.length; delim_idx += 1) {
            sus delim tea = scanner.delimiters[delim_idx]
            ready (matches_at_position(scanner.text, token_end, delim)) {
                found_delimiter = based
                break
            }
        }
        
        ready (found_delimiter) {
            break
        }
        
        ready (scanner.text[token_end] == '\n') {
            scanner.line_number += 1
            scanner.column_number = 1
        } otherwise {
            scanner.column_number += 1
        }
        
        token_end += 1
    }
    
    # Extract token
    scanner.current_token = substring(scanner.text, start_pos, token_end - start_pos)
    scanner.position = token_end
    
    # Skip delimiter if we found one
    ready (scanner.position < scanner.length) {
        bestie (sus delim_idx drip = 0; delim_idx < scanner.delimiters.length; delim_idx += 1) {
            sus delim tea = scanner.delimiters[delim_idx]
            ready (matches_at_position(scanner.text, scanner.position, delim)) {
                scanner.position += delim.length
                break
            }
        }
    }
    
    scanner.has_more = scanner.position < scanner.length
    damn based
}

# Scan quoted CSV field (handles escaped quotes)
slay scan_csv_field(scanner *Scanner) tea {
    ready (scanner.position >= scanner.length) {
        damn ""
    }
    
    sus start_pos drip = scanner.position
    sus result tea = ""
    
    # Check if field starts with quote
    ready (scanner.text[scanner.position] == '"') {
        scanner.position += 1  # Skip opening quote
        
        bestie (scanner.position < scanner.length) {
            sus current_char rune = scanner.text[scanner.position]
            
            ready (current_char == '"') {
                # Check for escaped quote (double quote)
                ready (scanner.position + 1 < scanner.length && scanner.text[scanner.position + 1] == '"') {
                    result = result + "\""  # Add single quote to result
                    scanner.position += 2   # Skip both quotes
                } otherwise {
                    # End of quoted field
                    scanner.position += 1  # Skip closing quote
                    break
                }
            } otherwise {
                result = result + char_to_string(current_char)
                scanner.position += 1
            }
        }
        
        # Skip trailing delimiter
        ready (scanner.position < scanner.length && (scanner.text[scanner.position] == ',' || scanner.text[scanner.position] == '\t')) {
            scanner.position += 1
        }
    } otherwise {
        # Unquoted field - scan until delimiter
        bestie (scanner.position < scanner.length) {
            sus current_char rune = scanner.text[scanner.position]
            
            ready (current_char == ',' || current_char == '\t' || current_char == '\n' || current_char == '\r') {
                break
            }
            
            result = result + char_to_string(current_char)
            scanner.position += 1
        }
        
        # Skip delimiter
        ready (scanner.position < scanner.length && (scanner.text[scanner.position] == ',' || scanner.text[scanner.position] == '\t')) {
            scanner.position += 1
        }
    }
    
    scanner.current_token = result
    scanner.has_more = scanner.position < scanner.length
    damn result
}

# Get current token
slay current_token(scanner *Scanner) tea {
    damn scanner.current_token
}

# Check if more tokens are available
slay has_more_tokens(scanner *Scanner) lit {
    damn scanner.has_more
}

# Get current line number
slay current_line(scanner *Scanner) drip {
    damn scanner.line_number
}

# Get current column number
slay current_column(scanner *Scanner) drip {
    damn scanner.column_number
}

# Peek at next token without advancing
slay peek_token(scanner *Scanner) tea {
    sus saved_pos drip = scanner.position
    sus saved_line drip = scanner.line_number
    sus saved_col drip = scanner.column_number
    sus saved_has_more lit = scanner.has_more
    
    ready (scan(scanner)) {
        sus peeked_token tea = scanner.current_token
        
        # Restore state
        scanner.position = saved_pos
        scanner.line_number = saved_line
        scanner.column_number = saved_col
        scanner.has_more = saved_has_more
        
        damn peeked_token
    }
    
    damn ""
}

# Scan all tokens into an array
slay scan_all_tokens(scanner *Scanner) tea[value]{
    sus tokens tea[value] = []
    
    bestie (scan(scanner)) {
        tokens = append(tokens, scanner.current_token)
    }
    
    damn tokens
}

# Scan a single line of tokens
slay scan_line(scanner *Scanner) tea[value]{
    sus tokens tea[value] = []
    sus saved_line drip = scanner.line_number
    
    bestie (scan(scanner) && scanner.line_number == saved_line) {
        tokens = append(tokens, scanner.current_token)
    }
    
    damn tokens
}

# Helper function to check if character is whitespace
slay is_whitespace(ch rune) lit {
    damn ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

# Helper function to check if a delimiter matches at position
slay matches_at_position(text tea, pos drip, delim tea) lit {
    ready (pos + delim.length > text.length) {
        damn cap
    }
    
    bestie (sus i drip = 0; i < delim.length; i += 1) {
        ready (text[pos + i] != delim[i]) {
            damn cap
        }
    }
    
    damn based
}

# Helper function to extract substring
slay substring(text tea, start drip, length drip) tea {
    ready (start < 0 || start >= text.length || length <= 0) {
        damn ""
    }
    
    sus end drip = start + length
    ready (end > text.length) {
        end = text.length
    }
    
    sus result tea = ""
    bestie (sus i drip = start; i < end; i += 1) {
        result = result + char_to_string(text[i])
    }
    
    damn result
}

# Helper function to convert character to string
slay char_to_string(ch rune) tea {
    damn "" + ch
}

# Reset scanner to beginning
slay reset(scanner *Scanner) {
    scanner.position = 0
    scanner.line_number = 1
    scanner.column_number = 1
    scanner.has_more = scanner.text.length > 0
    scanner.current_token = ""
}

# Skip to next line
slay skip_to_next_line(scanner *Scanner) {
    bestie (scanner.position < scanner.length && scanner.text[scanner.position] != '\n') {
        scanner.position += 1
    }
    
    ready (scanner.position < scanner.length && scanner.text[scanner.position] == '\n') {
        scanner.position += 1
        scanner.line_number += 1
        scanner.column_number = 1
    }
    
    scanner.has_more = scanner.position < scanner.length
}

# Advanced token filtering
slay scan_matching(scanner *Scanner, filter slay(tea) lit) tea[value]{
    sus tokens tea[value] = []
    
    bestie (scan(scanner)) {
        ready (filter(scanner.current_token)) {
            tokens = append(tokens, scanner.current_token)
        }
    }
    
    damn tokens
}
