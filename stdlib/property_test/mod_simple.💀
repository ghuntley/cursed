yeet "testz"

fr fr Simple Property-based testing framework for CURSED
fr fr Basic implementation with core functionality

fr fr Test configuration
sus property_test_count normie = 10
sus rng_state normie = 42

fr fr ===== RANDOM NUMBER GENERATION =====

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

fr fr ===== BASIC GENERATORS =====

slay gen_int(min_val normie, max_val normie) normie {
    damn rand_range(min_val, max_val)
}

slay gen_positive_int() normie {
    damn gen_int(1, 100)
}

slay gen_boolean() lit {
    sus val normie = rand_next() % 2
    damn val == 1
}

slay gen_small_string() tea {
    sus length normie = gen_int(1, 5)
    sus result tea = ""
    sus i normie = 0
    bestie i < length {
        result = result + "A"
        i = i + 1
    }
    damn result
}

fr fr ===== PROPERTY EXECUTION =====

slay property_holds(input_value normie, property_description tea) lit { fr fr Simple property: number should equal itself
    damn input_value == input_value
}

slay run_simple_property_test(description tea) lit {
    test_start("Property: " + description)
    
    sus test_count normie = 0
    sus passed_count normie = 0
    
    bestie test_count < property_test_count {
        sus generated_input normie = gen_int(-100, 100)
        
        vibes property_holds(generated_input, description) {
            passed_count = passed_count + 1
        } nah {
            vibez.spill("Property failed with input: " + tea(generated_input))
            assert_true(cap)
            damn cap
        }
        
        test_count = test_count + 1
    }
    
    vibez.spill("Property passed " + tea(passed_count) + "/" + tea(property_test_count) + " tests")
    assert_true(passed_count == property_test_count)
    damn based
}

fr fr ===== MATHEMATICAL PROPERTIES =====

slay test_addition_commutative() lit {
    test_start("Addition is commutative")
    
    sus test_count normie = 0
    sus passed_count normie = 0
    
    bestie test_count < property_test_count {
        sus a normie = gen_int(-50, 50)
        sus b normie = gen_int(-50, 50)
        
        vibes (a + b) == (b + a) {
            passed_count = passed_count + 1
        } nah {
            vibez.spill("Commutativity failed: " + tea(a) + " + " + tea(b))
            assert_true(cap)
            damn cap
        }
        
        test_count = test_count + 1
    }
    
    vibez.spill("Commutativity passed " + tea(passed_count) + "/" + tea(property_test_count) + " tests")
    assert_true(passed_count == property_test_count)
    damn based
}

slay test_identity_property() lit {
    test_start("Addition identity property")
    
    sus test_count normie = 0
    sus passed_count normie = 0
    
    bestie test_count < property_test_count {
        sus x normie = gen_int(-100, 100)
        
        vibes (x + 0) == x {
            passed_count = passed_count + 1
        } nah {
            vibez.spill("Identity failed for: " + tea(x))
            assert_true(cap)
            damn cap
        }
        
        test_count = test_count + 1
    }
    
    vibez.spill("Identity passed " + tea(passed_count) + "/" + tea(property_test_count) + " tests")
    assert_true(passed_count == property_test_count)
    damn based
}

fr fr ===== CONFIGURATION =====

slay set_test_count(count normie) {
    property_test_count = count
}

slay set_seed(seed normie) {
    rand_seed(seed)
}
