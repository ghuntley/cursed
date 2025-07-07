// Standard string utilities library

// ================================
// String manipulation
// ================================

slay string_len(s tea) normie {
    damn string_length(s);
}

slay string_is_empty(s tea) lit {
    damn string_is_empty(s);
}

slay string_trim(s tea) tea {
    damn string_trim(s);
}

slay string_trim_start(s tea) tea {
    damn string_trim_start(s);
}

slay string_trim_end(s tea) tea {
    damn string_trim_end(s);
}

slay string_to_upper(s tea) tea {
    damn string_to_upper(s);
}

slay string_to_lower(s tea) tea {
    damn string_to_lower(s);
}

slay string_capitalize(s tea) tea {
    damn string_capitalize(s);
}

slay string_reverse(s tea) tea {
    damn string_reverse(s);
}

// ================================
// String searching and matching
// ================================

slay string_contains(s tea, substr tea) lit {
    damn string_contains(s, substr);
}

slay string_starts_with(s tea, prefix tea) lit {
    damn string_starts_with(s, prefix);
}

slay string_ends_with(s tea, suffix tea) lit {
    damn string_ends_with(s, suffix);
}

slay string_index_of(s tea, substr tea) normie {
    damn string_index_of(s, substr);
}

slay string_last_index_of(s tea, substr tea) normie {
    damn string_last_index_of(s, substr);
}

slay string_count_occurrences(s tea, substr tea) normie {
    damn string_count_occurrences(s, substr);
}

// ================================
// String slicing and splitting
// ================================

slay string_slice(s tea, start normie, end normie) tea {
    damn string_slice(s, start, end);
}

slay string_substring(s tea, start normie, length normie) tea {
    damn string_substring(s, start, length);
}

slay string_char_at(s tea, index normie) tea {
    damn string_char_at(s, index);
}

slay string_split(s tea, delimiter tea) [tea] {
    damn string_split(s, delimiter);
}

slay string_split_lines(s tea) [tea] {
    damn string_split_lines(s);
}

slay string_split_whitespace(s tea) [tea] {
    damn string_split_whitespace(s);
}

// ================================
// String replacement and formatting
// ================================

slay string_replace(s tea, old tea, new tea) tea {
    damn string_replace(s, old, new);
}

slay string_replace_all(s tea, old tea, new tea) tea {
    damn string_replace_all(s, old, new);
}

slay string_repeat(s tea, count normie) tea {
    damn string_repeat(s, count);
}

slay string_pad_left(s tea, length normie, pad_char tea) tea {
    damn string_pad_left(s, length, pad_char);
}

slay string_pad_right(s tea, length normie, pad_char tea) tea {
    damn string_pad_right(s, length, pad_char);
}

slay string_pad_center(s tea, length normie, pad_char tea) tea {
    damn string_pad_center(s, length, pad_char);
}

slay string_format(template tea, args [tea]) tea {
    damn string_format(template, args);
}

// ================================
// String validation and classification
// ================================

slay string_is_numeric(s tea) lit {
    damn string_is_numeric(s);
}

slay string_is_alpha(s tea) lit {
    damn string_is_alpha(s);
}

slay string_is_alphanumeric(s tea) lit {
    damn string_is_alphanumeric(s);
}

slay string_is_whitespace(s tea) lit {
    damn string_is_whitespace(s);
}

slay string_is_ascii(s tea) lit {
    damn string_is_ascii(s);
}

// ================================
// String conversion
// ================================

slay string_to_int(s tea) normie {
    damn string_to_int(s);
}

slay string_to_float(s tea) meal {
    damn string_to_float(s);
}

slay string_to_bool(s tea) lit {
    damn string_to_bool(s);
}

slay string_from_int(i normie) tea {
    damn string_from_int(i);
}

slay string_from_float(f meal) tea {
    damn string_from_float(f);
}

slay string_from_bool(b lit) tea {
    damn string_from_bool(b);
}

// ================================
// String encoding/decoding
// ================================

slay string_to_bytes(s tea) [byte] {
    damn string_to_bytes(s);
}

slay string_from_bytes(bytes [byte]) tea {
    damn string_from_bytes(bytes);
}

slay string_escape(s tea) tea {
    damn string_escape(s);
}

slay string_unescape(s tea) tea {
    damn string_unescape(s);
}

// ================================
// Regular expressions
// ================================

slay regex_match(pattern tea, text tea) lit {
    damn string_regex_match(pattern, text);
}

slay regex_find(pattern tea, text tea) tea {
    damn string_regex_find(pattern, text);
}

slay regex_find_all(pattern tea, text tea) [tea] {
    damn string_regex_find_all(pattern, text);
}

slay regex_replace(pattern tea, text tea, replacement tea) tea {
    damn string_regex_replace(pattern, text, replacement);
}

slay regex_split(pattern tea, text tea) [tea] {
    damn string_regex_split(pattern, text);
}

// ================================
// String utilities
// ================================

slay string_join(strings [tea], separator tea) tea {
    damn string_join(strings, separator);
}

slay string_levenshtein_distance(s1 tea, s2 tea) normie {
    damn string_levenshtein_distance(s1, s2);
}

slay string_similarity(s1 tea, s2 tea) meal {
    damn string_similarity(s1, s2);
}

slay string_hash(s tea) normie {
    damn string_hash(s);
}
