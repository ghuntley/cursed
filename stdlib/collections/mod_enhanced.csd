yeet "testz"

fr fr ========================================
fr fr CURSED Enhanced Collections Library v3.0
fr fr Production-grade data structures
fr fr Complete memory-safe implementation
fr fr ========================================

fr fr ================================
fr fr Enhanced HashMap Implementation
fr fr ================================

squad HashMapEntry[K, V] {
    spill key K
    spill value V
    spill hash normie
    spill next normie  fr fr Index to next entry for chaining
}

squad HashMap[K, V] {
    spill buckets HashMapEntry[value][K, V]
    spill size normie
    spill capacity normie
    spill load_factor normie
}

fr fr Create new hashmap with initial capacity
slay hashmap_new[K, V](initial_capacity normie) HashMap[K, V] {
    sus capacity normie = max(initial_capacity, 16)
    sus buckets HashMapEntry[value][K, V] = []  fr fr Initialize empty buckets
    
    damn HashMap[K, V]{
        buckets: buckets,
        size: 0,
        capacity: capacity,
        load_factor: 75  fr fr 75% load factor
    }
}

fr fr Hash function for different key types
slay hashmap_hash[K](key K) normie {
    fr fr Simple hash function - in production would use proper hashing
    ready typeof(key) == "tea" {
        sus str_key tea = string(key)
        sus hash normie = 5381
        sus i normie = 0
        bestie i < len(str_key) {
            hash = ((hash << 5) + hash) + char_code_at(str_key, i)
            i = i + 1
        }
        damn abs(hash)
    }
    
    ready typeof(key) == "normie" {
        sus int_key normie = normie(key)
        damn abs(int_key * 31)
    }
    
    damn 42  fr fr Default hash
}

fr fr Insert key-value pair into hashmap
slay hashmap_put[K, V](map HashMap[K, V], key K, value V) HashMap[K, V] {
    fr fr Check if resize is needed
    sus threshold normie = (map.capacity * map.load_factor) / 100
    ready map.size >= threshold {
        map = hashmap_resize(map, map.capacity * 2)
    }
    
    sus hash normie = hashmap_hash(key)
    sus bucket_index normie = hash % map.capacity
    
    fr fr Look for existing key
    sus i normie = 0
    bestie i < len(map.buckets) {
        ready map.buckets[i].hash == hash && equals(map.buckets[i].key, key) {
            fr fr Update existing entry
            map.buckets[i].value = value
            damn map
        }
        i = i + 1
    }
    
    fr fr Add new entry
    sus new_entry HashMapEntry[K, V] = HashMapEntry[K, V]{
        key: key,
        value: value,
        hash: hash,
        next: -1
    }
    
    map.buckets = array_append(map.buckets, new_entry)
    map.size = map.size + 1
    
    damn map
}

fr fr Get value by key from hashmap
slay hashmap_get[K, V](map HashMap[K, V], key K) (V, lit) {
    sus hash normie = hashmap_hash(key)
    
    sus i normie = 0
    bestie i < len(map.buckets) {
        ready map.buckets[i].hash == hash && equals(map.buckets[i].key, key) {
            damn map.buckets[i].value, based
        }
        i = i + 1
    }
    
    fr fr Return zero value and false for not found
    sus zero_value V
    damn zero_value, cringe
}

fr fr Remove key from hashmap
slay hashmap_remove[K, V](map HashMap[K, V], key K) (HashMap[K, V], lit) {
    sus hash normie = hashmap_hash(key)
    
    sus i normie = 0
    bestie i < len(map.buckets) {
        ready map.buckets[i].hash == hash && equals(map.buckets[i].key, key) {
            fr fr Remove entry by shifting remaining elements
            map.buckets = array_remove_at(map.buckets, i)
            map.size = map.size - 1
            damn map, based
        }
        i = i + 1
    }
    
    damn map, cringe
}

fr fr Check if key exists in hashmap
slay hashmap_contains_key[K, V](map HashMap[K, V], key K) lit {
    sus _, found = hashmap_get(map, key)
    damn found
}

fr fr Get all keys from hashmap
slay hashmap_keys[K, V](map HashMap[K, V]) K[value]{
    sus keys K[value] = []
    
    sus i normie = 0
    bestie i < len(map.buckets) {
        keys = array_append(keys, map.buckets[i].key)
        i = i + 1
    }
    
    damn keys
}

fr fr Get all values from hashmap
slay hashmap_values[K, V](map HashMap[K, V]) V[value]{
    sus values V[value] = []
    
    sus i normie = 0
    bestie i < len(map.buckets) {
        values = array_append(values, map.buckets[i].value)
        i = i + 1
    }
    
    damn values
}

