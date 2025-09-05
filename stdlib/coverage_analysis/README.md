# CURSED Coverage Analysis

A comprehensive code coverage analysis system for CURSED projects that provides line, function, and branch coverage tracking with detailed reporting capabilities.

## Features

- **🔍 Code Instrumentation**: Automatically instrument CURSED code for coverage tracking
- **📊 Multiple Coverage Types**: Track line coverage, function coverage, and branch coverage
- **📄 Multiple Report Formats**: Generate HTML, JSON, XML, and console reports
- **⚙️ Configurable Thresholds**: Set minimum coverage requirements
- **🎯 Integration Ready**: Seamlessly integrates with testz framework and build systems
- **🔧 CLI Tool**: Standalone command-line tool for coverage analysis

## Quick Start

### Basic Usage

```bash
# Run coverage analysis on your project
cursed-coverage /path/to/your/project

# Generate HTML report with custom output directory
cursed-coverage /path/to/project --output coverage-reports --format html

# Set coverage thresholds
cursed-coverage /path/to/project --threshold-line 80 --threshold-function 75 --threshold-branch 70
```

### Using in CURSED Code

```cursed
yeet "coverage_analysis"

# Initialize coverage system
sus config CoverageConfig = {
    output_format: "html",
    output_directory: "coverage",
    threshold_line: 80.0,
    threshold_function: 75.0,
    threshold_branch: 70.0,
    include_patterns: ["src/", "stdlib/"],
    exclude_patterns: ["target/", "debug/"],
    instrument_tests: based
}

coverage_init(config)

# Run coverage analysis
sus success lit = run_coverage_analysis(".", config)
```

## CLI Tool Usage

### Installation

The `cursed-coverage` binary is built automatically with the CURSED compiler:

```bash
cargo build --release
./target/release/cursed-coverage --help
```

### Command Line Options

```bash
cursed-coverage [OPTIONS] <PROJECT_DIR>

OPTIONS:
    -o, --output <DIR>              Output directory for reports [default: coverage]
    -f, --format <FORMAT>           Report format: html, json, console, xml, all [default: html]
    --threshold-line <PERCENT>      Minimum line coverage threshold [default: 80]
    --threshold-function <PERCENT>  Minimum function coverage threshold [default: 80]
    --threshold-branch <PERCENT>    Minimum branch coverage threshold [default: 70]
    --include <PATTERN>             Include files matching pattern
    --exclude <PATTERN>             Exclude files matching pattern
    --instrument-tests              Include test files in coverage analysis
    -v, --verbose                   Verbose output
```

### Subcommands

#### Instrument Files

```bash
# Instrument a single file
cursed-coverage instrument input.💀 output.instrumented.💀
```

#### Generate Reports from Existing Data

```bash
# Generate HTML report from coverage data
cursed-coverage report coverage-data.json --format html --output reports/
```

#### Merge Coverage Data

```bash
# Merge multiple coverage data files
cursed-coverage merge file1.json file2.json file3.json
```

## API Reference

### Core Functions

#### `coverage_init(config CoverageConfig) lit`

Initialize the coverage analysis system with the specified configuration.

```cursed
sus config CoverageConfig = {
    output_format: "html",
    output_directory: "coverage",
    threshold_line: 80.0,
    threshold_function: 75.0,
    threshold_branch: 70.0,
    include_patterns: ["src/"],
    exclude_patterns: ["target/"],
    instrument_tests: cap
}

coverage_init(config)
```

#### `instrument_code(file_path tea, source_code tea) tea`

Instrument CURSED source code for coverage tracking.

```cursed
sus source tea = read_file("example.💀")
sus instrumented tea = instrument_code("example.💀", source)
write_file("example.instrumented.💀", instrumented)
```

#### `generate_coverage_report() CoverageReport`

Generate a comprehensive coverage report from collected data.

```cursed
sus report CoverageReport = generate_coverage_report()
vibez.spill("Line coverage: " + toString(report.line_coverage_percent) + "%")
```

### Coverage Tracking Functions

#### `coverage_track_line(file_path tea, line_number normie)`

Track execution of a specific line (automatically inserted during instrumentation).

#### `coverage_track_function(file_path tea, line_number normie, function_name tea)`

Track execution of a function (automatically inserted during instrumentation).

#### `coverage_track_branch(file_path tea, line_number normie, branch_taken lit)`

Track branch execution (automatically inserted during instrumentation).

### Report Generation Functions

#### `generate_html_report(report CoverageReport, output_path tea) lit`

Generate an HTML coverage report with interactive features.

```cursed
sus report CoverageReport = generate_coverage_report()
generate_html_report(report, "coverage-html")
```

#### `generate_json_report(report CoverageReport, output_path tea) lit`

Generate a JSON coverage report for integration with other tools.

```cursed
generate_json_report(report, "coverage-json")
```

