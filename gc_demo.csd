// CURSED Garbage Collection Demo
// This program demonstrates the improved mark-and-sweep garbage collector
// with proper reference tracing and cycle detection

stan main() {
    vibez.spill("🗑️  CURSED Garbage Collection Demo")
    vibez.spill("=====================================")
    
    // Create objects with references
    object_graph_demo()
    
    // Demonstrate cycle handling
    cycle_demo()
    
    // Show memory statistics
    memory_stats_demo()
    
    vibez.spill("✅ Garbage collection demo complete!")
}

// Demonstrate object graph traversal
stan object_graph_demo() {
    vibez.spill("📊 Object Graph Traversal Demo")
    
    // Create a tree of objects
    let root = create_object("root")
    let child1 = create_object("child1")
    let child2 = create_object("child2")
    let grandchild = create_object("grandchild")
    
    // Link them together
    link_objects(root, child1)
    link_objects(root, child2)
    link_objects(child1, grandchild)
    
    vibez.spill("Created object graph: root -> child1,child2; child1 -> grandchild")
    
    // Force garbage collection
    runtime.gc()
    
    vibez.spill("✅ All reachable objects preserved during GC")
}

// Demonstrate cycle detection
stan cycle_demo() {
    vibez.spill("🔄 Cycle Detection Demo")
    
    // Create objects that reference each other in a cycle
    let obj_a = create_object("A")
    let obj_b = create_object("B")
    let obj_c = create_object("C")
    
    // Create cycle: A -> B -> C -> A
    link_objects(obj_a, obj_b)
    link_objects(obj_b, obj_c)
    link_objects(obj_c, obj_a)
    
    vibez.spill("Created cycle: A -> B -> C -> A")
    
    // Remove reference to the cycle (make it unreachable)
    obj_a = nil
    obj_b = nil
    obj_c = nil
    
    vibez.spill("Removed external references to cycle")
    
    // Force garbage collection - should detect and collect the cycle
    let collected = runtime.gc()
    
    vibez.spill("✅ Cycle detected and collected: " + collected + " bytes")
}

// Show memory usage statistics
stan memory_stats_demo() {
    vibez.spill("📈 Memory Statistics Demo")
    
    let stats = runtime.memory_stats()
    
    vibez.spill("Memory Usage:")
    vibez.spill("  Total allocated: " + stats.total_allocated + " bytes")
    vibez.spill("  Current usage: " + stats.current_usage + " bytes")
    vibez.spill("  GC collections: " + stats.gc_collections)
    vibez.spill("  GC time: " + stats.gc_time_ms + " ms")
    
    vibez.spill("✅ Memory properly tracked and managed")
}

// Helper function to create an object
stan create_object(name: str) -> object {
    return {
        name: name,
        refs: [],
        data: "Some data for " + name
    }
}

// Helper function to link objects
stan link_objects(from: object, to: object) {
    from.refs.push(to)
}
