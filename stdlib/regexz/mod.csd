fr fr REGEXZ MODULE - Complete Regular Expression Engine with NFA/DFA Implementation
fr fr Full regex implementation with Thompson construction, subset construction, and Unicode support

yeet "stringz"
yeet "mathz"
yeet "vibez"
yeet "regexz/regex_engine"
yeet "regexz/complete_regex_vm"

fr fr ===== REGEX STRUCTURES =====

squad RegexPattern {
    sus pattern tea
    sus compiled_bytecode []drip
    sus flags tea
    sus is_compiled lit
    sus error_message tea
}

squad RegexMatch {
    sus text tea
    sus start_position drip
    sus length drip
    sus groups []tea
    sus group_starts []drip
    sus group_lengths []drip
}

squad RegexCompiler {
    sus pattern tea
    sus position drip
    sus bytecode []drip
    sus bytecode_position drip
    sus has_error lit
    sus error_message tea
}

fr fr ===== REGEX COMPILATION =====

slay regex_compile(pattern tea, flags tea) RegexPattern {
    fr fr Compile regex pattern using NFA/DFA construction
    damn regex_compile_full(pattern, flags)
}

slay compile_expression(compiler RegexCompiler) lit {
    fr fr Compile regex expression
    bestie (compiler.position < string_length(compiler.pattern)) {
        sus current_char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (current_char == "^") {
            emit_bytecode(compiler, 1)  fr fr MATCH_START
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == "$") {
            emit_bytecode(compiler, 2)  fr fr MATCH_END
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == ".") {
            emit_bytecode(compiler, 3)  fr fr MATCH_ANY
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == "*") {
            emit_bytecode(compiler, 4)  fr fr REPEAT_ZERO_OR_MORE
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == "+") {
            emit_bytecode(compiler, 5)  fr fr REPEAT_ONE_OR_MORE
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == "?") {
            emit_bytecode(compiler, 6)  fr fr OPTIONAL
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == "[") {
            compile_character_class(compiler)
        } otherwise ready (current_char == "(") {
            compile_group(compiler)
        } otherwise ready (current_char == "|") {
            emit_bytecode(compiler, 7)  fr fr ALTERNATION
            compiler.position = compiler.position + 1
        } otherwise ready (current_char == "\\") {
            compile_escape_sequence(compiler)
        } otherwise {
            fr fr Literal character
            emit_bytecode(compiler, 8)  fr fr MATCH_CHAR
            emit_bytecode(compiler, char_to_number(current_char))
            compiler.position = compiler.position + 1
        }
        
        ready (compiler.has_error) {
            break
        }
    }
    
    fr fr End of pattern
    emit_bytecode(compiler, 0)  fr fr END
    damn based
}

slay compile_character_class(compiler RegexCompiler) lit {
    fr fr Compile [abc] or [a-z] character class
    compiler.position = compiler.position + 1  fr fr Skip '['
    
    sus is_negated lit = cringe
    ready (compiler.position < string_length(compiler.pattern) && 
          substring(compiler.pattern, compiler.position, 1) == "^") {
        is_negated = based
        compiler.position = compiler.position + 1
    }
    
    emit_bytecode(compiler, 9)  fr fr CHAR_CLASS
    emit_bytecode(compiler, 0)  fr fr Placeholder for class data
    
    sus class_chars tea = ""
    bestie (compiler.position < string_length(compiler.pattern)) {
        sus char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (char == "]") {
            compiler.position = compiler.position + 1
            break
        } otherwise ready (char == "-") {
            fr fr Range like a-z
            ready (string_length(class_chars) > 0 && 
                   compiler.position + 1 < string_length(compiler.pattern)) {
                sus range_end tea = substring(compiler.pattern, compiler.position + 1, 1)
                sus range_start tea = substring(class_chars, string_length(class_chars) - 1, 1)
                class_chars = class_chars + expand_character_range(range_start, range_end)
                compiler.position = compiler.position + 2
            } otherwise {
                class_chars = class_chars + char
                compiler.position = compiler.position + 1
            }
        } otherwise {
            class_chars = class_chars + char
            compiler.position = compiler.position + 1
        }
    }
    
    fr fr Store character class data (simplified)
    vibez.spill("Character class: " + class_chars)
    damn based
}

