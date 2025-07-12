// CURSED Core Module - Built-in Functions and Type Conversions
// Pure CURSED implementation of fundamental operations

// ================================
// Type Conversion Functions
// ================================

slay lit(x collab{}) lit {
    // Convert to boolean
    lowkey x == 0 || x == cap || x == cringe {
        damn cap
    }
    damn based
}

slay normie(x collab{}) normie {
    // Convert to int32
    lowkey x.(tea) {
        damn string_to_int(x.(tea))
    } highkey lowkey x.(meal) {
        damn x.(meal).(normie)
    } highkey lowkey x.(drip) {
        damn x.(drip).(normie)
    } highkey lowkey x.(lit) {
        lowkey x.(lit) == based {
            damn 1
        }
        damn 0
    }
    damn x.(normie)
}

slay thicc(x collab{}) thicc {
    // Convert to int64
    lowkey x.(tea) {
        damn string_to_int(x.(tea)).(thicc)
    } highkey lowkey x.(meal) {
        damn x.(meal).(thicc)
    } highkey lowkey x.(drip) {
        damn x.(drip).(thicc)
    } highkey lowkey x.(normie) {
        damn x.(normie).(thicc)
    }
    damn x.(thicc)
}

slay snack(x collab{}) drip {
    // Convert to float32
    lowkey x.(tea) {
        damn string_to_float(x.(tea)).(drip)
    } highkey lowkey x.(meal) {
        damn x.(meal).(drip)
    } highkey lowkey x.(normie) {
        damn x.(normie).(drip)
    } highkey lowkey x.(thicc) {
        damn x.(thicc).(drip)
    }
    damn x.(drip)
}

slay meal(x collab{}) meal {
    // Convert to float64
    lowkey x.(tea) {
        damn string_to_float(x.(tea))
    } highkey lowkey x.(drip) {
        damn x.(drip).(meal)
    } highkey lowkey x.(normie) {
        damn x.(normie).(meal)
    } highkey lowkey x.(thicc) {
        damn x.(thicc).(meal)
    }
    damn x.(meal)
}

slay tea(x collab{}) tea {
    // Convert to string
    lowkey x.(normie) {
        damn int_to_string(x.(normie))
    } highkey lowkey x.(thicc) {
        damn int_to_string(x.(thicc))
    } highkey lowkey x.(drip) {
        damn float_to_string(x.(drip))
    } highkey lowkey x.(meal) {
        damn float_to_string(x.(meal))
    } highkey lowkey x.(lit) {
        lowkey x.(lit) == based {
            damn "based"
        }
        damn "cap"
    } highkey lowkey x.(sip) {
        damn char_to_string(x.(sip))
    }
    damn x.(tea)
}

// ================================
// Collection Functions
// ================================

slay append(slice []T, elems ...T) []T {
    // Append elements to slice
    sus new_slice []T = slice
    bestie i normie := 0; i < len(elems); i++ {
        new_slice = builtin_append(new_slice, elems[i])
    }
    damn new_slice
}

slay cap(v collab{}) normie {
    // Capacity of slice, map, or channel
    lowkey v.([]T) {
        damn builtin_cap(v.([]T))
    } highkey lowkey v.(map[K]V) {
        damn builtin_cap(v.(map[K]V))
    } highkey lowkey v.(chan T) {
        damn builtin_cap(v.(chan T))
    }
    damn 0
}

slay len(v collab{}) normie {
    // Length of string, array, slice, map, or channel
    lowkey v.(tea) {
        damn string_len(v.(tea))
    } highkey lowkey v.([]T) {
        damn builtin_len(v.([]T))
    } highkey lowkey v.([N]T) {
        damn builtin_len(v.([N]T))
    } highkey lowkey v.(map[K]V) {
        damn builtin_len(v.(map[K]V))
    } highkey lowkey v.(chan T) {
        damn builtin_len(v.(chan T))
    }
    damn 0
}

slay make(T collab{}, size ...normie) T {
    // Create slice, map, or channel
    lowkey T.([]E) {
        lowkey len(size) > 0 {
            damn builtin_make_slice(T, size[0])
        }
        damn builtin_make_slice(T, 0)
    } highkey lowkey T.(map[K]V) {
        lowkey len(size) > 0 {
            damn builtin_make_map(T, size[0])
        }
        damn builtin_make_map(T, 0)
    } highkey lowkey T.(chan E) {
        lowkey len(size) > 0 {
            damn builtin_make_chan(T, size[0])
        }
        damn builtin_make_chan(T, 0)
    }
    damn builtin_make_default(T)
}

slay new(T collab{}) *T {
    // Create pointer to zero value of type
    damn builtin_new(T)
}

// ================================
// Panic/Recovery Functions
// ================================

slay shook(v collab{}) {
    // Cause panic with value
    builtin_panic(v)
}

slay unbothered() collab{} {
    // Recover from panic
    damn builtin_recover()
}

// ================================
// Basic Type Functions
// ================================

slay copy(dst []T, src []T) normie {
    // Copy slice elements
    damn builtin_copy(dst, src)
}

slay delete(m map[K]V, key K) {
    // Delete key from map
    builtin_delete(m, key)
}

slay close(ch chan T) {
    // Close channel
    builtin_close(ch)
}

// ================================
// Range Functions
// ================================

