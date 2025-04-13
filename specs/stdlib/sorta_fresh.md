# SortaFresh (sort package)

## Overview
SortaFresh provides functions for sorting slices and user-defined collections with a fresh, efficient approach. It's inspired by Go's sort package but with enhanced features, intuitive interfaces, and support for modern sorting techniques.

## Core Interfaces

### `Sortable`
Interface for basic sorting operations.

```go
type Sortable interface {
    // Len returns the number of elements in the collection
    Len() int
    
    // Less reports whether element i should sort before element j
    Less(i, j int) bool
    
    // Swap swaps elements i and j
    Swap(i, j int)
}
```

### `SortableSearch`
Interface that extends Sortable with search capability.

```go
type SortableSearch interface {
    Sortable
    
    // Search returns the smallest index i such that a[i] >= value
    Search(value interface{}) int
}
```

## Core Functions

### Basic Sorting

```go
// Sort sorts data in ascending order as determined by the Less method
func Sort(data Sortable)

// Reverse sorts data in reverse order
func Reverse(data Sortable)

// IsSorted reports whether data is sorted
func IsSorted(data Sortable) bool

// Stable performs a stable sort on data
func Stable(data Sortable)

// Shuffle randomizes the order of elements in data
func Shuffle(data Sortable)
```

### Specialized Sorting Functions

```go
// SortInts sorts a slice of ints in ascending order
func SortInts(a []int)

// SortFloat64s sorts a slice of float64s in ascending order
func SortFloat64s(a []float64)

// SortStrings sorts a slice of strings in ascending order
func SortStrings(a []string)

// IntsAreSorted reports whether the slice is sorted in ascending order
func IntsAreSorted(a []int) bool

// Float64sAreSorted reports whether the slice is sorted in ascending order
func Float64sAreSorted(a []float64) bool

// StringsAreSorted reports whether the slice is sorted in ascending order
func StringsAreSorted(a []string) bool
```

### Search Functions

```go
// Search uses binary search to find the index in a sorted data structure
func Search(n int, f func(int) bool) int

// SearchInts searches for x in a sorted slice of ints and returns the index
func SearchInts(a []int, x int) int

// SearchFloat64s searches for x in a sorted slice of float64s and returns the index
func SearchFloat64s(a []float64, x float64) int

// SearchStrings searches for x in a sorted slice of strings and returns the index
func SearchStrings(a []string, x string) int
```

## Generic Sorting Functions

```go
// SortSlice sorts a slice using the provided less function
func SortSlice[T any](slice []T, less func(i, j int) bool)

// StableSortSlice performs a stable sort on a slice using the provided less function
func StableSortSlice[T any](slice []T, less func(i, j int) bool)

// SortFunc sorts slice with a comparison function
func SortFunc[T any](slice []T, cmp func(a, b T) int)

// StableSortFunc performs a stable sort using a comparison function
func StableSortFunc[T any](slice []T, cmp func(a, b T) int)

// SortBy sorts a slice by a key extracted from each element
func SortBy[T any, K cmp.Ordered](slice []T, key func(T) K)

// SortByFields sorts a slice of structs by multiple fields
func SortByFields[T any](slice []T, fields ...SortField[T])

type SortField[T any] struct {
    Extract func(T) interface{}
    Reverse bool
}
```

## Enhanced Sorting Features

### `FreshSorter`
High-performance, parallel sorter for large collections.

```go
type FreshSorter struct {}

// Constructors
func NewFreshSorter() *FreshSorter
func NewFreshSorterWithOptions(opts FreshSorterOptions) *FreshSorter

type FreshSorterOptions struct {
    Parallel     bool
    MaxGoroutines int
    StableSorting bool
    Threshold    int    // Threshold for switching to insertion sort
    ChunkSize    int    // Size of chunks for parallel sorting
}

// Methods
func (s *FreshSorter) Sort(data Sortable) *FreshSorter
func (s *FreshSorter) SortSlice(slice interface{}, less func(i, j int) bool) *FreshSorter
func (s *FreshSorter) SortFunc(slice interface{}, cmp interface{}) *FreshSorter
func (s *FreshSorter) Stable(data Sortable) *FreshSorter
func (s *FreshSorter) Reverse() *FreshSorter
func (s *FreshSorter) Shuffle() *FreshSorter
func (s *FreshSorter) SetParallel(enable bool) *FreshSorter
func (s *FreshSorter) SetOptions(opts FreshSorterOptions) *FreshSorter
```

### Multi-Key Sorting

```go
// OrderedBy returns a multi-key Sortable based on less functions
func OrderedBy(less ...func(i, j int) bool) *FreshMultiSorter

type FreshMultiSorter struct {}

// Methods for FreshMultiSorter
func (ms *FreshMultiSorter) Sort(data interface{}) *FreshMultiSorter
func (ms *FreshMultiSorter) Reverse() *FreshMultiSorter
func (ms *FreshMultiSorter) AsThenBy(less func(i, j int) bool) *FreshMultiSorter
```

### Top-K Selection

```go
// TopK returns the k largest elements in data
func TopK[T any](data []T, k int, less func(a, b T) bool) []T

// BottomK returns the k smallest elements in data
func BottomK[T any](data []T, k int, less func(a, b T) bool) []T

// Select returns the element that would be at index k if data were sorted
func Select[T any](data []T, k int, less func(a, b T) bool) T

// MedianOfMedians finds an approximate median in linear time
func MedianOfMedians[T any](data []T, less func(a, b T) bool) T
```