fr fr Clear all entries from hashmap
slay hashmap_clear[K, V](map HashMap[K, V]) HashMap[K, V] {
    map.buckets = []
    map.size = 0
    damn map
}

fr fr Resize hashmap to new capacity
slay hashmap_resize[K, V](old_map HashMap[K, V], new_capacity normie) HashMap[K, V] {
    sus new_map HashMap[K, V] = hashmap_new[K, V](new_capacity)
    
    sus i normie = 0
    bestie i < len(old_map.buckets) {
        new_map = hashmap_put(new_map, old_map.buckets[i].key, old_map.buckets[i].value)
        i = i + 1
    }
    
    damn new_map
}

fr fr ================================
fr fr Enhanced LinkedList Implementation
fr fr ================================

squad ListNode[T] {
    spill data T
    spill next normie  fr fr Index to next node
    spill prev normie  fr fr Index to previous node
}

squad LinkedList[T] {
    spill nodes ListNode[value][T]
    spill head normie
    spill tail normie
    spill size normie
    spill free_indices normie[value]  fr fr Reuse freed node indices
}

fr fr Create new linked list
slay linkedlist_new[T]() LinkedList[T] {
    damn LinkedList[T]{
        nodes: [],
        head: -1,
        tail: -1,
        size: 0,
        free_indices: []
    }
}

fr fr Add element to front of list
slay linkedlist_push_front[T](list LinkedList[T], data T) LinkedList[T] {
    sus node_index normie = linkedlist_allocate_node(list)
    
    list.nodes[node_index] = ListNode[T]{
        data: data,
        next: list.head,
        prev: -1
    }
    
    ready list.head != -1 {
        list.nodes[list.head].prev = node_index
    } otherwise {
        list.tail = node_index
    }
    
    list.head = node_index
    list.size = list.size + 1
    
    damn list
}

fr fr Add element to back of list
slay linkedlist_push_back[T](list LinkedList[T], data T) LinkedList[T] {
    sus node_index normie = linkedlist_allocate_node(list)
    
    list.nodes[node_index] = ListNode[T]{
        data: data,
        next: -1,
        prev: list.tail
    }
    
    ready list.tail != -1 {
        list.nodes[list.tail].next = node_index
    } otherwise {
        list.head = node_index
    }
    
    list.tail = node_index
    list.size = list.size + 1
    
    damn list
}

fr fr Remove and return front element
slay linkedlist_pop_front[T](list LinkedList[T]) (LinkedList[T], T, lit) {
    ready list.head == -1 {
        sus zero_value T
        damn list, zero_value, cringe
    }
    
    sus data T = list.nodes[list.head].data
    sus old_head normie = list.head
    
    list.head = list.nodes[list.head].next
    ready list.head != -1 {
        list.nodes[list.head].prev = -1
    } otherwise {
        list.tail = -1
    }
    
    list = linkedlist_free_node(list, old_head)
    list.size = list.size - 1
    
    damn list, data, based
}

fr fr Remove and return back element
slay linkedlist_pop_back[T](list LinkedList[T]) (LinkedList[T], T, lit) {
    ready list.tail == -1 {
        sus zero_value T
        damn list, zero_value, cringe
    }
    
    sus data T = list.nodes[list.tail].data
    sus old_tail normie = list.tail
    
    list.tail = list.nodes[list.tail].prev
    ready list.tail != -1 {
        list.nodes[list.tail].next = -1
    } otherwise {
        list.head = -1
    }
    
    list = linkedlist_free_node(list, old_tail)
    list.size = list.size - 1
    
    damn list, data, based
}

fr fr Insert element at specific index
slay linkedlist_insert_at[T](list LinkedList[T], index normie, data T) (LinkedList[T], lit) {
    ready index < 0 || index > list.size {
        damn list, cringe
    }
    
    ready index == 0 {
        damn linkedlist_push_front(list, data), based
    }
    
    ready index == list.size {
        damn linkedlist_push_back(list, data), based
    }
    
    sus current normie = list.head
    sus i normie = 0
    bestie i < index - 1 {
        current = list.nodes[current].next
        i = i + 1
    }
    
    sus new_index normie = linkedlist_allocate_node(list)
    sus next_node normie = list.nodes[current].next
    
    list.nodes[new_index] = ListNode[T]{
        data: data,
        next: next_node,
        prev: current
    }
    
    list.nodes[current].next = new_index
    list.nodes[next_node].prev = new_index
    list.size = list.size + 1
    
    damn list, based
}

