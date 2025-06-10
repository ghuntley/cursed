# CURSED Generics Guide

A comprehensive guide to writing generic code in the CURSED programming language, including generic structs, interfaces, functions, and the constraint system.

## Table of Contents

1. [Introduction to Generics](#introduction-to-generics)
2. [Generic Structs](#generic-structs)
3. [Generic Interfaces](#generic-interfaces)
4. [Generic Functions](#generic-functions)
5. [Constraint System](#constraint-system)
6. [Advanced Constraint Patterns](#advanced-constraint-patterns)
7. [Performance Considerations](#performance-considerations)
8. [Best Practices](#best-practices)
9. [Migration Guide](#migration-guide)
10. [Troubleshooting](#troubleshooting)

## Introduction to Generics

Generics in CURSED enable writing code that works with multiple types while maintaining type safety and performance. The CURSED generic system features:

- **Type Parameters**: Placeholder types for flexible code
- **Constraint System**: Ensuring type parameters meet specific requirements
- **Compile-Time Specialization**: Zero-cost abstractions through monomorphization
- **Constraint Resolution**: Sophisticated type checking and inference

### Basic Syntax

```cursed
// Generic function with type parameter T
vibes generic_function<T>() -> T {
    // Implementation
}

// Generic struct with multiple type parameters
squad Container<T, U> {
    data: T,
    metadata: U,
}

// Generic interface with constraints
collab Comparable<T> where T: Eq {
    vibes compare(other: T) -> i32;
}
```

## Generic Structs

Generic structs allow creating reusable data structures that work with different types.

### Basic Generic Struct

```cursed
// A generic container that holds any type
squad Box<T> {
    value: T,
}

// Usage
sus string_box = Box<String> {
    value: "hello",
};

sus int_box = Box<i32> {
    value: 42,
};
```

### Multiple Type Parameters

```cursed
// A key-value pair with different types
squad Pair<K, V> {
    key: K,
    value: V,
}

// Usage with constraints
sus user_scores = Pair<String, i32> {
    key: "alice",
    value: 100,
};
```

### Nested Generic Types

```cursed
// A result type that can hold success or error values
squad Result<T, E> {
    success: bool,
    data: T?,
    error: E?,
}

// Nested generic containers
squad Matrix<T> {
    rows: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

// Usage
sus int_matrix = Matrix<i32> {
    rows: vec![vec![1, 2], vec![3, 4]],
    width: 2,
    height: 2,
};
```

### Generic Struct Methods

```cursed
impl<T> Box<T> {
    // Constructor method
    vibes new(value: T) -> Box<T> {
        Box { value }
    }
    
    // Method that works with the generic type
    vibes get() -> T {
        self.value
    }
    
    // Method that transforms the type
    vibes map<U>(func: vibes(T) -> U) -> Box<U> {
        Box<U> { value: func(self.value) }
    }
}

// Usage
sus original = Box::new(42);
sus doubled = original.map(|x| x * 2);
```

## Generic Interfaces

Generic interfaces define contracts that types must implement, enabling polymorphism and code reuse.

### Basic Generic Interface

```cursed
// A generic interface for types that can be compared
collab Comparable<T> {
    vibes compare(other: T) -> i32;
    vibes equals(other: T) -> bool;
}

// Implementation for a specific type
impl Comparable<Person> for Person {
    vibes compare(other: Person) -> i32 {
        self.age - other.age
    }
    
    vibes equals(other: Person) -> bool {
        self.id == other.id
    }
}
```

### Interface with Associated Types

```cursed
// An iterator interface with associated types
collab Iterator {
    type Item;
    
    vibes next() -> Option<Self::Item>;
    vibes has_next() -> bool;
}

// Implementation for a vector iterator
impl Iterator for VecIterator<T> {
    type Item = T;
    
    vibes next() -> Option<T> {
        lowkey (self.index < self.data.len()) {
            sus value = self.data[self.index];
            self.index += 1;
            Some(value)
        } bestie {
            None
        }
    }
    
    vibes has_next() -> bool {
        self.index < self.data.len()
    }
}
```

### Multiple Generic Parameters

```cursed
// A generic mapping interface
collab Mapper<T, U> {
    vibes map(input: T) -> U;
    vibes can_map(input: T) -> bool;
}

// String to number converter
squad StringToNumber {}

impl Mapper<String, i32> for StringToNumber {
    vibes map(input: String) -> i32 {
        input.parse().unwrap_or(0)
    }
    
    vibes can_map(input: String) -> bool {
        input.parse::<i32>().is_ok()
    }
}
```

## Generic Functions

Generic functions work with multiple types while ensuring type safety through the constraint system.

### Basic Generic Functions

```cursed
// A generic swap function
vibes swap<T>(a: &mut T, b: &mut T) {
    sus temp = *a;
    *a = *b;
    *b = temp;
}

// Usage
sus mut x = 10;
sus mut y = 20;
swap(&mut x, &mut y); // x=20, y=10

sus mut name1 = "Alice";
sus mut name2 = "Bob";
swap(&mut name1, &mut name2);
```

### Generic Functions with Return Types

```cursed
// A generic function that creates default values
vibes default_value<T>() -> T where T: Default {
    T::default()
}

// A generic function for finding maximum
vibes max<T>(a: T, b: T) -> T where T: Comparable<T> {
    lowkey (a.compare(b) > 0) {
        a
    } bestie {
        b
    }
}

// Usage
sus max_int = max(10, 20);        // 20
sus max_str = max("apple", "banana"); // "banana"
```

### Generic Functions with Complex Logic

```cursed
// A generic filter function
vibes filter<T>(items: Vec<T>, predicate: vibes(T) -> bool) -> Vec<T> {
    sus mut result = Vec::new();
    
    lowkey sus item in items {
        lowkey (predicate(item)) {
            result.push(item);
        }
    }
    
    result
}

// A generic reduce function
vibes reduce<T, U>(items: Vec<T>, initial: U, reducer: vibes(U, T) -> U) -> U {
    sus mut accumulator = initial;
    
    lowkey sus item in items {
        accumulator = reducer(accumulator, item);
    }
    
    accumulator
}

// Usage
sus numbers = vec![1, 2, 3, 4, 5];
sus evens = filter(numbers, |x| x % 2 == 0);
sus sum = reduce(numbers, 0, |acc, x| acc + x);
```

## Constraint System

The constraint system ensures that generic type parameters meet specific requirements, providing compile-time safety guarantees.

### Basic Constraints

```cursed
// Requiring a type to implement specific interfaces
vibes print_comparable<T>(value: T) where T: Display + Debug {
    println!("{}", value);
    println!("{:?}", value);
}

// Multiple constraints with different syntax
vibes complex_operation<T>(value: T) -> T 
where 
    T: Clone + Eq + Hash,
    T: Default
{
    lowkey (value == T::default()) {
        value.clone()
    } bestie {
        value
    }
}
```

### Type Bounds and Relationships

```cursed
// Constraining relationships between types
vibes convert_and_process<T, U>(input: T) -> U 
where 
    T: Into<U>,
    U: Display + Clone
{
    sus converted = input.into();
    println!("Converted: {}", converted);
    converted.clone()
}

// Associated type constraints
vibes process_iterator<I>(iter: I) 
where 
    I: Iterator,
    I::Item: Display + Clone
{
    lowkey sus item in iter {
        println!("Item: {}", item);
    }
}
```

### Lifetime Constraints

```cursed
// Generic function with lifetime constraints
vibes longest<'a, T>(x: &'a T, y: &'a T) -> &'a T 
where 
    T: Comparable<T>
{
    lowkey (x.compare(*y) > 0) {
        x
    } bestie {
        y
    }
}

// Complex lifetime relationships
vibes process_with_context<'a, 'b, T>(
    data: &'a T, 
    context: &'b Context
) -> &'a T 
where 
    'b: 'a,  // context outlives data
    T: Processable
{
    context.validate(data);
    data
}
```

## Advanced Constraint Patterns

### Higher-Kinded Types

```cursed
// A generic interface for container types
collab Container<F> where F: * -> * {
    type Item;
    
    vibes map<A, B>(self, func: vibes(A) -> B) -> F<B> 
    where 
        Self: F<A>;
}

// Implementation for Option
impl Container<Option> for Option<T> {
    type Item = T;
    
    vibes map<A, B>(self, func: vibes(A) -> B) -> Option<B> {
        match self {
            Some(value) => Some(func(value)),
            None => None,
        }
    }
}
```

### Associated Type Projections

```cursed
// Complex associated type relationships
collab GraphNode {
    type EdgeType: Edge;
    type NodeId: Hash + Eq;
    
    vibes get_edges() -> Vec<Self::EdgeType>;
    vibes get_id() -> Self::NodeId;
}

// Usage with projections
vibes traverse_graph<N>(start: N) 
where 
    N: GraphNode,
    N::EdgeType: Weighted,
    N::NodeId: Display
{
    sus edges = start.get_edges();
    lowkey sus edge in edges {
        println!("Edge weight: {}", edge.weight());
    }
}
```

### Constraint Composition

```cursed
// Composing multiple constraint patterns
vibes advanced_operation<T, U, V>(a: T, b: U) -> V 
where 
    T: Into<V> + Clone + Send,
    U: TryInto<V> + Sync,
    V: Default + Display + Debug,
    T::Error: Display,
    U::Error: From<T::Error>
{
    sus result_a = a.into();
    sus result_b = b.try_into().unwrap_or_default();
    
    lowkey (result_a == V::default()) {
        result_b
    } bestie {
        result_a
    }
}
```

## Performance Considerations

### Monomorphization

CURSED uses monomorphization to generate specialized versions of generic code for each concrete type:

```cursed
// This function...
vibes process<T>(value: T) -> T where T: Clone {
    value.clone()
}

// When used with i32 and String...
sus int_result = process(42);
sus str_result = process("hello");

// Generates two specialized functions:
// - process_i32(value: i32) -> i32
// - process_String(value: String) -> String
```

### Code Size Considerations

```cursed
// Good: Generic function used with few types
vibes simple_generic<T>(value: T) -> T {
    value
}

// Potential issue: Very complex generic used with many types
vibes complex_generic<T>(value: T) -> T 
where 
    T: Clone + Debug + Display + Serialize + Deserialize
{
    // Complex implementation that could bloat code size
    // when instantiated with many different types
}

// Solution: Use trait objects for dynamic dispatch when appropriate
vibes process_displayable(value: &dyn Display) {
    println!("{}", value);
}
```

### Compile-Time Performance

```cursed
// Efficient: Simple constraints
vibes fast_generic<T>(value: T) where T: Clone {}

// Less efficient: Complex constraint resolution
vibes slow_generic<T, U, V>(a: T, b: U, c: V) 
where 
    T: Into<U> + From<V>,
    U: TryInto<T> + TryFrom<V>,
    V: From<T> + Into<U>,
    // Many complex relationships slow down compilation
{}

// Best practice: Keep constraints simple and focused
vibes well_designed<T>(value: T) -> T 
where 
    T: Clone + Eq  // Simple, clear constraints
{
    value.clone()
}
```

## Best Practices

### 1. Keep Constraints Minimal

```cursed
// Good: Only constrain what you need
vibes compare_and_clone<T>(a: T, b: T) -> T 
where 
    T: PartialEq + Clone
{
    lowkey (a == b) { a.clone() } bestie { b.clone() }
}

// Avoid: Over-constraining
vibes over_constrained<T>(value: T) -> T 
where 
    T: Clone + Debug + Display + Serialize + Hash + Eq + Ord
{
    value.clone() // Only needs Clone!
}
```

### 2. Use Descriptive Type Parameter Names

```cursed
// Good: Descriptive names
vibes convert<Input, Output>(value: Input) -> Output 
where 
    Input: Into<Output>
{
    value.into()
}

// Good: Standard conventions
vibes map_container<Container, Item, NewItem>(
    container: Container, 
    mapper: vibes(Item) -> NewItem
) -> Container 
where 
    Container: Mappable<Item, NewItem>
{
    container.map(mapper)
}

// Avoid: Single letters without meaning
vibes confusing<A, B, C, D>(a: A, b: B, c: C) -> D {
    // What do these types represent?
}
```

### 3. Design for Zero-Cost Abstractions

```cursed
// Good: Compile-time specialization
vibes efficient_operation<T>(items: Vec<T>) -> usize 
where 
    T: Countable
{
    items.iter().map(|item| item.count()).sum()
}

// Consider: Runtime polymorphism when appropriate
vibes runtime_operation(items: Vec<&dyn Countable>) -> usize {
    items.iter().map(|item| item.count()).sum()
}
```

### 4. Provide Clear Error Messages

```cursed
// Good: Descriptive constraints that generate helpful errors
vibes validate_input<T>(input: T) -> Result<T, ValidationError> 
where 
    T: Validateable + Clone + Debug
{
    input.validate()?;
    Ok(input.clone())
}

// Use custom constraint traits for better error messages
collab UserValidatable {
    vibes validate_user(&self) -> Result<(), UserValidationError>;
}

vibes process_user<T>(user: T) -> Result<T, UserValidationError> 
where 
    T: UserValidatable
{
    user.validate_user()?;
    Ok(user)
}
```

### 5. Document Generic APIs Thoroughly

```cursed
/// Processes a collection of items using a generic transformation.
/// 
/// # Type Parameters
/// 
/// * `Input` - The input item type, must implement `Clone` for internal operations
/// * `Output` - The output item type, must implement `Default` for initialization
/// * `Processor` - The transformation function type
/// 
/// # Examples
/// 
/// ```cursed
/// sus numbers = vec![1, 2, 3];
/// sus doubled = process_collection(numbers, |x| x * 2);
/// ```
vibes process_collection<Input, Output, Processor>(
    items: Vec<Input>, 
    processor: Processor
) -> Vec<Output> 
where 
    Input: Clone,
    Output: Default,
    Processor: Fn(Input) -> Output
{
    items.into_iter().map(processor).collect()
}
```

## Migration Guide

### From Non-Generic to Generic Code

#### Step 1: Identify Duplication

```cursed
// Before: Separate functions for different types
vibes print_int(value: i32) {
    println!("{}", value);
}

vibes print_string(value: String) {
    println!("{}", value);
}

// After: Single generic function
vibes print_value<T>(value: T) where T: Display {
    println!("{}", value);
}
```

#### Step 2: Extract Common Patterns

```cursed
// Before: Type-specific containers
squad IntBox {
    value: i32,
}

squad StringBox {
    value: String,
}

// After: Generic container
squad Box<T> {
    value: T,
}
```

#### Step 3: Add Appropriate Constraints

```cursed
// Before: Assuming specific types
vibes old_compare(a: i32, b: i32) -> i32 {
    a - b
}

// After: Generic with constraints
vibes compare<T>(a: T, b: T) -> Ordering 
where 
    T: Ord
{
    a.cmp(&b)
}
```

### Updating Existing Code

#### Gradual Migration Strategy

1. **Identify Generic Opportunities**: Look for code duplication across types
2. **Start with Simple Cases**: Begin with functions that only need basic constraints
3. **Add Constraints Incrementally**: Start permissive, add constraints as needed
4. **Test Thoroughly**: Ensure all type combinations work correctly
5. **Update Documentation**: Explain generic usage patterns

#### Common Migration Patterns

```cursed
// Pattern 1: Option/Result wrappers
// Before
squad IntResult {
    success: bool,
    value: i32,
    error: String,
}

// After
squad Result<T, E> {
    success: bool,
    value: T?,
    error: E?,
}

// Pattern 2: Collection operations
// Before
vibes find_int(items: Vec<i32>, target: i32) -> Option<usize> {
    // Implementation
}

// After
vibes find<T>(items: Vec<T>, target: T) -> Option<usize> 
where 
    T: PartialEq
{
    // Implementation
}
```

## Troubleshooting

### Common Constraint Errors

#### Error: "Type parameter does not satisfy constraints"

```cursed
// Problem
vibes broken_function<T>(value: T) {
    println!("{}", value); // Error: T doesn't implement Display
}

// Solution
vibes fixed_function<T>(value: T) where T: Display {
    println!("{}", value);
}
```

#### Error: "Cannot infer type parameter"

```cursed
// Problem
sus result = some_generic_function(); // Error: Can't infer T

// Solutions
sus result = some_generic_function::<i32>(); // Explicit type
sus result: i32 = some_generic_function();   // Type annotation
```

#### Error: "Conflicting implementations"

```cursed
// Problem: Overlapping implementations
impl<T> MyTrait for T where T: Display {}
impl<T> MyTrait for T where T: Debug {}   // Conflict!

// Solution: Make implementations non-overlapping
impl<T> MyTrait for T where T: Display + !Debug {}
impl<T> MyTrait for T where T: Debug + !Display {}
impl<T> MyTrait for T where T: Display + Debug {
    // Explicit implementation for overlap
}
```

### Performance Issues

#### Issue: Code bloat from over-monomorphization

```cursed
// Problem: Too many instantiations
vibes overused_generic<T>(value: T) -> T 
where 
    T: Clone + Debug + Display + Serialize
{
    // Used with 50+ different types = large binary
}

// Solution: Use trait objects for some cases
vibes use_trait_object(value: &dyn Display) {
    println!("{}", value);
}
```

#### Issue: Slow compilation

```cursed
// Problem: Complex constraint resolution
vibes complex_constraints<A, B, C, D, E>(a: A, b: B, c: C, d: D, e: E) 
where 
    A: Into<B> + From<C> + TryInto<D> + TryFrom<E>,
    B: Into<A> + From<D> + TryInto<C> + TryFrom<E>,
    // ... many more complex relationships
{}

// Solution: Simplify constraint graph
vibes simpler_approach<T, U>(input: T) -> U 
where 
    T: Into<U>
{
    input.into()
}
```

### Debugging Tips

1. **Use `--verbose-generics` flag** to see instantiation details
2. **Check constraint resolution** with `--debug-constraints`
3. **Measure compile times** with `--time-passes`
4. **Profile binary size** with generic usage statistics

### Best Practices for Debugging

```cursed
// Use Debug trait for troubleshooting
vibes debug_generic<T>(value: T) where T: Debug {
    println!("Debug: {:?}", value);
}

// Add helpful error context
vibes fallible_generic<T>(value: T) -> Result<T, ProcessingError> 
where 
    T: Processable
{
    value.process()
        .map_err(|e| ProcessingError::with_context(
            format!("Failed to process {}", std::any::type_name::<T>()),
            e
        ))
}
```

---

This guide provides comprehensive coverage of CURSED's generic system. For more advanced topics, see the [Type System Architecture](type_system_architecture.md) documentation.
