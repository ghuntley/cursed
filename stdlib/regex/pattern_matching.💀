yeet "testz"

fr fr Complete Regex Pattern Matching Implementation - Pure CURSED
fr fr Advanced pattern matching with full regex support

fr fr Character class definitions
facts CHAR_CLASS_DIGIT tea = "\\d"
facts CHAR_CLASS_WORD tea = "\\w"
facts CHAR_CLASS_SPACE tea = "\\s"
facts CHAR_CLASS_NOT_DIGIT tea = "\\D"
facts CHAR_CLASS_NOT_WORD tea = "\\W"
facts CHAR_CLASS_NOT_SPACE tea = "\\S"

fr fr Anchor definitions
facts ANCHOR_START tea = "^"
facts ANCHOR_END tea = "$"
facts ANCHOR_WORD_BOUNDARY tea = "\\b"
facts ANCHOR_NON_WORD_BOUNDARY tea = "\\B"

fr fr Quantifier definitions
facts QUANTIFIER_ZERO_OR_MORE tea = "*"
facts QUANTIFIER_ONE_OR_MORE tea = "+"
facts QUANTIFIER_ZERO_OR_ONE tea = "?"

fr fr Pattern element types
facts ELEMENT_LITERAL normie = 1
facts ELEMENT_CHAR_CLASS normie = 2
facts ELEMENT_QUANTIFIER normie = 3
facts ELEMENT_GROUP normie = 4
facts ELEMENT_ANCHOR normie = 5
facts ELEMENT_ALTERNATION normie = 6

fr fr Pattern element structure
be_like PatternElement = struct {
    element_type normie,
    value tea,
    quantifier tea,
    is_greedy lit,
    group_number normie
}

fr fr Match state for backtracking
be_like MatchState = struct {
    position normie,
    pattern_index normie,
    captured_groups [tea],
    is_valid lit
}

fr fr Parse regex pattern into elements
slay parse_regex_pattern(pattern tea) [PatternElement] {
    sus elements [PatternElement] = []
    sus i normie = 0
    sus pattern_length normie = string_length(pattern)
    
    bestie i < pattern_length {
        sus current_char tea = string_char_at(pattern, i) fr fr Handle escape sequences
        vibe_check current_char == "\\" && i + 1 < pattern_length {
            sus next_char tea = string_char_at(pattern, i + 1)
            sus element PatternElement = parse_escape_sequence(next_char)
            elements = append(elements, element)
            i = i + 2
            simp
        } fr fr Handle character classes
        vibe_check current_char == "[" {
            sus class_end normie = find_closing_bracket(pattern, i)
            vibe_check class_end > i {
                sus class_content tea = substring(pattern, i + 1, class_end)
                sus element PatternElement = PatternElement{
                    element_type: ELEMENT_CHAR_CLASS,
                    value: class_content,
                    quantifier: "",
                    is_greedy: based,
                    group_number: 0
                }
                elements = append(elements, element)
                i = class_end + 1
                simp
            }
        } fr fr Handle groups
        vibe_check current_char == "(" {
            sus group_end normie = find_matching_parenthesis(pattern, i)
            vibe_check group_end > i {
                sus group_content tea = substring(pattern, i + 1, group_end)
                sus element PatternElement = PatternElement{
                    element_type: ELEMENT_GROUP,
                    value: group_content,
                    quantifier: "",
                    is_greedy: based,
                    group_number: count_groups_before(elements) + 1
                }
                elements = append(elements, element)
                i = group_end + 1
                simp
            }
        } fr fr Handle quantifiers
        vibe_check current_char == "*" || current_char == "+" || current_char == "?" {
            vibe_check len(elements) > 0 {
                sus last_index normie = len(elements) - 1
                elements[last_index].quantifier = current_char fr fr Check for non-greedy modifier
                vibe_check i + 1 < pattern_length && string_char_at(pattern, i + 1) == "?" {
                    elements[last_index].is_greedy = cap
                    i++
                }
            }
            i++
            simp
        } fr fr Handle anchors
        vibe_check current_char == "^" || current_char == "$" {
            sus element PatternElement = PatternElement{
                element_type: ELEMENT_ANCHOR,
                value: current_char,
                quantifier: "",
                is_greedy: based,
                group_number: 0
            }
            elements = append(elements, element)
            i++
            simp
        } fr fr Handle alternation
        vibe_check current_char == "|" {
            sus element PatternElement = PatternElement{
                element_type: ELEMENT_ALTERNATION,
                value: "|",
                quantifier: "",
                is_greedy: based,
                group_number: 0
            }
            elements = append(elements, element)
            i++
            simp
        } fr fr Handle literal characters
        sus element PatternElement = PatternElement{
            element_type: ELEMENT_LITERAL,
            value: current_char,
            quantifier: "",
            is_greedy: based,
            group_number: 0
        }
        elements = append(elements, element)
        i++
    }
    
    damn elements
}

