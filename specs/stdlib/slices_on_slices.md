# SlicesOnSlices (slices package)

## Overview
SlicesOnSlices provides a collection of utility functions for manipulating slices of any type. It's inspired by Go's slices package but with a Gen Z twist, focusing on creating stacked functionality (slices on slices).

## Core Functions

### Slice Manipulation

```go
func Stack[S ~[]E, E any](s S, elem ...E) S // Appends elements (like slices.Append but renamed)
func Snip[S ~[]E, E any](s S, i, j int) S // Remove slice section (like slices.Delete)
func Inject[S ~[]E, E any](s S, i int, elem ...E) S // Insert at position (like slices.Insert)
func Clip[S ~[]E, E any](s S, i, j int) S // Return subslice (like slices.Clone of s[i:j])
func Dupe[S ~[]E, E any](s S) S // Clone slice (like slices.Clone)
```

### Slice Transformation

```go
func Morph[S ~[]E, E any, F any](s S, f func(E) F) []F // Map function over slice (like slices.Map)
func Filter[S ~[]E, E any](s S, f func(E) bool) S // Filter elements matching predicate
func Flip[S ~[]E, E any](s S) S // Reverse elements (like slices.Reverse)
func Blender[S ~[]E, E any](s S, less func(a, b E) bool) S // Sort slice (like slices.Sort)
```

### Slice Comparison

```go
func Twinning[S ~[]E, E comparable](s1, s2 S) bool // Check equality (like slices.Equal)
func TwinningFunc[S ~[]E, T ~[]F, E, F any](s1 S, s2 T, eq func(E, F) bool) bool // With custom equality function
func Vibe[S ~[]E, E comparable](s S, v E) bool // Contains element (like slices.Contains)
func VibeFunc[S ~[]E, E, F any](s S, v F, eq func(E, F) bool) bool // With custom equality function
```

### Slice Search

```go
func Detective[S ~[]E, E comparable](s S, v E) int // Find index of element (like slices.Index)
func DetectiveFunc[S ~[]E, E any, T any](s S, v T, eq func(E, T) bool) int // With custom equality function
func LowKey[S ~[]E, E cmp.Ordered](s S, v E) int // Binary search for insertion point
func LowKeyFunc[S ~[]E, E, T any](s S, v T, cmp func(E, T) int) int // With custom comparison function
```

### Slice Reduction

```go
func Compact[S ~[]E, E comparable](s S) S // Remove adjacent duplicates
func CompactFunc[S ~[]E, E any](s S, eq func(E, E) bool) S // With custom equality function
func Sum[S ~[]E, E constraints.Integer | constraints.Float](s S) E // Sum elements (new addition)
func Max[S ~[]E, E cmp.Ordered](s S) E // Return maximum element (new addition)
func Min[S ~[]E, E cmp.Ordered](s S) E // Return minimum element (new addition)
```

### Special Features

```go
func RandomChoice[S ~[]E, E any](s S) E // Return random element
func Shuffle[S ~[]E, E any](s S) S // Randomize elements
func Chunks[S ~[]E, E any](s S, size int) []S // Split into chunks of given size
func Rotate[S ~[]E, E any](s S, n int) S // Rotate elements by n positions
```

## Usage Example

```go
sus := []int{4, 2, 7, 1, 9}

// Sort the slice
sorted := SlicesOnSlices.Blender(sus, func(a, b int) bool { return a < b })
// sorted is now [1, 2, 4, 7, 9]

// Filter for even numbers
evens := SlicesOnSlices.Filter(sus, func(n int) bool { return n % 2 == 0 })
// evens is now [4, 2]

// Double each number
doubled := SlicesOnSlices.Morph(sus, func(n int) int { return n * 2 })
// doubled is now [8, 4, 14, 2, 18]

// Create chunks
chunks := SlicesOnSlices.Chunks(sus, 2)
// chunks is now [[4, 2], [7, 1], [9]]
```

## Implementation Guidelines
1. All functions should be generic to support any type
2. Preserve immutability - return new slices rather than modifying inputs
3. Optimize for performance and minimize allocations where possible
4. Use constraints to ensure type safety where appropriate
5. Provide comprehensive error handling for edge cases
6. Support parallel processing for expensive operations on large slices