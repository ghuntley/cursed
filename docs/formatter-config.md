# CURSED Formatter Configuration Reference 📋

Complete reference for all CURSED formatter configuration options.

## Configuration File Locations

The formatter searches for configuration files in this order:

1. `.cursed-fmt.toml` in current directory (highest priority)
2. `.cursed-fmt.toml` in parent directories (walking up the tree)
3. `~/.config/cursed/formatter.toml` (user global config)
4. Built-in defaults (lowest priority)

## Configuration File Format

Configuration files use TOML format with the following sections:

```toml
[formatting]
# Core formatting options

[comments]
# Comment formatting options

[newlines]
# Newline and spacing options

[strings]
# String literal formatting options
```

## Formatting Section

### Basic Options

#### `line_width`
- **Type**: `integer`
- **Default**: `80`
- **Description**: Maximum line width before wrapping
- **Example**: 
  ```toml
  line_width = 120
  ```

#### `tab_size`
- **Type**: `integer`
- **Default**: `4`
- **Description**: Number of spaces per indentation level
- **Example**: 
  ```toml
  tab_size = 2
  ```

#### `use_tabs`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Use tabs instead of spaces for indentation
- **Example**: 
  ```toml
  use_tabs = true
  ```

#### `trailing_commas`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Add trailing commas where syntactically valid
- **Example**: 
  ```toml
  trailing_commas = false
  ```

### Spacing Options

#### `space_before_colon`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Add space before colons in struct/map literals
- **Example**: 
  ```toml
  space_before_colon = true
  ```
  **Result**:
  ```cursed
  # false (default)
  sus person = Person{name: "Alice", age: 30}
  
  # true
  sus person = Person{name : "Alice", age : 30}
  ```

#### `spaces_around_operators`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Add spaces around binary operators
- **Example**: 
  ```toml
  spaces_around_operators = false
  ```
  **Result**:
  ```cursed
  # true (default)
  sus result = a + b * c
  
  # false
  sus result = a+b*c
  ```

### Layout Options

#### `function_args_layout`
- **Type**: `string`
- **Default**: `"auto"`
- **Options**: `"auto"`, `"always"`, `"never"`
- **Description**: When to break function arguments to multiple lines
- **Example**: 
  ```toml
  function_args_layout = "always"
  ```
  **Result**:
  ```cursed
  # "auto" - breaks when line is too long
  slay myFunction(arg1 string, arg2 int, arg3 bool) {}
  
  # "always" - always breaks
  slay myFunction(
      arg1 string,
      arg2 int,
      arg3 bool,
  ) {}
  
  # "never" - never breaks (may exceed line width)
  slay myFunction(arg1 string, arg2 int, arg3 bool, arg4 interface{}) {}
  ```

#### `struct_layout`
- **Type**: `string`
- **Default**: `"auto"`
- **Options**: `"auto"`, `"always"`, `"never"`
- **Description**: When to break struct/map literals to multiple lines
- **Example**: 
  ```toml
  struct_layout = "always"
  ```

#### `break_method_chains`
- **Type**: `string`
- **Default**: `"auto"`
- **Options**: `"auto"`, `"always"`, `"never"`
- **Description**: When to break chained method calls
- **Example**: 
  ```toml
  break_method_chains = "always"
  ```
  **Result**:
  ```cursed
  # "auto" - breaks when line is too long
  data.transform().filter().collect()
  
  # "always" - always breaks
  data
      .transform()
      .filter()
      .collect()
  ```

### Alignment Options

#### `align_assignments`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Align consecutive variable assignments
- **Example**: 
  ```toml
  align_assignments = true
  ```
  **Result**:
  ```cursed
  # false (default)
  sus name = "Alice"
  sus age = 30
  sus isActive = true
  
  # true
  sus name     = "Alice"
  sus age      = 30
  sus isActive = true
  ```

#### `align_struct_fields`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Align struct field colons vertically
- **Example**: 
  ```toml
  align_struct_fields = true
  ```
  **Result**:
  ```cursed
  # false (default)
  sus person = Person{
      name: "Alice",
      age: 30,
      email: "alice@example.com",
  }
  
  # true
  sus person = Person{
      name : "Alice",
      age  : 30,
      email: "alice@example.com",
  }
  ```

### Import Options

#### `sort_imports`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Sort import statements alphabetically
- **Example**: 
  ```toml
  sort_imports = false
  ```

#### `group_imports`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Group imports by type (standard library, third party, local)
- **Example**: 
  ```toml
  group_imports = false
  ```

## Comments Section

#### `preserve_comments`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Preserve original comment formatting
- **Example**: 
  ```toml
  preserve_comments = false
  ```

