# Tab Aesthetic Module

The `tab_aesthetic` module provides aligned text output formatting capabilities for CURSED, enabling the creation of well-formatted tables, columns, and structured text output. This module is particularly useful for development tools, debugging output, and self-hosting scenarios where readable formatted output is essential.

## Features

- **Column Alignment**: Support for left, right, and center alignment
- **Custom Separators**: Configurable column separators (tabs, pipes, commas, etc.)
- **Table Formatting**: Create formatted tables with headers and data rows
- **Multiple Output Formats**: Support for plain text, CSV, and Markdown table formats
- **Key-Value Tables**: Format configuration and status information
- **FFI-Free**: Pure CURSED implementation with no external dependencies

## Core Functions

### TabWriter Structure

```cursed
sus TabWriter struct {
    columns []tea,
    widths []normie,
    alignment tea,
    separator tea,
    padding normie
}
```

### Basic Tab Writer Operations

#### `tab_writer_new() TabWriter`
Creates a new tab writer with default settings (left alignment, tab separator, padding of 1).

```cursed
sus writer TabWriter = tab_writer_new()
```

#### `tab_writer_with_separator(sep tea) TabWriter`
Creates a tab writer with a custom separator.

```cursed
sus writer TabWriter = tab_writer_with_separator("|")
```

#### `tab_writer_with_alignment(align tea) TabWriter`
Creates a tab writer with custom alignment ("left", "right", "center").

```cursed
sus writer TabWriter = tab_writer_with_alignment("center")
```

#### Configuration Functions

- `tab_writer_set_alignment(writer *TabWriter, align tea) lit`
- `tab_writer_set_separator(writer *TabWriter, sep tea) lit`
- `tab_writer_set_padding(writer *TabWriter, pad normie) lit`

#### Data Management

- `tab_writer_add_row(writer *TabWriter, row []tea) lit` - Add a row of data
- `tab_writer_flush(writer *TabWriter) tea` - Format and return aligned output
- `tab_writer_clear(writer *TabWriter) lit` - Clear all data

### High-Level Formatting Functions

#### `tab_aesthetic_format_table(data [][]tea, alignment tea) tea`
Format a 2D array into an aligned table.

```cursed
sus data [][]tea = [][]tea{
    []tea{"Name", "Age", "City"},
    []tea{"John", "25", "NYC"},
    []tea{"Jane", "30", "LA"}
}
sus result tea = tab_aesthetic_format_table(data, "left")
```

#### `tab_aesthetic_table_with_headers(headers []tea, rows [][]tea, alignment tea) tea`
Create a table with headers and separator line.

```cursed
sus headers []tea = []tea{"Name", "Score"}
sus rows [][]tea = [][]tea{
    []tea{"Alice", "95"},
    []tea{"Bob", "87"}
}
sus result tea = tab_aesthetic_table_with_headers(headers, rows, "center")
```

#### `tab_aesthetic_key_value_table(keys []tea, values []tea) tea`
Format key-value pairs as an aligned table.

```cursed
sus keys []tea = []tea{"Name", "Version", "Author"}
sus values []tea = []tea{"CURSED", "1.0", "Developer"}
sus result tea = tab_aesthetic_key_value_table(keys, values)
```

#### `tab_aesthetic_markdown_table(headers []tea, rows [][]tea) tea`
Generate a markdown-formatted table.

```cursed
sus headers []tea = []tea{"Feature", "Status"}
sus rows [][]tea = [][]tea{
    []tea{"Parser", "Complete"},
    []tea{"Compiler", "Beta"}
}
sus result tea = tab_aesthetic_markdown_table(headers, rows)
```

## Usage Examples

### Basic Table Formatting

```cursed
yeet "tab_aesthetic"

# Create a simple aligned table
sus data [][]tea = [][]tea{
    []tea{"Language", "Type", "Year"},
    []tea{"CURSED", "Compiled", "2024"},
    []tea{"Go", "Compiled", "2009"},
    []tea{"Python", "Interpreted", "1991"}
}

sus table tea = tab_aesthetic_format_table(data, "left")
vibez.spill(table)
```

Output:
```
Language Type        Year
CURSED   Compiled    2024
Go       Compiled    2009
Python   Interpreted 1991
```

### Configuration Table

```cursed
sus config_keys []tea = []tea{"Compiler", "Version", "Target", "Debug"}
sus config_values []tea = []tea{"CURSED", "1.0.0", "x86_64", "Enabled"}
sus config_table tea = tab_aesthetic_key_value_table(config_keys, config_values)
vibez.spill(config_table)
```

Output:
```
Compiler CURSED
Version  1.0.0
Target   x86_64
Debug    Enabled
```

### Custom Alignment and Separators

```cursed
# Right-aligned table with pipe separators
sus writer TabWriter = tab_writer_with_separator("|")
tab_writer_set_alignment(&writer, "right")

sus row1 []tea = []tea{"Item", "Price", "Stock"}
sus row2 []tea = []tea{"Apple", "$1.50", "50"}
sus row3 []tea = []tea{"Orange", "$2.00", "25"}

tab_writer_add_row(&writer, row1)
tab_writer_add_row(&writer, row2)
tab_writer_add_row(&writer, row3)

sus result tea = tab_writer_flush(&writer)
vibez.spill(result)
```

### Markdown Table Generation

```cursed
sus headers []tea = []tea{"Module", "Status", "Coverage"}
sus rows [][]tea = [][]tea{
    []tea{"Parser", "Complete", "98%"},
    []tea{"Compiler", "Complete", "95%"},
    []tea{"Runtime", "Complete", "92%"}
}

sus markdown tea = tab_aesthetic_markdown_table(headers, rows)
vibez.spill(markdown)
```

Output:
```
| Module   | Status   | Coverage |
| ---      | ---      | ---      |
| Parser   | Complete | 98%      |
| Compiler | Complete | 95%      |
| Runtime  | Complete | 92%      |
```

## Use Cases

### Development Tools
- Formatting compiler output and diagnostics
- Creating structured debug information
- Displaying test results and coverage reports

### Self-Hosting Scenarios
- Bootstrap process status display
- Configuration file formatting
- Module dependency tables

### Data Presentation
- CSV-style data formatting
- Configuration summaries
- Performance metrics display

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/tab_aesthetic/test_tab_aesthetic.💀
```

Test both interpretation and compilation modes:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/tab_aesthetic/test_tab_aesthetic.💀

# Compilation mode
cargo run --bin cursed -- compile stdlib/tab_aesthetic/test_tab_aesthetic.💀
./test_tab_aesthetic
```

## Implementation Details

- **Pure CURSED**: No FFI dependencies, entirely implemented in CURSED
- **Memory Efficient**: Minimal memory overhead with efficient string operations
- **Flexible**: Supports various alignment options and separators
- **Robust**: Handles edge cases like empty data and varying column counts
- **Testing**: Comprehensive test suite with 30+ test cases

## Dependencies

- `testz` - Testing framework
- `stringz` - String manipulation utilities

## Performance Characteristics

- **Time Complexity**: O(n*m) where n is rows and m is columns
- **Space Complexity**: O(n*m) for storing formatted data
- **Efficiency**: Optimized for readability over raw performance
- **Scalability**: Suitable for moderate-sized tables (up to 1000s of rows)
