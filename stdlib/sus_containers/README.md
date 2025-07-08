# SusContainers Module

Data structures for storing collections with specialized access patterns, inspired by Go's container package with suspicious efficiency.

## Features

- **SusHeap**: Priority queue implementation with heap interface
- **SusList**: Doubly-linked list with O(1) operations
- **SusRing**: Circular list with efficient rotation and linking

## Components

### SusHeap
Provides heap operations through the SusHeapInterface:
- `Init(h)` - Initialize heap
- `Push(h, x)` - Add element maintaining heap property
- `Pop(h)` - Remove and return minimum/maximum element
- `Remove(h, i)` - Remove element at index i
- `Fix(h, i)` - Re-establish heap property after element change

### SusList
Doubly-linked list with efficient insertion/removal:
- `PushFront(v)` - Add element to front
- `PushBack(v)` - Add element to back  
- `Remove(e)` - Remove specific node
- `Len()` - Get list length

### SusRing
Circular list for efficient rotation:
- `Move(n)` - Move n positions in ring
- `Link(s)` - Link with another ring
- `Unlink(n)` - Remove n elements
- `Do(f)` - Apply function to all elements

## Usage Examples

```cursed
// Create and use a list
sus list := sus_containers.NewSusList()
sus node1 := list.PushBack("first")
sus node2 := list.PushFront("second")
vibez.spill("List length:", list.Len())

// Create a ring
sus ring := sus_containers.NewSusRing(5)
ring.Value = "start"
sus moved := ring.Move(2)
moved.Value = "moved"
```
