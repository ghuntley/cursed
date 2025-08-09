// Integration test for runtime generic type system with CURSED compiler

yeet "testz"
yeet "mathz"
yeet "stringz"

// Real-world generic examples that should work with the runtime system

// Generic Optional type with monomorphization
squad Option<T> {
    has_value lit
    value T
    
    slay some(val T) Option<T> {
        damn Option<T> { has_value: based, value: val }
    }
    
    slay none() Option<T> {
        damn Option<T> { has_value: cringe, value: default_value<T>() }
    }
    
    slay is_some(self) lit {
        damn self.has_value
    }
    
    slay is_none(self) lit {
        damn !self.has_value
    }
    
    slay unwrap(self) T {
        ready (self.has_value) {
            damn self.value
        } otherwise {
            panic("Attempted to unwrap None value")
        }
    }
    
    slay unwrap_or(self, default T) T {
        ready (self.has_value) {
            damn self.value
        } otherwise {
            damn default
        }
    }
}

// Generic Result type for error handling
squad Result<T, E> {
    is_ok lit
    ok_value T
    err_value E
    
    slay ok(val T) Result<T, E> {
        damn Result<T, E> { 
            is_ok: based, 
            ok_value: val, 
            err_value: default_value<E>() 
        }
    }
    
    slay err(error E) Result<T, E> {
        damn Result<T, E> { 
            is_ok: cringe, 
            ok_value: default_value<T>(), 
            err_value: error 
        }
    }
    
    slay is_ok(self) lit {
        damn self.is_ok
    }
    
    slay is_err(self) lit {
        damn !self.is_ok
    }
    
    slay unwrap(self) T {
        ready (self.is_ok) {
            damn self.ok_value
        } otherwise {
            panic("Attempted to unwrap error result")
        }
    }
}

// Generic Vector with dynamic allocation
squad Vec<T> {
    items []T
    capacity drip
    length drip
    
    slay new() Vec<T> {
        damn Vec<T> {
            items: allocate_array<T>(4),
            capacity: 4,
            length: 0
        }
    }
    
    slay push(self, item T) vibes {
        ready (self.length >= self.capacity) {
            self.resize(self.capacity * 2)
        }
        self.items[self.length] = item
        self.length = self.length + 1
    }
    
    slay pop(self) Option<T> {
        ready (self.length > 0) {
            self.length = self.length - 1
            damn Option<T>.some(self.items[self.length])
        } otherwise {
            damn Option<T>.none()
        }
    }
    
    slay get(self, index drip) Option<T> {
        ready (index < self.length) {
            damn Option<T>.some(self.items[index])
        } otherwise {
            damn Option<T>.none()
        }
    }
    
    slay len(self) drip {
        damn self.length
    }
    
    slay resize(self, new_capacity drip) vibes {
        sus new_items []T = allocate_array<T>(new_capacity)
        sus i drip = 0
        bestie (i < self.length) {
            new_items[i] = self.items[i]
            i = i + 1
        }
        deallocate_array(self.items)
        self.items = new_items
        self.capacity = new_capacity
    }
}

// Generic HashMap with key-value pairs
squad HashMap<K: Hashable + Eq, V> {
    buckets [][]Pair<K, V>
    bucket_count drip
    size drip
    
    slay new() HashMap<K, V> {
        sus bucket_count drip = 16
        sus buckets [][]Pair<K, V> = allocate_array<[]Pair<K, V>>(bucket_count)
        sus i drip = 0
        bestie (i < bucket_count) {
            buckets[i] = allocate_array<Pair<K, V>>(0)
            i = i + 1
        }
        
        damn HashMap<K, V> {
            buckets: buckets,
            bucket_count: bucket_count,
            size: 0
        }
    }
    
    slay insert(self, key K, value V) vibes {
        sus hash_val drip = hash(key)
        sus bucket_index drip = hash_val % self.bucket_count
        sus bucket []Pair<K, V> = self.buckets[bucket_index]
        
        // Check if key already exists
        sus i drip = 0
        bestie (i < len(bucket)) {
            ready (bucket[i].key == key) {
                bucket[i].value = value
                damn
            }
            i = i + 1
        }
        
        // Add new pair
        sus new_pair Pair<K, V> = Pair<K, V> { key: key, value: value }
        append_to_array(bucket, new_pair)
        self.size = self.size + 1
    }
    
    slay get(self, key K) Option<V> {
        sus hash_val drip = hash(key)
        sus bucket_index drip = hash_val % self.bucket_count
        sus bucket []Pair<K, V> = self.buckets[bucket_index]
        
        sus i drip = 0
        bestie (i < len(bucket)) {
            ready (bucket[i].key == key) {
                damn Option<V>.some(bucket[i].value)
            }
            i = i + 1
        }
        
        damn Option<V>.none()
    }
    
    slay len(self) drip {
        damn self.size
    }
}

squad Pair<K, V> {
    key K
    value V
}

// Test runtime compilation and type checking
slay test_runtime_compilation() vibes {
    test_start("Runtime Generic Compilation")
    
    // Test Option<drip> compilation and usage
    sus some_value Option<drip> = Option<drip>.some(42)
    assert_true(some_value.is_some())
    assert_eq_int(some_value.unwrap(), 42)
    
    sus none_value Option<drip> = Option<drip>.none()
    assert_true(none_value.is_none())
    assert_eq_int(none_value.unwrap_or(99), 99)
    
    // Test Option<tea> with different type
    sus some_string Option<tea> = Option<tea>.some("hello")
    assert_true(some_string.is_some())
    assert_eq_string(some_string.unwrap(), "hello")
    
    // Test Result<drip, tea> compilation
    sus ok_result Result<drip, tea> = Result<drip, tea>.ok(123)
    assert_true(ok_result.is_ok())
    assert_eq_int(ok_result.unwrap(), 123)
    
    sus err_result Result<drip, tea> = Result<drip, tea>.err("error occurred")
    assert_true(err_result.is_err())
    
    print_test_summary()
}

