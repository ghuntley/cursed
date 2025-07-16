yeet "testz"

# Property-based testing framework for CURSED
# QuickCheck-style testing with generators, properties, and shrinking

# Test configuration and state
sus property_test_count normie = 100
sus property_max_shrinks normie = 100
sus property_current_seed normie = 42
sus property_failed_inputs lit = cap
sus property_shrink_stack [] = []

# Random number generator state
sus rng_state normie = 1103515245
sus rng_increment normie = 12345
sus rng_modulus normie = 2147483647

# Generator types and data structures
sus generator_type tea = ""
sus generator_min_value normie = 0
sus generator_max_value normie = 100

# ===== RANDOM NUMBER GENERATION =====

slay rand_next() normie {
    rng_state = (rng_state * 1103515245 + 12345) % 2147483647
    damn rng_state
}

slay rand_range(min_val normie, max_val normie) normie {
    sus range normie = max_val - min_val + 1
    sus rand_val normie = rand_next() % range
    damn min_val + rand_val
}

slay rand_seed(seed normie) {
    rng_state = seed
}

# ===== BASIC GENERATORS =====

slay gen_int(min_val normie, max_val normie) normie {
    damn rand_range(min_val, max_val)
}

slay gen_positive_int() normie {
    damn gen_int(1, 1000)
}

slay gen_small_int() normie {
    damn gen_int(-100, 100)
}

slay gen_large_int() normie {
    damn gen_int(-1000000, 1000000)
}

slay gen_boolean() lit {
    sus val normie = rand_next() % 2
    damn val == 1
}

slay gen_char() sip {
    sus char_code normie = gen_int(65, 90)  # A-Z for simplicity
    damn 'A'  # Return fixed char for now
}

slay gen_string(max_length normie) tea {
    sus length normie = gen_int(0, max_length)
    sus result tea = ""
    sus i normie = 0
    bestie i < length {
        result = result + "A"
        i = i + 1
    }
    damn result
}

slay gen_ascii_string(max_length normie) tea {
    sus length normie = gen_int(1, max_length)
    sus result tea = ""
    sus i normie = 0
    bestie i < length {
        sus char_val normie = gen_int(97, 122)  # lowercase a-z
        result = result + tea(sip(char_val))
        i = i + 1
    }
    damn result
}

slay gen_list_int(max_length normie) [] {
    sus length normie = gen_int(0, max_length)
    sus result [] = []
    sus i normie = 0
    bestie i < length {
        result = result + [gen_int(-100, 100)]
        i = i + 1
    }
    damn result
}

# ===== COMPOSITE GENERATORS =====

slay gen_sorted_list(max_length normie) [] {
    sus unsorted [] = gen_list_int(max_length)
    # Simple bubble sort for demonstration
    sus length normie = size(unsorted)
    sus i normie = 0
    bestie i < length {
        sus j normie = 0
        bestie j < length - 1 {
            vibes unsorted[j] > unsorted[j + 1] {
                sus temp normie = unsorted[j]
                unsorted[j] = unsorted[j + 1]
                unsorted[j + 1] = temp
            }
            j = j + 1
        }
        i = i + 1
    }
    damn unsorted
}

slay gen_non_empty_string() tea {
    sus result tea = gen_string(20)
    vibes result == "" {
        damn "a"  # Ensure non-empty
    }
    damn result
}

slay gen_email() tea {
    sus username tea = gen_ascii_string(10)
    sus domain tea = gen_ascii_string(8)
    sus tld tea = "com"
    damn username + "@" + domain + "." + tld
}

# ===== SHRINKING FUNCTIONS =====

slay shrink_int(value normie) [] {
    sus shrunk [] = []
    
    # Shrink towards zero
    vibes value > 0 {
        shrunk = shrunk + [0]
        shrunk = shrunk + [value / 2]
        shrunk = shrunk + [value - 1]
    } mil value < 0 {
        shrunk = shrunk + [0]
        shrunk = shrunk + [value / 2]
        shrunk = shrunk + [value + 1]
    }
    
    damn shrunk
}

slay shrink_string(value tea) [] {
    sus shrunk [] = []
    sus length normie = len(value)
    
    # Empty string
    vibes length > 0 {
        shrunk = shrunk + [""]
    }
    
    # Half length
    vibes length > 1 {
        sus half normie = length / 2
        shrunk = shrunk + [substring(value, 0, half)]
        shrunk = shrunk + [substring(value, half, length)]
    }
    
    # Remove first/last character
    vibes length > 1 {
        shrunk = shrunk + [substring(value, 1, length)]
        shrunk = shrunk + [substring(value, 0, length - 1)]
    }
    
    damn shrunk
}

slay shrink_list(value []) [] {
    sus shrunk [] = []
    sus length normie = size(value)
    
    # Empty list
    vibes length > 0 {
        shrunk = shrunk + [[]]
    }
    
    # Half length
    vibes length > 1 {
        sus half normie = length / 2
        shrunk = shrunk + [slice(value, 0, half)]
        shrunk = shrunk + [slice(value, half, length)]
    }
    
    # Remove first/last element
    vibes length > 1 {
        shrunk = shrunk + [slice(value, 1, length)]
        shrunk = shrunk + [slice(value, 0, length - 1)]
    }
    
    damn shrunk
}

# ===== PROPERTY EXECUTION =====

