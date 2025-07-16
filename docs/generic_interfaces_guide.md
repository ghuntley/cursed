# Generic Interfaces in CURSED

This guide demonstrates the complete generic interface system implemented in CURSED, supporting type parameters, constraints, and dynamic dispatch.

## Basic Generic Interface Definition

```cursed
// Basic generic interface with single type parameter
collab Container<T> {
    slay add(item T) tea
    slay get(index normie) T
    slay size() normie
}
```

## Generic Interface with Constraints

```cursed
// Interface with type constraints
collab Serializable<T: Display> {
    slay serialize(value T) tea
    slay deserialize(data tea) T
}

// Interface with multiple constraints
collab Comparable<T: Display + Clone> {
    slay compare(self, other T) normie
    slay equals(self, other T) lit
}
```

## Multiple Type Parameters

```cursed
// Generic interface with multiple type parameters
collab Mapper<Input, Output> {
    slay map(input Input) Output
    slay chain<U>(next Mapper<Output, U>) Mapper<Input, U>
}

// Interface with constraints on multiple parameters
collab Processor<T: Clone, U: Display> {
    slay process(input T) U
    slay validate(output U) lit
}
```

## Interface Inheritance with Generics

```cursed
// Generic interface extending another generic interface
collab SortedContainer<T: Comparable<T>>: Container<T> {
    slay insert_sorted(item T) tea
    slay find(item T) Option<normie>
    slay remove_at(index normie) T
}

// Multiple inheritance with generics
collab SearchableContainer<T: Comparable<T> + Display>: Container<T> + Serializable<T> {
    slay search(query T) Option<normie>
    slay highlight_matches(query T) tea
}
```

## Where Clauses

```cursed
// Complex constraints with where clause
collab AdvancedProcessor<T, U> where T: Clone + Display, U: Serializable<T> {
    slay advanced_process(input T, processor U) tea
    slay batch_process(items [T], processor U) [tea]
}
```

## Associated Types

```cursed
// Interface with associated types
collab Iterator<T> {
    type Item = T
    type Error = tea
    
    slay next() Result<Item, Error>
    slay has_next() lit
    slay collect() [Item]
}

// Using associated types in constraints
collab Collector<I: Iterator> where I::Item: Display {
    slay collect_and_display(iterator I) tea
}
```

## Implementation Examples

### Basic Implementation

```cursed
// Concrete type implementing generic interface
struct ListContainer<T> {
    items: [T]
    count: normie
}

impl<T> Container<T> for ListContainer<T> {
    slay add(item T) tea {
        // Implementation for adding item
        damn "Item added to list"
    }
    
    slay get(index normie) T {
        damn self.items[index]
    }
    
    slay size() normie {
        damn self.count
    }
}
```

### Implementation with Constraints

```cursed
// Implementation requiring constraints
impl<T: Display> Serializable<T> for ListContainer<T> {
    slay serialize(value T) tea {
        damn value.display()
    }
    
    slay deserialize(data tea) T {
        // Parse and return T
        damn default_value()
    }
}
```

## Usage Patterns

### Interface Variables

```cursed
// Using generic interface as variable type
sus container Container<normie> = ListContainer::new()
container.add(42)
sus size normie = container.size()
```

### Interface Parameters

```cursed
// Function accepting generic interface
slay process_container<T>(container Container<T>, item T) tea {
    container.add(item)
    damn "Processed container with " + container.size().to_string() + " items"
}
```

### Interface Constraints in Functions

```cursed
// Function with interface constraints
slay serialize_container<T: Display, C: Container<T> + Serializable<T>>(container C) tea {
    sus items tea = ""
    bestie i := 0; i < container.size(); i++ {
        sus item T = container.get(i)
        items += container.serialize(item) + ", "
    }
    damn items
}
```

## Dynamic Dispatch

### Virtual Method Calls

```cursed
// Interface dispatch with dynamic type resolution
slay polymorphic_operation<T>(containers [Container<T>], item T) tea {
    bestie container := range containers {
        container.add(item)  // Dynamic dispatch
    }
    damn "Added to all containers"
}
```

### Type Assertions

