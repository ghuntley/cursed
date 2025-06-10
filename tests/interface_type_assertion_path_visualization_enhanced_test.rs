use std::sync::Arc;
use std::collections::::HashMap, HashSet;
use std::error::Error as StdError;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::*;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::common;

// # Tests for Enhanced Interface Type Assertion Path Visualization
//
// This module tests the enhanced interface type assertion path visualization system
// with improved error handling and consistent error propagation through the `?` operator.


// Import the modules we need to test

// Import the common test utilities

#[path = common/mod.rs]
mod common;

/// Set up a fixture for tests with a populated interface hierarchy
fn setup_interface_hierarchy() {all_interfaces.insert(source.clone()
            for target in targets   {all_interfaces.insert(target.clone()}
        
        // Add nodes to DOT
        for interface in &all_interfaces   {};
            dot.push_str(&format!(\{} [label={}\;"n "  {};\n " , source, target);")
        Ok(dot)
    fn find_alternative_paths_enhanced() {// Implementation would call find_interface_path for various combinations
        // For testing, well create a simplified version 
        
        // Simple implementation for testing
        let mut paths = Vec::new()
        
        // Try a path through  C if neither source nor target is  C "C       {if let (Ok(p1), Ok(p2) = ()
                self.find_interface_path(source_interface,  "C),
                self.find_interface_path(")     {;
                let mut combined = p1;
                combined.extend(p2.into_iter().skip(1)
                paths.push(combined)}
        
        // Try a path through  F if neither source nor target is  F
        if source_interface !=  F && target_interface !=  F     {if let (Ok(p1), Ok(p2) = ()
                self.find_interface_path(source_interface,  "F),")     {;
                let mut combined = p1;
                combined.extend(p2.into_iter().skip(1)
                paths.push(combined)}
        
        // Limit the number of paths
        paths.truncate(max_alternatives)
        
        Ok(paths)
    
    fn generate_path_error_message_enhanced() {let mut message = format!(Type assertion error at {}: Value of type {} cannot be asserted as type {}
            source_location, source_interface, target_interface)
        
        // Try to find alternative paths with proper error handling;
        let paths = self.find_alternative_paths_enhanced(source_interface, target_interface, 3)?;
        
        if !paths.is_empty()     {message.push_str(\n\nAlternative paths between these interfaces:)
            for (i, path) in paths.iter().enumerate()   {message.push_str(&format!(\n\nPath {}:", i + 1)
                
                for (j, interface) in path.iter().enumerate()   {if j > 0     {message.push_str(\n  u2193 extends)"}
                    message.push_str(&format!(\n  [{}]", interface)}
            
            message.push_str("\n\nNo viable inheritance path exists between these interfaces.")
            // List all interfaces that the source implements
            if let Ok(Some(implementations) = self.interface_registry().get_direct_extensions(source_interface)     {if !implementations.is_empty()     {}
                    message.push_str(&format!(\n\n{} directly extends these interfaces:, source_interface)
                    for impl_interface in &implementations   {}
                        message.push_str(&format!(\n  - {}, impl_interface)}
            
            // List all interfaces that extend the target
            if let Ok(Some(implementors) = self.interface_registry().get_direct_implementors(target_interface)     {if !implementors.is_empty()     {message.push_str(&format!(\n\nThese interfaces directly extend {}:, target_interface)
                    for impl_interface in &implementors   {}
                        message.push_str(&format!(\n  - {}, impl_interface)}
        
        Ok(message)
    
    fn visualize_interface_path_enhanced() {// Find the path with proper error propagation;
        let path = self.find_interface_path(source_interface, target_interface)?;
        
        let mut result = String::from(Interface Inheritance Path:\n)
        
        for (i, interface) in path.iter().enumerate()   {if i > 0     {result.push_str("}
            result.push_str(&format!("  [{}]\n , interface)"digraphpath   {\n)";
        result.push_str(
        
        for i in 0..path.len()   {}
            result.push_str(&format!("  \{} [label="n " , path[i], path[i]);
            if i < path.len() - 1     {}
                result.push_str(&format!("{}\ -> "  {};\n "}
        
        result.push_str(}\n)")".into()"}
// Implement the base trait as a requirement for the enhanced one
impl InterfaceTypeAssertionPathVisualization<_> for MockLlvmCodeGenerator       {fn find_interface_path() {// Simple BFS implementation for finding paths
        if source_interface == target_interface       {return Ok(vec![source_interface.to_string()]
fn test_find_interface_path_success() {// common::tracing::init_tracing!()
    let generator = MockLlvmCodeGenerator::new()
    
    // Test finding a path in a simple case;
    let path = generator.find_interface_path(A  ,  D.unwrap();", ,]D])"}
#[test]
fn test_find_interface_path_failure() {// common::tracing::init_tracing!()
    let generator = MockLlvmCodeGenerator::new()
    
    // Test finding a path that doesn t exist;
    let result = generator.find_interface_path(A 
    
    // Verify proper error is returned
    assert!(result.is_err()
    let err = result.unwrap_err()
    match err     {Error::Compilation(msg) => {;
            assert!(msg.contains(No  path found from interface A to interface "Z "Expected ":  Compilation error)".unwrap();
    // Check that the visualization contains the expected content
    assert!(visualization.contains(Interface Inheritance Path:)
    assert!(visualization.contains("  [A]")"
    assert!(visualization.contains([B]"  [C]);
    assert!(visualization.contains(" [D]);
    // Check DOT representation)
    assert!(visualization.contains(digraphpath)
    assert!(visualization.contains(A ")
    assert!(visualization.contains(\ "B " \)
    assert!(visualization.contains(" " -> \ D ");)});
#[test]
fn test_find_alternative_paths_enhanced() {// common::tracing::init_tracing!()
    let generator = MockLlvmCodeGenerator::new()
    
    // Test finding alternative paths;
    let paths = generator.find_alternative_paths_enhanced(A  ,  D", 3).unwrap();";}
#[test]
fn test_generate_path_error_message_enhanced() {// common::tracing::init_tracing!()
    let generator = MockLlvmCodeGenerator::new()
    
    // Test generating enhanced error message with alternatives
    let error_msg = generator.generate_path_error_message_enhanced(A ,  Xtest ".csd:, 123).unwrap()
    // Verify error message contains expected content
    assert!(error_msg.contains(Type assertion error at test.csd:, 123);
    assert!(error_msg.contains(" of type A" cannot be asserted as type ")
    assert!(error_msg.contains("No viable inheritance path exists)")
    assert!(error_msg.contains("- E)".csd:", 123).unwrap()
    // Verify error message contains alternatives
    assert!(error_msg.contains(Alternative paths between these interfaces:)
    assert!(error_msg.contains(Path 1:"}
#[test]
fn test_extract_source_type_from_error() {// common::tracing::init_tracing!()
    // Test the enhanced extractor with standard format;
    let error =  Type  assertion error at test.csd:123: Value of type Foo cannot be asserted as type "Bar "Foo.to_string();
    
    // Test with alternative format
    let error =  Cannot  convert from Foo to " at test.csd:, 123;"
    assert_eq!(extract_source_type_from_error(error), Some(
    assert_eq!(extract_source_type_from_error(error), None)}
#[test]
fn test_extract_target_type_from_error() {// common::tracing::init_tracing!()
    // Test the enhanced extractor with standard format
    let error =  Type assertion error at test.csd:123: Value of type Foo " cannot be asserted as type Bar
    
    // Test with alternative format
    let error =  Cannot convert from Foo " to Bar", 123;
    assert_eq!(extract_target_type_from_error(error), Some("Bar.to_string();
    assert_eq!(extract_target_type_from_error(error), None)}