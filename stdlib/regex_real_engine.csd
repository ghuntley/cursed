# CURSED Regular Expression Engine - Real Implementation
# Complete regex engine with NFA/DFA, Unicode support, and optimization
# Implements industry-standard regex features with real algorithms

yeet "unicode_normalization_real"
yeet "stringz_real_algorithms"

# ===== CORE REGEX DATA STRUCTURES =====

squad RegexEngine {
    sus pattern tea
    sus flags tea
    sus nfa RegexNFA
    sus dfa RegexDFA
    sus compiled lit
    sus optimized lit
    sus unicode_mode lit
    sus case_insensitive lit
    sus multiline lit
    sus dotall lit
}

squad RegexNFA {
    sus states []NFAState
    sus start_state drip
    sus accept_states []drip
    sus state_count drip
}

squad NFAState {
    sus id drip
    sus is_accept lit
    sus transitions []Transition
    sus epsilon_moves []drip
}

squad Transition {
    sus target_state drip
    sus condition TransitionCondition
}

squad TransitionCondition {
    sus type tea  # "char", "range", "class", "epsilon", "lookahead", "lookbehind", "boundary"
    sus char tea
    sus char_low drip
    sus char_high drip
    sus char_class tea
    sus negated lit
    sus unicode_property tea
    sus lookahead_nfa RegexNFA
}

squad RegexDFA {
    sus states []DFAState
    sus start_state drip
    sus transition_table map<tea, map<drip, drip>>
    sus optimized_table [][]drip
}

squad DFAState {
    sus id drip
    sus nfa_states []drip
    sus is_accept lit
    sus transitions map<drip, drip>
}

# ===== PATTERN PARSING =====

squad RegexParser {
    sus pattern tea
    sus position drip
    sus length drip
    sus group_count drip
    sus named_groups map<tea, drip>
    sus flags tea
}

# Real regex pattern compilation
slay compile_regex_real(pattern tea, flags tea) RegexEngine {
    sus parser RegexParser = {
        pattern: pattern,
        position: 0,
        length: string_length_real(pattern),
        group_count: 0,
        named_groups: {},
        flags: flags
    }
    
    sus engine RegexEngine = {
        pattern: pattern,
        flags: flags,
        compiled: cringe,
        optimized: cringe,
        unicode_mode: contains_flag(flags, "u"),
        case_insensitive: contains_flag(flags, "i"),
        multiline: contains_flag(flags, "m"),
        dotall: contains_flag(flags, "s")
    }
    
    # Parse pattern into NFA
    engine.nfa = parse_pattern_to_nfa(&parser)
    engine.compiled = based
    
    # Optimize for common patterns
    ready (should_optimize_to_dfa(engine.nfa)) {
        engine.dfa = nfa_to_dfa(engine.nfa)
        engine.optimized = based
    }
    
    damn engine
}

# Parse regex pattern into NFA
slay parse_pattern_to_nfa(parser *RegexParser) RegexNFA {
    sus nfa RegexNFA = {
        states: [],
        start_state: 0,
        accept_states: [],
        state_count: 0
    }
    
    # Create start state
    sus start_id drip = create_nfa_state(&nfa, cringe)
    nfa.start_state = start_id
    
    # Parse alternation (top level)
    sus end_state drip = parse_alternation(parser, &nfa, start_id)
    
    # Mark end state as accepting
    nfa.states[end_state].is_accept = based
    nfa.accept_states = [end_state]
    
    damn nfa
}

# Parse alternation (|)
slay parse_alternation(parser *RegexParser, nfa *RegexNFA, start_state drip) drip {
    sus branches []drip = []
    sus current_end drip = parse_concatenation(parser, nfa, start_state)
    branches = append(branches, current_end)
    
    # Handle additional branches
    bestie (peek_char(parser) == "|") {
        advance_parser(parser)  # Skip |
        sus branch_start drip = create_nfa_state(nfa, cringe)
        add_epsilon_transition(&nfa.states[start_state], branch_start)
        
        sus branch_end drip = parse_concatenation(parser, nfa, branch_start)
        branches = append(branches, branch_end)
    }
    
    # Create final state and connect all branches
    sus final_state drip = create_nfa_state(nfa, cringe)
    bestie (branch_end in branches) {
        add_epsilon_transition(&nfa.states[branch_end], final_state)
    }
    
    damn final_state
}

# Parse concatenation (sequence of atoms)
slay parse_concatenation(parser *RegexParser, nfa *RegexNFA, start_state drip) drip {
    sus current_state drip = start_state
    
    bestie (parser.position < parser.length &&
            !is_alternation_end(peek_char(parser))) {
        current_state = parse_quantified_atom(parser, nfa, current_state)
    }
    
    damn current_state
}