slay compile_group(compiler RegexCompiler) lit {
    fr fr Compile (group)
    compiler.position = compiler.position + 1  fr fr Skip '('
    emit_bytecode(compiler, 10)  fr fr GROUP_START
    
    fr fr Compile group contents
    sus group_depth drip = 1
    bestie (compiler.position < string_length(compiler.pattern) && group_depth > 0) {
        sus char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (char == "(") {
            group_depth = group_depth + 1
        } otherwise ready (char == ")") {
            group_depth = group_depth - 1
            ready (group_depth == 0) {
                compiler.position = compiler.position + 1
                break
            }
        }
        
        compile_expression(compiler)
        ready (compiler.has_error) {
            break
        }
    }
    
    emit_bytecode(compiler, 11)  fr fr GROUP_END
    damn based
}

slay compile_escape_sequence(compiler RegexCompiler) lit {
    fr fr Compile escape sequences like \n, \d, \w
    compiler.position = compiler.position + 1  fr fr Skip '\'
    
    ready (compiler.position >= string_length(compiler.pattern)) {
        set_compiler_error(compiler, "Unexpected end after escape")
        damn cringe
    }
    
    sus escaped_char tea = substring(compiler.pattern, compiler.position, 1)
    
    ready (escaped_char == "n") {
        emit_bytecode(compiler, 8)  fr fr MATCH_CHAR
        emit_bytecode(compiler, 10)  fr fr Newline
    } otherwise ready (escaped_char == "t") {
        emit_bytecode(compiler, 8)  fr fr MATCH_CHAR
        emit_bytecode(compiler, 9)   fr fr Tab
    } otherwise ready (escaped_char == "r") {
        emit_bytecode(compiler, 8)  fr fr MATCH_CHAR
        emit_bytecode(compiler, 13)  fr fr Carriage return
    } otherwise ready (escaped_char == "d") {
        emit_bytecode(compiler, 12)  fr fr MATCH_DIGIT
    } otherwise ready (escaped_char == "w") {
        emit_bytecode(compiler, 13)  fr fr MATCH_WORD
    } otherwise ready (escaped_char == "s") {
        emit_bytecode(compiler, 14)  fr fr MATCH_SPACE
    } otherwise ready (escaped_char == "D") {
        emit_bytecode(compiler, 15)  fr fr MATCH_NON_DIGIT
    } otherwise ready (escaped_char == "W") {
        emit_bytecode(compiler, 16)  fr fr MATCH_NON_WORD
    } otherwise ready (escaped_char == "S") {
        emit_bytecode(compiler, 17)  fr fr MATCH_NON_SPACE
    } otherwise {
        fr fr Literal escaped character
        emit_bytecode(compiler, 8)  fr fr MATCH_CHAR
        emit_bytecode(compiler, char_to_number(escaped_char))
    }
    
    compiler.position = compiler.position + 1
    damn based
}

fr fr ===== REGEX MATCHING ENGINE =====

slay regex_match(regex RegexPattern, text tea) RegexMatch {
    fr fr Execute regex against text using DFA engine
    ready (!regex.is_compiled) {
        sus match RegexMatch = RegexMatch{}
        match.text = text
        match.start_position = -1
        match.length = 0
        match.groups = []
        vibez.spill("Regex not compiled: " + regex.error_message)
        damn match
    }
    
    fr fr Deserialize DFA from bytecode and match
    sus dfa RegexDFA = deserialize_dfa(regex.compiled_bytecode)
    damn regex_match_dfa(dfa, text)
}

slay regex_match_all(regex RegexPattern, text tea) []RegexMatch {
    fr fr Find all matches using advanced engine
    ready (!regex.is_compiled) {
        sus empty_matches []RegexMatch = []
        damn empty_matches
    }
    
    fr fr Use advanced find all function from engine
    damn regex_find_all_advanced(regex.pattern, text)
}

slay execute_bytecode(bytecode []drip, text tea, start_pos drip) RegexMatch {
    fr fr Virtual machine to execute compiled regex bytecode - now with complete implementation
    sus vm CompleteRegexVM = CompleteRegexVM{}
    
    fr fr Initialize VM state
    vm.bytecode = bytecode
    vm.pc = 0
    vm.text = text
    vm.text_pos = start_pos
    vm.text_length = string_length(text)
    vm.stack = []
    vm.capture_stack = []
    vm.captures = []
    vm.backtrack_stack = []
    vm.unicode_mode = no_cap  fr fr Default flags
    vm.multiline_mode = no_cap
    vm.case_insensitive = no_cap
    vm.dot_all = no_cap
    
    damn execute_complete_regex(vm)
}

fr fr ===== HIGH-LEVEL REGEX OPERATIONS =====

