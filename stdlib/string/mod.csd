// Standard string utilities library

// ================================
// String manipulation
// ================================

fn string_len(s: string) -> int {
    return string_length(s);
}

fn string_is_empty(s: string) -> bool {
    return string_is_empty(s);
}

fn string_trim(s: string) -> string {
    return string_trim(s);
}

fn string_trim_start(s: string) -> string {
    return string_trim_start(s);
}

fn string_trim_end(s: string) -> string {
    return string_trim_end(s);
}

fn string_to_upper(s: string) -> string {
    return string_to_upper(s);
}

fn string_to_lower(s: string) -> string {
    return string_to_lower(s);
}

fn string_capitalize(s: string) -> string {
    return string_capitalize(s);
}

fn string_reverse(s: string) -> string {
    return string_reverse(s);
}

// ================================
// String searching and matching
// ================================

fn string_contains(s: string, substr: string) -> bool {
    return string_contains(s, substr);
}

fn string_starts_with(s: string, prefix: string) -> bool {
    return string_starts_with(s, prefix);
}

fn string_ends_with(s: string, suffix: string) -> bool {
    return string_ends_with(s, suffix);
}

fn string_index_of(s: string, substr: string) -> int {
    return string_index_of(s, substr);
}

fn string_last_index_of(s: string, substr: string) -> int {
    return string_last_index_of(s, substr);
}

fn string_count_occurrences(s: string, substr: string) -> int {
    return string_count_occurrences(s, substr);
}

// ================================
// String slicing and splitting
// ================================

fn string_slice(s: string, start: int, end: int) -> string {
    return string_slice(s, start, end);
}

fn string_substring(s: string, start: int, length: int) -> string {
    return string_substring(s, start, length);
}

fn string_char_at(s: string, index: int) -> string {
    return string_char_at(s, index);
}

fn string_split(s: string, delimiter: string) -> array {
    return string_split(s, delimiter);
}

fn string_split_lines(s: string) -> array {
    return string_split_lines(s);
}

fn string_split_whitespace(s: string) -> array {
    return string_split_whitespace(s);
}

// ================================
// String replacement and formatting
// ================================

fn string_replace(s: string, old: string, new: string) -> string {
    return string_replace(s, old, new);
}

fn string_replace_all(s: string, old: string, new: string) -> string {
    return string_replace_all(s, old, new);
}

fn string_repeat(s: string, count: int) -> string {
    return string_repeat(s, count);
}

fn string_pad_left(s: string, length: int, pad_char: string) -> string {
    return string_pad_left(s, length, pad_char);
}

fn string_pad_right(s: string, length: int, pad_char: string) -> string {
    return string_pad_right(s, length, pad_char);
}

fn string_pad_center(s: string, length: int, pad_char: string) -> string {
    return string_pad_center(s, length, pad_char);
}

fn string_format(template: string, args: array) -> string {
    return string_format(template, args);
}

// ================================
// String validation and classification
// ================================

fn string_is_numeric(s: string) -> bool {
    return string_is_numeric(s);
}

fn string_is_alpha(s: string) -> bool {
    return string_is_alpha(s);
}

fn string_is_alphanumeric(s: string) -> bool {
    return string_is_alphanumeric(s);
}

fn string_is_whitespace(s: string) -> bool {
    return string_is_whitespace(s);
}

fn string_is_ascii(s: string) -> bool {
    return string_is_ascii(s);
}

// ================================
// String conversion
// ================================

fn string_to_int(s: string) -> int {
    return string_to_int(s);
}

fn string_to_float(s: string) -> float {
    return string_to_float(s);
}

fn string_to_bool(s: string) -> bool {
    return string_to_bool(s);
}

fn string_from_int(i: int) -> string {
    return string_from_int(i);
}

fn string_from_float(f: float) -> string {
    return string_from_float(f);
}

fn string_from_bool(b: bool) -> string {
    return string_from_bool(b);
}

// ================================
// String encoding/decoding
// ================================

fn string_to_bytes(s: string) -> array {
    return string_to_bytes(s);
}

fn string_from_bytes(bytes: array) -> string {
    return string_from_bytes(bytes);
}

fn string_escape(s: string) -> string {
    return string_escape(s);
}

fn string_unescape(s: string) -> string {
    return string_unescape(s);
}

// ================================
// Regular expressions
// ================================

fn regex_match(pattern: string, text: string) -> bool {
    return string_regex_match(pattern, text);
}

fn regex_find(pattern: string, text: string) -> string {
    return string_regex_find(pattern, text);
}

fn regex_find_all(pattern: string, text: string) -> array {
    return string_regex_find_all(pattern, text);
}

fn regex_replace(pattern: string, text: string, replacement: string) -> string {
    return string_regex_replace(pattern, text, replacement);
}

fn regex_split(pattern: string, text: string) -> array {
    return string_regex_split(pattern, text);
}

// ================================
// String utilities
// ================================

fn string_join(strings: array, separator: string) -> string {
    return string_join(strings, separator);
}

fn string_levenshtein_distance(s1: string, s2: string) -> int {
    return string_levenshtein_distance(s1, s2);
}

fn string_similarity(s1: string, s2: string) -> float {
    return string_similarity(s1, s2);
}

fn string_hash(s: string) -> int {
    return string_hash(s);
}
