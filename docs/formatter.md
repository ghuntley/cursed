# CURSED Formatter (Making Your Code Pretty AF) ✨

The CURSED formatter (`cursed-fmt`) is a code formatting tool that makes your CURSED code look absolutely iconic! No more messy indentation or inconsistent spacing - this tool keeps your code clean and readable, periodt! 💅

## Installation (Getting That Formatter Glow-Up) 🔧

### Option 1: Install from Source (DIY Queen Energy)
```bash
# Clone the repo if you haven't already
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build the formatter
make build-formatter

# Install globally (optional but recommended)
cargo install --path . --bin cursed-fmt
```

### Option 2: Download Binary (Quick and Easy)
```bash
# Download from releases (when available)
curl -L https://github.com/ghuntley/cursed/releases/latest/download/cursed-fmt-linux -o cursed-fmt
chmod +x cursed-fmt
```

## Basic Usage (How to Slay with Formatting) 💫

### Format a Single File
```bash
# Format and print to stdout (preview mode)
cursed-fmt main.csd

# Format in-place (actually modify the file)
cursed-fmt -w main.csd

# Format multiple files
cursed-fmt -w src/*.csd
```

### Format Entire Directory
```bash
# Format all .csd files recursively
cursed-fmt -w ./src/

# Format with specific patterns
cursed-fmt -w --include="**/*.csd" ./
```

### Check Mode (Validate Without Changing)
```bash
# Check if files need formatting (exit code 1 if changes needed)
cursed-fmt --check src/

# Useful for CI/CD pipelines
cursed-fmt --check --quiet src/
```

## Command Line Options (All the Features, No Cap) 🎮

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `--write` | `-w` | Write result to file instead of stdout | `cursed-fmt -w main.csd` |
| `--check` | `-c` | Check if input is formatted, exit with code 1 if not | `cursed-fmt --check src/` |
| `--diff` | `-d` | Show formatting differences | `cursed-fmt --diff main.csd` |
| `--quiet` | `-q` | Suppress output except errors | `cursed-fmt -w -q src/` |
| `--config` | | Specify config file path | `cursed-fmt --config=.cursed-fmt.toml` |
| `--include` | | File patterns to include | `cursed-fmt --include="**/*.csd"` |
| `--exclude` | | File patterns to exclude | `cursed-fmt --exclude="**/test_*.csd"` |
| `--line-width` | | Override line width setting | `cursed-fmt --line-width=100` |
| `--tab-size` | | Override tab size setting | `cursed-fmt --tab-size=2` |
| `--help` | `-h` | Show help message | `cursed-fmt --help` |
| `--version` | `-V` | Show version info | `cursed-fmt --version` |

## Configuration (Customize Your Aesthetic) ⚙️

The formatter looks for configuration files in this order (first one wins):
1. `.cursed-fmt.toml` in current directory
2. `.cursed-fmt.toml` in parent directories (walking up)
3. `~/.config/cursed/formatter.toml` (global config)
4. Built-in defaults

### Configuration File Format

Create a `.cursed-fmt.toml` file in your project root:

```toml
# .cursed-fmt.toml - Making your code aesthetic since day 1 ✨

[formatting]
# Maximum line width before wrapping (default: 80)
line_width = 100

# Number of spaces per indentation level (default: 4)
tab_size = 4

# Use tabs instead of spaces (default: false)
use_tabs = false

# Add trailing commas where valid (default: true)
trailing_commas = true

# Space before colon in function signatures (default: false)
space_before_colon = false

# Spaces around binary operators (default: true)
spaces_around_operators = true

# Break function arguments to multiple lines (default: "auto")
# Options: "auto", "always", "never"
function_args_layout = "auto"

# Break struct/map literals to multiple lines (default: "auto")
struct_layout = "auto"

# Align consecutive assignments (default: false)
align_assignments = false

# Sort imports alphabetically (default: true)
sort_imports = true

# Group imports by type (default: true)
group_imports = true

[comments]
# Preserve comment formatting (default: true)
preserve_comments = true

# Align inline comments (default: false)
align_inline_comments = false

# Maximum width for comments before wrapping (default: 80)
comment_width = 80

[newlines]
# Blank lines after imports (default: 1)
blank_lines_after_imports = 1

# Blank lines between functions (default: 2)
blank_lines_between_functions = 2

# Blank lines at start/end of blocks (default: 0)
blank_lines_in_blocks = 0

[strings]
# Prefer single quotes when possible (default: false)
prefer_single_quotes = false

# Normalize string escape sequences (default: true)
normalize_escapes = true
```

