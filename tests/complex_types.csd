//! Complex types and generics documentation examples
//! 
//! Demonstrates comprehensive documentation for advanced CURSED language features
//! including generic types, constraints, and complex nested structures.

/// Generic container with type constraints
/// 
/// A flexible container that can hold any type implementing the Serializable interface.
/// Provides type-safe operations with compile-time generic type checking.
/// 
/// # Type Parameters
/// * `T` - The contained type, must implement Serializable
/// 
/// # Examples
/// 
/// ```cursed
/// facts string_container = new Container[String]()
/// string_container.add("hello")
/// facts value = string_container.get(0)
/// ```
squad Container[T: Serializable] {
    /// Internal storage for container items
    items: T[],
    /// Maximum capacity of the container
    capacity: Int,
    /// Current number of items stored
    size: Int,
}

/// Interface for serializable types
/// 
/// Types implementing this interface can be stored in containers
/// and transmitted over network protocols.
collab Serializable {
    /// Serialize object to byte array
    /// 
    /// # Returns
    /// Byte representation of the object
    damn to_bytes(self) -> Byte[]
    
    /// Deserialize object from byte array
    /// 
    /// # Arguments
    /// * `data` - Byte array containing serialized object data
    /// 
    /// # Returns
    /// Reconstructed object instance
    damn from_bytes(data: Byte[]) -> Self
}

/// Create a new empty container with specified capacity
/// 
/// # Type Parameters
/// * `T` - Type of items to store (must be Serializable)
/// 
/// # Arguments
/// * `capacity` - Maximum number of items the container can hold
/// 
/// # Returns
/// New empty container instance
/// 
/// # Examples
/// 
/// ```cursed
/// facts container = new Container[User](100)
/// ```
damn new Container[T: Serializable](capacity: Int) -> Container[T] {
    Container {
        items: [],
        capacity: capacity,
        size: 0,
    }
}

/// Add item to the container
/// 
/// # Arguments
/// * `item` - Item to add to the container
/// 
/// # Returns
/// True if item was added successfully, false if container is full
/// 
/// # Examples
/// 
/// ```cursed
/// facts success = container.add(user)
/// lowkey !success {
///     vibe_panic("Container is full!")
/// }
/// ```
damn slay add[T: Serializable](self: Container[T], item: T) -> Bool {
    lowkey self.size >= self.capacity {
        false
    } bestie {
        self.items.push(item)
        self.size += 1
        true
    }
}

/// Get item at specified index
/// 
/// # Arguments
/// * `index` - Zero-based index of item to retrieve
/// 
/// # Returns
/// Item at the specified index, or nil if index is out of bounds
/// 
/// # Examples
/// 
/// ```cursed
/// facts maybe_item = container.get(0)
/// lowkey maybe_item != nil {
///     facts item = maybe_item!
///     // Use item...
/// }
/// ```
damn slay get[T: Serializable](self: Container[T], index: Int) -> T? {
    lowkey index >= 0 && index < self.size {
        self.items[index]
    } bestie {
        nil
    }
}

/// Complex nested type with multiple generic parameters
/// 
/// Demonstrates advanced generic type usage with multiple constraints
/// and nested generic containers.
/// 
/// # Type Parameters
/// * `K` - Key type, must be comparable and hashable
/// * `V` - Value type, must be serializable
/// * `S` - Storage type, must implement the Storage interface
squad ComplexMap[K: Comparable + Hashable, V: Serializable, S: Storage[K, V]] {
    /// Primary storage backend
    storage: S,
    /// Metadata about stored items
    metadata: Map[K, ItemMetadata],
    /// Cache for frequently accessed items
    cache: Container[CacheEntry[K, V]],
}

/// Metadata for stored items
/// 
/// Tracks access patterns and storage statistics for optimization.
squad ItemMetadata {
    /// Number of times item has been accessed
    access_count: Int,
    /// Timestamp of last access
    last_accessed: Int,
    /// Size of stored item in bytes
    size_bytes: Int,
}

/// Cache entry combining key-value pair with metadata
/// 
/// Used internally by ComplexMap for efficient caching.
squad CacheEntry[K, V] {
    /// The cached key
    key: K,
    /// The cached value
    value: V,
    /// Entry priority for eviction algorithms
    priority: Int,
}

/// Storage interface for complex maps
/// 
/// Defines operations that storage backends must implement.
collab Storage[K, V] {
    /// Store key-value pair
    /// 
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The value to associate with the key
    /// 
    /// # Returns
    /// True if storage succeeded, false otherwise
    damn put(self, key: K, value: V) -> Bool
    
    /// Retrieve value by key
    /// 
    /// # Arguments
    /// * `key` - The key to look up
    /// 
    /// # Returns
    /// The associated value, or nil if key not found
    damn get(self, key: K) -> V?
    
    /// Remove key-value pair
    /// 
    /// # Arguments
    /// * `key` - The key to remove
    /// 
    /// # Returns
    /// True if key was found and removed, false otherwise
    damn remove(self, key: K) -> Bool
}

/// Type constraint interfaces for generic bounds
/// 
/// Basic constraint interface for comparable types.
collab Comparable {
    /// Compare this object with another
    /// 
    /// # Arguments
    /// * `other` - Object to compare with
    /// 
    /// # Returns
    /// Negative if less than, zero if equal, positive if greater than
    damn compare_to(self, other: Self) -> Int
}

/// Hash computation interface for map keys
/// 
/// Types implementing this interface can be used as map keys.
collab Hashable {
    /// Compute hash code for this object
    /// 
    /// # Returns
    /// Hash code as integer
    damn hash_code(self) -> Int
}
