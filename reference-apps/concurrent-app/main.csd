# CURSED Concurrent Application - Producer-Consumer with Goroutines and Channels
# Demonstrates: Concurrency patterns, channels, select operations, synchronization

yeet "vibez"
yeet "concurrenz"
yeet "mathz"
yeet "timez"
yeet "stringz"
yeet "arrayz"

# Work item structure
squad WorkItem {
    id drip
    data tea
    priority drip
    created_at drip
    processing_time_ms drip
}

# Producer configuration
squad ProducerConfig {
    id drip
    items_per_second drip
    burst_size drip
    priority_range [2]drip
    data_patterns []tea
}

# Consumer configuration  
squad ConsumerConfig {
    id drip
    processing_time_ms drip
    error_rate meal
    batch_size drip
}

# System statistics
squad SystemStats {
    items_produced drip
    items_consumed drip
    items_in_queue drip
    errors_count drip
    total_processing_time_ms drip
    start_time drip
    producers_active drip
    consumers_active drip
}

# Thread-safe statistics tracker
squad StatsTracker {
    stats SystemStats
    stats_mutex concurrenz.Mutex
}

# Create new statistics tracker
slay new_stats_tracker() StatsTracker {
    damn StatsTracker{
        stats: SystemStats{
            start_time: timez.now()
        },
        stats_mutex: concurrenz.new_mutex()
    }
}

# Update stats safely
slay update_stats(tracker *StatsTracker, update_func slay(*SystemStats)) {
    concurrenz.lock(&tracker.stats_mutex)
    defer concurrenz.unlock(&tracker.stats_mutex)
    update_func(&tracker.stats)
}

# Get stats snapshot safely
slay get_stats_snapshot(tracker *StatsTracker) SystemStats {
    concurrenz.lock(&tracker.stats_mutex)
    defer concurrenz.unlock(&tracker.stats_mutex)
    damn tracker.stats
}

# Priority queue implementation for work items
squad PriorityQueue {
    items []WorkItem
    queue_mutex concurrenz.Mutex
    not_empty concurrenz.Condition
}

# Create new priority queue
slay new_priority_queue() PriorityQueue {
    sus queue PriorityQueue = {
        items: [],
        queue_mutex: concurrenz.new_mutex(),
        not_empty: concurrenz.new_condition()
    }
    damn queue
}

# Add item to priority queue
slay enqueue(queue *PriorityQueue, item WorkItem) {
    concurrenz.lock(&queue.queue_mutex)
    defer concurrenz.unlock(&queue.queue_mutex)
    
    # Insert item in priority order (higher priority first)
    sus inserted lit = false
    sus new_items []WorkItem = []
    
    bestie (existing in queue.items) {
        ready (!inserted && item.priority > existing.priority) {
            new_items = arrayz.append(new_items, item)
            inserted = based
        }
        new_items = arrayz.append(new_items, existing)
    }
    
    ready (!inserted) {
        new_items = arrayz.append(new_items, item)
    }
    
    queue.items = new_items
    concurrenz.signal(&queue.not_empty)
}

# Remove highest priority item from queue
slay dequeue(queue *PriorityQueue) yikes<WorkItem> {
    concurrenz.lock(&queue.queue_mutex)
    defer concurrenz.unlock(&queue.queue_mutex)
    
    bestie (len(queue.items) == 0) {
        concurrenz.wait(&queue.not_empty, &queue.queue_mutex)
    }
    
    ready (len(queue.items) == 0) {
        yikes "queue is empty"
    }
    
    sus item WorkItem = queue.items[0]
    queue.items = arrayz.slice(queue.items, 1, len(queue.items))
    damn item
}

# Get queue size
slay queue_size(queue *PriorityQueue) drip {
    concurrenz.lock(&queue.queue_mutex)
    defer concurrenz.unlock(&queue.queue_mutex)
    damn len(queue.items)
}

