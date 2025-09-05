yeet "testz"
yeet "pure_collections"

test_start("Pure CURSED Collections Module Tests")

// Test Vector operations
sus vec [normie] = vector_new();
assert_true(vector_is_empty(vec))
assert_eq_int(vector_length(vec), 0)

vec = vector_push(vec, 10);
vec = vector_push(vec, 20);
vec = vector_push(vec, 30);

assert_false(vector_is_empty(vec))
assert_eq_int(vector_length(vec), 3)
assert_eq_int(vector_get(vec, 0), 10)
assert_eq_int(vector_get(vec, 1), 20)
assert_eq_int(vector_get(vec, 2), 30)

assert_true(vector_contains(vec, 20))
assert_false(vector_contains(vec, 40))

assert_eq_int(vector_find(vec, 20), 1)
assert_eq_int(vector_find(vec, 40), -1)

vec = vector_set(vec, 1, 25);
assert_eq_int(vector_get(vec, 1), 25)

sus popped normie = vector_pop(vec);
assert_eq_int(popped, 30)
assert_eq_int(vector_length(vec), 2)

sus slice [normie] = vector_slice(vec, 0, 2);
assert_eq_int(vector_length(slice), 2)

vec = vector_remove(vec, 0);
assert_eq_int(vector_length(vec), 1)
assert_eq_int(vector_get(vec, 0), 25)

sus vec2 [normie] = [40, 50];
sus concatenated [normie] = vector_concat(vec, vec2);
assert_eq_int(vector_length(concatenated), 3)

sus reversed [normie] = vector_reverse(concatenated);
assert_eq_int(vector_get(reversed, 0), 50)

vec = vector_clear(vec);
assert_true(vector_is_empty(vec))

// Test Stack operations
sus stack [normie] = stack_new();
assert_true(stack_is_empty(stack))

stack = stack_push(stack, 100);
stack = stack_push(stack, 200);

assert_false(stack_is_empty(stack))
assert_eq_int(stack_size(stack), 2)
assert_eq_int(stack_peek(stack), 200)

sus stack_popped normie = stack_pop(stack);
assert_eq_int(stack_popped, 200)
assert_eq_int(stack_size(stack), 1)

stack = stack_clear(stack);
assert_true(stack_is_empty(stack))

// Test Queue operations
sus queue [normie] = queue_new();
assert_true(queue_is_empty(queue))

queue = queue_enqueue(queue, 100);
queue = queue_enqueue(queue, 200);
queue = queue_enqueue(queue, 300);

assert_false(queue_is_empty(queue))
assert_eq_int(queue_size(queue), 3)
assert_eq_int(queue_front(queue), 100)

sus dequeued normie = queue_dequeue(queue);
assert_eq_int(dequeued, 100)
assert_eq_int(queue_size(queue), 2)
assert_eq_int(queue_front(queue), 200)

queue = queue_clear(queue);
assert_true(queue_is_empty(queue))

// Test Set operations
sus set [[normie]] = set_new();
assert_true(set_is_empty(set))

set = set_add(set, 10);
set = set_add(set, 20);
set = set_add(set, 10); // Duplicate

assert_false(set_is_empty(set))
assert_eq_int(set_size(set), 2) // Should still be 2

assert_true(set_contains(set, 10))
assert_true(set_contains(set, 20))
assert_false(set_contains(set, 30))

set = set_remove(set, 10);
assert_false(set_contains(set, 10))
assert_eq_int(set_size(set), 1)

sus set_array [normie] = set_to_array(set);
assert_eq_int(set_array.length, 1)

// Test Map operations
sus map [[(normie, normie)]] = map_new();
assert_true(map_is_empty(map))

map = map_put(map, 1, 100);
map = map_put(map, 2, 200);
map = map_put(map, 3, 300);

assert_false(map_is_empty(map))
assert_eq_int(map_size(map), 3)

assert_true(map_contains_key(map, 1))
assert_false(map_contains_key(map, 4))

