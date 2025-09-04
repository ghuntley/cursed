yeet "testz"
yeet "fs_test_vibe"

slay main_character() {
    test_start("fs_test_vibe Simple Tests") fr fr Test creating MapFS
    sus fsys fs_test_vibe.MapFS = fs_test_vibe.NewMapFS()
    assert_true(fsys.initialized) fr fr Test creating test files
    sus test_file fs_test_vibe.TestFile = fs_test_vibe.CreateTestFile("test.txt", "Hello World")
    assert_eq_string(test_file.Path, "test.txt")
    assert_eq_string(test_file.Content, "Hello World")
    assert_eq_int(test_file.Mode, 644)
    assert_false(test_file.IsDir) fr fr Test creating test directory
    sus test_dir fs_test_vibe.TestFile = fs_test_vibe.CreateTestDir("testdir")
    assert_eq_string(test_dir.Path, "testdir")
    assert_eq_int(test_dir.Mode, 755)
    assert_true(test_dir.IsDir) fr fr Test filesystem operations
    sus fs_test_result lit = fs_test_vibe.TestFS(fsys)
    assert_true(fs_test_result) fr fr Test filesystem comparison
    sus fsys2 fs_test_vibe.MapFS = fs_test_vibe.NewMapFS()
    sus equal_result lit = fs_test_vibe.Equal(fsys, fsys2)
    assert_true(equal_result) fr fr Test performance metrics
    sus metrics fs_test_vibe.PerformanceMetrics = fs_test_vibe.GetPerformanceMetrics(fsys)
    assert_eq_int(metrics.ReadsPerSecond, 100)
    assert_eq_int(metrics.WritesPerSecond, 50)
    assert_eq_int(metrics.ListsPerSecond, 25)
    assert_eq_int(metrics.TotalTime, 1000) fr fr Test fuzzing
    sus fuzz_result fs_test_vibe.FuzzResult = fs_test_vibe.RunFuzzer()
    assert_eq_int(fuzz_result.OperationsPerformed, 10)
    assert_eq_int(fuzz_result.SuccessfulOperations, 8)
    assert_eq_int(fuzz_result.FailedOperations, 2)
    assert_false(fuzz_result.FoundIssue)
    assert_eq_string(fuzz_result.IssueDescription, "No issues found")
    
    vibez.spill("✅ All fs_test_vibe tests passed!")
    print_test_summary()
}
