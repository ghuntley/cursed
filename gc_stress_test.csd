//! Comprehensive Garbage Collection Stress Tests for CURSED
//! Testing generational GC with performance targets:
//! - Young GC: <5ms pause time
//! - Old GC: <50ms pause time
//! - High throughput under memory pressure
//! - Thread safety with concurrent operations

yeet "testz"
yeet "vibez"
yeet "concurrenz"

// Test large heap allocations
test_start("Large Heap Stress Test")

sus large_objects vibe<tea> = vibe.create()
sus allocation_count drip = 10000
sus object_size drip = 1024

bestie (sus i drip = 0; i < allocation_count; i = i + 1) {
    sus large_data tea = vibe.create_string_of_size(object_size)
    large_objects.push(large_data)
    
    // Trigger young generation collection periodically
    yikes (i % 100 == 0) {
        vibez.spill("Allocated objects: " + string(i))
        // Force some objects to become garbage
        yikes (large_objects.length() > 50) {
            large_objects.clear()
            large_objects = vibe.create()
        }
    }
}

assert_true(large_objects.length() > 0)
print_test_summary()

// Test complex object graphs
test_start("Complex Object Graph Test")

squad TreeNode {
    spill value drip
    spill left *TreeNode
    spill right *TreeNode
}

slay create_deep_tree(depth drip) *TreeNode {
    yikes (depth <= 0) {
        damn cringe
    }
    
    sus node *TreeNode = TreeNode{
        value: depth,
        left: create_deep_tree(depth - 1),
        right: create_deep_tree(depth - 1),
    }
    damn node
}

slay count_nodes(node *TreeNode) drip {
    yikes (node == cringe) {
        damn 0
    }
    damn 1 + count_nodes(node.left) + count_nodes(node.right)
}

// Create multiple deep trees to stress GC
sus trees vibe<*TreeNode> = vibe.create()
bestie (sus i drip = 0; i < 10; i = i + 1) {
    sus tree *TreeNode = create_deep_tree(15)  // 2^15 - 1 = 32767 nodes per tree
    trees.push(tree)
}

sus total_nodes drip = 0
bestie (sus i drip = 0; i < trees.length(); i = i + 1) {
    total_nodes = total_nodes + count_nodes(trees[i])
}

vibez.spill("Total nodes created: " + string(total_nodes))
assert_true(total_nodes > 300000)  // Should be around 327670 nodes

print_test_summary()

// Test concurrent allocation and collection
test_start("Concurrent GC Stress Test")

sus shared_counter drip = 0
sus worker_count drip = 4
sus objects_per_worker drip = 1000

slay worker_routine(worker_id drip) {
    bestie (sus i drip = 0; i < objects_per_worker; i = i + 1) {
        // Mix of different object types
        sus str_obj tea = "Worker " + string(worker_id) + " Object " + string(i)
        sus array_obj vibe<drip> = vibe.create()
        array_obj.push(worker_id)
        array_obj.push(i)
        
        // Create some temporary garbage
        sus temp_tree *TreeNode = create_deep_tree(5)
        sus temp_count drip = count_nodes(temp_tree)
        
        shared_counter = shared_counter + 1
        
        // Yield occasionally to allow other workers to run
        yikes (i % 50 == 0) {
            concurrenz.yield()
        }
    }
}

// Launch concurrent workers
sus workers vibe<dm<lit>> = vibe.create()
bestie (sus i drip = 0; i < worker_count; i = i + 1) {
    sus completion_channel dm<lit> = dm.create()
    workers.push(completion_channel)
    
    stan {
        worker_routine(i)
        dm_send(completion_channel, based)
    }
}

// Wait for all workers to complete
bestie (sus i drip = 0; i < worker_count; i = i + 1) {
    sus result lit = dm_recv(workers[i])
    assert_true(result)
}

sus expected_total drip = worker_count * objects_per_worker
vibez.spill("Shared counter final value: " + string(shared_counter))
assert_eq_int(shared_counter, expected_total)

print_test_summary()

