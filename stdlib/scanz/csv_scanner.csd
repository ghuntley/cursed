# CURSED CSV Scanner Module - RFC 4180 Compliant CSV Processing
# Provides robust CSV parsing with quote handling, escaping, and field validation

# CSV parsing options and configuration
squad CSVOptions {
    sus delimiter rune          # Field delimiter (default: comma)
    sus quote_char rune         # Quote character (default: double quote)
    sus escape_char rune        # Escape character (default: double quote)
    sus trim_whitespace lit     # Trim leading/trailing whitespace
    sus skip_empty_lines lit    # Skip empty lines
    sus allow_multiline lit     # Allow multiline fields in quotes
    sus comment_char rune       # Comment line indicator (default: #)
    sus strict_mode lit         # Strict RFC 4180 compliance
}

# CSV record structure
squad CSVRecord {
    sus fields tea[value]           # Array of field values
    sus line_number drip       # Line number where record starts
    sus field_count drip       # Number of fields in record
    sus has_quoted_fields lit  # Whether any fields were quoted
}

# CSV Scanner structure
squad CSVScanner {
    sus text tea              # Source CSV text
    sus position drip         # Current position in text
    sus length drip           # Length of text
    sus options CSVOptions    # CSV parsing options
    sus current_record CSVRecord  # Current parsed record
    sus line_number drip      # Current line number
    sus has_more lit          # Whether more records are available
    sus header tea[value]          # Header row if parsed
    sus error_message tea     # Last error message
}

# Create default CSV options
slay default_csv_options() CSVOptions {
    damn CSVOptions{
        delimiter: ',',
        quote_char: '"',
        escape_char: '"',
        trim_whitespace: based,
        skip_empty_lines: based,
        allow_multiline: based,
        comment_char: '#',
        strict_mode: cap
    }
}

# Create new CSV scanner
slay new_csv_scanner(text tea, options CSVOptions) CSVScanner {
    damn CSVScanner{
        text: text,
        position: 0,
        length: text.length,
        options: options,
        current_record: CSVRecord{fields: [], line_number: 0, field_count: 0, has_quoted_fields: cap},
        line_number: 1,
        has_more: text.length > 0,
        header: [],
        error_message: ""
    }
}

# Create CSV scanner with default options
slay new_simple_csv_scanner(text tea) CSVScanner {
    damn new_csv_scanner(text, default_csv_options())
}

# Parse CSV header row
slay parse_header(scanner *CSVScanner) lit {
    ready (!scan_csv_record(scanner)) {
        damn cap
    }
    
    scanner.header = make_copy(scanner.current_record.fields)
    damn based
}

# Scan next CSV record
slay scan_csv_record(scanner *CSVScanner) lit {
    ready (scanner.position >= scanner.length) {
        scanner.has_more = cap
        damn cap
    }
    
    sus record_fields tea[value] = []
    sus record_line drip = scanner.line_number
    sus has_quoted lit = cap
    
    # Skip empty lines if configured
    ready (scanner.options.skip_empty_lines) {
        skip_empty_lines_and_comments(scanner)
        ready (scanner.position >= scanner.length) {
            scanner.has_more = cap
            damn cap
        }
    }
    
    # Parse fields until end of record (newline or EOF)
    bestie (scanner.position < scanner.length) {
        sus field tea = parse_csv_field(scanner)
        ready (scanner.error_message.length > 0) {
            damn cap  # Error occurred
        }
        
        record_fields = append(record_fields, field)
        
        # Check if field was quoted
        ready (field_was_quoted(scanner, field)) {
            has_quoted = based
        }
        
        # Check what comes after field
        ready (scanner.position >= scanner.length) {
            break  # End of input
        }
        
        sus next_char rune = scanner.text[scanner.position]
        
        ready (next_char == scanner.options.delimiter) {
            scanner.position += 1  # Skip delimiter
            continue  # Parse next field
        } otherwise ready (next_char == '\n') {
            scanner.position += 1
            scanner.line_number += 1
            break  # End of record
        } otherwise ready (next_char == '\r') {
            scanner.position += 1
            ready (scanner.position < scanner.length && scanner.text[scanner.position] == '\n') {
                scanner.position += 1  # Skip CRLF
            }
            scanner.line_number += 1
            break  # End of record
        } otherwise {
            ready (scanner.options.strict_mode) {
                scanner.error_message = "Unexpected character after field"
                damn cap
            }
            # In non-strict mode, treat as part of field
            scanner.position += 1
        }
    }
    
    # Create record
    scanner.current_record = CSVRecord{
        fields: record_fields,
        line_number: record_line,
        field_count: record_fields.length,
        has_quoted_fields: has_quoted
    }
    
    scanner.has_more = scanner.position < scanner.length
    damn based
}