# Producer worker function
slay producer_worker(config ProducerConfig, work_queue *PriorityQueue, stats *StatsTracker, shutdown chan<lit>) {
    vibez.spill("🏭 Producer", config.id, "started")
    
    update_stats(stats, slay(s *SystemStats) {
        s.producers_active = s.producers_active + 1
    })
    
    defer update_stats(stats, slay(s *SystemStats) {
        s.producers_active = s.producers_active - 1
    })
    
    sus item_counter drip = 1
    sus last_burst_time drip = timez.now()
    sus burst_count drip = 0
    
    bestie (based) {
        # Check for shutdown signal
        select {
            when <-shutdown -> {
                vibez.spill("🔴 Producer", config.id, "shutting down")
                damn
            }
            when default -> {
                # Continue producing
            }
        }
        
        sus current_time drip = timez.now()
        
        # Implement burst production pattern
        ready (current_time - last_burst_time >= 1000 || burst_count < config.burst_size) {
            # Generate work item
            sus priority drip = mathz.random(config.priority_range[0], config.priority_range[1])
            sus pattern tea = config.data_patterns[mathz.random(0, len(config.data_patterns) - 1)]
            sus data tea = stringz.format("Producer_%d_Item_%d_%s", config.id, item_counter, pattern)
            
            sus item WorkItem = {
                id: item_counter,
                data: data,
                priority: priority,
                created_at: current_time,
                processing_time_ms: mathz.random(50, 500)
            }
            
            enqueue(work_queue, item)
            
            update_stats(stats, slay(s *SystemStats) {
                s.items_produced = s.items_produced + 1
                s.items_in_queue = s.items_in_queue + 1
            })
            
            item_counter = item_counter + 1
            burst_count = burst_count + 1
            
            ready (burst_count >= config.burst_size) {
                last_burst_time = current_time
                burst_count = 0
            }
            
            # Rate limiting
            sus delay_ms drip = 1000 / config.items_per_second
            timez.sleep(delay_ms)
        }
    }
}

# Consumer worker function  
slay consumer_worker(config ConsumerConfig, work_queue *PriorityQueue, stats *StatsTracker, results chan<WorkItem>, shutdown chan<lit>) {
    vibez.spill("🏗️  Consumer", config.id, "started")
    
    update_stats(stats, slay(s *SystemStats) {
        s.consumers_active = s.consumers_active + 1
    })
    
    defer update_stats(stats, slay(s *SystemStats) {
        s.consumers_active = s.consumers_active - 1
    })
    
    sus processed_count drip = 0
    
    bestie (based) {
        select {
            when <-shutdown -> {
                vibez.spill("🔴 Consumer", config.id, "shutting down (processed", processed_count, "items)")
                damn
            }
            when default -> {
                # Try to get work item
                sus item WorkItem = dequeue(work_queue) fam {
                    when _ -> {
                        timez.sleep(100)  # Brief wait before retry
                        skip
                    }
                }
                
                # Simulate processing time with potential errors
                sus processing_start drip = timez.now()
                
                ready (mathz.random_float() < config.error_rate) {
                    # Simulate processing error
                    vibez.spill("⚠️  Consumer", config.id, "encountered error processing item", item.id)
                    
                    update_stats(stats, slay(s *SystemStats) {
                        s.errors_count = s.errors_count + 1
                        s.items_in_queue = s.items_in_queue - 1
                    })
                } otherwise {
                    # Simulate actual work
                    sus processing_delay drip = mathz.random(config.processing_time_ms / 2, config.processing_time_ms * 2)
                    timez.sleep(processing_delay)
                    
                    sus actual_processing_time drip = timez.now() - processing_start
                    
                    # Send result
                    item.processing_time_ms = actual_processing_time
                    results <- item
                    
                    update_stats(stats, slay(s *SystemStats) {
                        s.items_consumed = s.items_consumed + 1
                        s.items_in_queue = s.items_in_queue - 1
                        s.total_processing_time_ms = s.total_processing_time_ms + actual_processing_time
                    })
                    
                    processed_count = processed_count + 1
                }
            }
        }
    }
}

# Results collector function
slay results_collector(results chan<WorkItem>, stats *StatsTracker, shutdown chan<lit>) {
    vibez.spill("📊 Results collector started")
    
    sus high_priority_count drip = 0
    sus medium_priority_count drip = 0
    sus low_priority_count drip = 0
    
    bestie (based) {
        select {
            when item := <-results -> {
                # Categorize by priority
                ready (item.priority >= 8) {
                    high_priority_count = high_priority_count + 1
                } ready (item.priority >= 5) {
                    medium_priority_count = medium_priority_count + 1
                } otherwise {
                    low_priority_count = low_priority_count + 1
                }
                
                # Log high-priority items
                ready (item.priority >= 8) {
                    vibez.spill("🔥 High priority item processed:", item.id, 
                               "in", item.processing_time_ms, "ms")
                }
            }
            when <-shutdown -> {
                vibez.spill("📊 Results collector final stats:")
                vibez.spill("   High priority items:", high_priority_count)
                vibez.spill("   Medium priority items:", medium_priority_count)
                vibez.spill("   Low priority items:", low_priority_count)
                damn
            }
        }
    }
}

