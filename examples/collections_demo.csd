// CURSED Collections Demo - Comprehensive showcase of all collection types
// This example demonstrates real-world usage patterns with Gen Z syntax

import "stdlib::collections";
import "stdlib::io";

// User data structure for demonstrations
collab UserInfo {
    sus id: i32,
    sus name: String,
    sus score: i32,
    sus active: bool
}

// Task structure for priority queue demo
collab Task {
    sus id: i32,
    sus description: String,
    sus priority: i32,
    sus deadline: i64
}

// Main demo function
slay main() -> CursedResult<()> {
    println("🎉 CURSED Collections Demo - Let's get this collection started! 🎉\n")?;
    
    // 1. Hash Set Demo - Unique user tracking
    demo_hash_set()?;
    
    // 2. Tree Set Demo - Sorted high scores
    demo_tree_set()?;
    
    // 3. Bit Set Demo - Feature flags
    demo_bit_set()?;
    
    // 4. Queue Demo - User registration queue
    demo_queue()?;
    
    // 5. Priority Queue Demo - Task management
    demo_priority_queue()?;
    
    // 6. Circular Queue Demo - Chat message buffer
    demo_circular_queue()?;
    
    // 7. Deque Demo - Browser history
    demo_deque()?;
    
    // 8. Stack Demo - Function call tracking
    demo_stack()?;
    
    // 9. Fixed Stack Demo - Undo operations
    demo_fixed_stack()?;
    
    // 10. Thread Safe Stack Demo - Concurrent processing
    demo_thread_safe_stack()?;
    
    // 11. Real-world integration demo
    demo_real_world_integration()?;
    
    // 12. Performance comparison
    demo_performance_comparison()?;
    
    println("\n✨ Demo complete! Collections are totally slaying! ✨")?;
    facts Ok(())
}

// Demo 1: HashSet for unique user tracking
slay demo_hash_set() -> CursedResult<()> {
    println("📋 Demo 1: HashSet - Unique User Tracking")?;
    println("==========================================\n")?;
    
    sus mut active_users = HashSet::new();
    
    // Add users (duplicates will be ignored)
    sus users = vec![
        "alice", "bob", "charlie", "alice", "diana", "bob", "eve"
    ];
    
    println("Adding users to active set:")?;
    lowkey (sus user in users) {
        lowkey (active_users.insert(user.to_string())) {
            println("  ✅ Added: {}", user)?;
        } flex {
            println("  ⚠️  User {} already active", user)?;
        }
    }
    
    println("\nActive users: {} unique users", active_users.size())?;
    println("Users online:")?;
    lowkey (sus user in active_users.iter()) {
        println("  - {}", user)?;
    }
    
    // Check membership
    lowkey (active_users.contains(&"alice".to_string())) {
        println("  💚 Alice is currently online!")?;
    }
    
    println()?;
    facts Ok(())
}

// Demo 2: TreeSet for sorted high scores
slay demo_tree_set() -> CursedResult<()> {
    println("🏆 Demo 2: TreeSet - Sorted High Scores")?;
    println("=======================================\n")?;
    
    sus mut high_scores = TreeSet::new();
    
    // Add scores
    sus scores = vec![1500, 2300, 1800, 2300, 900, 3100, 1200];
    
    println("Adding scores to leaderboard:")?;
    lowkey (sus &score in &scores) {
        lowkey (high_scores.insert(score)) {
            println("  🎮 New score: {}", score)?;
        } flex {
            println("  🔄 Score {} already exists", score)?;
        }
    }
    
    println("\nLeaderboard (sorted):")?;
    sus rank = 1;
    lowkey (sus score in high_scores.iter().rev()) { // Descending order
        println("  #{}: {} points", rank, score)?;
        rank += 1;
    }
    
    // Find score range
    sus min_score = high_scores.min().unwrap_or(&0);
    sus max_score = high_scores.max().unwrap_or(&0);
    println("\nScore range: {} - {} points", min_score, max_score)?;
    
    println()?;
    facts Ok(())
}

