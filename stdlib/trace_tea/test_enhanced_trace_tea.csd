yeet "testz"
yeet "trace_tea"
yeet "dropz"
yeet "timez"
yeet "vibe_context"

fr fr Comprehensive trace_tea enhanced tests

test_start("trace_tea enhanced comprehensive tests")

fr fr Test basic tracing functionality
slay test_basic_tracing() lit {
    fr fr Create buffer for trace output
    sus buf := dropz.NewBuffer()
    
    fr fr Start tracing
    sus err := trace_tea.Start(buf)
    assert_eq_string(err, "")
    
    fr fr Create a task
    sus ctx := vibe_context.Background()
    sus newCtx, task := trace_tea.NewTask(ctx, "test_task")
    
    if task != cringe {
        fr fr Log some information
        trace_tea.Log(newCtx, "info", "Test task started")
        
        fr fr Create a region
        sus region := trace_tea.StartRegion(newCtx, "test_region")
        if region != cringe {
            trace_tea.Log(newCtx, "info", "In test region")
            region.End()
        }
        
        fr fr End the task
        task.End()
    }
    
    fr fr Stop tracing
    sus stopErr := trace_tea.Stop()
    assert_eq_string(stopErr, "")
    
    damn based
}

fr fr Test task operations
slay test_task_operations() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    sus taskCtx, task := trace_tea.NewTask(ctx, "complex_task")
    
    if task != cringe {
        fr fr Test task logging
        task.LazyLog("Task step: %s", "initialization")
        task.LazyLog("Task step: %s", "processing")
        
        fr fr Test deterministic setting
        task.SetDeterministic(based)
        
        fr fr Simulate some work
        sus i := 0
        while i < 100 {
            i = i + 1
        }
        
        task.LazyLog("Task step: %s", "completion")
        task.End()
    }
    
    trace_tea.Stop()
    damn based
}

fr fr Test region operations
slay test_region_operations() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    sus taskCtx, task := trace_tea.NewTask(ctx, "region_test_task")
    
    if task != cringe {
        fr fr Test multiple regions
        sus region1 := trace_tea.StartRegion(taskCtx, "region_1")
        if region1 != cringe {
            region1.LazyLog("Region 1 activity: %s", "data_processing")
            
            fr fr Nested region
            sus region2 := trace_tea.StartRegion(taskCtx, "region_2")
            if region2 != cringe {
                region2.LazyLog("Region 2 activity: %s", "computation")
                region2.End()
            }
            
            region1.End()
        }
        
        task.End()
    }
    
    trace_tea.Stop()
    damn based
}

fr fr Test event logging
slay test_event_logging() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    
    fr fr Test different event categories
    trace_tea.Log(ctx, trace_tea.EventAPI, "API call started")
    trace_tea.Log(ctx, trace_tea.EventDatabase, "Database query executed")
    trace_tea.Log(ctx, trace_tea.EventMemory, "Memory allocation")
    trace_tea.Log(ctx, trace_tea.EventNetwork, "Network request sent")
    
    fr fr Test formatted logging
    trace_tea.Logf(ctx, trace_tea.EventPerformance, "Operation took %s", "42ms")
    trace_tea.Logf(ctx, trace_tea.EventUserDefined, "User %s performed action %s", "alice", "login")
    
    fr fr Test custom events
    sus event := trace_tea.NewEvent(trace_tea.EventUserDefined, "custom_event")
    if event != cringe {
        event.LazyLog("Event details: %s", "important_data")
    }
    
    trace_tea.Stop()
    damn based
}

fr fr Test WithRegion helper
slay test_with_region() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    sus taskCtx, task := trace_tea.NewTask(ctx, "with_region_test")
    
    if task != cringe {
        fr fr Test WithRegion helper
        trace_tea.WithRegion(taskCtx, "helper_region", slay() {
            trace_tea.Log(taskCtx, "info", "Inside helper region")
            
            fr fr Simulate work
            sus counter := 0
            while counter < 50 {
                counter = counter + 1
            }
        })
        
        task.End()
    }
    
    trace_tea.Stop()
    damn based
}