# System monitor function
slay system_monitor(stats *StatsTracker, shutdown chan<lit>) {
    vibez.spill("📈 System monitor started")
    
    sus last_report_time drip = timez.now()
    sus last_produced drip = 0
    sus last_consumed drip = 0
    
    bestie (based) {
        select {
            when <-shutdown -> {
                vibez.spill("📈 System monitor shutting down")
                damn
            }
            when default -> {
                timez.sleep(5000)  # Report every 5 seconds
                
                sus current_stats SystemStats = get_stats_snapshot(stats)
                sus current_time drip = timez.now()
                sus time_diff drip = current_time - last_report_time
                
                # Calculate rates
                sus production_rate meal = (current_stats.items_produced - last_produced) * 1000.0 / time_diff
                sus consumption_rate meal = (current_stats.items_consumed - last_consumed) * 1000.0 / time_diff
                sus uptime_seconds drip = (current_time - current_stats.start_time) / 1000
                
                # Calculate average processing time
                sus avg_processing_time meal = ready (current_stats.items_consumed > 0) {
                    damn current_stats.total_processing_time_ms / current_stats.items_consumed
                } otherwise {
                    damn 0.0
                }
                
                vibez.spill("")
                vibez.spill("📊 System Status Report (Uptime:", uptime_seconds, "s)")
                vibez.spill("   Production: ", current_stats.items_produced, " items (", production_rate, "/s)")
                vibez.spill("   Consumption:", current_stats.items_consumed, " items (", consumption_rate, "/s)")
                vibez.spill("   Queue Size: ", current_stats.items_in_queue)
                vibez.spill("   Errors:     ", current_stats.errors_count)
                vibez.spill("   Avg Process:", avg_processing_time, "ms")
                vibez.spill("   Active:     ", current_stats.producers_active, " producers,", current_stats.consumers_active, " consumers")
                
                # Performance warnings
                ready (current_stats.items_in_queue > 100) {
                    vibez.spill("   ⚠️  High queue size - consider scaling consumers")
                }
                ready (production_rate > consumption_rate * 2) {
                    vibez.spill("   ⚠️  Production outpacing consumption")
                }
                ready (current_stats.errors_count > current_stats.items_consumed * 0.1) {
                    vibez.spill("   ⚠️  High error rate detected")
                }
                
                last_report_time = current_time
                last_produced = current_stats.items_produced
                last_consumed = current_stats.items_consumed
            }
        }
    }
}

# Load balancer for dynamic scaling
slay load_balancer(work_queue *PriorityQueue, stats *StatsTracker, 
                  producer_configs []ProducerConfig, consumer_configs []ConsumerConfig,
                  results chan<WorkItem>, shutdown chan<lit>) {
    vibez.spill("⚖️  Load balancer started")
    
    sus active_producers []drip = []
    sus active_consumers []drip = []
    
    # Start initial workers
    bestie (config in producer_configs) {
        go producer_worker(config, work_queue, stats, shutdown)
        active_producers = arrayz.append(active_producers, config.id)
    }
    
    bestie (config in consumer_configs) {
        go consumer_worker(config, work_queue, stats, results, shutdown)
        active_consumers = arrayz.append(active_consumers, config.id)
    }
    
    # Monitor and scale
    bestie (based) {
        select {
            when <-shutdown -> {
                vibez.spill("⚖️  Load balancer shutting down")
                damn
            }
            when default -> {
                timez.sleep(10000)  # Check every 10 seconds
                
                sus queue_size drip = queue_size(work_queue)
                sus current_stats SystemStats = get_stats_snapshot(stats)
                
                # Simple auto-scaling logic
                ready (queue_size > 50 && len(active_consumers) < 8) {
                    # Scale up consumers
                    sus new_consumer_id drip = len(consumer_configs) + len(active_consumers) + 1
                    sus new_config ConsumerConfig = {
                        id: new_consumer_id,
                        processing_time_ms: 200,
                        error_rate: 0.05,
                        batch_size: 1
                    }
                    
                    go consumer_worker(new_config, work_queue, stats, results, shutdown)
                    active_consumers = arrayz.append(active_consumers, new_consumer_id)
                    
                    vibez.spill("⚖️  Scaled up: added consumer", new_consumer_id, "(queue size:", queue_size, ")")
                }
            }
        }
    }
}

