# Enhanced String Library (string_enhanced)

A comprehensive, compiler-optimized string manipulation library for CURSED programs, providing advanced string operations, scanning, formatting, and code generation utilities.

## Overview

The `string_enhanced` module extends basic string functionality with specialized operations for compiler development, lexical analysis, code generation, and advanced text processing. Built entirely in pure CURSED with runtime optimizations.

## Features

- **String Interning**: Efficient string storage and deduplication
- **String Scanning**: Lexical analysis and parsing utilities
- **Code Formatting**: CURSED code generation helpers
- **Identifier Validation**: Keyword checking and naming validation
- **String Escaping**: Proper handling of string literals
- **Path Manipulation**: Module path resolution utilities
- **Case Conversion**: Snake case and Pascal case transformation
- **Indentation Management**: Code generation formatting tools

## API Reference

### String Interning

String interning provides efficient storage and comparison of frequently used strings.

#### `StringIntern_new() StringIntern`
Create a new string intern pool.

```cursed
yeet "string_enhanced"

sus intern StringIntern = StringIntern_new()
```

#### `StringIntern_intern(intern StringIntern, str tea) (normie, StringIntern)`
Add a string to the intern pool and get its ID.

```cursed
yeet "string_enhanced"

sus intern StringIntern = StringIntern_new()
sus (id, updated_intern) = StringIntern_intern(intern, "hello")
# Returns: (0, updated_intern) for first string
sus (id2, updated_intern2) = StringIntern_intern(updated_intern, "hello")
# Returns: (0, updated_intern2) - same ID for duplicate string
```

#### `StringIntern_get(intern StringIntern, id normie) tea`
Retrieve a string from the intern pool by ID.

```cursed
yeet "string_enhanced"

sus intern StringIntern = StringIntern_new()
sus (id, intern2) = StringIntern_intern(intern, "world")
sus retrieved tea = StringIntern_get(intern2, id)
# Returns: "world"
```

### String Scanning

Advanced string scanning for lexical analysis and parsing operations.

#### `StringScanner_new(source tea) StringScanner`
Create a new string scanner for the given source text.

```cursed
yeet "string_enhanced"

sus scanner StringScanner = StringScanner_new("hello world")
```

#### `StringScanner_current_char(scanner StringScanner) sip`
Get the character at the current scanner position.

```cursed
yeet "string_enhanced"

sus scanner StringScanner = StringScanner_new("hello")
sus ch sip = StringScanner_current_char(scanner)
# Returns: 'h'
```

#### `StringScanner_advance(scanner StringScanner) StringScanner`
Move the scanner to the next character, updating line/column tracking.

```cursed
yeet "string_enhanced"

sus scanner StringScanner = StringScanner_new("hi\nthere")
scanner = StringScanner_advance(scanner)  # 'i'
scanner = StringScanner_advance(scanner)  # '\n' (line=2, column=1)
scanner = StringScanner_advance(scanner)  # 't'
```

#### `StringScanner_skip_whitespace(scanner StringScanner) StringScanner`
Skip all whitespace characters from current position.

```cursed
yeet "string_enhanced"

sus scanner StringScanner = StringScanner_new("   hello")
scanner = StringScanner_skip_whitespace(scanner)
sus ch sip = StringScanner_current_char(scanner)
# Returns: 'h'
```

#### `StringScanner_read_while(scanner StringScanner, predicate tea) (tea, StringScanner)`
Read characters while they match the given predicate.

```cursed
yeet "string_enhanced"

sus scanner StringScanner = StringScanner_new("hello123")
sus (word, scanner2) = StringScanner_read_while(scanner, "alpha")
# Returns: ("hello", updated_scanner)

sus (number, scanner3) = StringScanner_read_while(scanner2, "digit")
# Returns: ("123", updated_scanner)
```

**Available predicates:**
- `"alpha"` - Alphabetic characters (a-z, A-Z)
- `"digit"` - Numeric characters (0-9)
- `"alnum"` - Alphanumeric characters
- `"ident"` - Identifier characters (alphanumeric + underscore)

### Code Generation Formatting

Functions for generating properly formatted CURSED code.

#### `format_function_signature(name tea, params []tea, return_type tea) tea`
Format a function signature.

```cursed
yeet "string_enhanced"

sus signature tea = format_function_signature("add", ["a normie", "b normie"], "normie")
# Returns: "slay add(a normie, b normie) normie"
```

#### `format_variable_declaration(name tea, type tea, value tea) tea`
Format a variable declaration.

```cursed
yeet "string_enhanced"

sus decl tea = format_variable_declaration("count", "normie", "42")
# Returns: "sus count normie = 42"
```

#### `format_function_call(name tea, args []tea) tea`
Format a function call.

```cursed
yeet "string_enhanced"

sus call tea = format_function_call("print", ["\"hello\"", "world"])
# Returns: "print(\"hello\", world)"
```

