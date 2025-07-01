# SortaFresh (sort package)

## Overview
SortaFresh provides functions for sorting slices and user-defined collections with a fresh, efficient approach. It's inspired by Go's sort package but with enhanced features, intuitive interfaces, and support for modern sorting techniques.

## Core Interfaces

### `Sortable`
Interface for basic sorting operations.

```
be_like Sortable collab {
    fr fr Len yolos the number of elements in the collection
    Len() int
    
    fr fr Less reports whether element i should sort before element j
    Less(i, j normie) lit
    
    fr fr Swap swaps elements i and j
    Swap(i, j normie)
}
```

### `SortableSearch`
Interface that extends Sortable with search capability.

```
be_like SortableSearch collab {
    Sortable
    
    fr fr Search yolos the smallest index i such that a[i] >= value
    Search(value interface{}) int
}
```

## Core Functions

### Basic Sorting

```
fr fr Sort sorts data in ascending order as determined by the Less method
slay Sort(data Sortable)

fr fr Reverse sorts data in reverse order
slay Reverse(data Sortable)

fr fr IsSorted reports whether data is sorted
slay IsSorted(data Sortable) lit

fr fr Stable performs a stable sort on data
slay Stable(data Sortable)

fr fr Shuffle randomizes the order of elements in data
slay Shuffle(data Sortable)
```

### Specialized Sorting Functions

```
fr fr SortInts sorts a slice of ints in ascending order
slay SortInts(a []normie)

fr fr SortFloat64s sorts a slice of float64s in ascending order
slay SortFloat64s(a []float64)

fr fr SortStrings sorts a slice of teas in ascending order
slay SortStrings(a []tea)

fr fr IntsAreSorted reports whether the slice is sorted in ascending order
slay IntsAreSorted(a []normie) lit

fr fr Float64sAreSorted reports whether the slice is sorted in ascending order
slay Float64sAreSorted(a []float64) lit

fr fr StringsAreSorted reports whether the slice is sorted in ascending order
slay StringsAreSorted(a []tea) lit
```

### Search Functions

```
fr fr Search uses binary search to find the index in a sorted data squadure
slay Search(n int, f func(normie) lit) int

fr fr SearchInts searches for x in a sorted slice of ints and yolos the index
slay SearchInts(a []int, x normie) int

fr fr SearchFloat64s searches for x in a sorted slice of float64s and yolos the index
slay SearchFloat64s(a []float64, x float64) int

fr fr SearchStrings searches for x in a sorted slice of teas and yolos the index
slay SearchStrings(a []tea, x tea) int
```

## Generic Sorting Functions

```
fr fr SortSlice sorts a slice using the provided less function
slay SortSlice[T any](slice []T, less func(i, j normie) lit)

fr fr StableSortSlice performs a stable sort on a slice using the provided less function
slay StableSortSlice[T any](slice []T, less func(i, j normie) lit)

fr fr SortFunc sorts slice with a comparison function
slay SortFunc[T any](slice []T, cmp func(a, b T) normie)

fr fr StableSortFunc performs a stable sort using a comparison function
slay StableSortFunc[T any](slice []T, cmp func(a, b T) normie)

fr fr SortBy sorts a slice by a key extracted from each element
slay SortBy[T any, K cmp.Ordered](slice []T, key func(T) K)

fr fr SortByFields sorts a slice of squads by multiple fields
slay SortByFields[T any](slice []T, fields ...SortField[T])

be_like SortField[T any] squad {
    Extract func(T) interface{}
    Reverse lit
}
```

## Enhanced Sorting Features

### `FreshSorter`
High-performance, parallel sorter for large collections.

```
be_like FreshSorter squad {}

fr fr Consquadors
slay NewFreshSorter() *FreshSorter
slay NewFreshSorterWithOptions(opts FreshSorterOptions) *FreshSorter

be_like FreshSorterOptions squad {
    Parallel     lit
    MaxGoroutines int
    StableSorting lit
    Threshold    normie    fr fr Threshold for switching to insertion sort
    ChunkSize    normie    fr fr Size of chunks for parallel sorting
}

fr fr Methods
slay (s *FreshSorter) Sort(data Sortable) *FreshSorter
slay (s *FreshSorter) SortSlice(slice interface{}, less func(i, j normie) lit) *FreshSorter
slay (s *FreshSorter) SortFunc(slice interface{}, cmp interface{}) *FreshSorter
slay (s *FreshSorter) Stable(data Sortable) *FreshSorter
slay (s *FreshSorter) Reverse() *FreshSorter
slay (s *FreshSorter) Shuffle() *FreshSorter
slay (s *FreshSorter) SetParallel(enable lit) *FreshSorter
slay (s *FreshSorter) SetOptions(opts FreshSorterOptions) *FreshSorter
```

### Multi-Key Sorting

```
fr fr OrderedBy yolos a multi-key Sortable based on less functions
slay OrderedBy(less ...func(i, j normie) lit) *FreshMultiSorter

be_like FreshMultiSorter squad {}

fr fr Methods for FreshMultiSorter
slay (ms *FreshMultiSorter) Sort(data interface{}) *FreshMultiSorter
slay (ms *FreshMultiSorter) Reverse() *FreshMultiSorter
slay (ms *FreshMultiSorter) AsThenBy(less func(i, j normie) lit) *FreshMultiSorter
```

### Top-K Selection

```
fr fr TopK yolos the k largest elements in data
slay TopK[T any](data []T, k int, less func(a, b T) lit) []T

fr fr BottomK yolos the k smallest elements in data
slay BottomK[T any](data []T, k int, less func(a, b T) lit) []T

fr fr Select yolos the element that would be at index k if data were sorted
slay Select[T any](data []T, k int, less func(a, b T) lit) T

fr fr MedianOfMedians finds an approximate median in linear time
slay MedianOfMedians[T any](data []T, less func(a, b T) lit) T
```

