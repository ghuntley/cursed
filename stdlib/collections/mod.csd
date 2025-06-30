// Standard collections library

// ================================
// Array/Vector operations
// ================================

fn array_new() -> array {
    return [];
}

fn array_with_capacity(capacity: int) -> array {
    return collections_array_with_capacity(capacity);
}

fn array_push(arr: array, item: any) -> array {
    collections_array_push(arr, item);
    return arr;
}

fn array_pop(arr: array) -> any {
    return collections_array_pop(arr);
}

fn array_insert(arr: array, index: int, item: any) -> array {
    collections_array_insert(arr, index, item);
    return arr;
}

fn array_remove(arr: array, index: int) -> any {
    return collections_array_remove(arr, index);
}

fn array_get(arr: array, index: int) -> any {
    return collections_array_get(arr, index);
}

fn array_set(arr: array, index: int, value: any) -> array {
    collections_array_set(arr, index, value);
    return arr;
}

fn array_len(arr: array) -> int {
    return collections_array_len(arr);
}

fn array_is_empty(arr: array) -> bool {
    return collections_array_is_empty(arr);
}

fn array_clear(arr: array) -> array {
    collections_array_clear(arr);
    return arr;
}

fn array_contains(arr: array, item: any) -> bool {
    return collections_array_contains(arr, item);
}

fn array_index_of(arr: array, item: any) -> int {
    return collections_array_index_of(arr, item);
}

fn array_reverse(arr: array) -> array {
    collections_array_reverse(arr);
    return arr;
}

fn array_sort(arr: array) -> array {
    collections_array_sort(arr);
    return arr;
}

fn array_slice(arr: array, start: int, end: int) -> array {
    return collections_array_slice(arr, start, end);
}

fn array_concat(arr1: array, arr2: array) -> array {
    return collections_array_concat(arr1, arr2);
}

fn array_join(arr: array, separator: string) -> string {
    return collections_array_join(arr, separator);
}

fn array_filter(arr: array, predicate: fn) -> array {
    return collections_array_filter(arr, predicate);
}

fn array_map(arr: array, mapper: fn) -> array {
    return collections_array_map(arr, mapper);
}

fn array_reduce(arr: array, reducer: fn, initial: any) -> any {
    return collections_array_reduce(arr, reducer, initial);
}

fn array_find(arr: array, predicate: fn) -> any {
    return collections_array_find(arr, predicate);
}

fn array_find_index(arr: array, predicate: fn) -> int {
    return collections_array_find_index(arr, predicate);
}

fn array_any(arr: array, predicate: fn) -> bool {
    return collections_array_any(arr, predicate);
}

fn array_all(arr: array, predicate: fn) -> bool {
    return collections_array_all(arr, predicate);
}

// ================================
// HashMap/Map operations  
// ================================

fn map_new() -> map {
    return {};
}

fn map_with_capacity(capacity: int) -> map {
    return collections_map_with_capacity(capacity);
}

fn map_set(m: map, key: string, value: any) -> map {
    collections_map_set(m, key, value);
    return m;
}

fn map_get(m: map, key: string) -> any {
    return collections_map_get(m, key);
}

fn map_get_or_default(m: map, key: string, default: any) -> any {
    return collections_map_get_or_default(m, key, default);
}

fn map_remove(m: map, key: string) -> any {
    return collections_map_remove(m, key);
}

fn map_contains_key(m: map, key: string) -> bool {
    return collections_map_contains_key(m, key);
}

fn map_keys(m: map) -> array {
    return collections_map_keys(m);
}

fn map_values(m: map) -> array {
    return collections_map_values(m);
}

fn map_entries(m: map) -> array {
    return collections_map_entries(m);
}

fn map_len(m: map) -> int {
    return collections_map_len(m);
}

fn map_is_empty(m: map) -> bool {
    return collections_map_is_empty(m);
}

fn map_clear(m: map) -> map {
    collections_map_clear(m);
    return m;
}

