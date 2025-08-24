# CURSED Scanner and TabWriter Module (scanz)

The `scanz` module provides comprehensive text scanning, parsing, and tabular data formatting capabilities for CURSED. It includes powerful text scanners with configurable delimiters, RFC 4180 compliant CSV parsing, and professional table formatting with alignment and styling options.

## Features

### Text Scanner
- **Flexible Delimiter Support**: Configure any combination of string delimiters
- **Token-by-Token Parsing**: Scan text incrementally with position tracking
- **Line and Column Tracking**: Track current position for error reporting
- **Peek Functionality**: Look ahead without advancing the scanner
- **Whitespace Handling**: Configurable whitespace skipping
- **Reset Capability**: Reset scanner to beginning for re-parsing

### CSV Scanner
- **RFC 4180 Compliant**: Full support for CSV standard specifications
- **Quote Handling**: Proper parsing of quoted fields with escape sequences
- **Multiline Support**: Handle multiline fields within quotes
- **Custom Options**: Configurable delimiters, quote characters, and parsing behavior
- **Header Processing**: Automatic header row detection and processing
- **Error Recovery**: Robust error handling with detailed error messages
- **Validation**: CSV structure validation and consistency checking

### TabWriter (Table Formatter)
- **Column Alignment**: Left, center, and right alignment options
- **Auto-sizing**: Automatic column width calculation based on content
- **Width Constraints**: Minimum and maximum width limits
- **Content Truncation**: Smart truncation with ellipsis for long content
- **Border Support**: Optional table borders with customizable characters
- **Multiple Formats**: Plain text, bordered, and CSV output formats
- **Padding Control**: Configurable cell padding for better readability

## Quick Start

### Basic Text Scanning

```cursed
yeet "scanz"

# Simple word scanning
sus text tea = "hello world CURSED programming"
sus words []tea = split_words(text)
vibez.spill("Words:", words)  # ["hello", "world", "CURSED", "programming"]

# Custom delimiter scanning
sus data tea = "apple,banana|grape;orange"
sus tokens []tea = scan_tokens(data, [",", "|", ";"])
vibez.spill("Tokens:", tokens)  # ["apple", "banana", "grape", "orange"]

# Line-by-line scanning
sus multi_line tea = "line1 word1 word2\nline2 word3 word4"
sus lines [][]tea = scan_lines(multi_line)
vibez.spill("Line 1:", lines[0])  # ["line1", "word1", "word2"]
vibez.spill("Line 2:", lines[1])  # ["line2", "word3", "word4"]
```

### CSV Parsing

```cursed
yeet "scanz"

# Basic CSV parsing
sus csv_text tea = "Name,Age,City\nJohn Doe,25,New York\nJane Smith,30,Los Angeles"
sus records [][]tea = parse_csv(csv_text)

vibez.spill("Header:", records[0])     # ["Name", "Age", "City"]
vibez.spill("Record 1:", records[1])   # ["John Doe", "25", "New York"]
vibez.spill("Record 2:", records[2])   # ["Jane Smith", "30", "Los Angeles"]

# CSV with quoted fields and commas
sus quoted_csv tea = "Name,Description,Salary\n\"Smith, John\",\"Senior Developer, Team Lead\",75000"
sus quoted_records [][]tea = parse_csv(quoted_csv)
vibez.spill("Name:", quoted_records[1][0])  # "Smith, John"
vibez.spill("Role:", quoted_records[1][1])  # "Senior Developer, Team Lead"
```

### Table Formatting

```cursed
yeet "scanz"

# Create and populate a table
sus headers []tea = ["Name", "Age", "City", "Country"]
sus writer TabWriter = create_table(headers)

add_row(&writer, ["John Doe", "25", "New York", "USA"])
add_row(&writer, ["Jane Smith", "30", "Los Angeles", "USA"])
add_row(&writer, ["Bob Johnson", "35", "Chicago", "USA"])

# Render basic table
sus table tea = render_table(&writer)
vibez.spill(table)

# Render with borders
sus bordered_table tea = render_table_with_border(&writer)
vibez.spill(bordered_table)
```

## Advanced Usage

### Custom Scanner Configuration

```cursed
# Create scanner with custom delimiters
sus custom_scanner Scanner = new_text_scanner(
    "data:value|info:details", 
    [":", "|"]
)

bestie (scan(&custom_scanner)) {
    vibez.spill("Token:", current_token(&custom_scanner))
    vibez.spill("Position:", current_line(&custom_scanner), current_column(&custom_scanner))
}
```

### Advanced CSV Options

```cursed
# Create CSV scanner with custom options
sus csv_options CSVOptions = CSVOptions{
    delimiter: '\t',              # Tab-separated values
    quote_char: '"',             # Double quote for quoting
    escape_char: '\\',           # Backslash for escaping
    trim_whitespace: based,      # Trim whitespace from fields
    skip_empty_lines: based,     # Skip empty lines
    allow_multiline: cap,        # Disallow multiline fields
    comment_char: '#',           # Hash for comment lines
    strict_mode: based           # Strict RFC 4180 compliance
}

sus tsv_scanner CSVScanner = new_csv_scanner(tsv_text, csv_options)
```

