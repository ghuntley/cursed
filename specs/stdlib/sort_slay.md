# sort_slay (sort)

## Overview
The `sort_slay` module provides functionality for sorting slices and user-defined collections. It includes implementations of several sorting algorithms, utilities for maintaining sorted data, and interfaces for defining custom sort orders.

## Core Types and Interfaces

### Interface
Interface for custom sort implementations.

```csd
type Interface interface {
  // Length of the collection
  Len() int
  
  // Reports whether element i should sort before element j
  Less(i, j int) bool
  
  // Swaps elements i and j
  Swap(i, j int)
}
```

### IntSlice, Float64Slice, StringSlice
Convenience types for sorting basic slices.

```csd
type IntSlice []int
type Float64Slice []float64
type StringSlice []string

func (s IntSlice) Len() int
func (s IntSlice) Less(i, j int) bool
func (s IntSlice) Swap(i, j int)
func (s IntSlice) Sort()

// Same methods for Float64Slice and StringSlice
```

## Core Functions

```csd
// Sort sorts data in-place
func Sort(data Interface)

// Stable sorts data in-place using a stable algorithm
func Stable(data Interface) 

// IsSorted reports whether data is sorted
func IsSorted(data Interface) bool

// Reverse returns the reverse sort order for data
func Reverse(data Interface) Interface

// Search uses binary search to find and return the smallest index i
// in [0, n) at which f(i) is true
func Search(n int, f func(int) bool) int

// Convenience functions for basic types
func Ints(a []int)
func Float64s(a []float64)
func Strings(a []string)

// Slice sorts a slice using the provided less function
func Slice(slice interface{}, less func(i, j int) bool)

// SliceStable sorts a slice using the provided less function while keeping the order of equal elements
func SliceStable(slice interface{}, less func(i, j int) bool)

// SliceIsSorted tests whether a slice is sorted
func SliceIsSorted(slice interface{}, less func(i, j int) bool) bool
```

## Advanced Sorting

```csd
// Find returns the smallest index i at which target <= data[i]
func Find(data Interface, target interface{}) int

// QuickSort implements quick sort algorithm
func QuickSort(data Interface)

// MergeSort implements merge sort algorithm
func MergeSort(data Interface)

// ShellSort implements shell sort algorithm
func ShellSort(data Interface)

// HeapSort implements heap sort algorithm
func HeapSort(data Interface)

// RadixSort implements radix sort for integers
func RadixSort(data []int)

// BucketSort implements bucket sort
func BucketSort(data []float64, bucketCount int)
```

## Enhanced Features

- **Parallel Sorting**: Multi-threaded sorting for large datasets
  ```csd
  sort_slay.ParallelSort(data, 4) // Use 4 goroutines
  ```

- **Adaptive Sorting**: Automatically select the best algorithm based on data characteristics
  ```csd
  sort_slay.AdaptiveSort(data) // Chooses algorithm based on data size and distribution
  ```

- **External Sorting**: Sort data larger than available memory
  ```csd
  sorter := sort_slay.NewExternalSorter("temp_dir", 1024*1024) // 1MB chunks
  sorter.Add(chunk1)
  sorter.Add(chunk2)
  result := sorter.Sort()
  ```

- **Sort Optimization**: Special optimizations for nearly-sorted data
  ```csd
  sort_slay.TimSort(data) // Hybrid algorithm good for partially sorted data
  ```

- **Concurrent Data Structures**: Thread-safe sorted collections
  ```csd
  sortedMap := sort_slay.NewConcurrentSortedMap()
  sortedMap.Put("key", value)
  keys := sortedMap.Keys() // Always sorted
  ```

- **Custom Comparators**: Function-based comparison for complex types
  ```csd
  sort_slay.SortBy(persons, func(p1, p2 Person) bool {
    return p1.Age < p2.Age
  })
  ```

## Usage Examples

```csd
// Sorting primitive slices
numbers := []int{5, 2, 6, 3, 1, 4}
sort_slay.Ints(numbers)
vibez.spill("%v", numbers) // [1 2 3 4 5 6]

floats := []float64{5.4, 2.1, 6.7, 3.5, 1.9, 4.2}
sort_slay.Float64s(floats)
vibez.spill("%v", floats) // [1.9 2.1 3.5 4.2 5.4 6.7]

names := []string{"Charlie", "Alice", "Bob", "Dave"}
sort_slay.Strings(names)
vibez.spill("%v", names) // ["Alice", "Bob", "Charlie", "Dave"]

// Using custom comparison with Slice
people := []struct {
  Name string
  Age  int
}{
  {"Alice", 25},
  {"Bob", 30},
  {"Charlie", 20},
}

// Sort by age
sort_slay.Slice(people, func(i, j int) bool {
  return people[i].Age < people[j].Age
})

vibez.spill("Sorted by age:")
for _, person := range people {
  vibez.spill("%s: %d", person.Name, person.Age)
}
// Output:
// Charlie: 20
// Alice: 25
// Bob: 30

// Sort by name
sort_slay.Slice(people, func(i, j int) bool {
  return people[i].Name < people[j].Name
})

vibez.spill("\nSorted by name:")
for _, person := range people {
  vibez.spill("%s: %d", person.Name, person.Age)
}
// Output:
// Alice: 25
// Bob: 30
// Charlie: 20

// Implementing the Interface for custom types
type Person struct {
  Name string
  Age  int
}

type ByAge []Person

func (a ByAge) Len() int           { return len(a) }
func (a ByAge) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByAge) Less(i, j int) bool { return a[i].Age < a[j].Age }

personList := []Person{
  {"Alice", 25},
  {"Bob", 30},
  {"Charlie", 20},
}

sort_slay.Sort(ByAge(personList))
vibez.spill("\nSorted with custom interface:")
for _, person := range personList {
  vibez.spill("%s: %d", person.Name, person.Age)
}

// Reverse sorting
reversedNames := sort_slay.StringSlice([]string{"Alice", "Bob", "Charlie"})
sort_slay.Sort(sort_slay.Reverse(reversedNames))
vibez.spill("\nReverse sorted: %v", reversedNames) // [Charlie Bob Alice]

// Binary search
numbers = []int{1, 2, 3, 4, 5, 6}
index := sort_slay.SearchInts(numbers, 4)
vibez.spill("\nFound 4 at index: %d", index) // 3

// Searching for insertion point
index = sort_slay.SearchInts(numbers, 3.5) // Value not in slice
vibez.spill("Insert 3.5 at index: %d", index) // 3

// Parallel sorting for large datasets
largeDataset := make([]int, 1000000)
for i := range largeDataset {
  largeDataset[i] = 1000000 - i
}

sort_slay.ParallelSort(sort_slay.IntSlice(largeDataset), 4)
vibez.spill("Parallel sorted first 5: %v", largeDataset[:5]) // [1 2 3 4 5]
```

## Implementation Guidelines

- Use optimized algorithms based on data size and characteristics
- Minimize memory allocations during sorting operations
- Provide stable sorting when order preservation is important
- Implement efficient binary search for sorted data
- Support parallel sorting for large datasets
- Allow custom comparison functions for flexibility
- Ensure thread safety for concurrent sorting operations
- Optimize for common cases like partially sorted data
- Support efficient external sorting for large datasets