# heap_slay (container/heap)

## Overview
The `heap_slay` module provides an implementation of the heap (priority queue) data structure. It supports insertion, removal, and extraction of the minimum or maximum element in logarithmic time. This implementation uses the interface approach, making it flexible for various element types.

## Core Types and Interfaces

### Interface
This interface must be implemented by any type that wants to use heap operations.

```csd
type Interface interface {
  // Len returns the length of the collection.
  Len() int
  
  // Less reports whether element i should sort before element j.
  Less(i, j int) bool
  
  // Swap swaps the elements with indexes i and j.
  Swap(i, j int)
  
  // Push adds an element to the collection.
  Push(x interface{})
  
  // Pop removes and returns the last element (highest index).
  Pop() interface{}
}
```

## Core Functions

```csd
// Establish heap ordering in the given slice.
func Init(h Interface)

// Push an element onto the heap and restore ordering.
func Push(h Interface, x interface{})

// Pop the minimum element from the heap and restore ordering.
func Pop(h Interface) interface{}

// Remove the element at index i from the heap and restore ordering.
func Remove(h Interface, i int) interface{}

// Fix re-establishes the heap ordering after element i's value has changed.
func Fix(h Interface, i int)

// Check if a heap is properly ordered.
func IsHeap(h Interface) bool
```

## Convenience Types

### IntHeap
A heap implementation for integers.

```csd
type IntHeap []int

func (h IntHeap) Len() int
func (h IntHeap) Less(i, j int) bool
func (h IntHeap) Swap(i, j int)
func (h *IntHeap) Push(x interface{})
func (h *IntHeap) Pop() interface{}
```

### StringHeap
A heap implementation for strings.

```csd
type StringHeap []string

func (h StringHeap) Len() int
func (h StringHeap) Less(i, j int) bool
func (h StringHeap) Swap(i, j int)
func (h *StringHeap) Push(x interface{})
func (h *StringHeap) Pop() interface{}
```

### PriorityQueue
A generic priority queue implementation with customizable priorities.

```csd
type Item struct {
  Value    interface{}
  Priority int
  Index    int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int
func (pq PriorityQueue) Less(i, j int) bool
func (pq PriorityQueue) Swap(i, j int)
func (pq *PriorityQueue) Push(x interface{})
func (pq *PriorityQueue) Pop() interface{}
func (pq *PriorityQueue) Update(item *Item, value interface{}, priority int)
```

## Enhanced Features

- **Generic Heap**: Type-safe heap with generics
  ```csd
  heap := heap_slay.NewHeap[int](func(a, b int) bool { return a < b })
  heap.Push(5)
  min := heap.Pop()
  ```

- **Concurrent Heap**: Thread-safe heap implementation
  ```csd
  concurrentHeap := heap_slay.NewConcurrentHeap[string]()
  go func() { concurrentHeap.Push("task1") }()
  go func() { task := concurrentHeap.Pop() }()
  ```

- **Bounded Heap**: Heap with maximum capacity
  ```csd
  boundedHeap := heap_slay.NewBoundedHeap[int](100) // Max 100 items
  ```

- **Heap Algorithms**: Additional heap algorithms
  ```csd
  // Merge multiple heaps
  mergedHeap := heap_slay.Merge(heap1, heap2, heap3)
  
  // Heapify a collection
  heap_slay.Heapify(slice)
  ```

- **Advanced Priority Queue**: Enhanced priority queue with more operations
  ```csd
  queue := heap_slay.NewPriorityQueue[Task]()
  queue.PushWithPriority(task, 5)
  queue.ChangePriority(task, 10)
  ```

## Usage Examples

