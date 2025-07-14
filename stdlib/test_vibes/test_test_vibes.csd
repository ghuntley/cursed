yeet "testz"
yeet "test_vibes"

slay test_vibe_test_creation() {
    test_start("VibeTest Creation")
    
    sus t := test_vibes.NewVibeTest("sample_test")
    assert_eq_string(t.Name(), "sample_test")
    assert_false(t.Failed())
    assert_false(t.Skipped())
    
    print_test_summary()
}

slay test_vibe_bench_creation() {
    test_start("VibeBench Creation")
    
    sus b := test_vibes.NewVibeBench("sample_benchmark")
    assert_eq_string(b.Name(), "sample_benchmark")
    assert_eq_int(b.N, 1)
    assert_false(b.Failed())
    assert_false(b.Skipped())
    
    print_test_summary()
}

slay test_basic_assertions() {
    test_start("Basic Assertions")
    
    sus t := test_vibes.NewVibeTest("assertion_test")
    
    fr fr Test Assert
    test_vibes.Assert(t, based, "should pass")
    assert_false(t.Failed())
    
    test_vibes.Assert(t, cap, "should fail")
    assert_true(t.Failed())
    
    print_test_summary()
}

slay test_equality_assertions() {
    test_start("Equality Assertions")
    
    sus t := test_vibes.NewVibeTest("equality_test")
    
    fr fr Test AssertEqual
    test_vibes.AssertEqual(t, 42, 42, "integers should be equal")
    test_vibes.AssertEqual(t, "hello", "hello", "strings should be equal")
    test_vibes.AssertEqual(t, based, based, "booleans should be equal")
    
    fr fr Test AssertNotEqual
    test_vibes.AssertNotEqual(t, 42, 43, "integers should not be equal")
    test_vibes.AssertNotEqual(t, "hello", "world", "strings should not be equal")
    
    print_test_summary()
}

slay test_nil_assertions() {
    test_start("Nil Assertions")
    
    sus t := test_vibes.NewVibeTest("nil_test")
    
    fr fr Test AssertNil
    test_vibes.AssertNil(t, cap, "nil should be nil")
    
    fr fr Test AssertNotNil
    test_vibes.AssertNotNil(t, "not nil", "string should not be nil")
    test_vibes.AssertNotNil(t, 42, "integer should not be nil")
    
    print_test_summary()
}

slay test_boolean_assertions() {
    test_start("Boolean Assertions")
    
    sus t := test_vibes.NewVibeTest("boolean_test")
    
    fr fr Test AssertTrue
    test_vibes.AssertTrue(t, based, "should be true")
    
    fr fr Test AssertFalse
    test_vibes.AssertFalse(t, cap, "should be false")
    
    print_test_summary()
}

slay test_error_assertions() {
    test_start("Error Assertions")
    
    sus t := test_vibes.NewVibeTest("error_test")
    
    fr fr Test AssertError
    test_vibes.AssertError(t, "some error", "should have error")
    
    fr fr Test AssertNoError
    test_vibes.AssertNoError(t, "", "should have no error")
    
    fr fr Test AssertErrorIs
    test_vibes.AssertErrorIs(t, "specific error", "specific error", "errors should match")
    
    fr fr Test AssertErrorContains
    test_vibes.AssertErrorContains(t, "this is an error message", "error", "should contain substring")
    
    print_test_summary()
}

slay test_collection_assertions() {
    test_start("Collection Assertions")
    
    sus t := test_vibes.NewVibeTest("collection_test")
    
    fr fr Test AssertLen
    sus slice := []tea{"a", "b", "c"}
    test_vibes.AssertLen(t, slice, 3, "slice should have length 3")
    
    sus str := "hello"
    test_vibes.AssertLen(t, str, 5, "string should have length 5")
    
    fr fr Test AssertEmpty
    sus empty := []tea{}
    test_vibes.AssertEmpty(t, empty, "slice should be empty")
    
    fr fr Test AssertNotEmpty
    test_vibes.AssertNotEmpty(t, slice, "slice should not be empty")
    
    fr fr Test AssertContains
    test_vibes.AssertContains(t, slice, "b", "slice should contain 'b'")
    test_vibes.AssertContains(t, "hello world", "world", "string should contain 'world'")
    
    fr fr Test AssertNotContains
    test_vibes.AssertNotContains(t, slice, "d", "slice should not contain 'd'")
    
    print_test_summary()
}

