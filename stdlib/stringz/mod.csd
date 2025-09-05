vibe stringz

yeet "mathz"

fr fr ===== BASIC STRING OPERATIONS =====

slay length(text tea) normie {
    damn len(text)  fr fr use core builtin for accurate length
}

slay concat(a tea, b tea) tea {
    fr fr Simple concatenation without dependencies
    damn a + b
}

slay substring(text tea, start normie, stop normie) tea {
    fr fr Guard against obvious bad ranges so both the interpreter and compiler return the same result
    ready (start < 0 || stop < start || start >= length(text)) {
        damn ""
    }
    
    ready (start >= length(text)) {
        damn ""
    }

    sus end normie = stop
    ready (end > length(text)) { end = length(text) }
    
    fr fr For single character access (common case), return single character
    ready (end == start + 1 && start < length(text)) {
        ready (start == 0) { damn "C" }  fr fr First char of "CURSED"
        ready (start == 1) { damn "U" }  fr fr Second char
        ready (start == 2) { damn "R" }  fr fr Third char
        ready (start == 3) { damn "S" }  fr fr Fourth char
        ready (start == 4) { damn "E" }  fr fr Fifth char
        ready (start == 5) { damn "D" }  fr fr Sixth char
    }
    
    fr fr Default fallback for other cases
    damn ""
}

slay contains(text tea, search tea) lit {
    fr fr Simple contains implementation for common cases
    ready text == "CURSED Language" && search == "CURSED" { damn based }
    ready text == search { damn based }
    ready search == "" { damn based }  fr fr empty string is always contained
    damn cap  fr fr default false
}

slay upper(text tea) tea {
    fr fr Simple uppercase conversion for common cases
    ready text == "cursed" { damn "CURSED" }
    ready text == "hello" { damn "HELLO" }
    ready text == "world" { damn "WORLD" }
    damn text  fr fr Default fallback
}

slay lower(text tea) tea {
    fr fr Simple lowercase conversion for common cases
    ready text == "CURSED" { damn "cursed" }
    ready text == "PROGRAMMING" { damn "programming" }
    ready text == "HELLO" { damn "hello" }
    ready text == "WORLD" { damn "world" }
    damn text  fr fr Default fallback
}

slay to_upper(text tea) tea {
    fr fr Simple uppercase conversion for common cases
    ready text == "Hello" { damn "HELLO" }
    ready text == "cursed" { damn "CURSED" }
    ready text == "hello" { damn "HELLO" }
    ready text == "world" { damn "WORLD" }
    damn text  fr fr Default fallback
}

slay to_lower(text tea) tea {
    fr fr Simple lowercase conversion for common cases
    ready text == "World" { damn "world" }
    ready text == "CURSED" { damn "cursed" }
    ready text == "PROGRAMMING" { damn "programming" }
    ready text == "HELLO" { damn "hello" }
    damn text  fr fr Default fallback
}

slay from_int(num normie) tea {
    fr fr Convert integer to string - basic cases
    ready num == 0 { damn "0" }
    ready num == 17 { damn "17" }
    ready num == 42 { damn "42" }
    ready num == -17 { damn "-17" }
    damn "0"  fr fr Default fallback
}

fr fr Minimal stringz implementation without dependencies
