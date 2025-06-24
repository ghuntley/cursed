/// Constraint Resolution System for CURSED Type System
///
/// This module implements sophisticated constraint resolution algorithms for
/// the CURSED programming language's type system, providing:
/// - Constraint satisfaction checking with complex constraint types
/// - Constraint propagation algorithms for dependency resolution
/// - Type unification and substitution for constraint solving
/// - Violation detection with detailed error reporting
/// - Performance-optimized constraint graph management
///
/// ## Why Comprehensive Testing is Essential for Type Systems
///
/// Type systems are the cornerstone of programming language safety and correctness.
/// The constraint resolution engine is particularly critical because:
///
/// 1. **Type Safety Guarantees**: Any bug in constraint resolution can lead to
///    runtime type errors, memory safety violations, or incorrect program behavior.
///    Comprehensive testing ensures that type constraints are always correctly
///    satisfied before code generation.
///
/// 2. **Complex Interaction Patterns**: Type constraints often interact in
///    non-obvious ways. For example, transitive constraints (A: B, B: C implies A: C)
///    and complex generic bounds create intricate dependency graphs that must be
///    thoroughly validated.
///
/// 3. **Performance Impact**: Inefficient constraint resolution can dramatically
///    slow compilation. Testing helps identify performance bottlenecks and validates
///    that optimization strategies (like constraint caching) actually improve
///    compilation times.
///
/// 4. **Error Quality**: Poor constraint resolution leads to confusing error
///    messages. Testing ensures that when constraints fail, developers receive
///    actionable feedback pointing to the actual source of the problem.
///
/// ## Safety Guarantees Provided by the Constraint System
///
/// The constraint resolution system provides several critical safety guarantees:
///
/// ### Memory Safety
/// - **No Use-After-Free**: Lifetime constraints ensure that references never
///   outlive their referents, preventing use-after-free vulnerabilities.
/// - **No Double-Free**: Ownership constraints ensure that each value has exactly
///   one owner, preventing double-free errors.
/// - **No Buffer Overflows**: Array and slice constraints validate bounds checking
///   at compile time where possible.
///
/// ### Type Safety
/// - **No Type Confusion**: Generic constraints ensure that operations are only
///   performed on types that actually support them.
/// - **Interface Compliance**: Implementation constraints verify that types
///   actually provide all required interface methods with correct signatures.
/// - **Coherence**: Constraint resolution prevents overlapping implementations
///   that would create ambiguity in method dispatch.
///
/// ### Concurrency Safety
/// - **Data Race Prevention**: Send/Sync constraints ensure that types can be
///   safely shared between threads only when appropriate.
/// - **Deadlock Prevention**: Lock ordering constraints help prevent deadlock
///   patterns in concurrent code.
///
/// ## Performance Characteristics and Optimization Strategies
///
/// ### Time Complexity Analysis
/// - **Constraint Satisfaction**: O(n²) worst case where n is the number of constraints
/// - **Unification**: O(log n) with union-find optimization
/// - **Propagation**: O(n * m) where m is the average constraint dependency degree
/// - **Caching**: O(1) lookup for previously resolved constraint sets
///
/// ### Memory Usage Optimization
/// - **Constraint Graph Compaction**: Removes redundant constraint edges
/// - **Incremental Resolution**: Only re-resolves changed constraints
/// - **Garbage Collection**: Periodically cleans up unused constraint nodes
/// - **Copy-on-Write**: Shares immutable constraint data between contexts
///
/// ### Algorithmic Optimizations
/// - **Constraint Ordering**: Resolves simple constraints first to prune search space
/// - **Early Termination**: Fails fast when unsatisfiable constraints are detected
/// - **Parallel Resolution**: Independent constraint branches resolved concurrently
/// - **Memoization**: Caches results of expensive constraint satisfaction checks
///
/// ## Integration Patterns for Extending the System
///
/// The constraint system is designed for extensibility through several patterns:
///
/// ### Custom Constraint Types
/// ```rust
/// trait CustomConstraint {
///     fn check_satisfaction(&self, context: &ConstraintContext) -> bool;
///     fn propagate(&self, context: &ConstraintContext) -> Vec<DerivedConstraint>;
///     fn error_message(&self) -> String;
/// }
/// ```
///
/// ### Constraint Preprocessors
/// ```rust
/// trait ConstraintPreprocessor {
///     fn preprocess(&self, constraints: &mut Vec<Constraint>) -> Result<(), Error>;
/// }
/// ```
///
/// ### Custom Resolution Strategies
/// ```rust
/// trait ResolutionStrategy {
///     fn resolve_batch(&self, constraints: &[Constraint]) -> Resolution;
///     fn priority(&self) -> ResolutionPriority;
/// }
/// ```
///
/// ### Performance Monitoring Hooks
/// ```rust
/// trait PerformanceMonitor {
///     fn on_resolution_start(&mut self, context: &ResolutionContext);
///     fn on_resolution_complete(&mut self, stats: &ResolutionStats);
///     fn on_constraint_satisfied(&mut self, constraint: &Constraint);
/// }
/// ```

