# MoodMap Module

## Overview
MoodMap provides a collection of functions for working with maps in various "moods" (conditions/states). It's inspired by Go's maps package but with enhanced functionality and a focus on expressiveness.

## Core Functions

### Basic Map Operations
- **Clone(m map[tea]normie) map[tea]normie** - Create a copy of the map
- **Copy(dst, src map[tea]normie)** - Copy all key/value pairs from src to dst
- **Clear(m map[tea]normie)** - Remove all entries from the map
- **DeleteFunc(m map[tea]normie, f slay(tea, normie) lit)** - Delete entries where function returns true

### Mood-Based Map Functions
- **Vibe(m map[tea]normie, f slay(tea, normie) lit) map[tea]normie** - Create new map with entries matching condition
- **Moody(m map[tea]normie, f slay(tea, normie) normie) map[tea]normie** - Transform map by applying function to each value
- **MoodSwitch(m map[tea]normie, f slay(tea, normie) lit) (map[tea]normie, map[tea]normie)** - Split map into two based on condition
- **MoodyMerge(maps []map[tea]normie, resolver slay(tea, normie, normie) normie) map[tea]normie** - Merge maps with conflict resolution
- **MoodCheck(m map[tea]normie, f slay(tea, normie) lit) lit** - Test if all entries satisfy condition
- **FeelingSome(m map[tea]normie, f slay(tea, normie) lit) lit** - Test if at least one entry satisfies condition

### Conversion Functions
- **KeyVibes(m map[tea]normie) []tea** - Extract slice of keys
- **ValueVibes(m map[tea]normie) []normie** - Extract slice of values
- **EntryVibes(m map[tea]normie) []MapEntry** - Get key-value pairs as slice of entries
- **MapFromPairs(pairs []MapEntry) map[tea]normie** - Create map from key-value pairs
- **MapFromKeys(keys []tea, valueFunc slay(tea) normie) map[tea]normie** - Create map using function to generate values
- **MapFromValues(values []normie, keyFunc slay(normie) tea) map[tea]normie** - Create map using function to generate keys

## Special Map Types

### DefaultMap
- **DefaultMap(m map[tea]normie, defaultValue normie) slay(tea) normie** - Create function that returns default for missing keys

### CounterMap
- **CounterMap() CounterMapStruct** - Create map for counting occurrences
  - **Increment(key tea)** - Increment counter for key
  - **Get(key tea) normie** - Get current count for key
  - **GetAll() map[tea]normie** - Get all counts as map
  - **Reset()** - Reset all counters

### CacheMap
- **CacheMap(expiry normie) CacheMapStruct** - Create map with automatic expiry
  - **Set(key tea, value normie)** - Store value with timestamp
  - **Get(key tea) (normie, lit)** - Get value if not expired
  - **Delete(key tea)** - Remove entry
  - **Clear()** - Remove all entries
  - **Size() normie** - Get number of entries

### NestedMap
- **NestedMap() NestedMapStruct** - Create nested map with arbitrary depth
  - **Set(keys []tea, value normie)** - Set value at key path
  - **Get(keys []tea) (normie, lit)** - Get value from key path
  - **Delete(keys []tea)** - Delete value at key path
  - **HasKey(keys []tea) lit** - Check if key path exists
  - **ToMap() map[tea]interface{}** - Get top-level map

### SyncMap
- **SyncMap() SyncMapStruct** - Create thread-safe map (simplified implementation)
  - **Store(key tea, value normie)** - Store key-value pair
  - **Load(key tea) (normie, lit)** - Load value for key
  - **Delete(key tea)** - Delete key
  - **Range(fn slay(tea, normie) lit)** - Iterate over entries
  - **LoadOrStore(key tea, value normie) (normie, lit)** - Load existing or store new
  - **LoadAndDelete(key tea) (normie, lit)** - Load and delete atomically
  - **Swap(key tea, value normie) (normie, lit)** - Swap value, return old
  - **CompareAndSwap(key tea, old, new normie) lit** - Atomic compare and swap