fn map_merge(m1: map, m2: map) -> map {
    return collections_map_merge(m1, m2);
}

fn map_filter(m: map, predicate: fn) -> map {
    return collections_map_filter(m, predicate);
}

fn map_map_values(m: map, mapper: fn) -> map {
    return collections_map_map_values(m, mapper);
}

// ================================
// Set operations
// ================================

fn set_new() -> set {
    return collections_set_new();
}

fn set_with_capacity(capacity: int) -> set {
    return collections_set_with_capacity(capacity);
}

fn set_add(s: set, item: any) -> set {
    collections_set_add(s, item);
    return s;
}

fn set_remove(s: set, item: any) -> bool {
    return collections_set_remove(s, item);
}

fn set_contains(s: set, item: any) -> bool {
    return collections_set_contains(s, item);
}

fn set_len(s: set) -> int {
    return collections_set_len(s);
}

fn set_is_empty(s: set) -> bool {
    return collections_set_is_empty(s);
}

fn set_clear(s: set) -> set {
    collections_set_clear(s);
    return s;
}

fn set_union(s1: set, s2: set) -> set {
    return collections_set_union(s1, s2);
}

fn set_intersection(s1: set, s2: set) -> set {
    return collections_set_intersection(s1, s2);
}

fn set_difference(s1: set, s2: set) -> set {
    return collections_set_difference(s1, s2);
}

fn set_is_subset(s1: set, s2: set) -> bool {
    return collections_set_is_subset(s1, s2);
}

fn set_is_superset(s1: set, s2: set) -> bool {
    return collections_set_is_superset(s1, s2);
}

fn set_to_array(s: set) -> array {
    return collections_set_to_array(s);
}

fn set_from_array(arr: array) -> set {
    return collections_set_from_array(arr);
}

// ================================
// Queue operations
// ================================

fn queue_new() -> queue {
    return collections_queue_new();
}

fn queue_enqueue(q: queue, item: any) -> queue {
    collections_queue_enqueue(q, item);
    return q;
}

fn queue_dequeue(q: queue) -> any {
    return collections_queue_dequeue(q);
}

fn queue_front(q: queue) -> any {
    return collections_queue_front(q);
}

fn queue_back(q: queue) -> any {
    return collections_queue_back(q);
}

fn queue_len(q: queue) -> int {
    return collections_queue_len(q);
}

fn queue_is_empty(q: queue) -> bool {
    return collections_queue_is_empty(q);
}

fn queue_clear(q: queue) -> queue {
    collections_queue_clear(q);
    return q;
}

// ================================
// Stack operations
// ================================

fn stack_new() -> stack {
    return collections_stack_new();
}

fn stack_push(s: stack, item: any) -> stack {
    collections_stack_push(s, item);
    return s;
}

fn stack_pop(s: stack) -> any {
    return collections_stack_pop(s);
}

fn stack_peek(s: stack) -> any {
    return collections_stack_peek(s);
}

fn stack_len(s: stack) -> int {
    return collections_stack_len(s);
}

fn stack_is_empty(s: stack) -> bool {
    return collections_stack_is_empty(s);
}

fn stack_clear(s: stack) -> stack {
    collections_stack_clear(s);
    return s;
}

// ================================
// Utility functions
// ================================

fn range(start: int, end: int) -> array {
    return collections_range(start, end);
}

fn range_step(start: int, end: int, step: int) -> array {
    return collections_range_step(start, end, step);
}

fn zip(arr1: array, arr2: array) -> array {
    return collections_zip(arr1, arr2);
}

fn flatten(nested_arr: array) -> array {
    return collections_flatten(nested_arr);
}

fn unique(arr: array) -> array {
    return collections_unique(arr);
}

fn count_occurrences(arr: array, item: any) -> int {
    return collections_count_occurrences(arr, item);
}

fn group_by(arr: array, key_fn: fn) -> map {
    return collections_group_by(arr, key_fn);
}

fn partition(arr: array, predicate: fn) -> array {
    return collections_partition(arr, predicate);
}