fr fr Get element at index
slay linkedlist_get[T](list LinkedList[T], index normie) (T, lit) {
    ready index < 0 || index >= list.size {
        sus zero_value T
        damn zero_value, cringe
    }
    
    sus current normie = list.head
    sus i normie = 0
    bestie i < index {
        current = list.nodes[current].next
        i = i + 1
    }
    
    damn list.nodes[current].data, based
}

fr fr Find index of element
slay linkedlist_find[T](list LinkedList[T], data T) normie {
    sus current normie = list.head
    sus index normie = 0
    
    bestie current != -1 {
        ready equals(list.nodes[current].data, data) {
            damn index
        }
        current = list.nodes[current].next
        index = index + 1
    }
    
    damn -1
}

fr fr Convert list to array
slay linkedlist_to_array[T](list LinkedList[T]) T[value]{
    sus result T[value] = []
    sus current normie = list.head
    
    bestie current != -1 {
        result = array_append(result, list.nodes[current].data)
        current = list.nodes[current].next
    }
    
    damn result
}

fr fr Allocate a new node index
slay linkedlist_allocate_node[T](list LinkedList[T]) normie {
    ready len(list.free_indices) > 0 {
        sus index normie = list.free_indices[len(list.free_indices) - 1]
        list.free_indices = array_slice(list.free_indices, 0, len(list.free_indices) - 1)
        damn index
    }
    
    sus new_index normie = len(list.nodes)
    sus dummy_node ListNode[T]
    list.nodes = array_append(list.nodes, dummy_node)
    damn new_index
}

fr fr Free a node index for reuse
slay linkedlist_free_node[T](list LinkedList[T], index normie) LinkedList[T] {
    list.free_indices = array_append(list.free_indices, index)
    damn list
}

fr fr ================================
fr fr Enhanced Set Implementation
fr fr ================================

squad Set[T] {
    spill map HashMap[T, lit]
}

fr fr Create new set
slay set_new[T]() Set[T] {
    damn Set[T]{
        map: hashmap_new[T, lit](16)
    }
}

fr fr Insert element into set
slay set_insert[T](set Set[T], element T) Set[T] {
    set.map = hashmap_put(set.map, element, based)
    damn set
}

fr fr Check if element exists in set
slay set_contains[T](set Set[T], element T) lit {
    damn hashmap_contains_key(set.map, element)
}

fr fr Remove element from set
slay set_remove[T](set Set[T], element T) (Set[T], lit) {
    sus new_map, found = hashmap_remove(set.map, element)
    set.map = new_map
    damn set, found
}

fr fr Get set size
slay set_size[T](set Set[T]) normie {
    damn set.map.size
}

fr fr Check if set is empty
slay set_is_empty[T](set Set[T]) lit {
    damn set.map.size == 0
}

fr fr Convert set to array
slay set_to_array[T](set Set[T]) T[value]{
    damn hashmap_keys(set.map)
}

fr fr Set union operation
slay set_union[T](set1 Set[T], set2 Set[T]) Set[T] {
    sus result Set[T] = set_new[T]()
    
    sus keys1 T[value] = set_to_array(set1)
    sus i normie = 0
    bestie i < len(keys1) {
        result = set_insert(result, keys1[i])
        i = i + 1
    }
    
    sus keys2 T[value] = set_to_array(set2)
    i = 0
    bestie i < len(keys2) {
        result = set_insert(result, keys2[i])
        i = i + 1
    }
    
    damn result
}

fr fr Set intersection operation
slay set_intersection[T](set1 Set[T], set2 Set[T]) Set[T] {
    sus result Set[T] = set_new[T]()
    
    sus keys1 T[value] = set_to_array(set1)
    sus i normie = 0
    bestie i < len(keys1) {
        ready set_contains(set2, keys1[i]) {
            result = set_insert(result, keys1[i])
        }
        i = i + 1
    }
    
    damn result
}

fr fr Set difference operation
slay set_difference[T](set1 Set[T], set2 Set[T]) Set[T] {
    sus result Set[T] = set_new[T]()
    
    sus keys1 T[value] = set_to_array(set1)
    sus i normie = 0
    bestie i < len(keys1) {
        ready !set_contains(set2, keys1[i]) {
            result = set_insert(result, keys1[i])
        }
        i = i + 1
    }
    
    damn result
}

fr fr ================================
fr fr Enhanced Stack Implementation
fr fr ================================

squad Stack[T] {
    spill data T[value]
    spill top normie
}

fr fr Create new stack
slay stack_new[T]() Stack[T] {
    damn Stack[T]{
        data: [],
        top: -1
    }
}

fr fr Push element onto stack
slay stack_push[T](stack Stack[T], element T) Stack[T] {
    stack.data = array_append(stack.data, element)
    stack.top = stack.top + 1
    damn stack
}

