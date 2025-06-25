//! Generic Collections Examples for CURSED Language
//! Demonstrates advanced generic programming with type-safe collections

// Generic List collection with comprehensive methods
collab List<T> {
    sus data: Array<T>
    sus size: Integer
    sus capacity: Integer

    // Constructor
    stan new() -> List<T> {
        periodt List<T> {
            data: Array<T>::with_capacity(10),
            size: 0,
            capacity: 10
        }
    }

    // Add element (covariant in T)
    slay push(mut sus self, item: T) {
        lowkey (self.size >= self.capacity) {
            self.resize()
        }
        self.data[self.size] = item
        self.size += 1
    }

    // Remove and return last element
    slay pop(mut sus self) -> Option<T> {
        lowkey (self.size == 0) {
            periodt None
        } highkey {
            self.size -= 1
            periodt Some(self.data[self.size])
        }
    }

    // Get element by index
    slay get(sus self, index: Integer) -> Option<T> {
        lowkey (index >= 0 && index < self.size) {
            periodt Some(self.data[index])
        } highkey {
            periodt None
        }
    }

    // Iterate over elements
    slay iter(sus self) -> ListIterator<T> {
        periodt ListIterator<T>::new(sus self)
    }

    // Map function over elements (higher-order generics)
    slay map<U>(sus self, f: fn(T) -> U) -> List<U> {
        sus result = List<U>::new()
        bestie sus i = 0; i < self.size; i++ {
            result.push(f(self.data[i]))
        }
        periodt result
    }

    // Filter elements based on predicate
    slay filter(sus self, predicate: fn(T) -> Boolean) -> List<T> {
        sus result = List<T>::new()
        bestie sus i = 0; i < self.size; i++ {
            lowkey (predicate(self.data[i])) {
                result.push(self.data[i])
            }
        }
        periodt result
    }

    // Fold/reduce operation
    slay fold<U>(sus self, initial: U, f: fn(U, T) -> U) -> U {
        sus acc = initial
        bestie sus i = 0; i < self.size; i++ {
            acc = f(acc, self.data[i])
        }
        periodt acc
    }

    // Private resize method
    slay resize(mut sus self) {
        sus new_capacity = self.capacity * 2
        sus new_data = Array<T>::with_capacity(new_capacity)
        bestie sus i = 0; i < self.size; i++ {
            new_data[i] = self.data[i]
        }
        self.data = new_data
        self.capacity = new_capacity
    }
}

// Generic Iterator for List
collab ListIterator<T> {
    sus list: sus List<T>
    sus index: Integer

    stan new(list: sus List<T>) -> ListIterator<T> {
        periodt ListIterator<T> {
            list: list,
            index: 0
        }
    }
}

// Iterator interface with associated types
collab Iterator<T> {
    type Item = T  // Associated type

    slay next(mut sus self) -> Option<Self::Item>
    slay has_next(sus self) -> Boolean
}

// Implement Iterator for ListIterator
impl<T> Iterator<T> bestie ListIterator<T> {
    type Item = T

    slay next(mut sus self) -> Option<T> {
        lowkey (self.index < self.list.size) {
            sus item = self.list.data[self.index]
            self.index += 1
            periodt Some(item)
        } highkey {
            periodt None
        }
    }

    slay has_next(sus self) -> Boolean {
        periodt self.index < self.list.size
    }
}