# Main concurrent application
slay main_character() {
    vibez.spill("🚀 CURSED Concurrent Application v1.0")
    vibez.spill("=====================================")
    vibez.spill("")
    vibez.spill("Features:")
    vibez.spill("  ✅ Producer-Consumer pattern")
    vibez.spill("  ✅ Priority queue processing")
    vibez.spill("  ✅ Goroutine-based concurrency")
    vibez.spill("  ✅ Channel communication")
    vibez.spill("  ✅ Select operations")
    vibez.spill("  ✅ Auto-scaling load balancer")
    vibez.spill("  ✅ Real-time monitoring")
    vibez.spill("  ✅ Thread-safe statistics")
    vibez.spill("")
    
    # Initialize system components
    sus work_queue PriorityQueue = new_priority_queue()
    sus stats StatsTracker = new_stats_tracker()
    sus results chan<WorkItem> = make_channel<WorkItem>()
    sus shutdown chan<lit> = make_channel<lit>()
    
    # Configure producers with different characteristics
    sus producer_configs []ProducerConfig = [
        {
            id: 1,
            items_per_second: 10,
            burst_size: 5,
            priority_range: [7, 10],
            data_patterns: ["URGENT", "CRITICAL", "HIGH_PRIORITY"]
        },
        {
            id: 2,
            items_per_second: 15,
            burst_size: 8,
            priority_range: [4, 7],
            data_patterns: ["NORMAL", "STANDARD", "ROUTINE"]
        },
        {
            id: 3,
            items_per_second: 20,
            burst_size: 12,
            priority_range: [1, 4],
            data_patterns: ["BATCH", "BACKGROUND", "LOW_PRIORITY"]
        }
    ]
    
    # Configure consumers with different processing capabilities
    sus consumer_configs []ConsumerConfig = [
        {id: 1, processing_time_ms: 150, error_rate: 0.02, batch_size: 1},
        {id: 2, processing_time_ms: 200, error_rate: 0.05, batch_size: 1},
        {id: 3, processing_time_ms: 100, error_rate: 0.01, batch_size: 1},
        {id: 4, processing_time_ms: 250, error_rate: 0.08, batch_size: 1}
    ]
    
    vibez.spill("🎬 Starting concurrent system...")
    vibez.spill("   Producers:", len(producer_configs))
    vibez.spill("   Consumers:", len(consumer_configs))
    vibez.spill("")
    
    # Start system components
    go results_collector(results, &stats, shutdown)
    go system_monitor(&stats, shutdown)
    go load_balancer(&work_queue, &stats, producer_configs, consumer_configs, results, shutdown)
    
    # Run system for demonstration
    vibez.spill("⏰ Running system for 30 seconds...")
    timez.sleep(30000)
    
    vibez.spill("")
    vibez.spill("🛑 Initiating graceful shutdown...")
    
    # Signal shutdown to all components
    go {
        shutdown <- based  # Results collector
        shutdown <- based  # System monitor  
        shutdown <- based  # Load balancer
        bestie (i drip in range(10)) {  # Extra shutdown signals for scaled workers
            shutdown <- based
        }
    }
    
    # Wait for components to shutdown
    timez.sleep(2000)
    
    # Final statistics report
    sus final_stats SystemStats = get_stats_snapshot(&stats)
    sus total_runtime drip = (timez.now() - final_stats.start_time) / 1000
    
    vibez.spill("")
    vibez.spill("📊 Final System Statistics")
    vibez.spill("==========================")
    vibez.spill("Runtime:           ", total_runtime, " seconds")
    vibez.spill("Items Produced:    ", final_stats.items_produced)
    vibez.spill("Items Consumed:    ", final_stats.items_consumed)
    vibez.spill("Items in Queue:    ", final_stats.items_in_queue)
    vibez.spill("Total Errors:      ", final_stats.errors_count)
    vibez.spill("Success Rate:      ", (final_stats.items_consumed * 100.0) / (final_stats.items_consumed + final_stats.errors_count), "%")
    
    ready (final_stats.items_consumed > 0) {
        sus avg_processing drip = final_stats.total_processing_time_ms / final_stats.items_consumed
        sus throughput meal = final_stats.items_consumed / total_runtime
        
        vibez.spill("Avg Processing:    ", avg_processing, " ms/item")
        vibez.spill("Throughput:        ", throughput, " items/second")
    }
    
    vibez.spill("")
    vibez.spill("🎉 Concurrent application demo completed!")
    vibez.spill("   ✅ Goroutines: Multiple producers and consumers")
    vibez.spill("   ✅ Channels: Work distribution and results collection")
    vibez.spill("   ✅ Select: Non-blocking communication patterns")
    vibez.spill("   ✅ Synchronization: Thread-safe statistics and priority queue")
    vibez.spill("   ✅ Patterns: Producer-consumer, auto-scaling, monitoring")
}

# Run the application
main()