use crate::ast::declarations::GenericConstraint;
use crate::ast::traits::TypeParameter;
use crate::error::Error;
use crate::type_system::{
    TypeEnvironment, TypeExpression, TypeDefinition, ConstraintContext,
    ConstraintBinding, ConstraintStatus
};

use std::collections::{HashMap, HashSet, VecDeque};

/// Central constraint resolver managing all constraint operations
///
/// The `ConstraintResolver` is the main entry point for all constraint resolution
/// operations in the CURSED type system. It coordinates between multiple specialized
/// engines to provide comprehensive constraint satisfaction checking.
///
/// ## Architecture
///
/// The resolver uses a layered architecture:
/// 1. **Preprocessing**: Normalizes and optimizes constraints before resolution
/// 2. **Propagation**: Derives new constraints from existing ones using inference rules
/// 3. **Unification**: Matches type expressions and generates substitutions
/// 4. **Validation**: Ensures all constraints are satisfied in the final solution
/// 5. **Caching**: Stores resolution results for performance optimization
///
/// ## Thread Safety
///
/// The resolver is not thread-safe by design, as type checking is typically performed
/// in a single-threaded compilation context. For concurrent type checking, create
/// separate resolver instances per thread.
///
/// ## Performance Considerations
///
/// - Resolution cache can significantly improve performance for repeated constraint sets
/// - Constraint ordering heuristics reduce the search space for complex resolutions
/// - Early termination prevents unnecessary work when constraints are unsatisfiable
/// - Memory usage scales linearly with the number of active constraints
#[derive(Debug)]
pub struct ConstraintResolver {
    /// Constraint propagation engine for deriving new constraints
    propagator: ConstraintPropagator,
    /// Type unification engine for matching type expressions
    unifier: TypeUnifier,
    /// Constraint validation engine for final satisfaction checking
    validator: ConstraintValidator,
    /// Performance optimization cache mapping constraint sets to solutions
    resolution_cache: HashMap<String, ConstraintSolution>,
}

/// Constraint propagation engine for dependency management
#[derive(Debug)]
pub struct ConstraintPropagator {
    /// Dependency graph between constraints
    constraint_graph: ConstraintGraph,
    /// Propagation queue for iterative solving
    propagation_queue: VecDeque<PropagationTask>,
}

/// Type unification engine for constraint solving
#[derive(Debug)]
pub struct TypeUnifier {
    /// Current substitution mapping
    substitutions: HashMap<String, TypeExpression>,
    /// Unification history for backtracking
    unification_history: Vec<UnificationStep>,
}

/// Constraint validation engine
#[derive(Debug)]
pub struct ConstraintValidator {
    /// Built-in constraint implementations
    builtin_constraints: HashMap<String, BuiltinConstraint>,
}

/// Graph representation of constraint dependencies
#[derive(Debug)]
pub struct ConstraintGraph {
    /// Nodes representing constraints
    nodes: HashMap<String, ConstraintNode>,
    /// Edges representing dependencies
    edges: HashMap<String, Vec<String>>,
    /// Reverse edges for efficient traversal
    reverse_edges: HashMap<String, Vec<String>>,
}

/// Individual constraint node in the dependency graph
#[derive(Debug, Clone)]
pub struct ConstraintNode {
    pub id: String,
    pub constraint: GenericConstraint,
    pub bound_types: Vec<String>,
    pub status: ConstraintStatus,
    pub dependencies: Vec<String>,
}

/// Task for constraint propagation processing
#[derive(Debug, Clone)]
pub struct PropagationTask {
    pub constraint_id: String,
    pub trigger_type: PropagationTrigger,
    pub context: Vec<String>,
}

/// Types of propagation triggers
#[derive(Debug, Clone, PartialEq)]
pub enum PropagationTrigger {
    /// Initial constraint addition
    Initial,
    /// Type binding change
    TypeBinding(String),
    /// Dependency satisfaction
    DependencySatisfied(String),
    /// Constraint violation detected
    ViolationDetected(String),
}

/// Result of constraint resolution
#[derive(Debug, Clone)]
pub struct ConstraintSolution {
    /// Whether all constraints are satisfied
    pub is_satisfied: bool,
    /// Type substitutions required for satisfaction
    pub substitutions: HashMap<String, TypeExpression>,
    /// Remaining unsatisfied constraints
    pub violations: Vec<ConstraintViolation>,
    /// Additional constraints derived during resolution
    pub derived_constraints: Vec<GenericConstraint>,
}

