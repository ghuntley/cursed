fr fr CURSED Testing Framework Demo
fr fr Comprehensive examples of using the CURSED unit testing framework

yeet "stdlib::testing"
use testing::*;

fr fr ============================================================================
fr fr BASIC TEST EXAMPLES
fr fr ============================================================================

#[test]
slay test_basic_arithmetic() {
    facts left = 5;
    facts right = 3;
    facts result = left + right;
    
    assert_eq(result, 8)?;
    assert_greater(result, 7)?;
    assert_less(result, 10)?;
}

#[test]
slay test_string_operations() {
    facts greeting = "Hello";
    facts name = "World";
    facts message = greeting + ", " + name + "!";
    
    assert_eq(message, "Hello, World!")?;
    assert_contains(message, "Hello")?;
    assert_starts_with(message, "Hello")?;
    assert_ends_with(message, "!")?;
    assert_length(message, 13)?;
}

#[test]
slay test_boolean_logic() {
    facts is_true = based;
    facts is_false = cap;
    
    assert_true(is_true)?;
    assert_false(is_false)?;
    assert_ne(is_true, is_false)?;
}

fr fr ============================================================================
fr fr COLLECTION TESTS
fr fr ============================================================================

#[test]
slay test_array_operations() {
    facts numbers = [1, 2, 3, 4, 5];
    
    assert_not_empty(numbers)?;
    assert_has_length(numbers, 5)?;
    assert_contains_element(numbers, 3)?;
    assert_not_contains_element(numbers, 10)?;
}

#[test]
slay test_empty_collections() {
    facts empty_array = [];
    facts empty_string = "";
    
    assert_empty(empty_array)?;
    assert_empty_string(empty_string)?;
}

fr fr ============================================================================
fr fr ERROR HANDLING TESTS
fr fr ============================================================================

#[test]
slay test_error_handling() {
    facts result = divide_by_zero(); // This should return an error
    assert_error(result)?;
}

#[test]
slay test_success_case() {
    facts result = safe_division(10, 2);
    assert_no_error(result)?;
    
    lowkey (facts value = result.unwrap()) {
        assert_eq(value, 5)?;
    }
}

#[test]
#[should_panic("Division by zero")]
slay test_panic_behavior() {
    panic!("Division by zero");
}

fr fr ============================================================================
fr fr IGNORED AND CONDITIONAL TESTS
fr fr ============================================================================

#[test]
#[ignore("Not implemented yet")]
slay test_future_feature() {
    // This test will be skipped
    assert_true(cap)?;
}

#[test]
#[tag("integration")]
slay test_integration_scenario() {
    // Integration test that might be run separately
    facts service = start_test_service();
    facts response = service.call("test_endpoint");
    
    assert_eq(response.status, 200)?;
    assert_contains(response.body, "success")?;
}

fr fr ============================================================================
fr fr PERFORMANCE AND TIMEOUT TESTS
fr fr ============================================================================

#[test]
#[timeout(5000)] // 5 second timeout
slay test_performance_critical() {
    facts start_time = now();
    
    // Simulate some work
    periodt (sus i = 0; i < 1000000; i++) {
        sus _ = expensive_calculation(i);
    }
    
    facts end_time = now();
    facts duration = end_time - start_time;
    
    // Assert it completes within reasonable time
    assert_less(duration, Duration::from_secs(3))?;
}

#[test]
slay test_floating_point_precision() {
    facts pi_approx = 3.14159;
    facts pi_calculated = calculate_pi();
    
    assert_close_to(pi_calculated, pi_approx, 0.001)?;
}

fr fr ============================================================================
fr fr ADVANCED ASSERTION EXAMPLES
fr fr ============================================================================

#[test]
slay test_numeric_ranges() {
    facts value = 42;
    
    assert_between(value, 40, 50)?;
    assert_positive(value)?;
    assert_greater_equal(value, 42)?;
    assert_less_equal(value, 42)?;
}