#### `generate_console_report(report CoverageReport)`

Display coverage results in the console.

```cursed
generate_console_report(report)
```

### Utility Functions

#### `check_coverage_thresholds(report CoverageReport) lit`

Check if coverage meets the configured thresholds.

```cursed
sus thresholds_met lit = check_coverage_thresholds(report)
lowkey !thresholds_met {
    vibez.spill("Coverage thresholds not met!")
}
```

#### `export_coverage_data(format tea, output_path tea) lit`

Export coverage data in the specified format.

```cursed
export_coverage_data("json", "coverage-export")
```

## Data Structures

### CoverageConfig

Configuration for the coverage analysis system:

```cursed
sus CoverageConfig struct {
    output_format tea           # "html", "json", "console", "xml", "all"
    output_directory tea        # Output directory path
    threshold_line meal         # Minimum line coverage (0-100)
    threshold_function meal     # Minimum function coverage (0-100)
    threshold_branch meal       # Minimum branch coverage (0-100)
    include_patterns [tea]      # File patterns to include
    exclude_patterns [tea]      # File patterns to exclude
    instrument_tests lit        # Whether to instrument test files
}
```

### CoverageReport

Comprehensive coverage analysis results:

```cursed
sus CoverageReport struct {
    total_lines normie              # Total number of lines
    covered_lines normie            # Number of covered lines
    total_functions normie          # Total number of functions
    covered_functions normie        # Number of covered functions
    total_branches normie           # Total number of branches
    covered_branches normie         # Number of covered branches
    line_coverage_percent meal      # Line coverage percentage
    function_coverage_percent meal  # Function coverage percentage
    branch_coverage_percent meal    # Branch coverage percentage
    file_reports [tea]             # Per-file coverage reports
}
```

### CoveragePoint

Individual coverage data point:

```cursed
sus CoveragePoint struct {
    file_path tea           # Source file path
    line_number normie      # Line number
    column_number normie    # Column number
    function_name tea       # Function name (if applicable)
    hit_count normie        # Number of times executed
    branch_taken lit        # Whether branch was taken
}
```

## Integration Examples

### With testz Framework

```cursed
yeet "testz"
yeet "coverage_analysis"

# Initialize coverage for tests
coverage_init({
    output_format: "html",
    output_directory: "test-coverage",
    threshold_line: 90.0,
    threshold_function: 85.0,
    threshold_branch: 80.0,
    include_patterns: ["src/"],
    exclude_patterns: [],
    instrument_tests: based
})

test_start("Example test with coverage")

# Your test code here
sus result normie = calculate_something(42)
assert_eq_int(result, 84)

# Generate coverage report after tests
sus report CoverageReport = generate_coverage_report()
generate_html_report(report, "test-coverage")

print_test_summary()
```

### With Build System

Add to your build configuration:

```toml
# CursedBuild.toml
[coverage]
enabled = true
output_format = "all"
output_directory = "coverage"
threshold_line = 80
threshold_function = 75
threshold_branch = 70
include_patterns = ["src/", "stdlib/"]
exclude_patterns = ["target/", "debug/", "test_"]
```

### CI/CD Integration

```bash
#!/bin/bash
# coverage-check.sh

# Run coverage analysis
cursed-coverage . \
    --format all \
    --output coverage-reports \
    --threshold-line 80 \
    --threshold-function 75 \
    --threshold-branch 70 \
    --exclude target/ \
    --exclude debug/

# Check exit code
if [ $? -eq 0 ]; then
    echo "✅ Coverage thresholds met"
    exit 0
else
    echo "❌ Coverage thresholds not met"
    exit 1
fi
```

## Report Formats

### HTML Report

Interactive HTML report with:
- Color-coded coverage metrics
- File-by-file coverage breakdown
- Line-by-line coverage visualization
- Sortable tables
- Coverage trend graphs

### JSON Report

Machine-readable JSON format for integration:

```json
{
  "summary": {
    "total_lines": 1000,
    "covered_lines": 850,
    "line_coverage_percent": 85.0,
    "total_functions": 50,
    "covered_functions": 45,
    "function_coverage_percent": 90.0
  },
  "files": [
    {
      "path": "src/main.💀",
      "total_lines": 100,
      "covered_lines": 90,
      "line_coverage_percent": 90.0,
      "lines": {
        "1": 5,
        "2": 3,
        "3": 0
      }
    }
  ],
  "timestamp": "2025-01-07T12:00:00Z"
}
```

### XML Report

Compatible with common CI/CD tools:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<coverage version="1.0" timestamp="2025-01-07T12:00:00Z">
    <project>
        <metrics lines_total="1000" lines_covered="850" line_percent="85.0" />
        <files>
            <file path="src/main.💀">
                <metrics lines_total="100" lines_covered="90" line_percent="90.0" />
            </file>
        </files>
    </project>
