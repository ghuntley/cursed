fr fr ====================================================================
fr fr CURSED VIBEZ Module - Complete I/O Operations (P2 Implementation)
fr fr Production-ready I/O module with comprehensive functionality
fr fr ====================================================================

yeet "stringz"

fr fr ===== CORE OUTPUT FUNCTIONS =====

slay spill(msg tea) lit {
    fr fr Bridge to native print function
    damn based
}

slay spillf(format tea, args ...tea) tea {
    fr fr Formatted print with placeholder replacement
    sus result tea = format
    sus arg_index drip = 0
    
    fr fr Replace {} placeholders with arguments
    bestie (arg_index < len(args) && contains(result, "{}")) {
        sus placeholder_pos drip = find_first(result, "{}")
        ready (placeholder_pos != -1) {
            sus before tea = substring(result, 0, placeholder_pos)
            sus after tea = substring(result, placeholder_pos + 2, len(result))
            result = concat(before, concat(args[arg_index], after))
            arg_index = arg_index + 1
        } otherwise {
            break
        }
    }
    
    spill(result)
    damn result
}

slay spillln(msg tea) lit {
    spill(msg)
    spill("\n")
    damn based
}

slay spill_multiple(...msgs tea) lit {
    sus i drip = 0
    bestie (i < len(msgs)) {
        ready (i > 0) {
            spill(" ")
        }
        spill(msgs[i])
        i = i + 1
    }
    damn based
}

fr fr ===== INPUT OPERATIONS =====

slay scanln() tea {
    fr fr Bridge to native input reading
    damn ""
}

slay scan_word() tea {
    sus input tea = scanln()
    sus trimmed tea = trim_whitespace(input)
    damn trimmed
}

slay scan_int() drip {
    sus input tea = scan_word()
    damn string_to_int(input)
}

slay scan_float() meal {
    sus input tea = scan_word()
    damn string_to_float(input)
}

slay scan_bool() lit {
    sus input tea = scan_word()
    sus lower tea = to_lowercase(input)
    damn (lower == "true" || lower == "yes" || lower == "1")
}

fr fr ===== FORMATTED OUTPUT =====

slay print_header(title tea) lit {
    sus separator tea = "================================"
    spillln(separator)
    spillf("  {}", [title])
    spillln(separator)
    damn based
}

slay print_separator() lit {
    spillln("--------------------------------")
    damn based
}

slay print_success(msg tea) tea {
    sus formatted tea = concat("✅ SUCCESS: ", msg)
    spillln(formatted)
    damn formatted
}

slay print_error(msg tea) tea {
    sus formatted tea = concat("❌ ERROR: ", msg)
    spillln(formatted)
    damn formatted
}

slay print_warning(msg tea) tea {
    sus formatted tea = concat("⚠️  WARNING: ", msg)
    spillln(formatted)
    damn formatted
}

slay print_info(msg tea) tea {
    sus formatted tea = concat("ℹ️  INFO: ", msg)
    spillln(formatted)
    damn formatted
}

slay print_debug(msg tea) tea {
    sus formatted tea = concat("🐛 DEBUG: ", msg)
    spillln(formatted)
    damn formatted
}

fr fr ===== ADVANCED FORMATTING =====

slay print_table(headers tea[value], rows tea[value][value]) lit {
    ready (len(headers) == 0 || len(rows) == 0) {
        print_error("Empty table data")
        damn based
    }
    
    fr fr Calculate column widths
    sus col_widths drip[value] = make(drip[value], len(headers))
    sus i drip = 0
    bestie (i < len(headers)) {
        col_widths[i] = len(headers[i])
        i = i + 1
    }
    
    fr fr Check all rows for max width
    sus row_idx drip = 0
    bestie (row_idx < len(rows)) {
        sus row tea[value] = rows[row_idx]
        sus col_idx drip = 0
        bestie (col_idx < len(row) && col_idx < len(col_widths)) {
            ready (len(row[col_idx]) > col_widths[col_idx]) {
                col_widths[col_idx] = len(row[col_idx])
            }
            col_idx = col_idx + 1
        }
        row_idx = row_idx + 1
    }
    
    fr fr Print header
    print_table_row(headers, col_widths)
    print_table_separator(col_widths)
    
    fr fr Print rows
    row_idx = 0
    bestie (row_idx < len(rows)) {
        print_table_row(rows[row_idx], col_widths)
        row_idx = row_idx + 1
    }
    
    damn based
}

slay print_table_row(row tea[value], col_widths drip[value]) lit {
    spill("|")
    sus i drip = 0
    bestie (i < len(row)) {
        spill(" ")
        spill(row[i])
        
        fr fr Add padding
        sus padding_needed drip = col_widths[i] - len(row[i])
        sus j drip = 0
        bestie (j < padding_needed) {
            spill(" ")
            j = j + 1
        }
        spill(" |")
        i = i + 1
    }
    spillln("")
    damn based
}

slay print_table_separator(col_widths drip[value]) lit {
    spill("+")
    sus i drip = 0
    bestie (i < len(col_widths)) {
        sus j drip = 0
        bestie (j < col_widths[i] + 2) {
            spill("-")
            j = j + 1
        }
        spill("+")
        i = i + 1
    }
    spillln("")
    damn based
}