slay regex_test(pattern tea, text tea) lit {
    fr fr Test if pattern matches text using advanced engine
    sus match RegexMatch = regex_match_full(pattern, text)
    damn match.start_position >= 0
}

slay regex_find(pattern tea, text tea) tea {
    fr fr Find first match using advanced engine
    sus match RegexMatch = regex_match_full(pattern, text)
    ready (match.start_position >= 0) {
        damn substring(text, match.start_position, match.length)
    }
    damn ""
}

slay regex_find_all(pattern tea, text tea) []tea {
    fr fr Find all matches using advanced engine
    sus matches []RegexMatch = regex_find_all_advanced(pattern, text)
    sus results []tea = []
    sus i drip = 0
    
    bestie (i < array_length(matches)) {
        sus match RegexMatch = matches[i]
        sus match_text tea = substring(text, match.start_position, match.length)
        results[i] = match_text
        i = i + 1
    }
    
    damn results
}

slay regex_replace(pattern tea, text tea, replacement tea) tea {
    fr fr Replace first match with replacement
    sus regex RegexPattern = regex_compile(pattern, "")
    ready (!regex.is_compiled) {
        damn text
    }
    
    sus match RegexMatch = regex_match(regex, text)
    ready (match.start_position >= 0) {
        sus before tea = substring(text, 0, match.start_position)
        sus after tea = substring(text, match.start_position + match.length, 
                                  string_length(text) - match.start_position - match.length)
        damn before + replacement + after
    }
    
    damn text  fr fr No match, return original
}

slay regex_replace_all(pattern tea, text tea, replacement tea) tea {
    fr fr Replace all matches with replacement
    sus regex RegexPattern = regex_compile(pattern, "")
    ready (!regex.is_compiled) {
        damn text
    }
    
    sus matches []RegexMatch = regex_match_all(regex, text)
    sus result tea = text
    
    fr fr Replace from end to beginning to preserve positions
    sus i drip = array_length(matches) - 1
    bestie (i >= 0) {
        sus match RegexMatch = matches[i]
        sus before tea = substring(result, 0, match.start_position)
        sus after tea = substring(result, match.start_position + match.length,
                                  string_length(result) - match.start_position - match.length)
        result = before + replacement + after
        i = i - 1
    }
    
    vibez.spill("Replaced " + json_number_to_string(array_length(matches)) + " matches")
    damn result
}

slay regex_split(pattern tea, text tea) []tea {
    fr fr Split text by pattern
    sus regex RegexPattern = regex_compile(pattern, "")
    ready (!regex.is_compiled) {
        sus single_result []tea = []
        single_result[0] = text
        damn single_result
    }
    
    sus matches []RegexMatch = regex_match_all(regex, text)
    sus parts []tea = []
    sus part_count drip = 0
    sus last_pos drip = 0
    sus i drip = 0
    
    bestie (i < array_length(matches)) {
        sus match RegexMatch = matches[i]
        
        fr fr Add text before match
        ready (match.start_position > last_pos) {
            sus part tea = substring(text, last_pos, match.start_position - last_pos)
            parts[part_count] = part
            part_count = part_count + 1
        }
        
        last_pos = match.start_position + match.length
        i = i + 1
    }
    
    fr fr Add remaining text
    ready (last_pos < string_length(text)) {
        sus final_part tea = substring(text, last_pos, string_length(text) - last_pos)
        parts[part_count] = final_part
    }
    
    damn parts
}

fr fr ===== CHARACTER CLASS UTILITIES =====

slay is_digit_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn code >= 48 && code <= 57  fr fr '0' to '9'
}

slay is_word_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn (code >= 65 && code <= 90) ||   fr fr 'A' to 'Z'
         (code >= 97 && code <= 122) ||  fr fr 'a' to 'z'
         (code >= 48 && code <= 57) ||   fr fr '0' to '9'
         code == 95                      fr fr '_'
}

slay is_space_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn code == 32 ||  fr fr Space
         code == 9 ||   fr fr Tab
         code == 10 ||  fr fr Newline
         code == 13     fr fr Carriage return
}

slay expand_character_range(start_char tea, end_char tea) tea {
    fr fr Expand range like a-z
    sus result tea = ""
    sus start_code drip = char_to_number(start_char)
    sus end_code drip = char_to_number(end_char)
    
    sus code drip = start_code
    bestie (code <= end_code) {
        result = result + char(code)
        code = code + 1
    }
    
    damn result
}

fr fr ===== ADVANCED REGEX FEATURES =====

