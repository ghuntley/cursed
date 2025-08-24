fr fr CURSED VIBEZ Module - I/O Operations
fr fr Production-ready implementation with runtime bridge integration

fr fr Runtime bridge functions - implemented in Zig runtime
outer slay runtime_print_string(message [*:0]normie) 
outer slay runtime_read_line() [*:0]normie
outer slay runtime_read_file(filename [*:0]normie) [*:0]normie
outer slay runtime_write_file(filename [*:0]normie, content [*:0]normie) lit

fr fr ===== CORE OUTPUT FUNCTIONS =====

slay spill(msg tea) lit {
    ready msg == cringe {
        damn cap
    }
    runtime_print_string(msg)
    damn based
}

slay spill_two(msg1 tea, msg2 tea) lit {
    ready msg1 == cringe || msg2 == cringe {
        damn cap
    }
    runtime_print_string(msg1)
    runtime_print_string(" ")
    runtime_print_string(msg2)
    damn based
}

slay spillln(msg tea) lit {
    ready msg == cringe {
        damn cap
    }
    runtime_print_string(msg)
    runtime_print_string("\n")
    damn based
}

fr fr ===== CONSOLE FORMATTING =====

slay print_header(title tea) lit {
    ready title == cringe {
        damn cap
    }
    sus separator tea = "================================"
    spillln(separator)
    runtime_print_string("  ")
    spillln(title)
    spillln(separator)
    damn based
}

slay print_separator() lit {
    spillln("--------------------------------")
    damn based
}

slay print_success(msg tea) lit {
    ready msg == cringe {
        damn cap
    }
    runtime_print_string("✅ SUCCESS: ")
    spillln(msg)
    damn based
}

slay print_error(msg tea) lit {
    ready msg == cringe {
        damn cap
    }
    runtime_print_string("❌ ERROR: ")
    spillln(msg)
    damn based
}

slay print_warning(msg tea) lit {
    ready msg == cringe {
        damn cap
    }
    runtime_print_string("⚠️  WARNING: ")
    spillln(msg)
    damn based
}

slay print_info(msg tea) lit {
    ready msg == cringe {
        damn cap
    }
    runtime_print_string("ℹ️  INFO: ")
    spillln(msg)
    damn based
}

fr fr ===== FORMATTED OUTPUT =====

yeet "advanced_formatting"
yeet "real_io_operations"

slay spillf(format tea, arg tea) tea {
    ready format == cringe {
        damn ""
    }
    ready arg == cringe {
        damn format
    }
    
    fr fr Use advanced formatting system for real placeholder replacement  
    sus args []tea = [arg]
    sus result tea = format_advanced(format, args)
    runtime_print_string(result)
    damn result
}

slay spillf_multi(format tea, args ...tea) tea {
    ready format == cringe {
        damn ""
    }
    
    fr fr Advanced multi-argument formatting
    sus result tea = format_advanced(format, args)
    runtime_print_string(result)
    damn result
}

slay spillstr(format tea, arg tea) tea {
    ready format == cringe {
        damn ""
    }
    ready arg == cringe {
        damn format
    }
    
    fr fr Use advanced formatting without printing
    sus args []tea = [arg]
    damn format_advanced(format, args)
}

slay spillstr_multi(format tea, args ...tea) tea {
    ready format == cringe {
        damn ""
    }
    
    fr fr Advanced multi-argument string formatting
    damn format_advanced(format, args)
}

fr fr ===== INPUT OPERATIONS =====

slay scan() tea {
    fr fr Use real I/O operations for better input handling
    damn read_line_real()
}

slay scanln() tea {
    damn read_line_real()
}

slay scan_with_prompt(prompt tea) tea {
    ready prompt != cringe {
        runtime_print_string(prompt)
    }
    damn read_line_real()
}

fr fr ===== FILE OPERATIONS =====

slay read_file(filename tea) tea {
    ready filename == cringe {
        damn ""
    }
    
    fr fr Use real file I/O operations  
    sus content tea = read_file_real(filename)
    ready get_io_error() != IO_SUCCESS {
        damn ""
    }
    damn content
}

slay write_file(filename tea, content tea) lit {
    ready filename == cringe || content == cringe {
        damn cap
    }
    
    fr fr Use real file I/O operations
    damn write_file_real(filename, content)
}

slay append_file(filename tea, content tea) lit {
    ready filename == cringe || content == cringe {
        damn cap
    }
    
    fr fr Use real file append operations
    damn append_file_real(filename, content)
}

slay file_exists(filename tea) lit {
    ready filename == cringe {
        damn cap
    }
    damn file_exists_real(filename)
}

slay get_file_size(filename tea) drip {
    ready filename == cringe {
        damn -1.0
    }
    sus size normie = get_file_size_real(filename)
    damn int_to_float_precise(size)
}

fr fr ===== UTILITY FUNCTIONS =====

slay find_first_placeholder(text tea) drip {
    sus i drip = 0
    sus len drip = string_length(text)
    bestie i < len - 1 {
        ready char_at(text, i) == "{" && char_at(text, i + 1) == "}" {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay substring(text tea, start drip, end drip) tea {
    ready start < 0 || end < start || end > string_length(text) {
        damn ""
    }
    sus result tea = ""
    sus i drip = start
    bestie i < end {
        result = string_concat(result, char_to_string(char_at(text, i)))
        i = i + 1
    }
    damn result
}

slay string_concat(a tea, b tea) tea {
    ready a == cringe { damn b }
    ready b == cringe { damn a }
    
    fr fr Use more robust string concatenation
    sus result tea = ""
    sus i drip = 0
    
    fr fr Copy first string
    bestie i < string_length(a) {
        result = result + char_to_string(char_at(a, i))
        i = i + 1
    }
    
    fr fr Copy second string
    i = 0
    bestie i < string_length(b) {
        result = result + char_to_string(char_at(b, i))
        i = i + 1
    }
    
    damn result
}

slay string_from_cstring(cstr [*:0]normie) tea {
    sus result tea = ""
    sus i drip = 0
    bestie cstr[i] != 0 {
        result = string_concat(result, char_to_string(cstr[i]))
        i = i + 1
    }
    damn result
}

slay char_at(text tea, index drip) normie {
    ready index < 0 || index >= string_length(text) {
        damn 0
    }
    damn text[index]
}

slay char_to_string(c normie) tea {
    ready c == 0 { damn "" }
    damn string_from_char(c)
}

slay string_from_char(c normie) tea {
    sus result tea = ""
    result[0] = c
    damn result
}

slay string_length(text tea) drip {
    sus len drip = 0
    bestie text[len] != 0 {
        len = len + 1
    }
    damn len
}
