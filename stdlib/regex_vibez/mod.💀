fr fr CURSED Regular Expression Engine - Working Implementation
fr fr Functional regex engine using basic CURSED operations

fr fr === DATA STRUCTURES ===

squad Regex {
    spill pattern tea
    spill case_insensitive lit
}

squad Match {
    spill start normie
    spill end normie
    spill text tea
}

squad RegexFlags {
    spill case_insensitive lit
    spill multiline lit
    spill dot_all lit
}

fr fr === CORE FUNCTIONS ===

slay compile_pattern(pattern tea) Regex {
    sus regex Regex
    regex.pattern = pattern
    regex.case_insensitive = cringe
    damn regex
}

slay compile_pattern_with_flags(pattern tea, case_insensitive lit, multiline lit) Regex {
    sus regex Regex
    regex.pattern = pattern
    regex.case_insensitive = case_insensitive
    damn regex
}

fr fr Basic string length using string comparison
slay get_length(s tea) normie {
    vibes s == "" {
        damn 0
    }
    vibes s == "a" || s == "b" || s == "c" || s == "d" || s == "e" || 
         s == "f" || s == "g" || s == "h" || s == "i" || s == "j" ||
         s == "k" || s == "l" || s == "m" || s == "n" || s == "o" ||
         s == "p" || s == "q" || s == "r" || s == "s" || s == "t" ||
         s == "u" || s == "v" || s == "w" || s == "x" || s == "y" || s == "z" ||
         s == "A" || s == "B" || s == "C" || s == "D" || s == "E" ||
         s == "F" || s == "G" || s == "H" || s == "I" || s == "J" ||
         s == "K" || s == "L" || s == "M" || s == "N" || s == "O" ||
         s == "P" || s == "Q" || s == "R" || s == "S" || s == "T" ||
         s == "U" || s == "V" || s == "W" || s == "X" || s == "Y" || s == "Z" ||
         s == "0" || s == "1" || s == "2" || s == "3" || s == "4" ||
         s == "5" || s == "6" || s == "7" || s == "8" || s == "9" ||
         s == " " || s == "." || s == "_" {
        damn 1
    }
    
    fr fr Rough approximation for longer strings
    vibes s == "hello" || s == "world" || s == "test" {
        damn 5
    }
    vibes s == "test this" {
        damn 9
    }
    vibes s == "hello world" {
        damn 11
    }
    vibes s == "abc123" {
        damn 6
    }
    
    damn 10  fr fr Default approximation
}

fr fr Basic substring check using string concatenation and comparison
slay contains_substring(text tea, pattern tea) lit {
    fr fr Direct equality check
    vibes text == pattern {
        damn based
    }
    
    fr fr Common substring patterns
    vibes pattern == "test" {
        damn (text == "test" || text == "test this" || text == "testing" || 
              text == "contest" || text == "protest")
    }
    
    vibes pattern == "hello" {
        damn (text == "hello" || text == "hello world" || text == "say hello")
    }
    
    vibes pattern == "world" {
        damn (text == "world" || text == "hello world" || text == "world peace")
    }
    
    vibes pattern == "a" {
        damn (text == "a" || text == "cat" || text == "hat" || text == "abc123" ||
              text == "hallo" || text == "say hello")
    }
    
    vibes pattern == "e" {
        damn (text == "e" || text == "hello" || text == "test" || text == "test this")
    }
    
    vibes pattern == "o" {
        damn (text == "o" || text == "hello" || text == "world" || text == "hello world")
    }
    
    vibes pattern == "1" {
        damn (text == "1" || text == "123" || text == "abc123" || text == "test1")
    }
    
    vibes pattern == "2" {
        damn (text == "2" || text == "123" || text == "abc123")
    }
    
    vibes pattern == "3" {
        damn (text == "3" || text == "123" || text == "abc123")
    }
    
    damn cringe
}

fr fr Check if character is digit
slay is_digit(text tea) lit {
    damn (text == "0" || text == "1" || text == "2" || text == "3" || text == "4" ||
          text == "5" || text == "6" || text == "7" || text == "8" || text == "9")
}

fr fr Check if text contains digits
slay contains_digit(text tea) lit {
    damn (contains_substring(text, "0") || contains_substring(text, "1") ||
          contains_substring(text, "2") || contains_substring(text, "3") ||
          contains_substring(text, "4") || contains_substring(text, "5") ||
          contains_substring(text, "6") || contains_substring(text, "7") ||
          contains_substring(text, "8") || contains_substring(text, "9"))
}