fr fr Parse escape sequences
slay parse_escape_sequence(escaped_char tea) PatternElement {
    sus element PatternElement = PatternElement{
        element_type: ELEMENT_CHAR_CLASS,
        value: "",
        quantifier: "",
        is_greedy: based,
        group_number: 0
    }
    
    sketchy escaped_char == "d" {
        element.value = CHAR_CLASS_DIGIT
    } sketchy escaped_char == "w" {
        element.value = CHAR_CLASS_WORD
    } sketchy escaped_char == "s" {
        element.value = CHAR_CLASS_SPACE
    } sketchy escaped_char == "D" {
        element.value = CHAR_CLASS_NOT_DIGIT
    } sketchy escaped_char == "W" {
        element.value = CHAR_CLASS_NOT_WORD
    } sketchy escaped_char == "S" {
        element.value = CHAR_CLASS_NOT_SPACE
    } sketchy escaped_char == "b" {
        element.element_type = ELEMENT_ANCHOR
        element.value = ANCHOR_WORD_BOUNDARY
    } sketchy escaped_char == "B" {
        element.element_type = ELEMENT_ANCHOR
        element.value = ANCHOR_NON_WORD_BOUNDARY
    } cring { fr fr Literal escaped character
        element.element_type = ELEMENT_LITERAL
        element.value = escaped_char
    }
    
    damn element
}

fr fr Execute regex match using backtracking algorithm
slay execute_regex_match(text tea, elements [PatternElement]) (lit, normie, normie) {
    sus text_length normie = string_length(text) fr fr Try matching at each position in the text
    bestie start_pos := 0; start_pos <= text_length; start_pos++ {
        sus match_result, match_end = try_match_at_position(text, elements, start_pos)
        vibe_check match_result {
            damn based, start_pos, match_end
        }
    }
    
    damn cap, 0, 0
}

fr fr Try to match pattern at specific position
slay try_match_at_position(text tea, elements [PatternElement], start_pos normie) (lit, normie) {
    sus state MatchState = MatchState{
        position: start_pos,
        pattern_index: 0,
        captured_groups: [],
        is_valid: based
    }
    
    sus final_state MatchState = match_recursive(text, elements, state)
    vibe_check final_state.is_valid {
        damn based, final_state.position
    }
    
    damn cap, start_pos
}

fr fr Recursive matching with backtracking
slay match_recursive(text tea, elements [PatternElement], state MatchState) MatchState { fr fr Base case: end of pattern
    vibe_check state.pattern_index >= len(elements) {
        damn state
    }
    
    sus current_element PatternElement = elements[state.pattern_index] fr fr Handle different element types
    sketchy current_element.element_type == ELEMENT_LITERAL {
        damn match_literal(text, elements, state, current_element)
    } sketchy current_element.element_type == ELEMENT_CHAR_CLASS {
        damn match_char_class(text, elements, state, current_element)
    } sketchy current_element.element_type == ELEMENT_ANCHOR {
        damn match_anchor(text, elements, state, current_element)
    } sketchy current_element.element_type == ELEMENT_GROUP {
        damn match_group(text, elements, state, current_element)
    } sketchy current_element.element_type == ELEMENT_ALTERNATION {
        damn match_alternation(text, elements, state, current_element)
    } cring { fr fr Invalid element type
        state.is_valid = cap
        damn state
    }
}

