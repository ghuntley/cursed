// Simplified test suite for grammar_drip module
yeet "testz"
yeet "grammar_drip"

// Test grammar rule validation
test_start("validate_grammar_rule - valid rule")
sus valid_rule lit = validate_grammar_rule("S -> NP VP")
assert_true(valid_rule)

test_start("validate_grammar_rule - empty rule")
sus empty_rule lit = validate_grammar_rule("")
assert_false(empty_rule)

// Test sentence validation
test_start("is_valid_sentence - proper sentence")
sus valid_sentence lit = is_valid_sentence("Hello world.")
assert_true(valid_sentence)

test_start("is_valid_sentence - empty string")
sus empty_sentence lit = is_valid_sentence("")
assert_false(empty_sentence)

// Test word counting
test_start("count_words - simple sentence")
sus word_count normie = count_words("Hello world")
assert_true(word_count > 0)

test_start("count_words - empty string")
sus empty_word_count normie = count_words("")
assert_eq_int(empty_word_count, 0)

// Test sentence counting
test_start("count_sentences - simple text")
sus sentence_count normie = count_sentences("Hello world.")
assert_true(sentence_count > 0)

test_start("count_sentences - empty string")
sus empty_sentence_count normie = count_sentences("")
assert_eq_int(empty_sentence_count, 0)

// Test balanced parentheses
test_start("has_balanced_parentheses - balanced")
sus balanced_parens lit = has_balanced_parentheses("(hello world)")
assert_true(balanced_parens)

test_start("has_balanced_parentheses - no parentheses")
sus no_parens lit = has_balanced_parentheses("hello world")
assert_true(no_parens)

// Test balanced brackets
test_start("has_balanced_brackets - balanced")
sus balanced_brackets lit = has_balanced_brackets("[hello world]")
assert_true(balanced_brackets)

test_start("has_balanced_brackets - no brackets")
sus no_brackets lit = has_balanced_brackets("hello world")
assert_true(no_brackets)

// Test balanced braces
test_start("has_balanced_braces - balanced")
sus balanced_braces lit = has_balanced_braces("{hello world}")
assert_true(balanced_braces)

test_start("has_balanced_braces - no braces")
sus no_braces lit = has_balanced_braces("hello world")
assert_true(no_braces)

// Test balanced quotes
test_start("has_balanced_quotes - balanced")
sus balanced_quotes lit = has_balanced_quotes("hello world")
assert_true(balanced_quotes)

test_start("has_balanced_quotes - no quotes")
sus no_quotes lit = has_balanced_quotes("hello world")
assert_true(no_quotes)

// Test punctuation validation
test_start("has_proper_punctuation - proper")
sus proper_punctuation lit = has_proper_punctuation("Hello world.")
assert_true(proper_punctuation)

test_start("has_proper_punctuation - empty")
sus empty_punctuation lit = has_proper_punctuation("")
assert_false(empty_punctuation)

// Test complexity scoring
test_start("calculate_complexity_score - simple")
sus complexity_score normie = calculate_complexity_score("Hello world.")
assert_true(complexity_score >= 0)

test_start("calculate_complexity_score - empty")
sus empty_complexity normie = calculate_complexity_score("")
assert_eq_int(empty_complexity, 0)

// Test pattern matching
test_start("contains_pattern - found")
sus pattern_found lit = contains_pattern("Hello world", "world")
assert_true(pattern_found)

test_start("contains_pattern - empty pattern")
sus empty_pattern lit = contains_pattern("Hello world", "")
assert_true(empty_pattern)

// Test rule structure validation
test_start("validate_rule_structure - valid")
sus valid_structure lit = validate_rule_structure("S -> NP VP")
assert_true(valid_structure)

test_start("validate_rule_structure - too short")
sus short_structure lit = validate_rule_structure("S")
assert_false(short_structure)

// Test capitalization
test_start("has_proper_capitalization - proper")
sus proper_capitalization lit = has_proper_capitalization("Hello world.")
assert_true(proper_capitalization)

test_start("has_proper_capitalization - empty")
sus empty_capitalization lit = has_proper_capitalization("")
assert_false(empty_capitalization)

// Test production rule parsing
test_start("parse_production_rule - valid")
sus parsed_rule lit = parse_production_rule("S -> NP VP")
assert_true(parsed_rule)

test_start("parse_production_rule - empty")
sus empty_parsed_rule lit = parse_production_rule("")
assert_false(empty_parsed_rule)

// Test character type counting
test_start("count_character_types - mixed text")
sus char_count normie = count_character_types("Hello123 world!")
assert_true(char_count > 0)

test_start("count_character_types - empty")
sus empty_char_count normie = count_character_types("")
assert_eq_int(empty_char_count, 0)

// Test individual character functions
test_start("char_is_uppercase - uppercase")
sus uppercase_test lit = char_is_uppercase('A')
assert_true(uppercase_test)

test_start("char_is_uppercase - lowercase")
sus lowercase_test lit = char_is_uppercase('a')
assert_false(lowercase_test)

test_start("char_is_lowercase - lowercase")
sus lowercase_test2 lit = char_is_lowercase('a')
assert_true(lowercase_test2)

test_start("char_is_lowercase - uppercase")
sus uppercase_test2 lit = char_is_lowercase('A')
assert_false(uppercase_test2)

test_start("char_is_letter - letter")
sus letter_test lit = char_is_letter('a')
assert_true(letter_test)

test_start("char_is_letter - digit")
sus digit_test lit = char_is_letter('1')
assert_false(digit_test)

test_start("char_is_digit - digit")
sus digit_test2 lit = char_is_digit('5')
assert_true(digit_test2)

test_start("char_is_digit - letter")
sus letter_test2 lit = char_is_digit('a')
assert_false(letter_test2)

test_start("char_is_alphanumeric - letter")
sus alnum_test lit = char_is_alphanumeric('a')
assert_true(alnum_test)

test_start("char_is_alphanumeric - digit")
sus alnum_test2 lit = char_is_alphanumeric('5')
assert_true(alnum_test2)

test_start("char_is_alphanumeric - punctuation")
sus punct_test lit = char_is_alphanumeric('.')
assert_false(punct_test)

test_start("char_is_whitespace - space")
sus space_test lit = char_is_whitespace(' ')
assert_true(space_test)

test_start("char_is_whitespace - tab")
sus tab_test lit = char_is_whitespace('\t')
assert_true(tab_test)

test_start("char_is_whitespace - letter")
sus letter_test3 lit = char_is_whitespace('a')
assert_false(letter_test3)

test_start("char_is_punctuation - period")
sus period_test lit = char_is_punctuation('.')
assert_true(period_test)

test_start("char_is_punctuation - comma")
sus comma_test lit = char_is_punctuation(',')
assert_true(comma_test)

test_start("char_is_punctuation - letter")
sus letter_test4 lit = char_is_punctuation('a')
assert_false(letter_test4)

print_test_summary()
