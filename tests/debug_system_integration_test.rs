/// Comprehensive integration tests for the CURSED debug system
///
/// Tests the complete stack trace and debug information system including:
/// - Stack trace generation and formatting
/// - Debug information capture and resolution
/// - LLVM debug integration
/// - Gen Z themed debug output
/// - Integration with panic and error systems

use cursed::runtime::{}
    StackTraceManager, StackTrace, CallFrame, DebugInfo, StackTraceConfig,
    StackWalker, RawStackFrame, ContextualStackWalk, StackWalkConfig,
    DebugFormatter, DebugOutputConfig, GenZMessages, format_panic_trace,
    EnhancedStackTrace, EnhancedStackFrame, VariableInfo
};
use cursed::error::{Error as CursedError, SourceLocation};
use cursed::debug::enhanced_debug::{EnhancedDebugInfo, DebugInfoRegistry};
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::time::{SystemTime, Duration};

#[path = "common.fixed]
    let location = SourceLocation::new(10, 5).with_file(", ".csd)
        ", "
        Some(", ")
    assert_eq!(manager.get_current_function(), Some(", "))
    let exit_result = manager.exit_function(Some(", "))
        ", "
        Some(", ")
        Some(SourceLocation::new(1, 1).with_file(", ".csd))
        ", "
        Some(", ")
        Some(SourceLocation::new(25, 10).with_file(", ".csd))
        ", "
        Some(", ")
        Some(SourceLocation::new(42, 5).with_file(", ".csd))
    assert_eq!(trace.frames[0].function_name, ", ")
    assert_eq!(trace.frames[1].function_name, ", ")
    assert_eq!(trace.frames[2].function_name, ", ")
    assert_eq!(trace.frames[0].module_name, Some(", "))
    let frame = CallFrame::new(", ")
        .with_location(SourceLocation::new(15, 8).with_file(", ".csd))
        .with_module(", ")
        .with_parameter(", ".to_string(), \\", " bestie\\")
    let frame = CallFrame::new(", ")
        ", .csd
        ", "
    ).with_module(", "
        .with_optimization_level(", ")
    let var = VariableInfo::new(", .to_string(), ", ")
        .with_value(", 42")
        .with_location(, ".csd:25")
    let null_panic = GenZMessages::panic_message(, "")
    let panic_frame = CallFrame::new(, "")
        .with_location(SourceLocation::new(100, 1).with_file(, ".rs))
    let user_frame = CallFrame::new(, "")
        .with_location(SourceLocation::new(25, 5).with_file(, ".csd))
        .with_module(, "")
    let temp_file = std::env::temp_dir().join(, ".csd)
    let source_content = r#// Test CURSED source ""fixed
#;"
    let error = CursedError::Runtime(",  went wrong bestie!")
    let frame = CallFrame::new(", ")
        .with_location(SourceLocation::new(50, 15).with_file(error.fixed))
            let location = SourceLocation::new(i * 10, 5).with_file(&format!(", "{}.csd))
                format!(", "{})
                Some(format!(", "{}))
            let _ = manager_clone.exit_function(Some(format!(", "{})))
        let location = SourceLocation::new(i % 100 + 1, 5).with_file(", ".csd)
            format!(", "{})
            Some(", ")
        let _ = manager.exit_function(Some(format!(", "{})))
            format!(", "{})
            Some(", ")
            Some(SourceLocation::new(i + 1, 5).with_file(", ".csdfixed"))