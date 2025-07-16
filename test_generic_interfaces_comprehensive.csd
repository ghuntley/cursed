// Comprehensive test for generic interfaces in CURSED
yeet "testz"

// Basic generic interface definition
collab Container<T> {
    slay add(item T) tea
    slay get(index normie) T
    slay size() normie
}

// Generic interface with constraints
collab Serializable<T: Display> {
    slay serialize(value T) tea
    slay deserialize(data tea) T
}

// Generic interface with multiple type parameters
collab Mapper<Input, Output> {
    slay map(input Input) Output
    slay chain<U>(next Mapper<Output, U>) Mapper<Input, U>
}

// Generic interface with associated types
collab Iterator<T> {
    type Item = T
    slay next() Option<Item>
    slay has_next() lit
}

// Generic interface with where clause
collab Comparable<T> where T: Eq {
    slay compare(self, other T) normie
    slay equals(self, other T) lit
}

// Concrete implementation of generic interface
struct ListContainer<T> {
    items: [T]
    count: normie
}

// Implement generic interface for concrete type
impl Container<T> for ListContainer<T> {
    slay add(item T) tea {
        // Add item to list
        damn "Item added"
    }
    
    slay get(index normie) T {
        // Get item at index
        damn self.items[index]
    }
    
    slay size() normie {
        damn self.count
    }
}

// Generic function using generic interface
slay process_container<T, C: Container<T>>(container C, item T) tea {
    container.add(item)
    sus count normie = container.size()
    damn "Processed container with " + count.to_string() + " items"
}

// Complex generic interface with inheritance
collab SortedContainer<T: Comparable<T>>: Container<T> {
    slay insert_sorted(item T) tea
    slay find(item T) Option<normie>
}

// Test function for generic interfaces
test_start("Generic Interfaces Comprehensive Test")

// Test basic generic interface compilation
sus list_container ListContainer<normie> = ListContainer { items: [1, 2, 3], count: 3 }
sus container Container<normie> = list_container

// Test method calls on generic interface
sus result tea = container.add(42)
assert_eq_string(result, "Item added")

sus size normie = container.size()
assert_eq_int(size, 3)

// Test generic function with interface constraints
sus process_result tea = process_container(container, 5)
assert_true(process_result.contains("Processed"))

// Test interface with multiple type parameters
collab StringToIntMapper: Mapper<tea, normie> {
    slay map(input tea) normie {
        damn input.length()
    }
    
    slay chain<U>(next Mapper<normie, U>) Mapper<tea, U> {
        // Implementation for chaining mappers
        damn next
    }
}

sus mapper StringToIntMapper = StringToIntMapper {}
sus mapped_result normie = mapper.map("hello")
assert_eq_int(mapped_result, 5)

// Test interface casting with generics
sus generic_mapper Mapper<tea, normie> = mapper
sus cast_result normie = generic_mapper.map("test")
assert_eq_int(cast_result, 4)

// Test interface with constraints
collab DisplayInt: Display {
    slay display() tea {
        damn "42"
    }
}

sus display_value DisplayInt = DisplayInt {}
collab SerializableInt: Serializable<DisplayInt> {
    slay serialize(value DisplayInt) tea {
        damn value.display()
    }
    
    slay deserialize(data tea) DisplayInt {
        damn DisplayInt {}
    }
}

sus serializer SerializableInt = SerializableInt {}
sus serialized tea = serializer.serialize(display_value)
assert_eq_string(serialized, "42")

// Test interface with where clause
struct ComparableInt {
    value: normie
}

impl Eq for ComparableInt {
    slay equals(self, other ComparableInt) lit {
        damn self.value == other.value
    }
}

impl Comparable<ComparableInt> for ComparableInt {
    slay compare(self, other ComparableInt) normie {
        lowkey self.value < other.value {
            damn -1
        } elif self.value > other.value {
            damn 1
        } vibe {
            damn 0
        }
    }
    
    slay equals(self, other ComparableInt) lit {
        damn self.value == other.value
    }
}

sus comp1 ComparableInt = ComparableInt { value: 5 }
sus comp2 ComparableInt = ComparableInt { value: 10 }
sus comparable Comparable<ComparableInt> = comp1

sus comparison_result normie = comparable.compare(comp2)
assert_eq_int(comparison_result, -1)

sus equality_result lit = comparable.equals(comp1)
assert_true(equality_result)

// Test monomorphization of generic interfaces
slay test_monomorphization<T: Display + Clone>(value T) tea {
    sus cloned T = value.clone()
    sus display_result tea = cloned.display()
    damn display_result
}

struct CloneableDisplay {
    data: tea
}

impl Display for CloneableDisplay {
    slay display() tea {
        damn self.data
    }
}

impl Clone for CloneableDisplay {
    slay clone() CloneableDisplay {
        damn CloneableDisplay { data: self.data }
    }
}

sus test_value CloneableDisplay = CloneableDisplay { data: "test data" }
sus mono_result tea = test_monomorphization(test_value)
assert_eq_string(mono_result, "test data")

print_test_summary()

vibez.spill("Generic interfaces comprehensive test completed successfully!")