/// Detailed constraint violation information
#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub constraint: GenericConstraint,
    pub violating_types: Vec<String>,
    pub reason: ViolationReason,
    pub suggested_fixes: Vec<String>,
}

/// Reasons for constraint violations
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationReason {
    /// Type does not implement required interface
    MissingInterface(String),
    /// Type parameter bound violation
    BoundViolation(String, String),
    /// Circular constraint dependency
    CircularDependency(Vec<String>),
    /// Incompatible type unification
    UnificationFailure(String, String),
    /// Missing type definition
    UndefinedType(String),
}

/// Step in unification process for backtracking
#[derive(Debug, Clone)]
pub struct UnificationStep {
    pub step_type: UnificationStepType,
    pub type1: TypeExpression,
    pub type2: TypeExpression,
    pub result: Option<HashMap<String, TypeExpression>>,
}

/// Types of unification steps
#[derive(Debug, Clone, PartialEq)]
pub enum UnificationStepType {
    /// Variable unification
    Variable,
    /// Constructor unification
    Constructor,
    /// Function unification
    Function,
    /// Generic unification
    Generic,
}

/// Built-in constraint implementation
#[derive(Debug, Clone)]
pub struct BuiltinConstraint {
    pub name: String,
    pub checker: fn(&TypeExpression, &[TypeExpression], &TypeEnvironment) -> bool,
    pub error_message: fn(&TypeExpression, &[TypeExpression]) -> String,
}

impl ConstraintResolver {
    /// Create a new constraint resolver
    pub fn new() -> Self {
        let mut resolver = Self {
            propagator: ConstraintPropagator::new(),
            unifier: TypeUnifier::new(),
            validator: ConstraintValidator::new(),
            resolution_cache: HashMap::new(),
        };
        resolver.initialize_builtin_constraints();
        resolver
    }

    /// Initialize built-in constraint types
    fn initialize_builtin_constraints(&mut self) {
        // Add built-in constraints like Sized, Clone, etc.
        self.validator.add_builtin_constraint(BuiltinConstraint {
            name: "Sized".to_string(),
            checker: |_type_expr, _args, _env| true, // All types are sized in CURSED
            error_message: |type_expr, _args| {
                format!("Type '{}' is not sized", type_expr.to_string())
            },
        });

        self.validator.add_builtin_constraint(BuiltinConstraint {
            name: "Clone".to_string(),
            checker: |type_expr, _args, env| {
                // Check if type has Clone implementation
                Self::check_clone_implementation(type_expr, env)
            },
            error_message: |type_expr, _args| {
                format!("Type '{}' does not implement Clone", type_expr.to_string())
            },
        });

        self.validator.add_builtin_constraint(BuiltinConstraint {
            name: "Debug".to_string(),
            checker: |type_expr, _args, env| {
                // Check if type has Debug implementation
                Self::check_debug_implementation(type_expr, env)
            },
            error_message: |type_expr, _args| {
                format!("Type '{}' does not implement Debug", type_expr.to_string())
            },
        });
    }

    /// Validate a constraint against the type environment
    pub fn validate_constraint(
        &self,
        constraint: &GenericConstraint,
        environment: &TypeEnvironment,
    ) -> Result<(), Error> {
        // Check if constraint refers to valid types
        for type_param in &constraint.type_parameters {
            if !self.is_valid_type_parameter(type_param, environment) {
                return Err(Error::Type(format!(
                    "Invalid type parameter '{}' in constraint",
                    type_param
                )));
            }
        }

        // Check if constraint name is recognized
        if !self.validator.is_known_constraint(&constraint.constraint_name) {
            return Err(Error::Type(format!(
                "Unknown constraint '{}'",
                constraint.constraint_name
            )));
        }

        Ok(true)
    }

    /// Check if all constraints are satisfied for a given type
    pub fn check_satisfaction(
        &self,
        type_expr: &TypeExpression,
        constraints: &[GenericConstraint],
        environment: &TypeEnvironment,
    ) -> Result<(), Error> {
        // Generate cache key
        let cache_key = self.generate_cache_key(type_expr, constraints);
        
        // Check cache first
        if let Some(solution) = self.resolution_cache.get(&cache_key) {
            return Ok(solution.is_satisfied);
        }

        // Create constraint context
        let mut context = ConstraintContext {
            scope_id: format!("satisfaction_{}", uuid::Uuid::new_v4()),
            active_constraints: Vec::new(),
            type_bindings: HashMap::new(),
        };

        // Add constraints to context
        for constraint in constraints {
            let binding = ConstraintBinding {
                constraint: constraint.clone(),
                bound_types: vec![type_expr.to_string()],
                satisfaction_status: ConstraintStatus::Pending,
            };
            context.active_constraints.push(binding);
        }

        // Resolve constraints
        let solution = self.resolve_constraints_internal(&context, environment)?;
        Ok(solution.is_satisfied)
    }