</coverage>
```

### Console Report

Text-based report for terminal output:

```
🎯 CURSED Coverage Report
========================

📊 Coverage Summary:
  📏 Lines:     850/1000 (85.0%)
  🔧 Functions: 45/50 (90.0%)
  🌿 Branches:  120/150 (80.0%)

📁 File Coverage:
  src/main.💀 (90.0% lines, 95.0% functions)
  src/utils.💀 (80.0% lines, 85.0% functions)

✅ All coverage thresholds met!
```

## Advanced Configuration

### Include/Exclude Patterns

Control which files are analyzed:

```cursed
sus config CoverageConfig = {
    # Include only source files
    include_patterns: ["src/", "stdlib/", "lib/"],
    
    # Exclude build artifacts and test files
    exclude_patterns: ["target/", "debug/", "test_", "examples/"],
    
    # Don't instrument test files
    instrument_tests: cap
}
```

### Custom Thresholds

Set different thresholds for different coverage types:

```cursed
sus config CoverageConfig = {
    threshold_line: 90.0,      # High line coverage requirement
    threshold_function: 85.0,  # Function coverage requirement
    threshold_branch: 75.0     # Lower branch coverage (harder to achieve)
}
```

### Multiple Output Formats

Generate all report formats simultaneously:

```cursed
sus config CoverageConfig = {
    output_format: "all",
    output_directory: "comprehensive-coverage"
}
```

## Best Practices

### 1. Set Realistic Thresholds

Start with achievable thresholds and gradually increase:

```cursed
# Initial setup
threshold_line: 70.0
threshold_function: 65.0
threshold_branch: 60.0

# As code matures
threshold_line: 85.0
threshold_function: 80.0
threshold_branch: 75.0
```

### 2. Exclude Generated Code

Don't include generated or vendor code in coverage:

```cursed
exclude_patterns: [
    "target/",
    "vendor/",
    "generated/",
    "third_party/"
]
```

### 3. Focus on Critical Code

Use include patterns to focus on important code:

```cursed
include_patterns: [
    "src/core/",
    "src/api/",
    "stdlib/"
]
```

### 4. Integrate with CI/CD

Make coverage checks part of your build process:

```bash
# In your CI script
cursed-coverage . --threshold-line 80 || exit 1
```

### 5. Regular Coverage Reviews

Review coverage reports regularly and investigate:
- Files with low coverage
- Functions that are never called
- Branches that are never taken

## Troubleshooting

### Common Issues

#### Coverage Data Not Collected

**Problem**: No coverage data is generated.

**Solutions**:
- Ensure files are properly included in coverage analysis
- Check that instrumentation is working correctly
- Verify that instrumented code is actually executed

#### Low Branch Coverage

**Problem**: Branch coverage is significantly lower than line coverage.

**Solutions**:
- Add tests for error conditions
- Test both positive and negative cases
- Include edge case testing

#### Instrumentation Errors

**Problem**: Instrumented code fails to compile or run.

**Solutions**:
- Check for syntax errors in original code
- Ensure all required modules are imported
- Verify that coverage tracking functions are available

### Debug Mode

Enable verbose output for debugging:

```bash
cursed-coverage . --verbose --format console
```

### Manual Instrumentation

For debugging, manually instrument a single file:

```bash
cursed-coverage instrument src/example.💀 src/example.instrumented.💀
cursed src/example.instrumented.💀
```

## Performance Considerations

### Large Projects

For large projects with many files:

1. **Use Include Patterns**: Focus on important code sections
2. **Parallel Execution**: The system supports parallel analysis
3. **Incremental Analysis**: Only analyze changed files when possible

### Memory Usage

Coverage data is stored in memory during analysis:

- Line coverage: ~8 bytes per tracked line
- Function coverage: ~32 bytes per tracked function
- Branch coverage: ~16 bytes per tracked branch

For very large projects, consider analyzing in batches.

## Contributing

To contribute to the coverage analysis system:

1. **Test Changes**: Run the comprehensive test suite:
   ```bash
   cursed stdlib/coverage_analysis/test_coverage_analysis.💀
   ```

2. **Add Examples**: Include examples for new features

3. **Update Documentation**: Keep this README up to date

4. **Performance Testing**: Test with large codebases

## License

This coverage analysis system is part of the CURSED programming language project and follows the same license terms.

## Version History

- **v1.0.0**: Initial release with basic line and function coverage
- **v1.1.0**: Added branch coverage support
- **v1.2.0**: HTML report generation with interactive features
- **v1.3.0**: CLI tool with multiple output formats
- **v1.4.0**: Integration with testz framework
- **v1.5.0**: Advanced threshold checking and CI/CD integration

## Support

For issues or questions about the coverage analysis system:

1. Check this documentation
2. Review the test cases for examples
3. File issues with detailed reproduction steps
4. Include coverage configuration and sample code
