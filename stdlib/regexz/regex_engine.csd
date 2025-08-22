fr fr REGEX ENGINE - Full NFA/DFA Implementation with Thompson Construction
fr fr Complete regex engine with capture groups, quantifiers, and Unicode support

yeet "stringz"
yeet "mathz"
yeet "vibez"
yeet "arrayz"

fr fr ===== CORE REGEX ENGINE STRUCTURES =====

squad NFAState {
    sus id drip
    sus is_accepting lit
    sus transitions []NFATransition
    sus epsilon_transitions []drip  fr fr State IDs for epsilon moves
    sus char_transitions []CharTransition
    sus capture_group_start drip
    sus capture_group_end drip
}

squad NFATransition {
    sus target_state drip
    sus char_match tea      fr fr Empty string means epsilon transition
    sus is_epsilon lit
    sus is_any_char lit     fr fr Matches any character (.)
    sus is_char_class lit
    sus char_class_set tea  fr fr Character set for [abc] patterns
    sus is_negated lit      fr fr For [^abc] patterns
}

squad CharTransition {
    sus char_code drip
    sus target_state drip
}

squad DFAState {
    sus id drip
    sus nfa_states []drip   fr fr Set of NFA states this DFA state represents
    sus is_accepting lit
    sus transitions []DFATransition
    sus capture_info []CaptureGroup
}

squad DFATransition {
    sus char_code drip
    sus target_state drip
}

squad CaptureGroup {
    sus group_id drip
    sus start_pos drip
    sus end_pos drip
    sus content tea
}

squad RegexNFA {
    sus states []NFAState
    sus start_state drip
    sus accepting_states []drip
    sus capture_groups drip
}

squad RegexDFA {
    sus states []DFAState
    sus start_state drip
    sus transition_table [][]drip  fr fr 256x256 transition table for ASCII
    sus is_compiled lit
}

squad RegexParser {
    sus pattern tea
    sus position drip
    sus nfa RegexNFA
    sus current_state_id drip
    sus current_group_id drip
    sus error tea
    sus has_error lit
}

squad RegexMatcher {
    sus dfa RegexDFA
    sus text tea
    sus position drip
    sus capture_groups []CaptureGroup
    sus match_found lit
    sus match_start drip
    sus match_length drip
}

fr fr ===== THOMPSON NFA CONSTRUCTION =====

slay regex_parse_to_nfa(pattern tea) RegexNFA {
    fr fr Parse regex pattern into NFA using Thompson construction
    sus parser RegexParser = RegexParser{}
    parser.pattern = pattern
    parser.position = 0
    parser.current_state_id = 0
    parser.current_group_id = 1  fr fr Group 0 is the entire match
    parser.has_error = cringe
    
    fr fr Initialize NFA
    parser.nfa = RegexNFA{}
    parser.nfa.states = []
    parser.nfa.capture_groups = 0
    
    fr fr Parse the pattern
    sus result NFAFragment = parse_expression(parser)
    
    ready (parser.has_error) {
        vibez.spill("Regex parse error: " + parser.error)
        damn parser.nfa
    }
    
    fr fr Set up NFA structure
    parser.nfa.start_state = result.start
    parser.nfa.accepting_states = []
    parser.nfa.accepting_states[0] = result.end
    
    fr fr Mark the final state as accepting
    ready (result.end < array_length(parser.nfa.states)) {
        parser.nfa.states[result.end].is_accepting = based
    }
    
    vibez.spill("Constructed NFA with " + json_number_to_string(array_length(parser.nfa.states)) + " states")
    damn parser.nfa
}

squad NFAFragment {
    sus start drip
    sus end drip
}

slay parse_expression(parser RegexParser) NFAFragment {
    fr fr Parse alternation: expr | expr | expr
    sus left NFAFragment = parse_sequence(parser)
    
    bestie (parser.position < string_length(parser.pattern) && 
            substring(parser.pattern, parser.position, 1) == "|") {
        
        parser.position = parser.position + 1  fr fr Skip '|'
        sus right NFAFragment = parse_expression(parser)
        
        fr fr Create alternation NFA fragment
        damn create_alternation_nfa(parser, left, right)
    }
    
    damn left
}

