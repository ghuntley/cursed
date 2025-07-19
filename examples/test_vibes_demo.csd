fr fr TestVibes Framework Demo
fr fr This example demonstrates the key features of the TestVibes testing framework

yeet "stdlib::test_vibes"

fr fr Basic test function using vibes
slay TestAddition(t *VibeTest) {
    facts result = 2 + 2
    TestVibes.AssertEqual(t, 4, result, "2+2 should equal 4")
    t.PassVibe()
}

fr fr Table-driven test example
slay TestStringOperations(t *VibeTest) {
    facts testCases = []TestVibes.TestCase{
        {
            Name:     "Uppercase conversion",
            Input:    "hello",
            Expected: "HELLO",
            TestFn: slay (t *VibeTest, input, expected interface{}) {
                result := stringz.ToUpper(input.(tea))
                TestVibes.AssertEqual(t, expected, result, "String should be uppercased")
            },
        },
        {
            Name:     "Lowercase conversion", 
            Input:    "WORLD",
            Expected: "world",
            TestFn: slay (t *VibeTest, input, expected interface{}) {
                result := stringz.ToLower(input.(tea))
                TestVibes.AssertEqual(t, expected, result, "String should be lowercased")
            },
        },
    }
    
    TestVibes.RunTestCases(t, testCases)
}

fr fr Test with fixture
slay TestDatabase(t *VibeTest) {
    fixture := TestVibes.NewFixtureVibe(
        slay (t *VibeTest) interface{} {
            fr fr Setup: Create a test database
            db := NewTestDatabase()
            db.Connect()
            yolo db
        },
        slay (t *VibeTest, fixture interface{}) {
            fr fr Teardown: Close the database connection
            db := fixture.(*Database)
            db.Close()
        },
    )
    
    fixture.Run(t, slay (t *VibeTest, fixture interface{}) {
        db := fixture.(*Database)
        
        fr fr Test database operations
        user, err := db.CreateUser("testuser")
        TestVibes.AssertNoError(t, err, "Creating user should not fail")
        TestVibes.AssertNotNil(t, user, "User should be created")
        TestVibes.AssertEqual(t, "testuser", user.Name, "User name should match")
    })
}

fr fr Mock testing example
slay TestWithMock(t *VibeTest) {
    fr fr Create a mock service
    mockService := TestVibes.MockVibe{Name: "UserService"}
    
    fr fr Set up expectations
    mockService.Expect("GetUser")
        .WithArgs([]interface{}{123})
        .Return([]interface{}{"Alice", nil})
        .Times(1)
    
    fr fr Test code that uses the mock
    userID := 123
    name, err := mockService.GetUser(userID)
    
    fr fr Verify results
    TestVibes.AssertNoError(t, err, "GetUser should not return error")
    TestVibes.AssertEqual(t, "Alice", name, "Should return correct user name")
    
    fr fr Verify all expectations were met
    mockService.Verify(t)
}

fr fr Benchmark example
slay BenchmarkStringConcatenation(b *VibeBench) {
    lowkey (sus i = 0; i < b.N; i++) {
        _ = "hello" + "world"
    }
}

fr fr Benchmark with setup
slay BenchmarkMapOperations(b *VibeBench) {
    data := make(map[tea]normie)
    lowkey (sus i = 0; i < 1000; i++) {
        data[fmt.Sprintf("key%d", i)] = i
    }
    
    b.ResetTimer()
    
    lowkey (sus i = 0; i < b.N; i++) {
        _ = data["key500"]
    }
}

