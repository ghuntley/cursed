fr fr CURSED Regex Module Tests
fr fr Comprehensive test suite for regex functionality

yeet "testz"
yeet "regex"

slay test_all_regex() {
    test_start("Regex Module Comprehensive Tests") fr fr Basic functionality tests
    test_basic_pattern_matching()
    test_wildcard_matching() 
    test_find_matches()
    test_replace_patterns()
    test_split_by_pattern()
    test_character_classes()
    test_validation_functions()
    test_match_positions()
    test_match_counting()
    test_complex_patterns() fr fr Advanced PCRE tests
    test_pcre_compilation()
    test_unicode_support()
    test_named_capture_groups()
    test_backreferences()
    test_lookahead_assertions()
    test_lookbehind_assertions()
    test_quantifiers_advanced()
    test_character_classes_unicode()
    test_anchors_and_boundaries()
    test_alternation_advanced() fr fr Optimization and performance tests
    test_regex_optimization()
    test_performance_benchmarks()
    test_complexity_analysis() fr fr Validation and security tests
    test_input_validation()
    test_catastrophic_backtracking_detection()
    test_unicode_escape_validation() fr fr Debugging and explanation tests
    test_regex_explanation()
    test_pattern_analysis()
    test_helper_functions()
    
    print_test_summary()
}

fr fr Basic functionality tests
slay test_basic_pattern_matching() {
    test_start("Basic Pattern Matching") fr fr Exact matches
    assert_true(regex.match_pattern("hello", "hello"))
    assert_false(regex.match_pattern("hello", "world"))
    assert_false(regex.match_pattern("", "hello"))
    assert_true(regex.match_pattern("", "")) fr fr Case sensitivity
    assert_false(regex.match_pattern("Hello", "hello"))
    assert_true(regex.match_pattern("Test", "Test")) fr fr Partial matches (should fail for exact match)
    assert_false(regex.match_pattern("hello world", "hello"))
    assert_false(regex.match_pattern("hello", "hello world"))
}

slay test_wildcard_matching() {
    test_start("Wildcard Pattern Matching") fr fr Asterisk wildcard
    assert_true(regex.match_wildcard("hello", "h*"))
    assert_true(regex.match_wildcard("hello", "*lo"))
    assert_true(regex.match_wildcard("hello", "h*o"))
    assert_true(regex.match_wildcard("hello", "*")) fr fr Question mark wildcard
    assert_true(regex.match_wildcard("hello", "h?llo"))
    assert_true(regex.match_wildcard("hello", "????o"))
    assert_false(regex.match_wildcard("hello", "h?lo")) fr fr Combined wildcards
    assert_true(regex.match_wildcard("hello", "h*l?o"))
    assert_true(regex.match_wildcard("test123", "test*"))
    assert_true(regex.match_wildcard("abc", "?*"))
}

slay test_find_matches() {
    test_start("Find Matches") fr fr Single matches
    sus matches [tea] = regex.find_matches("hello world", "hello")
    assert_eq_int(len(matches), 1)
    assert_eq_string(matches[0], "hello") fr fr Multiple matches
    sus multi [tea] = regex.find_matches("test test test", "test")
    assert_eq_int(len(multi), 3) fr fr No matches
    sus none [tea] = regex.find_matches("hello", "world")
    assert_eq_int(len(none), 0) fr fr Empty pattern
    sus empty [tea] = regex.find_matches("hello", "")
    assert_eq_int(len(empty), 0)
}

slay test_replace_patterns() {
    test_start("Replace Patterns") fr fr Single replacement
    sus result1 tea = regex.replace_pattern("hello world", "hello", "hi")
    assert_eq_string(result1, "hi world") fr fr No replacement needed
    sus result2 tea = regex.replace_pattern("test", "xyz", "abc")
    assert_eq_string(result2, "test")
}

slay test_split_by_pattern() {
    test_start("Split By Pattern") fr fr Basic split
    sus parts [tea] = regex.split_by_pattern("a,b,c", ",")
    assert_eq_int(len(parts), 3)
    assert_eq_string(parts[0], "a")
    assert_eq_string(parts[1], "b")
    assert_eq_string(parts[2], "c") fr fr No split
    sus no_split [tea] = regex.split_by_pattern("hello", ",")
    assert_eq_int(len(no_split), 1)
    assert_eq_string(no_split[0], "hello")
}