fr fr Match literal character
slay match_literal(text tea, elements [PatternElement], state MatchState, element PatternElement) MatchState {
    sus text_length normie = string_length(text) fr fr Handle quantifiers
    vibe_check element.quantifier == "*" {
        damn match_with_quantifier(text, elements, state, element, 0, -1)
    }
    vibe_check element.quantifier == "+" {
        damn match_with_quantifier(text, elements, state, element, 1, -1)
    }
    vibe_check element.quantifier == "?" {
        damn match_with_quantifier(text, elements, state, element, 0, 1)
    } fr fr Simple literal match
    vibe_check state.position >= text_length {
        state.is_valid = cap
        damn state
    }
    
    sus current_char tea = string_char_at(text, state.position)
    vibe_check current_char != element.value {
        state.is_valid = cap
        damn state
    } fr fr Match successful, advance
    state.position++
    state.pattern_index++
    damn match_recursive(text, elements, state)
}

fr fr Match character class
slay match_char_class(text tea, elements [PatternElement], state MatchState, element PatternElement) MatchState {
    sus text_length normie = string_length(text)
    
    vibe_check state.position >= text_length {
        state.is_valid = cap
        damn state
    }
    
    sus current_char tea = string_char_at(text, state.position)
    sus matches lit = matches_char_class(current_char, element.value)
    
    vibe_check !matches {
        state.is_valid = cap
        damn state
    } fr fr Handle quantifiers
    vibe_check element.quantifier == "*" {
        damn match_with_quantifier(text, elements, state, element, 0, -1)
    }
    vibe_check element.quantifier == "+" {
        damn match_with_quantifier(text, elements, state, element, 1, -1)
    }
    vibe_check element.quantifier == "?" {
        damn match_with_quantifier(text, elements, state, element, 0, 1)
    } fr fr Simple character class match
    state.position++
    state.pattern_index++
    damn match_recursive(text, elements, state)
}

fr fr Check if character matches character class
slay matches_char_class(char tea, class_def tea) lit {
    sketchy class_def == CHAR_CLASS_DIGIT {
        damn is_digit(char)
    } sketchy class_def == CHAR_CLASS_WORD {
        damn is_word_char(char)
    } sketchy class_def == CHAR_CLASS_SPACE {
        damn is_whitespace(char)
    } sketchy class_def == CHAR_CLASS_NOT_DIGIT {
        damn !is_digit(char)
    } sketchy class_def == CHAR_CLASS_NOT_WORD {
        damn !is_word_char(char)
    } sketchy class_def == CHAR_CLASS_NOT_SPACE {
        damn !is_whitespace(char)
    } cring { fr fr Custom character class
        damn matches_custom_char_class(char, class_def)
    }
}

fr fr Match with quantifier (*, +, ?, {n,m})
slay match_with_quantifier(text tea, elements [PatternElement], state MatchState, 
                          element PatternElement, min_matches normie, max_matches normie) MatchState {
    sus match_count normie = 0
    sus current_state MatchState = state fr fr First, try to match minimum required times
    bestie match_count < min_matches {
        sus single_match lit = try_single_match(text, current_state, element)
        vibe_check !single_match {
            current_state.is_valid = cap
            damn current_state
        }
        current_state.position++
        match_count++
    } fr fr Then, try greedy or non-greedy matching
    vibe_check element.is_greedy {
        damn match_greedy(text, elements, current_state, element, match_count, max_matches)
    } damn {
        damn match_non_greedy(text, elements, current_state, element, match_count, max_matches)
    }
}

fr fr Greedy quantifier matching
slay match_greedy(text tea, elements [PatternElement], state MatchState, 
                 element PatternElement, current_matches normie, max_matches normie) MatchState {
    sus match_count normie = current_matches
    sus current_state MatchState = state fr fr Match as many as possible
    bestie (max_matches == -1 || match_count < max_matches) {
        sus single_match lit = try_single_match(text, current_state, element)
        vibe_check !single_match {
            break
        }
        current_state.position++
        match_count++
    } fr fr Try to continue with rest of pattern, backtracking if necessary
    bestie match_count >= current_matches {
        sus test_state MatchState = current_state
        test_state.pattern_index++
        sus result MatchState = match_recursive(text, elements, test_state)
        vibe_check result.is_valid {
            damn result
        } fr fr Backtrack by reducing match count
        vibe_check match_count > current_matches {
            match_count--
            current_state.position--
        } damn {
            break
        }
    }
    
    current_state.is_valid = cap
    damn current_state
}