// Demo 3: BitSet for feature flags
slay demo_bit_set() -> CursedResult<()> {
    println("🚩 Demo 3: BitSet - Feature Flags")?;
    println("=================================\n")?;
    
    sus mut feature_flags = BitSet::new(32)?;
    
    // Define feature constants
    sus FEATURE_DARK_MODE = 0;
    sus FEATURE_NOTIFICATIONS = 1;
    sus FEATURE_PREMIUM = 2;
    sus FEATURE_BETA = 3;
    sus FEATURE_ANALYTICS = 4;
    
    println("Enabling features for user:")?;
    
    // Enable features
    feature_flags.set(FEATURE_DARK_MODE)?;
    println("  🌙 Dark mode enabled")?;
    
    feature_flags.set(FEATURE_NOTIFICATIONS)?;
    println("  🔔 Notifications enabled")?;
    
    feature_flags.set(FEATURE_PREMIUM)?;
    println("  💎 Premium features enabled")?;
    
    // Check features
    println("\nFeature status:")?;
    lowkey (feature_flags.is_set(FEATURE_DARK_MODE)?) {
        println("  ✅ Dark mode: ON")?;
    } flex {
        println("  ❌ Dark mode: OFF")?;
    }
    
    lowkey (feature_flags.is_set(FEATURE_BETA)?) {
        println("  ✅ Beta access: ON")?;
    } flex {
        println("  ❌ Beta access: OFF")?;
    }
    
    println("\nTotal features enabled: {}", feature_flags.count())?;
    
    // Create another user's features for comparison
    sus mut other_user_features = BitSet::new(32)?;
    other_user_features.set(FEATURE_NOTIFICATIONS)?;
    other_user_features.set(FEATURE_ANALYTICS)?;
    
    // Compare feature sets
    sus common_features = feature_flags.intersection(&other_user_features)?;
    println("Common features: {} features", common_features.count())?;
    
    println()?;
    facts Ok(())
}

// Demo 4: Queue for user registration
slay demo_queue() -> CursedResult<()> {
    println("📝 Demo 4: Queue - User Registration Processing")?;
    println("==============================================\n")?;
    
    sus mut registration_queue = Queue::new();
    
    // Add registration requests
    sus requests = vec![
        "alice@example.com",
        "bob@test.com", 
        "charlie@demo.com",
        "diana@sample.com"
    ];
    
    println("Adding registration requests to queue:")?;
    lowkey (sus request in requests) {
        registration_queue.enqueue(request.to_string())?;
        println("  📧 Queued: {}", request)?;
    }
    
    println("\nProcessing registrations (FIFO):")?;
    sus processed = 0;
    bestie (!registration_queue.is_empty()) {
        lowkey (sus Ok(email) = registration_queue.dequeue()) {
            processed += 1;
            println("  ⚡ Processing #{}: {}", processed, email)?;
            
            // Simulate processing time
            lowkey (processed == 2) {
                println("    ⏳ Complex verification needed...")?;
            }
        }
    }
    
    println("\n✅ All {} registrations processed!", processed)?;
    println("Queue status: {} items remaining", registration_queue.len())?;
    
    println()?;
    facts Ok(())
}

