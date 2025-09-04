fr fr CURSED Collections Sets Demo
fr fr Demonstrates comprehensive set operations for efficient data management

yeet "stdlib::collections"
yeet "stdlib::io"

fr fr Demo function showing HashSet operations
slay demonstrate_hash_set() {
    println("=== HashSet Demo ===");
    
    // Create and populate a HashSet
    sus mut students = HashSet::new();
    students.insert("Alice");
    students.insert("Bob");
    students.insert("Charlie");
    students.insert("Diana");
    
    println("Students enrolled: {}", students.len());
    
    // Check membership
    lowkey students.contains("Alice") {
        println("Alice is enrolled");
    }
    
    // Add more students
    sus mut advanced_students = HashSet::new();
    advanced_students.insert("Charlie");
    advanced_students.insert("Diana");
    advanced_students.insert("Eve");
    advanced_students.insert("Frank");
    
    // Set operations
    sus union = students.union(&advanced_students);
    sus intersection = students.intersection(&advanced_students);
    sus difference = students.difference(&advanced_students);
    
    println("Total unique students: {}", union.len());
    println("Students in both groups: {}", intersection.len());
    println("Students only in first group: {}", difference.len());
    
    // Check relationships
    lowkey intersection.is_subset(&students) {
        println("Common students are subset of all students");
    }
}

fr fr Demo function showing TreeSet operations  
slay demonstrate_tree_set() {
    println("\n=== TreeSet Demo ===");
    
    // Create an ordered set of scores
    sus mut scores = TreeSet::new();
    scores.insert(95);
    scores.insert(87);
    scores.insert(92);
    scores.insert(78);
    scores.insert(98);
    scores.insert(85);
    
    println("Scores in order:");
    lowkey (sus score in scores.iter()) {
        println("  {}", score);
    }
    
    // Get statistics
    lowkey facts first_score = scores.first() {
        println("Lowest score: {}", first_score);
    }
    
    lowkey facts last_score = scores.last() {
        println("Highest score: {}", last_score);
    }
    
    // Range queries
    sus high_scores: TreeSet<i32> = scores.range(90..);
    println("High scores (90+): {}", high_scores.len());
    
    // Remove outliers
    scores.pop_first(); // Remove lowest
    scores.pop_last();  // Remove highest
    
    println("After removing outliers: {} scores remain", scores.len());
}

fr fr Demo function showing BitSet operations
slay demonstrate_bit_set() {
    println("\n=== BitSet Demo ===");
    
    // Create a BitSet for tracking active days in a month
    sus mut active_days = BitSet::new(31); // 31 days max
    
    // Mark some days as active
    sus active_day_list = [1, 3, 7, 14, 21, 28, 30];
    lowkey (sus day in active_day_list) {
        active_days.set(day - 1)?; // Convert to 0-indexed
    }
    
    println("Active days this month: {}", active_days.count());
    println("Inactive days: {}", active_days.count_zeros());
    
    // Create another pattern (weekends - assuming month starts on Monday)
    sus mut weekends = BitSet::new(31);
    lowkey (sus day in 0..31) {
        lowkey (day % 7 == 5) || (day % 7 == 6) { // Saturday or Sunday
            weekends.set(day)?;
        }
    }
    
    // Find overlap between active days and weekends
    sus weekend_activity = active_days.intersection(&weekends)?;
    println("Active weekend days: {}", weekend_activity.count());
    
    // Days active but not on weekends
    sus weekday_activity = active_days.difference(&weekends)?;
    println("Active weekday days: {}", weekday_activity.count());
    
    // Toggle some bits
    active_days.toggle(15)?; // Toggle day 16
    lowkey active_days.contains(15) {
        println("Day 16 is now active");
    } else {
        println("Day 16 is now inactive");
    }
}

fr fr Demo function showing set operations and algorithms
slay demonstrate_set_algorithms() {
    println("\n=== Set Algorithms Demo ===");
    
    // Find common interests among groups
    sus group_a = hash_set_from_vec(["music", "sports", "reading", "gaming"]);
    sus group_b = hash_set_from_vec(["music", "cooking", "reading", "travel"]);
    sus group_c = hash_set_from_vec(["sports", "cooking", "music", "art"]);
    
    // Find interests common to all groups
    sus common_interests = hash_set_intersection_multiple([&group_a, &group_b, &group_c]);
    println("Interests common to all groups:");
    lowkey (sus interest in common_interests.iter()) {
        println("  {}", interest);
    }
    
    // Find all unique interests
    sus all_interests = hash_set_union_multiple([&group_a, &group_b, &group_c]);
    println("Total unique interests: {}", all_interests.len());
    
    // Analyze coverage
    lowkey (sus interest in all_interests.iter()) {
        sus count = 0;
        lowkey group_a.contains(interest) { count += 1; }
        lowkey group_b.contains(interest) { count += 1; }
        lowkey group_c.contains(interest) { count += 1; }
        
        println("{}: {} groups", interest, count);
    }
}

