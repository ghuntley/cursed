# Generics and Constraints

CURSED's generics system provides type-safe, zero-cost abstractions with powerful constraint mechanisms.

## Basic Generics

### Generic Functions
```cursed
# Simple generic function
slay identity[T](value T) T {
    damn value
}

# Usage
sus int_result normie = identity[normie](42)
sus str_result tea = identity[tea]("hello")
```

### Multiple Type Parameters
```cursed
slay pair[T, U](first T, second U) (T, U) {
    damn first, second
}

sus result = pair[normie, tea](42, "hello")
```

### Generic Structs
```cursed
struct Container[T] {
    value T
    count normie
}

sus int_container Container[normie] = Container[normie]{
    value: 42,
    count: 1,
}
```

## Type Constraints

### Interface Constraints
```cursed
interface Comparable[T] {
    compare(other T) normie
}

# Generic function with constraint
slay max[T: Comparable[T]](a T, b T) T {
    lowkey a.compare(b) > 0 {
        damn a
    }
    damn b
}
```

### Multiple Constraints
```cursed
interface Printable {
    to_string() tea
}

interface Serializable {
    serialize() []byte
}

slay process[T: Printable & Serializable](item T) {
    vibez.spill(item.to_string())
    sus data []byte = item.serialize()
    # Process serialized data
}
```

### Type Bounds
```cursed
# Numeric constraint
interface Numeric {
    add(other Self) Self
    multiply(other Self) Self
}

slay calculate[T: Numeric](a T, b T) T {
    damn a.add(b).multiply(a)
}
```

## Advanced Generic Patterns

### Generic Collections
```cursed
struct List[T] {
    items []T
    capacity normie
}

slay new_list[T](capacity normie) List[T] {
    damn List[T]{
        items: make([]T, 0, capacity),
        capacity: capacity,
    }
}

slay (list *List[T]) add(item T) {
    list.items = append(list.items, item)
}

slay (list *List[T]) get(index normie) T {
    damn list.items[index]
}
```

### Generic Methods
```cursed
struct Stack[T] {
    items []T
}

slay (s *Stack[T]) push(item T) {
    s.items = append(s.items, item)
}

slay (s *Stack[T]) pop() T {
    lowkey len(s.items) == 0 {
        yikes "Stack is empty"
    }
    
    sus item T = s.items[len(s.items)-1]
    s.items = s.items[:len(s.items)-1]
    damn item
}

slay (s *Stack[T]) is_empty() lit {
    damn len(s.items) == 0
}
```

## Generic Interfaces

### Basic Generic Interface
```cursed
interface Iterator[T] {
    next() (T, lit)
    has_next() lit
}

# Implementation
struct SliceIterator[T] {
    items []T
    index normie
}

slay (iter *SliceIterator[T]) next() (T, lit) {
    lowkey iter.index >= len(iter.items) {
        sus zero T
        damn zero, cap
    }
    
    sus item T = iter.items[iter.index]
    iter.index++
    damn item, based
}

slay (iter *SliceIterator[T]) has_next() lit {
    damn iter.index < len(iter.items)
}
```

### Complex Generic Interface
```cursed
interface Repository[T, ID] {
    create(entity T) (ID, tea)
    find_by_id(id ID) (T, lit)
    update(id ID, entity T) tea
    delete(id ID) tea
    list() []T
}

# Implementation
struct MemoryRepository[T, ID] {
    data map[ID]T
    next_id ID
}

slay (repo *MemoryRepository[T, ID]) create(entity T) (ID, tea) {
    sus id ID = repo.next_id
    repo.data[id] = entity
    repo.next_id++
    damn id, ""
}
```

## Type Inference

### Automatic Type Inference
```cursed
# Type can be inferred from usage
sus numbers []normie = [1, 2, 3, 4, 5]
sus list = new_list(10)  # Inferred as List[normie]
list.add(42)
```

### Partial Type Inference
```cursed
# Some types specified, others inferred
sus result = pair[normie]("hello", 42)  # Second type inferred
```

## Generic Constraints in Practice

### Comparable Types
```cursed
interface Ordered[T] {
    less_than(other T) lit
    equal_to(other T) lit
}

slay sort[T: Ordered[T]](items []T) []T {
    # Bubble sort implementation
    sus sorted []T = make([]T, len(items))
    copy(sorted, items)
    
    bestie i := 0; i < len(sorted); i++ {
        bestie j := 0; j < len(sorted)-1-i; j++ {
            lowkey !sorted[j].less_than(sorted[j+1]) && !sorted[j].equal_to(sorted[j+1]) {
                sorted[j], sorted[j+1] = sorted[j+1], sorted[j]
            }
        }
    }
    
    damn sorted
}
```

