yeet "testz"
yeet "vibe_context"
yeet "timez"

fr fr Test background context
slay test_background_context() {
    test_start("Background context")
    
    ctx := vibe_context.Background()
    
    fr fr Test deadline
    deadline, ok := ctx.Deadline()
    assert_eq_int(deadline, 0)
    assert_false(ok)
    
    fr fr Test done channel
    done := ctx.Done()
    assert_true(done == cap)
    
    fr fr Test error
    err := ctx.Err()
    assert_eq_string(err, "")
    
    fr fr Test value
    value := ctx.Value("test-key")
    assert_true(value == cap)
    
    fr fr Test vibe
    vibe := ctx.CheckVibe()
    assert_eq_string(vibe, "background")
}

fr fr Test empty context
slay test_empty_context() {
    test_start("Empty context")
    
    ctx := vibe_context.TODO()
    
    fr fr Test deadline
    deadline, ok := ctx.Deadline()
    assert_eq_int(deadline, 0)
    assert_false(ok)
    
    fr fr Test done channel
    done := ctx.Done()
    assert_true(done == cap)
    
    fr fr Test error
    err := ctx.Err()
    assert_eq_string(err, "")
    
    fr fr Test value
    value := ctx.Value("test-key")
    assert_true(value == cap)
    
    fr fr Test vibe
    vibe := ctx.CheckVibe()
    assert_eq_string(vibe, "empty")
}

fr fr Test cancellable context
slay test_cancellable_context() {
    test_start("Cancellable context")
    
    parent := vibe_context.Background()
    ctx, cancel := vibe_context.WithCancel(parent)
    
    fr fr Test initial state
    assert_eq_string(ctx.Err(), "")
    assert_false(vibe_context.IsCancelled(ctx))
    
    fr fr Cancel context
    cancel()
    
    fr fr Test cancelled state
    assert_true(vibe_context.IsCancelled(ctx))
    
    fr fr Test error after cancellation
    err := ctx.Err()
    assert_eq_string(err, "context canceled")
}

fr fr Test deadline context
slay test_deadline_context() {
    test_start("Deadline context")
    
    parent := vibe_context.Background()
    futureTime := 1609459200000000000 + 1000000000 fr fr 1 second in future
    ctx, cancel := vibe_context.WithDeadline(parent, futureTime)
    
    fr fr Test deadline
    deadline, ok := ctx.Deadline()
    assert_true(ok)
    assert_eq_int(deadline, futureTime)
    
    fr fr Test initial state
    assert_eq_string(ctx.Err(), "")
    assert_false(vibe_context.IsCancelled(ctx))
    
    fr fr Cancel context
    cancel()
    
    fr fr Test cancelled state
    assert_true(vibe_context.IsCancelled(ctx))
}

fr fr Test timeout context
slay test_timeout_context() {
    test_start("Timeout context")
    
    parent := vibe_context.Background()
    timeout := 1000000000 fr fr 1 second
    ctx, cancel := vibe_context.WithTimeout(parent, timeout)
    
    fr fr Test deadline exists
    deadline, ok := ctx.Deadline()
    assert_true(ok)
    assert_true(deadline > 0)
    
    fr fr Test initial state
    assert_eq_string(ctx.Err(), "")
    assert_false(vibe_context.IsCancelled(ctx))
    
    fr fr Cancel context
    cancel()
    
    fr fr Test cancelled state
    assert_true(vibe_context.IsCancelled(ctx))
}

fr fr Test value context
slay test_value_context() {
    test_start("Value context")
    
    parent := vibe_context.Background()
    key := "test-key"
    value := "test-value"
    
    ctx := vibe_context.WithValue(parent, key, value)
    
    fr fr Test value retrieval
    retrieved := ctx.Value(key)
    assert_eq_string(retrieved.(tea), value)
    
    fr fr Test non-existent key
    missing := ctx.Value("missing-key")
    assert_true(missing == cap)
    
    fr fr Test parent delegation
    assert_eq_string(ctx.CheckVibe(), "background")
}

fr fr Test vibe context
slay test_vibe_context() {
    test_start("Vibe context")
    
    parent := vibe_context.Background()
    vibe := "chill"
    
    ctx := vibe_context.WithVibe(parent, vibe)
    
    fr fr Test vibe
    checkVibe := ctx.CheckVibe()
    assert_eq_string(checkVibe, vibe)
    
    fr fr Test other methods delegate to parent
    deadline, ok := ctx.Deadline()
    assert_eq_int(deadline, 0)
    assert_false(ok)
    
    done := ctx.Done()
    assert_true(done == cap)
    
    err := ctx.Err()
    assert_eq_string(err, "")
}

fr fr Test nested contexts
slay test_nested_contexts() {
    test_start("Nested contexts")
    
    fr fr Create nested context chain
    base := vibe_context.Background()
    withValue := vibe_context.WithValue(base, "key1", "value1")
    withVibe := vibe_context.WithVibe(withValue, "relaxed")
    withCancel, cancel := vibe_context.WithCancel(withVibe)
    
    fr fr Test value lookup
    value := withCancel.Value("key1")
    assert_eq_string(value.(tea), "value1")
    
    fr fr Test vibe lookup
    vibe := withCancel.CheckVibe()
    assert_eq_string(vibe, "relaxed")
    
    fr fr Test cancellation
    cancel()
    assert_true(vibe_context.IsCancelled(withCancel))
}

