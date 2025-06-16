
//! Documentation System Integration Tests
//! 
//! Comprehensive end-to-end testing of the entire documentation generation
//! system. These tests validate the complete workflow from CURSED source
//! files to generated documentation in multiple formats.

use cursed::documentation::{DocumentationSystem, DocumentationConfig, OutputFormat};
use cursed::documentation::generator::DocumentationGenerator;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;
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

    fn create_test_documentation_config(temp_dir: &TempDir) -> DocumentationConfig {
        DocumentationConfig {
            source_dirs: vec![temp_dir.path().join("src")],
            output_dir: temp_dir.path().join("docs"),
            output_formats: vec![OutputFormat::Html, OutputFormat::Markdown],
            project: cursed::documentation::ProjectMetadata {
                name: "Test CURSED Project".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test project for documentation generation".to_string()),
                authors: vec!["Test Author".to_string()],
                homepage: Some("https://example.com".to_string()),
                repository: Some("https://github.com/test/cursed".to_string()),
                license: Some("MIT".to_string()),
            },
            options: cursed::documentation::DocOptions {
                include_private: false,
                include_source: true,
                generate_cross_refs: true,
                generate_search_index: true,
                include_examples: true,
                max_type_depth: 10,
                include_dependencies: false,
            },
            styling: cursed::documentation::StylingConfig {
                custom_css: vec![],
                template_dir: None,
                theme: "light".to_string(),
                colors: None,
                favicon: None,
                logo: None,
            },
        }
    }

    fn create_sample_cursed_files(src_dir: &std::path::Path) -> std::io::Result<()> {
        fs::create_dir_all(src_dir)?;
        
        // Create main.csd
        let main_content = r#"
//! Main module for the CURSED test project
//! 
//! This module demonstrates various CURSED language features
//! and serves as the entry point for the application.
//! 
//! @author Test Author
//! @version 1.0.0
//! @since 1.0.0

import "stdlib::io";
import "./math_utils";
import "./geometry";

/// Application entry point
/// 
/// Initializes the application and runs the main logic.
/// This function demonstrates error handling and module usage.
/// 
/// @return Exit code (0 for success, non-zero for failure)
/// @example
/// // Run the application
/// let exit_code = main();
/// assert_eq!(exit_code, 0);
/// @throws StartupError If initialization fails
/// @author Main Team
slay main() -> i32 {
    println("Welcome to CURSED Test Project!")?;
    
    // Demonstrate math utilities
    facts result = math_utils::fibonacci(10);
    println(&format!("Fibonacci(10) = {}", result))?;
    
    // Demonstrate geometry
    facts point = geometry::Point::new(3.0, 4.0);
    facts distance = point.distance_from_origin();
    println(&format!("Distance from origin: {}", distance))?;
    
    return 0;
}

/// Global application configuration
/// 
/// Stores runtime configuration settings loaded at startup.
/// These settings control various aspects of application behavior.
/// 
/// @since 1.0.0
sus mut APP_CONFIG: AppConfig = AppConfig::default();

/// Maximum number of retries for operations
/// 
/// This constant defines the maximum number of times the application
/// will retry failed operations before giving up.
/// 
/// @author Config Team
facts MAX_RETRIES: i32 = 3;
"#;
        fs::write(src_dir.join("main.csd"), main_content)?;
        
        // Create math_utils.csd
        let math_utils_content = r#"
//! Mathematical utility functions
//! 
//! This module provides common mathematical operations and algorithms
//! used throughout the CURSED test project. All functions are optimized
//! for performance and accuracy.
//! 
//! @author Math Team
//! @version 1.2.0
//! @since 1.0.0

/// Calculate the fibonacci number using recursion
/// 
/// This function calculates the nth number in the Fibonacci sequence
/// using a recursive approach. For large values of n, consider using
/// an iterative approach for better performance.
/// 
/// @param n The position in the Fibonacci sequence (must be >= 0)
/// @return The Fibonacci number at position n
/// @example
/// let fib5 = fibonacci(5);
/// assert_eq!(fib5, 5);
/// 
/// let fib10 = fibonacci(10);
/// assert_eq!(fib10, 55);
/// @complexity Time: O(2^n), Space: O(n)
/// @throws OverflowError If result exceeds i32 range
/// @author Fibonacci Team
/// @since 1.0.0
slay fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

/// Calculate the greatest common divisor of two numbers
/// 
/// Uses the Euclidean algorithm to find the GCD efficiently.
/// This is a fundamental operation used in many mathematical
/// calculations and cryptographic algorithms.
/// 
/// @param a First number (must be > 0)
/// @param b Second number (must be > 0)
/// @return The greatest common divisor of a and b
/// @example
/// let gcd_result = gcd(48, 18);
/// assert_eq!(gcd_result, 6);
/// @complexity Time: O(log(min(a, b))), Space: O(1)
/// @author Number Theory Team
/// @since 1.1.0
slay gcd(a: i32, b: i32) -> i32 {
    lowkey (b == 0) {
        return a;
    }
    return gcd(b, a % b);
}

/// Mathematical constants module
/// 
/// Contains commonly used mathematical constants with high precision.
/// These constants are used throughout the mathematical calculations.
/// 
/// @author Constants Team
mod constants {
    /// The mathematical constant π (pi)
    /// 
    /// The ratio of a circle's circumference to its diameter.
    /// Used in geometric calculations and trigonometry.
    /// 
    /// @precision 15 decimal places
    /// @since 1.0.0
    facts PI: f64 = 3.141592653589793;
    
    /// The mathematical constant e (Euler's number)
    /// 
    /// The base of natural logarithms, approximately 2.71828.
    /// Used in exponential and logarithmic calculations.
    /// 
    /// @precision 15 decimal places
    /// @since 1.0.0
    facts E: f64 = 2.718281828459045;
}
"#;
        fs::write(src_dir.join("math_utils.csd"), math_utils_content)?;
        
        // Create geometry.csd
        let geometry_content = r#"
//! Geometric types and operations
//! 
//! This module provides fundamental geometric types and operations
//! for 2D and 3D coordinate systems. All types implement standard
//! mathematical operations and transformations.
//! 
//! @author Geometry Team
//! @version 2.0.0
//! @since 1.0.0

/// Represents a point in 2D space
/// 
/// A point is defined by its x and y coordinates in a Cartesian
/// coordinate system. This struct provides basic operations for
/// working with 2D points including distance calculations.
/// 
/// @example
/// let origin = Point::new(0.0, 0.0);
/// let point = Point::new(3.0, 4.0);
/// let distance = point.distance_to(&origin);
/// assert_eq!(distance, 5.0);
/// @author Point Team
/// @since 1.0.0
squad Point {
    /// X coordinate of the point
    /// 
    /// The horizontal position in the coordinate system.
    /// Positive values indicate positions to the right of the origin.
    x: f64,
    
    /// Y coordinate of the point
    /// 
    /// The vertical position in the coordinate system.
    /// Positive values indicate positions above the origin.
    y: f64,
}

impl Point {
    /// Create a new point with the given coordinates
    /// 
    /// @param x The x coordinate
    /// @param y The y coordinate
    /// @return A new Point instance
    /// @example
    /// let point = Point::new(1.0, 2.0);
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, 2.0);
    /// @since 1.0.0
    slay new(x: f64, y: f64) -> Self {
        return Point { x, y };
    }
    
    /// Calculate the distance from this point to the origin
    /// 
    /// Uses the Pythagorean theorem to calculate the Euclidean
    /// distance from this point to the origin (0, 0).
    /// 
    /// @return The distance to the origin
    /// @example
    /// let point = Point::new(3.0, 4.0);
    /// let distance = point.distance_from_origin();
    /// assert_eq!(distance, 5.0);
    /// @complexity Time: O(1), Space: O(1)
    /// @since 1.0.0
    slay distance_from_origin(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }
    
    /// Calculate the distance between this point and another point
    /// 
    /// @param other The other point to calculate distance to
    /// @return The Euclidean distance between the points
    /// @example
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// let distance = p1.distance_to(&p2);
    /// assert_eq!(distance, 5.0);
    /// @since 1.1.0
    slay distance_to(&self, other: &Point) -> f64 {
        facts dx = self.x - other.x;
        facts dy = self.y - other.y;
        return (dx * dx + dy * dy).sqrt();
    }
}