### Team Configuration Examples

#### Minimal Config (Just the Essentials)
```toml
[formatting]
line_width = 120
tab_size = 2
trailing_commas = true
```

#### Strict Team Config (For When You Want Consistency)
```toml
[formatting]
line_width = 100
tab_size = 4
use_tabs = false
trailing_commas = true
function_args_layout = "always"
struct_layout = "auto"
align_assignments = true
sort_imports = true

[comments]
align_inline_comments = true
comment_width = 100

[newlines]
blank_lines_between_functions = 1
```

## Editor Integration (Making Your IDE Iconic) 🎨

### VSCode Integration

1. Install the CURSED language extension (when available), or
2. Set up custom formatting:

Create `.vscode/settings.json`:
```json
{
  "cursed.formatter.enable": true,
  "cursed.formatter.onSave": true,
  "cursed.formatter.command": "cursed-fmt",
  "cursed.formatter.args": ["-w"],
  "[cursed]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "cursed.formatter"
  }
}
```

### Vim/Neovim Integration

Add to your `.vimrc` or `init.vim`:
```vim
" CURSED formatter integration
autocmd FileType cursed setlocal formatprg=cursed-fmt
autocmd BufWritePre *.csd :silent! %!cursed-fmt

" Or use with vim-autoformat plugin
let g:formatdef_cursed = '"cursed-fmt"'
let g:formatters_cursed = ['cursed']
```

### Emacs Integration

Add to your Emacs config:
```elisp
(defun cursed-format-buffer ()
  "Format the current buffer using cursed-fmt."
  (interactive)
  (shell-command-on-region (point-min) (point-max) "cursed-fmt" t t))

(add-hook 'cursed-mode-hook
          (lambda ()
            (add-hook 'before-save-hook 'cursed-format-buffer nil 'local)))
```

## CI/CD Integration (Keeping Your Pipeline Fresh) 🔄

### GitHub Actions

Create `.github/workflows/format-check.yml`:
```yaml
name: Format Check
on: [push, pull_request]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup CURSED
        run: |
          # Install cursed-fmt (adjust as needed)
          curl -L https://github.com/ghuntley/cursed/releases/latest/download/cursed-fmt-linux -o cursed-fmt
          chmod +x cursed-fmt
          sudo mv cursed-fmt /usr/local/bin/
      
      - name: Check formatting
        run: cursed-fmt --check --quiet src/
      
      - name: Show diff if formatting needed
        if: failure()
        run: cursed-fmt --diff src/
```

### Pre-commit Hook

Create `.pre-commit-config.yaml`:
```yaml
repos:
  - repo: local
    hooks:
      - id: cursed-fmt
        name: Format CURSED code
        entry: cursed-fmt
        args: [--check]
        language: system
        files: \.csd$
        pass_filenames: true
```

Or create a simple git hook in `.git/hooks/pre-commit`:
```bash
#!/bin/bash
# Format CURSED files before commit

# Check if any .csd files are staged
if git diff --cached --name-only | grep -q '\.csd$'; then
    echo "Formatting CURSED files..."
    
    # Format staged .csd files
    git diff --cached --name-only | grep '\.csd$' | xargs cursed-fmt --check
    
    if [ $? -ne 0 ]; then
        echo "❌ Code formatting issues found. Run 'cursed-fmt -w src/' to fix."
        exit 1
    fi
    
    echo "✅ Code formatting looks good!"
fi
```

