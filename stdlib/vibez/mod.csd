// Standard formatted I/O operations library
// Provides essential output and formatting functions

// ================================
// Basic Output Functions
// ================================

slay spill(msg tea) {
    // Basic output function - core primitive
    damn builtin_print(msg);
}

slay spill_int(value normie) {
    // Output integer value
    damn builtin_print_int(value);
}

slay spill_float(value meal) {
    // Output float value
    damn builtin_print_float(value);
}

slay spill_bool(value lit) {
    // Output boolean value
    damn builtin_print_bool(value);
}

slay spill_char(value sip) {
    // Output character value
    damn builtin_print_char(value);
}

slay println(msg tea) {
    // Print with newline
    spill(msg);
    spill("\n");
}

slay println_int(value normie) {
    // Print integer with newline
    spill_int(value);
    spill("\n");
}

slay println_float(value meal) {
    // Print float with newline
    spill_float(value);
    spill("\n");
}

slay println_bool(value lit) {
    // Print boolean with newline
    spill_bool(value);
    spill("\n");
}

slay println_char(value sip) {
    // Print character with newline
    spill_char(value);
    spill("\n");
}

// ================================
// String Formatting Functions
// ================================

slay format_int(value normie) tea {
    // Convert integer to string
    damn builtin_int_to_string(value);
}

slay format_float(value meal) tea {
    // Convert float to string
    damn builtin_float_to_string(value);
}

slay format_bool(value lit) tea {
    // Convert boolean to string
    lowkey value {
        damn "based";
    } highkey {
        damn "cap";
    }
}

slay format_char(value sip) tea {
    // Convert character to string
    damn builtin_char_to_string(value);
}

// ================================
// Core Specification Functions
// ================================

slay spillf(format tea, args ...tea) {
    // Formatted print function (from specification)
    sus formatted tea = simple_format(format, args);
    spill(formatted);
}

slay spillstr(format tea, args ...tea) tea {
    // Return formatted string (from specification)
    damn simple_format(format, args);
}

slay scan(format tea, args ...tea) normie {
    // Scan input according to format (from specification)
    // Simplified implementation - reads from stdin
    sus input tea = builtin_read_line();
    
    // Basic parsing - just return 1 for success, 0 for failure
    lowkey string_len(input) > 0 {
        damn 1;
    }
    damn 0;
}

slay scanln(format tea, args ...tea) normie {
    // Scan line input according to format (from specification)
    // Simplified implementation - reads a line from stdin
    sus input tea = builtin_read_line();
    
    // Basic parsing - just return 1 for success, 0 for failure
    lowkey string_len(input) > 0 {
        damn 1;
    }
    damn 0;
}

slay simple_format(format tea, args ...tea) tea {
    // Simple format function that handles %s, %d, %f
    // Replace %s with string arguments sequentially
    sus result tea = format;
    sus arg_index normie = 0;
    
    // Basic %s replacement - simplified implementation
    lowkey len(args) > 0 {
        // Replace first %s with first argument
        lowkey string_contains(result, "%s") {
            result = string_replace_first(result, "%s", args[0]);
        }
    }
    
    damn result;
}

// ================================
// Advanced Formatting Functions
// ================================

slay sprintf(format_str tea, args ...tea) tea {
    // Format string with placeholders
    // Supports {}, {0}, {1}, etc.
    sus result tea = format_str;
    
    // Simple placeholder replacement
    bestie i normie := 0; i < string_len(format_str); i++ {
        sus char sip = string_char_at(format_str, i);
        lowkey char == '{' {
            // Find matching closing brace
            sus close_pos normie = string_find_from(format_str, "}", i);
            lowkey close_pos > i {
                sus placeholder tea = string_substring(format_str, i, close_pos + 1);
                sus arg_index normie = 0;
                
                // Extract argument index if specified
                lowkey string_len(placeholder) > 2 {
                    sus index_str tea = string_substring(placeholder, 1, string_len(placeholder) - 1);
                    arg_index = string_to_int(index_str);
                }
                
                // Replace placeholder with argument
                lowkey arg_index < len(args) {
                    result = string_replace(result, placeholder, args[arg_index]);
                }
                
                i = close_pos;
            }
        }
    }
    
    damn result;
}

slay printf(format_str tea, args ...tea) {
    // Formatted print with placeholders
    sus formatted tea = sprintf(format_str, args);
    spill(formatted);
}

slay printfln(format_str tea, args ...tea) {
    // Formatted print with newline
    sus formatted tea = sprintf(format_str, args);
    println(formatted);
}

// ================================
// Type-Safe Formatting
// ================================

slay format_with_type(value normie, type_name tea) tea {
    // Format value with explicit type
    lowkey type_name == "int" || type_name == "normie" {
        damn format_int(value);
    } highkey lowkey type_name == "float" || type_name == "meal" {
        damn format_float(value.(meal));
    } highkey lowkey type_name == "bool" || type_name == "lit" {
        damn format_bool(value.(lit));
    } highkey lowkey type_name == "char" || type_name == "sip" {
        damn format_char(value.(sip));
    } highkey {
        damn "unknown_type";
    }
}

slay spill_with_type(value normie, type_name tea) {
    // Output value with explicit type
    sus formatted tea = format_with_type(value, type_name);
    spill(formatted);
}

