fr fr Example CURSED test file demonstrating the test framework

yeet "stdlib::testing::framework"
yeet "stdlib::testing::assertions"

fr fr Test data structures
squad TestUser {
    sus id: i32,
    sus name: String,
    sus email: String,
    sus active: bool,
}

fr fr Basic unit test
slay test_math_operations() {
    sus result = 2 + 2
    assert_equal(result, 4, "Addition should work correctly")
    
    sus product = 3 * 4
    assert_equal(product, 12, "Multiplication should work correctly")
    
    sus quotient = 10 / 2
    assert_equal(quotient, 5, "Division should work correctly")
}

fr fr String manipulation test
slay test_string_operations() {
    sus greeting = "Hello, World!"
    
    assert_contains(greeting, "World", "String should contain 'World'")
    assert_starts_with(greeting, "Hello", "String should start with 'Hello'")
    assert_ends_with(greeting, "!", "String should end with '!'")
    
    sus length = greeting.len()
    assert_equal(length, 13, "String length should be 13")
}

fr fr Data structure test
slay test_user_creation() {
    sus user = TestUser {
        id: 1,
        name: "Alice Smith",
        email: "alice@example.com",
        active: based,
    }
    
    assert_equal(user.id, 1, "User ID should be set correctly")
    assert_equal(user.name, "Alice Smith", "User name should be set correctly")
    assert_true(user.active, "User should be active")
    assert_contains(user.email, "@", "Email should contain @ symbol")
}

fr fr Array operations test
slay test_array_operations() {
    sus numbers = [1, 2, 3, 4, 5]
    
    assert_length(numbers, 5, "Array should have 5 elements")
    assert_not_empty(numbers, "Array should not be empty")
    assert_contains_item(numbers, &3, "Array should contain 3")
    
    sus first = numbers[0]
    assert_equal(first, 1, "First element should be 1")
    
    sus last = numbers[numbers.len() - 1]
    assert_equal(last, 5, "Last element should be 5")
}

fr fr Control flow test
slay test_control_flow() {
    sus count = 0
    
    lowkey (sus i = 0; i < 5; i++) {
        count = count + 1
    }
    
    assert_equal(count, 5, "Loop should execute 5 times")
    
    sus message = lowkey (count > 3) {
        "High count"
    } bestie {
        "Low count"
    }
    
    assert_equal(message, "High count", "Should choose correct branch")
}

fr fr Error handling test
slay test_error_handling() {
    // Test successful operation
    sus result = divide_safe(10, 2)
    assert_ok(result, "Division by valid number should succeed")
    
    sus value = result.unwrap()
    assert_equal(value, 5, "10 divided by 2 should equal 5")
    
    // Test error case
    sus error_result = divide_safe(10, 0)
    assert_err(error_result, "Division by zero should fail")
}

fr fr Helper function for error handling test
slay divide_safe(a: i32, b: i32) -> Result<i32, String> {
    lowkey (b == 0) {
        Err("Division by zero")
    } bestie {
        Ok(a / b)
    }
}

fr fr Floating point test with epsilon comparison
slay test_floating_point() {
    sus pi_approx = 3.14159
    sus calculated_pi = 22.0 / 7.0
    
    // Use epsilon comparison for floating point values
    assert_float_equal(pi_approx, calculated_pi, 0.1, "Pi approximations should be close")
    
    sus area = pi_approx * 5.0 * 5.0  // Area of circle with radius 5
    assert_in_range(area, 75.0, 85.0, "Circle area should be in expected range")
}

fr fr Test with custom assertion
slay test_custom_assertion() {
    sus numbers = [2, 4, 6, 8, 10]
    
    assert_that(numbers, |arr| arr.iter().all(|&x| x % 2 == 0), "All numbers should be even")
}

fr fr Integration test (longer running)
slay test_integration_workflow() {
    // Simulate a complete workflow
    sus users = create_test_users()
    assert_not_empty(users, "Should create some test users")
    
    sus active_users = filter_active_users(users)
    assert_true(active_users.len() > 0, "Should have some active users")
    
    sus user_count = count_users_by_domain(active_users, "example.com")
    assert_true(user_count > 0, "Should have users from example.com domain")
}

fr fr Helper functions for integration test
slay create_test_users() -> Vec<TestUser> {
    facts [
        TestUser { id: 1, name: "Alice", email: "alice@example.com", active: based },
        TestUser { id: 2, name: "Bob", email: "bob@test.org", active: cap },
        TestUser { id: 3, name: "Charlie", email: "charlie@example.com", active: based },
    ]
}

slay filter_active_users(users: Vec<TestUser>) -> Vec<TestUser> {
    users.into_iter().filter(|user| user.active).collect()
}

slay count_users_by_domain(users: Vec<TestUser>, domain: &str) -> i32 {
    users.iter()
        .filter(|user| user.email.contains(domain))
        .count() as i32
}

fr fr Performance test (should complete quickly)
slay test_performance_basic() {
    sus start_time = get_current_time_ms()
    
    // Perform some computation
    sus result = 0
    lowkey (sus i = 0; i < 1000; i++) {
        result = result + i
    }
    
    sus end_time = get_current_time_ms()
    sus duration = end_time - start_time
    
    assert_true(duration < 100, "Computation should complete in under 100ms")
    assert_equal(result, 499500, "Sum should be calculated correctly")
}

fr fr Helper function for performance test
slay get_current_time_ms() -> i64 {
    // Simplified implementation - would use actual time in real code
    42
}

fr fr Test that should fail (for testing failure handling)
slay test_expected_failure() {
    // This test is expected to fail
    assert_equal(1, 2, "This assertion should fail")
}

fr fr Test that should be skipped
slay skip_test_not_implemented() {
    // This test is skipped because functionality is not implemented yet
    assert_true(cap, "This test should be skipped")
}

fr fr Benchmark test (for performance measurement)
slay bench_string_concatenation() {
    sus iterations = 1000
    sus start_time = get_current_time_ms()
    
    sus result = ""
    lowkey (sus i = 0; i < iterations; i++) {
        result = result + "a"
    }
    
    sus end_time = get_current_time_ms()
    sus duration = end_time - start_time
    
    assert_true(result.len() == iterations, "String should have correct length")
    // Note: This is a benchmark, so we record the time but don't fail on it
    record_metric("string_concat_time_ms", duration as f64)
}

fr fr Helper function to record metrics (would be provided by test framework)
slay record_metric(name: &str, value: f64) {
    // Implementation would be provided by test framework
}