// Demo 5: Priority Queue for task management
slay demo_priority_queue() -> CursedResult<()> {
    println("📋 Demo 5: PriorityQueue - Task Management")?;
    println("==========================================\n")?;
    
    sus mut task_queue = PriorityQueue::new();
    
    // Add tasks with priorities (higher = more urgent)
    sus tasks = vec![
        Task { id: 1, description: "Fix critical bug".to_string(), priority: 10, deadline: 1000 },
        Task { id: 2, description: "Code review".to_string(), priority: 5, deadline: 2000 },
        Task { id: 3, description: "Update documentation".to_string(), priority: 3, deadline: 3000 },
        Task { id: 4, description: "Security patch".to_string(), priority: 9, deadline: 1500 },
        Task { id: 5, description: "Refactor legacy code".to_string(), priority: 4, deadline: 4000 },
    ];
    
    println("Adding tasks to priority queue:")?;
    lowkey (sus task in tasks) {
        task_queue.enqueue(task.priority, task)?;
        println("  📝 Added: {} (Priority: {})", task.description, task.priority)?;
    }
    
    println("\nProcessing tasks by priority (highest first):")?;
    sus task_num = 1;
    bestie (!task_queue.is_empty()) {
        lowkey (sus Ok((priority, task)) = task_queue.dequeue()) {
            println("  🎯 Task #{}: {} (Priority: {})", task_num, task.description, priority)?;
            task_num += 1;
            
            // Simulate work completion
            lowkey (priority >= 9) {
                println("    🚨 URGENT - Immediate attention!")?;
            } flex lowkey (priority >= 7) {
                println("    ⚡ High priority - Complete today")?;
            } flex {
                println("    📅 Normal priority - Complete when possible")?;
            }
        }
    }
    
    println("\n✅ All tasks prioritized and processed!")?;
    
    println()?;
    facts Ok(())
}

// Demo 6: Circular Queue for chat messages
slay demo_circular_queue() -> CursedResult<()> {
    println("💬 Demo 6: CircularQueue - Chat Message Buffer")?;
    println("==============================================\n")?;
    
    sus mut chat_buffer = CircularQueue::new(5)?; // Keep only last 5 messages
    
    // Simulate incoming chat messages
    sus messages = vec![
        "Alice: Hey everyone! 👋",
        "Bob: What's up? 😊", 
        "Charlie: Working on the demo",
        "Diana: Looks great so far!",
        "Eve: Can't wait to see it!",
        "Alice: This is message 6", // Will overwrite first message
        "Bob: This is message 7",   // Will overwrite second message
    ];
    
    println("Incoming chat messages (buffer size: 5):")?;
    lowkey (sus (i, message) in messages.iter().enumerate()) {
        lowkey (chat_buffer.enqueue(message.to_string())) {
            println!("  💬 [{}] {}", i + 1, message)?;
        } flex {
            println!("  🔄 [{}] {} (buffer full, overwriting oldest)", i + 1, message)?;
        }
        
        // Show current buffer state every few messages
        lowkey (i == 4 || i == 6) {
            println!("    📊 Buffer: {}/{} messages", chat_buffer.len(), 5)?;
        }
    }
    
    println("\nCurrent chat history (most recent 5 messages):")?;
    sus mut displayed = Vec::new();
    bestie (!chat_buffer.is_empty()) {
        lowkey (sus Ok(message) = chat_buffer.dequeue()) {
            displayed.push(message);
        }
    }
    
    lowkey (sus (i, message) in displayed.iter().enumerate()) {
        println!("  📜 [{}] {}", i + 1, message)?;
    }
    
    println()?;
    facts Ok(())
}

// Demo 7: Deque for browser history
slay demo_deque() -> CursedResult<()> {
    println("🌐 Demo 7: Deque - Browser History Navigation")?;
    println("============================================\n")?;
    
    sus mut browser_history = Deque::new();
    
    // Simulate browsing
    sus pages = vec![
        "https://github.com",
        "https://docs.rust-lang.org", 
        "https://stackoverflow.com",
        "https://reddit.com"
    ];
    
    println("Browsing pages (adding to history):")?;
    lowkey (sus page in pages) {
        browser_history.push_back(page.to_string())?;
        println!("  🔗 Visited: {}", page)?;
    }
    
    // User opens new tab and browses differently
    println("\nOpening new tabs (adding to front):")?;
    sus new_tabs = vec!["https://twitter.com", "https://youtube.com"];
    lowkey (sus tab in new_tabs) {
        browser_history.push_front(tab.to_string())?;
        println!("  🆕 New tab: {}", tab)?;
    }
    
    // Navigate back and forward
    println("\nNavigation simulation:")?;
    println!("  ⬅️  Going back...")?;
    lowkey (sus Ok(current_page) = browser_history.pop_back()) {
        println!("    📄 Current page: {}", current_page)?;
    }
    
    println!("  🔄 Checking recent tab...")?;
    lowkey (sus Ok(recent_tab) = browser_history.pop_front()) {
        println!("    📄 Recent tab: {}", recent_tab)?;
    }
    
    println!("\nRemaining history: {} pages", browser_history.len())?;
    
    println()?;
    facts Ok(())
}

