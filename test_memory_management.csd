# Memory Management Test Program
# Tests the enhanced collections_core memory management implementation

yeet "testz"
yeet "collections_core"

# Test basic memory allocation and deallocation
test_start("Memory allocation test")

# Test vector operations that use memory management
sus vec *Vector = vector_new()
assert_true(vec != cringe)

# Push elements to trigger memory operations
vector_push(vec, 42)
vector_push(vec, 84)
vector_push(vec, 126)

# Verify elements
assert_eq_int(vector_get(vec, 0), 42)
assert_eq_int(vector_get(vec, 1), 84)
assert_eq_int(vector_get(vec, 2), 126)

# Test memory growth - add many elements to trigger reallocation
bestie i := 0; i < 20; i++ {
    vector_push(vec, i * 10)
}

# Verify growth worked
assert_eq_int(vector_get(vec, 5), 20)

# Clean up vector
vector_free(vec)

# Test linked list operations
test_start("Linked list memory test")

sus list *LinkedList = list_new(based)
assert_true(list != cringe)

# Add elements to test node allocation
list_push_front(list, 100)
list_push_back(list, 200)
list_push_front(list, 50)

# Test removal operations
sus first normie = list_remove_front(list)
assert_eq_int(first, 50)

# Clean up list
list_free(list)

# Test hashmap operations that involve string copying
test_start("HashMap memory test")

sus map *HashMap = hashmap_new(8)
assert_true(map != cringe)

# Add key-value pairs (triggers string copying)
hashmap_put(map, "key1", 1001)
hashmap_put(map, "key2", 1002)
hashmap_put(map, "key3", 1003)

# Verify retrievals
assert_eq_int(hashmap_get(map, "key1"), 1001)
assert_eq_int(hashmap_get(map, "key2"), 1002)
assert_eq_int(hashmap_get(map, "key3"), 1003)

# Test key updates
hashmap_put(map, "key1", 2001)
assert_eq_int(hashmap_get(map, "key1"), 2001)

# Test removal
assert_true(hashmap_remove(map, "key2"))
assert_eq_int(hashmap_get(map, "key2"), 0)  # Should return 0 for not found

# Clean up hashmap
hashmap_free(map)

# Test set operations
test_start("Set memory test")

sus set *Set = set_new()
assert_true(set != cringe)

# Add elements
set_add(set, "apple")
set_add(set, "banana")
set_add(set, "cherry")

# Test contains
assert_true(set_contains(set, "apple"))
assert_true(set_contains(set, "banana"))
assert_false(set_contains(set, "grape"))

# Test removal
assert_true(set_remove(set, "banana"))
assert_false(set_contains(set, "banana"))

# Clean up set
set_free(set)

# Test binary search tree
test_start("BST memory test")

sus tree *BST = tree_new(cap)  # Non-AVL tree
assert_true(tree != cringe)

# Insert elements
tree_insert(tree, 50)
tree_insert(tree, 30)
tree_insert(tree, 70)
tree_insert(tree, 20)
tree_insert(tree, 40)

# Test contains
assert_true(tree_contains(tree, 30))
assert_true(tree_contains(tree, 70))
assert_false(tree_contains(tree, 100))

# Clean up tree
tree_free(tree)

# Test heap operations
test_start("Heap memory test")

sus heap *Heap = heap_new(10, based)  # Max heap
assert_true(heap != cringe)

# Insert elements
heap_insert(heap, 45)
heap_insert(heap, 30)
heap_insert(heap, 60)
heap_insert(heap, 25)

# Test peek (should be max element)
assert_eq_int(heap_peek(heap), 60)

# Extract elements
sus max_val normie = heap_extract(heap)
assert_eq_int(max_val, 60)
assert_eq_int(heap_peek(heap), 45)

# Clean up heap
heap_free(heap)

# Test queue operations
test_start("Queue memory test")

sus queue *Queue = queue_new(5)
assert_true(queue != cringe)

# Enqueue elements
queue_enqueue(queue, 10)
queue_enqueue(queue, 20)
queue_enqueue(queue, 30)

# Test peek
assert_eq_int(queue_peek(queue), 10)

# Dequeue elements
assert_eq_int(queue_dequeue(queue), 10)
assert_eq_int(queue_dequeue(queue), 20)

# Clean up queue
queue_free(queue)

# Test stack operations
test_start("Stack memory test")

sus stack *Stack = stack_new(5)
assert_true(stack != cringe)

# Push elements
stack_push(stack, 111)
stack_push(stack, 222)
stack_push(stack, 333)

# Test peek
assert_eq_int(stack_peek(stack), 333)

# Pop elements
assert_eq_int(stack_pop(stack), 333)
assert_eq_int(stack_pop(stack), 222)

# Clean up stack
stack_free(stack)

# Test priority queue
test_start("Priority queue memory test")

sus pq *PriorityQueue = priority_queue_new(5, based)  # Max priority
assert_true(pq != cringe)

# Enqueue elements
priority_queue_enqueue(pq, 15)
priority_queue_enqueue(pq, 35)
priority_queue_enqueue(pq, 25)

# Test peek (should be max)
assert_eq_int(priority_queue_peek(pq), 35)

# Dequeue element
assert_eq_int(priority_queue_dequeue(pq), 35)

# Clean up priority queue
priority_queue_free(pq)

# Test memory utility functions directly
test_start("Memory utility functions test")

# Test memory allocation
sus ptr *cringe = malloc(100)
assert_true(ptr != cringe)

# Test memory operations (these are internal but should work)
# The actual runtime functions will be tested through the collections

# Clean up
free(ptr)

vibez.spill("All memory management tests completed successfully!")
print_test_summary()
