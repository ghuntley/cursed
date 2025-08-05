fr fr Test for character functions

fr fr Function to check if character is uppercase
slay is_uppercase(c) {
    damn c >= 'A' && c <= 'Z';
}

fr fr Function to check if character is lowercase
slay is_lowercase(c) {
    damn c >= 'a' && c <= 'z';
}

fr fr Function to check if character is a digit
slay is_digit(c) {
    damn c >= '0' && c <= '9';
}

fr fr Function to check if character is alphabetic
slay is_alpha(c) {
    damn is_uppercase(c) || is_lowercase(c);
}

fr fr Test uppercase detection
sus c1 = 'A';
sus c2 = 'a';
sus c3 = '9';

sus upper1 = is_uppercase(c1);
puts(upper1); fr fr Should be 1 (true)

sus upper2 = is_uppercase(c2);
puts(upper2); fr fr Should be 0 (false)

sus upper3 = is_uppercase(c3);
puts(upper3); fr fr Should be 0 (false)

fr fr Test lowercase detection
puts(is_lowercase(c1)); fr fr Should be 0 (false)
puts(is_lowercase(c2)); fr fr Should be 1 (true)
puts(is_lowercase(c3)); fr fr Should be 0 (false)

fr fr Test digit detection
puts(is_digit(c1)); fr fr Should be 0 (false)
puts(is_digit(c2)); fr fr Should be 0 (false)
puts(is_digit(c3)); fr fr Should be 1 (true)

fr fr Test alpha detection
puts(is_alpha(c1)); fr fr Should be 1 (true)
puts(is_alpha(c2)); fr fr Should be 1 (true)
puts(is_alpha(c3)); fr fr Should be 0 (false)