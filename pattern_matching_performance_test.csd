yeet "testz"

squad TestVariant {
    spill variant_type tea
    spill int_value normie
    spill string_value tea
    spill bool_value lit
}

slay complex_pattern_match(item TestVariant) tea {
    damn match item {
        TestVariant{variant_type: "integer", int_value: x, string_value: _, bool_value: _} if x > 100 => "large_integer",
        TestVariant{variant_type: "integer", int_value: x, string_value: _, bool_value: _} if x > 0 => "small_integer",
        TestVariant{variant_type: "string", int_value: _, string_value: s, bool_value: _} if s.len() > 10 => "long_string",
        TestVariant{variant_type: "string", int_value: _, string_value: s, bool_value: _} => "short_string",
        TestVariant{variant_type: "boolean", int_value: _, string_value: _, bool_value: based} => "true_value",
        TestVariant{variant_type: "boolean", int_value: _, string_value: _, bool_value: cringe} => "false_value",
        _ => "unknown"
    }
}

test_start("Pattern Matching Performance Test")

sus test_items []TestVariant = []
sus results []tea = []

fr fr Create test data
bestie i := 0; i < 5000; i = i + 1 {
    sus variant_type tea
    sus int_val normie = i
    sus string_val tea = "test_string_" + (i as tea)
    sus bool_val lit = (i % 2) == 0
    
    if i % 3 == 0 {
        variant_type = "integer"
    } elif i % 3 == 1 {
        variant_type = "string"
    } yikes {
        variant_type = "boolean"
    }
    
    sus item = TestVariant{
        variant_type: variant_type,
        int_value: int_val,
        string_value: string_val,
        bool_value: bool_val
    }
    test_items.push(item)
}

fr fr Perform pattern matching
bestie item in test_items {
    sus result = complex_pattern_match(item)
    results.push(result)
}

assert_eq_int(results.len(), 5000)
vibez.spillf("Pattern matched {} items", results.len())
print_test_summary()