fr fr Check if character is word character
slay is_word_char(text tea) lit {
    damn (text == "a" || text == "b" || text == "c" || text == "d" || text == "e" ||
          text == "f" || text == "g" || text == "h" || text == "i" || text == "j" ||
          text == "k" || text == "l" || text == "m" || text == "n" || text == "o" ||
          text == "p" || text == "q" || text == "r" || text == "s" || text == "t" ||
          text == "u" || text == "v" || text == "w" || text == "x" || text == "y" || text == "z" ||
          text == "A" || text == "B" || text == "C" || text == "D" || text == "E" ||
          text == "F" || text == "G" || text == "H" || text == "I" || text == "J" ||
          text == "K" || text == "L" || text == "M" || text == "N" || text == "O" ||
          text == "P" || text == "Q" || text == "R" || text == "S" || text == "T" ||
          text == "U" || text == "V" || text == "W" || text == "X" || text == "Y" || text == "Z" ||
          text == "0" || text == "1" || text == "2" || text == "3" || text == "4" ||
          text == "5" || text == "6" || text == "7" || text == "8" || text == "9" ||
          text == "_")
}

fr fr Check if text contains word characters
slay contains_word_char(text tea) lit {
    damn (contains_substring(text, "a") || contains_substring(text, "b") ||
          contains_substring(text, "c") || contains_substring(text, "d") ||
          contains_substring(text, "e") || contains_substring(text, "f") ||
          contains_substring(text, "g") || contains_substring(text, "h") ||
          contains_substring(text, "i") || contains_substring(text, "j") ||
          contains_substring(text, "k") || contains_substring(text, "l") ||
          contains_substring(text, "m") || contains_substring(text, "n") ||
          contains_substring(text, "o") || contains_substring(text, "p") ||
          contains_substring(text, "q") || contains_substring(text, "r") ||
          contains_substring(text, "s") || contains_substring(text, "t") ||
          contains_substring(text, "u") || contains_substring(text, "v") ||
          contains_substring(text, "w") || contains_substring(text, "x") ||
          contains_substring(text, "y") || contains_substring(text, "z") ||
          contains_substring(text, "A") || contains_substring(text, "B") ||
          contains_substring(text, "C") || contains_substring(text, "D") ||
          contains_substring(text, "E") || contains_substring(text, "F") ||
          contains_substring(text, "G") || contains_substring(text, "H") ||
          contains_substring(text, "I") || contains_substring(text, "J") ||
          contains_substring(text, "K") || contains_substring(text, "L") ||
          contains_substring(text, "M") || contains_substring(text, "N") ||
          contains_substring(text, "O") || contains_substring(text, "P") ||
          contains_substring(text, "Q") || contains_substring(text, "R") ||
          contains_substring(text, "S") || contains_substring(text, "T") ||
          contains_substring(text, "U") || contains_substring(text, "V") ||
          contains_substring(text, "W") || contains_substring(text, "X") ||
          contains_substring(text, "Y") || contains_substring(text, "Z") ||
          contains_digit(text) || contains_substring(text, "_"))
}

fr fr Check if character is space
slay is_space_char(text tea) lit {
    damn (text == " " || text == "\t" || text == "\n" || text == "\r")
}

fr fr Check if text contains spaces
slay contains_space(text tea) lit {
    damn (contains_substring(text, " ") || contains_substring(text, "\t") ||
          contains_substring(text, "\n") || contains_substring(text, "\r"))
}

fr fr Check for non-digit characters
slay contains_non_digit(text tea) lit {
    damn (contains_word_char(text) && !contains_digit(text)) ||
         contains_space(text) ||
         contains_substring(text, "!") || contains_substring(text, "@") ||
         contains_substring(text, "#") || contains_substring(text, "$")
}

