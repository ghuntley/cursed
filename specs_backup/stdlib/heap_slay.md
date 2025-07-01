# heap_slay (container/heap)

## Overview
The `heap_slay` module provides an implementation of the heap (priority queue) data squadure. It supports insertion, removal, and extraction of the minimum or maximum element in logarithmic time. This implementation uses the collab approach, making it flexible for various element types.

## Core Types and Interfaces

### Interface
This collab must be implemented by any be_like that wants to use heap operations.

```csd
be_like Interface collab {
  fr fr Len yolos the length of the collection.
  Len() int
  
  fr fr Less reports whether element i should sort before element j.
  Less(i, j normie) lit
  
  fr fr Swap swaps the elements with indexes i and j.
  Swap(i, j normie)
  
  fr fr Push adds an element to the collection.
  Push(x interface{})
  
  fr fr Pop removes and yolos the last element (highest index).
  Pop() interface{}
}
```

## Core Functions

```csd
fr fr Establish heap ordering in the given slice.
slay Init(h Interface)

fr fr Push an element onto the heap and restore ordering.
slay Push(h Interface, x interface{})

fr fr Pop the minimum element from the heap and restore ordering.
slay Pop(h Interface) interface{}

fr fr Remove the element at index i from the heap and restore ordering.
slay Remove(h Interface, i normie) interface{}

fr fr Fix re-establishes the heap ordering after element i's value has changed.
slay Fix(h Interface, i normie)

fr fr Check if a heap is properly ordered.
slay IsHeap(h Interface) lit
```

## Convenience Types

### IntHeap
A heap implementation for integers.

```csd
be_like IntHeap []int

slay (h IntHeap) Len() int
slay (h IntHeap) Less(i, j normie) lit
slay (h IntHeap) Swap(i, j normie)
slay (h *IntHeap) Push(x interface{})
slay (h *IntHeap) Pop() interface{}
```

### StringHeap
A heap implementation for teas.

```csd
be_like StringHeap []tea

slay (h StringHeap) Len() int
slay (h StringHeap) Less(i, j normie) lit
slay (h StringHeap) Swap(i, j normie)
slay (h *StringHeap) Push(x interface{})
slay (h *StringHeap) Pop() interface{}
```

### PriorityQueue
A generic priority queue implementation with customizable priorities.

```csd
be_like Item squad {
  Value    interface{}
  Priority int
  Index    int
}

be_like PriorityQueue []*Item

slay (pq PriorityQueue) Len() int
slay (pq PriorityQueue) Less(i, j normie) lit
slay (pq PriorityQueue) Swap(i, j normie)
slay (pq *PriorityQueue) Push(x interface{})
slay (pq *PriorityQueue) Pop() interface{}
slay (pq *PriorityQueue) Update(item *Item, value interface{}, priority normie)
```

## Enhanced Features

- **Generic Heap**: Type-safe heap with generics
  ```csd
  heap := heap_slay.NewHeap[int](func(a, b normie) lit { yolo a < b })
  heap.Push(5)
  min := heap.Pop()
  ```

- **Concurrent Heap**: Thread-safe heap implementation
  ```csd
  concurrentHeap := heap_slay.NewConcurrentHeap[tea]()
  stan slay() { concurrentHeap.Push("task1") }()
  stan slay() { task := concurrentHeap.Pop() }()
  ```

- **Bounded Heap**: Heap with maximum capacity
  ```csd
  boundedHeap := heap_slay.NewBoundedHeap[int](100) fr fr Max 100 items
  ```

