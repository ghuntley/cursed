# Error Core Module

Pure CURSED implementation of comprehensive error handling system for compiler self-hosting.

## Overview

The error_core module provides a complete error handling infrastructure essential for the CURSED compiler's self-hosting capability. It implements error creation, propagation, recovery, and reporting - all in pure CURSED without FFI dependencies.

## Key Components

### Error Types and Hierarchy

#### CursedError
Core error type with comprehensive information:
- **Message**: Human-readable error description
- **Category**: Type of error (syntax, type, runtime, internal, warning)
- **Level**: Severity level (warning, error, fatal)
- **Context**: Source location and additional context
- **Timestamp**: When the error occurred

#### ErrorContext
Source location and contextual information:
- **File**: Source file where error occurred
- **Line/Column**: Exact position in source code
- **Additional Context**: Supplementary information

#### Error Categories
- **Syntax Errors**: Parse errors, malformed code
- **Type Errors**: Type system violations
- **Runtime Errors**: Execution-time failures
- **Internal Errors**: Compiler bugs and internal failures
- **Warnings**: Non-fatal issues and suggestions

### Error Collection and Management

#### ErrorCollector
Manages multiple errors during compilation:
- Collects errors from all compilation phases
- Tracks error counts and severity
- Formats comprehensive error reports
- Determines compilation success/failure

#### Error Propagation
Supports CURSED's error handling keywords:
- **`yikes`**: Propagate errors up the call stack
- **`shook`**: Check if error is serious/fatal
- **`fam`**: Attempt error recovery

### Error Recovery System

#### ErrorRecovery
Intelligent error recovery strategies:
- **Syntax Recovery**: Insert missing tokens, remove unexpected tokens
- **Type Recovery**: Suggest type casts, similar types
- **Runtime Recovery**: Add safety checks, graceful degradation

#### Recovery Strategies
- `insert_missing_token` - Add missing semicolons, braces
- `remove_unexpected_token` - Skip malformed tokens
- `insert_type_cast` - Add type conversions
- `suggest_similar_type` - Recommend correct types
- `null_check` - Add null safety
- `bounds_check` - Add array bounds validation

## Core Functions

### Error Creation
```cursed
# Create different types of errors
sus syntax_err CursedError = error_create_syntax("Expected ';'", 10, 15)
sus type_err CursedError = error_create_type("Type mismatch", 20, 5)
sus runtime_err CursedError = error_create_runtime("Null pointer")
sus warning CursedError = error_create_warning("Unused variable", 5, 10)
sus internal_err CursedError = error_create_internal("Compiler bug")
```

### Error Context
```cursed
# Create and enhance error context
sus context ErrorContext = error_context_new("main.💀", 42, 10)
sus enhanced ErrorContext = error_context_add_context(context, "in function main")
sus location tea = error_context_format_location(enhanced)
```

### Error Collection
```cursed
# Collect and manage multiple errors
sus collector ErrorCollector = error_collector_new()
error_collector_add_error(collector, syntax_err)
error_collector_add_error(collector, type_err)

lowkey error_collector_has_errors(collector) {
    sus summary tea = error_collector_format_all_errors(collector)
    vibez.spill(summary)
}
```

### Error Recovery
```cursed
# Intelligent error recovery
sus recovery ErrorRecovery = error_recovery_new()
sus strategy tea = error_recovery_suggest_recovery(recovery, syntax_err)
sus recovered lit = error_recovery_apply_recovery(recovery, strategy)
```

## Compiler Integration

### Compilation Error Handling
```cursed
# Use in compiler phases
sus handler ErrorCollector = compiler_error_handler_new()

# Report errors during compilation
compiler_report_error(handler, parse_error)
compiler_report_error(handler, type_error)

# Check compilation status
lowkey compiler_has_compilation_errors(handler) {
    sus errors tea = compiler_get_error_summary(handler)
    vibez.spill("Compilation failed:\n" + errors)
    damn cap  # Indicate compilation failure
}
```

