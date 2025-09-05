fr fr heap_slay module comprehensive test suite
yeet "testz"
yeet "heap_slay"

fr fr Test min heap creation
test_start("heap_min_new")
sus h *Heap = heap_min_new(10)
assert_true(h.capacity == 10)
assert_true(h.size == 0)
assert_true(h.is_min_heap == based)
assert_true(heap_is_empty(h))
assert_false(heap_is_full(h))

fr fr Test max heap creation
test_start("heap_max_new")
sus h_max *Heap = heap_max_new(5)
assert_true(h_max.capacity == 5)
assert_true(h_max.size == 0)
assert_true(h_max.is_min_heap == cap)
assert_true(heap_is_empty(h_max))
assert_false(heap_is_full(h_max))

fr fr Test heap utility functions
test_start("heap_utility_functions")
assert_true(heap_parent(5) == 2)
assert_true(heap_parent(6) == 2)
assert_true(heap_left_child(2) == 5)
assert_true(heap_right_child(2) == 6)
assert_true(heap_parent(0) == 0) fr fr Root parent is itself in integer division

fr fr Test min heap insertion
test_start("heap_min_insert")
sus min_h *Heap = heap_min_new(10)
assert_true(heap_insert(min_h, 5))
assert_true(heap_insert(min_h, 3))
assert_true(heap_insert(min_h, 8))
assert_true(heap_insert(min_h, 1))
assert_true(heap_size(min_h) == 4)
assert_false(heap_is_empty(min_h))

fr fr Test min heap peek (should be minimum)
test_start("heap_min_peek")
sus min_root normie = heap_peek(min_h)
assert_true(min_root == 1) fr fr Minimum element should be at root

fr fr Test min heap extraction
test_start("heap_min_extract")
sus extracted normie = heap_extract(min_h)
assert_true(extracted == 1) fr fr Should extract minimum
assert_true(heap_size(min_h) == 3)
sus next_min normie = heap_peek(min_h)
assert_true(next_min == 3) fr fr Next minimum should be 3

fr fr Test max heap insertion
test_start("heap_max_insert")
sus max_h *Heap = heap_max_new(10)
assert_true(heap_insert(max_h, 5))
assert_true(heap_insert(max_h, 3))
assert_true(heap_insert(max_h, 8))
assert_true(heap_insert(max_h, 1))
assert_true(heap_size(max_h) == 4)

fr fr Test max heap peek (should be maximum)
test_start("heap_max_peek")
sus max_root normie = heap_peek(max_h)
assert_true(max_root == 8) fr fr Maximum element should be at root

fr fr Test max heap extraction
test_start("heap_max_extract")
sus max_extracted normie = heap_extract(max_h)
assert_true(max_extracted == 8) fr fr Should extract maximum
assert_true(heap_size(max_h) == 3)
sus next_max normie = heap_peek(max_h)
assert_true(next_max == 5) fr fr Next maximum should be 5

fr fr Test heap capacity limits
test_start("heap_capacity_limits")
sus small_h *Heap = heap_min_new(2)
assert_true(heap_insert(small_h, 10))
assert_true(heap_insert(small_h, 5))
assert_true(heap_is_full(small_h))
assert_false(heap_insert(small_h, 15)) fr fr Should fail when full

fr fr Test empty heap operations
test_start("heap_empty_operations")
sus empty_h *Heap = heap_min_new(5)
assert_true(heap_is_empty(empty_h))
sus empty_peek normie = heap_peek(empty_h)
assert_true(empty_peek == -1) fr fr Error return for empty heap
sus empty_extract normie = heap_extract(empty_h)
assert_true(empty_extract == -1) fr fr Error return for empty heap

fr fr Test heap build from array
test_start("heap_build_from_array")
sus arr normie[value] = [5, 3, 8, 1, 9, 2, 7]
sus build_h *Heap = heap_min_new(10)
assert_true(heap_build_from_array(build_h, arr, 7))
assert_true(heap_size(build_h) == 7)
assert_true(heap_validate(build_h)) fr fr Should maintain heap property

fr fr Test heap validation
test_start("heap_validation")
sus valid_h *Heap = heap_min_new(10)
heap_insert(valid_h, 2)
heap_insert(valid_h, 4)
heap_insert(valid_h, 6)
heap_insert(valid_h, 8)
assert_true(heap_validate(valid_h))

fr fr Test priority queue creation
test_start("pq_new")
sus pq *PriorityQueue = pq_new(10)
assert_true(pq_is_empty(pq))
assert_true(pq_size(pq) == 0)

fr fr Test priority queue operations
test_start("pq_operations")
sus pq_test *PriorityQueue = pq_new(10)
assert_true(pq_enqueue(pq_test, 5))
assert_true(pq_enqueue(pq_test, 2))
assert_true(pq_enqueue(pq_test, 8))
assert_true(pq_enqueue(pq_test, 1))
assert_true(pq_size(pq_test) == 4)
assert_false(pq_is_empty(pq_test))