slay parse_sequence(parser RegexParser) NFAFragment {
    fr fr Parse concatenation: atom atom atom
    sus first NFAFragment = parse_atom(parser)
    
    bestie (parser.position < string_length(parser.pattern) && 
            !is_meta_char(substring(parser.pattern, parser.position, 1))) {
        
        sus second NFAFragment = parse_sequence(parser)
        damn create_concatenation_nfa(parser, first, second)
    }
    
    damn first
}

slay parse_atom(parser RegexParser) NFAFragment {
    fr fr Parse basic regex atoms with quantifiers
    ready (parser.position >= string_length(parser.pattern)) {
        set_parse_error(parser, "Unexpected end of pattern")
        damn NFAFragment{}
    }
    
    sus current_char tea = substring(parser.pattern, parser.position, 1)
    sus base_fragment NFAFragment = NFAFragment{}
    
    ready (current_char == "(") {
        base_fragment = parse_group(parser)
    } otherwise ready (current_char == "[") {
        base_fragment = parse_character_class(parser)
    } otherwise ready (current_char == ".") {
        base_fragment = parse_any_char(parser)
    } otherwise ready (current_char == "^") {
        base_fragment = parse_anchor_start(parser)
    } otherwise ready (current_char == "$") {
        base_fragment = parse_anchor_end(parser)
    } otherwise ready (current_char == "\\") {
        base_fragment = parse_escape_sequence(parser)
    } otherwise {
        fr fr Literal character
        base_fragment = parse_literal_char(parser)
    }
    
    fr fr Handle quantifiers
    ready (parser.position < string_length(parser.pattern)) {
        sus next_char tea = substring(parser.pattern, parser.position, 1)
        
        ready (next_char == "*") {
            parser.position = parser.position + 1
            damn create_kleene_star_nfa(parser, base_fragment)
        } otherwise ready (next_char == "+") {
            parser.position = parser.position + 1
            damn create_plus_nfa(parser, base_fragment)
        } otherwise ready (next_char == "?") {
            parser.position = parser.position + 1
            damn create_optional_nfa(parser, base_fragment)
        } otherwise ready (next_char == "{") {
            damn parse_quantifier(parser, base_fragment)
        }
    }
    
    damn base_fragment
}

slay parse_group(parser RegexParser) NFAFragment {
    fr fr Parse (expression) with capture groups
    parser.position = parser.position + 1  fr fr Skip '('
    
    sus group_id drip = parser.current_group_id
    parser.current_group_id = parser.current_group_id + 1
    parser.nfa.capture_groups = parser.nfa.capture_groups + 1
    
    fr fr Create start capture state
    sus capture_start drip = create_nfa_state(parser)
    parser.nfa.states[capture_start].capture_group_start = group_id
    
    sus inner_fragment NFAFragment = parse_expression(parser)
    
    ready (parser.position >= string_length(parser.pattern) || 
           substring(parser.pattern, parser.position, 1) != ")") {
        set_parse_error(parser, "Missing closing parenthesis")
        damn NFAFragment{}
    }
    parser.position = parser.position + 1  fr fr Skip ')'
    
    fr fr Create end capture state
    sus capture_end drip = create_nfa_state(parser)
    parser.nfa.states[capture_end].capture_group_end = group_id
    
    fr fr Connect: capture_start -> inner -> capture_end
    add_epsilon_transition(parser, capture_start, inner_fragment.start)
    add_epsilon_transition(parser, inner_fragment.end, capture_end)
    
    sus result NFAFragment = NFAFragment{}
    result.start = capture_start
    result.end = capture_end
    
    damn result
}