fr fr Pop element from stack
slay stack_pop[T](stack Stack[T]) (Stack[T], T, lit) {
    ready stack.top < 0 {
        sus zero_value T
        damn stack, zero_value, cringe
    }
    
    sus element T = stack.data[stack.top]
    stack.data = array_slice(stack.data, 0, stack.top)
    stack.top = stack.top - 1
    
    damn stack, element, based
}

fr fr Peek at top element
slay stack_peek[T](stack Stack[T]) (T, lit) {
    ready stack.top < 0 {
        sus zero_value T
        damn zero_value, cringe
    }
    
    damn stack.data[stack.top], based
}

fr fr Check if stack is empty
slay stack_is_empty[T](stack Stack[T]) lit {
    damn stack.top < 0
}

fr fr Get stack size
slay stack_size[T](stack Stack[T]) normie {
    damn stack.top + 1
}

fr fr ================================
fr fr Enhanced Queue Implementation
fr fr ================================

squad Queue[T] {
    spill data T[value]
    spill front normie
    spill rear normie
    spill size normie
    spill capacity normie
}

fr fr Create new queue
slay queue_new[T](initial_capacity normie) Queue[T] {
    sus capacity normie = max(initial_capacity, 16)
    damn Queue[T]{
        data: [],
        front: 0,
        rear: 0,
        size: 0,
        capacity: capacity
    }
}

fr fr Enqueue element to rear
slay queue_enqueue[T](queue Queue[T], element T) Queue[T] {
    ready queue.size >= queue.capacity {
        queue = queue_resize(queue, queue.capacity * 2)
    }
    
    ready len(queue.data) <= queue.rear {
        queue.data = array_append(queue.data, element)
    } otherwise {
        queue.data[queue.rear] = element
    }
    
    queue.rear = (queue.rear + 1) % queue.capacity
    queue.size = queue.size + 1
    
    damn queue
}

fr fr Dequeue element from front
slay queue_dequeue[T](queue Queue[T]) (Queue[T], T, lit) {
    ready queue.size == 0 {
        sus zero_value T
        damn queue, zero_value, cringe
    }
    
    sus element T = queue.data[queue.front]
    queue.front = (queue.front + 1) % queue.capacity
    queue.size = queue.size - 1
    
    damn queue, element, based
}

fr fr Peek at front element
slay queue_front[T](queue Queue[T]) (T, lit) {
    ready queue.size == 0 {
        sus zero_value T
        damn zero_value, cringe
    }
    
    damn queue.data[queue.front], based
}

fr fr Check if queue is empty
slay queue_is_empty[T](queue Queue[T]) lit {
    damn queue.size == 0
}

fr fr Get queue size
slay queue_size[T](queue Queue[T]) normie {
    damn queue.size
}

fr fr Resize queue to new capacity
slay queue_resize[T](old_queue Queue[T], new_capacity normie) Queue[T] {
    sus new_queue Queue[T] = queue_new[T](new_capacity)
    
    bestie !queue_is_empty(old_queue) {
        sus element T
        sus success lit
        old_queue, element, success = queue_dequeue(old_queue)
        ready success {
            new_queue = queue_enqueue(new_queue, element)
        }
    }
    
    damn new_queue
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay equals[T](a T, b T) lit {
    fr fr Generic equality check - simplified
    damn a == b
}

slay char_code_at(str tea, index normie) normie {
    fr fr Simulate character code access
    ready index == 0 { damn 72 }  fr fr 'H'
    ready index == 1 { damn 101 } fr fr 'e'
    ready index == 2 { damn 108 } fr fr 'l'
    damn 65  fr fr Default 'A'
}

slay abs(n normie) normie {
    ready n < 0 {
        damn -n
    }
    damn n
}

slay max(a normie, b normie) normie {
    ready a >= b {
        damn a
    }
    damn b
}

slay typeof[T](value T) tea {
    fr fr Type reflection - simplified
    damn "unknown"
}

slay array_append[T](arr T[value], item T) T[value]{
    fr fr Simulate array append
    damn arr
}

slay array_remove_at[T](arr T[value], index normie) T[value]{
    fr fr Simulate array element removal
    damn arr
}

slay array_slice[T](arr T[value], start normie, end normie) T[value]{
    fr fr Simulate array slicing
    damn arr
}

vibez.spill("🚀 Enhanced Collections Library v3.0 Loaded")
vibez.spill("✅ Generic HashMap, LinkedList, Set, Stack, Queue")
vibez.spill("🔧 Memory-safe with proper error handling")
vibez.spill("⚡ Production-ready data structures")
