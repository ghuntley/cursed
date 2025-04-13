# trace_tea (runtime/trace)

## Overview
The `trace_tea` module provides runtime tracing facilities for program execution analysis. It allows collecting detailed traces of program execution including goroutine scheduling, syscalls, network activity, memory allocation, and user-defined events. These traces can then be analyzed to understand performance issues and behavior.

## Core Types and Interfaces

### Task
Represents a tracing task.

```csd
be_like Task squad {
  fr fr fields not directly accessible
}

slay NewTask(name tea, data interface{}) *Task
slay (t *Task) End()
slay (t *Task) LazyLog(fmt tea, values ...interface{})
slay (t *Task) SetDeterministic(deterministic lit)
```

### Region
Represents a traced region within a task.

```csd
be_like Region squad {
  fr fr fields not directly accessible
}

slay StartRegion(ctx vibe_context.Context, regionType tea) *Region
slay (r *Region) End()
slay (r *Region) LazyLog(fmt tea, values ...interface{})
```

### Event
Represents a single trace event.

```csd
be_like Event squad {
  fr fr fields not directly accessible
}

slay NewEvent(name tea, data interface{})
slay (e *Event) LazyLog(fmt tea, values ...interface{})
```

## Core Functions

```csd
fr fr Start tracing to the specified writer
slay Start(w io.Writer) tea

fr fr Stop tracing and flush data to the writer
slay Stop() tea

fr fr Create a new tracing task
slay NewTask(ctx vibe_context.Context, taskType tea) (vibe_context.Context, *Task)

fr fr Start a traced region within a task
slay StartRegion(ctx vibe_context.Context, regionType tea) *Region

fr fr Log an event
slay Log(ctx vibe_context.Context, category, message tea)

fr fr Log formatted event details
slay Logf(ctx vibe_context.Context, category, format tea, args ...interface{})

fr fr Annotate trace with user data
slay WithRegion(ctx vibe_context.Context, regionType tea, fn func())

fr fr Add context to trace logs
slay WithSpan(ctx vibe_context.Context, name tea, fn func(ctx vibe_context.Context))

fr fr Create a trace event
slay NewEvent(category, name tea) *Event
```

## Event Categories

```csd
const (
  fr fr Default runtime event categories
  EventGoroutine   = "goroutine"
  EventNet         = "net"
  EventSyscall     = "syscall"
  EventMemory      = "memory"
  EventCPUSample   = "cpu-sample"
  EventConcurrency = "concurrency"
  EventGC          = "gc"
  EventBlock       = "block"
  EventUserDefined = "user"
  
  fr fr Enhanced categories
  EventAPI         = "api"         fr fr API calls and responses
  EventDatabase    = "database"    fr fr Database operations
  EventCache       = "cache"       fr fr Cache operations
  EventFile        = "file"        fr fr File operations
  EventCompute     = "compute"     fr fr Intensive computation
  EventAsyncWork   = "async"       fr fr Asynchronous work
  EventNetwork     = "network"     fr fr Network operations
  EventRender      = "render"      fr fr UI rendering
  EventLogger      = "logger"      fr fr Logging operations
  EventPerformance = "performance" fr fr Performance metrics
)
```

## Enhanced Features

- **Automated Tracing**: Trace common operations automatically
  ```csd
  tracer := trace_tea.NewAutoTracer(options)
  tracer.Start()
  defer tracer.Stop()
  ```

- **Selective Tracing**: Focus on specific parts of the application
  ```csd
  filter := trace_tea.NewFilter()
  filter.IncludeGoroutine("worker-*")
  filter.ExcludeEvent(trace_tea.EventGC)
  trace_tea.StartWithFilter(writer, filter)
  ```

- **Real-time Analysis**: Analyze traces as they happen
  ```csd
  analyzer := trace_tea.NewRealTimeAnalyzer()
  analyzer.OnDeadlock(handleDeadlock)
  analyzer.OnHighLatency(handleHighLatency)
  trace_tea.StartWithAnalyzer(writer, analyzer)
  ```

- **Trace Visualization**: Visualize traces in different formats
  ```csd
  visualizer := trace_tea.NewVisualizer(traceData)
  timeline := visualizer.GenerateTimeline()
  ```