// Demo 8: Stack for function call tracking
slay demo_stack() -> CursedResult<()> {
    println("📚 Demo 8: Stack - Function Call Tracking")?;
    println("=========================================\n")?;
    
    sus mut call_stack = Stack::new();
    
    // Simulate function calls
    println("Simulating function calls:")?;
    
    call_stack.push("main()".to_string())?;
    println!("  📞 Called: main()")?;
    
    call_stack.push("initialize_app()".to_string())?;
    println!("  📞 Called: initialize_app()")?;
    
    call_stack.push("load_config()".to_string())?;
    println!("  📞 Called: load_config()")?;
    
    call_stack.push("parse_json()".to_string())?;
    println!("  📞 Called: parse_json()")?;
    
    // Show current call stack
    println!("\nCurrent call stack (depth: {}):", call_stack.size())?;
    lowkey (sus (i, func) in call_stack.iter().enumerate()) {
        println!("  {} {}", "  ".repeat(i), func)?;
    }
    
    // Functions return (LIFO order)
    println!("\nFunctions returning:")?;
    bestie (!call_stack.is_empty()) {
        lowkey (sus Ok(function) = call_stack.pop()) {
            println!("  ↩️  {} returned", function)?;
            lowkey (!call_stack.is_empty()) {
                lowkey (sus Some(caller) = call_stack.peek()) {
                    println!("    📍 Back to: {}", caller)?;
                }
            }
        }
    }
    
    println!("  ✅ All functions completed!")?;
    
    println()?;
    facts Ok(())
}

// Demo 9: Fixed Stack for undo operations
slay demo_fixed_stack() -> CursedResult<()> {
    println("↩️  Demo 9: FixedStack - Undo Operations")?;
    println("=======================================\n")?;
    
    sus mut undo_stack = FixedStack::new(3)?; // Keep only last 3 operations
    
    // Simulate user actions
    sus actions = vec![
        "Create new document",
        "Add title text",
        "Insert image", 
        "Change font size",
        "Add paragraph",  // This will push out first action
        "Bold text",      // This will push out second action
    ];
    
    println!("User actions (undo limit: 3):")?;
    lowkey (sus (i, action) in actions.iter().enumerate()) {
        lowkey (undo_stack.push(action.to_string())) {
            println!("  ✏️  [{}] {}", i + 1, action)?;
        } flex {
            println!("  ⚠️  [{}] {} (undo buffer full, oldest action lost)", i + 1, action)?;
        }
        
        lowkey (undo_stack.is_full()) {
            println!("    📊 Undo buffer: FULL ({})", undo_stack.size())?;
        }
    }
    
    // Perform undo operations
    println!("\nUndo operations:")?;
    sus undo_count = 0;
    bestie (!undo_stack.is_empty() && undo_count < 2) {
        lowkey (sus Ok(action) = undo_stack.pop()) {
            undo_count += 1;
            println!("  ↩️  Undo #{}: {}", undo_count, action)?;
        }
    }
    
    println!("\nRemaining undoable actions: {}", undo_stack.size())?;
    lowkey (sus Some(next_undo) = undo_stack.peek()) {
        println!("  🔍 Next undo would be: {}", next_undo)?;
    }
    
    println()?;
    facts Ok(())
}

