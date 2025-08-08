// regexz module - Alias for regex_vibez for standardized naming
// Import comprehensive regex functionality from regex_vibez

yeet "regex_vibez"

// Re-export all functions from regex_vibez for standardized naming
// This maintains backward compatibility while providing expected "regexz" module name

// Pattern compilation functions
slay compile(pattern tea) CompiledPattern {
    damn regex_vibez.compile_pattern(pattern)
}

slay compile_with_flags(pattern tea, flags tea) CompiledPattern {
    damn regex_vibez.compile_pattern_with_flags(pattern, flags)
}

// Basic matching functions
slay match(pattern tea, text tea) lit {
    damn regex_vibez.match_pattern(pattern, text)
}

slay match_start(pattern tea, text tea) lit {
    damn regex_vibez.match_start(pattern, text)
}

slay match_full(pattern tea, text tea) lit {
    damn regex_vibez.match_full(pattern, text)
}

// Find operations
slay find(pattern tea, text tea) []Match {
    damn regex_vibez.find_matches(pattern, text)
}

slay find_all(pattern tea, text tea) []Match {
    damn regex_vibez.find_matches(pattern, text)
}

slay find_first(pattern tea, text tea) Match {
    sus matches []Match = regex_vibez.find_matches(pattern, text)
    ready (len(matches) > 0) {
        damn matches[0]
    }
    damn Match{start: -1, end: -1, text: ""}
}

// Replace operations
slay replace(pattern tea, text tea, replacement tea) tea {
    damn regex_vibez.replace_pattern(pattern, text, replacement)
}

slay replace_all(pattern tea, text tea, replacement tea) tea {
    damn regex_vibez.replace_pattern(pattern, text, replacement)
}

// Advanced pattern matching
slay match_groups(pattern tea, text tea) []tea {
    damn regex_vibez.extract_groups(pattern, text)
}

slay named_groups(pattern tea, text tea) NamedGroups {
    damn regex_vibez.extract_named_groups(pattern, text)
}

// Character class shortcuts
slay is_digit(char tea) lit {
    damn regex_vibez.match_character_class(char, "\\d")
}

slay is_word_char(char tea) lit {
    damn regex_vibez.match_character_class(char, "\\w")
}

slay is_whitespace(char tea) lit {
    damn regex_vibez.match_character_class(char, "\\s")
}

// Validation functions
slay is_valid_pattern(pattern tea) lit {
    damn regex_vibez.validate_pattern(pattern)
}

slay validate_regex(pattern tea) ValidationResult {
    damn regex_vibez.validate_regex_syntax(pattern)
}

// Case-insensitive operations
slay match_ignore_case(pattern tea, text tea) lit {
    sus flags tea = "i"
    sus compiled CompiledPattern = regex_vibez.compile_pattern_with_flags(pattern, flags)
    damn regex_vibez.test_compiled_pattern(compiled, text)
}

slay replace_ignore_case(pattern tea, text tea, replacement tea) tea {
    sus flags tea = "i"
    sus compiled CompiledPattern = regex_vibez.compile_pattern_with_flags(pattern, flags)
    damn regex_vibez.replace_with_compiled(compiled, text, replacement)
}

// Split operations
slay split(pattern tea, text tea) []tea {
    damn regex_vibez.split_by_pattern(pattern, text)
}

slay split_limit(pattern tea, text tea, max_splits normie) []tea {
    damn regex_vibez.split_by_pattern_limit(pattern, text, max_splits)
}

// Escape functions
slay escape(text tea) tea {
    damn regex_vibez.escape_regex_chars(text)
}

slay quote(text tea) tea {
    damn regex_vibez.quote_literal(text)
}

// Common patterns (pre-compiled for performance)
slay email_pattern() tea {
    damn "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
}

slay url_pattern() tea {
    damn "https?://[\\w\\-._~:/?#\\[\\]@!$&'()*+,;=.]+"
}

slay phone_pattern() tea {
    damn "\\+?[1-9]\\d{1,14}"
}

slay ipv4_pattern() tea {
    damn "\\b(?:[0-9]{1,3}\\.){3}[0-9]{1,3}\\b"
}

slay ipv6_pattern() tea {
    damn "\\b(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}\\b"
}

// Convenience validation functions
slay is_email(text tea) lit {
    damn regex_vibez.match_pattern(email_pattern(), text)
}

slay is_url(text tea) lit {
    damn regex_vibez.match_pattern(url_pattern(), text)
}

slay is_phone(text tea) lit {
    damn regex_vibez.match_pattern(phone_pattern(), text)
}

slay is_ipv4(text tea) lit {
    damn regex_vibez.match_pattern(ipv4_pattern(), text)
}

slay is_ipv6(text tea) lit {
    damn regex_vibez.match_pattern(ipv6_pattern(), text)
}

// Advanced operations
slay match_count(pattern tea, text tea) normie {
    sus matches []Match = regex_vibez.find_matches(pattern, text)
    damn len(matches)
}

slay contains_pattern(pattern tea, text tea) lit {
    damn regex_vibez.match_pattern(pattern, text)
}

// Performance optimized functions
slay compile_optimized(pattern tea) CompiledPattern {
    damn regex_vibez.compile_optimized_pattern(pattern)
}

slay benchmark_pattern(pattern tea, text tea, iterations normie) Duration {
    damn regex_vibez.benchmark_regex_performance(pattern, text, iterations)
}