slay test_character_classes() {
    test_start("Character Classes") fr fr Test Unicode character classes
    assert_true(regex.match_unicode_class("a", "\\p{L}")) fr fr Letter
    assert_true(regex.match_unicode_class("5", "\\p{N}")) fr fr Number
    assert_true(regex.match_unicode_class(".", "\\p{P}")) fr fr Punctuation
    assert_true(regex.match_unicode_class(" ", "\\p{Z}")) fr fr Separator
}

slay test_validation_functions() {
    test_start("Validation Functions") fr fr Pattern validation
    assert_true(regex.regex_validate_input("hello", 100))
    assert_false(regex.regex_validate_input("hello", 3)) fr fr Bracket balance
    assert_true(regex.is_bracket_balanced("[]"))
    assert_true(regex.is_bracket_balanced("()"))
    assert_false(regex.is_bracket_balanced("["))
}

slay test_match_positions() {
    test_start("Match Positions") fr fr Test basic position concept
    assert_true(regex.match_pattern("hello", "hello"))
}

slay test_match_counting() {
    test_start("Match Counting") fr fr Count occurrences
    assert_eq_int(regex.count_occurrences("hello", "l"), 2)
    assert_eq_int(regex.count_occurrences("test*test+test?", "*"), 1)
    assert_eq_int(regex.count_occurrences("a|b|c", "|"), 2)
}

slay test_complex_patterns() {
    test_start("Complex Patterns") fr fr Test complex pattern handling
    assert_true(regex.has_complex_features("(?=hello)world"))
    assert_false(regex.has_complex_features("hello"))
}

fr fr Advanced PCRE tests
slay test_pcre_compilation() {
    test_start("PCRE Compilation") fr fr Test basic PCRE compilation
    sus engine regex.RegexEngine = regex.regex_compile_pcre("hello", 0)
    assert_eq_string(engine.pattern, "hello")
    assert_eq_int(engine.flags, 0)
    assert_false(engine.unicode_enabled)
    assert_eq_int(engine.optimization_level, 2) fr fr Test with flags
    sus unicode_engine regex.RegexEngine = regex.regex_compile_pcre("test", regex.PCRE_UNICODE)
    assert_true(unicode_engine.unicode_enabled) fr fr Test case insensitive flag
    sus case_engine regex.RegexEngine = regex.regex_compile_pcre("Test", regex.PCRE_IGNORECASE)
    assert_eq_int(case_engine.flags, regex.PCRE_IGNORECASE)
}

slay test_unicode_support() {
    test_start("Unicode Support") fr fr Test Unicode compilation
    sus engine regex.RegexEngine = regex.regex_compile_pcre("héllo", regex.PCRE_UNICODE)
    assert_true(engine.unicode_enabled) fr fr Test Unicode character classes
    assert_true(regex.match_unicode_class("a", "\\p{L}")) fr fr Letter
    assert_true(regex.match_unicode_class("5", "\\p{N}")) fr fr Number
    assert_true(regex.match_unicode_class(".", "\\p{P}")) fr fr Punctuation
    assert_true(regex.match_unicode_class(" ", "\\p{Z}")) fr fr Separator fr fr Test Unicode codepoint functions
    assert_true(regex.is_unicode_letter(65)) fr fr 'A'
    assert_true(regex.is_unicode_number(48)) fr fr '0'
    assert_true(regex.is_unicode_separator(32)) fr fr Space
}

slay test_named_capture_groups() {
    test_start("Named Capture Groups") fr fr Test named group pattern
    sus engine regex.RegexEngine = regex.regex_compile_pcre("(?<word>\\w+)", regex.PCRE_UNICODE)
    assert_eq_string(engine.pattern, "(?<word>\\w+)") fr fr Test named group extraction
    sus groups [regex.NamedGroup] = regex.regex_extract_named_groups(engine, "hello")
    assert_eq_int(len(groups), 1)
    assert_eq_string(groups[0].name, "word")
    assert_eq_string(groups[0].text, "hello")
}

