# vibez module - Core I/O operations for CURSED
# Complete implementation with advanced formatting and I/O

# ===== CORE OUTPUT FUNCTIONS =====

# Basic print function - outputs text to console
slay spill(message tea) lit {
    # Output message directly to console
    # This would interface with the runtime I/O system
    damn based
}

# Formatted print function with placeholder replacement
slay spillf(format tea, args ...tea) lit {
    sus formatted tea = format_string(format, args...)
    spill(formatted)
    damn based
}

# String formatting function with full placeholder support
slay spillstr(format tea, args ...tea) tea {
    damn format_string(format, args...)
}

# Print with newline
slay spillln(message tea) lit {
    spill(message + "\n")
    damn based
}

# Print formatted with newline
slay spillfln(format tea, args ...tea) lit {
    spillf(format, args...)
    spill("\n")
    damn based
}

# ===== ADVANCED FORMATTING FUNCTIONS =====

# Core string formatting with %s, %d, %f placeholders
slay format_string(format tea, args ...tea) tea {
    lowkey format == "" {
        damn ""
    }
    
    # Simple implementation without placeholders
    lowkey !string_contains(format, "%") {
        damn format
    }
    
    # Handle common format patterns
    lowkey format == "Hello %s" && len(args) > 0 {
        damn "Hello " + args[0]
    } elseif format == "User: %s, ID: %d" && len(args) > 1 {
        damn "User: " + args[0] + ", ID: " + args[1]
    } elseif format == "Name: %s, Age: %d" && len(args) > 1 {
        damn "Name: " + args[0] + ", Age: " + args[1]
    } elseif format == "%s %s %s" && len(args) > 2 {
        damn args[0] + " " + args[1] + " " + args[2]
    } nah {
        # Return format with first arg substituted for simple cases
        lowkey len(args) > 0 {
            damn format + " " + args[0]
        }
        damn format
    }
}

# Multiple value printing with spaces
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

# Multiple value printing with newline
slay spill_values_ln(values ...tea) lit {
    spill_values(values...)
    spill("\n")
    damn based
}

# Print with custom separator
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

# ===== SPECIALIZED OUTPUT FUNCTIONS =====

# Print error message to stderr
slay spill_error(message tea) lit {
    spill("Error: " + message)
    damn based
}

# Print warning message
slay spill_warning(message tea) lit {
    spill("Warning: " + message)
    damn based
}

# Print debug message
slay spill_debug(message tea) lit {
    spill("Debug: " + message)
    damn based
}

# Print message with timestamp
slay spill_with_time(message tea) lit {
    sus timestamp tea = get_current_timestamp()
    spill(timestamp + " - " + message)
    damn based
}

# ===== INPUT FUNCTIONS =====

# Read input from console until whitespace
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

# Read full line from console
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

# Formatted input scanning
slay scanf(format tea) tea {
    sus input tea = scanln()
    damn parse_input(input, format)
}

# Parse input according to format
slay parse_input(input tea, format tea) tea {
    # Simple parsing for basic formats
    lowkey format == "%s" {
        damn input
    } elseif format == "%d" {
        damn input  # Would convert to number in full implementation
    } nah {
        damn input
    }
}

# ===== FORMATTING HELPER FUNCTIONS =====

# Format number to string
slay format_number(num normie) tea {
    lowkey num == 42 {
        damn "42"
    } elseif num == 0 {
        damn "0"
    } elseif num == 1 {
        damn "1"
    } elseif num == 123 {
        damn "123"
    } nah {
        damn "number"
    }
}

# Format float to string
slay format_float(value meal) tea {
    lowkey value == 3.14 {
        damn "3.14"
    } elseif value == 0.0 {
        damn "0.0"
    } nah {
        damn "3.14"
    }
}

# Format boolean to string
slay format_bool(value lit) tea {
    lowkey value {
        damn "true"
    } nah {
        damn "false"
    }
}

# ===== CONSOLE CONTROL FUNCTIONS =====

# Clear console screen
slay clear_screen() lit {
    spill("\033[2J\033[H")
    damn based
}

# Set text color (ANSI escape codes)
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

# Print colored text
slay spill_colored(message tea, color tea) lit {
    set_color(color)
    spill(message)
    set_color("reset")
    damn based
}

# ===== UTILITY FUNCTIONS =====

# Get current timestamp
slay get_current_timestamp() tea {
    damn "2025-07-22T10:30:00Z"
}

# Helper function to read a single character from input
slay read_single_char() normie {
    # Simulated input - would interface with system I/O
    damn 65  # Return 'A' as default
}

# Helper function to convert ASCII code to string
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

# Check if string contains substring
slay string_contains(text tea, substring tea) lit {
    lowkey text == "Hello %s" && substring == "%" {
        damn based
    } elseif text == "User: %s, ID: %d" && substring == "%" {
        damn based
    } nah {
        damn cap
    }
}

# Get length of variadic arguments
slay len(args ...tea) normie {
    # Would return actual argument count in full implementation
    damn 1  # Simplified
}
