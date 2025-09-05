# CURSED Formatter Examples 📚

This directory contains examples demonstrating the CURSED code formatter (`cursed-fmt`) in action.

## Files Overview

### 📄 Code Examples

- **[`unformatted.💀`](unformatted.💀)** - Example of poorly formatted CURSED code
- **[`formatted.💀`](formatted.💀)** - The same code after formatting with default settings
- **[`complex-example.💀`](complex-example.💀)** - Complex CURSED code showing various formatting scenarios

### ⚙️ Configuration Examples

- **[`config-examples/.cursed-fmt.toml`](config-examples/.cursed-fmt.toml)** - Complete configuration with all options
- **[`config-examples/minimal.toml`](config-examples/minimal.toml)** - Minimal configuration for quick setup
- **[`config-examples/strict-team.toml`](config-examples/strict-team.toml)** - Strict team configuration for consistency
- **[`config-examples/compact.toml`](config-examples/compact.toml)** - Compact configuration for space-efficient code

### 🔧 Integration Examples

- **[`integration/vscode-settings.json`](integration/vscode-settings.json)** - VSCode settings for CURSED formatter
- **[`integration/format.sh`](integration/format.sh)** - Shell script for batch formatting
- **[`integration/pre-commit`](integration/pre-commit)** - Git pre-commit hook for automatic formatting
- **[`integration/.github-workflows-format.yml`](integration/.github-workflows-format.yml)** - GitHub Actions workflow

## Quick Start Examples

### 1. Format a Single File

```bash
# Preview formatting changes
cursed-fmt unformatted.💀

# Apply formatting
cursed-fmt -w unformatted.💀
```

### 2. Compare Before and After

```bash
# Show the difference
cursed-fmt --diff unformatted.💀

# Compare with the formatted version
diff unformatted.💀 formatted.💀
```

### 3. Use Different Configurations

```bash
# Use minimal configuration
cursed-fmt --config=config-examples/minimal.toml unformatted.💀

# Use strict team configuration
cursed-fmt --config=config-examples/strict-team.toml unformatted.💀

# Use compact configuration
cursed-fmt --config=config-examples/compact.toml unformatted.💀
```

### 4. Format All Examples

```bash
# Check formatting of all examples
cursed-fmt --check *.💀

# Format all CURSED files in this directory
cursed-fmt -w *.💀
```

## Integration Examples

### Setup VSCode Integration

1. Copy the VSCode settings:
   ```bash
   cp integration/vscode-settings.json .vscode/settings.json
   ```

2. Install CURSED language extension (when available)

3. Enjoy automatic formatting on save! ✨

### Setup Git Pre-commit Hook

```bash
# Copy the pre-commit hook
cp integration/pre-commit .git/hooks/

# Make it executable
chmod +x .git/hooks/pre-commit

# Now formatting will be checked on every commit
```

### Setup GitHub Actions

```bash
# Copy the workflow file
mkdir -p .github/workflows
cp integration/.github-workflows-format.yml .github/workflows/format.yml

# Commit and push to enable automated formatting checks
```

### Use the Format Script

```bash
# Make the script executable
chmod +x integration/format.sh

# Check formatting
./integration/format.sh --check

# Fix formatting issues
./integration/format.sh

# Use with custom config
./integration/format.sh --config=config-examples/strict-team.toml
```

## Configuration Examples Explained

### Minimal Configuration
Perfect for getting started quickly. Sets basic line width and tab size:

```toml
[formatting]
line_width = 120
tab_size = 2
trailing_commas = true
sort_imports = true
```

### Strict Team Configuration
For teams that want maximum consistency and automated formatting:

```toml
[formatting]
line_width = 100
tab_size = 4
function_args_layout = "always"
align_assignments = true
# ... more strict rules
```

### Compact Configuration
For projects where screen space is limited:

```toml
[formatting]
line_width = 120
tab_size = 2
trailing_commas = false
function_args_layout = "never"
# ... more compact settings
```

## Testing Your Configuration

### 1. Preview Changes
```bash
# See what would change with your config
cursed-fmt --config=your-config.toml --diff complex-example.💀
```

### 2. Validate Configuration
```bash
# Check if your config file is valid
cursed-fmt --validate-config --config=your-config.toml
```

### 3. Test on Real Code
```bash
# Format a test file with your config
cp unformatted.💀 test-formatting.💀
cursed-fmt --config=your-config.toml -w test-formatting.💀

# Compare results
diff unformatted.💀 test-formatting.💀
```

## Creating Your Own Examples

### 1. Before/After Examples

Create pairs of files showing formatting improvements:

```bash
# Create unformatted version
cat > my-example-unformatted.💀 << 'EOF'
sus messy=map[string]int{"a":1,"b":2}
slay messyFunction(x int,y string,z bool)int{vibe x+len(y)}
EOF

# Format it
cursed-fmt -w my-example-unformatted.💀
mv my-example-unformatted.💀 my-example-formatted.💀
```

### 2. Configuration Testing

Test different configurations on the same code:

```bash
# Test with different configs
cursed-fmt --config=config-examples/minimal.toml my-code.💀 > minimal-result.💀
cursed-fmt --config=config-examples/strict-team.toml my-code.💀 > strict-result.💀
cursed-fmt --config=config-examples/compact.toml my-code.💀 > compact-result.💀

# Compare results
diff minimal-result.💀 strict-result.💀
```

## Common Use Cases

### 1. Migrate Existing Codebase

```bash
# Format entire codebase with specific config
find src/ -name "*.💀" -exec cursed-fmt --config=config-examples/strict-team.toml -w {} \;

# Or use the format script
./integration/format.sh --config=config-examples/strict-team.toml src/
```

### 2. Set Up Team Standards

```bash
# Copy team config to project root
cp config-examples/strict-team.toml .cursed-fmt.toml

# Format all code with team standards
cursed-fmt -w src/

# Add to git
git add .cursed-fmt.toml src/
git commit -m "Add CURSED formatter config and format codebase"
```

### 3. CI/CD Integration

```bash
# Check formatting in CI
cursed-fmt --check --quiet src/

# Auto-fix formatting (for non-main branches)
cursed-fmt -w src/
git add src/
git commit -m "Auto-fix formatting [skip ci]"
```

## Tips and Tricks

### 1. Format Only Changed Files

```bash
# Format only files changed in current branch
git diff --name-only main..HEAD | grep '\.💀$' | xargs cursed-fmt -w
```

### 2. Exclude Specific Files

```bash
# Format all except generated files
cursed-fmt -w --exclude='**/generated/*' src/
```

### 3. Performance for Large Codebases

```bash
# Parallel formatting
find src/ -name "*.💀" | xargs -P 4 -I {} cursed-fmt -w {}
```

### 4. Integration with Other Tools

```bash
# Combine with linting
cursed-fmt -w src/ && cursed-lint src/

# Format before running tests
cursed-fmt -w src/ && make test
```

## Getting Help

If you have questions about the formatter examples:

1. Check the [main formatter documentation](../docs/formatter.md)
2. Look at the [configuration reference](../docs/formatter-config.md)
3. Try the examples in this directory
4. Open an issue if you find problems

Happy formatting! Your code is about to look absolutely iconic! ✨
