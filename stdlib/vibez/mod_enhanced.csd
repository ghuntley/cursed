fr fr Enhanced VIBEZ Module - Complete I/O Operations for CURSED
fr fr Production-ready implementation with comprehensive functionality

fr fr ===== CORE OUTPUT FUNCTIONS =====

fr fr Basic print function - outputs text to console with enhanced formatting
slay spill(message tea) lit {
    check message != "" {
        runtime_print_string(message)
    }
    damn based
}

fr fr Enhanced print function with multiple arguments
slay spill_multi(args ...tea) lit {
    sus output tea = ""
    sus i normie = 0
    bestie i < len_args(args) {
        check i > 0 {
            output = output + " "
        }
        output = output + args[i]
        i = i + 1
    }
    spill(output)
    damn based
}

fr fr Print with newline
slay spillln(message tea) lit {
    spill(message)
    spill("\n")
    damn based
}

fr fr Enhanced formatted print with proper placeholder replacement
slay spillf(format tea, args ...tea) lit {
    sus formatted tea = format_string_advanced(format, args)
    spill(formatted)
    damn based
}

fr fr Formatted print with newline
slay spillfln(format tea, args ...tea) lit {
    spillf(format, args)
    spill("\n")
    damn based
}

fr fr ===== ADVANCED STRING FORMATTING =====

fr fr Advanced string formatting with comprehensive placeholder support
slay format_string_advanced(format tea, args ...tea) tea {
    check format == "" {
        damn ""
    }
    
    check !string_contains(format, "%") {
        damn format
    }
    
    sus result tea = format
    sus arg_index normie = 0
    sus format_index normie = 0
    
    bestie format_index < string_length(format) {
        check string_char_at(format, format_index) == 37 { fr fr ASCII for '%'
            check format_index + 1 < string_length(format) {
                sus format_char normie = string_char_at(format, format_index + 1)
                check format_char == 115 { fr fr 's' for string
                    check arg_index < len_args(args) {
                        result = string_replace_at(result, format_index, 2, args[arg_index])
                        arg_index = arg_index + 1
                    }
                } highkey format_char == 100 { fr fr 'd' for decimal
                    check arg_index < len_args(args) {
                        sus num_str tea = int_to_string_safe(args[arg_index])
                        result = string_replace_at(result, format_index, 2, num_str)
                        arg_index = arg_index + 1
                    }
                } highkey format_char == 102 { fr fr 'f' for float
                    check arg_index < len_args(args) {
                        sus float_str tea = float_to_string_safe(args[arg_index])
                        result = string_replace_at(result, format_index, 2, float_str)
                        arg_index = arg_index + 1
                    }
                }
            }
        }
        format_index = format_index + 1
    }
    
    damn result
}

fr fr Enhanced integer to string conversion with error handling
slay int_to_string_safe(value tea) tea {
    check is_numeric_string(value) {
        sus num normie = string_to_int_safe(value)
        damn convert_int_to_string(num)
    }
    damn value
}

fr fr Enhanced float to string conversion
slay float_to_string_safe(value tea) tea {
    check is_float_string(value) {
        sus num meal = string_to_float_safe(value)
        damn convert_float_to_string(num)
    }
    damn value
}

fr fr ===== INPUT FUNCTIONS =====

fr fr Read a single line from input with enhanced error handling
slay read_line() tea {
    sus input tea = ""
    sus char normie = 0
    sus byte_count normie = 0
    
    bestie byte_count < 1024 { fr fr Prevent infinite loops
        char = runtime_read_char()
        check char == 10 || char == 13 || char == 0 { fr fr newline, carriage return, or null
            ghosted
        }
        input = input + char_to_string(char)
        byte_count = byte_count + 1
    }
    
    damn input
}

fr fr Read line with custom prompt
slay read_line_prompt(prompt tea) tea {
    spill(prompt)
    damn read_line()
}

fr fr Read integer with validation
slay read_int(prompt tea) normie {
    sus input tea = read_line_prompt(prompt)
    check is_numeric_string(input) {
        damn string_to_int_safe(input)
    }
    damn 0
}

fr fr Read float with validation
slay read_float(prompt tea) meal {
    sus input tea = read_line_prompt(prompt)
    check is_float_string(input) {
        damn string_to_float_safe(input)
    }
    damn 0.0
}