slay test_backreferences() {
    test_start("Backreferences") fr fr Test numbered backreferences
    sus match regex.AdvancedMatchResult = regex.AdvancedMatchResult{
        text: "hello",
        start: 0,
        end: 5,
        length: 5,
        capture_groups: [regex.CaptureGroup{
            group_number: 1,
            text: "hello",
            start: 0,
            end: 5
        }],
        named_groups: [],
        backreferences: []
    }
    
    sus expanded tea = regex.regex_expand_backreferences("Matched: \\1", match)
    assert_eq_string(expanded, "Matched: hello") fr fr Test named backreferences
    sus named_match regex.AdvancedMatchResult = regex.AdvancedMatchResult{
        text: "world",
        start: 0,
        end: 5,
        length: 5,
        capture_groups: [],
        named_groups: [regex.NamedGroup{
            name: "word",
            text: "world",
            start: 0,
            end: 5
        }],
        backreferences: []
    }
    
    sus named_expanded tea = regex.regex_expand_backreferences("Found: \\k<word>", named_match)
    assert_eq_string(named_expanded, "Found: world")
}

slay test_lookahead_assertions() {
    test_start("Lookahead Assertions") fr fr Test positive lookahead pattern
    sus engine regex.RegexEngine = regex.regex_compile_pcre("test(?=ing)", 0)
    assert_eq_string(engine.pattern, "test(?=ing)") fr fr Test negative lookahead pattern
    sus neg_engine regex.RegexEngine = regex.regex_compile_pcre("test(?!ing)", 0)
    assert_eq_string(neg_engine.pattern, "test(?!ing)") fr fr Test lookahead matching
    assert_true(regex.regex_match_with_assertions(engine, "testing", 0))
}

slay test_lookbehind_assertions() {
    test_start("Lookbehind Assertions") fr fr Test positive lookbehind pattern
    sus engine regex.RegexEngine = regex.regex_compile_pcre("(?<=pre)test", 0)
    assert_eq_string(engine.pattern, "(?<=pre)test") fr fr Test negative lookbehind pattern
    sus neg_engine regex.RegexEngine = regex.regex_compile_pcre("(?<!pre)test", 0)
    assert_eq_string(neg_engine.pattern, "(?<!pre)test") fr fr Test lookbehind matching
    assert_true(regex.regex_match_with_assertions(engine, "pretest", 3))
}

slay test_quantifiers_advanced() {
    test_start("Advanced Quantifiers") fr fr Test lazy quantifiers
    sus lazy_engine regex.RegexEngine = regex.regex_compile_pcre("a*?b", 0)
    assert_eq_string(lazy_engine.pattern, "a*?b") fr fr Test possessive quantifiers
    sus poss_engine regex.RegexEngine = regex.regex_compile_pcre("a*+b", 0)
    assert_eq_string(poss_engine.pattern, "a*+b") fr fr Test exact quantifiers
    sus exact_engine regex.RegexEngine = regex.regex_compile_pcre("a{3,5}", 0)
    assert_eq_string(exact_engine.pattern, "a{3,5}")
}

