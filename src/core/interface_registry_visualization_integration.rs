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
use std::sync::{Arc, RwLock};
use tracing::{debug, instrument};

use crate::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use crate::core::interface_registry_visualization::{ThreadSafeInterfaceRegistryVisualization, InterfaceRegistryExtensionWithVisualization, VisualizationFormat, VisualizationOptions};
use crate::error::Error;

/// Provides integration between the standard interface registry and the visualization system
pub struct InterfaceRegistryVisualizationIntegration {
    /// The interface extension registry
    pub registry: Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>,
    /// The visualization system
    pub visualization: ThreadSafeInterfaceRegistryVisualization,
}

impl std::fmt::Debug for InterfaceRegistryVisualizationIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InterfaceRegistryVisualizationIntegration")
            .field("registry", &"Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>")
            .field("visualization", &"Box<dyn InterfaceRegistryVisualization>")
            .finish()
    }
}

impl InterfaceRegistryVisualizationIntegration {
    /// Creates a new instance of the integration system
    pub fn new() -> Self {
        let registry = ThreadSafeInterfaceExtensionRegistry::new();
        let visualization = Box::new(crate::core::interface_registry_visualization::DefaultInterfaceRegistryVisualization::new(registry.clone()));
        
        Self {
            registry,
            visualization,
        }
    }
    
    /// Register an extension relationship in both systems
    #[instrument(level = "debug")]
    pub fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        debug!("Registering extension in integrated system: {} extends {}", source, target);
        
        // Register in the regular registry (the visualization shares the same registry)
        self.registry.write().map_err(|_| Error::Internal("Failed to acquire registry write lock".to_string()))?.register_extension(source, target)?;
        
        Ok(())
    }
    
    /// Generate a visualization in the specified format
    #[instrument(level = "debug")]
    pub fn visualize(&self, format: VisualizationFormat, options: &VisualizationOptions) -> Result<String, Error> {
        debug!("Generating visualization in format: {:?}", format);
        
        // Get the extension hierarchy from the registry
        let registry_hierarchy = self.registry.read().map_err(|_| Error::Internal("Failed to acquire registry read lock".to_string()))?.get_extension_hierarchy()?;
        
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
        
        // Use the registry's path finding capabilities
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire registry read lock".to_string()))?;
        let paths = <dyn InterfaceRegistryExtensionWithVisualization>::find_all_inheritance_paths(&*registry, source, target)?;
        
        // Limit the number of paths if requested
        let limited_paths = if max_paths > 0 && paths.len() > max_paths {
            paths.into_iter().take(max_paths).collect()
        } else {
            paths
        };
        
        Ok(limited_paths)
    }
}