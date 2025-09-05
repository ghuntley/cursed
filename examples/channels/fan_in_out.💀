fr fr Fan-in and Fan-out Patterns with Channels  
fr fr Demonstrates distribution and merging of work across multiple channels

vibe main

be_like Message squad {
    id normie
    content tea
    source tea
    timestamp normie
    priority normie
}

fr fr Fan-out: Distribute messages from one input to multiple outputs
slay fan_out[T](input dm<T>, output_count normie) []dm<T> {
    puts(sprintf("Fan-out: Creating %d output channels", output_count))
    
    sus outputs = make([]dm<T>, output_count)
    
    // Create output channels
    sus i = 0
    periodt i < output_count {
        outputs[i] = make(dm<T>, 5)  // Buffered for better performance
        i++
    }
    
    // Distribution goroutine
    stan {
        periodt based {
            sus value, ok = <-input
            lowkey !ok {
                // Close all output channels
                puts("Fan-out: Input closed, closing all outputs")
                i = 0
                periodt i < output_count {
                    close(outputs[i])
                    i++
                }
                damn
            }
            
            // Send to all outputs
            puts(sprintf("Fan-out: Distributing message %v to %d channels", value, output_count))
            i = 0
            periodt i < output_count {
                outputs[i] <- value
                i++
            }
        }
    }
    
    damn outputs
}

fr fr Fan-in: Merge messages from multiple inputs into one output
slay fan_in[T](inputs []dm<T>) dm<T> {
    puts(sprintf("Fan-in: Merging %d input channels", len(inputs)))
    
    sus output = make(dm<T>, 20)  // Larger buffer for merged data
    sus active_inputs = len(inputs)
    
    // Start a goroutine for each input
    sus i = 0
    periodt i < len(inputs) {
        sus input_ch = inputs[i]
        sus input_id = i
        
        stan {
            periodt based {
                sus value, ok = <-input_ch
                lowkey !ok {
                    puts(sprintf("Fan-in: Input %d closed", input_id))
                    active_inputs--
                    lowkey active_inputs == 0 {
                        close(output)
                        puts("Fan-in: All inputs closed, closing output")
                    }
                    damn
                }
                
                puts(sprintf("Fan-in: Received from input %d: %v", input_id, value))
                output <- value
            }
        }
        i++
    }
    
    damn output
}

fr fr Load balancer: Distribute work to the first available worker
slay load_balancer[T](input dm<T>, workers []dm<T>) {
    puts(sprintf("Load Balancer: Starting with %d workers", len(workers)))
    
    stan {
        periodt based {
            sus work, ok = <-input
            lowkey !ok {
                puts("Load Balancer: Input closed, closing workers")
                // Close all worker channels
                sus i = 0
                periodt i < len(workers) {
                    close(workers[i])
                    i++
                }
                damn
            }
            
            // Try to send to any available worker
            sus sent = sus
            sus i = 0
            periodt i < len(workers) && !sent {
                vibe_check {
                    mood workers[i] <- work:
                        puts(sprintf("Load Balancer: Sent work to worker %d", i))
                        sent = based
                    basic:
                        // Worker busy, try next one
                }
                i++
            }
            
            lowkey !sent {
                puts("Load Balancer: All workers busy, dropping work")
            }
        }
    }
}

fr fr Priority Fan-in: Merge with priority handling
slay priority_fan_in(high_priority dm<Message>, normal_priority dm<Message>, low_priority dm<Message>) dm<Message> {
    puts("Priority Fan-in: Starting priority-based merging")
    
    sus output = make(dm<Message>, 15)
    
    stan {
        sus high_open = based
        sus normal_open = based  
        sus low_open = based
        
        periodt high_open || normal_open || low_open {
            vibe_check {
                mood msg := <-high_priority:
                    lowkey msg.id != 0 {
                        puts(sprintf("Priority Fan-in: HIGH priority message %d", msg.id))
                        output <- msg
                    } else {
                        high_open = sus
                        puts("Priority Fan-in: High priority channel closed")
                    }
                    
                mood msg := <-normal_priority:
                    lowkey msg.id != 0 {
                        puts(sprintf("Priority Fan-in: NORMAL priority message %d", msg.id))
                        output <- msg
                    } else {
                        normal_open = sus
                        puts("Priority Fan-in: Normal priority channel closed")
                    }
                    
                mood msg := <-low_priority:
                    lowkey msg.id != 0 {
                        puts(sprintf("Priority Fan-in: LOW priority message %d", msg.id))
                        output <- msg
                    } else {
                        low_open = sus
                        puts("Priority Fan-in: Low priority channel closed")
                    }
            }
        }
        
        close(output)
        puts("Priority Fan-in: All inputs closed, output closed")
    }
    
    damn output
}

