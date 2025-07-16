yeet "testz"

# Generic Interfaces Comprehensive Test Suite
test_start("Generic Interface Support Tests")

# Test 1: Basic Generic Interface Definition
collab Container<T> {
    slay get() -> T
    slay set(value: T)
    slay size() -> normie
}

# Test 2: Interface with Multiple Type Parameters  
collab Map<K, V> {
    slay put(key: K, value: V)
    slay get(key: K) -> V
    slay contains_key(key: K) -> lit
    slay size() -> normie
}

# Test 3: Interface with Constraints
collab Comparable<T: Clone + Display> {
    slay compare(other: T) -> normie
    slay equals(other: T) -> lit
}

# Test 4: Interface Inheritance with Generics
collab Iterator<T> {
    slay next() -> T
    slay has_next() -> lit
}

collab MutableIterator<T>: Iterator<T> {
    slay remove()
    slay insert(item: T)
}

# Test 5: Associated Types
collab Collect<T> {
    be_like Item = T
    be_like IntoIter: Iterator<Item>
    
    slay collect() -> IntoIter
    slay from_iter(iter: IntoIter) -> Collection<T>
}

# Test 6: Generic Interface Implementation
struct Vector<T> {
    data: [T]
    length: normie
    capacity: normie
}

# Implement Container<T> for Vector<T>
vibe Container<T> for Vector<T> {
    slay get() -> T {
        # Implementation would access the data
        damn self.data[0]
    }
    
    slay set(value: T) {
        # Implementation would set the value
        self.data[0] = value
    }
    
    slay size() -> normie {
        damn self.length
    }
}

# Test 7: Concrete Implementation
struct IntList {
    data: [normie]
    count: normie
}

# Implement Container<normie> for IntList
vibe Container<normie> for IntList {
    slay get() -> normie {
        damn self.data[0]
    }
    
    slay set(value: normie) {
        self.data[0] = value
    }
    
    slay size() -> normie {
        damn self.count
    }
}

# Test 8: Higher-Order Interface with Function Types
collab Functor<F> {
    slay map<A, B>(self: F<A>, func: (A) -> B) -> F<B>
}

# Test 9: Interface with Where Clauses
collab Serialize<T> where T: Clone + Send {
    slay serialize(item: T) -> tea
    slay deserialize(data: tea) -> T
}

# Test 10: Generic Interface Composition
collab ReadWrite<T>: Container<T> + Iterator<T> {
    slay read_all() -> [T]
    slay write_all(items: [T])
}

# Validation Tests
slay test_generic_interface_instantiation() -> lit {
    # Test that generic interfaces can be instantiated with concrete types
    sus vec Vector<normie>
    sus result lit = vec.size() == 0
    damn result
}

slay test_interface_inheritance() -> lit {
    # Test that interface inheritance works with generics
    sus iter MutableIterator<normie>
    sus can_iterate lit = iter.has_next()
    sus can_mutate lit = based  # iter.remove() would be called
    damn can_iterate && can_mutate
}

slay test_constraint_validation() -> lit {
    # Test that generic constraints are properly validated
    sus comp Comparable<normie>  # normie should satisfy Clone + Display
    sus comparison_result normie = comp.compare(42)
    damn comparison_result >= 0
}

slay test_associated_types() -> lit {
    # Test that associated types work correctly
    sus collector Collect<normie>
    sus iter = collector.collect()
    sus has_items lit = iter.has_next()
    damn has_items
}

slay test_concrete_implementation() -> lit {
    # Test concrete implementation of generic interface
    sus int_list IntList
    int_list.data = [1, 2, 3]
    int_list.count = 3
    
    sus container Container<normie> = int_list
    sus size normie = container.size()
    damn size == 3
}

# Run all tests
assert_true(test_generic_interface_instantiation())
assert_true(test_interface_inheritance())  
assert_true(test_constraint_validation())
assert_true(test_associated_types())
assert_true(test_concrete_implementation())

print_test_summary()