### Numeric Constraints
```cursed
interface Number {
    add(other Self) Self
    subtract(other Self) Self
    multiply(other Self) Self
    divide(other Self) Self
    zero() Self
}

slay sum[T: Number](numbers []T) T {
    sus result T = T{}.zero()
    bestie num <- numbers {
        result = result.add(num)
    }
    damn result
}
```

## Performance Considerations

### Monomorphization
```cursed
# Each instantiation creates specialized code
sus int_stack Stack[normie] = Stack[normie]{}
sus str_stack Stack[tea] = Stack[tea]{}

# Compiler generates separate implementations:
# - Stack_normie_push, Stack_normie_pop
# - Stack_tea_push, Stack_tea_pop
```

### Generic Optimization
```cursed
# Inline-optimized generic function
slay fast_max[T: Comparable[T]](a T, b T) T {
    # Compiler can inline and optimize based on concrete types
    damn lowkey a.compare(b) > 0 ? a : b
}
```

## Common Generic Patterns

### Option Type
```cursed
struct Option[T] {
    value T
    has_value lit
}

slay some[T](value T) Option[T] {
    damn Option[T]{
        value: value,
        has_value: based,
    }
}

slay none[T]() Option[T] {
    sus zero T
    damn Option[T]{
        value: zero,
        has_value: cap,
    }
}

slay (opt Option[T]) is_some() lit {
    damn opt.has_value
}

slay (opt Option[T]) unwrap() T {
    lowkey !opt.has_value {
        yikes "Unwrapped None value"
    }
    damn opt.value
}
```

### Result Type
```cursed
struct Result[T, E] {
    value T
    error E
    is_ok lit
}

slay ok[T, E](value T) Result[T, E] {
    sus zero_err E
    damn Result[T, E]{
        value: value,
        error: zero_err,
        is_ok: based,
    }
}

slay err[T, E](error E) Result[T, E] {
    sus zero_val T
    damn Result[T, E]{
        value: zero_val,
        error: error,
        is_ok: cap,
    }
}
```

## Generic Testing

### Testing Generic Functions
```cursed
yeet "testz"

slay test_generic_stack() {
    test_start("Generic Stack Test")
    
    # Test with integers
    sus int_stack Stack[normie] = Stack[normie]{}
    int_stack.push(42)
    int_stack.push(24)
    
    assert_eq_int(int_stack.pop(), 24)
    assert_eq_int(int_stack.pop(), 42)
    assert_true(int_stack.is_empty())
    
    # Test with strings
    sus str_stack Stack[tea] = Stack[tea]{}
    str_stack.push("hello")
    str_stack.push("world")
    
    assert_eq_string(str_stack.pop(), "world")
    assert_eq_string(str_stack.pop(), "hello")
    assert_true(str_stack.is_empty())
    
    print_test_summary()
}
```

## Best Practices

### 1. Use Meaningful Type Parameter Names
```cursed
# Good
slay map[T, U](items []T, fn slay(T) U) []U { ... }

# Bad
slay map[A, B](items []A, fn slay(A) B) []B { ... }
```

### 2. Keep Constraints Minimal
```cursed
# Good - minimal constraint
slay print_item[T: Printable](item T) {
    vibez.spill(item.to_string())
}

# Bad - over-constrained
slay print_item[T: Printable & Serializable & Comparable[T]](item T) {
    vibez.spill(item.to_string())
}
```

### 3. Use Generic Collections Wisely
```cursed
# Good - generic collection for reusability
struct Cache[K, V] {
    data map[K]V
    max_size normie
}

# Bad - unnecessary generics
struct StringCache[T] {  # T is always tea
    data map[T]tea
}
```

### 4. Document Generic Types
```cursed
# Document type parameters and constraints
# T: The type of items stored in the container
# T must implement Comparable for sorting functionality
struct SortedContainer[T: Comparable[T]] {
    items []T
    sorted lit
}
```

## Advanced Features

### Associated Types
```cursed
interface Collection[T] {
    be_like Item = T
    be_like Iterator = Iterator[T]
    
    iter() Iterator
    add(item Item)
    size() normie
}
```

### Generic Type Aliases
```cursed
be_like StringMap[V] = map[tea]V
be_like IntList = List[normie]
be_like ResultStr[E] = Result[tea, E]
```

### Recursive Generic Types
```cursed
struct Tree[T] {
    value T
    left *Tree[T]
    right *Tree[T]
}

slay (tree *Tree[T]) insert(value T) {
    # Tree insertion logic
}
```

---

Next: [Interfaces and Traits →](02-interfaces.md)
