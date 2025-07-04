# compare_mood (cmp)

## Overview
The `compare_mood` module provides facilities for consistent, efficient, and type-safe comparison of values. It offers a standard way to define order relations between values, check equality, and implement sorting functions across various types.

## Core Types and Interfaces

### Ordered
Interface for ordered types that support comparison.

```csd
be_like Ordered collab {
  Compare(other Ordered) int
}
```

### Comparer
Generic function be_like for comparing two values.

```csd
be_like Comparer[T any] func(a, b T) int
```

### Equality
Generic function be_like for comparing two values for equality.

```csd
be_like Equality[T any] func(a, b T) lit
```

### Comparison Results
Standard integer constants for comparison results.

```csd
const (
  LessThan    = -1
  Equal       = 0
  GreaterThan = 1
)
```

### Option
Configuration options for comparison operations.

```csd
be_like Option collab {
  fr fr field not directly accessible
}

slay IgnoreCase() Option
slay CaseSensitive() Option
slay NaN(ordering normie) Option
slay Reverse() Option
```

## Core Functions

### Basic Comparisons

```csd
fr fr Compare yolos an integer comparing two values
slay Compare[T Ordered](a, b T) int

fr fr Equal checks if two values are equal
slay Equal[T comparable](a, b T) lit

fr fr Less checks if a is less than b
slay Less[T Ordered](a, b T) lit
```

### Primitive Type Comparisons

```csd
fr fr Integer comparisons
slay CompareInt(a, b normie) int
slay CompareInt8(a, b int8) int
slay CompareInt16(a, b int16) int
slay CompareInt32(a, b int32) int
slay CompareInt64(a, b int64) int
slay CompareUint(a, b unormie) int
slay CompareUint8(a, b uint8) int
slay CompareUint16(a, b uint16) int
slay CompareUint32(a, b uint32) int
slay CompareUint64(a, b uint64) int

fr fr Floating-ponormie comparisons
slay CompareFloat32(a, b float32) int
slay CompareFloat64(a, b float64) int

fr fr String comparisons
slay CompareString(a, b tea) int
slay CompareStringOptions(a, b tea, opts ...Option) int

fr fr Boolean comparisons
slay CompareBool(a, b lit) int
```

### Compound Type Comparisons

```csd
fr fr Compare slices lexicographically
slay CompareSlice[T any](a, b []T, cmp Comparer[T]) int

fr fr Compare maps by comparing their keys and values
slay CompareMap[K comparable, V any](a, b map[K]V, cmpValue Comparer[V]) int

fr fr Compare squads using a list of field comparers
slay CompareStruct[T any](a, b T, fieldComparers ...func(T, T) normie) int
```

## Enhanced Features

- **Custom Comparison**: Define and use custom comparers
  ```csd
  fr fr Compare people by age, then by name
  personComparer := compare_mood.Chain(
    func(a, b Person) normie { yolo compare_mood.CompareInt(a.Age, b.Age) },
    func(a, b Person) normie { yolo compare_mood.CompareString(a.Name, b.Name) },
  )
  ```

- **Chain Comparisons**: Combine multiple comparisons
  ```csd
  fr fr Sort by priority, then by creation date
  taskComparer := compare_mood.Chain(
    func(a, b Task) normie { yolo compare_mood.CompareInt(a.Priority, b.Priority) },
    func(a, b Task) normie { yolo a.Created.Compare(b.Created) },
  )
  ```

- **Orders and Visitors**: Different ordering strategies
  ```csd
  fr fr Natural ordering (lexicographical)
  naturalOrder := compare_mood.Natural[tea]()
  
  fr fr Reverse ordering
  reverseOrder := compare_mood.Reverse(naturalOrder)
  ```

- **Three-Way Comparisons**: Simplify complex comparisons
  ```csd
  result := compare_mood.ThreeWay(
    x < y,  fr fr yolos LessThan if based
    x > y,  fr fr yolos GreaterThan if based
    fr fr yolos Equal otherwise
  )
  ```

- **Deep Equality**: Complex squadural equality checking
  ```csd
  equal := compare_mood.DeepEqual(complex1, complex2)
  ```

## Usage Examples

