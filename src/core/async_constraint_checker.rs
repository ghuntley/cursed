//! # Asynchronous Constraint Checker
//!
//! This module provides asynchronous constraint checking for improved parallelism
//! in the interface registry system. It allows concurrent execution of multiple constraint
//! checks for better performance in code that uses many generic types with constraints.

use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tracing::{debug, info, trace, instrument, warn};

/// The maximum number of worker threads to use for constraint checking
const MAX_WORKERS: usize = 4;

/// A task for the async constraint checker
#[derive(Debug)]
struct ConstraintCheckTask {
    /// The type to check
    type_: Type,
    /// The interface name to check against
    interface_name: String,
}

/// The result of a constraint check task
#[derive(Debug)]
struct ConstraintCheckResult {
    /// The type that was checked
    type_: Type,
    /// The interface name that was checked
    interface_name: String,
    /// The result of the check
    result: Result<bool, Error>,
}

/// Asynchronous constraint checker that allows parallel constraint checking
#[derive(Debug)]
pub struct AsyncConstraintChecker {
    /// The interface registry to use for constraint checking
    registry: Arc<InterfaceRegistry>,
    /// Statistics for the async checker
    stats: Arc<Mutex<AsyncCheckerStats>>,
}

/// Statistics for the async constraint checker
#[derive(Debug, Default)]
struct AsyncCheckerStats {
    /// Number of tasks processed
    tasks_processed: usize,
    /// Number of tasks executed in parallel
    parallel_tasks: usize,
    /// Number of tasks executed sequentially
    sequential_tasks: usize,
    /// Maximum concurrency achieved
    max_concurrency: usize,
}

impl AsyncConstraintChecker {
    /// Create a new async constraint checker with the given registry
    pub fn new(registry: Arc<InterfaceRegistry>) -> Self {
        Self {
            registry,
            stats: Arc::new(Mutex::new(AsyncCheckerStats::default())),
        }
    }