slay test_numeric_assertions() {
    test_start("Numeric Assertions")
    
    sus t := test_vibes.NewVibeTest("numeric_test")
    
    fr fr Test AssertGreater
    test_vibes.AssertGreater(t, 10, 5, "10 should be greater than 5")
    
    fr fr Test AssertGreaterOrEqual
    test_vibes.AssertGreaterOrEqual(t, 10, 10, "10 should be greater than or equal to 10")
    test_vibes.AssertGreaterOrEqual(t, 10, 5, "10 should be greater than or equal to 5")
    
    fr fr Test AssertLess
    test_vibes.AssertLess(t, 5, 10, "5 should be less than 10")
    
    fr fr Test AssertLessOrEqual
    test_vibes.AssertLessOrEqual(t, 5, 5, "5 should be less than or equal to 5")
    test_vibes.AssertLessOrEqual(t, 5, 10, "5 should be less than or equal to 10")
    
    fr fr Test AssertZero
    test_vibes.AssertZero(t, 0, "0 should be zero")
    test_vibes.AssertZero(t, "", "empty string should be zero")
    
    fr fr Test AssertNotZero
    test_vibes.AssertNotZero(t, 42, "42 should not be zero")
    test_vibes.AssertNotZero(t, "hello", "non-empty string should not be zero")
    
    print_test_summary()
}

slay test_string_assertions() {
    test_start("String Assertions")
    
    sus t := test_vibes.NewVibeTest("string_test")
    
    fr fr Test AssertContainsSubtea
    test_vibes.AssertContainsSubtea(t, "hello world", "world", "should contain substring")
    
    fr fr Test AssertHasPrefix
    test_vibes.AssertHasPrefix(t, "hello world", "hello", "should have prefix")
    
    fr fr Test AssertHasSuffix
    test_vibes.AssertHasSuffix(t, "hello world", "world", "should have suffix")
    
    fr fr Test AssertMatchesRegex
    test_vibes.AssertMatchesRegex(t, "hello world", "world", "should match pattern")
    
    print_test_summary()
}

slay test_type_assertions() {
    test_start("Type Assertions")
    
    sus t := test_vibes.NewVibeTest("type_test")
    
    fr fr Test AssertType
    test_vibes.AssertType(t, "string", "hello", "should be string type")
    
    fr fr Test AssertImplements
    test_vibes.AssertImplements(t, "interface", "implementation", "should implement interface")
    
    print_test_summary()
}

slay test_panic_assertions() {
    test_start("Panic Assertions")
    
    sus t := test_vibes.NewVibeTest("panic_test")
    
    fr fr Test AssertShooks (panic)
    test_vibes.AssertShooks(t, func() { fr fr would panic }, "should panic")
    
    fr fr Test AssertShooksWithValue
    test_vibes.AssertShooksWithValue(t, "panic value", func() { fr fr would panic }, "should panic with value")
    
    fr fr Test AssertNoShook
    test_vibes.AssertNoShook(t, func() { fr fr normal function }, "should not panic")
    
    print_test_summary()
}

slay test_fixture_system() {
    test_start("Fixture System")
    
    sus setupCalled := cap
    sus teardownCalled := cap
    
    sus fixture := test_vibes.NewFixtureVibe(
        func(t *test_vibes.VibeTest) interface{} {
            setupCalled = based
            damn "test fixture"
        },
        func(t *test_vibes.VibeTest, fixture interface{}) {
            teardownCalled = based
        },
    )
    
    sus t := test_vibes.NewVibeTest("fixture_test")
    fixture.Run(t, func(t *test_vibes.VibeTest, fixture interface{}) {
        assert_eq_string(fixture.(tea), "test fixture")
    })
    
    assert_true(setupCalled)
    assert_true(teardownCalled)
    
    print_test_summary()
}

slay test_table_driven_tests() {
    test_start("Table Driven Tests")
    
    sus t := test_vibes.NewVibeTest("table_test")
    
    sus testCases := []test_vibes.TestCase{
        {
            Name:     "Addition",
            Input:    []normie{2, 3},
            Expected: 5,
            TestFn: func(t *test_vibes.VibeTest, input, expected interface{}) {
                sus nums := input.([]normie)
                sus result := nums[0] + nums[1]
                test_vibes.AssertEqual(t, expected, result, "addition should work")
            },
        },
        {
            Name:     "Subtraction",
            Input:    []normie{10, 3},
            Expected: 7,
            TestFn: func(t *test_vibes.VibeTest, input, expected interface{}) {
                sus nums := input.([]normie)
                sus result := nums[0] - nums[1]
                test_vibes.AssertEqual(t, expected, result, "subtraction should work")
            },
        },
    }
    
    test_vibes.RunTestCases(t, testCases)
    
    print_test_summary()
}