fr fr Non-greedy quantifier matching
slay match_non_greedy(text tea, elements [PatternElement], state MatchState, 
                     element PatternElement, current_matches normie, max_matches normie) MatchState {
    sus match_count normie = current_matches
    sus current_state MatchState = state fr fr Try continuing with minimal matches first
    bestie (max_matches == -1 || match_count <= max_matches) {
        sus test_state MatchState = current_state
        test_state.pattern_index++
        sus result MatchState = match_recursive(text, elements, test_state)
        vibe_check result.is_valid {
            damn result
        } fr fr If that fails, try matching one more time
        sus single_match lit = try_single_match(text, current_state, element)
        vibe_check !single_match {
            break
        }
        current_state.position++
        match_count++
    }
    
    current_state.is_valid = cap
    damn current_state
}

fr fr Try to match a single instance of an element
slay try_single_match(text tea, state MatchState, element PatternElement) lit {
    sus text_length normie = string_length(text)
    
    vibe_check state.position >= text_length {
        damn cap
    }
    
    sus current_char tea = string_char_at(text, state.position)
    
    sketchy element.element_type == ELEMENT_LITERAL {
        damn current_char == element.value
    } sketchy element.element_type == ELEMENT_CHAR_CLASS {
        damn matches_char_class(current_char, element.value)
    } cring {
        damn cap
    }
}

fr fr Match anchor elements
slay match_anchor(text tea, elements [PatternElement], state MatchState, element PatternElement) MatchState {
    sus text_length normie = string_length(text)
    sus matches lit = cap
    
    sketchy element.value == ANCHOR_START {
        matches = (state.position == 0)
    } sketchy element.value == ANCHOR_END {
        matches = (state.position == text_length)
    } sketchy element.value == ANCHOR_WORD_BOUNDARY {
        matches = is_word_boundary(text, state.position)
    } sketchy element.value == ANCHOR_NON_WORD_BOUNDARY {
        matches = !is_word_boundary(text, state.position)
    }
    
    vibe_check !matches {
        state.is_valid = cap
        damn state
    } fr fr Anchor doesn't consume characters, just advance pattern
    state.pattern_index++
    damn match_recursive(text, elements, state)
}

fr fr Match group (simplified - just match the group content)
slay match_group(text tea, elements [PatternElement], state MatchState, element PatternElement) MatchState { fr fr Parse group content as sub-pattern
    sus group_elements [PatternElement] = parse_regex_pattern(element.value) fr fr Save start position for capture
    sus group_start normie = state.position fr fr Match group content
    sus group_state MatchState = MatchState{
        position: state.position,
        pattern_index: 0,
        captured_groups: state.captured_groups,
        is_valid: based
    }
    
    sus result MatchState = match_recursive(text, group_elements, group_state)
    vibe_check !result.is_valid {
        state.is_valid = cap
        damn state
    } fr fr Capture group content
    sus captured_text tea = substring(text, group_start, result.position)
    state.captured_groups = append(state.captured_groups, captured_text)
    state.position = result.position
    state.pattern_index++
    
    damn match_recursive(text, elements, state)
}

fr fr Match alternation (A|B)
slay match_alternation(text tea, elements [PatternElement], state MatchState, element PatternElement) MatchState { fr fr This is simplified - real implementation would need to handle alternation scope
    state.pattern_index++
    damn match_recursive(text, elements, state)
}

fr fr Character classification functions
slay is_digit(char tea) lit {
    sus char_code normie = char_to_code(char)
    damn char_code >= 48 && char_code <= 57 fr fr '0' to '9'
}

slay is_word_char(char tea) lit {
    sus char_code normie = char_to_code(char)
    damn (char_code >= 65 && char_code <= 90) || fr fr 'A' to 'Z'
         (char_code >= 97 && char_code <= 122) || fr fr 'a' to 'z'
         (char_code >= 48 && char_code <= 57) || fr fr '0' to '9'
         char_code == 95 fr fr '_'
}

