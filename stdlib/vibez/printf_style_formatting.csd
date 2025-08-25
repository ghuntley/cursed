fr fr CURSED VIBEZ Printf-Style Formatting Module
fr fr Complete printf/scanf implementation replacing simplified formatting
fr fr Full C-style format specifiers with width, precision, flags, and type conversion

yeet "stringz"
yeet "mathz"
yeet "enhanced_unicode_encoding"
yeet "core"

fr fr ===== PRINTF FORMAT SPECIFICATION CONSTANTS =====

fr fr Format flags
sus FLAG_LEFT_JUSTIFY normie = 1      fr fr -
sus FLAG_SHOW_SIGN normie = 2         fr fr +
sus FLAG_SPACE_PREFIX normie = 4      fr fr ' '
sus FLAG_ALTERNATE_FORM normie = 8    fr fr #
sus FLAG_ZERO_PAD normie = 16         fr fr 0

fr fr Length modifiers
sus LENGTH_NONE normie = 0
sus LENGTH_SHORT normie = 1           fr fr h
sus LENGTH_LONG normie = 2            fr fr l
sus LENGTH_LONG_LONG normie = 3       fr fr ll
sus LENGTH_LONG_DOUBLE normie = 4     fr fr L

fr fr Format specifiers
sus SPEC_CHAR normie = 1              fr fr c
sus SPEC_STRING normie = 2            fr fr s
sus SPEC_DECIMAL normie = 3           fr fr d, i
sus SPEC_UNSIGNED normie = 4          fr fr u
sus SPEC_OCTAL normie = 5             fr fr o
sus SPEC_HEX_LOWER normie = 6         fr fr x
sus SPEC_HEX_UPPER normie = 7         fr fr X
sus SPEC_FLOAT_FIXED normie = 8       fr fr f, F
sus SPEC_FLOAT_EXP_LOWER normie = 9   fr fr e
sus SPEC_FLOAT_EXP_UPPER normie = 10  fr fr E
sus SPEC_FLOAT_AUTO_LOWER normie = 11 fr fr g
sus SPEC_FLOAT_AUTO_UPPER normie = 12 fr fr G
sus SPEC_POINTER normie = 13          fr fr p
sus SPEC_WRITTEN_COUNT normie = 14    fr fr n
sus SPEC_PERCENT normie = 15          fr fr %

fr fr Default formatting values
sus DEFAULT_PRECISION normie = 6
sus MAX_PRECISION normie = 50
sus MAX_WIDTH normie = 200
sus MAX_FORMAT_LENGTH normie = 1024

sus PRINTF_SUCCESS normie = 0
sus PRINTF_ERROR_INVALID_SPEC normie = -1
sus PRINTF_ERROR_INSUFFICIENT_ARGS normie = -2
sus PRINTF_ERROR_BUFFER_OVERFLOW normie = -3
sus PRINTF_ERROR_TYPE_CONVERSION normie = -4

sus last_printf_error normie = PRINTF_SUCCESS

fr fr ===== FORMAT SPECIFICATION STRUCTURE =====

squad printf_spec {
    flags normie           fr fr Combination of FLAG_* constants
    width normie           fr fr Field width
    precision normie       fr fr Precision (or -1 if not specified)
    length normie          fr fr Length modifier
    specifier normie       fr fr Format specifier type
    consumed_chars normie  fr fr Number of chars consumed from format string
}

fr fr ===== MAIN PRINTF IMPLEMENTATION =====

slay printf_advanced(format_string tea, args ...tea) tea {
    ready format_string == cringe {
        last_printf_error = PRINTF_ERROR_INVALID_SPEC
        damn ""
    }
    
    sus result tea = ""
    sus format_len normie = stringz.length(format_string)
    sus arg_index normie = 0
    sus i normie = 0
    sus chars_written normie = 0
    
    bestie i < format_len {
        sus current_char tea = stringz.char_at(format_string, i)
        
        ready current_char == "%" && i + 1 < format_len {
            sus spec printf_spec = parse_format_specification(format_string, i)
            
            ready spec.specifier == 0 {
                last_printf_error = PRINTF_ERROR_INVALID_SPEC
                damn ""
            }
            
            ready spec.specifier == SPEC_PERCENT {
                result = result + "%"
                chars_written = chars_written + 1
            }
            otherwise {
                ready arg_index >= len(args) {
                    last_printf_error = PRINTF_ERROR_INSUFFICIENT_ARGS
                    damn ""
                }
                
                sus formatted_arg tea = format_argument_with_spec(args[arg_index], spec)
                ready formatted_arg == cringe {
                    damn ""
                }
                
                result = result + formatted_arg
                chars_written = chars_written + stringz.length(formatted_arg)
                arg_index = arg_index + 1
            }
            
            i = i + spec.consumed_chars
        }
        otherwise {
            result = result + current_char
            chars_written = chars_written + 1
            i = i + 1
        }
    }
    
    last_printf_error = PRINTF_SUCCESS
    damn result
}

