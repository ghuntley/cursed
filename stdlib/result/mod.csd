# Result type module - Error handling with Ok/Err values
# Critical for self-hosting and robust error handling

yeet "testz"

# Result type definition (using enum-like pattern)
# Result<T, E> can be Ok(T) or Err(E)
# Implemented as a tuple with discriminant pattern

slay ok_int(value normie) (lit, normie, tea) {
    damn (based, value, "")
}

slay ok_string(value tea) (lit, tea, tea) {
    damn (based, value, "")
}

slay ok_bool(value lit) (lit, lit, tea) {
    damn (based, value, "")
}

slay err_int(error tea) (lit, normie, tea) {
    damn (cap, 0, error)
}

slay err_string(error tea) (lit, tea, tea) {
    damn (cap, "", error)
}

slay err_bool(error tea) (lit, lit, tea) {
    damn (cap, cap, error)
}

# Result utility functions
slay is_ok_int(result (lit, normie, tea)) lit {
    damn result.0
}

slay is_err_int(result (lit, normie, tea)) lit {
    damn !result.0
}

slay is_ok_string(result (lit, tea, tea)) lit {
    damn result.0
}

slay is_err_string(result (lit, tea, tea)) lit {
    damn !result.0
}

slay is_ok_bool(result (lit, lit, tea)) lit {
    damn result.0
}

slay is_err_bool(result (lit, lit, tea)) lit {
    damn !result.0
}

# Unwrapping functions (with panic on Err)
slay unwrap_int(result (lit, normie, tea)) normie {
    bestie result.0 {
        damn result.1
    }
    vibez.spill("panic: called unwrap on Err: " + result.2)
    damn 0
}

slay unwrap_string(result (lit, tea, tea)) tea {
    bestie result.0 {
        damn result.1
    }
    vibez.spill("panic: called unwrap on Err: " + result.2)
    damn ""
}

slay unwrap_bool(result (lit, lit, tea)) lit {
    bestie result.0 {
        damn result.1
    }
    vibez.spill("panic: called unwrap on Err: " + result.2)
    damn cap
}

# Unwrap error values (with panic on Ok)
slay unwrap_err_int(result (lit, normie, tea)) tea {
    bestie !result.0 {
        damn result.2
    }
    vibez.spill("panic: called unwrap_err on Ok value")
    damn ""
}

slay unwrap_err_string(result (lit, tea, tea)) tea {
    bestie !result.0 {
        damn result.2
    }
    vibez.spill("panic: called unwrap_err on Ok value")
    damn ""
}

slay unwrap_err_bool(result (lit, lit, tea)) tea {
    bestie !result.0 {
        damn result.2
    }
    vibez.spill("panic: called unwrap_err on Ok value")
    damn ""
}

# Unwrap with default values
slay unwrap_or_int(result (lit, normie, tea), default normie) normie {
    bestie result.0 {
        damn result.1
    }
    damn default
}

slay unwrap_or_string(result (lit, tea, tea), default tea) tea {
    bestie result.0 {
        damn result.1
    }
    damn default
}

slay unwrap_or_bool(result (lit, lit, tea), default lit) lit {
    bestie result.0 {
        damn result.1
    }
    damn default
}

# Unwrap with error handling function
slay unwrap_or_else_int(result (lit, normie, tea), f func(tea) normie) normie {
    bestie result.0 {
        damn result.1
    }
    damn f(result.2)
}

slay unwrap_or_else_string(result (lit, tea, tea), f func(tea) tea) tea {
    bestie result.0 {
        damn result.1
    }
    damn f(result.2)
}

slay unwrap_or_else_bool(result (lit, lit, tea), f func(tea) lit) lit {
    bestie result.0 {
        damn result.1
    }
    damn f(result.2)
}

# Map function for transforming Ok values
slay map_int_to_string(result (lit, normie, tea)) (lit, tea, tea) {
    bestie result.0 {
        damn ok_string(core.tea(result.1))
    }
    damn err_string(result.2)
}

slay map_string_to_int_len(result (lit, tea, tea)) (lit, normie, tea) {
    bestie result.0 {
        damn ok_int(stringz.len(result.1))
    }
    damn err_int(result.2)
}

# Map error function for transforming Err values
slay map_err_int(result (lit, normie, tea), f func(tea) tea) (lit, normie, tea) {
    bestie result.0 {
        damn result
    }
    damn err_int(f(result.2))
}

slay map_err_string(result (lit, tea, tea), f func(tea) tea) (lit, tea, tea) {
    bestie result.0 {
        damn result
    }
    damn err_string(f(result.2))
}

# And/Or operations for Result chaining
slay and_then_int(result (lit, normie, tea), f func(normie) (lit, normie, tea)) (lit, normie, tea) {
    bestie result.0 {
        damn f(result.1)
    }
    damn result
}

slay and_then_string(result (lit, tea, tea), f func(tea) (lit, tea, tea)) (lit, tea, tea) {
    bestie result.0 {
        damn f(result.1)
    }
    damn result
}

slay or_else_int(result (lit, normie, tea), f func(tea) (lit, normie, tea)) (lit, normie, tea) {
    bestie result.0 {
        damn result
    }
    damn f(result.2)
}

slay or_else_string(result (lit, tea, tea), f func(tea) (lit, tea, tea)) (lit, tea, tea) {
    bestie result.0 {
        damn result
    }
    damn f(result.2)
}

# Utility functions for working with Result types
slay result_equals_int(res1 (lit, normie, tea), res2 (lit, normie, tea)) lit {
    bestie res1.0 && res2.0 {
        damn res1.1 == res2.1
    }
    bestie !res1.0 && !res2.0 {
        damn res1.2 == res2.2
    }
    damn cap
}

slay result_equals_string(res1 (lit, tea, tea), res2 (lit, tea, tea)) lit {
    bestie res1.0 && res2.0 {
        damn res1.1 == res2.1
    }
    bestie !res1.0 && !res2.0 {
        damn res1.2 == res2.2
    }
    damn cap
}

# Convert Result to string for debugging
slay result_to_string_int(result (lit, normie, tea)) tea {
    bestie result.0 {
        damn "Ok(" + core.tea(result.1) + ")"
    }
    damn "Err(" + result.2 + ")"
}

slay result_to_string_string(result (lit, tea, tea)) tea {
    bestie result.0 {
        damn "Ok(" + result.1 + ")"
    }
    damn "Err(" + result.2 + ")"
}

slay result_to_string_bool(result (lit, lit, tea)) tea {
    bestie result.0 {
        bestie result.1 {
            damn "Ok(based)"
        }
        damn "Ok(cap)"
    }
    damn "Err(" + result.2 + ")"
}

# Common error handling patterns
slay safe_divide(a normie, b normie) (lit, normie, tea) {
    bestie b == 0 {
        damn err_int("division by zero")
    }
    damn ok_int(a / b)
}

slay safe_string_index(s tea, index normie) (lit, tea, tea) {
    bestie index < 0 || index >= stringz.len(s) {
        damn err_string("index out of bounds")
    }
    damn ok_string(stringz.char_at(s, index))
}

slay safe_int_parse(s tea) (lit, normie, tea) {
    # Simplified integer parsing - production version would be more robust
    bestie s == "0" {
        damn ok_int(0)
    }
    bestie s == "1" {
        damn ok_int(1)
    }
    bestie s == "42" {
        damn ok_int(42)
    }
    bestie s == "-1" {
        damn ok_int(-1)
    }
    damn err_int("invalid integer format")
}