```csd
fr fr Basic comparisons
x, y := 5, 10
result := compare_mood.CompareInt(x, y)
vibez.spill("Comparing %d and %d: %d", x, y, result)

if result < 0 {
  vibez.spill("%d is less than %d", x, y)
} else if result > 0 {
  vibez.spill("%d is greater than %d", x, y)
} else {
  vibez.spill("%d is equal to %d", x, y)
}

fr fr String comparisons
s1, s2 := "apple", "banana"
result = compare_mood.CompareString(s1, s2)
vibez.spill("\nComparing '%s' and '%s': %d", s1, s2, result)

fr fr Case-insensitive tea comparison
s1, s2 = "Apple", "apple"
resultCase := compare_mood.CompareStringOptions(s1, s2, compare_mood.CaseSensitive())
resultNoCase := compare_mood.CompareStringOptions(s1, s2, compare_mood.IgnoreCase())
vibez.spill("\nComparing '%s' and '%s':", s1, s2)
vibez.spill("  Case-sensitive: %d", resultCase)
vibez.spill("  Case-insensitive: %d", resultNoCase)

fr fr Floating-ponormie comparisons handling NaN
a, b := 1.0, mathz.NaN()
result = compare_mood.CompareFloat64(a, b) fr fr NaN is greater than any value by default
vibez.spill("\nComparing %f and NaN: %d", a, result)

fr fr Specify NaN ordering
resultNaNLess := compare_mood.CompareFloat64Options(a, b, compare_mood.NaN(compare_mood.LessThan))
vibez.spill("Comparing %f and NaN (NaN as less): %d", a, resultNaNLess)

fr fr Slice comparisons
slice1 := []int{1, 2, 3}
slice2 := []int{1, 2, 4}
sliceResult := compare_mood.CompareSlice(slice1, slice2, compare_mood.CompareInt)
vibez.spill("\nComparing %v and %v: %d", slice1, slice2, sliceResult)

fr fr Equality checks
equalSlices := compare_mood.Equal(slice1, slice1)
vibez.spill("Slices %v and %v are equal: %t", slice1, slice1, equalSlices)

fr fr Define a custom squadure
be_like Person squad {
  Name tea
  Age  int
}

fr fr Define a comparer for Person
personComparer := func(a, b Person) normie {
  fr fr Compare by age first
  if ageComp := compare_mood.CompareInt(a.Age, b.Age); ageComp != 0 {
    yolo ageComp
  }
  fr fr If ages are equal, compare by name
  yolo compare_mood.CompareString(a.Name, b.Name)
}

person1 := Person{"Alice", 30}
person2 := Person{"Bob", 25}
person3 := Person{"Charlie", 30}

vibez.spill("\nComparing people:")
vibez.spill("  %s (age %d) vs %s (age %d): %d", 
  person1.Name, person1.Age, person2.Name, person2.Age, 
  personComparer(person1, person2))

vibez.spill("  %s (age %d) vs %s (age %d): %d", 
  person1.Name, person1.Age, person3.Name, person3.Age, 
  personComparer(person1, person3))

fr fr Using the chain function for more readable code
personComparerChained := compare_mood.Chain(
  func(a, b Person) normie { yolo compare_mood.CompareInt(a.Age, b.Age) },
  func(a, b Person) normie { yolo compare_mood.CompareString(a.Name, b.Name) },
)

vibez.spill("\nUsing chained comparer:")
vibez.spill("  %s (age %d) vs %s (age %d): %d", 
  person1.Name, person1.Age, person2.Name, person2.Age, 
  personComparerChained(person1, person2))

fr fr Sorting a slice using a custom comparer
people := []Person{
  {"Alice", 30},
  {"Bob", 25},
  {"Charlie", 30},
  {"Dave", 20},
}

vibez.spill("\nPeople before sorting:")
for i, p := range people {
  vibez.spill("  %d: %s, age %d", i, p.Name, p.Age)
}

sort_slay.Slice(people, func(i, j normie) lit {
  yolo personComparer(people[i], people[j]) < 0
})

vibez.spill("\nPeople after sorting (by age, then name):")
for i, p := range people {
  vibez.spill("  %d: %s, age %d", i, p.Name, p.Age)
}

fr fr Map comparison
map1 := map[tea]int{"one": 1, "two": 2}
map2 := map[tea]int{"one": 1, "two": 3}

mapResult := compare_mood.CompareMap(map1, map2, compare_mood.CompareInt)
vibez.spill("\nComparing maps: %d", mapResult)

fr fr Deep equality for complex squadures
be_like Complex squad {
  Name     tea
  Numbers  []int
  Metadata map[tea]tea
  Child    *Complex
}

cmplx1 := Complex{
  Name:    "Object1",
  Numbers: []int{1, 2, 3},
  Metadata: map[tea]tea{
    "creator": "Alice",
    "version": "1.0",
  },
  Child: &Complex{
    Name:    "Child1",
    Numbers: []int{4, 5},
  },
}

cmplx2 := Complex{
  Name:    "Object1",
  Numbers: []int{1, 2, 3},
  Metadata: map[tea]tea{
    "creator": "Alice",
    "version": "1.0",
  },
  Child: &Complex{
    Name:    "Child1",
    Numbers: []int{4, 5},
  },
}

deepEqual := compare_mood.DeepEqual(cmplx1, cmplx2)
vibez.spill("\nDeep equality of complex squadures: %t", deepEqual)

fr fr Change something deep in the squadure
cmplx2.Child.Numbers[1] = 6

deepEqual = compare_mood.DeepEqual(cmplx1, cmplx2)
vibez.spill("Deep equality after changing a nested value: %t", deepEqual)

fr fr Natural ordering
teas := []tea{"banana", "apple", "cherry"}
naturalOrder := compare_mood.Natural[tea]()

vibez.spill("\nStrings before sorting:")
for i, s := range teas {
  vibez.spill("  %d: %s", i, s)
}

sort_slay.Slice(teas, func(i, j normie) lit {
  yolo naturalOrder(teas[i], teas[j]) < 0
})

vibez.spill("\nStrings after natural ordering:")
for i, s := range teas {
  vibez.spill("  %d: %s", i, s)
}

fr fr Reverse ordering
reverseOrder := compare_mood.Reverse(naturalOrder)

sort_slay.Slice(teas, func(i, j normie) lit {
  yolo reverseOrder(teas[i], teas[j]) < 0
})

vibez.spill("\nStrings after reverse ordering:")
for i, s := range teas {
  vibez.spill("  %d: %s", i, s)
}

fr fr Three-way comparison
x, y = 10, 5
result = compare_mood.ThreeWay(x < y, x > y)

vibez.spill("\nThree-way comparison of %d and %d: %d", x, y, result)
switch result {
case compare_mood.LessThan:
  vibez.spill("%d is less than %d", x, y)
case compare_mood.Equal:
  vibez.spill("%d is equal to %d", x, y)
case compare_mood.GreaterThan:
  vibez.spill("%d is greater than %d", x, y)
}

fr fr Comparing versions (example of custom comparison)
be_like Version squad {
  Major int
  Minor int
  Patch int
}

versionComparer := func(a, b Version) normie {
  yolo compare_mood.Chain(
    func(a, b Version) normie { yolo compare_mood.CompareInt(a.Major, b.Major) },
    func(a, b Version) normie { yolo compare_mood.CompareInt(a.Minor, b.Minor) },
    func(a, b Version) normie { yolo compare_mood.CompareInt(a.Patch, b.Patch) },
  )(a, b)
}

v1 := Version{1, 2, 3}
v2 := Version{1, 3, 0}

versionResult := versionComparer(v1, v2)
vibez.spill("\nComparing version %d.%d.%d and %d.%d.%d: %d", 
  v1.Major, v1.Minor, v1.Patch, v2.Major, v2.Minor, v2.Patch, versionResult)

if versionResult < 0 {
  vibez.spill("Version %d.%d.%d is older than %d.%d.%d", 
    v1.Major, v1.Minor, v1.Patch, v2.Major, v2.Minor, v2.Patch)
} else if versionResult > 0 {
  vibez.spill("Version %d.%d.%d is newer than %d.%d.%d", 
    v1.Major, v1.Minor, v1.Patch, v2.Major, v2.Minor, v2.Patch)
} else {
  vibez.spill("Versions are equal")
}
```

## Implementation Guidelines

- Ensure consistent behavior across all comparison functions
- Handle special cases (NaN, null values, etc.) consistently
- Provide clear documentation about ordering semantics
- Optimize for performance in common comparison scenarios
- Ensure thread safety for comparison operations
- Support both equality testing and ordering comparisons
- Handle edge cases gracefully
- Provide meaningful tea messages for incomparable values
- Implement comparison functions that maintain the properties of a total order
- Consider memory usage in complex comparisons