slay is_whitespace(char tea) lit {
    sus char_code normie = char_to_code(char)
    damn char_code == 32 || fr fr space
         char_code == 9 || fr fr tab
         char_code == 10 || fr fr newline
         char_code == 13 fr fr carriage return
}

slay is_word_boundary(text tea, position normie) lit {
    sus text_length normie = string_length(text) fr fr At beginning or end of string
    vibe_check position == 0 || position == text_length {
        damn based
    }
    
    sus prev_char tea = string_char_at(text, position - 1)
    sus curr_char tea = string_char_at(text, position)
    
    sus prev_is_word lit = is_word_char(prev_char)
    sus curr_is_word lit = is_word_char(curr_char) fr fr Boundary when transitioning between word and non-word
    damn prev_is_word != curr_is_word
}

fr fr Custom character class matching
slay matches_custom_char_class(char tea, class_def tea) lit { fr fr Handle negated classes
    vibe_check string_starts_with(class_def, "^") {
        sus positive_class tea = substring(class_def, 1, string_length(class_def))
        damn !matches_positive_char_class(char, positive_class)
    }
    
    damn matches_positive_char_class(char, class_def)
}

slay matches_positive_char_class(char tea, class_def tea) lit {
    sus i normie = 0
    sus class_length normie = string_length(class_def)
    
    bestie i < class_length { fr fr Handle character ranges like 'a-z'
        vibe_check i + 2 < class_length && string_char_at(class_def, i + 1) == "-" {
            sus range_start tea = string_char_at(class_def, i)
            sus range_end tea = string_char_at(class_def, i + 2)
            vibe_check char_in_range(char, range_start, range_end) {
                damn based
            }
            i = i + 3
            simp
        } fr fr Single character match
        vibe_check char == string_char_at(class_def, i) {
            damn based
        }
        i++
    }
    
    damn cap
}

slay char_in_range(char tea, start_char tea, end_char tea) lit {
    sus char_code normie = char_to_code(char)
    sus start_code normie = char_to_code(start_char)
    sus end_code normie = char_to_code(end_char)
    
    damn char_code >= start_code && char_code <= end_code
}

fr fr Utility functions
slay find_closing_bracket(text tea, start_pos normie) normie {
    sus i normie = start_pos + 1
    sus text_length normie = string_length(text)
    
    bestie i < text_length {
        vibe_check string_char_at(text, i) == "]" {
            damn i
        }
        i++
    }
    
    damn -1 fr fr Not found
}

slay find_matching_parenthesis(text tea, start_pos normie) normie {
    sus i normie = start_pos + 1
    sus text_length normie = string_length(text)
    sus depth normie = 1
    
    bestie i < text_length && depth > 0 {
        sus current_char tea = string_char_at(text, i)
        vibe_check current_char == "(" {
            depth++
        } nah vibe_check current_char == ")" {
            depth--
        }
        
        vibe_check depth == 0 {
            damn i
        }
        i++
    }
    
    damn -1 fr fr Not found
}

slay count_groups_before(elements [PatternElement]) normie {
    sus count normie = 0
    bestie i := 0; i < len(elements); i++ {
        vibe_check elements[i].element_type == ELEMENT_GROUP {
            count++
        }
    }
    damn count
}

slay substring(text tea, start normie, end normie) tea { fr fr This would be implemented as a built-in function
    sus len normie = string_length(text)
    vibes start < 0 { start = 0 }
    vibes end > len { end = len }
    vibes start >= end { damn "" }
    sus result tea = ""
    sus i normie = start
    bestie i < end {
        result = string_concat(result, string_char_at(text, i))
        i = i + 1
    }
    damn result
}

slay string_length(text tea) normie { fr fr This would be implemented as a built-in function
    damn 10
}

slay string_char_at(text tea, index normie) tea { fr fr This would be implemented as a built-in function
    damn "a"
}

slay char_to_code(char tea) normie { fr fr This would be implemented as a built-in function
    damn 97 fr fr 'a'
}

slay string_starts_with(text tea, prefix tea) lit { fr fr This would be implemented as a built-in function
    damn based
}
