// Simple Topological Sort Demo for CURSED Language
// Demonstrates graph algorithms using basic language features

slay main() {
    print("=== CURSED Topological Sort Demo ===");
    
    // Simulate a simple topological sort
    print("Input edges:");
    print("A -> B");
    print("B -> C");  
    print("B -> D");
    print("C -> E");
    print("D -> E");
    
    print("\nProcessing topological sort...");
    
    // Simulate Kahn's algorithm step by step
    print("Step 1: Find nodes with no incoming edges");
    print("Initial queue: [A]");
    
    print("Step 2: Process A");
    print("Remove A, add its children to consideration");
    print("Queue: [B]");
    
    print("Step 3: Process B");
    print("Remove B, add its children to consideration");
    print("Queue: [C, D]");
    
    print("Step 4: Process C");
    print("Remove C");
    print("Queue: [D]");
    
    print("Step 5: Process D");
    print("Remove D, check if E has no more incoming edges");
    print("Queue: [E]");
    
    print("Step 6: Process E");
    print("Remove E");
    print("Queue: []");
    
    print("\nTopological order: A B C D E");
    print("Alternative valid order: A B D C E");
    
    // Test with some calculations
    sus node_count = 5;
    sus edge_count = 5;
    print("\nGraph statistics:");
    print("Nodes:", node_count);
    print("Edges:", edge_count);
    
    lowkey (edge_count < node_count) {
        print("Graph is a forest (disconnected)");
    } highkey {
        print("Graph is connected");
    }
}
