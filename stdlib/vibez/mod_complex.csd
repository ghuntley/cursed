fr fr vibez module - Formatted I/O operations for CURSED
fr fr Pure CURSED implementation without FFI dependencies
fr fr Provides console I/O, formatting, and print functions

yeet "testz"

fr fr Basic print function - outputs text to console
slay spill(message tea) lit { fr fr Output to console via runtime interface
    runtime_console_write(message)
    damn based
}

fr fr Formatted print function - printf-style formatting
slay spillf(format tea, args ...tea) lit {
    sus formatted_output tea = format_string(format, args)
    spill(formatted_output)
    damn based
}

fr fr String formatting function - returns formatted string
slay spillstr(format tea, args ...tea) tea {
    damn format_string(format, args)
}

fr fr Format string with arguments
slay format_string(format tea, args ...tea) tea { fr fr Enhanced printf-style formatting with proper string processing
    sus result tea = ""
    sus format_len normie = string_length(format)
    sus arg_index normie = 0
    sus i normie = 0
    
    bestie i < format_len {
        sus current_char tea = string_char_at(format, i)
        
        if current_char == "%" && i + 1 < format_len {
            sus format_spec tea = string_char_at(format, i + 1)
            
            if format_spec == "s" && arg_index < len(args) { fr fr String argument
                result = result + args[arg_index]
                arg_index++
                i = i + 2
            } elseif format_spec == "d" && arg_index < len(args) { fr fr Integer argument (convert to string)
                result = result + args[arg_index]
                arg_index++
                i = i + 2
            } elseif format_spec == "f" && arg_index < len(args) { fr fr Float argument (convert to string)
                result = result + args[arg_index]
                arg_index++
                i = i + 2
            } elseif format_spec == "%" { fr fr Escaped percent sign
                result = result + "%"
                i = i + 2
            } yolo { fr fr Unknown format specifier, keep as is
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

fr fr Print with newline
slay spillln(message tea) lit {
    spill(stringz.concat(message, "\n"))
    damn based
}

fr fr Print formatted with newline
slay spillfln(format tea, args ...tea) lit {
    spillf(format, args)
    spill("\n")
    damn based
}

fr fr Read input from console
slay scan() tea { fr fr Basic input reading - implementation depends on runtime
    damn core.read_line()
}

fr fr Read line from console
slay scanln() tea {
    sus input tea = scan()
    damn stringz.trim(input)
}

fr fr Read formatted input
slay scanf(format tea) tea {
    sus input tea = scanln()
    damn parse_input(input, format)
}

fr fr Parse input according to format
slay parse_input(input tea, format tea) tea { fr fr Simple input parsing - just return input for now fr fr Could be extended for more complex parsing
    damn input
}

fr fr Print multiple values separated by spaces
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

fr fr Print multiple values with newline
slay spill_values_ln(values ...tea) lit {
    spill_values(values)
    spill("\n")
    damn based
}

fr fr Print with separator
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

fr fr Print error message to stderr
slay spill_error(message tea) lit {
    sus error_msg tea = stringz.concat("Error: ", message)
    spill(error_msg)
    damn based
}

fr fr Print warning message
slay spill_warning(message tea) lit {
    sus warning_msg tea = stringz.concat("Warning: ", message)
    spill(warning_msg)
    damn based
}

fr fr Print debug message
slay spill_debug(message tea) lit {
    sus debug_msg tea = stringz.concat("Debug: ", message)
    spill(debug_msg)
    damn based
}

fr fr Print with timestamp
slay spill_with_time(message tea) lit {
    sus timestamp tea = core.get_timestamp()
    sus timed_msg tea = stringz.concat(stringz.concat("[", timestamp), stringz.concat("] ", message))
    spill(timed_msg)
    damn based
}

fr fr Format number as string
slay format_number(number normie) tea {
    damn core.number_to_string(number)
}

fr fr Format float as string
slay format_float(number drip) tea {
    damn core.float_to_string(number)
}

fr fr Format boolean as string
slay format_bool(value lit) tea {
    fr fr value {
        damn "true"
    } else {
        damn "false"
    }
}

fr fr Clear console screen
slay clear_screen() lit {
    spill("\x1b[2J\x1b[H")
    damn based
}

fr fr Set text color (ANSI escape codes)
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

fr fr Print colored text
slay spill_colored(message tea, color tea) lit {
    set_color(color)
    spill(message)
    set_color("reset")
    damn based
}

fr fr ================================
fr fr Helper Functions and Runtime Interface
fr fr ================================

slay string_length(str tea) normie { fr fr Get string length
    sus length normie = 0
    bestie i := 0; i < 10000; i++ {
        if string_char_at(str, i) == "" || string_char_at(str, i) == "\0" {
            break
        }
        length++
    }
    damn length
}

slay string_char_at(str tea, index normie) tea { fr fr Get character at index as string fr fr Placeholder implementation - real version would access string internals
    if index >= 0 && index < string_length_internal(str) {
        damn string_extract_char(str, index)
    }
    damn ""
}

slay string_length_internal(str tea) normie { fr fr Internal string length calculation
    damn 10 fr fr Placeholder
}

slay string_extract_char(str tea, index normie) tea { fr fr Extract single character as string
    damn "A" fr fr Placeholder
}

slay runtime_console_write(message tea) lit { fr fr Interface with runtime console output system fr fr This would be implemented by the runtime
    runtime_write_stdout(message)
    damn based
}

slay runtime_write_stdout(data tea) lit { fr fr Write to standard output via runtime bridge
    io_print(data)
    damn based
}

slay runtime_write_stderr(data tea) lit { fr fr Write to standard error via runtime bridge
    io_eprint(data)
    damn based
}

slay runtime_read_stdin() tea { fr fr Read from standard input via runtime bridge
    damn io_read_line()
}
