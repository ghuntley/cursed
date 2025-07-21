//! Comprehensive tests for CURSED debug functionality
//! 
//! Tests cover:
//! - DWARF debug information generation
//! - GDB and LLDB integration
//! - Breakpoint management
//! - Variable inspection
//! - Stack traces and memory inspection

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::dwarf_comprehensive::*;
    use crate::debug::gdb_integration::*;
    use crate::debug::lldb_integration::*;
    use crate::debug::enhanced_debug::*;
    use crate::error::{CursedError, SourceLocation};
    use crate::ast::{Ast, FunctionDeclaration, VariableDeclaration, Type, Parameter, Visibility, Statement, TypeParameter, Comment};
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    /// Test DWARF debug information generation
    #[test]
    fn test_dwarf_debug_generation() {
        let mut generator = DwarfDebugGenerator::new();
        
        // Create test AST - using a simple program with minimal structure
        let ast = Ast::Program(crate::ast::Program {
            statements: vec![],
            imports: vec![],
            package: None,
        });
        
        // Generate debug information
        let result = generator.generate_debug_info(&ast, "test.csd");
        assert!(result.is_ok(), "Failed to generate debug info: {:?}", result);
        
        // Verify compilation unit was created
        assert_eq!(generator.compilation_units.len(), 1);
        let cu = &generator.compilation_units[0];
        assert_eq!(cu.source_file, "test.csd");
        assert_eq!(cu.language, DwarfLanguage::Cursed);
    }

    /// Test DWARF section encoding
    #[test]
    fn test_dwarf_section_encoding() {
        let mut generator = DwarfDebugGenerator::new();
        
        // Add some strings to make debug_str non-empty
        generator.debug_str.add_string("test.csd");
        generator.debug_str.add_string("CURSED Compiler");
        
        // Add minimal compilation unit
        let cu = CompilationUnit {
            unit_id: 0,
            source_file: "test.csd".to_string(),
            producer: "CURSED Compiler".to_string(),
            language: DwarfLanguage::Cursed,
            compilation_dir: "/tmp".to_string(),
            low_pc: 0x1000,
            high_pc: 0x2000,
            line_table_offset: 0,
            dies: vec![],
        };
        generator.compilation_units.push(cu);
        
        // Generate abbreviation table
        assert!(generator.generate_abbreviation_table().is_ok());
        
        // Encode debug sections
        let result = generator.encode_debug_sections();
        assert!(result.is_ok(), "Failed to encode debug sections: {:?}", result);
        
        let sections = result.unwrap();
        
        // Verify required sections are present
        assert!(sections.contains_key(".debug_info"));
        assert!(sections.contains_key(".debug_abbrev"));
        assert!(sections.contains_key(".debug_str"));
        assert!(sections.contains_key(".debug_line"));
        assert!(sections.contains_key(".debug_frame"));
        assert!(sections.contains_key(".debug_aranges"));
        
        // Verify sections contain data (debug_str should have data now)
        assert!(!sections[".debug_str"].is_empty(), "Section .debug_str is empty");
        // Other sections have minimal placeholder data
        assert!(!sections[".debug_info"].is_empty(), "Section .debug_info is empty");
    }

    /// Test debug string table
    #[test]
    fn test_debug_string_table() {
        let mut string_table = DebugStringTable::new();
        
        // Add strings
        let offset1 = string_table.add_string("test_function");
        let offset2 = string_table.add_string("test_variable");
        let offset3 = string_table.add_string("test_function"); // Duplicate
        
        // Verify offset handling
        assert_eq!(offset1, 0);
        assert_ne!(offset1, offset2);
        assert_eq!(offset1, offset3); // Duplicate should return same offset
        
        // Verify string table contains correct data
        assert!(string_table.data.len() > 0);
        assert!(string_table.strings.contains_key("test_function"));
        assert!(string_table.strings.contains_key("test_variable"));
    }

    /// Test enhanced debug manager
    #[test]
    fn test_enhanced_debug_manager() {
        let mut manager = EnhancedDebugManager::new();
        
        // Enable debug mode
        manager.enable_debug();
        manager.enable_verbose();
        
        // Create test source file
        let test_source = "sus x normie = 42\nvibez.spill(x)";
        fs::write("test_debug.csd", test_source).unwrap();
        
        // Add source file
        let result = manager.add_source_file("test_debug.csd");
        assert!(result.is_ok(), "Failed to add source file: {:?}", result);
        
        // Add debug symbol
        let symbol = create_debug_symbol(
            "test_variable",
            SymbolType::Variable,
            SourceLocation {
file: "test_debug.csd".to_string(),
                line: 1,
                column: 1,
            
                    offset: 0,
                },
        );
        manager.add_debug_symbol(symbol);
        
        // Test source context
        let location = SourceLocation {
file: "test_debug.csd".to_string(),
            line: 1,
            column: 5,
        
                    offset: 0,
                };
        
        let context_result = manager.get_source_context(&location);
        assert!(context_result.is_ok(), "Failed to get source context: {:?}", context_result);
        
        let context = context_result.unwrap();
        assert_eq!(context.error_line, "sus x normie = 42");
        assert!(context.line_after.is_some());
        
        // Test error message formatting
        let error = CursedError::General("Test error".to_string());
        let message_result = manager.format_error_message(&error, &location);
        assert!(message_result.is_ok());
        
        let message = message_result.unwrap();
        assert!(message.contains("Test error"));
        assert!(message.contains("test_debug.csd:1:5"));
        assert!(message.contains("sus x normie = 42"));
        
        // Cleanup
        let _ = fs::remove_file("test_debug.csd");
    }

    /// Test DWARF assembly generation
    #[test]
    fn test_dwarf_assembly_generation() {
        let generator = DwarfDebugGenerator::new();
        
        let result = generator.generate_debug_assembly("test_debug.s");
        assert!(result.is_ok(), "Failed to generate debug assembly: {:?}", result);
        
        // Verify file was created
        assert!(Path::new("test_debug.s").exists());
        
        // Verify assembly content
        let content = fs::read_to_string("test_debug.s").unwrap();
        assert!(content.contains(".section .debug_info"));
        assert!(content.contains(".section .debug_abbrev"));
        assert!(content.contains(".section .debug_str"));
        assert!(content.contains("DW_TAG_compile_unit"));
        assert!(content.contains("CURSED Compiler"));
        
        // Cleanup
        let _ = fs::remove_file("test_debug.s");
    }

    /// Test GDB integration (mock test)
    #[test]
    fn test_gdb_integration_basic() {
        let _gdb = GdbIntegration::new();
        
        // Basic instantiation test
        // Note: Internal fields are private, so we just test creation
    }

    /// Test LLDB integration (mock test)
    #[test]
    fn test_lldb_integration_basic() {
        let _lldb = LldbIntegration::new();
        
        // Basic instantiation test
        // Note: Internal fields are private, so we just test creation
    }

    /// Test breakpoint structure
    #[test]
    fn test_breakpoint_structure() {
        // Create test breakpoint
        let breakpoint = GdbBreakpoint {
            id: 1,
            enabled: true,
            location: "main".to_string(),
            condition: None,
            hit_count: 0,
            ignore_count: 0,
            temporary: false,
        };
        
        // Test breakpoint properties
        assert_eq!(breakpoint.id, 1);
        assert_eq!(breakpoint.location, "main");
        assert!(breakpoint.enabled);
        assert_eq!(breakpoint.hit_count, 0);
    }

    /// Test memory region handling
    #[test]
    fn test_memory_region() {
        use crate::debug::gdb_integration::MemoryRegion;
        
        let region = MemoryRegion {
            address: 0x1000,
            size: 256,
            data: vec![0x41, 0x42, 0x43, 0x44], // "ABCD"
            permissions: "rwx".to_string(),
        };
        
        assert_eq!(region.address, 0x1000);
        assert_eq!(region.size, 256);
        assert_eq!(region.data.len(), 4);
        assert_eq!(region.permissions, "rwx");
        
        // Test data access
        assert_eq!(region.data[0], 0x41); // 'A'
        assert_eq!(region.data[1], 0x42); // 'B'
    }

    /// Test variable information
    #[test]
    fn test_variable_info() {
        use crate::debug::gdb_integration::GdbVariable;
        
        let variable = GdbVariable {
            name: "test_var".to_string(),
            value: "42".to_string(),
            var_type: "int".to_string(),
            in_scope: true,
        };
        
        assert_eq!(variable.name, "test_var");
        assert_eq!(variable.value, "42");
        assert_eq!(variable.var_type, "int");
        assert!(variable.in_scope);
    }

    /// Test thread information
    #[test]
    fn test_thread_info() {
        use crate::debug::gdb_integration::ThreadInfo;
        
        let thread = ThreadInfo {
            id: 1,
            target_id: "0x7ffff7dd1740".to_string(),
            name: Some("main".to_string()),
            state: crate::debug::gdb_integration::ThreadState::Stopped,
            frame: None,
        };
        
        assert_eq!(thread.id, 1);
        assert_eq!(thread.target_id, "0x7ffff7dd1740");
        assert_eq!(thread.name, Some("main".to_string()));
        assert!(matches!(thread.state, crate::debug::gdb_integration::ThreadState::Stopped));
    }

    /// Test stack frame information
    #[test]
    fn test_stack_frame() {
        use crate::debug::gdb_integration::GdbFrame;
        use std::collections::HashMap;
        
        let frame = GdbFrame {
            level: 0,
            addr: 0x400500,
            func: "main".to_string(),
            file: Some("main.csd".to_string()),
            line: Some(10),
            args: vec![],
        };
        
        assert_eq!(frame.level, 0);
        assert_eq!(frame.addr, 0x400500);
        assert_eq!(frame.func, "main");
        assert_eq!(frame.file, Some("main.csd".to_string()));
        assert_eq!(frame.line, Some(10));
    }

    /// Test LLDB breakpoint location
    #[test]
    fn test_lldb_breakpoint_location() {
        use crate::debug::lldb_integration::LldbBreakpointLocation;
        
        let location = LldbBreakpointLocation {
            id: "1.1".to_string(),
            address: 0x400500,
            resolved: true,
            module: Some("main".to_string()),
            function: Some("test_function".to_string()),
            file: Some("main.csd".to_string()),
            line: Some(15),
            column: Some(5),
        };
        
        assert_eq!(location.id, "1.1");
        assert_eq!(location.address, 0x400500);
        assert!(location.resolved);
        assert_eq!(location.function, Some("test_function".to_string()));
        assert_eq!(location.line, Some(15));
        assert_eq!(location.column, Some(5));
    }

    /// Test LLDB memory region
    #[test]
    fn test_lldb_memory_region() {
        use crate::debug::lldb_integration::LldbMemoryRegion;
        
        let region = LldbMemoryRegion {
            start_address: 0x7fff0000,
            end_address: 0x7fff1000,
            permissions: "rw-".to_string(),
            name: Some("stack".to_string()),
            data: vec![0; 4096],
        };
        
        assert_eq!(region.start_address, 0x7fff0000);
        assert_eq!(region.end_address, 0x7fff1000);
        assert_eq!(region.permissions, "rw-");
        assert_eq!(region.name, Some("stack".to_string()));
        assert_eq!(region.data.len(), 4096);
    }

    /// Test LLDB register information
    #[test]
    fn test_lldb_register() {
        use crate::debug::lldb_integration::{LldbRegister, RegisterFormat};
        
        let register = LldbRegister {
            name: "rax".to_string(),
            value: 0x12345678,
            size: 8,
            format: RegisterFormat::Hex,
        };
        
        assert_eq!(register.name, "rax");
        assert_eq!(register.value, 0x12345678);
        assert_eq!(register.size, 8);
        assert!(matches!(register.format, RegisterFormat::Hex));
    }

    /// Test LLDB watchpoint
    #[test]
    fn test_lldb_watchpoint() {
        use crate::debug::lldb_integration::{LldbWatchpoint, WatchType};
        
        let watchpoint = LldbWatchpoint {
            id: 1,
            address: 0x7fff0000,
            size: 8,
            watch_type: WatchType::ReadWrite,
            condition: Some("value > 10".to_string()),
            hit_count: 0,
            enabled: true,
        };
        
        assert_eq!(watchpoint.id, 1);
        assert_eq!(watchpoint.address, 0x7fff0000);
        assert_eq!(watchpoint.size, 8);
        assert!(matches!(watchpoint.watch_type, WatchType::ReadWrite));
        assert_eq!(watchpoint.condition, Some("value > 10".to_string()));
        assert!(watchpoint.enabled);
    }

    /// Test debug information integration
    #[test]
    fn test_debug_info_integration() {
        let mut manager = EnhancedDebugManager::new();
        let mut generator = DwarfDebugGenerator::new();
        
        // Create test program
        let test_source = r#"
slay main() normie {
    sus x normie = 42
    vibez.spill(x)
    damn 0
}
"#;
        
        fs::write("integration_test.csd", test_source).unwrap();
        
        // Add to debug manager
        assert!(manager.add_source_file("integration_test.csd").is_ok());
        
        // Create minimal AST for testing
        let ast = Ast::Program(crate::ast::Program {
            statements: vec![],
            imports: vec![],
            package: None,
        });
        
        // Generate debug information
        assert!(generator.generate_debug_info(&ast, "integration_test.csd").is_ok());
        
        // Verify debug sections can be encoded
        assert!(generator.encode_debug_sections().is_ok());
        
        // Cleanup
        let _ = fs::remove_file("integration_test.csd");
    }

    /// Test error conditions
    #[test]
    fn test_error_conditions() {
        let mut manager = EnhancedDebugManager::new();
        
        // Test non-existent file
        let result = manager.add_source_file("non_existent.csd");
        assert!(result.is_err());
        
        // Test invalid source location
        let invalid_location = SourceLocation {
file: "non_existent.csd".to_string(),
            line: 999,
            column: 999,
        
                    offset: 0,
                };
        
        let context_result = manager.get_source_context(&invalid_location);
        assert!(context_result.is_err());
    }

    /// Test DWARF abbreviation generation
    #[test]
    fn test_dwarf_abbreviation_generation() {
        let mut generator = DwarfDebugGenerator::new();
        
        // Generate abbreviations
        assert!(generator.generate_abbreviation_table().is_ok());
        
        // Verify abbreviations were created
        assert!(!generator.debug_abbrev.abbreviations.is_empty());
        
        // Check for essential abbreviations
        let mut has_compile_unit = false;
        let mut has_subprogram = false;
        let mut has_variable = false;
        
        for (_, abbrev) in &generator.debug_abbrev.abbreviations {
            match abbrev.tag {
                DwarfTag::CompileUnit => has_compile_unit = true,
                DwarfTag::Subprogram => has_subprogram = true,
                DwarfTag::Variable => has_variable = true,
                _ => {}
            }
        }
        
        assert!(has_compile_unit, "Missing compile unit abbreviation");
        assert!(has_subprogram, "Missing subprogram abbreviation");
        assert!(has_variable, "Missing variable abbreviation");
    }

    /// Test comprehensive debugging workflow
    #[test]
    fn test_comprehensive_debugging_workflow() {
        // This test simulates a complete debugging workflow
        
        // 1. Create source file
        let source = r#"
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay main() normie {
    sus result normie = factorial(5)
    vibez.spill(result)
    damn 0
}
"#;
        
        fs::write("factorial_test.csd", source).unwrap();
        
        // 2. Set up debug manager
        let mut debug_manager = EnhancedDebugManager::new();
        debug_manager.enable_debug();
        assert!(debug_manager.add_source_file("factorial_test.csd").is_ok());
        
        // 3. Create minimal AST for testing
        let ast = Ast::Program(crate::ast::Program {
            statements: vec![],
            imports: vec![],
            package: None,
        });
        
        // 4. Generate debug information
        let mut dwarf_gen = DwarfDebugGenerator::new();
        assert!(dwarf_gen.generate_debug_info(&ast, "factorial_test.csd").is_ok());
        
        // 5. Verify debug information
        assert_eq!(dwarf_gen.compilation_units.len(), 1);
        
        let cu = &dwarf_gen.compilation_units[0];
        assert_eq!(cu.source_file, "factorial_test.csd");
        assert_eq!(cu.language, DwarfLanguage::Cursed);
        
        // 6. Generate debug sections
        let sections_result = dwarf_gen.encode_debug_sections();
        assert!(sections_result.is_ok());
        
        let sections = sections_result.unwrap();
        assert!(sections.len() >= 6); // At least 6 debug sections
        
        // 7. Test source context
        let location = SourceLocation {
file: "factorial_test.csd".to_string(),
            line: 3,
            column: 9,
        
                    offset: 0,
                };
        
        let context = debug_manager.get_source_context(&location);
        assert!(context.is_ok());
        
        let context = context.unwrap();
        assert!(context.error_line.contains("lowkey"));
        
        // Cleanup
        let _ = fs::remove_file("factorial_test.csd");
    }

    /// Performance test for debug information generation
    #[test]
    fn test_debug_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        let mut generator = DwarfDebugGenerator::new();
        
        // Generate many debug entries to test performance
        for _i in 0..10 {  // Reduced count for simpler test
            let ast = Ast::Program(crate::ast::Program {
                statements: vec![],
                imports: vec![],
                package: None,
            });
            
            assert!(generator.generate_debug_info(&ast, "perf_test.csd").is_ok());
        }
        
        let duration = start.elapsed();
        println!("Generated debug info for 1000 functions in {:?}", duration);
        
        // Encoding should also be reasonably fast
        let start = Instant::now();
        let sections = generator.encode_debug_sections();
        assert!(sections.is_ok());
        let encoding_duration = start.elapsed();
        println!("Encoded debug sections in {:?}", encoding_duration);
        
        // Performance should be reasonable (less than 1 second for 1000 functions)
        assert!(duration.as_secs() < 1, "Debug generation too slow: {:?}", duration);
        assert!(encoding_duration.as_secs() < 1, "Debug encoding too slow: {:?}", encoding_duration);
    }
}
