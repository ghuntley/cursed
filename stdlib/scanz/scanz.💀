# CURSED Scanner and TabWriter Module (scanz)
# Main module file that exports all scanner and tabwriter functionality
# Provides text scanning, CSV parsing, and tabular data formatting

yeet "scanner"      # Text scanning with delimiters
yeet "tabwriter"    # Table formatting and alignment
yeet "csv_scanner"  # RFC 4180 compliant CSV processing

# Re-export core scanner functions
slay new_text_scanner(text tea, delims tea[value]) Scanner {
    damn new_scanner(text, delims)
}

slay new_default_scanner(text tea) Scanner {
    damn new_scanner(text, [])
}

slay scan_tokens(text tea, delimiters tea[value]) tea[value]{
    sus scanner Scanner = new_scanner(text, delimiters)
    damn scan_all_tokens(&scanner)
}

slay scan_lines(text tea) tea[value][value] {
    sus scanner Scanner = new_scanner(text, ["\n", "\r\n"])
    sus lines tea[value][value] = []
    
    bestie (scan(&scanner)) {
        sus line tea = current_token(&scanner)
        ready (line.length > 0) {
            sus line_scanner Scanner = new_scanner(line, [" ", "\t"])
            sus line_tokens tea[value] = scan_all_tokens(&line_scanner)
            lines = append(lines, line_tokens)
        }
    }
    
    damn lines
}

# Re-export CSV scanner functions
slay parse_csv(text tea) tea[value][value] {
    sus scanner CSVScanner = new_simple_csv_scanner(text)
    sus records tea[value][value] = []
    
    bestie (scan_csv_record(&scanner)) {
        records = append(records, current_record(&scanner).fields)
    }
    
    damn records
}

slay parse_csv_with_options(text tea, options CSVOptions) tea[value][value] {
    sus scanner CSVScanner = new_csv_scanner(text, options)
    sus records tea[value][value] = []
    
    bestie (scan_csv_record(&scanner)) {
        records = append(records, current_record(&scanner).fields)
    }
    
    damn records
}

# Re-export TabWriter functions
slay create_table(headers tea[value]) TabWriter {
    damn new_simple_tabwriter(headers)
}

slay format_table(data tea[value][value], headers tea[value]) tea {
    sus writer TabWriter = new_simple_tabwriter(headers)
    add_rows(&writer, data)
    damn render_table(&writer)
}

slay format_table_with_border(data tea[value][value], headers tea[value]) tea {
    sus writer TabWriter = new_simple_tabwriter(headers)
    add_rows(&writer, data)
    damn render_table_with_border(&writer)
}

# Utility functions for common text processing tasks

# Split text into words (whitespace delimited)
slay split_words(text tea) tea[value]{
    sus scanner Scanner = new_scanner(text, [" ", "\t", "\n", "\r"])
    damn scan_all_tokens(&scanner)
}

# Split text into lines
slay split_text_lines(text tea) tea[value]{
    sus scanner Scanner = new_scanner(text, ["\n", "\r\n"])
    sus lines tea[value] = []
    
    bestie (scan(&scanner)) {
        lines = append(lines, current_token(&scanner))
    }
    
    damn lines
}

# Parse space-separated values
slay parse_ssv(text tea) tea[value][value] {
    sus lines tea[value] = split_text_lines(text)
    sus records tea[value][value] = []
    
    bestie (sus i drip = 0; i < lines.length; i += 1) {
        sus line tea = lines[i]
        ready (line.length > 0) {
            sus fields tea[value] = split_words(line)
            records = append(records, fields)
        }
    }
    
    damn records
}

# Parse tab-separated values
slay parse_tsv(text tea) tea[value][value] {
    sus scanner CSVScanner = new_csv_scanner(text, CSVOptions{
        delimiter: '\t',
        quote_char: '"',
        escape_char: '"',
        trim_whitespace: cap,
        skip_empty_lines: based,
        allow_multiline: cap,
        comment_char: '#',
        strict_mode: based
    })
    
    sus records tea[value][value] = []
    bestie (scan_csv_record(&scanner)) {
        records = append(records, current_record(&scanner).fields)
    }
    
    damn records
}

# Advanced text analysis functions

# Count tokens in text
slay count_tokens(text tea, delimiters tea[value]) drip {
    sus scanner Scanner = new_scanner(text, delimiters)
    sus count drip = 0
    
    bestie (scan(&scanner)) {
        count += 1
    }
    
    damn count
}

# Find longest token
slay find_longest_token(text tea, delimiters tea[value]) tea {
    sus scanner Scanner = new_scanner(text, delimiters)
    sus longest tea = ""
    
    bestie (scan(&scanner)) {
        sus token tea = current_token(&scanner)
        ready (token.length > longest.length) {
            longest = token
        }
    }
    
    damn longest
}

# Get token statistics
slay get_token_stats(text tea, delimiters tea[value]) drip[value]{
    sus scanner Scanner = new_scanner(text, delimiters)
    sus total_count drip = 0
    sus total_length drip = 0
    sus max_length drip = 0
    sus min_length drip = 999999
    
    bestie (scan(&scanner)) {
        sus token tea = current_token(&scanner)
        total_count += 1
        total_length += token.length
        
        ready (token.length > max_length) {
            max_length = token.length
        }
        
        ready (token.length < min_length) {
            min_length = token.length
        }
    }
    
    sus avg_length drip = 0
    ready (total_count > 0) {
        avg_length = total_length / total_count
    } otherwise {
        min_length = 0
    }
    
    sus stats drip[value] = []
    stats = append(stats, total_count)   # 0: Total count
    stats = append(stats, avg_length)    # 1: Average length
    stats = append(stats, max_length)    # 2: Maximum length
    stats = append(stats, min_length)    # 3: Minimum length
    
    damn stats
}

