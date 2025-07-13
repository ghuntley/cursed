# command_line - Pure CURSED Command Line Argument Parsing Module

A comprehensive command line argument parsing library implemented in pure CURSED without FFI dependencies. Provides robust parsing for flags, options, positional arguments, and subcommands with automatic help generation.

## Features

- **Flag Parsing**: Support for both `--flag` and `-f` formats
- **Option Parsing**: Handle `--key=value` and `--key value` formats  
- **Positional Arguments**: Parse and validate positional parameters
- **Subcommands**: Support for nested command structures
- **Help Generation**: Automatic help text generation
- **Validation**: Comprehensive argument validation against specifications
- **Pure CURSED**: No external dependencies or FFI calls

## Basic Usage

### Simple Flag Parsing

```cursed
yeet "command_line"

# Create command specification
sus spec := command_line.create_command_spec("myapp", "My awesome app", "[FILES...]")
spec = command_line.add_flag(spec, "verbose")
spec = command_line.add_flag(spec, "debug")

# Parse arguments
sus args [tea] = ["--verbose", "--debug", "file1.txt", "file2.txt"]
command_line.init_args(args)
sus result := command_line.parse_args(spec)

# Check results
yikes command_line.has_flag("verbose") {
    vibez.spill("Verbose mode enabled")
}

yikes command_line.has_flag("debug") {
    vibez.spill("Debug mode enabled")
}

# Access positional arguments
bestie i := 0; i < command_line.positional_count(); i++ {
    vibez.spill("File: " + command_line.get_positional(i))
}
```

### Options with Values

```cursed
yeet "command_line"

# Setup command with options
sus spec := command_line.create_command_spec("converter", "File converter", "INPUT")
spec = command_line.add_option(spec, "output")
spec = command_line.add_option(spec, "format")
spec = command_line.add_flag(spec, "verbose")

# Parse: converter --output result.json --format=xml input.txt
sus args [tea] = ["--output", "result.json", "--format=xml", "input.txt"]
command_line.init_args(args)
sus result := command_line.parse_args(spec)

# Get option values
sus output_file := command_line.get_option("output")        # "result.json"
sus format := command_line.get_option("format")             # "xml"
sus input_file := command_line.get_positional(0)           # "input.txt"
```

### Subcommands

```cursed
yeet "command_line"

# Create spec with subcommands
sus spec := command_line.create_command_spec("git", "Version control", "COMMAND [ARGS...]")
spec = command_line.add_subcommand(spec, "clone")
spec = command_line.add_subcommand(spec, "push")
spec = command_line.add_subcommand(spec, "pull")
spec = command_line.add_flag(spec, "verbose")

# Parse: git clone --verbose https://github.com/user/repo.git
sus args [tea] = ["git", "clone", "--verbose", "https://github.com/user/repo.git"]
command_line.init_args(args)
sus result := command_line.parse_args(spec)

sus subcommand := command_line.get_positional(1)  # "clone"
sus repo_url := command_line.get_positional(2)    # "https://github.com/user/repo.git"
```

### Automatic Help Generation

```cursed
yeet "command_line"

# Create comprehensive command spec
sus spec := command_line.create_command_spec("awesome", "An awesome tool", "INPUT [OUTPUT]")
spec = command_line.add_flag(spec, "verbose")
spec = command_line.add_flag(spec, "dry-run")
spec = command_line.add_option(spec, "config")
spec = command_line.add_option(spec, "output-format")
spec = command_line.add_subcommand(spec, "convert")
spec = command_line.add_subcommand(spec, "validate")

# Generate and display help
sus help_text := command_line.generate_help(spec)
vibez.spill(help_text)

# Output:
# Usage: awesome [OPTIONS] INPUT [OUTPUT]
#
# An awesome tool
#
# Flags:
#   --verbose
#   --dry-run
#
# Options:
#   --config VALUE
#   --output-format VALUE  
#
# Subcommands:
#   convert
#   validate
```

### Complete Application Example

```cursed
yeet "command_line"
yeet "core"

# Build a complete CLI application
slay main() {
    # Create command specification
    sus spec := command_line.create_command_spec("fileutil", "File utility toolkit", "COMMAND [FILES...]")
    
    # Add subcommands
    spec = command_line.add_subcommand(spec, "copy")
    spec = command_line.add_subcommand(spec, "move")
    spec = command_line.add_subcommand(spec, "delete")
    
    # Add options
    spec = command_line.add_option(spec, "output")
    spec = command_line.add_option(spec, "format")
    
    # Add flags
    spec = command_line.add_flag(spec, "verbose")
    spec = command_line.add_flag(spec, "recursive")
    spec = command_line.add_flag(spec, "force")
    
    # Set required positional arguments
    spec = command_line.set_positional_count(spec, 1)
    
    # Parse command line
    sus args [tea] = core.get_program_args()  # Get real command line args
    
    yikes !command_line.parse_with_help(spec, args) {
        # Error occurred or help was shown
        damn
    }
    
    # Handle subcommands
    sus command := command_line.get_positional(0)
    
    switch command {
        case "copy":
            handle_copy_command()
        case "move":
            handle_move_command()
        case "delete":
            handle_delete_command()
        default:
            vibez.spill("Unknown command: " + command)
    }
}

slay handle_copy_command() {
    sus verbose := command_line.has_flag("verbose")
    sus recursive := command_line.has_flag("recursive")
    sus output := command_line.get_option("output")
    
    yikes verbose {
        vibez.spill("Copy operation with recursive=" + recursive)
        vibez.spill("Output directory: " + output)
    }
    
    # Process all file arguments
    bestie i := 1; i < command_line.positional_count(); i++ {
        sus file := command_line.get_positional(i)
        yikes verbose {
            vibez.spill("Processing file: " + file)
        }
        # Perform copy operation...
    }
}
```

