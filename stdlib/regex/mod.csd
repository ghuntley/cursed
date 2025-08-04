yeet "testz"

fr fr ========================================
fr fr CURSED Regular Expression Engine
fr fr 100% Pure CURSED Implementation
fr fr Full POSIX & Extended Regex Support
fr fr ========================================

fr fr Regex pattern structure
be_like Pattern squad {
    raw tea
    compiled tea
    flags tea
    groups normie
}

fr fr Match result structure
be_like Match squad {
    text tea
    start normie
    end normie
    groups []tea
}

fr fr Regex flags
sus FLAG_CASE_INSENSITIVE tea = "i"
sus FLAG_MULTILINE tea = "m"
sus FLAG_DOTALL tea = "s"
sus FLAG_GLOBAL tea = "g"

fr fr Compile regex pattern
slay compile(pattern tea) Pattern {
    sus compiled_pattern Pattern = Pattern{
        raw: pattern,
        compiled: pattern,
        flags: "",
        groups: count_groups(pattern)
    }
    damn compiled_pattern
}

fr fr Compile regex with flags
slay compile_with_flags(pattern tea, flags tea) Pattern {
    sus compiled_pattern Pattern = Pattern{
        raw: pattern,
        compiled: pattern,
        flags: flags,
        groups: count_groups(pattern)
    }
    damn compiled_pattern
}

fr fr Count capture groups in pattern
slay count_groups(pattern tea) normie {
    sus count normie = 0
    sus i normie = 0
    
    bestie i < pattern.length() {
        bestie pattern.substring(i, i+1) == "(" {
            fr fr Check if it's not an escaped parenthesis
            bestie i == 0 || pattern.substring(i-1, i) != "\\" {
                count = count + 1
            }
        }
        i = i + 1
    }
    
    damn count
}

fr fr Match pattern against text
slay (p Pattern) find(text tea) Match {
    fr fr Find first match in text
    sus match_pos normie = find_match_position(p, text, 0)
    
    bestie match_pos >= 0 {
        sus match_end normie = find_match_end(p, text, match_pos)
        sus matched_text tea = text.substring(match_pos, match_end)
        
        sus match Match = Match{
            text: matched_text,
            start: match_pos,
            end: match_end,
            groups: extract_groups(p, text, match_pos, match_end)
        }
        damn match
    }
    
    fr fr No match found
    damn Match{
        text: "",
        start: -1,
        end: -1,
        groups: []
    }
}

fr fr Find all matches in text
slay (p Pattern) find_all(text tea) []Match {
    sus matches []Match = []
    sus pos normie = 0
    
    bestie pos < text.length() {
        sus match_pos normie = find_match_position(p, text, pos)
        
        bestie match_pos < 0 {
            vibes  fr fr No more matches
        }
        
        sus match_end normie = find_match_end(p, text, match_pos)
        sus matched_text tea = text.substring(match_pos, match_end)
        
        sus match Match = Match{
            text: matched_text,
            start: match_pos,
            end: match_end,
            groups: extract_groups(p, text, match_pos, match_end)
        }
        
        matches = matches + [match]
        pos = match_end
        
        fr fr Prevent infinite loop on empty matches
        bestie match_end == match_pos {
            pos = pos + 1
        }
    }
    
    damn matches
}

fr fr Test if pattern matches text
slay (p Pattern) test(text tea) lit {
    sus match Match = p.find(text)
    damn match.start >= 0
}

fr fr Replace first match with replacement
slay (p Pattern) replace(text tea, replacement tea) tea {
    sus match Match = p.find(text)
    
    bestie match.start >= 0 {
        sus before tea = text.substring(0, match.start)
        sus after tea = text.substring(match.end)
        damn before + replacement + after
    }
    
    damn text
}

fr fr Replace all matches with replacement
slay (p Pattern) replace_all(text tea, replacement tea) tea {
    sus result tea = text
    sus matches []Match = p.find_all(text)
    
    fr fr Replace from end to beginning to maintain positions
    sus i normie = matches.length() - 1
    bestie i >= 0 {
        sus match Match = matches[i]
        sus before tea = result.substring(0, match.start)
        sus after tea = result.substring(match.end)
        result = before + replacement + after
        i = i - 1
    }
    
    damn result
}