// Generic Map collection with key-value pairs
collab Map<K, V> where K: Hash + Eq {
    sus buckets: Array<List<Pair<K, V>>>
    sus size: Integer
    sus bucket_count: Integer

    stan new() -> Map<K, V> {
        sus bucket_count = 16
        sus buckets = Array<List<Pair<K, V>>>::with_capacity(bucket_count)
        bestie sus i = 0; i < bucket_count; i++ {
            buckets[i] = List<Pair<K, V>>::new()
        }
        periodt Map<K, V> {
            buckets: buckets,
            size: 0,
            bucket_count: bucket_count
        }
    }

    // Insert key-value pair
    slay insert(mut sus self, key: K, value: V) {
        sus bucket_index = self.hash_key(sus key) % self.bucket_count
        sus bucket = mut sus self.buckets[bucket_index]

        // Check if key already exists
        bestie mut sus i = 0; i < bucket.size; i++ {
            lowkey (bucket.data[i].key == key) {
                bucket.data[i].value = value
                periodt
            }
        }

        // Key doesn't exist, add new pair
        bucket.push(Pair<K, V>::new(key, value))
        self.size += 1
    }

    // Get value by key
    slay get(sus self, key: sus K) -> Option<V> {
        sus bucket_index = self.hash_key(key) % self.bucket_count
        sus bucket = sus self.buckets[bucket_index]

        bestie sus i = 0; i < bucket.size; i++ {
            lowkey (bucket.data[i].key == *key) {
                periodt Some(bucket.data[i].value)
            }
        }
        periodt None
    }

    // Remove key-value pair
    slay remove(mut sus self, key: sus K) -> Option<V> {
        sus bucket_index = self.hash_key(key) % self.bucket_count
        sus bucket = mut sus self.buckets[bucket_index]

        bestie mut sus i = 0; i < bucket.size; i++ {
            lowkey (bucket.data[i].key == *key) {
                sus value = bucket.data[i].value
                // Remove by shifting elements
                bestie sus j = i; j < bucket.size - 1; j++ {
                    bucket.data[j] = bucket.data[j + 1]
                }
                bucket.size -= 1
                self.size -= 1
                periodt Some(value)
            }
        }
        periodt None
    }

    // Check if key exists
    slay contains_key(sus self, key: sus K) -> Boolean {
        periodt self.get(key).is_some()
    }

    // Get all keys
    slay keys(sus self) -> List<K> {
        sus result = List<K>::new()
        bestie sus i = 0; i < self.bucket_count; i++ {
            sus bucket = sus self.buckets[i]
            bestie sus j = 0; j < bucket.size; j++ {
                result.push(bucket.data[j].key)
            }
        }
        periodt result
    }

    // Private hash function
    slay hash_key(sus self, key: sus K) -> Integer {
        // Simple hash function (would use trait in real implementation)
        periodt key.hash() as Integer
    }
}

// Key-value pair structure
collab Pair<K, V> {
    sus key: K
    sus value: V

    stan new(key: K, value: V) -> Pair<K, V> {
        periodt Pair<K, V> { key: key, value: value }
    }
}

// Generic Set collection
collab Set<T> where T: Hash + Eq {
    sus map: Map<T, Boolean>

    stan new() -> Set<T> {
        periodt Set<T> {
            map: Map<T, Boolean>::new()
        }
    }

    // Add element to set
    slay insert(mut sus self, item: T) {
        self.map.insert(item, true)
    }

    // Check if element exists
    slay contains(sus self, item: sus T) -> Boolean {
        periodt self.map.contains_key(item)
    }

    // Remove element from set
    slay remove(mut sus self, item: sus T) -> Boolean {
        periodt self.map.remove(item).is_some()
    }

    // Get all elements
    slay elements(sus self) -> List<T> {
        periodt self.map.keys()
    }

    // Set union (higher-order operation)
    slay union(sus self, other: sus Set<T>) -> Set<T> {
        sus result = Set<T>::new()
        sus self_elements = self.elements()
        sus other_elements = other.elements()

        bestie sus i = 0; i < self_elements.size; i++ {
            result.insert(self_elements.data[i])
        }
        bestie sus i = 0; i < other_elements.size; i++ {
            result.insert(other_elements.data[i])
        }
        periodt result
    }

    // Set intersection
    slay intersection(sus self, other: sus Set<T>) -> Set<T> {
        sus result = Set<T>::new()
        sus self_elements = self.elements()

        bestie sus i = 0; i < self_elements.size; i++ {
            lowkey (other.contains(sus self_elements.data[i])) {
                result.insert(self_elements.data[i])
            }
        }
        periodt result
    }
}

// Generic Option type with monadic operations
collab Option<T> {
    sus value: T?
    sus has_value: Boolean

    stan Some(value: T) -> Option<T> {
        periodt Option<T> {
            value: value,
            has_value: true
        }
    }

    stan None() -> Option<T> {
        periodt Option<T> {
            value: nil,
            has_value: false
        }
    }

    // Check if has value
    slay is_some(sus self) -> Boolean {
        periodt self.has_value
    }

    slay is_none(sus self) -> Boolean {
        periodt !self.has_value
    }

    // Unwrap value (unsafe)
    slay unwrap(sus self) -> T {
        lowkey (self.has_value) {
            periodt self.value
        } highkey {
            panic("Called unwrap on None value")
        }
    }

    // Unwrap with default
    slay unwrap_or(sus self, default: T) -> T {
        lowkey (self.has_value) {
            periodt self.value
        } highkey {
            periodt default
        }
    }

    // Map operation (Functor)
    slay map<U>(sus self, f: fn(T) -> U) -> Option<U> {
        lowkey (self.has_value) {
            periodt Option<U>::Some(f(self.value))
        } highkey {
            periodt Option<U>::None()
        }
    }

    // Bind operation (Monad)
    slay bind<U>(sus self, f: fn(T) -> Option<U>) -> Option<U> {
        lowkey (self.has_value) {
            periodt f(self.value)
        } highkey {
            periodt Option<U>::None()
        }
    }

    // Filter operation
    slay filter(sus self, predicate: fn(T) -> Boolean) -> Option<T> {
        lowkey (self.has_value && predicate(self.value)) {
            periodt Option<T>::Some(self.value)
        } highkey {
            periodt Option<T>::None()
        }
    }
}