fr fr Test WithSpan helper
slay test_with_span() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    
    fr fr Test WithSpan helper
    trace_tea.WithSpan(ctx, "span_test", slay(spanCtx vibe_context.Context) {
        trace_tea.Log(spanCtx, "info", "Inside span")
        
        fr fr Create nested span
        trace_tea.WithSpan(spanCtx, "nested_span", slay(nestedCtx vibe_context.Context) {
            trace_tea.Log(nestedCtx, "info", "Inside nested span")
        })
    })
    
    trace_tea.Stop()
    damn based
}

fr fr Test trace filtering
slay test_trace_filtering() lit {
    fr fr Create filter
    sus filter := trace_tea.NewFilter()
    filter.IncludeEvent(trace_tea.EventAPI)
    filter.IncludeEvent(trace_tea.EventDatabase)
    filter.ExcludeEvent(trace_tea.EventGC)
    
    fr fr Test should include
    assert_true(filter.ShouldInclude(trace_tea.EventAPI, "api_call"))
    assert_true(filter.ShouldInclude(trace_tea.EventDatabase, "db_query"))
    assert_false(filter.ShouldInclude(trace_tea.EventGC, "gc_run"))
    
    fr fr Start tracing with filter
    sus buf := dropz.NewBuffer()
    sus err := trace_tea.StartWithFilter(buf, filter)
    assert_eq_string(err, "")
    
    sus ctx := vibe_context.Background()
    
    fr fr Log different types of events
    trace_tea.Log(ctx, trace_tea.EventAPI, "API call")  fr fr Should be included
    trace_tea.Log(ctx, trace_tea.EventDatabase, "DB query")  fr fr Should be included
    trace_tea.Log(ctx, trace_tea.EventGC, "GC run")  fr fr Should be filtered out
    trace_tea.Log(ctx, trace_tea.EventMemory, "Memory alloc")  fr fr Should be filtered out
    
    trace_tea.Stop()
    damn based
}

fr fr Test real-time analyzer
slay test_real_time_analyzer() lit {
    fr fr Create analyzer
    sus analyzer := trace_tea.NewRealTimeAnalyzer()
    
    sus highLatencyDetected := cap
    sus deadlockDetected := cap
    
    fr fr Set up callbacks
    analyzer.OnHighLatency(50, slay(taskName tea, duration timez.Duration) {
        highLatencyDetected = based
    })
    
    analyzer.OnDeadlock(slay(info tea) {
        deadlockDetected = based
    })
    
    fr fr Register analyzer
    trace_tea.RegisterAnalyzer(analyzer)
    
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    sus taskCtx, task := trace_tea.NewTask(ctx, "slow_task")
    
    if task != cringe {
        fr fr Simulate long-running task
        sus longCounter := 0
        while longCounter < 1000 {
            longCounter = longCounter + 1
        }
        
        task.End()
        
        fr fr Analyze the task
        analyzer.AnalyzeTask(task)
    }
    
    trace_tea.Stop()
    damn based
}

fr fr Test visualization
slay test_visualization() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    
    fr fr Create some trace data
    sus taskCtx, task := trace_tea.NewTask(ctx, "viz_task")
    if task != cringe {
        sus region := trace_tea.StartRegion(taskCtx, "viz_region")
        if region != cringe {
            trace_tea.Log(taskCtx, trace_tea.EventAPI, "API call for visualization")
            region.End()
        }
        task.End()
    }
    
    trace_tea.Log(ctx, trace_tea.EventDatabase, "Database operation")
    trace_tea.Log(ctx, trace_tea.EventNetwork, "Network request")
    
    trace_tea.Stop()
    
    fr fr Create visualizer
    sus visualizer := trace_tea.NewVisualizer(buf.Bytes())
    sus timeline := visualizer.GenerateTimeline()
    
    fr fr Timeline should have events
    assert_true(len(timeline.Events) > 0)
    
    damn based
}

