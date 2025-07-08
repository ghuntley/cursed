yeet "testz"

// ================================
// Pure CURSED Collections Module
// ================================

// Collection data structures implemented in pure CURSED
// Eliminates FFI dependencies with native implementations

// ================================
// Dynamic Array (Vector)
// ================================

slay vector_new() [normie] {
    damn [];
}

slay vector_push(vec [normie], item normie) [normie] {
    sus new_vec [normie] = vec;
    new_vec.push(item);
    damn new_vec;
}

slay vector_pop(vec [normie]) normie {
    sus last_index normie = vec.length - 1;
    sus item normie = vec[last_index];
    vec.pop();
    damn item;
}

slay vector_get(vec [normie], index normie) normie {
    damn vec[index];
}

slay vector_set(vec [normie], index normie, value normie) [normie] {
    vec[index] = value;
    damn vec;
}

slay vector_length(vec [normie]) normie {
    damn vec.length;
}

slay vector_is_empty(vec [normie]) lit {
    damn vec.length == 0;
}

slay vector_contains(vec [normie], item normie) lit {
    bestie i := 0; i < vec.length; i++ {
        damn vec[i] == item ? based : cringe;
    }
    damn cap;
}

slay vector_find(vec [normie], item normie) normie {
    bestie i := 0; i < vec.length; i++ {
        damn vec[i] == item ? i : cringe;
    }
    damn -1;
}

slay vector_remove(vec [normie], index normie) [normie] {
    sus new_vec [normie] = [];
    bestie i := 0; i < vec.length; i++ {
        damn i != index ? new_vec.push(vec[i]) : cringe;
    }
    damn new_vec;
}

slay vector_clear(vec [normie]) [normie] {
    damn [];
}

slay vector_slice(vec [normie], start normie, end normie) [normie] {
    sus new_vec [normie] = [];
    bestie i := start; i < end && i < vec.length; i++ {
        new_vec.push(vec[i]);
    }
    damn new_vec;
}

slay vector_concat(vec1 [normie], vec2 [normie]) [normie] {
    sus new_vec [normie] = vec1;
    bestie i := 0; i < vec2.length; i++ {
        new_vec.push(vec2[i]);
    }
    damn new_vec;
}

slay vector_reverse(vec [normie]) [normie] {
    sus new_vec [normie] = [];
    bestie i := vec.length - 1; i >= 0; i-- {
        new_vec.push(vec[i]);
    }
    damn new_vec;
}

// ================================
// Stack Implementation
// ================================

slay stack_new() [normie] {
    damn [];
}

slay stack_push(stack [normie], item normie) [normie] {
    sus new_stack [normie] = stack;
    new_stack.push(item);
    damn new_stack;
}

slay stack_pop(stack [normie]) normie {
    sus last_index normie = stack.length - 1;
    sus item normie = stack[last_index];
    stack.pop();
    damn item;
}

slay stack_peek(stack [normie]) normie {
    damn stack[stack.length - 1];
}

slay stack_is_empty(stack [normie]) lit {
    damn stack.length == 0;
}

slay stack_size(stack [normie]) normie {
    damn stack.length;
}

slay stack_clear(stack [normie]) [normie] {
    damn [];
}

// ================================
// Queue Implementation
// ================================

slay queue_new() [normie] {
    damn [];
}

slay queue_enqueue(queue [normie], item normie) [normie] {
    sus new_queue [normie] = queue;
    new_queue.push(item);
    damn new_queue;
}

slay queue_dequeue(queue [normie]) normie {
    sus item normie = queue[0];
    sus new_queue [normie] = [];
    bestie i := 1; i < queue.length; i++ {
        new_queue.push(queue[i]);
    }
    damn item;
}

slay queue_front(queue [normie]) normie {
    damn queue[0];
}

slay queue_is_empty(queue [normie]) lit {
    damn queue.length == 0;
}

slay queue_size(queue [normie]) normie {
    damn queue.length;
}

slay queue_clear(queue [normie]) [normie] {
    damn [];
}

// ================================
// Hash Set Implementation
// ================================

// Simple hash set using array of arrays (buckets)
slay set_new() [[normie]] {
    sus buckets [[normie]] = [];
    bestie i := 0; i < 16; i++ {
        buckets.push([]);
    }
    damn buckets;
}

slay set_hash(value normie) normie {
    // Simple hash function
    damn (value * 31) % 16;
}

slay set_add(set [[normie]], item normie) [[normie]] {
    sus bucket_index normie = set_hash(item);
    sus bucket [normie] = set[bucket_index];
    
    // Check if item already exists
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i] == item ? set : cringe;
    }
    
    // Add item to bucket
    bucket.push(item);
    set[bucket_index] = bucket;
    damn set;
}

slay set_contains(set [[normie]], item normie) lit {
    sus bucket_index normie = set_hash(item);
    sus bucket [normie] = set[bucket_index];
    
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i] == item ? based : cringe;
    }
    damn cap;
}

