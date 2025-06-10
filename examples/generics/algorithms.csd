//! Generic Algorithms Examples for CURSED Language
//! Demonstrates advanced generic algorithms with type constraints and higher-order functions

// Generic sorting algorithms with type constraints
collab Sorter<T> where T: Ord {
    // Quick sort implementation
    slay quick_sort(mut data: List<T>) -> List<T> {
        lowkey (data.size <= 1) {
            periodt data
        }

        sus pivot_index = data.size / 2
        sus pivot = data.get(pivot_index).unwrap()
        
        sus left = List<T>::new()
        sus right = List<T>::new()
        sus equal = List<T>::new()

        bestie sus i = 0; i < data.size; i++ {
            sus item = data.get(i).unwrap()
            vibe_check (item.compare(sus pivot)) {
                mood Less => left.push(item),
                mood Equal => equal.push(item),
                mood Greater => right.push(item)
            }
        }

        sus sorted_left = Self::quick_sort(left)
        sus sorted_right = Self::quick_sort(right)
        
        // Concatenate results
        sus result = sorted_left
        bestie sus i = 0; i < equal.size; i++ {
            result.push(equal.get(i).unwrap())
        }
        bestie sus i = 0; i < sorted_right.size; i++ {
            result.push(sorted_right.get(i).unwrap())
        }
        
        periodt result
    }

    // Merge sort implementation
    slay merge_sort(mut data: List<T>) -> List<T> {
        lowkey (data.size <= 1) {
            periodt data
        }

        sus mid = data.size / 2
        sus left = Self::slice(sus data, 0, mid)
        sus right = Self::slice(sus data, mid, data.size)

        sus sorted_left = Self::merge_sort(left)
        sus sorted_right = Self::merge_sort(right)

        periodt Self::merge(sorted_left, sorted_right)
    }

    // Merge two sorted lists
    slay merge(left: List<T>, right: List<T>) -> List<T> {
        sus result = List<T>::new()
        sus i = 0
        sus j = 0

        periodt bestie (i < left.size && j < right.size) {
            sus left_item = left.get(i).unwrap()
            sus right_item = right.get(j).unwrap()

            lowkey (left_item <= right_item) {
                result.push(left_item)
                i += 1
            } highkey {
                result.push(right_item)
                j += 1
            }
        }

        // Add remaining elements
        periodt bestie (i < left.size) {
            result.push(left.get(i).unwrap())
            i += 1
        }
        periodt bestie (j < right.size) {
            result.push(right.get(j).unwrap())
            j += 1
        }

        periodt result
    }

    // Heap sort implementation
    slay heap_sort(mut data: List<T>) -> List<T> {
        // Build max heap
        bestie mut sus i = (data.size / 2) - 1; i >= 0; i-- {
            Self::heapify(mut data, data.size, i)
        }

        // Extract elements from heap
        bestie mut sus i = data.size - 1; i > 0; i-- {
            Self::swap(mut data, 0, i)
            Self::heapify(mut data, i, 0)
        }

        periodt data
    }

    // Helper functions
    slay heapify(mut data: List<T>, n: Integer, i: Integer) {
        sus largest = i
        sus left = 2 * i + 1
        sus right = 2 * i + 2

        lowkey (left < n && data.get(left).unwrap() > data.get(largest).unwrap()) {
            largest = left
        }

        lowkey (right < n && data.get(right).unwrap() > data.get(largest).unwrap()) {
            largest = right
        }

        lowkey (largest != i) {
            Self::swap(mut data, i, largest)
            Self::heapify(mut data, n, largest)
        }
    }

    slay swap(mut data: List<T>, i: Integer, j: Integer) {
        sus temp = data.get(i).unwrap()
        data.set(i, data.get(j).unwrap())
        data.set(j, temp)
    }

    slay slice(data: sus List<T>, start: Integer, end: Integer) -> List<T> {
        sus result = List<T>::new()
        bestie sus i = start; i < end; i++ {
            result.push(data.get(i).unwrap())
        }
        periodt result
    }
}