## Usage Examples

### Basic Operations
```cursed
yeet "mood_map"

fr fr Basic map operations
sus original := make(map[tea]normie)
original["one"] = 1
original["two"] = 2
original["three"] = 3

sus copied := mood_map.Clone(original)
vibez.spill("Copied map size:", len(copied))

fr fr Clear all entries
mood_map.Clear(copied)
vibez.spill("After clear:", len(copied)) fr fr 0
```

### Mood-Based Operations
```cursed
fr fr Filter map entries (Vibe)
sus evenOnly := mood_map.Vibe(original, slay(k tea, v normie) lit {
    damn v % 2 == 0
})
vibez.spill("Even values:", evenOnly) fr fr map[two:2]

fr fr Transform values (Moody)
sus squared := mood_map.Moody(original, slay(k tea, v normie) normie {
    damn v * v
})
vibez.spill("Squared:", squared) fr fr map[one:1 two:4 three:9]

fr fr Split by condition (MoodSwitch)
sus even, odd := mood_map.MoodSwitch(original, slay(k tea, v normie) lit {
    damn v % 2 == 0
})
vibez.spill("Even:", even) fr fr map[two:2]
vibez.spill("Odd:", odd)   fr fr map[one:1 three:3]
```

### Conversion Functions
```cursed
fr fr Extract keys and values
sus keys := mood_map.KeyVibes(original)
sus values := mood_map.ValueVibes(original)
vibez.spill("Keys:", keys)     fr fr [one two three]
vibez.spill("Values:", values) fr fr [1 2 3]

fr fr Create from pairs
sus pairs := []mood_map.MapEntry{
    {Key: "a", Value: 10},
    {Key: "b", Value: 20}
}
sus fromPairs := mood_map.MapFromPairs(pairs)
vibez.spill("From pairs:", fromPairs)

fr fr Create from keys with value function
sus keyList := []tea{"x", "y", "z"}
sus fromKeys := mood_map.MapFromKeys(keyList, slay(key tea) normie {
    damn len(key) * 10
})
vibez.spill("From keys:", fromKeys) fr fr map[x:10 y:10 z:10]
```

### Counter Map
```cursed
fr fr Count occurrences
sus counter := mood_map.CounterMap()
counter.Increment("apple")
counter.Increment("banana")
counter.Increment("apple")

vibez.spill("Apple count:", counter.Get("apple"))   fr fr 2
vibez.spill("Banana count:", counter.Get("banana")) fr fr 1
vibez.spill("All counts:", counter.GetAll())

counter.Reset()
vibez.spill("After reset:", counter.Get("apple"))   fr fr 0
```

### Cache Map
```cursed
fr fr Cache with expiry
sus cache := mood_map.CacheMap(100) fr fr Expire after 100 time units
cache.Set("session1", 12345)
cache.Set("session2", 67890)

sus value, found := cache.Get("session1")
if found {
    vibez.spill("Session value:", value)
}

vibez.spill("Cache size:", cache.Size()) fr fr 2

cache.Delete("session1")
vibez.spill("After delete:", cache.Size()) fr fr 1
```

### Nested Map
```cursed
fr fr Nested key-value storage
sus nested := mood_map.NestedMap()
sus userPath := []tea{"users", "123", "profile", "age"}
nested.Set(userPath, 25)

sus age, found := nested.Get(userPath)
if found {
    vibez.spill("User age:", age) fr fr 25
}

sus configPath := []tea{"config", "theme", "color"}
nested.Set(configPath, 255)

vibez.spill("Has user age:", nested.HasKey(userPath))     fr fr true
vibez.spill("Has invalid:", nested.HasKey([]tea{"invalid"})) fr fr false

nested.Delete(userPath)
vibez.spill("After delete:", nested.HasKey(userPath))    fr fr false
```