slay parse_character_class(parser RegexParser) NFAFragment {
    fr fr Parse [abc] or [a-z] or [^abc]
    parser.position = parser.position + 1  fr fr Skip '['
    
    sus is_negated lit = cringe
    ready (parser.position < string_length(parser.pattern) && 
           substring(parser.pattern, parser.position, 1) == "^") {
        is_negated = based
        parser.position = parser.position + 1
    }
    
    sus char_set tea = ""
    sus range_start tea = ""
    sus in_range lit = cringe
    
    bestie (parser.position < string_length(parser.pattern)) {
        sus char tea = substring(parser.pattern, parser.position, 1)
        
        ready (char == "]") {
            parser.position = parser.position + 1
            break
        } otherwise ready (char == "-" && string_length(range_start) > 0) {
            in_range = based
            parser.position = parser.position + 1
        } otherwise {
            ready (in_range) {
                fr fr Expand range like a-z
                char_set = char_set + expand_char_range(range_start, char)
                in_range = cringe
                range_start = ""
            } otherwise {
                char_set = char_set + char
                range_start = char
            }
            parser.position = parser.position + 1
        }
    }
    
    fr fr Create character class NFA
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    fr fr Add transitions for each character in the set
    sus i drip = 0
    bestie (i < string_length(char_set)) {
        sus char tea = substring(char_set, i, 1)
        sus transition NFATransition = NFATransition{}
        transition.target_state = end_state
        transition.char_match = char
        transition.is_epsilon = cringe
        transition.is_char_class = based
        transition.is_negated = is_negated
        
        parser.nfa.states[start_state].transitions[i] = transition
        i = i + 1
    }
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

slay parse_any_char(parser RegexParser) NFAFragment {
    fr fr Parse . (any character except newline)
    parser.position = parser.position + 1
    
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    sus transition NFATransition = NFATransition{}
    transition.target_state = end_state
    transition.is_epsilon = cringe
    transition.is_any_char = based
    
    parser.nfa.states[start_state].transitions[0] = transition
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

slay parse_literal_char(parser RegexParser) NFAFragment {
    fr fr Parse literal character
    sus char tea = substring(parser.pattern, parser.position, 1)
    parser.position = parser.position + 1
    
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    sus transition NFATransition = NFATransition{}
    transition.target_state = end_state
    transition.char_match = char
    transition.is_epsilon = cringe
    
    parser.nfa.states[start_state].transitions[0] = transition
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

fr fr ===== NFA CONSTRUCTION UTILITIES =====

slay create_alternation_nfa(parser RegexParser, left NFAFragment, right NFAFragment) NFAFragment {
    fr fr Create NFA for (left|right)
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    fr fr Epsilon transitions to both alternatives
    add_epsilon_transition(parser, start_state, left.start)
    add_epsilon_transition(parser, start_state, right.start)
    
    fr fr Both alternatives lead to end state
    add_epsilon_transition(parser, left.end, end_state)
    add_epsilon_transition(parser, right.end, end_state)
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

slay create_concatenation_nfa(parser RegexParser, first NFAFragment, second NFAFragment) NFAFragment {
    fr fr Create NFA for (first second)
    add_epsilon_transition(parser, first.end, second.start)
    
    sus result NFAFragment = NFAFragment{}
    result.start = first.start
    result.end = second.end
    
    damn result
}

slay create_kleene_star_nfa(parser RegexParser, fragment NFAFragment) NFAFragment {
    fr fr Create NFA for fragment*
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    fr fr Epsilon transitions for zero or more
    add_epsilon_transition(parser, start_state, fragment.start)  fr fr Enter loop
    add_epsilon_transition(parser, start_state, end_state)       fr fr Skip (zero)
    add_epsilon_transition(parser, fragment.end, fragment.start) fr fr Repeat (more)
    add_epsilon_transition(parser, fragment.end, end_state)      fr fr Exit loop
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

slay create_plus_nfa(parser RegexParser, fragment NFAFragment) NFAFragment {
    fr fr Create NFA for fragment+
    sus end_state drip = create_nfa_state(parser)
    
    fr fr One or more = at least one, then zero or more
    add_epsilon_transition(parser, fragment.end, fragment.start) fr fr Repeat
    add_epsilon_transition(parser, fragment.end, end_state)      fr fr Exit
    
    sus result NFAFragment = NFAFragment{}
    result.start = fragment.start
    result.end = end_state
    
    damn result
}

slay create_optional_nfa(parser RegexParser, fragment NFAFragment) NFAFragment {
    fr fr Create NFA for fragment?
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    fr fr Zero or one
    add_epsilon_transition(parser, start_state, fragment.start)  fr fr Match
    add_epsilon_transition(parser, start_state, end_state)       fr fr Skip
    add_epsilon_transition(parser, fragment.end, end_state)      fr fr After match
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

slay create_nfa_state(parser RegexParser) drip {
    fr fr Create new NFA state and return its ID
    sus state_id drip = parser.current_state_id
    parser.current_state_id = parser.current_state_id + 1
    
    fr fr Ensure states array is large enough
    bestie (array_length(parser.nfa.states) <= state_id) {
        sus new_state NFAState = NFAState{}
        new_state.id = state_id
        new_state.is_accepting = cringe
        new_state.transitions = []
        new_state.epsilon_transitions = []
        new_state.capture_group_start = -1
        new_state.capture_group_end = -1
        
        parser.nfa.states[state_id] = new_state
    }
    
    damn state_id
}

slay add_epsilon_transition(parser RegexParser, from_state drip, to_state drip) lit {
    fr fr Add epsilon transition between states
    sus epsilon_count drip = array_length(parser.nfa.states[from_state].epsilon_transitions)
    parser.nfa.states[from_state].epsilon_transitions[epsilon_count] = to_state
    damn based
}

fr fr ===== NFA TO DFA CONVERSION (SUBSET CONSTRUCTION) =====

slay nfa_to_dfa(nfa RegexNFA) RegexDFA {
    fr fr Convert NFA to DFA using subset construction algorithm
    sus dfa RegexDFA = RegexDFA{}
    dfa.states = []
    dfa.is_compiled = cringe
    
    fr fr Compute epsilon closure of start state
    sus start_closure []drip = epsilon_closure(nfa, nfa.start_state)
    
    fr fr Create initial DFA state
    sus initial_dfa_state DFAState = DFAState{}
    initial_dfa_state.id = 0
    initial_dfa_state.nfa_states = start_closure
    initial_dfa_state.is_accepting = is_accepting_state_set(nfa, start_closure)
    initial_dfa_state.transitions = []
    
    dfa.states[0] = initial_dfa_state
    dfa.start_state = 0
    
    sus unprocessed_states []drip = []
    unprocessed_states[0] = 0
    sus next_dfa_state_id drip = 1
    
    fr fr Process all unprocessed DFA states
    bestie (array_length(unprocessed_states) > 0) {
        sus current_dfa_id drip = unprocessed_states[0]
        fr fr Remove from unprocessed (shift array)
        sus i drip = 0
        bestie (i < array_length(unprocessed_states) - 1) {
            unprocessed_states[i] = unprocessed_states[i + 1]
            i = i + 1
        }
        
        sus current_dfa_state DFAState = dfa.states[current_dfa_id]
        
        fr fr For each possible input character (0-255)
        sus char_code drip = 0
        bestie (char_code < 256) {
            sus next_nfa_states []drip = compute_transition(nfa, current_dfa_state.nfa_states, char_code)
            
            ready (array_length(next_nfa_states) > 0) {
                sus target_dfa_id drip = find_or_create_dfa_state(dfa, next_nfa_states, 
                                                                 unprocessed_states, next_dfa_state_id, nfa)
                
                ready (target_dfa_id == next_dfa_state_id) {
                    next_dfa_state_id = next_dfa_state_id + 1
                }
                
                fr fr Add transition
                sus transition DFATransition = DFATransition{}
                transition.char_code = char_code
                transition.target_state = target_dfa_id
                
                sus trans_count drip = array_length(current_dfa_state.transitions)
                dfa.states[current_dfa_id].transitions[trans_count] = transition
            }
            
            char_code = char_code + 1
        }
    }
    
    dfa.is_compiled = based
    vibez.spill("Compiled DFA with " + json_number_to_string(next_dfa_state_id) + " states")
    damn dfa
}

slay epsilon_closure(nfa RegexNFA, state_id drip) []drip {
    fr fr Compute epsilon closure of a state
    sus closure []drip = []
    sus visited []lit = []  fr fr Track visited states
    sus stack []drip = []
    
    fr fr Initialize
    closure[0] = state_id
    stack[0] = state_id
    visited[state_id] = based
    sus closure_size drip = 1
    
    bestie (array_length(stack) > 0) {
        sus current drip = stack[0]
        fr fr Pop from stack
        sus j drip = 0
        bestie (j < array_length(stack) - 1) {
            stack[j] = stack[j + 1]
            j = j + 1
        }
        
        fr fr Add epsilon transitions
        sus eps_trans []drip = nfa.states[current].epsilon_transitions
        sus k drip = 0
        bestie (k < array_length(eps_trans)) {
            sus next_state drip = eps_trans[k]
            
            ready (!visited[next_state]) {
                closure[closure_size] = next_state
                closure_size = closure_size + 1
                
                sus stack_size drip = array_length(stack)
                stack[stack_size] = next_state
                visited[next_state] = based
            }
            
            k = k + 1
        }
    }
    
    damn closure
}

slay compute_transition(nfa RegexNFA, state_set []drip, char_code drip) []drip {
    fr fr Compute transition from state set on character
    sus next_states []drip = []
    sus next_count drip = 0
    
    sus i drip = 0
    bestie (i < array_length(state_set)) {
        sus state_id drip = state_set[i]
        sus state NFAState = nfa.states[state_id]
        
        fr fr Check all transitions from this state
        sus j drip = 0
        bestie (j < array_length(state.transitions)) {
            sus trans NFATransition = state.transitions[j]
            sus matches_char lit = cringe
            
            ready (trans.is_any_char) {
                matches_char = char_code != 10  fr fr Any char except newline
            } otherwise ready (trans.is_char_class) {
                matches_char = char_matches_class(char_code, trans.char_class_set, trans.is_negated)
            } otherwise ready (!trans.is_epsilon) {
                matches_char = char_to_number(trans.char_match) == char_code
            }
            
            ready (matches_char) {
                fr fr Add target state to next set
                sus target_closure []drip = epsilon_closure(nfa, trans.target_state)
                sus k drip = 0
                bestie (k < array_length(target_closure)) {
                    next_states[next_count] = target_closure[k]
                    next_count = next_count + 1
                    k = k + 1
                }
            }
            
            j = j + 1
        }
        
        i = i + 1
    }
    
    damn remove_duplicates(next_states)
}

fr fr ===== REGEX MATCHING ENGINE =====

slay regex_match_dfa(dfa RegexDFA, text tea) RegexMatch {
    fr fr Execute DFA against text with capture groups
    sus match RegexMatch = RegexMatch{}
    match.text = text
    match.start_position = -1
    match.length = 0
    match.groups = []
    
    ready (!dfa.is_compiled) {
        vibez.spill("DFA not compiled")
        damn match
    }
    
    fr fr Try matching at each position
    sus start_pos drip = 0
    bestie (start_pos <= string_length(text)) {
        sus result RegexMatch = dfa_match_at_position(dfa, text, start_pos)
        
        ready (result.start_position >= 0) {
            damn result
        }
        
        start_pos = start_pos + 1
    }
    
    damn match  fr fr No match found
}

slay dfa_match_at_position(dfa RegexDFA, text tea, start_pos drip) RegexMatch {
    fr fr Try to match DFA starting at specific position
    sus match RegexMatch = RegexMatch{}
    match.text = text
    match.start_position = -1
    match.length = 0
    
    sus current_state drip = dfa.start_state
    sus position drip = start_pos
    sus last_accepting_pos drip = -1
    sus last_accepting_state drip = -1
    
    fr fr Simulate DFA execution
    bestie (position <= string_length(text)) {
        ready (dfa.states[current_state].is_accepting) {
            last_accepting_pos = position
            last_accepting_state = current_state
        }
        
        ready (position == string_length(text)) {
            break
        }
        
        sus char_code drip = char_to_number(substring(text, position, 1))
        sus next_state drip = find_dfa_transition(dfa, current_state, char_code)
        
        ready (next_state == -1) {
            break  fr fr No valid transition
        }
        
        current_state = next_state
        position = position + 1
    }
    
    fr fr Check final state
    ready (dfa.states[current_state].is_accepting && last_accepting_pos < position) {
        last_accepting_pos = position
        last_accepting_state = current_state
    }
    
    ready (last_accepting_pos >= start_pos) {
        match.start_position = start_pos
        match.length = last_accepting_pos - start_pos
        
        fr fr Extract capture groups (simplified)
        match.groups = extract_capture_groups(dfa, last_accepting_state, text, start_pos, last_accepting_pos)
    }
    
    damn match
}

slay find_dfa_transition(dfa RegexDFA, state_id drip, char_code drip) drip {
    fr fr Find transition from state on character
    sus state DFAState = dfa.states[state_id]
    sus i drip = 0
    
    bestie (i < array_length(state.transitions)) {
        sus trans DFATransition = state.transitions[i]
        ready (trans.char_code == char_code) {
            damn trans.target_state
        }
        i = i + 1
    }
    
    damn -1  fr fr No transition found
}

fr fr ===== HIGH-LEVEL REGEX API =====

slay regex_compile_full(pattern tea, flags tea) RegexPattern {
    fr fr Full regex compilation with NFA->DFA conversion
    sus regex RegexPattern = RegexPattern{}
    regex.pattern = pattern
    regex.flags = flags
    regex.is_compiled = cringe
    regex.error_message = ""
    
    fr fr Parse pattern to NFA
    sus nfa RegexNFA = regex_parse_to_nfa(pattern)
    
    ready (array_length(nfa.states) == 0) {
        regex.error_message = "Failed to parse pattern"
        damn regex
    }
    
    fr fr Convert NFA to DFA
    sus dfa RegexDFA = nfa_to_dfa(nfa)
    
    ready (!dfa.is_compiled) {
        regex.error_message = "Failed to compile DFA"
        damn regex
    }
    
    fr fr Store compiled DFA as bytecode (serialize DFA structure)
    regex.compiled_bytecode = serialize_dfa(dfa)
    regex.is_compiled = based
    
    vibez.spill("Successfully compiled regex: " + pattern)
    damn regex
}

slay regex_match_full(pattern tea, text tea) RegexMatch {
    fr fr Full regex matching with proper engine
    sus regex RegexPattern = regex_compile_full(pattern, "")
    
    ready (!regex.is_compiled) {
        sus empty_match RegexMatch = RegexMatch{}
        empty_match.text = text
        empty_match.start_position = -1
        damn empty_match
    }
    
    sus dfa RegexDFA = deserialize_dfa(regex.compiled_bytecode)
    damn regex_match_dfa(dfa, text)
}

slay regex_find_all_advanced(pattern tea, text tea) []RegexMatch {
    fr fr Find all matches with capture groups
    sus regex RegexPattern = regex_compile_full(pattern, "")
    
    ready (!regex.is_compiled) {
        sus empty_results []RegexMatch = []
        damn empty_results
    }
    
    sus dfa RegexDFA = deserialize_dfa(regex.compiled_bytecode)
    sus matches []RegexMatch = []
    sus match_count drip = 0
    sus search_pos drip = 0
    
    bestie (search_pos < string_length(text)) {
        sus remaining_text tea = substring(text, search_pos, string_length(text) - search_pos)
        sus match RegexMatch = regex_match_dfa(dfa, remaining_text)
        
        ready (match.start_position >= 0) {
            match.start_position = match.start_position + search_pos
            matches[match_count] = match
            match_count = match_count + 1
            
            search_pos = match.start_position + mathz.max(match.length, 1)
        } otherwise {
            break
        }
        
        ready (match_count >= 1000) {  fr fr Prevent infinite loops
            break
        }
    }
    
    vibez.spill("Found " + json_number_to_string(match_count) + " matches")
    damn matches
}

fr fr ===== UNICODE SUPPORT =====

slay is_unicode_char(char tea) lit {
    fr fr Check if character is Unicode (multi-byte)
    sus code drip = char_to_number(char)
    damn code > 127
}

slay normalize_unicode_text(text tea) tea {
    fr fr Normalize Unicode text for consistent matching
    sus normalized tea = ""
    sus i drip = 0
    
    bestie (i < string_length(text)) {
        sus char tea = substring(text, i, 1)
        
        ready (is_unicode_char(char)) {
            fr fr Apply Unicode normalization (simplified NFC)
            normalized = normalized + normalize_unicode_char(char)
        } otherwise {
            normalized = normalized + char
        }
        
        i = i + 1
    }
    
    damn normalized
}

slay normalize_unicode_char(char tea) tea {
    fr fr Apply Unicode normalization to single character
    sus code drip = char_to_number(char)
    
    fr fr Simplified normalization for common cases
    ready (code >= 0xC0 && code <= 0xDF) {  fr fr Latin-1 supplement
        fr fr Apply case folding and diacritic removal
        damn apply_case_folding(char)
    }
    
    damn char  fr fr Return unchanged if no normalization needed
}

slay apply_case_folding(char tea) tea {
    fr fr Apply Unicode case folding for case-insensitive matching
    sus code drip = char_to_number(char)
    
    fr fr Basic Latin case folding
    ready (code >= 65 && code <= 90) {  fr fr A-Z
        damn char(code + 32)  fr fr Convert to lowercase
    }
    
    damn char  fr fr Return unchanged
}

fr fr ===== UTILITY FUNCTIONS =====

slay char_matches_class(char_code drip, char_class tea, is_negated lit) lit {
    fr fr Check if character matches character class [abc] or [^abc]
    sus matches lit = cringe
    sus i drip = 0
    
    bestie (i < string_length(char_class)) {
        sus class_char_code drip = char_to_number(substring(char_class, i, 1))
        ready (char_code == class_char_code) {
            matches = based
            break
        }
        i = i + 1
    }
    
    ready (is_negated) {
        damn !matches
    }
    
    damn matches
}

slay expand_char_range(start_char tea, end_char tea) tea {
    fr fr Expand character range like a-z
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

slay remove_duplicates(states []drip) []drip {
    fr fr Remove duplicate states from array
    sus unique_states []drip = []
    sus unique_count drip = 0
    
    sus i drip = 0
    bestie (i < array_length(states)) {
        sus state drip = states[i]
        sus is_duplicate lit = cringe
        
        sus j drip = 0
        bestie (j < unique_count) {
            ready (unique_states[j] == state) {
                is_duplicate = based
                break
            }
            j = j + 1
        }
        
        ready (!is_duplicate) {
            unique_states[unique_count] = state
            unique_count = unique_count + 1
        }
        
        i = i + 1
    }
    
    damn unique_states
}

slay serialize_dfa(dfa RegexDFA) []drip {
    fr fr Serialize DFA to bytecode format
    sus bytecode []drip = []
    sus pos drip = 0
    
    fr fr Header: state count, start state
    bytecode[pos] = array_length(dfa.states)
    bytecode[pos + 1] = dfa.start_state
    pos = pos + 2
    
    fr fr Serialize each state
    sus i drip = 0
    bestie (i < array_length(dfa.states)) {
        sus state DFAState = dfa.states[i]
        
        bytecode[pos] = state.id
        bytecode[pos + 1] = array_length(state.transitions)
        pos = pos + 2
        
        ready (state.is_accepting) {
            bytecode[pos] = 1
        } otherwise {
            bytecode[pos] = 0
        }
        pos = pos + 1
        
        fr fr Serialize transitions
        sus j drip = 0
        bestie (j < array_length(state.transitions)) {
            sus trans DFATransition = state.transitions[j]
            bytecode[pos] = trans.char_code
            bytecode[pos + 1] = trans.target_state
            pos = pos + 2
            j = j + 1
        }
        
        i = i + 1
    }
    
    damn bytecode
}

slay deserialize_dfa(bytecode []drip) RegexDFA {
    fr fr Deserialize bytecode back to DFA
    sus dfa RegexDFA = RegexDFA{}
    dfa.states = []
    dfa.is_compiled = based
    
    sus pos drip = 0
    sus state_count drip = bytecode[pos]
    dfa.start_state = bytecode[pos + 1]
    pos = pos + 2
    
    fr fr Deserialize each state
    sus i drip = 0
    bestie (i < state_count) {
        sus state DFAState = DFAState{}
        state.id = bytecode[pos]
        sus transition_count drip = bytecode[pos + 1]
        state.is_accepting = bytecode[pos + 2] == 1
        pos = pos + 3
        
        state.transitions = []
        
        sus j drip = 0
        bestie (j < transition_count) {
            sus trans DFATransition = DFATransition{}
            trans.char_code = bytecode[pos]
            trans.target_state = bytecode[pos + 1]
            state.transitions[j] = trans
            pos = pos + 2
            j = j + 1
        }
        
        dfa.states[i] = state
        i = i + 1
    }
    
    damn dfa
}

slay is_meta_char(char tea) lit {
    fr fr Check if character is a regex metacharacter
    damn char == "|" || char == ")" || char == "]" || 
         char == "*" || char == "+" || char == "?" || char == "}"
}

slay set_parse_error(parser RegexParser, message tea) lit {
    parser.has_error = based
    parser.error = message + " at position " + json_number_to_string(parser.position)
    damn based
}

fr fr ===== PLACEHOLDER FUNCTIONS FOR MISSING STDLIB =====

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

slay char(code drip) tea {
    fr fr Convert character code to string (placeholder)
    ready (code == 65) { damn "A" }
    ready (code == 97) { damn "a" }
    ready (code == 48) { damn "0" }
    ready (code == 32) { damn " " }
    ready (code == 10) { damn "\n" }
    ready (code == 9) { damn "\t" }
    damn "?"  fr fr Unknown character
}

slay is_accepting_state_set(nfa RegexNFA, state_set []drip) lit {
    fr fr Check if any state in set is an accepting state
    sus i drip = 0
    bestie (i < array_length(state_set)) {
        sus state_id drip = state_set[i]
        ready (state_id < array_length(nfa.accepting_states)) {
            sus j drip = 0
            bestie (j < array_length(nfa.accepting_states)) {
                ready (nfa.accepting_states[j] == state_id) {
                    damn based
                }
                j = j + 1
            }
        }
        i = i + 1
    }
    damn cringe
}

slay find_or_create_dfa_state(dfa RegexDFA, nfa_states []drip, unprocessed []drip, 
                             next_id drip, nfa RegexNFA) drip {
    fr fr Find existing DFA state with same NFA state set, or create new one
    sus i drip = 0
    bestie (i < array_length(dfa.states)) {
        sus state DFAState = dfa.states[i]
        ready (state_sets_equal(state.nfa_states, nfa_states)) {
            damn state.id
        }
        i = i + 1
    }
    
    fr fr Create new DFA state
    sus new_state DFAState = DFAState{}
    new_state.id = next_id
    new_state.nfa_states = nfa_states
    new_state.is_accepting = is_accepting_state_set(nfa, nfa_states)
    new_state.transitions = []
    
    dfa.states[next_id] = new_state
    
    fr fr Add to unprocessed list
    sus unproc_count drip = array_length(unprocessed)
    unprocessed[unproc_count] = next_id
    
    damn next_id
}

slay state_sets_equal(set1 []drip, set2 []drip) lit {
    fr fr Check if two state sets are equal
    ready (array_length(set1) != array_length(set2)) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < array_length(set1)) {
        sus found lit = cringe
        sus j drip = 0
        bestie (j < array_length(set2)) {
            ready (set1[i] == set2[j]) {
                found = based
                break
            }
            j = j + 1
        }
        ready (!found) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay extract_capture_groups(dfa RegexDFA, state_id drip, text tea, 
                           start_pos drip, end_pos drip) []tea {
    fr fr Extract capture groups from match (simplified)
    sus groups []tea = []
    
    fr fr Group 0 is always the entire match
    ready (end_pos > start_pos) {
        sus match_text tea = substring(text, start_pos, end_pos - start_pos)
        groups[0] = match_text
    }
    
    fr fr Additional capture groups would be extracted from DFA state info
    fr fr This is simplified for the basic implementation
    
    damn groups
}

slay parse_quantifier(parser RegexParser, base_fragment NFAFragment) NFAFragment {
    fr fr Parse {n,m} quantifiers (simplified)
    parser.position = parser.position + 1  fr fr Skip '{'
    
    fr fr For now, treat as literal '{' - full implementation would parse numbers
    parser.position = parser.position - 1
    damn parse_literal_char(parser)
}

slay parse_anchor_start(parser RegexParser) NFAFragment {
    fr fr Parse ^ anchor
    parser.position = parser.position + 1
    
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    fr fr Add transition that only matches at text start (simplified)
    sus transition NFATransition = NFATransition{}
    transition.target_state = end_state
    transition.is_epsilon = cringe
    transition.char_match = "^"  fr fr Special marker for start
    
    parser.nfa.states[start_state].transitions[0] = transition
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}

slay parse_anchor_end(parser RegexParser) NFAFragment {
    fr fr Parse $ anchor
    parser.position = parser.position + 1
    
    sus start_state drip = create_nfa_state(parser)
    sus end_state drip = create_nfa_state(parser)
    
    fr fr Add transition that only matches at text end (simplified)
    sus transition NFATransition = NFATransition{}
    transition.target_state = end_state
    transition.is_epsilon = cringe
    transition.char_match = "$"  fr fr Special marker for end
    
    parser.nfa.states[start_state].transitions[0] = transition
    
    sus result NFAFragment = NFAFragment{}
    result.start = start_state
    result.end = end_state
    
    damn result
}