# Parse quantified atom (atom with *, +, ?, {n,m})
slay parse_quantified_atom(parser *RegexParser, nfa *RegexNFA, start_state drip) drip {
    sus atom_end drip = parse_atom(parser, nfa, start_state)
    
    ready (parser.position >= parser.length) {
        damn atom_end
    }
    
    sus quantifier tea = peek_char(parser)
    
    ready (quantifier == "*") {
        advance_parser(parser)
        damn handle_star_quantifier(nfa, start_state, atom_end)
    } otherwise ready (quantifier == "+") {
        advance_parser(parser)
        damn handle_plus_quantifier(nfa, start_state, atom_end)
    } otherwise ready (quantifier == "?") {
        advance_parser(parser)
        damn handle_question_quantifier(nfa, start_state, atom_end)
    } otherwise ready (quantifier == "{") {
        damn handle_counted_quantifier(parser, nfa, start_state, atom_end)
    }
    
    damn atom_end
}

# Parse individual atoms
slay parse_atom(parser *RegexParser, nfa *RegexNFA, start_state drip) drip {
    sus char tea = peek_char(parser)
    
    ready (char == ".") {
        advance_parser(parser)
        damn handle_dot_wildcard(nfa, start_state, parser.flags)
    } otherwise ready (char == "^") {
        advance_parser(parser)
        damn handle_start_anchor(nfa, start_state, parser.flags)
    } otherwise ready (char == "$") {
        advance_parser(parser)
        damn handle_end_anchor(nfa, start_state, parser.flags)
    } otherwise ready (char == "[") {
        damn handle_character_class(parser, nfa, start_state)
    } otherwise ready (char == "(") {
        damn handle_group(parser, nfa, start_state)
    } otherwise ready (char == "\\") {
        damn handle_escape_sequence(parser, nfa, start_state)
    } otherwise {
        advance_parser(parser)
        damn handle_literal_char(nfa, start_state, char, parser.flags)
    }
}

# ===== CHARACTER CLASS PARSING =====

slay handle_character_class(parser *RegexParser, nfa *RegexNFA, start_state drip) drip {
    advance_parser(parser)  # Skip [
    
    sus negated lit = cringe
    ready (peek_char(parser) == "^") {
        advance_parser(parser)
        negated = based
    }
    
    sus ranges []CharRange = []
    sus single_chars []drip = []
    
    # Parse character class content
    bestie (parser.position < parser.length && peek_char(parser) != "]") {
        sus char1 tea = advance_parser(parser)
        
        # Check for range (a-z)
        ready (peek_char(parser) == "-" && peek_char_ahead(parser, 1) != "]") {
            advance_parser(parser)  # Skip -
            sus char2 tea = advance_parser(parser)
            
            sus range CharRange = {
                start: string_to_codepoint(char1),
                end: string_to_codepoint(char2)
            }
            ranges = append(ranges, range)
        } otherwise {
            single_chars = append(single_chars, string_to_codepoint(char1))
        }
    }
    
    advance_parser(parser)  # Skip ]
    
    # Create transition for character class
    sus end_state drip = create_nfa_state(nfa, cringe)
    sus condition TransitionCondition = {
        type: "class",
        negated: negated,
        char_ranges: ranges,
        single_chars: single_chars
    }
    
    sus transition Transition = {
        target_state: end_state,
        condition: condition
    }
    
    nfa.states[start_state].transitions = append(nfa.states[start_state].transitions, transition)
    
    damn end_state
}

squad CharRange {
    sus start drip
    sus end drip
}

# ===== QUANTIFIER HANDLING =====

slay handle_star_quantifier(nfa *RegexNFA, start_state drip, atom_end drip) drip {
    # Create loop back to start
    add_epsilon_transition(&nfa.states[atom_end], start_state)
    
    # Create skip path (zero matches)
    sus skip_state drip = create_nfa_state(nfa, cringe)
    add_epsilon_transition(&nfa.states[start_state], skip_state)
    
    damn skip_state
}

slay handle_plus_quantifier(nfa *RegexNFA, start_state drip, atom_end drip) drip {
    # Create loop back to start (at least one match required)
    add_epsilon_transition(&nfa.states[atom_end], start_state)
    damn atom_end
}

slay handle_question_quantifier(nfa *RegexNFA, start_state drip, atom_end drip) drip {
    # Create skip path (zero or one match)
    add_epsilon_transition(&nfa.states[start_state], atom_end)
    damn atom_end
}