    /// Check a set of constraints in parallel
    ///
    /// # Arguments
    ///
    /// * `constraints` - A vector of (type, interface_name) pairs to check
    ///
    /// # Returns
    ///
    /// A vector of results corresponding to each constraint
    #[instrument(skip(self, constraints), level = "debug")]
    pub fn check_constraints_parallel(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>> {
        let num_constraints = constraints.len();
        
        // If there are too few constraints, just check them sequentially
        if num_constraints <= 1 {
            self.update_stats(num_constraints, 0, num_constraints);
            return constraints
                .into_iter()
                .map(|(type_, interface_name)| {
                    self.registry.check_implementation(&type_, &interface_name)
                })
                .collect();
        }
        
        // Determine number of worker threads to use
        let num_workers = std::cmp::min(MAX_WORKERS, num_constraints);
        
        debug!("Checking {} constraints with {} worker threads", num_constraints, num_workers);
        
        // Convert constraints into tasks
        let tasks: Vec<ConstraintCheckTask> = constraints.iter()
            .map(|(type_, interface_name)| {
                ConstraintCheckTask {
                    type_: type_.clone(),
                    interface_name: interface_name.clone(),
                }
            })
            .collect();
        
        // Create a shared task queue and results collection
        let tasks = Arc::new(Mutex::new(tasks));
        let results = Arc::new(Mutex::new(Vec::with_capacity(num_constraints)));
        
        // Spawn worker threads
        let mut handles = Vec::with_capacity(num_workers);
        for i in 0..num_workers {
            let tasks = Arc::clone(&tasks);
            let results = Arc::clone(&results);
            let registry = Arc::clone(&self.registry);
            
            let handle = thread::spawn(move || {
                trace!("Worker {} started", i);
                loop {
                    // Get a task from the queue
                    let task_opt = {
                        let mut tasks = tasks.lock().unwrap();
                        if tasks.is_empty() {
                            None
                        } else {
                            Some(tasks.remove(0))
                        }
                    };
                    
                    // Process the task if there is one
                    if let Some(task) = task_opt {
                        let type_clone = task.type_.clone();
                        let interface_name_clone = task.interface_name.clone();
                        
                        trace!("Worker {} processing task: {:?} implements {}", i, task.type_, task.interface_name);
                        
                        // Check the constraint
                        let result = registry.check_implementation(&task.type_, &task.interface_name);
                        
                        // Store the result
                        let check_result = ConstraintCheckResult {
                            type_: task.type_,
                            interface_name: task.interface_name,
                            result,
                        };
                        
                        let mut results = results.lock().unwrap();
                        results.push(check_result);
                        
                        trace!("Worker {} completed task: {:?} implements {}", i, type_clone, interface_name_clone);
                    } else {
                        // No more tasks
                        break;
                    }
                }
                trace!("Worker {} finished", i);
            });
            
            handles.push(handle);
        }
        
        // Wait for all workers to finish
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Calculate stats for this run
        self.update_stats(num_constraints, num_workers, 0);
        
        // Collect and return results in the order of the original constraints
        let mut result_map = std::collections::HashMap::new();
        let results = results.lock().unwrap();
        
        for result in results.iter() {
            let key = (result.type_.clone(), result.interface_name.clone());
            result_map.insert(key, result.result.clone());
        }
        
        // Return results in the original order
        let mut ordered_results = Vec::with_capacity(num_constraints);
        for (type_, interface_name) in constraints {
            let key = (type_.clone(), interface_name.clone());
            if let Some(result) = result_map.get(&key) {
                ordered_results.push(result.clone());
            } else {
                // This should never happen if all tasks were processed
                warn!("Missing result for {:?} implements {}", type_, interface_name);
                ordered_results.push(Err(Error::new("CNST01", "Missing constraint check result", None)));
            }
        }
        
        ordered_results
    }
    
    /// Check a single set of generic constraints in parallel
    ///
    /// This is especially useful for checking all the constraints of a generic type
    /// at once.
    ///
    /// # Arguments
    ///
    /// * `type_args` - The concrete type arguments
    /// * `type_params` - The type parameters of the generic type
    /// * `constraints` - The constraints on the type parameters
    ///
    /// # Returns
    ///
    /// `Ok(true)` if all constraints are satisfied, `Ok(false)` otherwise,
    /// or `Err` if there was an error during the check.
    #[instrument(skip(self, type_args, type_params, constraints), level = "debug")]
    pub fn check_generic_constraints_parallel(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error> {
        // Check if we have the right number of type arguments
        if type_args.len() != type_params.len() {
            debug!(
                "Wrong number of type arguments: expected {}, got {}",
                type_params.len(),
                type_args.len()
            );
            return Ok(false);
        }
        
        // Create a mapping from type parameter names to concrete types
        let mut type_map = std::collections::HashMap::new();
        for (i, param_name) in type_params.iter().enumerate() {
            type_map.insert(param_name.clone(), type_args[i].clone());
        }
        
        // If there are no constraints, return true
        if constraints.is_empty() {
            return Ok(true);
        }
        
        // Convert constraints into (type, interface) pairs for parallel checking
        let mut constraint_pairs = Vec::with_capacity(constraints.len());
        for (param_name, interface_name) in constraints {
            if let Some(concrete_type) = type_map.get(param_name) {
                constraint_pairs.push((concrete_type.clone(), interface_name.clone()));
            } else {
                // This should never happen if the type parameter lists match
                debug!("Type parameter {} not found in mapping", param_name);
                return Ok(false);
            }
        }
        
        // Check all constraints in parallel
        let results = self.check_constraints_parallel(constraint_pairs);
        
        // All constraints must be satisfied
        for result in results {
            match result {
                Ok(satisfied) => {
                    if !satisfied {
                        debug!("One of the constraints is not satisfied");
                        return Ok(false);
                    }
                }
                Err(err) => {
                    // Propagate any errors
                    return Err(err);
                }
            }
        }
        
        // All constraints satisfied
        debug!("All generic constraints satisfied");
        Ok(true)
    }
    
    /// Update statistics for the async constraint checker
    fn update_stats(&self, total: usize, parallel: usize, sequential: usize) {
        let mut stats = self.stats.lock().unwrap();
        stats.tasks_processed += total;
        stats.parallel_tasks += parallel;
        stats.sequential_tasks += sequential;
        stats.max_concurrency = std::cmp::max(stats.max_concurrency, parallel);
    }
    
    /// Get statistics for the async constraint checker
    pub fn get_stats(&self) -> (usize, usize, usize, usize) {
        let stats = self.stats.lock().unwrap();
        (
            stats.tasks_processed,
            stats.parallel_tasks,
            stats.sequential_tasks,
            stats.max_concurrency,
        )
    }
}

/// Extension trait for the InterfaceRegistry to add async constraint checking methods
pub trait AsyncConstraintChecking {
    /// Check a set of constraints in parallel
    fn check_constraints_parallel(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>>;
    
    /// Check a single set of generic constraints in parallel
    fn check_generic_constraints_parallel(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error>;
}

impl AsyncConstraintChecking for InterfaceRegistry {
    fn check_constraints_parallel(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>> {
        let checker = AsyncConstraintChecker::new(Arc::new(self.clone()));
        checker.check_constraints_parallel(constraints)
    }
    
    fn check_generic_constraints_parallel(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error> {
        let checker = AsyncConstraintChecker::new(Arc::new(self.clone()));
        checker.check_generic_constraints_parallel(type_args, type_params, constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_async_constraint_checker_single_constraint() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Check a single constraint
        let constraints = vec![(Type::Normie, "Numeric".to_string())];
        let results = checker.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Ok(true));
    }
    
    #[test]
    fn test_async_constraint_checker_multiple_constraints() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Check multiple constraints
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
            (Type::Lit, "Numeric".to_string()), // Should be false
        ];
        
        let results = checker.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Ok(true));   // Normie implements Numeric
        assert_eq!(results[1], Ok(true));   // Tea implements Comparable
        assert_eq!(results[2], Ok(false));  // Lit does not implement Numeric
    }
    
    #[test]
    fn test_async_checker_generic_constraints() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Type arguments and parameters
        let type_args = vec![Type::Tea, Type::Normie];
        let type_params = vec!["K".to_string(), "V".to_string()];
        
        // Constraints: K must implement Comparable
        let constraints = vec![("K".to_string(), "Comparable".to_string())];
        
        // Check constraints in parallel - should succeed because Tea implements Comparable
        let result = checker.check_generic_constraints_parallel(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(true));
        
        // Now try with a type that doesn't implement Comparable
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let type_args = vec![non_comparable, Type::Normie];
        
        let result = checker.check_generic_constraints_parallel(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(false));
    }
    
    #[test]
    fn test_async_constraint_extension_trait() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Check constraints through the extension trait
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
        ];
        
        let results = registry.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Ok(true));  // Normie implements Numeric
        assert_eq!(results[1], Ok(true));  // Tea implements Comparable
        
        // Test the generic constraints method
        let type_args = vec![Type::Tea, Type::Normie];
        let type_params = vec!["K".to_string(), "V".to_string()];
        let constraints = vec![("K".to_string(), "Comparable".to_string())];
        
        let result = registry.check_generic_constraints_parallel(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(true));
    }
}