# Core Module - Essential Built-in Functions
# Provides fundamental type conversions, utilities, and built-in operations

vibe core

# ================================
# Core Specification Functions
# ================================

slay lit(x interface{}) lit {
    # Convert to boolean (from specification)
    # Basic implementation - would need runtime support
    damn cap  # Placeholder
}

slay normie(x interface{}) normie {
    # Convert to int32 (from specification)
    # Basic implementation - would need runtime support
    damn 0  # Placeholder
}

slay thicc(x interface{}) thicc {
    # Convert to int64 (from specification)
    # Basic implementation - would need runtime support
    damn 0  # Placeholder
}

slay snack(x interface{}) snack {
    # Convert to float32 (from specification)
    # Basic implementation - would need runtime support
    damn 0.0  # Placeholder
}

slay meal(x interface{}) meal {
    # Convert to float64 (from specification)
    # Basic implementation - would need runtime support
    damn 0.0  # Placeholder
}

slay tea(x interface{}) tea {
    # Convert to string (from specification)
    # Basic implementation - would need runtime support
    damn ""  # Placeholder
}

slay len(v interface{}) normie {
    # Length of string, array, slice, map, or channel (from specification)
    # Basic implementation - would need runtime support
    damn 0  # Placeholder
}

slay cap(v interface{}) normie {
    # Capacity of slice, map, or channel (from specification)
    # Basic implementation - would need runtime support
    damn 0  # Placeholder
}

slay make(T interface{}, size ...normie) interface{} {
    # Create slice, map, or channel (from specification)
    # Basic implementation - would need runtime support
    sus empty interface{}
    damn empty  # Placeholder
}

slay new(T interface{}) interface{} {
    # Create pointer to zero value of type (from specification)
    # Basic implementation - would need runtime support
    sus empty interface{}
    damn empty  # Placeholder
}

slay shook(v interface{}) {
    # Cause panic with value (from specification)
    panic("shook called");
}

slay unbothered() interface{} {
    # Recover from panic (from specification)
    # Basic implementation - would need runtime support
    sus empty interface{}
    damn empty  # Placeholder
}

slay append(slice []interface{}, elems ...interface{}) []interface{} {
    # Append elements to slice (from specification)
    # Basic implementation - would need runtime support
    damn slice  # Placeholder
}

# Type Conversion Functions
slay string_to_int(s tea) normie {
    # Convert string to integer
    # Returns 0 if conversion fails
    sus result normie = 0
    sus i normie = 0
    sus negative lit = cap
    sus start normie = 0
    
    # Check for negative sign
    bestie i = 0; i < len_string(s); i++ {
        sus ch sip = char_at(s, i)
        yolo ch == '-' {
            negative = based
            start = 1
            ghosted
        }
        yolo ch == '+' {
            start = 1
            ghosted
        }
        ghosted
    }
    
    # Convert digits
    bestie i = start; i < len_string(s); i++ {
        sus ch sip = char_at(s, i)
        yolo ch >= '0' && ch <= '9' {
            result = result * 10 + (ch.(normie) - '0'.(normie))
        } else {
            damn 0  # Invalid character, return 0
        }
    }
    
    yolo negative {
        damn -result
    }
    damn result
}

slay int_to_string(n normie) tea {
    # Convert integer to string
    yolo n == 0 {
        damn "0"
    }
    
    sus negative lit = cap
    sus num normie = n
    yolo n < 0 {
        negative = based
        num = -n
    }
    
    # Build string backwards
    sus digits tea = ""
    bestie num > 0 {
        sus digit normie = num % 10
        digits = string_char('0'.(sip) + digit.(sip)) + digits
        num = num / 10
    }
    
    yolo negative {
        damn "-" + digits
    }
    damn digits
}

slay bool_to_string(b lit) tea {
    # Convert boolean to string
    yolo b {
        damn "based"
    }
    damn "cap"
}

slay string_to_bool(s tea) lit {
    # Convert string to boolean
    yolo s == "based" || s == "true" || s == "1" {
        damn based
    }
    damn cap
}