### Thread-Safe Map
```cursed
fr fr Thread-safe operations
sus syncMap := mood_map.SyncMap()
syncMap.Store("key1", 100)
syncMap.Store("key2", 200)

sus value, exists := syncMap.Load("key1")
if exists {
    vibez.spill("Loaded:", value) fr fr 100
}

fr fr Load or store
sus existing, wasLoaded := syncMap.LoadOrStore("key1", 999)
vibez.spill("Existing value:", existing) fr fr 100
vibez.spill("Was loaded:", wasLoaded)    fr fr true

fr fr Atomic operations
sus oldValue, hadValue := syncMap.Swap("key1", 150)
vibez.spill("Old value:", oldValue) fr fr 100

sus swapped := syncMap.CompareAndSwap("key1", 150, 175)
vibez.spill("Swapped:", swapped) fr fr true

fr fr Iterate over entries
syncMap.Range(slay(key tea, value normie) lit {
    vibez.spill("Entry:", key, "=", value)
    damn based fr fr Continue iteration
})
```

### Advanced Operations
```cursed
fr fr Merge multiple maps
sus map1 := make(map[tea]normie)
map1["a"] = 1
map1["b"] = 2

sus map2 := make(map[tea]normie)
map2["b"] = 3
map2["c"] = 4

sus maps := []map[tea]normie{map1, map2}
sus merged := mood_map.MoodyMerge(maps, slay(key tea, old, new normie) normie {
    damn old + new fr fr Sum conflicting values
})
vibez.spill("Merged:", merged) fr fr map[a:1 b:5 c:4]

fr fr Check conditions
sus allPositive := mood_map.MoodCheck(merged, slay(k tea, v normie) lit {
    damn v > 0
})
vibez.spill("All positive:", allPositive) fr fr true

sus hasLarge := mood_map.FeelingSome(merged, slay(k tea, v normie) lit {
    damn v > 3
})
vibez.spill("Has large values:", hasLarge) fr fr true

fr fr Default map
sus baseMap := make(map[tea]normie)
baseMap["exists"] = 42
sus defaulter := mood_map.DefaultMap(baseMap, 999)

vibez.spill("Existing:", defaulter("exists")) fr fr 42
vibez.spill("Missing:", defaulter("missing"))  fr fr 999
```

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Functional Programming** - Higher-order functions for map operations
3. **Type Safety** - Strong typing for all operations
4. **Performance Optimized** - Efficient algorithms with minimal allocations
5. **Thread Safety** - SyncMap provides concurrent access patterns
6. **Expressive API** - Mood-based naming for intuitive usage
7. **Flexible Conversions** - Multiple ways to create and transform maps

## Error Handling

Most operations are designed to be safe and not panic:
- Missing keys return zero values or false flags
- Invalid operations are handled gracefully
- Functions return appropriate success/failure indicators

## Performance Considerations

- **Clone** creates a shallow copy (efficient for most use cases)
- **Vibe** and **Moody** create new maps (safe but uses more memory)
- **Clear** is more efficient than creating a new map
- **Counter** and **Cache** maps use internal storage optimization
- **SyncMap** uses simplified locking (real implementation would use sync.Map)

## Use Cases

1. **Data Processing** - Filter, transform, and aggregate map data
2. **Configuration Management** - Nested configuration with default values
3. **Caching** - Time-based expiry and LRU-style caching
4. **Counting** - Frequency analysis and statistics
5. **Concurrent Programming** - Thread-safe map operations
6. **Functional Programming** - Map operations in functional style

## Implementation Notes

This is a pure CURSED implementation that provides comprehensive map manipulation functionality without external dependencies. The module focuses on:

- Expressive, mood-based function naming
- Functional programming patterns
- Type-safe operations
- Performance optimization
- Comprehensive test coverage
- Clear documentation and examples

The implementation serves both practical map manipulation needs and educational purposes for understanding functional programming concepts with maps.
