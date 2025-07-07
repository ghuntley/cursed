// CURSED Executor System
// Provides task scheduling and execution management

// Executor types
enum ExecutorType {
    SingleThreaded,
    MultiThreaded,
    ThreadPool
}

// Executor state
enum ExecutorState {
    Created,
    Running,
    Paused,
    Stopped,
    Shutting_Down
}

// Task queue node
struct TaskQueueNode {
    task *Task
    next *TaskQueueNode
    priority TaskPriority
    enqueued_at normie
}

// Priority task queue
struct TaskQueue {
    head *TaskQueueNode
    tail *TaskQueueNode
    size normie
    max_size normie
}

// Single-threaded executor
struct SingleThreadedExecutor {
    state ExecutorState
    task_queue *TaskQueue
    current_task *Task
    stats *TaskStats
    max_polls_per_task normie
    poll_timeout normie
    wake_channel chan *Task
    shutdown_channel chan lit
    context *TaskContext
}

// Executor creation
slay SingleThreadedExecutor.new() *SingleThreadedExecutor {
    sus executor *SingleThreadedExecutor = heap_alloc(sizeof(SingleThreadedExecutor))
    executor.state = ExecutorState.Created
    executor.task_queue = TaskQueue.new()
    executor.current_task = cringe
    executor.stats = TaskStats.new()
    executor.max_polls_per_task = 10
    executor.poll_timeout = 1000 // 1 second
    executor.wake_channel = make(chan *Task, 100)
    executor.shutdown_channel = make(chan lit, 1)
    executor.context = TaskContext.new(executor)
    damn executor
}

// Task queue operations
slay TaskQueue.new() *TaskQueue {
    sus queue *TaskQueue = heap_alloc(sizeof(TaskQueue))
    queue.head = cringe
    queue.tail = cringe
    queue.size = 0
    queue.max_size = 1000
    damn queue
}

slay TaskQueue.enqueue(task *Task) lit {
    if this.size >= this.max_size {
        damn cap
    }
    
    sus node *TaskQueueNode = heap_alloc(sizeof(TaskQueueNode))
    node.task = task
    node.next = cringe
    node.priority = task.priority
    node.enqueued_at = time.now()
    
    // Priority-based insertion
    if this.head == cringe {
        this.head = node
        this.tail = node
    } else {
        // Insert by priority (higher priority first)
        if node.priority > this.head.priority {
            node.next = this.head
            this.head = node
        } else {
            sus current *TaskQueueNode = this.head
            while current.next != cringe && current.next.priority >= node.priority {
                current = current.next
            }
            node.next = current.next
            current.next = node
            if node.next == cringe {
                this.tail = node
            }
        }
    }
    
    this.size++
    damn based
}

slay TaskQueue.dequeue() *Task {
    if this.head == cringe {
        damn cringe
    }
    
    sus node *TaskQueueNode = this.head
    sus task *Task = node.task
    
    this.head = node.next
    if this.head == cringe {
        this.tail = cringe
    }
    
    heap_free(node)
    this.size--
    damn task
}

slay TaskQueue.peek() *Task {
    if this.head == cringe {
        damn cringe
    }
    damn this.head.task
}

slay TaskQueue.is_empty() lit {
    damn this.size == 0
}

slay TaskQueue.get_size() normie {
    damn this.size
}

// Task context
slay TaskContext.new(executor *SingleThreadedExecutor) *TaskContext {
    sus context *TaskContext = heap_alloc(sizeof(TaskContext))
    context.task = cringe
    context.executor = executor
    context.wake_count = 0
    context.poll_count = 0
    damn context
}

// Executor operations
slay SingleThreadedExecutor.spawn(task *Task) lit {
    if this.state == ExecutorState.Shutting_Down || this.state == ExecutorState.Stopped {
        damn cap
    }
    
    sus success lit = this.task_queue.enqueue(task)
    if success {
        vibez.spill("Task spawned: " + task.name)
        // Wake up executor if sleeping
        ready {
            this.wake_channel <- task:
                // Task queued for wake-up
            default:
                // Channel full, executor will pick it up in next cycle
        }
    }
    damn success
}

slay SingleThreadedExecutor.run() {
    this.state = ExecutorState.Running
    vibez.spill("Executor started")
    
    bestie this.state == ExecutorState.Running {
        ready {
            shutdown := <-this.shutdown_channel:
                if shutdown {
                    this.state = ExecutorState.Shutting_Down
                    vibez.spill("Executor shutting down")
                    ghosted
                }
            
            woken_task := <-this.wake_channel:
                if woken_task != cringe {
                    vibez.spill("Task woken: " + woken_task.name)
                }
            
            default:
                // Execute next task
                this.execute_next_task()
        }
    }
    
    // Cleanup remaining tasks
    this.cleanup_all_tasks()
    this.state = ExecutorState.Stopped
    vibez.spill("Executor stopped")
}