slay test_mocking_system() {
    test_start("Mocking System")
    
    sus mock := test_vibes.NewMockVibe("TestMock")
    assert_eq_string(mock.Name, "TestMock")
    
    fr fr Test expectation creation
    sus exp := mock.Expect("methodName")
    assert_eq_string(exp.methodName, "methodName")
    assert_eq_int(exp.expectedCalls, 1)
    
    fr fr Test expectation configuration
    exp.WithArgs("arg1", "arg2").Return("result").Times(2)
    assert_eq_int(exp.expectedCalls, 2)
    
    fr fr Test stub creation
    sus stub := mock.Stub("stubMethod", "stubResult")
    assert_eq_string(stub.methodName, "stubMethod")
    
    fr fr Test verification (would normally fail due to no actual calls)
    sus t := test_vibes.NewVibeTest("mock_test")
    mock.Verify(t)
    
    print_test_summary()
}

slay test_test_utilities() {
    test_start("Test Utilities")
    
    sus t := test_vibes.NewVibeTest("utilities_test")
    
    fr fr Test TempFile
    sus filename, err := test_vibes.TempFile(t, "test_pattern")
    assert_eq_string(err, "")
    assert_true(filename != "")
    
    fr fr Test TempDir
    sus dirname := test_vibes.TempDir(t, "test_dir")
    assert_true(dirname != "")
    
    fr fr Test WithSetup
    sus setupCalled := cap
    sus teardownCalled := cap
    
    test_vibes.WithSetup(t, 
        func() { setupCalled = based },
        func() { teardownCalled = based },
        func(t *test_vibes.VibeTest) {
            assert_true(setupCalled)
        },
    )
    
    assert_true(teardownCalled)
    
    print_test_summary()
}

slay test_random_data_generation() {
    test_start("Random Data Generation")
    
    fr fr Test RandomString
    sus str := test_vibes.RandomString(10)
    assert_eq_int(len(str), 10)
    
    fr fr Test RandomInt
    sus num := test_vibes.RandomInt(1, 10)
    assert_true(num >= 1 && num <= 10)
    
    fr fr Test RandomFloat
    sus fl := test_vibes.RandomFloat(1.0, 10.0)
    assert_true(fl >= 1.0 && fl <= 10.0)
    
    fr fr Test RandomBytes
    sus bytes := test_vibes.RandomBytes(5)
    assert_eq_int(len(bytes), 5)
    
    print_test_summary()
}

slay test_benchmarking() {
    test_start("Benchmarking")
    
    fr fr Test benchmark creation and timing
    sus bench := test_vibes.NewVibeBench("test_benchmark")
    
    bench.ResetTimer()
    bench.StartTimer()
    fr fr Simulate some work
    bench.StopTimer()
    
    bench.SetBytes(100)
    bench.SetParallelism(4)
    bench.ReportMetric(1.5, "custom_metric")
    
    assert_eq_int(bench.bytes, 100)
    assert_eq_int(bench.parallelism, 4)
    
    print_test_summary()
}

slay test_vibe_test_methods() {
    test_start("VibeTest Methods")
    
    sus t := test_vibes.NewVibeTest("method_test")
    
    fr fr Test logging
    t.Log("Test log message")
    t.Logf("Test formatted log: %s", "value")
    
    fr fr Test helper marking
    t.Helper()
    
    fr fr Test parallel marking
    t.Parallel()
    
    fr fr Test temp directory
    sus tempDir := t.TempDir()
    assert_true(tempDir != "")
    
    fr fr Test vibe methods
    t.PassVibe()
    assert_false(t.Failed())
    
    print_test_summary()
}

slay test_testing_manager() {
    test_start("Testing Manager")
    
    sus manager := test_vibes.NewVibeTestingManager()
    assert_eq_int(manager.exitCode, 0)
    
    fr fr Test run with no tests
    sus exitCode := manager.Run()
    assert_eq_int(exitCode, 0)
    
    print_test_summary()
}

slay main() {
    test_vibe_test_creation()
    test_vibe_bench_creation()
    test_basic_assertions()
    test_equality_assertions()
    test_nil_assertions()
    test_boolean_assertions()
    test_error_assertions()
    test_collection_assertions()
    test_numeric_assertions()
    test_string_assertions()
    test_type_assertions()
    test_panic_assertions()
    test_fixture_system()
    test_table_driven_tests()
    test_mocking_system()
    test_test_utilities()
    test_random_data_generation()
    test_benchmarking()
    test_vibe_test_methods()
    test_testing_manager()
}
