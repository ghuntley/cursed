/// fr fr Example demonstrating the TestVibes testing framework for CURSED
/// 
/// This example shows how to use the TestVibes testing framework to write
/// comprehensive tests with good vibes! 🧪✨

use cursed::stdlib::packages::test_vibes::*;

fn main() {
    println!("🧪 TestVibes Framework Example");
    println!("==============================");
    
    // Initialize the test vibes package
    init_test_vibes();
    
    // Example 1: Basic testing
    println!("\n📝 Basic Testing Example:");
    example_basic_testing();
    
    // Example 2: Assertions
    println!("\n✅ Assertion Examples:");
    example_assertions();
    
    // Example 3: Fixtures
    println!("\n🏗️  Fixture Example:");
    example_fixtures();
    
    // Example 4: Mocking
    println!("\n🎭 Mocking Example:");
    example_mocking();
    
    // Example 5: Benchmarking
    println!("\n⚡ Benchmarking Example:");
    example_benchmarking();
    
    // Example 6: Test Runner
    println!("\n🏃 Test Runner Example:");
    example_test_runner();
    
    println!("\n🎉 All examples completed successfully!");
}

fn example_basic_testing() {
    let mut test = VibeTest::new("basic_math_test".to_string());
    
    // Test some basic math
    let result = 2 + 2;
    if result == 4 {
        test.log(&["Math works correctly!"]);
        test.pass_vibe();
    } else {
        test.fail_vibe("Math is broken!");
    }
    
    let test_result = test.get_result();
    println!("  Test '{}': {}", test_result.name, 
             if test_result.passed { "PASS ✅" } else { "FAIL ❌" });
}

fn example_assertions() {
    let mut test = VibeTest::new("assertion_test".to_string());
    
    // Basic assertions
    assert_equal(&mut test, 5, 5, "numbers should be equal");
    assert_true(&mut test, true, "true should be true");
    assert_false(&mut test, false, "false should be false");
    
    // String assertions
    assert_contains_substr(&mut test, "hello world", "world", "should contain substring");
    assert_has_prefix(&mut test, "TestVibes", "Test", "should start with Test");
    
    // Collection assertions
    let numbers = vec![1, 2, 3, 4, 5];
    assert_len(&mut test, &numbers, 5, "should have 5 elements");
    assert_contains(&mut test, &numbers, &3, "should contain 3");
    
    let test_result = test.get_result();
    println!("  Assertion test: {}", 
             if test_result.passed { "PASS ✅" } else { "FAIL ❌" });
    
    if test_result.failed {
        for error in &test_result.errors {
            println!("    Error: {}", error);
        }
    }
}

fn example_fixtures() {
    let fixture = FixtureVibe::new(
        |_t| {
            // Setup: create a mock database
            println!("    🔧 Setting up test database...");
            DatabaseFixture::new("test://localhost/testdb")
        },
        |_t, mut db| {
            // Teardown: cleanup
            println!("    🧹 Cleaning up test database...");
            db.disconnect();
        },
    );
    
    let mut test = VibeTest::new("database_test".to_string());
    fixture.run(&mut test, |t, mut db| {
        // Test database operations
        let connect_result = db.connect();
        if connect_result.is_ok() {
            t.log(&["Successfully connected to test database"]);
            
            let query_result = db.execute_query("SELECT 1");
            if query_result.is_ok() {
                t.log(&["Query executed successfully"]);
                t.pass_vibe();
            } else {
                t.fail_vibe("Query failed");
            }
        } else {
            t.fail_vibe("Failed to connect to database");
        }
    });
    
    let test_result = test.get_result();
    println!("  Fixture test: {}", 
             if test_result.passed { "PASS ✅" } else { "FAIL ❌" });
}

fn example_mocking() {
    let mock = MockVibe::new("api_service");
    
    // Set up expectations
    let _expectation = mock.expect("get_user").times(1);
    
    // Simulate API calls
    println!("    📞 Simulating API call...");
    let _result = mock.record_call("get_user", vec!["user_id_123".to_string()]);
    
    // Verify expectations
    let mut test = VibeTest::new("mock_test".to_string());
    mock.verify(&mut test);
    
    let test_result = test.get_result();
    println!("  Mock test: {}", 
             if test_result.passed { "PASS ✅" } else { "FAIL ❌" });
}

fn example_benchmarking() {
    println!("    ⏱️  Running benchmark...");
    
    let benchmark = Benchmark::new("string_concatenation", |b| {
        for _ in 0..b.iterations() {
            let _result = format!("{}{}", "hello", "world");
        }
    }).iterations(1000);
    
    let result = benchmark.run();
    println!("  Benchmark '{}': {} iterations, {:.2} ns/op", 
             result.name, result.iterations, result.ns_per_op);
}

fn example_test_runner() {
    let config = TestRunnerConfig {
        parallel: false,
        fail_fast: false,
        verbose: true,
        timeout: Some(std::time::Duration::from_secs(30)),
        filter: None,
        benchmark_iterations: 100,
        warmup_iterations: 10,
    };
    
    let mut runner = TestRunner::new(config);
    
    // Add some tests
    runner.add_test("arithmetic_test", |t| {
        let result = 10 + 5;
        assert_equal(t, 15, result, "addition should work");
        t.pass_vibe();
    });
    
    runner.add_test("string_test", |t| {
        let greeting = "Hello, TestVibes!";
        assert_contains_substr(t, greeting, "TestVibes", "should contain framework name");
        t.pass_vibe();
    });
    
    runner.add_benchmark("multiplication_bench", |b| {
        for _ in 0..b.iterations() {
            let _ = 7 * 8;
        }
    });
    
    println!("    🏃 Running test suite...");
    let summary = runner.run_tests();
    
    println!("  Test Suite Summary:");
    println!("    Total Tests: {}", summary.total_tests);
    println!("    Passed: {}", summary.passed_tests);
    println!("    Failed: {}", summary.failed_tests);
    println!("    Benchmarks: {}", summary.total_benchmarks);
    println!("    Duration: {:.2?}", summary.total_duration);
    println!("    Result: {}", if summary.all_passed() { "SUCCESS ✅" } else { "FAILED ❌" });
}

// Helper fixture for database testing
use cursed::stdlib::packages::test_vibes::fixtures::DatabaseFixture;