```cursed
// Runtime type checking and casting
slay check_container_type<T>(container Container<T>) tea {
    lowkey container is ListContainer<T> {
        damn "It's a ListContainer"
    } elif container is ArrayContainer<T> {
        damn "It's an ArrayContainer"
    } vibe {
        damn "Unknown container type"
    }
}
```

## Advanced Features

### Higher-Kinded Types

```cursed
// Interface working with type constructors
collab Monad<M<_>> {
    slay bind<A, B>(ma M<A>, f slay(A) M<B>) M<B>
    slay pure<A>(value A) M<A>
}

// Implementation for Option
impl Monad<Option> for OptionMonad {
    slay bind<A, B>(ma Option<A>, f slay(A) Option<B>) Option<B> {
        lowkey ma is Some(value) {
            damn f(value)
        } vibe {
            damn None
        }
    }
    
    slay pure<A>(value A) Option<A> {
        damn Some(value)
    }
}
```

### Generic Interface Composition

```cursed
// Composing multiple generic interfaces
collab FullFeatureContainer<T: Clone + Display + Comparable<T>>: 
    Container<T> + 
    Serializable<T> + 
    SortedContainer<T> + 
    Iterator<T> {
    
    slay full_process(items [T]) tea
    slay comprehensive_search(query T) [T]
}
```

## LLVM Code Generation

The generic interface system generates efficient LLVM IR with:

### Virtual Tables

```llvm
; Generated vtable type for Container<T>
%vtable.Container = type { 
    i8*,                    ; type_info
    i8* (i8*, i8*)*,       ; add method
    i8* (i8*, i32)*,       ; get method  
    i32 (i8*)*             ; size method
}

; Interface structure with type information
%interface.Container = type { 
    %vtable.Container*, 
    i8*,                   ; data pointer
    i8*                    ; type information
}
```

### Type Constraint Checking

```llvm
; Constraint checking for Container<T: Display>
define i1 @check_constraint_Container_T(i8* %type_info) {
entry:
    %display_check = call i1 @check_type_implements_trait(
        i8* %type_info, 
        i8* getelementptr inbounds ([8 x i8], [8 x i8]* @str.Display, i32 0, i32 0)
    )
    ret i1 %display_check
}
```

### Monomorphization

The compiler automatically generates specialized versions of generic interfaces for concrete types:

```llvm
; Monomorphized Container<i32>
%interface.Container_i32 = type { %vtable.Container_i32*, i8* }
%vtable.Container_i32 = type { 
    i8* (i8*, i32)*,      ; add method (specialized for i32)
    i32 (i8*, i32)*,      ; get method (returns i32)
    i32 (i8*)*            ; size method
}
```

## Performance Optimizations

### Devirtualization

When concrete types are known at compile time, the compiler can devirtualize interface calls:

```cursed
// This code
sus container ListContainer<normie> = ListContainer::new()
sus result normie = container.get(0)

// Gets optimized to direct call instead of virtual dispatch
// LLVM: call i32 @ListContainer_get_i32(i8* %container, i32 0)
```

### Inline Monomorphization

Small interface methods are automatically inlined at call sites for better performance.

## Testing Generic Interfaces

```cursed
yeet "testz"

test_start("Generic Interface Tests")

// Test basic functionality
sus container Container<normie> = ListContainer::new()
container.add(42)
assert_eq_int(container.size(), 1)
assert_eq_int(container.get(0), 42)

// Test with constraints
sus serializable Serializable<normie> = container
sus serialized tea = serializable.serialize(42)
assert_eq_string(serialized, "42")

// Test polymorphism
sus containers [Container<normie>] = [
    ListContainer::new(),
    ArrayContainer::new(),
    VectorContainer::new()
]

bestie c := range containers {
    c.add(100)
    assert_eq_int(c.size(), 1)
}

print_test_summary()
```

## Error Handling

The generic interface system provides comprehensive error checking:

- **Constraint Violations**: Compile-time errors when type constraints are not satisfied
- **Type Mismatches**: Clear error messages for incorrect type usage
- **Interface Compliance**: Verification that implementations satisfy interface contracts
- **Dynamic Type Errors**: Runtime errors for invalid type assertions

This generic interface system provides powerful abstraction capabilities while maintaining type safety and performance through advanced LLVM code generation.