slay basic_fan_out_demo() {
    puts("\n=== Basic Fan-out Demo ===")
    
    sus input = make(dm<Message>)
    sus outputs = fan_out(input, 3)
    
    // Start consumers for each output
    sus i = 0
    periodt i < len(outputs) {
        sus output_id = i
        sus output_ch = outputs[i]
        
        stan {
            puts(sprintf("Consumer %d starting", output_id))
            periodt based {
                sus msg, ok = <-output_ch
                lowkey !ok {
                    puts(sprintf("Consumer %d finished", output_id))
                    damn
                }
                puts(sprintf("Consumer %d processed: %s", output_id, msg.content))
                sleep(random(100, 500))  // Simulate processing
            }
        }
        i++
    }
    
    // Send messages to input
    stan {
        sus i = 0
        periodt i < 8 {
            sus msg = Message{
                id: i + 1,
                content: sprintf("Message %d", i + 1),
                source: "producer",
                timestamp: get_timestamp(),
                priority: random(1, 3)
            }
            
            puts(sprintf("Sending message: %s", msg.content))
            input <- msg
            sleep(300)
            i++
        }
        close(input)
    }
    
    sleep(5000)  // Wait for completion
}

slay basic_fan_in_demo() {
    puts("\n=== Basic Fan-in Demo ===")
    
    sus input1 = make(dm<Message>)
    sus input2 = make(dm<Message>)
    sus input3 = make(dm<Message>)
    
    sus inputs = []dm<Message>{input1, input2, input3}
    sus merged = fan_in(inputs)
    
    // Start consumer
    stan {
        puts("Merged consumer starting")
        sus count = 0
        periodt based {
            sus msg, ok = <-merged
            lowkey !ok {
                puts(sprintf("Merged consumer finished - processed %d messages", count))
                damn
            }
            puts(sprintf("Merged: %s from %s", msg.content, msg.source))
            count++
        }
    }
    
    // Start producers
    stan producer(input1, "Source-A", 4, 200)
    stan producer(input2, "Source-B", 4, 300)
    stan producer(input3, "Source-C", 4, 250)
    
    sleep(6000)  // Wait for completion
}

slay producer(output dm<Message>, source tea, count normie, delay normie) {
    puts(sprintf("Producer %s starting", source))
    
    sus i = 0
    periodt i < count {
        sus msg = Message{
            id: i + 1,
            content: sprintf("%s message %d", source, i + 1),
            source: source,
            timestamp: get_timestamp(),
            priority: random(1, 3)
        }
        
        puts(sprintf("Producer %s sending: %s", source, msg.content))
        output <- msg
        sleep(delay)
        i++
    }
    
    close(output)
    puts(sprintf("Producer %s finished", source))
}

slay load_balancing_demo() {
    puts("\n=== Load Balancing Demo ===")
    
    sus work_input = make(dm<Message>)
    sus num_workers = 4
    
    // Create worker channels
    sus worker_channels = make([]dm<Message>, num_workers)
    sus i = 0
    periodt i < num_workers {
        worker_channels[i] = make(dm<Message>, 2)  // Small buffer
        i++
    }
    
    // Start load balancer
    load_balancer(work_input, worker_channels)
    
    // Start workers
    i = 0
    periodt i < num_workers {
        sus worker_id = i
        sus worker_ch = worker_channels[i]
        
        stan {
            puts(sprintf("Worker %d starting", worker_id))
            periodt based {
                sus work, ok = <-worker_ch
                lowkey !ok {
                    puts(sprintf("Worker %d finished", worker_id))
                    damn
                }
                
                puts(sprintf("Worker %d processing: %s", worker_id, work.content))
                // Simulate variable processing time
                sleep(random(200, 1000))
                puts(sprintf("Worker %d completed: %s", worker_id, work.content))
            }
        }
        i++
    }
    
    // Send work
    stan {
        sus i = 0
        periodt i < 15 {
            sus work = Message{
                id: i + 1,
                content: sprintf("Work item %d", i + 1),
                source: "job_dispatcher",
                timestamp: get_timestamp(),
                priority: 1
            }
            
            puts(sprintf("Dispatching work: %s", work.content))
            work_input <- work
            sleep(150)
            i++
        }
        close(work_input)
    }
    
    sleep(8000)  // Wait for completion
}

