# CURSED Language Linter

The CURSED linter enforces code style, detects potential issues, and ensures proper usage of Gen Z slang keywords in CURSED programs.

## Features

### Core Linting Capabilities

- **Gen Z Slang Enforcement**: Detects usage of Go-style keywords and suggests CURSED equivalents
- **Code Style Checking**: Line length, indentation, whitespace, and formatting issues
- **Naming Conventions**: Identifier naming patterns and consistency
- **Complexity Analysis**: Function complexity, parameter counts, and nesting depth
- **Correctness Checks**: Unused variables, imports, and other logical issues
- **Security Awareness**: Basic security anti-patterns
- **Performance Hints**: Suggestions for better performance

### Lint Categories

1. **Style**: Code formatting and visual consistency
2. **Naming**: Identifier naming conventions
3. **GenZSlang**: Proper CURSED keyword usage vs Go-style
4. **Performance**: Performance-related suggestions
5. **Complexity**: Code complexity and maintainability
6. **Deprecated**: Usage of deprecated language features
7. **Security**: Security-related concerns
8. **Correctness**: Logical correctness and unused code

### Severity Levels

- **Error**: Code that will not compile or is fundamentally broken
- **Warning**: Code that compiles but violates best practices
- **Suggestion**: Minor style improvements
- **Info**: Informational messages

## Usage

### Command Line Interface

```bash
# Lint a single file
cursed-lint src/main.csd

# Lint multiple files
cursed-lint src/*.csd

# Lint entire directory recursively
cursed-lint src/

# Use specific configuration
cursed-lint --config .cursed-lint.toml src/

# Output formats
cursed-lint --format json src/main.csd      # JSON output
cursed-lint --format compact src/main.csd   # Compact format
cursed-lint --format checkstyle src/        # Checkstyle XML

# Severity filtering
cursed-lint --quiet src/        # Only errors and warnings
cursed-lint --verbose src/      # Include suggestions and info

# Configuration options
cursed-lint --strict src/                    # Strict mode
cursed-lint --relaxed src/                   # Relaxed mode
cursed-lint --max-line-length 80 src/       # Custom line length
cursed-lint --disable line_too_long src/    # Disable specific rules

# Category filtering
cursed-lint --categories style,naming src/  # Only specific categories

# Statistics and documentation
cursed-lint --stats src/                     # Show statistics
cursed-lint --show-rules                     # List all rules
```

### Configuration File

Create a `.cursed-lint.toml` file in your project root:

```toml
[general]
strict_mode = false
max_line_length = 100
max_function_parameters = 6

[disabled_rules]
rules = ["trailing_whitespace"]

[categories]
style = true
gen_z_slang = true
```

### Programmatic API

```rust
use cursed::tools::linter::{CursedLinter, LinterConfig, LintSeverity};

let config = LinterConfig::strict();
let mut linter = CursedLinter::new(config);

let source = r#"
vibe my_package

slay main() {
    sus x = 42
    print(x)
}
"#;

let results = linter.lint(source)?;

for result in results {
    println!("{}: {} at {}:{}", 
             result.severity, 
             result.message, 
             result.line, 
             result.column);
}
```

## Linting Rules

### Style Rules

| Rule ID | Description | Severity |
|---------|-------------|----------|
| `line_too_long` | Lines exceeding maximum length | Warning |
| `trailing_whitespace` | Trailing spaces or tabs | Suggestion |
| `mixed_indentation` | Mixed tabs and spaces | Warning |
| `long_comment` | Overly long comments | Suggestion |
| `long_string_literal` | Very long string literals | Suggestion |
| `unnecessary_escape` | Unnecessary escape sequences | Suggestion |

### Gen Z Slang Rules

| Rule ID | Description | Severity |
|---------|-------------|----------|
| `go_style_keyword` | Go keywords instead of CURSED | Error |
| `go_style_comment` | `//` instead of `fr fr` | Warning |
| `go_style_block_comment` | `/* */` instead of `no cap` ... `on god` | Warning |
| `go_style_channel` | `chan` instead of `dm` | Warning |

### Naming Rules

| Rule ID | Description | Severity |
|---------|-------------|----------|
| `single_letter_variable` | Non-descriptive single-letter names | Suggestion |
| `identifier_too_long` | Overly long identifiers | Warning |
| `generic_function_name` | Generic function names like `doSomething` | Suggestion |
| `mixed_naming_style` | Mixed camelCase and snake_case | Warning |
| `non_ascii_identifier` | Non-ASCII characters in identifiers | Suggestion |
| `invalid_package_name` | Invalid package name characters | Error |
| `empty_package_name` | Empty package name | Error |

### Complexity Rules

| Rule ID | Description | Severity |
|---------|-------------|----------|
| `too_many_parameters` | Functions with too many parameters | Warning |
| `deep_nesting` | Deeply nested code blocks | Warning |
| `function_too_long` | Functions exceeding length limit | Warning |
| `cognitive_complexity` | High cognitive complexity | Warning |

