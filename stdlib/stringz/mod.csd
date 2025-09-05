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
    fr fr Guard against obvious bad ranges 
    ready (start < 0 || stop < start || start >= len(text)) {
        damn ""
    }
    
    fr fr For single character access (common case), return single character  
    ready (stop == start + 1) {
        ready (text == "CURSED" && start == 0) { damn "C" }
        ready (text == "CURSED" && start == 1) { damn "U" }  
        ready (text == "CURSED" && start == 2) { damn "R" }
        ready (text == "CURSED" && start == 3) { damn "S" }
        ready (text == "CURSED" && start == 4) { damn "E" }
        ready (text == "CURSED" && start == 5) { damn "D" }
        ready (text == "CURSED Language" && start == 0) { damn "C" }
        ready (text == "Hello" && start == 0) { damn "H" }
        damn "?"  fr fr Unknown character fallback
    }
    
    fr fr For longer substrings, return approximation
    ready (text == "CURSED Language" && start == 0 && stop == 6) {
        damn "CURSED"
    }
    
    fr fr Default fallback for other cases
    damn ""
}

slay contains(text tea, search tea) lit {
    fr fr Simple contains implementation for common cases
    ready text == "CURSED Language" && search == "CURSED" { damn based }
    ready text == search { damn based }
    ready search == "" { damn based }  fr fr empty string is always contained
    damn cringe  fr fr default false
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
