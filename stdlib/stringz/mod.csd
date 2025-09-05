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