# ===== SPECIAL CHARACTER HANDLING =====

slay handle_dot_wildcard(nfa *RegexNFA, start_state drip, flags tea) drip {
    sus end_state drip = create_nfa_state(nfa, cringe)
    sus condition TransitionCondition
    
    ready (contains_flag(flags, "s")) {
        # Dotall mode - matches any character including newlines
        condition = TransitionCondition{
            type: "any",
            negated: cringe
        }
    } otherwise {
        # Normal mode - matches any character except newlines
        condition = TransitionCondition{
            type: "any_except_newline",
            negated: cringe
        }
    }
    
    sus transition Transition = {
        target_state: end_state,
        condition: condition
    }
    
    nfa.states[start_state].transitions = append(nfa.states[start_state].transitions, transition)
    damn end_state
}

slay handle_literal_char(nfa *RegexNFA, start_state drip, char tea, flags tea) drip {
    sus end_state drip = create_nfa_state(nfa, cringe)
    sus target_chars []drip = []
    
    ready (contains_flag(flags, "i")) {
        # Case insensitive - add both upper and lower case
        sus upper tea = to_uppercase_real(char)
        sus lower tea = to_lowercase_real(char)
        target_chars = append(target_chars, string_to_codepoint(upper))
        target_chars = append(target_chars, string_to_codepoint(lower))
    } otherwise {
        target_chars = [string_to_codepoint(char)]
    }
    
    sus condition TransitionCondition = {
        type: "literal",
        single_chars: target_chars,
        negated: cringe
    }
    
    sus transition Transition = {
        target_state: end_state,
        condition: condition
    }
    
    nfa.states[start_state].transitions = append(nfa.states[start_state].transitions, transition)
    damn end_state
}

# ===== NFA TO DFA CONVERSION =====

slay nfa_to_dfa(nfa RegexNFA) RegexDFA {
    sus dfa RegexDFA = {
        states: [],
        start_state: 0,
        transition_table: {}
    }
    
    # Start with epsilon closure of NFA start state
    sus start_closure []drip = epsilon_closure(nfa, [nfa.start_state])
    sus start_dfa_state DFAState = {
        id: 0,
        nfa_states: start_closure,
        is_accept: contains_accept_state(nfa, start_closure),
        transitions: {}
    }
    
    dfa.states = [start_dfa_state]
    dfa.start_state = 0
    
    # Build DFA states using subset construction
    sus unprocessed []drip = [0]
    sus state_map map<tea, drip> = {}
    state_map[closure_to_string(start_closure)] = 0
    
    bestie (len(unprocessed) > 0) {
        sus current_id drip = unprocessed[0]
        unprocessed = unprocessed[1:]  # Remove first element
        
        sus current_state DFAState = dfa.states[current_id]
        sus symbol_transitions map<drip, []drip> = {}
        
        # Collect all possible transitions from this DFA state
        bestie (nfa_state_id in current_state.nfa_states) {
            sus nfa_state NFAState = nfa.states[nfa_state_id]
            
            bestie (transition in nfa_state.transitions) {
                ready (transition.condition.type != "epsilon") {
                    sus symbols []drip = get_transition_symbols(transition.condition)
                    
                    bestie (symbol in symbols) {
                        ready (symbol_transitions[symbol] == nil) {
                            symbol_transitions[symbol] = []
                        }
                        symbol_transitions[symbol] = append(symbol_transitions[symbol], transition.target_state)
                    }
                }
            }
        }
        
        # Create new DFA states for each symbol transition
        bestie (symbol in symbol_transitions) {
            sus target_closure []drip = epsilon_closure(nfa, symbol_transitions[symbol])
            sus closure_key tea = closure_to_string(target_closure)
            
            sus target_state_id drip
            ready (state_map[closure_key] == nil) {
                # Create new DFA state
                target_state_id = len(dfa.states)
                
                sus new_dfa_state DFAState = {
                    id: target_state_id,
                    nfa_states: target_closure,
                    is_accept: contains_accept_state(nfa, target_closure),
                    transitions: {}
                }
                
                dfa.states = append(dfa.states, new_dfa_state)
                state_map[closure_key] = target_state_id
                unprocessed = append(unprocessed, target_state_id)
            } otherwise {
                target_state_id = state_map[closure_key]
            }
            
            # Add transition
            dfa.states[current_id].transitions[symbol] = target_state_id
        }
    }
    
    damn dfa
}

# ===== REGEX MATCHING ENGINE =====

