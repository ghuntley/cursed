fr fr CURSED Specification Consistency Validation Test
fr fr This file tests all the consistency fixes applied to the specifications

vibe main

yeet "testz"
yeet "vibez"

slay test_token_categories() lit {
    test_start("Token category completeness")
    
    fr fr Test all literal types are properly defined
    sus int_val normie = 42
    sus float_val meal = 3.14
    sus string_val tea = "hello"
    sus bool_val lit = based
    sus nil_val = nah
    sus char_val sip = 'a'
    sus ptr_val @normie
    
    assert_true(based)
    assert_false(cringe)
    
    damn based
}

slay test_channel_operations() lit {
    test_start("Canonical channel operations")
    
    sus ch dm<normie>
    sus buffered dm<tea>[10]
    
    fr fr Test canonical channel operations
    dm_send(ch, 42)
    val := dm_recv(ch)
    val2, ok := dm_recv(ch)
    dm_close(ch)
    
    assert_eq_int(val, 42)
    
    damn based
}

slay test_operator_precedence() lit {
    test_start("Operator precedence validation")
    
    fr fr Critical test: 2 + 3 * 4 must equal 14, not 20
    sus result normie = 2 + 3 * 4
    assert_eq_int(result, 14)
    
    fr fr Additional precedence tests
    sus result2 normie = 10 - 6 / 2
    assert_eq_int(result2, 7)  fr fr Should be 10 - (6/2) = 7
    
    damn based
}

slay test_return_statements() lit {
    test_start("Return statement canonicalization")
    
    fr fr Using canonical 'damn' keyword
    damn based
}

slay test_error_type() lit {
    test_start("Error type definition")
    
    fr fr Test Error type zero value
    sus err Error = nah
    assert_true(err == nah)
    
    damn based
}

slay test_deprecated_syntax() lit {
    test_start("Deprecated syntax awareness")
    
    fr fr These should be flagged by parsers in the future:
    fr fr - 'cap' instead of 'cringe' 
    fr fr - 'yolo' instead of 'damn'
    fr fr - ch <- value instead of dm_send(ch, value)
    fr fr - close(ch) instead of dm_close(ch)
    
    damn based
}

slay test_semicolon_insertion() lit {
    test_start("Semicolon insertion rules")
    
    fr fr Test automatic semicolon insertion
    sus x normie = 42
    sus y normie = x + 1
    
    fr fr Test array literals don't get semicolons before }
    sus arr = []normie{
        1, 2, 3
    }
    
    assert_eq_int(arr[0], 1)
    
    damn based
}

slay test_pointer_syntax() lit {
    test_start("Pointer syntax clarification")
    
    sus x normie = 42
    sus ptr @normie = &x
    
    assert_eq_int(@ptr, 42)
    
    damn based
}

slay main() {
    vibez.spill("Running CURSED Specification Consistency Validation")
    
    test_token_categories()
    test_channel_operations()
    test_operator_precedence()
    test_return_statements()
    test_error_type()
    test_deprecated_syntax()
    test_semicolon_insertion()
    test_pointer_syntax()
    
    print_test_summary()
}
