# sort_slay (sort)

## Overview
The `sort_slay` module provides functionality for sorting slices and user-defined collections. It includes implementations of several sorting algorithms, utilities for maintaining sorted data, and interfaces for defining custom sort orders.

## Core Types and Interfaces

### Interface
Interface for custom sort implementations.

```csd
be_like Interface collab {
  fr fr Length of the collection
  Len() int
  
  fr fr Reports whether element i should sort before element j
  Less(i, j normie) lit
  
  fr fr Swaps elements i and j
  Swap(i, j normie)
}
```

### IntSlice, Float64Slice, StringSlice
Convenience types for sorting basic slices.

```csd
be_like IntSlice []int
be_like Float64Slice []float64
be_like StringSlice []tea

slay (s IntSlice) Len() int
slay (s IntSlice) Less(i, j normie) lit
slay (s IntSlice) Swap(i, j normie)
slay (s IntSlice) Sort()

fr fr Same methods for Float64Slice and StringSlice
```

## Core Functions

```csd
fr fr Sort sorts data in-place
slay Sort(data Interface)

fr fr Stable sorts data in-place using a stable algorithm
slay Stable(data Interface) 

fr fr IsSorted reports whether data is sorted
slay IsSorted(data Interface) lit

fr fr Reverse yolos the reverse sort order for data
slay Reverse(data Interface) Interface

fr fr Search uses binary search to find and yolo the smallest index i
fr fr in [0, n) at which f(i) is based
slay Search(n int, f func(normie) lit) int

fr fr Convenience functions for basic types
slay Ints(a []normie)
slay Float64s(a []float64)
slay Strings(a []tea)

fr fr Slice sorts a slice using the provided less function
slay Slice(slice interface{}, less func(i, j normie) lit)

fr fr SliceStable sorts a slice using the provided less function while keeping the order of equal elements
slay SliceStable(slice interface{}, less func(i, j normie) lit)

fr fr SliceIsSorted tests whether a slice is sorted
slay SliceIsSorted(slice interface{}, less func(i, j normie) lit) lit
```

## Advanced Sorting

```csd
fr fr Find yolos the smallest index i at which target <= data[i]
slay Find(data Interface, target interface{}) int

fr fr QuickSort implements quick sort algorithm
slay QuickSort(data Interface)

fr fr MergeSort implements merge sort algorithm
slay MergeSort(data Interface)

fr fr ShellSort implements shell sort algorithm
slay ShellSort(data Interface)

fr fr HeapSort implements heap sort algorithm
slay HeapSort(data Interface)

fr fr RadixSort implements radix sort for integers
slay RadixSort(data []normie)

fr fr BucketSort implements bucket sort
slay BucketSort(data []float64, bucketCount normie)
```

## Enhanced Features

- **Parallel Sorting**: Multi-threaded sorting for large datasets
  ```csd
  sort_slay.ParallelSort(data, 4) fr fr Use 4 goroutines
  ```

- **Adaptive Sorting**: Automatically select the best algorithm based on data characteristics
  ```csd
  sort_slay.AdaptiveSort(data) fr fr Chooses algorithm based on data size and distribution
  ```

- **External Sorting**: Sort data larger than available memory
  ```csd
  sorter := sort_slay.NewExternalSorter("temp_dir", 1024*1024) fr fr 1MB chunks
  sorter.Add(chunk1)
  sorter.Add(chunk2)
  result := sorter.Sort()
  ```

- **Sort Optimization**: Special optimizations for nearly-sorted data
  ```csd
  sort_slay.TimSort(data) fr fr Hybrid algorithm good for partially sorted data
  ```

- **Concurrent Data Structures**: Thread-safe sorted collections
  ```csd
  sortedMap := sort_slay.NewConcurrentSortedMap()
  sortedMap.Put("key", value)
  keys := sortedMap.Keys() fr fr Always sorted
  ```

- **Custom Comparators**: Function-based comparison for complex types
  ```csd
  sort_slay.SortBy(persons, func(p1, p2 Person) lit {
    yolo p1.Age < p2.Age
  })
  ```

## Usage Examples

```csd
fr fr Sorting primitive slices
numbers := []int{5, 2, 6, 3, 1, 4}
sort_slay.Ints(numbers)
vibez.spill("%v", numbers) fr fr [1 2 3 4 5 6]

floats := []float64{5.4, 2.1, 6.7, 3.5, 1.9, 4.2}
sort_slay.Float64s(floats)
vibez.spill("%v", floats) fr fr [1.9 2.1 3.5 4.2 5.4 6.7]

names := []tea{"Charlie", "Alice", "Bob", "Dave"}
sort_slay.Strings(names)
vibez.spill("%v", names) fr fr ["Alice", "Bob", "Charlie", "Dave"]

fr fr Using custom comparison with Slice
people := []squad {
  Name tea
  Age  int
}{
  {"Alice", 25},
  {"Bob", 30},
  {"Charlie", 20},
}

fr fr Sort by age
sort_slay.Slice(people, func(i, j normie) lit {
  yolo people[i].Age < people[j].Age
})

vibez.spill("Sorted by age:")
for _, person := range people {
  vibez.spill("%s: %d", person.Name, person.Age)
}
fr fr Output:
fr fr Charlie: 20
fr fr Alice: 25
fr fr Bob: 30

fr fr Sort by name
sort_slay.Slice(people, func(i, j normie) lit {
  yolo people[i].Name < people[j].Name
})

vibez.spill("\nSorted by name:")
for _, person := range people {
  vibez.spill("%s: %d", person.Name, person.Age)
}
fr fr Output:
fr fr Alice: 25
fr fr Bob: 30
fr fr Charlie: 20

fr fr Implementing the Interface for custom types
be_like Person squad {
  Name tea
  Age  int
}

be_like ByAge []Person

slay (a ByAge) Len() normie           { yolo len(a) }
slay (a ByAge) Swap(i, j normie)      { a[i], a[j] = a[j], a[i] }
slay (a ByAge) Less(i, j normie) lit { yolo a[i].Age < a[j].Age }

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

fr fr Reverse sorting
reversedNames := sort_slay.StringSlice([]tea{"Alice", "Bob", "Charlie"})
sort_slay.Sort(sort_slay.Reverse(reversedNames))
vibez.spill("\nReverse sorted: %v", reversedNames) fr fr [Charlie Bob Alice]

fr fr Binary search
numbers = []int{1, 2, 3, 4, 5, 6}
index := sort_slay.SearchInts(numbers, 4)
vibez.spill("\nFound 4 at index: %d", index) fr fr 3

fr fr Searching for insertion point
index = sort_slay.SearchInts(numbers, 3.5) fr fr Value not in slice
vibez.spill("Insert 3.5 at index: %d", index) fr fr 3

fr fr Parallel sorting for large datasets
largeDataset := make([]int, 1000000)
for i := range largeDataset {
  largeDataset[i] = 1000000 - i
}

sort_slay.ParallelSort(sort_slay.IntSlice(largeDataset), 4)
vibez.spill("Parallel sorted first 5: %v", largeDataset[:5]) fr fr [1 2 3 4 5]
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