### Advanced Table Configuration

```cursed
# Create table with custom column configuration
sus columns []TableColumn = [
    TableColumn{
        header: "ID",
        width: 5,
        min_width: 3,
        max_width: 10,
        alignment: ColumnAlignment.RIGHT,
        padding: 1,
        truncate: cap
    },
    TableColumn{
        header: "Description",
        width: 30,
        min_width: 10,
        max_width: 50,
        alignment: ColumnAlignment.LEFT,
        padding: 2,
        truncate: based
    }
]

sus custom_writer TabWriter = new_tabwriter(columns)
```

## API Reference

### Scanner Functions

#### Core Scanner Operations
- `new_text_scanner(text tea, delims []tea) Scanner` - Create new scanner with delimiters
- `scan(scanner *Scanner) lit` - Scan next token, returns true if token found
- `current_token(scanner *Scanner) tea` - Get current scanned token
- `has_more_tokens(scanner *Scanner) lit` - Check if more tokens available
- `peek_token(scanner *Scanner) tea` - Peek at next token without advancing
- `reset(scanner *Scanner)` - Reset scanner to beginning

#### Position Tracking
- `current_line(scanner *Scanner) drip` - Get current line number (1-based)
- `current_column(scanner *Scanner) drip` - Get current column number (1-based)
- `skip_to_next_line(scanner *Scanner)` - Skip to next line

#### Utility Functions
- `scan_all_tokens(scanner *Scanner) []tea` - Scan all remaining tokens
- `scan_line(scanner *Scanner) []tea` - Scan tokens from current line only
- `scan_matching(scanner *Scanner, filter slay(tea) lit) []tea` - Scan tokens matching filter

### CSV Functions

#### CSV Scanner Operations
- `new_csv_scanner(text tea, options CSVOptions) CSVScanner` - Create CSV scanner
- `new_simple_csv_scanner(text tea) CSVScanner` - Create CSV scanner with defaults
- `scan_csv_record(scanner *CSVScanner) lit` - Scan next CSV record
- `current_record(scanner *CSVScanner) CSVRecord` - Get current parsed record
- `parse_header(scanner *CSVScanner) lit` - Parse and store header row
- `get_header(scanner *CSVScanner) []tea` - Get parsed header

#### CSV Utility Functions
- `parse_csv(text tea) [][]tea` - Parse entire CSV into records
- `parse_csv_with_options(text tea, options CSVOptions) [][]tea` - Parse with options
- `parse_all_records(scanner *CSVScanner) []CSVRecord` - Parse all records
- `validate_csv(scanner *CSVScanner) lit` - Validate CSV structure consistency

### TabWriter Functions

#### Table Creation and Management
- `new_tabwriter(columns []TableColumn) TabWriter` - Create with column config
- `create_table(headers []tea) TabWriter` - Create simple table with headers
- `add_row(writer *TabWriter, row_data []tea)` - Add single row
- `add_rows(writer *TabWriter, rows [][]tea)` - Add multiple rows
- `clear_rows(writer *TabWriter)` - Clear all rows

#### Rendering Functions
- `render_table(writer *TabWriter) tea` - Render plain table
- `render_table_with_border(writer *TabWriter) tea` - Render with borders
- `render_csv(writer *TabWriter, include_header lit) tea` - Render as CSV

#### Configuration Functions
- `set_separator(writer *TabWriter, sep tea)` - Set column separator
- `set_header_separator(writer *TabWriter, sep tea)` - Set header separator
- `set_auto_size(writer *TabWriter, auto_size lit)` - Enable/disable auto-sizing
- `auto_size_columns(writer *TabWriter)` - Manually trigger auto-sizing

### Utility Functions

#### Text Processing
- `split_words(text tea) []tea` - Split text into words (whitespace delimited)
- `split_text_lines(text tea) []tea` - Split text into lines
- `parse_ssv(text tea) [][]tea` - Parse space-separated values
- `parse_tsv(text tea) [][]tea` - Parse tab-separated values

#### Analysis Functions
- `count_tokens(text tea, delimiters []tea) drip` - Count tokens in text
- `find_longest_token(text tea, delimiters []tea) tea` - Find longest token
- `get_token_stats(text tea, delimiters []tea) []drip` - Get token statistics

#### Quick Formatting
- `quick_table(data [][]tea) tea` - Quick table with auto-generated headers
- `format_table(data [][]tea, headers []tea) tea` - Format table without borders
- `format_table_with_border(data [][]tea, headers []tea) tea` - Format with borders
- `format_aligned_columns(data [][]tea, separators []tea) tea` - Aligned columns

## Data Structures