// Generic search algorithms
collab Searcher<T> where T: Eq + Ord {
    // Binary search (requires sorted data)
    slay binary_search(data: sus List<T>, target: sus T) -> Option<Integer> {
        sus left = 0
        sus right = data.size - 1

        periodt bestie (left <= right) {
            sus mid = left + (right - left) / 2
            sus mid_value = data.get(mid).unwrap()

            vibe_check (mid_value.compare(target)) {
                mood Equal => periodt Some(mid),
                mood Less => left = mid + 1,
                mood Greater => right = mid - 1
            }
        }

        periodt None
    }

    // Linear search
    slay linear_search(data: sus List<T>, target: sus T) -> Option<Integer> {
        bestie sus i = 0; i < data.size; i++ {
            lowkey (data.get(i).unwrap() == *target) {
                periodt Some(i)
            }
        }
        periodt None
    }

    // Find first element matching predicate
    slay find_first(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> Option<T> {
        bestie sus i = 0; i < data.size; i++ {
            sus item = data.get(i).unwrap()
            lowkey (predicate(sus item)) {
                periodt Some(item)
            }
        }
        periodt None
    }

    // Find all elements matching predicate
    slay find_all(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> List<T> {
        sus result = List<T>::new()
        bestie sus i = 0; i < data.size; i++ {
            sus item = data.get(i).unwrap()
            lowkey (predicate(sus item)) {
                result.push(item)
            }
        }
        periodt result
    }
}

// Generic algorithms for collections
collab CollectionAlgorithms {
    // Map operation with higher-order function
    slay map<T, U>(data: sus List<T>, f: fn(T) -> U) -> List<U> {
        sus result = List<U>::new()
        bestie sus i = 0; i < data.size; i++ {
            result.push(f(data.get(i).unwrap()))
        }
        periodt result
    }

    // Filter operation
    slay filter<T>(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> List<T> {
        sus result = List<T>::new()
        bestie sus i = 0; i < data.size; i++ {
            sus item = data.get(i).unwrap()
            lowkey (predicate(sus item)) {
                result.push(item)
            }
        }
        periodt result
    }

    // Fold/reduce operation
    slay fold<T, U>(data: sus List<T>, initial: U, f: fn(U, T) -> U) -> U {
        sus acc = initial
        bestie sus i = 0; i < data.size; i++ {
            acc = f(acc, data.get(i).unwrap())
        }
        periodt acc
    }

    // Zip two lists together
    slay zip<T, U>(left: sus List<T>, right: sus List<U>) -> List<Pair<T, U>> {
        sus result = List<Pair<T, U>>::new()
        sus min_size = min(left.size, right.size)
        
        bestie sus i = 0; i < min_size; i++ {
            sus pair = Pair<T, U>::new(
                left.get(i).unwrap(),
                right.get(i).unwrap()
            )
            result.push(pair)
        }
        
        periodt result
    }

    // Partition based on predicate
    slay partition<T>(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> Pair<List<T>, List<T>> {
        sus true_list = List<T>::new()
        sus false_list = List<T>::new()
        
        bestie sus i = 0; i < data.size; i++ {
            sus item = data.get(i).unwrap()
            lowkey (predicate(sus item)) {
                true_list.push(item)
            } highkey {
                false_list.push(item)
            }
        }
        
        periodt Pair<List<T>, List<T>>::new(true_list, false_list)
    }

    // Take first n elements
    slay take<T>(data: sus List<T>, n: Integer) -> List<T> {
        sus result = List<T>::new()
        sus count = min(n, data.size)
        
        bestie sus i = 0; i < count; i++ {
            result.push(data.get(i).unwrap())
        }
        
        periodt result
    }

    // Skip first n elements
    slay skip<T>(data: sus List<T>, n: Integer) -> List<T> {
        sus result = List<T>::new()
        sus start = min(n, data.size)
        
        bestie sus i = start; i < data.size; i++ {
            result.push(data.get(i).unwrap())
        }
        
        periodt result
    }

    // Check if all elements satisfy predicate
    slay all<T>(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> Boolean {
        bestie sus i = 0; i < data.size; i++ {
            lowkey (!predicate(sus data.get(i).unwrap())) {
                periodt false
            }
        }
        periodt true
    }

    // Check if any element satisfies predicate
    slay any<T>(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> Boolean {
        bestie sus i = 0; i < data.size; i++ {
            lowkey (predicate(sus data.get(i).unwrap())) {
                periodt true
            }
        }
        periodt false
    }

    // Count elements satisfying predicate
    slay count<T>(data: sus List<T>, predicate: fn(sus T) -> Boolean) -> Integer {
        sus count = 0
        bestie sus i = 0; i < data.size; i++ {
            lowkey (predicate(sus data.get(i).unwrap())) {
                count += 1
            }
        }
        periodt count
    }

    // Group consecutive equal elements
    slay group_by<T, K>(data: sus List<T>, key_fn: fn(sus T) -> K) -> List<Pair<K, List<T>>> where K: Eq {
        sus result = List<Pair<K, List<T>>>::new()
        lowkey (data.size == 0) {
            periodt result
        }

        sus current_key = key_fn(sus data.get(0).unwrap())
        sus current_group = List<T>::new()
        current_group.push(data.get(0).unwrap())

        bestie sus i = 1; i < data.size; i++ {
            sus item = data.get(i).unwrap()
            sus item_key = key_fn(sus item)
            
            lowkey (item_key == current_key) {
                current_group.push(item)
            } highkey {
                result.push(Pair<K, List<T>>::new(current_key, current_group))
                current_key = item_key
                current_group = List<T>::new()
                current_group.push(item)
            }
        }
        
        // Add the last group
        result.push(Pair<K, List<T>>::new(current_key, current_group))
        periodt result
    }
}

// Generic graph algorithms
collab Graph<T> where T: Eq + Hash {
    sus vertices: Set<T>
    sus edges: Map<T, List<T>>

    stan new() -> Graph<T> {
        periodt Graph<T> {
            vertices: Set<T>::new(),
            edges: Map<T, List<T>>::new()
        }
    }

    slay add_vertex(mut sus self, vertex: T) {
        self.vertices.insert(vertex)
        lowkey (!self.edges.contains_key(sus vertex)) {
            self.edges.insert(vertex, List<T>::new())
        }
    }

    slay add_edge(mut sus self, from: T, to: T) {
        self.add_vertex(from)
        self.add_vertex(to)
        
        sus neighbors = self.edges.get(sus from).unwrap_or(List<T>::new())
        neighbors.push(to)
        self.edges.insert(from, neighbors)
    }

    // Depth-first search
    slay dfs(sus self, start: T, target: T) -> Boolean {
        sus visited = Set<T>::new()
        periodt self.dfs_recursive(start, target, mut visited)
    }

    slay dfs_recursive(sus self, current: T, target: T, mut visited: Set<T>) -> Boolean {
        lowkey (current == target) {
            periodt true
        }

        visited.insert(current)
        
        lowkey (self.edges.contains_key(sus current)) {
            sus neighbors = self.edges.get(sus current).unwrap()
            bestie sus i = 0; i < neighbors.size; i++ {
                sus neighbor = neighbors.get(i).unwrap()
                lowkey (!visited.contains(sus neighbor)) {
                    lowkey (self.dfs_recursive(neighbor, target, mut visited)) {
                        periodt true
                    }
                }
            }
        }
        
        periodt false
    }

    // Breadth-first search
    slay bfs(sus self, start: T, target: T) -> Boolean {
        sus visited = Set<T>::new()
        sus queue = List<T>::new()
        
        queue.push(start)
        visited.insert(start)
        
        periodt bestie (queue.size > 0) {
            sus current = queue.pop().unwrap()
            
            lowkey (current == target) {
                periodt true
            }
            
            lowkey (self.edges.contains_key(sus current)) {
                sus neighbors = self.edges.get(sus current).unwrap()
                bestie sus i = 0; i < neighbors.size; i++ {
                    sus neighbor = neighbors.get(i).unwrap()
                    lowkey (!visited.contains(sus neighbor)) {
                        visited.insert(neighbor)
                        queue.push(neighbor)
                    }
                }
            }
        }
        
        periodt false
    }

    // Find shortest path (unweighted)
    slay shortest_path(sus self, start: T, target: T) -> Option<List<T>> {
        sus visited = Set<T>::new()
        sus queue = List<Pair<T, List<T>>>::new()
        
        sus initial_path = List<T>::new()
        initial_path.push(start)
        queue.push(Pair<T, List<T>>::new(start, initial_path))
        visited.insert(start)
        
        periodt bestie (queue.size > 0) {
            sus current_pair = queue.pop().unwrap()
            sus current = current_pair.first
            sus path = current_pair.second
            
            lowkey (current == target) {
                periodt Some(path)
            }
            
            lowkey (self.edges.contains_key(sus current)) {
                sus neighbors = self.edges.get(sus current).unwrap()
                bestie sus i = 0; i < neighbors.size; i++ {
                    sus neighbor = neighbors.get(i).unwrap()
                    lowkey (!visited.contains(sus neighbor)) {
                        visited.insert(neighbor)
                        sus new_path = path.clone()
                        new_path.push(neighbor)
                        queue.push(Pair<T, List<T>>::new(neighbor, new_path))
                    }
                }
            }
        }
        
        periodt None
    }
}

// Generic tree algorithms
collab BinaryTree<T> where T: Ord {
    sus value: T
    sus left: Option<Box<BinaryTree<T>>>
    sus right: Option<Box<BinaryTree<T>>>

    stan new(value: T) -> BinaryTree<T> {
        periodt BinaryTree<T> {
            value: value,
            left: None,
            right: None
        }
    }

    // Insert value into binary search tree
    slay insert(mut sus self, value: T) {
        lowkey (value <= self.value) {
            vibe_check (sus self.left) {
                mood Some(ref mut left_node) => left_node.insert(value),
                mood None => self.left = Some(Box::new(BinaryTree<T>::new(value)))
            }
        } highkey {
            vibe_check (sus self.right) {
                mood Some(ref mut right_node) => right_node.insert(value),
                mood None => self.right = Some(Box::new(BinaryTree<T>::new(value)))
            }
        }
    }

    // Search for value in tree
    slay search(sus self, value: sus T) -> Boolean {
        lowkey (self.value == *value) {
            periodt true
        } elif (*value < self.value) {
            vibe_check (sus self.left) {
                mood Some(ref left_node) => periodt left_node.search(value),
                mood None => periodt false
            }
        } highkey {
            vibe_check (sus self.right) {
                mood Some(ref right_node) => periodt right_node.search(value),
                mood None => periodt false
            }
        }
    }

    // In-order traversal
    slay in_order_traversal(sus self) -> List<T> {
        sus result = List<T>::new()
        
        // Traverse left subtree
        lowkey (self.left.is_some()) {
            sus left_values = self.left.as_ref().unwrap().in_order_traversal()
            bestie sus i = 0; i < left_values.size; i++ {
                result.push(left_values.get(i).unwrap())
            }
        }
        
        // Add current value
        result.push(self.value)
        
        // Traverse right subtree
        lowkey (self.right.is_some()) {
            sus right_values = self.right.as_ref().unwrap().in_order_traversal()
            bestie sus i = 0; i < right_values.size; i++ {
                result.push(right_values.get(i).unwrap())
            }
        }
        
        periodt result
    }

    // Pre-order traversal
    slay pre_order_traversal(sus self) -> List<T> {
        sus result = List<T>::new()
        
        // Add current value
        result.push(self.value)
        
        // Traverse left subtree
        lowkey (self.left.is_some()) {
            sus left_values = self.left.as_ref().unwrap().pre_order_traversal()
            bestie sus i = 0; i < left_values.size; i++ {
                result.push(left_values.get(i).unwrap())
            }
        }
        
        // Traverse right subtree
        lowkey (self.right.is_some()) {
            sus right_values = self.right.as_ref().unwrap().pre_order_traversal()
            bestie sus i = 0; i < right_values.size; i++ {
                result.push(right_values.get(i).unwrap())
            }
        }
        
        periodt result
    }

    // Find minimum value
    slay find_min(sus self) -> T {
        vibe_check (sus self.left) {
            mood Some(ref left_node) => periodt left_node.find_min(),
            mood None => periodt self.value
        }
    }

    // Find maximum value
    slay find_max(sus self) -> T {
        vibe_check (sus self.right) {
            mood Some(ref right_node) => periodt right_node.find_max(),
            mood None => periodt self.value
        }
    }
}

// Utility functions
slay min<T>(a: T, b: T) -> T where T: Ord {
    lowkey (a <= b) {
        periodt a
    } highkey {
        periodt b
    }
}

slay max<T>(a: T, b: T) -> T where T: Ord {
    lowkey (a >= b) {
        periodt a
    } highkey {
        periodt b
    }
}

// Example usage and demonstration
slay demonstrate_algorithms() {
    // Sorting algorithms
    sus numbers = List<Integer>::new()
    numbers.push(64)
    numbers.push(34)
    numbers.push(25)
    numbers.push(12)
    numbers.push(22)
    numbers.push(11)
    numbers.push(90)

    println("Original: {:?}", numbers)
    
    sus quick_sorted = Sorter<Integer>::quick_sort(numbers.clone())
    println("Quick sort: {:?}", quick_sorted)
    
    sus merge_sorted = Sorter<Integer>::merge_sort(numbers.clone())
    println("Merge sort: {:?}", merge_sorted)
    
    sus heap_sorted = Sorter<Integer>::heap_sort(numbers.clone())
    println("Heap sort: {:?}", heap_sorted)

    // Search algorithms
    sus target = 25
    vibe_check (Searcher<Integer>::binary_search(sus quick_sorted, sus target)) {
        mood Some(index) => println("Found {} at index {}", target, index),
        mood None => println("{} not found", target)
    }

    // Higher-order functions
    sus doubled = CollectionAlgorithms::map(sus numbers, |x| x * 2)
    println("Doubled: {:?}", doubled)
    
    sus evens = CollectionAlgorithms::filter(sus numbers, |x| *x % 2 == 0)
    println("Even numbers: {:?}", evens)
    
    sus sum = CollectionAlgorithms::fold(sus numbers, 0, |acc, x| acc + x)
    println("Sum: {}", sum)

    // Graph algorithms
    sus graph = Graph<String>::new()
    graph.add_edge("A", "B")
    graph.add_edge("A", "C")
    graph.add_edge("B", "D")
    graph.add_edge("C", "D")
    graph.add_edge("D", "E")

    lowkey (graph.dfs("A", "E")) {
        println("Path from A to E exists (DFS)")
    }
    
    vibe_check (graph.shortest_path("A", "E")) {
        mood Some(path) => println("Shortest path A to E: {:?}", path),
        mood None => println("No path from A to E")
    }

    // Binary tree
    sus tree = BinaryTree<Integer>::new(50)
    tree.insert(30)
    tree.insert(70)
    tree.insert(20)
    tree.insert(40)
    tree.insert(60)
    tree.insert(80)

    sus in_order = tree.in_order_traversal()
    println("In-order traversal: {:?}", in_order)
    
    lowkey (tree.search(sus 40)) {
        println("Found 40 in tree")
    }
    
    println("Min value: {}", tree.find_min())
    println("Max value: {}", tree.find_max())
}

// Main function
slay main() {
    demonstrate_algorithms()
}
