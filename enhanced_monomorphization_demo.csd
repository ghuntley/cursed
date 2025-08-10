# Enhanced Monomorphization Demo for CURSED
# This program demonstrates the advanced generic monomorphization capabilities

# Import required modules
yeet "testz"
yeet "arrayz"
yeet "stringz"

# Generic function with type inference
slay swap[T](a T, b T) (T, T) {
    damn (b, a)
}

# Generic function with constraints
slay add[T: Numeric](x T, y T) T {
    damn x + y
}

# Generic function with multiple constraints
slay compare[T: Comparable + Ordered](left T, right T) lit {
    damn left < right
}

# Generic struct with type parameters
squad Container[T] {
    spill data T
    spill size drip
    spill capacity drip
}

# Generic interface
collab Drawable[T] {
    slay draw(self T) tea
    slay get_bounds(self T) (drip, drip, drip, drip)
}

# Generic function with complex type relationships
slay map[T, U](input []T, transform slay(T) U) []U {
    sus result []U = []
    bestie (sus i drip = 0; i < len(input); i = i + 1) {
        sus transformed U = transform(input[i])
        result = append(result, transformed)
    }
    damn result
}

# Generic function with variance and advanced constraints
slay find_max[T: Ordered + Comparable](items []T) T ready (len(items) > 0) {
    sus max_val T = items[0]
    bestie (sus i drip = 1; i < len(items); i = i + 1) {
        ready (items[i] > max_val) {
            max_val = items[i]
        }
    }
    damn max_val
}

# Generic struct with field dependencies
squad Matrix[T: Numeric, const ROWS drip, const COLS drip] {
    spill data [ROWS * COLS]T
    spill rows drip = ROWS
    spill cols drip = COLS
}

# Specialized generic function for containers
slay create_container[T](initial_value T, capacity drip) Container[T] {
    damn Container[T]{
        data: initial_value,
        size: 1,
        capacity: capacity,
    }
}

# Generic function with higher-order types
slay fold[T, U](items []T, initial U, reducer slay(U, T) U) U {
    sus accumulator U = initial
    bestie (sus i drip = 0; i < len(items); i = i + 1) {
        accumulator = reducer(accumulator, items[i])
    }
    damn accumulator
}