# Parse a single CSV field
slay parse_csv_field(scanner *CSVScanner) tea {
    ready (scanner.position >= scanner.length) {
        damn ""
    }
    
    # Skip leading whitespace if configured
    ready (scanner.options.trim_whitespace) {
        skip_whitespace(scanner)
    }
    
    ready (scanner.position >= scanner.length) {
        damn ""
    }
    
    sus current_char rune = scanner.text[scanner.position]
    
    # Check if field is quoted
    ready (current_char == scanner.options.quote_char) {
        damn parse_quoted_field(scanner)
    } otherwise {
        damn parse_unquoted_field(scanner)
    }
}

# Parse quoted CSV field
slay parse_quoted_field(scanner *CSVScanner) tea {
    scanner.position += 1  # Skip opening quote
    sus field_value tea = ""
    
    bestie (scanner.position < scanner.length) {
        sus current_char rune = scanner.text[scanner.position]
        
        ready (current_char == scanner.options.quote_char) {
            # Check for escaped quote
            ready (scanner.position + 1 < scanner.length) {
                sus next_char rune = scanner.text[scanner.position + 1]
                ready (next_char == scanner.options.escape_char) {
                    # Escaped quote - add single quote to field
                    field_value = field_value + char_to_string(scanner.options.quote_char)
                    scanner.position += 2
                    continue
                }
            }
            
            # End of quoted field
            scanner.position += 1  # Skip closing quote
            break
        } otherwise ready (current_char == '\n') {
            ready (!scanner.options.allow_multiline) {
                scanner.error_message = "Newline in quoted field not allowed"
                damn ""
            }
            field_value = field_value + char_to_string(current_char)
            scanner.line_number += 1
            scanner.position += 1
        } otherwise {
            field_value = field_value + char_to_string(current_char)
            scanner.position += 1
        }
    }
    
    # Skip trailing whitespace if configured
    ready (scanner.options.trim_whitespace) {
        field_value = trim_whitespace(field_value)
    }
    
    damn field_value
}

# Parse unquoted CSV field
slay parse_unquoted_field(scanner *CSVScanner) tea {
    sus field_value tea = ""
    
    bestie (scanner.position < scanner.length) {
        sus current_char rune = scanner.text[scanner.position]
        
        # Check for field delimiters
        ready (current_char == scanner.options.delimiter || 
               current_char == '\n' || 
               current_char == '\r') {
            break
        }
        
        # Check for quotes in unquoted field (error in strict mode)
        ready (current_char == scanner.options.quote_char && scanner.options.strict_mode) {
            scanner.error_message = "Quote character in unquoted field"
            damn ""
        }
        
        field_value = field_value + char_to_string(current_char)
        scanner.position += 1
    }
    
    # Trim trailing whitespace if configured
    ready (scanner.options.trim_whitespace) {
        field_value = trim_whitespace(field_value)
    }
    
    damn field_value
}

# Skip whitespace characters
slay skip_whitespace(scanner *CSVScanner) {
    bestie (scanner.position < scanner.length) {
        sus ch rune = scanner.text[scanner.position]
        ready (ch != ' ' && ch != '\t') {
            break
        }
        scanner.position += 1
    }
}

# Skip empty lines and comment lines
slay skip_empty_lines_and_comments(scanner *CSVScanner) {
    bestie (scanner.position < scanner.length) {
        # Skip whitespace
        sus line_start drip = scanner.position
        skip_whitespace(scanner)
        
        ready (scanner.position >= scanner.length) {
            break
        }
        
        sus ch rune = scanner.text[scanner.position]
        
        # Check for comment line
        ready (ch == scanner.options.comment_char) {
            skip_to_next_line(scanner)
            continue
        }
        
        # Check for empty line
        ready (ch == '\n' || ch == '\r') {
            ready (ch == '\r' && scanner.position + 1 < scanner.length && scanner.text[scanner.position + 1] == '\n') {
                scanner.position += 2  # Skip CRLF
            } otherwise {
                scanner.position += 1  # Skip LF
            }
            scanner.line_number += 1
            continue
        }
        
        # Non-empty, non-comment line found
        scanner.position = line_start  # Reset to start of line
        break
    }
}