#### `align_inline_comments`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Align inline comments to the same column
- **Example**: 
  ```toml
  align_inline_comments = true
  ```
  **Result**:
  ```cursed
  # false (default)
  sus name = "Alice"    fr fr Person's name
  sus age = 30 fr fr Person's age
  
  # true
  sus name = "Alice"    fr fr Person's name
  sus age = 30          fr fr Person's age
  ```

#### `comment_width`
- **Type**: `integer`
- **Default**: `80`
- **Description**: Maximum width for comments before wrapping
- **Example**: 
  ```toml
  comment_width = 100
  ```

#### `normalize_comment_spacing`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Normalize spacing in comments
- **Example**: 
  ```toml
  normalize_comment_spacing = false
  ```

## Newlines Section

#### `blank_lines_after_imports`
- **Type**: `integer`
- **Default**: `1`
- **Description**: Number of blank lines after import statements
- **Example**: 
  ```toml
  blank_lines_after_imports = 2
  ```

#### `blank_lines_between_functions`
- **Type**: `integer`
- **Default**: `2`
- **Description**: Number of blank lines between function definitions
- **Example**: 
  ```toml
  blank_lines_between_functions = 1
  ```

#### `blank_lines_in_blocks`
- **Type**: `integer`
- **Default**: `0`
- **Description**: Number of blank lines at start/end of code blocks
- **Example**: 
  ```toml
  blank_lines_in_blocks = 1
  ```
  **Result**:
  ```cursed
  # 0 (default)
  slay myFunction() {
      sus x = 42
      vibe x
  }
  
  # 1
  slay myFunction() {
  
      sus x = 42
      vibe x
  
  }
  ```

#### `max_blank_lines`
- **Type**: `integer`
- **Default**: `2`
- **Description**: Maximum consecutive blank lines to preserve
- **Example**: 
  ```toml
  max_blank_lines = 1
  ```

## Strings Section

#### `prefer_single_quotes`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Use single quotes when possible
- **Example**: 
  ```toml
  prefer_single_quotes = true
  ```
  **Result**:
  ```cursed
  # false (default)
  sus message = "Hello, world!"
  
  # true (when no single quotes in string)
  sus message = 'Hello, world!'
  ```

#### `normalize_escapes`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Normalize string escape sequences
- **Example**: 
  ```toml
  normalize_escapes = false
  ```

#### `break_long_strings`
- **Type**: `boolean`
- **Default**: `true`
- **Description**: Break long string literals across multiple lines
- **Example**: 
  ```toml
  break_long_strings = false
  ```

#### `prefer_raw_strings`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: Use raw strings when beneficial
- **Example**: 
  ```toml
  prefer_raw_strings = true
  ```

## Configuration Examples

### Minimal Configuration
```toml
[formatting]
line_width = 120
tab_size = 2
```

### Team Configuration
```toml
[formatting]
line_width = 100
tab_size = 4
trailing_commas = true
function_args_layout = "auto"
align_assignments = true
sort_imports = true

[comments]
align_inline_comments = true

[newlines]
blank_lines_between_functions = 1
```

### Compact Configuration
```toml
[formatting]
line_width = 120
tab_size = 2
trailing_commas = false
function_args_layout = "never"
struct_layout = "never"

[newlines]
blank_lines_after_imports = 0
blank_lines_between_functions = 1
max_blank_lines = 1
```

### Strict Configuration
```toml
[formatting]
line_width = 100
tab_size = 4
use_tabs = false
trailing_commas = true
function_args_layout = "always"
struct_layout = "always"
align_assignments = true
align_struct_fields = true
sort_imports = true
group_imports = true

[comments]
align_inline_comments = true
comment_width = 100
normalize_comment_spacing = true

[newlines]
blank_lines_after_imports = 2
blank_lines_between_functions = 2
blank_lines_in_blocks = 0
max_blank_lines = 1

[strings]
normalize_escapes = true
break_long_strings = true
```

## Environment Variables

You can override configuration options using environment variables:

- `CURSED_FMT_LINE_WIDTH`: Override line width
- `CURSED_FMT_TAB_SIZE`: Override tab size
- `CURSED_FMT_CONFIG`: Specify config file path

Example:
```bash
CURSED_FMT_LINE_WIDTH=120 cursed-fmt main.csd
```

## Command Line Overrides

Some options can be overridden from the command line:

```bash
# Override line width
cursed-fmt --line-width=120 main.csd

# Override tab size
cursed-fmt --tab-size=2 main.csd

# Use specific config file
cursed-fmt --config=team.toml main.csd
```

## Configuration Validation

The formatter validates configuration files and will report errors for:

- Invalid TOML syntax
- Unknown configuration keys
- Invalid values for known keys
- Out-of-range numeric values

Use `cursed-fmt --validate-config` to check your configuration file without formatting any code.

This configuration system gives you complete control over how your CURSED code is formatted! ✨
