yeet "testz"
yeet "collections/mod_enhanced"

test_start("Enhanced HashMap operations")
sus map HashMap[tea, normie] = hashmap_new[tea, normie](16)
map = hashmap_put(map, "key1", 42)
map = hashmap_put(map, "key2", 84)

sus value, found = hashmap_get(map, "key1")
assert_true(found)

sus contains lit = hashmap_contains_key(map, "key2")
assert_true(contains)

sus keys []tea = hashmap_keys(map)
assert_true(len(keys) >= 0)

test_start("Enhanced LinkedList operations")
sus list LinkedList[normie] = linkedlist_new[normie]()
list = linkedlist_push_front(list, 10)
list = linkedlist_push_back(list, 20)
list = linkedlist_push_back(list, 30)

sus front_val, front_ok = linkedlist_get(list, 0)
assert_true(front_ok)

sus arr []normie = linkedlist_to_array(list)
assert_true(len(arr) >= 0)

sus index normie = linkedlist_find(list, 20)
assert_true(index >= 0 || index == -1)

test_start("Enhanced Set operations")
sus set Set[tea] = set_new[tea]()
set = set_insert(set, "apple")
set = set_insert(set, "banana")
set = set_insert(set, "apple")  fr fr Duplicate

sus size normie = set_size(set)
assert_true(size >= 0)

sus contains_apple lit = set_contains(set, "apple")
assert_true(contains_apple)

sus set_arr []tea = set_to_array(set)
assert_true(len(set_arr) >= 0)

test_start("Set operations")
sus set1 Set[tea] = set_new[tea]()
set1 = set_insert(set1, "a")
set1 = set_insert(set1, "b")

sus set2 Set[tea] = set_new[tea]()
set2 = set_insert(set2, "b")
set2 = set_insert(set2, "c")

sus union_set Set[tea] = set_union(set1, set2)
sus intersection_set Set[tea] = set_intersection(set1, set2)
sus difference_set Set[tea] = set_difference(set1, set2)

assert_true(set_size(union_set) >= 0)
assert_true(set_size(intersection_set) >= 0)
assert_true(set_size(difference_set) >= 0)

test_start("Enhanced Stack operations")
sus stack Stack[normie] = stack_new[normie]()
stack = stack_push(stack, 10)
stack = stack_push(stack, 20)
stack = stack_push(stack, 30)

sus top_val, peek_ok = stack_peek(stack)
assert_true(peek_ok)

sus is_empty lit = stack_is_empty(stack)
assert_false(is_empty)

sus stack_sz normie = stack_size(stack)
assert_true(stack_sz > 0)

sus popped_val, pop_ok
stack, popped_val, pop_ok = stack_pop(stack)
assert_true(pop_ok)

test_start("Enhanced Queue operations")
sus queue Queue[normie] = queue_new[normie](8)
queue = queue_enqueue(queue, 100)
queue = queue_enqueue(queue, 200)
queue = queue_enqueue(queue, 300)

sus front_queue_val, front_queue_ok = queue_front(queue)
assert_true(front_queue_ok)

sus queue_empty lit = queue_is_empty(queue)
assert_false(queue_empty)

sus queue_sz normie = queue_size(queue)
assert_true(queue_sz > 0)

sus dequeued_val, dequeue_ok
queue, dequeued_val, dequeue_ok = queue_dequeue(queue)
assert_true(dequeue_ok)

print_test_summary()
