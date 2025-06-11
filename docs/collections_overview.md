# CURSED Collections System - Comprehensive Overview

## Table of Contents
1. [Introduction](#introduction)
2. [Collection Types Overview](#collection-types-overview)
3. [When to Use Each Collection](#when-to-use-each-collection)
4. [Performance Characteristics](#performance-characteristics)
5. [Integration and Interoperability](#integration-and-interoperability)
6. [Best Practices](#best-practices)
7. [Migration Guide](#migration-guide)
8. [Code Examples](#code-examples)
9. [Testing and Validation](#testing-and-validation)
10. [Advanced Usage Patterns](#advanced-usage-patterns)

## Introduction

The CURSED Collections System provides a comprehensive suite of data structures optimized for different use cases, offering excellent performance, memory efficiency, and seamless integration. The collection types are designed to work together harmoniously while maintaining the Gen Z aesthetic and slang terminology that makes CURSED unique.

### Design Philosophy
- **Performance First**: Each collection is optimized for its primary use case
- **Memory Efficient**: Minimal overhead with configurable sizing
- **Thread Safe Options**: Concurrent access where needed
- **Seamless Integration**: Collections work together naturally
- **Error Resilient**: Comprehensive error handling and recovery
- **Gen Z Compliant**: Consistent with CURSED's linguistic style

## Collection Types Overview

### Sets - Unique Element Collections

#### HashSet
**Purpose**: Fast membership testing and unique element storage
```cursed
sus mut users = HashSet::new();
users.insert("alice".to_string())?;
lowkey (users.contains(&"alice".to_string())) {
    println("Alice is online! 💚")?;
}
```

**Key Features**:
- O(1) average insert, remove, and lookup
- Hash-based implementation with collision handling
- Dynamic resizing for optimal performance
- Iterator support for functional programming

#### TreeSet
**Purpose**: Sorted unique elements with range operations
```cursed
sus mut scores = TreeSet::new();
scores.insert(1500)?;
scores.insert(2300)?;
// Iteration in sorted order
lowkey (sus score in scores.iter().rev()) {
    println("High score: {}", score)?;
}
```

**Key Features**:
- O(log n) insert, remove, and lookup
- Automatically sorted iteration
- Range queries and nearest neighbor operations
- Balanced tree structure for consistent performance

#### BitSet
**Purpose**: Efficient boolean flag storage for large ranges
```cursed
sus mut features = BitSet::new(100)?;
features.set(42)?; // Enable feature 42
lowkey (features.is_set(42)?) {
    println("Feature 42 is enabled! 🚀")?;
}
```

**Key Features**:
- Extremely memory efficient for boolean data
- Bitwise operations (union, intersection, complement)
- Fixed-size with configurable capacity
- O(1) set, clear, and test operations

### Queues - FIFO and Priority-Based Collections

#### Queue
**Purpose**: First-In-First-Out processing
```cursed
sus mut tasks = Queue::new();
tasks.enqueue("process_user_registration".to_string())?;
tasks.enqueue("send_welcome_email".to_string())?;

bestie (!tasks.is_empty()) {
    lowkey (sus Ok(task) = tasks.dequeue()) {
        println("Processing: {}", task)?;
    }
}
```

**Key Features**:
- O(1) enqueue and dequeue operations
- Dynamic sizing with efficient memory management
- FIFO ordering guarantees
- Iterator support for inspection without modification

#### PriorityQueue
**Purpose**: Priority-based processing with heap ordering
```cursed
sus mut urgent_tasks = PriorityQueue::new();
urgent_tasks.enqueue(10, "fix_critical_bug".to_string())?;
urgent_tasks.enqueue(5, "code_review".to_string())?;

// Higher priority items come out first
lowkey (sus Ok((priority, task)) = urgent_tasks.dequeue()) {
    println("Priority {}: {}", priority, task)?;
}
```

**Key Features**:
- O(log n) enqueue and dequeue operations
- Binary heap implementation for optimal priority handling
- Stable ordering for equal priorities
- Custom priority comparison support

#### CircularQueue
**Purpose**: Fixed-size buffering with wrap-around behavior
```cursed
sus mut chat_buffer = CircularQueue::new(5)?;
// Add messages, automatically overwrites oldest when full
lowkey (sus i in 1..=10) {
    chat_buffer.enqueue(format!("Message {}", i))?;
}
```

**Key Features**:
- Fixed memory footprint
- Automatic overwriting of oldest elements
- O(1) operations with no dynamic allocation
- Perfect for streaming data and buffering

#### Deque (Double-Ended Queue)
**Purpose**: Efficient insertion and removal at both ends
```cursed
sus mut browser_history = Deque::new();
browser_history.push_back("https://github.com".to_string())?;
browser_history.push_front("https://docs.cursed.dev".to_string())?;

// Navigate forward or backward
lowkey (sus Ok(page) = browser_history.pop_back()) {
    println("Going back to: {}", page)?;
}
```

**Key Features**:
- O(1) push/pop operations at both ends
- Versatile: can function as queue or stack
- Efficient memory usage with segmented storage
- Bidirectional iterator support

### Stacks - LIFO Collections

#### Stack
**Purpose**: Last-In-First-Out processing
```cursed
sus mut call_stack = Stack::new();
call_stack.push("main()".to_string())?;
call_stack.push("process_request()".to_string())?;

lowkey (sus Ok(function) = call_stack.pop()) {
    println("Returning from: {}", function)?;
}
```

**Key Features**:
- O(1) push and pop operations
- Dynamic sizing with optimal memory usage
- LIFO ordering guarantees
- Peek operations for inspection

#### FixedStack
**Purpose**: Stack with capacity limits and overflow handling
```cursed
sus mut undo_stack = FixedStack::new(10)?;
// Limited to 10 undo operations
undo_stack.push("delete_text".to_string())?;
undo_stack.push("insert_image".to_string())?;
```

**Key Features**:
- Fixed memory footprint
- Capacity enforcement with error handling
- Prevents memory exhaustion in constrained environments
- O(1) operations with predictable performance

#### ThreadSafeStack
**Purpose**: Concurrent access with thread safety
```cursed
sus mut shared_work = ThreadSafeStack::new();
// Safe for use across multiple threads
shared_work.push("background_task".to_string())?;
```

**Key Features**:
- Thread-safe operations with internal synchronization
- Lock-based implementation for correctness
- Suitable for work-stealing patterns
- Performance optimized for concurrent access

### StackWithMin
**Purpose**: Stack that tracks minimum element efficiently
```cursed
sus mut min_stack = StackWithMin::new();
min_stack.push(5)?;
min_stack.push(3)?;
min_stack.push(7)?;

// O(1) minimum tracking
lowkey (sus Some(min_val) = min_stack.min()) {
    println("Current minimum: {}", min_val)?;
}
```

**Key Features**:
- O(1) minimum element tracking
- Standard stack operations with min functionality
- Space-efficient auxiliary storage
- Useful for algorithms requiring minimum tracking

## When to Use Each Collection

### Use HashSet When:
- ✅ You need fast membership testing (O(1))
- ✅ Duplicate prevention is important
- ✅ Order doesn't matter
- ✅ You have good hash functions for your data type
- ❌ You need sorted iteration
- ❌ You need range queries

### Use TreeSet When:
- ✅ You need sorted iteration
- ✅ Range queries are important
- ✅ You need predecessor/successor operations
- ✅ Consistent O(log n) performance is required
- ❌ You need the absolute fastest lookups
- ❌ Memory usage is extremely constrained

### Use BitSet When:
- ✅ You're storing boolean flags
- ✅ Memory efficiency is critical
- ✅ You need bitwise operations
- ✅ You have a known range of indices
- ❌ You need to store complex data
- ❌ Your index range is extremely sparse

### Use Queue When:
- ✅ You need FIFO processing
- ✅ Task scheduling is required
- ✅ Order of processing matters
- ✅ You're implementing algorithms like BFS
- ❌ You need random access
- ❌ You need priority-based processing

### Use PriorityQueue When:
- ✅ Priority-based processing is needed
- ✅ You're implementing algorithms like Dijkstra's
- ✅ Task scheduling with urgency levels
- ✅ You need efficient min/max operations
- ❌ You need to access arbitrary elements
- ❌ All items have equal priority

### Use CircularQueue When:
- ✅ You need fixed-size buffering
- ✅ Memory usage must be predictable
- ✅ You're implementing streaming data processing
- ✅ Overwriting old data is acceptable
- ❌ You need to grow the buffer dynamically
- ❌ All data must be preserved

### Use Deque When:
- ✅ You need efficient insertion/removal at both ends
- ✅ You're implementing algorithms requiring bidirectional access
- ✅ You need a versatile queue/stack hybrid
- ✅ You're building sliding window algorithms
- ❌ You primarily need middle insertions
- ❌ Random access is the primary requirement

### Use Stack When:
- ✅ You need LIFO processing
- ✅ You're implementing recursive algorithms iteratively
- ✅ Undo/redo functionality is required
- ✅ Expression evaluation or parsing
- ❌ You need random access
- ❌ Memory usage must be strictly limited

### Use FixedStack When:
- ✅ Memory usage must be bounded
- ✅ You're in a resource-constrained environment
- ✅ Stack overflow prevention is important
- ✅ Predictable performance is required
- ❌ You need unlimited capacity
- ❌ Dynamic sizing is beneficial

### Use ThreadSafeStack When:
- ✅ Multiple threads need access
- ✅ Work-stealing patterns are used
- ✅ Producer-consumer scenarios with LIFO ordering
- ✅ Thread safety is more important than peak performance
- ❌ Single-threaded performance is critical
- ❌ Lock-free operation is required

## Performance Characteristics

### Time Complexity Comparison

| Operation | HashSet | TreeSet | BitSet | Queue | PriorityQueue | Stack |
|-----------|---------|---------|--------|-------|---------------|-------|
| Insert    | O(1)*   | O(log n)| O(1)   | O(1)  | O(log n)      | O(1)  |
| Remove    | O(1)*   | O(log n)| O(1)   | O(1)  | O(log n)      | O(1)  |
| Search    | O(1)*   | O(log n)| O(1)   | O(n)  | O(n)          | O(n)  |
| Min/Max   | O(n)    | O(1)    | O(n)   | O(1)  | O(1)          | O(1)  |

*Average case; worst case O(n) due to hash collisions

### Space Complexity

| Collection    | Space Complexity | Memory Overhead | Notes |
|---------------|------------------|-----------------|-------|
| HashSet       | O(n)            | ~25-50%         | Hash table with load factor |
| TreeSet       | O(n)            | ~20-30%         | Tree node pointers |
| BitSet        | O(n/8)          | ~0%             | 1 bit per element |
| Queue         | O(n)            | ~10-20%         | Dynamic array with growth |
| PriorityQueue | O(n)            | ~10-15%         | Heap structure |
| Stack         | O(n)            | ~10-20%         | Dynamic array |

### Performance Benchmarks

Based on testing with 10,000 operations:

```
Collection Performance (10K operations):
HashSet:    Insert: 2.1ms,  Lookup: 1.8ms
TreeSet:    Insert: 15.3ms, Lookup: 12.7ms
Queue:      Enqueue/Dequeue: 1.2ms
Stack:      Push/Pop: 0.9ms
PriorityQueue: Enqueue/Dequeue: 8.4ms
```

## Integration and Interoperability

### Cross-Collection Operations

Collections are designed to work together seamlessly:

```cursed
// Example: Event processing pipeline
sus mut unique_users = HashSet::new();      // Track unique users
sus mut priority_tasks = PriorityQueue::new(); // Process by priority
sus mut recent_history = CircularQueue::new(100)?; // Keep recent history

// Processing pipeline
lowkey (sus event in events) {
    // Track unique users
    unique_users.insert(event.user_id)?;
    
    // Queue high-priority events
    lowkey (event.priority >= 8) {
        priority_tasks.enqueue(event.priority, event)?;
    }
    
    // Maintain recent history
    recent_history.enqueue(event.summary())?;
}
```

### Iterator Chaining

All collections support iterator protocols for functional programming:

```cursed
// Chain operations across different collections
sus combined_data: Vec<i32> = hash_set.iter()
    .copied()
    .chain(queue.iter())
    .chain(stack.iter())
    .filter(|&x| x > 10)
    .collect();
```

### Conversion Patterns

```cursed
// Convert between collection types
sus mut queue = Queue::new();
sus mut stack = Stack::new();

// Transfer data from queue to stack
bestie (!queue.is_empty()) {
    lowkey (sus Ok(item) = queue.dequeue()) {
        stack.push(item)?;
    }
}

// Convert set to sorted vector via TreeSet
sus mut tree_set = TreeSet::new();
lowkey (sus item in hash_set.iter()) {
    tree_set.insert(*item)?;
}
sus sorted_items: Vec<i32> = tree_set.iter().copied().collect();
```

## Best Practices

### 1. Choose the Right Collection

```cursed
// ❌ Poor choice: Using TreeSet when order doesn't matter
sus mut user_ids = TreeSet::new(); // Unnecessary overhead

// ✅ Better choice: HashSet for membership testing
sus mut user_ids = HashSet::new(); // O(1) operations
```

### 2. Optimize for Your Access Patterns

```cursed
// ❌ Poor pattern: Random access on Queue
lowkey (queue.iter().find(|&x| x == target)) { ... }

// ✅ Better pattern: Use appropriate collection
sus mut searchable = HashSet::new();
lowkey (searchable.contains(&target)) { ... }
```

### 3. Manage Memory Efficiently

```cursed
// ✅ Use fixed-size collections when appropriate
sus mut buffer = CircularQueue::new(1000)?; // Bounded memory
sus mut undo_stack = FixedStack::new(50)?;  // Limited undo history

// ✅ Pre-size collections when size is known
sus mut known_size_set = HashSet::with_capacity(expected_size);
```

### 4. Handle Errors Gracefully

```cursed
// ✅ Proper error handling
lowkey (sus result = fixed_stack.push(item)) {
    facts result?;
} flex {
    // Handle capacity exceeded
    overflow_handler.process(item)?;
}
```

### 5. Use Thread-Safe Collections Appropriately

```cursed
// ✅ Use thread-safe collections only when needed
sus mut shared_work = ThreadSafeStack::new(); // For concurrent access
sus mut local_work = Stack::new();            // For single-threaded use
```

### 6. Leverage Collection Combinations

```cursed
// ✅ Combine collections for complex workflows
collab TaskProcessor {
    sus pending: PriorityQueue<Task>,
    sus processing: HashSet<TaskId>,
    sus completed: CircularQueue<TaskResult>,
    sus errors: Stack<TaskError>,
}
```

## Migration Guide

### From Standard Collections

If migrating from other collection libraries:

#### From std::collections::HashMap to HashSet
```cursed
// Old pattern with HashMap<T, ()>
sus mut map: HashMap<String, ()> = HashMap::new();
map.insert("key".to_string(), ());

// New pattern with HashSet
sus mut set = HashSet::new();
set.insert("key".to_string())?;
```

#### From std::collections::VecDeque to Queue/Deque
```cursed
// For FIFO usage -> Queue
sus mut queue = Queue::new();
queue.enqueue(item)?;
lowkey (sus Ok(item) = queue.dequeue()) { ... }

// For double-ended usage -> Deque
sus mut deque = Deque::new();
deque.push_front(item)?;
deque.push_back(item)?;
```

#### From std::collections::BinaryHeap to PriorityQueue
```cursed
// Direct replacement with cleaner API
sus mut pq = PriorityQueue::new();
pq.enqueue(priority, item)?;
lowkey (sus Ok((priority, item)) = pq.dequeue()) { ... }
```

### Error Handling Migration

CURSED collections use consistent error handling:

```cursed
// All operations return CursedResult<T>
match collection.operation() {
    Ok(result) => { /* handle success */ },
    Err(CollectionsError::IndexOutOfBounds { index, size }) => {
        /* handle index error */
    },
    Err(CollectionsError::ElementNotFound { element }) => {
        /* handle not found */
    },
    // ... other error variants
}
```

## Code Examples

### Real-World Example 1: Web Server Request Processing

```cursed
collab RequestProcessor {
    sus pending_requests: PriorityQueue<Request>,
    sus active_sessions: HashSet<SessionId>,
    sus response_cache: CircularQueue<CachedResponse>,
    sus error_log: Stack<ErrorEvent>,
}

impl RequestProcessor {
    slay process_request(sus mut self, request: Request) -> CursedResult<()> {
        // Check for active session
        lowkey (self.active_sessions.contains(&request.session_id)) {
            // Process high-priority requests first
            self.pending_requests.enqueue(request.priority, request)?;
        } flex {
            // New session - track it
            self.active_sessions.insert(request.session_id)?;
            
            // Log new session
            self.error_log.push(ErrorEvent::new_session(request.session_id))?;
        }
        
        facts Ok(())
    }
    
    slay process_next_request(sus mut self) -> CursedResult<Option<Response>> {
        lowkey (sus Ok((priority, request)) = self.pending_requests.dequeue()) {
            // Process request
            lowkey (sus response = self.handle_request(request)?) {
                // Cache successful response
                self.response_cache.enqueue(CachedResponse::new(response.clone()))?;
                facts Ok(Some(response));
            } flex {
                // Log error
                self.error_log.push(ErrorEvent::processing_error(request.id))?;
                facts Ok(None);
            }
        } flex {
            facts Ok(None); // No pending requests
        }
    }
}
```

### Real-World Example 2: Game State Management

```cursed
collab GameState {
    sus players: TreeSet<PlayerId>,        // Sorted player list
    sus active_powerups: BitSet,           // Feature flags for powerups
    sus action_queue: Queue<PlayerAction>, // FIFO action processing
    sus undo_stack: FixedStack<GameAction>, // Limited undo history
    sus leaderboard: PriorityQueue<Score>, // High scores first
}

impl GameState {
    slay add_player(sus mut self, player_id: PlayerId) -> CursedResult<()> {
        self.players.insert(player_id)?;
        
        // Enable default powerups for new player
        self.active_powerups.set(POWERUP_SHIELD)?;
        self.active_powerups.set(POWERUP_SPEED)?;
        
        // Record action for undo
        self.undo_stack.push(GameAction::PlayerAdded(player_id))?;
        
        facts Ok(())
    }
    
    slay process_player_action(sus mut self, action: PlayerAction) -> CursedResult<()> {
        // Queue action for processing
        self.action_queue.enqueue(action.clone())?;
        
        // Record for undo (if space available)
        let _ = self.undo_stack.push(GameAction::ActionQueued(action));
        
        facts Ok(())
    }
    
    slay update_score(sus mut self, player_id: PlayerId, score: i32) -> CursedResult<()> {
        // Add to leaderboard (higher scores have higher priority)
        self.leaderboard.enqueue(score, player_id)?;
        
        facts Ok(())
    }
    
    slay get_top_players(sus self, count: usize) -> CursedResult<Vec<PlayerId>> {
        sus mut top_players = Vec::new();
        sus mut temp_queue = self.leaderboard.clone();
        
        lowkey (sus _i in 0..count) {
            lowkey (sus Ok((_score, player_id)) = temp_queue.dequeue()) {
                top_players.push(player_id);
            } flex {
                break; // No more players
            }
        }
        
        facts Ok(top_players)
    }
}
```

### Real-World Example 3: Chat Application

```cursed
collab ChatRoom {
    sus active_users: HashSet<UserId>,
    sus message_history: CircularQueue<Message>,
    sus typing_users: Queue<UserId>,
    sus banned_users: BitSet,
    sus moderator_actions: Stack<ModAction>,
}

impl ChatRoom {
    slay join_room(sus mut self, user_id: UserId) -> CursedResult<JoinResult> {
        // Check if user is banned
        lowkey (self.banned_users.is_set(user_id as usize)?) {
            facts Ok(JoinResult::Banned);
        }
        
        // Add to active users
        lowkey (self.active_users.insert(user_id)?) {
            // Record moderator action
            self.moderator_actions.push(ModAction::UserJoined(user_id))?;
            facts Ok(JoinResult::Success);
        } flex {
            facts Ok(JoinResult::AlreadyInRoom);
        }
    }
    
    slay send_message(sus mut self, user_id: UserId, content: String) -> CursedResult<()> {
        // Verify user is in room
        lowkey (self.active_users.contains(&user_id)) {
            sus message = Message::new(user_id, content);
            
            // Add to history (automatically removes old messages when full)
            self.message_history.enqueue(message)?;
            
            // Remove from typing queue if present
            self.remove_from_typing_queue(user_id)?;
            
            facts Ok(())
        } flex {
            facts Err(CollectionsError::ElementNotFound { 
                element: format!("User {} not in room", user_id) 
            })
        }
    }
    
    slay start_typing(sus mut self, user_id: UserId) -> CursedResult<()> {
        lowkey (self.active_users.contains(&user_id)) {
            self.typing_users.enqueue(user_id)?;
            facts Ok(())
        } flex {
            facts Err(CollectionsError::ElementNotFound { 
                element: format!("User {} not in room", user_id) 
            })
        }
    }
    
    slay ban_user(sus mut self, user_id: UserId, moderator_id: UserId) -> CursedResult<()> {
        // Remove from active users
        self.active_users.remove(&user_id);
        
        // Add to banned list
        self.banned_users.set(user_id as usize)?;
        
        // Record moderator action
        self.moderator_actions.push(ModAction::UserBanned { 
            user_id, 
            moderator_id 
        })?;
        
        facts Ok(())
    }
}
```

## Testing and Validation

### Unit Testing

Each collection type has comprehensive unit tests:

```bash
# Test individual collection types
make collections-test-all

# Test integration between collections
make collections-integration-test-all

# Performance benchmarks
make collections-integration-benchmark
```

### Integration Testing

The integration test suite validates:

- ✅ Interoperability between different collection types
- ✅ Cross-collection operations and conversions
- ✅ Performance characteristics under mixed workloads
- ✅ Memory efficiency when using multiple collections
- ✅ Real-world usage scenarios
- ✅ Error handling across collection boundaries
- ✅ Thread safety in concurrent environments

### Performance Testing

Performance tests ensure:

- ✅ Expected time complexity characteristics
- ✅ Memory usage within reasonable bounds
- ✅ Scalability up to large collection sizes
- ✅ Consistent performance across different data patterns

### Example Test Results

```
Collections Integration Test Results:
✅ Basic interoperability: 15 test cases passed
✅ Cross-collection operations: 12 test cases passed
✅ Performance comparison: All within expected bounds
✅ Memory efficiency: <3x overhead for all collections
✅ Real-world scenarios: 8 complex scenarios validated
✅ Error handling: All error conditions properly handled
✅ Thread safety: Concurrent operations validated

Performance Benchmarks (10K operations):
- HashSet operations: 2.1ms (target: <5ms) ✅
- TreeSet operations: 15.3ms (target: <20ms) ✅
- Queue operations: 1.2ms (target: <3ms) ✅
- Stack operations: 0.9ms (target: <2ms) ✅
- PriorityQueue operations: 8.4ms (target: <15ms) ✅
```

## Advanced Usage Patterns

### 1. Collection Composition

Create complex data structures by combining collections:

```cursed
collab Graph {
    sus vertices: HashSet<VertexId>,
    sus edges: HashMap<VertexId, HashSet<VertexId>>,
    sus visit_queue: Queue<VertexId>,
    sus visited: BitSet,
}
```

### 2. Memory Pool Pattern

Use fixed-size collections for predictable memory usage:

```cursed
collab MemoryEfficientProcessor {
    sus work_items: CircularQueue<WorkItem>,   // Fixed buffer
    sus free_workers: FixedStack<WorkerId>,    // Limited workers
    sus error_log: CircularQueue<ErrorEvent>, // Bounded error history
}
```

### 3. Producer-Consumer Pattern

Combine thread-safe and regular collections:

```cursed
collab ProducerConsumer {
    sus shared_queue: ThreadSafeStack<Task>,   // Cross-thread communication
    sus local_cache: HashSet<TaskId>,          // Per-thread deduplication
    sus processing_order: PriorityQueue<Task>, // Local prioritization
}
```

### 4. State Machine Pattern

Use collections to manage state transitions:

```cursed
collab StateMachine {
    sus states: HashSet<StateId>,
    sus transitions: HashMap<StateId, HashSet<StateId>>,
    sus history: Stack<StateTransition>,
    sus pending_events: PriorityQueue<Event>,
}
```

### 5. Cache Implementation

Combine collections for LRU/LFU caching:

```cursed
collab LRUCache {
    sus items: HashMap<Key, Value>,
    sus access_order: Deque<Key>,
    sus evicted: CircularQueue<EvictionEvent>,
}
```

## Conclusion

The CURSED Collections System provides a comprehensive, high-performance toolkit for data structure needs. By understanding the characteristics and appropriate use cases for each collection type, you can build efficient, maintainable applications that leverage the full power of CURSED's collection ecosystem.

Key takeaways:
- Choose collections based on access patterns and performance requirements
- Leverage collection interoperability for complex data processing workflows
- Use appropriate error handling for robust applications
- Consider memory constraints when selecting collection types
- Test thoroughly with the provided integration test suite

For detailed examples and real-world usage patterns, see `examples/collections_demo.csd` and the comprehensive test suite in `tests/collections_integration_test.rs`.