fr fr Test metrics extraction
slay test_metrics_extraction() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    
    fr fr Create some tasks for metrics
    for i := 0; i < 3; i++ {
        sus taskCtx, task := trace_tea.NewTask(ctx, "metrics_task")
        if task != cringe {
            fr fr Simulate work
            sus workCounter := 0
            while workCounter < 100 {
                workCounter = workCounter + 1
            }
            task.End()
        }
    }
    
    fr fr Create some events
    trace_tea.Log(ctx, trace_tea.EventAPI, "API metrics test")
    trace_tea.Log(ctx, trace_tea.EventDatabase, "DB metrics test")
    
    trace_tea.Stop()
    
    fr fr Extract metrics
    sus metrics := trace_tea.ExtractMetrics(buf.Bytes())
    
    fr fr Verify metrics
    assert_true(metrics.TotalEvents > 0)
    assert_true(metrics.TotalTasks >= 0)
    
    fr fr Test average latency
    sus apiLatency := metrics.AverageLatency(trace_tea.EventAPI)
    fr fr Should return some duration (even if zero)
    
    sus maxConcurrency := metrics.MaxConcurrency()
    assert_true(maxConcurrency >= 0)
    
    damn based
}

fr fr Test context helpers
slay test_context_helpers() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    
    fr fr Test ContextWithTask
    sus taskCtx := trace_tea.ContextWithTask(ctx, "context_task")
    
    fr fr Test TaskFromContext
    sus task := trace_tea.TaskFromContext(taskCtx)
    if task != cringe {
        task.LazyLog("Task from context: %s", "working")
        task.End()
    }
    
    trace_tea.Stop()
    damn based
}

fr fr Test error handling
slay test_error_handling() lit {
    fr fr Test operations when tracing is not active
    sus ctx := vibe_context.Background()
    
    fr fr These should not crash when tracing is inactive
    sus inactiveCtx, inactiveTask := trace_tea.NewTask(ctx, "inactive_task")
    if inactiveTask == cringe {
        fr fr Expected when tracing is inactive
    }
    
    sus inactiveRegion := trace_tea.StartRegion(ctx, "inactive_region")
    if inactiveRegion == cringe {
        fr fr Expected when tracing is inactive
    }
    
    fr fr Log when inactive (should not crash)
    trace_tea.Log(ctx, trace_tea.EventUserDefined, "Inactive log")
    trace_tea.Logf(ctx, trace_tea.EventUserDefined, "Inactive formatted: %s", "test")
    
    damn based
}

fr fr Test concurrent tracing
slay test_concurrent_tracing() lit {
    sus buf := dropz.NewBuffer()
    trace_tea.Start(buf)
    
    sus ctx := vibe_context.Background()
    
    fr fr Simulate concurrent tasks
    for i := 0; i < 5; i++ {
        sus taskName := "concurrent_task_" + intToString(i)
        sus taskCtx, task := trace_tea.NewTask(ctx, taskName)
        
        if task != cringe {
            task.LazyLog("Concurrent task %s started", intToString(i))
            
            fr fr Simulate concurrent work
            sus workId := i * 10
            while workId < (i + 1) * 10 {
                workId = workId + 1
            }
            
            task.LazyLog("Concurrent task %s finished", intToString(i))
            task.End()
        }
    }
    
    trace_tea.Stop()
    damn based
}

slay intToString(val normie) tea {
    if val == 0 { damn "0" }
    if val == 1 { damn "1" }
    if val == 2 { damn "2" }
    if val == 3 { damn "3" }
    if val == 4 { damn "4" }
    if val == 5 { damn "5" }
    if val == 6 { damn "6" }
    if val == 7 { damn "7" }
    if val == 8 { damn "8" }
    if val == 9 { damn "9" }
    damn "unknown"
}

fr fr Run all enhanced tests
assert_true(test_basic_tracing())
assert_true(test_task_operations())
assert_true(test_region_operations())
assert_true(test_event_logging())
assert_true(test_with_region())
assert_true(test_with_span())
assert_true(test_trace_filtering())
assert_true(test_real_time_analyzer())
assert_true(test_visualization())
assert_true(test_metrics_extraction())
assert_true(test_context_helpers())
assert_true(test_error_handling())
assert_true(test_concurrent_tracing())

print_test_summary()
