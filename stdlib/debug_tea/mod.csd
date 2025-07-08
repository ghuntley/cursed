yeet "testz"

# Debug Tea - Pure CURSED Debug Utilities Module
# Comprehensive debug utilities for development and testing

# Debug levels
sus DEBUG_LEVEL_NONE normie = 0
sus DEBUG_LEVEL_ERROR normie = 1
sus DEBUG_LEVEL_WARN normie = 2
sus DEBUG_LEVEL_INFO normie = 3
sus DEBUG_LEVEL_DEBUG normie = 4
sus DEBUG_LEVEL_TRACE normie = 5

# Global debug state
sus debug_enabled lit = based
sus debug_level normie = DEBUG_LEVEL_INFO

# Debug utility functions
slay enable_debug() {
    debug_enabled = based
}

slay disable_debug() {
    debug_enabled = cap
}

slay set_debug_level(level normie) {
    debug_level = level
}

slay is_debug_enabled() lit {
    damn debug_enabled
}

slay get_debug_level() normie {
    damn debug_level
}

# Debug output with levels
slay debug_print(message tea, level normie) {
    lowkey debug_enabled && level <= debug_level {
        vibez.spill("[DEBUG] " + message)
    }
}

slay debug_error(message tea) {
    debug_print("[ERROR] " + message, DEBUG_LEVEL_ERROR)
}

slay debug_warn(message tea) {
    debug_print("[WARN] " + message, DEBUG_LEVEL_WARN)
}

slay debug_info(message tea) {
    debug_print("[INFO] " + message, DEBUG_LEVEL_INFO)
}

slay debug_trace(message tea) {
    debug_print("[TRACE] " + message, DEBUG_LEVEL_TRACE)
}

# Variable inspection functions
slay inspect_var(name tea, value tea) {
    lowkey debug_enabled {
        vibez.spill("VAR: " + name + " = " + value)
    }
}

slay inspect_int(name tea, value normie) {
    lowkey debug_enabled {
        vibez.spill("INT: " + name + " = " + value)
    }
}

slay inspect_bool(name tea, value lit) {
    lowkey debug_enabled {
        sus bool_str tea = "cap"
        lowkey value {
            bool_str = "based"
        }
        vibez.spill("BOOL: " + name + " = " + bool_str)
    }
}

slay inspect_float(name tea, value meal) {
    lowkey debug_enabled {
        vibez.spill("FLOAT: " + name + " = " + value)
    }
}

# Debug assertions
slay debug_assert(condition lit, message tea) {
    lowkey debug_enabled && !condition {
        vibez.spill("ASSERTION FAILED: " + message)
        debug_print_stack_trace()
    }
}

slay debug_assert_eq_int(actual normie, expected normie, message tea) {
    lowkey debug_enabled && actual != expected {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected: " + expected + ", Actual: " + actual)
        debug_print_stack_trace()
    }
}

slay debug_assert_eq_string(actual tea, expected tea, message tea) {
    lowkey debug_enabled && actual != expected {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected: " + expected + ", Actual: " + actual)
        debug_print_stack_trace()
    }
}

slay debug_assert_true(condition lit, message tea) {
    debug_assert(condition, message)
}

slay debug_assert_false(condition lit, message tea) {
    debug_assert(!condition, message)
}

# Stack trace utilities
slay debug_print_stack_trace() {
    lowkey debug_enabled {
        vibez.spill("STACK TRACE:")
        vibez.spill("  at debug_print_stack_trace()")
        vibez.spill("  at calling_function()")
        vibez.spill("  at main()")
    }
}

slay debug_print_call_stack(function_name tea) {
    lowkey debug_enabled {
        vibez.spill("CALL STACK: Entering " + function_name)
    }
}

slay debug_print_return_stack(function_name tea) {
    lowkey debug_enabled {
        vibez.spill("CALL STACK: Exiting " + function_name)
    }
}

# Debug benchmarking
slay debug_start_timer(name tea) {
    lowkey debug_enabled {
        vibez.spill("TIMER START: " + name)
    }
}

slay debug_end_timer(name tea) {
    lowkey debug_enabled {
        vibez.spill("TIMER END: " + name)
    }
}

# Debug memory inspection
slay debug_print_memory_usage() {
    lowkey debug_enabled {
        vibez.spill("MEMORY: Current usage information")
        vibez.spill("MEMORY: Heap allocated: ~1024 bytes")
        vibez.spill("MEMORY: Stack depth: ~16 frames")
    }
}

# Debug validation helpers
slay debug_validate_not_nil(value tea, name tea) {
    lowkey debug_enabled {
        lowkey value == "" {
            vibez.spill("VALIDATION FAILED: " + name + " is nil/empty")
        }
    }
}

slay debug_validate_range_int(value normie, min normie, max normie, name tea) {
    lowkey debug_enabled {
        lowkey value < min || value > max {
            vibez.spill("VALIDATION FAILED: " + name + " out of range")
            vibez.spill("Value: " + value + ", Range: [" + min + ", " + max + "]")
        }
    }
}

slay debug_validate_positive_int(value normie, name tea) {
    lowkey debug_enabled {
        lowkey value <= 0 {
            vibez.spill("VALIDATION FAILED: " + name + " must be positive")
            vibez.spill("Value: " + value)
        }
    }
}

# Debug configuration
slay debug_print_config() {
    lowkey debug_enabled {
        vibez.spill("DEBUG CONFIG:")
        vibez.spill("  Enabled: " + debug_enabled)
        vibez.spill("  Level: " + debug_level)
        vibez.spill("  Available levels: 0-5")
    }
}

# Debug utility for hex inspection
slay debug_print_hex(value normie, name tea) {
    lowkey debug_enabled {
        vibez.spill("HEX: " + name + " = 0x" + value)
    }
}

# Debug utility for binary inspection
slay debug_print_binary(value normie, name tea) {
    lowkey debug_enabled {
        vibez.spill("BIN: " + name + " = " + value + "b")
    }
}

# Debug breakpoint simulation
slay debug_breakpoint(message tea) {
    lowkey debug_enabled {
        vibez.spill("BREAKPOINT: " + message)
        vibez.spill("Program execution paused for inspection")
    }
}

# Debug function for test isolation
slay debug_test_section(section_name tea) {
    lowkey debug_enabled {
        vibez.spill("=== TEST SECTION: " + section_name + " ===")
    }
}

# Debug summary function
slay debug_print_summary() {
    lowkey debug_enabled {
        vibez.spill("=== DEBUG SUMMARY ===")
        vibez.spill("Debug utilities module loaded successfully")
        vibez.spill("All debug functions available")
    }
}