slay test_character_classes_unicode() {
    test_start("Unicode Character Classes") fr fr Test Unicode letter categories
    assert_true(regex.is_unicode_letter(0x41)) fr fr 'A'
    assert_true(regex.is_unicode_letter(0x61)) fr fr 'a'
    assert_true(regex.is_unicode_letter(0xC0)) fr fr 'À'
    assert_false(regex.is_unicode_letter(0x30)) fr fr '0' fr fr Test Unicode number categories
    assert_true(regex.is_unicode_number(0x30)) fr fr '0'
    assert_true(regex.is_unicode_number(0x39)) fr fr '9'
    assert_true(regex.is_unicode_number(0x660)) fr fr Arabic-Indic digit
    assert_false(regex.is_unicode_number(0x41)) fr fr 'A' fr fr Test Unicode punctuation
    assert_true(regex.is_unicode_punctuation(0x21)) fr fr '!'
    assert_true(regex.is_unicode_punctuation(0x2E)) fr fr '.'
    assert_false(regex.is_unicode_punctuation(0x41)) fr fr 'A' fr fr Test Unicode symbols
    assert_true(regex.is_unicode_symbol(0x24)) fr fr '$'
    assert_true(regex.is_unicode_symbol(0x2B)) fr fr '+'
    assert_false(regex.is_unicode_symbol(0x41)) fr fr 'A' fr fr Test Unicode separators
    assert_true(regex.is_unicode_separator(0x20)) fr fr Space
    assert_true(regex.is_unicode_separator(0x09)) fr fr Tab
    assert_false(regex.is_unicode_separator(0x41)) fr fr 'A' fr fr Test Unicode marks
    assert_true(regex.is_unicode_mark(0x300)) fr fr Combining mark
    assert_false(regex.is_unicode_mark(0x41)) fr fr 'A' fr fr Test Unicode other/control
    assert_true(regex.is_unicode_other(0x01)) fr fr Control character
    assert_true(regex.is_unicode_other(0xE000)) fr fr Private use
    assert_false(regex.is_unicode_other(0x41)) fr fr 'A'
}

slay test_anchors_and_boundaries() {
    test_start("Anchors and Boundaries") fr fr Test start anchor
    sus start_engine regex.RegexEngine = regex.regex_compile_pcre("^hello", 0)
    assert_eq_string(start_engine.pattern, "^hello") fr fr Test end anchor
    sus end_engine regex.RegexEngine = regex.regex_compile_pcre("world$", 0)
    assert_eq_string(end_engine.pattern, "world$") fr fr Test word boundaries
    sus word_engine regex.RegexEngine = regex.regex_compile_pcre("\\bword\\b", 0)
    assert_eq_string(word_engine.pattern, "\\bword\\b")
}

slay test_alternation_advanced() {
    test_start("Advanced Alternation") fr fr Test basic alternation
    sus alt_engine regex.RegexEngine = regex.regex_compile_pcre("cat|dog", 0)
    assert_eq_string(alt_engine.pattern, "cat|dog") fr fr Test grouped alternation
    sus group_alt_engine regex.RegexEngine = regex.regex_compile_pcre("(cat|dog)s", 0)
    assert_eq_string(group_alt_engine.pattern, "(cat|dog)s")
}

fr fr Optimization and performance tests
slay test_regex_optimization() {
    test_start("Regex Optimization") fr fr Test pattern optimization
    sus optimized tea = regex.optimize_regex_pattern("a{1}b*b*")
    assert_eq_string(optimized, "ab*") fr fr Should remove {1} and merge b*b* fr fr Test redundant quantifier removal
    sus no_redundant tea = regex.remove_redundant_quantifiers("test{1}")
    assert_eq_string(no_redundant, "test")
}

slay test_performance_benchmarks() {
    test_start("Performance Benchmarks") fr fr Test benchmark function
    sus report tea = regex.regex_benchmark("hello", "hello world", 100)
    assert_true(regex.string_contains(report, "Benchmark Results"))
    assert_true(regex.string_contains(report, "Pattern: hello"))
    assert_true(regex.string_contains(report, "Iterations: 100")) fr fr Test DFA vs NFA performance
    sus simple_pattern tea = "hello"
    sus complex_pattern tea = "(?=hello)world" fr fr Simple patterns should prefer DFA
    sus simple_engine regex.RegexEngine = regex.regex_compile_pcre(simple_pattern, 0)
    assert_true(regex.should_use_dfa(simple_engine.compiled_nfa)) fr fr Complex patterns should use NFA
    sus complex_engine regex.RegexEngine = regex.regex_compile_pcre(complex_pattern, 0)
    assert_true(regex.has_complex_features(complex_pattern))
}