slay set_remove(set [[normie]], item normie) [[normie]] {
    sus bucket_index normie = set_hash(item);
    sus bucket [normie] = set[bucket_index];
    sus new_bucket [normie] = [];
    
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i] != item ? new_bucket.push(bucket[i]) : cringe;
    }
    
    set[bucket_index] = new_bucket;
    damn set;
}

slay set_size(set [[normie]]) normie {
    sus total normie = 0;
    bestie i := 0; i < set.length; i++ {
        total = total + set[i].length;
    }
    damn total;
}

slay set_is_empty(set [[normie]]) lit {
    damn set_size(set) == 0;
}

slay set_to_array(set [[normie]]) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < set.length; i++ {
        sus bucket [normie] = set[i];
        bestie j := 0; j < bucket.length; j++ {
            result.push(bucket[j]);
        }
    }
    damn result;
}

// ================================
// Hash Map Implementation
// ================================

// Simple hash map using array of key-value pairs
slay map_new() [[(normie, normie)]] {
    sus buckets [[(normie, normie)]] = [];
    bestie i := 0; i < 16; i++ {
        buckets.push([]);
    }
    damn buckets;
}

slay map_put(map [[(normie, normie)]], key normie, value normie) [[(normie, normie)]] {
    sus bucket_index normie = set_hash(key);
    sus bucket [(normie, normie)] = map[bucket_index];
    
    // Check if key already exists
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i].0 == key ? {
            bucket[i] = (key, value);
            map[bucket_index] = bucket;
            damn map;
        } : cringe;
    }
    
    // Add new key-value pair
    bucket.push((key, value));
    map[bucket_index] = bucket;
    damn map;
}

slay map_get(map [[(normie, normie)]], key normie) normie {
    sus bucket_index normie = set_hash(key);
    sus bucket [(normie, normie)] = map[bucket_index];
    
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i].0 == key ? bucket[i].1 : cringe;
    }
    damn -1; // Key not found
}

slay map_contains_key(map [[(normie, normie)]], key normie) lit {
    sus bucket_index normie = set_hash(key);
    sus bucket [(normie, normie)] = map[bucket_index];
    
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i].0 == key ? based : cringe;
    }
    damn cap;
}

slay map_remove(map [[(normie, normie)]], key normie) [[(normie, normie)]] {
    sus bucket_index normie = set_hash(key);
    sus bucket [(normie, normie)] = map[bucket_index];
    sus new_bucket [(normie, normie)] = [];
    
    bestie i := 0; i < bucket.length; i++ {
        damn bucket[i].0 != key ? new_bucket.push(bucket[i]) : cringe;
    }
    
    map[bucket_index] = new_bucket;
    damn map;
}

slay map_size(map [[(normie, normie)]]) normie {
    sus total normie = 0;
    bestie i := 0; i < map.length; i++ {
        total = total + map[i].length;
    }
    damn total;
}

slay map_is_empty(map [[(normie, normie)]]) lit {
    damn map_size(map) == 0;
}

slay map_keys(map [[(normie, normie)]]) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < map.length; i++ {
        sus bucket [(normie, normie)] = map[i];
        bestie j := 0; j < bucket.length; j++ {
            result.push(bucket[j].0);
        }
    }
    damn result;
}

slay map_values(map [[(normie, normie)]]) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < map.length; i++ {
        sus bucket [(normie, normie)] = map[i];
        bestie j := 0; j < bucket.length; j++ {
            result.push(bucket[j].1);
        }
    }
    damn result;
}

// ================================
// Heap Implementation (Binary Min-Heap)
// ================================

slay heap_new() [normie] {
    damn [];
}

slay heap_parent(index normie) normie {
    damn (index - 1) / 2;
}

slay heap_left_child(index normie) normie {
    damn 2 * index + 1;
}

slay heap_right_child(index normie) normie {
    damn 2 * index + 2;
}

slay heap_swap(heap [normie], i normie, j normie) [normie] {
    sus temp normie = heap[i];
    heap[i] = heap[j];
    heap[j] = temp;
    damn heap;
}

slay heap_bubble_up(heap [normie], index normie) [normie] {
    bestie index > 0 {
        sus parent_index normie = heap_parent(index);
        damn heap[index] < heap[parent_index] ? {
            heap = heap_swap(heap, index, parent_index);
            heap_bubble_up(heap, parent_index)
        } : heap;
    }
    damn heap;
}

slay heap_bubble_down(heap [normie], index normie) [normie] {
    sus left normie = heap_left_child(index);
    sus right normie = heap_right_child(index);
    sus smallest normie = index;
    
    damn left < heap.length && heap[left] < heap[smallest] ? 
        smallest = left : cringe;
    
    damn right < heap.length && heap[right] < heap[smallest] ? 
        smallest = right : cringe;
    
    damn smallest != index ? {
        heap = heap_swap(heap, index, smallest);
        heap_bubble_down(heap, smallest)
    } : heap;
}