fr fr Read boolean with enhanced parsing
slay read_bool(prompt tea) lit {
    sus input tea = read_line_prompt(prompt)
    sus lower tea = string_to_lower(input)
    check lower == "true" || lower == "yes" || lower == "1" || lower == "based" {
        damn based
    }
    damn cringe
}

fr fr ===== FILE OPERATIONS =====

fr fr Read entire file with error handling
slay read_file_safe(filename tea) (tea, tea) {
    check file_exists_safe(filename) {
        sus content tea = runtime_read_file_content(filename)
        sus error tea = get_last_error()
        check error == "" {
            damn (content, "")
        }
        damn ("", error)
    }
    damn ("", "File not found")
}

fr fr Write file with comprehensive error handling
slay write_file_safe(filename tea, content tea) (lit, tea) {
    clear_last_error()
    sus success lit = runtime_write_file_content(filename, content)
    sus error tea = get_last_error()
    damn (success, error)
}

fr fr Append to file with safety checks
slay append_file_safe(filename tea, content tea) (lit, tea) {
    sus existing_content tea = ""
    sus error tea = ""
    
    check file_exists_safe(filename) {
        (existing_content, error) = read_file_safe(filename)
        check error != "" {
            damn (cringe, error)
        }
    }
    
    sus new_content tea = existing_content + content
    damn write_file_safe(filename, new_content)
}

fr fr ===== DIRECTORY OPERATIONS =====

fr fr List directory with enhanced error handling
slay list_directory_safe(path tea) ([]tea, tea) {
    check directory_exists_safe(path) {
        sus files []tea = runtime_list_directory_files(path)
        sus error tea = get_last_error()
        damn (files, error)
    }
    damn ([], "Directory not found")
}

fr fr Create directory with parent creation
slay create_directory_recursive(path tea) (lit, tea) {
    clear_last_error()
    sus success lit = runtime_create_directory_all(path)
    sus error tea = get_last_error()
    damn (success, error)
}

fr fr ===== CONSOLE FORMATTING =====

fr fr Set console text color with comprehensive color support
slay set_text_color(color tea) lit {
    check color == "black" { spill("\033[30m") }
    highkey color == "red" { spill("\033[31m") }
    highkey color == "green" { spill("\033[32m") }
    highkey color == "yellow" { spill("\033[33m") }
    highkey color == "blue" { spill("\033[34m") }
    highkey color == "magenta" { spill("\033[35m") }
    highkey color == "cyan" { spill("\033[36m") }
    highkey color == "white" { spill("\033[37m") }
    highkey color == "reset" { spill("\033[0m") }
    damn based
}

fr fr Set background color
slay set_background_color(color tea) lit {
    check color == "black" { spill("\033[40m") }
    highkey color == "red" { spill("\033[41m") }
    highkey color == "green" { spill("\033[42m") }
    highkey color == "yellow" { spill("\033[43m") }
    highkey color == "blue" { spill("\033[44m") }
    highkey color == "magenta" { spill("\033[45m") }
    highkey color == "cyan" { spill("\033[46m") }
    highkey color == "white" { spill("\033[47m") }
    damn based
}

fr fr Print colored text with automatic reset
slay spill_colored(text tea, color tea) lit {
    set_text_color(color)
    spill(text)
    set_text_color("reset")
    damn based
}

fr fr Clear console screen
slay clear_screen() lit {
    spill("\033[2J\033[H")
    damn based
}

fr fr Move cursor to position
slay move_cursor(row normie, col normie) lit {
    spillf("\033[%d;%dH", [int_to_string_safe(row), int_to_string_safe(col)])
    damn based
}

fr fr ===== ERROR AND LOGGING =====

fr fr Enhanced error printing with severity levels
slay log_error(message tea) lit {
    spill_colored("[ERROR] ", "red")
    spillln(message)
    damn based
}

slay log_warning(message tea) lit {
    spill_colored("[WARNING] ", "yellow")
    spillln(message)
    damn based
}

slay log_info(message tea) lit {
    spill_colored("[INFO] ", "green")
    spillln(message)
    damn based
}

