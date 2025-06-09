# CURSED Language Linter

The CURSED linter is a comprehensive code analysis tool that helps maintain code quality and consistency. It provides style checking, error detection, performance optimization suggestions, and CURSED-specific best practices.

## Features

### Production-Ready Capabilities
- **Rule-based Analysis**: Comprehensive rule system with configurable severity levels
- **Multiple Output Formats**: Human-readable, JSON, Checkstyle XML, and SARIF
- **Auto-fixing**: Automatic correction of style and formatting issues
- **Configuration System**: Flexible configuration with TOML, JSON, and YAML support
- **Performance**: Optimized for large codebases with parallel processing
- **Integration**: CI/CD ready with proper exit codes and reporting

### Rule Categories

#### Style Rules
- **Line Length**: Enforce maximum line length limits
- **Trailing Whitespace**: Detect and fix trailing whitespace
- **Mixed Indentation**: Ensure consistent indentation (tabs vs spaces)
- **Empty Lines**: Limit consecutive empty lines
- **Naming Conventions**: Enforce consistent naming patterns
- **Operator Spacing**: Ensure proper spacing around operators
- **Comma Spacing**: Consistent spacing around commas
- **Brace Style**: Enforce consistent brace placement

#### Correctness Rules
- **Unused Variables**: Detect variables that are declared but never used
- **Unused Functions**: Find functions that are never called
- **Unreachable Code**: Identify code that can never be executed
- **Dead Code**: Detect expressions with no effect
- **Variable Shadowing**: Warn about variables that shadow outer scope
- **Unused Imports**: Find imports that are never used

#### Performance Rules
- **Unnecessary Allocations**: Detect memory allocations that could be avoided
- **String Concatenation**: Suggest efficient string building patterns
- **Inefficient Loops**: Recommend iterator patterns over manual loops
- **Redundant Clones**: Find unnecessary clone operations

#### Complexity Rules
- **Cyclomatic Complexity**: Measure function complexity
- **Nesting Depth**: Limit maximum nesting levels
- **Parameter Count**: Enforce reasonable function parameter limits
- **Cognitive Complexity**: Assess how difficult code is to understand

#### CURSED-Specific Rules
- **Gen Z Naming**: Encourage use of Gen Z slang in identifiers
- **Slang Usage**: Suggest proper Gen Z slang patterns
- **Interface Design**: Check interface implementation patterns
- **Goroutine Best Practices**: Ensure proper goroutine usage
- **Channel Usage**: Validate channel operation patterns

## Installation and Usage

### Command Line Interface

```bash
# Basic linting
cursed-lint file.csd

# Lint directory recursively
cursed-lint --recursive src/

# Auto-fix issues
cursed-lint --fix src/

# Generate configuration
cursed-lint --generate-config .cursed-lint.toml

# List all rules
cursed-lint --list-rules

# Explain specific rule
cursed-lint --explain line-length
```

### Makefile Integration

```bash
# Lint all CURSED files
make cursed-lint

# Strict linting for CI
make cursed-lint-check

# Auto-fix issues
make cursed-lint-fix

# Show detailed statistics
make cursed-lint-stats

# Show help
make cursed-lint-help
```

### Configuration

The linter supports multiple configuration file formats:

#### TOML Configuration (`.cursed-lint.toml`)
```toml
auto_fix = false
min_severity = "info"

[general]
max_line_length = 100
indent_size = 4
enforce_genz_naming = true

[rules.style]
enabled = true
default_severity = "warning"

[output]
format = "human"
use_colors = true
```

#### JSON Configuration (`.cursed-lint.json`)
```json
{
  "auto_fix": false,
  "min_severity": "info",
  "general": {
    "max_line_length": 100,
    "indent_size": 4,
    "enforce_genz_naming": true
  },
  "rules": {
    "style": {
      "enabled": true,
      "default_severity": "warning"
    }
  },
  "output": {
    "format": "human",
    "use_colors": true
  }
}
```

### Output Formats

#### Human Format (Default)
```
src/main.csd
  warning: 5:12: Line exceeds maximum length of 100 characters [line-length]
    Suggestion: Consider breaking this line into multiple lines
  info: 8:25: Line has trailing whitespace [trailing-whitespace]
    Suggestion: Remove trailing whitespace

Processed 1 files in 15ms: 0 errors, 1 warnings, 1 info (1 auto-fixable)
```

#### JSON Format
```json
{
  "results": [
    {
      "file": "src/main.csd",
      "issues": [
        {
          "rule": "line-length",
          "category": "style",
          "severity": "warning",
          "message": "Line exceeds maximum length",
          "line": 5,
          "column": 12,
          "suggestion": "Consider breaking this line"
        }
      ]
    }
  ],
  "statistics": {
    "files_processed": 1,
    "total_issues": 2,
    "errors": 0,
    "warnings": 1,
    "info": 1,
    "auto_fixable": 1,
    "processing_time_ms": 15
  }
}
```

#### Checkstyle XML Format
```xml
<?xml version="1.0" encoding="UTF-8"?>
<checkstyle version="1.0">
  <file name="src/main.csd">
    <error line="5" column="12" severity="warning" 
           message="Line exceeds maximum length" source="line-length"/>
  </file>
</checkstyle>
```

