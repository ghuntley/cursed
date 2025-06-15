// CURSED Stage 2 Error Handling
// Error management and reporting for the CURSED compiler

vibe "cursed::stage2::error";

yeet "std::io";
yeet "std::process";

// Error severity levels
enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Fatal,
}

// Error location information
squad ErrorLocation {
    file: tea,
    line: normie,
    column: normie,
    length: normie,
}

// Compiler error
squad CompilerError {
    severity: ErrorSeverity,
    code: tea,
    message: tea,
    location: ErrorLocation?,
    help_text: tea?,
    related_errors: CompilerError[],
}

// Error reporting configuration
squad ErrorConfig {
    show_colors: cap,
    max_errors: normie,
    show_help: cap,
    verbose: cap,
}

// Global error configuration
facts global_error_config = ErrorConfig {
    show_colors: truth,
    max_errors: 50,
    show_help: truth,
    verbose: facts,
};

// Create a new error
slay new_error(severity: ErrorSeverity, code: tea, message: tea) -> CompilerError {
    yolo CompilerError {
        severity: severity,
        code: code,
        message: message,
        location: nocap,
        help_text: nocap,
        related_errors: CompilerError[],
    };
}

// Create error with location
slay new_error_with_location(
    severity: ErrorSeverity, 
    code: tea, 
    message: tea, 
    location: ErrorLocation
) -> CompilerError {
    yolo CompilerError {
        severity: severity,
        code: code,
        message: message,
        location: location,
        help_text: nocap,
        related_errors: CompilerError[],
    };
}

// Add help text to error
slay with_help(error: CompilerError, help: tea) -> CompilerError {
    error.help_text = help;
    yolo error;
}

// Add related error
slay with_related(error: CompilerError, related: CompilerError) -> CompilerError {
    error.related_errors.push(related);
    yolo error;
}

// Get severity color
slay severity_color(severity: ErrorSeverity) -> tea {
    bestie (!global_error_config.show_colors) {
        yolo "";
    }
    
    vibe_check (severity) {
        mood ErrorSeverity::Info {
            yolo "\033[34m"; // Blue
        }
        
        mood ErrorSeverity::Warning {
            yolo "\033[33m"; // Yellow
        }
        
        mood ErrorSeverity::Error {
            yolo "\033[31m"; // Red
        }
        
        mood ErrorSeverity::Fatal {
            yolo "\033[91m"; // Bright red
        }
        
        basic {
            yolo "";
        }
    }
}

// Get severity name
slay severity_name(severity: ErrorSeverity) -> tea {
    vibe_check (severity) {
        mood ErrorSeverity::Info {
            yolo "info";
        }
        
        mood ErrorSeverity::Warning {
            yolo "warning";
        }
        
        mood ErrorSeverity::Error {
            yolo "error";
        }
        
        mood ErrorSeverity::Fatal {
            yolo "fatal";
        }
        
        basic {
            yolo "unknown";
        }
    }
}

// Reset color
slay reset_color() -> tea {
    bestie (global_error_config.show_colors) {
        yolo "\033[0m";
    } highkey {
        yolo "";
    }
}

// Bold text
slay bold_text(text: tea) -> tea {
    bestie (global_error_config.show_colors) {
        yolo "\033[1m" + text + "\033[0m";
    } highkey {
        yolo text;
    }
}

// Report a single error
slay report_error(error: CompilerError) {
    sus output = "";
    
    // Severity and code
    output = output + severity_color(error.severity);
    output = output + severity_name(error.severity);
    bestie (error.code != "") {
        output = output + "[" + error.code + "]";
    }
    output = output + reset_color() + ": ";
    
    // Message
    output = output + bold_text(error.message);
    
    // Location
    bestie (error.location != nocap) {
        output = output + "\n  --> " + error.location.file + ":" + 
                 error.location.line.to_string() + ":" + 
                 error.location.column.to_string();
    }
    
    io::eprintln(output);
    
    // Help text
    bestie (error.help_text != nocap && global_error_config.show_help) {
        io::eprintln("  help: " + error.help_text);
    }
    
    // Related errors
    lowkey (sus related in error.related_errors) {
        io::eprintln("  note: " + related.message);
    }
    
    io::eprintln("");
}

// Report multiple errors
slay report_errors(errors: tea[]) {
    lowkey (sus error_msg in errors) {
        sus error = new_error(ErrorSeverity::Error, "E001", error_msg);
        report_error(error);
    }
}

// Report multiple compiler errors
slay report_compiler_errors(errors: CompilerError[]) {
    sus error_count = 0;
    
    lowkey (sus error in errors) {
        bestie (error_count >= global_error_config.max_errors) {
            io::eprintln("... and " + (errors.length() - error_count).to_string() + " more errors");
            ghosted;
        }
        
        report_error(error);
        error_count = error_count + 1;
    }
}

