//! # Interface Registry Visualization Integration
//!
//! This module integrates the visualization systems for the interface registry,
//! making it easier to visualize interface type hierarchies, detect cycles,
//! and generate useful debugging representations.
//!
//! It provides the common integration point between the interface extension registry
//! and the visualization system, ensuring consistent behavior and error handling.

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use tracing::{debug, instrument};

use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::core::interface_registry_visualization::{ThreadSafeInterfaceRegistryVisualization, InterfaceRegistryExtensionWithVisualization, VisualizationFormat, VisualizationOptions};
use crate::error::Error;

/// Provides integration between the standard interface registry and the visualization system
#[derive(Debug)]
pub struct InterfaceRegistryVisualizationIntegration {
    /// The interface extension registry
    pub registry: ThreadSafeInterfaceExtensionRegistry,
    /// The visualization system
    pub visualization: ThreadSafeInterfaceRegistryVisualization,
}

impl InterfaceRegistryVisualizationIntegration {
    /// Creates a new instance of the integration system
    pub fn new() -> Self {
        Self {
            registry: ThreadSafeInterfaceExtensionRegistry::new(),
            visualization: ThreadSafeInterfaceRegistryVisualization::new(),
        }
    }
    
    /// Register an extension relationship in both systems
    #[instrument(level = "debug")]
    pub fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        debug!("Registering extension in integrated system: {} extends {}", source, target);
        
        // Register in the regular registry
        self.registry.register_extension(source, target)?;
        
        // Register in the visualization system
        self.visualization.register_extension(source, target)?;
        
        Ok(())
    }
    
    /// Generate a visualization in the specified format
    #[instrument(level = "debug")]
    pub fn visualize(&self, format: VisualizationFormat, options: &VisualizationOptions) -> Result<String, Error> {
        debug!("Generating visualization in format: {:?}", format);
        
        // Get the extension hierarchy from both systems
        let registry_hierarchy = self.registry.get_extension_hierarchy()?;
        
        // Convert to the format expected by the visualization system
        let mut visualization_hierarchy = HashMap::new();
        for (source, targets) in registry_hierarchy {
            let targets_vec: Vec<String> = targets.into_iter().collect();
            visualization_hierarchy.insert(source, targets_vec);
        }
        
        // Generate the visualization based on the format
        match format {
            VisualizationFormat::Ascii => {
                self.visualization.generate_ascii_tree(&visualization_hierarchy, options)
            },
            VisualizationFormat::Dot => {
                self.visualization.generate_dot_graph(&visualization_hierarchy, options)
            },
            VisualizationFormat::Json => {
                self.visualization.generate_json_representation(&visualization_hierarchy, options)
            },
            VisualizationFormat::Text => {
                // Simple text format
                let mut result = String::from("Interface Hierarchy:\n");
                
                for (source, targets) in &visualization_hierarchy {
                    writeln!(&mut result, "{}:", source).unwrap();
                    for target in targets {
                        writeln!(&mut result, "  - {}", target).unwrap();
                    }
                }
                
                Ok(result)
            },
        }
    }
    
    /// Find paths between interfaces
    #[instrument(level = "debug")]
    pub fn find_paths(
        &self,
        source: &str,
        target: &str,
        max_paths: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding paths from {} to {}", source, target);
        
        // Try to use the registry's built-in path finding
        self.registry.find_interface_paths(source, target, max_paths)
    }
}