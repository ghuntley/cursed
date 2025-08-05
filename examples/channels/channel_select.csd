fr fr Advanced Select Operations with Channels
fr fr Demonstrates complex multi-channel coordination patterns

vibe main

be_like Event squad {
    event_type tea
    data tea
    timestamp normie
    source tea
}

be_like ServiceStatus squad {
    service_name tea
    status tea
    response_time normie
    error_count normie
}

fr fr Multi-channel coordinator using select
slay event_coordinator() {
    puts("=== Event Coordinator Demo ===")
    
    // Create multiple event channels
    sus user_events = make(dm<Event>)
    sus system_events = make(dm<Event>)
    sus admin_events = make(dm<Event>)
    sus timeout_ch = time.After(10000)  // 10 second timeout
    
    // Start event producers
    stan user_event_producer(user_events)
    stan system_event_producer(system_events)
    stan admin_event_producer(admin_events)
    
    puts("Event coordinator starting - listening for events...")
    
    sus event_count = 0
    sus user_count = 0
    sus system_count = 0
    sus admin_count = 0
    
    periodt based {
        vibe_check {
            mood event := <-user_events:
                user_count++
                event_count++
                puts(sprintf("[USER] %s: %s", event.event_type, event.data))
                process_user_event(event)
                
            mood event := <-system_events:
                system_count++
                event_count++
                puts(sprintf("[SYSTEM] %s: %s", event.event_type, event.data))
                process_system_event(event)
                
            mood event := <-admin_events:
                admin_count++
                event_count++
                puts(sprintf("[ADMIN] %s: %s", event.event_type, event.data))
                process_admin_event(event)
                
            mood <-timeout_ch:
                puts("\n=== Event Coordinator Timeout ===")
                puts(sprintf("Total events processed: %d", event_count))
                puts(sprintf("User events: %d", user_count))
                puts(sprintf("System events: %d", system_count))
                puts(sprintf("Admin events: %d", admin_count))
                damn
                
            basic:
                // No events ready - do maintenance work
                puts("No events ready, performing maintenance...")
                sleep(200)
        }
    }
}

slay user_event_producer(events dm<Event>) {
    sus event_types = []tea{"login", "logout", "purchase", "view_page", "click_button"}
    
    sus i = 0
    periodt i < 8 {
        sus event = Event{
            event_type: event_types[i % len(event_types)],
            data: sprintf("User action %d", i+1),
            timestamp: get_timestamp(),
            source: "web_app"
        }
        
        events <- event
        sleep(random(800, 1500))
        i++
    }
    
    close(events)
}

slay system_event_producer(events dm<Event>) {
    sus event_types = []tea{"cpu_alert", "memory_warning", "disk_full", "network_error", "service_restart"}
    
    sus i = 0
    periodt i < 6 {
        sus event = Event{
            event_type: event_types[i % len(event_types)],
            data: sprintf("System alert %d", i+1),
            timestamp: get_timestamp(),
            source: "monitoring"
        }
        
        events <- event
        sleep(random(1200, 2000))
        i++
    }
    
    close(events)
}

slay admin_event_producer(events dm<Event>) {
    sus event_types = []tea{"user_created", "policy_updated", "backup_completed", "maintenance_scheduled"}
    
    sus i = 0
    periodt i < 4 {
        sus event = Event{
            event_type: event_types[i % len(event_types)],
            data: sprintf("Admin action %d", i+1),
            timestamp: get_timestamp(),
            source: "admin_panel"
        }
        
        events <- event
        sleep(random(2000, 3000))
        i++
    }
    
    close(events)
}

slay process_user_event(event Event) {
    vibe_check event.event_type {
        mood "login":
            puts("  → Processing user login")
        mood "purchase":
            puts("  → Processing purchase transaction")
        basic:
            puts("  → Processing general user event")
    }
}

slay process_system_event(event Event) {
    vibe_check event.event_type {
        mood "cpu_alert":
            puts("  → ALERT: High CPU usage detected!")
        mood "memory_warning":
            puts("  → WARNING: Memory usage high")
        basic:
            puts("  → Processing system event")
    }
}

slay process_admin_event(event Event) {
    puts("  → Processing admin event (high priority)")
}

