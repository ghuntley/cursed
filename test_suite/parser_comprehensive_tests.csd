# ===================================================================
# ORACLE PRIORITY 1: Comprehensive Parser Tests
# 30+ fuzz and golden tests for 100% spec compliance
# ===================================================================

# Test 1: Complex bestie loop headers with nested expressions
bestie (i := 0; (array[i] + complex_expr(x, y)) < max_val && check_condition(i); i++) {
    complex_operation(i, array[get_index(i) + 1])
}

# Test 2: Nested ready/otherwise conditionals with complex expressions
ready (x > 0 && (y * z) > threshold || special_case(data)) {
    process_positive(x)
} otherwise ready (x < 0) {
    process_negative(x) 
} otherwise {
    process_zero()
}

# Test 3: Chained method calls inside array indexing
result := data[parser.getIndex().calculate(offset)].process().getValue()
complex_array[obj.method().chain().call()] = new_value

# Test 4: Complex operator precedence chains
result := a + b * c - d / e % f
comparison := (x + y) * z == w && p || q != r

# Test 5: Nested function calls with complex arguments
fn_call(
    another_fn(x + y, array[index]),
    complex_expr(a * b, c / d),
    nested.method().call(param)
)

# Test 6: Complex array and struct literals with expressions
array := [
    compute_value(x),
    complex_expr(a, b) + offset,
    nested[index].property
]

struct_data := {
    field1: calculate(x, y),
    field2: array[complex_index(i)],
    field3: obj.method().result
}

# Test 7: Complex assignment operators with precedence
x += y * z + w
array[i] -= func_call(a, b) / divisor
obj.property *= (base + offset) % modulus

# Test 8: Nested error handling with complex expressions
result := complex_operation(x, y) fam {
    when error -> fallback_value(z)
} fam {
    alternative_error_handler()
}

# Test 9: Complex channel operations with expressions
dm_send(channel, complex_calculation(x, y, z))
value := dm_recv(channels[get_channel_index(priority)])

# Test 10: Pattern matching with nested expressions
sick variable {
    mood complex_pattern(x) -> process_match(x, y),
    mood array[i] -> handle_array_match(),
    basic -> default_handler(fallback_value)
}

# Test 11: Complex generic type instantiations
generic_func<Complex<T>, Array<U>>(
    param1,
    array[complex_index],
    obj.method().result
)

# Test 12: Deeply nested parentheses and operators
result := (((a + b) * c) - ((d / e) % f)) + (g * (h + i))

# Test 13: Complex string interpolations and concatenations  
message := "Result: " + calculate(x, y).toString() + " (status: " + status + ")"

# Test 14: Complex loop conditions with multiple clauses
bestie (condition1() && (array[i] > threshold || special_check(data))) {
    process_with_complex_logic()
    ready (early_exit_condition(i, data)) {
        ghosted
    }
}

# Test 15: Complex defer statements with expressions
later cleanup_with_complex_args(
    resource.get().handle,
    calculate_cleanup_params(x, y)
)

# Test 16: Complex goroutine spawning with expressions
stan worker(
    channels[worker_id],
    config.get_worker_params(id),
    data[start_index:end_index]
)

# Test 17: Complex select statements with expressions
ready {
    mood dm_send(output_channels[priority], result) -> {
        log_send_success(priority, result)
    },
    mood response := dm_recv(input_channels[get_input_id()]) -> {
        process_response(response, context)
    },
    basic -> handle_timeout(current_time() - start_time)
}

# Test 18: Complex type assertions and conversions
value := interface_obj.(Complex<T>)
converted := convert_type<Target>(source.method().result)

# Test 19: Complex slice operations with expressions
slice := array[start_index(x):end_index(y):step_size(z)]
result := data[complex_start():complex_end()]

# Test 20: Complex lambda expressions and closures
mapper := slay(x) { damn x * multiplier + offset }
filter := slay(item) { damn item.property > threshold && validate(item) }

# Test 21: Complex interface implementations with expressions
impl ComplexInterface<T> for MyStruct {
    slay complex_method(param T) T {
        damn process_with_complex_logic(param, self.field)
    }
}

# Test 22: Complex async/await with expressions  
result := await complex_async_operation(
    param1,
    await another_async_call(x, y),
    calculate_sync_param(z)
)

# Test 23: Complex macro invocations with expressions
macro_call!(
    complex_arg(a, b),
    array[nested_index],
    obj.method().chain()
)

# Test 24: Complex import statements with expressions
yeet "module" { 
    symbol1 as alias1, 
    symbol2,
    nested_symbol.sub_symbol 
}

# Test 25: Complex constraint specifications
slay generic_func<T: Constraint1 + Constraint2<U>, U: Send + Sync>(param T) -> Result<T, Error> {
    damn process_constrained(param)
}

# Test 26: Complex match guards with expressions
sick value {
    mood pattern ready (guard_condition(x, y)) -> handle_guarded(),
    mood Complex { field1, field2 } ready (field1 > field2) -> process_complex(),
    basic -> default_case()
}

# Test 27: Complex range expressions with expressions
bestie item := flex array[start..calculate_end(size)] {
    process_item_with_range(item, get_context())
}

# Test 28: Complex attribute applications with expressions
@[derive(Clone, Debug)]
@[cfg(target = get_target_config())]
squad ComplexStruct<T> {
    @[validate(range(min_val(), max_val()))]
    field: T
}

# Test 29: Complex unsafe blocks with expressions  
unsafe {
    raw_pointer := get_raw_ptr(obj) as *mut T
    *raw_pointer = calculate_unsafe_value(context)
}

# Test 30: Complex lifetime annotations with expressions
slay complex_lifetime_fn<'a, 'b>(x: &'a Complex<T>, y: &'b Array<U>) -> &'a Result<T, Error>
where 'a: 'b {
    damn process_with_lifetimes(x, y, get_context())
}

# Test 31: Deeply nested expressions with all operators
final_result := (
    (a + b) * (c - d) / (e + f) % (g * h) + 
    (i && j || k) == (l != m) && 
    (n > o || p < q) && 
    array[complex_index(x, y, z)].method().chain(param) +
    struct_literal{ field: nested_calc(a, b, c) }.access()
)

# Test 32: Complex error propagation chains
result := risky_operation1()?.risky_operation2()?.risky_operation3()?
final := chain_of_fallible_calls(a, b, c).propagate_error()?
