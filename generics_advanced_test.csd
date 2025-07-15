# Advanced Generics Test
# Test generic interfaces, constraints, and complex type relationships

# Generic interface with associated types
collab Iterator<T> {
    type Item: Display
    
    next(self) -> Option<Self::Item>
    has_next(self) -> lit
}

# Generic interface with constraints
collab Comparable<T: Display> {
    compare(self, other: T) -> normie
}

# Generic function with where clause
slay sort<T>(items: [T]) -> [T] where T: Comparable<T> + Clone {
    # Simplified bubble sort implementation
    sus n normie = items.length()
    sus sorted_items = items.clone()
    
    bestie i := 0; i < n - 1; i++ {
        bestie j := 0; j < n - i - 1; j++ {
            lowkey sorted_items[j].compare(sorted_items[j + 1]) > 0 {
                sus temp = sorted_items[j]
                sorted_items[j] = sorted_items[j + 1]
                sorted_items[j + 1] = temp
            }
        }
    }
    
    damn sorted_items
}

# Generic struct with multiple constraints
struct SortedContainer<T: Comparable<T> + Clone + Display> {
    items: [T]
}

# Generic method implementation
slay impl<T: Comparable<T> + Clone + Display> SortedContainer<T> {
    slay new() -> SortedContainer<T> {
        damn SortedContainer { items: [] }
    }
    
    slay insert(self, item: T) {
        self.items.push(item)
        self.items = sort(self.items)
    }
    
    slay get(self, index: normie) -> Option<T> {
        lowkey index < self.items.length() {
            damn Some(self.items[index])
        } else {
            damn None
        }
    }
    
    slay display(self) -> tea {
        damn self.items.display()
    }
}

# Higher-order generic function
slay map<T, U>(items: [T], func: slay(T) -> U) -> [U] {
    sus result: [U] = []
    
    bestie item in items {
        result.push(func(item))
    }
    
    damn result
}

# Generic function with multiple bounds
slay process<T, U>(input: T, processor: slay(T) -> U) -> U 
where 
    T: Clone + Display,
    U: Display + Default
{
    vibez.spill("Processing:", input.display())
    sus result = processor(input)
    vibez.spill("Result:", result.display())
    damn result
}

# Test usage
slay main() {
    # Test sorted container
    sus container SortedContainer<normie> = SortedContainer.new()
    container.insert(42)
    container.insert(24)
    container.insert(100)
    container.insert(1)
    
    vibez.spill("Sorted container:", container.display())
    
    # Test map function
    sus numbers = [1, 2, 3, 4, 5]
    sus doubled = map(numbers, slay(x: normie) -> normie { damn x * 2 })
    vibez.spill("Doubled numbers:", doubled)
    
    # Test process function
    sus processed = process(42, slay(x: normie) -> tea { damn x.to_string() })
    vibez.spill("Processed result:", processed)
    
    # Test generic with multiple type parameters
    sus vec1 = [1, 2, 3]
    sus vec2 = ["a", "b", "c"]
    sus zipped = zip(vec1, vec2)
    vibez.spill("Zipped:", zipped)
}

# Generic function with complex constraints
slay zip<T, U>(vec1: [T], vec2: [U]) -> [(T, U)] 
where
    T: Clone,
    U: Clone
{
    sus result: [(T, U)] = []
    sus min_length = min(vec1.length(), vec2.length())
    
    bestie i := 0; i < min_length; i++ {
        result.push((vec1[i].clone(), vec2[i].clone()))
    }
    
    damn result
}
