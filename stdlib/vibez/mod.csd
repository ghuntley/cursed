# vibez module - Formatted I/O operations for CURSED
# Pure CURSED implementation without FFI dependencies
# Provides console I/O, formatting, and print functions

yeet "core"
yeet "stringz"

# Basic print function - outputs text to console
slay spill(message tea) lit {
    # Basic output to console
    # Uses core runtime for actual output
    core.print(message)
    damn based
}

# Formatted print function - printf-style formatting
slay spillf(format tea, args ...tea) lit {
    sus formatted_output tea = format_string(format, args)
    spill(formatted_output)
    damn based
}

# String formatting function - returns formatted string
slay spillstr(format tea, args ...tea) tea {
    damn format_string(format, args)
}

# Format string with arguments
slay format_string(format tea, args ...tea) tea {
    sus result tea = format
    sus arg_index normie = 0
    
    # Simple format string processing
    # Replace %s with string arguments, %d with numbers
    sus i normie = 0
    stan i < stringz.length(format) {
        sus char sip = stringz.char_at(format, i)
        
        fr fr char == '%' && i + 1 < stringz.length(format) {
            sus next_char sip = stringz.char_at(format, i + 1)
            
            fr fr next_char == 's' && arg_index < args.length {
                # Replace %s with string argument
                sus before tea = stringz.substring(result, 0, i)
                sus after tea = stringz.substring(result, i + 2, stringz.length(result))
                result = stringz.concat(stringz.concat(before, args[arg_index]), after)
                arg_index++
                i++
            } else fr fr next_char == 'd' && arg_index < args.length {
                # Replace %d with number argument
                sus before tea = stringz.substring(result, 0, i)
                sus after tea = stringz.substring(result, i + 2, stringz.length(result))
                result = stringz.concat(stringz.concat(before, args[arg_index]), after)
                arg_index++
                i++
            }
        }
        i++
    }
    
    damn result
}

# Print with newline
slay spillln(message tea) lit {
    spill(stringz.concat(message, "\n"))
    damn based
}

# Print formatted with newline
slay spillfln(format tea, args ...tea) lit {
    spillf(format, args)
    spill("\n")
    damn based
}

# Read input from console
slay scan() tea {
    # Basic input reading - implementation depends on runtime
    damn core.read_line()
}

# Read line from console
slay scanln() tea {
    sus input tea = scan()
    damn stringz.trim(input)
}

# Read formatted input
slay scanf(format tea) tea {
    sus input tea = scanln()
    damn parse_input(input, format)
}

# Parse input according to format
slay parse_input(input tea, format tea) tea {
    # Simple input parsing - just return input for now
    # Could be extended for more complex parsing
    damn input
}

# Print multiple values separated by spaces
slay spill_values(values ...tea) lit {
    sus i normie = 0
    stan i < values.length {
        spill(values[i])
        fr fr i < values.length - 1 {
            spill(" ")
        }
        i++
    }
    damn based
}

# Print multiple values with newline
slay spill_values_ln(values ...tea) lit {
    spill_values(values)
    spill("\n")
    damn based
}

# Print with separator
slay spill_sep(separator tea, values ...tea) lit {
    sus i normie = 0
    stan i < values.length {
        spill(values[i])
        fr fr i < values.length - 1 {
            spill(separator)
        }
        i++
    }
    damn based
}

# Print error message to stderr
slay spill_error(message tea) lit {
    sus error_msg tea = stringz.concat("Error: ", message)
    spill(error_msg)
    damn based
}

# Print warning message
slay spill_warning(message tea) lit {
    sus warning_msg tea = stringz.concat("Warning: ", message)
    spill(warning_msg)
    damn based
}

# Print debug message
slay spill_debug(message tea) lit {
    sus debug_msg tea = stringz.concat("Debug: ", message)
    spill(debug_msg)
    damn based
}

# Print with timestamp
slay spill_with_time(message tea) lit {
    sus timestamp tea = core.get_timestamp()
    sus timed_msg tea = stringz.concat(stringz.concat("[", timestamp), stringz.concat("] ", message))
    spill(timed_msg)
    damn based
}

# Format number as string
slay format_number(number normie) tea {
    damn core.number_to_string(number)
}

# Format float as string
slay format_float(number drip) tea {
    damn core.float_to_string(number)
}

# Format boolean as string
slay format_bool(value lit) tea {
    fr fr value {
        damn "true"
    } else {
        damn "false"
    }
}

# Clear console screen
slay clear_screen() lit {
    spill("\x1b[2J\x1b[H")
    damn based
}

# Set text color (ANSI escape codes)
slay set_color(color tea) lit {
    sus color_code tea = ""
    fr fr color == "red" {
        color_code = "\x1b[31m"
    } else fr fr color == "green" {
        color_code = "\x1b[32m"
    } else fr fr color == "blue" {
        color_code = "\x1b[34m"
    } else fr fr color == "yellow" {
        color_code = "\x1b[33m"
    } else fr fr color == "reset" {
        color_code = "\x1b[0m"
    }
    spill(color_code)
    damn based
}

# Print colored text
slay spill_colored(message tea, color tea) lit {
    set_color(color)
    spill(message)
    set_color("reset")
    damn based
}