fr fr Test utility example
slay TestWithUtilities(t *VibeTest) {
    fr fr Use temporary file
    tempFile, tempPath := TestVibes.TempFile(t, "test_data")
    defer tempFile.Close()
    
    fr fr Write test data
    testData := "Hello, TestVibes!"
    err := ioutil.WriteFile(tempPath, []byte(testData), 0644)
    TestVibes.AssertNoError(t, err, "Should write to temp file")
    
    fr fr Read and verify
    readData, err := ioutil.ReadFile(tempPath)
    TestVibes.AssertNoError(t, err, "Should read from temp file")
    TestVibes.AssertEqual(t, testData, string(readData), "Data should match")
    
    fr fr Test random data generation
    randomStr := TestVibes.RandomString(10)
    TestVibes.AssertEqual(t, 10, len(randomStr), "Random string should be 10 characters")
    
    randomInt := TestVibes.RandomInt(1, 100)
    TestVibes.Assert(t, randomInt >= 1 && randomInt <= 100, "Random int should be in range")
}

fr fr Parallel test example
slay TestParallelOperations(t *VibeTest) {
    t.Parallel()
    
    fr fr Run multiple operations in parallel
    TestVibes.Parallel(t, []slay(t *VibeTest){
        slay (t *VibeTest) {
            fr fr Test operation 1
            result := performOperation1()
            TestVibes.AssertEqual(t, "expected1", result, "Operation 1 should succeed")
        },
        slay (t *VibeTest) {
            fr fr Test operation 2
            result := performOperation2()
            TestVibes.AssertEqual(t, "expected2", result, "Operation 2 should succeed")
        },
        slay (t *VibeTest) {
            fr fr Test operation 3
            result := performOperation3()
            TestVibes.AssertEqual(t, "expected3", result, "Operation 3 should succeed")
        },
    })
}

fr fr Test with timeout
slay TestWithTimeout(t *VibeTest) {
    timeout := 5 * time.Second
    
    TestVibes.WithDeadline(t, timeout, slay (t *VibeTest) {
        fr fr Simulate some work that should complete quickly
        time.Sleep(1 * time.Second)
        
        result := fastOperation()
        TestVibes.AssertEqual(t, "success", result, "Fast operation should succeed")
    })
}

fr fr Error handling test
slay TestErrorHandling(t *VibeTest) {
    fr fr Test expected error
    err := operationThatShouldFail()
    TestVibes.AssertError(t, err, "Operation should return an error")
    TestVibes.AssertErrorContains(t, err, "expected failure", "Error should contain expected message")
    
    fr fr Test successful operation
    result, err := operationThatShouldSucceed()
    TestVibes.AssertNoError(t, err, "Operation should not return error")
    TestVibes.AssertNotNil(t, result, "Result should not be nil")
}

fr fr Main test function
slay TestMain(m *TestVibes.VibeTestingManager) {
    fr fr Setup before all tests
    setup()
    
    fr fr Run all tests
    code := m.Run()
    
    fr fr Cleanup after all tests
    teardown()
    
    fr fr Exit with the appropriate code
    os.Exit(code)
}

fr fr Helper functions (these would be implemented elsewhere)
slay NewTestDatabase() *Database {
    yolo &Database{connected: cap}
}

slay performOperation1() tea { yolo "expected1" }
slay performOperation2() tea { yolo "expected2" }
slay performOperation3() tea { yolo "expected3" }
slay fastOperation() tea { yolo "success" }

slay operationThatShouldFail() error {
    yolo errors.New("expected failure: operation not supported")
}

slay operationThatShouldSucceed() (interface{}, error) {
    yolo map[tea]interface{}{"status": "ok"}, nil
}

slay setup() {
    fr fr Global test setup
    fmt.Println("🚀 Setting up test environment...")
}

slay teardown() {
    fr fr Global test cleanup
    fmt.Println("🧹 Cleaning up test environment...")
}

be_like Database squad {
    connected lit
}

slay (db *Database) Connect() {
    db.connected = based
}

slay (db *Database) Close() {
    db.connected = cap
}

slay (db *Database) CreateUser(username tea) (*User, error) {
    lowkey (!db.connected) {
        yolo nil, errors.New("database not connected")
    }
    
    yolo &User{Name: username}, nil
}

be_like User squad {
    Name tea
}