slay test_complexity_analysis() {
    test_start("Pattern Complexity Analysis") fr fr Test simple pattern
    sus simple_analysis tea = regex.analyze_pattern_complexity("hello")
    assert_true(regex.string_contains(simple_analysis, "Complexity level: LOW")) fr fr Test medium complexity pattern
    sus medium_analysis tea = regex.analyze_pattern_complexity("hello.*world")
    assert_true(regex.string_contains(medium_analysis, "Pattern Complexity Analysis")) fr fr Test high complexity pattern
    sus complex_analysis tea = regex.analyze_pattern_complexity("(?=hello)(?!world)(a+)+")
    assert_true(regex.string_contains(complex_analysis, "Complexity level: HIGH")) fr fr Test specific complexity components
    sus quantifier_analysis tea = regex.analyze_pattern_complexity("a*b+c?")
    assert_true(regex.string_contains(quantifier_analysis, "Quantifiers: 3"))
    
    sus group_analysis tea = regex.analyze_pattern_complexity("(a)(b)(c)")
    assert_true(regex.string_contains(group_analysis, "Groups: 3"))
    
    sus alt_analysis tea = regex.analyze_pattern_complexity("a|b|c")
    assert_true(regex.string_contains(alt_analysis, "Alternations: 2"))
}

fr fr Validation and security tests
slay test_input_validation() {
    test_start("Input Validation") fr fr Test pattern length validation
    assert_true(regex.regex_validate_input("hello", 100))
    assert_false(regex.regex_validate_input("hello", 3)) fr fr Test catastrophic backtracking detection
    assert_false(regex.regex_validate_input("(.*)*", 100))
    assert_false(regex.regex_validate_input("(a+)+", 100))
    assert_true(regex.regex_validate_input("hello", 100)) fr fr Test bracket balance
    assert_true(regex.regex_validate_input("[a-z]", 100))
    assert_true(regex.regex_validate_input("(test)", 100))
    assert_false(regex.regex_validate_input("[a-z", 100))
    assert_false(regex.regex_validate_input("(test", 100))
}

slay test_catastrophic_backtracking_detection() {
    test_start("Catastrophic Backtracking Detection") fr fr Test dangerous patterns
    assert_true(regex.has_catastrophic_backtracking("(.*)*"))
    assert_true(regex.has_catastrophic_backtracking("(a+)+"))
    assert_true(regex.has_catastrophic_backtracking("(a*)+"))
    assert_true(regex.has_catastrophic_backtracking("(.*)+"))) fr fr Test safe patterns
    assert_false(regex.has_catastrophic_backtracking("hello"))
    assert_false(regex.has_catastrophic_backtracking("a*"))
    assert_false(regex.has_catastrophic_backtracking("(abc)+"))
}

slay test_unicode_escape_validation() {
    test_start("Unicode Escape Validation") fr fr Test valid Unicode escapes
    assert_true(regex.validate_unicode_escapes("\\u0041"))
    assert_true(regex.validate_unicode_escapes("\\U00000041"))
    assert_true(regex.validate_unicode_escapes("\\x41"))
    assert_true(regex.validate_unicode_escapes("normal")) fr fr Test hex escape validation
    assert_true(regex.is_valid_hex_escape("\\u0041", 0))
    assert_true(regex.is_valid_hex_escape("\\x41", 0)) fr fr Test bracket balance
    assert_true(regex.is_bracket_balanced("[]"))
    assert_true(regex.is_bracket_balanced("()"))
    assert_true(regex.is_bracket_balanced("[a-z](test)"))
    assert_false(regex.is_bracket_balanced("["))
    assert_false(regex.is_bracket_balanced("("))
}