assert_eq_int(map_get(map, 1), 100)
assert_eq_int(map_get(map, 2), 200)
assert_eq_int(map_get(map, 4), -1) // Not found

map = map_put(map, 1, 150); // Update existing key
assert_eq_int(map_get(map, 1), 150)

map = map_remove(map, 2);
assert_false(map_contains_key(map, 2))
assert_eq_int(map_size(map), 2)

sus keys [normie] = map_keys(map);
assert_eq_int(keys.length, 2)

sus values [normie] = map_values(map);
assert_eq_int(values.length, 2)

// Test Heap operations
sus heap [normie] = heap_new();
assert_true(heap_is_empty(heap))

heap = heap_push(heap, 30);
heap = heap_push(heap, 10);
heap = heap_push(heap, 20);
heap = heap_push(heap, 5);

assert_false(heap_is_empty(heap))
assert_eq_int(heap_size(heap), 4)
assert_eq_int(heap_peek(heap), 5) // Min heap

sus min1 normie = heap_pop(heap);
assert_eq_int(min1, 5)

sus min2 normie = heap_pop(heap);
assert_eq_int(min2, 10)

assert_eq_int(heap_size(heap), 2)

// Test Sorting algorithms
sus unsorted [normie] = [64, 34, 25, 12, 22, 11, 90];
sus bubble_sorted [normie] = sort_bubble(unsorted);

assert_eq_int(bubble_sorted[0], 11)
assert_eq_int(bubble_sorted[1], 12)
assert_eq_int(bubble_sorted[6], 90)

sus quick_sorted [normie] = sort_quick(unsorted);
assert_eq_int(quick_sorted[0], 11)
assert_eq_int(quick_sorted[1], 12)
assert_eq_int(quick_sorted[6], 90)

// Test Search algorithms
sus sorted_array [normie] = [1, 3, 5, 7, 9, 11, 13, 15];

assert_eq_int(search_linear(sorted_array, 7), 3)
assert_eq_int(search_linear(sorted_array, 16), -1)

assert_eq_int(search_binary(sorted_array, 7), 3)
assert_eq_int(search_binary(sorted_array, 16), -1)

// Test Utility functions
sus numbers [normie] = [1, 2, 3, 4, 5];

// Test filter (even numbers)
sus evens [normie] = array_filter(numbers, slay(x normie) lit { damn x % 2 == 0; });
assert_eq_int(evens.length, 2)

// Test map (double values)
sus doubled [normie] = array_map(numbers, slay(x normie) normie { damn x * 2; });
assert_eq_int(doubled[0], 2)
assert_eq_int(doubled[4], 10)

// Test reduce (sum)
sus total normie = array_reduce(numbers, slay(acc normie, x normie) normie { damn acc + x; }, 0);
assert_eq_int(total, 15)

// Test all/any
assert_true(array_all(numbers, slay(x normie) lit { damn x > 0; }))
assert_false(array_all(numbers, slay(x normie) lit { damn x > 3; }))

assert_true(array_any(numbers, slay(x normie) lit { damn x > 4; }))
assert_false(array_any(numbers, slay(x normie) lit { damn x > 10; }))

// Test count
sus count normie = array_count(numbers, slay(x normie) lit { damn x > 3; });
assert_eq_int(count, 2)

// Test unique
sus duplicates [normie] = [1, 2, 2, 3, 3, 3];
sus unique [normie] = array_unique(duplicates);
assert_eq_int(unique.length, 3)

// Test flatten
sus nested [[normie]] = [[1, 2], [3, 4], [5]];
sus flattened [normie] = array_flatten(nested);
assert_eq_int(flattened.length, 5)
assert_eq_int(flattened[0], 1)
assert_eq_int(flattened[4], 5)

// Test chunk
sus data [normie] = [1, 2, 3, 4, 5, 6, 7];
sus chunks [[normie]] = array_chunk(data, 3);
assert_eq_int(chunks.length, 3)
assert_eq_int(chunks[0].length, 3)
assert_eq_int(chunks[2].length, 1)

print_test_summary()
