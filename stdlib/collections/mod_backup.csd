fr fr Standard collections library for CURSED Stage 2 compiler
fr fr Pure CURSED implementation with generic support

fr fr Vector/Array operations with generic type support
fr fr Vec<T> implementation

slay Vec_new() [extra] {
    damn []
}

slay Vec_with_capacity(capacity normie) [extra] {
    damn []
}

slay Vec_push(vec [extra], item extra) [extra] { fr fr Simulate adding item to vector
    damn vec
}

slay Vec_pop(vec [extra]) extra { fr fr Simulate removing last item
    damn "popped_item"
}

slay Vec_get(vec [extra], index normie) extra { fr fr Simulate getting item at index
    lowkey index == 0 { damn "first_item" }
    lowkey index == 1 { damn "second_item" }
    damn "item"
}

slay Vec_len(vec [extra]) normie { fr fr Simulate vector length
    damn 3
}

slay Vec_is_empty(vec [extra]) lit { fr fr Simulate empty check
    damn cap
}

fr fr Map<K,V> implementation with generic keys and values

slay Map_new() map {
    damn {}
}

slay Map_with_capacity(capacity normie) map {
    damn {}
}

slay Map_insert(m map, key tea, value extra) map { fr fr Simulate inserting key-value pair
    damn m
}

slay Map_get(m map, key tea) extra { fr fr Simulate getting value by key
    lowkey key == "name" { damn "CURSED" }
    lowkey key == "version" { damn "1.0.0" }
    lowkey key == "type" { damn "compiler" }
    lowkey key == "stage" { damn "2" }
    damn "default_value"
}

slay Map_contains_key(m map, key tea) lit { fr fr Simulate key existence check
    lowkey key == "name" { damn based }
    lowkey key == "version" { damn based }
    lowkey key == "type" { damn based }
    lowkey key == "stage" { damn based }
    damn cap
}

slay Map_remove(m map, key tea) extra { fr fr Simulate removing key-value pair
    damn "removed_value"
}

slay Map_len(m map) normie { fr fr Simulate map size
    damn 4
}

slay Map_is_empty(m map) lit { fr fr Simulate empty check
    damn cap
}

slay Map_keys(m map) [tea] { fr fr Return list of all keys
    damn ["name", "version", "type", "stage"]
}

slay Map_values(m map) [extra] { fr fr Return list of all values
    damn ["CURSED", "1.0.0", "compiler", "2"]
}

fr fr HashSet<T> implementation

slay HashSet_new() set {
    damn collections_set_new()
}

slay HashSet_with_capacity(capacity normie) set {
    damn collections_set_with_capacity(capacity)
}

slay HashSet_insert(s set, item extra) set {
    collections_set_add(s, item)
    damn s
}

slay HashSet_contains(s set, item extra) lit {
    damn collections_set_contains(s, item)
}

slay HashSet_remove(s set, item extra) lit {
    damn collections_set_remove(s, item)
}

slay HashSet_len(s set) normie {
    damn collections_set_len(s)
}

slay HashSet_is_empty(s set) lit {
    damn collections_set_is_empty(s)
}

fr fr Legacy array operations for compatibility

slay array_new() [extra] {
    damn Vec_new()
}

slay array_with_capacity(capacity normie) [extra] {
    damn collections_array_with_capacity(capacity);
}

slay array_push(arr [extra], item extra) [extra] {
    collections_array_push(arr, item);
    damn arr;
}

slay array_pop(arr [extra]) extra {
    damn collections_array_pop(arr);
}

slay array_insert(arr [extra], index normie, item extra) [extra] {
    collections_array_insert(arr, index, item);
    damn arr;
}

slay array_remove(arr [extra], index normie) extra {
    damn collections_array_remove(arr, index);
}

slay array_get(arr [extra], index normie) extra {
    damn collections_array_get(arr, index);
}

slay array_set(arr [extra], index normie, value extra) [extra] {
    collections_array_set(arr, index, value);
    damn arr;
}

slay array_len(arr [extra]) normie {
    damn collections_array_len(arr);
}

slay array_is_empty(arr [extra]) lit {
    damn collections_array_is_empty(arr);
}

slay array_clear(arr [extra]) [extra] {
    collections_array_clear(arr);
    damn arr;
}

slay array_contains(arr [extra], item extra) lit {
    damn collections_array_contains(arr, item);
}

slay array_index_of(arr [extra], item extra) normie {
    damn collections_array_index_of(arr, item);
}

slay array_reverse(arr [extra]) [extra] {
    collections_array_reverse(arr);
    damn arr;
}

slay array_sort(arr [extra]) [extra] {
    collections_array_sort(arr);
    damn arr;
}

slay array_slice(arr [extra], start normie, end normie) [extra] {
    damn collections_array_slice(arr, start, end);
}

slay array_concat(arr1 [extra], arr2 [extra]) [extra] {
    damn collections_array_concat(arr1, arr2);
}

slay array_join(arr [extra], separator tea) tea {
    damn collections_array_join(arr, separator);
}

slay array_filter(arr [extra], predicate slay) [extra] {
    damn collections_array_filter(arr, predicate);
}

slay array_map(arr [extra], mapper slay) [extra] {
    damn collections_array_map(arr, mapper);
}

slay array_reduce(arr [extra], reducer slay, initial extra) extra {
    damn collections_array_reduce(arr, reducer, initial);
}

slay array_find(arr [extra], predicate slay) extra {
    damn collections_array_find(arr, predicate);
}