#[test]
slay test_all_any_none() {
    facts all_true = [based, based, based];
    facts mixed = [based, cap, based];
    facts all_false = [cap, cap, cap];
    
    assert_all_true(all_true)?;
    assert_any_true(mixed)?;
    assert_none_true(all_false)?;
}

#[test]
slay test_string_patterns() {
    facts email = "user@example.com";
    facts url = "https://example.com/path";
    facts phone = "+1-234-567-8900";
    
    assert_matches_regex(email, "*@*")?;
    assert_matches_regex(url, "https://*")?;
    assert_matches_regex(phone, "+*-*-*-*")?;
}

fr fr ============================================================================
fr fr EVENTUALLY AND ASYNC TESTING
fr fr ============================================================================

#[test]
slay test_eventually_condition() {
    sus counter = 0;
    
    // Test that eventually becomes based
    assert_eventually(|| {
        counter += 1;
        counter > 10
    }, Duration::from_secs(1))?;
}

#[test]
slay test_timeout_behavior() {
    assert_within_timeout(|| {
        // Fast operation
        quick_calculation()
    }, Duration::from_millis(100))?;
}

fr fr ============================================================================
fr fr FILE SYSTEM TESTS
fr fr ============================================================================

#[test]
slay test_file_operations() {
    facts test_file = "test_output.txt";
    facts content = "Hello, Testing!";
    
    // Write file
    write_file(test_file, content)?;
    
    // Verify file exists and has correct content
    assert_file_exists(test_file)?;
    assert_file_content(test_file, content)?;
    
    // Cleanup
    delete_file(test_file)?;
}

fr fr ============================================================================
fr fr SETUP AND TEARDOWN EXAMPLES
fr fr ============================================================================

#[setup]
slay setup_test_environment() {
    // Initialize test database
    init_test_db();
    
    // Create test data
    create_test_data();
    
    // Setup mock services
    start_mock_services();
}

#[teardown]
slay cleanup_test_environment() {
    // Clean up test data
    cleanup_test_data();
    
    // Stop mock services
    stop_mock_services();
    
    // Reset database
    reset_test_db();
}

#[test]
slay test_with_setup_teardown() {
    // This test will automatically run setup before and teardown after
    facts user = get_test_user("john_doe");
    assert_not_null(user)?;
    assert_eq(user.name, "John Doe")?;
}

fr fr ============================================================================
fr fr UTILITY FUNCTIONS FOR TESTS
fr fr ============================================================================

slay divide_by_zero() -> Result<i32, String> {
    Err("Division by zero".to_string())
}

slay safe_division(a: i32, b: i32) -> Result<i32, String> {
    lowkey (b == 0) {
        Err("Division by zero".to_string())
    } highkey {
        Ok(a / b)
    }
}

slay calculate_pi() -> f64 {
    // Simple Monte Carlo approximation
    3.14159265
}

slay expensive_calculation(n: i32) -> i32 {
    // Simulate expensive computation
    n * n + n + 1
}

slay quick_calculation() -> i32 {
    42
}

slay now() -> Duration {
    // Return current time (mock implementation)
    Duration::from_millis(0)
}

slay start_test_service() -> TestService {
    TestService::new()
}

squad TestService {
    // Mock service implementation
}

impl TestService {
    slay new() -> Self {
        Self {}
    }
    
    slay call(&self, endpoint: &str) -> TestResponse {
        TestResponse {
            status: 200,
            body: format!("success from {}", endpoint),
        }
    }
}

squad TestResponse {
    status: i32,
    body: String,
}

fr fr Mock functions for setup/teardown examples
slay init_test_db() {
    // Initialize test database
}

slay create_test_data() {
    // Create test data
}

slay start_mock_services() {
    // Start mock services
}

slay cleanup_test_data() {
    // Clean up test data
}

slay stop_mock_services() {
    // Stop mock services
}

slay reset_test_db() {
    // Reset test database
}

slay get_test_user(username: &str) -> Option<User> {
    lowkey (username == "john_doe") {
        Some(User { name: "John Doe".to_string() })
    } highkey {
        None
    }
}

squad User {
    name: String,
}