fr fr Service health monitor with timeouts
slay service_monitor() {
    puts("\n=== Service Health Monitor ===")
    
    sus services = []tea{"api_server", "database", "cache", "auth_service", "file_storage"}
    sus status_channels = make([]dm<ServiceStatus>, len(services))
    
    // Create status channels for each service
    sus i = 0
    periodt i < len(services) {
        status_channels[i] = make(dm<ServiceStatus>)
        i++
    }
    
    // Start health checkers for each service
    i = 0
    periodt i < len(services) {
        stan health_checker(services[i], status_channels[i])
        i++
    }
    
    puts("Service monitor started - waiting for health reports...")
    
    sus healthy_services = 0
    sus unhealthy_services = 0
    sus timeout_count = 0
    sus check_timeout = time.After(15000)  // 15 second total timeout
    
    periodt based {
        vibe_check {
            mood status := <-status_channels[0]:  // api_server
                process_service_status(status)
                lowkey status.status == "healthy" {
                    healthy_services++
                } else {
                    unhealthy_services++
                }
                
            mood status := <-status_channels[1]:  // database
                process_service_status(status)
                lowkey status.status == "healthy" {
                    healthy_services++
                } else {
                    unhealthy_services++
                }
                
            mood status := <-status_channels[2]:  // cache
                process_service_status(status)
                lowkey status.status == "healthy" {
                    healthy_services++
                } else {
                    unhealthy_services++
                }
                
            mood status := <-status_channels[3]:  // auth_service
                process_service_status(status)
                lowkey status.status == "healthy" {
                    healthy_services++
                } else {
                    unhealthy_services++
                }
                
            mood status := <-status_channels[4]:  // file_storage
                process_service_status(status)
                lowkey status.status == "healthy" {
                    healthy_services++
                } else {
                    unhealthy_services++
                }
                
            mood <-check_timeout:
                puts("\n=== Service Monitor Summary ===")
                puts(sprintf("Healthy services: %d", healthy_services))
                puts(sprintf("Unhealthy services: %d", unhealthy_services))
                puts(sprintf("Timeout incidents: %d", timeout_count))
                damn
                
            basic:
                timeout_count++
                puts("No service responses - potential network issue")
                sleep(500)
        }
    }
}

slay health_checker(service_name tea, status_ch dm<ServiceStatus>) {
    puts(sprintf("Health checker for %s started", service_name))
    
    sus check_count = 0
    sus error_count = 0
    
    periodt check_count < 5 {
        // Simulate health check
        sleep(random(1000, 3000))
        
        sus is_healthy = random(1, 10) > 2  // 80% chance of being healthy
        sus response_time = random(50, 500)
        
        lowkey !is_healthy {
            error_count++
        }
        
        sus status = ServiceStatus{
            service_name: service_name,
            status: is_healthy ? "healthy" : "unhealthy",
            response_time: response_time,
            error_count: error_count
        }
        
        status_ch <- status
        check_count++
    }
    
    close(status_ch)
    puts(sprintf("Health checker for %s completed", service_name))
}

slay process_service_status(status ServiceStatus) {
    lowkey status.status == "healthy" {
        puts(sprintf("✓ %s: Healthy (RT: %dms)", status.service_name, status.response_time))
    } else {
        puts(sprintf("✗ %s: Unhealthy (Errors: %d)", status.service_name, status.error_count))
    }
}

fr fr Work dispatcher with priority queues
slay priority_work_dispatcher() {
    puts("\n=== Priority Work Dispatcher ===")
    
    sus critical_work = make(dm<Event>)
    sus high_work = make(dm<Event>)
    sus normal_work = make(dm<Event>)
    sus low_work = make(dm<Event>)
    sus worker_available = make(dm<lit>)
    
    // Start work generators
    stan generate_priority_work(critical_work, "critical", 3, 2000)
    stan generate_priority_work(high_work, "high", 5, 1500)
    stan generate_priority_work(normal_work, "normal", 8, 1000)
    stan generate_priority_work(low_work, "low", 10, 800)
    
    // Start workers
    sus num_workers = 3
    sus i = 0
    periodt i < num_workers {
        stan priority_worker(i+1, worker_available)
        i++
    }
    
    puts("Priority dispatcher starting - processing work by priority...")
    
    sus dispatched = 0
    sus timeout = time.After(20000)  // 20 second timeout
    
    periodt based {
        // Wait for worker to become available
        vibe_check {
            mood <-worker_available:
                // Worker is available, dispatch highest priority work
                vibe_check {
                    mood work := <-critical_work:
                        puts(sprintf("→ Dispatching CRITICAL work: %s", work.data))
                        dispatched++
                        
                    mood work := <-high_work:
                        puts(sprintf("→ Dispatching HIGH work: %s", work.data))
                        dispatched++
                        
                    mood work := <-normal_work:
                        puts(sprintf("→ Dispatching NORMAL work: %s", work.data))
                        dispatched++
                        
                    mood work := <-low_work:
                        puts(sprintf("→ Dispatching LOW work: %s", work.data))
                        dispatched++
                        
                    basic:
                        puts("No work available")
                }
                
            mood <-timeout:
                puts(sprintf("\nPriority dispatcher timeout - dispatched %d work items", dispatched))
                damn
        }
    }
}