fr fr Test priority queue peek and dequeue
test_start("pq_peek_dequeue")
sus pq_peek_val normie = pq_peek(pq_test)
assert_true(pq_peek_val == 1) fr fr Highest priority (lowest value)
sus pq_dequeue_val normie = pq_dequeue(pq_test)
assert_true(pq_dequeue_val == 1)
assert_true(pq_size(pq_test) == 3)
sus pq_next_peek normie = pq_peek(pq_test)
assert_true(pq_next_peek == 2) fr fr Next highest priority

fr fr Test kth largest element
test_start("heap_kth_largest")
sus kth_arr normie[value] = [3, 1, 4, 1, 5, 9, 2, 6, 5]
sus kth_3 normie = heap_kth_largest(kth_arr, 9, 3)
assert_true(kth_3 == 5) fr fr 3rd largest should be 5 (9, 6, 5)
sus kth_1 normie = heap_kth_largest(kth_arr, 9, 1)
assert_true(kth_1 == 9) fr fr 1st largest should be 9

fr fr Test heap compare function
test_start("heap_compare")
sus min_heap *Heap = heap_min_new(5)
sus max_heap *Heap = heap_max_new(5)
assert_true(heap_compare(min_heap, 3, 5)) fr fr 3 < 5 for min heap
assert_false(heap_compare(min_heap, 5, 3)) fr fr 5 > 3 for min heap
assert_false(heap_compare(max_heap, 3, 5)) fr fr 3 < 5 for max heap
assert_true(heap_compare(max_heap, 5, 3)) fr fr 5 > 3 for max heap

fr fr Test heap with duplicate values
test_start("heap_duplicates")
sus dup_h *Heap = heap_min_new(10)
heap_insert(dup_h, 5)
heap_insert(dup_h, 5)
heap_insert(dup_h, 3)
heap_insert(dup_h, 5)
heap_insert(dup_h, 3)
assert_true(heap_size(dup_h) == 5)
assert_true(heap_validate(dup_h))
sus dup_min normie = heap_extract(dup_h)
assert_true(dup_min == 3)

fr fr Test heap with single element
test_start("heap_single_element")
sus single_h *Heap = heap_min_new(5)
heap_insert(single_h, 42)
assert_true(heap_size(single_h) == 1)
assert_true(heap_peek(single_h) == 42)
assert_true(heap_validate(single_h))
sus single_extract normie = heap_extract(single_h)
assert_true(single_extract == 42)
assert_true(heap_is_empty(single_h))

fr fr Test heap with reverse sorted input
test_start("heap_reverse_sorted")
sus rev_h *Heap = heap_min_new(10)
heap_insert(rev_h, 9)
heap_insert(rev_h, 8)
heap_insert(rev_h, 7)
heap_insert(rev_h, 6)
heap_insert(rev_h, 5)
assert_true(heap_validate(rev_h))
assert_true(heap_peek(rev_h) == 5) fr fr Minimum should be 5

fr fr Test heap with already sorted input
test_start("heap_sorted_input")
sus sorted_h *Heap = heap_max_new(10)
heap_insert(sorted_h, 1)
heap_insert(sorted_h, 2)
heap_insert(sorted_h, 3)
heap_insert(sorted_h, 4)
heap_insert(sorted_h, 5)
assert_true(heap_validate(sorted_h))
assert_true(heap_peek(sorted_h) == 5) fr fr Maximum should be 5

fr fr Test error conditions
test_start("heap_error_conditions")
sus err_h *Heap = heap_min_new(0) fr fr Zero capacity
assert_true(heap_is_empty(err_h))
assert_true(heap_is_full(err_h))
assert_false(heap_insert(err_h, 1)) fr fr Should fail

sus invalid_kth normie = heap_kth_largest(kth_arr, 9, 0)
assert_true(invalid_kth == -1) fr fr Invalid k
sus invalid_kth2 normie = heap_kth_largest(kth_arr, 9, 10)
assert_true(invalid_kth2 == -1) fr fr k > array size

fr fr Test heap swap functionality
test_start("heap_swap")
sus swap_h *Heap = heap_min_new(5)
heap_insert(swap_h, 10)
heap_insert(swap_h, 5)
sus before_swap_0 normie = swap_h.data[0]
sus before_swap_1 normie = swap_h.data[1]
heap_swap(swap_h, 0, 1)
assert_true(swap_h.data[0] == before_swap_1)
assert_true(swap_h.data[1] == before_swap_0)

fr fr Test complex heap operations sequence
test_start("heap_complex_sequence")
sus complex_h *Heap = heap_min_new(20)
fr fr Insert multiple elements
bestie i := 1; i <= 10; i++ {
    heap_insert(complex_h, i * 3)
}
assert_true(heap_size(complex_h) == 10)
assert_true(heap_validate(complex_h))

fr fr Extract half the elements
bestie i := 0; i < 5; i++ {
    heap_extract(complex_h)
}
assert_true(heap_size(complex_h) == 5)
assert_true(heap_validate(complex_h))

fr fr Insert more elements
bestie i := 1; i <= 5; i++ {
    heap_insert(complex_h, i)
}
assert_true(heap_size(complex_h) == 10)
assert_true(heap_validate(complex_h))

print_test_summary()