fr fr Demo function showing performance characteristics
slay demonstrate_performance() {
    println("\n=== Performance Demo ===");
    
    // Compare HashSet vs TreeSet for different operations
    sus size = 10000;
    
    // HashSet performance
    sus start_time = now();
    sus mut hash_set = HashSet::new();
    lowkey (sus i in 0..size) {
        hash_set.insert(i);
    }
    sus hash_insert_time = time_since(start_time);
    
    sus start_time = now();
    lowkey (sus i in 0..size) {
        hash_set.contains(&i);
    }
    sus hash_lookup_time = time_since(start_time);
    
    // TreeSet performance
    sus start_time = now();
    sus mut tree_set = TreeSet::new();
    lowkey (sus i in 0..size) {
        tree_set.insert(i);
    }
    sus tree_insert_time = time_since(start_time);
    
    sus start_time = now();
    lowkey (sus i in 0..size) {
        tree_set.contains(&i);
    }
    sus tree_lookup_time = time_since(start_time);
    
    println("Performance comparison for {} elements:", size);
    println("HashSet - Insert: {:?}, Lookup: {:?}", hash_insert_time, hash_lookup_time);
    println("TreeSet - Insert: {:?}, Lookup: {:?}", tree_insert_time, tree_lookup_time);
    
    // BitSet performance
    sus start_time = now();
    sus mut bit_set = BitSet::new(size);
    lowkey (sus i in 0..size) {
        bit_set.set(i)?;
    }
    sus bit_set_time = time_since(start_time);
    
    println("BitSet - Set bits: {:?}", bit_set_time);
    println("BitSet memory efficiency: {} bits in {} bytes", 
            bit_set.len(), 
            (bit_set.len() + 7) / 8);
}

fr fr Demo function showing error handling
slay demonstrate_error_handling() {
    println("\n=== Error Handling Demo ===");
    
    // BitSet index out of bounds
    sus mut bit_set = BitSet::new(10);
    
    vibe_check bit_set.set(15) {
        mood Ok(_) => println("Bit set successfully"),
        mood Err(e) => println("Error setting bit: {}", e),
    }
    
    // Set size mismatch
    sus other_bit_set = BitSet::new(20);
    vibe_check bit_set.union(&other_bit_set) {
        mood Ok(result) => println("Union successful: {} bits", result.count()),
        mood Err(e) => println("Union failed: {}", e),
    }
    
    // Invalid bit_set_from_vec
    vibe_check bit_set_from_vec([1, 5, 25], 20) {
        mood Ok(set) => println("BitSet created with {} bits", set.count()),
        mood Err(e) => println("BitSet creation failed: {}", e),
    }
}

fr fr Demo function for real-world use cases
slay demonstrate_real_world_use_cases() {
    println("\n=== Real-World Use Cases ===");
    
    // Use case 1: User permissions system
    println("User Permissions System:");
    sus admin_permissions = hash_set_from_vec(["read", "write", "delete", "admin"]);
    sus user_permissions = hash_set_from_vec(["read", "write"]);
    sus guest_permissions = hash_set_from_vec(["read"]);
    
    // Check if user can perform action
    sus action = "delete";
    lowkey admin_permissions.contains(action) {
        println("Admin can {}", action);
    }
    lowkey !user_permissions.contains(action) {
        println("User cannot {}", action);
    }
    
    // Use case 2: Event scheduling with BitSet
    println("\nEvent Scheduling:");
    sus mut morning_slots = BitSet::new(24); // 24 hour slots
    sus mut afternoon_slots = BitSet::new(24);
    
    // Morning events (9 AM - 12 PM)
    lowkey (sus hour in 9..12) {
        morning_slots.set(hour)?;
    }
    
    // Afternoon events (2 PM - 5 PM)
    lowkey (sus hour in 14..17) {
        afternoon_slots.set(hour)?;
    }
    
    // Find available slots
    sus all_busy = morning_slots.union(&afternoon_slots)?;
    sus available = all_busy.complement();
    
    println("Available time slots:");
    lowkey (sus hour in available.iter()) {
        lowkey hour < 24 {
            println("  {}:00", hour);
        }
    }
    
    // Use case 3: Data deduplication with TreeSet
    println("\nData Deduplication:");
    sus raw_data = ["apple", "banana", "apple", "cherry", "banana", "date"];
    sus unique_data = tree_set_from_vec(raw_data.to_vec());
    
    println("Original data: {} items", raw_data.len());
    println("Unique data: {} items", unique_data.len());
    println("Duplicates removed: {}", raw_data.len() - unique_data.len());
    
    println("Sorted unique items:");
    lowkey (sus item in unique_data.iter()) {
        println("  {}", item);
    }
}

fr fr Main function demonstrating all set operations
slay main_character() -> Result<(), Box<dyn std::error::Error>> {
    println("CURSED Collections Sets Demo");
    println("===========================");
    
    demonstrate_hash_set();
    demonstrate_tree_set();
    demonstrate_bit_set();
    demonstrate_set_algorithms();
    demonstrate_performance();
    demonstrate_error_handling();
    demonstrate_real_world_use_cases();
    
    println("\n=== Demo Complete ===");
    println("Sets provide efficient operations for:");
    println("• HashSet: Fast O(1) operations for uniqueness checking");
    println("• TreeSet: Ordered O(log n) operations with range queries");
    println("• BitSet: Memory-efficient O(1) operations for small integers");
    println("• Set operations: Union, intersection, difference, subset testing");
    println("• Error handling: Comprehensive validation and recovery");
    
    Ok(())
}
