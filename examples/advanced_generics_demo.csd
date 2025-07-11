// Advanced Generics Demo for CURSED
// Demonstrates type constraints, generic interfaces, and higher-kinded types

yeet "testz"

// 1. Basic Generic Function with Constraints
slay safe_clone<T>(value T) T where T: Clone + Debug {
    vibez.spill("Cloning value:", value)
    damn value.clone()
}

// 2. Generic Interface Definition
trait Comparable<T> {
    slay compare(self, other T) normie
    slay equals(self, other T) lit {
        damn self.compare(other) == 0
    }
}

// 3. Interface Implementation
impl Comparable<normie> for normie {
    slay compare(self, other normie) normie {
        if self < other {
            damn -1
        } else if self > other {
            damn 1
        } else {
            damn 0
        }
    }
}

// 4. Generic Container with Multiple Constraints
struct Container<T> where T: Clone + Debug + Send {
    items Array<T>
    name tea
}

impl<T> Container<T> where T: Clone + Debug + Send {
    slay new(name tea) Container<T> {
        damn Container {
            items: Array::new(),
            name,
        }
    }
    
    slay add(self, item T) {
        vibez.spill("Adding item to container", self.name, ":", item)
        self.items.push(item)
    }
    
    slay get(self, index normie) Option<T> {
        if index < self.items.len() {
            damn Some(self.items[index].clone())
        } else {
            damn None
        }
    }
    
    slay size(self) normie {
        damn self.items.len()
    }
}

// 5. Higher-Kinded Type: Functor
trait Functor<F> {
    slay map<A, B>(self F<A>, f slay(A) B) F<B>
}

// 6. Functor Implementation for Option
impl Functor<Option> for Option<T> {
    slay map<A, B>(self Option<A>, f slay(A) B) Option<B> {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

// 7. Generic Interface with Associated Types
trait Iterator {
    type Item
    slay next(self) Option<Self::Item>
    slay count(self) normie
}

// 8. Iterator Implementation
struct NumberIterator {
    current normie
    max normie
}

impl NumberIterator {
    slay new(max normie) NumberIterator {
        damn NumberIterator { current: 0, max }
    }
}

impl Iterator for NumberIterator {
    type Item = normie
    
    slay next(self) Option<normie> {
        if self.current < self.max {
            sus result normie = self.current
            self.current += 1
            damn Some(result)
        } else {
            damn None
        }
    }
    
    slay count(self) normie {
        damn self.max - self.current
    }
}

// 9. Generic Function with Where Clauses
slay process_collection<T, U>(items Array<T>, processor slay(T) U) Array<U>
    where T: Clone + Debug,
          U: Clone + Debug {
    sus results Array<U> = Array::new()
    
    for item in items {
        vibez.spill("Processing item:", item)
        sus processed U = processor(item)
        results.push(processed)
    }
    
    damn results
}

// 10. Advanced Constraint Example
slay sort_and_display<T>(items Array<T>) 
    where T: Clone + Debug + PartialOrd {
    sus sorted Array<T> = items.clone()
    sorted.sort()
    
    vibez.spill("Sorted items:")
    for item in sorted {
        vibez.spill("  -", item)
    }
}

// 11. Variance Example (Covariant)
trait Producer<+T> {
    slay produce() T
}

struct NumberProducer {
    value normie
}

impl Producer<normie> for NumberProducer {
    slay produce() normie {
        damn self.value
    }
}

// 12. Associated Type with Constraints
trait Collect<T> {
    type Output: Debug
    type Error: Debug
    
    slay collect(self) Result<Self::Output, Self::Error>
}

impl Collect<normie> for Array<normie> {
    type Output = Array<normie>
    type Error = tea
    
    slay collect(self) Result<Array<normie>, tea> {
        if self.is_empty() {
            damn Err("Cannot collect empty array")
        } else {
            damn Ok(self)
        }
    }
}

// 13. Main Function - Demo All Features
slay main() {
    vibez.spill("=== Advanced Generics Demo ===")
    
    // Test basic generics with constraints
    vibez.spill("\n1. Basic Generic Function with Constraints:")
    sus number normie = 42
    sus cloned_number normie = safe_clone(number)
    vibez.spill("Original:", number, "Cloned:", cloned_number)
    
    // Test interface implementation
    vibez.spill("\n2. Generic Interface Implementation:")
    sus compare_result normie = (10).compare(5)
    vibez.spill("10 compared to 5:", compare_result)
    vibez.spill("10 equals 5:", (10).equals(5))
    vibez.spill("10 equals 10:", (10).equals(10))
    
    // Test generic container
    vibez.spill("\n3. Generic Container with Multiple Constraints:")
    sus container Container<normie> = Container::new("NumberContainer")
    container.add(1)
    container.add(2)
    container.add(3)
    vibez.spill("Container size:", container.size())
    
    match container.get(1) {
        Some(value) => vibez.spill("Item at index 1:", value),
        None => vibez.spill("No item at index 1"),
    }
    
    // Test higher-kinded types (Functor)
    vibez.spill("\n4. Higher-Kinded Types (Functor):")
    sus maybe_number Option<normie> = Some(42)
    sus maybe_doubled Option<normie> = maybe_number.map(|n| n * 2)
    
    match maybe_doubled {
        Some(value) => vibez.spill("Doubled value:", value),
        None => vibez.spill("No value to double"),
    }
    
    // Test iterator with associated types
    vibez.spill("\n5. Iterator with Associated Types:")
    sus iterator NumberIterator = NumberIterator::new(3)
    vibez.spill("Iterator count:", iterator.count())
    
    while based {
        match iterator.next() {
            Some(value) => vibez.spill("Next value:", value),
            None => {
                vibez.spill("Iterator exhausted")
                break
            },
        }
    }
    
    // Test generic function with where clauses
    vibez.spill("\n6. Generic Function with Where Clauses:")
    sus numbers Array<normie> = [1, 2, 3, 4, 5]
    sus doubled Array<normie> = process_collection(numbers, |n| n * 2)
    vibez.spill("Doubled numbers:", doubled)
    
    // Test sorting with constraints
    vibez.spill("\n7. Sorting with Advanced Constraints:")
    sus unsorted Array<normie> = [3, 1, 4, 1, 5, 9, 2, 6]
    sort_and_display(unsorted)
    
    // Test producer (variance)
    vibez.spill("\n8. Producer (Covariant):")
    sus producer NumberProducer = NumberProducer { value: 100 }
    sus produced normie = producer.produce()
    vibez.spill("Produced value:", produced)
    
    // Test collect with associated types
    vibez.spill("\n9. Collect with Associated Types:")
    sus data Array<normie> = [1, 2, 3]
    match data.collect() {
        Ok(result) => vibez.spill("Collection successful:", result),
        Err(error) => vibez.spill("Collection failed:", error),
    }
    
    vibez.spill("\n=== Demo Complete ===")
}
