# CURSED Type System Architecture

Technical documentation for the CURSED programming language's type system, covering architecture, implementation details, performance characteristics, and extension patterns.

## Table of Contents

1. [System Overview](#system-overview)
2. [Core Components](#core-components)
3. [Constraint Resolution Engine](#constraint-resolution-engine)
4. [Generic Instantiation System](#generic-instantiation-system)
5. [Type Inference Pipeline](#type-inference-pipeline)
6. [LLVM Integration](#llvm-integration)
7. [Performance Characteristics](#performance-characteristics)
8. [Extension Points](#extension-points)
9. [Memory Management](#memory-management)
10. [Testing Strategy](#testing-strategy)

## System Overview

The CURSED type system is a sophisticated compile-time framework that provides:

- **Static Type Safety**: Compile-time verification of type correctness
- **Generic Programming**: Zero-cost abstraction through monomorphization
- **Constraint Satisfaction**: Advanced constraint resolution for type safety
- **Type Inference**: Automatic type deduction with explicit fallbacks
- **Higher-Kinded Types**: Support for generic type constructors
- **Associated Types**: Type-level computation and projection
- **Lifetime Management**: Memory safety through ownership tracking

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    CURSED Type System                       │
├─────────────────────────────────────────────────────────────┤
│  Parser AST → Type Analysis → Constraint Resolution → LLVM  │
└─────────────────────────────────────────────────────────────┘

┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Type Parser   │───▶│ Type Inference  │───▶│   Constraint    │
│                 │    │                 │    │   Resolution    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Generic Types   │    │  Type Variables │    │   Unification   │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 ▼
┌─────────────────────────────────────────────────────────────┐
│                Generic Instantiation                       │
├─────────────────────────────────────────────────────────────┤
│  Monomorphization → Instance Cache → LLVM Code Generation   │
└─────────────────────────────────────────────────────────────┘
```

### Core Design Principles

1. **Correctness First**: Type safety is never compromised for performance
2. **Zero-Cost Abstractions**: Generic code compiles to efficient native code
3. **Compositional Design**: Components can be used independently
4. **Extensible Architecture**: New type features can be added incrementally
5. **Diagnostic Quality**: Clear error messages with actionable feedback

## Core Components

### Type Environment

The `TypeEnvironment` is the central registry for all type information:

```rust
/// Central type registry and environment management
pub struct TypeEnvironment {
    /// Type definitions indexed by name
    type_definitions: HashMap<String, TypeDefinition>,
    /// Active type variables and their constraints
    type_variables: HashMap<String, TypeVariable>,
    /// Scope stack for nested type environments
    scope_stack: Vec<TypeScope>,
    /// Cache for expensive type operations
    operation_cache: HashMap<String, CachedResult>,
}
```

**Key Responsibilities:**
- Type definition storage and retrieval
- Type variable lifecycle management
- Scope-based type visibility
- Type operation caching for performance

### Type Expression System

Type expressions represent all possible types in the system:

```rust
/// Comprehensive type expression representation
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    /// Primitive types (i32, String, bool, etc.)
    Primitive(PrimitiveType),
    /// User-defined types (structs, interfaces)
    Named(String, Vec<TypeExpression>),
    /// Generic type parameters
    Parameter(String),
    /// Function types with parameters and return type
    Function(Vec<TypeExpression>, Box<TypeExpression>),
    /// Tuple types with multiple elements
    Tuple(Vec<TypeExpression>),
    /// Array types with element type and size
    Array(Box<TypeExpression>, Option<usize>),
    /// Slice types with element type
    Slice(Box<TypeExpression>),
    /// Optional types (nullable)
    Optional(Box<TypeExpression>),
    /// Result types for error handling
    Result(Box<TypeExpression>, Box<TypeExpression>),
    /// Associated type projections
    Projection(Box<TypeExpression>, String),
    /// Higher-kinded type constructors
    Constructor(String, Vec<TypeExpression>),
    /// Type-level variables for inference
    Variable(TypeVariableId),
}
```

### Constraint System

The constraint system ensures type safety through sophisticated checking:

```rust
/// Type constraint representation
#[derive(Debug, Clone)]
pub enum TypeConstraint {
    /// Type implements an interface
    Implements(TypeExpression, String),
    /// Type equality constraint
    Equals(TypeExpression, TypeExpression),
    /// Subtyping relationship
    Subtype(TypeExpression, TypeExpression),
    /// Lifetime constraints
    Outlives(LifetimeId, LifetimeId),
    /// Associated type binding
    AssociatedType(TypeExpression, String, TypeExpression),
    /// Complex constraint combinations
    Conjunction(Vec<TypeConstraint>),
    /// Alternative constraint satisfaction
    Disjunction(Vec<TypeConstraint>),
}
```

## Constraint Resolution Engine

### Resolution Algorithm

The constraint resolver uses a sophisticated multi-phase approach:

```rust
impl ConstraintResolver {
    /// Main constraint resolution entry point
    pub fn resolve_constraints(
        &mut self,
        constraints: &[TypeConstraint],
        context: &ConstraintContext,
    ) -> Result<ConstraintSolution, ConstraintError> {
        // Phase 1: Constraint normalization and preprocessing
        let normalized = self.normalize_constraints(constraints)?;
        
        // Phase 2: Dependency analysis and ordering
        let resolution_order = self.analyze_dependencies(&normalized)?;
        
        // Phase 3: Iterative constraint solving
        let mut solution = ConstraintSolution::new();
        for constraint_batch in resolution_order {
            solution = self.solve_constraint_batch(constraint_batch, solution)?;
        }
        
        // Phase 4: Solution validation and optimization
        self.validate_solution(&solution, context)?;
        Ok(self.optimize_solution(solution))
    }
}
```

### Constraint Propagation

```rust
/// Advanced constraint propagation engine
impl ConstraintPropagator {
    /// Propagate constraints through dependency graph
    pub fn propagate_constraints(
        &mut self,
        initial_constraints: &[TypeConstraint],
    ) -> Result<Vec<TypeConstraint>, PropagationError> {
        let mut constraint_queue = VecDeque::from(initial_constraints.to_vec());
        let mut derived_constraints = Vec::new();
        
        while let Some(constraint) = constraint_queue.pop_front() {
            // Apply propagation rules
            let new_constraints = self.apply_propagation_rules(&constraint)?;
            
            for new_constraint in new_constraints {
                if !self.constraint_seen(&new_constraint) {
                    constraint_queue.push_back(new_constraint.clone());
                    derived_constraints.push(new_constraint);
                }
            }
        }
        
        Ok(derived_constraints)
    }
}
```

### Unification Engine

Type unification handles complex type matching and substitution:

```rust
impl TypeUnifier {
    /// Unify two type expressions
    pub fn unify(
        &mut self,
        left: &TypeExpression,
        right: &TypeExpression,
    ) -> Result<Substitution, UnificationError> {
        match (left, right) {
            // Variable unification with occurs check
            (TypeExpression::Variable(var), ty) |
            (ty, TypeExpression::Variable(var)) => {
                self.unify_variable(*var, ty)
            }
            
            // Structural unification for compound types
            (TypeExpression::Named(name1, args1), TypeExpression::Named(name2, args2)) => {
                if name1 == name2 && args1.len() == args2.len() {
                    self.unify_lists(args1, args2)
                } else {
                    Err(UnificationError::TypeMismatch(left.clone(), right.clone()))
                }
            }
            
            // Function type unification
            (TypeExpression::Function(params1, ret1), TypeExpression::Function(params2, ret2)) => {
                let param_subst = self.unify_lists(params1, params2)?;
                let ret_subst = self.unify(ret1, ret2)?;
                Ok(param_subst.compose(&ret_subst))
            }
            
            // Recursive unification for other types...
            _ => self.unify_structural(left, right)
        }
    }
}
```

## Generic Instantiation System

### Monomorphization Pipeline

The generic instantiation system transforms generic code into concrete, optimized implementations:

```rust
impl GenericInstantiator {
    /// Instantiate a generic type with concrete type arguments
    pub fn instantiate_type(
        &mut self,
        generic_type: &TypeDefinition,
        type_arguments: &[TypeExpression],
        context: &InstantiationContext,
    ) -> Result<InstantiatedType, InstantiationError> {
        // Check instantiation cache first
        let cache_key = self.generate_cache_key(generic_type, type_arguments);
        if let Some(cached) = self.instance_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Validate type arguments against constraints
        self.validate_type_arguments(generic_type, type_arguments)?;
        
        // Perform type substitution
        let substitution = self.create_substitution(
            &generic_type.type_parameters,
            type_arguments,
        )?;
        
        // Generate concrete type
        let instantiated = self.apply_substitution(generic_type, &substitution)?;
        
        // Cache result for future use
        self.instance_cache.insert(cache_key, instantiated.clone());
        
        Ok(instantiated)
    }
}
```

### Type Substitution Engine

```rust
impl TypeSubstitution {
    /// Apply type parameter substitution throughout a type definition
    pub fn apply_substitution(
        &self,
        type_def: &TypeDefinition,
        substitution: &SubstitutionMap,
    ) -> Result<TypeDefinition, SubstitutionError> {
        let mut result = type_def.clone();
        
        // Substitute in field types
        for field in &mut result.fields {
            field.field_type = self.substitute_type_expression(
                &field.field_type,
                substitution,
            )?;
        }
        
        // Substitute in method signatures
        for method in &mut result.methods {
            method.signature = self.substitute_signature(
                &method.signature,
                substitution,
            )?;
        }
        
        // Substitute in constraints
        for constraint in &mut result.constraints {
            *constraint = self.substitute_constraint(constraint, substitution)?;
        }
        
        Ok(result)
    }
}
```

### Instance Cache Management

High-performance caching system for instantiated types:

```rust
/// Optimized cache for generic instantiations
impl InstanceCache {
    /// Get or create a cached instantiation
    pub fn get_or_create<F>(
        &mut self,
        key: &str,
        creator: F,
    ) -> Result<InstantiatedType, CacheError>
    where
        F: FnOnce() -> Result<InstantiatedType, InstantiationError>,
    {
        // Check cache first
        if let Some(cached) = self.cache.get(key) {
            self.cache_stats.hits += 1;
            return Ok(cached.clone());
        }
        
        // Create new instantiation
        let instance = creator()?;
        
        // Manage cache size
        if self.cache.len() >= self.max_cache_size {
            self.evict_least_recently_used();
        }
        
        // Store in cache
        self.cache.insert(key.to_string(), instance.clone());
        self.cache_stats.misses += 1;
        
        Ok(instance)
    }
}
```

## Type Inference Pipeline

### Inference Algorithm

The type inference system uses Hindley-Milner style algorithm with extensions:

```rust
impl TypeInference {
    /// Infer types for an expression
    pub fn infer_expression(
        &mut self,
        expr: &Expression,
        context: &InferenceContext,
    ) -> Result<TypeExpression, InferenceError> {
        match expr {
            Expression::Literal(lit) => self.infer_literal_type(lit),
            Expression::Variable(name) => self.lookup_variable_type(name, context),
            Expression::FunctionCall(func, args) => {
                self.infer_function_call_type(func, args, context)
            }
            Expression::FieldAccess(obj, field) => {
                self.infer_field_access_type(obj, field, context)
            }
            Expression::Generic(base, type_args) => {
                self.infer_generic_instantiation(base, type_args, context)
            }
            _ => self.infer_complex_expression(expr, context),
        }
    }
    
    /// Infer function call with generic instantiation
    fn infer_function_call_type(
        &mut self,
        func: &Expression,
        args: &[Expression],
        context: &InferenceContext,
    ) -> Result<TypeExpression, InferenceError> {
        // Infer function type
        let func_type = self.infer_expression(func, context)?;
        
        // Infer argument types
        let arg_types: Result<Vec<_>, _> = args
            .iter()
            .map(|arg| self.infer_expression(arg, context))
            .collect();
        let arg_types = arg_types?;
        
        // Unify with function signature
        match func_type {
            TypeExpression::Function(param_types, return_type) => {
                self.unify_function_call(&param_types, &arg_types, &return_type)
            }
            TypeExpression::Variable(var) => {
                // Create fresh type variables and unify
                let fresh_return = self.fresh_type_variable();
                let fresh_params = arg_types.clone();
                let func_constraint = TypeExpression::Function(fresh_params, Box::new(fresh_return.clone()));
                
                self.add_constraint(TypeConstraint::Equals(
                    TypeExpression::Variable(var),
                    func_constraint,
                ))?;
                
                Ok(fresh_return)
            }
            _ => Err(InferenceError::NotCallable(func_type)),
        }
    }
}
```

### Constraint Generation

Type inference generates constraints that are solved by the constraint resolver:

```rust
impl ConstraintGenerator {
    /// Generate constraints for type checking
    pub fn generate_constraints(
        &mut self,
        program: &Program,
    ) -> Result<Vec<TypeConstraint>, ConstraintError> {
        let mut constraints = Vec::new();
        
        // Generate constraints for all declarations
        for declaration in &program.declarations {
            constraints.extend(self.generate_declaration_constraints(declaration)?);
        }
        
        // Generate constraints for function bodies
        for function in &program.functions {
            constraints.extend(self.generate_function_constraints(function)?);
        }
        
        Ok(constraints)
    }
    
    /// Generate constraints for generic instantiations
    fn generate_generic_constraints(
        &mut self,
        generic_def: &GenericDefinition,
        type_args: &[TypeExpression],
    ) -> Result<Vec<TypeConstraint>, ConstraintError> {
        let mut constraints = Vec::new();
        
        // Validate type argument count
        if type_args.len() != generic_def.type_parameters.len() {
            return Err(ConstraintError::ArityMismatch {
                expected: generic_def.type_parameters.len(),
                actual: type_args.len(),
            });
        }
        
        // Generate constraint for each type parameter
        for (param, arg) in generic_def.type_parameters.iter().zip(type_args.iter()) {
            // Generate constraints from parameter bounds
            for bound in &param.bounds {
                constraints.push(TypeConstraint::Implements(arg.clone(), bound.clone()));
            }
        }
        
        Ok(constraints)
    }
}
```

## LLVM Integration

### Type Compilation Pipeline

The type system integrates with LLVM through a sophisticated compilation pipeline:

```rust
impl LLVMTypeCompiler {
    /// Compile a CURSED type to LLVM type
    pub fn compile_type(
        &self,
        cursed_type: &TypeExpression,
        context: &CompilationContext,
    ) -> Result<BasicTypeEnum<'ctx>, CompilationError> {
        match cursed_type {
            TypeExpression::Primitive(prim) => self.compile_primitive_type(prim),
            TypeExpression::Named(name, args) => {
                self.compile_named_type(name, args, context)
            }
            TypeExpression::Function(params, ret) => {
                self.compile_function_type(params, ret, context)
            }
            TypeExpression::Parameter(param) => {
                // Look up instantiated type
                context.resolve_type_parameter(param)
            }
            _ => self.compile_complex_type(cursed_type, context),
        }
    }
    
    /// Compile generic instantiation to optimized LLVM code
    pub fn compile_generic_instantiation(
        &self,
        instance: &InstantiatedType,
        context: &CompilationContext,
    ) -> Result<StructType<'ctx>, CompilationError> {
        // Generate optimized struct layout
        let field_types: Result<Vec<_>, _> = instance
            .fields
            .iter()
            .map(|field| self.compile_type(&field.field_type, context))
            .collect();
        
        let field_types = field_types?;
        
        // Create LLVM struct type with optimal layout
        let struct_type = context.llvm_context.opaque_struct_type(&instance.name);
        struct_type.set_body(&field_types, false);
        
        Ok(struct_type)
    }
}
```

### Generic Function Compilation

```rust
impl GenericFunctionCompiler {
    /// Compile specialized version of generic function
    pub fn compile_specialized_function(
        &self,
        generic_func: &GenericFunction,
        type_args: &[TypeExpression],
        context: &CompilationContext,
    ) -> Result<FunctionValue<'ctx>, CompilationError> {
        // Create specialized function name
        let specialized_name = self.generate_specialized_name(generic_func, type_args);
        
        // Check if already compiled
        if let Some(existing) = context.get_compiled_function(&specialized_name) {
            return Ok(existing);
        }
        
        // Create substitution for type parameters
        let substitution = self.create_type_substitution(
            &generic_func.type_parameters,
            type_args,
        )?;
        
        // Substitute types in function signature
        let specialized_signature = self.apply_substitution_to_signature(
            &generic_func.signature,
            &substitution,
        )?;
        
        // Compile specialized function
        let llvm_function = self.compile_function_with_signature(
            &specialized_name,
            &specialized_signature,
            context,
        )?;
        
        // Compile function body with specialized types
        self.compile_function_body(
            &generic_func.body,
            &llvm_function,
            &substitution,
            context,
        )?;
        
        Ok(llvm_function)
    }
}
```

## Performance Characteristics

### Compilation Performance

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| Type lookup | O(1) amortized | O(n) | Hash table with caching |
| Constraint resolution | O(n²) worst case | O(n) | Depends on constraint complexity |
| Type unification | O(n log n) | O(n) | Union-find with path compression |
| Generic instantiation | O(k) | O(k) | Where k = size of instantiated type |
| Cache lookup | O(1) | O(c) | Where c = cache size |

### Runtime Performance

- **Zero-cost abstractions**: Generic code compiles to same performance as hand-written code
- **Monomorphization**: No runtime overhead for generic dispatch
- **Inline optimization**: LLVM optimizes specialized functions aggressively
- **Memory layout**: Optimized struct layouts for cache efficiency

### Memory Usage

```rust
/// Memory usage statistics for type system components
pub struct TypeSystemMemoryStats {
    /// Type definitions storage
    pub type_definitions: usize,
    /// Active type variables
    pub type_variables: usize,
    /// Constraint graph storage
    pub constraint_graph: usize,
    /// Instance cache memory
    pub instance_cache: usize,
    /// Total memory usage
    pub total_usage: usize,
}

impl TypeEnvironment {
    /// Get detailed memory usage statistics
    pub fn memory_stats(&self) -> TypeSystemMemoryStats {
        TypeSystemMemoryStats {
            type_definitions: self.calculate_type_definitions_memory(),
            type_variables: self.calculate_type_variables_memory(),
            constraint_graph: self.constraint_resolver.memory_usage(),
            instance_cache: self.generic_instantiator.cache_memory_usage(),
            total_usage: self.calculate_total_memory(),
        }
    }
}
```

## Extension Points

### Adding New Type Features

The type system is designed for extensibility:

```rust
/// Trait for extending the type system with new type kinds
pub trait TypeExtension {
    type TypeData: Clone + Debug;
    
    /// Type extension identifier
    fn extension_id(&self) -> &str;
    
    /// Parse type extension from AST
    fn parse_type(&self, ast: &TypeAst) -> Result<Self::TypeData, ParseError>;
    
    /// Compile type extension to LLVM
    fn compile_type(
        &self,
        data: &Self::TypeData,
        context: &CompilationContext,
    ) -> Result<BasicTypeEnum<'ctx>, CompilationError>;
    
    /// Generate constraints for type extension
    fn generate_constraints(
        &self,
        data: &Self::TypeData,
        context: &ConstraintContext,
    ) -> Result<Vec<TypeConstraint>, ConstraintError>;
}

/// Registry for type extensions
pub struct TypeExtensionRegistry {
    extensions: HashMap<String, Box<dyn TypeExtension>>,
}

impl TypeExtensionRegistry {
    /// Register a new type extension
    pub fn register<T: TypeExtension + 'static>(&mut self, extension: T) {
        self.extensions.insert(
            extension.extension_id().to_string(),
            Box::new(extension),
        );
    }
}
```

### Custom Constraint Types

```rust
/// Trait for implementing custom constraint types
pub trait CustomConstraint: Debug + Clone {
    /// Check if constraint is satisfied
    fn is_satisfied(
        &self,
        context: &ConstraintContext,
    ) -> Result<bool, ConstraintError>;
    
    /// Propagate constraint to derive new constraints
    fn propagate(
        &self,
        context: &ConstraintContext,
    ) -> Result<Vec<TypeConstraint>, ConstraintError>;
    
    /// Generate error message for unsatisfied constraint
    fn error_message(&self) -> String;
}

/// Built-in custom constraint implementations
#[derive(Debug, Clone)]
pub enum BuiltinConstraint {
    /// Requires type to have specific memory layout
    MemoryLayout(MemoryLayoutRequirement),
    /// Requires type to be thread-safe
    ThreadSafe,
    /// Requires type to be serializable
    Serializable(SerializationFormat),
    /// Custom user-defined constraint
    Custom(Box<dyn CustomConstraint>),
}
```

### Integration Hooks

```rust
/// Hooks for integrating with the type system
pub trait TypeSystemHooks {
    /// Called before type resolution begins
    fn pre_resolution(&mut self, context: &mut TypeContext) -> Result<(), HookError>;
    
    /// Called after successful type resolution
    fn post_resolution(&mut self, context: &TypeContext) -> Result<(), HookError>;
    
    /// Called when constraint resolution fails
    fn on_constraint_error(
        &mut self,
        error: &ConstraintError,
        context: &TypeContext,
    ) -> Result<Option<ConstraintSolution>, HookError>;
    
    /// Called during generic instantiation
    fn on_instantiation(
        &mut self,
        instance: &InstantiatedType,
        context: &TypeContext,
    ) -> Result<(), HookError>;
}
```

## Memory Management

### Lifetime Management

The type system integrates with CURSED's ownership and lifetime system:

```rust
/// Lifetime tracking for type expressions
#[derive(Debug, Clone)]
pub struct LifetimeAnnotation {
    /// Lifetime identifier
    pub lifetime_id: LifetimeId,
    /// Lifetime bounds and relationships
    pub bounds: Vec<LifetimeBound>,
    /// Source location for error reporting
    pub source_location: SourceLocation,
}

/// Lifetime constraint validation
impl LifetimeChecker {
    /// Validate lifetime constraints in type expression
    pub fn check_lifetimes(
        &self,
        type_expr: &TypeExpression,
        context: &LifetimeContext,
    ) -> Result<(), LifetimeError> {
        match type_expr {
            TypeExpression::Reference(inner, lifetime) => {
                self.check_reference_lifetime(inner, lifetime, context)
            }
            TypeExpression::Function(params, ret) => {
                self.check_function_lifetimes(params, ret, context)
            }
            _ => self.check_nested_lifetimes(type_expr, context),
        }
    }
}
```

### Memory Safety Guarantees

```rust
/// Memory safety analysis for type operations
impl MemorySafetyAnalyzer {
    /// Verify memory safety of type operations
    pub fn analyze_type_safety(
        &self,
        operation: &TypeOperation,
        context: &SafetyContext,
    ) -> Result<SafetyReport, SafetyError> {
        let mut report = SafetyReport::new();
        
        // Check for use-after-free vulnerabilities
        report.use_after_free = self.check_use_after_free(operation, context)?;
        
        // Check for double-free vulnerabilities
        report.double_free = self.check_double_free(operation, context)?;
        
        // Check for buffer overflows
        report.buffer_overflow = self.check_buffer_overflow(operation, context)?;
        
        // Check for null pointer dereferences
        report.null_pointer = self.check_null_pointer_deref(operation, context)?;
        
        Ok(report)
    }
}
```

## Testing Strategy

### Unit Testing Framework

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test constraint resolution with complex constraints
    #[test]
    fn test_complex_constraint_resolution() {
        let mut resolver = ConstraintResolver::new();
        let constraints = vec![
            TypeConstraint::Implements(
                TypeExpression::Parameter("T".to_string()),
                "Display".to_string(),
            ),
            TypeConstraint::Implements(
                TypeExpression::Parameter("T".to_string()),
                "Clone".to_string(),
            ),
        ];
        
        let solution = resolver.resolve_constraints(&constraints, &ConstraintContext::default())
            .expect("Should resolve constraints successfully");
        
        assert!(solution.is_satisfiable());
        assert_eq!(solution.substitutions.len(), 0); // No concrete substitutions yet
    }
    
    /// Test generic instantiation caching
    #[test]
    fn test_generic_instantiation_cache() {
        let mut instantiator = GenericInstantiator::new();
        let generic_type = create_test_generic_type();
        let type_args = vec![TypeExpression::Primitive(PrimitiveType::Int32)];
        
        // First instantiation should cache the result
        let instance1 = instantiator.instantiate_type(
            &generic_type,
            &type_args,
            &InstantiationContext::default(),
        ).expect("First instantiation should succeed");
        
        // Second instantiation should use cache
        let instance2 = instantiator.instantiate_type(
            &generic_type,
            &type_args,
            &InstantiationContext::default(),
        ).expect("Second instantiation should succeed");
        
        assert_eq!(instance1.name, instance2.name);
        assert_eq!(instantiator.cache_hits(), 1);
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    /// Test complete type checking pipeline
    #[test]
    fn test_complete_type_checking_pipeline() {
        let source_code = r#"
            squad Container<T> where T: Clone {
                value: T,
            }
            
            impl<T> Container<T> where T: Clone {
                vibes new(value: T) -> Container<T> {
                    Container { value }
                }
                
                vibes get() -> T {
                    self.value.clone()
                }
            }
            
            vibes main() {
                sus container = Container::new(42);
                sus value = container.get();
            }
        "#;
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check_program_source(source_code);
        
        assert!(result.is_ok(), "Type checking should succeed");
        
        let type_info = result.unwrap();
        assert_eq!(type_info.generic_instantiations.len(), 1);
        assert_eq!(type_info.constraint_violations.len(), 0);
    }
}
```

### Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    /// Property test for type unification commutativity
    proptest! {
        #[test]
        fn unification_is_commutative(
            left in type_expression_strategy(),
            right in type_expression_strategy()
        ) {
            let mut unifier1 = TypeUnifier::new();
            let mut unifier2 = TypeUnifier::new();
            
            let result1 = unifier1.unify(&left, &right);
            let result2 = unifier2.unify(&right, &left);
            
            // Unification should be commutative
            match (result1, result2) {
                (Ok(subst1), Ok(subst2)) => {
                    // Both should produce equivalent substitutions
                    prop_assert!(substitutions_equivalent(&subst1, &subst2));
                }
                (Err(_), Err(_)) => {
                    // Both should fail in the same way
                }
                _ => {
                    // Should not have different success/failure
                    prop_assert!(false, "Unification results should be consistent");
                }
            }
        }
    }
}
```

---

This architecture documentation provides a comprehensive technical overview of the CURSED type system. For user-focused information, see the [Generics Guide](generics_guide.md).