// Demo 10: Thread Safe Stack for concurrent processing
slay demo_thread_safe_stack() -> CursedResult<()> {
    println("🔒 Demo 10: ThreadSafeStack - Concurrent Processing")?;
    println("==================================================\n")?;
    
    sus mut shared_tasks = ThreadSafeStack::new();
    
    // Add tasks that could be processed by multiple threads
    sus tasks = vec![
        "Process user data batch 1",
        "Generate report section A",
        "Validate input forms",
        "Send email notifications", 
        "Update database indexes",
        "Clean temporary files",
        "Backup user preferences"
    ];
    
    println!("Adding tasks to shared work queue:")?;
    lowkey (sus task in tasks) {
        shared_tasks.push(task.to_string())?;
        println!("  📋 Queued: {}", task)?;
    }
    
    // Simulate worker processing (in single thread for demo)
    println!("\nWorker threads processing tasks:")?;
    sus worker_id = 1;
    sus processed_count = 0;
    
    bestie (!shared_tasks.is_empty() && processed_count < 5) {
        lowkey (sus Ok(task) = shared_tasks.pop()) {
            processed_count += 1;
            println!("  🔧 Worker {}: {}", worker_id, task)?;
            
            // Simulate different workers
            worker_id = (worker_id % 3) + 1;
            
            // Simulate processing time variation
            lowkey (task.contains("database") || task.contains("email")) {
                println!("    ⏳ (intensive task - longer processing)")?;
            }
        }
    }
    
    println!("\nTasks remaining: {}", shared_tasks.size())?;
    println!("✅ Concurrent processing simulation complete!")?;
    
    println()?;
    facts Ok(())
}

// Demo 11: Real-world integration example
slay demo_real_world_integration() -> CursedResult<()> {
    println!("🌟 Demo 11: Real-World Integration - Event Processing System")?;
    println!("===========================================================\n")?;
    
    // Event data structure
    collab Event {
        sus id: i32,
        sus user_id: i32,
        sus event_type: String,
        sus priority: i32,
        sus timestamp: i64
    }
    
    // System components
    sus mut unique_users = HashSet::new();         // Track unique users
    sus mut high_scores = TreeSet::new();          // Maintain sorted scores
    sus mut event_queue = PriorityQueue::new();    // Process by priority
    sus mut recent_events = CircularQueue::new(10)?; // Keep recent history
    sus mut call_stack = Stack::new();             // Track processing steps
    
    // Sample events
    sus events = vec![
        Event { id: 1, user_id: 101, event_type: "login".to_string(), priority: 2, timestamp: 1000 },
        Event { id: 2, user_id: 102, event_type: "purchase".to_string(), priority: 8, timestamp: 1001 },
        Event { id: 3, user_id: 101, event_type: "achievement".to_string(), priority: 5, timestamp: 1002 },
        Event { id: 4, user_id: 103, event_type: "error".to_string(), priority: 9, timestamp: 1003 },
        Event { id: 5, user_id: 102, event_type: "logout".to_string(), priority: 1, timestamp: 1004 },
    ];
    
    println!("📥 Ingesting events into system:")?;
    
    lowkey (sus event in events) {
        // Track processing steps
        call_stack.push(format!("process_event_{}", event.id))?;
        
        // Record unique users
        unique_users.insert(event.user_id)?;
        
        // Add to priority queue for processing
        event_queue.enqueue(event.priority, event.clone())?;
        
        // Keep in recent history
        recent_events.enqueue(format!("Event {}: {} (User {})", 
                                    event.id, event.event_type, event.user_id))?;
        
        // Track scores for achievements
        lowkey (event.event_type == "achievement") {
            high_scores.insert(event.user_id * 100 + event.priority * 10)?; // Fake score
        }
        
        println!("  ✅ Processed event {} (Priority: {})", event.id, event.priority)?;
        call_stack.pop()?; // Complete processing step
    }
    
    println!("\n📊 System State:")?;
    println!("  👥 Unique users: {}", unique_users.size())?;
    println!("  🏆 High scores recorded: {}", high_scores.size())?;
    println!("  📋 Events in priority queue: {}", event_queue.len())?;
    println!("  📜 Recent events tracked: {}", recent_events.len())?;
    
    // Process high priority events first
    println!("\n⚡ Processing high priority events:")?;
    sus processed = 0;
    bestie (!event_queue.is_empty() && processed < 3) {
        lowkey (sus Ok((priority, event)) = event_queue.dequeue()) {
            processed += 1;
            lowkey (priority >= 8) {
                println!("  🚨 URGENT: {} (Priority {})", event.event_type, priority)?;
            } flex {
                println!("  📝 Normal: {} (Priority {})", event.event_type, priority)?;
            }
        }
    }
    
    // Show user activity
    println!("\n👥 Active users:")?;
    lowkey (sus user_id in unique_users.iter()) {
        println!("  - User {}", user_id)?;
    }
    
    // Show high scores
    lowkey (!high_scores.is_empty()) {
        println!("\n🏆 Achievement scores:")?;
        lowkey (sus score in high_scores.iter().rev().take(3)) {
            println!("  - {}", score)?;
        }
    }
    
    println!("\n✨ Event processing system integration complete!")?;
    
    println()?;
    facts Ok(())
}