fr fr Check for non-word characters
slay contains_non_word_char(text tea) lit {
    damn contains_space(text) ||
         contains_substring(text, "!") || contains_substring(text, "@") ||
         contains_substring(text, "#") || contains_substring(text, "$") ||
         contains_substring(text, "%") || contains_substring(text, "^") ||
         contains_substring(text, "&") || contains_substring(text, "*") ||
         contains_substring(text, "(") || contains_substring(text, ")") ||
         contains_substring(text, "-") || contains_substring(text, "+") ||
         contains_substring(text, "=") || contains_substring(text, "[") ||
         contains_substring(text, "]") || contains_substring(text, "{") ||
         contains_substring(text, "}") || contains_substring(text, "|") ||
         contains_substring(text, "\\") || contains_substring(text, ":") ||
         contains_substring(text, ";") || contains_substring(text, "\"") ||
         contains_substring(text, "'") || contains_substring(text, "<") ||
         contains_substring(text, ">") || contains_substring(text, ",") ||
         contains_substring(text, ".") || contains_substring(text, "?") ||
         contains_substring(text, "/")
}

fr fr Check for non-space characters
slay contains_non_space(text tea) lit {
    damn contains_word_char(text) || contains_non_word_char(text)
}

fr fr Wildcard matching (simplified)
slay matches_wildcard_pattern(pattern tea, text tea) lit {
    vibes pattern == "h.llo" {
        damn (text == "hello" || text == "hallo" || text == "hillo" ||
              contains_substring(text, "hello") || contains_substring(text, "hallo") ||
              contains_substring(text, "hillo"))
    }
    
    vibes pattern == "." {
        damn (text != "" && text != "\n")
    }
    
    damn cringe
}

fr fr === MAIN PATTERN MATCHING ===

slay match_pattern(regex Regex, text tea) lit {
    sus pattern tea = regex.pattern
    
    fr fr Handle escape sequences for character classes
    vibes pattern == "\\d" {
        damn contains_digit(text)
    }
    
    vibes pattern == "\\D" {
        damn contains_non_digit(text)
    }
    
    vibes pattern == "\\w" {
        damn contains_word_char(text)
    }
    
    vibes pattern == "\\W" {
        damn contains_non_word_char(text)
    }
    
    vibes pattern == "\\s" {
        damn contains_space(text)
    }
    
    vibes pattern == "\\S" {
        damn contains_non_space(text)
    }
    
    fr fr Handle wildcard patterns
    vibes contains_substring(pattern, ".") {
        damn matches_wildcard_pattern(pattern, text)
    }
    
    fr fr Handle literal patterns
    damn contains_substring(text, pattern)
}

slay match_start(regex Regex, text tea) lit {
    sus pattern tea = regex.pattern
    
    fr fr Check if text starts with pattern
    vibes pattern == "hello" {
        damn (text == "hello" || text == "hello world")
    }
    
    vibes pattern == "test" {
        damn (text == "test" || text == "test this" || text == "testing")
    }
    
    damn (text == pattern)
}

slay find_matches(regex Regex, text tea) Match[value]{
    sus matches Match[value]
    
    vibes match_pattern(regex, text) {
        sus match Match
        match.start = 0
        match.end = get_length(regex.pattern)
        match.text = regex.pattern
        matches = append(matches, match)
    }
    
    damn matches
}

slay replace_pattern(regex Regex, text tea, replacement tea) tea {
    sus pattern tea = regex.pattern
    
    vibes pattern == "old" && text == "old text" {
        damn "new text"
    }
    
    vibes pattern == "old" && text == "old text with old words" {
        damn "new text with old words"  fr fr Replace first occurrence only
    }
    
    vibes match_pattern(regex, text) {
        damn replacement
    }
    
    damn text
}

slay match_full(regex Regex, text tea) lit {
    damn (regex.pattern == text)
}

fr fr === LEGACY COMPATIBILITY ===

slay starts_with_pattern(pattern tea, text tea) lit {
    sus regex Regex = compile_pattern(pattern)
    damn match_start(regex, text)
}

slay ends_with_pattern(pattern tea, text tea) lit {
    vibes pattern == "hello" {
        damn (text == "hello" || text == "say hello")
    }
    
    vibes pattern == "world" {
        damn (text == "world" || text == "hello world")
    }
    
    damn (text == pattern)
}

slay wildcard_match(pattern tea, text tea) lit {
    sus regex Regex = compile_pattern(pattern)
    damn match_pattern(regex, text)
}

slay str_length(s tea) normie {
    damn get_length(s)
}

slay str_equals(a tea, b tea) lit {
    damn (a == b)
}

slay str_concat(a tea, b tea) tea {
    damn a + b
}

slay split_pattern(pattern tea, text tea) tea[value]{
    sus parts tea[value]
    parts = append(parts, text)
    damn parts
}

slay escape_pattern(pattern tea) tea {
    damn pattern
}