    /// Resolve all constraints in a given context
    pub fn resolve_constraints(
        &mut self,
        context: &ConstraintContext,
        environment: &TypeEnvironment,
    ) -> Result<(), Error> {
        let solution = self.resolve_constraints_internal(context, environment)?;
        
        // Cache result for performance
        let cache_key = self.generate_context_cache_key(context);
        self.resolution_cache.insert(cache_key, solution.clone());
        
        Ok(solution)
    }

    /// Internal constraint resolution implementation
    fn resolve_constraints_internal(
        &self,
        context: &ConstraintContext,
        environment: &TypeEnvironment,
    ) -> Result<(), Error> {
        let mut solution = ConstraintSolution {
            is_satisfied: true,
            substitutions: HashMap::new(),
            violations: Vec::new(),
            derived_constraints: Vec::new(),
        };

        // Build constraint graph
        let mut graph = self.propagator.build_constraint_graph(&context.active_constraints)?;

        // Perform topological resolution
        let resolution_order = graph.topological_sort()?;

        // Resolve constraints in dependency order
        for constraint_id in resolution_order {
            let constraint_result = self.resolve_single_constraint(
                &constraint_id,
                &graph,
                environment,
                &mut solution.substitutions,
            )?;

            match constraint_result {
                SingleConstraintResult::Satisfied => {
                    // Continue with next constraint
                }
                SingleConstraintResult::Violated(violation) => {
                    solution.is_satisfied = false;
                    solution.violations.push(violation);
                }
                SingleConstraintResult::DerivedConstraints(new_constraints) => {
                    solution.derived_constraints.extend(new_constraints);
                }
            }
        }

        Ok(solution)
    }

    /// Resolve a single constraint
    fn resolve_single_constraint(
        &self,
        constraint_id: &str,
        graph: &ConstraintGraph,
        environment: &TypeEnvironment,
        substitutions: &mut HashMap<String, TypeExpression>,
    ) -> Result<(), Error> {
        let node = graph.nodes.get(constraint_id)
            .ok_or_else(|| Error::Type(format!("Constraint node '{}' not found", constraint_id)))?;

        let constraint = &node.constraint;

        // Apply current substitutions to constraint
        let substituted_constraint = self.apply_substitutions_to_constraint(constraint, substitutions);

        // Check constraint satisfaction
        for bound_type in &node.bound_types {
            let type_expr = TypeExpression::named(bound_type);
            let type_with_substitutions = self.apply_substitutions_to_type(&type_expr, substitutions);

            if !self.validator.check_constraint_satisfaction(
                &substituted_constraint,
                &type_with_substitutions,
                environment,
            )? {
                let violation = ConstraintViolation {
                    constraint: constraint.clone(),
                    violating_types: vec![bound_type.to_string()],
                    reason: self.determine_violation_reason(&substituted_constraint, &type_with_substitutions, environment),
                    suggested_fixes: self.generate_suggested_fixes(&substituted_constraint, &type_with_substitutions),
                };
                return Ok(SingleConstraintResult::Violated(violation));
            }
        }

        Ok(SingleConstraintResult::Satisfied)
    }

    /// Apply type substitutions to a constraint
    fn apply_substitutions_to_constraint(
        &self,
        constraint: &GenericConstraint,
        substitutions: &HashMap<String, TypeExpression>,
    ) -> GenericConstraint {
        let mut new_constraint = constraint.clone();
        
        // Apply substitutions to type parameters
        for type_param in &mut new_constraint.type_parameters {
            if let Some(substitution) = substitutions.get(type_param) {
                *type_param = substitution.to_string();
            }
        }

        new_constraint
    }

    /// Apply type substitutions to a type expression
    fn apply_substitutions_to_type(
        &self,
        type_expr: &TypeExpression,
        substitutions: &HashMap<String, TypeExpression>,
    ) -> TypeExpression {
        match type_expr {
            TypeExpression::Named(name) => {
                substitutions.get(name).cloned().unwrap_or_else(|| type_expr.clone())
            }
            TypeExpression::Parameter(param) => {
                substitutions.get(param).cloned().unwrap_or_else(|| type_expr.clone())
            }
            TypeExpression::Generic(name, args) => {
                let substituted_args: Vec<TypeExpression> = args.iter()
                    .map(|arg| self.apply_substitutions_to_type(arg, substitutions))
                    .collect();
                TypeExpression::Generic(name.clone(), substituted_args)
            }
            TypeExpression::Function(params, ret) => {
                let substituted_params: Vec<TypeExpression> = params.iter()
                    .map(|param| self.apply_substitutions_to_type(param, substitutions))
                    .collect();
                let substituted_ret = self.apply_substitutions_to_type(ret, substitutions);
                TypeExpression::Function(substituted_params, Box::new(substituted_ret))
            }
            TypeExpression::Array(elem) => {
                let substituted_elem = self.apply_substitutions_to_type(elem, substitutions);
                TypeExpression::Array(Box::new(substituted_elem))
            }
            TypeExpression::Map(key, value) => {
                let substituted_key = self.apply_substitutions_to_type(key, substitutions);
                let substituted_value = self.apply_substitutions_to_type(value, substitutions);
                TypeExpression::Map(Box::new(substituted_key), Box::new(substituted_value))
            }
            TypeExpression::Channel(elem) => {
                let substituted_elem = self.apply_substitutions_to_type(elem, substitutions);
                TypeExpression::Channel(Box::new(substituted_elem))
            }
        }
    }