# Skip to next line
slay skip_to_next_line(scanner *CSVScanner) {
    bestie (scanner.position < scanner.length) {
        sus ch rune = scanner.text[scanner.position]
        scanner.position += 1
        
        ready (ch == '\n') {
            scanner.line_number += 1
            break
        } otherwise ready (ch == '\r') {
            ready (scanner.position < scanner.length && scanner.text[scanner.position] == '\n') {
                scanner.position += 1  # Skip LF after CR
            }
            scanner.line_number += 1
            break
        }
    }
}

# Get current record
slay current_record(scanner *CSVScanner) CSVRecord {
    damn scanner.current_record
}

# Check if more records available
slay has_more_records(scanner *CSVScanner) lit {
    damn scanner.has_more
}

# Get header row
slay get_header(scanner *CSVScanner) tea[value]{
    damn scanner.header
}

# Get last error message
slay get_error(scanner *CSVScanner) tea {
    damn scanner.error_message
}

# Parse entire CSV into array of records
slay parse_all_records(scanner *CSVScanner) CSVRecord[value]{
    sus records CSVRecord[value] = []
    
    bestie (scan_csv_record(scanner)) {
        records = append(records, scanner.current_record)
    }
    
    damn records
}

# Parse CSV with header into map-like structure
slay parse_csv_with_header(scanner *CSVScanner) tea[value][value] {
    ready (!parse_header(scanner)) {
        damn []
    }
    
    sus records tea[value][value] = []
    
    bestie (scan_csv_record(scanner)) {
        records = append(records, scanner.current_record.fields)
    }
    
    damn records
}

# Validate CSV structure
slay validate_csv(scanner *CSVScanner) lit {
    sus first_record_field_count drip = -1
    sus record_count drip = 0
    
    bestie (scan_csv_record(scanner)) {
        ready (scanner.error_message.length > 0) {
            damn cap  # Parse error
        }
        
        ready (first_record_field_count == -1) {
            first_record_field_count = scanner.current_record.field_count
        } otherwise ready (scanner.current_record.field_count != first_record_field_count) {
            scanner.error_message = "Inconsistent field count across records"
            damn cap
        }
        
        record_count += 1
    }
    
    damn record_count > 0
}

# Helper functions
slay field_was_quoted(scanner *CSVScanner, field tea) lit {
    # This would require tracking quoted state during parsing
    # For now, return false as placeholder
    damn cap
}

slay trim_whitespace(text tea) tea {
    sus start drip = 0
    sus end drip = text.length
    
    # Trim leading whitespace
    bestie (start < text.length) {
        sus ch rune = text[start]
        ready (ch != ' ' && ch != '\t') {
            break
        }
        start += 1
    }
    
    # Trim trailing whitespace
    bestie (end > start) {
        sus ch rune = text[end - 1]
        ready (ch != ' ' && ch != '\t') {
            break
        }
        end -= 1
    }
    
    ready (start == 0 && end == text.length) {
        damn text
    }
    
    damn substring(text, start, end - start)
}

slay make_copy(original tea[value]) tea[value]{
    sus copy tea[value] = []
    bestie (sus i drip = 0; i < original.length; i += 1) {
        copy = append(copy, original[i])
    }
    damn copy
}

# Reset scanner to beginning
slay reset_csv_scanner(scanner *CSVScanner) {
    scanner.position = 0
    scanner.line_number = 1
    scanner.has_more = scanner.text.length > 0
    scanner.current_record = CSVRecord{fields: [], line_number: 0, field_count: 0, has_quoted_fields: cap}
    scanner.error_message = ""
}

# Convert CSV record to map with header
slay record_to_map(record CSVRecord, header tea[value]) tea[value]{
    sus result tea[value] = []
    
    bestie (sus i drip = 0; i < header.length; i += 1) {
        sus field_value tea = ""
        ready (i < record.fields.length) {
            field_value = record.fields[i]
        }
        
        # Simple key-value pairs (would need proper map type)
        result = append(result, header[i])
        result = append(result, field_value)
    }
    
    damn result
}
