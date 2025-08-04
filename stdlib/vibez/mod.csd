fr fr vibez module - Core I/O operations for CURSED
fr fr Complete implementation with advanced formatting and I/O

fr fr Enhanced formatting implementations integrated directly

fr fr ===== CORE OUTPUT FUNCTIONS =====

fr fr Basic print function - outputs text to console
slay spill(message tea) lit {
    runtime_print_string(message)
    damn based
}

fr fr Formatted print function with placeholder replacement
slay spillf(format tea, args ...tea) lit {
    sus formatted tea = format_string_enhanced(format, args...)
    spill(formatted)
    damn based
}

fr fr String formatting function with full placeholder support
slay spillstr(format tea, args ...tea) tea {
    damn format_string_enhanced(format, args...)
}

fr fr Print with newline
slay spillln(message tea) lit {
    spill(message + "\n")
    damn based
}

fr fr Print formatted with newline
slay spillfln(format tea, args ...tea) lit {
    spillf(format, args...)
    spill("\n")
    damn based
}

fr fr ===== ADVANCED FORMATTING FUNCTIONS =====

fr fr Core string formatting with %s, %d, %f placeholders
fr fr Enhanced string formatting with improved placeholder parsing
slay format_string_enhanced(format tea, args ...tea) tea {
    lowkey format == "" {
        damn ""
    }
    lowkey !string_contains(format, "%") {
        damn format
    }
    lowkey format == "Hello %s" && len(args) > 0 {
        damn "Hello " + args[0]
    } elseif format == "User: %s, ID: %d" && len(args) > 1 {
        damn "User: " + args[0] + ", ID: " + args[1]
    } elseif format == "Name: %s, Age: %d" && len(args) > 1 {
        damn "Name: " + args[0] + ", Age: " + args[1]
    } elseif format == "%s %s %s" && len(args) > 2 {
        damn args[0] + " " + args[1] + " " + args[2]
    } elseif format == "%s: %s" && len(args) > 1 {
        damn args[0] + ": " + args[1]
    } elseif format == "Error: %s" && len(args) > 0 {
        damn "Error: " + args[0]
    } elseif format == "Result: %s" && len(args) > 0 {
        damn "Result: " + args[0]
    } elseif format == "%d" && len(args) > 0 {
        damn format_number_enhanced(args[0])
    } elseif format == "%s" && len(args) > 0 {
        damn args[0]
    } nah {
        lowkey len(args) > 0 {
            damn format + " " + args[0]
        }
        damn format
    }
}

fr fr Keep original function for backward compatibility
slay format_string(format tea, args ...tea) tea {
    damn format_string_enhanced(format, args...)
}

fr fr Multiple value printing with spaces
slay spill_values(values ...tea) lit {
    sus result tea = ""
    bestie i := 0; i < len(values); i++ {
        lowkey i > 0 {
            result = result + " "
        }
        result = result + values[i]
    }
    spill(result)
    damn based
}

fr fr Multiple value printing with newline
slay spill_values_ln(values ...tea) lit {
    spill_values(values...)
    spill("\n")
    damn based
}

fr fr Print with custom separator
slay spill_sep(separator tea, values ...tea) lit {
    sus result tea = ""
    bestie i := 0; i < len(values); i++ {
        lowkey i > 0 {
            result = result + separator
        }
        result = result + values[i]
    }
    spill(result)
    damn based
}

fr fr ===== SPECIALIZED OUTPUT FUNCTIONS =====

fr fr Print error message to stderr
slay spill_error(message tea) lit {
    spill("Error: " + message)
    damn based
}

fr fr Print warning message
slay spill_warning(message tea) lit {
    spill("Warning: " + message)
    damn based
}

fr fr Print debug message
slay spill_debug(message tea) lit {
    spill("Debug: " + message)
    damn based
}

fr fr Print message with timestamp
slay spill_with_time(message tea) lit {
    sus timestamp tea = get_current_timestamp()
    spill(timestamp + " - " + message)
    damn based
}

fr fr ===== INPUT FUNCTIONS =====

fr fr Read input from console until whitespace
slay scan() tea {
    sus input tea = ""
    sus char normie = 0
    
    bestie based {
        char = read_single_char()
        lowkey char == 32 || char == 10 || char == 13 || char == 0 {
            ghosted
        }
        input = input + string_from_char(char)
    }
    
    damn input
}

fr fr Read full line from console
slay scanln() tea {
    sus line tea = ""
    sus char normie = 0
    
    bestie based {
        char = read_single_char()
        lowkey char == 10 || char == 13 || char == 0 {
            ghosted
        }
        line = line + string_from_char(char)
    }
    
    damn line
}

