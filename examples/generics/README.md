# CURSED Generic Programming Examples

This directory contains comprehensive examples demonstrating the advanced generic programming capabilities of the CURSED programming language.

## Examples Overview

### 1. Generic Collections (`collections.💀`)

Demonstrates type-safe collections with comprehensive generic features:

- **Generic List<T>**: Dynamic list with covariant type parameter
- **Generic Map<K, V>**: Hash map with key and value type parameters
- **Generic Set<T>**: Set collection with hash/equality constraints
- **Option<T>**: Monadic optional value type
- **Result<T, E>**: Error handling with generic success/error types
- **Iterator Interface**: Associated types for iteration patterns

**Key Features:**
- Higher-order functions (map, filter, fold)
- Type constraints and bounds
- Associated types for iterators
- Covariance in collection types
- Memory-safe operations

### 2. Generic Algorithms (`algorithms.💀`)

Showcases advanced algorithmic programming with generics:

- **Sorting Algorithms**: Quick sort, merge sort, heap sort with `Ord` constraints
- **Search Algorithms**: Binary search, linear search with type constraints
- **Collection Algorithms**: Functional programming primitives
- **Graph Algorithms**: Generic graph with DFS/BFS implementations
- **Tree Algorithms**: Binary search tree with generic operations

**Key Features:**
- Type constraints for ordering and equality
- Higher-order function parameters
- Generic data structures
- Algorithm specialization based on type properties
- Performance-optimized generic code

### 3. Advanced Constraints (`advanced_constraints.💀`)

Demonstrates sophisticated type system features:

- **Associated Types**: Iterator patterns with `Item` associated type
- **Higher-Kinded Types**: Functor, Applicative, Monad type classes
- **Type-Level Computation**: Compile-time type validation
- **Phantom Types**: Zero-cost abstractions for state machines
- **GADTs**: Generalized Algebraic Data Types for type safety
- **Async/Await**: Generic futures and async result types

**Key Features:**
- Complex constraint relationships
- Type-level programming
- State machine verification at compile time
- Monadic composition patterns
- Advanced type safety guarantees

### 4. Complete System Demo (`complete_system_demo.💀`)

Real-world application demonstrating all features working together:

- **E-commerce System**: Product database with generic caching
- **Database Abstraction**: Generic CRUD operations with constraints
- **Web Service**: Dependency injection with generic types
- **Caching Layer**: Multi-level caching with variance
- **Async Operations**: Monadic async result composition
- **Performance Analytics**: Higher-order function analytics

**Key Features:**
- Enterprise-grade architecture patterns
- Generic dependency injection
- Async/await with error handling
- Performance optimization techniques
- Real-world constraint usage

## Type System Features Demonstrated

### Associated Types
```cursed
collab Iterator<T> {
    type Item = T  // Associated type
    type IntoIter: Iterator<Self::Item>
    
    slay next(mut sus self) -> Option<Self::Item>
}
```

### Higher-Kinded Types
```cursed
collab Functor<F> {
    slay map<A, B>(fa: F<A>, f: fn(A) -> B) -> F<B>
}

collab Monad<M>: Functor<M> {
    slay pure<A>(value: A) -> M<A>
    slay bind<A, B>(ma: M<A>, f: fn(A) -> M<B>) -> M<B>
}
```

### Variance and Type Safety
```cursed
// List<T> is covariant in T
sus int_list: List<Integer> = List::new()
sus number_list: List<Number> = int_list  // Safe conversion

// Function types are contravariant in parameters
sus func: fn(Number) -> String = |n| n.to_string()
sus int_func: fn(Integer) -> String = func  // Safe conversion
```

### Advanced Constraints
```cursed
collab Database<T> where T: Serialize + Deserialize + Clone + Send + Sync {
    slay save(mut sus self, item: T) -> Result<(), DatabaseError>
    slay find_where<F>(mut sus self, condition: F) -> Result<List<T>, DatabaseError>
    where F: Fn(sus T) -> Boolean + Send + Sync
}
```

### Performance Optimization
```cursed
// Automatic optimization based on usage patterns:
// - Monomorphization for hot paths with few instantiations
// - Dynamic dispatch for many instantiations
// - JIT compilation for performance-critical code
// - Memory layout optimization for cache efficiency
```

## Compilation and Execution

To compile and run these examples:

```bash
# Compile individual examples
cursed compile examples/generics/collections.💀
cursed compile examples/generics/algorithms.💀
cursed compile examples/generics/advanced_constraints.💀
cursed compile examples/generics/complete_system_demo.💀

# Run examples
cursed run examples/generics/collections
cursed run examples/generics/algorithms
cursed run examples/generics/advanced_constraints
cursed run examples/generics/complete_system_demo

# Run with optimization profiling
cursed run --profile-generics examples/generics/complete_system_demo
```

## Performance Characteristics

The generic system provides:

1. **Zero-Cost Abstractions**: Generic types compile to efficient native code
2. **Adaptive Optimization**: Runtime profiling guides optimization decisions
3. **Memory Efficiency**: Optimized layouts and variance-aware allocations
4. **JIT Compilation**: Hot path specialization for maximum performance
5. **Cache-Friendly**: Memory layout optimization for modern CPU architectures

## Advanced Features

### Compile-Time Type Verification
```cursed
// State machine with compile-time verification
sus conn = Connection<Closed>::new()
sus open_conn = conn.open()           // Type changes to Connection<Open>
sus auth_conn = open_conn.authenticate("creds")? // Type changes to Connection<Authenticated>
auth_conn.send_data("Hello")          // Only available for authenticated connections
```

### Monadic Error Handling
```cursed
sus result = async_operation()
    .bind(|data| process_data(data))
    .map(|processed| format_output(processed))
    .map_err(|error| log_error(error))
    .await()
```

### Generic Dependency Injection
```cursed
collab WebService<D, R> where D: Database, R: Router {
    sus database: D
    sus router: R
    
    slay handle_request<T>(mut sus self, request: Request) -> Response<T>
    where T: Serialize + Deserialize + Send + Sync
}
```

## Testing

The examples include comprehensive test scenarios:

```bash
# Run generic system tests
cursed test examples/generics/

# Run with performance profiling
cursed test --profile examples/generics/

# Run specific constraint tests
cursed test examples/generics/advanced_constraints.💀
```

## Documentation

Each example file contains detailed documentation explaining:
- Type relationships and constraints
- Performance characteristics
- Memory safety guarantees
- Usage patterns and best practices
- Integration with the broader CURSED ecosystem

These examples demonstrate that CURSED provides enterprise-grade generic programming capabilities suitable for high-performance, type-safe applications requiring sophisticated abstraction patterns.
