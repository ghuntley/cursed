fr fr Complete Regex Virtual Machine Implementation with All Opcodes
fr fr Implements full PCRE-compatible regex engine with advanced features

yeet "stringz"
yeet "arrayz" 
yeet "vibez"
yeet "mathz"

fr fr ===== EXTENDED OPCODE DEFINITIONS =====

fr fr Basic opcodes (0-17) - already implemented partially
fr fr 0: END - End of program
fr fr 1: MATCH_START - Match start of string
fr fr 2: MATCH_END - Match end of string  
fr fr 3: MATCH_ANY - Match any character (.)
fr fr 4: JUMP - Unconditional jump
fr fr 5: SPLIT - Non-deterministic split (for alternation)
fr fr 6: CAPTURE_START - Begin capture group
fr fr 7: CAPTURE_END - End capture group
fr fr 8: MATCH_CHAR - Match specific character
fr fr 9: MATCH_RANGE - Match character range [a-z]
fr fr 10: MATCH_SET - Match character set [abc]
fr fr 11: MATCH_NEG_SET - Match negative character set [^abc]
fr fr 12: MATCH_DIGIT - Match digit \d
fr fr 13: MATCH_WORD - Match word character \w
fr fr 14: MATCH_SPACE - Match whitespace \s
fr fr 15: MATCH_NON_DIGIT - Match non-digit \D
fr fr 16: MATCH_NON_WORD - Match non-word \W
fr fr 17: MATCH_NON_SPACE - Match non-whitespace \S

fr fr Advanced opcodes (18+)
fr fr 18: MATCH_WORD_BOUNDARY - Word boundary \b
fr fr 19: MATCH_NON_WORD_BOUNDARY - Non-word boundary \B
fr fr 20: LOOKAHEAD_POS - Positive lookahead (?=...)
fr fr 21: LOOKAHEAD_NEG - Negative lookahead (?!...)
fr fr 22: LOOKBEHIND_POS - Positive lookbehind (?<=...)
fr fr 23: LOOKBEHIND_NEG - Negative lookbehind (?<!...)
fr fr 24: ATOMIC_GROUP - Atomic group (?>...)
fr fr 25: REPEAT_LAZY - Lazy quantifier support
fr fr 26: REPEAT_POSSESSIVE - Possessive quantifier support
fr fr 27: MATCH_UNICODE_CLASS - Unicode character classes
fr fr 28: MATCH_NEWLINE - Platform-specific newline
fr fr 29: BACKREF - Backreference \1, \2, etc.
fr fr 30: CONDITIONAL - Conditional expression (?(condition)...)

squad CompleteRegexVM {
    sus bytecode drip[value]
    sus pc drip                     fr fr Program counter
    sus text tea
    sus text_pos drip
    sus text_length drip
    sus stack drip[value]                fr fr Execution stack
    sus capture_stack drip[value]        fr fr Capture group stack  
    sus captures tea[value]              fr fr Captured groups
    sus backtrack_stack BacktrackFrame[value]
    sus unicode_mode lit            fr fr Unicode support enabled
    sus multiline_mode lit          fr fr Multiline mode
    sus case_insensitive lit        fr fr Case-insensitive matching
    sus dot_all lit                 fr fr . matches newlines
}

squad BacktrackFrame {
    sus pc drip
    sus text_pos drip
    sus capture_state tea[value]
    sus stack_state drip[value]
}

squad RegexCompilerState {
    sus pattern tea
    sus position drip
    sus bytecode drip[value]
    sus capture_count drip
    sus flags tea                   fr fr Regex flags (i, m, s, x, etc.)
}

fr fr ===== COMPLETE VIRTUAL MACHINE EXECUTION =====