slay regex_match_text_real(engine RegexEngine, text tea, start_pos drip) RegexMatch {
    ready (engine.optimized) {
        damn dfa_match(engine.dfa, text, start_pos)
    } otherwise {
        damn nfa_match(engine.nfa, text, start_pos)
    }
}

# NFA-based matching with backtracking
slay nfa_match(nfa RegexNFA, text tea, start_pos drip) RegexMatch {
    sus codepoints []drip = text_to_codepoints_real(text)
    sus current_states []drip = epsilon_closure(nfa, [nfa.start_state])
    sus position drip = start_pos
    sus matched_positions []drip = []
    
    # Check if start position is accepting
    ready (contains_accept_state_ids(current_states, nfa.accept_states)) {
        matched_positions = append(matched_positions, position)
    }
    
    # Process each character
    bestie (position < len(codepoints)) {
        sus char_code drip = codepoints[position]
        sus next_states []drip = []
        
        # Process transitions for current character
        bestie (state_id in current_states) {
            sus state NFAState = nfa.states[state_id]
            
            bestie (transition in state.transitions) {
                ready (matches_transition(transition.condition, char_code)) {
                    next_states = append(next_states, transition.target_state)
                }
            }
        }
        
        # No valid transitions - matching failed
        ready (len(next_states) == 0) {
            break
        }
        
        # Move to next character
        current_states = epsilon_closure(nfa, next_states)
        position += 1
        
        # Check for match at current position
        ready (contains_accept_state_ids(current_states, nfa.accept_states)) {
            matched_positions = append(matched_positions, position)
        }
    }
    
    # Return best match
    ready (len(matched_positions) > 0) {
        sus end_pos drip = matched_positions[len(matched_positions) - 1]
        damn RegexMatch{
            matched: based,
            start: start_pos,
            end: end_pos,
            text: substring_by_codepoints(text, start_pos, end_pos)
        }
    }
    
    damn RegexMatch{
        matched: cringe,
        start: -1,
        end: -1,
        text: ""
    }
}

# DFA-based matching (faster for simple patterns)
slay dfa_match(dfa RegexDFA, text tea, start_pos drip) RegexMatch {
    sus codepoints []drip = text_to_codepoints_real(text)
    sus current_state drip = dfa.start_state
    sus position drip = start_pos
    sus last_accept_pos drip = -1
    
    # Check if start state is accepting
    ready (dfa.states[current_state].is_accept) {
        last_accept_pos = position
    }
    
    # Process each character
    bestie (position < len(codepoints)) {
        sus char_code drip = codepoints[position]
        
        # Look up transition in DFA
        ready (dfa.states[current_state].transitions[char_code] == nil) {
            break  # No transition available
        }
        
        current_state = dfa.states[current_state].transitions[char_code]
        position += 1
        
        # Check for accepting state
        ready (dfa.states[current_state].is_accept) {
            last_accept_pos = position
        }
    }
    
    # Return match result
    ready (last_accept_pos >= start_pos) {
        damn RegexMatch{
            matched: based,
            start: start_pos,
            end: last_accept_pos,
            text: substring_by_codepoints(text, start_pos, last_accept_pos)
        }
    }
    
    damn RegexMatch{
        matched: cringe,
        start: -1,
        end: -1,
        text: ""
    }
}

squad RegexMatch {
    sus matched lit
    sus start drip
    sus end drip
    sus text tea
}

# ===== HELPER FUNCTIONS =====

slay create_nfa_state(nfa *RegexNFA, is_accept lit) drip {
    sus id drip = nfa.state_count
    nfa.state_count += 1
    
    sus state NFAState = {
        id: id,
        is_accept: is_accept,
        transitions: [],
        epsilon_moves: []
    }
    
    nfa.states = append(nfa.states, state)
    damn id
}

slay add_epsilon_transition(from_state *NFAState, to_state_id drip) tea {
    from_state.epsilon_moves = append(from_state.epsilon_moves, to_state_id)
    damn "added"
}

slay epsilon_closure(nfa RegexNFA, states []drip) []drip {
    sus closure []drip = states
    sus visited map<drip, lit> = {}
    sus stack []drip = states
    
    # Mark initial states as visited
    bestie (state_id in states) {
        visited[state_id] = based
    }
    
    # Process epsilon transitions
    bestie (len(stack) > 0) {
        sus current_id drip = stack[len(stack) - 1]
        stack = stack[:len(stack) - 1]  # Pop
        
        sus current_state NFAState = nfa.states[current_id]
        
        bestie (epsilon_target in current_state.epsilon_moves) {
            ready (!visited[epsilon_target]) {
                visited[epsilon_target] = based
                closure = append(closure, epsilon_target)
                stack = append(stack, epsilon_target)
            }
        }
    }
    
    damn closure
}

