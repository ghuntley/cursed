fr fr Option type module - Optional values that can be Some or None
fr fr Critical for self-hosting and error handling

yeet "testz"

fr fr Option type definition (using enum-like pattern)
fr fr Option<T> can be Some(T) or None
fr fr Implemented as a tuple with discriminant pattern

slay some_int(value normie) (lit, normie) {
    damn (based, value)
}

slay some_string(value tea) (lit, tea) {
    damn (based, value)
}

slay some_bool(value lit) (lit, lit) {
    damn (based, value)
}

slay none_int() (lit, normie) {
    damn (cap, 0)
}

slay none_string() (lit, tea) {
    damn (cap, "")
}

slay none_bool() (lit, lit) {
    damn (cap, cap)
}

fr fr Option utility functions
slay is_some_int(option (lit, normie)) lit {
    damn option.0
}

slay is_none_int(option (lit, normie)) lit {
    damn !option.0
}

slay is_some_string(option (lit, tea)) lit {
    damn option.0
}

slay is_none_string(option (lit, tea)) lit {
    damn !option.0
}

slay is_some_bool(option (lit, lit)) lit {
    damn option.0
}

slay is_none_bool(option (lit, lit)) lit {
    damn !option.0
}

fr fr Unwrapping functions (with panic on None)
slay unwrap_int(option (lit, normie)) normie {
    bestie option.0 {
        damn option.1
    }
    vibez.spill("panic: called unwrap on None value")
    damn 0
}

slay unwrap_string(option (lit, tea)) tea {
    bestie option.0 {
        damn option.1
    }
    vibez.spill("panic: called unwrap on None value")
    damn ""
}

slay unwrap_bool(option (lit, lit)) lit {
    bestie option.0 {
        damn option.1
    }
    vibez.spill("panic: called unwrap on None value")
    damn cap
}

fr fr Unwrap with default values
slay unwrap_or_int(option (lit, normie), default normie) normie {
    bestie option.0 {
        damn option.1
    }
    damn default
}

slay unwrap_or_string(option (lit, tea), default tea) tea {
    bestie option.0 {
        damn option.1
    }
    damn default
}

slay unwrap_or_bool(option (lit, lit), default lit) lit {
    bestie option.0 {
        damn option.1
    }
    damn default
}

fr fr Map function for transforming Option values
slay map_int_to_string(option (lit, normie)) (lit, tea) {
    bestie option.0 {
        damn some_string(core.tea(option.1))
    }
    damn none_string()
}

slay map_string_to_int_len(option (lit, tea)) (lit, normie) {
    bestie option.0 {
        damn some_int(stringz.len(option.1))
    }
    damn none_int()
}

fr fr Filter function for conditional values
slay filter_int(option (lit, normie), condition func(normie) lit) (lit, normie) {
    bestie option.0 && condition(option.1) {
        damn option
    }
    damn none_int()
}

slay filter_string(option (lit, tea), condition func(tea) lit) (lit, tea) {
    bestie option.0 && condition(option.1) {
        damn option
    }
    damn none_string()
}

fr fr And/Or operations for Option chaining
slay and_then_int(option (lit, normie), f func(normie) (lit, normie)) (lit, normie) {
    bestie option.0 {
        damn f(option.1)
    }
    damn none_int()
}

slay and_then_string(option (lit, tea), f func(tea) (lit, tea)) (lit, tea) {
    bestie option.0 {
        damn f(option.1)
    }
    damn none_string()
}

slay or_else_int(option (lit, normie), f func() (lit, normie)) (lit, normie) {
    bestie option.0 {
        damn option
    }
    damn f()
}

slay or_else_string(option (lit, tea), f func() (lit, tea)) (lit, tea) {
    bestie option.0 {
        damn option
    }
    damn f()
}

fr fr Utility functions for working with Option types
slay option_equals_int(opt1 (lit, normie), opt2 (lit, normie)) lit {
    bestie opt1.0 && opt2.0 {
        damn opt1.1 == opt2.1
    }
    damn opt1.0 == opt2.0
}

slay option_equals_string(opt1 (lit, tea), opt2 (lit, tea)) lit {
    bestie opt1.0 && opt2.0 {
        damn opt1.1 == opt2.1
    }
    damn opt1.0 == opt2.0
}

fr fr Convert Option to string for debugging
slay option_to_string_int(option (lit, normie)) tea {
    bestie option.0 {
        damn "Some(" + core.tea(option.1) + ")"
    }
    damn "None"
}

slay option_to_string_string(option (lit, tea)) tea {
    bestie option.0 {
        damn "Some(" + option.1 + ")"
    }
    damn "None"
}

slay option_to_string_bool(option (lit, lit)) tea {
    bestie option.0 {
        bestie option.1 {
            damn "Some(based)"
        }
        damn "Some(cap)"
    }
    damn "None"
}
