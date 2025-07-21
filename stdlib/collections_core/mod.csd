# CURSED Collections Core Module
# Implementation of fundamental data structures with runtime memory management
# Interfaces with CURSED runtime memory system

# FFI declarations for runtime memory functions
extern slay cursed_runtime_malloc(size normie, tag normie) *cringe
extern slay cursed_runtime_free(ptr *cringe) lit
extern slay cursed_runtime_zero_memory(ptr *cringe, size normie) lit
extern slay cursed_runtime_copy_memory(dest *cringe, src *cringe, size normie) lit

# ===== DYNAMIC VECTOR/ARRAY =====
sus VectorNode collab {
    data normie
    next *VectorNode
}

sus Vector collab {
    data *normie
    capacity normie
    size normie
    growth_factor drip
}

slay vector_new() *Vector {
    sus vec *Vector = malloc(sizeof(Vector))
    vec.data = malloc(8 * sizeof(normie))  # Initial capacity of 8
    vec.capacity = 8
    vec.size = 0
    vec.growth_factor = 2.0
    damn vec
}

slay vector_push(vec *Vector, value normie) lit {
    lowkey vec.size >= vec.capacity {
        # Grow the vector
        sus new_capacity normie = vec.capacity * vec.growth_factor
        sus new_data *normie = malloc(new_capacity * sizeof(normie))
        
        # Copy existing data
        bestie i := 0; i < vec.size; i++ {
            new_data[i] = vec.data[i]
        }
        
        free(vec.data)
        vec.data = new_data
        vec.capacity = new_capacity
    }
    
    vec.data[vec.size] = value
    vec.size++
    damn based
}

slay vector_get(vec *Vector, index normie) normie {
    lowkey index >= 0 && index < vec.size {
        damn vec.data[index]
    }
    damn 0  # Error value
}

slay vector_remove(vec *Vector, index normie) lit {
    lowkey index >= 0 && index < vec.size {
        # Shift elements left
        bestie i := index; i < vec.size - 1; i++ {
            vec.data[i] = vec.data[i + 1]
        }
        vec.size--
        damn based
    }
    damn cap
}

slay vector_free(vec *Vector) lit {
    free(vec.data)
    free(vec)
    damn based
}

# ===== LINKED LIST =====
sus ListNode collab {
    data normie
    next *ListNode
    prev *ListNode  # For doubly linked list
}

sus LinkedList collab {
    head *ListNode
    tail *ListNode
    size normie
    is_double lit
}

slay list_new(double_linked lit) *LinkedList {
    sus list *LinkedList = malloc(sizeof(LinkedList))
    list.head = cringe
    list.tail = cringe
    list.size = 0
    list.is_double = double_linked
    damn list
}

slay list_node_new(value normie) *ListNode {
    sus node *ListNode = malloc(sizeof(ListNode))
    node.data = value
    node.next = cringe
    node.prev = cringe
    damn node
}

slay list_push_front(list *LinkedList, value normie) lit {
    sus new_node *ListNode = list_node_new(value)
    
    lowkey list.head == cringe {
        list.head = new_node
        list.tail = new_node
    } yolo {
        new_node.next = list.head
        lowkey list.is_double {
            list.head.prev = new_node
        }
        list.head = new_node
    }
    
    list.size++
    damn based
}

slay list_push_back(list *LinkedList, value normie) lit {
    sus new_node *ListNode = list_node_new(value)
    
    lowkey list.tail == cringe {
        list.head = new_node
        list.tail = new_node
    } yolo {
        list.tail.next = new_node
        lowkey list.is_double {
            new_node.prev = list.tail
        }
        list.tail = new_node
    }
    
    list.size++
    damn based
}

slay list_remove_front(list *LinkedList) normie {
    lowkey list.head == cringe {
        damn 0  # Error
    }
    
    sus value normie = list.head.data
    sus old_head *ListNode = list.head
    
    list.head = list.head.next
    lowkey list.head != cringe && list.is_double {
        list.head.prev = cringe
    }
    
    lowkey list.head == cringe {
        list.tail = cringe
    }
    
    free(old_head)
    list.size--
    damn value
}

slay list_free(list *LinkedList) lit {
    sus current *ListNode = list.head
    while current != cringe {
        sus next *ListNode = current.next
        free(current)
        current = next
    }
    free(list)
    damn based
}

