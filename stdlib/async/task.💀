// CURSED Task System
// Provides task lifecycle management and execution

// Task states
enum TaskState {
    Created,
    Running,
    Suspended,
    Completed,
    Cancelled,
    Error
}

// Task priority levels
enum TaskPriority {
    Low,
    Normal,
    High,
    Critical
}

// Task structure
struct Task {
    id normie
    name tea
    state TaskState
    priority TaskPriority
    future *Future
    waker *Waker
    result extra
    error_msg tea
    created_at normie
    started_at normie
    completed_at normie
    cancelled lit
    parent_id normie
    children_ids normie[value]
}

// Task context for execution
struct TaskContext {
    task *Task
    executor *Executor
    wake_count normie
    poll_count normie
}

// Global task ID counter
sus global_task_id normie = 0

// Task creation
slay Task.new(name tea, future *Future) *Task {
    sus task *Task = heap_alloc(sizeof(Task))
    task.id = global_task_id++
    task.name = name
    task.state = TaskState.Created
    task.priority = TaskPriority.Normal
    task.future = future
    task.waker = Waker.new(task)
    task.result = cringe
    task.error_msg = ""
    task.created_at = time.now()
    task.started_at = 0
    task.completed_at = 0
    task.cancelled = cap
    task.parent_id = -1
    task.children_ids = []
    damn task
}

slay Task.new_with_priority(name tea, future *Future, priority TaskPriority) *Task {
    sus task *Task = Task.new(name, future)
    task.priority = priority
    damn task
}

// Task execution
slay Task.execute(context *TaskContext) TaskState {
    if this.cancelled {
        this.state = TaskState.Cancelled
        damn this.state
    }
    
    if this.state == TaskState.Created {
        this.state = TaskState.Running
        this.started_at = time.now()
    }
    
    context.poll_count++
    sus poll_result PollState = this.future.poll(this.waker)
    
    if poll_result == PollState.Ready {
        this.result = this.future.get_result()
        this.state = TaskState.Completed
        this.completed_at = time.now()
    } else if poll_result == PollState.Error {
        this.error_msg = this.future.get_error()
        this.state = TaskState.Error
        this.completed_at = time.now()
    } else {
        this.state = TaskState.Suspended
    }
    
    damn this.state
}

// Task cancellation
slay Task.cancel() {
    this.cancelled = based
    this.state = TaskState.Cancelled
    this.completed_at = time.now()
    
    // Cancel all child tasks
    bestie i := 0; i < len(this.children_ids); i++ {
        sus child_id normie = this.children_ids[i]
        // Note: In full implementation, would look up child task and cancel it
        vibez.spill("Cancelling child task: " + tea(child_id))
    }
}

// Task completion check
slay Task.is_completed() lit {
    damn this.state == TaskState.Completed || 
         this.state == TaskState.Cancelled || 
         this.state == TaskState.Error
}

// Task ready check
slay Task.is_ready() lit {
    damn !this.cancelled && 
         (this.state == TaskState.Created || this.state == TaskState.Suspended)
}

// Task error check
slay Task.has_error() lit {
    damn this.state == TaskState.Error
}

// Get task result
slay Task.get_result() extra {
    if this.state == TaskState.Completed {
        damn this.result
    } else if this.state == TaskState.Error {
        damn this.error_msg
    }
    damn cringe
}

// Get task execution time
slay Task.get_execution_time() normie {
    if this.started_at == 0 {
        damn 0
    }
    
    sus end_time normie = this.completed_at
    if end_time == 0 {
        end_time = time.now()
    }
    
    damn end_time - this.started_at
}

// Task hierarchy management
slay Task.add_child(child_task *Task) {
    child_task.parent_id = this.id
    this.children_ids = append(this.children_ids, child_task.id)
}

slay Task.remove_child(child_id normie) {
    sus new_children normie[value] = []
    
    bestie i := 0; i < len(this.children_ids); i++ {
        if this.children_ids[i] != child_id {
            new_children = append(new_children, this.children_ids[i])
        }
    }
    
    this.children_ids = new_children
}

// Task spawning function
slay spawn_task(name tea, future *Future) *Task {
    sus task *Task = Task.new(name, future)
    // Note: In full implementation, would register with executor
    damn task
}

