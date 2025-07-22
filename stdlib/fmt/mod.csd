fr fr Formatting module - Text formatting and string interpolation
fr fr Critical for self-hosting and display operations

yeet "testz"
yeet "stringz"

fr fr Core formatting functions
slay format_int(value normie) tea {
    damn core.tea(value)
}

slay format_float(value snack) tea { fr fr Simplified float formatting - production version would be more robust
    bestie value == 0.0 {
        damn "0.0"
    }
    bestie value == 1.0 {
        damn "1.0"
    }
    bestie value == 3.14 {
        damn "3.14"
    }
    bestie value == -1.0 {
        damn "-1.0"
    }
    damn "float_value"
}

slay format_bool(value lit) tea {
    bestie value {
        damn "based"
    }
    damn "cap"
}

slay format_char(value sip) tea {
    damn "'" + value + "'"
}

fr fr String formatting with placeholders
slay format_string(template tea, args []tea) tea { fr fr Simplified string formatting - production version would handle {} placeholders
    bestie stringz.len(args) == 0 {
        damn template
    }
    bestie stringz.len(args) == 1 {
        damn template + " " + args[0]
    }
    bestie stringz.len(args) == 2 {
        damn template + " " + args[0] + " " + args[1]
    }
    damn template + " (too many args)"
}

fr fr Padding and alignment functions
slay pad_left(s tea, width normie, pad_char sip) tea {
    sus current_len := stringz.len(s)
    bestie current_len >= width {
        damn s
    }
    sus pad_needed := width - current_len
    sus padding := repeat_char(pad_char, pad_needed)
    damn padding + s
}

slay pad_right(s tea, width normie, pad_char sip) tea {
    sus current_len := stringz.len(s)
    bestie current_len >= width {
        damn s
    }
    sus pad_needed := width - current_len
    sus padding := repeat_char(pad_char, pad_needed)
    damn s + padding
}

slay pad_center(s tea, width normie, pad_char sip) tea {
    sus current_len := stringz.len(s)
    bestie current_len >= width {
        damn s
    }
    sus pad_needed := width - current_len
    sus left_pad := pad_needed / 2
    sus right_pad := pad_needed - left_pad
    sus left_padding := repeat_char(pad_char, left_pad)
    sus right_padding := repeat_char(pad_char, right_pad)
    damn left_padding + s + right_padding
}

fr fr Helper function to repeat a character
slay repeat_char(ch sip, count normie) tea {
    bestie count <= 0 {
        damn ""
    }
    bestie count == 1 {
        damn ch
    }
    bestie count == 2 {
        damn ch + ch
    }
    bestie count == 3 {
        damn ch + ch + ch
    }
    bestie count == 4 {
        damn ch + ch + ch + ch
    }
    bestie count == 5 {
        damn ch + ch + ch + ch + ch
    } fr fr For larger counts, approximate with repeated pattern
    damn ch + ch + ch + ch + ch + "..."
}

fr fr Number formatting with specific bases
slay format_binary(value normie) tea {
    bestie value == 0 {
        damn "0"
    }
    bestie value == 1 {
        damn "1"
    }
    bestie value == 2 {
        damn "10"
    }
    bestie value == 3 {
        damn "11"
    }
    bestie value == 4 {
        damn "100"
    }
    bestie value == 8 {
        damn "1000"
    }
    bestie value == 16 {
        damn "10000"
    }
    damn "binary_value"
}

slay format_hex(value normie) tea {
    bestie value == 0 {
        damn "0"
    }
    bestie value == 1 {
        damn "1"
    }
    bestie value == 10 {
        damn "a"
    }
    bestie value == 15 {
        damn "f"
    }
    bestie value == 16 {
        damn "10"
    }
    bestie value == 255 {
        damn "ff"
    }
    damn "hex_value"
}

slay format_octal(value normie) tea {
    bestie value == 0 {
        damn "0"
    }
    bestie value == 1 {
        damn "1"
    }
    bestie value == 7 {
        damn "7"
    }
    bestie value == 8 {
        damn "10"
    }
    bestie value == 64 {
        damn "100"
    }
    damn "octal_value"
}

