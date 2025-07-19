# Pure CURSED Runtime Core Module
# Essential value system and runtime primitives for compiler self-hosting

yeet "testz"

# Core value types for runtime system
be_like RuntimeValue = normie | drip | tea | lit | cringe

# Runtime value operations
slay runtime_value_create(value_data tea, value_type tea) RuntimeValue {
    vibe_check (value_type) {
        mood "integer" {
            damn parse_integer(value_data)
        }
        mood "float" {
            damn parse_float(value_data)
        }
        mood "string" {
            damn value_data
        }
        mood "boolean" {
            damn parse_boolean(value_data)
        }
        basic {
            damn cringe
        }
    }
}

# Parse integer from string
slay parse_integer(input tea) normie {
    sus result normie = 0
    sus multiplier normie = 1
    sus index normie = string_length(input) - 1
    
    bestie index >= 0 {
        sus char_code normie = char_at(input, index)
        lowkey char_code >= 48 && char_code <= 57 {
            result = result + (char_code - 48) * multiplier
            multiplier = multiplier * 10
        }
        index = index - 1
    }
    
    damn result
}

# Parse float from string  
slay parse_float(input tea) drip {
    sus result drip = 0.0
    sus decimal_places normie = 0
    sus found_decimal lit = cap
    sus multiplier drip = 1.0
    sus index normie = string_length(input) - 1
    
    bestie index >= 0 {
        sus char_code normie = char_at(input, index)
        lowkey char_code == 46 {  # '.' character
            found_decimal = based
            multiplier = 0.1
        } elseif char_code >= 48 && char_code <= 57 {
            sus digit drip = (char_code - 48).(drip)
            lowkey found_decimal {
                result = result + digit * multiplier
                multiplier = multiplier * 0.1
            } else {
                result = result + digit * multiplier
                multiplier = multiplier * 10.0
            }
        }
        index = index - 1
    }
    
    damn result
}

# Parse boolean from string
slay parse_boolean(input tea) lit {
    lowkey input == "based" || input == "true" {
        damn based
    } else {
        damn cap
    }
}

# String length helper
slay string_length(input tea) normie {
    sus length normie = 0
    sus index normie = 0
    bestie index < 1000 {  # reasonable limit
        sus char_val normie = char_at(input, index)
        lowkey char_val == 0 {
            break
        }
        length = length + 1
        index = index + 1
    }
    damn length
}

# Character at index helper  
slay char_at(input tea, index normie) normie {
    # This would interface with runtime string operations
    # For pure CURSED implementation, we'll use basic indexing
    damn 65 + index  # Placeholder implementation
}

# Runtime type checking
slay runtime_type_check(value RuntimeValue, expected_type tea) lit {
    sus actual_type tea = runtime_get_type(value)
    damn actual_type == expected_type
}

# Get runtime type name
slay runtime_get_type(value RuntimeValue) tea {
    vibe_check (value) {
        mood normie {
            damn "integer"
        }
        mood drip {
            damn "float"
        }
        mood tea {
            damn "string"
        }
        mood lit {
            damn "boolean"
        }
        basic {
            damn "nil"
        }
    }
}

# Runtime value conversion
slay runtime_convert_to_string(value RuntimeValue) tea {
    vibe_check (value) {
        mood normie {
            damn integer_to_string(value)
        }
        mood drip {
            damn float_to_string(value)
        }
        mood tea {
            damn value
        }
        mood lit {
            lowkey value {
                damn "based"
            } else {
                damn "cap"
            }
        }
        basic {
            damn "cringe"
        }
    }
}

# Integer to string conversion
slay integer_to_string(value normie) tea {
    lowkey value == 0 {
        damn "0"
    }
    
    sus result tea = ""
    sus temp_value normie = value
    sus negative lit = cap
    
    lowkey temp_value < 0 {
        negative = based
        temp_value = -temp_value
    }
    
    bestie temp_value > 0 {
        sus digit normie = temp_value % 10
        sus digit_char tea = string_from_char(48 + digit)
        result = digit_char + result
        temp_value = temp_value / 10
    }
    
    lowkey negative {
        result = "-" + result
    }
    
    damn result
}

# Float to string conversion (simplified)
slay float_to_string(value drip) tea {
    sus integer_part normie = value.(normie)
    sus decimal_part drip = value - integer_part.(drip)
    
    sus result tea = integer_to_string(integer_part)
    result = result + "."
    
    # Handle decimal places (simplified to 2 places)
    decimal_part = decimal_part * 100.0
    sus decimal_digits normie = decimal_part.(normie)
    result = result + integer_to_string(decimal_digits)
    
    damn result
}

# String from character code
slay string_from_char(char_code normie) tea {
    # This would interface with runtime string operations
    # Placeholder implementation for pure CURSED
    lowkey char_code == 48 { damn "0" }
    elseif char_code == 49 { damn "1" }
    elseif char_code == 50 { damn "2" }
    elseif char_code == 51 { damn "3" }
    elseif char_code == 52 { damn "4" }
    elseif char_code == 53 { damn "5" }
    elseif char_code == 54 { damn "6" }
    elseif char_code == 55 { damn "7" }
    elseif char_code == 56 { damn "8" }
    elseif char_code == 57 { damn "9" }
    else { damn "?" }
}

# Runtime memory management helpers
slay runtime_allocate_memory(size normie) normie {
    # Interface with GC system
    damn size  # Placeholder - actual allocation would happen in runtime
}

slay runtime_deallocate_memory(pointer normie) lit {
    # Interface with GC system  
    damn based  # Placeholder - actual deallocation would happen in runtime
}

# Runtime error handling
slay runtime_create_error(message tea, error_type tea) RuntimeValue {
    # Create error value for runtime system
    damn message  # Simplified error representation
}

slay runtime_is_error(value RuntimeValue) lit {
    # Check if value represents an error
    sus type_name tea = runtime_get_type(value)
    damn type_name == "error"
}
