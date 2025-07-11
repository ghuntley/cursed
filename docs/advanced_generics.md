# Advanced Generics in CURSED

This document describes the advanced generics system implemented in CURSED, including type constraints, generic interfaces, and higher-kinded types.

## Table of Contents

1. [Overview](#overview)
2. [Type Constraints](#type-constraints)
3. [Generic Interfaces](#generic-interfaces)
4. [Higher-Kinded Types](#higher-kinded-types)
5. [Advanced Features](#advanced-features)
6. [Examples](#examples)
7. [Implementation Details](#implementation-details)

## Overview

CURSED's advanced generics system provides:

- **Type Constraints**: Restrict generic type parameters with trait bounds
- **Generic Interfaces**: Define reusable interfaces with type parameters
- **Higher-Kinded Types**: Support for type constructors and functors
- **Associated Types**: Define types that are associated with interfaces
- **Where Clauses**: Express complex constraints on generic types
- **Variance Annotations**: Control subtyping relationships in generics

## Type Constraints

### Basic Constraints

```cursed
// Function with Clone constraint
slay duplicate<T>(value T) T where T: Clone {
    damn value.clone()
}

// Multiple constraints
slay process<T>(value T) where T: Clone + Debug + Send {
    vibez.spill("Processing:", value)
    damn value.clone()
}
```

### Supported Constraint Types

- **Trait Bounds**: `T: Clone`, `T: Debug`, `T: Send`
- **Lifetime Bounds**: `T: 'static`
- **Equality Constraints**: `T = ConcreteType`
- **Associated Type Constraints**: `T::Item = U`
- **Size Constraints**: `T: Sized`
- **Copy Constraints**: `T: Copy`
- **Marker Constraints**: `T: Send + Sync`

### Advanced Constraint Examples

```cursed
// Equality constraint
slay same_type<T, U>(a T, b U) where T = U {
    // T and U must be the same type
}

// Associated type constraint  
slay collect_items<T>(iterator T) where T: Iterator<Item = normie> {
    // Iterator must yield integers
}

// Conditional constraints
slay conditional<T>(value T) where T: Clone, T: Debug {
    // Both Clone and Debug required
}
```

## Generic Interfaces

### Interface Definition

```cursed
// Simple interface
trait Drawable {
    slay draw(self)
}

// Generic interface with type parameters
trait Container<T> {
    slay add(self, item T)
    slay get(self, index normie) Option<T>
}

// Interface with associated types
trait Iterator {
    type Item
    slay next(self) Option<Self::Item>
}
```

### Interface Implementation

```cursed
// Implement generic interface
impl Container<normie> for Array<normie> {
    slay add(self, item normie) {
        // Add item to array
    }
    
    slay get(self, index normie) Option<normie> {
        // Get item from array
    }
}

// Implementation with associated types
impl Iterator for Array<normie> {
    type Item = normie
    
    slay next(self) Option<normie> {
        // Return next item
    }
}
```

### Interface Inheritance

```cursed
// Parent interface
trait Drawable {
    slay draw(self)
}

// Child interface inheriting from parent
trait ColoredDrawable : Drawable {
    slay set_color(self, color tea)
}
```

### Variance Annotations

```cursed
// Covariant type parameter
trait Producer<+T> {
    slay produce() T
}

// Contravariant type parameter
trait Consumer<-T> {
    slay consume(item T)
}

// Invariant type parameter (default)
trait Container<T> {
    slay get() T
    slay set(item T)
}
```

## Higher-Kinded Types

### Type Constructors

```cursed
// Define higher-kinded type
trait Functor<F> {
    slay map<A, B>(self F<A>, f slay(A) B) F<B>
}

// Implementation for Option
impl Functor<Option> for Option<T> {
    slay map<A, B>(self Option<A>, f slay(A) B) Option<B> {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}
```

### Kind System

The kind system classifies types:

- `*` - Concrete types (normie, tea, etc.)
- `* -> *` - Type constructors taking one type (Array, Option)
- `* -> * -> *` - Type constructors taking two types (Result, Map)
- `(* -> *) -> *` - Higher-order kinds (taking type constructors)

### Built-in Type Constructors

```cursed
// Array: * -> *
Array<normie>  // Array of integers

// Option: * -> *
Option<tea>    // Optional string

// Result: * -> * -> *
Result<normie, tea>  // Result with int success, string error
```

## Advanced Features

### Where Clauses

```cursed
// Complex where clause
slay advanced_function<T, U, V>(a T, b U) V
    where T: Clone + Send,
          U: Debug + Into<V>,
          V: Default {
    // Function body
}
```

### Associated Types

```cursed
// Interface with multiple associated types
trait Collect<T> {
    type Output
    type Error
    
    slay collect(self) Result<Self::Output, Self::Error>
}

// Implementation specifying associated types
impl Collect<normie> for Array<normie> {
    type Output = Array<normie>
    type Error = tea
    
    slay collect(self) Result<Array<normie>, tea> {
        // Implementation
    }
}
```

### Phantom Types

```cursed
// Phantom type for compile-time safety
struct TypedId<T> {
    id normie
    _phantom PhantomData<T>
}

// Usage
sus user_id TypedId<User> = TypedId::new(123)
sus product_id TypedId<Product> = TypedId::new(456)
// user_id and product_id are different types at compile time
```

### Higher-Ranked Trait Bounds

```cursed
// Function taking a closure that works with any lifetime
slay higher_ranked<F>(f F) where F: for<'a> Fn(&'a tea) -> &'a tea {
    sus input tea = "test"
    sus result tea = f(&input)
}
```

## Examples

### Generic Collection with Constraints

```cursed
struct SafeContainer<T> where T: Clone + Debug + Send {
    items Array<T>
    capacity normie
}

impl<T> SafeContainer<T> where T: Clone + Debug + Send {
    slay new(capacity normie) SafeContainer<T> {
        damn SafeContainer {
            items: Array::new(),
            capacity,
        }
    }
    
    slay add(self, item T) lit {
        if self.items.len() < self.capacity {
            self.items.push(item)
            damn based
        } else {
            damn cap
        }
    }
    
    slay get(self, index normie) Option<T> {
        if index < self.items.len() {
            damn Some(self.items[index].clone())
        } else {
            damn None
        }
    }
}
```

### Functor Implementation

```cursed
trait Functor<F> {
    slay map<A, B>(self F<A>, f slay(A) B) F<B>
}

impl Functor<Option> for Option<T> {
    slay map<A, B>(self Option<A>, f slay(A) B) Option<B> {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

// Usage
sus maybe_number Option<normie> = Some(42)
sus maybe_string Option<tea> = maybe_number.map(|n| n.to_string())
```

### Generic Interface with Associated Types

```cursed
trait IntoIterator {
    type Item
    type IntoIter: Iterator<Item = Self::Item>
    
    slay into_iter(self) Self::IntoIter
}

impl IntoIterator for Array<normie> {
    type Item = normie
    type IntoIter = ArrayIterator<normie>
    
    slay into_iter(self) ArrayIterator<normie> {
        damn ArrayIterator::new(self)
    }
}

struct ArrayIterator<T> {
    array Array<T>
    index normie
}

impl<T> Iterator for ArrayIterator<T> {
    type Item = T
    
    slay next(self) Option<T> {
        if self.index < self.array.len() {
            sus item T = self.array[self.index].clone()
            self.index += 1
            damn Some(item)
        } else {
            damn None
        }
    }
}
```

## Implementation Details

### Type System Architecture

The advanced generics system consists of several key components:

1. **AdvancedConstraintChecker**: Validates type constraints and dependencies
2. **GenericInterfaceChecker**: Manages interface definitions and implementations
3. **HigherKindedTypeSystem**: Handles type constructors and kind inference
4. **ConstraintDependencyGraph**: Resolves constraint dependencies

### Constraint Resolution

Constraints are resolved in dependency order:

1. Build dependency graph from type parameter relationships
2. Perform topological sort to determine resolution order
3. Check each constraint against concrete types
4. Validate interface implementations

### Kind Inference

The kind system infers kinds for type expressions:

1. Primitive types have kind `*`
2. Type constructors have kinds like `* -> *`
3. Higher-kinded types have kinds like `(* -> *) -> *`
4. Kind checking ensures proper constructor application

### Interface Checking

Interface implementations are validated by:

1. Checking all required methods are implemented
2. Verifying all associated types are bound
3. Validating constraint satisfaction
4. Ensuring proper inheritance relationships

### Compilation Integration

The advanced generics system integrates with CURSED's compilation pipeline:

1. **Parse Phase**: Generic syntax is parsed into AST nodes
2. **Type Check Phase**: Constraints and interfaces are validated
3. **Codegen Phase**: Generic types are monomorphized or use runtime dispatch
4. **Optimization Phase**: Specialized versions are generated for performance

### Performance Considerations

- **Monomorphization**: Generic functions are specialized for each type
- **Constraint Caching**: Constraint satisfaction results are cached
- **Interface Dispatch**: Virtual dispatch for interface methods
- **Kind Inference**: Efficient kind checking with memoization

## Future Extensions

Planned enhancements to the generics system:

1. **GADTs**: Generalized Algebraic Data Types
2. **Type Families**: Associated type families
3. **Dependent Types**: Limited dependent type support
4. **Effect Types**: Track computational effects in type system
5. **Linear Types**: Support for linear type system
6. **Refinement Types**: Types with logical predicates

## Testing

The advanced generics system includes comprehensive tests:

- Unit tests for each component
- Integration tests with the compiler
- Property-based testing for constraint resolution
- Performance benchmarks for type checking
- End-to-end tests with CURSED programs

See `tests/advanced_generics_test.csd` and `tests/type_system/advanced_generics_test.rs` for comprehensive test suites.
