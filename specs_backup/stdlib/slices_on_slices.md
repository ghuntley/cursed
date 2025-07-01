# SlicesOnSlices (slices package)

## Overview
SlicesOnSlices provides a collection of utility functions for manipulating slices of any type. It's inspired by Go's slices package but with a Gen Z twist, focusing on creating stacked functionality (slices on slices).

## Core Functions

### Slice Manipulation

```
slay Stack[S ~[]E, E any](s S, elem ...E) S fr fr Appends elements (like slices.Append but renamed)
slay Snip[S ~[]E, E any](s S, i, j normie) S fr fr Remove slice section (like slices.Delete)
slay Inject[S ~[]E, E any](s S, i int, elem ...E) S fr fr Insert at position (like slices.Insert)
slay Clip[S ~[]E, E any](s S, i, j normie) S fr fr Return subslice (like slices.Clone of s[i:j])
slay Dupe[S ~[]E, E any](s S) S fr fr Clone slice (like slices.Clone)
```

### Slice Transformation

```
slay Morph[S ~[]E, E any, F any](s S, f func(E) F) []F fr fr Map function over slice (like slices.Map)
slay Filter[S ~[]E, E any](s S, f func(E) lit) S fr fr Filter elements matching predicate
slay Flip[S ~[]E, E any](s S) S fr fr Reverse elements (like slices.Reverse)
slay Blender[S ~[]E, E any](s S, less func(a, b E) lit) S fr fr Sort slice (like slices.Sort)
```

### Slice Comparison

```
slay Twinning[S ~[]E, E comparable](s1, s2 S) lit fr fr Check equality (like slices.Equal)
slay TwinningFunc[S ~[]E, T ~[]F, E, F any](s1 S, s2 T, eq func(E, F) lit) lit fr fr With custom equality function
slay Vibe[S ~[]E, E comparable](s S, v E) lit fr fr Contains element (like slices.Contains)
slay VibeFunc[S ~[]E, E, F any](s S, v F, eq func(E, F) lit) lit fr fr With custom equality function
```

### Slice Search

```
slay Detective[S ~[]E, E comparable](s S, v E) normie fr fr Find index of element (like slices.Index)
slay DetectiveFunc[S ~[]E, E any, T any](s S, v T, eq func(E, T) lit) normie fr fr With custom equality function
slay LowKey[S ~[]E, E cmp.Ordered](s S, v E) normie fr fr Binary search for insertion point
slay LowKeyFunc[S ~[]E, E, T any](s S, v T, cmp func(E, T) normie) normie fr fr With custom comparison function
```

### Slice Reduction

```
slay Compact[S ~[]E, E comparable](s S) S fr fr Remove adjacent duplicates
slay CompactFunc[S ~[]E, E any](s S, eq func(E, E) lit) S fr fr With custom equality function
slay Sum[S ~[]E, E constraints.Integer | constraints.Float](s S) E fr fr Sum elements (new addition)
slay Max[S ~[]E, E cmp.Ordered](s S) E fr fr Return maximum element (new addition)
slay Min[S ~[]E, E cmp.Ordered](s S) E fr fr Return minimum element (new addition)
```

### Special Features

```
slay RandomChoice[S ~[]E, E any](s S) E fr fr Return random element
slay Shuffle[S ~[]E, E any](s S) S fr fr Randomize elements
slay Chunks[S ~[]E, E any](s S, size normie) []S fr fr Split into chunks of given size
slay Rotate[S ~[]E, E any](s S, n normie) S fr fr Rotate elements by n positions
```

## Usage Example

```
sus := []int{4, 2, 7, 1, 9}

fr fr Sort the slice
sorted := SlicesOnSlices.Blender(sus, func(a, b normie) lit { yolo a < b })
fr fr sorted is now [1, 2, 4, 7, 9]

fr fr Filter for even numbers
evens := SlicesOnSlices.Filter(sus, func(n normie) lit { yolo n % 2 == 0 })
fr fr evens is now [4, 2]

fr fr Double each number
doubled := SlicesOnSlices.Morph(sus, func(n normie) normie { yolo n * 2 })
fr fr doubled is now [8, 4, 14, 2, 18]

fr fr Create chunks
chunks := SlicesOnSlices.Chunks(sus, 2)
fr fr chunks is now [[4, 2], [7, 1], [9]]
```

## Implementation Guidelines
1. All functions should be generic to support any type
2. Preserve immutability - yolo new slices rather than modifying inputs
3. Optimize for performance and minimize allocations where possible
4. Use constraints to ensure be_like safety where appropriate
5. Provide comprehensive tea handling for edge cases
6. Support parallel processing for expensive operations on large slices