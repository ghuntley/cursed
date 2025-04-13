# MoodMap (maps package)

## Overview
MoodMap provides a collection of functions for working with maps in various "moods" (conditions/states). It's inspired by Go's maps package but with enhanced functionality and a focus on expressiveness.

## Core Functions

### Basic Map Operations

```go
// Clone returns a copy of the map
func Clone[M ~map[K]V, K comparable, V any](m M) M

// Copy copies all key/value pairs from src to dst
func Copy[M1, M2 ~map[K]V, K comparable, V any](dst M1, src M2)

// Clear removes all entries from the map
func Clear[M ~map[K]V, K comparable, V any](m M)

// DeleteFunc deletes entries where f returns true
func DeleteFunc[M ~map[K]V, K comparable, V any](m M, f func(K, V) bool)
```

## Enhanced Map Operations

### Mood-Based Map Functions

```go
// Vibe creates a new map from entries that match the given "vibe" (condition)
func Vibe[M ~map[K]V, K comparable, V any](m M, f func(K, V) bool) M

// Moody transforms a map by applying a function to each value based on its "mood"
func Moody[M1 ~map[K]V1, M2 ~map[K]V2, K comparable, V1, V2 any](m M1, f func(K, V1) V2) M2

// MoodSwitch returns two maps - one for keys passing a condition, one for others
func MoodSwitch[M ~map[K]V, K comparable, V any](m M, f func(K, V) bool) (M, M)

// MoodyMerge merges maps with a function to resolve conflicts
func MoodyMerge[M ~map[K]V, K comparable, V any](maps []M, resolver func(K, V, V) V) M

// MoodCheck tests if all entries satisfy a condition
func MoodCheck[M ~map[K]V, K comparable, V any](m M, f func(K, V) bool) bool

// FeelingSome tests if at least one entry satisfies a condition
func FeelingSome[M ~map[K]V, K comparable, V any](m M, f func(K, V) bool) bool
```

### Conversion Functions

```go
// KeyVibes extracts a slice of keys from the map
func KeyVibes[M ~map[K]V, K comparable, V any](m M) []K

// ValueVibes extracts a slice of values from the map
func ValueVibes[M ~map[K]V, K comparable, V any](m M) []V

// EntryVibes returns key-value pairs as a slice of struct entries
func EntryVibes[M ~map[K]V, K comparable, V any](m M) []struct{ Key K; Value V }

// MapFromPairs creates a map from key-value pairs
func MapFromPairs[K comparable, V any](pairs []struct{ Key K; Value V }) map[K]V

// MapFromKeys creates a map using a function to generate values from keys
func MapFromKeys[K comparable, V any](keys []K, valueFunc func(K) V) map[K]V

// MapFromValues creates a map using a function to generate keys from values
func MapFromValues[K comparable, V any](values []V, keyFunc func(V) K) map[K]V
```

### Special Map Types

```go
// DefaultMap returns a map with default values for missing keys
func DefaultMap[K comparable, V any](m map[K]V, defaultValue V) func(K) V

// CounterMap increments values for keys
func CounterMap[K comparable]() struct {
    Increment func(K)
    Get func(K) int
    GetAll func() map[K]int
    Reset func()
}

// CacheMap creates a map with automatic expiry of entries
func CacheMap[K comparable, V any](expiry time.Duration) struct {
    Set func(K, V)
    Get func(K) (V, bool)
    Delete func(K)
    Clear func()
    Size func() int
}

// NestedMap creates a nested map structure with arbitrary depth
func NestedMap[V any]() struct {
    Set func(keys []string, value V)
    Get func(keys []string) (V, bool)
    Delete func(keys []string)
    HasKey func(keys []string) bool
    ToMap func() map[string]interface{}
}
```

## Synchronization Wrappers

```go
// SyncMap creates a thread-safe version of any map
func SyncMap[K comparable, V any]() struct {
    Store func(K, V)
    Load func(K) (V, bool)
    Delete func(K)
    Range func(func(K, V) bool)
    LoadOrStore func(K, V) (V, bool)
    LoadAndDelete func(K) (V, bool)
    Swap func(K, V) (V, bool)
    CompareAndSwap func(K, V, V) bool
}
```

## Usage Example

```go
// Basic map operations
original := map[string]int{"one": 1, "two": 2, "three": 3}
copied := mood_map.Clone(original)

// Filter map entries (keep only even values)
moodFiltered := mood_map.Vibe(original, func(k string, v int) bool {
    return v%2 == 0
})
// moodFiltered is map[string]int{"two": 2}

// Transform map values
transformed := mood_map.Moody(original, func(k string, v int) string {
    return fmt.Sprintf("%d", v*v)
})
// transformed is map[string]string{"one": "1", "two": "4", "three": "9"}

// Split map by condition
even, odd := mood_map.MoodSwitch(original, func(k string, v int) bool {
    return v%2 == 0
})
// even is map[string]int{"two": 2}
// odd is map[string]int{"one": 1, "three": 3}

// Extract keys and values
keys := mood_map.KeyVibes(original)
values := mood_map.ValueVibes(original)

// Create a counter map
counter := mood_map.CounterMap[string]()
counter.Increment("a")
counter.Increment("a")
counter.Increment("b")
counts := counter.GetAll()
// counts is map[string]int{"a": 2, "b": 1}

// Create a cache map with expiration
cache := mood_map.CacheMap[string, int](5 * time.Minute)
cache.Set("key1", 100)
value, found := cache.Get("key1")
// value is 100, found is true

// Create a nested map
nested := mood_map.NestedMap[int]()
nested.Set([]string{"user", "profile", "age"}, 25)
age, _ := nested.Get([]string{"user", "profile", "age"})
// age is 25

// Use a thread-safe map
syncMap := mood_map.SyncMap[string, int]()
syncMap.Store("key", 100)
value, _ = syncMap.Load("key")
// value is 100
```

## Implementation Guidelines
1. All functions should be generic to work with any key and value types
2. Operations should be efficient with minimal allocations
3. Avoid unnecessary copying when possible
4. Thread-safe versions should use mutex or atomic operations
5. Error handling should be clear and precise
6. Documentation should provide examples for each function