### Error Propagation in Compiler
```cursed
# Error handling with CURSED keywords
slay parse_expression() ASTNode {
    sus result ASTNode = try_parse_primary()
    
    # Check for parse errors
    lowkey shook parse_error {
        yikes error_create_syntax("Invalid expression", line, col)
    }
    
    # Attempt recovery
    lowkey fam parse_error {
        sus recovered ASTNode = recover_expression()
        damn recovered
    }
    
    damn result
}
```

## Advanced Features

### Error Message Templates
```cursed
# Contextual error messages
slay create_type_mismatch_error(expected tea, actual tea, line normie, col normie) CursedError {
    sus message tea = "Type mismatch: expected '" + expected + "', got '" + actual + "'"
    damn error_create_type(message, line, col)
}

slay create_undefined_variable_error(name tea, line normie, col normie) CursedError {
    sus message tea = "Undefined variable '" + name + "'"
    sus error CursedError = error_create_type(message, line, col)
    
    # Add context with suggestion
    sus context ErrorContext = error_context_new("source.💀", line, col)
    sus enhanced ErrorContext = error_context_add_context(context, "Did you mean '" + suggest_similar_name(name) + "'?")
    
    damn error_with_context(error, enhanced)
}
```

### Batch Error Processing
```cursed
# Process multiple errors efficiently
slay process_compilation_errors(errors [CursedError]) tea {
    sus fatal_count normie = 0
    sus warning_count normie = 0
    sus error_count normie = 0
    
    sus index normie = 0
    bestie index < array_length(errors) {
        sus error CursedError = array_get(errors, index)
        
        lowkey error_is_fatal(error) {
            fatal_count = fatal_count + 1
        } elseif error.level == "warning" {
            warning_count = warning_count + 1
        } else {
            error_count = error_count + 1
        }
        
        index = index + 1
    }
    
    sus summary tea = "Compilation summary: "
    summary = summary + integer_to_string(fatal_count) + " fatal, "
    summary = summary + integer_to_string(error_count) + " errors, "
    summary = summary + integer_to_string(warning_count) + " warnings"
    
    damn summary
}
```

## Testing

Comprehensive test suite validates all error handling:

```bash
cargo run --bin cursed stdlib/error_core/test_error_core.💀
```

Tests cover:
- Error creation for all categories and levels
- Error context formatting and enhancement
- Error collection and batch processing
- Recovery strategy suggestion and application
- Error propagation with yikes/shook/fam
- Compiler integration scenarios
- Edge cases and error conditions

## Self-Hosting Significance

Critical for compiler self-hosting:

1. **Robust Error Reporting**: Clear, actionable error messages
2. **Recovery Strategies**: Continue compilation after errors
3. **Error Propagation**: Proper error handling throughout compiler
4. **Development Experience**: Helpful error messages for CURSED developers
5. **Debugging Support**: Detailed error context for troubleshooting

## Performance Considerations

- **Lazy Formatting**: Error messages formatted only when needed
- **Efficient Collection**: Minimal overhead during normal compilation
- **Recovery Caching**: Cache recovery strategies for similar errors
- **Context Sharing**: Reuse error contexts when possible

## Integration Points

Works with other stdlib modules:
- **compiler_core**: Provides error handling for all compilation phases
- **runtime_core**: Handles runtime value errors
- **fs**: File I/O error handling
- **process**: External process error management
- **testz**: Test failure reporting and analysis

## Error Recovery Examples

### Syntax Error Recovery
```cursed
# Missing semicolon recovery
# Before: "sus x normie = 42" (missing semicolon)
# Recovery: Insert semicolon → "sus x normie = 42;"

# Unexpected token recovery  
# Before: "sus x normie = = 42;" (double equals)
# Recovery: Remove extra token → "sus x normie = 42;"
```

### Type Error Recovery
```cursed
# Type mismatch recovery
# Before: "sus x normie = 3.14" (float assigned to int)
# Recovery: Insert cast → "sus x normie = 3.14.(normie)"

# Undefined type recovery
# Before: "sus x CustomType = value" (unknown type)
# Recovery: Suggest → "Did you mean 'normie'?"
```

This comprehensive error handling system ensures the CURSED compiler can provide excellent developer experience while maintaining robust self-hosting capabilities.