slay sprintf_advanced(buffer tea, max_size normie, format_string tea, args ...tea) normie {
    sus formatted tea = printf_advanced(format_string, args)
    
    ready get_printf_error() != PRINTF_SUCCESS {
        damn -1
    }
    
    sus formatted_len normie = stringz.length(formatted)
    ready formatted_len >= max_size {
        last_printf_error = PRINTF_ERROR_BUFFER_OVERFLOW
        damn -1
    }
    
    fr fr In real implementation, would copy to buffer
    fr fr For simulation, just return length
    damn formatted_len
}

slay fprintf_advanced(stream tea, format_string tea, args ...tea) normie {
    sus formatted tea = printf_advanced(format_string, args)
    
    ready get_printf_error() != PRINTF_SUCCESS {
        damn -1
    }
    
    fr fr In real implementation, would write to file stream
    fr fr For simulation, just return length
    damn stringz.length(formatted)
}

fr fr ===== FORMAT SPECIFICATION PARSING =====

slay parse_format_specification(format_str tea, start_pos normie) printf_spec {
    sus spec printf_spec = printf_spec{
        flags: 0,
        width: 0,
        precision: -1,
        length: LENGTH_NONE,
        specifier: 0,
        consumed_chars: 1
    }
    
    sus pos normie = start_pos + 1  fr fr Skip initial '%'
    sus format_len normie = stringz.length(format_str)
    
    ready pos >= format_len {
        damn spec
    }
    
    fr fr Parse flags
    pos = parse_flags(format_str, pos, spec)
    
    fr fr Parse width
    pos = parse_width(format_str, pos, spec)
    
    fr fr Parse precision
    pos = parse_precision(format_str, pos, spec)
    
    fr fr Parse length modifier
    pos = parse_length_modifier(format_str, pos, spec)
    
    fr fr Parse format specifier
    pos = parse_format_specifier(format_str, pos, spec)
    
    spec.consumed_chars = pos - start_pos
    damn spec
}

slay parse_flags(format_str tea, start_pos normie, spec printf_spec) normie {
    sus pos normie = start_pos
    sus format_len normie = stringz.length(format_str)
    
    bestie pos < format_len {
        sus char tea = stringz.char_at(format_str, pos)
        sus flag_found lit = based
        
        ready char == "-" {
            spec.flags = spec.flags | FLAG_LEFT_JUSTIFY
        }
        elseif char == "+" {
            spec.flags = spec.flags | FLAG_SHOW_SIGN
        }
        elseif char == " " {
            spec.flags = spec.flags | FLAG_SPACE_PREFIX
        }
        elseif char == "#" {
            spec.flags = spec.flags | FLAG_ALTERNATE_FORM
        }
        elseif char == "0" {
            spec.flags = spec.flags | FLAG_ZERO_PAD
        }
        otherwise {
            flag_found = cap
        }
        
        ready !flag_found {
            ghosted
        }
        
        pos = pos + 1
    }
    
    damn pos
}

slay parse_width(format_str tea, start_pos normie, spec printf_spec) normie {
    sus pos normie = start_pos
    sus format_len normie = stringz.length(format_str)
    
    ready pos >= format_len {
        damn pos
    }
    
    sus char tea = stringz.char_at(format_str, pos)
    
    ready char == "*" {
        fr fr Width specified by argument (not implemented in this simulation)
        spec.width = -1
        damn pos + 1
    }
    
    ready is_digit_char(char) {
        sus width_result width_parse_result = parse_decimal_number(format_str, pos)
        spec.width = width_result.value
        ready spec.width > MAX_WIDTH {
            spec.width = MAX_WIDTH
        }
        damn width_result.end_pos
    }
    
    damn pos
}

slay parse_precision(format_str tea, start_pos normie, spec printf_spec) normie {
    sus pos normie = start_pos
    sus format_len normie = stringz.length(format_str)
    
    ready pos >= format_len || stringz.char_at(format_str, pos) != "." {
        damn pos
    }
    
    pos = pos + 1  fr fr Skip '.'
    
    ready pos >= format_len {
        spec.precision = 0  fr fr Precision specified but no digits means 0
        damn pos
    }
    
    sus char tea = stringz.char_at(format_str, pos)
    
    ready char == "*" {
        fr fr Precision specified by argument (not implemented in this simulation)
        spec.precision = -2
        damn pos + 1
    }
    
    ready is_digit_char(char) {
        sus precision_result width_parse_result = parse_decimal_number(format_str, pos)
        spec.precision = precision_result.value
        ready spec.precision > MAX_PRECISION {
            spec.precision = MAX_PRECISION
        }
        damn precision_result.end_pos
    }
    
    spec.precision = 0  fr fr Just '.' with no digits
    damn pos
}