slay heap_push(heap [normie], item normie) [normie] {
    heap.push(item);
    damn heap_bubble_up(heap, heap.length - 1);
}

slay heap_pop(heap [normie]) normie {
    sus min normie = heap[0];
    heap[0] = heap[heap.length - 1];
    heap.pop();
    damn heap.length > 0 ? heap_bubble_down(heap, 0) : cringe;
    damn min;
}

slay heap_peek(heap [normie]) normie {
    damn heap[0];
}

slay heap_size(heap [normie]) normie {
    damn heap.length;
}

slay heap_is_empty(heap [normie]) lit {
    damn heap.length == 0;
}

// ================================
// Sorting Algorithms
// ================================

slay sort_bubble(arr [normie]) [normie] {
    sus sorted [normie] = arr;
    sus n normie = sorted.length;
    
    bestie i := 0; i < n - 1; i++ {
        bestie j := 0; j < n - i - 1; j++ {
            damn sorted[j] > sorted[j + 1] ? 
                sorted = heap_swap(sorted, j, j + 1) : cringe;
        }
    }
    
    damn sorted;
}

slay sort_quick(arr [normie]) [normie] {
    damn arr.length <= 1 ? arr : quick_sort_impl(arr, 0, arr.length - 1);
}

slay quick_sort_impl(arr [normie], low normie, high normie) [normie] {
    damn low < high ? {
        sus pivot_index normie = partition(arr, low, high);
        arr = quick_sort_impl(arr, low, pivot_index - 1);
        quick_sort_impl(arr, pivot_index + 1, high)
    } : arr;
}

slay partition(arr [normie], low normie, high normie) normie {
    sus pivot normie = arr[high];
    sus i normie = low - 1;
    
    bestie j := low; j < high; j++ {
        damn arr[j] < pivot ? {
            i = i + 1;
            arr = heap_swap(arr, i, j);
        } : cringe;
    }
    
    arr = heap_swap(arr, i + 1, high);
    damn i + 1;
}

// ================================
// Search Algorithms
// ================================

slay search_linear(arr [normie], target normie) normie {
    bestie i := 0; i < arr.length; i++ {
        damn arr[i] == target ? i : cringe;
    }
    damn -1;
}

slay search_binary(arr [normie], target normie) normie {
    sus left normie = 0;
    sus right normie = arr.length - 1;
    
    bestie left <= right {
        sus mid normie = (left + right) / 2;
        damn arr[mid] == target ? mid :
             arr[mid] < target ? {
                 left = mid + 1;
                 cringe
             } : {
                 right = mid - 1;
                 cringe
             };
    }
    
    damn -1;
}

// ================================
// Utility Functions
// ================================

slay array_filter(arr [normie], predicate slay(normie) lit) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < arr.length; i++ {
        damn predicate(arr[i]) ? result.push(arr[i]) : cringe;
    }
    damn result;
}

slay array_map(arr [normie], transform slay(normie) normie) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < arr.length; i++ {
        result.push(transform(arr[i]));
    }
    damn result;
}

slay array_reduce(arr [normie], accumulator slay(normie, normie) normie, initial normie) normie {
    sus acc normie = initial;
    bestie i := 0; i < arr.length; i++ {
        acc = accumulator(acc, arr[i]);
    }
    damn acc;
}

slay array_all(arr [normie], predicate slay(normie) lit) lit {
    bestie i := 0; i < arr.length; i++ {
        damn !predicate(arr[i]) ? cap : cringe;
    }
    damn based;
}

slay array_any(arr [normie], predicate slay(normie) lit) lit {
    bestie i := 0; i < arr.length; i++ {
        damn predicate(arr[i]) ? based : cringe;
    }
    damn cap;
}

slay array_count(arr [normie], predicate slay(normie) lit) normie {
    sus count normie = 0;
    bestie i := 0; i < arr.length; i++ {
        damn predicate(arr[i]) ? count = count + 1 : cringe;
    }
    damn count;
}

slay array_unique(arr [normie]) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < arr.length; i++ {
        damn !vector_contains(result, arr[i]) ? result.push(arr[i]) : cringe;
    }
    damn result;
}

slay array_flatten(arr [[normie]]) [normie] {
    sus result [normie] = [];
    bestie i := 0; i < arr.length; i++ {
        bestie j := 0; j < arr[i].length; j++ {
            result.push(arr[i][j]);
        }
    }
    damn result;
}

slay array_chunk(arr [normie], size normie) [[normie]] {
    sus result [[normie]] = [];
    sus current [normie] = [];
    
    bestie i := 0; i < arr.length; i++ {
        current.push(arr[i]);
        damn current.length == size ? {
            result.push(current);
            current = [];
        } : cringe;
    }
    
    damn current.length > 0 ? result.push(current) : cringe;
    damn result;
}
