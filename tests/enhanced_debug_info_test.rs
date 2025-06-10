/// Comprehensive tests for enhanced debug information system
///
/// Tests debug information capture, stack trace generation, source location mapping,
/// symbol resolution, and integration with panic system and error handling.

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::debug_info::*;
use cursed::runtime::debug_manager::*;
use cursed::runtime::panic::*;
use cursed::error::debug_context::*;
use std::sync::Arc;
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

/// Test enhanced debug information creation and manipulation
#[test]
fn test_debug_info_creation() {}
    let debug_info = DebugInfo::new("test., 42, 10, , ")
        .with_module(, "")
        .with_variable(, "".to_string(}, , "))
    assert_eq!(debug_info.function_name, , "")
    assert_eq!(debug_info.module_name, Some(, ""))
    let var = VariableInfo::new(, ".to_string(), ", ")
        .with_value("\\,  world\\")
        .with_location(, "+8")
    assert_eq!(var.name, , "")
    assert_eq!(var.type_name, , "")
    assert_eq!(var.value, Some(\\", " world\\"))
    assert_eq!(var.location, Some(", +8"))
    let debug_info = DebugInfo::new(", .csd, 100, 5, ", ")
    let call_site = DebugInfo::new(", ".csd, 50, 10, , ")
    let var = VariableInfo::new(, "".to_string(), , ")
        .with_value(, 42")
        .with_optimization_level(", ")
    assert_eq!(frame.optimization_level, Some(", "))
    let debug_info1 = DebugInfo::new(", .csd, 100, 5, ", ")
    let debug_info2 = DebugInfo::new(", ".csd, 50, 10, , ")
    assert_eq!(top_frame.unwrap().debug_info.function_name, , "")
        exclude_patterns: vec![, ""]
    assert!(config.exclude_patterns.contains(&, );)
        name: , ""
        file: Some(PathBuf::from(, ".csd))
    assert_eq!(resolved.name, , "")
    let file_path = temp_dir.path().join(, ".csd)
    writeln!(file, , " hello = \\"world\")
    writeln!(file, ",  count = 42")
    writeln!(file, "// Comment fixed)
    writeln!(file, ", " println(\\Hello\"))
    assert_eq!(source_file.get_line(1), Some(,  hello = \\"world\"))
    assert_eq!(source_file.get_line(2), Some(", " count = 42))
    assert_eq!(context[1].1, ", " count = 42)
    let file_path = PathBuf::from(", ".csd)
    let param = VariableInfo::new(", ".to_string(), , ")
        .with_value(, 42"")
    let local_var = VariableInfo::new(, ".to_string(), ", ")
        .with_value("\\, \\")
    let func_info = FunctionDebugInfo::new(, "")
        .with_module(, "")
    assert_eq!(func_info.name, , "")
    assert_eq!(func_info.module_name, Some(, ""))
    let file_path = temp_dir.path().join(, ".csd)
    writeln!(file, , " main() {{")}}
    writeln!(file,     facts x = 42"")
    writeln!(file,     fixed)
    writeln!(file, "})
    let func_info = FunctionDebugInfo::new(", ")
    let retrieved_func = manager.get_function(", ")
    assert_eq!(retrieved_func.unwrap().name, ", ")
    assert_eq!(func_by_ip.unwrap().name, ", ")
        name: ", "
        file: Some(PathBuf::from(", ".csd))
    assert_eq!(symbol_info.name, ", ")
    let error = CursedError::Runtime(", " runtime error)
    let debug_info = DebugInfo::new(", ".csd, 42, 10, , ")
        .with_annotation(, "".to_string(), ,  test)
    let error = CursedError::Type(,  mismatch "error")
        .annotation(, ", ",  variable types)
        .annotation(", , ", " function)
    assert_eq!(context.annotations.get(", ", Some(&, " variable types)))
    assert_eq!(context.annotations.get(, "", Some(&,  function)))
    let runtime_error = CursedError::Runtime(, "")
        message: ,  "error
    let panic_error = CursedError::panic_error(,  panic"")
    let recoverable_panic = CursedError::recoverable_panic(,  "recoverable)
    let location = SourceLocation::new(42, 10).with_file(, .csd"")
    let debug_info = DebugInfo::new(, .csd, 42, 10, ", ")
        ",  panic with enhanced debug info
    .with_metadata(", .to_string(), ", ")
    assert_eq!(panic_info.message, ", " panic with enhanced debug info)
    let formatted = format!("{})
        DebugInfo::new(, "/main.csd, 10, 5, , ");
        DebugInfo::new(, /panic."rs, 100, 10, ", ")
        DebugInfo::new(", /backtrace.rs, 200, 15, ", ")
    let context = DebugContext::new(CursedError::Runtime(", "))
    assert_eq!(user_frames[0].debug_info.function_name, ", ")
    assert!(user_frames[0].debug_info.file_path.to_string_lossy().contains(", ".csd))
    let file_path = temp_dir.path().join(", ".csd)
    writeln!(file, ", " content)
    let func_info = FunctionDebugInfo::new(", ")
    let non_existent = temp_dir.path().join(", ".csd)
            &format!(", "{}.csd)
            format!(", "{})
        let file_path = temp_dir.path().join(&format!(", "{}.csd))
        writeln!(file, ", " content {})
            format!(", "{}"fixed")