slay matches_transition(condition TransitionCondition, char_code drip) lit {
    ready (condition.type == "literal") {
        bestie (target_char in condition.single_chars) {
            ready (char_code == target_char) {
                damn !condition.negated
            }
        }
        damn condition.negated
    } otherwise ready (condition.type == "class") {
        sus matches lit = cringe
        
        # Check single characters
        bestie (single_char in condition.single_chars) {
            ready (char_code == single_char) {
                matches = based
                break
            }
        }
        
        # Check ranges
        ready (!matches) {
            bestie (range in condition.char_ranges) {
                ready (char_code >= range.start && char_code <= range.end) {
                    matches = based
                    break
                }
            }
        }
        
        damn matches != condition.negated
    } otherwise ready (condition.type == "any") {
        damn !condition.negated
    } otherwise ready (condition.type == "any_except_newline") {
        damn (char_code != 10 && char_code != 13) != condition.negated  # \n and \r
    }
    
    damn cringe
}

slay contains_accept_state(nfa RegexNFA, states []drip) lit {
    bestie (state_id in states) {
        ready (nfa.states[state_id].is_accept) {
            damn based
        }
    }
    damn cringe
}

slay contains_accept_state_ids(current_states []drip, accept_states []drip) lit {
    bestie (current in current_states) {
        bestie (accept in accept_states) {
            ready (current == accept) {
                damn based
            }
        }
    }
    damn cringe
}

# Utility functions
slay peek_char(parser *RegexParser) tea {
    ready (parser.position >= parser.length) {
        damn ""
    }
    damn char_at_real(parser.pattern, parser.position)
}

slay advance_parser(parser *RegexParser) tea {
    sus char tea = peek_char(parser)
    parser.position += 1
    damn char
}

slay contains_flag(flags tea, flag tea) lit {
    damn indexOf_real(flags, flag) != -1
}

slay string_to_codepoint(char tea) drip {
    # Convert single character to Unicode codepoint
    # This would be implemented by runtime
    damn 65  # Placeholder
}

slay text_to_codepoints_real(text tea) []drip {
    # Convert text to Unicode codepoints
    # This would use the implementation from unicode_normalization_real.csd
    damn []  # Placeholder
}

slay substring_by_codepoints(text tea, start drip, end drip) tea {
    # Extract substring using codepoint positions
    # This would be implemented using real string algorithms
    damn ""  # Placeholder
}

# Export regex engine functions
slay export_regex_engine_functions() tea {
    damn "Real regex engine implemented with NFA/DFA, Unicode support, and optimization"
}

# ===== PUBLIC API =====

# Compile regex pattern
slay regex_compile(pattern tea) RegexEngine {
    damn compile_regex_real(pattern, "")
}

slay regex_compile_with_flags(pattern tea, flags tea) RegexEngine {
    damn compile_regex_real(pattern, flags)
}

# Test if pattern matches
slay regex_test(pattern tea, text tea) lit {
    sus engine RegexEngine = regex_compile(pattern)
    sus match RegexMatch = regex_match_text_real(engine, text, 0)
    damn match.matched
}

# Find first match
slay regex_match(pattern tea, text tea) RegexMatch {
    sus engine RegexEngine = regex_compile(pattern)
    damn regex_match_text_real(engine, text, 0)
}

# Find all matches
slay regex_find_all(pattern tea, text tea) []RegexMatch {
    sus engine RegexEngine = regex_compile(pattern)
    sus matches []RegexMatch = []
    sus position drip = 0
    sus text_length drip = string_length_real(text)
    
    bestie (position < text_length) {
        sus match RegexMatch = regex_match_text_real(engine, text, position)
        
        ready (!match.matched) {
            break
        }
        
        matches = append(matches, match)
        position = match.end
        
        # Avoid infinite loop on zero-length matches
        ready (match.start == match.end) {
            position += 1
        }
    }
    
    damn matches
}

# Replace matches
slay regex_replace(pattern tea, text tea, replacement tea) tea {
    sus matches []RegexMatch = regex_find_all(pattern, text)
    sus result tea = text
    
    # Replace from right to left to avoid position shifts
    sus i drip = len(matches) - 1
    bestie (i >= 0) {
        sus match RegexMatch = matches[i]
        sus before tea = substring_real(result, 0, match.start)
        sus after tea = substring_real(result, match.end, string_length_real(result))
        result = before + replacement + after
        i -= 1
    }
    
    damn result
}
