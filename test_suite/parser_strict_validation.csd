# ===================================================================
# ORACLE PRIORITY 1: Parser Strict Validation Tests
# Tests designed to pass with -Zparser-strict flag
# ===================================================================

# Edge Case 1: Operator precedence with function calls
result1 := func1() + func2() * func3()      # Should parse as: func1() + (func2() * func3())
result2 := array[i] + method().call() / 2   # Should parse as: array[i] + (method().call() / 2)

# Edge Case 2: Complex member access chains
deep := obj.field.method().property.sub_method().result
nested := array[0].property.method(param).field

# Edge Case 3: Mixed indexing and member access
complex1 := array[obj.method().index].property
complex2 := obj.array_field[calculate_index()].method()

# Edge Case 4: Nested parentheses with operators
calc1 := (a + (b * (c - d))) / ((e + f) * g)
calc2 := ((func1() + func2()) * (func3() - func4())) % modulus

# Edge Case 5: Assignment operators with complex RHS
array[i] += complex_func(x, y).property * factor
obj.field *= (base + offset) / (count + 1)

# Edge Case 6: Logical operators with function calls
condition := check1() && (check2() || check3())
filter := validate(item) || (fallback_check(item) && emergency_check())

# Edge Case 7: Comparison chains with mixed types
result := (int_value > float_func()) == (string_len() < max_size)
compare := (array.len() >= min_size) && (array[0] != null_value)

# Edge Case 8: Complex ternary-like expressions (using ready/otherwise)
value := ready (condition) { expression1 } otherwise { expression2 }

# Edge Case 9: Nested function calls with operators in arguments
result := outer_func(
    inner1(a + b, c * d),
    inner2(e / f, g % h),
    inner3(array[i], obj.method())
)

# Edge Case 10: Complex array/slice operations
slice1 := array[start:end][sub_start:sub_end]
slice2 := matrix[row][column:column+width]

# Edge Case 11: Chained assignments (right-associative)
a := b := c := complex_expression(x, y, z)

# Edge Case 12: Mixed string concatenation and arithmetic
message := "Value: " + (base + offset).toString() + " of " + total.toString()

# Edge Case 13: Complex boolean expressions with short-circuiting
result := expensive_check1() && 
         (quick_check() || expensive_check2()) &&
         final_validation(data)

# Edge Case 14: Nested struct/array literals with expressions
complex_data := {
    array_field: [
        compute1(x),
        compute2(y) + offset,
        obj.method().result
    ],
    nested_struct: {
        field1: calculate(a, b),
        field2: array[complex_index()]
    }
}

# Edge Case 15: Complex type casting with expressions
casted := (complex_expression(a, b) + offset) as TargetType
converted := convert<Target>(source.method().result + adjustment)

# Edge Case 16: Error handling with complex expressions
result := risky_operation(
    param1 + adjustment,
    array[complex_index()],
    obj.method().result
) fam {
    when error -> fallback_calculation(backup_data)
}

# Edge Case 17: Complex pattern matching expressions
sick complex_expression(input.process().result) {
    mood pattern1(x) ready (x > threshold) -> handle_pattern1(x),
    mood pattern2 -> obj.method().handle(),
    basic -> default_handler(get_fallback())
}

# Edge Case 18: Goroutine with complex expressions
stan worker_function(
    channels[get_worker_id()],
    config.get_parameters(worker_type),
    data[start_index:calculate_end()]
)

# Edge Case 19: Complex select with expressions
ready {
    mood dm_send(output_channel, process_result(data)) -> {
        log_success(get_timestamp())
    },
    mood input := dm_recv(input_channels[priority]) -> {
        handle_input(input, get_context())  
    }
}

# Edge Case 20: Complex loop with expressions in all parts
bestie (
    i := calculate_start(config); 
    (i < calculate_end(data.size()) && condition_check(i)); 
    i += calculate_step(progress)
) {
    process_iteration(data[i], get_iteration_context(i))
}

# Edge Case 21: Complex defer with expressions
later cleanup_function(
    resource.get_handle(),
    calculate_cleanup_params(start_time, end_time),
    error_context.get_summary()
)

# Edge Case 22: Nested lambda expressions
mapper := slay(x) { 
    damn processor(x, slay(y) { damn y * multiplier })
}

# Edge Case 23: Complex macro calls with expressions
complex_macro!(
    process_arg(base + offset),
    array[nested_call()],
    {
        field1: compute_field1(),
        field2: nested.access().result
    }
)

# Edge Case 24: Complex generic instantiation
result := generic_function<
    ComplexType<T, U>,
    ProcessorType<compute_type_param()>
>(
    param1.method().result,
    calculate_param2(context)
)

# Edge Case 25: Complex unsafe operations
unsafe {
    ptr := get_raw_pointer(obj) as *mut ComplexType
    *ptr = ComplexType{
        field: calculate_unsafe_value(),
        data: process_raw_data(input)
    }
}

# Edge Case 26: Complex async expressions  
result := await complex_async_call(
    param1,
    await nested_async(base + offset),
    sync_calculation(data)
)

# Edge Case 27: Complex range expressions
bestie (key, value) := flex map.entries()[filter_start():filter_end()] {
    ready (complex_filter(key, value)) {
        process_entry(key, transform_value(value))
    }
}

# Edge Case 28: Complex attribute expressions
@[validate(complex_validator(min_value(), max_value()))]
@[cfg(feature = get_feature_name())]
squad ComplexStruct {
    @[transform(processor.get_transformer())]
    field: ComplexFieldType<T>
}

# Edge Case 29: Complex lifetime and constraint expressions
slay complex_function<
    'a, 'b, T: Clone + Send + CustomTrait<U>,
    U: Sync + 'static
>(
    param1: &'a ComplexType<T>,
    param2: &'b ProcessorType<U>
) -> Result<ProcessedType<T, U>, ComplexError<'a>>
where
    T: ProcessingTrait<U>,
    U: ValidationTrait + 'a
{
    damn process_complex_logic(param1, param2, get_context())
}

# Edge Case 30: Ultimate complexity test - all features combined
ultimate_result := complex_generic_function<
    ComplexType<T, compute_type_param()>
>(
    unsafe {
        get_unsafe_data(raw_ptr as *const DataType)
    }.process().await,
    
    ready (validate_condition(input.transform())) {
        array[complex_index_calculation(
            base_value + offset,
            multiplier * factor
        )].method_chain().result
    } otherwise {
        fallback_processor(
            backup_data.get_safe_value(),
            error_context.get_recovery_params()
        )
    },
    
    sick pattern_input {
        mood ComplexPattern { field1, field2 } ready (field1 > field2) -> {
            process_pattern_match(field1, field2, context)
        },
        basic -> default_complex_handler()
    } fam {
        when ComplexError(code) -> {
            error_recovery_processor(code, get_error_context())
        }
    }
)

# Success message
vibez.spill("All parser strict validation tests defined!")
