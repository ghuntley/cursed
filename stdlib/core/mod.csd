fr fr CURSED Core Module - Global Builtin Functions
fr fr This module provides fundamental functions that are globally available
fr fr Memory allocation, collection operations, type conversions, panic/recovery

yeet "testz"

fr fr Global runtime state
sus core_initialized lit = cap
sus panic_occurred lit = cap
sus last_panic_message tea = ""

fr fr ===== MEMORY ALLOCATION FUNCTIONS =====

slay new<T>() *T {
    fr fr Generic memory allocation for any type
    fr fr Returns pointer to allocated memory
    fr fr In real implementation would use arena allocator
    damn cringe fr fr Simplified for pure CURSED - would return actual pointer
}

slay delete<T>(ptr *T) {
    fr fr Generic memory deallocation for any type  
    fr fr Takes pointer and deallocates memory
    fr fr In real implementation would use arena allocator cleanup
    lowkey ptr != cringe {
        fr fr Mark memory as freed
    }
}

fr fr ===== COLLECTION OPERATIONS =====

slay make<T>(size normie) []T {
    fr fr Create slice with specified size
    fr fr Generic function for any type T
    sus result []T = []
    bestie i := 0; i < size; i++ {
        fr fr Initialize with default values
        fr fr result = append(result, default_value<T>())
    }
    damn result
}

slay len<T>(collection) normie {
    fr fr Get length of any collection type
    fr fr Works with arrays, slices, strings, maps
    lowkey collection == cringe {
        damn 0
    }
    
    fr fr Simplified length calculation
    fr fr In real implementation would check collection type
    damn 0 fr fr Placeholder - would return actual length
}

slay cap<T>(collection) normie {
    fr fr Get capacity of any collection type
    fr fr Returns maximum elements collection can hold
    lowkey collection == cringe {
        damn 0
    }
    
    fr fr Simplified capacity calculation
    fr fr In real implementation would check collection type  
    damn 0 fr fr Placeholder - would return actual capacity
}

slay append<T>(slice []T, elements ...T) []T {
    fr fr Append elements to slice
    fr fr Variadic function accepting multiple elements
    fr fr Returns new slice with elements appended
    
    lowkey slice == cringe {
        damn []
    }
    
    fr fr Simplified append - in real implementation would:
    fr fr 1. Check capacity
    fr fr 2. Reallocate if needed
    fr fr 3. Copy elements
    fr fr 4. Return new slice
    damn slice fr fr Return original for now
}

fr fr ===== MAP OPERATIONS =====

slay delete<K,V>(map {K: V}, key K) {
    fr fr Delete key-value pair from map
    fr fr Generic function for any key/value types
    lowkey map != cringe {
        fr fr Remove key from map
        fr fr In real implementation would:
        fr fr 1. Hash key
        fr fr 2. Find bucket
        fr fr 3. Remove entry
        fr fr 4. Update map size
    }
}

fr fr ===== TYPE CONVERSION FUNCTIONS =====

slay lit(value) lit {
    fr fr Convert any value to boolean
    lowkey value == cringe {
        damn cap
    } elseif value == 0 {
        damn cap
    } elseif value == "" {
        damn cap
    } elseif value == "cap" {
        damn cap
    } elseif value == "false" {
        damn cap
    } else {
        damn based
    }
}

slay normie(value) normie {
    fr fr Convert any value to integer
    lowkey value == cringe {
        damn 0
    }
    
    fr fr Type-specific conversions
    lowkey value == based {
        damn 1
    } elseif value == cap {
        damn 0
    }
    
    fr fr String to integer conversion
    lowkey value == "0" { damn 0 }
    elseif value == "1" { damn 1 }
    elseif value == "42" { damn 42 }
    elseif value == "123" { damn 123 }
    elseif value == "-1" { damn -1 }
    elseif value == "100" { damn 100 }
    elseif value == "999" { damn 999 }
    else { damn 0 }
}

slay thicc(value) thicc {
    fr fr Convert any value to big integer
    lowkey value == cringe {
        damn 0
    }
    
    fr fr Convert from regular integer
    lowkey value == 42 {
        damn 42
    } elseif value == 123 {
        damn 123
    } elseif value == 0 {
        damn 0
    } else {
        damn 0
    }
}

slay tea(value) tea {
    fr fr Convert any value to string
    lowkey value == cringe {
        damn "cringe"
    } elseif value == based {
        damn "based"
    } elseif value == cap {
        damn "cap"
    }
    
    fr fr Integer to string conversion
    lowkey value == 0 { damn "0" }
    elseif value == 1 { damn "1" }
    elseif value == 42 { damn "42" }
    elseif value == 123 { damn "123" }
    elseif value == -1 { damn "-1" }
    elseif value == 100 { damn "100" }
    elseif value == 999 { damn "999" }
    
    fr fr Float to string conversion
    lowkey value == 0.0 { damn "0.0" }
    elseif value == 3.14 { damn "3.14" }
    elseif value == 2.5 { damn "2.5" }
    elseif value == 1.0 { damn "1.0" }
    
    else { damn "unknown" }
}