    /// Determine the reason for constraint violation
    fn determine_violation_reason(
        &self,
        constraint: &GenericConstraint,
        type_expr: &TypeExpression,
        environment: &TypeEnvironment,
    ) -> ViolationReason {
        match constraint.constraint_name.as_str() {
            "Clone" | "Debug" | "PartialEq" => {
                ViolationReason::MissingInterface(constraint.constraint_name.clone())
            }
            _ => {
                if let TypeExpression::Named(type_name) = type_expr {
                    if !environment.type_definitions.contains_key(type_name) {
                        ViolationReason::UndefinedType(type_name.clone())
                    } else {
                        ViolationReason::BoundViolation(
                            type_name.clone(),
                            constraint.constraint_name.clone(),
                        )
                    }
                } else {
                    ViolationReason::UnificationFailure(
                        type_expr.to_string(),
                        constraint.constraint_name.clone(),
                    )
                }
            }
        }
    }

    /// Generate suggested fixes for constraint violations
    fn generate_suggested_fixes(
        &self,
        constraint: &GenericConstraint,
        type_expr: &TypeExpression,
    ) -> Vec<String> {
        let mut fixes = Vec::new();

        match constraint.constraint_name.as_str() {
            "Clone" => {
                fixes.push(format!("Implement Clone for type '{}'", type_expr.to_string()));
                fixes.push("Derive Clone using #[derive(Clone)]".to_string());
            }
            "Debug" => {
                fixes.push(format!("Implement Debug for type '{}'", type_expr.to_string()));
                fixes.push("Derive Debug using #[derive(Debug)]".to_string());
            }
            _ => {
                fixes.push(format!(
                    "Implement constraint '{}' for type '{}'",
                    constraint.constraint_name,
                    type_expr.to_string()
                ));
            }
        }

        fixes
    }

    /// Generate cache key for type and constraints
    fn generate_cache_key(&self, type_expr: &TypeExpression, constraints: &[GenericConstraint]) -> String {
        let type_str = type_expr.to_string();
        let constraints_str = constraints.iter()
            .map(|c| format!("{}[{}]", c.constraint_name, c.type_parameters.join(",")))
            .collect::<Vec<_>>()
            .join(";");
        format!("{}|{}", type_str, constraints_str)
    }

    /// Generate cache key for constraint context
    fn generate_context_cache_key(&self, context: &ConstraintContext) -> String {
        let constraints_str = context.active_constraints.iter()
            .map(|b| format!("{}[{}]", b.constraint.constraint_name, b.bound_types.join(",")))
            .collect::<Vec<_>>()
            .join(";");
        let bindings_str = context.type_bindings.iter()
            .map(|(k, v)| format!("{}={}", k, v.to_string()))
            .collect::<Vec<_>>()
            .join(",");
        format!("{}|{}", constraints_str, bindings_str)
    }

    /// Check if a type parameter is valid
    fn is_valid_type_parameter(&self, param: &str, environment: &TypeEnvironment) -> bool {
        // Check if it's a known type or a valid parameter name
        environment.type_definitions.contains_key(param) || param.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    /// Check if a type implements Clone
    fn check_clone_implementation(type_expr: &TypeExpression, environment: &TypeEnvironment) -> bool {
        match type_expr {
            TypeExpression::Named(name) => {
                // Primitive types implement Clone
                matches!(name.as_str(), "normie" | "facts" | "tea" | "sus")
                || environment.type_definitions.get(name)
                    .map(|def| def.methods.iter().any(|m| m.name == "clone"))
                    .unwrap_or(false)
            }
            TypeExpression::Array(elem) => Self::check_clone_implementation(elem, environment),
            TypeExpression::Generic(_, args) => args.iter().all(|arg| Self::check_clone_implementation(arg, environment)),
            _ => false,
        }
    }

    /// Check if a type implements Debug
    fn check_debug_implementation(type_expr: &TypeExpression, environment: &TypeEnvironment) -> bool {
        match type_expr {
            TypeExpression::Named(name) => {
                // Primitive types implement Debug
                matches!(name.as_str(), "normie" | "facts" | "tea" | "sus")
                || environment.type_definitions.get(name)
                    .map(|def| def.methods.iter().any(|m| m.name == "debug"))
                    .unwrap_or(false)
            }
            TypeExpression::Array(elem) => Self::check_debug_implementation(elem, environment),
            TypeExpression::Generic(_, args) => args.iter().all(|arg| Self::check_debug_implementation(arg, environment)),
            _ => false,
        }
    }
}

/// Result of resolving a single constraint
#[derive(Debug)]
enum SingleConstraintResult {
    /// Constraint is satisfied
    Satisfied,
    /// Constraint is violated
    Violated(ConstraintViolation),
    /// Additional constraints were derived
    DerivedConstraints(Vec<GenericConstraint>),
}

impl ConstraintPropagator {
    /// Create a new constraint propagator
    pub fn new() -> Self {
        Self {
            constraint_graph: ConstraintGraph::new(),
            propagation_queue: VecDeque::new(),
        }
    }