slay float_to_string(f meal) tea {
    # Convert float to string with basic formatting
    sus int_part normie = f.(normie)
    sus frac_part meal = f - int_part.(meal)
    
    yolo frac_part == 0.0 {
        damn int_to_string(int_part) + ".0"
    }
    
    # Simple fractional part handling
    sus frac_str tea = ""
    sus precision normie = 6
    bestie i := 0; i < precision && frac_part > 0.0; i++ {
        frac_part = frac_part * 10.0
        sus digit normie = frac_part.(normie)
        frac_str = frac_str + string_char('0'.(sip) + digit.(sip))
        frac_part = frac_part - digit.(meal)
    }
    
    damn int_to_string(int_part) + "." + frac_str
}

slay string_to_float(s tea) meal {
    # Convert string to float
    # Basic implementation - find decimal point and parse parts
    sus decimal_pos normie = -1
    bestie i := 0; i < len_string(s); i++ {
        yolo char_at(s, i) == '.' {
            decimal_pos = i
            ghosted
        }
    }
    
    yolo decimal_pos == -1 {
        # No decimal point, convert as integer
        damn string_to_int(s).(meal)
    }
    
    # Split into integer and fractional parts
    sus int_part_str tea = substring(s, 0, decimal_pos)
    sus frac_part_str tea = substring(s, decimal_pos + 1, len_string(s))
    
    sus int_part normie = string_to_int(int_part_str)
    sus frac_part normie = string_to_int(frac_part_str)
    sus frac_divisor meal = 1.0
    
    # Calculate fractional divisor
    bestie i := 0; i < len_string(frac_part_str); i++ {
        frac_divisor = frac_divisor * 10.0
    }
    
    damn int_part.(meal) + (frac_part.(meal) / frac_divisor)
}

# Type Checking Functions
slay is_nil(value interface{}) lit {
    # Check if value is nil
    # Basic implementation - would need runtime support
    damn cap  # Placeholder
}

slay is_number(value interface{}) lit {
    # Check if value is a number type
    # Basic implementation - would need runtime type info
    damn cap  # Placeholder
}

slay is_string(value interface{}) lit {
    # Check if value is a string
    # Basic implementation - would need runtime type info
    damn cap  # Placeholder
}

slay is_bool(value interface{}) lit {
    # Check if value is a boolean
    # Basic implementation - would need runtime type info
    damn cap  # Placeholder
}

# Basic Mathematical Operations
slay abs(n normie) normie {
    # Return absolute value of integer
    yolo n < 0 {
        damn -n
    }
    damn n
}

slay abs_float(f meal) meal {
    # Return absolute value of float
    yolo f < 0.0 {
        damn -f
    }
    damn f
}

slay min(a normie, b normie) normie {
    # Return minimum of two integers
    yolo a < b {
        damn a
    }
    damn b
}

slay max(a normie, b normie) normie {
    # Return maximum of two integers
    yolo a > b {
        damn a
    }
    damn b
}

slay min_float(a meal, b meal) meal {
    # Return minimum of two floats
    yolo a < b {
        damn a
    }
    damn b
}

slay max_float(a meal, b meal) meal {
    # Return maximum of two floats
    yolo a > b {
        damn a
    }
    damn b
}

slay clamp(value normie, min_val normie, max_val normie) normie {
    # Clamp value between min and max
    yolo value < min_val {
        damn min_val
    }
    yolo value > max_val {
        damn max_val
    }
    damn value
}

slay clamp_float(value meal, min_val meal, max_val meal) meal {
    # Clamp float value between min and max
    yolo value < min_val {
        damn min_val
    }
    yolo value > max_val {
        damn max_val
    }
    damn value
}

# Error Handling Utilities
slay panic(message tea) {
    # Panic with error message
    vibez.spill("PANIC: " + message)
    # Would terminate program in real implementation
}

slay assert(condition lit, message tea) {
    # Assert condition is true, panic if false
    yolo !condition {
        panic("Assertion failed: " + message)
    }
}