slay log_debug(message tea) lit {
    spill_colored("[DEBUG] ", "cyan")
    spillln(message)
    damn based
}

fr fr Timestamped logging
slay log_with_timestamp(level tea, message tea) lit {
    sus timestamp tea = get_current_timestamp()
    spillf("[%s] %s: %s\n", [timestamp, level, message])
    damn based
}

fr fr ===== STRING UTILITY FUNCTIONS =====

fr fr Get character at index in string
slay string_char_at(text tea, index normie) normie {
    check index >= 0 && index < string_length(text) {
        damn runtime_get_char_at_index(text, index)
    }
    damn 0
}

fr fr Replace substring at specific position
slay string_replace_at(text tea, start normie, length normie, replacement tea) tea {
    check start >= 0 && start < string_length(text) {
        sus before tea = string_substring_safe(text, 0, start)
        sus after tea = string_substring_safe(text, start + length, string_length(text))
        damn before + replacement + after
    }
    damn text
}

fr fr Safe substring extraction
slay string_substring_safe(text tea, start normie, end normie) tea {
    check start >= 0 && end >= start && end <= string_length(text) {
        damn runtime_substring(text, start, end)
    }
    damn ""
}

fr fr Convert string to lowercase
slay string_to_lower(text tea) tea {
    sus result tea = ""
    sus i normie = 0
    bestie i < string_length(text) {
        sus char normie = string_char_at(text, i)
        check char >= 65 && char <= 90 { fr fr A-Z
            char = char + 32 fr fr Convert to lowercase
        }
        result = result + char_to_string(char)
        i = i + 1
    }
    damn result
}

fr fr Convert string to uppercase
slay string_to_upper(text tea) tea {
    sus result tea = ""
    sus i normie = 0
    bestie i < string_length(text) {
        sus char normie = string_char_at(text, i)
        check char >= 97 && char <= 122 { fr fr a-z
            char = char - 32 fr fr Convert to uppercase
        }
        result = result + char_to_string(char)
        i = i + 1
    }
    damn result
}

