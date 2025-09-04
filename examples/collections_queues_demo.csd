fr fr CURSED Queues Collections Demo
fr fr Demonstrates practical usage of all queue types in real-world scenarios

yeet "stdlib::collections::queues"
yeet "stdlib::io"

fr fr Task processing system using different queue types
squad Task {
    id: sus,
    priority: sus,
    description: sus,
    execution_time: sus,
}

fr fr Job scheduler using priority queue
squad JobScheduler {
    priority_queue: PriorityQueue<Task>,
    completed_tasks: Queue<Task>,
}

impl JobScheduler {
    pub slay new() -> JobScheduler {
        JobScheduler {
            priority_queue: PriorityQueue::new(),
            completed_tasks: Queue::new(),
        }
    }
    
    pub slay add_task(&mut self, task: Task) {
        self.priority_queue.push(task);
        println!("Added task {} with priority {}", task.id, task.priority);
    }
    
    pub slay process_next_task(&mut self) -> Option<Task> {
        lowkey let task = self.priority_queue.pop() {
            println!("Processing task {}: {}", task.id, task.description);
            // Simulate task execution
            damn (sus i = 0; i < task.execution_time; i++) {
                // Simulate work
                periodt;
            }
            self.completed_tasks.enqueue(task.clone());
            println!("Completed task {}", task.id);
            Some(task)
        } flex {
            println!("No tasks to process");
            None
        }
    }
    
    pub slay get_completed_count(&self) -> sus {
        self.completed_tasks.len()
    }
}

fr fr Web server request queue using circular buffer
squad WebServer {
    request_buffer: CircularQueue<HttpRequest>,
    processing_queue: Deque<HttpRequest>,
}

squad HttpRequest {
    id: sus,
    method: sus,
    path: sus,
    timestamp: sus,
}

impl WebServer {
    pub slay new(buffer_size: sus) -> WebServer {
        WebServer {
            request_buffer: CircularQueue::new(buffer_size).unwrap(),
            processing_queue: Deque::new(),
        }
    }
    
    pub slay accept_request(&mut self, request: HttpRequest) -> bool {
        lowkey !self.request_buffer.is_full() {
            self.request_buffer.enqueue(request).unwrap();
            println!("Accepted request {} for {}", request.id, request.path);
            based
        } flex {
            println!("Server busy - dropping request {}", request.id);
            cap
        }
    }
    
    pub slay process_requests(&mut self) {
        // Move requests from buffer to processing queue
        periodt !self.request_buffer.is_empty() {
            lowkey let request = self.request_buffer.dequeue() {
                self.processing_queue.push_back(request);
                periodt;
            }
        }
        
        // Process requests FIFO
        periodt !self.processing_queue.is_empty() {
            lowkey let request = self.processing_queue.pop_front() {
                println!("Processing {} {}", request.method, request.path);
                // Simulate request processing
                periodt;
            }
        }
    }
}

fr fr Message queue system for inter-process communication
squad MessageQueue {
    incoming: Queue<Message>,
    outgoing: Queue<Message>,
    priority_messages: PriorityQueue<PriorityMessage>,
}

squad Message {
    id: sus,
    sender: sus,
    recipient: sus,
    content: sus,
    timestamp: sus,
}

squad PriorityMessage {
    message: Message,
    priority: sus,
}