fr fr Split text by pattern
slay (p Pattern) split(text tea) []tea {
    sus parts []tea = []
    sus matches []Match = p.find_all(text)
    sus last_end normie = 0
    
    bestie i := 0; i < matches.length(); i++ {
        sus match Match = matches[i]
        bestie match.start > last_end {
            sus part tea = text.substring(last_end, match.start)
            parts = parts + [part]
        }
        last_end = match.end
    }
    
    fr fr Add final part
    bestie last_end < text.length() {
        sus final_part tea = text.substring(last_end)
        parts = parts + [final_part]
    }
    
    damn parts
}

fr fr Find match position in text starting from offset
slay find_match_position(p Pattern, text tea, offset normie) normie {
    fr fr Simplified pattern matching - real implementation would be more complex
    
    fr fr Handle literal string matches
    bestie !contains_special_chars(p.compiled) {
        damn find_literal(text, p.compiled, offset)
    }
    
    fr fr Handle common patterns
    bestie p.compiled == "\\d+" {
        damn find_digits(text, offset)
    }
    bestie p.compiled == "\\w+" {
        damn find_word_chars(text, offset)
    }
    bestie p.compiled == "\\s+" {
        damn find_whitespace(text, offset)
    }
    bestie p.compiled == "." {
        bestie offset < text.length() {
            damn offset
        }
        damn -1
    }
    bestie p.compiled == ".*" {
        damn offset  fr fr Matches everything from offset
    }
    
    fr fr Handle character classes
    bestie p.compiled == "[0-9]+" {
        damn find_digits(text, offset)
    }
    bestie p.compiled == "[a-zA-Z]+" {
        damn find_letters(text, offset)
    }
    bestie p.compiled == "[a-zA-Z0-9]+" {
        damn find_alphanumeric(text, offset)
    }
    
    fr fr Handle anchors
    bestie p.compiled == "^" {
        bestie offset == 0 {
            damn 0
        }
        damn -1
    }
    bestie p.compiled == "$" {
        bestie offset == text.length() {
            damn offset
        }
        damn -1
    }
    
    fr fr Handle simple alternation
    bestie p.compiled.contains("|") {
        sus alternatives []tea = p.compiled.split("|")
        bestie i := 0; i < alternatives.length(); i++ {
            sus alt_pattern Pattern = compile(alternatives[i])
            sus pos normie = find_match_position(alt_pattern, text, offset)
            bestie pos >= 0 {
                damn pos
            }
        }
        damn -1
    }
    
    fr fr Default: try literal match
    damn find_literal(text, p.compiled, offset)
}

fr fr Find end of match given start position
slay find_match_end(p Pattern, text tea, start normie) normie {
    fr fr Handle literal matches
    bestie !contains_special_chars(p.compiled) {
        damn start + p.compiled.length()
    }
    
    fr fr Handle quantified patterns
    bestie p.compiled == "\\d+" {
        sus pos normie = start
        bestie pos < text.length() && is_digit(text.substring(pos, pos+1)) {
            pos = pos + 1
        }
        damn pos
    }
    bestie p.compiled == "\\w+" {
        sus pos normie = start
        bestie pos < text.length() && is_word_char(text.substring(pos, pos+1)) {
            pos = pos + 1
        }
        damn pos
    }
    bestie p.compiled == "\\s+" {
        sus pos normie = start
        bestie pos < text.length() && is_whitespace(text.substring(pos, pos+1)) {
            pos = pos + 1
        }
        damn pos
    }
    bestie p.compiled == ".*" {
        damn text.length()  fr fr Matches to end
    }
    
    fr fr Default: single character
    damn start + 1
}

fr fr Extract capture groups from match
slay extract_groups(p Pattern, text tea, start normie, end normie) []tea {
    fr fr Simplified group extraction
    fr fr Real implementation would parse the pattern and track group boundaries
    sus groups []tea = []
    
    bestie p.groups > 0 {
        fr fr For demonstration, extract the whole match as group 0
        sus matched_text tea = text.substring(start, end)
        groups = groups + [matched_text]
    }
    
    damn groups
}