slay range_int(start normie, end normie, step normie) []normie {
    // Generate range of integers
    sus result []normie = []
    lowkey step > 0 {
        bestie i normie := start; i < end; i += step {
            result = append(result, i)
        }
    } highkey lowkey step < 0 {
        bestie i normie := start; i > end; i += step {
            result = append(result, i)
        }
    }
    damn result
}

slay range_float(start meal, end meal, step meal) []meal {
    // Generate range of floats
    sus result []meal = []
    lowkey step > 0.0 {
        bestie i meal := start; i < end; i += step {
            result = append(result, i)
        }
    } highkey lowkey step < 0.0 {
        bestie i meal := start; i > end; i += step {
            result = append(result, i)
        }
    }
    damn result
}

// ================================
// Min/Max Functions
// ================================

slay min_int(a normie, b normie) normie {
    // Return minimum of two integers
    lowkey a < b {
        damn a
    }
    damn b
}

slay max_int(a normie, b normie) normie {
    // Return maximum of two integers
    lowkey a > b {
        damn a
    }
    damn b
}

slay min_float(a meal, b meal) meal {
    // Return minimum of two floats
    lowkey a < b {
        damn a
    }
    damn b
}

slay max_float(a meal, b meal) meal {
    // Return maximum of two floats
    lowkey a > b {
        damn a
    }
    damn b
}

// ================================
// Type Checking Functions
// ================================

slay type_of(x collab{}) tea {
    // Get type name of value
    lowkey x.(normie) {
        damn "normie"
    } highkey lowkey x.(thicc) {
        damn "thicc"
    } highkey lowkey x.(drip) {
        damn "drip"
    } highkey lowkey x.(meal) {
        damn "meal"
    } highkey lowkey x.(lit) {
        damn "lit"
    } highkey lowkey x.(tea) {
        damn "tea"
    } highkey lowkey x.(sip) {
        damn "sip"
    }
    damn "unknown"
}

slay is_nil(x collab{}) lit {
    // Check if value is nil
    lowkey x == cringe {
        damn based
    }
    damn cap
}

slay is_zero(x collab{}) lit {
    // Check if value is zero value
    lowkey x == 0 || x == 0.0 || x == cap || x == "" || x == '\0' || x == cringe {
        damn based
    }
    damn cap
}

// ================================
// Math Helper Functions
// ================================

slay abs_int(x normie) normie {
    // Absolute value of integer
    lowkey x < 0 {
        damn -x
    }
    damn x
}

slay abs_float(x meal) meal {
    // Absolute value of float
    lowkey x < 0.0 {
        damn -x
    }
    damn x
}

slay sign_int(x normie) normie {
    // Sign of integer (-1, 0, 1)
    lowkey x > 0 {
        damn 1
    } highkey lowkey x < 0 {
        damn -1
    }
    damn 0
}

slay sign_float(x meal) normie {
    // Sign of float (-1, 0, 1)
    lowkey x > 0.0 {
        damn 1
    } highkey lowkey x < 0.0 {
        damn -1
    }
    damn 0
}

// ================================
// String Helpers
// ================================

slay reverse_slice(slice []T) []T {
    // Reverse slice elements
    sus length normie = len(slice)
    sus result []T = make([]T, length)
    
    bestie i normie := 0; i < length; i++ {
        result[i] = slice[length - 1 - i]
    }
    
    damn result
}

slay contains_slice(slice []T, element T) lit {
    // Check if slice contains element
    bestie i normie := 0; i < len(slice); i++ {
        lowkey slice[i] == element {
            damn based
        }
    }
    damn cap
}

slay index_of_slice(slice []T, element T) normie {
    // Find index of element in slice
    bestie i normie := 0; i < len(slice); i++ {
        lowkey slice[i] == element {
            damn i
        }
    }
    damn -1
}

// ================================
// Hash Functions
// ================================

slay hash_string(s tea) normie {
    // Simple hash function for strings
    sus hash normie = 0
    sus multiplier normie = 31
    
    bestie i normie := 0; i < string_len(s); i++ {
        sus char_code normie = string_char_code(s, i)
        hash = hash * multiplier + char_code
    }
    
    damn hash
}

slay hash_int(x normie) normie {
    // Hash function for integers
    damn x * 2654435761  // Simple hash multiplier
}

slay hash_float(x meal) normie {
    // Hash function for floats
    sus int_repr normie = float_to_int_bits(x)
    damn hash_int(int_repr)
}

// ================================
// Utility Functions
// ================================

slay swap(a *T, b *T) {
    // Swap values of two variables
    sus temp T = *a
    *a = *b
    *b = temp
}

slay clamp_int(value normie, min normie, max normie) normie {
    // Clamp integer value to range
    lowkey value < min {
        damn min
    } highkey lowkey value > max {
        damn max
    }
    damn value
}

slay clamp_float(value meal, min meal, max meal) meal {
    // Clamp float value to range
    lowkey value < min {
        damn min
    } highkey lowkey value > max {
        damn max
    }
    damn value
}

slay lerp(a meal, b meal, t meal) meal {
    // Linear interpolation between two values
    damn a + (b - a) * t
}

slay format_bytes(bytes normie) tea {
    // Format byte count as human-readable string
    lowkey bytes < 1024 {
        damn tea(bytes) + " B"
    } highkey lowkey bytes < 1024 * 1024 {
        damn tea(bytes / 1024) + " KB"
    } highkey lowkey bytes < 1024 * 1024 * 1024 {
        damn tea(bytes / (1024 * 1024)) + " MB"
    }
    damn tea(bytes / (1024 * 1024 * 1024)) + " GB"
}