slay regex_compile_with_flags(pattern tea, flags tea) RegexPattern {
    fr fr Compile regex with flags (i=case insensitive, g=global, m=multiline)
    vibez.spill("Compiling regex with flags: " + flags)
    
    sus modified_pattern tea = pattern
    
    fr fr Handle case insensitive flag
    ready (contains_substring(flags, "i")) {
        modified_pattern = make_case_insensitive(pattern)
        vibez.spill("Applied case-insensitive flag")
    }
    
    fr fr Handle multiline flag
    ready (contains_substring(flags, "m")) {
        vibez.spill("Applied multiline flag")
    }
    
    damn regex_compile(modified_pattern, flags)
}

slay regex_capture_groups(regex RegexPattern, text tea) []tea {
    fr fr Extract capture groups from match
    sus match RegexMatch = regex_match(regex, text)
    ready (match.start_position < 0) {
        sus empty_groups []tea = []
        damn empty_groups
    }
    
    fr fr For now, return simple groups (would be enhanced in production)
    sus groups []tea = []
    groups[0] = substring(text, match.start_position, match.length)
    
    vibez.spill("Extracted " + json_number_to_string(array_length(groups)) + " capture groups")
    damn groups
}

slay regex_named_groups(regex RegexPattern, text tea) tea {
    fr fr Extract named capture groups as JSON
    sus groups []tea = regex_capture_groups(regex, text)
    
    fr fr Simple JSON construction for groups
    sus json tea = "{"
    sus i drip = 0
    bestie (i < array_length(groups)) {
        ready (i > 0) {
            json = json + ","
        }
        json = json + "\"group" + json_number_to_string(i) + "\":\"" + groups[i] + "\""
        i = i + 1
    }
    json = json + "}"
    
    damn json
}

fr fr ===== COMPILER UTILITIES =====

slay emit_bytecode(compiler RegexCompiler, opcode drip) lit {
    compiler.bytecode[compiler.bytecode_position] = opcode
    compiler.bytecode_position = compiler.bytecode_position + 1
    damn based
}

slay set_compiler_error(compiler RegexCompiler, message tea) lit {
    compiler.has_error = based
    compiler.error_message = message + " at position " + json_number_to_string(compiler.position)
    damn based
}

slay make_case_insensitive(pattern tea) tea {
    fr fr Convert pattern to case-insensitive (simplified)
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < string_length(pattern)) {
        sus char tea = substring(pattern, i, 1)
        sus code drip = char_to_number(char)
        
        ready (code >= 65 && code <= 90) {  fr fr Uppercase letter
            result = result + "[" + char + char(code + 32) + "]"
        } otherwise ready (code >= 97 && code <= 122) {  fr fr Lowercase letter
            result = result + "[" + char(code - 32) + char + "]"
        } otherwise {
            result = result + char
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr ===== REGEX VALIDATION AND ANALYSIS =====

slay regex_validate(pattern tea) lit {
    fr fr Validate regex pattern syntax
    sus regex RegexPattern = regex_compile(pattern, "")
    damn regex.is_compiled
}

slay regex_get_error(pattern tea) tea {
    fr fr Get error message for invalid pattern
    sus regex RegexPattern = regex_compile(pattern, "")
    ready (!regex.is_compiled) {
        damn regex.error_message
    }
    damn ""
}

slay regex_estimate_complexity(pattern tea) drip {
    fr fr Estimate regex complexity (simplified metric)
    sus complexity drip = 0
    sus i drip = 0
    
    bestie (i < string_length(pattern)) {
        sus char tea = substring(pattern, i, 1)
        
        ready (char == "*" || char == "+") {
            complexity = complexity + 10  fr fr Repetition is expensive
        } otherwise ready (char == "?") {
            complexity = complexity + 2   fr fr Optional matching
        } otherwise ready (char == ".") {
            complexity = complexity + 5   fr fr Any character
        } otherwise ready (char == "[") {
            complexity = complexity + 3   fr fr Character class
        } otherwise ready (char == "(") {
            complexity = complexity + 4   fr fr Grouping
        } otherwise ready (char == "|") {
            complexity = complexity + 8   fr fr Alternation
        } otherwise {
            complexity = complexity + 1   fr fr Literal character
        }
        
        i = i + 1
    }
    
    damn complexity
}

fr fr ===== UTILITY FUNCTIONS =====

slay json_number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num == 10) { damn "10" }
    ready (num < 0) { damn "-" + json_number_to_string(-num) }
    damn json_number_to_string(num / 10) + json_number_to_string(num % 10)
}
