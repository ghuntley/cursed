# SortaFresh Module

A comprehensive sorting and caching library for CURSED with modern algorithms and Gen Z-inspired features.

## Overview

SortaFresh provides efficient sorting algorithms, caching mechanisms, and specialized sorting functions with a fresh, modern approach. It includes both traditional sorting algorithms and Gen Z-themed sorting features.

## Features

### Core Sorting Functions

- **SortInts(arr []normie) []normie** - Sort integers in ascending order
- **SortStrings(arr []tea) []tea** - Sort strings in ascending order  
- **SortFloat64s(arr []meal) []meal** - Sort floats in ascending order
- **StableSort(arr []normie) []normie** - Stable sorting (preserves equal element order)
- **ReverseSort(arr []normie) []normie** - Sort in descending order
- **Shuffle(arr []normie) []normie** - Randomly shuffle array elements

### Sorting Validation

- **IntsAreSorted(arr []normie) lit** - Check if integer array is sorted
- **StringsAreSorted(arr []tea) lit** - Check if string array is sorted
- **Float64sAreSorted(arr []meal) lit** - Check if float array is sorted

### Binary Search Functions

- **SearchInts(arr []normie, target normie) normie** - Binary search in sorted integers
- **SearchStrings(arr []tea, target tea) normie** - Binary search in sorted strings
- **SearchFloat64s(arr []meal, target meal) normie** - Binary search in sorted floats

### Selection Algorithms

- **TopK(arr []normie, k normie) []normie** - Get k largest elements
- **BottomK(arr []normie, k normie) []normie** - Get k smallest elements
- **Median(arr []normie) normie** - Find median element
- **QuickSelect(arr []normie, k normie) normie** - Find k-th smallest element

### Caching System

- **NewSortCache(size normie) SortCache** - Create a new sort cache
- **CachedSort(cache SortCache, arr []normie) []normie** - Sort with caching
- **ClearCache(cache SortCache)** - Clear cache contents
- **GetCacheStats(cache SortCache) (normie, normie)** - Get cache statistics

### Gen Z Sorting Features

- **VibeSort(arr []normie) []normie** - Sort by "vibe" score (lower values = better vibes)
- **NoCapSort(arr []normie) []normie** - Absolute factual ordering (regular sort)
- **BussinSort(arr []normie) []normie** - Sort by "bussin" score (higher = more excellent)
- **SlaySort(arr []normie) []normie** - High-performance sort that "slays"
- **YeetSort(arr []normie, min_value normie) []normie** - Filter and sort (remove unwanted elements)

## Usage Examples

### Basic Sorting

```cursed
yeet "sorta_fresh"

slay main() {
    // Sort integers
    sus numbers []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted []normie = SortInts(numbers)
    vibez.spill("Sorted:", sorted)  // [1, 1, 2, 3, 4, 5, 6, 9]
    
    // Sort strings
    sus words []tea = ["zebra", "apple", "banana", "cherry"]
    sus sorted_words []tea = SortStrings(words)
    vibez.spill("Sorted words:", sorted_words)  // ["apple", "banana", "cherry", "zebra"]
    
    // Check if sorted
    sus is_sorted lit = IntsAreSorted(sorted)
    vibez.spill("Is sorted:", is_sorted)  // based (true)
}
```

### Binary Search

```cursed
yeet "sorta_fresh"

slay main() {
    sus sorted_nums []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    sus index normie = SearchInts(sorted_nums, 5)
    vibez.spill("Index of 5:", index)  // 4
    
    sus not_found normie = SearchInts(sorted_nums, 10)
    vibez.spill("Index of 10:", not_found)  // -1
}
```

### Top-K Selection

```cursed
yeet "sorta_fresh"

slay main() {
    sus scores []normie = [85, 92, 78, 95, 88, 76, 90, 94]
    
    sus top_three []normie = TopK(scores, 3)
    vibez.spill("Top 3 scores:", top_three)  // [95, 94, 92]
    
    sus bottom_three []normie = BottomK(scores, 3)
    vibez.spill("Bottom 3 scores:", bottom_three)  // [76, 78, 85]
    
    sus median_score normie = Median(scores)
    vibez.spill("Median score:", median_score)
}
```

### Caching System

