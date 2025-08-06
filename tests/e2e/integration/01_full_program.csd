yeet "testz"
yeet "vibez"

test_start("Full Program Integration Test")

# Complete program demonstrating all major features

# Data structures
squad Task {
    spill id drip
    spill name tea
    spill completed lit
    spill priority drip
}

squad TaskManager {
    spill tasks []Task
    spill next_id drip
}

# Constructor
slay TaskManager.new() TaskManager {
    damn TaskManager{
        tasks: [],
        next_id: 1
    }
}

# Methods
slay TaskManager.add_task(name tea, priority drip) {
    sus new_task Task = Task{
        id: this.next_id,
        name: name,
        completed: cringe,
        priority: priority
    }
    
    this.tasks = append(this.tasks, new_task)
    this.next_id = this.next_id + 1
    
    vibez.spillf("Added task: %s (ID: %d, Priority: %d)", 
        name, new_task.id, priority)
}

slay TaskManager.complete_task(id drip) (lit, tea) {
    sus i drip
    range i, 0, len(this.tasks) {
        ready (this.tasks[i].id == id) {
            this.tasks[i].completed = based
            vibez.spillf("Completed task: %s", this.tasks[i].name)
            damn based, ""
        }
    }
    damn cringe, "task not found"
}

slay TaskManager.list_tasks() {
    vibez.spill("Task List:")
    vibez.spill("---------")
    
    ready (len(this.tasks) == 0) {
        vibez.spill("No tasks found")
        damn
    }
    
    range task in this.tasks {
        sus status tea = "[ ]"
        ready (task.completed) {
            status = "[x]"
        }
        
        vibez.spillf("%s %d. %s (Priority: %d)", 
            status, task.id, task.name, task.priority)
    }
    vibez.spill("")
}

slay TaskManager.get_stats() (drip, drip, drip) {
    sus total drip = len(this.tasks)
    sus completed drip = 0
    sus high_priority drip = 0
    
    range task in this.tasks {
        ready (task.completed) {
            completed = completed + 1
        }
        ready (task.priority >= 3) {
            high_priority = high_priority + 1
        }
    }
    
    damn total, completed, high_priority
}

# Error handling wrapper
slay safe_complete_task(manager *TaskManager, id drip) {
    shook {
        (success, err) := manager.complete_task(id)
        ready (!success) {
            vibez.spill_err("Error: " + err)
        }
    } catch (msg tea) {
        vibez.spill_err("Panic in complete_task: " + msg)
    }
}

# Concurrent task processing
sus processing_complete lit = cringe
sus processed_count drip = 0

slay process_high_priority_tasks(manager *TaskManager) {
    vibez.spill("Processing high priority tasks...")
    
    range task in manager.tasks {
        ready (task.priority >= 3 && !task.completed) {
            # Simulate processing time
            vibez.spillf("Processing task: %s", task.name)
            processed_count = processed_count + 1
        }
    }
    
    processing_complete = based
    vibez.spill("High priority processing complete")
}

# Main program logic
vibez.spill("=== CURSED Task Manager Demo ===")
vibez.spill("")

# Create task manager
sus manager TaskManager = TaskManager.new()

# Add some tasks
manager.add_task("Write documentation", 2)
manager.add_task("Fix critical bug", 4)
manager.add_task("Code review", 3)
manager.add_task("Update dependencies", 1)
manager.add_task("Deploy to production", 5)

# List initial tasks
manager.list_tasks()

# Complete some tasks
(success1, err1) := manager.complete_task(1)
assert_true(success1)
assert_eq_string(err1, "")

safe_complete_task(&manager, 3)

# Try to complete non-existent task
(success2, err2) := manager.complete_task(99)
assert_false(success2)
assert_eq_string(err2, "task not found")

# Show updated list
manager.list_tasks()

# Start concurrent processing
stan process_high_priority_tasks(&manager)

# Wait for processing
bestie (!processing_complete) {
    # Wait for completion
}

# Get statistics
(total, completed, high_priority) := manager.get_stats()

vibez.spill("=== Final Statistics ===")
vibez.spillf("Total tasks: %d", total)
vibez.spillf("Completed: %d", completed)
vibez.spillf("High priority: %d", high_priority)
vibez.spillf("Processed in background: %d", processed_count)

# Assertions for validation
assert_eq_int(total, 5)
assert_eq_int(completed, 2)
assert_eq_int(high_priority, 2)
assert_eq_int(processed_count, 1) # Only one high-priority incomplete task

vibez.spill("")
vibez.spill("=== Integration Test Complete ===")

print_test_summary()
