//! # Tests for Enhanced Interface Type Assertion Path Visualization
//!
//! This module tests the enhanced interface type assertion path visualization system
//! with improved error handling and consistent error propagation through the `?` operator.

use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;

// Import the modules we need to test
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::*;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;

// Import the common test utilities
use crate::common;

#[path = "common.rs"]
mod common;

/// Set up a fixture for tests with a populated interface hierarchy
fn setup_interface_hierarchy() -> ThreadSafeInterfaceExtensionRegistry {
    // Initialize tracing for this test
    common::tracing::setup();
    
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Set up a sample interface hierarchy for testing
    // A -> B -> C -> D
    // |    |    ^
    // v    v    |
    // E -> F ---+
    // Simple diamond-like inheritance pattern
    
    registry.register_extension("A", "B").unwrap();
    registry.register_extension("A", "E").unwrap();
    registry.register_extension("B", "C").unwrap();
    registry.register_extension("B", "F").unwrap();
    registry.register_extension("E", "F").unwrap();
    registry.register_extension("F", "C").unwrap();
    registry.register_extension("C", "D").unwrap();
    
    // Add some isolated interfaces for testing error cases
    registry.register_extension("X", "Y").unwrap();
    registry.register_extension("Y", "Z").unwrap();
    
    registry
}

/// Mock LLVM CodeGenerator for testing that holds a registry but doesn't need LLVM initialization
struct MockLlvmCodeGenerator {
    registry_extensions: ThreadSafeInterfaceExtensionRegistry,
}

impl MockLlvmCodeGenerator {
    fn new() -> Self {
        MockLlvmCodeGenerator {
            registry_extensions: setup_interface_hierarchy(),
        }
    }
    
    fn interface_registry(&self) -> &ThreadSafeInterfaceExtensionRegistry {
        &self.registry_extensions
    }
}

// Implement the necessary trait for our mock
impl EnhancedInterfaceTypeAssertionPathVisualization<'_> for MockLlvmCodeGenerator {
    fn generate_interface_hierarchy_dot_enhanced(&self) -> Result<String, Error> {
        let mut dot = String::from("digraph interface_hierarchy {\n");
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");
        
        // Get the complete hierarchy from the registry with proper error propagation
        let hierarchy = self.interface_registry().get_extension_hierarchy()?;
        
        // Add all nodes first
        let mut all_interfaces = HashSet::new();
        
        // Collect all interface names
        for (source, targets) in &hierarchy {
            all_interfaces.insert(source.clone());
            for target in targets {
                all_interfaces.insert(target.clone());
            }
        }
        
        // Add nodes to DOT
        for interface in &all_interfaces {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        // Add edges
        for (source, targets) in &hierarchy {
            for target in targets {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
            }
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }
    
    fn find_alternative_paths_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        // Implementation would call find_interface_path for various combinations
        // For testing, we'll create a simplified version
        
        // Simple implementation for testing
        let mut paths = Vec::new();
        
        // Try a path through "C" if neither source nor target is "C"
        if source_interface != "C" && target_interface != "C" {
            if let (Ok(p1), Ok(p2)) = (
                self.find_interface_path(source_interface, "C"),
                self.find_interface_path("C", target_interface)
            ) {
                let mut combined = p1;
                combined.extend(p2.into_iter().skip(1));
                paths.push(combined);
            }
        }
        
        // Try a path through "F" if neither source nor target is "F"
        if source_interface != "F" && target_interface != "F" {
            if let (Ok(p1), Ok(p2)) = (
                self.find_interface_path(source_interface, "F"),
                self.find_interface_path("F", target_interface)
            ) {
                let mut combined = p1;
                combined.extend(p2.into_iter().skip(1));
                paths.push(combined);
            }
        }
        
        // Limit the number of paths
        paths.truncate(max_alternatives);
        
        Ok(paths)
    }
    
    fn generate_path_error_message_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        let mut message = format!(
            "Type assertion error at {}: Value of type '{}' cannot be asserted as type '{}'",
            source_location, source_interface, target_interface
        );
        
        // Try to find alternative paths with proper error handling
        let paths = self.find_alternative_paths_enhanced(source_interface, target_interface, 3)?;
        
        if !paths.is_empty() {
            message.push_str("\n\nAlternative paths between these interfaces:");
            
            for (i, path) in paths.iter().enumerate() {
                message.push_str(&format!("\n\nPath {}:", i + 1));
                
                for (j, interface) in path.iter().enumerate() {
                    if j > 0 {
                        message.push_str("\n  u2193 extends");
                    }
                    message.push_str(&format!("\n  [{}]", interface));
                }
            }
            
            message.push_str("\n\nConsider implementing the missing interfaces in the hierarchy.");
        } else {
            message.push_str("\n\nNo viable inheritance path exists between these interfaces.");
            
            // List all interfaces that the source implements
            if let Ok(Some(implementations)) = self.interface_registry().get_direct_extensions(source_interface) {
                if !implementations.is_empty() {
                    message.push_str(&format!("\n\n'{}' directly extends these interfaces:", source_interface));
                    for impl_interface in &implementations {
                        message.push_str(&format!("\n  - {}", impl_interface));
                    }
                }
            }
            
            // List all interfaces that extend the target
            if let Ok(Some(implementors)) = self.interface_registry().get_direct_implementors(target_interface) {
                if !implementors.is_empty() {
                    message.push_str(&format!("\n\nThese interfaces directly extend '{}':", target_interface));
                    for impl_interface in &implementors {
                        message.push_str(&format!("\n  - {}", impl_interface));
                    }
                }
            }
        }
        
        Ok(message)
    }
    
    fn visualize_interface_path_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error> {
        // Find the path with proper error propagation
        let path = self.find_interface_path(source_interface, target_interface)?;
        
        let mut result = String::from("Interface Inheritance Path:\n");
        
        for (i, interface) in path.iter().enumerate() {
            if i > 0 {
                result.push_str("  u2193 extends\n");
            }
            result.push_str(&format!("  [{}]\n", interface));
        }
        
        // Add DOT representation for testing
        result.push_str("\nDOT representation:\n");
        result.push_str("digraph path {\n");
        result.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");
        
        for i in 0..path.len() {
            result.push_str(&format!("  \"{}\" [label=\"{}\"];\n", path[i], path[i]));
            
            if i < path.len() - 1 {
                result.push_str(&format!("  \"{}\" -> \"{}\";\n", path[i], path[i + 1]));
            }
        }
        
        result.push_str("}\n");
        
        Ok(result)
    }
    
    // This trait method requires actual LLVM, so mock it with an Err for our test purposes
    fn compile_type_assertion_with_path_visualization_enhanced(
        &mut self,
        _type_assertion: &cursed::ast::expressions::TypeAssertion,
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error> {
        Err(Error::Compilation("Mock implementation does not support full compilation".into()))
    }
}