fr fr Test multiple values
slay test_multiple_values() {
    test_start("Multiple values")
    
    parent := vibe_context.Background()
    values := map[interface{}]interface{}{
        "key1": "value1",
        "key2": 42,
        "key3": based
    }
    
    ctx := vibe_context.WithValues(parent, values)
    
    fr fr Test all values
    assert_eq_string(ctx.Value("key1").(tea), "value1")
    assert_eq_int(ctx.Value("key2").(normie), 42)
    assert_true(ctx.Value("key3").(lit))
}

fr fr Test context helpers
slay test_context_helpers() {
    test_start("Context helpers")
    
    parent := vibe_context.Background()
    ctx := vibe_context.WithValue(parent, "test-key", "test-value")
    
    fr fr Test HasValue
    assert_true(vibe_context.HasValue(ctx, "test-key"))
    assert_false(vibe_context.HasValue(ctx, "missing-key"))
    
    fr fr Test GetValue
    value := vibe_context.GetValue(ctx, "test-key")
    assert_eq_string(value.(tea), "test-value")
    
    fr fr Test GetDepth
    depth := vibe_context.GetDepth(ctx)
    assert_true(depth > 0)
}

fr fr Test context chaining
slay test_context_chaining() {
    test_start("Context chaining")
    
    base := vibe_context.Background()
    
    fr fr Create chain of contexts
    ctx1 := vibe_context.WithValue(base, "level", 1)
    ctx2 := vibe_context.WithValue(ctx1, "level", 2)
    ctx3 := vibe_context.WithValue(ctx2, "level", 3)
    
    fr fr Test value lookup (should get most recent)
    level := ctx3.Value("level")
    assert_eq_int(level.(normie), 3)
    
    fr fr Test depth
    depth := vibe_context.GetDepth(ctx3)
    assert_true(depth >= 3)
}

fr fr Test context expiration
slay test_context_expiration() {
    test_start("Context expiration")
    
    parent := vibe_context.Background()
    duration := 1000000000 fr fr 1 second
    ctx, cancel := vibe_context.WithExpiration(parent, duration)
    
    fr fr Test deadline exists
    deadline, ok := ctx.Deadline()
    assert_true(ok)
    assert_true(deadline > 0)
    
    fr fr Cancel context
    cancel()
    assert_true(vibe_context.IsCancelled(ctx))
}

fr fr Test context cloning
slay test_context_cloning() {
    test_start("Context cloning")
    
    parent := vibe_context.Background()
    original := vibe_context.WithValue(parent, "key", "value")
    
    fr fr Clone context
    cloned := vibe_context.Clone(original)
    
    fr fr Test cloned context has same value
    value := cloned.Value("key")
    assert_eq_string(value.(tea), "value")
}

fr fr Test context merging
slay test_context_merging() {
    test_start("Context merging")
    
    base := vibe_context.Background()
    ctx1 := vibe_context.WithValue(base, "key1", "value1")
    ctx2 := vibe_context.WithValue(base, "key2", "value2")
    
    contexts := []vibe_context.Context{ctx1, ctx2}
    merged := vibe_context.MergeContexts(contexts)
    
    fr fr Test merged context has values from both
    value1 := merged.Value("key1")
    value2 := merged.Value("key2")
    
    assert_eq_string(value1.(tea), "value1")
    assert_eq_string(value2.(tea), "value2")
}

fr fr Test detached context
slay test_detached_context() {
    test_start("Detached context")
    
    detached := vibe_context.Detached()
    
    fr fr Test detached context behaves like background
    assert_eq_string(detached.CheckVibe(), "background")
    
    deadline, ok := detached.Deadline()
    assert_eq_int(deadline, 0)
    assert_false(ok)
}

fr fr Test error constants
slay test_error_constants() {
    test_start("Error constants")
    
    fr fr Test cancel error
    parent := vibe_context.Background()
    ctx, cancel := vibe_context.WithCancel(parent)
    cancel()
    
    err := ctx.Err()
    assert_eq_string(err, "context canceled")
}

fr fr Test multiple vibes
slay test_multiple_vibes() {
    test_start("Multiple vibes")
    
    parent := vibe_context.Background()
    vibes := []tea{"chill", "focused", "energetic"}
    
    ctx := vibe_context.WithVibes(parent, vibes)
    
    fr fr Test final vibe (should be last one)
    vibe := ctx.CheckVibe()
    assert_eq_string(vibe, "energetic")
}

fr fr Run all tests
slay main() {
    test_background_context()
    test_empty_context()
    test_cancellable_context()
    test_deadline_context()
    test_timeout_context()
    test_value_context()
    test_vibe_context()
    test_nested_contexts()
    test_multiple_values()
    test_context_helpers()
    test_context_chaining()
    test_context_expiration()
    test_context_cloning()
    test_context_merging()
    test_detached_context()
    test_error_constants()
    test_multiple_vibes()
    
    print_test_summary()
}

main()