### Makefile Integration

Add to your `Makefile`:
```makefile
.PHONY: format format-check format-fix

# Check if code is formatted
format-check:
	cursed-fmt --check --quiet src/

# Show formatting differences
format-diff:
	cursed-fmt --diff src/

# Fix formatting issues
format-fix:
	cursed-fmt -w src/

# Alias for convenience
format: format-fix
```

## Troubleshooting (When Things Get Sus) 🔧

### Common Issues

**❌ "cursed-fmt: command not found"**
- Solution: Make sure cursed-fmt is installed and in your PATH
```bash
# Check if installed
which cursed-fmt

# Install if missing
cargo install --path . --bin cursed-fmt
```

**❌ "Permission denied"**
- Solution: Check file permissions
```bash
chmod +x cursed-fmt
```

**❌ "Config file not found"**
- Solution: The formatter works without config (uses defaults)
- Check config file path and permissions
```bash
# Check current config
cursed-fmt --show-config

# Use specific config file
cursed-fmt --config=./my-config.toml main.csd
```

**❌ "Formatting breaks my code"**
- This shouldn't happen! If it does, please file an issue
- Use `--diff` to see what would change before using `--write`
- Back up your code before batch formatting

**❌ "Slow performance on large files"**
- Use `--include`/`--exclude` patterns to limit files processed
- Consider formatting smaller chunks at a time
- Report performance issues with file size details

### Getting Help

```bash
# Show detailed help
cursed-fmt --help

# Check version
cursed-fmt --version

# Show current configuration
cursed-fmt --show-config

# Verbose output for debugging
cursed-fmt --verbose src/
```

### Reporting Issues

If you find bugs or have feature requests:
1. Check existing issues first
2. Include example code that demonstrates the problem
3. Include your configuration file
4. Mention your OS and cursed-fmt version

## Best Practices (How to Stay Iconic) 💎

### For Individuals
- Set up editor integration for format-on-save
- Use consistent configuration across projects
- Format before committing (use pre-commit hooks)

### For Teams
- Agree on a shared `.cursed-fmt.toml` configuration
- Include formatter checks in CI/CD pipeline
- Document your formatting standards in project README
- Consider running formatter in "fix" mode in CI for auto-formatting PRs

### For Open Source Projects
- Include a `.cursed-fmt.toml` in your repository root
- Add formatting checks to CI/CD
- Mention formatting requirements in CONTRIBUTING.md
- Consider providing an EditorConfig file for consistency

## Advanced Usage (For the Power Users) ⚡

### Custom Formatting Rules

While the formatter comes with sensible defaults, you can create custom rules:

```bash
# Format with temporary overrides
cursed-fmt --line-width=120 --tab-size=2 main.csd

# Use environment variables for one-off changes
CURSED_FMT_LINE_WIDTH=100 cursed-fmt main.csd
```

### Integration with Other Tools

```bash
# Use with find for complex file selection
find src/ -name "*.csd" -not -path "*/generated/*" | xargs cursed-fmt -w

# Combine with git for formatting only changed files
git diff --name-only --cached | grep '\.csd$' | xargs cursed-fmt -w

# Format specific functions/ranges (future feature)
cursed-fmt --range=10:50 main.csd
```

### Performance Optimization

```bash
# Parallel processing for large codebases
find src/ -name "*.csd" | xargs -P 4 -I {} cursed-fmt -w {}

# Skip files that are already formatted (faster CI)
cursed-fmt --check --fail-fast src/
```

That's the tea on the CURSED formatter! Your code is about to look absolutely immaculate! ✨

For more advanced features and API documentation, check out the [Developer Guide](formatter-dev.md).