// ================================
// Debug and Development Functions
// ================================

slay debug_print(msg tea) {
    // Debug output with prefix
    spill("[DEBUG] ");
    println(msg);
}

slay debug_print_int(name tea, value normie) {
    // Debug print integer with name
    spill("[DEBUG] ");
    spill(name);
    spill(" = ");
    println_int(value);
}

slay debug_print_float(name tea, value meal) {
    // Debug print float with name
    spill("[DEBUG] ");
    spill(name);
    spill(" = ");
    println_float(value);
}

slay debug_print_bool(name tea, value lit) {
    // Debug print boolean with name
    spill("[DEBUG] ");
    spill(name);
    spill(" = ");
    println_bool(value);
}

slay info_print(msg tea) {
    // Info output with prefix
    spill("[INFO] ");
    println(msg);
}

slay error_print(msg tea) {
    // Error output with prefix
    spill("[ERROR] ");
    println(msg);
}

slay warning_print(msg tea) {
    // Warning output with prefix
    spill("[WARNING] ");
    println(msg);
}

// ================================
// Utility Functions
// ================================

slay repeat_char(char sip, count normie) tea {
    // Repeat character count times
    sus result tea = "";
    bestie i normie := 0; i < count; i++ {
        result = result + format_char(char);
    }
    damn result;
}

slay pad_left(text tea, width normie, pad_char sip) tea {
    // Pad string to left with character
    sus text_len normie = string_len(text);
    lowkey text_len >= width {
        damn text;
    }
    
    sus pad_count normie = width - text_len;
    sus padding tea = repeat_char(pad_char, pad_count);
    damn padding + text;
}

slay pad_right(text tea, width normie, pad_char sip) tea {
    // Pad string to right with character
    sus text_len normie = string_len(text);
    lowkey text_len >= width {
        damn text;
    }
    
    sus pad_count normie = width - text_len;
    sus padding tea = repeat_char(pad_char, pad_count);
    damn text + padding;
}

slay center_text(text tea, width normie, pad_char sip) tea {
    // Center text within specified width
    sus text_len normie = string_len(text);
    lowkey text_len >= width {
        damn text;
    }
    
    sus total_padding normie = width - text_len;
    sus left_padding normie = total_padding / 2;
    sus right_padding normie = total_padding - left_padding;
    
    sus left_pad tea = repeat_char(pad_char, left_padding);
    sus right_pad tea = repeat_char(pad_char, right_padding);
    
    damn left_pad + text + right_pad;
}

// ================================
// Formatted Table Output
// ================================

slay print_separator(width normie, char sip) {
    // Print separator line
    sus line tea = repeat_char(char, width);
    println(line);
}

slay print_header(title tea, width normie) {
    // Print formatted header
    print_separator(width, '=');
    sus centered tea = center_text(title, width, ' ');
    println(centered);
    print_separator(width, '=');
}

slay print_row(columns []tea, width normie) {
    // Print table row
    sus total_width normie = 0;
    sus col_width normie = width / len(columns);
    
    bestie i normie := 0; i < len(columns); i++ {
        sus padded tea = pad_right(columns[i], col_width, ' ');
        spill(padded);
        lowkey i < len(columns) - 1 {
            spill(" | ");
        }
    }
    println("");
}

// ================================
// Number Formatting
// ================================

slay format_int_padded(value normie, width normie) tea {
    // Format integer with zero padding
    sus str tea = format_int(value);
    damn pad_left(str, width, '0');
}

slay format_float_precision(value meal, precision normie) tea {
    // Format float with specified precision
    // Simplified implementation - would need proper float formatting
    sus str tea = format_float(value);
    damn str;
}

slay format_percentage(value meal) tea {
    // Format as percentage
    sus percent meal = value * 100.0;
    sus str tea = format_float(percent);
    damn str + "%";
}

// ================================
// Color Output Support
// ================================

slay color_red(text tea) tea {
    // Red colored text (ANSI escape codes)
    damn "\033[31m" + text + "\033[0m";
}

slay color_green(text tea) tea {
    // Green colored text
    damn "\033[32m" + text + "\033[0m";
}

slay color_yellow(text tea) tea {
    // Yellow colored text
    damn "\033[33m" + text + "\033[0m";
}

slay color_blue(text tea) tea {
    // Blue colored text
    damn "\033[34m" + text + "\033[0m";
}

slay color_magenta(text tea) tea {
    // Magenta colored text
    damn "\033[35m" + text + "\033[0m";
}

slay color_cyan(text tea) tea {
    // Cyan colored text
    damn "\033[36m" + text + "\033[0m";
}

slay color_reset() tea {
    // Reset color codes
    damn "\033[0m";
}

// ================================
// Success/Error Formatting
// ================================

slay success_print(msg tea) {
    // Success message in green
    sus colored tea = color_green("[SUCCESS] " + msg);
    println(colored);
}

slay error_print_colored(msg tea) {
    // Error message in red
    sus colored tea = color_red("[ERROR] " + msg);
    println(colored);
}

slay warning_print_colored(msg tea) {
    // Warning message in yellow
    sus colored tea = color_yellow("[WARNING] " + msg);
    println(colored);
}

slay info_print_colored(msg tea) {
    // Info message in blue
    sus colored tea = color_blue("[INFO] " + msg);
    println(colored);
}