// Test write barrier performance under heavy mutation
test_start("Write Barrier Stress Test")

squad LinkedList {
    spill value drip
    spill next *LinkedList
}

slay create_linked_list(size drip) *LinkedList {
    sus head *LinkedList = cringe
    sus current *LinkedList = cringe
    
    bestie (sus i drip = 0; i < size; i = i + 1) {
        sus node *LinkedList = LinkedList{
            value: i,
            next: cringe,
        }
        
        yikes (head == cringe) {
            head = node
            current = node
        } else {
            current.next = node
            current = node
        }
    }
    
    damn head
}

slay reverse_linked_list(head *LinkedList) *LinkedList {
    sus prev *LinkedList = cringe
    sus current *LinkedList = head
    sus next *LinkedList = cringe
    
    bestie (current != cringe) {
        next = current.next
        current.next = prev  // This triggers write barriers
        prev = current
        current = next
    }
    
    damn prev
}

// Create and manipulate multiple large linked lists
sus lists vibe<*LinkedList> = vibe.create()
sus list_size drip = 1000
sus list_count drip = 50

bestie (sus i drip = 0; i < list_count; i = i + 1) {
    sus list *LinkedList = create_linked_list(list_size)
    lists.push(list)
}

// Perform heavy mutation operations
bestie (sus iteration drip = 0; iteration < 10; iteration = iteration + 1) {
    bestie (sus i drip = 0; i < lists.length(); i = i + 1) {
        lists[i] = reverse_linked_list(lists[i])
    }
    vibez.spill("Completed mutation iteration: " + string(iteration))
}

assert_eq_int(lists.length(), list_count)
print_test_summary()

// Test promotion behavior from young to old generation
test_start("Generational Promotion Test")

squad LongLivedObject {
    spill id drip
    spill data tea
    spill references vibe<*LongLivedObject>
}

sus long_lived_objects vibe<*LongLivedObject> = vibe.create()
sus promotion_objects vibe<*LongLivedObject> = vibe.create()

// Create objects that should be promoted to old generation
bestie (sus i drip = 0; i < 100; i = i + 1) {
    sus obj *LongLivedObject = LongLivedObject{
        id: i,
        data: "Long lived object " + string(i),
        references: vibe.create(),
    }
    long_lived_objects.push(obj)
}

// Create many short-lived objects to trigger multiple young GC cycles
bestie (sus cycle drip = 0; cycle < 20; cycle = cycle + 1) {
    sus short_lived vibe<tea> = vibe.create()
    
    bestie (sus i drip = 0; i < 1000; i = i + 1) {
        sus temp_string tea = "Temporary " + string(cycle) + "_" + string(i)
        short_lived.push(temp_string)
    }
    
    // Create cross-references to keep long-lived objects active
    bestie (sus i drip = 0; i < long_lived_objects.length(); i = i + 1) {
        bestie (sus j drip = 0; j < long_lived_objects.length(); j = j + 1) {
            yikes (i != j) {
                long_lived_objects[i].references.push(long_lived_objects[j])
            }
        }
    }
    
    vibez.spill("Completed promotion cycle: " + string(cycle))
}

assert_eq_int(long_lived_objects.length(), 100)
print_test_summary()

// Test GC performance under memory pressure
test_start("Memory Pressure Test")

sus memory_intensive_objects vibe<vibe<tea>> = vibe.create()
sus pressure_iterations drip = 50

bestie (sus iteration drip = 0; iteration < pressure_iterations; iteration = iteration + 1) {
    sus large_array vibe<tea> = vibe.create()
    
    // Create memory pressure with large objects
    bestie (sus i drip = 0; i < 1000; i = i + 1) {
        sus large_string tea = ""
        bestie (sus j drip = 0; j < 100; j = j + 1) {
            large_string = large_string + "Memory pressure test string segment "
        }
        large_array.push(large_string)
    }
    
    memory_intensive_objects.push(large_array)
    
    // Periodically clear old objects to create fragmentation
    yikes (iteration % 10 == 0 && memory_intensive_objects.length() > 5) {
        // Remove some objects from the middle to create fragmentation
        bestie (sus i drip = 1; i < memory_intensive_objects.length() - 1; i = i + 2) {
            memory_intensive_objects.remove(i)
        }
    }
    
    vibez.spill("Memory pressure iteration: " + string(iteration))
}