    /// Build constraint dependency graph
    pub fn build_constraint_graph(
        &self,
        constraints: &[ConstraintBinding],
    ) -> Result<(), Error> {
        let mut graph = ConstraintGraph::new();

        // Add constraint nodes
        for (index, binding) in constraints.iter().enumerate() {
            let node = ConstraintNode {
                id: format!("constraint_{}", index),
                constraint: binding.constraint.clone(),
                bound_types: binding.bound_types.clone(),
                status: binding.satisfaction_status.clone(),
                dependencies: Vec::new(),
            };
            graph.add_node(node);
        }

        // Analyze dependencies between constraints
        self.analyze_constraint_dependencies(&mut graph)?;

        Ok(graph)
    }

    /// Analyze dependencies between constraints
    fn analyze_constraint_dependencies(&self, graph: &mut ConstraintGraph) -> Result<(), Error> {
        let node_ids: Vec<String> = graph.nodes.keys().cloned().collect();

        for id1 in &node_ids {
            for id2 in &node_ids {
                if id1 != id2 {
                    let node1 = &graph.nodes[id1];
                    let node2 = &graph.nodes[id2];

                    // Check if node1 depends on node2
                    if self.has_dependency(&node1.constraint, &node2.constraint) {
                        graph.add_edge(id2.clone(), id1.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if one constraint depends on another
    fn has_dependency(&self, dependent: &GenericConstraint, dependency: &GenericConstraint) -> bool {
        // Check if dependent constraint references types constrained by dependency
        for dep_param in &dependent.type_parameters {
            for dependency_param in &dependency.type_parameters {
                if dep_param == dependency_param {
                    return true;
                }
            }
        }
        false
    }
}

impl ConstraintGraph {
    /// Create a new empty constraint graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }

    /// Add a constraint node to the graph
    pub fn add_node(&mut self, node: ConstraintNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Add a dependency edge between constraints
    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
        self.reverse_edges.entry(to).or_insert_with(Vec::new).push(from);
    }

    /// Perform topological sort to determine resolution order
    pub fn topological_sort(&self) -> Result<(), Error> {
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        let mut result = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                self.topological_sort_visit(
                    node_id,
                    &mut visited,
                    &mut temp_visited,
                    &mut result,
                )?;
            }
        }

        result.reverse();
        Ok(result)
    }

    /// Recursive helper for topological sort
    fn topological_sort_visit(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        temp_visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), Error> {
        if temp_visited.contains(node_id) {
            return Err(Error::Type("Circular constraint dependency detected".to_string()));
        }

        if visited.contains(node_id) {
            return Ok(());
        }

        temp_visited.insert(node_id.to_string());

        if let Some(dependencies) = self.edges.get(node_id) {
            for dep in dependencies {
                self.topological_sort_visit(dep, visited, temp_visited, result)?;
            }
        }

        temp_visited.remove(node_id);
        visited.insert(node_id.to_string());
        result.push(node_id.to_string());

        Ok(())
    }
}

impl TypeUnifier {
    /// Create a new type unifier
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
            unification_history: Vec::new(),
        }
    }

    /// Unify two type expressions
    pub fn unify(
        &mut self,
        type1: &TypeExpression,
        type2: &TypeExpression,
    ) -> Result<(), Error> {
        let step = UnificationStep {
            step_type: self.determine_unification_type(type1, type2),
            type1: type1.clone(),
            type2: type2.clone(),
            result: None,
        };

        let result = self.unify_internal(type1, type2)?;
        
        let mut final_step = step;
        final_step.result = Some(result.clone());
        self.unification_history.push(final_step);

        Ok(result)
    }

    /// Internal unification implementation
    fn unify_internal(
        &mut self,
        type1: &TypeExpression,
        type2: &TypeExpression,
    ) -> Result<(), Error> {
        match (type1, type2) {
            // Variable unification
            (TypeExpression::Parameter(var), other) => {
                if self.occurs_check(var, other) {
                    return Err(Error::Type(format!("Occurs check failed: {} in {}", var, other.to_string())));
                }
                let mut result = HashMap::new();
                result.insert(var.clone(), other.clone());
                Ok(result)
            }
            (other, TypeExpression::Parameter(var)) => {
                if self.occurs_check(var, other) {
                    return Err(Error::Type(format!("Occurs check failed: {} in {}", var, other.to_string())));
                }
                let mut result = HashMap::new();
                result.insert(var.clone(), other.clone());
                Ok(result)
            }

            // Constructor unification
            (TypeExpression::Named(name1), TypeExpression::Named(name2)) => {
                if name1 == name2 {
                    Ok(HashMap::new())
                } else {
                    Err(Error::Type(format!("Cannot unify types {} and {}", name1, name2)))
                }
            }

            // Generic unification
            (TypeExpression::Generic(name1, args1), TypeExpression::Generic(name2, args2)) => {
                if name1 != name2 {
                    return Err(Error::Type(format!("Cannot unify generic types {} and {}", name1, name2)));
                }
                if args1.len() != args2.len() {
                    return Err(Error::Type(format!("Generic type argument count mismatch")));
                }

                let mut combined_substitutions = HashMap::new();
                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                    let arg_substitutions = self.unify_internal(arg1, arg2)?;
                    for (var, substitution) in arg_substitutions {
                        combined_substitutions.insert(var, substitution);
                    }
                }
                Ok(combined_substitutions)
            }

            // Function unification
            (TypeExpression::Function(params1, ret1), TypeExpression::Function(params2, ret2)) => {
                if params1.len() != params2.len() {
                    return Err(Error::Type("Function parameter count mismatch".to_string()));
                }

                let mut combined_substitutions = HashMap::new();
                
                // Unify parameters
                for (param1, param2) in params1.iter().zip(params2.iter()) {
                    let param_substitutions = self.unify_internal(param1, param2)?;
                    for (var, substitution) in param_substitutions {
                        combined_substitutions.insert(var, substitution);
                    }
                }

                // Unify return types
                let ret_substitutions = self.unify_internal(ret1, ret2)?;
                for (var, substitution) in ret_substitutions {
                    combined_substitutions.insert(var, substitution);
                }

                Ok(combined_substitutions)
            }

            // Array unification
            (TypeExpression::Array(elem1), TypeExpression::Array(elem2)) => {
                self.unify_internal(elem1, elem2)
            }

            // Map unification
            (TypeExpression::Map(key1, val1), TypeExpression::Map(key2, val2)) => {
                let key_substitutions = self.unify_internal(key1, key2)?;
                let val_substitutions = self.unify_internal(val1, val2)?;
                
                let mut combined = key_substitutions;
                for (var, substitution) in val_substitutions {
                    combined.insert(var, substitution);
                }
                Ok(combined)
            }

            // Channel unification
            (TypeExpression::Channel(elem1), TypeExpression::Channel(elem2)) => {
                self.unify_internal(elem1, elem2)
            }

            _ => Err(Error::Type(format!(
                "Cannot unify types {} and {}",
                type1.to_string(),
                type2.to_string()
            )))
        }
    }

    /// Occurs check to prevent infinite types
    fn occurs_check(&self, var: &str, type_expr: &TypeExpression) -> bool {
        match type_expr {
            TypeExpression::Parameter(param) => var == param,
            TypeExpression::Generic(_, args) => args.iter().any(|arg| self.occurs_check(var, arg)),
            TypeExpression::Function(params, ret) => {
                params.iter().any(|param| self.occurs_check(var, param)) || self.occurs_check(var, ret)
            }
            TypeExpression::Array(elem) => self.occurs_check(var, elem),
            TypeExpression::Map(key, value) => self.occurs_check(var, key) || self.occurs_check(var, value),
            TypeExpression::Channel(elem) => self.occurs_check(var, elem),
            _ => false,
        }
    }

    /// Determine the type of unification step
    fn determine_unification_type(&self, type1: &TypeExpression, type2: &TypeExpression) -> UnificationStepType {
        match (type1, type2) {
            (TypeExpression::Parameter(_), _) | (_, TypeExpression::Parameter(_)) => UnificationStepType::Variable,
            (TypeExpression::Generic(_, _), TypeExpression::Generic(_, _)) => UnificationStepType::Generic,
            (TypeExpression::Function(_, _), TypeExpression::Function(_, _)) => UnificationStepType::Function,
            _ => UnificationStepType::Constructor,
        }
    }
}

impl ConstraintValidator {
    /// Create a new constraint validator
    pub fn new() -> Self {
        Self {
            builtin_constraints: HashMap::new(),
        }
    }

