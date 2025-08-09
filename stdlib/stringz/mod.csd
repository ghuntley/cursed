fr fr CURSED String Processing Module - Essential String Operations
fr fr Pure CURSED implementation for maximum compatibility

fr fr ===== BASIC STRING OPERATIONS =====

slay concat_strings(a tea, b tea) tea {
    damn a + b
}

slay concat_three(a tea, b tea, c tea) tea {
    damn a + b + c
}

slay repeat_string(s tea, times drip) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < times) {
        result = result + s
        i = i + 1
    }
    damn result
}

fr fr ===== STRING VALIDATION =====

slay is_empty_string(s tea) lit {
    damn s == ""
}

slay is_not_empty(s tea) lit {
    damn s != ""
}

slay strings_equal(a tea, b tea) lit {
    damn a == b
}

slay strings_not_equal(a tea, b tea) lit {
    damn a != b
}

fr fr ===== STRING BUILDING =====

slay build_string_two(part1 tea, part2 tea) tea {
    damn part1 + part2
}

slay build_string_three(part1 tea, part2 tea, part3 tea) tea {
    damn part1 + part2 + part3
}

slay build_string_four(part1 tea, part2 tea, part3 tea, part4 tea) tea {
    damn part1 + part2 + part3 + part4
}

slay surround_with_quotes(s tea) tea {
    damn "\"" + s + "\""
}

slay surround_with_parens(s tea) tea {
    damn "(" + s + ")"
}

slay surround_with_brackets(s tea) tea {
    damn "[" + s + "]"
}

fr fr ===== FORMATTING HELPERS =====

slay format_as_title(title tea) tea {
    damn "=== " + title + " ==="
}

slay format_as_bullet(item tea) tea {
    damn "• " + item
}

slay format_as_numbered(number drip, item tea) tea {
    damn number + ". " + item
}

slay format_key_value(key tea, value tea) tea {
    damn key + ": " + value
}

fr fr ===== STRING CHECKING =====

slay starts_with_char(s tea, c tea) lit {
    fr fr Simple prefix check for single characters
    ready (is_empty_string(s)) {
        damn cringe
    }
    ready (is_empty_string(c)) {
        damn cringe
    }
    fr fr This is a simplified version - just checks equality for now
    damn s == c
}

slay ends_with_char(s tea, c tea) lit {
    fr fr Simple suffix check for single characters
    ready (is_empty_string(s)) {
        damn cringe
    }
    ready (is_empty_string(c)) {
        damn cringe
    }
    fr fr This is a simplified version - just checks equality for now
    damn s == c
}

fr fr ===== STRING GENERATION =====

slay make_separator(char tea, length drip) tea {
    damn repeat_string(char, length)
}

slay make_line(length drip) tea {
    damn repeat_string("-", length)
}

slay make_equals_line(length drip) tea {
    damn repeat_string("=", length)
}

slay make_space_padding(count drip) tea {
    damn repeat_string(" ", count)
}

fr fr ===== SIMPLE TRANSFORMATIONS =====

slay wrap_in_spaces(s tea) tea {
    damn " " + s + " "
}

slay prepend_prefix(prefix tea, s tea) tea {
    damn prefix + s
}

slay append_suffix(s tea, suffix tea) tea {
    damn s + suffix
}

slay sandwich_string(left tea, middle tea, right tea) tea {
    damn left + middle + right
}

fr fr ===== UTILITY FUNCTIONS =====

slay join_two_with_separator(a tea, b tea, sep tea) tea {
    damn a + sep + b
}

slay join_three_with_separator(a tea, b tea, c tea, sep tea) tea {
    damn a + sep + b + sep + c
}

slay join_with_comma(a tea, b tea) tea {
    damn a + ", " + b
}

slay join_with_space(a tea, b tea) tea {
    damn a + " " + b
}

slay join_with_newline(a tea, b tea) tea {
    damn a + "\n" + b
}