### Correctness Rules

| Rule ID | Description | Severity |
|---------|-------------|----------|
| `unused_variable` | Variables declared but not used | Warning |
| `unused_import` | Imports that are not used | Warning |
| `empty_import_path` | Empty import paths | Error |
| `parse_error` | Syntax errors | Error |

## Configuration

### Presets

#### Strict Mode
- Shorter line length (80 characters)
- Fewer function parameters (4 max)
- Required documentation
- All checks enabled

#### Relaxed Mode
- Longer line length (120 characters)
- More function parameters (10 max)
- Optional documentation
- Some style checks disabled

#### Minimal Mode
- Only correctness checks
- Style rules disabled
- Focus on compilation issues

### Custom Rules

You can disable specific rules or entire categories:

```bash
# Disable specific rules
cursed-lint --disable line_too_long,trailing_whitespace src/

# Check only specific categories
cursed-lint --categories correctness,gen_z_slang src/
```

## Integration

### Editor Integration

The linter outputs standard formats that can be integrated with editors:

- **JSON**: For programmatic processing
- **Checkstyle XML**: For Jenkins and other CI tools
- **Compact**: For editor quickfix lists

### CI/CD Integration

```bash
# Exit with error code only on errors (not warnings)
cursed-lint --error-only src/

# Generate XML report for Jenkins
cursed-lint --format checkstyle src/ > lint-results.xml

# Show only summary statistics
cursed-lint --stats --quiet src/
```

### VSCode Integration

Example `.vscode/tasks.json`:

```json
{
    "tasks": [
        {
            "label": "CURSED Lint",
            "type": "shell",
            "command": "cursed-lint",
            "args": ["--format", "compact", "${file}"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "silent",
                "focus": false,
                "panel": "shared"
            },
            "problemMatcher": {
                "owner": "cursed-lint",
                "fileLocation": "absolute",
                "pattern": {
                    "regexp": "^(.+):(\\d+):(\\d+):\\s+(error|warning|suggestion|info)\\s+\\[(.+)\\]\\s+(.+)$",
                    "file": 1,
                    "line": 2,
                    "column": 3,
                    "severity": 4,
                    "code": 5,
                    "message": 6
                }
            }
        }
    ]
}
```

## Examples

### Basic Usage

```bash
# Check a simple CURSED file
$ cursed-lint examples/hello.csd

examples/hello.csd
  12:81 warning [line_too_long] Line length 95 exceeds maximum of 80 (style)
    suggestion: Consider breaking this line into multiple lines
  15:1 suggestion [single_letter_variable] Variable 'x' has a non-descriptive single-letter name (naming)
    suggestion: Use a more descriptive variable name
```

### JSON Output

```bash
$ cursed-lint --format json examples/hello.csd
{
  "file": "examples/hello.csd",
  "issues": [
    {
      "rule_id": "line_too_long",
      "severity": "Warning",
      "category": "Style", 
      "message": "Line length 95 exceeds maximum of 80",
      "line": 12,
      "column": 81,
      "suggestion": "Consider breaking this line into multiple lines"
    }
  ]
}
```

### Configuration Example

```toml
# .cursed-lint.toml
[general]
max_line_length = 120
strict_mode = false

[disabled_rules]
rules = ["single_letter_variable"]

[categories]
style = true
gen_z_slang = true
naming = false
```

## Advanced Features

### Custom Rule Development

The linter architecture supports custom rules. Rules are implemented as functions that analyze the AST and source code to detect patterns.

### Performance

The linter is designed for speed:
- Parallel file processing
- Incremental analysis
- Efficient AST traversal
- Configurable rule sets

### Error Recovery

The linter continues analysis even when parse errors occur, providing as much feedback as possible about code quality issues.

## Best Practices

1. **Start with Relaxed Mode**: Begin with relaxed settings and gradually increase strictness
2. **Project-Specific Configuration**: Use `.cursed-lint.toml` for consistent team settings
3. **CI Integration**: Run linting in continuous integration pipelines
4. **Editor Integration**: Set up real-time linting in your development environment
5. **Gradual Adoption**: Disable problematic rules initially and enable them over time
6. **Team Standards**: Discuss and agree on team linting standards

## Troubleshooting

### Common Issues

1. **Too Many Warnings**: Start with relaxed mode or disable specific rules
2. **Parse Errors**: Fix syntax errors before addressing style issues
3. **Performance**: Use `--categories` to limit rule sets for large codebases
4. **False Positives**: Disable specific rules that don't apply to your project

### Getting Help

```bash
# Show all available rules
cursed-lint --show-rules

# Get help on command line options
cursed-lint --help

# Check configuration
cursed-lint --config-check .cursed-lint.toml
```