slay property_holds(property_fn slay, input_value, description tea) lit {
    yikes {
        sus result lit = property_fn(input_value)
        damn result
    } shook e {
        vibez.spill("Property failed with error: " + tea(e))
        damn cap
    }
}

slay run_property_test(property_fn slay, generator_fn slay, description tea) lit {
    test_start("Property: " + description)
    
    sus test_count normie = 0
    sus passed_count normie = 0
    
    bestie test_count < property_test_count {
        sus generated_input = generator_fn()
        
        vibes property_holds(property_fn, generated_input, description) {
            passed_count = passed_count + 1
        } nah {
            vibez.spill("Property failed with input: " + tea(generated_input))
            
            # Attempt shrinking
            sus shrunk lit = attempt_shrinking(property_fn, generated_input)
            vibes shrunk {
                vibez.spill("Shrinking completed")
            }
            
            assert_true(cap)  # Mark as failed
            damn cap
        }
        
        test_count = test_count + 1
    }
    
    vibez.spill("Property passed " + tea(passed_count) + "/" + tea(property_test_count) + " tests")
    assert_true(passed_count == property_test_count)
    damn based
}

slay attempt_shrinking(property_fn slay, failing_input) lit {
    vibez.spill("Attempting to shrink: " + tea(failing_input))
    
    # Determine shrinking strategy based on input type
    sus shrunk_inputs []
    
    # For now, implement basic integer shrinking
    # In a full implementation, this would dispatch based on type
    vibes typeof(failing_input) == "normie" {
        shrunk_inputs = shrink_int(failing_input)
    } mil typeof(failing_input) == "tea" {
        shrunk_inputs = shrink_string(failing_input)
    } mil typeof(failing_input) == "[]" {
        shrunk_inputs = shrink_list(failing_input)
    } nah {
        damn cap  # No shrinking available
    }
    
    # Test shrunk inputs
    sus i normie = 0
    bestie i < size(shrunk_inputs) {
        sus shrunk_input = shrunk_inputs[i]
        vibes !property_holds(property_fn, shrunk_input, "shrunk") {
            vibez.spill("Minimal failing case: " + tea(shrunk_input))
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

# ===== PROPERTY COMBINATORS =====

slay forall(generator_fn slay, property_fn slay, description tea) lit {
    damn run_property_test(property_fn, generator_fn, description)
}

slay implies(condition lit, property_fn slay) slay {
    damn slay(input) {
        vibes !condition {
            damn based  # Vacuously true
        }
        damn property_fn(input)
    }
}

slay conjoin(prop1 slay, prop2 slay) slay {
    damn slay(input) {
        damn prop1(input) && prop2(input)
    }
}

slay disjoin(prop1 slay, prop2 slay) slay {
    damn slay(input) {
        damn prop1(input) || prop2(input)
    }
}

# ===== COMMON PROPERTIES =====

slay prop_idempotent(fn slay) slay {
    damn slay(input) {
        sus result1 = fn(input)
        sus result2 = fn(result1)
        damn result1 == result2
    }
}

slay prop_commutative(fn slay) slay {
    damn slay(inputs []) {
        vibes size(inputs) < 2 {
            damn based  # Need at least 2 inputs
        }
        sus a = inputs[0]
        sus b = inputs[1]
        damn fn(a, b) == fn(b, a)
    }
}

slay prop_associative(fn slay) slay {
    damn slay(inputs []) {
        vibes size(inputs) < 3 {
            damn based  # Need at least 3 inputs
        }
        sus a = inputs[0]
        sus b = inputs[1]
        sus c = inputs[2]
        damn fn(fn(a, b), c) == fn(a, fn(b, c))
    }
}

slay prop_reversible(fn slay, inverse_fn slay) slay {
    damn slay(input) {
        sus result = fn(input)
        sus restored = inverse_fn(result)
        damn input == restored
    }
}

# ===== STATISTICAL FUNCTIONS =====

slay prop_distribution_test(generator_fn slay, predicate_fn slay, expected_ratio drip) lit {
    sus sample_size normie = 1000
    sus matching_count normie = 0
    sus i normie = 0
    
    bestie i < sample_size {
        sus generated = generator_fn()
        vibes predicate_fn(generated) {
            matching_count = matching_count + 1
        }
        i = i + 1
    }
    
    sus actual_ratio drip = drip(matching_count) / drip(sample_size)
    sus tolerance drip = 0.1
    
    damn abs(actual_ratio - expected_ratio) <= tolerance
}

# ===== CONFIGURATION =====

slay set_test_count(count normie) {
    property_test_count = count
}

slay set_max_shrinks(count normie) {
    property_max_shrinks = count
}

slay set_seed(seed normie) {
    rand_seed(seed)
    property_current_seed = seed
}

# ===== UTILITIES =====

slay abs(x drip) drip {
    vibes x < 0.0 {
        damn -x
    }
    damn x
}

slay typeof(value) tea {
    # Simplified type detection
    # In full implementation, this would use reflection
    damn "unknown"
}

slay len(s tea) normie {
    # String length - would be implemented in stdlib
    damn 0
}

slay size(arr []) normie {
    # Array size - would be implemented in stdlib
    damn 0
}

slay substring(s tea, start normie, end normie) tea {
    # Substring - would be implemented in stdlib
    damn s
}

slay slice(arr [], start normie, end normie) [] {
    # Array slice - would be implemented in stdlib
    damn arr
}