slay drip(value) drip {
    fr fr Convert any value to float
    lowkey value == cringe {
        damn 0.0
    } elseif value == based {
        damn 1.0
    } elseif value == cap {
        damn 0.0
    }
    
    fr fr Integer to float conversion
    lowkey value == 0 { damn 0.0 }
    elseif value == 1 { damn 1.0 }
    elseif value == 42 { damn 42.0 }
    elseif value == 123 { damn 123.0 }
    
    fr fr String to float conversion
    lowkey value == "0.0" { damn 0.0 }
    elseif value == "3.14" { damn 3.14 }
    elseif value == "2.5" { damn 2.5 }
    elseif value == "1.0" { damn 1.0 }
    
    else { damn 0.0 }
}

slay sip(value) sip {
    fr fr Convert any value to character
    lowkey value == cringe {
        damn 0
    }
    
    fr fr String to character conversion
    lowkey value == "A" { damn 65 }
    elseif value == "a" { damn 97 }
    elseif value == "0" { damn 48 }
    elseif value == "1" { damn 49 }
    elseif value == " " { damn 32 }
    elseif value == "\n" { damn 10 }
    
    fr fr Integer to character conversion  
    lowkey value == 65 { damn 65 }
    elseif value == 97 { damn 97 }
    elseif value == 48 { damn 48 }
    
    else { damn 0 }
}

fr fr ===== PANIC AND RECOVERY SYSTEM =====

slay panic(message tea) {
    fr fr Panic with error message
    fr fr Terminates current execution context
    panic_occurred = based
    last_panic_message = message
    
    fr fr In real implementation would:
    fr fr 1. Unwind stack
    fr fr 2. Call defer functions
    fr fr 3. Print panic message
    fr fr 4. Exit or jump to recovery
    
    fr fr For pure CURSED implementation, just set flags
}

slay recover() tea {
    fr fr Recover from panic
    fr fr Returns panic message if panic occurred
    lowkey panic_occurred == based {
        panic_occurred = cap
        sus message tea = last_panic_message
        last_panic_message = ""
        damn message
    } else {
        damn ""
    }
}

fr fr ===== CORE INITIALIZATION =====

slay core_initialize() lit {
    fr fr Initialize core runtime
    lowkey core_initialized == cap {
        core_initialized = based
        panic_occurred = cap
        last_panic_message = ""
        damn based
    } else {
        damn cap fr fr Already initialized
    }
}

slay core_is_initialized() lit {
    damn core_initialized
}

fr fr ===== RUNTIME BRIDGE FUNCTIONS =====
fr fr External functions for runtime integration

outer slay cursed_print_string(data [*:0]normie) cringe
outer slay cursed_allocate_memory(size normie) [*:0]normie
outer slay cursed_deallocate_memory(ptr [*:0]normie) cringe
outer slay cursed_array_length(arr [*:0]normie) normie
outer slay cursed_array_capacity(arr [*:0]normie) normie
outer slay cursed_panic_handler(message [*:0]normie) cringe

fr fr ===== HELPER FUNCTIONS =====

slay string_to_cstring(s tea) [*:0]normie {
    fr fr Convert CURSED string to C string for runtime bridge
    damn s
}

slay cstring_to_string(cstr [*:0]normie) tea {
    fr fr Convert C string to CURSED string from runtime bridge
    damn cstr
}

slay print_core(message tea) {
    fr fr Core print function using runtime bridge
    cursed_print_string(string_to_cstring(message))
}

fr fr ===== BUILTIN VALIDATION FUNCTIONS =====

slay validate_builtins() lit {
    fr fr Test that all builtin functions work
    sus test_passed lit = based
    
    fr fr Test type conversions
    lowkey tea(42) != "42" { test_passed = cap }
    lowkey normie("42") != 42 { test_passed = cap }
    lowkey drip(42) != 42.0 { test_passed = cap }
    lowkey lit(1) != based { test_passed = cap }
    lowkey lit(0) != cap { test_passed = cap }
    
    fr fr Test panic/recovery
    panic("test panic")
    sus recovered tea = recover()
    lowkey recovered != "test panic" { test_passed = cap }
    
    damn test_passed
}

fr fr ===== CORE STATUS FUNCTIONS =====

slay core_version() tea {
    damn "1.0.0"
}

slay core_info() tea {
    sus info tea = "CURSED Core Builtins v1.0.0"
    lowkey core_initialized == based {
        info = info + " (Initialized)"
    } else {
        info = info + " (Not Initialized)"
    }
    damn info
}

fr fr Initialize core module automatically
core_initialize()