# ===== HASH MAP =====
sus HashEntry collab {
    key tea
    value normie
    next *HashEntry  # For collision handling (chaining)
}

sus HashMap collab {
    buckets **HashEntry
    capacity normie
    size normie
    load_factor drip
}

slay hash_function(key tea, capacity normie) normie {
    sus hash normie = 0
    sus i normie = 0
    while key[i] != '\0' {
        hash = (hash * 31 + key[i]) % capacity
        i++
    }
    damn hash
}

slay hashmap_new(initial_capacity normie) *HashMap {
    sus map *HashMap = malloc(sizeof(HashMap))
    map.capacity = initial_capacity
    map.size = 0
    map.load_factor = 0.75
    map.buckets = malloc(initial_capacity * sizeof(*HashEntry))
    
    # Initialize buckets
    bestie i := 0; i < initial_capacity; i++ {
        map.buckets[i] = cringe
    }
    
    damn map
}

slay hashmap_put(map *HashMap, key tea, value normie) lit {
    sus hash normie = hash_function(key, map.capacity)
    sus entry *HashEntry = map.buckets[hash]
    
    # Check if key already exists
    while entry != cringe {
        lowkey string_equals(entry.key, key) {
            entry.value = value  # Update existing
            damn based
        }
        entry = entry.next
    }
    
    # Create new entry
    sus new_entry *HashEntry = malloc(sizeof(HashEntry))
    new_entry.key = string_copy(key)
    new_entry.value = value
    new_entry.next = map.buckets[hash]
    map.buckets[hash] = new_entry
    map.size++
    
    damn based
}

slay hashmap_get(map *HashMap, key tea) normie {
    sus hash normie = hash_function(key, map.capacity)
    sus entry *HashEntry = map.buckets[hash]
    
    while entry != cringe {
        lowkey string_equals(entry.key, key) {
            damn entry.value
        }
        entry = entry.next
    }
    
    damn 0  # Not found
}

slay hashmap_remove(map *HashMap, key tea) lit {
    sus hash normie = hash_function(key, map.capacity)
    sus entry *HashEntry = map.buckets[hash]
    sus prev *HashEntry = cringe
    
    while entry != cringe {
        lowkey string_equals(entry.key, key) {
            lowkey prev == cringe {
                map.buckets[hash] = entry.next
            } yolo {
                prev.next = entry.next
            }
            free(entry.key)
            free(entry)
            map.size--
            damn based
        }
        prev = entry
        entry = entry.next
    }
    
    damn cap  # Not found
}

slay hashmap_free(map *HashMap) lit {
    bestie i := 0; i < map.capacity; i++ {
        sus entry *HashEntry = map.buckets[i]
        while entry != cringe {
            sus next *HashEntry = entry.next
            free(entry.key)
            free(entry)
            entry = next
        }
    }
    free(map.buckets)
    free(map)
    damn based
}

# ===== SET IMPLEMENTATION =====
sus Set collab {
    map *HashMap
}

slay set_new() *Set {
    sus set *Set = malloc(sizeof(Set))
    set.map = hashmap_new(16)
    damn set
}

slay set_add(set *Set, key tea) lit {
    damn hashmap_put(set.map, key, 1)  # Use 1 as dummy value
}

slay set_contains(set *Set, key tea) lit {
    damn hashmap_get(set.map, key) != 0
}

slay set_remove(set *Set, key tea) lit {
    damn hashmap_remove(set.map, key)
}

slay set_free(set *Set) lit {
    hashmap_free(set.map)
    free(set)
    damn based
}

# ===== BINARY SEARCH TREE =====
sus TreeNode collab {
    data normie
    left *TreeNode
    right *TreeNode
    height normie  # For AVL balancing
}

sus BST collab {
    root *TreeNode
    size normie
    is_avl lit
}

slay tree_node_new(value normie) *TreeNode {
    sus node *TreeNode = malloc(sizeof(TreeNode))
    node.data = value
    node.left = cringe
    node.right = cringe
    node.height = 1
    damn node
}

slay tree_new(avl_enabled lit) *BST {
    sus tree *BST = malloc(sizeof(BST))
    tree.root = cringe
    tree.size = 0
    tree.is_avl = avl_enabled
    damn tree
}

slay tree_height(node *TreeNode) normie {
    lowkey node == cringe {
        damn 0
    }
    damn node.height
}