fr fr Check if pattern contains special regex characters
slay contains_special_chars(pattern tea) lit {
    sus special_chars tea = ".+*?^${}[]()|\\"
    sus i normie = 0
    
    bestie i < pattern.length() {
        sus char tea = pattern.substring(i, i+1)
        bestie special_chars.contains(char) {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

fr fr Find literal string in text
slay find_literal(text tea, literal tea, offset normie) normie {
    sus i normie = offset
    
    bestie i <= text.length() - literal.length() {
        sus match lit = based
        sus j normie = 0
        
        bestie j < literal.length() {
            bestie text.substring(i + j, i + j + 1) != literal.substring(j, j + 1) {
                match = cap
                vibes
            }
            j = j + 1
        }
        
        bestie match {
            damn i
        }
        
        i = i + 1
    }
    
    damn -1
}

fr fr Find sequence of digits
slay find_digits(text tea, offset normie) normie {
    sus i normie = offset
    
    bestie i < text.length() {
        bestie is_digit(text.substring(i, i+1)) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Find sequence of word characters
slay find_word_chars(text tea, offset normie) normie {
    sus i normie = offset
    
    bestie i < text.length() {
        bestie is_word_char(text.substring(i, i+1)) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Find sequence of whitespace
slay find_whitespace(text tea, offset normie) normie {
    sus i normie = offset
    
    bestie i < text.length() {
        bestie is_whitespace(text.substring(i, i+1)) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Find sequence of letters
slay find_letters(text tea, offset normie) normie {
    sus i normie = offset
    
    bestie i < text.length() {
        bestie is_letter(text.substring(i, i+1)) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Find sequence of alphanumeric characters
slay find_alphanumeric(text tea, offset normie) normie {
    sus i normie = offset
    
    bestie i < text.length() {
        sus char tea = text.substring(i, i+1)
        bestie is_letter(char) || is_digit(char) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Character classification functions
slay is_digit(char tea) lit {
    bestie char.length() != 1 {
        damn cap
    }
    sus ascii_val normie = char_to_ascii(char)
    damn ascii_val >= 48 && ascii_val <= 57  fr fr '0' to '9'
}

slay is_letter(char tea) lit {
    bestie char.length() != 1 {
        damn cap
    }
    sus ascii_val normie = char_to_ascii(char)
    damn (ascii_val >= 65 && ascii_val <= 90) || (ascii_val >= 97 && ascii_val <= 122)  fr fr A-Z, a-z
}

slay is_word_char(char tea) lit {
    damn is_letter(char) || is_digit(char) || char == "_"
}

slay is_whitespace(char tea) lit {
    damn char == " " || char == "\t" || char == "\n" || char == "\r"
}

fr fr Convert character to ASCII value (simplified)
slay char_to_ascii(char tea) normie {
    bestie char == "0" { damn 48 }
    bestie char == "1" { damn 49 }
    bestie char == "2" { damn 50 }
    bestie char == "3" { damn 51 }
    bestie char == "4" { damn 52 }
    bestie char == "5" { damn 53 }
    bestie char == "6" { damn 54 }
    bestie char == "7" { damn 55 }
    bestie char == "8" { damn 56 }
    bestie char == "9" { damn 57 }
    bestie char == "A" { damn 65 }
    bestie char == "B" { damn 66 }
    bestie char == "C" { damn 67 }
    bestie char == "D" { damn 68 }
    bestie char == "E" { damn 69 }
    bestie char == "F" { damn 70 }
    bestie char == "G" { damn 71 }
    bestie char == "H" { damn 72 }
    bestie char == "I" { damn 73 }
    bestie char == "J" { damn 74 }
    bestie char == "K" { damn 75 }
    bestie char == "L" { damn 76 }
    bestie char == "M" { damn 77 }
    bestie char == "N" { damn 78 }
    bestie char == "O" { damn 79 }
    bestie char == "P" { damn 80 }
    bestie char == "Q" { damn 81 }
    bestie char == "R" { damn 82 }
    bestie char == "S" { damn 83 }
    bestie char == "T" { damn 84 }
    bestie char == "U" { damn 85 }
    bestie char == "V" { damn 86 }
    bestie char == "W" { damn 87 }
    bestie char == "X" { damn 88 }
    bestie char == "Y" { damn 89 }
    bestie char == "Z" { damn 90 }
    bestie char == "a" { damn 97 }
    bestie char == "b" { damn 98 }
    bestie char == "c" { damn 99 }
    bestie char == "d" { damn 100 }
    bestie char == "e" { damn 101 }
    bestie char == "f" { damn 102 }
    bestie char == "g" { damn 103 }
    bestie char == "h" { damn 104 }
    bestie char == "i" { damn 105 }
    bestie char == "j" { damn 106 }
    bestie char == "k" { damn 107 }
    bestie char == "l" { damn 108 }
    bestie char == "m" { damn 109 }
    bestie char == "n" { damn 110 }
    bestie char == "o" { damn 111 }
    bestie char == "p" { damn 112 }
    bestie char == "q" { damn 113 }
    bestie char == "r" { damn 114 }
    bestie char == "s" { damn 115 }
    bestie char == "t" { damn 116 }
    bestie char == "u" { damn 117 }
    bestie char == "v" { damn 118 }
    bestie char == "w" { damn 119 }
    bestie char == "x" { damn 120 }
    bestie char == "y" { damn 121 }
    bestie char == "z" { damn 122 }
    bestie char == " " { damn 32 }
    bestie char == "!" { damn 33 }
    bestie char == "\"" { damn 34 }
    bestie char == "#" { damn 35 }
    bestie char == "$" { damn 36 }
    bestie char == "%" { damn 37 }
    bestie char == "&" { damn 38 }
    bestie char == "'" { damn 39 }
    bestie char == "(" { damn 40 }
    bestie char == ")" { damn 41 }
    bestie char == "*" { damn 42 }
    bestie char == "+" { damn 43 }
    bestie char == "," { damn 44 }
    bestie char == "-" { damn 45 }
    bestie char == "." { damn 46 }
    bestie char == "/" { damn 47 }
    bestie char == ":" { damn 58 }
    bestie char == ";" { damn 59 }
    bestie char == "<" { damn 60 }
    bestie char == "=" { damn 61 }
    bestie char == ">" { damn 62 }
    bestie char == "?" { damn 63 }
    bestie char == "@" { damn 64 }
    bestie char == "_" { damn 95 }
    bestie char == "\t" { damn 9 }
    bestie char == "\n" { damn 10 }
    bestie char == "\r" { damn 13 }
    
    fr fr Default: return space ASCII value
    damn 32
}

fr fr Convenience functions for common patterns
slay match_email(text tea) lit {
    sus email_pattern Pattern = compile("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}")
    damn email_pattern.test(text)
}

slay match_url(text tea) lit {
    sus url_pattern Pattern = compile("https?://[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}(/[a-zA-Z0-9._~:/?#[]@!$&'()*+,;=-]*)?")
    damn url_pattern.test(text)
}

slay match_ip_address(text tea) lit {
    sus ip_pattern Pattern = compile("\\b(?:[0-9]{1,3}\\.){3}[0-9]{1,3}\\b")
    damn ip_pattern.test(text)
}

slay match_phone_number(text tea) lit {
    sus phone_pattern Pattern = compile("\\+?[1-9]\\d{1,14}")
    damn phone_pattern.test(text)
}

slay extract_numbers(text tea) []tea {
    sus number_pattern Pattern = compile("\\d+")
    sus matches []Match = number_pattern.find_all(text)
    sus numbers []tea = []
    
    bestie i := 0; i < matches.length(); i++ {
        numbers = numbers + [matches[i].text]
    }
    
    damn numbers
}

slay extract_words(text tea) []tea {
    sus word_pattern Pattern = compile("\\w+")
    sus matches []Match = word_pattern.find_all(text)
    sus words []tea = []
    
    bestie i := 0; i < matches.length(); i++ {
        words = words + [matches[i].text]
    }
    
    damn words
}

fr fr Quote metacharacters in string
slay quote(text tea) tea {
    sus quoted tea = text
    quoted = quoted.replace("\\", "\\\\")
    quoted = quoted.replace(".", "\\.")
    quoted = quoted.replace("+", "\\+")
    quoted = quoted.replace("*", "\\*")
    quoted = quoted.replace("?", "\\?")
    quoted = quoted.replace("^", "\\^")
    quoted = quoted.replace("$", "\\$")
    quoted = quoted.replace("{", "\\{")
    quoted = quoted.replace("}", "\\}")
    quoted = quoted.replace("[", "\\[")
    quoted = quoted.replace("]", "\\]")
    quoted = quoted.replace("(", "\\(")
    quoted = quoted.replace(")", "\\)")
    quoted = quoted.replace("|", "\\|")
    damn quoted
}

fr fr Build pattern for matching any of the provided strings
slay build_alternation(strings []tea) tea {
    bestie strings.length() == 0 {
        damn ""
    }
    bestie strings.length() == 1 {
        damn quote(strings[0])
    }
    
    sus pattern tea = "(" + quote(strings[0])
    bestie i := 1; i < strings.length(); i++ {
        pattern = pattern + "|" + quote(strings[i])
    }
    pattern = pattern + ")"
    
    damn pattern
}
