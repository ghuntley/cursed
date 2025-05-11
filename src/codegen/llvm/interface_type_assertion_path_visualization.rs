//! Interface Type Assertion Path Visualization
//!
//! This module provides functionality for visualizing the inheritance path
//! between interface types during type assertions. It helps with debugging
//! and error reporting when type assertions fail, by showing the path that
//! would be required for a successful type assertion.
//!
//! The visualization shows both the current actual type and the target type,
//! as well as any intermediate types in the inheritance hierarchy that would
//! need to be traversed to make the assertion valid.

use crate::error::SourceLocation;
use crate::error::type_assertion_error::TypeAssertionError;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use std::collections::{HashMap, HashSet};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;

/// Extension trait for LlvmCodeGenerator to add path visualization capabilities
pub trait InterfaceTypeAssertionPathVisualization<'ctx> {
    /// Get a visualization of the inheritance path between two types
    fn visualize_type_path(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Result<String, Error>;

    /// Get the runtime type ID from an interface value with enhanced error checking
    fn get_runtime_type_id(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        source_location: Option<SourceLocation>,
    ) -> Result<(u64, String), Error>;

    /// Check if a type assertion is valid and generate visualization on failure
    fn check_type_assertion_with_visualization(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get a list of interfaces implemented by a type
    fn get_implemented_interfaces(&self, type_id: u64, inheritance_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64>;
    
    /// Get a list of types that implement an interface
    fn get_interface_implementors(&self, interface_id: u64, inheritance_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64>;
    
    /// Get the relationship type between two types in the inheritance hierarchy
    fn get_relationship_type(&self, from_id: u64, to_id: u64, inheritance_map: &HashMap<u64, HashSet<u64>>) -> String;
    
    /// Get or insert a runtime function in the LLVM module
    fn get_or_insert_runtime_function(&mut self, name: &str) -> Option<inkwell::values::FunctionValue<'ctx>>;
}

impl<'ctx> InterfaceTypeAssertionPathVisualization<'ctx> for LlvmCodeGenerator<'ctx> {
    fn visualize_type_path(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Result<String, Error> {
        // Ensure the registry is initialized
        if self.interface_type_registry.is_none() {
            return Ok(format!(
                "Type Path Visualization:\n\n{} --?--> {}\n\nNo type registry available to determine the path.",
                from_type, to_type
            ));
        }

        let registry = self.interface_type_registry.as_ref().unwrap();
        
        // Try to get the type information from the registry
        let from_id = match registry.get_type_id(from_type) {
            Ok(id) => id,
            Err(_) => return Ok(format!(
                "Type Path Visualization:\n\n{} (unknown type) --?--> {}\n\nThe source type is not registered in the type registry.",
                from_type, to_type
            )),
        };
        
        let to_id = match registry.get_type_id(to_type) {
            Ok(id) => id,
            Err(_) => return Ok(format!(
                "Type Path Visualization:\n\n{} --?--> {} (unknown type)\n\nThe target type is not registered in the type registry.",
                from_type, to_type
            )),
        };
        
        // If the types are the same, it's a direct match
        if from_id == to_id {
            return Ok(format!(
                "Type Path Visualization:\n\n{} --DIRECT MATCH--> {}\n\nTypes are identical, no conversion needed.",
                from_type, to_type
            ));
        }
        
        // Get the inheritance map from the registry if available
        let inheritance_map = match registry.get_inheritance_map() {
            Some(map) => map,
            None => return Ok(format!(
                "Type Path Visualization:\n\n{} --?--> {}\n\nNo inheritance information available in the registry.",
                from_type, to_type
            )),
        };
        
        // Try to find a path from from_type to to_type
        let path = self.find_inheritance_path(from_id, to_id, &inheritance_map)?;
        
        // Build the visualization based on the path
        if path.is_empty() {
            // No path found - provide a detailed explanation with potential alternatives
            
            // Get implemented interfaces for the source type if available
            let source_implements = self.get_implemented_interfaces(from_id, &inheritance_map);
            
            // Get types that implement the target interface if it's an interface
            let target_implementors = self.get_interface_implementors(to_id, &inheritance_map);
            
            // Create a more helpful error message with suggestions
            let mut path_str = format!(
                "Type Path Visualization:\n\n{} --INCOMPATIBLE--> {}\n\nNo valid conversion path exists between these types.",
                from_type, to_type
            );
            
            // Add information about source type's interfaces
            if !source_implements.is_empty() {
                path_str.push_str("\nThe source type implements the following interfaces:\n");
                for (i, &interface_id) in source_implements.iter().enumerate() {
                    let interface_name = registry.get_type_name(interface_id)
                        .unwrap_or_else(|_| format!("unknown(0x{:x})", interface_id));
                    path_str.push_str(&format!("  {}. {}\n", i + 1, interface_name));
                }
            }
            
            // Add information about target interface implementors
            if !target_implementors.is_empty() {
                path_str.push_str("\nThe target interface is implemented by:\n");
                for (i, &impl_id) in target_implementors.iter().enumerate() {
                    let impl_name = registry.get_type_name(impl_id)
                        .unwrap_or_else(|_| format!("unknown(0x{:x})", impl_id));
                    path_str.push_str(&format!("  {}. {}\n", i + 1, impl_name));
                }
            }
            
            // Suggest possible solutions
            path_str.push_str("\nPossible solutions:\n");
            path_str.push_str("  1. Add missing interface implementation\n");
            path_str.push_str("  2. Use a different type that is compatible with the target\n");
            path_str.push_str("  3. Create an adapter or conversion function\n");
            
            Ok(path_str)
        } else {
            // Convert type IDs to names
            let mut path_names = Vec::new();
            let mut path_details = Vec::new();
            
            for type_id in &path {
                // Get basic name
                let type_name = registry.get_type_name(*type_id)
                    .unwrap_or_else(|_| format!("unknown(0x{:x})", type_id));
                path_names.push(type_name.clone());
                
                // Try to get additional type metadata if available
                let type_detail = if let Ok(metadata) = registry.get_type_metadata(*type_id) {
                    let mut detail = type_name.clone();
                    
                    // Add relevant metadata for better context
                    if let Some(module) = metadata.get("module") {
                        detail.push_str(&format!(" [module: {}]", module));
                    }
                    
                    if let Some(kind) = metadata.get("kind") {
                        detail.push_str(&format!(" [{}]", kind));
                    }
                    
                    detail
                } else {
                    type_name
                };
                
                path_details.push(type_detail);
            }
            
            // Create the visualization with the path
            let mut path_str = String::new();
            path_str.push_str(&format!("\nType Path Visualization:\n\n"));
            
            // Add source type information
            path_str.push_str(&format!("Source: {} (ID: 0x{:x})\n", path_details[0], path[0]));
            path_str.push_str(&format!("Target: {} (ID: 0x{:x})\n\n", path_details[path_details.len()-1], path[path.len()-1]));
            
            // Build enhanced path diagram
            if path_names.len() <= 2 {
                // Direct inheritance
                path_str.push_str(&format!("{} --DIRECT--> {}\n", from_type, to_type));
            } else {
                // Path with intermediate types - create a more visual representation
                path_str.push_str(&format!("{} \n", path_details[0]));
                
                for i in 1..path_details.len() - 1 {
                    // Add information about the relationship type if available
                    let relation_type = self.get_relationship_type(path[i-1], path[i], &inheritance_map);
                    
                    // Show the relationship type in the arrow
                    path_str.push_str(&format!("  |\n  | [{}]\n  v\n", relation_type));
                    path_str.push_str(&format!("{} \n", path_details[i]));
                }
                
                // Final relationship to target type
                let final_relation = self.get_relationship_type(
                    path[path.len()-2], 
                    path[path.len()-1], 
                    &inheritance_map
                );
                
                path_str.push_str(&format!("  |\n  | [{}]\n  v\n", final_relation));
                path_str.push_str(&format!("{} (target)\n", path_details[path_details.len()-1]));
            }
            
            // Add explanation with more details
            path_str.push_str("\n");
            if path_names.len() <= 2 {
                path_str.push_str("Direct implementation relationship exists between these types.\n");
            } else {
                path_str.push_str(&format!(
                    "Conversion path has {} intermediate types across {} inheritance levels.\n",
                    path_names.len() - 2, path_names.len() - 1
                ));
                
                // Add path summary
                path_str.push_str("\nPath summary: ");
                for (i, name) in path_names.iter().enumerate() {
                    if i > 0 {
                        path_str.push_str(" -> ");
                    }
                    path_str.push_str(name);
                }
                path_str.push_str("\n");
            }
            
            Ok(path_str)
        }
    }
    
    /// Get a list of interfaces implemented by a type
    fn get_implemented_interfaces(&self, type_id: u64, inheritance_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
        // Find direct implementations first
        let mut interfaces = Vec::new();
        
        if let Some(direct_interfaces) = inheritance_map.get(&type_id) {
            interfaces.extend(direct_interfaces.iter());
        }
        
        // Add interfaces that this type might implement indirectly
        // by checking if this type appears as an implementor for any interface
        for (&interface_id, implementors) in inheritance_map.iter() {
            if implementors.contains(&type_id) && !interfaces.contains(&interface_id) {
                interfaces.push(interface_id);
            }
        }
        
        interfaces
    }
    
    /// Get a list of types that implement an interface
    fn get_interface_implementors(&self, interface_id: u64, inheritance_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
        let mut implementors = Vec::new();
        
        // Check if the given type is an interface by looking for types that implement it
        for (&type_id, interfaces) in inheritance_map.iter() {
            if interfaces.contains(&interface_id) {
                implementors.push(type_id);
            }
        }
        
        implementors
    }
    
    /// Get the relationship type between two connected types
    fn get_relationship_type(&self, from_id: u64, to_id: u64, inheritance_map: &HashMap<u64, HashSet<u64>>) -> String {
        // Direct implementation is the most common case
        if let Some(implementations) = inheritance_map.get(&from_id) {
            if implementations.contains(&to_id) {
                return "implements".to_string();
            }
        }
        
        // Check for extension relationship (one interface extending another)
        if let Some(registry) = &self.interface_type_registry {
            if let Ok(from_type) = registry.get_type_name(from_id) {
                if let Ok(to_type) = registry.get_type_name(to_id) {
                    // Check if both are interfaces (this is a heuristic, would be better to check actual type kind)
                    if from_type.ends_with("Interface") && to_type.ends_with("Interface") {
                        return "extends".to_string();
                    }
                }
            }
        }
        
        // Check for embedded relationship
        // This would require more detailed type information than we have here
        
        // Default to a generic relationship
        "inherits".to_string()
    }
    fn get_runtime_type_id(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        source_location: Option<SourceLocation>,
    ) -> Result<(u64, String), Error> {
        // First, extract the type ID using the base functionality
        let type_id = match self.get_interface_type_id(interface_value) {
            Ok(id) => id,
            Err(e) => return Err(Error::TypeAssertion(
                TypeAssertionError::new("interface", "any")
                    .with_message(format!("Failed to extract type ID from interface value: {}", e))
                    .with_location(source_location.unwrap_or_else(|| SourceLocation {
                        line: 0,
                        column: 0,
                        file: None,
                        source_line: String::new(),
                    }))
            )),
        };
        
        // Convert the type ID to a runtime constant if possible
        let const_id = match type_id.as_int_value().get_zero_extended_constant() {
            Some(id) => id,
            None => {
                // For dynamic IDs, generate LLVM IR to print the ID at runtime
                // Get the current function
                let current_fn = match self.current_function() {
                    Some(fn_val) => fn_val,
                    None => return Ok((0, "<dynamic-type-id-no-function>".to_string())),
                };
                
                // Create basic blocks for debug printing and continuation
                let debug_block = self.context().append_basic_block(current_fn, "type_id_debug");
                let continue_block = self.context().append_basic_block(current_fn, "type_id_continue");
                
                // Jump to debug block
                self.builder().build_unconditional_branch(debug_block)
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                
                // Position at debug block
                self.builder().position_at_end(debug_block);
                
                // Call debug print function if available
                if let Some(debug_fn) = self.get_or_insert_runtime_function("print_type_id") {
                    let _ = self.builder().build_call(
                        debug_fn,
                        &[type_id.into()],
                        "debug_type_id"
                    ).map_err(|e| Error::Compilation(e.to_string()))?;
                }
                
                // Continue execution
                self.builder().build_unconditional_branch(continue_block)
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                
                // Position at continue block
                self.builder().position_at_end(continue_block);
                
                // Since we can't determine the actual ID at compile time, use a special marker
                return Ok((0, "<dynamic-type-id>".to_string()));
            }
        };
        
        // Try to look up the type name and additional metadata in the registry
        let type_info = if let Some(registry) = &self.interface_type_registry {
            // Get basic type name
            let type_name = match registry.get_type_name(const_id) {
                Ok(name) => name,
                Err(_) => format!("unknown(0x{:x})", const_id),
            };
            
            // Try to get additional type metadata if available
            let type_metadata = match registry.get_type_metadata(const_id) {
                Ok(metadata) => {
                    // If we have metadata, construct a richer type description
                    let mut description = type_name.clone();
                    
                    // Add module information if available
                    if let Some(module) = metadata.get("module") {
                        description.push_str(&format!(" [module: {}]", module));
                    }
                    
                    // Add namespace information if available
                    if let Some(namespace) = metadata.get("namespace") {
                        description.push_str(&format!(" [namespace: {}]", namespace));
                    }
                    
                    description
                },
                Err(_) => type_name, // Use basic name if metadata not available
            };
            
            type_metadata
        } else {
            // If no registry, just use the raw type ID as hexadecimal
            format!("unknown(0x{:x})", const_id)
        };
        
        Ok((const_id, type_info))
    }
    
    /// Gets a function from the runtime support or inserts it if not present
    fn get_or_insert_runtime_function(&mut self, name: &str) -> Option<inkwell::values::FunctionValue<'ctx>> {
        // Check if the module has this function already
        if let Some(func) = self.module().get_function(name) {
            return Some(func);
        }
        
        // Define the print_type_id runtime function if needed
        if name == "print_type_id" {
            let fn_type = self.context().i64_type().fn_type(&[self.context().i64_type().into()], false);
            let fn_val = self.module().add_function(name, fn_type, None);
            return Some(fn_val);
        }
        
        None
    }
    
    fn check_type_assertion_with_visualization(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the runtime type information
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(interface_value, source_location.clone())?;
        
        // Get the target type ID
        let target_type_id = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(target_type).unwrap_or_else(|_| self.hash_type_name(target_type)),
            None => self.hash_type_name(target_type),
        };
        
        // Check if the types are compatible
        if self.is_type_compatible(actual_type_id, target_type_id) {
            // Create a constant true value for compatible types
            let true_value = self.context().bool_type().const_int(1, false);
            return Ok(true_value.into());
        }
        
        // Create the visualization
        let visualization = self.visualize_type_path(&actual_type_name, target_type)?;
        
        // Create a detailed error with the visualization
        let detailed_error = TypeAssertionError::new("interface", target_type)
            .with_message(format!("Type assertion failed: {} is not a {}", actual_type_name, target_type))
            .with_actual_type(actual_type_name, Some(actual_type_id))
            .with_target_type_id(target_type_id);
            
        if let Some(loc) = source_location {
            return Err(Error::TypeAssertion(
                detailed_error.with_location(loc).with_message(visualization)
            ));
        }
        
        // Create a constant false value for incompatible types
        let false_value = self.context().bool_type().const_int(0, false);
        Ok(false_value.into())
    }
}

// Helper methods for path visualization
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Find a path between two types in the inheritance hierarchy
    fn find_inheritance_path(
        &self,
        from_type_id: u64,
        to_type_id: u64,
        inheritance_map: &HashMap<u64, HashSet<u64>>,
    ) -> Result<Vec<u64>, Error> {
        // If from and to are the same, return a single-element path
        if from_type_id == to_type_id {
            return Ok(vec![from_type_id]);
        }
        
        // Use bidirectional search for better performance on deep hierarchies
        // This helps find paths more efficiently in complex type hierarchies
        let mut visited_forward = HashSet::new();
        let mut visited_backward = HashSet::new();
        
        // Forward search from source type
        let mut queue_forward = vec![(from_type_id, vec![from_type_id])];
        
        // Backward search from target type (we need to build a reverse map)
        let mut queue_backward = vec![(to_type_id, vec![to_type_id])];
        
        // Build a reverse inheritance map for backward search
        let reverse_map = self.build_reverse_inheritance_map(inheritance_map)?;
        
        // Track the meeting point to reconstruct the path
        let mut meeting_point = None;
        
        // Search iteration limit to prevent infinite loops
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000; // Adjust based on expected hierarchy depth
        
        // Bidirectional search loop
        while !queue_forward.is_empty() && !queue_backward.is_empty() && iterations < MAX_ITERATIONS {
            // Forward step
            if let Some((current_id, current_path)) = queue_forward.pop() {
                // Mark as visited in forward direction
                visited_forward.insert(current_id);
                
                // Check if we reached a node visited by backward search
                if visited_backward.contains(&current_id) {
                    meeting_point = Some(current_id);
                    break;
                }
                
                // Add all direct implementations to the queue
                if let Some(implementations) = inheritance_map.get(&current_id) {
                    for &impl_id in implementations {
                        if !visited_forward.contains(&impl_id) {
                            let mut new_path = current_path.clone();
                            new_path.push(impl_id);
                            queue_forward.push((impl_id, new_path));
                        }
                    }
                }
            }
            
            // Backward step
            if let Some((current_id, current_path)) = queue_backward.pop() {
                // Mark as visited in backward direction
                visited_backward.insert(current_id);
                
                // Check if we reached a node visited by forward search
                if visited_forward.contains(&current_id) {
                    meeting_point = Some(current_id);
                    break;
                }
                
                // Add all types that implement this interface to the queue
                if let Some(implemented_by) = reverse_map.get(&current_id) {
                    for &impl_id in implemented_by {
                        if !visited_backward.contains(&impl_id) {
                            let mut new_path = current_path.clone();
                            new_path.insert(0, impl_id); // Insert at the beginning for reverse path
                            queue_backward.push((impl_id, new_path));
                        }
                    }
                }
            }
            
            iterations += 1;
        }
        
        // If we found a meeting point, reconstruct the path
        if let Some(meeting_id) = meeting_point {
            // Find the path from source to meeting point
            let forward_path = queue_forward.iter()
                .find(|(id, _)| *id == meeting_id)
                .map(|(_, path)| path.clone())
                .unwrap_or_else(|| {
                    // If not in the queue, it might be the last visited node
                    let mut path = vec![from_type_id];
                    // We should properly reconstruct this path, but for simplicity
                    // we're just adding the meeting point if it's different
                    if meeting_id != from_type_id {
                        path.push(meeting_id);
                    }
                    path
                });
            
            // Find the path from meeting point to target
            let backward_path = queue_backward.iter()
                .find(|(id, _)| *id == meeting_id)
                .map(|(_, path)| path.clone())
                .unwrap_or_else(|| {
                    // If not in the queue, it might be the last visited node
                    let mut path = vec![];
                    // Add the meeting point if it's not the target
                    if meeting_id != to_type_id {
                        path.push(meeting_id);
                    }
                    path.push(to_type_id);
                    path
                });
            
            // Merge paths, removing duplicate meeting point
            let mut complete_path = forward_path;
            // Skip the first element of backward_path if it's the meeting point
            let backward_start = if !backward_path.is_empty() && backward_path[0] == meeting_id { 1 } else { 0 };
            complete_path.extend_from_slice(&backward_path[backward_start..]);
            
            return Ok(complete_path);
        }
        
        // Fallback to regular BFS if bidirectional search fails or hits iteration limit
        if iterations >= MAX_ITERATIONS {
            // Restart with simple BFS for reliability
            let mut visited = HashSet::new();
            let mut queue = vec![(from_type_id, vec![from_type_id])];
            
            while let Some((current_id, current_path)) = queue.pop() {
                // If we reached the target type, return the path
                if current_id == to_type_id {
                    return Ok(current_path);
                }
                
                // Mark the current type as visited
                visited.insert(current_id);
                
                // Add all direct implementations/extensions to the queue
                if let Some(implementations) = inheritance_map.get(&current_id) {
                    for &impl_id in implementations {
                        if !visited.contains(&impl_id) {
                            let mut new_path = current_path.clone();
                            new_path.push(impl_id);
                            queue.push((impl_id, new_path));
                        }
                    }
                }
            }
        }
        
        // No path found
        Ok(vec![])
    }
    
    /// Builds a reverse inheritance map for backward search
    fn build_reverse_inheritance_map(
        &self,
        inheritance_map: &HashMap<u64, HashSet<u64>>,
    ) -> Result<HashMap<u64, HashSet<u64>>, Error> {
        let mut reverse_map: HashMap<u64, HashSet<u64>> = HashMap::new();
        
        // For each type, add its implementations to the reverse map
        for (&type_id, implementations) in inheritance_map.iter() {
            for &impl_id in implementations {
                reverse_map.entry(impl_id)
                    .or_insert_with(HashSet::new)
                    .insert(type_id);
            }
        }
        
        Ok(reverse_map)
    }
    
    /// Check if one type is compatible with another (either same type or implements the interface)
    fn is_type_compatible(&self, actual_type_id: u64, target_type_id: u64) -> bool {
        // Same type is always compatible
        if actual_type_id == target_type_id {
            return true;
        }
        
        // Check inheritance relationship
        if let Some(registry) = &self.interface_type_registry {
            if let Some(inheritance_map) = registry.get_inheritance_map() {
                // Check if there's a path from actual to target type
                match self.find_inheritance_path(actual_type_id, target_type_id, &inheritance_map) {
                    Ok(path) => return !path.is_empty(),
                    Err(_) => return false,
                }
            }
        }
        
        false
    }
}

/// Register the type assertion path visualization with the compiler
pub fn register_type_assertion_path_visualization() {
    tracing::trace!("Interface type assertion path visualization registered");
}