# Quick table formatting for debug output
slay quick_table(data tea[value][value]) tea {
    ready (data.length == 0) {
        damn ""
    }
    
    sus headers tea[value] = []
    bestie (sus i drip = 0; i < data[0].length; i += 1) {
        headers = append(headers, "Column" + int_to_string(i + 1))
    }
    
    damn format_table_with_border(data, headers)
}

# Format data as aligned columns without borders
slay format_aligned_columns(data tea[value][value], separators tea[value]) tea {
    ready (data.length == 0) {
        damn ""
    }
    
    # Calculate column widths
    sus col_widths drip[value] = []
    bestie (sus row_idx drip = 0; row_idx < data.length; row_idx += 1) {
        bestie (sus col_idx drip = 0; col_idx < data[row_idx].length; col_idx += 1) {
            sus field_width drip = data[row_idx][col_idx].length
            
            ready (col_idx >= col_widths.length) {
                col_widths = append(col_widths, field_width)
            } otherwise ready (field_width > col_widths[col_idx]) {
                col_widths[col_idx] = field_width
            }
        }
    }
    
    sus result tea = ""
    sus sep tea = "  "  # Default separator
    
    ready (separators.length > 0) {
        sep = separators[0]
    }
    
    # Format each row
    bestie (sus row_idx drip = 0; row_idx < data.length; row_idx += 1) {
        bestie (sus col_idx drip = 0; col_idx < data[row_idx].length; col_idx += 1) {
            sus field tea = data[row_idx][col_idx]
            sus padding drip = col_widths[col_idx] - field.length
            
            result = result + field
            
            # Add padding
            bestie (sus i drip = 0; i < padding; i += 1) {
                result = result + " "
            }
            
            ready (col_idx < data[row_idx].length - 1) {
                result = result + sep
            }
        }
        result = result + "\n"
    }
    
    damn result
}

# Helper function for integer to string conversion
slay int_to_string(value drip) tea {
    ready (value == 0) {
        damn "0"
    }
    
    sus result tea = ""
    sus num drip = value
    sus negative lit = cap
    
    ready (num < 0) {
        negative = based
        num = -num
    }
    
    bestie (num > 0) {
        sus digit drip = num % 10
        result = char_to_string('0' + digit) + result
        num = num / 10
    }
    
    ready (negative) {
        result = "-" + result
    }
    
    damn result
}

# Examples and documentation functions

# Demonstrate scanner functionality
slay demo_scanner() tea {
    sus demo_text tea = "Hello,world|tab\tseparated\nline1\nline2"
    sus result tea = "=== Scanner Demo ===\n"
    
    # Basic word scanning
    sus words tea[value] = split_words("Hello world this is a test")
    result = result + "Words: " + join_strings(words, ", ") + "\n"
    
    # Custom delimiter scanning
    sus scanner Scanner = new_scanner(demo_text, [",", "|", "\t", "\n"])
    sus tokens tea[value] = scan_all_tokens(&scanner)
    result = result + "Custom tokens: " + join_strings(tokens, " | ") + "\n"
    
    # Line-by-line scanning
    sus line_data tea[value][value] = scan_lines("line1 word1 word2\nline2 word3 word4")
    result = result + "Line scanning:\n"
    bestie (sus i drip = 0; i < line_data.length; i += 1) {
        result = result + "  Line " + int_to_string(i + 1) + ": " + join_strings(line_data[i], ", ") + "\n"
    }
    
    damn result
}

# Demonstrate CSV functionality
slay demo_csv() tea {
    sus csv_text tea = "Name,Age,City\nJohn,25,\"New York\"\nJane,30,\"Los Angeles\""
    sus result tea = "=== CSV Demo ===\n"
    
    sus records tea[value][value] = parse_csv(csv_text)
    result = result + "Parsed " + int_to_string(records.length) + " records:\n"
    
    bestie (sus i drip = 0; i < records.length; i += 1) {
        result = result + "  Record " + int_to_string(i + 1) + ": " + join_strings(records[i], " | ") + "\n"
    }
    
    damn result
}

# Demonstrate table formatting
slay demo_table() tea {
    sus result tea = "=== Table Demo ===\n"
    
    sus headers tea[value] = ["Name", "Age", "City", "Country"]
    sus data tea[value][value] = [
        ["John Doe", "25", "New York", "USA"],
        ["Jane Smith", "30", "Los Angeles", "USA"],
        ["Bob Johnson", "35", "Chicago", "USA"]
    ]
    
    result = result + "Basic table:\n"
    result = result + format_table(data, headers) + "\n"
    
    result = result + "Table with border:\n"
    result = result + format_table_with_border(data, headers) + "\n"
    
    damn result
}

# Helper function to join strings
slay join_strings(strings tea[value], separator tea) tea {
    ready (strings.length == 0) {
        damn ""
    }
    
    sus result tea = strings[0]
    bestie (sus i drip = 1; i < strings.length; i += 1) {
        result = result + separator + strings[i]
    }
    
    damn result
}