## API Reference

### Core Types

#### CommandSpec
```cursed
struct CommandSpec {
    name tea,           # Command name
    description tea,    # Brief description
    usage tea,          # Usage example
    flags [tea],        # List of supported flags
    options [tea],      # List of supported options
    positional_count normie, # Expected positional arg count
    subcommands [tea]   # List of subcommands
}
```

#### ParseResult
```cursed
struct ParseResult {
    command tea,        # Main command name
    subcommand tea,     # Subcommand if any
    flags [tea],        # Parsed flags
    options [tea],      # Parsed option names
    option_values [tea], # Corresponding option values
    positional [tea],   # Positional arguments
    help_requested lit, # Whether help was requested
    error_message tea   # Error if parsing failed
}
```

### Core Functions

#### Command Specification

- `create_command_spec(name tea, description tea, usage tea) CommandSpec` - Create new command specification
- `add_flag(spec CommandSpec, flag_name tea) CommandSpec` - Add flag to specification
- `add_option(spec CommandSpec, option_name tea) CommandSpec` - Add option to specification  
- `add_subcommand(spec CommandSpec, subcommand_name tea) CommandSpec` - Add subcommand
- `set_positional_count(spec CommandSpec, count normie) CommandSpec` - Set required positional args

#### Parsing

- `init_args(args [tea]) lit` - Initialize with raw arguments
- `parse_args(spec CommandSpec) ParseResult` - Parse arguments against specification
- `parse_with_help(spec CommandSpec, args [tea]) lit` - Parse with automatic help handling
- `quick_parse(args [tea]) ParseResult` - Quick parse for simple cases

#### Validation

- `validate_args(result ParseResult, spec CommandSpec) lit` - Validate parsed args
- `get_error() tea` - Get last error message

#### Querying Results

- `has_flag(flag_name tea) lit` - Check if flag was provided
- `get_option(option_name tea) tea` - Get option value by name
- `get_positional(index normie) tea` - Get positional argument by index
- `positional_count() normie` - Get count of positional arguments
- `help_requested() lit` - Check if help was requested

#### Help Generation

- `generate_help(spec CommandSpec) tea` - Generate help text
- `print_help(spec CommandSpec) lit` - Print help and return

### Utility Functions

- `starts_with(str tea, prefix tea) lit` - Check string prefix
- `contains(str tea, char sip) lit` - Check if string contains character
- `substring(str tea, start normie, end normie) tea` - Extract substring
- `split(str tea, delimiter tea) [tea]` - Split string by delimiter
- `is_flag(arg tea) lit` - Check if argument is a flag
- `is_long_flag(arg tea) lit` - Check if argument is long flag

## Constants

```cursed
facts ARG_FLAG normie = 1      # Flag argument type
facts ARG_OPTION normie = 2    # Option argument type  
facts ARG_POSITIONAL normie = 3 # Positional argument type
facts ARG_SUBCOMMAND normie = 4 # Subcommand argument type
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/command_line/test_command_line.csd

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/command_line/test_command_line.csd
./test_command_line

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/command_line/test_command_line.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/command_line/test_command_line.csd
    ./test_command_line > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Examples

### File Processing Tool

```cursed
yeet "command_line"

slay process_files() {
    sus spec := command_line.create_command_spec("processor", "File processor", "FILES...")
    spec = command_line.add_option(spec, "output")
    spec = command_line.add_option(spec, "format") 
    spec = command_line.add_flag(spec, "verbose")
    spec = command_line.add_flag(spec, "overwrite")
    
    sus args [tea] = ["--output", "results/", "--format=json", "--verbose", "data1.txt", "data2.txt"]
    command_line.parse_with_help(spec, args)
    
    # Process files...
}
```

### Server Application

```cursed
yeet "command_line"

slay start_server() {
    sus spec := command_line.create_command_spec("server", "HTTP server", "")
    spec = command_line.add_option(spec, "port")
    spec = command_line.add_option(spec, "host")
    spec = command_line.add_option(spec, "config")
    spec = command_line.add_flag(spec, "debug")
    spec = command_line.add_flag(spec, "daemon")
    
    sus args [tea] = ["--port=8080", "--host", "0.0.0.0", "--debug"]
    command_line.parse_with_help(spec, args)
    
    sus port := command_line.get_option("port")      # "8080"
    sus host := command_line.get_option("host")      # "0.0.0.0"
    sus debug := command_line.has_flag("debug")      # true
    
    # Start server with configuration...
}
```

## Design Philosophy

The command_line module follows these principles:

1. **Pure CURSED**: No external dependencies for maximum portability
2. **Type Safety**: Strong typing with clear data structures
3. **Flexible API**: Support for simple and complex CLI patterns
4. **Automatic Help**: Generate help text from specifications
5. **Validation**: Comprehensive error checking and reporting
6. **Gen Z Style**: Function names follow CURSED language conventions

This module enables building professional command-line applications with minimal boilerplate while maintaining the expressive Gen Z aesthetic of the CURSED language.