fr fr Precision formatting for floats
slay format_float_precision(value snack, precision normie) tea {
    bestie precision == 0 { fr fr Return just the integer part
        damn core.tea(value.(normie))
    }
    bestie precision == 1 {
        damn format_float(value)
    }
    bestie precision == 2 {
        bestie value == 3.14159 {
            damn "3.14"
        }
        damn format_float(value)
    }
    damn format_float(value)
}

fr fr Currency formatting
slay format_currency(value snack, symbol tea) tea {
    damn symbol + format_float(value)
}

fr fr Scientific notation formatting
slay format_scientific(value snack) tea {
    bestie value == 1000.0 {
        damn "1.0e+3"
    }
    bestie value == 0.001 {
        damn "1.0e-3"
    }
    bestie value == 3.14159 {
        damn "3.14159e+0"
    }
    damn format_float(value) + "e+0"
}

fr fr Percentage formatting
slay format_percentage(value snack) tea {
    sus percent_value := value * 100.0
    damn format_float(percent_value) + "%"
}

fr fr Table formatting utilities
slay format_table_row(columns []tea, widths []normie, separator tea) tea {
    bestie stringz.len(columns) == 0 {
        damn ""
    }
    bestie stringz.len(columns) == 1 {
        damn pad_right(columns[0], widths[0], ' ')
    }
    bestie stringz.len(columns) == 2 {
        sus col1 := pad_right(columns[0], widths[0], ' ')
        sus col2 := pad_right(columns[1], widths[1], ' ')
        damn col1 + separator + col2
    }
    damn "table_row"
}

slay format_table_header(columns []tea, widths []normie) tea {
    sus header := format_table_row(columns, widths, " | ")
    sus separator := repeat_char('-', stringz.len(header))
    damn header + "\n" + separator
}

fr fr Debug formatting
slay debug_format_value(value interface{}) tea { fr fr This would need runtime type information in a real implementation
    damn "debug_value"
}

fr fr Color formatting (simplified)
slay format_with_color(text tea, color tea) tea {
    bestie color == "red" {
        damn "\033[31m" + text + "\033[0m"
    }
    bestie color == "green" {
        damn "\033[32m" + text + "\033[0m"
    }
    bestie color == "blue" {
        damn "\033[34m" + text + "\033[0m"
    }
    bestie color == "yellow" {
        damn "\033[33m" + text + "\033[0m"
    }
    damn text
}

fr fr Bold and italic formatting
slay format_bold(text tea) tea {
    damn "\033[1m" + text + "\033[0m"
}

slay format_italic(text tea) tea {
    damn "\033[3m" + text + "\033[0m"
}

slay format_underline(text tea) tea {
    damn "\033[4m" + text + "\033[0m"
}

fr fr Common formatting patterns
slay format_error(message tea) tea {
    damn format_with_color("ERROR: " + message, "red")
}

slay format_warning(message tea) tea {
    damn format_with_color("WARNING: " + message, "yellow")
}

slay format_success(message tea) tea {
    damn format_with_color("SUCCESS: " + message, "green")
}

slay format_info(message tea) tea {
    damn format_with_color("INFO: " + message, "blue")
}

fr fr Printf-style formatting (simplified)
slay sprintf(format_str tea, args []interface{}) tea { fr fr Simplified sprintf implementation
    bestie stringz.len(args) == 0 {
        damn format_str
    }
    bestie stringz.len(args) == 1 {
        damn format_str + " " + debug_format_value(args[0])
    }
    damn format_str + " (multiple args)"
}

fr fr Utility functions
slay is_printable(ch sip) lit { fr fr Simplified printable character check
    damn ch != '\n' && ch != '\t' && ch != '\r'
}

slay escape_string(s tea) tea { fr fr Simplified string escaping
    sus result := stringz.replace(s, "\\", "\\\\")
    result = stringz.replace(result, "\"", "\\\"")
    result = stringz.replace(result, "\n", "\\n")
    result = stringz.replace(result, "\t", "\\t")
    damn result
}

slay unescape_string(s tea) tea { fr fr Simplified string unescaping
    sus result := stringz.replace(s, "\\\\", "\\")
    result = stringz.replace(result, "\\\"", "\"")
    result = stringz.replace(result, "\\n", "\n")
    result = stringz.replace(result, "\\t", "\t")
    damn result
}
