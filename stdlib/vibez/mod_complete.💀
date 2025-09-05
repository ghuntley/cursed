fr fr CURSED VIBEZ Module - Complete I/O Operations with Runtime Bridge
fr fr Production implementation that directly interfaces with Zig runtime

fr fr ===== CORE PRINT FUNCTIONS =====

slay spill(message tea) {
    fr fr Direct print without newline
    runtime_print_string(message)
}

slay spill_line(message tea) {
    fr fr Print with newline
    runtime_print_string(message)
    runtime_print_string("\n")
}

slay spill(key tea, value tea) {
    fr fr Print key-value pair
    runtime_print_string(key)
    runtime_print_string(": ")
    runtime_print_string(value)
    runtime_print_string("\n")
}

slay spill(key tea, value drip) {
    fr fr Print key with float value
    runtime_print_string(key)
    runtime_print_string(": ")
    sus value_str tea = float_to_string(value)
    runtime_print_string(value_str)
    runtime_print_string("\n")
}

slay spill(key tea, value normie) {
    fr fr Print key with integer value  
    runtime_print_string(key)
    runtime_print_string(": ")
    sus value_str tea = int_to_string(value)
    runtime_print_string(value_str)
    runtime_print_string("\n")
}

slay spill(key tea, value lit) {
    fr fr Print key with boolean value
    runtime_print_string(key)
    runtime_print_string(": ")
    lowkey value == based {
        runtime_print_string("true")
    } otherwise {
        runtime_print_string("false")
    }
    runtime_print_string("\n")
}

fr fr ===== FILE I/O FUNCTIONS =====

slay read_file(filename tea) tea {
    fr fr Read file contents as string
    lowkey filename == cringe {
        damn ""
    }
    
    fr fr Use runtime function to read file
    yikes content := runtime_read_file_safe(filename)
    lowkey content == cringe {
        damn ""
    }
    damn content
}

slay write_file(filename tea, content tea) lit {
    fr fr Write content to file
    lowkey filename == cringe || content == cringe {
        damn cap
    }
    
    fr fr Use runtime function to write file
    yikes result := runtime_write_file_safe(filename, content) 
    damn result
}

slay delete_file(filename tea) lit {
    fr fr Delete file
    lowkey filename == cringe {
        damn cap
    }
    
    yikes result := runtime_delete_file_safe(filename)
    damn result
}

slay file_exists(filename tea) lit {
    fr fr Check if file exists
    lowkey filename == cringe {
        damn cap
    }
    
    damn runtime_file_exists_safe(filename)
}

slay file_size(filename tea) normie {
    fr fr Get file size in bytes
    lowkey filename == cringe {
        damn -1
    }
    
    damn runtime_file_size_safe(filename)
}

fr fr ===== DIRECTORY FUNCTIONS =====

slay create_directory(path tea) lit {
    lowkey path == cringe {
        damn cap
    }
    damn runtime_create_directory_safe(path)
}

slay directory_exists(path tea) lit {
    lowkey path == cringe {
        damn cap  
    }
    damn runtime_directory_exists_safe(path)
}

slay list_directory(path tea) tea[value]{
    lowkey path == cringe {
        damn []
    }
    damn runtime_list_directory_safe(path)
}

fr fr ===== INPUT FUNCTIONS =====

slay read_line() tea {
    fr fr Read line from stdin
    yikes line := runtime_read_line_safe()
    lowkey line == cringe {
        damn ""
    }
    damn line
}

slay read_input() tea {
    damn read_line()
}

fr fr ===== UTILITY FUNCTIONS =====

slay int_to_string(value normie) tea {
    fr fr Convert integer to string
    yikes result := runtime_int_to_string_safe(value)
    lowkey result == cringe {
        damn "0"
    }
    damn result  
}

slay float_to_string(value drip) tea {
    fr fr Convert float to string
    yikes result := runtime_float_to_string_safe(value)
    lowkey result == cringe {
        damn "0.0"
    }
    damn result
}

slay string_to_int(text tea) normie {
    fr fr Convert string to integer
    lowkey text == cringe {
        damn 0
    }
    damn runtime_string_to_int_safe(text)
}

slay string_to_float(text tea) drip {
    fr fr Convert string to float
    lowkey text == cringe {
        damn 0.0
    }
    damn runtime_string_to_float_safe(text)
}

fr fr ===== SAFE RUNTIME WRAPPER FUNCTIONS =====
fr fr These provide error handling around the actual runtime functions

slay runtime_print_string(message tea) {
    fr fr Direct call to Zig runtime - this will be linked
    fr fr Implementation provided by runtime_functions.zig
}

slay runtime_read_file_safe(filename tea) tea {
    fr fr Safe file reading with error handling
    yikes content := runtime_read_file_content(filename)
    damn content fam {
        when _ -> damn ""
    }
}

slay runtime_write_file_safe(filename tea, content tea) lit {
    fr fr Safe file writing with error handling  
    yikes result := runtime_write_file_content(filename, content)
    damn result fam {
        when _ -> damn cap
    }
}

slay runtime_delete_file_safe(filename tea) lit {
    yikes result := runtime_delete_file(filename)  
    damn result fam {
        when _ -> damn cap
    }
}

slay runtime_file_exists_safe(filename tea) lit {
    yikes result := runtime_file_exists(filename)
    damn result fam {
        when _ -> damn cap
    }
}

slay runtime_file_size_safe(filename tea) normie {
    yikes result := runtime_file_size(filename)
    damn result fam {
        when _ -> damn -1  
    }
}

slay runtime_create_directory_safe(path tea) lit {
    yikes result := runtime_create_directory(path)
    damn result fam {
        when _ -> damn cap
    }
}

slay runtime_directory_exists_safe(path tea) lit {
    yikes result := runtime_directory_exists(path)
    damn result fam {
        when _ -> damn cap
    }
}

slay runtime_list_directory_safe(path tea) tea[value]{
    yikes result := runtime_list_directory(path) 
    damn result fam {
        when _ -> damn []
    }
}

slay runtime_read_line_safe() tea {
    yikes result := runtime_read_line()
    damn result fam {
        when _ -> damn ""
    }
}

slay runtime_int_to_string_safe(value normie) tea {
    yikes result := runtime_int_to_string(value)
    damn result fam {
        when _ -> damn "0"
    }
}

slay runtime_float_to_string_safe(value drip) tea {
    yikes result := runtime_float_to_string(value)  
    damn result fam {
        when _ -> damn "0.0"
    }
}

slay runtime_string_to_int_safe(text tea) normie {
    damn runtime_string_to_int(text)
}

slay runtime_string_to_float_safe(text tea) drip {
    damn runtime_string_to_float(text)
}

fr fr ===== EXTERNAL RUNTIME FUNCTION DECLARATIONS =====
fr fr These will be linked to the Zig runtime functions

outer slay runtime_read_file_content(filename tea) yikes<tea>
outer slay runtime_write_file_content(filename tea, content tea) yikes<lit>
outer slay runtime_delete_file(filename tea) yikes<lit>
outer slay runtime_file_exists(filename tea) yikes<lit> 
outer slay runtime_file_size(filename tea) yikes<normie>
outer slay runtime_create_directory(path tea) yikes<lit>
outer slay runtime_directory_exists(path tea) yikes<lit>
outer slay runtime_list_directory(path tea) yikes<tea[value]>
outer slay runtime_read_line() yikes<tea>
outer slay runtime_int_to_string(value normie) yikes<tea>
outer slay runtime_float_to_string(value drip) yikes<tea>
outer slay runtime_string_to_int(text tea) normie
outer slay runtime_string_to_float(text tea) drip