#### `format_array_type(element_type tea) tea`
Format an array type declaration.

```cursed
yeet "string_enhanced"

sus array_type tea = format_array_type("normie")
# Returns: "[normie]"
```

### Identifier Validation

Functions for validating CURSED identifiers and keywords.

#### `is_valid_identifier(name tea) lit`
Check if a string is a valid CURSED identifier.

```cursed
yeet "string_enhanced"

sus valid1 lit = is_valid_identifier("my_var")     # Returns: based
sus valid2 lit = is_valid_identifier("_private")   # Returns: based
sus valid3 lit = is_valid_identifier("123invalid") # Returns: cringe
sus valid4 lit = is_valid_identifier("")           # Returns: cringe
```

#### `is_cursed_keyword(word tea) lit`
Check if a string is a reserved CURSED keyword.

```cursed
yeet "string_enhanced"

sus keyword1 lit = is_cursed_keyword("slay")     # Returns: based
sus keyword2 lit = is_cursed_keyword("sus")      # Returns: based
sus keyword3 lit = is_cursed_keyword("my_var")   # Returns: cringe
```

### String Escaping

Functions for handling string literal escaping and unescaping.

#### `escape_string_literal(str tea) tea`
Escape a string for use as a string literal.

```cursed
yeet "string_enhanced"

sus escaped tea = escape_string_literal("hello\nworld")
# Returns: "\"hello\\nworld\""
```

#### `unescape_string_literal(str tea) tea`
Unescape a string literal to its actual content.

```cursed
yeet "string_enhanced"

sus unescaped tea = unescape_string_literal("\"hello\\nworld\"")
# Returns: "hello\nworld"
```

### Path Manipulation

Functions for handling module and file path operations.

#### `normalize_module_path(path tea) tea`
Normalize a module path by removing parent directory references.

```cursed
yeet "string_enhanced"

sus normalized tea = normalize_module_path("../std/collections")
# Returns: "std/collections"
```

#### `module_path_to_file_path(module_path tea) tea`
Convert a module path to a file path.

```cursed
yeet "string_enhanced"

sus file_path tea = module_path_to_file_path("std::collections")
# Returns: "std/collections.csd"
```

#### `file_path_to_module_path(file_path tea) tea`
Convert a file path to a module path.

```cursed
yeet "string_enhanced"

sus module_path tea = file_path_to_module_path("std/collections.csd")
# Returns: "std::collections"
```

### Indentation Management

Utilities for managing code indentation during generation.

#### `IndentationManager_new(indent_string tea) IndentationManager`
Create a new indentation manager.

```cursed
yeet "string_enhanced"

sus manager IndentationManager = IndentationManager_new("    ") # 4 spaces
```

#### `IndentationManager_increase(manager IndentationManager) IndentationManager`
Increase indentation level.

```cursed
yeet "string_enhanced"

sus manager IndentationManager = IndentationManager_new("  ")
manager = IndentationManager_increase(manager)  # Level 1
manager = IndentationManager_increase(manager)  # Level 2
```

#### `IndentationManager_current_indent(manager IndentationManager) tea`
Get the current indentation string.

```cursed
yeet "string_enhanced"

sus manager IndentationManager = IndentationManager_new("  ")
manager = IndentationManager_increase(manager)
sus indent tea = IndentationManager_current_indent(manager)
# Returns: "  " (2 spaces)
```

#### `IndentationManager_indent_line(manager IndentationManager, line tea) tea`
Apply current indentation to a line of code.

```cursed
yeet "string_enhanced"

sus manager IndentationManager = IndentationManager_new("    ")
manager = IndentationManager_increase(manager)
sus indented tea = IndentationManager_indent_line(manager, "sus x normie = 5")
# Returns: "    sus x normie = 5"
```

### Case Conversion

Functions for converting between naming conventions.

#### `to_snake_case(str tea) tea`
Convert string to snake_case.

```cursed
yeet "string_enhanced"

sus snake tea = to_snake_case("MyVariableName")
# Returns: "my_variable_name"
```

#### `to_pascal_case(str tea) tea`
Convert string to PascalCase.

```cursed
yeet "string_enhanced"

sus pascal tea = to_pascal_case("my_variable_name")
# Returns: "MyVariableName"
```

## Usage Examples

### Lexical Analysis