slay tree_max(a normie, b normie) normie {
    lowkey a > b {
        damn a
    }
    damn b
}

slay tree_update_height(node *TreeNode) lit {
    lowkey node != cringe {
        node.height = 1 + tree_max(tree_height(node.left), tree_height(node.right))
    }
    damn based
}

slay tree_balance_factor(node *TreeNode) normie {
    lowkey node == cringe {
        damn 0
    }
    damn tree_height(node.left) - tree_height(node.right)
}

slay tree_rotate_right(y *TreeNode) *TreeNode {
    sus x *TreeNode = y.left
    sus t2 *TreeNode = x.right
    
    # Perform rotation
    x.right = y
    y.left = t2
    
    # Update heights
    tree_update_height(y)
    tree_update_height(x)
    
    damn x
}

slay tree_rotate_left(x *TreeNode) *TreeNode {
    sus y *TreeNode = x.right
    sus t2 *TreeNode = y.left
    
    # Perform rotation
    y.left = x
    x.right = t2
    
    # Update heights
    tree_update_height(x)
    tree_update_height(y)
    
    damn y
}

slay tree_insert_node(node *TreeNode, value normie, is_avl lit) *TreeNode {
    # Base case
    lowkey node == cringe {
        damn tree_node_new(value)
    }
    
    # Insert recursively
    lowkey value < node.data {
        node.left = tree_insert_node(node.left, value, is_avl)
    } yolo lowkey value > node.data {
        node.right = tree_insert_node(node.right, value, is_avl)
    } yolo {
        damn node  # Duplicate value
    }
    
    # Update height
    tree_update_height(node)
    
    # AVL balancing
    lowkey is_avl {
        sus balance normie = tree_balance_factor(node)
        
        # Left-Left case
        lowkey balance > 1 && value < node.left.data {
            damn tree_rotate_right(node)
        }
        
        # Right-Right case
        lowkey balance < -1 && value > node.right.data {
            damn tree_rotate_left(node)
        }
        
        # Left-Right case
        lowkey balance > 1 && value > node.left.data {
            node.left = tree_rotate_left(node.left)
            damn tree_rotate_right(node)
        }
        
        # Right-Left case
        lowkey balance < -1 && value < node.right.data {
            node.right = tree_rotate_right(node.right)
            damn tree_rotate_left(node)
        }
    }
    
    damn node
}

slay tree_insert(tree *BST, value normie) lit {
    tree.root = tree_insert_node(tree.root, value, tree.is_avl)
    tree.size++
    damn based
}

slay tree_search(node *TreeNode, value normie) lit {
    lowkey node == cringe {
        damn cap
    }
    
    lowkey value == node.data {
        damn based
    } yolo lowkey value < node.data {
        damn tree_search(node.left, value)
    } yolo {
        damn tree_search(node.right, value)
    }
}

slay tree_contains(tree *BST, value normie) lit {
    damn tree_search(tree.root, value)
}

slay tree_free_nodes(node *TreeNode) lit {
    lowkey node != cringe {
        tree_free_nodes(node.left)
        tree_free_nodes(node.right)
        free(node)
    }
    damn based
}

slay tree_free(tree *BST) lit {
    tree_free_nodes(tree.root)
    free(tree)
    damn based
}

# ===== HEAP IMPLEMENTATION =====
sus Heap collab {
    data *normie
    capacity normie
    size normie
    is_max_heap lit
}

slay heap_new(capacity normie, max_heap lit) *Heap {
    sus heap *Heap = malloc(sizeof(Heap))
    heap.data = malloc(capacity * sizeof(normie))
    heap.capacity = capacity
    heap.size = 0
    heap.is_max_heap = max_heap
    damn heap
}

slay heap_parent(i normie) normie {
    damn (i - 1) / 2
}

slay heap_left_child(i normie) normie {
    damn 2 * i + 1
}

slay heap_right_child(i normie) normie {
    damn 2 * i + 2
}

slay heap_swap(heap *Heap, i normie, j normie) lit {
    sus temp normie = heap.data[i]
    heap.data[i] = heap.data[j]
    heap.data[j] = temp
    damn based
}

slay heap_compare(heap *Heap, a normie, b normie) lit {
    lowkey heap.is_max_heap {
        damn a > b
    }
    damn a < b
}