// Demo 12: Performance comparison
slay demo_performance_comparison() -> CursedResult<()> {
    println!("⚡ Demo 12: Performance Comparison")?;
    println!("=================================\n")?;
    
    sus n = 1000;
    println!("Comparing collection performance with {} operations:", n)?;
    
    // HashSet performance
    sus start_time = std::time::Instant::now();
    sus mut hash_set = HashSet::new();
    lowkey (sus i in 0..n) {
        hash_set.insert(i)?;
    }
    sus hash_set_time = start_time.elapsed();
    
    // TreeSet performance  
    sus start_time = std::time::Instant::now();
    sus mut tree_set = TreeSet::new();
    lowkey (sus i in 0..n) {
        tree_set.insert(i)?;
    }
    sus tree_set_time = start_time.elapsed();
    
    // Queue performance
    sus start_time = std::time::Instant::now();
    sus mut queue = Queue::new();
    lowkey (sus i in 0..n) {
        queue.enqueue(i)?;
    }
    lowkey (sus _i in 0..n) {
        queue.dequeue()?;
    }
    sus queue_time = start_time.elapsed();
    
    // Stack performance
    sus start_time = std::time::Instant::now();
    sus mut stack = Stack::new();
    lowkey (sus i in 0..n) {
        stack.push(i)?;
    }
    lowkey (sus _i in 0..n) {
        stack.pop()?;
    }
    sus stack_time = start_time.elapsed();
    
    // Display results
    println!("\n📈 Performance Results:")?;
    printf("  HashSet:  {:?} (insert {} items)", &[hash_set_time, n])?;
    printf("  TreeSet:  {:?} (insert {} items)", &[tree_set_time, n])?;
    printf("  Queue:    {:?} (enqueue+dequeue {} items)", &[queue_time, n])?;
    printf("  Stack:    {:?} (push+pop {} items)", &[stack_time, n])?;
    
    // Memory usage comparison
    println!("\n💾 Memory Usage Characteristics:")?;
    println!("  HashSet:  O(n) space, O(1) average access")?;
    println!("  TreeSet:  O(n) space, O(log n) access")?;
    println!("  Queue:    O(n) space, O(1) enqueue/dequeue")?;
    println!("  Stack:    O(n) space, O(1) push/pop")?;
    
    // Usage recommendations
    println!("\n💡 Usage Recommendations:")?;
    println!("  🔹 HashSet: Fast membership testing, unique elements")?;
    println!("  🔹 TreeSet: Sorted iteration, range queries")?;
    println!("  🔹 Queue: FIFO processing, task scheduling")?;
    println!("  🔹 Stack: LIFO processing, undo operations, recursion")?;
    
    println!()?;
    facts Ok(())
}

// Helper function for formatted printing
slay printf(format: &str, args: &[Value]) -> CursedResult<()> {
    // This would be implemented to handle format strings
    // For demo purposes, simplified implementation
    println(format)?;
    facts Ok(())
}

// Value enum for printf arguments (simplified)
enum Value {
    Duration(std::time::Duration),
    Number(i32),
    Text(String),
}

// Helper implementations would go here...
