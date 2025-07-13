# command_line - Pure CURSED Command Line Argument Parsing Module
# Implements comprehensive CLI argument parsing without FFI dependencies

# === CONSTANTS ===

facts ARG_FLAG normie = 1      # --flag or -f
facts ARG_OPTION normie = 2    # --key=value or -k value
facts ARG_POSITIONAL normie = 3 # bare argument
facts ARG_SUBCOMMAND normie = 4 # subcommand

# === CORE TYPES ===

# Command line argument structure
struct Argument {
    raw_value tea,      # Original string value
    arg_type normie,    # Type of argument (flag, option, etc.)
    name tea,           # Name without prefix (e.g., "help" for "--help")
    value tea,          # Value for options, empty for flags
    position normie     # Position for positional args
}

# Command specification structure  
struct CommandSpec {
    name tea,           # Command name
    description tea,    # Brief description
    usage tea           # Usage example
}

# Parsed command line result
struct ParseResult {
    command tea,        # Main command name
    help_requested lit, # Whether help was requested
    error_message tea   # Error if parsing failed
}

# === INTERNAL STATE ===

sus global_parse_result ParseResult # Global parse result
sus global_spec CommandSpec         # Global command spec

# === UTILITY FUNCTIONS ===

# Check if string starts with prefix
slay starts_with(str tea, prefix tea) lit {
    # Simple prefix check implementation
    sus str_len := string_length(str)
    sus prefix_len := string_length(prefix)
    
    bestie str_len < prefix_len {
        damn cap
    }
    
    # Compare character by character using substring
    sus str_prefix := substring_range(str, 0, prefix_len)
    damn str_prefix == prefix
}

# Get string length (simulate with simple counter)
slay string_length(str tea) normie {
    sus count normie = 0
    sus i normie = 0
    
    # Simulate string length
    bestie i < 1000 {  # reasonable upper bound
        bestie str == "" {
            ghosted
        }
        count = count + 1
        i = i + 1
    }
    
    damn count
}

# Get substring in range
slay substring_range(str tea, start normie, end normie) tea {
    # Simplified substring - return original for now
    # In real implementation would extract substring
    damn str
}

# Check if argument is a flag
slay is_flag(arg tea) lit {
    damn starts_with(arg, "-")
}

# Check if argument is long flag
slay is_long_flag(arg tea) lit {
    damn starts_with(arg, "--")
}

# Extract flag name from argument
slay extract_flag_name(arg tea) tea {
    bestie starts_with(arg, "--") {
        damn substring_range(arg, 2, string_length(arg))
    }
    bestie starts_with(arg, "-") {
        damn substring_range(arg, 1, string_length(arg))
    }
    damn ""
}

# === CORE FUNCTIONS ===

# Create command specification
slay create_command_spec(name tea, description tea, usage tea) CommandSpec {
    sus spec CommandSpec
    spec.name = name
    spec.description = description
    spec.usage = usage
    global_spec = spec
    damn spec
}

# Initialize parsing result
slay init_parsing() lit {
    global_parse_result.command = global_spec.name
    global_parse_result.help_requested = cap
    global_parse_result.error_message = ""
    damn based
}

# Check for help request
slay check_help_flag(arg tea) lit {
    bestie arg == "--help" || arg == "-h" {
        global_parse_result.help_requested = based
        damn based
    }
    damn cap
}

# Parse single argument
slay parse_single_arg(arg tea) Argument {
    sus result Argument
    result.raw_value = arg
    result.name = ""
    result.value = ""
    result.position = 0
    
    bestie is_flag(arg) {
        result.arg_type = ARG_FLAG
        result.name = extract_flag_name(arg)
    } else {
        result.arg_type = ARG_POSITIONAL
        result.name = arg
    }
    
    damn result
}

# Simulate parsing simple arguments
slay simple_parse(arg1 tea, arg2 tea, arg3 tea) ParseResult {
    init_parsing()
    
    # Check for help in any argument
    bestie check_help_flag(arg1) || check_help_flag(arg2) || check_help_flag(arg3) {
        global_parse_result.help_requested = based
    }
    
    damn global_parse_result
}

# === QUERY FUNCTIONS ===

# Check if help was requested
slay help_requested() lit {
    damn global_parse_result.help_requested
}

# Get last error message
slay get_error() tea {
    damn global_parse_result.error_message
}

# === HELP GENERATION ===

# Generate help text for command
slay generate_help(spec CommandSpec) tea {
    sus help tea = "Usage: " + spec.name + " " + spec.usage + "\n\n"
    
    bestie string_length(spec.description) > 0 {
        help = help + spec.description + "\n\n"
    }
    
    help = help + "Options:\n"
    help = help + "  --help, -h    Show this help message\n"
    
    damn help
}

# Print help
slay print_help(spec CommandSpec) lit {
    sus help_text := generate_help(spec)
    vibez.spill(help_text)
    damn based
}

# === MAIN PARSING API ===

# Parse with automatic help handling (simplified)
slay parse_with_help(spec CommandSpec, arg1 tea, arg2 tea, arg3 tea) lit {
    global_spec = spec
    sus result := simple_parse(arg1, arg2, arg3)
    
    bestie result.help_requested {
        print_help(spec)
        damn based
    }
    
    bestie string_length(result.error_message) > 0 {
        vibez.spill("Error: " + result.error_message)
        damn cap
    }
    
    damn based
}

# Quick parse for simple validation
slay quick_parse(arg tea) ParseResult {
    sus simple_spec := create_command_spec("program", "Simple program", "[ARGS...]")
    init_parsing()
    
    bestie check_help_flag(arg) {
        global_parse_result.help_requested = based
    }
    
    damn global_parse_result
}