```cursed
yeet "sorta_fresh"

slay main() {
    // Create cache with size 5
    sus cache SortCache = NewSortCache(5)
    
    sus data []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    
    // First sort - computed and cached
    sus sorted1 []normie = CachedSort(cache, data)
    
    // Second sort - retrieved from cache
    sus sorted2 []normie = CachedSort(cache, data)
    
    sus current_size normie
    sus max_size normie
    (current_size, max_size) = GetCacheStats(cache)
    vibez.spill("Cache stats:", current_size, "/", max_size)
    
    // Clear cache
    ClearCache(cache)
}
```

### Gen Z Sorting Features

```cursed
yeet "sorta_fresh"

slay main() {
    sus data []normie = [5, 3, 8, 1, 9, 2, 7, 4, 6]
    
    // Sort by vibe (lower values = better vibes)
    sus vibe_sorted []normie = VibeSort(data)
    vibez.spill("Vibe sorted:", vibe_sorted)
    
    // Sort by bussin score (higher values = more excellent)
    sus bussin_sorted []normie = BussinSort(data)
    vibez.spill("Bussin sorted:", bussin_sorted)
    
    // Filter out low values and sort the rest
    sus yeeted []normie = YeetSort(data, 5)
    vibez.spill("Yeeted (≥5):", yeeted)  // [5, 6, 7, 8, 9]
    
    // High-performance sort
    sus slayed []normie = SlaySort(data)
    vibez.spill("Slayed:", slayed)
    
    // No cap - just regular sort with attitude
    sus no_cap []normie = NoCapSort(data)
    vibez.spill("No cap:", no_cap)
}
```

### Stable vs Unstable Sorting

```cursed
yeet "sorta_fresh"

slay main() {
    sus data []normie = [3, 1, 4, 1, 5, 9, 2, 6]
    
    // Stable sort preserves order of equal elements
    sus stable_sorted []normie = StableSort(data)
    vibez.spill("Stable sorted:", stable_sorted)
    
    // Regular sort (may not preserve order of equal elements)
    sus regular_sorted []normie = SortInts(data)
    vibez.spill("Regular sorted:", regular_sorted)
    
    // Reverse sort
    sus reversed []normie = ReverseSort(data)
    vibez.spill("Reverse sorted:", reversed)
}
```

## Performance Characteristics

- **SortInts/SortStrings/SortFloat64s**: O(n log n) average case (QuickSort)
- **StableSort**: O(n log n) guaranteed (MergeSort)
- **SearchInts/SearchStrings/SearchFloat64s**: O(log n) binary search
- **TopK/BottomK**: O(n log n) via full sort
- **QuickSelect**: O(n) average case, O(n²) worst case
- **Median**: O(n log n) via sort
- **Shuffle**: O(n) linear time

## Caching Benefits

The caching system provides:
- **Reduced Computation**: Avoid re-sorting identical arrays
- **Memory Efficiency**: FIFO cache with configurable size
- **Performance Monitoring**: Cache hit/miss statistics
- **Flexibility**: Easy to clear or resize cache

## Algorithm Details

### Core Algorithms
- **QuickSort**: Fast general-purpose sorting
- **MergeSort**: Stable sorting with guaranteed O(n log n)
- **Binary Search**: Efficient searching in sorted arrays
- **QuickSelect**: Linear-time selection algorithm

### Gen Z Features
- **VibeSort**: Custom scoring based on "vibe" (100 - value)
- **BussinSort**: Scoring based on "bussin" factor (value * 2)
- **YeetSort**: Filtering combined with sorting
- **SlaySort**: Optimized QuickSort implementation

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/sorta_fresh/test_sorta_fresh.csd
```

Test both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/sorta_fresh/test_sorta_fresh.csd
cargo run --bin cursed -- compile stdlib/sorta_fresh/test_sorta_fresh.csd
./test_sorta_fresh
```

## Implementation Notes

- Pure CURSED implementation with no external dependencies
- Follows existing stdlib patterns and conventions
- Comprehensive error handling for edge cases
- Memory-efficient with proper array copying
- Modular design for easy extension

## Future Enhancements

- Parallel sorting for large datasets
- Additional specialized algorithms (RadixSort, CountingSort)
- Custom comparison functions
- Priority queue integration
- Advanced caching strategies (LRU, LFU)
