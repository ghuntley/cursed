fr fr CURSED VIBEZ Module - I/O Operations
fr fr Core implementation in pure CURSED language
fr fr Provides primary I/O functionality for CURSED programs

yeet "stringz"

fr fr ===== CORE GLOBAL STATE =====

sus io_initialized lit = cap
sus last_io_error tea = ""
sus default_output_stream tea = "stdout"
sus default_input_stream tea = "stdin"

fr fr Runtime bridges - these interface with the underlying system
fr fr In the actual implementation, these would be provided by the runtime
slay runtime_print(message tea) lit
slay runtime_read_line() tea  
slay runtime_read_file(path tea) (tea, tea)
slay runtime_write_file(path tea, content tea) (lit, tea)

fr fr ===== INITIALIZATION =====

slay init() lit {
    ready io_initialized == cap {
        io_initialized = based
        last_io_error = ""
        damn based
    }
    damn based
}

slay is_initialized() lit {
    damn io_initialized
}

fr fr ===== ERROR HANDLING =====

slay get_last_error() tea {
    damn last_io_error
}

slay clear_error() {
    last_io_error = ""
}

slay set_error(error tea) {
    last_io_error = error
}

fr fr ===== PRIMARY OUTPUT FUNCTIONS =====

fr fr spill() - Primary output function for printing text
slay spill(message tea) lit {
    ready is_initialized() == cap {
        init()
    }
    
    ready message == cringe {
        set_error("spill: cannot print null message")
        damn cap
    }
    
    sus success lit = runtime_print(message)
    ready success == cap {
        set_error("spill: failed to print message")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spillln() - Print with automatic newline
slay spillln(message tea) lit {
    ready is_initialized() == cap {
        init()
    }
    
    ready message == cringe {
        set_error("spillln: cannot print null message")
        damn cap
    }
    
    sus message_with_newline tea = message + "\n"
    sus success lit = runtime_print(message_with_newline)
    ready success == cap {
        set_error("spillln: failed to print message with newline")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spill_values() - Print multiple values separated by spaces
slay spill_values(values []tea) lit {
    ready is_initialized() == cap {
        init()
    }
    
    ready values == cringe || len(values) == 0 {
        set_error("spill_values: no values provided")
        damn cap
    }
    
    sus combined tea = ""
    bestie i := 0; i < len(values); i++ {
        ready i > 0 {
            combined = combined + " "
        }
        combined = combined + values[i]
    }
    
    sus success lit = spill(combined)
    ready success == cap {
        set_error("spill_values: failed to print combined values")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spill_sep() - Print values with custom separator
slay spill_sep(separator tea, values []tea) lit {
    ready is_initialized() == cap {
        init()
    }
    
    ready values == cringe || len(values) == 0 {
        set_error("spill_sep: no values provided")
        damn cap
    }
    
    ready separator == cringe {
        separator = " "
    }
    
    sus combined tea = ""
    bestie i := 0; i < len(values); i++ {
        ready i > 0 {
            combined = combined + separator
        }
        combined = combined + values[i]
    }
    
    sus success lit = spill(combined)
    ready success == cap {
        set_error("spill_sep: failed to print separated values")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spillf() - Formatted print (basic implementation)
slay spillf(format tea, args []tea) lit {
    ready is_initialized() == cap {
        init()
    }
    
    ready format == cringe {
        set_error("spillf: format string cannot be null")
        damn cap
    }
    
    sus formatted tea = format_string(format, args)
    sus success lit = spill(formatted)
    ready success == cap {
        set_error("spillf: failed to print formatted string")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spillstr() - Format string without printing
slay spillstr(format tea, args []tea) tea {
    ready format == cringe {
        set_error("spillstr: format string cannot be null")
        damn ""
    }
    
    sus formatted tea = format_string(format, args)
    clear_error()
    damn formatted
}

fr fr ===== SPECIALIZED OUTPUT FUNCTIONS =====

fr fr spill_error() - Print error message with prefix
slay spill_error(message tea) lit {
    ready message == cringe {
        set_error("spill_error: error message cannot be null")
        damn cap
    }
    
    sus error_message tea = "ERROR: " + message
    sus success lit = spill(error_message)
    ready success == cap {
        set_error("spill_error: failed to print error message")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spill_warning() - Print warning message with prefix
slay spill_warning(message tea) lit {
    ready message == cringe {
        set_error("spill_warning: warning message cannot be null")
        damn cap
    }
    
    sus warning_message tea = "WARNING: " + message
    sus success lit = spill(warning_message)
    ready success == cap {
        set_error("spill_warning: failed to print warning message")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr spill_debug() - Print debug message with prefix
slay spill_debug(message tea) lit {
    ready message == cringe {
        set_error("spill_debug: debug message cannot be null")
        damn cap
    }
    
    sus debug_message tea = "DEBUG: " + message
    sus success lit = spill(debug_message)
    ready success == cap {
        set_error("spill_debug: failed to print debug message")
        damn cap
    }
    
    clear_error()
    damn based
}

fr fr ===== INPUT FUNCTIONS =====

fr fr input() - Read user input with optional prompt
slay input(prompt tea) tea {
    ready is_initialized() == cap {
        init()
    }
    
    ready prompt != cringe && stringz.length(prompt) > 0 {
        sus print_success lit = spill(prompt)
        ready print_success == cap {
            set_error("input: failed to print prompt")
            damn ""
        }
    }
    
    sus user_input tea = runtime_read_line()
    ready user_input == cringe {
        set_error("input: failed to read user input")
        damn ""
    }
    
    fr fr Remove trailing newline if present
    sus trimmed_input tea = stringz.trim_whitespace(user_input)
    clear_error()
    damn trimmed_input
}

fr fr read_line() - Read a line from stdin
slay read_line() tea {
    ready is_initialized() == cap {
        init()
    }
    
    sus line tea = runtime_read_line()
    ready line == cringe {
        set_error("read_line: failed to read line from input")
        damn ""
    }
    
    clear_error()
    damn line
}

fr fr ===== FILE I/O FUNCTIONS =====

fr fr read_file() - Read file contents as string
slay read_file(filename tea) (tea, tea) {
    ready is_initialized() == cap {
        init()
    }
    
    ready filename == cringe || stringz.length(filename) == 0 {
        set_error("read_file: filename cannot be empty")
        damn ("", "filename cannot be empty")
    }
    
    sus content tea, error tea = runtime_read_file(filename)
    ready error != cringe && stringz.length(error) > 0 {
        set_error("read_file: " + error)
        damn ("", error)
    }
    
    clear_error()
    damn (content, "")
}

fr fr write_file() - Write content to file
slay write_file(filename tea, content tea) (lit, tea) {
    ready is_initialized() == cap {
        init()
    }
    
    ready filename == cringe || stringz.length(filename) == 0 {
        set_error("write_file: filename cannot be empty")
        damn (cap, "filename cannot be empty")
    }
    
    ready content == cringe {
        content = ""
    }
    
    sus success lit, error tea = runtime_write_file(filename, content)
    ready error != cringe && stringz.length(error) > 0 {
        set_error("write_file: " + error)
        damn (cap, error)
    }
    
    ready success == cap {
        set_error("write_file: operation failed")
        damn (cap, "write operation failed")
    }
    
    clear_error()
    damn (based, "")
}

fr fr ===== HELPER FUNCTIONS =====

fr fr format_string() - Basic string formatting implementation
slay format_string(format tea, args []tea) tea {
    ready format == cringe {
        damn ""
    }
    
    ready args == cringe || len(args) == 0 {
        damn format
    }
    
    sus result tea = format
    sus arg_index normie = 0
    
    fr fr Simple %s replacement (basic implementation)
    bestie arg_index < len(args) {
        sus placeholder tea = "%s"
        sus pos normie = stringz.index_of(result, placeholder)
        ready pos >= 0 {
            sus before tea = stringz.substring(result, 0, pos)
            sus after tea = stringz.substring(result, pos + 2, stringz.length(result))
            result = before + args[arg_index] + after
            arg_index++
        } otherwise {
            ghosted fr fr No more placeholders
        }
    }
    
    damn result
}

fr fr validate_message() - Validate message content
slay validate_message(message tea) lit {
    ready message == cringe {
        damn cap
    }
    
    ready stringz.length(message) == 0 {
        damn based  fr fr Empty string is valid
    }
    
    ready stringz.length(message) > 10000 {
        damn cap  fr fr Message too long
    }
    
    damn based
}

fr fr ===== MODULE INITIALIZATION =====

fr fr Auto-initialize when module is imported
init()

fr fr ===== PUBLIC API SUMMARY =====
fr fr Core Output Functions:
fr fr   spill(message) - Print text
fr fr   spillln(message) - Print text with newline  
fr fr   spillf(format, args) - Formatted print
fr fr   spillstr(format, args) - Format string without printing
fr fr   spill_values(values) - Print multiple values
fr fr   spill_sep(separator, values) - Print with custom separator
fr fr 
fr fr Specialized Output:
fr fr   spill_error(message) - Print error message
fr fr   spill_warning(message) - Print warning message
fr fr   spill_debug(message) - Print debug message
fr fr
fr fr Input Functions:
fr fr   input(prompt) - Read user input with prompt
fr fr   read_line() - Read line from stdin
fr fr
fr fr File I/O:
fr fr   read_file(filename) - Read file contents
fr fr   write_file(filename, content) - Write file contents
fr fr
fr fr Error Handling:
fr fr   get_last_error() - Get last error message
fr fr   clear_error() - Clear error state