/// Represents a rectangle defined by two points
/// 
/// A rectangle is defined by its top-left and bottom-right corners.
/// This provides a convenient way to work with rectangular regions
/// in 2D space.
/// 
/// @example
/// let rect = Rectangle::new(
///     Point::new(0.0, 0.0),
///     Point::new(10.0, 5.0)
/// );
/// let area = rect.area();
/// assert_eq!(area, 50.0);
/// @author Rectangle Team
/// @since 2.0.0
squad Rectangle {
    /// Top-left corner of the rectangle
    top_left: Point,
    
    /// Bottom-right corner of the rectangle
    bottom_right: Point,
}

impl Rectangle {
    /// Create a new rectangle from two points
    /// 
    /// @param top_left The top-left corner point
    /// @param bottom_right The bottom-right corner point
    /// @return A new Rectangle instance
    /// @example
    /// let rect = Rectangle::new(
    ///     Point::new(1.0, 1.0),
    ///     Point::new(5.0, 3.0)
    /// );
    /// @since 2.0.0
    slay new(top_left: Point, bottom_right: Point) -> Self {
        return Rectangle { top_left, bottom_right };
    }
    
    /// Calculate the area of the rectangle
    /// 
    /// @return The area as width × height
    /// @example
    /// let rect = Rectangle::new(
    ///     Point::new(0.0, 0.0),
    ///     Point::new(4.0, 3.0)
    /// );
    /// assert_eq!(rect.area(), 12.0);
    /// @since 2.0.0
    slay area(&self) -> f64 {
        facts width = self.bottom_right.x - self.top_left.x;
        facts height = self.bottom_right.y - self.top_left.y;
        return width * height;
    }
}

