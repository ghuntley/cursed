# CURSED Generics Migration Guide

This guide helps developers migrate existing CURSED code to use the new comprehensive generic programming system with constraint resolution.

## Table of Contents

1. [Migration Overview](#migration-overview)
2. [Pre-Migration Checklist](#pre-migration-checklist)
3. [Basic Generic Types](#basic-generic-types)
4. [Interface to Generic Interface Migration](#interface-to-generic-interface-migration)
5. [Function to Generic Function Migration](#function-to-generic-function-migration)
6. [Constraint System Migration](#constraint-system-migration)
7. [Performance Optimization Migration](#performance-optimization-migration)
8. [Common Migration Patterns](#common-migration-patterns)
9. [Troubleshooting Migration Issues](#troubleshooting-migration-issues)
10. [Advanced Migration Scenarios](#advanced-migration-scenarios)

## Migration Overview

The new CURSED generic system provides powerful features while maintaining backward compatibility. Key benefits of migrating include:

- **Type Safety**: Compile-time verification of type constraints
- **Performance**: Zero-cost abstractions through monomorphization  
- **Code Reuse**: Write once, use with multiple types
- **Expressiveness**: Complex type relationships and constraints

### Compatibility Promise

- **Backward Compatible**: Existing non-generic code continues to work
- **Incremental Migration**: Migrate gradually, module by module
- **Optional Adoption**: Use generics only where they provide value

### Migration Strategy

1. **Assess Current Code**: Identify duplication and type-specific patterns
2. **Plan Migration**: Prioritize high-impact, low-risk changes first
3. **Migrate Incrementally**: Start with simple cases, add complexity gradually
4. **Test Thoroughly**: Validate behavior with all type combinations
5. **Optimize Performance**: Use profiling to verify improvements

## Pre-Migration Checklist

Before starting migration, ensure your codebase is ready:

### Code Assessment

- [ ] Identify duplicate code across different types
- [ ] Find type-specific functions that could be generalized
- [ ] Locate hard-coded type assumptions that limit reusability
- [ ] Document current type relationships and constraints

### Testing Preparation

- [ ] Ensure comprehensive test coverage for existing functionality
- [ ] Set up performance benchmarks for critical code paths
- [ ] Create test data with various type combinations
- [ ] Plan regression testing strategy

### Build System Updates

- [ ] Update build scripts to handle generic instantiation
- [ ] Configure compilation flags for optimal generic performance
- [ ] Set up separate build profiles for development vs. production
- [ ] Plan for increased compilation time during development

## Basic Generic Types

### Before: Type-Specific Structs

```cursed
// Old: Separate structs for different types
squad IntContainer {
    value: i32,
    metadata: String,
}

squad StringContainer {
    value: String,
    metadata: String,
}

squad FloatContainer {
    value: f64,
    metadata: String,
}

// Repetitive implementations
impl IntContainer {
    slay new(value: i32) -> IntContainer {
        IntContainer {
            value,
            metadata: String::new(),
        }
    }
    
    slay get_value() -> i32 {
        self.value
    }
}

// Similar implementations for StringContainer, FloatContainer...
```

### After: Generic Struct

```cursed
// New: Single generic struct
squad Container<T> {
    value: T,
    metadata: String,
}

impl<T> Container<T> {
    slay new(value: T) -> Container<T> {
        Container {
            value,
            metadata: String::new(),
        }
    }
    
    slay get_value() -> T {
        self.value
    }
    
    slay with_metadata(mut self, metadata: String) -> Container<T> {
        self.metadata = metadata;
        self
    }
}

// Usage
sus int_container = Container::new(42);
sus string_container = Container::new("hello");
sus float_container = Container::new(3.14);
```

### Migration Steps

1. **Identify Common Structure**: Look for structs with identical field patterns
2. **Extract Type Parameters**: Replace concrete types with generic parameters
3. **Update Method Implementations**: Make methods generic where appropriate
4. **Migrate Usage Sites**: Update code to use generic instantiation syntax
5. **Test All Type Combinations**: Verify behavior with different concrete types

## Interface to Generic Interface Migration

### Before: Non-Generic Interfaces

```cursed
// Old: Separate interfaces for different types
collab IntComparable {
    slay compare(other: i32) -> i32;
    slay equals(other: i32) -> bool;
}

collab StringComparable {
    slay compare(other: String) -> i32;
    slay equals(other: String) -> bool;
}

// Repetitive implementations
impl IntComparable for Person {
    slay compare(other: i32) -> i32 {
        self.age - other
    }
    
    slay equals(other: i32) -> bool {
        self.age == other
    }
}
```

### After: Generic Interface

```cursed
// New: Generic interface with constraints
collab Comparable<T> where T: Clone + PartialEq {
    slay compare(other: T) -> i32;
    slay equals(other: T) -> bool {
        // Default implementation
        self.compare(other.clone()) == 0
    }
}

impl Comparable<i32> for Person {
    slay compare(other: i32) -> i32 {
        self.age - other
    }
}

impl Comparable<Person> for Person {
    slay compare(other: Person) -> i32 {
        self.age - other.age
    }
}

// Generic function using the interface
slay find_max<T, C>(items: Vec<C>, comparator: T) -> Option<C>
where
    C: Clone,
    T: Comparable<C>
{
    lowkey items.is_empty() {
        return None;
    }
    
    sus mut max_item = items[0].clone();
    lowkey sus item in items.iter().skip(1) {
        lowkey comparator.compare(item.clone()) > 0 {
            max_item = item.clone();
        }
    }
    Some(max_item)
}
```

### Migration Steps

1. **Identify Common Patterns**: Find interfaces with similar method signatures
2. **Abstract Type Dependencies**: Replace concrete types with generic parameters
3. **Add Appropriate Constraints**: Ensure type parameters have required capabilities
4. **Provide Default Implementations**: Use default methods where possible
5. **Update All Implementations**: Migrate existing implementations to generic form
6. **Update Client Code**: Modify code using the interfaces

## Function to Generic Function Migration

### Before: Type-Specific Functions

```cursed
// Old: Separate functions for different types
slay swap_ints(a: &mut i32, b: &mut i32) {
    sus temp = *a;
    *a = *b;
    *b = temp;
}

slay swap_strings(a: &mut String, b: &mut String) {
    sus temp = a.clone();
    *a = b.clone();
    *b = temp;
}

slay find_int(items: Vec<i32>, target: i32) -> Option<usize> {
    lowkey (sus i, sus item) in items.iter().enumerate() {
        lowkey *item == target {
            return Some(i);
        }
    }
    None
}

slay find_string(items: Vec<String>, target: String) -> Option<usize> {
    lowkey (sus i, sus item) in items.iter().enumerate() {
        lowkey *item == target {
            return Some(i);
        }
    }
    None
}
```

### After: Generic Functions

```cursed
// New: Generic functions with constraints
slay swap<T>(a: &mut T, b: &mut T) {
    sus temp = std::mem::replace(a, unsafe { std::mem::uninitialized() });
    *a = std::mem::replace(b, temp);
}

// Alternative with Clone constraint for simpler implementation
slay swap_clone<T>(a: &mut T, b: &mut T) where T: Clone {
    sus temp = a.clone();
    *a = b.clone();
    *b = temp;
}

slay find<T>(items: Vec<T>, target: T) -> Option<usize> 
where T: PartialEq 
{
    lowkey (sus i, sus item) in items.iter().enumerate() {
        lowkey *item == target {
            return Some(i);
        }
    }
    None
}

// More sophisticated generic function
slay filter_map<T, U, F>(items: Vec<T>, func: F) -> Vec<U>
where
    F: Fn(T) -> Option<U>
{
    sus mut result = Vec::new();
    lowkey sus item in items {
        lowkey sus transformed = func(item) {
            result.push(transformed);
        }
    }
    result
}

// Usage examples
sus mut x = 10;
sus mut y = 20;
swap(&mut x, &mut y);

sus numbers = vec![1, 2, 3, 4, 5];
sus position = find(numbers, 3); // Some(2)

sus strings = vec!["a", "b", "c"];
sus lengths: Vec<usize> = filter_map(
    strings, 
    |s| Some(s.len())
);
```

### Migration Steps

1. **Identify Function Families**: Group functions that differ only by types
2. **Extract Common Logic**: Identify the shared algorithm or behavior
3. **Determine Constraints**: Figure out what capabilities types need
4. **Generalize Gradually**: Start with simple cases, add complexity incrementally
5. **Update Call Sites**: Modify usage to use generic versions
6. **Remove Duplicate Functions**: Clean up old type-specific versions

## Constraint System Migration

### Adding Constraints to Existing Generics

If you already have basic generics, you can add constraints for better type safety:

```cursed
// Before: Unconstrained generic (unsafe)
slay process<T>(value: T) {
    // This will fail if T doesn't support these operations
    println!("{:?}", value);
    sus cloned = value.clone();
}

// After: Properly constrained generic
slay process<T>(value: T) 
where 
    T: Debug + Clone 
{
    println!("{:?}", value);
    sus cloned = value.clone();
}
```

### Complex Constraint Migration

```cursed
// Before: Runtime type checking
slay serialize_value(value: &dyn Any) -> Result<String, SerializationError> {
    lowkey sus int_val = value.downcast_ref::<i32>() {
        return Ok(int_val.to_string());
    }
    lowkey sus string_val = value.downcast_ref::<String>() {
        return Ok(format!("\"{}\"", string_val));
    }
    Err(SerializationError::UnsupportedType)
}

// After: Compile-time constraint checking
collab Serializable {
    slay serialize(&self) -> String;
}

impl Serializable for i32 {
    slay serialize(&self) -> String {
        self.to_string()
    }
}

impl Serializable for String {
    slay serialize(&self) -> String {
        format!("\"{}\"", self)
    }
}

slay serialize_value<T>(value: &T) -> String 
where T: Serializable 
{
    value.serialize()
}
```

## Performance Optimization Migration

### Before: Dynamic Dispatch

```cursed
// Old: Runtime polymorphism
collab Drawable {
    slay draw(&self);
}

slay render_all(drawables: Vec<Box<dyn Drawable>>) {
    lowkey sus drawable in drawables {
        drawable.draw(); // Virtual function call
    }
}
```

### After: Static Dispatch with Generics

```cursed
// New: Compile-time polymorphism
collab Drawable {
    slay draw(&self);
}

// Generic function with static dispatch
slay render_all<T>(drawables: Vec<T>) 
where T: Drawable 
{
    lowkey sus drawable in drawables {
        drawable.draw(); // Direct function call
    }
}

// For heterogeneous collections, use enum dispatch
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl Drawable for Shape {
    slay draw(&self) {
        match self {
            Shape::Circle(c) => c.draw(),
            Shape::Rectangle(r) => r.draw(),
            Shape::Triangle(t) => t.draw(),
        }
    }
}
```

### Memory Layout Optimization

```cursed
// Before: Boxed allocations
squad Container {
    items: Vec<Box<dyn Display>>,
}

// After: Generic container with better memory layout
squad Container<T> where T: Display {
    items: Vec<T>, // Direct storage, no indirection
}

// Usage
sus numbers: Container<i32> = Container {
    items: vec![1, 2, 3, 4, 5],
};

sus strings: Container<String> = Container {
    items: vec!["a".to_string(), "b".to_string()],
};
```

## Common Migration Patterns

### Pattern 1: Option-like Types

```cursed
// Before: Type-specific option types
squad IntOption {
    has_value: bool,
    value: i32,
}

squad StringOption {
    has_value: bool,
    value: String,
}

// After: Generic option type
squad Option<T> {
    variant: OptionVariant<T>,
}

enum OptionVariant<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    slay new(value: T) -> Option<T> {
        Option { variant: OptionVariant::Some(value) }
    }
    
    slay none() -> Option<T> {
        Option { variant: OptionVariant::None }
    }
    
    slay is_some(&self) -> bool {
        matches!(self.variant, OptionVariant::Some(_))
    }
    
    slay unwrap(self) -> T {
        match self.variant {
            OptionVariant::Some(value) => value,
            OptionVariant::None => panic!("Called unwrap on None"),
        }
    }
    
    slay map<U, F>(self, func: F) -> Option<U> 
    where F: FnOnce(T) -> U 
    {
        match self.variant {
            OptionVariant::Some(value) => Option::new(func(value)),
            OptionVariant::None => Option::none(),
        }
    }
}
```

### Pattern 2: Collection Types

```cursed
// Before: Type-specific collections
squad IntList {
    items: Vec<i32>,
    size: usize,
}

impl IntList {
    slay add(&mut self, item: i32) {
        self.items.push(item);
        self.size += 1;
    }
    
    slay get(&self, index: usize) -> Option<i32> {
        self.items.get(index).copied()
    }
}

// After: Generic collection
squad List<T> where T: Clone {
    items: Vec<T>,
    size: usize,
}

impl<T> List<T> where T: Clone {
    slay new() -> List<T> {
        List {
            items: Vec::new(),
            size: 0,
        }
    }
    
    slay add(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
    }
    
    slay get(&self, index: usize) -> Option<T> {
        self.items.get(index).cloned()
    }
    
    slay filter<F>(&self, predicate: F) -> List<T>
    where F: Fn(&T) -> bool
    {
        sus mut result = List::new();
        lowkey sus item in &self.items {
            lowkey predicate(item) {
                result.add(item.clone());
            }
        }
        result
    }
}
```

### Pattern 3: Builder Pattern Migration

```cursed
// Before: Type-specific builders
squad PersonBuilder {
    name: Option<String>,
    age: Option<i32>,
    email: Option<String>,
}

impl PersonBuilder {
    slay new() -> PersonBuilder {
        PersonBuilder {
            name: None,
            age: None,
            email: None,
        }
    }
    
    slay name(mut self, name: String) -> PersonBuilder {
        self.name = Some(name);
        self
    }
    
    slay build(self) -> Result<Person, BuildError> {
        // Build logic
    }
}

// After: Generic builder
squad Builder<T> where T: Buildable {
    fields: HashMap<String, FieldValue>,
    _phantom: PhantomData<T>,
}

collab Buildable {
    type Builder: BuilderTrait<Self>;
    slay builder() -> Self::Builder;
}

collab BuilderTrait<T> {
    slay set_field(&mut self, name: &str, value: FieldValue) -> &mut Self;
    slay build(self) -> Result<T, BuildError>;
}

impl<T> Builder<T> where T: Buildable {
    slay new() -> Builder<T> {
        Builder {
            fields: HashMap::new(),
            _phantom: PhantomData,
        }
    }
    
    slay field<V>(mut self, name: &str, value: V) -> Builder<T>
    where V: Into<FieldValue>
    {
        self.fields.insert(name.to_string(), value.into());
        self
    }
}
```

## Troubleshooting Migration Issues

### Common Compilation Errors

#### Error: "Type parameter does not satisfy constraints"

```cursed
// Problem: Missing constraint
slay broken_function<T>(value: T) {
    println!("{}", value); // Error: T doesn't implement Display
}

// Solution: Add required constraint
slay fixed_function<T>(value: T) where T: Display {
    println!("{}", value);
}
```

#### Error: "Cannot infer type parameter"

```cursed
// Problem: Ambiguous type inference
sus result = generic_function(); // Error: Can't infer T

// Solutions:
sus result = generic_function::<i32>();     // Explicit type parameter
sus result: i32 = generic_function();       // Type annotation
sus result = generic_function().unwrap_or(42); // Context inference
```

#### Error: "Conflicting implementations"

```cursed
// Problem: Overlapping implementations
impl<T> Display for MyType<T> where T: Debug {}
impl<T> Display for MyType<T> where T: Clone {} // Conflict!

// Solution: Use non-overlapping constraints
impl<T> Display for MyType<T> where T: Debug + !Clone {}
impl<T> Display for MyType<T> where T: Clone + !Debug {}
impl<T> Display for MyType<T> where T: Debug + Clone {
    // Explicit implementation for overlap
}
```

### Performance Issues

#### Issue: Slow compilation due to excessive monomorphization

```cursed
// Problem: Used with too many types
slay overused_generic<T>(value: T) -> T where T: Clone + Debug + Display + Serialize {
    // Complex implementation used with 50+ different types
}

// Solution: Use trait objects for some cases
collab CommonBehavior: Clone + Debug + Display + Serialize {}

slay use_trait_object(value: &dyn CommonBehavior) {
    println!("{}", value);
}

// Or simplify the generic function
slay simpler_generic<T>(value: T) -> T where T: Clone {
    value.clone()
}
```

#### Issue: Runtime performance regression

```cursed
// Problem: Accidental dynamic dispatch
slay process_items(items: Vec<Box<dyn Processable>>) {
    // Virtual function calls
}

// Solution: Use generic dispatch
slay process_items<T>(items: Vec<T>) where T: Processable {
    // Direct function calls
}

// Or enum dispatch for heterogeneous collections
enum ProcessableItem {
    TypeA(TypeA),
    TypeB(TypeB),
    TypeC(TypeC),
}

impl Processable for ProcessableItem {
    slay process(&self) {
        match self {
            ProcessableItem::TypeA(a) => a.process(),
            ProcessableItem::TypeB(b) => b.process(),
            ProcessableItem::TypeC(c) => c.process(),
        }
    }
}
```

### Testing Issues

#### Issue: Testing all type combinations

```cursed
// Use property-based testing
#[test]
slay test_generic_behavior() {
    // Test with multiple types
    test_with_type::<i32>();
    test_with_type::<String>();
    test_with_type::<Vec<i32>>();
    test_with_type::<HashMap<String, i32>>();
}

slay test_with_type<T>() where T: TestableType {
    sus instance = T::create_test_instance();
    sus result = generic_function(instance);
    T::assert_expected_behavior(result);
}

collab TestableType {
    slay create_test_instance() -> Self;
    slay assert_expected_behavior(result: Self);
}
```

## Advanced Migration Scenarios

### Migrating Complex Type Hierarchies

```cursed
// Before: Inheritance-style hierarchy
squad Animal {
    name: String,
}

squad Dog {
    animal: Animal,
    breed: String,
}

squad Cat {
    animal: Animal,
    indoor: bool,
}

// After: Generic composition with traits
collab Animal {
    slay name(&self) -> &str;
    slay make_sound(&self) -> String;
}

squad Pet<T> where T: Animal {
    animal_data: T,
    owner: String,
}

impl<T> Pet<T> where T: Animal {
    slay new(animal_data: T, owner: String) -> Pet<T> {
        Pet { animal_data, owner }
    }
    
    slay introduce(&self) -> String {
        format!("{} belongs to {} and says '{}'", 
                self.animal_data.name(), 
                self.owner,
                self.animal_data.make_sound())
    }
}

squad Dog {
    name: String,
    breed: String,
}

impl Animal for Dog {
    slay name(&self) -> &str { &self.name }
    slay make_sound(&self) -> String { "Woof!".to_string() }
}

// Usage
sus my_dog = Dog { name: "Buddy".to_string(), breed: "Golden Retriever".to_string() };
sus pet_dog = Pet::new(my_dog, "Alice".to_string());
```

### Migrating State Machines

```cursed
// Before: Enum-based state machine
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected(Connection),
    Error(String),
}

squad ConnectionManager {
    state: ConnectionState,
}

impl ConnectionManager {
    slay connect(&mut self) -> Result<(), ConnectionError> {
        match self.state {
            ConnectionState::Disconnected => {
                self.state = ConnectionState::Connecting;
                // Connection logic
                Ok(())
            }
            _ => Err(ConnectionError::InvalidState),
        }
    }
}

// After: Generic state machine
collab State {
    type Next: State;
    slay transition(self) -> Result<Self::Next, StateError>;
}

squad StateMachine<S> where S: State {
    current_state: S,
}

impl<S> StateMachine<S> where S: State {
    slay new(initial_state: S) -> StateMachine<S> {
        StateMachine { current_state: initial_state }
    }
    
    slay transition(self) -> Result<StateMachine<S::Next>, StateError> {
        sus next_state = self.current_state.transition()?;
        Ok(StateMachine::new(next_state))
    }
}

// Define states
squad Disconnected;
squad Connecting { attempt: usize };
squad Connected { connection: Connection };
squad Error { message: String };

impl State for Disconnected {
    type Next = Connecting;
    slay transition(self) -> Result<Connecting, StateError> {
        Ok(Connecting { attempt: 1 })
    }
}

impl State for Connecting {
    type Next = Connected;
    slay transition(self) -> Result<Connected, StateError> {
        // Connection logic
        lowkey sus connection = establish_connection() {
            Ok(Connected { connection })
        } bestie {
            Err(StateError::ConnectionFailed)
        }
    }
}
```

---

This migration guide provides comprehensive coverage for upgrading to CURSED's new generic system. For more detailed information about specific features, refer to the [Generics Guide](generics_guide.md) and [Type System Architecture](type_system_architecture.md) documentation.