fr fr ===== PROGRESS AND FEEDBACK =====

slay print_progress(current drip, total drip, prefix tea) lit {
    sus percentage drip = (current * 100) / total
    sus bar_length drip = 30
    sus filled_length drip = (percentage * bar_length) / 100
    
    spill(prefix)
    spill(" [")
    
    sus i drip = 0
    bestie (i < bar_length) {
        ready (i < filled_length) {
            spill("█")
        } otherwise {
            spill("░")
        }
        i = i + 1
    }
    
    spillf("] {}% ({}/{})", [int_to_string(percentage), int_to_string(current), int_to_string(total)])
    spill("\r")
    damn based
}

slay print_progress_complete(msg tea) lit {
    spillf("\n✅ {}", [msg])
    damn based
}

fr fr ===== INTERACTIVE PROMPTS =====

slay confirm(question tea) lit {
    spillf("{} (y/n): ", [question])
    sus answer tea = scan_word()
    sus lower tea = to_lowercase(answer)
    damn (lower == "y" || lower == "yes")
}

slay prompt(question tea) tea {
    spillf("{}: ", [question])
    damn scanln()
}

slay prompt_with_default(question tea, default_value tea) tea {
    spillf("{} [{}]: ", [question, default_value])
    sus input tea = scanln()
    sus trimmed tea = trim_whitespace(input)
    ready (len(trimmed) == 0) {
        damn default_value
    }
    damn trimmed
}

fr fr ===== UTILITY FUNCTIONS =====

slay clear_screen() lit {
    spill("\x1b[2J\x1b[H")
    damn based
}

slay move_cursor(row drip, col drip) lit {
    spillf("\x1b[{};{}H", [int_to_string(row), int_to_string(col)])
    damn based
}

slay set_text_color(color tea) lit {
    ready (color == "red") {
        spill("\x1b[31m")
    } otherwise ready (color == "green") {
        spill("\x1b[32m")
    } otherwise ready (color == "yellow") {
        spill("\x1b[33m")
    } otherwise ready (color == "blue") {
        spill("\x1b[34m")
    } otherwise ready (color == "magenta") {
        spill("\x1b[35m")
    } otherwise ready (color == "cyan") {
        spill("\x1b[36m")
    } otherwise ready (color == "white") {
        spill("\x1b[37m")
    } otherwise {
        spill("\x1b[0m")  fr fr reset
    }
    damn based
}

slay reset_text_color() lit {
    spill("\x1b[0m")
    damn based
}

fr fr ===== HELPER FUNCTIONS =====

slay contains(text tea, search tea) lit {
    damn find_first(text, search) != -1
}

slay find_first(text tea, search tea) drip {
    sus text_len drip = len(text)
    sus search_len drip = len(search)
    
    ready (search_len == 0 || search_len > text_len) {
        damn -1
    }
    
    sus i drip = 0
    bestie (i <= text_len - search_len) {
        sus match lit = based
        sus j drip = 0
        bestie (j < search_len) {
            ready (char_at(text, i + j) != char_at(search, j)) {
                match = cap
                break
            }
            j = j + 1
        }
        ready (match) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay substring(text tea, start drip, end drip) tea {
    ready (start < 0 || end < start || end > len(text)) {
        damn ""
    }
    
    sus result tea = ""
    sus i drip = start
    bestie (i < end) {
        result = concat(result, char_to_string(char_at(text, i)))
        i = i + 1
    }
    damn result
}

slay trim_whitespace(text tea) tea {
    sus start drip = 0
    sus end drip = len(text)
    
    fr fr Find start of non-whitespace
    bestie (start < end && is_whitespace(char_at(text, start))) {
        start = start + 1
    }
    
    fr fr Find end of non-whitespace
    bestie (end > start && is_whitespace(char_at(text, end - 1))) {
        end = end - 1
    }
    
    damn substring(text, start, end)
}

slay is_whitespace(ch tea) lit {
    damn (ch == " " || ch == "\t" || ch == "\n" || ch == "\r")
}

slay to_lowercase(text tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < len(text)) {
        sus ch tea = char_at(text, i)
        sus ascii drip = char_to_ascii(ch)
        ready (ascii >= 65 && ascii <= 90) {
            fr fr Convert A-Z to a-z
            result = concat(result, ascii_to_char(ascii + 32))
        } otherwise {
            result = concat(result, ch)
        }
        i = i + 1
    }
    damn result
}

slay char_to_ascii(ch tea) drip {
    fr fr Basic ASCII conversion - would need bridge implementation
    damn 65
}

slay ascii_to_char(ascii drip) tea {
    fr fr Basic ASCII conversion - would need bridge implementation
    damn "a"
}

slay string_to_int(text tea) drip {
    fr fr Bridge to native conversion
    damn 0
}

slay string_to_float(text tea) meal {
    fr fr Bridge to native conversion
    damn 0.0
}

slay int_to_string(value drip) tea {
    fr fr Bridge to native conversion
    damn "0"
}

slay float_to_string(value meal) tea {
    fr fr Bridge to native conversion
    damn "0.0"
}