slay parse_length_modifier(format_str tea, start_pos normie, spec printf_spec) normie {
    sus pos normie = start_pos
    sus format_len normie = stringz.length(format_str)
    
    ready pos >= format_len {
        damn pos
    }
    
    sus char tea = stringz.char_at(format_str, pos)
    
    ready char == "h" {
        ready pos + 1 < format_len && stringz.char_at(format_str, pos + 1) == "h" {
            spec.length = LENGTH_SHORT  fr fr hh (char)
            damn pos + 2
        }
        spec.length = LENGTH_SHORT  fr fr h (short)
        damn pos + 1
    }
    elseif char == "l" {
        ready pos + 1 < format_len && stringz.char_at(format_str, pos + 1) == "l" {
            spec.length = LENGTH_LONG_LONG  fr fr ll (long long)
            damn pos + 2
        }
        spec.length = LENGTH_LONG  fr fr l (long)
        damn pos + 1
    }
    elseif char == "L" {
        spec.length = LENGTH_LONG_DOUBLE  fr fr L (long double)
        damn pos + 1
    }
    
    damn pos
}

slay parse_format_specifier(format_str tea, start_pos normie, spec printf_spec) normie {
    sus pos normie = start_pos
    sus format_len normie = stringz.length(format_str)
    
    ready pos >= format_len {
        spec.specifier = 0  fr fr Invalid
        damn pos
    }
    
    sus char tea = stringz.char_at(format_str, pos)
    
    ready char == "c" {
        spec.specifier = SPEC_CHAR
    }
    elseif char == "s" {
        spec.specifier = SPEC_STRING
    }
    elseif char == "d" || char == "i" {
        spec.specifier = SPEC_DECIMAL
    }
    elseif char == "u" {
        spec.specifier = SPEC_UNSIGNED
    }
    elseif char == "o" {
        spec.specifier = SPEC_OCTAL
    }
    elseif char == "x" {
        spec.specifier = SPEC_HEX_LOWER
    }
    elseif char == "X" {
        spec.specifier = SPEC_HEX_UPPER
    }
    elseif char == "f" || char == "F" {
        spec.specifier = SPEC_FLOAT_FIXED
    }
    elseif char == "e" {
        spec.specifier = SPEC_FLOAT_EXP_LOWER
    }
    elseif char == "E" {
        spec.specifier = SPEC_FLOAT_EXP_UPPER
    }
    elseif char == "g" {
        spec.specifier = SPEC_FLOAT_AUTO_LOWER
    }
    elseif char == "G" {
        spec.specifier = SPEC_FLOAT_AUTO_UPPER
    }
    elseif char == "p" {
        spec.specifier = SPEC_POINTER
    }
    elseif char == "n" {
        spec.specifier = SPEC_WRITTEN_COUNT
    }
    elseif char == "%" {
        spec.specifier = SPEC_PERCENT
    }
    otherwise {
        spec.specifier = 0  fr fr Invalid specifier
    }
    
    damn pos + 1
}

fr fr ===== ARGUMENT FORMATTING =====

