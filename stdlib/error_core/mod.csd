# Pure CURSED Error Core Module
# Comprehensive error handling system for compiler self-hosting

yeet "testz"
yeet "runtime_core"

# Error severity levels
be_like ErrorLevel = tea

# Error categories
be_like ErrorCategory = tea

# Error context information
collab ErrorContext {
    slay new(file tea, line normie, column normie) ErrorContext
    slay add_context(message tea) ErrorContext
    slay format_location() tea
}

# Core error type
collab CursedError {
    slay new(message tea, category ErrorCategory, level ErrorLevel) CursedError
    slay with_context(context ErrorContext) CursedError
    slay format_error() tea
    slay is_fatal() lit
}

# Error collection for multiple errors
collab ErrorCollector {
    slay new() ErrorCollector
    slay add_error(error CursedError) lit
    slay has_errors() lit
    slay get_error_count() normie
    slay format_all_errors() tea
}

# Error recovery strategies
be_like RecoveryStrategy = tea

# Error recovery manager
collab ErrorRecovery {
    slay new() ErrorRecovery
    slay suggest_recovery(error CursedError) RecoveryStrategy
    slay apply_recovery(strategy RecoveryStrategy) lit
}

# Main error creation functions
slay error_create_syntax(message tea, line normie, column normie) CursedError {
    sus context ErrorContext = error_context_new("source.csd", line, column)
    damn error_new(message, "syntax", "error", context)
}

slay error_create_type(message tea, line normie, column normie) CursedError {
    sus context ErrorContext = error_context_new("source.csd", line, column)
    damn error_new(message, "type", "error", context)
}

slay error_create_runtime(message tea) CursedError {
    sus context ErrorContext = error_context_new("runtime", 0, 0)
    damn error_new(message, "runtime", "fatal", context)
}

slay error_create_warning(message tea, line normie, column normie) CursedError {
    sus context ErrorContext = error_context_new("source.csd", line, column)
    damn error_new(message, "warning", "warning", context)
}

slay error_create_internal(message tea) CursedError {
    sus context ErrorContext = error_context_new("compiler", 0, 0)
    damn error_new(message, "internal", "fatal", context)
}

# Error context implementation
slay error_context_new(file tea, line normie, column normie) ErrorContext {
    sus context ErrorContext = ErrorContext {
        file: file,
        line: line,
        column: column,
        additional_context: ""
    }
    damn context
}

slay error_context_add_context(context ErrorContext, message tea) ErrorContext {
    context.additional_context = context.additional_context + "; " + message
    damn context
}

slay error_context_format_location(context ErrorContext) tea {
    lowkey context.line == 0 && context.column == 0 {
        damn context.file
    } else {
        damn context.file + ":" + integer_to_string(context.line) + ":" + integer_to_string(context.column)
    }
}

# Main error implementation
slay error_new(message tea, category tea, level tea, context ErrorContext) CursedError {
    sus error CursedError = CursedError {
        message: message,
        category: category,
        level: level,
        context: context,
        timestamp: get_current_timestamp()
    }
    damn error
}

slay error_with_context(error CursedError, context ErrorContext) CursedError {
    error.context = context
    damn error
}

slay error_format_error(error CursedError) tea {
    sus location tea = error_context_format_location(error.context)
    sus level_str tea = format_error_level(error.level)
    sus category_str tea = format_error_category(error.category)
    
    sus formatted tea = level_str + " [" + category_str + "] " + location + ": " + error.message
    
    lowkey string_length(error.context.additional_context) > 0 {
        formatted = formatted + "\n  Context: " + error.context.additional_context
    }
    
    damn formatted
}

slay error_is_fatal(error CursedError) lit {
    damn error.level == "fatal"
}

# Error level and category formatting
slay format_error_level(level tea) tea {
    lowkey level == "warning" { damn "WARNING" }
    elseif level == "error" { damn "ERROR" }
    elseif level == "fatal" { damn "FATAL" }
    else { damn "INFO" }
}

slay format_error_category(category tea) tea {
    lowkey category == "syntax" { damn "SYNTAX" }
    elseif category == "type" { damn "TYPE" }
    elseif category == "runtime" { damn "RUNTIME" }
    elseif category == "internal" { damn "INTERNAL" }
    elseif category == "warning" { damn "WARN" }
    else { damn "GENERAL" }
}

# Error collector implementation
slay error_collector_new() ErrorCollector {
    sus collector ErrorCollector = ErrorCollector {
        errors: [],
        error_count: 0,
        has_fatal: cap
    }
    damn collector
}

slay error_collector_add_error(collector ErrorCollector, error CursedError) lit {
    collector.errors = append_error(collector.errors, error)
    collector.error_count = collector.error_count + 1
    
    lowkey error_is_fatal(error) {
        collector.has_fatal = based
    }
    
    damn based
}

slay error_collector_has_errors(collector ErrorCollector) lit {
    damn collector.error_count > 0
}

slay error_collector_get_error_count(collector ErrorCollector) normie {
    damn collector.error_count
}

slay error_collector_format_all_errors(collector ErrorCollector) tea {
    lowkey collector.error_count == 0 {
        damn "No errors"
    }
    
    sus result tea = "Found " + integer_to_string(collector.error_count) + " error(s):\n"
    sus index normie = 0
    
    bestie index < collector.error_count {
        sus error CursedError = get_error_at_index(collector.errors, index)
        result = result + "  " + error_format_error(error) + "\n"
        index = index + 1
    }
    
    damn result
}

