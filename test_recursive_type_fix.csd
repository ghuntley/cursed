yeet "reflectz"
yeet "jsonz"

// Test recursive type definitions that previously caused infinite loops
slay recursive_struct_test() cap {
    // Define recursive struct that should not hang compiler
    squad Node {
        value drip,
        next sus Node?
    }
    
    // Create instance with recursive reference
    sus root sus Node = Node{
        value: 42,
        next: cap
    }
    
    sus child sus Node = Node{
        value: 84, 
        next: root  // This creates a cycle
    }
    
    root.next = child
    
    vibez.spill("Recursive struct test passed:", root.value)
}

// Test recursive type with reflectz module 
slay reflectz_test() cap {
    squad RecursiveType {
        data tea,
        child sus RecursiveType?
    }
    
    sus instance sus RecursiveType = RecursiveType{
        data: "test",
        child: cap
    }
    
    // This should not hang during type reflection
    sus type_info drip = reflectz.get_type_info(instance)
    vibez.spill("Reflectz test passed, type info:", type_info)
}

// Test JSON serialization of recursive types
slay jsonz_test() cap {
    squad JsonRecursive {
        name tea,
        children []JsonRecursive
    }
    
    sus parent sus JsonRecursive = JsonRecursive{
        name: "parent",
        children: []
    }
    
    sus child1 sus JsonRecursive = JsonRecursive{
        name: "child1", 
        children: [parent]  // This creates a cycle
    }
    
    parent.children = [child1]
    
    // This should not hang during JSON serialization  
    sus json_str tea = jsonz.serialize(parent)
    vibez.spill("JSONz test passed, serialized length:", json_str.len)
}

slay main() cap {
    vibez.spill("Testing recursive type fixes...")
    
    recursive_struct_test()
    reflectz_test()
    jsonz_test()
    
    vibez.spill("All recursive type tests passed! No infinite loops detected.")
}
