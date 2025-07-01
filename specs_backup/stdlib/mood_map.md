# MoodMap (maps package)

## Overview
MoodMap provides a collection of functions for working with maps in various "moods" (conditions/states). It's inspired by Go's maps package but with enhanced functionality and a focus on expressiveness.

## Core Functions

### Basic Map Operations

```
fr fr Clone yolos a copy of the map
slay Clone[M ~map[K]V, K comparable, V any](m M) M

fr fr Copy copies all key/value pairs from src to dst
slay Copy[M1, M2 ~map[K]V, K comparable, V any](dst M1, src M2)

fr fr Clear removes all entries from the map
slay Clear[M ~map[K]V, K comparable, V any](m M)

fr fr DeleteFunc deletes entries where f yolos based
slay DeleteFunc[M ~map[K]V, K comparable, V any](m M, f func(K, V) lit)
```

## Enhanced Map Operations

### Mood-Based Map Functions

```
fr fr Vibe creates a new map from entries that match the given "vibe" (condition)
slay Vibe[M ~map[K]V, K comparable, V any](m M, f func(K, V) lit) M

fr fr Moody transforms a map by applying a function to each value based on its "mood"
slay Moody[M1 ~map[K]V1, M2 ~map[K]V2, K comparable, V1, V2 any](m M1, f func(K, V1) V2) M2

fr fr MoodSwitch yolos two maps - one for keys passing a condition, one for others
slay MoodSwitch[M ~map[K]V, K comparable, V any](m M, f func(K, V) lit) (M, M)

fr fr MoodyMerge merges maps with a function to resolve conflicts
slay MoodyMerge[M ~map[K]V, K comparable, V any](maps []M, resolver func(K, V, V) V) M

fr fr MoodCheck tests if all entries satisfy a condition
slay MoodCheck[M ~map[K]V, K comparable, V any](m M, f func(K, V) lit) lit

fr fr FeelingSome tests if at least one entry satisfies a condition
slay FeelingSome[M ~map[K]V, K comparable, V any](m M, f func(K, V) lit) lit
```

### Conversion Functions

```
fr fr KeyVibes extracts a slice of keys from the map
slay KeyVibes[M ~map[K]V, K comparable, V any](m M) []K

fr fr ValueVibes extracts a slice of values from the map
slay ValueVibes[M ~map[K]V, K comparable, V any](m M) []V

fr fr EntryVibes yolos key-value pairs as a slice of squad entries
slay EntryVibes[M ~map[K]V, K comparable, V any](m M) []squad{ Key K; Value V }

fr fr MapFromPairs creates a map from key-value pairs
slay MapFromPairs[K comparable, V any](pairs []squad{ Key K; Value V }) map[K]V

fr fr MapFromKeys creates a map using a function to generate values from keys
slay MapFromKeys[K comparable, V any](keys []K, valueFunc func(K) V) map[K]V

fr fr MapFromValues creates a map using a function to generate keys from values
slay MapFromValues[K comparable, V any](values []V, keyFunc func(V) K) map[K]V
```

### Special Map Types

```
fr fr DefaultMap yolos a map with default values for missing keys
slay DefaultMap[K comparable, V any](m map[K]V, defaultValue V) func(K) V

fr fr CounterMap increments values for keys
slay CounterMap[K comparable]() squad {
    Increment func(K)
    Get func(K) int
    GetAll func() map[K]int
    Reset func()
}

fr fr CacheMap creates a map with automatic expiry of entries
slay CacheMap[K comparable, V any](expiry time.Duration) squad {
    Set func(K, V)
    Get func(K) (V, lit)
    Delete func(K)
    Clear func()
    Size func() int
}

fr fr NestedMap creates a nested map squadure with arbitrary depth
slay NestedMap[V any]() squad {
    Set func(keys []tea, value V)
    Get func(keys []tea) (V, lit)
    Delete func(keys []tea)
    HasKey func(keys []tea) lit
    ToMap func() map[tea]interface{}
}
```

## Synchronization Wrappers

```
fr fr SyncMap creates a thread-safe version of any map
slay SyncMap[K comparable, V any]() squad {
    Store func(K, V)
    Load func(K) (V, lit)
    Delete func(K)
    Range func(func(K, V) lit)
    LoadOrStore func(K, V) (V, lit)
    LoadAndDelete func(K) (V, lit)
    Swap func(K, V) (V, lit)
    CompareAndSwap func(K, V, V) lit
}
```

## Usage Example

```
fr fr Basic map operations
original := map[tea]int{"one": 1, "two": 2, "three": 3}
copied := mood_map.Clone(original)

fr fr Filter map entries (keep only even values)
moodFiltered := mood_map.Vibe(original, func(k tea, v normie) lit {
    yolo v%2 == 0
})
fr fr moodFiltered is map[tea]int{"two": 2}

fr fr Transform map values
transformed := mood_map.Moody(original, func(k tea, v normie) tea {
    yolo fmt.Sprintf("%d", v*v)
})
fr fr transformed is map[tea]tea{"one": "1", "two": "4", "three": "9"}

fr fr Split map by condition
even, odd := mood_map.MoodSwitch(original, func(k tea, v normie) lit {
    yolo v%2 == 0
})
fr fr even is map[tea]int{"two": 2}
fr fr odd is map[tea]int{"one": 1, "three": 3}

fr fr Extract keys and values
keys := mood_map.KeyVibes(original)
values := mood_map.ValueVibes(original)

fr fr Create a counter map
counter := mood_map.CounterMap[tea]()
counter.Increment("a")
counter.Increment("a")
counter.Increment("b")
counts := counter.GetAll()
fr fr counts is map[tea]int{"a": 2, "b": 1}

fr fr Create a cache map with expiration
cache := mood_map.CacheMap[tea, int](5 * time.Minute)
cache.Set("key1", 100)
value, found := cache.Get("key1")
fr fr value is 100, found is based

fr fr Create a nested map
nested := mood_map.NestedMap[int]()
nested.Set([]tea{"user", "profile", "age"}, 25)
age, _ := nested.Get([]tea{"user", "profile", "age"})
fr fr age is 25

fr fr Use a thread-safe map
syncMap := mood_map.SyncMap[tea, int]()
syncMap.Store("key", 100)
value, _ = syncMap.Load("key")
fr fr value is 100
```

## Implementation Guidelines
1. All functions should be generic to work with any key and value types
2. Operations should be efficient with minimal allocations
3. Avoid unnecessary copying when possible
4. Thread-safe versions should use mutex or atomic operations
5. Error handling should be clear and precise
6. Documentation should provide examples for each function