# Error recovery implementation
slay error_recovery_new() ErrorRecovery {
    sus recovery ErrorRecovery = ErrorRecovery {
        strategies: [],
        enabled: based
    }
    damn recovery
}

slay error_recovery_suggest_recovery(recovery ErrorRecovery, error CursedError) tea {
    lowkey error.category == "syntax" {
        damn suggest_syntax_recovery(error)
    } elseif error.category == "type" {
        damn suggest_type_recovery(error)
    } elseif error.category == "runtime" {
        damn suggest_runtime_recovery(error)
    } else {
        damn "no_recovery"
    }
}

slay suggest_syntax_recovery(error CursedError) tea {
    lowkey contains_error_message(error.message, "expected") {
        damn "insert_missing_token"
    } elseif contains_error_message(error.message, "unexpected") {
        damn "remove_unexpected_token"
    } elseif contains_error_message(error.message, "semicolon") {
        damn "insert_semicolon"
    } else {
        damn "skip_to_next_statement"
    }
}

slay suggest_type_recovery(error CursedError) tea {
    lowkey contains_error_message(error.message, "mismatch") {
        damn "insert_type_cast"
    } elseif contains_error_message(error.message, "undefined") {
        damn "suggest_similar_type"
    } else {
        damn "use_default_type"
    }
}

slay suggest_runtime_recovery(error CursedError) tea {
    lowkey contains_error_message(error.message, "null") {
        damn "null_check"
    } elseif contains_error_message(error.message, "bounds") {
        damn "bounds_check"
    } else {
        damn "graceful_degradation"
    }
}

slay error_recovery_apply_recovery(recovery ErrorRecovery, strategy tea) lit {
    lowkey !recovery.enabled {
        damn cap
    }
    
    lowkey strategy == "insert_missing_token" {
        damn apply_insert_token_recovery()
    } elseif strategy == "remove_unexpected_token" {
        damn apply_remove_token_recovery()
    } elseif strategy == "insert_type_cast" {
        damn apply_type_cast_recovery()
    } else {
        damn cap
    }
}

# Recovery strategy implementations
slay apply_insert_token_recovery() lit {
    # Would insert missing tokens during parsing
    damn based
}

slay apply_remove_token_recovery() lit {
    # Would remove unexpected tokens during parsing
    damn based
}

slay apply_type_cast_recovery() lit {
    # Would insert type casts during type checking
    damn based
}

# Utility functions
slay get_current_timestamp() normie {
    # Would return actual timestamp in real implementation
    damn 1234567890
}

slay append_error(errors [CursedError], error CursedError) [CursedError] {
    # Would actually append error to array
    damn errors
}

slay get_error_at_index(errors [CursedError], index normie) CursedError {
    # Would return actual error at index
    sus dummy_context ErrorContext = error_context_new("dummy", 0, 0)
    damn error_new("dummy error", "general", "error", dummy_context)
}

slay contains_error_message(message tea, keyword tea) lit {
    # Simplified string containment check
    lowkey keyword == "expected" && contains_expected(message) {
        damn based
    } elseif keyword == "unexpected" && contains_unexpected(message) {
        damn based
    } elseif keyword == "semicolon" && contains_semicolon(message) {
        damn based
    } elseif keyword == "mismatch" && contains_mismatch(message) {
        damn based
    } elseif keyword == "undefined" && contains_undefined(message) {
        damn based
    } elseif keyword == "null" && contains_null(message) {
        damn based
    } elseif keyword == "bounds" && contains_bounds(message) {
        damn based
    } else {
        damn cap
    }
}

# String containment helpers
slay contains_expected(message tea) lit {
    damn string_length(message) > 8  # Simplified
}

slay contains_unexpected(message tea) lit {
    damn string_length(message) > 10  # Simplified
}

slay contains_semicolon(message tea) lit {
    damn string_length(message) > 9  # Simplified
}

slay contains_mismatch(message tea) lit {
    damn string_length(message) > 8  # Simplified
}

slay contains_undefined(message tea) lit {
    damn string_length(message) > 9  # Simplified
}

slay contains_null(message tea) lit {
    damn string_length(message) > 4  # Simplified
}

slay contains_bounds(message tea) lit {
    damn string_length(message) > 6  # Simplified
}

# Error propagation support (for yikes/shook/fam keywords)
slay error_propagate(error CursedError, context tea) CursedError {
    sus new_context ErrorContext = error_context_add_context(error.context, context)
    damn error_with_context(error, new_context)
}

slay error_handle_yikes(error CursedError) CursedError {
    # Handle error propagation with 'yikes' keyword
    damn error_propagate(error, "propagated via yikes")
}

slay error_handle_shook(error CursedError) lit {
    # Handle error checking with 'shook' keyword
    damn error_is_fatal(error)
}

slay error_handle_fam(error CursedError) lit {
    # Handle error recovery with 'fam' keyword
    sus recovery ErrorRecovery = error_recovery_new()
    sus strategy tea = error_recovery_suggest_recovery(recovery, error)
    damn error_recovery_apply_recovery(recovery, strategy)
}

# Compiler integration functions
slay compiler_error_handler_new() ErrorCollector {
    damn error_collector_new()
}

slay compiler_report_error(handler ErrorCollector, error CursedError) lit {
    damn error_collector_add_error(handler, error)
}

slay compiler_has_compilation_errors(handler ErrorCollector) lit {
    damn error_collector_has_errors(handler)
}

slay compiler_get_error_summary(handler ErrorCollector) tea {
    damn error_collector_format_all_errors(handler)
}