## Configuration Options

### General Settings
- `max_line_length`: Maximum allowed line length (default: 100)
- `indent_style`: Use "spaces" or "tabs" for indentation
- `indent_size`: Number of spaces per indent level (default: 4)
- `enforce_genz_naming`: Encourage Gen Z naming conventions
- `file_extensions`: File extensions to process (default: ["csd"])
- `ignore_patterns`: Glob patterns to exclude from linting

### Rule Configuration
Each rule category can be configured:
- `enabled`: Whether the category is active
- `default_severity`: Default severity for rules in the category
- `rule_overrides`: Per-rule severity and parameter overrides

### Output Configuration
- `format`: Output format (human, json, checkstyle, sarif)
- `show_rule_names`: Include rule names in output
- `show_severity`: Display severity levels
- `show_suggestions`: Include fix suggestions
- `use_colors`: Enable colored output
- `max_issues_per_file`: Limit issues displayed per file

## CI/CD Integration

### GitHub Actions
```yaml
name: CURSED Linting
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Build linter
      run: cargo build --bin cursed_lint_new
    - name: Run linter
      run: make cursed-lint-check
```

### Pre-commit Hooks
```bash
# Install git hooks for automatic linting
./scripts/install-git-hooks.sh

# Or manually add to .git/hooks/pre-commit:
#!/bin/sh
make cursed-lint-check
```

## IDE Integration

### Language Server Protocol (LSP)
The linter can be integrated with LSP servers for real-time feedback:

```json
{
  "cursed-lint": {
    "command": "cursed-lint",
    "args": ["--format", "json", "--"],
    "filetypes": ["cursed"]
  }
}
```

### Editor Plugins
- **VS Code**: Configure as external linter
- **Vim/Neovim**: Use with ALE or similar linting plugins
- **Emacs**: Integrate with flycheck
- **IntelliJ**: Configure as external tool

## Performance and Scalability

### Optimization Features
- **Parallel Processing**: Multi-threaded file processing
- **Incremental Analysis**: Skip unchanged files
- **Memory Efficiency**: Streaming analysis for large files
- **Caching**: Rule result caching for repeated analysis

### Benchmarks
- **Small Projects** (<100 files): <1 second
- **Medium Projects** (1000 files): ~10 seconds
- **Large Projects** (10k+ files): ~2 minutes

## Extending the Linter

### Custom Rules
Implement the `LintRule` trait to create custom rules:

```rust
use cursed::linter::rules::{LintRule, RuleCategory, RuleSeverity};

struct MyCustomRule;

impl LintRule for MyCustomRule {
    fn name(&self) -> &'static str { "my-custom-rule" }
    fn category(&self) -> RuleCategory { RuleCategory::Style }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Custom rule description" }
    
    fn check_text(&self, source: &str, _file_name: Option<&str>) -> Result<Vec<LintIssue>, Error> {
        // Implementation
        Ok(Vec::new())
    }
}
```

### Plugin System
The linter supports external plugins through dynamic loading:

```toml
[plugins]
enabled = ["security-rules", "performance-plus"]

[plugins.security-rules]
path = "/path/to/security-rules.so"
config = { strict_mode = true }
```

## Best Practices

### Development Workflow
1. **Initial Setup**: Generate configuration with `make cursed-lint-init`
2. **Regular Linting**: Use `make cursed-lint` during development
3. **Pre-commit**: Install git hooks for automatic checking
4. **CI Integration**: Use `make cursed-lint-check` in build pipeline
5. **Auto-fixing**: Periodically run `make cursed-lint-fix`

### Configuration Management
- Start with default configuration
- Gradually customize rules based on project needs
- Document rule changes in project README
- Use different configurations for different environments
- Keep configuration in version control

### Team Integration
- Establish team coding standards
- Configure consistent rule severities
- Use shared configuration files
- Regular rule review and updates
- Training on CURSED-specific patterns

## Troubleshooting

### Common Issues

#### "Rule not found" Error
- Check rule name spelling with `--list-rules`
- Verify rule category is enabled
- Check configuration file syntax

#### Performance Issues
- Use `--exclude` to skip large generated files
- Enable parallel processing with `--jobs`
- Consider incremental linting for large projects

#### Configuration Not Loading
- Verify file format (TOML/JSON/YAML)
- Check file permissions
- Use absolute paths for includes
- Validate syntax with online validators

#### False Positives
- Adjust rule parameters in configuration
- Use inline comments to disable specific rules
- Report bugs with minimal reproduction cases

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug cursed-lint file.csd

# Verbose output
cursed-lint --verbose --stats file.csd
```

## Contributing

### Adding New Rules
1. Create rule implementation in appropriate category module
2. Add tests with comprehensive coverage
3. Update documentation with examples
4. Add to rule registry in category configuration

### Reporting Issues
- Provide minimal reproduction case
- Include configuration file
- Specify environment details
- Use debug output for investigation

### Development Setup
```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Install development environment
./scripts/setup-dev-environment.sh

# Run tests
make test

# Build linter
cargo build --bin cursed_lint_new
```

The CURSED linter is designed to be a production-ready tool that grows with your project needs while maintaining the unique character of the CURSED programming language.