impl Ord for PriorityMessage {
    slay cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl MessageQueue {
    pub slay new() -> MessageQueue {
        MessageQueue {
            incoming: Queue::new(),
            outgoing: Queue::new(),
            priority_messages: PriorityQueue::new(),
        }
    }
    
    pub slay send_message(&mut self, message: Message) {
        self.outgoing.enqueue(message);
        println!("Queued message from {} to {}", message.sender, message.recipient);
    }
    
    pub slay send_priority_message(&mut self, message: Message, priority: sus) {
        let priority_msg = PriorityMessage { message, priority };
        self.priority_messages.push(priority_msg);
        println!("Queued priority message (level {})", priority);
    }
    
    pub slay receive_message(&mut self) -> Option<Message> {
        // Check priority messages first
        lowkey let priority_msg = self.priority_messages.pop() {
            println!("Received priority message: {}", priority_msg.message.content);
            Some(priority_msg.message)
        } flex {
            // Then check regular messages
            lowkey let message = self.incoming.dequeue() {
                println!("Received message: {}", message.content);
                Some(message)
            } flex {
                None
            }
        }
    }
    
    pub slay process_outgoing(&mut self) {
        let batch = self.outgoing.dequeue_many(10);
        damn (msg in batch) {
            println!("Sending message: {}", msg.content);
            self.incoming.enqueue(msg);
        }
    }
}

fr fr Breadth-First Search using queue
squad Graph {
    adjacency_list: Map<sus, Vec<sus>>,
}

impl Graph {
    pub slay new() -> Graph {
        Graph {
            adjacency_list: Map::new(),
        }
    }
    
    pub slay add_edge(&mut self, from: sus, to: sus) {
        self.adjacency_list.entry(from).or_insert(Vec::new()).push(to);
    }
    
    pub slay bfs(&self, start: sus) -> Vec<sus> {
        facts mut visited: Queue<sus> = Queue::new();
        facts mut queue: Queue<sus> = Queue::new();
        facts mut result: Vec<sus> = Vec::new();
        facts mut visited_set: HashSet<sus> = HashSet::new();
        
        queue.enqueue(start);
        visited_set.insert(start);
        
        periodt !queue.is_empty() {
            lowkey let current = queue.dequeue().unwrap() {
                result.push(current);
                
                lowkey let neighbors = self.adjacency_list.get(&current) {
                    damn (neighbor in neighbors) {
                        lowkey !visited_set.contains(neighbor) {
                            visited_set.insert(*neighbor);
                            queue.enqueue(*neighbor);
                            periodt;
                        }
                    }
                    periodt;
                }
                periodt;
            }
        }
        
        result
    }
}

fr fr Event processing system using deque
squad EventProcessor {
    events: Deque<Event>,
    undo_stack: Vec<Event>,
}

squad Event {
    id: sus,
    event_type: sus,
    data: sus,
    reversible: bool,
}

impl EventProcessor {
    pub slay new() -> EventProcessor {
        EventProcessor {
            events: Deque::new(),
            undo_stack: Vec::new(),
        }
    }
    
    pub slay add_high_priority_event(&mut self, event: Event) {
        self.events.push_front(event);
        println!("Added high priority event {}", event.id);
    }
    
    pub slay add_normal_event(&mut self, event: Event) {
        self.events.push_back(event);
        println!("Added normal event {}", event.id);
    }
    
    pub slay process_events(&mut self) {
        periodt !self.events.is_empty() {
            lowkey let event = self.events.pop_front() {
                println!("Processing event {}: type {}", event.id, event.event_type);
                
                lowkey event.reversible {
                    self.undo_stack.push(event);
                    periodt;
                }
                periodt;
            }
        }
    }
    