// Generic Result type for error handling
collab Result<T, E> {
    sus value: T?
    sus error: E?
    sus is_success: Boolean

    stan Ok(value: T) -> Result<T, E> {
        periodt Result<T, E> {
            value: value,
            error: nil,
            is_success: true
        }
    }

    stan Err(error: E) -> Result<T, E> {
        periodt Result<T, E> {
            value: nil,
            error: error,
            is_success: false
        }
    }

    // Check if successful
    slay is_ok(sus self) -> Boolean {
        periodt self.is_success
    }

    slay is_err(sus self) -> Boolean {
        periodt !self.is_success
    }

    // Unwrap value
    slay unwrap(sus self) -> T {
        lowkey (self.is_success) {
            periodt self.value
        } highkey {
            panic("Called unwrap on Err result")
        }
    }

    // Map over success value
    slay map<U>(sus self, f: fn(T) -> U) -> Result<U, E> {
        lowkey (self.is_success) {
            periodt Result<U, E>::Ok(f(self.value))
        } highkey {
            periodt Result<U, E>::Err(self.error)
        }
    }

    // Map over error value
    slay map_err<F>(sus self, f: fn(E) -> F) -> Result<T, F> {
        lowkey (self.is_success) {
            periodt Result<T, F>::Ok(self.value)
        } highkey {
            periodt Result<T, F>::Err(f(self.error))
        }
    }

    // Bind operation
    slay bind<U>(sus self, f: fn(T) -> Result<U, E>) -> Result<U, E> {
        lowkey (self.is_success) {
            periodt f(self.value)
        } highkey {
            periodt Result<U, E>::Err(self.error)
        }
    }
}

// Example usage function
slay demonstrate_collections() {
    // Create a list of integers
    sus numbers = List<Integer>::new()
    numbers.push(1)
    numbers.push(2)
    numbers.push(3)
    numbers.push(4)
    numbers.push(5)

    // Map to double each number
    sus doubled = numbers.map(|x| x * 2)

    // Filter even numbers
    sus evens = doubled.filter(|x| x % 2 == 0)

    // Fold to sum
    sus sum = evens.fold(0, |acc, x| acc + x)

    println("Sum of doubled numbers: {}", sum)

    // Create a map
    sus ages = Map<String, Integer>::new()
    ages.insert("Alice", 25)
    ages.insert("Bob", 30)
    ages.insert("Charlie", 35)

    // Get age
    vibe_check (ages.get("Alice")) {
        mood Some(age) => println("Alice is {} years old", age),
        mood None => println("Alice not found")
    }

    // Create a set
    sus fruits = Set<String>::new()
    fruits.insert("apple")
    fruits.insert("banana")
    fruits.insert("cherry")

    sus other_fruits = Set<String>::new()
    other_fruits.insert("banana")
    other_fruits.insert("date")

    sus all_fruits = fruits.union(sus other_fruits)
    println("All fruits: {:?}", all_fruits.elements())

    // Work with Option
    sus maybe_number = Option<Integer>::Some(42)
    sus doubled_maybe = maybe_number.map(|x| x * 2)
    
    vibe_check (doubled_maybe) {
        mood Some(value) => println("Doubled value: {}", value),
        mood None => println("No value")
    }

    // Work with Result
    sus result = divide(10, 2)
    vibe_check (result) {
        mood Ok(value) => println("Division result: {}", value),
        mood Err(error) => println("Division error: {}", error)
    }
}

// Helper function that returns Result
slay divide(a: Integer, b: Integer) -> Result<Integer, String> {
    lowkey (b == 0) {
        periodt Result<Integer, String>::Err("Division by zero")
    } highkey {
        periodt Result<Integer, String>::Ok(a / b)
    }
}

// Main function to run examples
slay main() {
    demonstrate_collections()
}
