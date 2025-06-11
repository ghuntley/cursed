# CURSED Collections Iterator System

This document provides comprehensive documentation for the CURSED collections iterator system, which enables functional programming patterns and efficient data processing.

## Overview

The CURSED iterator system provides:
- **Lazy Evaluation**: Operations are only executed when needed
- **Functional Programming**: Map, filter, fold, reduce operations
- **Collection Integration**: Works seamlessly with sets, queues, and stacks
- **Performance**: Efficient memory usage and processing
- **Simplicity**: Easy-to-use API avoiding complex type constraints

## Core Traits

### SimpleIterator<T>

The main trait for iteration with fundamental methods:

```rust
pub trait SimpleIterator<T> {
    fn next(&mut self) -> Option<T>;
    fn count(self) -> usize;
    fn collect(self) -> Vec<T>;
    fn find<P>(self, predicate: P) -> Option<T>;
    fn any<P>(self, predicate: P) -> bool;
    fn all<P>(self, predicate: P) -> bool;
    // ... more methods
}
```

### SimpleIntoIterator<T>

Trait for converting collections into iterators:

```rust
pub trait SimpleIntoIterator<T> {
    type Iterator: SimpleIterator<T>;
    fn into_iter(self) -> Self::Iterator;
}
```

## Iterator Types

### VecIterator<T>
Iterator for vector-like collections:
```cursed
let items = vec![1, 2, 3, 4, 5];
let iter = VecIterator::new(items);
let result: Vec<i32> = iter.collect();
```

### RangeIterator
Iterator for numeric ranges:
```cursed
let range = simple_range(0, 10);  // 0..10 (exclusive)
let result: Vec<i32> = range.collect(); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

let stepped = simple_range_step(0, 10, 2);  // 0, 2, 4, 6, 8
```

## Iterator Adapters

### Map - Transform Elements
Transform each element using a function:
```cursed
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = VecIterator::new(numbers)
    .map(|x| x * 2)
    .collect();
// Result: [2, 4, 6, 8, 10]
```

### Filter - Select Elements
Filter elements based on a predicate:
```cursed
let numbers = simple_range(1, 11);
let evens: Vec<i32> = numbers
    .filter(|&x| x % 2 == 0)
    .collect();
// Result: [2, 4, 6, 8, 10]
```

### Take - Limit Elements
Take only the first n elements:
```cursed
let numbers = simple_range(1, 100);
let first_five: Vec<i32> = numbers
    .take(5)
    .collect();
// Result: [1, 2, 3, 4, 5]
```

### Skip - Skip Elements
Skip the first n elements:
```cursed
let numbers = simple_range(1, 11);
let after_three: Vec<i32> = numbers
    .skip(3)
    .collect();
// Result: [4, 5, 6, 7, 8, 9, 10]
```

## Aggregation Operations

### Fold - Accumulate with Initial Value
```cursed
let numbers = simple_range(1, 6);
let sum = numbers.fold(0, |acc, x| acc + x);
// Result: 15 (1+2+3+4+5)
```

### Reduce - Combine Elements
```cursed
let numbers = simple_range(1, 6);
let product = numbers.reduce(|acc, x| acc * x);
// Result: Some(120) (1*2*3*4*5)
```

## Utility Functions

### SimpleIteratorUtils
Collection of utility functions for common operations:

```cursed
import "stdlib::collections";

let numbers = simple_range(1, 6);

// Sum all elements
let sum = SimpleIteratorUtils::sum(numbers.clone());  // 15

// Find minimum
let min = SimpleIteratorUtils::min(numbers.clone());  // Some(1)

// Find maximum  
let max = SimpleIteratorUtils::max(numbers);          // Some(5)

// Partition into two groups
let all_numbers = simple_range(1, 11);
let (evens, odds) = SimpleIteratorUtils::partition(
    all_numbers,
    |&x| x % 2 == 0
);
// evens: [2, 4, 6, 8, 10]
// odds:  [1, 3, 5, 7, 9]
```

## Collection Integration

### HashSet
```cursed
sus mut set = HashSet::new();
set.insert(3);
set.insert(1);
set.insert(4);

let items: Vec<i32> = SimpleIntoIterator::into_iter(set).collect();
// Note: Order not guaranteed for HashSet
```

### TreeSet
```cursed
sus mut set = TreeSet::new();
set.insert(3);
set.insert(1);
set.insert(4);

let items: Vec<i32> = SimpleIntoIterator::into_iter(set).collect();
// Result: [1, 3, 4] (TreeSet maintains order)
```

### BitSet
```cursed
sus mut bit_set = BitSet::new(20);
bit_set.set(3).unwrap();
bit_set.set(7).unwrap();
bit_set.set(11).unwrap();

let items: Vec<usize> = SimpleIntoIterator::into_iter(bit_set).collect();
// Result: [3, 7, 11]
```

## Chaining Operations

Combine multiple operations for powerful data processing:

```cursed
let numbers = simple_range(1, 21);

let result: Vec<i32> = numbers
    .filter(|&x| x % 3 == 0)        // Multiples of 3: [3, 6, 9, 12, 15, 18]
    .map(|x| x * x)                 // Square them: [9, 36, 81, 144, 225, 324]
    .filter(|&x| x % 2 == 0)        // Even squares: [36, 144, 324]
    .take(2)                        // First 2: [36, 144]
    .collect();

// Result: [36, 144]
```

## Advanced Examples

### Data Processing Pipeline
```cursed
// Process sales data
facts sales_data = [
    ("Electronics", 599.99, 2),
    ("Books", 19.99, 5),
    ("Electronics", 299.99, 1),
    ("Clothing", 49.99, 3),
];

// Calculate total revenue by category
facts electronics_revenue = VecIterator::new(sales_data)
    .filter(|(category, _, _)| category == "Electronics")
    .map(|(_, price, qty)| price * qty)
    .fold(0.0, |acc, revenue| acc + revenue);

printf("Electronics revenue: ${:.2}\n", &[electronics_revenue])?;
```

### Text Analysis
```cursed
facts words = vec!["apple", "banana", "cherry", "apricot", "blueberry"];

// Group by first letter
sus word_groups = std::collections::HashMap::new();
facts mut iter = VecIterator::new(words);

periodt iter.next() {
    Some(word) => {
        facts first_char = word.chars().next().unwrap();
        word_groups.entry(first_char).or_insert_with(Vec::new).push(word);
    }
    None => periodt
}

// Find longest word
facts longest = VecIterator::new(words)
    .reduce(|acc, word| {
        lowkey word.len() > acc.len() { word } flex { acc }
    });
```

## Performance Characteristics

### Memory Efficiency
- **Lazy Evaluation**: Operations only execute when needed
- **Iterator Chains**: Minimal intermediate allocations
- **Collection Size**: Operations work with any collection size

### Time Complexity
- **next()**: O(1) for most iterators
- **collect()**: O(n) where n is the number of elements
- **filter()**: O(n) time, O(1) space (lazy)
- **map()**: O(n) time, O(1) space (lazy)

### Best Practices

1. **Chain Operations**: Combine multiple operations for efficiency
   ```cursed
   // Good: Chained operations
   data.filter(predicate).map(transform).take(10).collect()
   
   // Less efficient: Multiple passes
   facts filtered = data.filter(predicate).collect();
   facts mapped = VecIterator::new(filtered).map(transform).collect();
   facts taken = VecIterator::new(mapped).take(10).collect();
   ```

2. **Early Termination**: Use `take()` and `find()` to limit processing
   ```cursed
   // Process only what's needed
   large_dataset.filter(expensive_predicate).take(10).collect()
   ```

3. **Avoid Unnecessary Collections**: Use iterators directly when possible
   ```cursed
   // Good: Direct iteration
   data.any(|x| x > threshold)
   
   // Less efficient: Collect then check
   facts vec = data.collect();
   vec.iter().any(|&x| x > threshold)
   ```

## Error Handling

Most iterator operations are infallible, but some may return `Option` or `Result`:

```cursed
// find() returns Option
periodt numbers.find(|&x| x > 10) {
    Some(found) => printf("Found: {}\n", &[found])?,
    None => println("Not found")?,
}

// reduce() returns Option (empty iterator case)
periodt numbers.reduce(|a, b| a + b) {
    Some(sum) => printf("Sum: {}\n", &[sum])?,
    None => println("Empty iterator")?,
}
```

## Integration with CURSED Language Features

### Gen Z Slang Keywords
```cursed
sus mut results = Vec::new();

lowkey (sus item in iterator) {
    facts processed = item * 2;
    lowkey processed > 10 {
        results.push(processed);
    }
}
```

### Error Propagation
```cursed
sus process_data(data: Vec<i32>) -> Result<Vec<i32>, String> {
    facts result = VecIterator::new(data)
        .map(|x| x * 2)
        .filter(|&x| x > 0)
        .collect();
    
    lowkey result.is_empty() {
        Err("No valid data processed".to_string())
    } flex {
        Ok(result)
    }
}
```

## Testing

The iterator system includes comprehensive tests:

```bash
# Run iterator tests
./fix_linking.sh cargo test --test simple_iterators_test

# Run with verbose output
./fix_linking.sh cargo test --test simple_iterators_test -- --nocapture
```

## Summary

The CURSED collections iterator system provides:

- ✅ **Functional Programming**: Map, filter, fold, reduce operations
- ✅ **Collection Integration**: Works with all collection types
- ✅ **Performance**: Lazy evaluation and efficient memory usage
- ✅ **Simplicity**: Easy-to-use API avoiding complex constraints
- ✅ **Extensibility**: Easy to add new iterator types and operations
- ✅ **Testing**: Comprehensive test coverage with real-world examples

This iterator system enables powerful data processing capabilities while maintaining the simplicity and Gen Z aesthetic that makes CURSED unique.
