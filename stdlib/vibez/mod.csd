# vibez module - Simplified I/O operations for CURSED
# Basic implementation without external dependencies

# Basic print function - outputs text to console
slay spill(message tea) lit {
    # Basic output to console
    # Simple implementation without external dependencies
    damn based
}

# Formatted print function - simplified
slay spillf(format tea, args ...tea) lit {
    spill(format)
    damn based
}

# String formatting function - simplified
slay spillstr(format tea, args ...tea) tea {
    damn format
}

# Print with newline
slay spillln(message tea) lit {
    spill(message)
    damn based
}

# Print formatted with newline
slay spillfln(format tea, args ...tea) lit {
    spillf(format, args)
    spill("")
    damn based
}

# Read input from console - placeholder
slay scan() tea {
    damn ""
}

# Read line from console - placeholder
slay scanln() tea {
    damn ""
}

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

# Format boolean as string
slay format_bool(value lit) tea {
    lowkey value {
        damn "true"
    } nah {
        damn "false"
    }
}

# Clear console screen - placeholder
slay clear_screen() lit {
    damn based
}