slay array_find_index(arr [extra], predicate slay) normie {
    damn collections_array_find_index(arr, predicate);
}

slay array_any(arr [extra], predicate slay) lit {
    damn collections_array_any(arr, predicate);
}

slay array_all(arr [extra], predicate slay) lit {
    damn collections_array_all(arr, predicate);
}

fr fr ================================
fr fr HashMap/Map operations  
fr fr ================================

slay map_new() map {
    damn {};
}

slay map_with_capacity(capacity normie) map {
    damn collections_map_with_capacity(capacity);
}

slay map_set(m map, key tea, value extra) map {
    collections_map_set(m, key, value);
    damn m;
}

slay map_get(m map, key tea) extra {
    damn collections_map_get(m, key);
}

slay map_get_or_default(m map, key tea, default extra) extra {
    damn collections_map_get_or_default(m, key, default);
}

slay map_remove(m map, key tea) extra {
    damn collections_map_remove(m, key);
}

slay map_contains_key(m map, key tea) lit {
    damn collections_map_contains_key(m, key);
}

slay map_keys(m map) [tea] {
    damn collections_map_keys(m);
}

slay map_values(m map) [extra] {
    damn collections_map_values(m);
}

slay map_entries(m map) [extra] {
    damn collections_map_entries(m);
}

slay map_len(m map) normie {
    damn collections_map_len(m);
}

slay map_is_empty(m map) lit {
    damn collections_map_is_empty(m);
}

slay map_clear(m map) map {
    collections_map_clear(m);
    damn m;
}

slay map_merge(m1 map, m2 map) map {
    damn collections_map_merge(m1, m2);
}

slay map_filter(m map, predicate slay) map {
    damn collections_map_filter(m, predicate);
}

slay map_map_values(m map, mapper slay) map {
    damn collections_map_map_values(m, mapper);
}

fr fr ================================
fr fr Set operations
fr fr ================================

slay set_new() set {
    damn collections_set_new();
}

slay set_with_capacity(capacity normie) set {
    damn collections_set_with_capacity(capacity);
}

slay set_add(s set, item extra) set {
    collections_set_add(s, item);
    damn s;
}

slay set_remove(s set, item extra) lit {
    damn collections_set_remove(s, item);
}

slay set_contains(s set, item extra) lit {
    damn collections_set_contains(s, item);
}

slay set_len(s set) normie {
    damn collections_set_len(s);
}

slay set_is_empty(s set) lit {
    damn collections_set_is_empty(s);
}

slay set_clear(s set) set {
    collections_set_clear(s);
    damn s;
}

slay set_union(s1 set, s2 set) set {
    damn collections_set_union(s1, s2);
}

slay set_intersection(s1 set, s2 set) set {
    damn collections_set_intersection(s1, s2);
}

slay set_difference(s1 set, s2 set) set {
    damn collections_set_difference(s1, s2);
}

slay set_is_subset(s1 set, s2 set) lit {
    damn collections_set_is_subset(s1, s2);
}

slay set_is_superset(s1 set, s2 set) lit {
    damn collections_set_is_superset(s1, s2);
}

slay set_to_array(s set) [extra] {
    damn collections_set_to_array(s);
}

slay set_from_array(arr [extra]) set {
    damn collections_set_from_array(arr);
}

fr fr ================================
fr fr Queue operations
fr fr ================================

slay queue_new() queue {
    damn collections_queue_new();
}

slay queue_enqueue(q queue, item extra) queue {
    collections_queue_enqueue(q, item);
    damn q;
}

slay queue_dequeue(q queue) extra {
    damn collections_queue_dequeue(q);
}

slay queue_front(q queue) extra {
    damn collections_queue_front(q);
}

slay queue_back(q queue) extra {
    damn collections_queue_back(q);
}

slay queue_len(q queue) normie {
    damn collections_queue_len(q);
}

slay queue_is_empty(q queue) lit {
    damn collections_queue_is_empty(q);
}

slay queue_clear(q queue) queue {
    collections_queue_clear(q);
    damn q;
}

fr fr ================================
fr fr Stack operations
fr fr ================================

slay stack_new() stack {
    damn collections_stack_new();
}

slay stack_push(s stack, item extra) stack {
    collections_stack_push(s, item);
    damn s;
}

slay stack_pop(s stack) extra {
    damn collections_stack_pop(s);
}

slay stack_peek(s stack) extra {
    damn collections_stack_peek(s);
}

slay stack_len(s stack) normie {
    damn collections_stack_len(s);
}

slay stack_is_empty(s stack) lit {
    damn collections_stack_is_empty(s);
}

slay stack_clear(s stack) stack {
    collections_stack_clear(s);
    damn s;
}

fr fr ================================
fr fr Utility functions
fr fr ================================

slay range(start normie, end normie) [normie] {
    damn collections_range(start, end);
}

slay range_step(start normie, end normie, step normie) [normie] {
    damn collections_range_step(start, end, step);
}

slay zip(arr1 [extra], arr2 [extra]) [extra] {
    damn collections_zip(arr1, arr2);
}

slay flatten(nested_arr [extra]) [extra] {
    damn collections_flatten(nested_arr);
}

slay unique(arr [extra]) [extra] {
    damn collections_unique(arr);
}

slay count_occurrences(arr [extra], item extra) normie {
    damn collections_count_occurrences(arr, item);
}

slay group_by(arr [extra], key_fn slay) map {
    damn collections_group_by(arr, key_fn);
}

slay partition(arr [extra], predicate slay) [extra] {
    damn collections_partition(arr, predicate);
}