```csd
// Basic IntHeap example
h := &heap_slay.IntHeap{2, 1, 5}
heap_slay.Init(h)
vibez.spill("IntHeap after Init: %v", *h)

heap_slay.Push(h, 3)
vibez.spill("IntHeap after Push(3): %v", *h)

min := heap_slay.Pop(h)
vibez.spill("Popped minimum: %d", min)
vibez.spill("IntHeap after Pop: %v", *h)

// Custom heap implementation for a Person struct
type Person struct {
  Name string
  Age  int
}

type PersonHeap []Person

func (h PersonHeap) Len() int           { return len(h) }
func (h PersonHeap) Less(i, j int) bool { return h[i].Age < h[j].Age } // Min heap by age
func (h PersonHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *PersonHeap) Push(x interface{}) {
  *h = append(*h, x.(Person))
}

func (h *PersonHeap) Pop() interface{} {
  old := *h
  n := len(old)
  x := old[n-1]
  *h = old[0 : n-1]
  return x
}

// Using the custom heap
people := &PersonHeap{
  {"Alice", 25},
  {"Bob", 30},
  {"Charlie", 20},
}

heap_slay.Init(people)
vibez.spill("\nPersonHeap initial state:")
for i, p := range *people {
  vibez.spill("  %d: %s, %d years old", i, p.Name, p.Age)
}

heap_slay.Push(people, Person{"Dave", 35})
vibez.spill("\nPersonHeap after pushing Dave:")
for i, p := range *people {
  vibez.spill("  %d: %s, %d years old", i, p.Name, p.Age)
}

youngest := heap_slay.Pop(people).(Person)
vibez.spill("\nYoungest person: %s, %d years old", youngest.Name, youngest.Age)

vibez.spill("\nPersonHeap after popping youngest:")
for i, p := range *people {
  vibez.spill("  %d: %s, %d years old", i, p.Name, p.Age)
}

// PriorityQueue example (tasks with priorities)
type Task struct {
  Name     string
  Priority int
  Index    int
}

type TaskQueue []*Task

func (pq TaskQueue) Len() int { return len(pq) }

func (pq TaskQueue) Less(i, j int) bool {
  // We want Pop to give us the highest, not lowest, priority
  return pq[i].Priority > pq[j].Priority
}

func (pq TaskQueue) Swap(i, j int) {
  pq[i], pq[j] = pq[j], pq[i]
  pq[i].Index = i
  pq[j].Index = j
}

func (pq *TaskQueue) Push(x interface{}) {
  n := len(*pq)
  task := x.(*Task)
  task.Index = n
  *pq = append(*pq, task)
}

func (pq *TaskQueue) Pop() interface{} {
  old := *pq
  n := len(old)
  task := old[n-1]
  old[n-1] = nil  // avoid memory leak
  task.Index = -1 // for safety
  *pq = old[0 : n-1]
  return task
}

// Update modifies the priority of a Task in the queue
func (pq *TaskQueue) Update(task *Task, name string, priority int) {
  task.Name = name
  task.Priority = priority
  heap_slay.Fix(pq, task.Index)
}

// Using the priority queue
tasks := &TaskQueue{
  &Task{"Write docs", 3, 0},
  &Task{"Fix bug", 7, 1},
  &Task{"Implement feature", 5, 2},
}

heap_slay.Init(tasks)
vibez.spill("\nTask priority queue initial state:")
for i, t := range *tasks {
  vibez.spill("  %d: %s (priority %d)", i, t.Name, t.Priority)
}

// Add a new task
newTask := &Task{"Code review", 4, 0}
heap_slay.Push(tasks, newTask)
vibez.spill("\nAfter pushing 'Code review':")
for i, t := range *tasks {
  vibez.spill("  %d: %s (priority %d)", i, t.Name, t.Priority)
}

// Update priority of an existing task
tasks.Update(newTask, "Urgent code review", 10)
vibez.spill("\nAfter updating 'Code review' to priority 10:")
for i, t := range *tasks {
  vibez.spill("  %d: %s (priority %d)", i, t.Name, t.Priority)
}

// Process tasks in order of priority
vibez.spill("\nProcessing tasks in priority order:")
for tasks.Len() > 0 {
  task := heap_slay.Pop(tasks).(*Task)
  vibez.spill("  Processing: %s (priority %d)", task.Name, task.Priority)
}

// Using enhanced features

// Generic heap
genericHeap := heap_slay.NewHeap[int](func(a, b int) bool { return a < b }) // Min heap
genericHeap.Push(42)
genericHeap.Push(17)
genericHeap.Push(30)
genericHeap.Push(5)

vibez.spill("\nGeneric heap elements in order:")
for !genericHeap.IsEmpty() {
  vibez.spill("  %d", genericHeap.Pop())
}

// Concurrent heap
concurrentHeap := heap_slay.NewConcurrentHeap[string](func(a, b string) bool { 
  return len(a) < len(b) // Sort by string length
})

// Simulate concurrent access
go func() {
  concurrentHeap.Push("hello")
  concurrentHeap.Push("world")
}()

go func() {
  concurrentHeap.Push("a")
  concurrentHeap.Push("loooong")
}()

// Wait for operations to complete
timez.Sleep(100 * timez.Millisecond)

vibez.spill("\nConcurrent heap elements in order:")
for !concurrentHeap.IsEmpty() {
  vibez.spill("  %s", concurrentHeap.Pop())
}

// Bounded heap
boundedHeap := heap_slay.NewBoundedHeap[int](3, func(a, b int) bool { return a > b }) // Max heap with 3 elements

boundedHeap.Push(1)
boundedHeap.Push(5)
boundedHeap.Push(3)
vibez.spill("\nBounded heap current state: %v", boundedHeap.Elements())

// Try to push more elements than the capacity
boundedHeap.Push(10)
vibez.spill("After pushing 10: %v", boundedHeap.Elements()) // Should evict smallest element (1)

boundedHeap.Push(7)
vibez.spill("After pushing 7: %v", boundedHeap.Elements()) // Should evict next smallest (3)

// Advanced priority queue operations
advancedQueue := heap_slay.NewPriorityQueue[string]()
advancedQueue.PushWithPriority("task1", 5)
advancedQueue.PushWithPriority("task2", 3)
advancedQueue.PushWithPriority("task3", 7)

vibez.spill("\nAdvanced priority queue:")
vibez.spill("  Highest priority item: %s", advancedQueue.Peek())

// Change priority of an item
advancedQueue.ChangePriority("task2", 10)
vibez.spill("  After changing task2 priority to 10:")
vibez.spill("  Highest priority item: %s", advancedQueue.Peek())

// Process all items
vibez.spill("  Items in priority order:")
for !advancedQueue.IsEmpty() {
  vibez.spill("    %s", advancedQueue.Pop())
}
```

## Implementation Guidelines

- Implement efficient heap operations (O(log n) time complexity)
- Ensure heap property is maintained after all operations
- Provide clear documentation for the heap interface
- Handle edge cases (empty heap, single element, etc.)
- Support both min and max heap configurations
- Implement memory-efficient heap operations
- Provide useful error messages for invalid operations
- Support type safety when possible
- Maintain index integrity for priority queues
- Optimize for common heap operations