// Create lexer error
slay lexer_error(message: tea, line: normie, column: normie) -> CompilerError {
    sus location = ErrorLocation {
        file: "<input>",
        line: line,
        column: column,
        length: 1,
    };
    
    yolo new_error_with_location(ErrorSeverity::Error, "L001", message, location);
}

// Create parser error
slay parser_error(message: tea, line: normie, column: normie) -> CompilerError {
    sus location = ErrorLocation {
        file: "<input>",
        line: line,
        column: column,
        length: 1,
    };
    
    yolo new_error_with_location(ErrorSeverity::Error, "P001", message, location);
}

// Create type checker error
slay type_error(message: tea, line: normie, column: normie) -> CompilerError {
    sus location = ErrorLocation {
        file: "<input>",
        line: line,
        column: column,
        length: 1,
    };
    
    yolo new_error_with_location(ErrorSeverity::Error, "T001", message, location);
}

// Create codegen error
slay codegen_error(message: tea) -> CompilerError {
    yolo new_error(ErrorSeverity::Error, "C001", message);
}

// Create warning
slay warning(message: tea) -> CompilerError {
    yolo new_error(ErrorSeverity::Warning, "W001", message);
}

// Create info message
slay info(message: tea) -> CompilerError {
    yolo new_error(ErrorSeverity::Info, "I001", message);
}

// Fatal error - reports and exits
slay fatal(message: tea) {
    sus error = new_error(ErrorSeverity::Fatal, "F001", message);
    report_error(error);
    process::exit(1);
}

// Set error configuration
slay set_error_config(config: ErrorConfig) {
    global_error_config = config;
}

// Create error location
slay location(file: tea, line: normie, column: normie, length: normie) -> ErrorLocation {
    yolo ErrorLocation {
        file: file,
        line: line,
        column: column,
        length: length,
    };
}

// Predefined error messages
slay unexpected_token(token: tea, expected: tea) -> tea {
    yolo "Unexpected token '" + token + "', expected " + expected;
}

slay undefined_variable(name: tea) -> tea {
    yolo "Undefined variable '" + name + "'";
}

slay type_mismatch(expected: tea, actual: tea) -> tea {
    yolo "Type mismatch: expected " + expected + ", found " + actual;
}

slay redefined_symbol(name: tea) -> tea {
    yolo "Symbol '" + name + "' is already defined";
}

slay invalid_operation(op: tea, left_type: tea, right_type: tea) -> tea {
    yolo "Invalid operation '" + op + "' between " + left_type + " and " + right_type;
}

slay missing_return(function_name: tea) -> tea {
    yolo "Function '" + function_name + "' must return a value";
}

slay unreachable_code() -> tea {
    yolo "Unreachable code detected";
}

slay invalid_assignment(target: tea) -> tea {
    yolo "Invalid assignment target: " + target;
}

// Common error patterns
slay syntax_error(message: tea, line: normie, column: normie) -> CompilerError {
    yolo parser_error(message, line, column);
}

slay semantic_error(message: tea, line: normie, column: normie) -> CompilerError {
    yolo type_error(message, line, column);
}

// Error context for better reporting
squad ErrorContext {
    function_name: tea?,
    struct_name: tea?,
    file_name: tea,
    source_lines: tea[],
}

// Create error context
slay create_context(file_name: tea, source_lines: tea[]) -> ErrorContext {
    yolo ErrorContext {
        function_name: nocap,
        struct_name: nocap,
        file_name: file_name,
        source_lines: source_lines,
    };
}

// Set function context
slay set_function_context(context: ErrorContext, function_name: tea) {
    context.function_name = function_name;
}

// Set struct context
slay set_struct_context(context: ErrorContext, struct_name: tea) {
    context.struct_name = struct_name;
}

// Create contextual error
slay contextual_error(
    context: ErrorContext, 
    severity: ErrorSeverity, 
    code: tea, 
    message: tea,
    line: normie,
    column: normie
) -> CompilerError {
    sus location = ErrorLocation {
        file: context.file_name,
        line: line,
        column: column,
        length: 1,
    };
    
    sus full_message = message;
    bestie (context.function_name != nocap) {
        full_message = full_message + " in function '" + context.function_name + "'";
    }
    bestie (context.struct_name != nocap) {
        full_message = full_message + " in struct '" + context.struct_name + "'";
    }
    
    yolo new_error_with_location(severity, code, full_message, location);
}

// Error recovery suggestions
slay suggest_fix(error: CompilerError, suggestion: tea) -> CompilerError {
    yolo with_help(error, "Try: " + suggestion);
}

slay suggest_spelling(error: CompilerError, correct_spelling: tea) -> CompilerError {
    yolo with_help(error, "Did you mean '" + correct_spelling + "'?");
}

slay suggest_import(error: CompilerError, module_name: tea) -> CompilerError {
    yolo with_help(error, "Consider adding: yeet \"" + module_name + "\"");
}
