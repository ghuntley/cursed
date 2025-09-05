# CSV Module

A comprehensive CSV (Comma-Separated Values) processing module for CURSED that implements RFC 4180 standard with additional features for data manipulation.

## Overview

The CSV module provides a complete suite of functions for parsing, generating, and manipulating CSV data. It supports various delimiters, quoted fields, escaped characters, and common CSV variations found in real-world data.

## Features

- **RFC 4180 Compliant**: Full support for standard CSV format
- **Multiple Delimiters**: Auto-detection and support for comma, semicolon, and tab delimiters
- **Quoted Fields**: Proper handling of fields containing commas, quotes, and newlines
- **Header Support**: Parse CSV with or without headers, convert to key-value pairs
- **Data Manipulation**: Filter, sort, transpose, and modify CSV data
- **Validation**: Check CSV syntax and structure
- **Round-trip Consistency**: Parse and stringify operations preserve data integrity

## Core Functions

### Parsing Functions

#### `parse(csv_string tea) array`
Parse a CSV string into an array of arrays (rows and columns).

```cursed
sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
sus result array = csv.parse(csv_data)
# result[0] = ["name", "age", "city"]
# result[1] = ["John", "25", "NYC"]
# result[2] = ["Jane", "30", "LA"]
```

#### `parse_with_headers(csv_string tea) array`
Parse CSV with first row as headers, returning array of key-value pair arrays.

```cursed
sus csv_data tea = "name,age\nJohn,25\nJane,30"
sus result array = csv.parse_with_headers(csv_data)
# result[0] = [["name", "John"], ["age", "25"]]
# result[1] = [["name", "Jane"], ["age", "30"]]
```

#### `parse_row(row_string tea, delimiter tea) array`
Parse a single CSV row with specified delimiter.

```cursed
sus row tea = "John,25,NYC"
sus fields array = csv.parse_row(row, ",")
# fields = ["John", "25", "NYC"]
```

### Generation Functions

#### `stringify(data array) tea`
Convert array of arrays to CSV string.

```cursed
sus data array = [["name", "age"], ["John", "25"], ["Jane", "30"]]
sus csv_string tea = csv.stringify(data)
# csv_string = "name,age\nJohn,25\nJane,30"
```

#### `stringify_with_headers(data array, headers array) tea`
Convert data array with separate headers to CSV string.

```cursed
sus data array = [["John", "25"], ["Jane", "30"]]
sus headers array = ["name", "age"]
sus csv_string tea = csv.stringify_with_headers(data, headers)
# csv_string = "name,age\nJohn,25\nJane,30"
```

### Field Processing Functions

#### `escape_field(field tea) tea`
Escape field for CSV output (adds quotes if needed).

```cursed
sus field tea = "value, with comma"
sus escaped tea = csv.escape_field(field)
# escaped = "\"value, with comma\""
```

#### `unescape_field(field tea) tea`
Remove CSV escaping from field.

```cursed
sus field tea = "\"value, with comma\""
sus unescaped tea = csv.unescape_field(field)
# unescaped = "value, with comma"
```

### Analysis Functions

#### `detect_delimiter(csv_string tea) tea`
Auto-detect delimiter (comma, semicolon, or tab).

```cursed
sus csv_data tea = "name;age;city\nJohn;25;NYC"
sus delimiter tea = csv.detect_delimiter(csv_data)
# delimiter = ";"
```

#### `validate(csv_string tea) lit`
Validate CSV syntax and structure.

```cursed
sus valid_csv tea = "name,age\nJohn,25\nJane,30"
sus is_valid lit = csv.validate(valid_csv)
# is_valid = based (true)
```

#### `count_rows(csv_string tea) normie`
Count number of rows in CSV.

#### `count_columns(csv_string tea) normie`
Count number of columns in first row.

#### `get_headers(csv_string tea) array`
Extract headers from first row.

### Data Manipulation Functions

#### `filter_rows(data array, column_index normie, value tea) array`
Filter rows by column value.