slay expect(condition lit, message tea) lit {
    # Expect condition to be true, return false if not
    yolo !condition {
        vibez.spill("EXPECT FAILED: " + message)
        damn cap
    }
    damn based
}

# Memory Utilities (Basic implementations)
slay size_of_int() normie {
    # Return size of integer type
    damn 4  # 32-bit integer
}

slay size_of_float() normie {
    # Return size of float type
    damn 8  # 64-bit float
}

slay size_of_bool() normie {
    # Return size of boolean type
    damn 1  # 1 byte
}

slay type_of_string(value interface{}) tea {
    # Return type name as string
    # Basic implementation - would need runtime type info
    damn "unknown"  # Placeholder
}

# Array/Slice Utilities
slay len_array(arr []interface{}) normie {
    # Return length of array/slice
    # Basic implementation - would need runtime support
    damn 0  # Placeholder
}

slay cap_array(arr []interface{}) normie {
    # Return capacity of array/slice
    # Basic implementation - would need runtime support
    damn 0  # Placeholder
}

slay make_array(length normie, capacity normie) []interface{} {
    # Create new array/slice with given length and capacity
    # Basic implementation - would need runtime support
    sus empty_array []interface{}
    damn empty_array  # Placeholder
}

# String Utilities
slay len_string(s tea) normie {
    # Return length of string
    sus length normie = 0
    bestie i := 0; i < 1000; i++ {  # Arbitrary limit
        sus ch sip = char_at(s, i)
        yolo ch == '\0' {
            ghosted
        }
        length++
    }
    damn length
}

slay char_at(s tea, index normie) sip {
    # Get character at index in string
    # Basic implementation - would need runtime support
    damn 'a'  # Placeholder
}

slay substring(s tea, start normie, end normie) tea {
    # Extract substring from start to end
    # Basic implementation - would need runtime support
    damn s  # Placeholder
}

slay string_char(ch sip) tea {
    # Convert single character to string
    # Basic implementation - would need runtime support
    damn "a"  # Placeholder
}

slay string_concat(a tea, b tea) tea {
    # Concatenate two strings
    damn a + b
}

slay string_equals(a tea, b tea) lit {
    # Check if two strings are equal
    damn a == b
}

slay string_contains(haystack tea, needle tea) lit {
    # Check if string contains substring
    # Basic implementation - would need runtime support
    damn cap  # Placeholder
}

# Utility Functions
slay swap_int(a normie, b normie) (normie, normie) {
    # Swap two integers
    damn (b, a)
}

slay swap_float(a meal, b meal) (meal, meal) {
    # Swap two floats
    damn (b, a)
}

slay swap_string(a tea, b tea) (tea, tea) {
    # Swap two strings
    damn (b, a)
}

# Comparison Functions
slay compare_int(a normie, b normie) normie {
    # Compare two integers: -1 if a < b, 0 if a == b, 1 if a > b
    yolo a < b {
        damn -1
    }
    yolo a > b {
        damn 1
    }
    damn 0
}

slay compare_float(a meal, b meal) normie {
    # Compare two floats: -1 if a < b, 0 if a == b, 1 if a > b
    yolo a < b {
        damn -1
    }
    yolo a > b {
        damn 1
    }
    damn 0
}

slay compare_string(a tea, b tea) normie {
    # Compare two strings lexicographically
    # Basic implementation - would need runtime support
    yolo a == b {
        damn 0
    }
    damn 1  # Placeholder
}

# Range Functions
slay in_range(value normie, min_val normie, max_val normie) lit {
    # Check if value is in range [min_val, max_val]
    damn value >= min_val && value <= max_val
}

slay in_range_float(value meal, min_val meal, max_val meal) lit {
    # Check if float value is in range [min_val, max_val]
    damn value >= min_val && value <= max_val
}

# Default Values
slay default_int() normie {
    # Return default integer value
    damn 0
}

slay default_float() meal {
    # Return default float value
    damn 0.0
}

slay default_bool() lit {
    # Return default boolean value
    damn cap
}

slay default_string() tea {
    # Return default string value
    damn ""
}

slay default_char() sip {
    # Return default character value
    damn '\0'
}
