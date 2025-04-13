# SusContainers (container package)

## Overview
SusContainers provides data squadures for storing collections of data with specialized access patterns. It's inspired by Go's container package with a suspicious (sus) twist.

## Components

### `SusHeap`
A priority queue implementation similar to Go's container/heap.

```
collab SusHeapInterface {
    Len() int
    Less(i, j normie) lit
    Swap(i, j normie)
    Push(x interface{})
    Pop() interface{}
}

fr fr Functions operating on SusHeapInterface
slay Init(h SusHeapInterface)
slay Push(h SusHeapInterface, x interface{})
slay Pop(h SusHeapInterface) interface{}
slay Remove(h SusHeapInterface, i normie) interface{}
slay Fix(h SusHeapInterface, i normie)
```

### `SusList`
A doubly-linked list implementation similar to Go's container/list.

```
be_like SusNode squad {
    Prev, Next *SusNode
    Data interface{}
}

be_like SusList squad {
    Head, Tail *SusNode
    Length int
}

fr fr Methods for SusList
Func (l *SusList) PushFront(v interface{}) *SusNode
Func (l *SusList) PushBack(v interface{}) *SusNode
Func (l *SusList) Remove(e *SusNode) interface{}
Func (l *SusList) Len() int
```

### `SusRing`
A circular list implementation similar to Go's container/ring.

```
be_like SusRing squad {
    Value interface{}
    Next, Prev *SusRing
}

fr fr Methods for SusRing
Func (r *SusRing) Len() int
Func (r *SusRing) Link(s *SusRing) *SusRing
Func (r *SusRing) Unlink(n normie) *SusRing
Func (r *SusRing) Move(n normie) *SusRing
Func (r *SusRing) Do(f func(interface{}))
```

## Usage Examples

```
fr fr Example of using SusHeap
facts SusIntHeap []int

Lit(h SusIntHeap) normie { yolo len(h) }
Less(h SusIntHeap, i, j normie) lit { yolo h[i] < h[j] }
Swap(h SusIntHeap, i, j normie) { h[i], h[j] = h[j], h[i] }

Push(h *SusIntHeap, x interface{}) {
    *h = append(*h, x.(normie))
}

Pop(h *SusIntHeap) interface{} {
    old := *h
    n := len(old)
    x := old[n-1]
    *h = old[0 : n-1]
    yolo x
}

fr fr Client code
h := &SusIntHeap{2, 1, 5}
Init(h)
Push(h, 3)
smallerst := Pop(h).(normie)
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