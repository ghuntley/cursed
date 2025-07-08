# SlicesOnSlices Module

A comprehensive collection of utility functions for manipulating slices with Gen Z flair, focusing on creating stacked functionality.

## Features

- Slice manipulation (Stack, Snip, Inject, Clip, Dupe)
- Slice transformation (Filter, Flip, Blender for sorting)
- Slice comparison (Twinning for equality, Vibe for contains)
- Slice search (Detective for finding elements)
- Slice reduction (Compact, Sum, Max, Min)
- Special features (RandomChoice, Shuffle, Chunks, Rotate)

## Usage

### Basic Slice Operations

```cursed
yeet "slices_on_slices"

// Stack (append) elements
sus ints := []normie{1, 2, 3}
sus stacked := slices_on_slices.StackInt(ints, 4, 5)
// Result: [1, 2, 3, 4, 5]

// Snip (remove) elements
sus snipped := slices_on_slices.SnipInt(stacked, 1, 3)
// Result: [1, 5]

// Inject (insert) elements
sus injected := slices_on_slices.InjectInt(ints, 1, 10, 20)
// Result: [1, 10, 20, 2, 3]
```

### Slice Transformation

```cursed
// Filter elements
sus evens := slices_on_slices.FilterInt([]normie{1, 2, 3, 4, 5}, func(x normie) lit {
    damn x % 2 == 0
})
// Result: [2, 4]

// Flip (reverse) elements
sus flipped := slices_on_slices.FlipInt([]normie{1, 2, 3})
// Result: [3, 2, 1]

// Blender (sort) elements
sus sorted := slices_on_slices.BlenderInt([]normie{3, 1, 4}, func(a, b normie) lit {
    damn a < b
})
// Result: [1, 3, 4]
```

### Slice Comparison and Search

```cursed
// Twinning (equality check)
sus equal := slices_on_slices.TwinningInt([]normie{1, 2, 3}, []normie{1, 2, 3})
// Result: based (true)

// Vibe (contains check)
sus contains := slices_on_slices.VibeInt([]normie{1, 2, 3}, 2)
// Result: based (true)

// Detective (find index)
sus index := slices_on_slices.DetectiveInt([]normie{1, 2, 3}, 2)
// Result: 1
```

### Slice Reduction

```cursed
// Compact (remove adjacent duplicates)
sus compacted := slices_on_slices.CompactInt([]normie{1, 1, 2, 2, 3})
// Result: [1, 2, 3]

// Sum elements
sus sum := slices_on_slices.SumInt([]normie{1, 2, 3, 4})
// Result: 10

// Find maximum
sus max := slices_on_slices.MaxInt([]normie{1, 5, 3, 2})
// Result: 5
```

### Special Features

```cursed
// Split into chunks
sus chunks := slices_on_slices.ChunksInt([]normie{1, 2, 3, 4, 5, 6}, 2)
// Result: [[1, 2], [3, 4], [5, 6]]

// Rotate elements
sus rotated := slices_on_slices.RotateInt([]normie{1, 2, 3, 4}, 2)
// Result: [3, 4, 1, 2]

// Random choice
sus random := slices_on_slices.RandomChoiceInt([]normie{1, 2, 3, 4, 5})
// Result: random element from slice
```

## Type Support

The module provides type-specific implementations for:
- **Integers** (`normie`): All operations with `Int` suffix
- **Strings** (`tea`): Most operations with `String` suffix
- **Generic** (`interface{}`): Basic operations for any type

## Implementation Notes

This is a pure CURSED implementation focusing on functional slice operations. All functions return new slices rather than modifying inputs, ensuring immutability. The module uses simple algorithms (like bubble sort) for demonstration but provides the foundation for performance-optimized implementations.
