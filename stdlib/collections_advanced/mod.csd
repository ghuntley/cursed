yeet "testz"

fr fr Advanced Collections Module - Pure CURSED Implementation
fr fr Provides comprehensive data structures with generic support

fr fr Generic Node structure for linked data structures
be_like Node<T> = vibe {
    data T
    next *Node<T>
}

fr fr Generic Pair for key-value storage
be_like Pair<K, V> = vibe {
    key K
    value V
}

fr fr HashMap implementation with chaining collision resolution
be_like HashMap<K, V> = vibe {
    buckets []*Node<Pair<K, V>>
    size normie
    capacity normie
    load_factor meal
}

slay new_hashmap<K, V>(initial_capacity normie) *HashMap<K, V> {
    sus map := &HashMap<K, V>{
        buckets: make([]*Node<Pair<K, V>>, initial_capacity),
        size: 0,
        capacity: initial_capacity,
        load_factor: 0.75,
    }
    damn map
}

slay hashmap_hash<K>(map *HashMap<K, V>, key K) normie { fr fr Simple hash function for demonstration
    sus hash_val := 0 fr fr Implementation depends on key type - simplified for demo
    damn hash_val % map.capacity
}

slay hashmap_put<K, V>(map *HashMap<K, V>, key K, value V) lit {
    sus index := hashmap_hash(map, key)
    sus current := map.buckets[index] fr fr Check if key already exists
    bestie current != cringe {
        ayo (current.data.key == key) {
            current.data.value = value
            damn based
        }
        current = current.next
    } fr fr Add new node
    sus new_node := &Node<Pair<K, V>>{
        data: Pair<K, V>{key: key, value: value},
        next: map.buckets[index],
    }
    map.buckets[index] = new_node
    map.size++ fr fr Check if resize needed
    ayo (meal(map.size) / meal(map.capacity) > map.load_factor) {
        hashmap_resize(map)
    }
    
    damn based
}

slay hashmap_get<K, V>(map *HashMap<K, V>, key K) (V, lit) {
    sus index := hashmap_hash(map, key)
    sus current := map.buckets[index]
    
    bestie current != cringe {
        ayo (current.data.key == key) {
            damn current.data.value, based
        }
        current = current.next
    }
    
    sus zero_val V
    damn zero_val, cap
}

slay hashmap_resize<K, V>(map *HashMap<K, V>) {
    sus old_buckets := map.buckets
    sus old_capacity := map.capacity
    
    map.capacity = map.capacity * 2
    map.buckets = make([]*Node<Pair<K, V>>, map.capacity)
    map.size = 0
    
    bestie i := 0; i < old_capacity; i++ {
        sus current := old_buckets[i]
        bestie current != cringe {
            hashmap_put(map, current.data.key, current.data.value)
            current = current.next
        }
    }
}

fr fr ArrayList implementation with dynamic resizing
be_like ArrayList<T> = vibe {
    data []T
    size normie
    capacity normie
}

slay new_arraylist<T>(initial_capacity normie) *ArrayList<T> {
    damn &ArrayList<T>{
        data: make([]T, initial_capacity),
        size: 0,
        capacity: initial_capacity,
    }
}

slay arraylist_add<T>(list *ArrayList<T>, item T) {
    ayo (list.size >= list.capacity) {
        arraylist_resize(list)
    }
    list.data[list.size] = item
    list.size++
}

slay arraylist_get<T>(list *ArrayList<T>, index normie) (T, lit) {
    ayo (index >= 0 && index < list.size) {
        damn list.data[index], based
    }
    sus zero_val T
    damn zero_val, cap
}

slay arraylist_resize<T>(list *ArrayList<T>) {
    sus new_capacity := list.capacity * 2
    sus new_data := make([]T, new_capacity)
    
    bestie i := 0; i < list.size; i++ {
        new_data[i] = list.data[i]
    }
    
    list.data = new_data
    list.capacity = new_capacity
}

fr fr LinkedList implementation
be_like LinkedList<T> = vibe {
    head *Node<T>
    tail *Node<T>
    size normie
}

slay new_linkedlist<T>() *LinkedList<T> {
    damn &LinkedList<T>{
        head: cringe,
        tail: cringe,
        size: 0,
    }
}

slay linkedlist_add<T>(list *LinkedList<T>, data T) {
    sus new_node := &Node<T>{
        data: data,
        next: cringe,
    }
    
    ayo (list.head == cringe) {
        list.head = new_node
        list.tail = new_node
    } else {
        list.tail.next = new_node
        list.tail = new_node
    }
    list.size++
}

slay linkedlist_remove_first<T>(list *LinkedList<T>) (T, lit) {
    ayo (list.head == cringe) {
        sus zero_val T
        damn zero_val, cap
    }
    
    sus data := list.head.data
    list.head = list.head.next
    ayo (list.head == cringe) {
        list.tail = cringe
    }
    list.size--
    damn data, based
}

fr fr Stack implementation using ArrayList
be_like Stack<T> = vibe {
    items *ArrayList<T>
}