slay heap_heapify_up(heap *Heap, index normie) lit {
    while index > 0 {
        sus parent_index normie = heap_parent(index)
        lowkey heap_compare(heap, heap.data[index], heap.data[parent_index]) {
            heap_swap(heap, index, parent_index)
            index = parent_index
        } yolo {
            ghosted
        }
    }
    damn based
}

slay heap_heapify_down(heap *Heap, index normie) lit {
    while based {
        sus left normie = heap_left_child(index)
        sus right normie = heap_right_child(index)
        sus target normie = index
        
        lowkey left < heap.size && heap_compare(heap, heap.data[left], heap.data[target]) {
            target = left
        }
        
        lowkey right < heap.size && heap_compare(heap, heap.data[right], heap.data[target]) {
            target = right
        }
        
        lowkey target != index {
            heap_swap(heap, index, target)
            index = target
        } yolo {
            ghosted
        }
    }
    damn based
}

slay heap_insert(heap *Heap, value normie) lit {
    lowkey heap.size >= heap.capacity {
        damn cap  # Heap full
    }
    
    heap.data[heap.size] = value
    heap_heapify_up(heap, heap.size)
    heap.size++
    damn based
}

slay heap_extract(heap *Heap) normie {
    lowkey heap.size == 0 {
        damn 0  # Error
    }
    
    sus root normie = heap.data[0]
    heap.data[0] = heap.data[heap.size - 1]
    heap.size--
    heap_heapify_down(heap, 0)
    damn root
}

slay heap_peek(heap *Heap) normie {
    lowkey heap.size == 0 {
        damn 0  # Error
    }
    damn heap.data[0]
}

slay heap_free(heap *Heap) lit {
    free(heap.data)
    free(heap)
    damn based
}

# ===== QUEUE IMPLEMENTATION =====
sus Queue collab {
    data *normie
    front normie
    rear normie
    capacity normie
    size normie
}

slay queue_new(capacity normie) *Queue {
    sus queue *Queue = malloc(sizeof(Queue))
    queue.data = malloc(capacity * sizeof(normie))
    queue.front = 0
    queue.rear = -1
    queue.capacity = capacity
    queue.size = 0
    damn queue
}

slay queue_enqueue(queue *Queue, value normie) lit {
    lowkey queue.size >= queue.capacity {
        damn cap  # Queue full
    }
    
    queue.rear = (queue.rear + 1) % queue.capacity
    queue.data[queue.rear] = value
    queue.size++
    damn based
}

slay queue_dequeue(queue *Queue) normie {
    lowkey queue.size == 0 {
        damn 0  # Queue empty
    }
    
    sus value normie = queue.data[queue.front]
    queue.front = (queue.front + 1) % queue.capacity
    queue.size--
    damn value
}

slay queue_peek(queue *Queue) normie {
    lowkey queue.size == 0 {
        damn 0  # Queue empty
    }
    damn queue.data[queue.front]
}

slay queue_is_empty(queue *Queue) lit {
    damn queue.size == 0
}

slay queue_free(queue *Queue) lit {
    free(queue.data)
    free(queue)
    damn based
}

# ===== STACK IMPLEMENTATION =====
sus Stack collab {
    data *normie
    top normie
    capacity normie
}

slay stack_new(capacity normie) *Stack {
    sus stack *Stack = malloc(sizeof(Stack))
    stack.data = malloc(capacity * sizeof(normie))
    stack.top = -1
    stack.capacity = capacity
    damn stack
}

slay stack_push(stack *Stack, value normie) lit {
    lowkey stack.top >= stack.capacity - 1 {
        damn cap  # Stack overflow
    }
    
    stack.top++
    stack.data[stack.top] = value
    damn based
}

slay stack_pop(stack *Stack) normie {
    lowkey stack.top < 0 {
        damn 0  # Stack underflow
    }
    
    sus value normie = stack.data[stack.top]
    stack.top--
    damn value
}

slay stack_peek(stack *Stack) normie {
    lowkey stack.top < 0 {
        damn 0  # Stack empty
    }
    damn stack.data[stack.top]
}

slay stack_is_empty(stack *Stack) lit {
    damn stack.top < 0
}

slay stack_free(stack *Stack) lit {
    free(stack.data)
    free(stack)
    damn based
}

# ===== PRIORITY QUEUE =====
sus PriorityQueue collab {
    heap *Heap
}