fr fr Debugging and explanation tests
slay test_regex_explanation() {
    test_start("Regex Explanation") fr fr Test basic explanation
    sus explanation tea = regex.regex_explain("hello")
    assert_true(regex.string_contains(explanation, "Regular Expression Explanation"))
    assert_true(regex.string_contains(explanation, "Pattern: hello")) fr fr Test quantifier explanation
    sus quant_explanation tea = regex.regex_explain("a*b+c?")
    assert_true(regex.string_contains(quant_explanation, "Quantifiers"))
    assert_true(regex.string_contains(quant_explanation, "* : Zero or more"))
    assert_true(regex.string_contains(quant_explanation, "+ : One or more"))
    assert_true(regex.string_contains(quant_explanation, "? : Zero or one")) fr fr Test character class explanation
    sus class_explanation tea = regex.regex_explain("\\d\\w\\s.")
    assert_true(regex.string_contains(class_explanation, "Character Classes"))
    assert_true(regex.string_contains(class_explanation, "\\d : Any digit"))
    assert_true(regex.string_contains(class_explanation, "\\w : Any word character"))
    assert_true(regex.string_contains(class_explanation, "\\s : Any whitespace"))
    assert_true(regex.string_contains(class_explanation, ". : Any character")) fr fr Test anchor explanation
    sus anchor_explanation tea = regex.regex_explain("^hello$")
    assert_true(regex.string_contains(anchor_explanation, "Anchors"))
    assert_true(regex.string_contains(anchor_explanation, "^ : Start of string"))
    assert_true(regex.string_contains(anchor_explanation, "$ : End of string")) fr fr Test group explanation
    sus group_explanation tea = regex.regex_explain("(test)(?:group)(?<name>named)")
    assert_true(regex.string_contains(group_explanation, "Groups"))
    assert_true(regex.string_contains(group_explanation, "(...) : Capturing group"))
    assert_true(regex.string_contains(group_explanation, "(?:...) : Non-capturing"))
    assert_true(regex.string_contains(group_explanation, "(?<name>...) : Named")) fr fr Test assertion explanation
    sus assertion_explanation tea = regex.regex_explain("(?=test)(?!bad)(?<=pre)(?<!not)")
    assert_true(regex.string_contains(assertion_explanation, "Assertions"))
    assert_true(regex.string_contains(assertion_explanation, "(?=...) : Positive lookahead"))
    assert_true(regex.string_contains(assertion_explanation, "(?!...) : Negative lookahead"))
    assert_true(regex.string_contains(assertion_explanation, "(?<=...) : Positive lookbehind"))
    assert_true(regex.string_contains(assertion_explanation, "(?<!...) : Negative lookbehind"))
}

slay test_pattern_analysis() {
    test_start("Pattern Analysis") fr fr Test structure analysis
    sus analysis tea = regex.analyze_pattern_structure("hello|world^test$\\b")
    assert_true(regex.string_contains(analysis, "Structure Analysis"))
    assert_true(regex.string_contains(analysis, "Contains alternation"))
    assert_true(regex.string_contains(analysis, "Anchored at start"))
    assert_true(regex.string_contains(analysis, "Anchored at end"))
    assert_true(regex.string_contains(analysis, "Contains word boundaries")) fr fr Test quantifier analysis
    sus quant_analysis tea = regex.explain_quantifiers("a*b+c?d{2,5}")
    assert_true(regex.string_contains(quant_analysis, "Quantifiers"))
    assert_true(regex.string_contains(quant_analysis, "* : Zero or more"))
    assert_true(regex.string_contains(quant_analysis, "+ : One or more"))
    assert_true(regex.string_contains(quant_analysis, "? : Zero or one"))
    assert_true(regex.string_contains(quant_analysis, "{n,m} : Between n and m"))
}

slay test_helper_functions() {
    test_start("Helper Functions") fr fr Test Unicode codepoint function
    assert_eq_int(regex.get_unicode_codepoint("A"), 65)
    assert_eq_int(regex.get_unicode_codepoint("0"), 48)
    assert_eq_int(regex.get_unicode_codepoint(" "), 32) fr fr Test wildcard to regex conversion
    sus wildcard_regex tea = regex.wildcard_to_regex("test*")
    assert_eq_string(wildcard_regex, "^test.*$")
    
    sus question_regex tea = regex.wildcard_to_regex("t?st")
    assert_eq_string(question_regex, "^t.st$") fr fr Test string helper functions
    assert_eq_string(regex.int_to_string(42), "42")
    assert_eq_string(regex.float_to_string(3.14), "3.14") fr fr Test string length
    assert_eq_int(regex.string_length(""), 0)
    assert_eq_int(regex.string_length("hello"), 5)
}

fr fr Run all tests
test_all_regex()
