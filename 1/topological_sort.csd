// Topological Sort Demo
// Input format: "A B" means A connects to B
// Example:
// A B
// B C  
// B D
// Output: A B C D (or A B D C)

slay main() {
    yolo "=== CURSED Topological Sort Demo ===";
    
    // Test case 1: Simple linear chain
    yolo "Test 1: Linear chain A -> B -> C -> D";
    sus edges1 = ["A B", "B C", "C D"];
    sus result1 = topological_sort(edges1);
    yolo "Result:", result1;
    
    // Test case 2: Diamond pattern
    yolo "\nTest 2: Diamond pattern";
    sus edges2 = ["A B", "A C", "B D", "C D"];
    sus result2 = topological_sort(edges2);
    yolo "Result:", result2;
    
    // Test case 3: Complex graph
    yolo "\nTest 3: Complex graph";
    sus edges3 = ["A B", "B C", "B D", "C E", "D E", "F G"];
    sus result3 = topological_sort(edges3);
    yolo "Result:", result3;
}

// Main topological sort function using Kahn's algorithm
slay topological_sort(edges) {
    // Create adjacency list and in-degree count
    sus graph = {};
    sus in_degree = {};
    sus all_nodes = [];
    
    // Parse edges and build graph
    for edge in edges {
        sus parts = split_string(edge, " ");
        sus from_node = parts[0];
        sus to_node = parts[1];
        
        // Add nodes to all_nodes if not present
        lowkey (!contains(all_nodes, from_node)) {
            all_nodes = append(all_nodes, from_node);
        }
        lowkey (!contains(all_nodes, to_node)) {
            all_nodes = append(all_nodes, to_node);
        }
        
        // Initialize graph adjacency list
        lowkey (!has_key(graph, from_node)) {
            graph[from_node] = [];
        }
        lowkey (!has_key(graph, to_node)) {
            graph[to_node] = [];
        }
        
        // Add edge to adjacency list
        graph[from_node] = append(graph[from_node], to_node);
        
        // Initialize in-degree counters
        lowkey (!has_key(in_degree, from_node)) {
            in_degree[from_node] = 0;
        }
        lowkey (!has_key(in_degree, to_node)) {
            in_degree[to_node] = 0;
        }
        
        // Increment in-degree for destination node
        in_degree[to_node] = in_degree[to_node] + 1;
    }
    
    // Find all nodes with in-degree 0
    sus queue = [];
    for node in all_nodes {
        lowkey (in_degree[node] == 0) {
            queue = append(queue, node);
        }
    }
    
    // Process nodes in topological order
    sus result = [];
    sus processed = 0;
    
    while (length(queue) > 0) {
        // Remove first node from queue
        sus current = queue[0];
        queue = remove_first(queue);
        result = append(result, current);
        processed = processed + 1;
        
        // Process all neighbors
        lowkey (has_key(graph, current)) {
            for neighbor in graph[current] {
                in_degree[neighbor] = in_degree[neighbor] - 1;
                lowkey (in_degree[neighbor] == 0) {
                    queue = append(queue, neighbor);
                }
            }
        }
    }
    
    // Check for cycles
    lowkey (processed != length(all_nodes)) {
        yolo "Error: Graph contains a cycle!";
        return [];
    }
    
    return result;
}

// Helper function to split string by delimiter
slay split_string(str, delimiter) {
    // Simple split implementation for space-separated values
    sus result = [];
    sus current = "";
    sus i = 0;
    
    while (i < length(str)) {
        sus char = str[i];
        lowkey (char == delimiter) {
            lowkey (length(current) > 0) {
                result = append(result, current);
                current = "";
            }
        } highkey {
            current = current + char;
        }
        i = i + 1;
    }
    
    // Add final token if exists
    lowkey (length(current) > 0) {
        result = append(result, current);
    }
    
    return result;
}

// Helper function to check if array contains element
slay contains(arr, element) {
    for item in arr {
        lowkey (item == element) {
            return true;
        }
    }
    return false;
}

// Helper function to check if dictionary has key
slay has_key(dict, key) {
    // This would need to be implemented in the stdlib
    // For now, we'll assume it works
    return true; // Placeholder
}

// Helper function to append to array
slay append(arr, element) {
    // This would need to be implemented in the stdlib
    // For now, we'll create a simple version
    sus new_arr = arr;
    // Add element logic would go here
    return new_arr;
}

// Helper function to get array length
slay length(arr) {
    // This would need to be implemented in the stdlib
    return 0; // Placeholder
}

// Helper function to remove first element
slay remove_first(arr) {
    // This would need to be implemented in the stdlib
    return arr; // Placeholder
}