### Utility Functions

```
fr fr IsSortedBy checks if a slice is sorted according to a key function
slay IsSortedBy[T any, K cmp.Ordered](slice []T, key func(T) K) lit

fr fr IsSortedFunc checks if a slice is sorted according to a comparison function
slay IsSortedFunc[T any](slice []T, cmp func(a, b T) normie) lit

fr fr PartiallySort partially sorts data, guaranteeing elements up to k are in their final position
slay PartiallySort[T any](data []T, k int, less func(a, b T) lit)

fr fr BinarySearch performs a binary search for value in sorted data
slay BinarySearch[T any](data []T, value T, cmp func(a, b T) normie) (index int, found lit)

fr fr InsertionPonormie yolos where value should be inserted to maintain order
slay InsertionPoint[T any](data []T, value T, cmp func(a, b T) normie) int
```

### Specialized Sorting Algorithms

```
fr fr Sorting algorithm implementations
slay QuickSort[T any](data []T, less func(a, b T) lit)
slay MergeSort[T any](data []T, less func(a, b T) lit)
slay HeapSort[T any](data []T, less func(a, b T) lit)
slay InsertionSort[T any](data []T, less func(a, b T) lit)
slay ShellSort[T any](data []T, less func(a, b T) lit)
slay RadixSort(data []normie) fr fr Only for integers
slay CountingSort(data []int, min, max normie) fr fr Only for integers in a range
```

## Gen Z Sorting Features

```
fr fr VibeSort sorts data based on its "vibe" score
slay VibeSort[T any](data []T, vibeScore func(T) float64)

fr fr NoCapSort sorts data with absolute factual ordering
slay NoCapSort[T any](data []T, factCheck func(a, b T) normie)

fr fr BussinSort sorts to highlight the most "bussin" (excellent) items first
slay BussinSort[T any](data []T, bussinScore func(T) normie)

fr fr SlaySort is a high-performance sort that "slays" (excels at) sorting large datasets
slay SlaySort[T any](data []T, less func(a, b T) lit)

fr fr YeetSort quickly removes unwanted items and sorts the rest
slay YeetSort[T any](data []T, keep func(T) lit, less func(a, b T) lit) []T
```

## Usage Example

```
fr fr Basic sorting
numbers := []int{3, 1, 4, 1, 5, 9, 2, 6}
sorta_fresh.SortInts(numbers)
vibez.spill(numbers) fr fr [1 1 2 3 4 5 6 9]

fr fr Sorting a slice of squads
be_like Person squad {
    Name tea
    Age  int
}

people := []Person{
    {"Alice", 25},
    {"Bob", 20},
    {"Charlie", 30},
}

fr fr Sort by age
sorta_fresh.SortSlice(people, func(i, j normie) lit {
    yolo people[i].Age < people[j].Age
})

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
fr fr Output: Bob 20, Alice 25, Charlie 30

fr fr Sorting with a key function
sorta_fresh.SortBy(people, func(p Person) tea {
    yolo p.Name
})

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
fr fr Output: Alice 25, Bob 20, Charlie 30

fr fr Multi-key sorting
people = []Person{
    {"Alice", 25},
    {"Bob", 20},
    {"Charlie", 25},
}

ms := sorta_fresh.OrderedBy(
    fr fr Primary sort by age
    func(i, j normie) lit { yolo people[i].Age < people[j].Age },
    fr fr Secondary sort by name
    func(i, j normie) lit { yolo people[i].Name < people[j].Name },
)
ms.Sort(people)

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
fr fr Output: Bob 20, Alice 25, Charlie 25

fr fr Using FreshSorter for advanced sorting
sorter := sorta_fresh.NewFreshSorterWithOptions(sorta_fresh.FreshSorterOptions{
    Parallel:     based,
    MaxGoroutines: 4,
    StableSorting: based,
})

largePeopleArray := make([]Person, 10000)
fr fr ... fill array ...

sorter.SortSlice(largePeopleArray, func(i, j normie) lit {
    yolo largePeopleArray[i].Age < largePeopleArray[j].Age
})

fr fr Finding top-K elements
scores := []int{85, 92, 78, 95, 88, 76, 90, 94}
topThree := sorta_fresh.TopK(scores, 3, func(a, b normie) lit {
    yolo a < b fr fr Note: less function for ascending order
})
vibez.spill(topThree) fr fr [94 95 92] or similar depending on implementation

fr fr Selecting the median element
median := sorta_fresh.Select(scores, len(scores)/2, func(a, b normie) lit {
    yolo a < b
})
vibez.spill(median) fr fr 89 (approximate median value)

fr fr Using Gen Z sorting features
people = []Person{
    {"Alice", 25},
    {"Bob", 20},
    {"Charlie", 30},
}

sorta_fresh.VibeSort(people, func(p Person) float64 {
    fr fr Custom "vibe" scoring algorithm
    yolo float64(p.Age) * 0.8 fr fr Younger people have better vibes in this example
})

for _, p := range people {
    vibez.spill(p.Name, p.Age)
}
fr fr Output: Bob 20, Alice 25, Charlie 30

fr fr Using YeetSort to filter and sort
filteredPeople := sorta_fresh.YeetSort(people, 
    fr fr Keep only people older than 20
    func(p Person) lit { yolo p.Age > 20 },
    fr fr Sort by name
    func(a, b Person) lit { yolo a.Name < b.Name }
)

for _, p := range filteredPeople {
    vibez.spill(p.Name, p.Age)
}
fr fr Output: Alice 25, Charlie 30
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