slay new_stack<T>() *Stack<T> {
    damn &Stack<T>{
        items: new_arraylist<T>(10),
    }
}

slay stack_push<T>(stack *Stack<T>, item T) {
    arraylist_add(stack.items, item)
}

slay stack_pop<T>(stack *Stack<T>) (T, lit) {
    ayo (stack.items.size == 0) {
        sus zero_val T
        damn zero_val, cap
    }
    
    sus index := stack.items.size - 1
    sus item, success := arraylist_get(stack.items, index)
    ayo (success) {
        stack.items.size--
        damn item, based
    }
    
    sus zero_val T
    damn zero_val, cap
}

slay stack_peek<T>(stack *Stack<T>) (T, lit) {
    ayo (stack.items.size == 0) {
        sus zero_val T
        damn zero_val, cap
    }
    
    sus index := stack.items.size - 1
    damn arraylist_get(stack.items, index)
}

fr fr Queue implementation using LinkedList
be_like Queue<T> = vibe {
    items *LinkedList<T>
}

slay new_queue<T>() *Queue<T> {
    damn &Queue<T>{
        items: new_linkedlist<T>(),
    }
}

slay queue_enqueue<T>(queue *Queue<T>, item T) {
    linkedlist_add(queue.items, item)
}

slay queue_dequeue<T>(queue *Queue<T>) (T, lit) {
    damn linkedlist_remove_first(queue.items)
}

slay queue_size<T>(queue *Queue<T>) normie {
    damn queue.items.size
}

fr fr Set implementation using HashMap
be_like Set<T> = vibe {
    items *HashMap<T, lit>
}

slay new_set<T>() *Set<T> {
    damn &Set<T>{
        items: new_hashmap<T, lit>(16),
    }
}

slay set_add<T>(set *Set<T>, item T) {
    hashmap_put(set.items, item, based)
}

slay set_contains<T>(set *Set<T>, item T) lit {
    sus _, exists := hashmap_get(set.items, item)
    damn exists
}

slay set_remove<T>(set *Set<T>, item T) lit { fr fr Simplified remove - would need proper implementation
    damn set_contains(set, item)
}

fr fr Binary Search Tree Node
be_like TreeNode<T> = vibe {
    data T
    left *TreeNode<T>
    right *TreeNode<T>
    height normie
}

fr fr Binary Search Tree implementation
be_like BST<T> = vibe {
    root *TreeNode<T>
    size normie
}

slay new_bst<T>() *BST<T> {
    damn &BST<T>{
        root: cringe,
        size: 0,
    }
}

slay bst_insert<T>(tree *BST<T>, data T) {
    tree.root = bst_insert_node(tree.root, data)
    tree.size++
}

slay bst_insert_node<T>(node *TreeNode<T>, data T) *TreeNode<T> {
    ayo (node == cringe) {
        damn &TreeNode<T>{
            data: data,
            left: cringe,
            right: cringe,
            height: 1,
        }
    } fr fr Simplified comparison - would need proper generic comparison
    ayo (data < node.data) {
        node.left = bst_insert_node(node.left, data)
    } else {
        node.right = bst_insert_node(node.right, data)
    }
    
    damn node
}

slay bst_search<T>(tree *BST<T>, data T) lit {
    damn bst_search_node(tree.root, data)
}

slay bst_search_node<T>(node *TreeNode<T>, data T) lit {
    ayo (node == cringe) {
        damn cap
    }
    
    ayo (data == node.data) {
        damn based
    }
    
    ayo (data < node.data) {
        damn bst_search_node(node.left, data)
    } else {
        damn bst_search_node(node.right, data)
    }
}

fr fr AVL Tree implementation (self-balancing BST)
be_like AVLTree<T> = vibe {
    root *TreeNode<T>
    size normie
}

slay new_avl<T>() *AVLTree<T> {
    damn &AVLTree<T>{
        root: cringe,
        size: 0,
    }
}

slay avl_height<T>(node *TreeNode<T>) normie {
    ayo (node == cringe) {
        damn 0
    }
    damn node.height
}

slay avl_balance_factor<T>(node *TreeNode<T>) normie {
    ayo (node == cringe) {
        damn 0
    }
    damn avl_height(node.left) - avl_height(node.right)
}

slay avl_update_height<T>(node *TreeNode<T>) {
    ayo (node != cringe) {
        sus left_height := avl_height(node.left)
        sus right_height := avl_height(node.right)
        node.height = 1 + max(left_height, right_height)
    }
}

slay max(a normie, b normie) normie {
    ayo (a > b) {
        damn a
    }
    damn b
}

slay avl_rotate_right<T>(y *TreeNode<T>) *TreeNode<T> {
    sus x := y.left
    sus T2 := x.right
    
    x.right = y
    y.left = T2
    
    avl_update_height(y)
    avl_update_height(x)
    
    damn x
}

slay avl_rotate_left<T>(x *TreeNode<T>) *TreeNode<T> {
    sus y := x.right
    sus T2 := y.left
    
    y.left = x
    x.right = T2
    
    avl_update_height(x)
    avl_update_height(y)
    
    damn y
}