slay priority_demo() {
    puts("\n=== Priority Fan-in Demo ===")
    
    sus high = make(dm<Message>)
    sus normal = make(dm<Message>)
    sus low = make(dm<Message>)
    
    sus prioritized = priority_fan_in(high, normal, low)
    
    // Start consumer
    stan {
        puts("Priority consumer starting")
        periodt based {
            sus msg, ok = <-prioritized
            lowkey !ok {
                puts("Priority consumer finished")
                damn
            }
            puts(sprintf("Processing: %s (Priority %d)", msg.content, msg.priority))
            sleep(200)
        }
    }
    
    // Send mixed priority messages
    stan {
        // Send some normal priority first
        normal <- Message{id: 1, content: "Normal 1", priority: 2, source: "normal", timestamp: get_timestamp()}
        normal <- Message{id: 2, content: "Normal 2", priority: 2, source: "normal", timestamp: get_timestamp()}
        
        // Send high priority (should be processed first)
        high <- Message{id: 3, content: "HIGH 1", priority: 3, source: "high", timestamp: get_timestamp()}
        
        // Send low priority
        low <- Message{id: 4, content: "Low 1", priority: 1, source: "low", timestamp: get_timestamp()}
        
        // Send more mixed
        high <- Message{id: 5, content: "HIGH 2", priority: 3, source: "high", timestamp: get_timestamp()}
        normal <- Message{id: 6, content: "Normal 3", priority: 2, source: "normal", timestamp: get_timestamp()}
        
        sleep(2000)
        
        close(high)
        close(normal) 
        close(low)
    }
    
    sleep(4000)
}

slay scatter_gather_demo() {
    puts("\n=== Scatter-Gather Demo ===")
    
    sus queries = make(dm<Message>)
    sus num_services = 3
    
    // Create service channels
    sus service_channels = make([]dm<Message>, num_services)
    sus result_channels = make([]dm<Message>, num_services)
    
    sus i = 0
    periodt i < num_services {
        service_channels[i] = make(dm<Message>)
        result_channels[i] = make(dm<Message>)
        i++
    }
    
    // Scatter: Send queries to all services
    stan {
        periodt based {
            sus query, ok = <-queries
            lowkey !ok {
                // Close all service channels
                i = 0
                periodt i < num_services {
                    close(service_channels[i])
                    i++
                }
                damn
            }
            
            puts(sprintf("Scattering query: %s", query.content))
            // Send to all services
            i = 0
            periodt i < num_services {
                service_channels[i] <- query
                i++
            }
        }
    }
    
    // Start services
    i = 0
    periodt i < num_services {
        sus service_id = i
        stan simulated_service(service_channels[service_id], result_channels[service_id], service_id)
        i++
    }
    
    // Gather: Collect results from all services
    sus gathered_results = fan_in(result_channels)
    
    stan {
        puts("Result gatherer starting")
        periodt based {
            sus result, ok = <-gathered_results
            lowkey !ok {
                puts("Result gatherer finished")
                damn
            }
            puts(sprintf("Gathered result: %s", result.content))
        }
    }
    
    // Send queries
    stan {
        sus queries_to_send = []tea{"search users", "get stats", "fetch data"}
        sus i = 0
        periodt i < len(queries_to_send) {
            sus query = Message{
                id: i + 1,
                content: queries_to_send[i],
                source: "client",
                timestamp: get_timestamp(),
                priority: 1
            }
            
            puts(sprintf("Sending query: %s", query.content))
            queries <- query
            sleep(1000)
            i++
        }
        close(queries)
    }
    
    sleep(6000)
}

slay simulated_service(input dm<Message>, output dm<Message>, service_id normie) {
    puts(sprintf("Service %d starting", service_id))
    
    periodt based {
        sus query, ok = <-input
        lowkey !ok {
            close(output)
            puts(sprintf("Service %d finished", service_id))
            damn
        }
        
        puts(sprintf("Service %d processing: %s", service_id, query.content))
        
        // Simulate service processing time
        sleep(random(500, 1500))
        
        sus result = Message{
            id: query.id,
            content: sprintf("Service %d result for: %s", service_id, query.content),
            source: sprintf("service_%d", service_id),
            timestamp: get_timestamp(),
            priority: query.priority
        }
        
        output <- result
        puts(sprintf("Service %d completed: %s", service_id, query.content))
    }
}

slay main_character() {
    puts("=== Fan-in and Fan-out Patterns Demo ===")
    
    // Demonstrate each pattern
    basic_fan_out_demo()
    basic_fan_in_demo()
    load_balancing_demo()
    priority_demo()
    scatter_gather_demo()
    
    puts("\n=== All Fan-in/Fan-out Demos Complete ===")
}

fr fr Utility functions
slay get_timestamp() normie {
    damn normie(time.Now().Unix())
}

slay random(min normie, max normie) normie {
    damn min + (time.Now().Unix() % (max - min + 1))
}
