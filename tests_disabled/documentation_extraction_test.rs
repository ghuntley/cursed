//! Documentation Extraction from AST Tests
//! 
//! Tests the documentation extraction system that walks the CURSED AST
//! and builds comprehensive documentation structures. This is critical
//! for accurate API documentation generation.

use cursed::documentation::generator::{DocumentationGenerator, DocGeneratorConfig};
use cursed::documentation::{ExtractedDocumentation, DocumentationConfig};
use cursed::ast::*;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::SourceLocation;
use std::path::PathBuf;
use tracing::{debug, info};

#[path = "common.rs"]
mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_generator() -> DocumentationGenerator {
        let config = DocumentationConfig::default();
        DocumentationGenerator::new(config).unwrap()
    }

    fn parse_cursed_source(source: &str) -> Program {
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_documentation_generator_creation() {
        init_tracing!();
        info!("Testing documentation generator creation");
        
        let generator = create_test_generator();
        assert!(generator.config().project.name == "CURSED Project");
        debug!("Documentation generator created successfully");
    }

    #[test]
    fn test_function_documentation_extraction() {
        init_tracing!();
        info!("Testing function documentation extraction");
        
        let source = r#"
/// Calculate the factorial of a number
/// 
/// This function uses recursion to calculate the factorial.
/// The time complexity is O(n) and space complexity is O(n).
/// 
/// @param n The number to calculate factorial for (must be >= 0)
/// @return The factorial result
/// @example
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// @throws OverflowError If the result is too large
/// @since 1.0.0
/// @author Math Team
slay factorial(n: i32) -> i32 {
    lowkey (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_factorial.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        assert_eq!(extracted.functions.len(), 1);
        
        let func_doc = &extracted.functions[0];
        assert_eq!(func_doc.name, "factorial");
        assert!(func_doc.description.as_ref().unwrap().contains("recursion"));
        assert!(func_doc.description.as_ref().unwrap().contains("O(n)"));
        
        // Check parameters
        assert_eq!(func_doc.parameters.len(), 1);
        assert_eq!(func_doc.parameters[0].name, "n");
        assert_eq!(func_doc.parameters[0].param_type, "i32");
        
        // Check return type
        assert_eq!(func_doc.return_type.as_ref().unwrap(), "i32");
        
        debug!("Function documentation extracted successfully: {:?}", func_doc.name);
    }

    #[test]
    fn test_struct_documentation_extraction() {
        init_tracing!();
        info!("Testing struct documentation extraction");
        
        let source = r#"
/// Represents a point in 2D space
/// 
/// This struct provides basic coordinate storage and operations
/// for working with 2D points in geometric calculations.
/// 
/// @example
/// let point = Point { x: 10.0, y: 20.0 };
/// let distance = point.distance_from_origin();
/// @author Geometry Team
squad Point {
    /// X coordinate
    x: f64,
    /// Y coordinate  
    y: f64,
}

impl Point {
    /// Calculate distance from origin
    /// @return The distance as a floating point number
    slay distance_from_origin(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_point.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        assert_eq!(extracted.types.len(), 1);
        
        let type_doc = &extracted.types[0];
        assert_eq!(type_doc.name, "Point");
        assert!(type_doc.description.as_ref().unwrap().contains("2D space"));
        assert!(type_doc.description.as_ref().unwrap().contains("coordinate storage"));
        
        // Check fields
        assert_eq!(type_doc.fields.len(), 2);
        assert_eq!(type_doc.fields[0].name, "x");
        assert_eq!(type_doc.fields[1].name, "y");
        
        // Check methods
        assert_eq!(type_doc.methods.len(), 1);
        assert_eq!(type_doc.methods[0].name, "distance_from_origin");
        
        debug!("Struct documentation extracted successfully: {:?}", type_doc.name);
    }

    #[test]
    fn test_interface_documentation_extraction() {
        init_tracing!();
        info!("Testing interface documentation extraction");
        
        let source = r#"
/// Defines a drawable object interface
/// 
/// This interface provides methods that all drawable objects
/// must implement for rendering in the graphics system.
/// 
/// @author Graphics Team
/// @since 2.0.0
collab Drawable {
    /// Draw the object on the screen
    /// @param context The graphics context to draw on
    /// @param x The x coordinate to draw at
    /// @param y The y coordinate to draw at
    slay draw(context: &GraphicsContext, x: f64, y: f64);
    
    /// Get the bounding box of the object
    /// @return The bounding box rectangle
    slay get_bounds() -> Rectangle;
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_drawable.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        assert_eq!(extracted.types.len(), 1);
        
        let interface_doc = &extracted.types[0];
        assert_eq!(interface_doc.name, "Drawable");
        assert!(interface_doc.description.as_ref().unwrap().contains("drawable object"));
        assert!(interface_doc.description.as_ref().unwrap().contains("graphics system"));
        
        // Check methods
        assert_eq!(interface_doc.methods.len(), 2);
        assert_eq!(interface_doc.methods[0].name, "draw");
        assert_eq!(interface_doc.methods[1].name, "get_bounds");
        
        debug!("Interface documentation extracted successfully: {:?}", interface_doc.name);
    }

    #[test]
    fn test_generic_type_documentation_extraction() {
        init_tracing!();
        info!("Testing generic type documentation extraction");
        
        let source = r#"
/// Generic container for any type of value
/// 
/// This container provides type-safe storage and operations
/// for values of any type T that implements the Clone trait.
/// 
/// @param T The type of value to store (must implement Clone)
/// @example
/// let container = Container::new(42);
/// let value = container.get();
/// @author Collections Team
squad Container<T: Clone> {
    /// The stored value
    value: T,
}

impl<T: Clone> Container<T> {
    /// Create a new container with the given value
    /// @param value The initial value to store
    /// @return A new container instance
    slay new(value: T) -> Self {
        return Container { value };
    }
    
    /// Get a copy of the stored value
    /// @return A clone of the stored value
    slay get(&self) -> T {
        return self.value.clone();
    }
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_container.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        assert_eq!(extracted.types.len(), 1);
        
        let generic_doc = &extracted.types[0];
        assert_eq!(generic_doc.name, "Container");
        assert!(generic_doc.description.as_ref().unwrap().contains("Generic container"));
        assert!(generic_doc.description.as_ref().unwrap().contains("type-safe storage"));
        
        // Check generic parameters
        assert!(generic_doc.type_parameters.len() >= 1);
        
        // Check methods
        assert_eq!(generic_doc.methods.len(), 2);
        assert_eq!(generic_doc.methods[0].name, "new");
        assert_eq!(generic_doc.methods[1].name, "get");
        
        debug!("Generic type documentation extracted successfully: {:?}", generic_doc.name);
    }

    #[test]
    fn test_constant_and_variable_extraction() {
        init_tracing!();
        info!("Testing constant and variable documentation extraction");
        
        let source = r#"
/// Mathematical constant PI
/// 
/// The ratio of a circle's circumference to its diameter.
/// Used in geometric calculations and trigonometry.
/// 
/// @since 1.0.0
/// @author Math Constants Team
facts PI: f64 = 3.14159265358979323846;

/// Maximum allowed connections
/// 
/// The maximum number of concurrent connections the server
/// can handle before rejecting new requests.
/// 
/// @author Network Team
sus MAX_CONNECTIONS: i32 = 1000;

/// Application configuration
/// 
/// Global configuration object loaded at startup.
/// Contains all runtime settings for the application.
sus mut config: AppConfig = AppConfig::default();
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_constants.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        
        // Check constants
        assert!(extracted.constants.len() >= 1);
        let pi_constant = extracted.constants.iter()
            .find(|c| c.name == "PI")
            .expect("PI constant should be found");
        assert!(pi_constant.description.as_ref().unwrap().contains("circumference"));
        
        // Check variables
        assert!(extracted.variables.len() >= 2);
        let max_conn_var = extracted.variables.iter()
            .find(|v| v.name == "MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS variable should be found");
        assert!(max_conn_var.description.as_ref().unwrap().contains("concurrent connections"));
        
        debug!("Constants and variables extracted successfully");
    }

    #[test]
    fn test_module_documentation_extraction() {
        init_tracing!();
        info!("Testing module documentation extraction");
        
        let source = r#"
//! Mathematical Utilities Module
//! 
//! This module provides common mathematical functions and constants
//! for use throughout the application. It includes basic arithmetic,
//! trigonometric functions, and statistical operations.
//! 
//! @author Math Team
//! @version 2.1.0
//! @since 1.0.0

/// Calculate the square root using Newton's method
/// @param x The number to find the square root of
/// @return The square root approximation
slay sqrt(x: f64) -> f64 {
    // Implementation here
    return x.sqrt();
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("math_utils.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        
        // Check module documentation
        assert!(extracted.module_doc.is_some());
        let module_doc = extracted.module_doc.unwrap();
        assert_eq!(module_doc.name, "math_utils");
        assert!(module_doc.description.as_ref().unwrap().contains("Mathematical Utilities"));
        assert!(module_doc.description.as_ref().unwrap().contains("trigonometric functions"));
        
        // Check that functions are also extracted
        assert_eq!(extracted.functions.len(), 1);
        assert_eq!(extracted.functions[0].name, "sqrt");
        
        debug!("Module documentation extracted successfully: {:?}", module_doc.name);
    }

    #[test]
    fn test_complex_nested_extraction() {
        init_tracing!();
        info!("Testing complex nested documentation extraction");
        
        let source = r#"
//! Complex data structures module

/// A binary tree node for generic data
/// @param T The type of data stored in the node
squad TreeNode<T> {
    /// The data stored in this node
    data: T,
    /// Left child node
    left: Option<Box<TreeNode<T>>>,
    /// Right child node  
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> TreeNode<T> {
    /// Insert a new value into the tree
    /// @param value The value to insert
    slay insert(&mut self, value: T) {
        // Implementation
    }
    
    /// Search for a value in the tree
    /// @param value The value to search for
    /// @return True if found, false otherwise
    slay search(&self, value: &T) -> bool {
        // Implementation
        return false;
    }
}

/// Tree traversal algorithms
/// @author Algorithm Team
mod traversal {
    /// Perform in-order traversal
    /// @param root The root node to start from
    /// @return Vector of values in order
    slay in_order<T>(root: &TreeNode<T>) -> Vec<&T> {
        // Implementation
        return Vec::new();
    }
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("tree_structures.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        
        // Check complex type extraction
        assert_eq!(extracted.types.len(), 1);
        let tree_node = &extracted.types[0];
        assert_eq!(tree_node.name, "TreeNode");
        assert_eq!(tree_node.fields.len(), 3);
        assert_eq!(tree_node.methods.len(), 2);
        
        // Check submodule extraction
        assert!(extracted.submodules.len() >= 1);
        
        debug!("Complex nested documentation extracted successfully");
    }

    #[test]
    fn test_error_handling_in_extraction() {
        init_tracing!();
        info!("Testing error handling during extraction");
        
        let generator = create_test_generator();
        
        // Test with malformed AST (empty program)
        let empty_ast = Program {
            statements: vec![],
            comments: vec![],
        };
        let file_path = PathBuf::from("empty.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&empty_ast, &file_path, "")
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        assert_eq!(extracted.functions.len(), 0);
        assert_eq!(extracted.types.len(), 0);
        assert_eq!(extracted.constants.len(), 0);
        
        debug!("Error handling in extraction tested successfully");
    }

    #[test]
    fn test_source_location_preservation() {
        init_tracing!();
        info!("Testing source location preservation in extraction");
        
        let source = r#"
/// Test function
slay test_func() -> i32 {
    return 42;
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_location.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        assert_eq!(extracted.functions.len(), 1);
        
        let func_doc = &extracted.functions[0];
        assert_eq!(func_doc.name, "test_func");
        
        // Check that source location is preserved
        assert!(func_doc.location.line > 0);
        assert!(func_doc.location.column >= 0);
        
        debug!("Source location preserved: line {}, column {}", 
               func_doc.location.line, func_doc.location.column);
    }

    #[test]
    fn test_extraction_metadata() {
        init_tracing!();
        info!("Testing extraction metadata generation");
        
        let source = r#"
/// Function one
slay func_one() {}

/// Function two  
slay func_two() {}

/// Test struct
squad TestStruct {
    field: i32,
}
"#;
        
        let ast = parse_cursed_source(source);
        let generator = create_test_generator();
        let file_path = PathBuf::from("test_metadata.csd");
        
        let result = tokio_test::block_on(
            generator.extract_from_ast(&ast, &file_path, source)
        );
        assert!(result.is_ok());
        
        let extracted = result.unwrap();
        
        // Check metadata
        assert!(extracted.metadata.extracted_at <= chrono::Utc::now());
        assert!(!extracted.metadata.generator_version.is_empty());
        assert_eq!(extracted.metadata.item_count, 3); // 2 functions + 1 struct
        assert!(extracted.metadata.processing_time_ms >= 0);
        
        debug!("Extraction metadata generated: {} items in {}ms", 
               extracted.metadata.item_count, extracted.metadata.processing_time_ms);
    }
}