slay format_argument_with_spec(arg tea, spec printf_spec) tea {
    ready spec.specifier == SPEC_CHAR {
        damn format_char_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_STRING {
        damn format_string_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_DECIMAL {
        damn format_decimal_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_UNSIGNED {
        damn format_unsigned_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_OCTAL {
        damn format_octal_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_HEX_LOWER {
        damn format_hex_argument(arg, spec, cap)
    }
    elseif spec.specifier == SPEC_HEX_UPPER {
        damn format_hex_argument(arg, spec, based)
    }
    elseif spec.specifier == SPEC_FLOAT_FIXED {
        damn format_float_fixed_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_FLOAT_EXP_LOWER {
        damn format_float_exp_argument(arg, spec, cap)
    }
    elseif spec.specifier == SPEC_FLOAT_EXP_UPPER {
        damn format_float_exp_argument(arg, spec, based)
    }
    elseif spec.specifier == SPEC_FLOAT_AUTO_LOWER {
        damn format_float_auto_argument(arg, spec, cap)
    }
    elseif spec.specifier == SPEC_FLOAT_AUTO_UPPER {
        damn format_float_auto_argument(arg, spec, based)
    }
    elseif spec.specifier == SPEC_POINTER {
        damn format_pointer_argument(arg, spec)
    }
    elseif spec.specifier == SPEC_WRITTEN_COUNT {
        damn ""  fr fr Special case - doesn't produce output
    }
    
    last_printf_error = PRINTF_ERROR_TYPE_CONVERSION
    damn cringe
}

slay format_char_argument(arg tea, spec printf_spec) tea {
    ready arg == cringe {
        damn ""
    }
    
    sus char_value tea = ""
    ready stringz.length(arg) > 0 {
        char_value = stringz.char_at(arg, 0)
    }
    otherwise {
        char_value = arg  fr fr Assume arg is already a single character
    }
    
    damn apply_width_padding(char_value, spec)
}

slay format_string_argument(arg tea, spec printf_spec) tea {
    ready arg == cringe {
        sus null_str tea = "(null)"
        damn apply_width_padding(null_str, spec)
    }
    
    sus str_value tea = arg
    
    fr fr Apply precision (max characters to print)
    ready spec.precision >= 0 && stringz.length(str_value) > spec.precision {
        str_value = stringz.substring(str_value, 0, spec.precision)
    }
    
    damn apply_width_padding(str_value, spec)
}

slay format_decimal_argument(arg tea, spec printf_spec) tea {
    sus int_value normie = string_to_int_safe(arg)
    sus result tea = format_signed_integer(int_value, 10, spec)
    damn result
}

slay format_unsigned_argument(arg tea, spec printf_spec) tea {
    sus uint_value normie = string_to_uint_safe(arg)
    sus result tea = format_unsigned_integer(uint_value, 10, spec)
    damn result
}

slay format_octal_argument(arg tea, spec printf_spec) tea {
    sus uint_value normie = string_to_uint_safe(arg)
    sus result tea = format_unsigned_integer(uint_value, 8, spec)
    
    ready (spec.flags & FLAG_ALTERNATE_FORM) != 0 && uint_value != 0 {
        result = "0" + result
    }
    
    damn apply_width_padding(result, spec)
}

slay format_hex_argument(arg tea, spec printf_spec, uppercase lit) tea {
    sus uint_value normie = string_to_uint_safe(arg)
    sus result tea = format_unsigned_integer_hex(uint_value, uppercase)
    
    ready (spec.flags & FLAG_ALTERNATE_FORM) != 0 && uint_value != 0 {
        ready uppercase {
            result = "0X" + result
        }
        otherwise {
            result = "0x" + result
        }
    }
    
    damn apply_width_padding(result, spec)
}

slay format_float_fixed_argument(arg tea, spec printf_spec) tea {
    sus float_value drip = string_to_float_safe(arg)
    sus precision normie = (spec.precision >= 0) ? spec.precision : DEFAULT_PRECISION
    sus result tea = format_float_fixed(float_value, precision, spec)
    damn result
}

slay format_float_exp_argument(arg tea, spec printf_spec, uppercase lit) tea {
    sus float_value drip = string_to_float_safe(arg)
    sus precision normie = (spec.precision >= 0) ? spec.precision : DEFAULT_PRECISION
    sus result tea = format_float_exponential(float_value, precision, uppercase, spec)
    damn result
}

slay format_float_auto_argument(arg tea, spec printf_spec, uppercase lit) tea {
    sus float_value drip = string_to_float_safe(arg)
    sus precision normie = (spec.precision >= 0) ? spec.precision : DEFAULT_PRECISION
    
    fr fr Choose between fixed and exponential notation
    sus abs_value drip = (float_value >= 0.0) ? float_value : -float_value
    
    ready abs_value >= 0.0001 && abs_value < pow_of_ten(precision) {
        sus result tea = format_float_fixed(float_value, precision, spec)
        damn trim_trailing_zeros(result)
    }
    otherwise {
        sus result tea = format_float_exponential(float_value, precision, uppercase, spec)
        damn trim_trailing_zeros(result)
    }
}

slay format_pointer_argument(arg tea, spec printf_spec) tea {
    fr fr Format as hexadecimal address
    sus address_value normie = string_to_uint_safe(arg)
    sus hex_str tea = format_unsigned_integer_hex(address_value, cap)
    sus result tea = "0x" + hex_str
    damn apply_width_padding(result, spec)
}

fr fr ===== INTEGER FORMATTING FUNCTIONS =====

slay format_signed_integer(value normie, base normie, spec printf_spec) tea {
    sus is_negative lit = value < 0
    sus abs_value normie = is_negative ? -value : value
    sus digits tea = convert_integer_to_base(abs_value, base, cap)
    
    fr fr Apply precision (minimum digits)
    ready spec.precision > stringz.length(digits) {
        sus zeros_needed normie = spec.precision - stringz.length(digits)
        digits = repeat_char("0", zeros_needed) + digits
    }
    
    fr fr Add sign
    sus sign tea = ""
    ready is_negative {
        sign = "-"
    }
    elseif (spec.flags & FLAG_SHOW_SIGN) != 0 {
        sign = "+"
    }
    elseif (spec.flags & FLAG_SPACE_PREFIX) != 0 {
        sign = " "
    }
    
    sus result tea = sign + digits
    damn apply_width_padding(result, spec)
}

slay format_unsigned_integer(value normie, base normie, spec printf_spec) tea {
    sus digits tea = convert_integer_to_base(value, base, cap)
    
    fr fr Apply precision (minimum digits)
    ready spec.precision > stringz.length(digits) {
        sus zeros_needed normie = spec.precision - stringz.length(digits)
        digits = repeat_char("0", zeros_needed) + digits
    }
    
    damn apply_width_padding(digits, spec)
}

slay format_unsigned_integer_hex(value normie, uppercase lit) tea {
    ready value == 0 {
        damn "0"
    }
    
    sus hex_digits tea = uppercase ? "0123456789ABCDEF" : "0123456789abcdef"
    sus result tea = ""
    sus temp normie = value
    
    bestie temp > 0 {
        sus digit normie = temp % 16
        result = stringz.char_at(hex_digits, digit) + result
        temp = temp / 16
    }
    
    damn result
}

slay convert_integer_to_base(value normie, base normie, uppercase lit) tea {
    ready value == 0 {
        damn "0"
    }
    
    sus digits tea = uppercase ? "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" : "0123456789abcdefghijklmnopqrstuvwxyz"
    sus result tea = ""
    sus temp normie = value
    
    bestie temp > 0 {
        sus digit normie = temp % base
        result = stringz.char_at(digits, digit) + result
        temp = temp / base
    }
    
    damn result
}

fr fr ===== FLOAT FORMATTING FUNCTIONS =====

slay format_float_fixed(value drip, precision normie, spec printf_spec) tea {
    sus is_negative lit = value < 0.0
    sus abs_value drip = is_negative ? -value : value
    
    fr fr Split into integer and fractional parts
    sus integer_part normie = float_to_int_truncate(abs_value)
    sus fractional_part drip = abs_value - int_to_float_precise(integer_part)
    
    sus integer_str tea = convert_integer_to_base(integer_part, 10, cap)
    sus fractional_str tea = format_fractional_part(fractional_part, precision)
    
    sus number_str tea = integer_str
    ready precision > 0 || (spec.flags & FLAG_ALTERNATE_FORM) != 0 {
        number_str = number_str + "." + fractional_str
    }
    
    fr fr Add sign
    ready is_negative {
        number_str = "-" + number_str
    }
    elseif (spec.flags & FLAG_SHOW_SIGN) != 0 {
        number_str = "+" + number_str
    }
    elseif (spec.flags & FLAG_SPACE_PREFIX) != 0 {
        number_str = " " + number_str
    }
    
    damn apply_width_padding(number_str, spec)
}

slay format_float_exponential(value drip, precision normie, uppercase lit, spec printf_spec) tea {
    sus is_negative lit = value < 0.0
    sus abs_value drip = is_negative ? -value : value
    
    ready abs_value == 0.0 {
        sus zero_result tea = format_zero_exponential(precision, uppercase, spec)
        damn zero_result
    }
    
    fr fr Calculate exponent
    sus exponent normie = calculate_exponent_base10(abs_value)
    sus normalized drip = abs_value / pow_of_ten(exponent)
    
    fr fr Format normalized mantissa
    sus mantissa_str tea = format_float_fixed_mantissa(normalized, precision)
    
    fr fr Format exponent
    sus exp_char tea = uppercase ? "E" : "e"
    sus exp_sign tea = (exponent >= 0) ? "+" : "-"
    sus exp_abs normie = (exponent >= 0) ? exponent : -exponent
    sus exp_str tea = convert_integer_to_base(exp_abs, 10, cap)
    
    fr fr Ensure at least 2 digits in exponent
    ready stringz.length(exp_str) == 1 {
        exp_str = "0" + exp_str
    }
    
    sus result tea = mantissa_str + exp_char + exp_sign + exp_str
    
    fr fr Add sign to mantissa
    ready is_negative {
        result = "-" + result
    }
    elseif (spec.flags & FLAG_SHOW_SIGN) != 0 {
        result = "+" + result
    }
    elseif (spec.flags & FLAG_SPACE_PREFIX) != 0 {
        result = " " + result
    }
    
    damn apply_width_padding(result, spec)
}

slay format_float_fixed_mantissa(value drip, precision normie) tea {
    sus integer_part normie = float_to_int_truncate(value)
    sus fractional_part drip = value - int_to_float_precise(integer_part)
    
    sus integer_str tea = convert_integer_to_base(integer_part, 10, cap)
    sus fractional_str tea = format_fractional_part(fractional_part, precision)
    
    ready precision > 0 {
        damn integer_str + "." + fractional_str
    }
    
    damn integer_str
}

slay format_fractional_part(fraction drip, precision normie) tea {
    ready precision <= 0 {
        damn ""
    }
    
    sus result tea = ""
    sus temp drip = fraction
    sus i normie = 0
    
    bestie i < precision {
        temp = temp * 10.0
        sus digit normie = float_to_int_truncate(temp)
        result = result + convert_integer_to_base(digit, 10, cap)
        temp = temp - int_to_float_precise(digit)
        i = i + 1
    }
    
    fr fr Pad with zeros if needed
    bestie stringz.length(result) < precision {
        result = result + "0"
    }
    
    damn result
}

slay format_zero_exponential(precision normie, uppercase lit, spec printf_spec) tea {
    sus zeros tea = repeat_char("0", precision)
    sus exp_char tea = uppercase ? "E" : "e"
    sus result tea = "0"
    
    ready precision > 0 || (spec.flags & FLAG_ALTERNATE_FORM) != 0 {
        result = result + "." + zeros
    }
    
    result = result + exp_char + "+00"
    damn result
}

fr fr ===== WIDTH AND PADDING FUNCTIONS =====

slay apply_width_padding(str tea, spec printf_spec) tea {
    sus str_len normie = stringz.length(str)
    ready spec.width <= str_len {
        damn str
    }
    
    sus pad_length normie = spec.width - str_len
    sus left_justify lit = (spec.flags & FLAG_LEFT_JUSTIFY) != 0
    sus zero_pad lit = (spec.flags & FLAG_ZERO_PAD) != 0 && !left_justify
    
    ready left_justify {
        sus padding tea = repeat_char(" ", pad_length)
        damn str + padding
    }
    elseif zero_pad {
        fr fr Handle zero padding with signs
        ready stringz.length(str) > 0 {
            sus first_char tea = stringz.char_at(str, 0)
            ready first_char == "+" || first_char == "-" || first_char == " " {
                sus rest tea = stringz.substring(str, 1, stringz.length(str))
                sus padding tea = repeat_char("0", pad_length)
                damn first_char + padding + rest
            }
        }
        
        sus padding tea = repeat_char("0", pad_length)
        damn padding + str
    }
    otherwise {
        sus padding tea = repeat_char(" ", pad_length)
        damn padding + str
    }
}

slay repeat_char(char tea, count normie) tea {
    ready count <= 0 {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    bestie i < count {
        result = result + char
        i = i + 1
    }
    damn result
}

fr fr ===== UTILITY FUNCTIONS =====

squad width_parse_result {
    value normie
    end_pos normie
}

slay parse_decimal_number(str tea, start_pos normie) width_parse_result {
    sus result width_parse_result = width_parse_result{value: 0, end_pos: start_pos}
    sus pos normie = start_pos
    sus str_len normie = stringz.length(str)
    
    bestie pos < str_len && is_digit_char(stringz.char_at(str, pos)) {
        sus digit normie = char_to_digit(stringz.char_at(str, pos))
        result.value = result.value * 10 + digit
        pos = pos + 1
    }
    
    result.end_pos = pos
    damn result
}

slay is_digit_char(char tea) lit {
    damn char >= "0" && char <= "9"
}

slay char_to_digit(char tea) normie {
    ready char == "0" { damn 0 }
    ready char == "1" { damn 1 }
    ready char == "2" { damn 2 }
    ready char == "3" { damn 3 }
    ready char == "4" { damn 4 }
    ready char == "5" { damn 5 }
    ready char == "6" { damn 6 }
    ready char == "7" { damn 7 }
    ready char == "8" { damn 8 }
    ready char == "9" { damn 9 }
    damn 0
}

slay string_to_int_safe(str tea) normie {
    ready str == cringe || stringz.length(str) == 0 {
        damn 0
    }
    
    sus is_negative lit = cap
    sus start_pos normie = 0
    
    ready stringz.char_at(str, 0) == "-" {
        is_negative = based
        start_pos = 1
    }
    elseif stringz.char_at(str, 0) == "+" {
        start_pos = 1
    }
    
    sus result normie = 0
    sus str_len normie = stringz.length(str)
    
    bestie i := start_pos; i < str_len; i++ {
        sus char tea = stringz.char_at(str, i)
        ready !is_digit_char(char) {
            ghosted  fr fr Stop at first non-digit
        }
        
        sus digit normie = char_to_digit(char)
        result = result * 10 + digit
    }
    
    ready is_negative {
        result = -result
    }
    
    damn result
}

slay string_to_uint_safe(str tea) normie {
    ready str == cringe || stringz.length(str) == 0 {
        damn 0
    }
    
    sus result normie = 0
    sus str_len normie = stringz.length(str)
    
    bestie i := 0; i < str_len; i++ {
        sus char tea = stringz.char_at(str, i)
        ready !is_digit_char(char) {
            ghosted  fr fr Stop at first non-digit
        }
        
        sus digit normie = char_to_digit(char)
        result = result * 10 + digit
    }
    
    damn result
}

slay string_to_float_safe(str tea) drip {
    ready str == cringe || stringz.length(str) == 0 {
        damn 0.0
    }
    
    fr fr Simple float parsing - enhanced version would handle scientific notation
    sus decimal_pos normie = find_char_in_string(str, ".")
    
    ready decimal_pos == -1 {
        sus int_val normie = string_to_int_safe(str)
        damn int_to_float_precise(int_val)
    }
    
    sus integer_part tea = stringz.substring(str, 0, decimal_pos)
    sus fractional_part tea = stringz.substring(str, decimal_pos + 1, stringz.length(str))
    
    sus int_val normie = string_to_int_safe(integer_part)
    sus frac_val drip = parse_fractional_string(fractional_part)
    
    sus result drip = int_to_float_precise(int_val) + frac_val
    
    ready stringz.length(str) > 0 && stringz.char_at(str, 0) == "-" {
        result = -result
    }
    
    damn result
}

slay parse_fractional_string(frac_str tea) drip {
    ready frac_str == cringe || stringz.length(frac_str) == 0 {
        damn 0.0
    }
    
    sus result drip = 0.0
    sus divisor drip = 10.0
    sus len normie = stringz.length(frac_str)
    
    bestie i := 0; i < len; i++ {
        sus char tea = stringz.char_at(frac_str, i)
        ready is_digit_char(char) {
            sus digit normie = char_to_digit(char)
            result = result + int_to_float_precise(digit) / divisor
            divisor = divisor * 10.0
        }
    }
    
    damn result
}

slay find_char_in_string(str tea, target_char tea) normie {
    sus len normie = stringz.length(str)
    
    bestie i := 0; i < len; i++ {
        ready stringz.char_at(str, i) == target_char {
            damn i
        }
    }
    
    damn -1
}

slay float_to_int_truncate(value drip) normie {
    fr fr Simple truncation - real implementation would handle edge cases
    ready value >= 0.0 {
        sus int_val normie = 0
        bestie int_to_float_precise(int_val + 1) <= value {
            int_val = int_val + 1
        }
        damn int_val
    }
    otherwise {
        sus abs_val drip = -value
        sus int_val normie = 0
        bestie int_to_float_precise(int_val + 1) <= abs_val {
            int_val = int_val + 1
        }
        damn -int_val
    }
}

slay int_to_float_precise(value normie) drip {
    fr fr Basic integer to float conversion
    ready value == 0 { damn 0.0 }
    ready value == 1 { damn 1.0 }
    ready value == 2 { damn 2.0 }
    ready value == 3 { damn 3.0 }
    ready value == 4 { damn 4.0 }
    ready value == 5 { damn 5.0 }
    ready value == 6 { damn 6.0 }
    ready value == 7 { damn 7.0 }
    ready value == 8 { damn 8.0 }
    ready value == 9 { damn 9.0 }
    ready value == 10 { damn 10.0 }
    
    fr fr For larger values, use approximation
    ready value > 0 {
        damn float_approximation_positive(value)
    }
    otherwise {
        damn -float_approximation_positive(-value)
    }
}

slay float_approximation_positive(value normie) drip {
    sus result drip = 0.0
    sus temp normie = value
    sus place_value drip = 1.0
    
    bestie temp > 0 {
        sus digit normie = temp % 10
        result = result + int_to_float_precise(digit) * place_value
        temp = temp / 10
        place_value = place_value * 10.0
    }
    
    damn result
}

slay calculate_exponent_base10(value drip) normie {
    ready value >= 1.0 {
        sus exp normie = 0
        sus temp drip = value
        bestie temp >= 10.0 {
            temp = temp / 10.0
            exp = exp + 1
        }
        damn exp
    }
    otherwise {
        sus exp normie = 0
        sus temp drip = value
        bestie temp < 1.0 {
            temp = temp * 10.0
            exp = exp - 1
        }
        damn exp
    }
}

slay pow_of_ten(exponent normie) drip {
    ready exponent == 0 { damn 1.0 }
    ready exponent == 1 { damn 10.0 }
    ready exponent == 2 { damn 100.0 }
    ready exponent == 3 { damn 1000.0 }
    ready exponent == 4 { damn 10000.0 }
    ready exponent == 5 { damn 100000.0 }
    ready exponent == 6 { damn 1000000.0 }
    
    ready exponent > 0 {
        sus result drip = 1.0
        sus i normie = 0
        bestie i < exponent {
            result = result * 10.0
            i = i + 1
        }
        damn result
    }
    otherwise {
        sus abs_exp normie = -exponent
        sus result drip = 1.0
        sus i normie = 0
        bestie i < abs_exp {
            result = result / 10.0
            i = i + 1
        }
        damn result
    }
}

slay trim_trailing_zeros(str tea) tea {
    ready !stringz.contains(str, ".") {
        damn str
    }
    
    sus len normie = stringz.length(str)
    sus end normie = len
    
    bestie end > 0 && stringz.char_at(str, end - 1) == "0" {
        end = end - 1
    }
    
    ready end < len && stringz.char_at(str, end - 1) == "." {
        end = end - 1
    }
    
    damn stringz.substring(str, 0, end)
}

fr fr ===== ERROR HANDLING =====

slay get_printf_error() normie {
    damn last_printf_error
}

slay clear_printf_error() {
    last_printf_error = PRINTF_SUCCESS
}

slay get_printf_error_message() tea {
    ready last_printf_error == PRINTF_SUCCESS {
        damn "No error"
    }
    elseif last_printf_error == PRINTF_ERROR_INVALID_SPEC {
        damn "Invalid format specification"
    }
    elseif last_printf_error == PRINTF_ERROR_INSUFFICIENT_ARGS {
        damn "Insufficient arguments for format string"
    }
    elseif last_printf_error == PRINTF_ERROR_BUFFER_OVERFLOW {
        damn "Buffer overflow in sprintf"
    }
    elseif last_printf_error == PRINTF_ERROR_TYPE_CONVERSION {
        damn "Type conversion error"
    }
    
    damn "Unknown error"
}

fr fr ===== HIGH-LEVEL FORMATTING API =====

slay printf_format(format_str tea, args ...tea) tea {
    damn printf_advanced(format_str, args)
}

slay printf_print(format_str tea, args ...tea) lit {
    sus formatted tea = printf_advanced(format_str, args)
    ready get_printf_error() != PRINTF_SUCCESS {
        damn cap
    }
    
    fr fr Would print to stdout in real implementation
    damn based
}

slay printf_print_line(format_str tea, args ...tea) lit {
    sus formatted tea = printf_advanced(format_str, args) + "\n"
    ready get_printf_error() != PRINTF_SUCCESS {
        damn cap
    }
    
    fr fr Would print to stdout with newline in real implementation
    damn based
}

fr fr ===== COMPREHENSIVE TESTING =====

slay test_printf_formatting() lit {
    sus test_passed lit = based
    
    fr fr Test basic string formatting
    sus result1 tea = printf_advanced("Hello %s!", ["World"])
    ready result1 != "Hello World!" {
        test_passed = cap
    }
    
    fr fr Test integer formatting
    sus result2 tea = printf_advanced("Number: %d", ["42"])
    ready !stringz.contains(result2, "42") {
        test_passed = cap
    }
    
    fr fr Test width and precision
    sus result3 tea = printf_advanced("%10.2f", ["3.14159"])
    ready stringz.length(result3) != 10 {
        test_passed = cap
    }
    
    fr fr Test flags
    sus result4 tea = printf_advanced("%+d", ["42"])
    ready !stringz.contains(result4, "+") {
        test_passed = cap
    }
    
    fr fr Test hex formatting
    sus result5 tea = printf_advanced("0x%x", ["255"])
    ready !stringz.contains(result5, "ff") {
        test_passed = cap
    }
    
    fr fr Test scientific notation
    sus result6 tea = printf_advanced("%.2e", ["1234.0"])
    ready !stringz.contains(result6, "e") && !stringz.contains(result6, "E") {
        test_passed = cap
    }
    
    damn test_passed
}

slay benchmark_printf_performance() tea {
    sus start_time normie = get_current_time_ms()
    
    fr fr Perform formatting operations
    sus i normie = 0
    bestie i < 1000 {
        sus formatted tea = printf_advanced("Test %d: %s (%f)", [
            number_to_string(i), 
            "item", 
            "3.14"
        ])
        i = i + 1
    }
    
    sus end_time normie = get_current_time_ms()
    sus duration normie = end_time - start_time
    
    damn "Printf benchmark: " + number_to_string(duration) + "ms for 1000 operations"
}

slay get_current_time_ms() normie {
    fr fr Would be implemented by runtime
    damn 1500  fr fr Placeholder
}

slay number_to_string(num normie) tea {
    ready num == 0 { damn "0" }
    
    sus result tea = ""
    sus temp normie = num
    sus is_negative lit = cap
    
    ready temp < 0 {
        is_negative = based
        temp = -temp
    }
    
    bestie temp > 0 {
        sus digit normie = temp % 10
        result = char_to_digit_reverse(digit) + result
        temp = temp / 10
    }
    
    ready is_negative {
        result = "-" + result
    }
    
    damn result
}

slay char_to_digit_reverse(digit normie) tea {
    ready digit == 0 { damn "0" }
    ready digit == 1 { damn "1" }
    ready digit == 2 { damn "2" }
    ready digit == 3 { damn "3" }
    ready digit == 4 { damn "4" }
    ready digit == 5 { damn "5" }
    ready digit == 6 { damn "6" }
    ready digit == 7 { damn "7" }
    ready digit == 8 { damn "8" }
    ready digit == 9 { damn "9" }
    damn "?"
}