/// Drawable interface for geometric shapes
/// 
/// This interface defines the contract that all drawable geometric
/// shapes must implement. It provides a standard way to render
/// shapes in various graphics contexts.
/// 
/// @author Graphics Team
/// @since 2.0.0
collab Drawable {
    /// Draw the shape using the given graphics context
    /// 
    /// @param context The graphics context to draw on
    /// @param style The drawing style to use
    slay draw(&self, context: &GraphicsContext, style: &DrawStyle);
    
    /// Get the bounding box of the shape
    /// 
    /// @return A rectangle representing the shape's bounds
    slay get_bounds(&self) -> Rectangle;
}
"#;
        fs::write(src_dir.join("geometry.csd"), geometry_content)?;
        
        Ok(())
    }

    #[test]
    fn test_end_to_end_documentation_generation() {
        init_tracing!();
        info!("Testing end-to-end documentation generation");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_documentation_config(&temp_dir);
        
        // Create sample CURSED source files
        let src_dir = temp_dir.path().join("src");
        create_sample_cursed_files(&src_dir).unwrap();
        
        // Create documentation system
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        
        // Generate documentation
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        let doc_result = result.unwrap();
        assert!(doc_result.files_processed > 0);
        assert!(doc_result.items_documented > 0);
        assert!(doc_result.processing_time_ms > 0);
        
        // Check that output files were created
        let output_dir = temp_dir.path().join("docs");
        assert!(output_dir.exists());
        
        // Check for HTML files
        assert!(output_dir.join("index.html").exists());
        assert!(output_dir.join("main.html").exists());
        assert!(output_dir.join("math_utils.html").exists());
        assert!(output_dir.join("geometry.html").exists());
        
        // Check for Markdown files
        assert!(output_dir.join("README.md").exists());
        assert!(output_dir.join("main.md").exists());
        assert!(output_dir.join("math_utils.md").exists());
        assert!(output_dir.join("geometry.md").exists());
        
        // Check for search index
        assert!(output_dir.join("search.js").exists());
        
        // Check for CSS
        assert!(output_dir.join("styles.css").exists());
        
        debug!("End-to-end documentation generation completed successfully");
    }

    #[test]
    fn test_search_index_generation() {
        init_tracing!();
        info!("Testing search index generation");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_documentation_config(&temp_dir);
        
        let src_dir = temp_dir.path().join("src");
        create_sample_cursed_files(&src_dir).unwrap();
        
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Check search index was generated
        let search_index = doc_system.search_index();
        assert!(!search_index.is_empty());
        
        // Verify specific items are in the index
        let fibonacci_entry = search_index.iter()
            .find(|entry| entry.title == "fibonacci");
        assert!(fibonacci_entry.is_some());
        
        let point_entry = search_index.iter()
            .find(|entry| entry.title == "Point");
        assert!(point_entry.is_some());
        
        let main_entry = search_index.iter()
            .find(|entry| entry.title == "main");
        assert!(main_entry.is_some());
        
        // Check that search index file was written
        let search_file = temp_dir.path().join("docs").join("search.js");
        assert!(search_file.exists());
        
        let search_content = fs::read_to_string(search_file).unwrap();
        assert!(search_content.contains("fibonacci"));
        assert!(search_content.contains("Point"));
        assert!(search_content.contains("const searchIndex = ["));
        
        debug!("Search index generation tested successfully");
    }

    #[test]
    fn test_html_output_quality() {
        init_tracing!();
        info!("Testing HTML output quality");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_documentation_config(&temp_dir);
        
        let src_dir = temp_dir.path().join("src");
        create_sample_cursed_files(&src_dir).unwrap();
        
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Read and validate HTML content
        let index_html = fs::read_to_string(temp_dir.path().join("docs").join("index.html")).unwrap();
        
        // Check HTML structure
        assert!(index_html.contains("<!DOCTYPE html>"));
        assert!(index_html.contains("<html"));
        assert!(index_html.contains("</html>"));
        assert!(index_html.contains("<head>"));
        assert!(index_html.contains("</head>"));
        assert!(index_html.contains("<body>"));
        assert!(index_html.contains("</body>"));
        
        // Check content
        assert!(index_html.contains("Test CURSED Project"));
        assert!(index_html.contains("A test project for documentation generation"));
        
        // Check navigation
        assert!(index_html.contains("<nav"));
        assert!(index_html.contains("main.html"));
        assert!(index_html.contains("math_utils.html"));
        assert!(index_html.contains("geometry.html"));
        
        // Check module HTML files
        let math_html = fs::read_to_string(temp_dir.path().join("docs").join("math_utils.html")).unwrap();
        assert!(math_html.contains("fibonacci"));
        assert!(math_html.contains("Calculate the fibonacci number"));
        assert!(math_html.contains("@param n"));
        assert!(math_html.contains("@return"));
        assert!(math_html.contains("@example"));
        
        debug!("HTML output quality validated successfully");
    }

    #[test]
    fn test_markdown_output_quality() {
        init_tracing!();
        info!("Testing Markdown output quality");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_documentation_config(&temp_dir);
        
        let src_dir = temp_dir.path().join("src");
        create_sample_cursed_files(&src_dir).unwrap();
        
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Read and validate Markdown content
        let readme_md = fs::read_to_string(temp_dir.path().join("docs").join("README.md")).unwrap();
        
        // Check README structure
        assert!(readme_md.contains("# Test CURSED Project"));
        assert!(readme_md.contains("## Overview"));
        assert!(readme_md.contains("## Modules"));
        assert!(readme_md.contains("A test project for documentation generation"));
        
        // Check module links
        assert!(readme_md.contains("[main](main.md)"));
        assert!(readme_md.contains("[math_utils](math_utils.md)"));
        assert!(readme_md.contains("[geometry](geometry.md)"));
        
        // Check module Markdown files
        let math_md = fs::read_to_string(temp_dir.path().join("docs").join("math_utils.md")).unwrap();
        assert!(math_md.contains("# math_utils"));
        assert!(math_md.contains("## fibonacci"));
        assert!(math_md.contains("### Parameters"));
        assert!(math_md.contains("### Returns"));
        assert!(math_md.contains("### Examples"));
        assert!(math_md.contains("```cursed"));
        
        debug!("Markdown output quality validated successfully");
    }

    #[test]
    fn test_error_handling_and_recovery() {
        init_tracing!();
        info!("Testing error handling and recovery");
        
        let temp_dir = TempDir::new().unwrap();
        let mut config = create_test_documentation_config(&temp_dir);
        
        // Point to a non-existent source directory
        config.source_dirs = vec![temp_dir.path().join("nonexistent")];
        
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        
        // Should not fail completely, but should handle missing directories gracefully
        assert!(result.is_ok());
        let doc_result = result.unwrap();
        assert_eq!(doc_result.files_processed, 0);
        
        debug!("Error handling and recovery tested successfully");
    }
}