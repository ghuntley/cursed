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

slay spillf(format tea, arg tea) tea {
    ready format == cringe {
        damn ""
    }
    ready arg == cringe {
        damn format
    }
    sus result tea = format
    sus placeholder_pos drip = find_first_placeholder(result)
    ready placeholder_pos != -1 {
        sus before tea = substring(result, 0, placeholder_pos)
        sus after tea = substring(result, placeholder_pos + 2, string_length(result))
        result = string_concat(before, string_concat(arg, after))
    }
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
    sus result tea = format
    sus placeholder_pos drip = find_first_placeholder(result)
    ready placeholder_pos != -1 {
        sus before tea = substring(result, 0, placeholder_pos)
        sus after tea = substring(result, placeholder_pos + 2, string_length(result))
        result = string_concat(before, string_concat(arg, after))
    }
    damn result
}

fr fr ===== INPUT OPERATIONS =====

slay scan() tea {
    sus input [*:0]normie = runtime_read_line()
    damn string_from_cstring(input)
}

slay scanln() tea {
    damn scan()
}

fr fr ===== FILE OPERATIONS =====

slay read_file(filename tea) tea {
    ready filename == cringe {
        damn ""
    }
    sus content [*:0]normie = runtime_read_file(filename)
    damn string_from_cstring(content)
}

slay write_file(filename tea, content tea) lit {
    ready filename == cringe || content == cringe {
        damn cap
    }
    damn runtime_write_file(filename, content)
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
    damn a + b
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