slay generate_priority_work(work_ch dm<Event>, priority tea, count normie, delay normie) {
    puts(sprintf("Generating %d %s priority work items", count, priority))
    
    sus i = 0
    periodt i < count {
        sus work = Event{
            event_type: "work_item",
            data: sprintf("%s priority work %d", priority, i+1),
            timestamp: get_timestamp(),
            source: "work_generator"
        }
        
        work_ch <- work
        sleep(delay)
        i++
    }
    
    close(work_ch)
    puts(sprintf("%s priority work generation complete", priority))
}

slay priority_worker(worker_id normie, available dm<lit>) {
    puts(sprintf("Priority worker %d started", worker_id))
    
    periodt based {
        // Signal that worker is available
        available <- based
        
        // Simulate work processing
        sleep(random(500, 1500))
        puts(sprintf("Worker %d completed work", worker_id))
    }
}

fr fr Rate limiting with channels
slay rate_limiter_demo() {
    puts("\n=== Rate Limiter Demo ===")
    
    sus requests = make(dm<Event>)
    sus rate_limit = 3  // 3 requests per interval
    sus interval = 2000  // 2 seconds
    
    // Start request generator
    stan request_generator(requests)
    
    // Start rate limiter
    stan rate_limiter(requests, rate_limit, interval)
    
    sleep(15000)  // Run for 15 seconds
}

slay request_generator(requests dm<Event>) {
    puts("Request generator started")
    
    sus i = 0
    periodt i < 20 {
        sus request = Event{
            event_type: "api_request",
            data: sprintf("Request %d", i+1),
            timestamp: get_timestamp(),
            source: "client"
        }
        
        requests <- request
        sleep(random(200, 800))  // Variable request rate
        i++
    }
    
    close(requests)
    puts("Request generator finished")
}

slay rate_limiter(requests dm<Event>, limit normie, interval normie) {
    puts(sprintf("Rate limiter started - %d requests per %dms", limit, interval))
    
    sus tokens = make(dm<lit>, limit)
    
    // Fill token bucket initially
    sus i = 0
    periodt i < limit {
        tokens <- based
        i++
    }
    
    // Token replenishment
    stan {
        periodt based {
            sleep(interval)
            // Refill tokens
            i = 0
            periodt i < limit {
                vibe_check {
                    mood tokens <- based:
                        // Token added
                    basic:
                        // Bucket full, skip
                }
                i++
            }
            puts(sprintf("Tokens replenished (%d available)", len(tokens)))
        }
    }
    
    // Process requests with rate limiting
    periodt based {
        sus request, ok = <-requests
        lowkey !ok {
            puts("Rate limiter finished")
            damn
        }
        
        vibe_check {
            mood <-tokens:
                // Token available, process request
                puts(sprintf("✓ Processing: %s", request.data))
                // Simulate request processing
                sleep(100)
                
            basic:
                // No tokens available, reject request
                puts(sprintf("✗ Rate limited: %s", request.data))
        }
    }
}

slay main() {
    puts("=== Advanced Channel Select Operations ===")
    
    // Demonstrate various select patterns
    event_coordinator()
    service_monitor()
    priority_work_dispatcher()
    rate_limiter_demo()
    
    puts("\n=== All Select Demos Complete ===")
}

fr fr Utility functions
slay get_timestamp() normie {
    damn normie(time.Now().Unix())
}

slay random(min normie, max normie) normie {
    damn min + (time.Now().Unix() % (max - min + 1))
}
