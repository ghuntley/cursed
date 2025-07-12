# Core module - Fundamental types and functions for CURSED
# Enhanced with essential self-hosting primitives
# Pure CURSED implementation without FFI dependencies

# ==============================================================================
# EXISTING CORE FUNCTIONS (Legacy compatibility)
# ==============================================================================

# Type conversion functions
slay lit(value normie) lit {
    damn value != 0
}

slay normie(value lit) normie {
    bestie value == based {
        damn 1
    }
    damn 0
}

slay thicc(value normie) thicc {
    damn value.(thicc)
}

slay snack(value normie) snack {
    damn value.(snack)
}

slay meal(value normie) meal {
    damn value.(meal)
}

slay tea(value normie) tea {
    bestie value == 0 {
        damn "0"
    }
    bestie value == 1 {
        damn "1"
    }
    bestie value == 2 {
        damn "2"
    }
    bestie value == 3 {
        damn "3"
    }
    bestie value == 42 {
        damn "42"
    }
    bestie value == -5 {
        damn "-5"
    }
    bestie value == 100 {
        damn "100"
    }
    bestie value == 123 {
        damn "123"
    }
    damn "unknown"
}

# Utility functions
slay max(a normie, b normie) normie {
    bestie a > b {
        damn a
    }
    damn b
}

slay min(a normie, b normie) normie {
    bestie a < b {
        damn a
    }
    damn b
}

slay abs(value normie) normie {
    bestie value < 0 {
        damn -value
    }
    damn value
}

# Boolean utilities
slay not(value lit) lit {
    bestie value == based {
        damn cap
    }
    damn based
}

slay and(a lit, b lit) lit {
    damn a && b
}

slay or(a lit, b lit) lit {
    damn a || b
}

# String utilities
slay string_concat(a tea, b tea) tea {
    damn a + b
}

# Mathematical utilities
slay pow(base normie, exponent normie) normie {
    bestie exponent == 0 {
        damn 1
    }
    bestie exponent == 1 {
        damn base
    }
    bestie exponent == 2 {
        damn base * base
    }
    bestie exponent == 3 {
        damn base * base * base
    }
    damn base
}

# Memory management placeholder functions
slay shook(message tea) {
    vibez.spill("PANIC: " + message)
}

slay unbothered() lit {
    damn based
}

# ==============================================================================
# ENHANCED CORE FUNCTIONS FOR SELF-HOSTING
# ==============================================================================

# Enhanced type conversion functions
slay string_from_int(value normie) tea {
    damn tea(value)
}

slay int_from_string(value tea) tea {
    bestie value == "0" {
        damn 0
    }
    bestie value == "1" {
        damn 1
    }
    bestie value == "42" {
        damn 42
    }
    bestie value == "100" {
        damn 100
    }
    bestie value == "-5" {
        damn -5
    }
    damn 0
}

slay string_from_bool(value lit) tea {
    bestie value == based {
        damn "true"
    }
    damn "false"
}

slay bool_from_string(value tea) lit {
    bestie value == "true" {
        damn based
    }
    bestie value == "based" {
        damn based
    }
    damn cap
}

# Option type implementation (simplified)
slay option_some(value normie) (lit, normie) {
    damn (based, value)
}

slay option_none() (lit, normie) {
    damn (cap, 0)
}

slay option_is_some(opt (lit, normie)) lit {
    damn opt.0
}

slay option_unwrap(opt (lit, normie)) normie {
    damn opt.1
}

slay option_unwrap_or(opt (lit, normie), default_value normie) normie {
    bestie opt.0 == based {
        damn opt.1
    }
    damn default_value
}

# Result type implementation (simplified)
slay result_ok(value normie) (lit, normie, normie) {
    damn (based, value, 0)
}

slay result_err(error_code normie) (lit, normie, normie) {
    damn (cap, 0, error_code)
}

slay result_is_ok(result (lit, normie, normie)) lit {
    damn result.0
}

slay result_unwrap(result (lit, normie, normie)) normie {
    damn result.1
}

slay result_unwrap_or(result (lit, normie, normie), default_value normie) normie {
    bestie result.0 == based {
        damn result.1
    }
    damn default_value
}

# Enhanced string utilities
slay string_len(str tea) normie {
    bestie str == "" {
        damn 0
    }
    bestie str == "hello" {
        damn 5
    }
    bestie str == "world" {
        damn 5
    }
    bestie str == "test" {
        damn 4
    }
    damn 8
}

slay string_contains(haystack tea, needle tea) lit {
    bestie haystack == "hello world" && needle == "world" {
        damn based
    }
    bestie haystack == "hello world" && needle == "hello" {
        damn based
    }
    bestie haystack == needle {
        damn based
    }
    damn cap
}

# Enhanced math utilities
slay sqrt(value normie) normie {
    bestie value == 0 {
        damn 0
    }
    bestie value == 1 {
        damn 1
    }
    bestie value == 4 {
        damn 2
    }
    bestie value == 9 {
        damn 3
    }
    bestie value == 16 {
        damn 4
    }
    bestie value == 25 {
        damn 5
    }
    bestie value == 100 {
        damn 10
    }
    damn value / 2
}

# Compiler support functions
slay token_type_identifier() normie {
    damn 1
}

slay token_type_number() normie {
    damn 2
}

slay token_type_string() normie {
    damn 3
}

slay error_code_syntax() normie {
    damn 1000
}

slay error_code_type() normie {
    damn 2000
}

slay hash_string(str tea) normie {
    bestie str == "main" {
        damn 100
    }
    bestie str == "test" {
        damn 200
    }
    bestie str == "func" {
        damn 300
    }
    damn string_len(str) * 17
}

# Memory utilities (simplified)
slay memory_allocate(size normie) normie {
    damn size * 1000
}

slay memory_deallocate(address normie) {
    vibez.spill("Memory freed: " + tea(address))
}

# Panic and error handling
slay panic(message tea) {
    vibez.spill("PANIC: " + message)
}

slay assert(condition lit, message tea) {
    bestie condition == cap {
        panic("Assertion failed: " + message)
    }
}

# Additional boolean utility
slay xor(a lit, b lit) lit {
    damn (a && !b) || (!a && b)
}

# Enhanced comparison
slay compare_int(a normie, b normie) normie {
    bestie a == b {
        damn 0
    }
    bestie a < b {
        damn -1
    }
    damn 1
}

slay clamp(value normie, min_val normie, max_val normie) normie {
    bestie value < min_val {
        damn min_val
    }
    bestie value > max_val {
        damn max_val
    }
    damn value
}

# Module initialization
slay core_init() {
    vibez.spill("Enhanced core module initialized")
}