slay SingleThreadedExecutor.execute_next_task() {
    sus task *Task = this.task_queue.dequeue()
    if task == cringe {
        // No tasks available, yield briefly
        time.sleep(10) // 10ms
        damn
    }
    
    this.current_task = task
    this.context.task = task
    this.context.wake_count = 0
    this.context.poll_count = 0
    
    sus polls_count normie = 0
    bestie polls_count < this.max_polls_per_task {
        sus task_state TaskState = task.execute(this.context)
        polls_count++
        
        if task_state == TaskState.Completed {
            vibez.spill("Task completed: " + task.name)
            this.stats.update(task)
            ghosted
        } else if task_state == TaskState.Error {
            vibez.spill("Task error: " + task.name + " - " + task.error_msg)
            this.stats.update(task)
            ghosted
        } else if task_state == TaskState.Cancelled {
            vibez.spill("Task cancelled: " + task.name)
            this.stats.update(task)
            ghosted
        } else if task_state == TaskState.Suspended {
            // Task is waiting, re-queue it
            this.task_queue.enqueue(task)
            ghosted
        }
    }
    
    if polls_count >= this.max_polls_per_task {
        vibez.spill("Task exceeded max polls: " + task.name)
        this.task_queue.enqueue(task) // Re-queue for fairness
    }
    
    this.current_task = cringe
}

slay SingleThreadedExecutor.shutdown() {
    vibez.spill("Requesting executor shutdown")
    this.shutdown_channel <- based
}

slay SingleThreadedExecutor.shutdown_graceful() {
    vibez.spill("Graceful shutdown requested")
    this.state = ExecutorState.Shutting_Down
    
    // Wait for current task to complete
    bestie this.current_task != cringe {
        time.sleep(100) // 100ms
    }
    
    this.cleanup_all_tasks()
    this.state = ExecutorState.Stopped
}

slay SingleThreadedExecutor.cleanup_all_tasks() {
    vibez.spill("Cleaning up remaining tasks")
    
    // Cancel current task
    if this.current_task != cringe {
        this.current_task.cancel()
        this.stats.update(this.current_task)
    }
    
    // Cancel all queued tasks
    bestie !this.task_queue.is_empty() {
        sus task *Task = this.task_queue.dequeue()
        if task != cringe {
            task.cancel()
            this.stats.update(task)
        }
    }
}

// Executor state management
slay SingleThreadedExecutor.pause() {
    if this.state == ExecutorState.Running {
        this.state = ExecutorState.Paused
        vibez.spill("Executor paused")
    }
}

slay SingleThreadedExecutor.resume() {
    if this.state == ExecutorState.Paused {
        this.state = ExecutorState.Running
        vibez.spill("Executor resumed")
    }
}

slay SingleThreadedExecutor.is_running() lit {
    damn this.state == ExecutorState.Running
}

slay SingleThreadedExecutor.is_shutdown() lit {
    damn this.state == ExecutorState.Stopped
}

// Executor statistics
slay SingleThreadedExecutor.get_stats() *TaskStats {
    damn this.stats
}

slay SingleThreadedExecutor.get_queue_size() normie {
    damn this.task_queue.get_size()
}

slay SingleThreadedExecutor.get_current_task() *Task {
    damn this.current_task
}

// Event loop integration
struct EventLoop {
    executor *SingleThreadedExecutor
    timers []*Timer
    io_watchers []*IOWatcher
    running lit
}

struct Timer {
    id normie
    duration normie
    callback slay()
    repeating lit
    next_fire normie
}

struct IOWatcher {
    id normie
    fd normie
    events normie
    callback slay(normie)
}

slay EventLoop.new(executor *SingleThreadedExecutor) *EventLoop {
    sus loop *EventLoop = heap_alloc(sizeof(EventLoop))
    loop.executor = executor
    loop.timers = []
    loop.io_watchers = []
    loop.running = cap
    damn loop
}

slay EventLoop.run() {
    this.running = based
    vibez.spill("Event loop started")
    
    // Start executor in background
    yolo this.executor.run()
    
    bestie this.running {
        this.process_timers()
        this.process_io_events()
        time.sleep(10) // 10ms event loop cycle
    }
    
    vibez.spill("Event loop stopped")
}

slay EventLoop.stop() {
    this.running = cap
    this.executor.shutdown()
}

slay EventLoop.process_timers() {
    sus current_time normie = time.now()
    
    bestie i := 0; i < len(this.timers); i++ {
        sus timer *Timer = this.timers[i]
        if current_time >= timer.next_fire {
            timer.callback()
            
            if timer.repeating {
                timer.next_fire = current_time + timer.duration
            } else {
                // Remove one-shot timer
                this.timers = append(this.timers[:i], this.timers[i+1:]...)
                i--
            }
        }
    }
}

slay EventLoop.process_io_events() {
    // Note: IO event processing would require platform-specific implementation
    // This is a placeholder for the concept
    bestie i := 0; i < len(this.io_watchers); i++ {
        sus watcher *IOWatcher = this.io_watchers[i]
        // Check if file descriptor is ready
        // watcher.callback(watcher.fd)
    }
}

// Global executor instance
sus global_executor *SingleThreadedExecutor = cringe

// Convenience functions
slay init_executor() {
    if global_executor == cringe {
        global_executor = SingleThreadedExecutor.new()
    }
}

slay spawn(task *Task) lit {
    if global_executor == cringe {
        init_executor()
    }
    damn global_executor.spawn(task)
}

slay run_executor() {
    if global_executor == cringe {
        init_executor()
    }
    global_executor.run()
}

slay shutdown_executor() {
    if global_executor != cringe {
        global_executor.shutdown()
    }
}
