# Format Module

The `fmt` module provides comprehensive text formatting and string interpolation capabilities. Critical for self-hosting and display operations.

## Core Functions

### Basic Formatting
- `format_int(value normie) tea` - Format integer to string
- `format_float(value snack) tea` - Format float to string
- `format_bool(value lit) tea` - Format boolean to string
- `format_char(value sip) tea` - Format character to string

### String Formatting
- `format_string(template tea, args []tea) tea` - Format string with arguments
- `pad_left(s tea, width normie, pad_char sip) tea` - Left-pad string
- `pad_right(s tea, width normie, pad_char sip) tea` - Right-pad string
- `pad_center(s tea, width normie, pad_char sip) tea` - Center-pad string

### Number Base Formatting
- `format_binary(value normie) tea` - Format as binary
- `format_hex(value normie) tea` - Format as hexadecimal
- `format_octal(value normie) tea` - Format as octal
- `format_scientific(value snack) tea` - Format in scientific notation

### Precision and Currency
- `format_float_precision(value snack, precision normie) tea` - Format float with precision
- `format_currency(value snack, symbol tea) tea` - Format as currency
- `format_percentage(value snack) tea` - Format as percentage

### Table Formatting
- `format_table_row(columns []tea, widths []normie, separator tea) tea` - Format table row
- `format_table_header(columns []tea, widths []normie) tea` - Format table header

### Color and Style
- `format_with_color(text tea, color tea) tea` - Apply color formatting
- `format_bold(text tea) tea` - Apply bold formatting
- `format_italic(text tea) tea` - Apply italic formatting
- `format_underline(text tea) tea` - Apply underline formatting

### Message Formatting
- `format_error(message tea) tea` - Format error message
- `format_warning(message tea) tea` - Format warning message
- `format_success(message tea) tea` - Format success message
- `format_info(message tea) tea` - Format info message

### String Utilities
- `escape_string(s tea) tea` - Escape special characters
- `unescape_string(s tea) tea` - Unescape special characters
- `is_printable(ch sip) lit` - Check if character is printable
- `repeat_char(ch sip, count normie) tea` - Repeat character

## Usage Example

```cursed
yeet "fmt"

# Basic formatting
sus formatted := fmt.format_int(42)
vibez.spill("Number: " + formatted)

# Padding
sus padded := fmt.pad_left("hello", 10, ' ')
vibez.spill("Padded: '" + padded + "'")

# Color formatting
sus colored := fmt.format_with_color("Success!", "green")
vibez.spill(colored)

# Currency formatting
sus price := fmt.format_currency(19.99, "$")
vibez.spill("Price: " + price)

# Table formatting
sus columns := []tea{"Name", "Age"}
sus widths := []normie{10, 5}
sus header := fmt.format_table_header(columns, widths)
vibez.spill(header)
```

## Testing

```bash
cargo run --bin cursed stdlib/fmt/test_fmt.💀
```

## Status

✅ **Production Ready** - Fully implemented and tested
- Comprehensive formatting functions
- Color and style support
- Table formatting utilities
- String manipulation and escaping
- Zero FFI dependencies