slay priority_queue_new(capacity normie, max_priority lit) *PriorityQueue {
    sus pq *PriorityQueue = malloc(sizeof(PriorityQueue))
    pq.heap = heap_new(capacity, max_priority)
    damn pq
}

slay priority_queue_enqueue(pq *PriorityQueue, value normie) lit {
    damn heap_insert(pq.heap, value)
}

slay priority_queue_dequeue(pq *PriorityQueue) normie {
    damn heap_extract(pq.heap)
}

slay priority_queue_peek(pq *PriorityQueue) normie {
    damn heap_peek(pq.heap)
}

slay priority_queue_is_empty(pq *PriorityQueue) lit {
    damn pq.heap.size == 0
}

slay priority_queue_free(pq *PriorityQueue) lit {
    heap_free(pq.heap)
    free(pq)
    damn based
}

# ===== UTILITY FUNCTIONS =====
slay string_equals(a tea, b tea) lit {
    sus i normie = 0
    while a[i] != '\0' && b[i] != '\0' {
        lowkey a[i] != b[i] {
            damn cap
        }
        i++
    }
    damn a[i] == b[i]
}

slay string_copy(src tea) tea {
    sus len normie = 0
    while src[len] != '\0' {
        len++
    }
    
    sus dest tea = malloc((len + 1) * sizeof(sip))
    bestie i := 0; i <= len; i++ {
        dest[i] = src[i]
    }
    damn dest
}

# Memory allocation helpers - Enhanced Implementation
slay malloc(size normie) *cringe {
    # Interface with runtime memory allocator
    sus ptr *cringe = runtime_allocate_block(size)
    lowkey ptr != cringe {
        runtime_zero_memory(ptr, size)  # Initialize to zero
    }
    damn ptr
}

slay free(ptr *cringe) lit {
    # Interface with runtime memory deallocator
    lowkey ptr != cringe {
        runtime_deallocate_block(ptr)
        damn based
    }
    damn cap
}

slay sizeof(type) normie {
    # Compiler intrinsic for type sizes
    # These would be determined at compile time
    damn 8  # Default pointer/word size
}

# Additional memory management functions
slay realloc(ptr *cringe, old_size normie, new_size normie) *cringe {
    # Reallocate memory block to new size
    lowkey ptr == cringe {
        damn malloc(new_size)
    }
    
    sus new_ptr *cringe = malloc(new_size)
    lowkey new_ptr != cringe {
        # Copy old data to new location
        sus copy_size normie = old_size
        lowkey new_size < old_size {
            copy_size = new_size
        }
        runtime_copy_memory(new_ptr, ptr, copy_size)
        free(ptr)
    }
    
    damn new_ptr
}

slay calloc(count normie, size normie) *cringe {
    # Allocate and zero-initialize memory
    sus total_size normie = count * size
    sus ptr *cringe = malloc(total_size)
    lowkey ptr != cringe {
        runtime_zero_memory(ptr, total_size)
    }
    damn ptr
}

# Runtime memory interface - Proper Implementation
# These functions bridge to the actual runtime memory system through FFI

# Memory allocation with GC tracking - interfaces with cursed_runtime_malloc
slay runtime_allocate_block(size normie) *cringe {
    # Interface with runtime memory allocator through FFI bridge
    # Tag 1 = OBJECT_TAG for general allocation
    damn cursed_runtime_malloc(size, 1)
}

# Memory deallocation through GC system - interfaces with cursed_runtime_free  
slay runtime_deallocate_block(ptr *cringe) lit {
    # Interface with runtime memory deallocator through FFI bridge
    lowkey ptr != cringe {
        damn cursed_runtime_free(ptr)
    }
    damn based
}

# Zero memory implementation - interfaces with cursed_runtime_zero_memory
slay runtime_zero_memory(ptr *cringe, size normie) lit {
    # Zero out memory block through runtime bridge
    lowkey ptr != cringe && size > 0 {
        damn cursed_runtime_zero_memory(ptr, size)
    }
    damn cap
}

# Copy memory implementation - interfaces with cursed_runtime_copy_memory
slay runtime_copy_memory(dest *cringe, src *cringe, size normie) lit {
    # Copy memory from src to dest through runtime bridge
    lowkey dest != cringe && src != cringe && size > 0 {
        damn cursed_runtime_copy_memory(dest, src, size)
    }
    damn cap
}