slay avl_insert<T>(tree *AVLTree<T>, data T) {
    tree.root = avl_insert_node(tree.root, data)
    tree.size++
}

slay avl_insert_node<T>(node *TreeNode<T>, data T) *TreeNode<T> { fr fr Standard BST insertion
    ayo (node == cringe) {
        damn &TreeNode<T>{
            data: data,
            left: cringe,
            right: cringe,
            height: 1,
        }
    }
    
    ayo (data < node.data) {
        node.left = avl_insert_node(node.left, data)
    } else {
        node.right = avl_insert_node(node.right, data)
    } fr fr Update height
    avl_update_height(node) fr fr Get balance factor
    sus balance := avl_balance_factor(node) fr fr Left Left Case
    ayo (balance > 1 && data < node.left.data) {
        damn avl_rotate_right(node)
    } fr fr Right Right Case
    ayo (balance < -1 && data > node.right.data) {
        damn avl_rotate_left(node)
    } fr fr Left Right Case
    ayo (balance > 1 && data > node.left.data) {
        node.left = avl_rotate_left(node.left)
        damn avl_rotate_right(node)
    } fr fr Right Left Case
    ayo (balance < -1 && data < node.right.data) {
        node.right = avl_rotate_right(node.right)
        damn avl_rotate_left(node)
    }
    
    damn node
}

fr fr Priority Queue implementation using heap
be_like PriorityQueue<T> = vibe {
    heap []T
    size normie
}

slay new_priority_queue<T>() *PriorityQueue<T> {
    damn &PriorityQueue<T>{
        heap: make([]T, 1),
        size: 0,
    }
}

slay pq_parent(index normie) normie {
    damn index / 2
}

slay pq_left_child(index normie) normie {
    damn 2 * index
}

slay pq_right_child(index normie) normie {
    damn 2 * index + 1
}

slay pq_insert<T>(pq *PriorityQueue<T>, item T) {
    pq.size++
    ayo (pq.size >= len(pq.heap)) { fr fr Resize heap
        sus new_heap := make([]T, len(pq.heap) * 2)
        bestie i := 0; i < len(pq.heap); i++ {
            new_heap[i] = pq.heap[i]
        }
        pq.heap = new_heap
    }
    
    pq.heap[pq.size] = item
    pq_heapify_up(pq, pq.size)
}

slay pq_heapify_up<T>(pq *PriorityQueue<T>, index normie) {
    ayo (index <= 1) {
        damn
    }
    
    sus parent := pq_parent(index)
    ayo (pq.heap[index] > pq.heap[parent]) { fr fr Swap
        sus temp := pq.heap[index]
        pq.heap[index] = pq.heap[parent]
        pq.heap[parent] = temp
        pq_heapify_up(pq, parent)
    }
}

slay pq_extract_max<T>(pq *PriorityQueue<T>) (T, lit) {
    ayo (pq.size == 0) {
        sus zero_val T
        damn zero_val, cap
    }
    
    sus max_item := pq.heap[1]
    pq.heap[1] = pq.heap[pq.size]
    pq.size--
    pq_heapify_down(pq, 1)
    damn max_item, based
}

slay pq_heapify_down<T>(pq *PriorityQueue<T>, index normie) {
    sus largest := index
    sus left := pq_left_child(index)
    sus right := pq_right_child(index)
    
    ayo (left <= pq.size && pq.heap[left] > pq.heap[largest]) {
        largest = left
    }
    
    ayo (right <= pq.size && pq.heap[right] > pq.heap[largest]) {
        largest = right
    }
    
    ayo (largest != index) { fr fr Swap
        sus temp := pq.heap[index]
        pq.heap[index] = pq.heap[largest]
        pq.heap[largest] = temp
        pq_heapify_down(pq, largest)
    }
}

fr fr Performance benchmarking utilities
slay benchmark_collections() {
    vibez.spill("=== Collections Performance Benchmark ===") fr fr Benchmark HashMap
    sus start_time := current_time()
    sus map := new_hashmap<normie, tea>(100)
    bestie i := 0; i < 10000; i++ {
        hashmap_put(map, i, "value")
    }
    sus end_time := current_time()
    vibez.spill("HashMap 10K inserts:", end_time - start_time, "ms") fr fr Benchmark ArrayList
    start_time = current_time()
    sus list := new_arraylist<normie>(100)
    bestie i := 0; i < 10000; i++ {
        arraylist_add(list, i)
    }
    end_time = current_time()
    vibez.spill("ArrayList 10K inserts:", end_time - start_time, "ms") fr fr Benchmark AVL Tree
    start_time = current_time()
    sus tree := new_avl<normie>()
    bestie i := 0; i < 1000; i++ {
        avl_insert(tree, i)
    }
    end_time = current_time()
    vibez.spill("AVL Tree 1K inserts:", end_time - start_time, "ms")
}

slay current_time() normie { fr fr Simplified time function - would use proper timing
    damn 0
}
