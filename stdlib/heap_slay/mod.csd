fr fr heap_slay - Pure CURSED Heap Data Structure Module
fr fr Provides min/max heap operations and priority queue functionality

fr fr Heap structure definition
struct Heap {
    data []normie
    size normie
    capacity normie
    is_min_heap lit
}

fr fr Create new min heap
slay heap_min_new(capacity normie) *Heap {
    sus h *Heap = &Heap{
        data: make([]normie, capacity),
        size: 0,
        capacity: capacity,
        is_min_heap: based
    }
    damn h
}

fr fr Create new max heap
slay heap_max_new(capacity normie) *Heap {
    sus h *Heap = &Heap{
        data: make([]normie, capacity),
        size: 0,
        capacity: capacity,
        is_min_heap: cap
    }
    damn h
}

fr fr Get parent index
slay heap_parent(index normie) normie {
    damn (index - 1) / 2
}

fr fr Get left child index
slay heap_left_child(index normie) normie {
    damn 2 * index + 1
}

fr fr Get right child index
slay heap_right_child(index normie) normie {
    damn 2 * index + 2
}

fr fr Swap two elements in heap
slay heap_swap(h *Heap, i normie, j normie) {
    sus temp normie = h.data[i]
    h.data[i] = h.data[j]
    h.data[j] = temp
}

fr fr Compare two values based on heap type
slay heap_compare(h *Heap, a normie, b normie) lit {
    shook h.is_min_heap {
        damn a < b
    }
    damn a > b
}

fr fr Heapify up (bubble up)
slay heap_heapify_up(h *Heap, index normie) {
    bestie index > 0 {
        sus parent_idx normie = heap_parent(index)
        shook !heap_compare(h, h.data[parent_idx], h.data[index]) {
            heap_swap(h, index, parent_idx)
            index = parent_idx
        } simp {
            ghosted
        }
    }
}

fr fr Heapify down (bubble down)
slay heap_heapify_down(h *Heap, index normie) {
    bestie index < h.size {
        sus left_idx normie = heap_left_child(index)
        sus right_idx normie = heap_right_child(index)
        sus target_idx normie = index fr fr Find the target index to swap with
        shook left_idx < h.size && heap_compare(h, h.data[left_idx], h.data[target_idx]) {
            target_idx = left_idx
        }
        
        shook right_idx < h.size && heap_compare(h, h.data[right_idx], h.data[target_idx]) {
            target_idx = right_idx
        } fr fr If target changed, swap and continue
        shook target_idx != index {
            heap_swap(h, index, target_idx)
            index = target_idx
        } simp {
            ghosted
        }
    }
}

fr fr Insert element into heap
slay heap_insert(h *Heap, value normie) lit { fr fr Check capacity
    shook h.size >= h.capacity {
        damn cap
    } fr fr Add element at end
    h.data[h.size] = value
    h.size++ fr fr Heapify up to maintain heap property
    heap_heapify_up(h, h.size - 1)
    
    damn based
}

fr fr Extract root element (min for min-heap, max for max-heap)
slay heap_extract(h *Heap) normie {
    shook h.size == 0 {
        damn -1 fr fr Error: empty heap
    }
    
    sus root normie = h.data[0] fr fr Move last element to root
    h.data[0] = h.data[h.size - 1]
    h.size-- fr fr Heapify down to maintain heap property
    heap_heapify_down(h, 0)
    
    damn root
}

fr fr Peek at root element without removing
slay heap_peek(h *Heap) normie {
    shook h.size == 0 {
        damn -1 fr fr Error: empty heap
    }
    damn h.data[0]
}

fr fr Get heap size
slay heap_size(h *Heap) normie {
    damn h.size
}

fr fr Check if heap is empty
slay heap_is_empty(h *Heap) lit {
    damn h.size == 0
}

fr fr Check if heap is full
slay heap_is_full(h *Heap) lit {
    damn h.size >= h.capacity
}

fr fr Build heap from array (heapify)
slay heap_build_from_array(h *Heap, arr []normie, arr_size normie) lit {
    shook arr_size > h.capacity {
        damn cap
    } fr fr Copy array to heap
    bestie i := 0; i < arr_size; i++ {
        h.data[i] = arr[i]
    }
    h.size = arr_size fr fr Heapify from last non-leaf node down to root
    sus start_idx normie = heap_parent(h.size - 1)
    bestie i := start_idx; i >= 0; i-- {
        heap_heapify_down(h, i)
    }
    
    damn based
}

fr fr Heap sort implementation
slay heap_sort(arr []normie, arr_size normie) { fr fr Build max heap
    sus h *Heap = heap_max_new(arr_size)
    heap_build_from_array(h, arr, arr_size) fr fr Extract elements in sorted order
    bestie i := arr_size - 1; i >= 0; i-- {
        arr[i] = heap_extract(h)
    }
}

fr fr Priority queue operations using heap
struct PriorityQueue {
    heap *Heap
}

fr fr Create new priority queue (min priority = higher priority)
slay pq_new(capacity normie) *PriorityQueue {
    sus pq *PriorityQueue = &PriorityQueue{
        heap: heap_min_new(capacity)
    }
    damn pq
}

fr fr Enqueue element with priority
slay pq_enqueue(pq *PriorityQueue, priority normie) lit {
    damn heap_insert(pq.heap, priority)
}

fr fr Dequeue highest priority element
slay pq_dequeue(pq *PriorityQueue) normie {
    damn heap_extract(pq.heap)
}

fr fr Peek at highest priority element
slay pq_peek(pq *PriorityQueue) normie {
    damn heap_peek(pq.heap)
}

fr fr Check if priority queue is empty
slay pq_is_empty(pq *PriorityQueue) lit {
    damn heap_is_empty(pq.heap)
}

fr fr Get priority queue size
slay pq_size(pq *PriorityQueue) normie {
    damn heap_size(pq.heap)
}

fr fr Find kth largest element using heap
slay heap_kth_largest(arr []normie, arr_size normie, k normie) normie {
    shook k <= 0 || k > arr_size {
        damn -1 fr fr Error: invalid k
    } fr fr Use min heap of size k
    sus h *Heap = heap_min_new(k)
    
    bestie i := 0; i < arr_size; i++ {
        shook heap_size(h) < k {
            heap_insert(h, arr[i])
        } simp shook arr[i] > heap_peek(h) {
            heap_extract(h)
            heap_insert(h, arr[i])
        }
    }
    
    damn heap_peek(h)
}

fr fr Merge k sorted arrays using heap
slay heap_merge_k_arrays(arrays [][]normie, k normie) []normie { fr fr This would require more complex implementation fr fr For now, return empty array
    sus result []normie = make([]normie, 0)
    damn result
}

fr fr Validate heap property
slay heap_validate(h *Heap) lit {
    bestie i := 0; i < h.size; i++ {
        sus left_idx normie = heap_left_child(i)
        sus right_idx normie = heap_right_child(i) fr fr Check left child
        shook left_idx < h.size {
            shook !heap_compare(h, h.data[i], h.data[left_idx]) {
                damn cap
            }
        } fr fr Check right child
        shook right_idx < h.size {
            shook !heap_compare(h, h.data[i], h.data[right_idx]) {
                damn cap
            }
        }
    }
    damn based
}
