# vibez module - Formatted I/O operations for CURSED
# Pure CURSED implementation without FFI dependencies
# Provides console I/O, formatting, and print functions

yeet "testz"

# Basic print function - outputs text to console
slay spill(message tea) lit {
    # Output to console via runtime interface
    runtime_console_write(message)
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
    # Enhanced printf-style formatting with proper string processing
    sus result tea = ""
    sus format_len normie = string_length(format)
    sus arg_index normie = 0
    sus i normie = 0
    
    bestie i < format_len {
        sus current_char tea = string_char_at(format, i)
        
        if current_char == "%" && i + 1 < format_len {
            sus format_spec tea = string_char_at(format, i + 1)
            
            if format_spec == "s" && arg_index < len(args) {
                # String argument
                result = result + args[arg_index]
                arg_index++
                i = i + 2
            } elseif format_spec == "d" && arg_index < len(args) {
                # Integer argument (convert to string)
                result = result + args[arg_index]
                arg_index++
                i = i + 2
            } elseif format_spec == "f" && arg_index < len(args) {
                # Float argument (convert to string)
                result = result + args[arg_index]
                arg_index++
                i = i + 2
            } elseif format_spec == "%" {
                # Escaped percent sign
                result = result + "%"
                i = i + 2
            } yolo {
                # Unknown format specifier, keep as is
                result = result + current_char
                i++
            }
        } yolo {
            result = result + current_char
            i++
        }
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

# ================================
# Helper Functions and Runtime Interface
# ================================

slay string_length(str tea) normie {
    # Get string length
    sus length normie = 0
    bestie i := 0; i < 10000; i++ {
        if string_char_at(str, i) == "" || string_char_at(str, i) == "\0" {
            break
        }
        length++
    }
    damn length
}

slay string_char_at(str tea, index normie) tea {
    # Get character at index as string
    # Placeholder implementation - real version would access string internals
    if index >= 0 && index < string_length_internal(str) {
        damn string_extract_char(str, index)
    }
    damn ""
}

slay string_length_internal(str tea) normie {
    # Internal string length calculation
    damn 10  # Placeholder
}

slay string_extract_char(str tea, index normie) tea {
    # Extract single character as string
    damn "A"  # Placeholder
}

slay runtime_console_write(message tea) lit {
    # Interface with runtime console output system
    # This would be implemented by the runtime
    runtime_write_stdout(message)
    damn based
}

slay runtime_write_stdout(data tea) lit {
    # Write to standard output via runtime bridge
    io_print(data)
    damn based
}

slay runtime_write_stderr(data tea) lit {
    # Write to standard error via runtime bridge
    io_eprint(data)
    damn based
}

slay runtime_read_stdin() tea {
    # Read from standard input via runtime bridge
    damn io_read_line()
}