### Utility Functions

```go
// IsSortedBy checks if a slice is sorted according to a key function
func IsSortedBy[T any, K cmp.Ordered](slice []T, key func(T) K) bool

// IsSortedFunc checks if a slice is sorted according to a comparison function
func IsSortedFunc[T any](slice []T, cmp func(a, b T) int) bool

// PartiallySort partially sorts data, guaranteeing elements up to k are in their final position
func PartiallySort[T any](data []T, k int, less func(a, b T) bool)

// BinarySearch performs a binary search for value in sorted data
func BinarySearch[T any](data []T, value T, cmp func(a, b T) int) (index int, found bool)

// InsertionPoint returns where value should be inserted to maintain order
func InsertionPoint[T any](data []T, value T, cmp func(a, b T) int) int
```

### Specialized Sorting Algorithms

```go
// Sorting algorithm implementations
func QuickSort[T any](data []T, less func(a, b T) bool)
func MergeSort[T any](data []T, less func(a, b T) bool)
func HeapSort[T any](data []T, less func(a, b T) bool)
func InsertionSort[T any](data []T, less func(a, b T) bool)
func ShellSort[T any](data []T, less func(a, b T) bool)
func RadixSort(data []int) // Only for integers
func CountingSort(data []int, min, max int) // Only for integers in a range
```

## Gen Z Sorting Features

```go
// VibeSort sorts data based on its "vibe" score
func VibeSort[T any](data []T, vibeScore func(T) float64)

// NoCapSort sorts data with absolute factual ordering
func NoCapSort[T any](data []T, factCheck func(a, b T) int)

// BussinSort sorts to highlight the most "bussin" (excellent) items first
func BussinSort[T any](data []T, bussinScore func(T) int)

// SlaySort is a high-performance sort that "slays" (excels at) sorting large datasets
func SlaySort[T any](data []T, less func(a, b T) bool)

// YeetSort quickly removes unwanted items and sorts the rest
func YeetSort[T any](data []T, keep func(T) bool, less func(a, b T) bool) []T
```

## Usage Example

```go
// Basic sorting
numbers := []int{3, 1, 4, 1, 5, 9, 2, 6}
sorta_fresh.SortInts(numbers)
vibez.spill(numbers) // [1 1 2 3 4 5 6 9]

// Sorting a slice of structs
type Person struct {
    Name string
    Age  int
}

people := []Person{
    {"Alice", 25},
    {"Bob", 20},
    {"Charlie", 30},
}

// Sort by age
sorta_fresh.SortSlice(people, func(i, j int) bool {
    return people[i].Age < people[j].Age
})

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
// Output: Bob 20, Alice 25, Charlie 30

// Sorting with a key function
sorta_fresh.SortBy(people, func(p Person) string {
    return p.Name
})

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
// Output: Alice 25, Bob 20, Charlie 30

// Multi-key sorting
people = []Person{
    {"Alice", 25},
    {"Bob", 20},
    {"Charlie", 25},
}

ms := sorta_fresh.OrderedBy(
    // Primary sort by age
    func(i, j int) bool { return people[i].Age < people[j].Age },
    // Secondary sort by name
    func(i, j int) bool { return people[i].Name < people[j].Name },
)
ms.Sort(people)

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
// Output: Bob 20, Alice 25, Charlie 25

// Using FreshSorter for advanced sorting
sorter := sorta_fresh.NewFreshSorterWithOptions(sorta_fresh.FreshSorterOptions{
    Parallel:     true,
    MaxGoroutines: 4,
    StableSorting: true,
})

largePeopleArray := make([]Person, 10000)
// ... fill array ...

sorter.SortSlice(largePeopleArray, func(i, j int) bool {
    return largePeopleArray[i].Age < largePeopleArray[j].Age
})

// Finding top-K elements
scores := []int{85, 92, 78, 95, 88, 76, 90, 94}
topThree := sorta_fresh.TopK(scores, 3, func(a, b int) bool {
    return a < b // Note: less function for ascending order
})
vibez.spill(topThree) // [94 95 92] or similar depending on implementation

// Selecting the median element
median := sorta_fresh.Select(scores, len(scores)/2, func(a, b int) bool {
    return a < b
})
vibez.spill(median) // 89 (approximate median value)

// Using Gen Z sorting features
people = []Person{
    {"Alice", 25},
    {"Bob", 20},
    {"Charlie", 30},
}

sorta_fresh.VibeSort(people, func(p Person) float64 {
    // Custom "vibe" scoring algorithm
    return float64(p.Age) * 0.8 // Younger people have better vibes in this example
})

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
// Output: Bob 20, Alice 25, Charlie 30

// Using YeetSort to filter and sort
filteredPeople := sorta_fresh.YeetSort(people, 
    // Keep only people older than 20
    func(p Person) bool { return p.Age > 20 },
    // Sort by name
    func(a, b Person) bool { return a.Name < b.Name }
)

for _, p := range filteredPeople {
    vibez.spill(p.Name, p.Age)
}
// Output: Alice 25, Charlie 30
```

## Implementation Guidelines
1. Optimize performance for common sorting scenarios
2. Implement efficient algorithms for different data types and sizes
3. Ensure stable sorting works correctly when requested
4. Support concurrent sorting for large datasets
5. Provide consistent interfaces across different sorting functions
6. Include comprehensive documentation with examples
7. Handle edge cases (empty slices, one-element slices, etc.)
8. Implement memory-efficient sorting for large datasets