// Test dynamic collections
slay test_dynamic_collections() vibes {
    test_start("Dynamic Generic Collections")
    
    // Test Vec<drip>
    sus int_vec Vec<drip> = Vec<drip>.new()
    int_vec.push(1)
    int_vec.push(2)
    int_vec.push(3)
    
    assert_eq_int(int_vec.len(), 3)
    
    sus first Option<drip> = int_vec.get(0)
    assert_true(first.is_some())
    assert_eq_int(first.unwrap(), 1)
    
    sus popped Option<drip> = int_vec.pop()
    assert_true(popped.is_some())
    assert_eq_int(popped.unwrap(), 3)
    assert_eq_int(int_vec.len(), 2)
    
    // Test Vec<tea>
    sus string_vec Vec<tea> = Vec<tea>.new()
    string_vec.push("first")
    string_vec.push("second")
    
    assert_eq_int(string_vec.len(), 2)
    
    sus first_str Option<tea> = string_vec.get(0)
    assert_eq_string(first_str.unwrap(), "first")
    
    print_test_summary()
}

// Test HashMap with constraints
slay test_constrained_generics() vibes {
    test_start("Constrained Generic Types")
    
    // Test HashMap<tea, drip> where tea satisfies Hashable + Eq
    sus map HashMap<tea, drip> = HashMap<tea, drip>.new()
    map.insert("one", 1)
    map.insert("two", 2)
    map.insert("three", 3)
    
    assert_eq_int(map.len(), 3)
    
    sus value Option<drip> = map.get("two")
    assert_true(value.is_some())
    assert_eq_int(value.unwrap(), 2)
    
    sus missing Option<drip> = map.get("four")
    assert_true(missing.is_none())
    
    print_test_summary()
}

// Test monomorphization caching
slay test_monomorphization_caching() vibes {
    test_start("Monomorphization Caching")
    
    // Create multiple instances of the same generic type
    sus opt1 Option<drip> = Option<drip>.some(1)
    sus opt2 Option<drip> = Option<drip>.some(2)
    sus opt3 Option<drip> = Option<drip>.some(3)
    
    // These should all use the same monomorphized Option<drip> code
    assert_eq_int(opt1.unwrap(), 1)
    assert_eq_int(opt2.unwrap(), 2)
    assert_eq_int(opt3.unwrap(), 3)
    
    // Different type should create new monomorphized version
    sus str_opt Option<tea> = Option<tea>.some("cached")
    assert_eq_string(str_opt.unwrap(), "cached")
    
    print_test_summary()
}

// Test complex nested generics
slay test_nested_generics() vibes {
    test_start("Nested Generic Types")
    
    // Test nested generics: Vec<Option<drip>>
    sus vec_of_options Vec<Option<drip>> = Vec<Option<drip>>.new()
    vec_of_options.push(Option<drip>.some(1))
    vec_of_options.push(Option<drip>.none())
    vec_of_options.push(Option<drip>.some(3))
    
    assert_eq_int(vec_of_options.len(), 3)
    
    sus first_option Option<Option<drip>> = vec_of_options.get(0)
    assert_true(first_option.is_some())
    assert_true(first_option.unwrap().is_some())
    assert_eq_int(first_option.unwrap().unwrap(), 1)
    
    sus second_option Option<Option<drip>> = vec_of_options.get(1)
    assert_true(second_option.is_some())
    assert_true(second_option.unwrap().is_none())
    
    // Test HashMap<tea, Vec<drip>>
    sus map_of_vecs HashMap<tea, Vec<drip>> = HashMap<tea, Vec<drip>>.new()
    
    sus numbers Vec<drip> = Vec<drip>.new()
    numbers.push(1)
    numbers.push(2)
    numbers.push(3)
    
    map_of_vecs.insert("numbers", numbers)
    
    sus retrieved_vec Option<Vec<drip>> = map_of_vecs.get("numbers")
    assert_true(retrieved_vec.is_some())
    assert_eq_int(retrieved_vec.unwrap().len(), 3)
    
    print_test_summary()
}

// Main test runner
slay main() drip {
    vibez.spill("Starting Runtime Generic Integration Tests...")
    
    test_runtime_compilation()
    test_dynamic_collections()
    test_constrained_generics()
    test_monomorphization_caching()
    test_nested_generics()
    
    vibez.spill("All runtime generic integration tests completed!")
    damn 0
}

// Helper functions (placeholders for actual implementations)
slay default_value<T>() T {
    // Return type-appropriate default value
    // This would be implemented by the runtime system
    panic("default_value not implemented for this type")
}

slay allocate_array<T>(size drip) []T {
    // Allocate array of generic type T
    // This would be implemented by the runtime memory management
    panic("allocate_array not implemented")
}

slay deallocate_array<T>(arr []T) vibes {
    // Deallocate array of generic type T
    // This would be implemented by the runtime memory management
}

slay append_to_array<T>(arr []T, item T) vibes {
    // Append item to dynamic array
    // This would be implemented by the runtime array system
}

slay hash<T: Hashable>(value T) drip {
    // Hash function for generic hashable type
    // This would be implemented by the trait system
    damn 0
}

slay panic(message tea) vibes {
    vibez.spill("PANIC: " + message)
    // Exit or throw exception
}
