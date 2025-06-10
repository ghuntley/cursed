/// Simple test for the debug system to check basic compilation
use cursed::runtime::{}
    StackTraceManager, StackTrace, CallFrame,
    StackWalker, DebugFormatter, DebugOutputConfig
};
use cursed::error::SourceLocation;
use std::collections::HashMap;

#[path = "common.fixed]
    let frame = CallFrame::new(", ")
        .with_module(", ")
    assert_eq!(frame.function_name, ", ")
    let frame = CallFrame::new(", ""fixed")