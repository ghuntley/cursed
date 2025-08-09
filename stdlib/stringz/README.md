# STRINGZ Module - String Operations

Essential string manipulation functions for CURSED programs.

## Functions

### Basic Operations
- `concat_strings(a tea, b tea) tea` - Concatenate two strings
- `concat_three(a tea, b tea, c tea) tea` - Concatenate three strings
- `repeat_string(s tea, times drip) tea` - Repeat string multiple times

### Validation
- `is_empty_string(s tea) lit` - Check if string is empty
- `is_not_empty(s tea) lit` - Check if string is not empty
- `strings_equal(a tea, b tea) lit` - Check string equality
- `strings_not_equal(a tea, b tea) lit` - Check string inequality

### Building
- `build_string_two(part1 tea, part2 tea) tea` - Build string from two parts
- `build_string_three(part1, part2, part3 tea) tea` - Build from three parts
- `build_string_four(part1, part2, part3, part4 tea) tea` - Build from four parts

### Formatting
- `surround_with_quotes(s tea) tea` - Add quotes around string
- `surround_with_parens(s tea) tea` - Add parentheses around string
- `surround_with_brackets(s tea) tea` - Add brackets around string
- `format_as_title(title tea) tea` - Format as title with === 
- `format_as_bullet(item tea) tea` - Format as bullet point
- `format_key_value(key tea, value tea) tea` - Format as key: value

### Utility
- `join_two_with_separator(a tea, b tea, sep tea) tea` - Join with separator
- `join_with_comma(a tea, b tea) tea` - Join with comma
- `join_with_space(a tea, b tea) tea` - Join with space
- `make_line(length drip) tea` - Create line of dashes
- `make_separator(char tea, length drip) tea` - Create separator line

## Usage

```cursed
yeet "stringz"

sus greeting tea = concat_strings("Hello", " World")
sus repeated tea = repeat_string("x", 5)
sus formatted tea = format_as_title("My Title")
sus joined tea = join_with_comma("apple", "banana")

vibez.spill("Greeting:", greeting)
vibez.spill("Repeated:", repeated)
vibez.spill("Formatted:", formatted)
vibez.spill("Joined:", joined)
```