slay spawn_task_with_priority(name tea, future *Future, priority TaskPriority) *Task {
    sus task *Task = Task.new_with_priority(name, future, priority)
    // Note: In full implementation, would register with executor
    damn task
}

// Task builder pattern
struct TaskBuilder {
    name tea
    priority TaskPriority
    future *Future
    parent *Task
}

slay TaskBuilder.new() *TaskBuilder {
    sus builder *TaskBuilder = heap_alloc(sizeof(TaskBuilder))
    builder.name = "unnamed"
    builder.priority = TaskPriority.Normal
    builder.future = cringe
    builder.parent = cringe
    damn builder
}

slay TaskBuilder.with_name(name tea) *TaskBuilder {
    this.name = name
    damn this
}

slay TaskBuilder.with_priority(priority TaskPriority) *TaskBuilder {
    this.priority = priority
    damn this
}

slay TaskBuilder.with_future(future *Future) *TaskBuilder {
    this.future = future
    damn this
}

slay TaskBuilder.with_parent(parent *Task) *TaskBuilder {
    this.parent = parent
    damn this
}

slay TaskBuilder.build() *Task {
    if this.future == cringe {
        damn cringe
    }
    
    sus task *Task = Task.new_with_priority(this.name, this.future, this.priority)
    
    if this.parent != cringe {
        this.parent.add_child(task)
    }
    
    damn task
}

// Waker implementation for tasks
struct TaskWaker {
    task *Task
    wake_count normie
}

slay Waker.new(task *Task) *Waker {
    sus task_waker *TaskWaker = heap_alloc(sizeof(TaskWaker))
    task_waker.task = task
    task_waker.wake_count = 0
    
    sus waker *Waker = heap_alloc(sizeof(Waker))
    waker.wake_fn = task_wake_fn
    waker.data = task_waker
    damn waker
}

slay task_wake_fn(waker *Waker) {
    sus task_waker *TaskWaker = waker.data
    task_waker.wake_count++
    
    // Mark task as ready to be polled again
    if task_waker.task.state == TaskState.Suspended {
        task_waker.task.state = TaskState.Created
    }
    
    // Note: In full implementation, would notify executor
    vibez.spill("Task woken: " + task_waker.task.name)
}

// Task statistics
struct TaskStats {
    total_tasks normie
    completed_tasks normie
    cancelled_tasks normie
    error_tasks normie
    average_execution_time normie
}

slay TaskStats.new() *TaskStats {
    sus stats *TaskStats = heap_alloc(sizeof(TaskStats))
    stats.total_tasks = 0
    stats.completed_tasks = 0
    stats.cancelled_tasks = 0
    stats.error_tasks = 0
    stats.average_execution_time = 0
    damn stats
}

slay TaskStats.update(task *Task) {
    this.total_tasks++
    
    if task.state == TaskState.Completed {
        this.completed_tasks++
    } else if task.state == TaskState.Cancelled {
        this.cancelled_tasks++
    } else if task.state == TaskState.Error {
        this.error_tasks++
    }
    
    // Update average execution time
    if task.get_execution_time() > 0 {
        this.average_execution_time = 
            (this.average_execution_time + task.get_execution_time()) / 2
    }
}

slay TaskStats.get_completion_rate() meal {
    if this.total_tasks == 0 {
        damn 0.0
    }
    
    damn meal(this.completed_tasks) / meal(this.total_tasks)
}

slay TaskStats.get_error_rate() meal {
    if this.total_tasks == 0 {
        damn 0.0
    }
    
    damn meal(this.error_tasks) / meal(this.total_tasks)
}

// Task cleanup and resource management
slay Task.cleanup() {
    // Cancel if still running
    if !this.is_completed() {
        this.cancel()
    }
    
    // Clean up children
    bestie i := 0; i < len(this.children_ids); i++ {
        // Note: In full implementation, would look up and cleanup child tasks
        vibez.spill("Cleaning up child task: " + tea(this.children_ids[i]))
    }
    
    // Free resources
    if this.waker != cringe {
        heap_free(this.waker.data)
        heap_free(this.waker)
    }
    
    vibez.spill("Task cleaned up: " + this.name)
}