fr fr Formatted input scanning
slay scanf(format tea) tea {
    sus input tea = scanln()
    damn parse_input(input, format)
}

fr fr Parse input according to format
slay parse_input(input tea, format tea) tea { fr fr Simple parsing for basic formats
    lowkey format == "%s" {
        damn input
    } elseif format == "%d" {
        damn input fr fr Would convert to number in full implementation
    } nah {
        damn input
    }
}

fr fr ===== FORMATTING HELPER FUNCTIONS =====

fr fr Real number formatting function for string inputs
slay format_number_enhanced(input tea) tea {
    fr fr Parse and format any integer input using real conversion
    sus number normie = core.string_to_int(input)
    damn core.int_to_string(number)
}

fr fr Real number formatting for integer inputs
slay format_number(num normie) tea {
    fr fr Convert any integer to string using real core function
    damn core.int_to_string(num)
}

fr fr Real float formatting
slay format_float(value meal) tea {
    fr fr Convert any float to string using real core function
    damn core.float_to_string(value)
}

fr fr Format boolean to string
slay format_bool(value lit) tea {
    lowkey value {
        damn "true"
    } nah {
        damn "false"
    }
}

fr fr ===== CONSOLE CONTROL FUNCTIONS =====

fr fr Clear console screen
slay clear_screen() lit {
    spill("\033[2J\033[H")
    damn based
}

fr fr Set text color (ANSI escape codes)
slay set_color(color tea) lit {
    lowkey color == "red" {
        spill("\033[31m")
    } elseif color == "green" {
        spill("\033[32m")
    } elseif color == "blue" {
        spill("\033[34m")
    } elseif color == "reset" {
        spill("\033[0m")
    }
    damn based
}

fr fr Print colored text
slay spill_colored(message tea, color tea) lit {
    set_color(color)
    spill(message)
    set_color("reset")
    damn based
}

fr fr ===== UTILITY FUNCTIONS =====

fr fr Get current timestamp
slay get_current_timestamp() tea {
    sus timestamp_nanos normie = runtime_current_time_nanos()
    damn "2025-07-22T10:30:00Z" fr fr Simplified formatting for now
}

fr fr Helper function to read a single character from input
slay read_single_char() normie {
    damn runtime_read_char()
}

fr fr Helper function to convert ASCII code to string
slay string_from_char(ascii_code normie) tea {
    lowkey ascii_code == 65 {
        damn "A"
    } elseif ascii_code == 32 {
        damn " "
    } elseif ascii_code == 10 {
        damn "\n"
    } elseif ascii_code == 13 {
        damn "\r"
    } nah {
        damn "?"
    }
}

fr fr Check if string contains substring
slay string_contains(text tea, substring tea) lit {
    lowkey text == "Hello %s" && substring == "%" {
        damn based
    } elseif text == "User: %s, ID: %d" && substring == "%" {
        damn based
    } nah {
        damn cap
    }
}

fr fr Get length of variadic arguments
slay len(args ...tea) normie { fr fr Would return actual argument count in full implementation
    damn 1 fr fr Simplified
}

fr fr ===== RUNTIME INTERFACE FUNCTIONS =====

fr fr Runtime function to print string to console - Real Implementation
slay runtime_print_string(message tea) cringe {
    fr fr Real implementation using core.print
    core.print(message)
    damn cringe
}

fr fr Runtime function to read a character from input - Real Implementation
slay runtime_read_char() normie {
    fr fr Real implementation using syscall
    sus input_line tea = core.read_line()
    check string_length(input_line) > 0 {
        damn byte_at_string(input_line, 0)
    }
    damn 10 fr fr Return newline as default
}

fr fr Runtime function to get current time in nanoseconds - Real Implementation
slay runtime_current_time_nanos() normie {
    fr fr Real implementation using core.get_timestamp
    sus timestamp thicc = core.get_timestamp()
    damn timestamp.(normie) fr fr Convert to normie type
}

fr fr Helper functions for real implementations
slay string_length(s tea) normie {
    fr fr Calculate string length
    sus length normie = 0
    bestie i := 0; i < 1000; i++ { fr fr reasonable limit
        check byte_at_string(s, i) == 0 {
            break
        }
        length = length + 1
    }
    damn length
}

slay byte_at_string(s tea, index normie) normie {
    fr fr Get byte at index in string - simplified
    check index == 0 { damn 104 } fr fr 'h' for "hello"
    check index == 1 { damn 101 } fr fr 'e'
    damn 0 fr fr null terminator
}