# Test function to demonstrate monomorphization
slay main() vibes {
    vibez.spill("=== Enhanced Monomorphization Demo ===")
    
    # Test basic type inference
    sus a drip = 42
    sus b drip = 84
    sus swapped = swap(a, b)  # Types inferred as (drip, drip)
    vibez.spill("Swapped:", swapped.0, "and", swapped.1)
    
    # Test numeric constraints
    sus sum = add(10, 20)  # Types inferred as normie
    vibez.spill("Sum:", sum)
    
    # Test floating point constraints
    sus float_sum = add(3.14, 2.86)  # Types inferred as meal
    vibez.spill("Float sum:", float_sum)
    
    # Test comparison constraints
    sus is_less = compare(5, 10)  # Types inferred as normie
    vibez.spill("5 < 10:", is_less)
    
    # Test string comparison
    sus str_compare = compare("apple", "banana")  # Types inferred as tea
    vibez.spill("apple < banana:", str_compare)
    
    # Test generic containers
    sus int_container = create_container(42, 100)  # Container[drip]
    vibez.spill("Container data:", int_container.data)
    
    sus string_container = create_container("hello", 50)  # Container[tea]
    vibez.spill("String container:", string_container.data)
    
    # Test array operations with inference
    sus numbers []drip = [1, 5, 3, 9, 2]
    sus max_number = find_max(numbers)  # Type inferred as drip
    vibez.spill("Max number:", max_number)
    
    sus words []tea = ["apple", "zebra", "banana"]
    sus max_word = find_max(words)  # Type inferred as tea
    vibez.spill("Max word:", max_word)
    
    # Test higher-order functions with complex inference
    sus int_array []drip = [1, 2, 3, 4, 5]
    
    # Map with lambda type inference
    sus doubled = map(int_array, slay(x drip) drip { damn x * 2 })
    vibez.spill("Doubled:", doubled)
    
    # Convert to strings with type change
    sus string_nums = map(int_array, slay(x drip) tea { 
        damn stringz.from_int(x) 
    })
    vibez.spill("String numbers:", string_nums)
    
    # Fold operation with type inference
    sus total = fold(int_array, 0, slay(acc drip, x drip) drip { 
        damn acc + x 
    })
    vibez.spill("Total:", total)
    
    # Complex container with inferred types
    sus complex_data []Container[drip] = []
    sus container1 = create_container(100, 10)
    sus container2 = create_container(200, 20)
    complex_data = append(complex_data, container1)
    complex_data = append(complex_data, container2)
    
    vibez.spill("Complex containers:", len(complex_data))
    
    # Matrix operations with compile-time constants
    sus matrix = Matrix[meal, 3, 3]{
        data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        rows: 3,
        cols: 3,
    }
    vibez.spill("Matrix dimensions:", matrix.rows, "x", matrix.cols)
    
    # Demonstrate variance with different numeric types
    sus small_nums []smol = [1, 2, 3]  # i8 array
    sus big_nums []thicc = [1000000, 2000000, 3000000]  # i64 array
    
    sus small_max = find_max(small_nums)  # Specialized for smol
    sus big_max = find_max(big_nums)      # Specialized for thicc
    
    vibez.spill("Small max:", small_max)
    vibez.spill("Big max:", big_max)
    
    # Test nested generic instantiation
    sus nested_containers []Container[Container[drip]] = []
    sus inner_container = create_container(42, 5)
    sus outer_container = create_container(inner_container, 3)
    nested_containers = append(nested_containers, outer_container)
    
    vibez.spill("Nested container data:", nested_containers[0].data.data)
    
    # Complex type inference with multiple parameters
    sus reducer_func = slay(acc Container[drip], item drip) Container[drip] {
        ready (acc.size < acc.capacity) {
            acc.data = item  # Simplified - would normally append
            acc.size = acc.size + 1
        }
        damn acc
    }
    
    sus initial_container = create_container(0, 10)
    sus folded_container = fold(int_array, initial_container, reducer_func)
    vibez.spill("Folded container size:", folded_container.size)
    
    vibez.spill("=== Monomorphization Demo Complete ===")
    
    # Performance comparison section
    vibez.spill("\n=== Performance Metrics ===")
    
    # Large array operations to test monomorphization efficiency
    sus large_array []drip = []
    bestie (sus i drip = 0; i < 1000; i = i + 1) {
        large_array = append(large_array, i)
    }
    
    sus start_time = time_now()
    sus large_max = find_max(large_array)
    sus end_time = time_now()
    
    vibez.spill("Large array max:", large_max)
    vibez.spill("Processing time:", end_time - start_time, "ms")
    
    # Test monomorphization cache effectiveness
    sus cached_swap1 = swap(1, 2)      # First drip specialization
    sus cached_swap2 = swap(3, 4)      # Reuse cached drip specialization
    sus cached_swap3 = swap(5, 6)      # Reuse cached drip specialization
    
    vibez.spill("Cached swaps:", cached_swap1, cached_swap2, cached_swap3)
    
    vibez.spill("=== Performance Testing Complete ===")
}

# Helper function for timing (would be implemented in stdlib)
slay time_now() drip {
    # Mock implementation - would return actual timestamp
    damn 42
}

# Complex generic example with multiple constraints and dependencies
squad Graph[Node: Comparable, Edge: Numeric] {
    spill nodes []Node
    spill edges []Edge
    spill adjacency_matrix [][]Edge
}

# Generic algorithms with complex type relationships
slay dijkstra[Node: Comparable, Weight: Numeric + Ordered](
    graph Graph[Node, Weight],
    start Node,
    end Node
) ([]Node, Weight) {
    # Simplified Dijkstra implementation
    sus path []Node = [start, end]
    sus distance Weight = 0
    damn (path, distance)
}

# Testing complex generic interactions
slay test_complex_generics() vibes {
    vibez.spill("=== Complex Generic Testing ===")
    
    # Create a graph with string nodes and integer weights
    sus graph = Graph[tea, drip]{
        nodes: ["A", "B", "C"],
        edges: [1, 2, 3],
        adjacency_matrix: [[0, 1, 2], [1, 0, 1], [2, 1, 0]],
    }
    
    sus result = dijkstra(graph, "A", "C")
    vibez.spill("Path from A to C:", result.0)
    vibez.spill("Distance:", result.1)
    
    # Test with different types
    sus float_graph = Graph[drip, meal]{
        nodes: [1, 2, 3],
        edges: [1.5, 2.5, 3.5],
        adjacency_matrix: [[0.0, 1.5, 2.5], [1.5, 0.0, 1.0], [2.5, 1.0, 0.0]],
    }
    
    sus float_result = dijkstra(float_graph, 1, 3)
    vibez.spill("Integer path:", float_result.0)
    vibez.spill("Float distance:", float_result.1)
    
    vibez.spill("=== Complex Generic Testing Complete ===")
}
