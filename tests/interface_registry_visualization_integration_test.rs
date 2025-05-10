//! # Interface Registry Visualization Integration Test
//!
//! This test module verifies the proper integration of the interface registry visualization trait
//! with comprehensive error handling and consistent error propagation.

#[path = "common.rs"]
mod common;

use common::tracing;
use cursed::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, ThreadSafeInterfaceRegistryVisualization};
use cursed::error::Error;

#[test]
fn test_interface_registry_visualization_integration() {
    tracing::setup();
    
    // Create a registry with multiple interfaces and extension relationships
    let registry = ThreadSafeInterfaceRegistryVisualization::new();
    
    // Set up interface inheritance relationships with proper error handling
    registry.register_extension("Reader", "FileReader").expect("Registration failed");
    registry.register_extension("Reader", "NetworkReader").expect("Registration failed");
    registry.register_extension("Writer", "FileWriter").expect("Registration failed");
    registry.register_extension("Writer", "NetworkWriter").expect("Registration failed");
    registry.register_extension("FileReader", "BinaryFileReader").expect("Registration failed");
    registry.register_extension("FileReader", "TextFileReader").expect("Registration failed");
    registry.register_extension("FileWriter", "BinaryFileWriter").expect("Registration failed");
    registry.register_extension("FileWriter", "TextFileWriter").expect("Registration failed");
    registry.register_extension("Serializer", "JSONSerializer").expect("Registration failed");
    registry.register_extension("Serializer", "XMLSerializer").expect("Registration failed");
    registry.register_extension("TextProcessor", "MarkdownProcessor").expect("Registration failed");
    registry.register_extension("TextProcessor", "HTMLProcessor").expect("Registration failed");
    
    // Get all interfaces to verify registration
    let all_interfaces = registry.get_all_interfaces().expect("Failed to get interfaces");
    assert!(all_interfaces.contains("Reader"));
    assert!(all_interfaces.contains("FileReader"));
    assert!(all_interfaces.contains("Writer"));
    assert!(all_interfaces.contains("NetworkWriter"));
    assert!(all_interfaces.contains("BinaryFileReader"));
    assert!(all_interfaces.contains("TextFileReader"));
    
    // Test getting direct extensions
    let reader_extensions = registry.get_direct_extensions("Reader").expect("Failed to get extensions");
    assert!(reader_extensions.is_some());
    let extensions = reader_extensions.unwrap();
    assert_eq!(extensions.len(), 2);
    assert!(extensions.contains(&"FileReader".to_string()));
    assert!(extensions.contains(&"NetworkReader".to_string()));
    
    // Test getting direct implementors
    let file_reader_implementors = registry.get_direct_implementors("FileReader").expect("Failed to get implementors");
    assert!(file_reader_implementors.is_some());
    let implementors = file_reader_implementors.unwrap();
    assert_eq!(implementors.len(), 1);
    assert!(implementors.contains(&"Reader".to_string()));
    
    // Test the extends relationship
    assert!(registry.extends("Reader", "FileReader").expect("Extension check failed"));
    assert!(registry.extends("Reader", "BinaryFileReader").expect("Extension check failed"));
    assert!(!registry.extends("Reader", "Writer").expect("Extension check failed"));
    
    // Test finding inheritance paths
    let path = registry.find_inheritance_path("Reader", "BinaryFileReader").expect("Failed to find path");
    assert_eq!(path, vec!["Reader", "FileReader", "BinaryFileReader"]);
    
    // Test finding all inheritance paths
    let paths = registry.find_all_inheritance_paths("Reader", "BinaryFileReader").expect("Failed to find paths");
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec!["Reader", "FileReader", "BinaryFileReader"]);
    
    // Test cycle detection with no cycles
    let cycles = registry.detect_cycles().expect("Failed to detect cycles");
    assert_eq!(cycles.len(), 0);
    
    // Create a cycle and test detection
    registry.register_extension("BinaryFileReader", "Reader").expect("Registration failed");
    let cycles = registry.detect_cycles().expect("Failed to detect cycles");
    assert!(cycles.len() > 0);
    
    // Test ASCII visualization
    let ascii = registry.visualize_hierarchy_ascii().expect("Failed to generate ASCII visualization");
    assert!(ascii.contains("Reader"));
    assert!(ascii.contains("FileReader"));
    assert!(ascii.contains("BinaryFileReader"));
    
    // Test DOT visualization
    let dot = registry.visualize_hierarchy_dot().expect("Failed to generate DOT visualization");
    assert!(dot.contains("Reader"));
    assert!(dot.contains("FileReader"));
    assert!(dot.contains("digraph"));
    
    // Test path visualization
    let path_ascii = registry.visualize_path_ascii(&["Reader".to_string(), "FileReader".to_string(), "BinaryFileReader".to_string()])
        .expect("Failed to generate path ASCII visualization");
    assert!(path_ascii.contains("Reader"));
    assert!(path_ascii.contains("FileReader"));
    assert!(path_ascii.contains("BinaryFileReader"));
    assert!(path_ascii.contains("extends"));
    
    // Test path DOT visualization
    let path_dot = registry.visualize_path_dot(&["Reader".to_string(), "FileReader".to_string(), "BinaryFileReader".to_string()])
        .expect("Failed to generate path DOT visualization");
    assert!(path_dot.contains("Reader"));
    assert!(path_dot.contains("FileReader"));
    assert!(path_dot.contains("BinaryFileReader"));
    assert!(path_dot.contains("digraph"));
    
    // Test error handling for non-existent paths
    match registry.find_inheritance_path("Reader", "JSONSerializer") {
        Ok(_) => panic!("Should not find a path between unrelated interfaces"),
        Err(Error::Compilation(msg)) => {
            assert!(msg.contains("No inheritance path found"));
            assert!(msg.contains("Reader"));
            assert!(msg.contains("JSONSerializer"));
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
    
    // Test error handling for all paths
    match registry.find_all_inheritance_paths("Reader", "JSONSerializer") {
        Ok(_) => panic!("Should not find paths between unrelated interfaces"),
        Err(Error::Compilation(msg)) => {
            assert!(msg.contains("No inheritance paths found"));
            assert!(msg.contains("Reader"));
            assert!(msg.contains("JSONSerializer"));
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
    
    // Test getting all extensions (transitive closure)
    let all_reader_extensions = registry.get_all_extensions("Reader").expect("Failed to get all extensions");
    assert!(all_reader_extensions.contains("FileReader"));
    assert!(all_reader_extensions.contains("NetworkReader"));
    assert!(all_reader_extensions.contains("BinaryFileReader"));
    assert!(all_reader_extensions.contains("TextFileReader"));
    assert!(!all_reader_extensions.contains("Writer"));
    
    // Test getting all implementors (reverse transitive closure)
    let all_binary_file_reader_implementors = registry.get_all_implementors("BinaryFileReader").expect("Failed to get all implementors");
    assert!(all_binary_file_reader_implementors.contains("FileReader"));
    assert!(all_binary_file_reader_implementors.contains("Reader"));
    assert!(!all_binary_file_reader_implementors.contains("TextFileReader"));
    assert!(!all_binary_file_reader_implementors.contains("Writer"));
}

// Test error propagation throughout the API
#[test]
fn test_error_propagation() {
    tracing::setup();
    
    let registry = ThreadSafeInterfaceRegistryVisualization::new();
    
    // Verify error propagation works consistently through multiple layers
    registry.register_extension("A", "B").expect("Registration failed");
    registry.register_extension("B", "C").expect("Registration failed");
    registry.register_extension("C", "D").expect("Registration failed");
    
    // Create a test function that uses multiple registry operations
    let test_operation = || -> Result<(), Error> {
        // Layer 1: Get extensions
        let extensions = registry.get_direct_extensions("A")?;
        assert!(extensions.is_some());
        
        // Layer 2: Find paths through extensions
        for ext in extensions.unwrap() {
            let path = registry.find_inheritance_path(&ext, "D")?;
            assert!(path.contains(&"C".to_string()));
            
            // Layer 3: Visualize the path
            let _visualization = registry.visualize_path_ascii(&path)?;
        }
        
        Ok(())
    };
    
    // Verify proper error propagation
    test_operation().expect("Operation failed, error propagation not working");
}

// Test thread safety
#[test]
fn test_thread_safety() {
    tracing::setup();
    
    let registry = std::sync::Arc::new(ThreadSafeInterfaceRegistryVisualization::new());
    
    // Register some interfaces
    registry.register_extension("Base", "Derived1").expect("Registration failed");
    registry.register_extension("Base", "Derived2").expect("Registration failed");
    registry.register_extension("Derived1", "DerivedA").expect("Registration failed");
    registry.register_extension("Derived2", "DerivedB").expect("Registration failed");
    
    // Create multiple threads that access the registry concurrently
    let mut handles = vec![];
    
    for i in 0..10 {
        let registry_clone = registry.clone();
        let handle = std::thread::spawn(move || {
            // Each thread performs different operations on the registry
            match i % 4 {
                0 => {
                    // Thread type 0: add new extensions
                    let new_source = format!("New{}", i);
                    let new_target = format!("Target{}", i);
                    registry_clone.register_extension(&new_source, &new_target).expect("Registration failed");
                },
                1 => {
                    // Thread type 1: check extensions
                    let extends = registry_clone.extends("Base", "DerivedA").expect("Extension check failed");
                    assert!(extends);
                },
                2 => {
                    // Thread type 2: find paths
                    let path = registry_clone.find_inheritance_path("Base", "DerivedA").expect("Failed to find path");
                    assert_eq!(path.len(), 3);
                },
                3 => {
                    // Thread type 3: generate visualizations
                    let _ascii = registry_clone.visualize_hierarchy_ascii().expect("Failed to generate ASCII visualization");
                    let all_interfaces = registry_clone.get_all_interfaces().expect("Failed to get interfaces");
                    assert!(all_interfaces.contains("Base"));
                },
                _ => unreachable!(),
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    // Verify the registry is still consistent
    let all_interfaces = registry.get_all_interfaces().expect("Failed to get interfaces");
    assert!(all_interfaces.contains("Base"));
    assert!(all_interfaces.contains("Derived1"));
    assert!(all_interfaces.contains("Derived2"));
    assert!(all_interfaces.contains("DerivedA"));
    assert!(all_interfaces.contains("DerivedB"));
    
    // Also verify that the new interfaces were added
    for i in 0..10 {
        if i % 4 == 0 {
            let new_source = format!("New{}", i);
            assert!(all_interfaces.contains(&new_source));
        }
    }
}

// Test interface registry with complex inheritance hierarchies
#[test]
fn test_complex_inheritance_hierarchy() {
    tracing::setup();
    
    let registry = ThreadSafeInterfaceRegistryVisualization::new();
    
    // Create a more complex inheritance hierarchy
    //
    //           BaseInterface
    //           /          \
    //     Interface1     Interface2
    //    /        \     /        \
    // Impl1A    Impl1B Impl2A    Impl2B
    //    \      /
    //      Shared
    
    registry.register_extension("BaseInterface", "Interface1").expect("Registration failed");
    registry.register_extension("BaseInterface", "Interface2").expect("Registration failed");
    registry.register_extension("Interface1", "Impl1A").expect("Registration failed");
    registry.register_extension("Interface1", "Impl1B").expect("Registration failed");
    registry.register_extension("Interface2", "Impl2A").expect("Registration failed");
    registry.register_extension("Interface2", "Impl2B").expect("Registration failed");
    registry.register_extension("Impl1A", "Shared").expect("Registration failed");
    registry.register_extension("Impl1B", "Shared").expect("Registration failed");
    
    // Test multiple inheritance paths
    let paths = registry.find_all_inheritance_paths("BaseInterface", "Shared").expect("Failed to find paths");
    assert_eq!(paths.len(), 2);
    
    // Verify the two different paths
    let path1_exists = paths.iter().any(|path| 
        path.len() == 4 && path[0] == "BaseInterface" && path[1] == "Interface1" && 
        path[2] == "Impl1A" && path[3] == "Shared"
    );
    
    let path2_exists = paths.iter().any(|path| 
        path.len() == 4 && path[0] == "BaseInterface" && path[1] == "Interface1" && 
        path[2] == "Impl1B" && path[3] == "Shared"
    );
    
    assert!(path1_exists, "Missing path through Impl1A");
    assert!(path2_exists, "Missing path through Impl1B");
    
    // Test visualization of complex hierarchy
    let ascii = registry.visualize_hierarchy_ascii().expect("Failed to generate ASCII visualization");
    assert!(ascii.contains("BaseInterface"));
    assert!(ascii.contains("Interface1"));
    assert!(ascii.contains("Shared"));
    
    // Verify path visualization for multiple paths
    for path in &paths {
        let path_ascii = registry.visualize_path_ascii(path).expect("Failed to generate path ASCII visualization");
        assert!(path_ascii.contains("BaseInterface"));
        assert!(path_ascii.contains("Interface1"));
        assert!(path_ascii.contains("Shared"));
    }
}