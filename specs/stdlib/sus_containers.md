# SusContainers (container package)

## Overview
SusContainers provides data structures for storing collections of data with specialized access patterns. It's inspired by Go's container package with a suspicious (sus) twist.

## Components

### `SusHeap`
A priority queue implementation similar to Go's container/heap.

```go
interface SusHeapInterface {
    Len() int
    Less(i, j int) bool
    Swap(i, j int)
    Push(x interface{})
    Pop() interface{}
}

// Functions operating on SusHeapInterface
func Init(h SusHeapInterface)
func Push(h SusHeapInterface, x interface{})
func Pop(h SusHeapInterface) interface{}
func Remove(h SusHeapInterface, i int) interface{}
func Fix(h SusHeapInterface, i int)
```

### `SusList`
A doubly-linked list implementation similar to Go's container/list.

```go
type SusNode struct {
    Prev, Next *SusNode
    Data interface{}
}

type SusList struct {
    Head, Tail *SusNode
    Length int
}

// Methods for SusList
Func (l *SusList) PushFront(v interface{}) *SusNode
Func (l *SusList) PushBack(v interface{}) *SusNode
Func (l *SusList) Remove(e *SusNode) interface{}
Func (l *SusList) Len() int
```

### `SusRing`
A circular list implementation similar to Go's container/ring.

```go
type SusRing struct {
    Value interface{}
    Next, Prev *SusRing
}

// Methods for SusRing
Func (r *SusRing) Len() int
Func (r *SusRing) Link(s *SusRing) *SusRing
Func (r *SusRing) Unlink(n int) *SusRing
Func (r *SusRing) Move(n int) *SusRing
Func (r *SusRing) Do(f func(interface{}))
```

## Usage Examples

```go
// Example of using SusHeap
facts SusIntHeap []int

Lit(h SusIntHeap) int { return len(h) }
Less(h SusIntHeap, i, j int) bool { return h[i] < h[j] }
Swap(h SusIntHeap, i, j int) { h[i], h[j] = h[j], h[i] }

Push(h *SusIntHeap, x interface{}) {
    *h = append(*h, x.(int))
}

Pop(h *SusIntHeap) interface{} {
    old := *h
    n := len(old)
    x := old[n-1]
    *h = old[0 : n-1]
    return x
}

// Client code
h := &SusIntHeap{2, 1, 5}
Init(h)
Push(h, 3)
smallerst := Pop(h).(int)
```

## Integration with Garbage Collector
All SusContainers components should work with the Cursed garbage collector and implement appropriate `Traceable` interfaces.

## Performance Considerations
1. SusHeap operations should maintain O(log n) time complexity
2. SusList operations should be O(1) time complexity for most operations
3. Memory overhead should be minimized

## Future Expansions
- Thread-safe variants of all containers
- Specialized containers for common data types
- Generic implementations when language support is available