fr fr Check if string contains substring
slay string_contains(text tea, substring tea) lit {
    check substring == "" {
        damn based
    }
    check string_length(substring) > string_length(text) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i <= string_length(text) - string_length(substring) {
        sus match lit = based
        sus j normie = 0
        bestie j < string_length(substring) {
            check string_char_at(text, i + j) != string_char_at(substring, j) {
                match = cringe
                ghosted
            }
            j = j + 1
        }
        check match {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

fr fr ===== VALIDATION FUNCTIONS =====

fr fr Check if string represents a number
slay is_numeric_string(text tea) lit {
    check text == "" {
        damn cringe
    }
    
    sus i normie = 0
    sus start normie = 0
    
    fr fr Check for leading minus sign
    check string_char_at(text, 0) == 45 { fr fr '-'
        start = 1
        check string_length(text) == 1 {
            damn cringe
        }
    }
    
    i = start
    bestie i < string_length(text) {
        sus char normie = string_char_at(text, i)
        check char < 48 || char > 57 { fr fr Not 0-9
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

fr fr Check if string represents a float
slay is_float_string(text tea) lit {
    check text == "" {
        damn cringe
    }
    
    sus decimal_count normie = 0
    sus i normie = 0
    sus start normie = 0
    
    fr fr Check for leading minus sign
    check string_char_at(text, 0) == 45 { fr fr '-'
        start = 1
        check string_length(text) == 1 {
            damn cringe
        }
    }
    
    i = start
    bestie i < string_length(text) {
        sus char normie = string_char_at(text, i)
        check char == 46 { fr fr '.'
            decimal_count = decimal_count + 1
            check decimal_count > 1 {
                damn cringe
            }
        } highkey char < 48 || char > 57 { fr fr Not 0-9
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

fr fr ===== CONVERSION FUNCTIONS =====

fr fr Convert ASCII code to character string
slay char_to_string(ascii_code normie) tea {
    check ascii_code >= 32 && ascii_code <= 126 { fr fr Printable ASCII
        damn runtime_char_to_string(ascii_code)
    }
    damn ""
}

fr fr Safe string to integer conversion
slay string_to_int_safe(text tea) normie {
    check is_numeric_string(text) {
        damn runtime_string_to_int(text)
    }
    damn 0
}

fr fr Safe string to float conversion
slay string_to_float_safe(text tea) meal {
    check is_float_string(text) {
        damn runtime_string_to_float(text)
    }
    damn 0.0
}

fr fr Convert integer to string with bounds checking
slay convert_int_to_string(value normie) tea {
    damn runtime_int_to_string(value)
}

fr fr Convert float to string with precision control
slay convert_float_to_string(value meal) tea {
    damn runtime_float_to_string(value)
}

fr fr ===== HELPER FUNCTIONS =====

fr fr Get length of string
slay string_length(text tea) normie {
    damn runtime_string_length(text)
}

fr fr Get length of variadic arguments
slay len_args(args ...tea) normie {
    damn runtime_get_arg_count(args)
}

fr fr Get current timestamp in ISO format
slay get_current_timestamp() tea {
    damn runtime_get_current_time_iso()
}

fr fr Check if file exists
slay file_exists_safe(filename tea) lit {
    damn runtime_file_exists(filename)
}

fr fr Check if directory exists
slay directory_exists_safe(path tea) lit {
    damn runtime_directory_exists(path)
}

fr fr Get last error message
slay get_last_error() tea {
    damn runtime_get_last_error()
}

fr fr Clear last error
slay clear_last_error() cringe {
    runtime_clear_last_error()
    damn cringe
}

fr fr ===== RUNTIME INTERFACE FUNCTIONS =====

fr fr Core runtime functions that interface with the Zig runtime
slay runtime_print_string(message tea) cringe {
    fr fr Implemented in Zig runtime
    core.print(message)
    damn cringe
}

slay runtime_read_char() normie {
    fr fr Implemented in Zig runtime  
    damn core.read_char()
}

slay runtime_get_char_at_index(text tea, index normie) normie {
    fr fr Implemented in Zig runtime
    damn core.get_char_at(text, index)
}

slay runtime_substring(text tea, start normie, end normie) tea {
    fr fr Implemented in Zig runtime
    damn core.substring(text, start, end)
}

slay runtime_string_length(text tea) normie {
    fr fr Implemented in Zig runtime
    damn core.string_length(text)
}

slay runtime_char_to_string(ascii_code normie) tea {
    fr fr Implemented in Zig runtime
    damn core.char_to_string(ascii_code)
}

slay runtime_string_to_int(text tea) normie {
    fr fr Implemented in Zig runtime
    damn core.string_to_int(text)
}

slay runtime_string_to_float(text tea) meal {
    fr fr Implemented in Zig runtime
    damn core.string_to_float(text)
}

slay runtime_int_to_string(value normie) tea {
    fr fr Implemented in Zig runtime
    damn core.int_to_string(value)
}

slay runtime_float_to_string(value meal) tea {
    fr fr Implemented in Zig runtime
    damn core.float_to_string(value)
}

slay runtime_get_arg_count(args ...tea) normie {
    fr fr Implemented in Zig runtime
    damn core.get_variadic_count(args)
}

slay runtime_get_current_time_iso() tea {
    fr fr Implemented in Zig runtime
    damn core.get_current_time_iso()
}

slay runtime_read_file_content(filename tea) tea {
    fr fr Implemented in Zig runtime
    damn core.read_file(filename)
}

slay runtime_write_file_content(filename tea, content tea) lit {
    fr fr Implemented in Zig runtime
    damn core.write_file(filename, content)
}

slay runtime_file_exists(filename tea) lit {
    fr fr Implemented in Zig runtime
    damn core.file_exists(filename)
}

slay runtime_directory_exists(path tea) lit {
    fr fr Implemented in Zig runtime
    damn core.directory_exists(path)
}

slay runtime_list_directory_files(path tea) []tea {
    fr fr Implemented in Zig runtime
    damn core.list_directory(path)
}

slay runtime_create_directory_all(path tea) lit {
    fr fr Implemented in Zig runtime
    damn core.create_directory_recursive(path)
}

slay runtime_get_last_error() tea {
    fr fr Implemented in Zig runtime
    damn core.get_last_error()
}

slay runtime_clear_last_error() cringe {
    fr fr Implemented in Zig runtime
    core.clear_last_error()
    damn cringe
}