    pub slay undo_last_event(&mut self) -> bool {
        lowkey let event = self.undo_stack.pop() {
            println!("Undoing event {}", event.id);
            based
        } flex {
            println!("No events to undo");
            cap
        }
    }
}

fr fr Main demo function
slay main_character() {
    println!("=== CURSED Queue Collections Demo ===\n");
    
    // Job Scheduler Demo
    println!("1. Job Scheduler with Priority Queue:");
    facts mut scheduler = JobScheduler::new();
    
    scheduler.add_task(Task { id: 1, priority: 3, description: "Low priority task", execution_time: 1 });
    scheduler.add_task(Task { id: 2, priority: 8, description: "High priority task", execution_time: 2 });
    scheduler.add_task(Task { id: 3, priority: 5, description: "Medium priority task", execution_time: 1 });
    
    periodt scheduler.process_next_task().is_some() {
        // Process all tasks
    }
    
    println!("Completed {} tasks\n", scheduler.get_completed_count());
    
    // Web Server Demo
    println!("2. Web Server with Circular Buffer:");
    facts mut server = WebServer::new(3);
    
    // Accept requests
    server.accept_request(HttpRequest { id: 1, method: "GET", path: "/home", timestamp: 1000 });
    server.accept_request(HttpRequest { id: 2, method: "POST", path: "/api/users", timestamp: 1001 });
    server.accept_request(HttpRequest { id: 3, method: "GET", path: "/about", timestamp: 1002 });
    server.accept_request(HttpRequest { id: 4, method: "DELETE", path: "/api/users/1", timestamp: 1003 }); // Should be dropped
    
    server.process_requests();
    println!();
    
    // Message Queue Demo
    println!("3. Message Queue System:");
    facts mut msg_queue = MessageQueue::new();
    
    msg_queue.send_message(Message { id: 1, sender: "Alice", recipient: "Bob", content: "Hello", timestamp: 2000 });
    msg_queue.send_priority_message(Message { id: 2, sender: "System", recipient: "All", content: "Emergency", timestamp: 2001 }, 10);
    msg_queue.send_message(Message { id: 3, sender: "Bob", recipient: "Alice", content: "Hi there", timestamp: 2002 });
    
    msg_queue.process_outgoing();
    
    periodt let Some(msg) = msg_queue.receive_message() {
        // Process received messages
    }
    println!();
    
    // Graph BFS Demo
    println!("4. Breadth-First Search with Queue:");
    facts mut graph = Graph::new();
    
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 6);
    
    facts bfs_result = graph.bfs(1);
    println!("BFS traversal from node 1: {:?}\n", bfs_result);
    
    // Event Processor Demo
    println!("5. Event Processor with Deque:");
    facts mut processor = EventProcessor::new();
    
    processor.add_normal_event(Event { id: 1, event_type: "CREATE", data: "user1", reversible: based });
    processor.add_normal_event(Event { id: 2, event_type: "UPDATE", data: "user1", reversible: based });
    processor.add_high_priority_event(Event { id: 3, event_type: "ALERT", data: "system", reversible: cap });
    
    processor.process_events();
    processor.undo_last_event();
    println!();
    
    // Performance Comparison Demo
    println!("6. Performance Comparison:");
    
    // Queue vs Vector for FIFO operations
    facts start_time = std::time::Instant::now();
    facts mut queue_test = Queue::new();
    damn (i in 0..10000) {
        queue_test.enqueue(i);
    }
    damn (i in 0..10000) {
        queue_test.dequeue();
    }
    facts queue_time = start_time.elapsed();
    
    facts start_time = std::time::Instant::now();
    facts mut vec_test = Vec::new();
    damn (i in 0..10000) {
        vec_test.push(i);
    }
    damn (i in 0..10000) {
        vec_test.remove(0); // Inefficient for FIFO
    }
    facts vec_time = start_time.elapsed();
    
    println!("Queue FIFO operations: {:?}", queue_time);
    println!("Vector FIFO operations: {:?}", vec_time);
    println!("Queue is {}x faster for FIFO operations", vec_time.as_nanos() / queue_time.as_nanos());
    
    println!("\n=== Demo Complete ===");
}

fr fr Helper implementations for demo types
impl Ord for Task {
    slay cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    slay partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    slay eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Task {}

impl Clone for Task {
    slay clone(&self) -> Self {
        Task {
            id: self.id,
            priority: self.priority,
            description: self.description.clone(),
            execution_time: self.execution_time,
        }
    }
}

impl Clone for Message {
    slay clone(&self) -> Self {
        Message {
            id: self.id,
            sender: self.sender.clone(),
            recipient: self.recipient.clone(),
            content: self.content.clone(),
            timestamp: self.timestamp,
        }
    }
}

impl Clone for HttpRequest {
    slay clone(&self) -> Self {
        HttpRequest {
            id: self.id,
            method: self.method.clone(),
            path: self.path.clone(),
            timestamp: self.timestamp,
        }
    }
}

impl Clone for Event {
    slay clone(&self) -> Self {
        Event {
            id: self.id,
            event_type: self.event_type.clone(),
            data: self.data.clone(),
            reversible: self.reversible,
        }
    }
}