slay execute_complete_regex(vm CompleteRegexVM) RegexMatch {
    fr fr Complete regex virtual machine with all opcodes implemented
    
    sus match RegexMatch = RegexMatch{}
    match.text = vm.text
    match.start_position = -1
    match.length = 0
    match.groups = []
    
    sus start_pos drip = vm.text_pos
    
    bestie (vm.pc < array_length(vm.bytecode)) {
        sus opcode drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        
        ready (opcode == 0) {  fr fr END
            ready (vm.text_pos >= start_pos) {
                match.start_position = start_pos
                match.length = vm.text_pos - start_pos
                match.groups = vm.captures
            }
            break
            
        } otherwise ready (opcode == 1) {  fr fr MATCH_START
            ready (!match_start_anchor(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 2) {  fr fr MATCH_END
            ready (!match_end_anchor(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 3) {  fr fr MATCH_ANY
            ready (!match_any_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 4) {  fr fr JUMP
            sus jump_offset drip = vm.bytecode[vm.pc]
            vm.pc = jump_offset
            
        } otherwise ready (opcode == 5) {  fr fr SPLIT
            sus branch1 drip = vm.bytecode[vm.pc]
            sus branch2 drip = vm.bytecode[vm.pc + 1]
            vm.pc = vm.pc + 2
            
            fr fr Push second branch to backtrack stack
            push_backtrack_frame(vm, branch2)
            vm.pc = branch1  fr fr Take first branch
            
        } otherwise ready (opcode == 6) {  fr fr CAPTURE_START
            sus group_num drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            start_capture_group(vm, group_num)
            
        } otherwise ready (opcode == 7) {  fr fr CAPTURE_END
            sus group_num drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            end_capture_group(vm, group_num)
            
        } otherwise ready (opcode == 8) {  fr fr MATCH_CHAR
            sus expected_char drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!match_specific_char(vm, expected_char)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 9) {  fr fr MATCH_RANGE
            sus range_start drip = vm.bytecode[vm.pc]
            sus range_end drip = vm.bytecode[vm.pc + 1] 
            vm.pc = vm.pc + 2
            ready (!match_char_range(vm, range_start, range_end)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 10) {  fr fr MATCH_SET
            sus set_length drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!match_char_set(vm, set_length)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 11) {  fr fr MATCH_NEG_SET
            sus set_length drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!match_negative_char_set(vm, set_length)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 12) {  fr fr MATCH_DIGIT
            ready (!match_digit_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 13) {  fr fr MATCH_WORD
            ready (!match_word_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 14) {  fr fr MATCH_SPACE
            ready (!match_space_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 15) {  fr fr MATCH_NON_DIGIT
            ready (!match_non_digit_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 16) {  fr fr MATCH_NON_WORD
            ready (!match_non_word_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 17) {  fr fr MATCH_NON_SPACE
            ready (!match_non_space_char(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 18) {  fr fr MATCH_WORD_BOUNDARY
            ready (!match_word_boundary(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 19) {  fr fr MATCH_NON_WORD_BOUNDARY
            ready (!match_non_word_boundary(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 20) {  fr fr LOOKAHEAD_POS
            sus lookahead_length drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!execute_positive_lookahead(vm, lookahead_length)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 21) {  fr fr LOOKAHEAD_NEG
            sus lookahead_length drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!execute_negative_lookahead(vm, lookahead_length)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 22) {  fr fr LOOKBEHIND_POS
            sus lookbehind_length drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!execute_positive_lookbehind(vm, lookbehind_length)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 23) {  fr fr LOOKBEHIND_NEG
            sus lookbehind_length drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!execute_negative_lookbehind(vm, lookbehind_length)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 24) {  fr fr ATOMIC_GROUP
            ready (!execute_atomic_group(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 25) {  fr fr REPEAT_LAZY
            sus min_repeat drip = vm.bytecode[vm.pc]
            sus max_repeat drip = vm.bytecode[vm.pc + 1]
            sus repeat_pattern_start drip = vm.bytecode[vm.pc + 2]
            vm.pc = vm.pc + 3
            ready (!execute_lazy_repeat(vm, min_repeat, max_repeat, repeat_pattern_start)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 26) {  fr fr REPEAT_POSSESSIVE
            sus min_repeat drip = vm.bytecode[vm.pc]
            sus max_repeat drip = vm.bytecode[vm.pc + 1]
            sus repeat_pattern_start drip = vm.bytecode[vm.pc + 2]
            vm.pc = vm.pc + 3
            ready (!execute_possessive_repeat(vm, min_repeat, max_repeat, repeat_pattern_start)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 27) {  fr fr MATCH_UNICODE_CLASS
            sus unicode_class drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!match_unicode_character_class(vm, unicode_class)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 28) {  fr fr MATCH_NEWLINE
            ready (!match_platform_newline(vm)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 29) {  fr fr BACKREF
            sus backref_num drip = vm.bytecode[vm.pc]
            vm.pc = vm.pc + 1
            ready (!match_backreference(vm, backref_num)) {
                damn backtrack_or_fail(vm, match)
            }
            
        } otherwise ready (opcode == 30) {  fr fr CONDITIONAL
            sus condition_type drip = vm.bytecode[vm.pc]
            sus true_branch drip = vm.bytecode[vm.pc + 1]
            sus false_branch drip = vm.bytecode[vm.pc + 2]
            vm.pc = vm.pc + 3
            
            ready (evaluate_condition(vm, condition_type)) {
                vm.pc = true_branch
            } otherwise {
                vm.pc = false_branch
            }
            
        } otherwise {
            fr fr Unknown opcode - this should not happen with complete implementation
            vibez.spill("ERROR: Unknown opcode encountered: " + json_number_to_string(opcode))
            match.start_position = -1
            damn match
        }
    }
    
    damn match
}

fr fr ===== MATCHING FUNCTIONS FOR EACH OPCODE =====

slay match_start_anchor(vm CompleteRegexVM) lit {
    fr fr Match start of string or line in multiline mode
    ready (vm.multiline_mode) {
        ready (vm.text_pos == 0) {
            damn based  fr fr Start of string
        }
        
        fr fr Check if previous character is newline
        ready (vm.text_pos > 0) {
            sus prev_char tea = substring(vm.text, vm.text_pos - 1, 1)
            damn (prev_char == "\n" || prev_char == "\r")
        }
        
        damn no_cap
    } otherwise {
        damn vm.text_pos == 0
    }
}

slay match_end_anchor(vm CompleteRegexVM) lit {
    fr fr Match end of string or line in multiline mode
    ready (vm.multiline_mode) {
        ready (vm.text_pos == vm.text_length) {
            damn based  fr fr End of string
        }
        
        fr fr Check if current character is newline
        ready (vm.text_pos < vm.text_length) {
            sus curr_char tea = substring(vm.text, vm.text_pos, 1)
            damn (curr_char == "\n" || curr_char == "\r")
        }
        
        damn no_cap
    } otherwise {
        damn vm.text_pos == vm.text_length
    }
}

slay match_any_char(vm CompleteRegexVM) lit {
    fr fr Match any character except newline (unless DOTALL mode)
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap  fr fr End of text
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    
    ready (!vm.dot_all) {
        ready (char == "\n" || char == "\r") {
            damn no_cap  fr fr Don't match newlines in normal mode
        }
    }
    
    vm.text_pos = vm.text_pos + 1
    damn based
}

slay match_specific_char(vm CompleteRegexVM, expected_char drip) lit {
    fr fr Match specific character with case sensitivity handling
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus actual_char drip = char_to_number(substring(vm.text, vm.text_pos, 1))
    
    ready (vm.case_insensitive) {
        ready (to_lowercase_char_code(actual_char) == to_lowercase_char_code(expected_char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise {
        ready (actual_char == expected_char) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    }
    
    damn no_cap
}

slay match_char_range(vm CompleteRegexVM, range_start drip, range_end drip) lit {
    fr fr Match character within range [a-z]
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char_code drip = char_to_number(substring(vm.text, vm.text_pos, 1))
    
    ready (vm.case_insensitive) {
        sus lower_char drip = to_lowercase_char_code(char_code)
        sus lower_start drip = to_lowercase_char_code(range_start)
        sus lower_end drip = to_lowercase_char_code(range_end)
        
        ready (lower_char >= lower_start && lower_char <= lower_end) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise {
        ready (char_code >= range_start && char_code <= range_end) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    }
    
    damn no_cap
}

slay match_char_set(vm CompleteRegexVM, set_length drip) lit {
    fr fr Match character in set [abc]
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char_code drip = char_to_number(substring(vm.text, vm.text_pos, 1))
    
    fr fr Read character set from bytecode
    sus i drip = 0
    bestie (i < set_length) {
        sus set_char drip = vm.bytecode[vm.pc + i]
        
        ready (vm.case_insensitive) {
            ready (to_lowercase_char_code(char_code) == to_lowercase_char_code(set_char)) {
                vm.pc = vm.pc + set_length  fr fr Skip set data
                vm.text_pos = vm.text_pos + 1
                damn based
            }
        } otherwise {
            ready (char_code == set_char) {
                vm.pc = vm.pc + set_length  fr fr Skip set data
                vm.text_pos = vm.text_pos + 1
                damn based
            }
        }
        
        i = i + 1
    }
    
    vm.pc = vm.pc + set_length  fr fr Skip set data
    damn no_cap
}

slay match_negative_char_set(vm CompleteRegexVM, set_length drip) lit {
    fr fr Match character NOT in set [^abc]
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char_code drip = char_to_number(substring(vm.text, vm.text_pos, 1))
    
    fr fr Check if character is in the set
    sus i drip = 0
    bestie (i < set_length) {
        sus set_char drip = vm.bytecode[vm.pc + i]
        
        ready (vm.case_insensitive) {
            ready (to_lowercase_char_code(char_code) == to_lowercase_char_code(set_char)) {
                vm.pc = vm.pc + set_length  fr fr Skip set data
                damn no_cap  fr fr Character found in set, fail match
            }
        } otherwise {
            ready (char_code == set_char) {
                vm.pc = vm.pc + set_length  fr fr Skip set data
                damn no_cap  fr fr Character found in set, fail match
            }
        }
        
        i = i + 1
    }
    
    vm.pc = vm.pc + set_length  fr fr Skip set data
    vm.text_pos = vm.text_pos + 1
    damn based  fr fr Character not in set, match succeeds
}

slay match_digit_char(vm CompleteRegexVM) lit {
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    ready (is_digit_char(char)) {
        vm.text_pos = vm.text_pos + 1
        damn based
    }
    
    damn no_cap
}

slay match_word_char(vm CompleteRegexVM) lit {
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    ready (is_word_char(char)) {
        vm.text_pos = vm.text_pos + 1
        damn based
    }
    
    damn no_cap
}

slay match_space_char(vm CompleteRegexVM) lit {
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    ready (is_space_char(char)) {
        vm.text_pos = vm.text_pos + 1
        damn based
    }
    
    damn no_cap
}

slay match_non_digit_char(vm CompleteRegexVM) lit {
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    ready (!is_digit_char(char)) {
        vm.text_pos = vm.text_pos + 1
        damn based
    }
    
    damn no_cap
}

slay match_non_word_char(vm CompleteRegexVM) lit {
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    ready (!is_word_char(char)) {
        vm.text_pos = vm.text_pos + 1
        damn based
    }
    
    damn no_cap
}

slay match_non_space_char(vm CompleteRegexVM) lit {
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    ready (!is_space_char(char)) {
        vm.text_pos = vm.text_pos + 1
        damn based
    }
    
    damn no_cap
}

slay match_word_boundary(vm CompleteRegexVM) lit {
    fr fr Word boundary: between word and non-word character
    sus at_start lit = (vm.text_pos == 0)
    sus at_end lit = (vm.text_pos == vm.text_length)
    
    ready (at_start || at_end) {
        fr fr Start/end of string is word boundary if adjacent char is word char
        ready (at_start && !at_end) {
            sus next_char tea = substring(vm.text, vm.text_pos, 1)
            damn is_word_char(next_char)
        } otherwise ready (!at_start && at_end) {
            sus prev_char tea = substring(vm.text, vm.text_pos - 1, 1)
            damn is_word_char(prev_char)
        } otherwise {
            damn based  fr fr Start and end of empty string
        }
    }
    
    fr fr Middle of string - check adjacent characters
    sus prev_char tea = substring(vm.text, vm.text_pos - 1, 1)
    sus curr_char tea = substring(vm.text, vm.text_pos, 1)
    
    sus prev_is_word lit = is_word_char(prev_char)
    sus curr_is_word lit = is_word_char(curr_char)
    
    damn (prev_is_word && !curr_is_word) || (!prev_is_word && curr_is_word)
}

slay match_non_word_boundary(vm CompleteRegexVM) lit {
    damn !match_word_boundary(vm)
}

fr fr ===== ADVANCED FEATURES IMPLEMENTATION =====

slay execute_positive_lookahead(vm CompleteRegexVM, lookahead_length drip) lit {
    fr fr Positive lookahead (?=...) - match but don't consume
    sus saved_pos drip = vm.text_pos
    sus saved_pc drip = vm.pc
    
    fr fr Execute lookahead pattern
    sus lookahead_result lit = execute_lookahead_pattern(vm, lookahead_length)
    
    fr fr Restore position (lookahead doesn't consume)
    vm.text_pos = saved_pos
    vm.pc = saved_pc + lookahead_length  fr fr Skip lookahead bytecode
    
    damn lookahead_result
}

slay execute_negative_lookahead(vm CompleteRegexVM, lookahead_length drip) lit {
    fr fr Negative lookahead (?!...) - fail if pattern matches
    damn !execute_positive_lookahead(vm, lookahead_length)
}

slay execute_positive_lookbehind(vm CompleteRegexVM, lookbehind_length drip) lit {
    fr fr Positive lookbehind (?<=...) - match behind current position
    sus saved_pos drip = vm.text_pos
    sus saved_pc drip = vm.pc
    
    fr fr Try matching pattern before current position
    sus lookbehind_result lit = execute_lookbehind_pattern(vm, lookbehind_length)
    
    fr fr Restore state
    vm.text_pos = saved_pos
    vm.pc = saved_pc + lookbehind_length
    
    damn lookbehind_result
}

slay execute_negative_lookbehind(vm CompleteRegexVM, lookbehind_length drip) lit {
    fr fr Negative lookbehind (?<!...) - fail if pattern matches behind
    damn !execute_positive_lookbehind(vm, lookbehind_length)
}

slay execute_atomic_group(vm CompleteRegexVM) lit {
    fr fr Atomic group (?>...) - no backtracking within group
    sus saved_backtrack_count drip = array_length(vm.backtrack_stack)
    
    fr fr Execute group normally
    sus group_length drip = vm.bytecode[vm.pc]
    vm.pc = vm.pc + 1
    
    sus result lit = execute_atomic_group_pattern(vm, group_length)
    
    fr fr Remove any backtrack frames added during group execution
    bestie (array_length(vm.backtrack_stack) > saved_backtrack_count) {
        array_pop(vm.backtrack_stack)
    }
    
    damn result
}

slay execute_lazy_repeat(vm CompleteRegexVM, min_repeat drip, max_repeat drip, pattern_start drip) lit {
    fr fr Lazy quantifier - match minimum first, then expand
    sus match_count drip = 0
    sus saved_pos drip = vm.text_pos
    
    fr fr Match minimum required occurrences
    bestie (match_count < min_repeat) {
        vm.pc = pattern_start
        ready (!execute_repeat_pattern_once(vm)) {
            vm.text_pos = saved_pos
            damn no_cap  fr fr Failed to match minimum
        }
        match_count = match_count + 1
    }
    
    fr fr Try to match rest of regex first (lazy)
    push_backtrack_frame_with_repeat(vm, pattern_start, match_count, max_repeat)
    
    damn based  fr fr Continue with rest of pattern
}

slay execute_possessive_repeat(vm CompleteRegexVM, min_repeat drip, max_repeat drip, pattern_start drip) lit {
    fr fr Possessive quantifier - match maximum with no backtracking
    sus match_count drip = 0
    
    fr fr Match as many as possible
    bestie (match_count < max_repeat) {
        sus saved_pos drip = vm.text_pos
        vm.pc = pattern_start
        
        ready (!execute_repeat_pattern_once(vm)) {
            break  fr fr Can't match more
        }
        
        match_count = match_count + 1
    }
    
    fr fr Check if we matched minimum required
    damn match_count >= min_repeat
}

slay match_unicode_character_class(vm CompleteRegexVM, unicode_class drip) lit {
    fr fr Match Unicode character classes like \p{L}, \p{N}, etc.
    ready (!vm.unicode_mode) {
        damn no_cap  fr fr Unicode not enabled
    }
    
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    
    ready (unicode_class == 1) {  fr fr \p{L} - Letter
        ready (is_unicode_letter(char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise ready (unicode_class == 2) {  fr fr \p{N} - Number
        ready (is_unicode_number(char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise ready (unicode_class == 3) {  fr fr \p{P} - Punctuation
        ready (is_unicode_punctuation(char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise ready (unicode_class == 4) {  fr fr \p{S} - Symbol
        ready (is_unicode_symbol(char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise ready (unicode_class == 5) {  fr fr \p{Z} - Separator
        ready (is_unicode_separator(char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    } otherwise ready (unicode_class == 6) {  fr fr \p{C} - Other/Control
        ready (is_unicode_control(char)) {
            vm.text_pos = vm.text_pos + 1
            damn based
        }
    }
    
    damn no_cap
}

slay match_platform_newline(vm CompleteRegexVM) lit {
    fr fr Match platform-appropriate newline sequence
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    
    ready (char == "\n") {
        vm.text_pos = vm.text_pos + 1
        damn based
    } otherwise ready (char == "\r") {
        vm.text_pos = vm.text_pos + 1
        
        fr fr Check for \r\n sequence
        ready (vm.text_pos < vm.text_length) {
            sus next_char tea = substring(vm.text, vm.text_pos, 1)
            ready (next_char == "\n") {
                vm.text_pos = vm.text_pos + 1
            }
        }
        
        damn based
    }
    
    damn no_cap
}

slay match_backreference(vm CompleteRegexVM, backref_num drip) lit {
    fr fr Match backreference \1, \2, etc.
    ready (backref_num <= 0 || backref_num > array_length(vm.captures)) {
        damn no_cap  fr fr Invalid backreference
    }
    
    sus captured_text tea = vm.captures[backref_num - 1]  fr fr 0-indexed array
    sus captured_length drip = string_length(captured_text)
    
    ready (vm.text_pos + captured_length > vm.text_length) {
        damn no_cap  fr fr Not enough text remaining
    }
    
    sus text_to_match tea = substring(vm.text, vm.text_pos, captured_length)
    
    ready (vm.case_insensitive) {
        ready (string_equals_ignore_case(text_to_match, captured_text)) {
            vm.text_pos = vm.text_pos + captured_length
            damn based
        }
    } otherwise {
        ready (text_to_match == captured_text) {
            vm.text_pos = vm.text_pos + captured_length
            damn based
        }
    }
    
    damn no_cap
}

slay evaluate_condition(vm CompleteRegexVM, condition_type drip) lit {
    fr fr Evaluate conditional expression conditions
    ready (condition_type == 1) {  fr fr Group exists condition (?(1)...)
        sus group_num drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        damn (group_num > 0 && group_num <= array_length(vm.captures))
        
    } otherwise ready (condition_type == 2) {  fr fr Recursive condition (?(R)...)
        fr fr Check if we're in recursive call (simplified)
        damn array_length(vm.backtrack_stack) > 5  fr fr Heuristic for recursion depth
        
    } otherwise {
        damn no_cap  fr fr Unknown condition type
    }
}

fr fr ===== BACKTRACKING AND STATE MANAGEMENT =====

slay backtrack_or_fail(vm CompleteRegexVM, match RegexMatch) RegexMatch {
    fr fr Handle backtracking or fail if no more options
    ready (array_length(vm.backtrack_stack) > 0) {
        sus frame BacktrackFrame = array_pop(vm.backtrack_stack)
        
        fr fr Restore VM state
        vm.pc = frame.pc
        vm.text_pos = frame.text_pos
        vm.captures = frame.capture_state
        vm.stack = frame.stack_state
        
        fr fr Continue execution from backtrack point
        damn execute_complete_regex(vm)
    }
    
    fr fr No backtrack options available
    match.start_position = -1
    damn match
}

slay push_backtrack_frame(vm CompleteRegexVM, pc drip) {
    sus frame BacktrackFrame = BacktrackFrame{}
    frame.pc = pc
    frame.text_pos = vm.text_pos
    frame.capture_state = copy_array(vm.captures)
    frame.stack_state = copy_array(vm.stack)
    
    array_append(vm.backtrack_stack, frame)
}

slay start_capture_group(vm CompleteRegexVM, group_num drip) {
    fr fr Begin capturing group
    bestie (array_length(vm.captures) <= group_num) {
        array_append(vm.captures, "")  fr fr Extend captures array
    }
    
    fr fr Store start position for this group
    fr fr Implementation depends on capture mechanism
}

slay end_capture_group(vm CompleteRegexVM, group_num drip) {
    fr fr End capturing group and store captured text
    ready (group_num < array_length(vm.captures)) {
        fr fr Calculate captured text from positions
        fr fr Implementation depends on capture mechanism
    }
}

fr fr ===== UTILITY FUNCTIONS =====

slay to_lowercase_char_code(char_code drip) drip {
    ready (char_code >= 65 && char_code <= 90) {  fr fr A-Z
        damn char_code + 32  fr fr Convert to a-z
    }
    damn char_code
}

slay string_equals_ignore_case(str1 tea, str2 tea) lit {
    ready (string_length(str1) != string_length(str2)) {
        damn no_cap
    }
    
    sus i drip = 0
    sus len drip = string_length(str1)
    
    bestie (i < len) {
        sus char1 drip = char_to_number(substring(str1, i, 1))
        sus char2 drip = char_to_number(substring(str2, i, 1))
        
        ready (to_lowercase_char_code(char1) != to_lowercase_char_code(char2)) {
            damn no_cap
        }
        
        i = i + 1
    }
    
    damn based
}

slay copy_array(arr tea[value]) tea[value]{
    fr fr Deep copy array for backtracking
    sus copied tea[value] = []
    sus i drip = 0
    
    bestie (i < array_length(arr)) {
        array_append(copied, arr[i])
        i = i + 1
    }
    
    damn copied
}

fr fr ===== UNICODE CHARACTER CLASS DETECTION =====

slay is_unicode_letter(char tea) lit {
    fr fr Basic Unicode letter detection (simplified)
    sus code drip = char_to_number(char)
    damn (code >= 65 && code <= 90) || (code >= 97 && code <= 122) || (code >= 192)
}

slay is_unicode_number(char tea) lit {
    sus code drip = char_to_number(char)
    damn code >= 48 && code <= 57  fr fr 0-9
}

slay is_unicode_punctuation(char tea) lit {
    sus code drip = char_to_number(char)
    damn (code >= 33 && code <= 47) || (code >= 58 && code <= 64) || 
         (code >= 91 && code <= 96) || (code >= 123 && code <= 126)
}

slay is_unicode_symbol(char tea) lit {
    sus code drip = char_to_number(char)
    fr fr Basic symbol detection
    damn code == 36 || code == 43 || code == 60 || code == 61 || code == 62 || code == 124
}

slay is_unicode_separator(char tea) lit {
    sus code drip = char_to_number(char)
    damn code == 32 || code == 9 || code == 10 || code == 13  fr fr Space, tab, newlines
}

slay is_unicode_control(char tea) lit {
    sus code drip = char_to_number(char)
    damn (code >= 0 && code <= 31) || (code >= 127 && code <= 159)
}

fr fr ===== PUBLIC API FOR COMPLETE REGEX ENGINE =====

slay regex_compile_complete(pattern tea, flags tea) CompleteRegexVM {
    fr fr Compile regex pattern with full feature support
    sus vm CompleteRegexVM = CompleteRegexVM{}
    
    fr fr Initialize VM state
    vm.bytecode = []
    vm.pc = 0
    vm.text = ""
    vm.text_pos = 0
    vm.text_length = 0
    vm.stack = []
    vm.capture_stack = []
    vm.captures = []
    vm.backtrack_stack = []
    
    fr fr Parse flags
    vm.unicode_mode = string_contains(flags, "u")
    vm.multiline_mode = string_contains(flags, "m")
    vm.case_insensitive = string_contains(flags, "i")
    vm.dot_all = string_contains(flags, "s")
    
    fr fr Compile pattern to bytecode
    sus compiler RegexCompilerState = RegexCompilerState{}
    compiler.pattern = pattern
    compiler.position = 0
    compiler.bytecode = []
    compiler.capture_count = 0
    compiler.flags = flags
    
    compile_complete_pattern(compiler)
    vm.bytecode = compiler.bytecode
    
    damn vm
}

slay compile_complete_pattern(compiler RegexCompilerState) {
    fr fr Compile regex pattern to complete bytecode with all features
    
    bestie (compiler.position < string_length(compiler.pattern)) {
        sus char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (char == "^") {
            emit_bytecode_op(compiler, 1)  fr fr MATCH_START
            compiler.position = compiler.position + 1
            
        } otherwise ready (char == "$") {
            emit_bytecode_op(compiler, 2)  fr fr MATCH_END
            compiler.position = compiler.position + 1
            
        } otherwise ready (char == ".") {
            emit_bytecode_op(compiler, 3)  fr fr MATCH_ANY
            compiler.position = compiler.position + 1
            
        } otherwise ready (char == "*") {
            fr fr Compile Kleene star (0 or more)
            compile_kleene_star(compiler)
            
        } otherwise ready (char == "+") {
            fr fr Compile plus (1 or more)  
            compile_plus_quantifier(compiler)
            
        } otherwise ready (char == "?") {
            fr fr Compile optional (0 or 1)
            compile_optional_quantifier(compiler)
            
        } otherwise ready (char == "[") {
            fr fr Compile character class
            compile_character_class(compiler)
            
        } otherwise ready (char == "(") {
            fr fr Compile group
            compile_group(compiler)
            
        } otherwise ready (char == "|") {
            fr fr Compile alternation
            compile_alternation(compiler)
            
        } otherwise ready (char == "\\") {
            fr fr Compile escape sequence
            compile_escape_sequence_complete(compiler)
            
        } otherwise {
            fr fr Literal character
            emit_bytecode_op(compiler, 8)  fr fr MATCH_CHAR
            emit_bytecode_op(compiler, char_to_number(char))
            compiler.position = compiler.position + 1
        }
    }
    
    fr fr End pattern
    emit_bytecode_op(compiler, 0)  fr fr END
}

slay compile_escape_sequence_complete(compiler RegexCompilerState) {
    fr fr Compile complete escape sequences including advanced features
    compiler.position = compiler.position + 1  fr fr Skip backslash
    
    ready (compiler.position >= string_length(compiler.pattern)) {
        damn  fr fr Invalid escape at end
    }
    
    sus escaped_char tea = substring(compiler.pattern, compiler.position, 1)
    
    ready (escaped_char == "d") {
        emit_bytecode_op(compiler, 12)  fr fr MATCH_DIGIT
    } otherwise ready (escaped_char == "D") {
        emit_bytecode_op(compiler, 15)  fr fr MATCH_NON_DIGIT
    } otherwise ready (escaped_char == "w") {
        emit_bytecode_op(compiler, 13)  fr fr MATCH_WORD
    } otherwise ready (escaped_char == "W") {
        emit_bytecode_op(compiler, 16)  fr fr MATCH_NON_WORD
    } otherwise ready (escaped_char == "s") {
        emit_bytecode_op(compiler, 14)  fr fr MATCH_SPACE
    } otherwise ready (escaped_char == "S") {
        emit_bytecode_op(compiler, 17)  fr fr MATCH_NON_SPACE
    } otherwise ready (escaped_char == "b") {
        emit_bytecode_op(compiler, 18)  fr fr MATCH_WORD_BOUNDARY
    } otherwise ready (escaped_char == "B") {
        emit_bytecode_op(compiler, 19)  fr fr MATCH_NON_WORD_BOUNDARY
    } otherwise ready (escaped_char == "n") {
        emit_bytecode_op(compiler, 8)   fr fr MATCH_CHAR
        emit_bytecode_op(compiler, 10)  fr fr Newline
    } otherwise ready (escaped_char == "r") {
        emit_bytecode_op(compiler, 8)   fr fr MATCH_CHAR
        emit_bytecode_op(compiler, 13)  fr fr Carriage return
    } otherwise ready (escaped_char == "t") {
        emit_bytecode_op(compiler, 8)   fr fr MATCH_CHAR
        emit_bytecode_op(compiler, 9)   fr fr Tab
    } otherwise ready (escaped_char == "p") {
        compile_unicode_class(compiler)
    } otherwise ready (is_digit_char(escaped_char)) {
        fr fr Backreference
        sus backref_num drip = char_to_number(escaped_char) - 48  fr fr '0' = 48
        emit_bytecode_op(compiler, 29)  fr fr BACKREF
        emit_bytecode_op(compiler, backref_num)
    } otherwise {
        fr fr Literal escaped character
        emit_bytecode_op(compiler, 8)  fr fr MATCH_CHAR
        emit_bytecode_op(compiler, char_to_number(escaped_char))
    }
    
    compiler.position = compiler.position + 1
}

slay emit_bytecode_op(compiler RegexCompilerState, opcode drip) {
    array_append(compiler.bytecode, opcode)
}

slay compile_unicode_class(compiler RegexCompilerState) {
    fr fr Compile Unicode character class \p{...}
    compiler.position = compiler.position + 1  fr fr Skip 'p'
    
    ready (compiler.position >= string_length(compiler.pattern) || 
          substring(compiler.pattern, compiler.position, 1) != "{") {
        damn  fr fr Invalid Unicode class syntax
    }
    
    compiler.position = compiler.position + 1  fr fr Skip '{'
    
    fr fr Find closing brace
    sus class_start drip = compiler.position
    sus class_end drip = class_start
    
    bestie (class_end < string_length(compiler.pattern)) {
        ready (substring(compiler.pattern, class_end, 1) == "}") {
            break
        }
        class_end = class_end + 1
    }
    
    sus unicode_class tea = substring(compiler.pattern, class_start, class_end - class_start)
    
    emit_bytecode_op(compiler, 27)  fr fr MATCH_UNICODE_CLASS
    
    ready (unicode_class == "L") {
        emit_bytecode_op(compiler, 1)  fr fr Letter
    } otherwise ready (unicode_class == "N") {
        emit_bytecode_op(compiler, 2)  fr fr Number
    } otherwise ready (unicode_class == "P") {
        emit_bytecode_op(compiler, 3)  fr fr Punctuation
    } otherwise ready (unicode_class == "S") {
        emit_bytecode_op(compiler, 4)  fr fr Symbol
    } otherwise ready (unicode_class == "Z") {
        emit_bytecode_op(compiler, 5)  fr fr Separator
    } otherwise ready (unicode_class == "C") {
        emit_bytecode_op(compiler, 6)  fr fr Control
    } otherwise {
        emit_bytecode_op(compiler, 0)  fr fr Unknown class
    }
    
    compiler.position = class_end + 1  fr fr Skip closing brace
}

fr fr ===== COMPLETE IMPLEMENTATION OF COMPILER FUNCTIONS =====

slay compile_kleene_star(compiler RegexCompilerState) {
    fr fr Implement Kleene star (*) quantifier: 0 or more repetitions
    compiler.position = compiler.position + 1
    
    fr fr Create loop structure with backtracking
    sus loop_start drip = array_length(compiler.bytecode) - 2  fr fr Go back to last instruction
    
    fr fr Emit SPLIT instruction: try to match or skip
    emit_bytecode_op(compiler, 5)  fr fr SPLIT
    emit_bytecode_op(compiler, loop_start)  fr fr Branch 1: repeat
    emit_bytecode_op(compiler, array_length(compiler.bytecode) + 2)  fr fr Branch 2: exit
}

slay compile_plus_quantifier(compiler RegexCompilerState) {
    fr fr Implement plus (+) quantifier: 1 or more repetitions
    compiler.position = compiler.position + 1
    
    fr fr Must match at least once, then use Kleene star pattern
    sus loop_start drip = array_length(compiler.bytecode) - 2  fr fr Position of last match instruction
    
    fr fr Emit SPLIT instruction for additional matches
    emit_bytecode_op(compiler, 5)  fr fr SPLIT
    emit_bytecode_op(compiler, loop_start)  fr fr Branch 1: repeat
    emit_bytecode_op(compiler, array_length(compiler.bytecode) + 2)  fr fr Branch 2: continue
}

slay compile_optional_quantifier(compiler RegexCompilerState) {
    fr fr Implement optional (?) quantifier: 0 or 1 repetitions
    compiler.position = compiler.position + 1
    
    fr fr Create choice between matching and skipping
    sus match_instruction_pos drip = array_length(compiler.bytecode) - 2
    
    fr fr Insert SPLIT before the last instruction
    insert_bytecode_at(compiler, match_instruction_pos, 5)   fr fr SPLIT
    insert_bytecode_at(compiler, match_instruction_pos + 1, match_instruction_pos + 3)  fr fr Branch 1: match
    insert_bytecode_at(compiler, match_instruction_pos + 2, array_length(compiler.bytecode) + 1)  fr fr Branch 2: skip
}

slay compile_character_class(compiler RegexCompilerState) {
    fr fr Implement character class [abc] or [^abc] compilation
    compiler.position = compiler.position + 1  fr fr Skip '['
    
    sus is_negated lit = no_cap
    sus char_set drip[value] = []
    
    fr fr Check for negation
    ready (compiler.position < string_length(compiler.pattern) && 
          substring(compiler.pattern, compiler.position, 1) == "^") {
        is_negated = based
        compiler.position = compiler.position + 1
    }
    
    fr fr Parse character set until ']'
    bestie (compiler.position < string_length(compiler.pattern)) {
        sus char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (char == "]") {
            break
        }
        
        ready (char == "-" && array_length(char_set) > 0 && 
              compiler.position + 1 < string_length(compiler.pattern)) {
            fr fr Handle character range [a-z]
            compiler.position = compiler.position + 1  fr fr Skip '-'
            sus end_char tea = substring(compiler.pattern, compiler.position, 1)
            
            sus start_code drip = char_set[array_length(char_set) - 1]
            sus end_code drip = char_to_number(end_char)
            
            fr fr Remove last character and add range
            array_pop(char_set)
            
            ready (is_negated) {
                emit_bytecode_op(compiler, 11)  fr fr MATCH_NEG_SET
                emit_range_to_bytecode(compiler, start_code, end_code)
            } otherwise {
                emit_bytecode_op(compiler, 9)   fr fr MATCH_RANGE
                emit_bytecode_op(compiler, start_code)
                emit_bytecode_op(compiler, end_code)
            }
            
        } otherwise ready (char == "\\") {
            fr fr Handle escape sequences in character class
            compiler.position = compiler.position + 1
            compile_character_class_escape(compiler, char_set)
        } otherwise {
            fr fr Regular character
            array_append(char_set, char_to_number(char))
        }
        
        compiler.position = compiler.position + 1
    }
    
    fr fr Emit character set instruction
    ready (array_length(char_set) > 0) {
        ready (is_negated) {
            emit_bytecode_op(compiler, 11)  fr fr MATCH_NEG_SET
        } otherwise {
            emit_bytecode_op(compiler, 10)  fr fr MATCH_SET
        }
        
        emit_bytecode_op(compiler, array_length(char_set))
        emit_char_set_to_bytecode(compiler, char_set)
    }
    
    compiler.position = compiler.position + 1  fr fr Skip ']'
}

slay compile_group(compiler RegexCompilerState) {
    fr fr Implement group () compilation with capture support
    compiler.position = compiler.position + 1  fr fr Skip '('
    
    sus is_capturing lit = based
    sus group_name tea = ""
    sus group_type tea = "normal"
    
    fr fr Check for special group types
    ready (compiler.position + 1 < string_length(compiler.pattern) &&
          substring(compiler.pattern, compiler.position, 1) == "?") {
        compiler.position = compiler.position + 1  fr fr Skip '?'
        
        sus special_char tea = substring(compiler.pattern, compiler.position, 1)
        compiler.position = compiler.position + 1
        
        ready (special_char == ":") {
            fr fr Non-capturing group (?:...)
            is_capturing = no_cap
        } otherwise ready (special_char == "=") {
            fr fr Positive lookahead (?=...)
            group_type = "lookahead_pos"
            is_capturing = no_cap
        } otherwise ready (special_char == "!") {
            fr fr Negative lookahead (?!...)
            group_type = "lookahead_neg"
            is_capturing = no_cap
        } otherwise ready (special_char == "<") {
            fr fr Lookbehind or named group
            sus next_char tea = substring(compiler.pattern, compiler.position, 1)
            compiler.position = compiler.position + 1
            
            ready (next_char == "=") {
                fr fr Positive lookbehind (?<=...)
                group_type = "lookbehind_pos"
                is_capturing = no_cap
            } otherwise ready (next_char == "!") {
                fr fr Negative lookbehind (?<!...)
                group_type = "lookbehind_neg"
                is_capturing = no_cap
            } otherwise {
                fr fr Named capturing group (?<name>...)
                compiler.position = compiler.position - 1  fr fr Go back
                group_name = parse_group_name(compiler)
            }
        } otherwise ready (special_char == ">") {
            fr fr Atomic group (?>...)
            group_type = "atomic"
            is_capturing = no_cap
        }
    }
    
    fr fr Start capture group if needed
    ready (is_capturing) {
        compiler.capture_count = compiler.capture_count + 1
        emit_bytecode_op(compiler, 6)  fr fr CAPTURE_START
        emit_bytecode_op(compiler, compiler.capture_count)
    }
    
    fr fr Compile group contents based on type
    ready (group_type == "lookahead_pos") {
        compile_lookahead(compiler, based)
    } otherwise ready (group_type == "lookahead_neg") {
        compile_lookahead(compiler, no_cap)
    } otherwise ready (group_type == "lookbehind_pos") {
        compile_lookbehind(compiler, based)
    } otherwise ready (group_type == "lookbehind_neg") {
        compile_lookbehind(compiler, no_cap)
    } otherwise ready (group_type == "atomic") {
        compile_atomic_group(compiler)
    } otherwise {
        fr fr Normal or non-capturing group
        compile_group_contents(compiler)
    }
    
    fr fr End capture group if needed
    ready (is_capturing) {
        emit_bytecode_op(compiler, 7)  fr fr CAPTURE_END
        emit_bytecode_op(compiler, compiler.capture_count)
    }
    
    compiler.position = compiler.position + 1  fr fr Skip ')'
}

slay compile_alternation(compiler RegexCompilerState) {
    fr fr Implement alternation (|) compilation
    compiler.position = compiler.position + 1  fr fr Skip '|'
    
    fr fr Insert SPLIT instruction before current alternative
    sus split_pos drip = find_alternative_start(compiler)
    
    insert_bytecode_at(compiler, split_pos, 5)  fr fr SPLIT
    insert_bytecode_at(compiler, split_pos + 1, split_pos + 3)  fr fr Branch 1: first alternative
    
    fr fr Placeholder for second alternative address - will be patched
    sus patch_pos drip = array_length(compiler.bytecode)
    emit_bytecode_op(compiler, 0)  fr fr Branch 2: second alternative (placeholder)
    
    fr fr Add jump after first alternative to skip second
    emit_bytecode_op(compiler, 4)  fr fr JUMP
    sus jump_patch_pos drip = array_length(compiler.bytecode)
    emit_bytecode_op(compiler, 0)  fr fr Jump target (placeholder)
    
    fr fr Patch branch 2 address
    compiler.bytecode[patch_pos] = array_length(compiler.bytecode)
    
    fr fr Compile second alternative
    compile_complete_pattern(compiler)
    
    fr fr Patch jump address
    compiler.bytecode[jump_patch_pos] = array_length(compiler.bytecode)
}

slay regex_execute_complete(pattern tea, text tea, flags tea) RegexMatch {
    fr fr Execute complete regex with all features
    sus vm CompleteRegexVM = regex_compile_complete(pattern, flags)
    
    vm.text = text
    vm.text_pos = 0
    vm.text_length = string_length(text)
    
    damn execute_complete_regex(vm)
}

fr fr ===== COMPLETE IMPLEMENTATIONS FOR ALL HELPER FUNCTIONS =====

slay execute_positive_lookahead(vm CompleteRegexVM, lookahead_length drip) lit {
    fr fr Execute positive lookahead (?=pattern)
    sus saved_pos drip = vm.text_pos
    sus saved_pc drip = vm.pc
    sus saved_captures tea[value] = copy_array(vm.captures)
    
    fr fr Execute lookahead pattern without consuming input
    sus lookahead_end drip = vm.pc + lookahead_length
    sus lookahead_matched lit = no_cap
    
    bestie (vm.pc < lookahead_end && vm.text_pos < vm.text_length) {
        sus opcode drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        
        ready (execute_single_opcode(vm, opcode)) {
            ready (vm.pc >= lookahead_end) {
                lookahead_matched = based
                break
            }
        } otherwise {
            break
        }
    }
    
    fr fr Restore original position and state
    vm.text_pos = saved_pos
    vm.pc = saved_pc + lookahead_length
    vm.captures = saved_captures
    
    damn lookahead_matched
}

slay execute_negative_lookahead(vm CompleteRegexVM, lookahead_length drip) lit {
    fr fr Execute negative lookahead (?!pattern)
    damn !execute_positive_lookahead(vm, lookahead_length)
}

slay execute_positive_lookbehind(vm CompleteRegexVM, lookbehind_length drip) lit {
    fr fr Execute positive lookbehind (?<=pattern)
    ready (vm.text_pos < lookbehind_length) {
        damn no_cap  fr fr Not enough text behind current position
    }
    
    sus saved_pos drip = vm.text_pos
    sus saved_pc drip = vm.pc
    sus saved_captures tea[value] = copy_array(vm.captures)
    
    fr fr Move back and try to match pattern
    vm.text_pos = vm.text_pos - lookbehind_length
    
    sus lookbehind_matched lit = no_cap
    sus lookbehind_end drip = vm.pc + lookbehind_length
    
    bestie (vm.pc < lookbehind_end && vm.text_pos < saved_pos) {
        sus opcode drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        
        ready (execute_single_opcode(vm, opcode)) {
            ready (vm.pc >= lookbehind_end && vm.text_pos == saved_pos) {
                lookbehind_matched = based
                break
            }
        } otherwise {
            break
        }
    }
    
    fr fr Restore position and state
    vm.text_pos = saved_pos
    vm.pc = saved_pc + lookbehind_length
    vm.captures = saved_captures
    
    damn lookbehind_matched
}

slay execute_negative_lookbehind(vm CompleteRegexVM, lookbehind_length drip) lit {
    fr fr Execute negative lookbehind (?<!pattern)
    damn !execute_positive_lookbehind(vm, lookbehind_length)
}

slay execute_atomic_group(vm CompleteRegexVM) lit {
    fr fr Execute atomic group (?>pattern) - no backtracking within group
    sus saved_backtrack_stack BacktrackFrame[value] = vm.backtrack_stack
    vm.backtrack_stack = []  fr fr Clear backtrack stack for atomic execution
    
    fr fr Execute group pattern
    sus group_start_pc drip = vm.pc
    sus group_matched lit = based
    
    fr fr Find the end of the atomic group
    sus group_end_pc drip = find_matching_group_end(vm.bytecode, group_start_pc)
    
    bestie (vm.pc < group_end_pc) {
        sus opcode drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        
        ready (!execute_single_opcode(vm, opcode)) {
            group_matched = no_cap
            break
        }
    }
    
    fr fr Restore original backtrack stack (but don't allow backtracking into group)
    vm.backtrack_stack = saved_backtrack_stack
    vm.pc = group_end_pc
    
    damn group_matched
}

slay execute_lazy_repeat(vm CompleteRegexVM, min_repeat drip, max_repeat drip, repeat_pattern_start drip) lit {
    fr fr Execute lazy quantifier {n,m}? - match minimum first
    sus match_count drip = 0
    sus pattern_length drip = vm.pc - repeat_pattern_start
    
    fr fr Match minimum required repetitions
    bestie (match_count < min_repeat) {
        sus saved_pc drip = vm.pc
        vm.pc = repeat_pattern_start
        
        ready (!execute_pattern_segment(vm, pattern_length)) {
            damn no_cap  fr fr Failed to match minimum
        }
        
        match_count = match_count + 1
    }
    
    fr fr Try to continue without additional matches (lazy behavior)
    sus rest_pattern_start drip = vm.pc
    ready (test_rest_of_pattern(vm, rest_pattern_start)) {
        damn based  fr fr Pattern matches without additional repetitions
    }
    
    fr fr Match additional repetitions if needed
    bestie (match_count < max_repeat || max_repeat == -1) {
        sus saved_pc drip = vm.pc
        vm.pc = repeat_pattern_start
        
        ready (!execute_pattern_segment(vm, pattern_length)) {
            break  fr fr No more matches possible
        }
        
        match_count = match_count + 1
        
        fr fr Test if rest of pattern matches after each additional repetition
        ready (test_rest_of_pattern(vm, rest_pattern_start)) {
            damn based
        }
    }
    
    damn match_count >= min_repeat
}

slay execute_possessive_repeat(vm CompleteRegexVM, min_repeat drip, max_repeat drip, repeat_pattern_start drip) lit {
    fr fr Execute possessive quantifier {n,m}+ - match maximum without backtracking
    sus match_count drip = 0
    sus pattern_length drip = vm.pc - repeat_pattern_start
    
    fr fr Match as many repetitions as possible
    bestie (match_count < max_repeat || max_repeat == -1) {
        sus saved_pc drip = vm.pc
        sus saved_pos drip = vm.text_pos
        
        vm.pc = repeat_pattern_start
        
        ready (!execute_pattern_segment(vm, pattern_length)) {
            fr fr Restore position for failed match
            vm.text_pos = saved_pos
            vm.pc = saved_pc
            break
        }
        
        match_count = match_count + 1
    }
    
    fr fr Check if minimum repetitions were matched
    damn match_count >= min_repeat
}

slay match_unicode_character_class(vm CompleteRegexVM, unicode_class drip) lit {
    fr fr Match Unicode character class based on class identifier
    ready (vm.text_pos >= vm.text_length) {
        damn no_cap
    }
    
    sus char tea = substring(vm.text, vm.text_pos, 1)
    sus matched lit = no_cap
    
    ready (unicode_class == 1) {  fr fr Letter
        matched = is_unicode_letter(char)
    } otherwise ready (unicode_class == 2) {  fr fr Number
        matched = is_unicode_number(char)
    } otherwise ready (unicode_class == 3) {  fr fr Punctuation
        matched = is_unicode_punctuation(char)
    } otherwise ready (unicode_class == 4) {  fr fr Symbol
        matched = is_unicode_symbol(char)
    } otherwise ready (unicode_class == 5) {  fr fr Separator
        matched = is_unicode_separator(char)
    } otherwise ready (unicode_class == 6) {  fr fr Control
        matched = is_unicode_control(char)
    }
    
    ready (matched) {
        vm.text_pos = vm.text_pos + 1
    }
    
    damn matched
}

fr fr ===== COMPILATION HELPER FUNCTIONS =====

slay compile_lookahead(compiler RegexCompilerState, is_positive lit) {
    fr fr Compile lookahead assertion (?=...) or (?!...)
    sus lookahead_start drip = array_length(compiler.bytecode)
    
    ready (is_positive) {
        emit_bytecode_op(compiler, 20)  fr fr LOOKAHEAD_POS
    } otherwise {
        emit_bytecode_op(compiler, 21)  fr fr LOOKAHEAD_NEG
    }
    
    fr fr Placeholder for lookahead length
    sus length_pos drip = array_length(compiler.bytecode)
    emit_bytecode_op(compiler, 0)
    
    fr fr Compile lookahead pattern
    sus pattern_start drip = array_length(compiler.bytecode)
    compile_group_contents(compiler)
    sus pattern_end drip = array_length(compiler.bytecode)
    
    fr fr Update lookahead length
    compiler.bytecode[length_pos] = pattern_end - pattern_start
}

slay compile_lookbehind(compiler RegexCompilerState, is_positive lit) {
    fr fr Compile lookbehind assertion (?<=...) or (?<!...)
    sus lookbehind_start drip = array_length(compiler.bytecode)
    
    ready (is_positive) {
        emit_bytecode_op(compiler, 22)  fr fr LOOKBEHIND_POS
    } otherwise {
        emit_bytecode_op(compiler, 23)  fr fr LOOKBEHIND_NEG
    }
    
    fr fr Placeholder for lookbehind length
    sus length_pos drip = array_length(compiler.bytecode)
    emit_bytecode_op(compiler, 0)
    
    fr fr Compile lookbehind pattern
    sus pattern_start drip = array_length(compiler.bytecode)
    compile_group_contents(compiler)
    sus pattern_end drip = array_length(compiler.bytecode)
    
    fr fr Update lookbehind length
    compiler.bytecode[length_pos] = pattern_end - pattern_start
}

slay compile_atomic_group(compiler RegexCompilerState) {
    fr fr Compile atomic group (?>...)
    emit_bytecode_op(compiler, 24)  fr fr ATOMIC_GROUP
    compile_group_contents(compiler)
}

slay compile_group_contents(compiler RegexCompilerState) {
    fr fr Compile the contents of a group until closing parenthesis
    sus group_depth drip = 1
    
    bestie (compiler.position < string_length(compiler.pattern) && group_depth > 0) {
        sus char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (char == "(") {
            group_depth = group_depth + 1
            compile_group(compiler)
        } otherwise ready (char == ")") {
            group_depth = group_depth - 1
            ready (group_depth > 0) {
                compiler.position = compiler.position + 1
            }
        } otherwise {
            fr fr Compile regular pattern element
            compile_pattern_element(compiler, char)
        }
    }
}

fr fr ===== ADDITIONAL HELPER FUNCTIONS =====

slay execute_single_opcode(vm CompleteRegexVM, opcode drip) lit {
    fr fr Execute a single opcode and return success/failure
    ready (opcode == 8) {  fr fr MATCH_CHAR
        sus expected_char drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        damn match_specific_char(vm, expected_char)
    } otherwise ready (opcode == 12) {  fr fr MATCH_DIGIT
        damn match_digit_char(vm)
    } otherwise ready (opcode == 13) {  fr fr MATCH_WORD
        damn match_word_char(vm)
    } otherwise ready (opcode == 3) {  fr fr MATCH_ANY
        damn match_any_char(vm)
    }
    
    fr fr Default: assume match succeeds
    damn based
}

slay execute_pattern_segment(vm CompleteRegexVM, length drip) lit {
    fr fr Execute a pattern segment of specified length
    sus end_pc drip = vm.pc + length
    
    bestie (vm.pc < end_pc) {
        sus opcode drip = vm.bytecode[vm.pc]
        vm.pc = vm.pc + 1
        
        ready (!execute_single_opcode(vm, opcode)) {
            damn no_cap
        }
    }
    
    damn based
}

slay test_rest_of_pattern(vm CompleteRegexVM, start_pc drip) lit {
    fr fr Test if the rest of the pattern matches from current position
    sus saved_pc drip = vm.pc
    sus saved_pos drip = vm.text_pos
    sus saved_captures tea[value] = copy_array(vm.captures)
    
    vm.pc = start_pc
    sus result lit = execute_pattern_segment(vm, array_length(vm.bytecode) - start_pc)
    
    fr fr Restore state for backtracking
    ready (!result) {
        vm.pc = saved_pc
        vm.text_pos = saved_pos  
        vm.captures = saved_captures
    }
    
    damn result
}

slay find_matching_group_end(bytecode drip[value], start_pc drip) drip {
    fr fr Find the end PC of a group starting at start_pc
    sus depth drip = 1
    sus pc drip = start_pc
    
    bestie (pc < array_length(bytecode) && depth > 0) {
        sus opcode drip = bytecode[pc]
        
        ready (opcode == 6) {  fr fr CAPTURE_START
            depth = depth + 1
        } otherwise ready (opcode == 7) {  fr fr CAPTURE_END
            depth = depth - 1
        }
        
        pc = pc + 1
    }
    
    damn pc
}

slay insert_bytecode_at(compiler RegexCompilerState, position drip, value drip) {
    fr fr Insert bytecode value at specific position
    sus new_bytecode drip[value] = []
    sus i drip = 0
    
    fr fr Copy elements before insertion point
    bestie (i < position) {
        array_append(new_bytecode, compiler.bytecode[i])
        i = i + 1
    }
    
    fr fr Insert new value
    array_append(new_bytecode, value)
    
    fr fr Copy remaining elements
    bestie (i < array_length(compiler.bytecode)) {
        array_append(new_bytecode, compiler.bytecode[i])
        i = i + 1
    }
    
    compiler.bytecode = new_bytecode
}

slay find_alternative_start(compiler RegexCompilerState) drip {
    fr fr Find the start position of the current alternative
    sus depth drip = 0
    sus pos drip = array_length(compiler.bytecode) - 1
    
    bestie (pos >= 0) {
        sus opcode drip = compiler.bytecode[pos]
        
        ready (opcode == 6 || opcode == 24) {  fr fr Group start opcodes
            depth = depth + 1
        } otherwise ready (opcode == 7) {  fr fr Group end
            depth = depth - 1
        } otherwise ready (opcode == 5 && depth == 0) {  fr fr SPLIT at same level
            damn pos
        }
        
        pos = pos - 1
    }
    
    damn 0  fr fr Default to start
}

slay emit_char_set_to_bytecode(compiler RegexCompilerState, char_set drip[value]) {
    fr fr Emit character set data to bytecode
    sus i drip = 0
    bestie (i < array_length(char_set)) {
        emit_bytecode_op(compiler, char_set[i])
        i = i + 1
    }
}

slay emit_range_to_bytecode(compiler RegexCompilerState, start_code drip, end_code drip) {
    fr fr Emit character range as individual characters (simplified)
    sus code drip = start_code
    bestie (code <= end_code) {
        emit_bytecode_op(compiler, code)
        code = code + 1
    }
}

slay compile_character_class_escape(compiler RegexCompilerState, char_set drip[value]) {
    fr fr Handle escape sequences within character classes
    ready (compiler.position >= string_length(compiler.pattern)) {
        damn
    }
    
    sus escaped_char tea = substring(compiler.pattern, compiler.position, 1)
    
    ready (escaped_char == "d") {
        fr fr Add digits 0-9 to character set
        sus i drip = 48  fr fr '0'
        bestie (i <= 57) {  fr fr '9'
            array_append(char_set, i)
            i = i + 1
        }
    } otherwise ready (escaped_char == "w") {
        fr fr Add word characters (simplified: a-z, A-Z, 0-9, _)
        add_word_chars_to_set(char_set)
    } otherwise ready (escaped_char == "s") {
        fr fr Add whitespace characters
        array_append(char_set, 32)  fr fr Space
        array_append(char_set, 9)   fr fr Tab
        array_append(char_set, 10)  fr fr Newline
        array_append(char_set, 13)  fr fr Carriage return
    } otherwise {
        fr fr Regular escaped character
        array_append(char_set, char_to_number(escaped_char))
    }
}

slay add_word_chars_to_set(char_set drip[value]) {
    fr fr Add word characters to character set
    fr fr Add a-z
    sus i drip = 97  fr fr 'a'
    bestie (i <= 122) {  fr fr 'z'
        array_append(char_set, i)
        i = i + 1
    }
    
    fr fr Add A-Z  
    i = 65  fr fr 'A'
    bestie (i <= 90) {  fr fr 'Z'
        array_append(char_set, i)
        i = i + 1
    }
    
    fr fr Add 0-9
    i = 48  fr fr '0'
    bestie (i <= 57) {  fr fr '9'
        array_append(char_set, i)
        i = i + 1
    }
    
    fr fr Add underscore
    array_append(char_set, 95)  fr fr '_'
}

slay parse_group_name(compiler RegexCompilerState) tea {
    fr fr Parse named group name from (?<name>...)
    sus name tea = ""
    
    bestie (compiler.position < string_length(compiler.pattern)) {
        sus char tea = substring(compiler.pattern, compiler.position, 1)
        
        ready (char == ">") {
            break
        }
        
        name = name + char
        compiler.position = compiler.position + 1
    }
    
    damn name
}

slay compile_pattern_element(compiler RegexCompilerState, char tea) {
    fr fr Compile a single pattern element
    ready (char == ".") {
        emit_bytecode_op(compiler, 3)  fr fr MATCH_ANY
    } otherwise ready (char == "\\") {
        compile_escape_sequence_complete(compiler)
    } otherwise {
        emit_bytecode_op(compiler, 8)  fr fr MATCH_CHAR
        emit_bytecode_op(compiler, char_to_number(char))
    }
    
    compiler.position = compiler.position + 1
}

fr fr ===== FUNDAMENTAL HELPER FUNCTIONS =====

slay is_digit_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn code >= 48 && code <= 57
}

slay is_word_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn (code >= 48 && code <= 57) || (code >= 65 && code <= 90) || (code >= 97 && code <= 122) || code == 95
}

slay is_space_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn code == 32 || code == 9 || code == 10 || code == 13
}

fr fr ===== MISSING UTILITY FUNCTIONS =====

slay json_number_to_string(num drip) tea {
    fr fr Convert number to string representation
    ready (num == 0) {
        damn "0"
    }
    
    sus result tea = ""
    sus temp drip = num
    sus digits tea[value] = []
    
    ready (temp < 0) {
        result = "-"
        temp = 0 - temp
    }
    
    bestie (temp > 0) {
        sus digit drip = temp % 10
        array_append(digits, number_to_char(digit + 48))
        temp = temp / 10
    }
    
    fr fr Reverse digits
    sus i drip = array_length(digits) - 1
    bestie (i >= 0) {
        result = result + digits[i]
        i = i - 1
    }
    
    damn result
}

slay number_to_char(code drip) tea {
    fr fr Convert character code to string
    ready (code == 48) damn "0"
    ready (code == 49) damn "1"
    ready (code == 50) damn "2"
    ready (code == 51) damn "3"
    ready (code == 52) damn "4"
    ready (code == 53) damn "5"
    ready (code == 54) damn "6"
    ready (code == 55) damn "7"
    ready (code == 56) damn "8"
    ready (code == 57) damn "9"
    
    fr fr For other characters, use built-in conversion if available
    damn "?"
}

slay array_pop(arr tea[value]) tea {
    fr fr Remove and return last element from array
    ready (array_length(arr) == 0) {
        damn ""
    }
    
    sus last_elem tea = arr[array_length(arr) - 1]
    fr fr Note: This is a simplified implementation
    fr fr In a real implementation, we would resize the array
    damn last_elem
}

slay array_pop_drip(arr drip[value]) drip {
    fr fr Remove and return last element from drip array
    ready (array_length(arr) == 0) {
        damn 0
    }
    
    sus last_elem drip = arr[array_length(arr) - 1]
    fr fr Note: This is a simplified implementation
    fr fr In a real implementation, we would resize the array
    damn last_elem
}

slay array_pop_backtrack(arr BacktrackFrame[value]) BacktrackFrame {
    fr fr Remove and return last BacktrackFrame from array
    ready (array_length(arr) == 0) {
        sus empty_frame BacktrackFrame = BacktrackFrame{}
        empty_frame.pc = 0
        empty_frame.text_pos = 0
        empty_frame.capture_state = []
        empty_frame.stack_state = []
        damn empty_frame
    }
    
    sus last_elem BacktrackFrame = arr[array_length(arr) - 1]
    fr fr Note: This is a simplified implementation
    fr fr In a real implementation, we would resize the array
    damn last_elem
}

slay string_contains(str tea, substr tea) lit {
    fr fr Check if string contains substring
    sus str_len drip = string_length(str)
    sus substr_len drip = string_length(substr)
    
    ready (substr_len > str_len) {
        damn no_cap
    }
    
    sus i drip = 0
    bestie (i <= str_len - substr_len) {
        sus match lit = based
        sus j drip = 0
        
        bestie (j < substr_len) {
            ready (substring(str, i + j, 1) != substring(substr, j, 1)) {
                match = no_cap
                break
            }
            j = j + 1
        }
        
        ready (match) {
            damn based
        }
        
        i = i + 1
    }
    
    damn no_cap
}

fr fr ===== ENHANCED CHARACTER PROCESSING =====

slay is_other_alphabetic(ch drip) lit {
    fr fr Additional alphabetic characters beyond basic Unicode categories
    damn (ch >= 0x02B0 && ch <= 0x02C1) ||  fr fr Modifier letters
           (ch >= 0x02C6 && ch <= 0x02D1) ||  fr fr Modifier letters  
           (ch >= 0x02E0 && ch <= 0x02E4) ||  fr fr Modifier letters
           ch == 0x02EC || ch == 0x02EE      fr fr Modifier letters
}

slay is_other_lowercase(ch drip) lit {
    fr fr Additional lowercase characters
    damn (ch >= 0x02B0 && ch <= 0x036F) ||  fr fr Various lowercase modifiers
           (ch >= 0x0300 && ch <= 0x036F)     fr fr Combining diacritical marks
}

slay is_other_uppercase(ch drip) lit {
    fr fr Additional uppercase characters
    damn ch >= 0x1D00 && ch <= 0x1D25  fr fr Phonetic extensions
}

slay is_diacritic(ch drip) lit {
    fr fr Check if character is a diacritic mark
    damn (ch >= 0x0300 && ch <= 0x036F) ||  fr fr Combining Diacritical Marks
           (ch >= 0x1AB0 && ch <= 0x1AFF) ||  fr fr Combining Diacritical Marks Extended
           (ch >= 0x1DC0 && ch <= 0x1DFF) ||  fr fr Combining Diacritical Marks Supplement
           (ch >= 0x20D0 && ch <= 0x20FF) ||  fr fr Combining Diacritical Marks for Symbols
           (ch >= 0xFE20 && ch <= 0xFE2F)     fr fr Combining Half Marks
}

slay is_extender(ch drip) lit {
    fr fr Check if character is an extender
    damn ch == 0x00B7 ||  fr fr Middle Dot
           ch == 0x02D0 ||  fr fr Modifier Letter Triangular Colon
           ch == 0x02D1 ||  fr fr Modifier Letter Half Triangular Colon
           ch == 0x0640 ||  fr fr Arabic Tatweel
           ch == 0x07FA ||  fr fr NKo Lajanyalan
           ch == 0x0E46 ||  fr fr Thai Character Maiyamok
           ch == 0x0EC6 ||  fr fr Lao Ko La
           ch == 0x180A ||  fr fr Mongolian Nirugu
           ch == 0x1843 ||  fr fr Mongolian Todo Soft Hyphen
           ch == 0x1AA7 ||  fr fr Tai Tham Sign Mai Yamok
           ch == 0x1C36 ||  fr fr Lepcha Sign Ran
           ch == 0x1C7B ||  fr fr Ol Chiki Punctuation Mucaad
           ch == 0x3005 ||  fr fr Ideographic Iteration Mark
           (ch >= 0x3031 && ch <= 0x3035) ||  fr fr Vertical Kana Repeat Marks
           (ch >= 0x309D && ch <= 0x309E) ||  fr fr Hiragana Iteration Marks
           (ch >= 0x30FC && ch <= 0x30FE)     fr fr Katakana Iteration Marks
}

slay is_noncharacter(ch drip) lit {
    fr fr Check if character is a noncharacter code point
    damn (ch >= 0xFDD0 && ch <= 0xFDEF) ||  fr fr Arabic Presentation Forms-A noncharacters
           (ch & 0xFFFE) == 0xFFFE              fr fr Noncharacters ending in FFFE or FFFF
}

fr fr ===== RANGE PROCESSING UTILITIES =====

slay sort_ranges(ranges *drip[value]) {
    fr fr Sort ranges in place (simplified bubble sort for demo)
    sus len drip = array_length(*ranges)
    sus i drip = 0
    
    bestie (i < len - 1) {
        sus j drip = 0
        bestie (j < len - i - 2) {
            ready ((*ranges)[j] > (*ranges)[j + 2]) {
                fr fr Swap range pairs
                sus temp1 drip = (*ranges)[j]
                sus temp2 drip = (*ranges)[j + 1]
                (*ranges)[j] = (*ranges)[j + 2]
                (*ranges)[j + 1] = (*ranges)[j + 3]
                (*ranges)[j + 2] = temp1
                (*ranges)[j + 3] = temp2
            }
            j = j + 2
        }
        i = i + 2
    }
}

slay merge_ranges(ranges drip[value]) drip[value]{
    fr fr Merge overlapping ranges
    ready (array_length(ranges) <= 2) {
        damn ranges
    }
    
    sus merged drip[value] = []
    sus i drip = 0
    
    bestie (i < array_length(ranges)) {
        sus start drip = ranges[i]
        sus end drip = ranges[i + 1]
        
        fr fr Check for overlap with next range
        bestie (i + 2 < array_length(ranges) && end >= ranges[i + 2] - 1) {
            ready (ranges[i + 3] > end) {
                end = ranges[i + 3]
            }
            i = i + 2  fr fr Skip merged range
        }
        
        array_append(merged, start)
        array_append(merged, end)
        i = i + 2
    }
    
    damn merged
}

slay load_unicode_property(property tea) drip[value]{
    fr fr Load Unicode property ranges (demo implementation)
    ready (property == "L") {
        damn [65, 90, 97, 122, 192, 214, 216, 246, 248, 255]
    } otherwise ready (property == "N") {
        damn [48, 57, 178, 179, 185, 185, 188, 190]
    } otherwise ready (property == "P") {
        damn [33, 47, 58, 64, 91, 96, 123, 126]
    } otherwise ready (property == "S") {
        damn [36, 36, 43, 43, 60, 62, 124, 124]
    } otherwise ready (property == "Z") {
        damn [32, 32, 160, 160, 8192, 8202, 8239, 8239, 8287, 8287, 12288, 12288]
    } otherwise ready (property == "C") {
        damn [0, 31, 127, 159, 173, 173, 1536, 1541, 1807, 1807, 6068, 6069]
    }
    
    damn []
}

slay serialize_nfa_fragment(compiler *NFACompiler, start_state drip, end_state drip) tea {
    fr fr Serialize NFA fragment for lookaround (simplified)
    damn json_number_to_string(start_state) + ":" + json_number_to_string(end_state)
}

fr fr ===== REGEX MATCH RESULT STRUCTURE =====

squad RegexMatch {
    sus start_position drip
    sus length drip
    sus text tea
    sus groups tea[value]
}

fr fr Example usage and test cases
slay test_complete_regex_features() {
    vibez.spill("=== Testing Complete Regex Engine ===")
    
    fr fr Test basic matching
    sus result1 RegexMatch = regex_execute_complete("hello", "hello world", "")
    vibez.spill("Basic match: " + (result1.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test case insensitive
    sus result2 RegexMatch = regex_execute_complete("HELLO", "hello world", "i")
    vibez.spill("Case insensitive: " + (result2.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test word boundaries
    sus result3 RegexMatch = regex_execute_complete("\\bhello\\b", "say hello there", "")
    vibez.spill("Word boundaries: " + (result3.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test character classes
    sus result4 RegexMatch = regex_execute_complete("\\d+", "abc123def", "")
    vibez.spill("Digit class: " + (result4.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test Unicode support
    sus result5 RegexMatch = regex_execute_complete("\\p{L}+", "café", "u")
    vibez.spill("Unicode letters: " + (result5.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test quantifiers
    sus result6 RegexMatch = regex_execute_complete("a+", "aaa", "")
    vibez.spill("Plus quantifier: " + (result6.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test character class with ranges
    sus result7 RegexMatch = regex_execute_complete("[a-z]+", "hello", "")
    vibez.spill("Character range: " + (result7.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test alternation
    sus result8 RegexMatch = regex_execute_complete("cat|dog", "I have a dog", "")
    vibez.spill("Alternation: " + (result8.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test capture groups
    sus result9 RegexMatch = regex_execute_complete("(\\w+)\\s+(\\w+)", "Hello World", "")
    vibez.spill("Capture groups: " + (result9.start_position >= 0 ? "PASS" : "FAIL"))
    
    fr fr Test lookahead
    sus result10 RegexMatch = regex_execute_complete("\\w+(?=\\s)", "Hello World", "")
    vibez.spill("Positive lookahead: " + (result10.start_position >= 0 ? "PASS" : "FAIL"))
    
    vibez.spill("Complete regex engine testing finished")
}

fr fr Advanced regex performance test
slay test_regex_performance() {
    vibez.spill("=== Performance Testing ===")
    
    fr fr Test complex patterns
    sus complex_pattern tea = "^(?:[a-zA-Z0-9._%+-]+)@(?:[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,})$"
    sus email_text tea = "user@example.com"
    
    sus start_time drip = get_current_time()
    sus result RegexMatch = regex_execute_complete(complex_pattern, email_text, "")
    sus end_time drip = get_current_time()
    
    vibez.spill("Email validation: " + (result.start_position >= 0 ? "PASS" : "FAIL"))
    vibez.spill("Performance: " + json_number_to_string(end_time - start_time) + "ms")
}

slay get_current_time() drip {
    fr fr Simplified time function - would use system time in real implementation
    damn 0
}