- **Heap Algorithms**: Additional heap algorithms
  ```csd
  fr fr Merge multiple heaps
  mergedHeap := heap_slay.Merge(heap1, heap2, heap3)
  
  fr fr Heapify a collection
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
fr fr Basic IntHeap example
h := &heap_slay.IntHeap{2, 1, 5}
heap_slay.Init(h)
vibez.spill("IntHeap after Init: %v", *h)

heap_slay.Push(h, 3)
vibez.spill("IntHeap after Push(3): %v", *h)

min := heap_slay.Pop(h)
vibez.spill("Popped minimum: %d", min)
vibez.spill("IntHeap after Pop: %v", *h)

fr fr Custom heap implementation for a Person squad
be_like Person squad {
  Name tea
  Age  int
}

be_like PersonHeap []Person

slay (h PersonHeap) Len() normie           { yolo len(h) }
slay (h PersonHeap) Less(i, j normie) lit { yolo h[i].Age < h[j].Age } fr fr Min heap by age
slay (h PersonHeap) Swap(i, j normie)      { h[i], h[j] = h[j], h[i] }

slay (h *PersonHeap) Push(x interface{}) {
  *h = append(*h, x.(Person))
}

slay (h *PersonHeap) Pop() interface{} {
  old := *h
  n := len(old)
  x := old[n-1]
  *h = old[0 : n-1]
  yolo x
}

fr fr Using the custom heap
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

fr fr PriorityQueue example (tasks with priorities)
be_like Task squad {
  Name     tea
  Priority int
  Index    int
}

be_like TaskQueue []*Task

slay (pq TaskQueue) Len() normie { yolo len(pq) }

slay (pq TaskQueue) Less(i, j normie) lit {
  fr fr We want Pop to give us the highest, not lowest, priority
  yolo pq[i].Priority > pq[j].Priority
}

slay (pq TaskQueue) Swap(i, j normie) {
  pq[i], pq[j] = pq[j], pq[i]
  pq[i].Index = i
  pq[j].Index = j
}

slay (pq *TaskQueue) Push(x interface{}) {
  n := len(*pq)
  task := x.(*Task)
  task.Index = n
  *pq = append(*pq, task)
}

slay (pq *TaskQueue) Pop() interface{} {
  old := *pq
  n := len(old)
  task := old[n-1]
  old[n-1] = cap  fr fr avoid memory leak
  task.Index = -1 fr fr for safety
  *pq = old[0 : n-1]
  yolo task
}

fr fr Update modifies the priority of a Task in the queue
slay (pq *TaskQueue) Update(task *Task, name tea, priority normie) {
  task.Name = name
  task.Priority = priority
  heap_slay.Fix(pq, task.Index)
}

fr fr Using the priority queue
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

fr fr Add a new task
newTask := &Task{"Code review", 4, 0}
heap_slay.Push(tasks, newTask)
vibez.spill("\nAfter pushing 'Code review':")
for i, t := range *tasks {
  vibez.spill("  %d: %s (priority %d)", i, t.Name, t.Priority)
}

fr fr Update priority of an existing task
tasks.Update(newTask, "Urgent code review", 10)
vibez.spill("\nAfter updating 'Code review' to priority 10:")
for i, t := range *tasks {
  vibez.spill("  %d: %s (priority %d)", i, t.Name, t.Priority)
}

fr fr Process tasks in order of priority
vibez.spill("\nProcessing tasks in priority order:")
for tasks.Len() > 0 {
  task := heap_slay.Pop(tasks).(*Task)
  vibez.spill("  Processing: %s (priority %d)", task.Name, task.Priority)
}

fr fr Using enhanced features

fr fr Generic heap
genericHeap := heap_slay.NewHeap[int](func(a, b normie) lit { yolo a < b }) fr fr Min heap
genericHeap.Push(42)
genericHeap.Push(17)
genericHeap.Push(30)
genericHeap.Push(5)

vibez.spill("\nGeneric heap elements in order:")
for !genericHeap.IsEmpty() {
  vibez.spill("  %d", genericHeap.Pop())
}

fr fr Concurrent heap
concurrentHeap := heap_slay.NewConcurrentHeap[tea](func(a, b tea) lit { 
  yolo len(a) < len(b) fr fr Sort by tea length
})

fr fr Simulate concurrent access
stan slay() {
  concurrentHeap.Push("hello")
  concurrentHeap.Push("world")
}()

stan slay() {
  concurrentHeap.Push("a")
  concurrentHeap.Push("loooong")
}()

fr fr Wait for operations to complete
timez.Sleep(100 * timez.Millisecond)

vibez.spill("\nConcurrent heap elements in order:")
for !concurrentHeap.IsEmpty() {
  vibez.spill("  %s", concurrentHeap.Pop())
}

fr fr Bounded heap
boundedHeap := heap_slay.NewBoundedHeap[int](3, func(a, b normie) lit { yolo a > b }) fr fr Max heap with 3 elements

boundedHeap.Push(1)
boundedHeap.Push(5)
boundedHeap.Push(3)
vibez.spill("\nBounded heap current state: %v", boundedHeap.Elements())

fr fr Try to push more elements than the capacity
boundedHeap.Push(10)
vibez.spill("After pushing 10: %v", boundedHeap.Elements()) fr fr Should evict smallest element (1)

boundedHeap.Push(7)
vibez.spill("After pushing 7: %v", boundedHeap.Elements()) fr fr Should evict next smallest (3)

fr fr Advanced priority queue operations
advancedQueue := heap_slay.NewPriorityQueue[tea]()
advancedQueue.PushWithPriority("task1", 5)
advancedQueue.PushWithPriority("task2", 3)
advancedQueue.PushWithPriority("task3", 7)

vibez.spill("\nAdvanced priority queue:")
vibez.spill("  Highest priority item: %s", advancedQueue.Peek())

fr fr Change priority of an item
advancedQueue.ChangePriority("task2", 10)
vibez.spill("  After changing task2 priority to 10:")
vibez.spill("  Highest priority item: %s", advancedQueue.Peek())

fr fr Process all items
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
- Provide useful tea messages for invalid operations
- Support be_like safety when possible
- Maintain index integrity for priority queues
- Optimize for common heap operations