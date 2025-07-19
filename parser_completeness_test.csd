fr fr Test comprehensive parser completeness
yeet "testz"

fr fr Function signature parsing with all advanced features
slay advanced_function<T: Clone + Debug>(
    param_one normie,
    param_two sus meal = 3.14,
    ...variadic_params
) -> (normie, tea, lit) {
    damn (42, "test", based)
}

fr fr Interface composition and inheritance
collab Displayable {
    slay show() -> tea
}

collab Serializable {
    slay serialize() -> tea  
}

collab Advanced: Displayable + Serializable {
    slay to_json() -> tea
}

fr fr Pattern matching with all pattern types
slay test_patterns(value normie) -> normie {
    match value {
        fr fr Literal patterns
        42 => 1,
        
        fr fr Range patterns  
        1..10 => 2,
        
        fr fr Tuple destructuring
        (x, y) => x + y,
        
        fr fr Array destructuring with rest
        [head, ...tail] => head,
        
        fr fr Struct destructuring
        Person { name, age } => age,
        
        fr fr Enum patterns
        Some(x) => x,
        None => 0,
        
        fr fr Guard patterns
        x when x > 100 => x * 2,
        
        fr fr Wildcard
        _ => -1
    }
}

fr fr Type switches with variable binding
slay test_type_switch(value interface{}) -> normie {
    vibe_check value {
        normie:
            damn value.(normie)
        tea:
            damn len(value.(tea))
        lit:
            damn if value.(lit) { 1 } else { 0 }
        _:
            damn -1
    }
}

fr fr Error handling with all keywords
slay test_error_handling() -> normie yikes OhNo {
    shook test_function() fam err {
        damn -1
    }
    damn 42
}

fr fr Select statement with channel operations
slay test_select(ch1 dm<normie>, ch2 dm<tea>) -> normie {
    select {
        val := <-ch1:
            damn val
        msg := <-ch2:
            damn len(msg)
        default:
            damn 0
    }
}

fr fr Generic constraints and bounds
slay generic_constrained<T, U>(
    data T
) -> U where T: Clone + Debug, U: Default {
    damn U::default()
}

test_start("parser completeness")
assert_true(advanced_function<normie>(1, 2.0) == (42, "test", based))
print_test_summary()