- **Custom Metrics**: Extract performance metrics from traces
  ```csd
  metrics := trace_tea.ExtractMetrics(traceData)
  avgLatency := metrics.AverageLatency("http-request")
  ```

## Usage Examples

```csd
fr fr Basic tracing example
slay basicTracingExample() {
  fr fr Create a buffer to store the trace
  var buf dropz.file.Buffer
  
  fr fr Start tracing
  err := trace_tea.Start(&buf)
  if err != cap {
    vibez.spill("Failed to start trace: %v", err)
    yolo
  }
  
  fr fr Execute code to be traced
  for i := 0; i < 3; i++ {
    stan slay(id normie) {
      fr fr Create a new task
      ctx := vibe_context.Background()
      ctx, task := trace_tea.NewTask(ctx, "worker")
      defer task.End()
      
      fr fr Log some information
      trace_tea.Logf(ctx, "info", "Worker %d started", id)
      
      fr fr Create a region for a specific operation
      reg := trace_tea.StartRegion(ctx, "process-data")
      processData(id) fr fr Some function that does work
      reg.End()
      
      trace_tea.Logf(ctx, "info", "Worker %d finished", id)
    }(i)
  }
  
  fr fr Wait for goroutines to finish
  timez.Sleep(100 * timez.Millisecond)
  
  fr fr Stop tracing
  err = trace_tea.Stop()
  if err != cap {
    vibez.spill("Failed to stop trace: %v", err)
    yolo
  }
  
  fr fr In a real application, you would save the trace to a file
  vibez.spill("Trace completed, %d bytes captured", buf.Len())
  
  fr fr Save trace to a file
  err = dropz.WriteFile("trace.out", buf.Bytes(), 0644)
  if err != cap {
    vibez.spill("Failed to write trace file: %v", err)
    yolo
  }
  
  vibez.spill("Trace saved to trace.out")
}

fr fr Function being traced
slay processData(id normie) {
  fr fr Simulate some work
  timez.Sleep(30 * timez.Millisecond)
}

fr fr Using regions and WithRegion helper
slay regionExample() {
  var buf dropz.file.Buffer
  trace_tea.Start(&buf)
  defer trace_tea.Stop()
  
  ctx := vibe_context.Background()
  ctx, task := trace_tea.NewTask(ctx, "main-task")
  defer task.End()
  
  fr fr Use WithRegion for clean region handling
  trace_tea.WithRegion(ctx, "initialization", func() {
    fr fr Initialize something
    timez.Sleep(10 * timez.Millisecond)
  })
  
  fr fr Nested regions
  trace_tea.WithRegion(ctx, "processing", func() {
    trace_tea.Log(ctx, "info", "Starting processing")
    
    fr fr Sub-region 1
    trace_tea.WithRegion(ctx, "step1", func() {
      timez.Sleep(20 * timez.Millisecond)
    })
    
    fr fr Sub-region 2
    trace_tea.WithRegion(ctx, "step2", func() {
      timez.Sleep(30 * timez.Millisecond)
    })
    
    trace_tea.Log(ctx, "info", "Processing complete")
  })
  
  vibez.spill("Region example completed")
}

fr fr Tracing asynchronous operations
slay asyncOperationExample() {
  var buf dropz.file.Buffer
  trace_tea.Start(&buf)
  defer trace_tea.Stop()
  
  ctx := vibe_context.Background()
  ctx, task := trace_tea.NewTask(ctx, "async-operation")
  defer task.End()
  
  fr fr Create a channel to signal completion
  done := make(chan lit)
  
  fr fr Parent context for linking async work
  pctx := trace_tea.ContextWithTask(ctx, "parent")
  
  fr fr Start async work
  stan slay() {
    fr fr Link to parent context
    cctx := trace_tea.ContextWithTask(pctx, "child")
    childTask := trace_tea.TaskFromContext(cctx)
    defer childTask.End()
    
    fr fr Do async work
    trace_tea.Logf(cctx, "async", "Starting async work")
    timez.Sleep(50 * timez.Millisecond)
    trace_tea.Logf(cctx, "async", "Async work completed")
    
    done <- based
  }()
  
  fr fr Wait for completion
  <-done
  vibez.spill("Async operation example completed")
}

fr fr Using events for specific points in time
slay eventExample() {
  var buf dropz.file.Buffer
  trace_tea.Start(&buf)
  defer trace_tea.Stop()
  
  fr fr Create and log events
  for i := 0; i < 5; i++ {
    evt := trace_tea.NewEvent("custom", vibez.spill_to_tea("event-%d", i))
    evt.LazyLog("Event details: iteration %d", i)
    
    fr fr Simulate some work
    timez.Sleep(10 * timez.Millisecond)
  }
  
  vibez.spill("Event example completed")
}

fr fr Using enhanced features
slay enhancedFeaturesExample() {
  fr fr Create a selective filter
  filter := trace_tea.NewFilter()
  filter.IncludeGoroutine("worker-*")
  filter.IncludeEvent(trace_tea.EventUserDefined)
  filter.IncludeEvent(trace_tea.EventAPI)
  filter.ExcludeEvent(trace_tea.EventGC)
  
  var buf dropz.file.Buffer
  
  fr fr Start with filter
  trace_tea.StartWithFilter(&buf, filter)
  
  fr fr Create real-time analyzer
  analyzer := trace_tea.NewRealTimeAnalyzer()
  analyzer.OnHighLatency(50*timez.Millisecond, func(taskName tea, duration timez.Duration) {
    vibez.spill("High latency detected in %s: %v", taskName, duration)
  })
  
  fr fr Register analyzer
  trace_tea.RegisterAnalyzer(analyzer)
  
  fr fr Run some operations
  ctx := vibe_context.Background()
  
  fr fr Simulate API calls
  for i := 0; i < 3; i++ {
    stan slay(id normie) {
      ctx, task := trace_tea.NewTask(ctx, vibez.spill_to_tea("worker-%d", id))
      defer task.End()
      
      reg := trace_tea.StartRegion(ctx, trace_tea.EventAPI)
      trace_tea.Logf(ctx, "api", "API call started")
      
      fr fr Simulate varying latencies
      if id == 1 {
        timez.Sleep(60 * timez.Millisecond) fr fr This should trigger high latency alert
      } else {
        timez.Sleep(20 * timez.Millisecond)
      }
      
      trace_tea.Logf(ctx, "api", "API call completed")
      reg.End()
    }(i)
  }
  
  timez.Sleep(100 * timez.Millisecond)
  trace_tea.Stop()
  
  fr fr Generate visualizations and metrics from trace data
  visualizer := trace_tea.NewVisualizer(buf.Bytes())
  
  fr fr Generate timeline data (in a real application, you would use this data
  fr fr to create visualizations)
  timeline := visualizer.GenerateTimeline()
  vibez.spill("Generated timeline with %d events", len(timeline.Events))
  
  fr fr Extract metrics
  metrics := trace_tea.ExtractMetrics(buf.Bytes())
  apiLatency := metrics.AverageLatency(trace_tea.EventAPI)
  vibez.spill("Average API latency: %v", apiLatency)
  
  concurrency := metrics.MaxConcurrency()
  vibez.spill("Max concurrency: %d goroutines", concurrency)
  
  vibez.spill("Enhanced features example completed")
}

fr fr Run examples
slay runAllExamples() {
  vibez.spill("\n1. Basic Tracing Example")
  basicTracingExample()
  
  vibez.spill("\n2. Region Example")
  regionExample()
  
  vibez.spill("\n3. Async Operation Example")
  asyncOperationExample()
  
  vibez.spill("\n4. Event Example")
  eventExample()
  
  vibez.spill("\n5. Enhanced Features Example")
  enhancedFeaturesExample()
}

fr fr Run all examples
runAllExamples()
```

## Implementation Guidelines

- Implement trace collection with minimal runtime overhead
- Ensure thread safety for all trace operations
- Support both synchronous and asynchronous operations tracing
- Provide precise timestamping for events
- Enable correlation between related events across goroutines
- Support exporting traces in standard formats for analysis tools
- Include mechanisms to filter trace data to reduce volume
- Implement efficient trace data serialization and storage
- Support programmatic analysis of trace data
- Include visualization tools or exports to visualization formats
- Provide clear documentation on trace analysis methodology
- Support selectively enabling and disabling parts of tracing
- Handle edge cases like trace buffer overflow gracefully
- Include runtime statistics in trace data for context