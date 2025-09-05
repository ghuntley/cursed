# Grammar Drip Module

A comprehensive grammar parsing and validation library for the CURSED programming language. This module provides tools for analyzing text structure, validating grammar rules, and processing natural language patterns.

## Features

### Grammar Rule Validation
- **validate_grammar_rule(rule)** - Validates basic grammar rule structure
- **validate_rule_structure(rule)** - Validates production rule format
- **parse_production_rule(rule)** - Parses BNF/EBNF production rules

### Text Analysis
- **count_words(text)** - Counts words in text
- **count_sentences(text)** - Counts sentences in text
- **count_character_types(text)** - Counts different character types
- **extract_words(text)** - Extracts word array from text

### Sentence Validation
- **is_valid_sentence(text)** - Validates basic sentence structure
- **has_proper_punctuation(text)** - Checks punctuation rules
- **has_proper_capitalization(text)** - Validates capitalization rules

### Balance Checking
- **has_balanced_parentheses(text)** - Checks parentheses balance
- **has_balanced_brackets(text)** - Checks bracket balance
- **has_balanced_braces(text)** - Checks brace balance
- **has_balanced_quotes(text)** - Checks quote balance

### Text Processing
- **contains_pattern(text, pattern)** - Pattern matching
- **calculate_complexity_score(text)** - Text complexity analysis

### Character Utilities
- **char_is_uppercase(ch)** - Uppercase character check
- **char_is_lowercase(ch)** - Lowercase character check
- **char_is_letter(ch)** - Letter character check
- **char_is_digit(ch)** - Digit character check
- **char_is_alphanumeric(ch)** - Alphanumeric character check
- **char_is_whitespace(ch)** - Whitespace character check
- **char_is_punctuation(ch)** - Punctuation character check

## Usage Examples

### Basic Text Analysis
```cursed
yeet "grammar_drip"

sus text tea = "Hello world! How are you?"
sus word_count normie = count_words(text)
sus sentence_count normie = count_sentences(text)
sus is_valid lit = is_valid_sentence(text)

vibez.spill("Words: " + word_count)
vibez.spill("Sentences: " + sentence_count)
vibez.spill("Valid: " + is_valid)
```

### Grammar Rule Validation
```cursed
yeet "grammar_drip"

sus rule tea = "S -> NP VP"
sus is_valid_rule lit = validate_rule_structure(rule)
sus parsed lit = parse_production_rule(rule)

vibez.spill("Rule is valid: " + is_valid_rule)
vibez.spill("Successfully parsed: " + parsed)
```

### Balance Checking
```cursed
yeet "grammar_drip"

sus code tea = "function(param) { return [1, 2, 3]; }"
sus balanced_parens lit = has_balanced_parentheses(code)
sus balanced_brackets lit = has_balanced_brackets(code)
sus balanced_braces lit = has_balanced_braces(code)

vibez.spill("Parentheses balanced: " + balanced_parens)
vibez.spill("Brackets balanced: " + balanced_brackets)
vibez.spill("Braces balanced: " + balanced_braces)
```

### Text Complexity Analysis
```cursed
yeet "grammar_drip"

sus simple_text tea = "Hello world."
sus complex_text tea = "The quick brown fox jumps over the lazy dog."

sus simple_score normie = calculate_complexity_score(simple_text)
sus complex_score normie = calculate_complexity_score(complex_text)

vibez.spill("Simple complexity: " + simple_score)
vibez.spill("Complex complexity: " + complex_score)
```

### Character Analysis
```cursed
yeet "grammar_drip"

sus ch sip = 'A'
sus is_upper lit = char_is_uppercase(ch)
sus is_letter lit = char_is_letter(ch)
sus is_alnum lit = char_is_alphanumeric(ch)

vibez.spill("Character: " + ch)
vibez.spill("Is uppercase: " + is_upper)
vibez.spill("Is letter: " + is_letter)
vibez.spill("Is alphanumeric: " + is_alnum)
```

### Pattern Matching
```cursed
yeet "grammar_drip"

sus text tea = "The quick brown fox"
sus pattern tea = "quick"
sus contains lit = contains_pattern(text, pattern)

vibez.spill("Text contains '" + pattern + "': " + contains)
```

## Grammar Rule Formats

### BNF (Backus-Naur Form)
```
<expr> -> <term> '+' <expr> | <term>
<term> -> <factor> '*' <term> | <factor>
<factor> -> '(' <expr> ')' | <number>
```

### EBNF (Extended Backus-Naur Form)
```
expr -> term ('+' term)*
term -> factor ('*' factor)*
factor -> '(' expr ')' | number
```

### Context-Free Grammar Rules
```
S -> NP VP
NP -> Det Noun | Pronoun
VP -> Verb NP | Verb
Det -> 'the' | 'a'
Noun -> 'cat' | 'dog'
Verb -> 'runs' | 'jumps'
```

## Text Validation Rules

### Sentence Structure
1. Must start with a capital letter
2. Must end with punctuation (. ! ?)
3. Must contain at least one character
4. Proper capitalization after sentence endings

### Balance Requirements
1. Every opening parenthesis must have a closing parenthesis
2. Every opening bracket must have a closing bracket
3. Every opening brace must have a closing brace
4. Quotes must appear in pairs

### Complexity Scoring
- Based on average words per sentence
- Higher scores indicate more complex text
- Useful for readability analysis

## Implementation Details

### Pure CURSED Implementation
- No external dependencies or FFI bridges
- Uses only CURSED language features
- Efficient string processing algorithms
- Comprehensive error handling

### Performance Characteristics
- Linear time complexity for most operations
- Optimized for typical text processing workloads
- Memory efficient with minimal allocations
- Suitable for real-time text analysis

### Testing Coverage
- 70+ comprehensive test functions
- Tests for all major functionality
- Edge case handling verification
- Both positive and negative test cases

## Use Cases

### Natural Language Processing
- Text preprocessing and validation
- Grammar checking applications
- Language learning tools
- Content analysis systems

### Code Analysis
- Programming language parsing
- Syntax validation
- Code formatting tools
- IDE development support

### Document Processing
- Text quality assessment
- Readability analysis
- Content validation
- Automated editing tools

### Educational Applications
- Grammar teaching tools
- Language learning aids
- Writing assistance software
- Text complexity measurement

## Error Handling

The module provides robust error handling for:
- Invalid input text
- Malformed grammar rules
- Unbalanced delimiters
- Empty or null strings
- Out-of-bounds access

## Future Enhancements

Potential future additions:
- Advanced regex pattern matching
- Context-sensitive grammar support
- Machine learning integration
- Multi-language support
- Performance optimizations

## Testing

Run the comprehensive test suite:
```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/grammar_drip/test_grammar_drip.💀

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/grammar_drip/test_grammar_drip.💀
./test_grammar_drip
```

The module includes 70+ test functions covering all major functionality, edge cases, and error conditions.
