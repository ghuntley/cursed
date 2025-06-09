# CURSED Formatter Integration

This document describes how the CURSED formatter is integrated into the project's build system and development workflow.

## Quick Start

### Setup Development Environment
```bash
# Set up complete development environment
./scripts/setup-dev-environment.sh

# Install pre-commit hooks for automatic formatting
./scripts/install-git-hooks.sh
```

### Basic Formatting Commands
```bash
# Format all CURSED (.csd) files
make fmt

# Check if files are properly formatted (for CI)
make fmt-check

# Show what changes would be made without applying them
make fmt-diff

# Format Rust (.rs) files
make fmt-fix

# Get help with formatting commands
make fmt-help
```

## Integration Components

### 1. Build System Integration

#### Cargo.toml
- Added `cursed-fmt` binary target
- Integrated with existing build process
- No additional dependencies required

#### Makefile
- `make fmt` - Format all CURSED source files
- `make fmt-check` - Check formatting (CI-friendly)
- `make fmt-diff` - Preview formatting changes
- `make fmt-fix` - Format Rust files (existing functionality)
- `make fmt-help` - Show formatting help

### 2. CI/CD Integration

#### GitHub Actions Workflow (`.github/workflows/formatting.yml`)
- **Rust Formatting Job**: Validates Rust code formatting
- **CURSED Formatting Job**: Validates CURSED code formatting
- **Formatting Summary**: Generates comprehensive reports
- **Artifact Upload**: Saves formatting diffs for failed builds

Features:
- Runs on push and pull request
- Caches dependencies for faster builds
- Provides detailed diff output on failures
- Supports both Ubuntu and macOS (configurable)

### 3. Pre-commit Hooks

#### Installation
```bash
./scripts/install-git-hooks.sh
```

#### Features
- Automatically formats code before commits
- Handles both Rust and CURSED files
- Re-stages formatted files
- Runs basic compilation checks
- Provides colored output for better UX
- Backup and restore on formatting errors

#### Hook Behavior
1. Detects staged files (`.rs` and `.csd`)
2. Formats Rust files with `cargo fmt`
3. Builds and runs `cursed-fmt` on CURSED files
4. Re-stages any modified files
5. Runs basic compilation and lint checks
6. Exits with error if files were modified (requires re-commit)

### 4. Configuration

#### Default Configuration (`.cursed_fmt.toml`)
```toml
[general]
max_line_length = 100
indent_size = 4
use_spaces = true

[functions]
brace_style = "same_line"
align_parameters = true

[go_style]
enable_go_conventions = true
align_struct_fields = true

[gen_z_formatting]
preserve_slang_spacing = true
align_method_chains = true
```

#### Customization
- Project-wide settings in `.cursed_fmt.toml`
- File-specific overrides supported
- Ignore patterns for generated files
- Performance tuning options

### 5. Development Scripts

#### `scripts/setup-dev-environment.sh`
Complete development environment setup:
- Builds project and formatter
- Installs git hooks
- Checks formatting status
- Runs basic tests
- Displays useful commands
- Validates configuration

#### `scripts/install-git-hooks.sh`
Git hooks installation:
- Detects existing hooks
- Creates backups
- Installs CURSED pre-commit hook
- Tests installation
- Provides usage documentation

#### `scripts/pre-commit-hook.sh`
Pre-commit hook implementation:
- Multi-language formatting support
- Incremental file processing
- Error handling and recovery
- Colored output
- Performance optimizations

## Development Workflow

### Standard Workflow
1. **Setup**: Run `./scripts/setup-dev-environment.sh`
2. **Code**: Write your CURSED or Rust code
3. **Preview**: Use `make fmt-diff` to see potential changes
4. **Format**: Run `make fmt` to format CURSED files
5. **Commit**: Git hooks automatically format and check code

### CI/CD Workflow
1. **Push/PR**: Triggers formatting checks
2. **Validation**: Both Rust and CURSED files validated
3. **Reports**: Detailed formatting reports generated
4. **Artifacts**: Diff files uploaded for failed checks

### Manual Formatting
```bash
# Format everything
make fmt && make fmt-fix

# Check everything
make fmt-check && make rust-fmt-check

# See what would change
make fmt-diff
```

## File Structure

```
├── .cursed_fmt.toml                    # Formatter configuration
├── .github/workflows/formatting.yml   # CI formatting checks
├── scripts/
│   ├── setup-dev-environment.sh       # Complete dev setup
│   ├── install-git-hooks.sh          # Git hooks installer
│   └── pre-commit-hook.sh             # Pre-commit hook
└── src/bin/cursed_fmt.rs              # Formatter binary
```

## Troubleshooting

### Common Issues

#### "cursed-fmt not found"
```bash
# Build the formatter
cargo build --bin cursed-fmt
```

#### "Hook not executable"
```bash
# Re-install hooks
./scripts/install-git-hooks.sh
```

#### "Formatting check failed in CI"
```bash
# Check locally
make fmt-check

# See differences
make fmt-diff

# Apply formatting
make fmt
```

### Skip Formatting
```bash
# Skip pre-commit hook
git commit --no-verify

# Skip specific files in .cursed_fmt.toml
ignore_patterns = ["path/to/file.csd"]
```

## Performance Considerations

### Large Codebases
- Formatter uses parallel processing
- AST caching for repeated operations
- Configurable file size thresholds

### CI Optimization
- Dependency caching
- Conditional job execution
- Artifact compression

### Development Speed
- Incremental formatting in pre-commit hooks
- Fast diff preview mode
- Background building of formatter

## Best Practices

### Project Standards
1. Always run formatter before committing
2. Use `make fmt-diff` to preview changes
3. Keep `.cursed_fmt.toml` in version control
4. Document any project-specific formatting rules

### Team Development
1. Set up pre-commit hooks for all developers
2. Include formatting checks in code review
3. Use consistent configuration across environments
4. Automate formatting in CI/CD pipeline

### Configuration Management
1. Use project-wide configuration file
2. Document any custom formatting rules
3. Review configuration changes in PRs
4. Test configuration changes thoroughly

## Future Enhancements

### Planned Features
- IDE integration plugins
- Advanced semantic formatting
- Custom rule development API
- Integration with more editors

### Performance Improvements
- Incremental formatting
- Better caching strategies
- Parallel processing optimizations
- Memory usage improvements

This integration provides a comprehensive, production-ready formatting solution that seamlessly fits into the CURSED development workflow while maintaining high code quality standards.