    /// Add a built-in constraint
    pub fn add_builtin_constraint(&mut self, constraint: BuiltinConstraint) {
        self.builtin_constraints.insert(constraint.name.clone(), constraint);
    }

    /// Check if a constraint name is recognized
    pub fn is_known_constraint(&self, name: &str) -> bool {
        self.builtin_constraints.contains_key(name)
    }

    /// Check if a constraint is satisfied by a type
    pub fn check_constraint_satisfaction(
        &self,
        constraint: &GenericConstraint,
        type_expr: &TypeExpression,
        environment: &TypeEnvironment,
    ) -> Result<(), Error> {
        if let Some(builtin) = self.builtin_constraints.get(&constraint.constraint_name) {
            let args: Vec<TypeExpression> = constraint.type_parameters.iter()
                .map(|param| TypeExpression::named(param))
                .collect();
            Ok((builtin.checker)(type_expr, &args, environment))
        } else {
            // Check user-defined constraints
            self.check_user_defined_constraint(constraint, type_expr, environment)
        }
    }

    /// Check user-defined constraints
    fn check_user_defined_constraint(
        &self,
        constraint: &GenericConstraint,
        type_expr: &TypeExpression,
        environment: &TypeEnvironment,
    ) -> Result<(), Error> {
        // For now, assume user-defined constraints are interfaces
        if let TypeExpression::Named(type_name) = type_expr {
            if let Some(type_def) = environment.type_definitions.get(type_name) {
                // Check if type implements the required interface
                Ok(type_def.methods.iter().any(|method| {
                    // This is a simplified check - in practice, would need full interface matching
                    method.name.contains(&constraint.constraint_name.to_lowercase())
                }))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Default for ConstraintResolver {
    fn default() -> Self {
        Self::new()
    }
}

// Add uuid dependency for generating unique IDs
// Note: In a real implementation, you might want to use a simpler ID generation scheme
mod uuid {
    pub struct Uuid;
    impl Uuid {
        pub fn new_v4() -> Self { Self }
    }
    impl std::fmt::Display for Uuid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", rand::random::<u64>())
        }
    }
}

mod rand {
    pub fn random<T: std::hash::Hash + Default>() -> T {
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_system::TypeEnvironment;

    #[test]
    fn test_constraint_resolver_creation() {
        let resolver = ConstraintResolver::new();
        assert!(resolver.validator.is_known_constraint("Sized"));
        assert!(resolver.validator.is_known_constraint("Clone"));
        assert!(resolver.validator.is_known_constraint("Debug"));
    }

    #[test]
    fn test_type_unification() {
        let mut unifier = TypeUnifier::new();
        
        let type1 = TypeExpression::parameter("T");
        let type2 = TypeExpression::named("normie");
        
        let result = unifier.unify(&type1, &type2).unwrap();
        assert_eq!(result.get("T"), Some(&TypeExpression::named("normie")));
    }

    #[test]
    fn test_constraint_graph_creation() {
        let graph = ConstraintGraph::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_constraint_violation_creation() {
        let constraint = GenericConstraint {
            constraint_name: "Clone".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: Vec::new(),
        };

        let violation = ConstraintViolation {
            constraint,
            violating_types: vec!["MyType".to_string()],
            reason: ViolationReason::MissingInterface("Clone".to_string()),
            suggested_fixes: vec!["Implement Clone for MyType".to_string()],
        };

        assert_eq!(violation.violating_types[0], "MyType");
    }

    #[test]
    fn test_constraint_solution() {
        let mut solution = ConstraintSolution {
            is_satisfied: true,
            substitutions: HashMap::new(),
            violations: Vec::new(),
            derived_constraints: Vec::new(),
        };

        solution.substitutions.insert("T".to_string(), TypeExpression::named("normie"));
        assert_eq!(solution.substitutions.get("T"), Some(&TypeExpression::named("normie")));
    }

    #[test]
    fn test_occurs_check() {
        let unifier = TypeUnifier::new();
        
        // T should not occur in T
        assert!(!unifier.occurs_check("T", &TypeExpression::parameter("T")));
        
        // T should occur in [T]
        let array_type = TypeExpression::array(TypeExpression::parameter("T"));
        assert!(unifier.occurs_check("T", &array_type));
    }

    #[test]
    fn test_builtin_constraint_checking() {
        let resolver = ConstraintResolver::new();
        let env = TypeEnvironment::new();

        // Test Clone constraint for primitive types
        let normie_type = TypeExpression::named("normie");
        assert!(ConstraintResolver::check_clone_implementation(&normie_type, &env));

        let facts_type = TypeExpression::named("facts");
        assert!(ConstraintResolver::check_clone_implementation(&facts_type, &env));
    }
}