```cursed
yeet "string_enhanced"

slay tokenize_simple(source tea) []tea {
    sus scanner StringScanner = StringScanner_new(source)
    sus tokens []tea = []
    
    bestie !StringScanner_is_at_end(scanner) {
        scanner = StringScanner_skip_whitespace(scanner)
        
        vibes StringScanner_is_at_end(scanner) {
            break
        }
        
        sus ch sip = StringScanner_current_char(scanner)
        vibes StringScanner_is_alpha(ch) {
            sus (identifier, updated_scanner) = StringScanner_read_while(scanner, "ident")
            tokens = append(tokens, identifier)
            scanner = updated_scanner
        } elseif StringScanner_is_digit(ch) {
            sus (number, updated_scanner) = StringScanner_read_while(scanner, "digit")
            tokens = append(tokens, number)
            scanner = updated_scanner
        } nah {
            scanner = StringScanner_advance(scanner)
        }
    }
    
    damn tokens
}

sus tokens []tea = tokenize_simple("hello 123 world")
# Returns: ["hello", "123", "world"]
```

### Code Generation

```cursed
yeet "string_enhanced"

slay generate_function(name tea, params []tea, body []tea) tea {
    sus manager IndentationManager = IndentationManager_new("    ")
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    
    # Function signature
    sus signature tea = format_function_signature(name, params, "lit")
    sb = RuntimeStringBuilder_append(sb, signature)
    sb = RuntimeStringBuilder_append(sb, " {\n")
    
    # Function body
    manager = IndentationManager_increase(manager)
    bestie i := 0; i < len(body); i = i + 1 {
        sus indented_line tea = IndentationManager_indent_line(manager, body[i])
        sb = RuntimeStringBuilder_append(sb, indented_line)
        sb = RuntimeStringBuilder_append(sb, "\n")
    }
    
    # Closing brace
    sb = RuntimeStringBuilder_append(sb, "}")
    
    damn RuntimeStringBuilder_to_string(sb)
}

sus function_code tea = generate_function("test", ["x normie"], ["damn based"])
# Generates properly indented function code
```

### Module Path Resolution

```cursed
yeet "string_enhanced"

slay resolve_module_imports(imports []tea) []tea {
    sus resolved []tea = []
    
    bestie i := 0; i < len(imports); i = i + 1 {
        sus import_path tea = imports[i]
        sus normalized tea = normalize_module_path(import_path)
        sus file_path tea = module_path_to_file_path(normalized)
        resolved = append(resolved, file_path)
    }
    
    damn resolved
}

sus imports []tea = ["../std::collections", "utils::string"]
sus resolved []tea = resolve_module_imports(imports)
# Returns: ["std/collections.csd", "utils/string.csd"]
```

### String Interning System

```cursed
yeet "string_enhanced"

slay build_symbol_table(identifiers []tea) StringIntern {
    sus intern StringIntern = StringIntern_new()
    
    bestie i := 0; i < len(identifiers); i = i + 1 {
        sus (id, updated_intern) = StringIntern_intern(intern, identifiers[i])
        intern = updated_intern
        vibez.spillf("Interned '{}' with ID {}", identifiers[i], id)
    }
    
    damn intern
}

sus symbols []tea = ["variable", "function", "variable", "class"]
sus symbol_table StringIntern = build_symbol_table(symbols)
# Creates efficient symbol storage with deduplication
```

## Testing

The module includes comprehensive test coverage:

```bash
# Run module tests
./cursed-unified stdlib/string_enhanced/test_string_enhanced.csd

# Test specific functionality
echo 'yeet "testz"
yeet "string_enhanced"

test_start("string scanning test")
sus scanner StringScanner = StringScanner_new("hello123")
sus ch sip = StringScanner_current_char(scanner)
assert_eq_string(string_from_char(ch), "h")
print_test_summary()' > test_scanning.csd

./cursed-unified test_scanning.csd
```

## Performance Characteristics

- **String Interning**: O(1) lookup after initial O(n) insertion
- **Scanner Operations**: O(1) character access and advancement
- **Code Generation**: Optimized string building with minimal allocations
- **Path Operations**: Linear time complexity for path transformations

## Dependencies

- **`string`** - Basic string operations
- **`runtime_core`** - Runtime data structures (HashMap, Vec, StringBuilder)
- **`testz`** - Testing framework

## Related Modules

- [`string`](../string/README.md) - Basic string operations
- [`parser`](../parser/README.md) - Parser utilities using string scanning
- [`compiler_core`](../compiler_core/README.md) - Compiler infrastructure
- [`fmt`](../fmt/README.md) - String formatting utilities

## Best Practices

1. **String Interning**: Use for frequently repeated strings to save memory
2. **Scanner Usage**: Always check `is_at_end()` before reading characters
3. **Code Generation**: Use indentation manager for clean output formatting
4. **Identifier Validation**: Always validate identifiers before using in code
5. **Path Handling**: Normalize paths before file system operations

## Version History

- **v1.0.0** - Enhanced string library with compiler optimization features

## Contributing

When contributing to `string_enhanced`:

1. Maintain pure CURSED implementation (no FFI)
2. Add comprehensive test coverage for new functions
3. Follow existing naming conventions and patterns
4. Update documentation for new functionality
5. Ensure performance characteristics are documented

## License

Part of the CURSED programming language stdlib - see main project license.
