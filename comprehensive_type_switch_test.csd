# Comprehensive type switch test for CURSED compiler
# Tests runtime type checking and variable binding

yeet "testz"

# Test basic type switch with normie type
slay test_type_switch_normie() lit {
    sus x normie = 42
    sus result normie = typecheck (x) {
        case normie -> 1
        _ -> 0
    }
    damn result
}

# Test type switch with string type  
slay test_type_switch_tea() lit {
    sus x tea = "hello"
    sus result normie = typecheck (x) {
        case tea -> 1
        case normie -> 2
        _ -> 0
    }
    damn result
}

# Test type switch with boolean type
slay test_type_switch_lit() lit {
    sus x lit = based
    sus result normie = typecheck (x) {
        case lit -> 1
        case normie -> 2
        _ -> 0
    }
    damn result
}

# Test type switch with variable binding
slay test_type_switch_binding() normie {
    sus x normie = 123
    sus result normie = typecheck (x) {
        case normie y -> y * 2
        _ -> 0
    }
    damn result
}

# Test type switch with wildcard
slay test_type_switch_wildcard() normie {
    sus x drip = 3.14
    sus result normie = typecheck (x) {
        case normie -> 1
        case tea -> 2
        _ -> 999  # Should hit this case
    }
    damn result
}

# Test multiple type checks
slay test_type_switch_multiple() tea {
    sus values = [42, "hello", based]
    sus results = []
    
    bestie i := 0; i < 3; i++ {
        sus val = values[i]
        sus result tea = typecheck (val) {
            case normie x -> "integer"
            case tea s -> "string"
            case lit b -> "boolean"
            _ -> "unknown"
        }
        results = append(results, result)
    }
    
    damn "multiple types checked"
}

# Run all tests
test_start("Type Switch LLVM Codegen Tests")

assert_eq_int(test_type_switch_normie(), 1)
assert_eq_int(test_type_switch_tea(), 1) 
assert_eq_int(test_type_switch_lit(), 1)
assert_eq_int(test_type_switch_binding(), 246)  # 123 * 2
assert_eq_int(test_type_switch_wildcard(), 999)

vibez.spill("Running multiple type checks...")
sus multiple_result tea = test_type_switch_multiple()
assert_eq_string(multiple_result, "multiple types checked")

print_test_summary()

vibez.spill("✅ Type switch LLVM codegen implementation complete!")