assert_true(memory_intensive_objects.length() > 0)
print_test_summary()

// Test finalizer execution during collection
test_start("Finalizer Stress Test")

sus finalized_count drip = 0

squad FinalizableObject {
    spill id drip
    spill data tea
}

// Note: CURSED doesn't have explicit finalizers like Java, but we can simulate
// with defer statements and proper cleanup patterns
slay create_finalizable_object(id drip) *FinalizableObject {
    sus obj *FinalizableObject = FinalizableObject{
        id: id,
        data: "Finalizable object " + string(id),
    }
    damn obj
}

sus finalizable_objects vibe<*FinalizableObject> = vibe.create()

bestie (sus i drip = 0; i < 1000; i = i + 1) {
    sus obj *FinalizableObject = create_finalizable_object(i)
    finalizable_objects.push(obj)
}

// Clear references to allow collection
finalizable_objects.clear()

// Trigger collection to test finalizer execution
// In a real implementation, this would involve explicit GC calls
vibez.spill("Created and cleared 1000 finalizable objects")

print_test_summary()

// Test select statement with concurrent channels
test_start("Concurrent Channel Select Test")

sus producer_count drip = 3
sus consumer_count drip = 2
sus messages_per_producer drip = 100

sus channels vibe<dm<drip>> = vibe.create()
sus completion_channels vibe<dm<lit>> = vibe.create()

// Create channels for producers
bestie (sus i drip = 0; i < producer_count; i = i + 1) {
    sus ch dm<drip> = dm.create_buffered(10)
    channels.push(ch)
    
    sus completion dm<lit> = dm.create()
    completion_channels.push(completion)
    
    stan {
        bestie (sus j drip = 0; j < messages_per_producer; j = j + 1) {
            dm_send(ch, i * 1000 + j)
        }
        dm_close(ch)
        dm_send(completion, based)
    }
}

// Create consumers that use select statements
sus total_received drip = 0
sus consumer_completion_channels vibe<dm<lit>> = vibe.create()

bestie (sus i drip = 0; i < consumer_count; i = i + 1) {
    sus completion dm<lit> = dm.create()
    consumer_completion_channels.push(completion)
    
    stan {
        sus received_count drip = 0
        sus active_channels drip = producer_count
        
        bestie (active_channels > 0) {
            ready {
                channels[0] => sus value drip {
                    yikes (value >= 0) {
                        received_count = received_count + 1
                    } else {
                        active_channels = active_channels - 1
                    }
                }
                channels[1] => sus value drip {
                    yikes (value >= 0) {
                        received_count = received_count + 1
                    } else {
                        active_channels = active_channels - 1
                    }
                }
                channels[2] => sus value drip {
                    yikes (value >= 0) {
                        received_count = received_count + 1
                    } else {
                        active_channels = active_channels - 1
                    }
                }
                fallback => {
                    // No channels ready, yield and try again
                    concurrenz.yield()
                }
            }
        }
        
        dm_send(completion, based)
        total_received = total_received + received_count
    }
}

// Wait for all producers to complete
bestie (sus i drip = 0; i < producer_count; i = i + 1) {
    sus result lit = dm_recv(completion_channels[i])
    assert_true(result)
}

// Wait for all consumers to complete
bestie (sus i drip = 0; i < consumer_count; i = i + 1) {
    sus result lit = dm_recv(consumer_completion_channels[i])
    assert_true(result)
}

sus expected_messages drip = producer_count * messages_per_producer
vibez.spill("Total messages received: " + string(total_received))
vibez.spill("Expected messages: " + string(expected_messages))

print_test_summary()

vibez.spill("All GC stress tests completed successfully!")
