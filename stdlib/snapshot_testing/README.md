# Snapshot Testing Framework

A comprehensive snapshot testing framework for CURSED that provides robust test verification through content snapshots and intelligent diff generation.

## Overview

The snapshot testing framework captures the output of your tests and compares them against previously saved snapshots. When tests fail, it provides detailed diffs to help you understand what changed.

## Key Features

- **Snapshot Creation & Comparison**: Automatically create and compare test output snapshots
- **Intelligent Diff Generation**: Line-by-line diff output with clear change indicators
- **Content Normalization**: Handles cross-platform line endings and whitespace
- **Update Mode**: Batch update snapshots when changes are intentional
- **File Management**: Automatic snapshot file organization and cleanup
- **Integration**: Seamless integration with testz testing framework

## Core Functions

### `snapshot_test(test_name tea, actual_output tea) lit`
Main function for snapshot testing. Creates a new snapshot if none exists, or compares against existing snapshot.

```cursed
yeet "snapshot_testing"

sus test_output tea = "Hello, World!\nExpected output here."
sus result lit = snapshot_test("my_test", test_output)
assert_true(result)
```

### `compare_with_snapshot(name tea, content tea) lit`
Alternative interface for snapshot comparison.

```cursed
sus content tea = generate_test_output()
assert_true(compare_with_snapshot("output_test", content))
```

### `update_snapshots() lit`
Enable update mode to regenerate all snapshots.

```cursed
update_snapshots()  # All subsequent snapshot tests will update files
```

### `update_snapshot(test_name tea, new_content tea) lit`
Force update a specific snapshot file.

```cursed
update_snapshot("specific_test", "New expected content")
```

## File Management

### `list_snapshots() []tea`
Get all snapshot files in the snapshot directory.

### `delete_snapshot(test_name tea) lit`
Remove a specific snapshot file.

### `cleanup_orphaned_snapshots(active_tests []tea) lit`
Remove snapshot files that no longer have corresponding tests.

## Diff Generation

The framework provides detailed diff output when snapshots don't match:

```
Snapshot diff for: my_test
=====================================
Expected:
- Hello, World!
- This is expected output.

Actual:
+ Hello, World!
+ This is modified output.
=====================================
Line-by-line diff:
Line 2:
- This is expected output.
+ This is modified output.
```

## Directory Structure

Snapshots are stored in `.snapshots/` directory:
```
.snapshots/
├── my_test.snap
├── output_test.snap
└── integration_test.snap
```

## Usage Patterns

### Basic Test Verification
```cursed
yeet "testz"
yeet "snapshot_testing"

test_start("My Feature Test")
sus output tea = run_my_feature()
assert_true(snapshot_test("feature_output", output))
print_test_summary()
```

### JSON/Structured Data Testing
```cursed
sus json_output tea = "{\n  \"name\": \"test\",\n  \"value\": 42\n}"
assert_true(snapshot_test("json_structure", json_output))
```

### Large Content Testing
```cursed
sus large_output tea = generate_large_report()
assert_true(snapshot_test("large_report", large_output))
```

### Batch Updates
```cursed
update_snapshots()  # Enable update mode
run_all_snapshot_tests()  # All snapshots will be updated
```

## Advanced Features

### Content Normalization
- Removes trailing whitespace
- Normalizes line endings across platforms (`\r\n`, `\r`, `\n`)
- Handles empty lines consistently

### Statistics and Validation
```cursed
vibez.spill(get_snapshot_stats())  # Display snapshot statistics
assert_true(validate_snapshots())  # Validate all snapshots
```

### Cross-Platform Support
The framework handles different line ending formats automatically, ensuring consistent behavior across Windows, macOS, and Linux.

## Best Practices

1. **Use Descriptive Test Names**: Choose clear, descriptive names for your snapshot tests
2. **Review Diffs Carefully**: When snapshots fail, review the diff output to ensure changes are intentional
3. **Update Intentionally**: Only update snapshots when the changes are expected and correct
4. **Regular Cleanup**: Use `cleanup_orphaned_snapshots()` to remove unused snapshot files
5. **Version Control**: Commit snapshot files to version control to track changes over time

## Integration with CI/CD

The snapshot testing framework integrates seamlessly with continuous integration:

```bash
# Run tests in CI
cargo run --bin cursed stdlib/snapshot_testing/test_snapshot_testing.csd

# Validate all snapshots
cargo run --bin cursed -- validate-snapshots

# Check for orphaned snapshots
cargo run --bin cursed -- cleanup-snapshots
```

## Testing the Framework

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/snapshot_testing/test_snapshot_testing.csd
```

The test suite covers:
- Basic snapshot functionality
- Diff generation and content comparison
- Edge cases and error handling
- Integration with testz framework
- Cross-platform compatibility
- Large content handling

## Pure CURSED Implementation

This framework is implemented entirely in pure CURSED without external dependencies, ensuring:
- Maximum portability across platforms
- Self-hosting capability
- Consistent behavior in both interpretation and compilation modes
- Zero external runtime dependencies

The framework depends only on:
- `testz` - Testing framework integration
- `dropz` - File I/O operations  
- `stringz` - String manipulation utilities