// Implement the base trait as a requirement for the enhanced one
impl InterfaceTypeAssertionPathVisualization<'_> for MockLlvmCodeGenerator {
    fn find_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error> {
        // Simple BFS implementation for finding paths
        if source_interface == target_interface {
            return Ok(vec![source_interface.to_string()]);
        }
        
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut path_map = HashMap::new();
        
        // Start BFS from source_interface
        queue.push_back(source_interface.to_string());
        visited.insert(source_interface.to_string());
        path_map.insert(source_interface.to_string(), (None, vec![source_interface.to_string()]));
        
        // Perform BFS to find the shortest path
        while let Some(current) = queue.pop_front() {
            // Get direct extensions from the current interface
            match self.interface_registry().get_direct_extensions(&current) {
                Ok(Some(direct_extensions)) => {
                    for next in direct_extensions {
                        if !visited.contains(&next) {
                            visited.insert(next.clone());
                            queue.push_back(next.clone());
                            
                            // Update path
                            let mut new_path = path_map.get(&current).cloned().unwrap_or_else(|| (None, Vec::new())).1;
                            new_path.push(next.clone());
                            path_map.insert(next.clone(), (Some(current.clone()), new_path.clone()));
                            
                            // Check if we've reached the target
                            if &next == target_interface {
                                return Ok(new_path);
                            }
                        }
                    }
                },
                Ok(None) => {
                    // No direct extensions for this interface
                },
                Err(e) => {
                    // Proper error propagation
                    return Err(e);
                }
            }
        }
        
        // No path found
        Err(Error::Compilation(format!(
            "No path found from interface '{}' to interface '{}'",
            source_interface, target_interface
        )))
    }
    
    fn generate_interface_hierarchy_dot(&self) -> Result<String, Error> {
        // This will be replaced by the enhanced implementation when tested
        self.generate_interface_hierarchy_dot_enhanced()
    }
    
    fn visualize_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error> {
        // This will be replaced by the enhanced implementation when tested
        self.visualize_interface_path_enhanced(source_interface, target_interface)
    }
    
    fn generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        // This will be replaced by the enhanced implementation when tested
        self.generate_path_error_message_enhanced(source_interface, target_interface, source_location)
    }
    
    fn find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        // This will be replaced by the enhanced implementation when tested
        self.find_alternative_paths_enhanced(source_interface, target_interface, max_alternatives)
    }
    
    fn compile_type_assertion_with_path_visualization(
        &mut self,
        type_assertion: &cursed::ast::expressions::TypeAssertion,
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error> {
        // This will be replaced by the enhanced implementation when tested
        self.compile_type_assertion_with_path_visualization_enhanced(type_assertion)
    }
}

#[test]
fn test_generate_interface_hierarchy_dot_enhanced() {
    let generator = MockLlvmCodeGenerator::new();
    
    // Test the enhanced dot generation with proper error handling
    let dot = generator.generate_interface_hierarchy_dot_enhanced().unwrap();
    
    // Check that the DOT graph contains all the expected interfaces
    assert!(dot.contains("digraph interface_hierarchy"));
    assert!(dot.contains("\"A\" [label=\"A\"];"));
    assert!(dot.contains("\"B\" [label=\"B\"];"));
    assert!(dot.contains("\"C\" [label=\"C\"];"));
    assert!(dot.contains("\"D\" [label=\"D\"];"));
    assert!(dot.contains("\"E\" [label=\"E\"];"));
    assert!(dot.contains("\"F\" [label=\"F\"];"));
    
    // Check that the DOT graph contains all the expected edges
    assert!(dot.contains("\"A\" -> \"B\";"));
    assert!(dot.contains("\"A\" -> \"E\";"));
    assert!(dot.contains("\"B\" -> \"C\";"));
    assert!(dot.contains("\"C\" -> \"D\";"));
    assert!(dot.contains("\"B\" -> \"F\";"));
    assert!(dot.contains("\"E\" -> \"F\";"));
    assert!(dot.contains("\"F\" -> \"C\";"));
}