### Scanner Structure
```cursed
squad Scanner {
    sus text tea             # Source text
    sus position drip        # Current position
    sus length drip          # Text length
    sus delimiters []tea     # Delimiter strings
    sus skip_whitespace lit  # Skip whitespace flag
    sus current_token tea    # Current token
    sus line_number drip     # Line number (1-based)
    sus column_number drip   # Column number (1-based)
    sus has_more lit         # More tokens available
}
```

### CSV Options Structure
```cursed
squad CSVOptions {
    sus delimiter rune          # Field delimiter
    sus quote_char rune         # Quote character
    sus escape_char rune        # Escape character
    sus trim_whitespace lit     # Trim whitespace
    sus skip_empty_lines lit    # Skip empty lines
    sus allow_multiline lit     # Allow multiline fields
    sus comment_char rune       # Comment indicator
    sus strict_mode lit         # Strict compliance
}
```

### Table Column Structure
```cursed
squad TableColumn {
    sus header tea          # Column header
    sus width drip          # Column width
    sus min_width drip      # Minimum width
    sus max_width drip      # Maximum width
    sus alignment drip      # Alignment (LEFT/CENTER/RIGHT)
    sus padding drip        # Cell padding
    sus truncate lit        # Truncate content
}
```

## Examples and Use Cases

### Log File Analysis
```cursed
yeet "scanz"

slay analyze_log_file(log_content tea) {
    sus lines [][]tea = scan_lines(log_content)
    sus stats []drip = [0, 0, 0]  # info, warn, error counts
    
    bestie (sus i drip = 0; i < lines.length; i += 1) {
        ready (lines[i].length >= 3) {
            sus level tea = lines[i][2]
            ready (level == "INFO") {
                stats[0] += 1
            } otherwise ready (level == "WARN") {
                stats[1] += 1
            } otherwise ready (level == "ERROR") {
                stats[2] += 1
            }
        }
    }
    
    vibez.spill("Log Analysis:")
    vibez.spill("INFO:", stats[0], "WARN:", stats[1], "ERROR:", stats[2])
}
```

### CSV Data Processing
```cursed
yeet "scanz"

slay process_employee_data(csv_data tea) {
    sus scanner CSVScanner = new_simple_csv_scanner(csv_data)
    
    # Parse header
    ready (!parse_header(&scanner)) {
        vibez.spill("Failed to parse CSV header")
        damn
    }
    
    sus headers []tea = get_header(&scanner)
    sus writer TabWriter = create_table(headers)
    
    # Process records
    bestie (scan_csv_record(&scanner)) {
        sus record CSVRecord = current_record(&scanner)
        add_row(&writer, record.fields)
    }
    
    # Display formatted table
    vibez.spill(render_table_with_border(&writer))
}
```

### Configuration File Parser
```cursed
yeet "scanz"

slay parse_config_file(config_content tea) {
    sus lines []tea = split_text_lines(config_content)
    sus config_pairs [][]tea = []
    
    bestie (sus i drip = 0; i < lines.length; i += 1) {
        sus line tea = lines[i]
        
        # Skip comments and empty lines
        ready (line.length == 0 || line[0] == '#') {
            continue
        }
        
        # Parse key=value pairs
        sus kv_scanner Scanner = new_text_scanner(line, ["="])
        sus kv_tokens []tea = scan_all_tokens(&kv_scanner)
        
        ready (kv_tokens.length == 2) {
            config_pairs = append(config_pairs, kv_tokens)
        }
    }
    
    # Display as table
    sus headers []tea = ["Key", "Value"]
    sus formatted tea = format_table_with_border(config_pairs, headers)
    vibez.spill("Configuration:")
    vibez.spill(formatted)
}
```

## Performance Considerations

- **Memory Efficiency**: Scanners process text incrementally without loading everything into memory
- **Large Files**: Use line-by-line scanning for large files to minimize memory usage
- **Token Caching**: Current token is cached to avoid re-parsing
- **Column Auto-sizing**: Can be expensive for large tables; disable if performance is critical
- **CSV Validation**: Strict mode validation adds overhead but ensures data integrity

## Error Handling

The scanz module provides comprehensive error handling:

- **Scanner Errors**: Position tracking helps identify problematic input locations
- **CSV Errors**: Detailed error messages for malformed CSV data
- **Table Errors**: Validation of column configurations and data consistency
- **Recovery**: Robust error recovery allows continued processing after errors

## Integration with Other Modules

The scanz module integrates well with other CURSED stdlib modules:

- **filez**: Read files and process with scanners
- **networkz**: Parse HTTP headers and form data
- **jsonz**: Pre-process JSON data with custom delimiters
- **xmlz**: Parse XML attributes and text content
- **stringz**: Enhanced string processing with scanning

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed-zig stdlib/scanz/test_scanz.csd
```

The test suite includes:
- Basic scanner functionality
- Custom delimiter handling
- CSV parsing edge cases
- Table formatting options
- Performance stress tests
- Real-world data scenarios
- Error handling validation

## License

Part of the CURSED programming language standard library.
Copyright (c) 2025 The CURSED Team.