```cursed
sus data array = [["name", "age"], ["John", "25"], ["Jane", "30"], ["John", "35"]]
sus filtered array = csv.filter_rows(data, 0, "John")
# Returns rows where first column equals "John"
```

#### `sort_by_column(data array, column_index normie) array`
Sort rows by specified column.

```cursed
sus data array = [["name", "age"], ["John", "30"], ["Jane", "25"]]
sus sorted array = csv.sort_by_column(data, 1)
# Sorts by age column
```

#### `get_column(data array, column_index normie) array`
Extract specific column as array.

```cursed
sus data array = [["name", "age"], ["John", "25"], ["Jane", "30"]]
sus names array = csv.get_column(data, 0)
# names = ["name", "John", "Jane"]
```

#### `remove_column(data array, column_index normie) array`
Remove column from data.

#### `transpose(data array) array`
Transpose rows and columns.

```cursed
sus data array = [["name", "age"], ["John", "25"], ["Jane", "30"]]
sus transposed array = csv.transpose(data)
# transposed[0] = ["name", "John", "Jane"]
# transposed[1] = ["age", "25", "30"]
```

## CSV Format Support

### Standard Features
- Comma-separated values
- Quoted fields with double quotes
- Escaped quotes within fields (`""`)
- Fields containing commas, newlines, and quotes
- Empty fields and rows

### Extended Features
- Multiple delimiters (comma, semicolon, tab)
- Automatic delimiter detection
- Header row processing
- Data validation and structure checking

### Line Endings
Supports different line ending formats:
- Unix/Linux: `\n`
- Windows: `\r\n` 
- Classic Mac: `\r`

## Usage Patterns

### Basic CSV Processing
```cursed
yeet "csv"

sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
sus parsed array = csv.parse(csv_data)

# Process each row
bestie i := 0; i < string.len(parsed); i++ {
    sus row array = parsed[i]
    vibez.spill("Row: " + csv.stringify([row]))
}
```

### Working with Headers
```cursed
sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
sus headers array = csv.get_headers(csv_data)
sus data_with_headers array = csv.parse_with_headers(csv_data)

# Access data as key-value pairs
sus first_record array = data_with_headers[0]
bestie i := 0; i < string.len(first_record); i++ {
    sus pair array = first_record[i]
    vibez.spill(pair[0] + ": " + pair[1])
}
```

### Data Analysis
```cursed
sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA\nBob,25,Chicago"
sus parsed array = csv.parse(csv_data)

# Filter by age
sus young_people array = csv.filter_rows(parsed, 1, "25")

# Sort by name
sus sorted_data array = csv.sort_by_column(parsed, 0)

# Get all cities
sus cities array = csv.get_column(parsed, 2)
```

## Performance Considerations

### Memory Usage
- Large CSV files are processed in memory
- Consider streaming for very large files
- Use filtering early to reduce memory footprint

### Processing Speed
- Parsing is O(n) where n is the number of characters
- Sorting is O(n log n) where n is the number of rows
- Filtering is O(n) where n is the number of rows

### Best Practices
- Validate CSV structure before processing
- Use appropriate data types for numeric columns
- Consider delimiter detection for unknown formats
- Handle empty fields gracefully in your application logic

## Error Handling

The module handles various error conditions gracefully:

- **Malformed CSV**: `validate()` returns `cap` (false) for invalid structure
- **Empty data**: Functions return empty arrays for empty input
- **Missing fields**: Short rows are padded with empty strings
- **Invalid indices**: Column operations handle out-of-bounds gracefully

## Testing

The module includes comprehensive tests covering:
- Basic parsing and generation
- Various delimiters and formats
- Quoted fields and escape sequences
- Empty fields and edge cases
- Data manipulation operations
- Round-trip consistency
- Error conditions

Run tests with:
```bash
cargo run --bin cursed stdlib/csv/test_csv.💀
```

## Integration

Import the CSV module in your CURSED programs:

```cursed
yeet "csv"
yeet "string"  # Often needed for string operations

# Use CSV functions
sus data array = csv.parse(csv_string)
```

The module integrates seamlessly with other CURSED stdlib modules like `string` for additional text processing capabilities.