#[test]
fn test_find_interface_path_success() {
    let generator = MockLlvmCodeGenerator::new();
    
    // Test finding a path in a simple case
    let path = generator.find_interface_path("A", "D").unwrap();
    
    // Verify the path is correct (should be A -> B -> C -> D)
    assert_eq!(path, vec!["A", "B", "C", "D"]);
}

#[test]
fn test_find_interface_path_failure() {
    let generator = MockLlvmCodeGenerator::new();
    
    // Test finding a path that doesn't exist
    let result = generator.find_interface_path("A", "Z");
    
    // Verify proper error is returned
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        Error::Compilation(msg) => {
            assert!(msg.contains("No path found from interface 'A' to interface 'Z'"));
        },
        _ => panic!("Expected Compilation error")
    }
}

#[test]
fn test_visualize_interface_path_enhanced() {
    let generator = MockLlvmCodeGenerator::new();
    
    // Test the enhanced path visualization
    let visualization = generator.visualize_interface_path_enhanced("A", "D").unwrap();
    
    // Check that the visualization contains the expected content
    assert!(visualization.contains("Interface Inheritance Path:"));
    assert!(visualization.contains("  [A]"));
    assert!(visualization.contains("  ↓ extends"));
    assert!(visualization.contains("  [B]"));
    assert!(visualization.contains("  [C]"));
    assert!(visualization.contains("  [D]"));
    
    // Check DOT representation
    assert!(visualization.contains("digraph path"));
    assert!(visualization.contains("\"A\" -> \"B\";"));
    assert!(visualization.contains("\"B\" -> \"C\";"));
    assert!(visualization.contains("\"C\" -> \"D\";"));
}

#[test]
fn test_find_alternative_paths_enhanced() {
    let generator = MockLlvmCodeGenerator::new();
    
    // Test finding alternative paths
    let paths = generator.find_alternative_paths_enhanced("A", "D", 3).unwrap();
    
    // Should find at least one alternative path in our test data
    assert!(!paths.is_empty());
    
    // Verify the alternative paths are valid
    for path in &paths {
        assert!(path[0] == "A");
        assert!(path[path.len() - 1] == "D");
    }
}

#[test]
fn test_generate_path_error_message_enhanced() {
    let generator = MockLlvmCodeGenerator::new();
    
    // Test generating enhanced error message with alternatives
    let error_msg = generator.generate_path_error_message_enhanced("A", "X", "test.csd:123").unwrap();
    
    // Verify error message contains expected content
    assert!(error_msg.contains("Type assertion error at test.csd:123"));
    assert!(error_msg.contains("Value of type 'A' cannot be asserted as type 'X'"));
    assert!(error_msg.contains("No viable inheritance path exists"));
    
    // It should list what A extends
    assert!(error_msg.contains("'A' directly extends these interfaces:"));
    assert!(error_msg.contains("- B"));
    assert!(error_msg.contains("- E"));
    
    // Test with interfaces that have alternatives
    let error_msg = generator.generate_path_error_message_enhanced("A", "D", "test.csd:123").unwrap();
    
    // Verify error message contains alternatives
    assert!(error_msg.contains("Alternative paths between these interfaces:"));
    assert!(error_msg.contains("Path 1:"));
}

#[test]
fn test_extract_source_type_from_error() {
    // Test the enhanced extractor with standard format
    let error = "Type assertion error at test.csd:123: Value of type 'Foo' cannot be asserted as type 'Bar'";
    assert_eq!(extract_source_type_from_error(error), Some("Foo".to_string()));
    
    // Test with alternative format
    let error = "Cannot convert from 'Foo' to 'Bar' at test.csd:123";
    assert_eq!(extract_source_type_from_error(error), Some("Foo".to_string()));
    
    // Test with malformed error
    let error = "Type assertion error: value cannot be asserted";
    assert_eq!(extract_source_type_from_error(error), None);
}

#[test]
fn test_extract_target_type_from_error() {
    // Test the enhanced extractor with standard format
    let error = "Type assertion error at test.csd:123: Value of type 'Foo' cannot be asserted as type 'Bar'";
    assert_eq!(extract_target_type_from_error(error), Some("Bar".to_string()));
    
    // Test with alternative format
    let error = "Cannot convert from 'Foo' to 'Bar' at test.csd:123";
    assert_eq!(extract_target_type_from_error(error), Some("Bar".to_string()));
    
    // Test with malformed error
    let error = "Type assertion error: value cannot be asserted";
    assert_eq!(extract_target_type_from_error(error), None);
}