# trace_tea (runtime/trace)

## Overview
The `trace_tea` module provides runtime tracing facilities for program execution analysis. It allows collecting detailed traces of program execution including goroutine scheduling, syscalls, network activity, memory allocation, and user-defined events. These traces can then be analyzed to understand performance issues and behavior.

## Core Types and Interfaces

### Task
Represents a tracing task.

```csd
type Task struct {
  // fields not directly accessible
}

func NewTask(name string, data interface{}) *Task
func (t *Task) End()
func (t *Task) LazyLog(fmt string, values ...interface{})
func (t *Task) SetDeterministic(deterministic bool)
```

### Region
Represents a traced region within a task.

```csd
type Region struct {
  // fields not directly accessible
}

func StartRegion(ctx vibe_context.Context, regionType string) *Region
func (r *Region) End()
func (r *Region) LazyLog(fmt string, values ...interface{})
```

### Event
Represents a single trace event.

```csd
type Event struct {
  // fields not directly accessible
}

func NewEvent(name string, data interface{})
func (e *Event) LazyLog(fmt string, values ...interface{})
```

## Core Functions

```csd
// Start tracing to the specified writer
func Start(w io.Writer) error

// Stop tracing and flush data to the writer
func Stop() error

// Create a new tracing task
func NewTask(ctx vibe_context.Context, taskType string) (vibe_context.Context, *Task)

// Start a traced region within a task
func StartRegion(ctx vibe_context.Context, regionType string) *Region

// Log an event
func Log(ctx vibe_context.Context, category, message string)

// Log formatted event details
func Logf(ctx vibe_context.Context, category, format string, args ...interface{})

// Annotate trace with user data
func WithRegion(ctx vibe_context.Context, regionType string, fn func())

// Add context to trace logs
func WithSpan(ctx vibe_context.Context, name string, fn func(ctx vibe_context.Context))

// Create a trace event
func NewEvent(category, name string) *Event
```

## Event Categories

```csd
const (
  // Default runtime event categories
  EventGoroutine   = "goroutine"
  EventNet         = "net"
  EventSyscall     = "syscall"
  EventMemory      = "memory"
  EventCPUSample   = "cpu-sample"
  EventConcurrency = "concurrency"
  EventGC          = "gc"
  EventBlock       = "block"
  EventUserDefined = "user"
  
  // Enhanced categories
  EventAPI         = "api"         // API calls and responses
  EventDatabase    = "database"    // Database operations
  EventCache       = "cache"       // Cache operations
  EventFile        = "file"        // File operations
  EventCompute     = "compute"     // Intensive computation
  EventAsyncWork   = "async"       // Asynchronous work
  EventNetwork     = "network"     // Network operations
  EventRender      = "render"      // UI rendering
  EventLogger      = "logger"      // Logging operations
  EventPerformance = "performance" // Performance metrics
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
// Basic tracing example
func basicTracingExample() {
  // Create a buffer to store the trace
  var buf dropz.file.Buffer
  
  // Start tracing
  err := trace_tea.Start(&buf)
  if err != nil {
    vibez.spill("Failed to start trace: %v", err)
    return
  }
  
  // Execute code to be traced
  for i := 0; i < 3; i++ {
    go func(id int) {
      // Create a new task
      ctx := vibe_context.Background()
      ctx, task := trace_tea.NewTask(ctx, "worker")
      defer task.End()
      
      // Log some information
      trace_tea.Logf(ctx, "info", "Worker %d started", id)
      
      // Create a region for a specific operation
      reg := trace_tea.StartRegion(ctx, "process-data")
      processData(id) // Some function that does work
      reg.End()
      
      trace_tea.Logf(ctx, "info", "Worker %d finished", id)
    }(i)
  }
  
  // Wait for goroutines to finish
  timez.Sleep(100 * timez.Millisecond)
  
  // Stop tracing
  err = trace_tea.Stop()
  if err != nil {
    vibez.spill("Failed to stop trace: %v", err)
    return
  }
  
  // In a real application, you would save the trace to a file
  vibez.spill("Trace completed, %d bytes captured", buf.Len())
  
  // Save trace to a file
  err = dropz.WriteFile("trace.out", buf.Bytes(), 0644)
  if err != nil {
    vibez.spill("Failed to write trace file: %v", err)
    return
  }
  
  vibez.spill("Trace saved to trace.out")
}

// Function being traced
func processData(id int) {
  // Simulate some work
  timez.Sleep(30 * timez.Millisecond)
}

// Using regions and WithRegion helper
func regionExample() {
  var buf dropz.file.Buffer
  trace_tea.Start(&buf)
  defer trace_tea.Stop()
  
  ctx := vibe_context.Background()
  ctx, task := trace_tea.NewTask(ctx, "main-task")
  defer task.End()
  
  // Use WithRegion for clean region handling
  trace_tea.WithRegion(ctx, "initialization", func() {
    // Initialize something
    timez.Sleep(10 * timez.Millisecond)
  })
  
  // Nested regions
  trace_tea.WithRegion(ctx, "processing", func() {
    trace_tea.Log(ctx, "info", "Starting processing")
    
    // Sub-region 1
    trace_tea.WithRegion(ctx, "step1", func() {
      timez.Sleep(20 * timez.Millisecond)
    })
    
    // Sub-region 2
    trace_tea.WithRegion(ctx, "step2", func() {
      timez.Sleep(30 * timez.Millisecond)
    })
    
    trace_tea.Log(ctx, "info", "Processing complete")
  })
  
  vibez.spill("Region example completed")
}

// Tracing asynchronous operations
func asyncOperationExample() {
  var buf dropz.file.Buffer
  trace_tea.Start(&buf)
  defer trace_tea.Stop()
  
  ctx := vibe_context.Background()
  ctx, task := trace_tea.NewTask(ctx, "async-operation")
  defer task.End()
  
  // Create a channel to signal completion
  done := make(chan bool)
  
  // Parent context for linking async work
  pctx := trace_tea.ContextWithTask(ctx, "parent")
  
  // Start async work
  go func() {
    // Link to parent context
    cctx := trace_tea.ContextWithTask(pctx, "child")
    childTask := trace_tea.TaskFromContext(cctx)
    defer childTask.End()
    
    // Do async work
    trace_tea.Logf(cctx, "async", "Starting async work")
    timez.Sleep(50 * timez.Millisecond)
    trace_tea.Logf(cctx, "async", "Async work completed")
    
    done <- true
  }()
  
  // Wait for completion
  <-done
  vibez.spill("Async operation example completed")
}

// Using events for specific points in time
func eventExample() {
  var buf dropz.file.Buffer
  trace_tea.Start(&buf)
  defer trace_tea.Stop()
  
  // Create and log events
  for i := 0; i < 5; i++ {
    evt := trace_tea.NewEvent("custom", vibez.spill_to_string("event-%d", i))
    evt.LazyLog("Event details: iteration %d", i)
    
    // Simulate some work
    timez.Sleep(10 * timez.Millisecond)
  }
  
  vibez.spill("Event example completed")
}

// Using enhanced features
func enhancedFeaturesExample() {
  // Create a selective filter
  filter := trace_tea.NewFilter()
  filter.IncludeGoroutine("worker-*")
  filter.IncludeEvent(trace_tea.EventUserDefined)
  filter.IncludeEvent(trace_tea.EventAPI)
  filter.ExcludeEvent(trace_tea.EventGC)
  
  var buf dropz.file.Buffer
  
  // Start with filter
  trace_tea.StartWithFilter(&buf, filter)
  
  // Create real-time analyzer
  analyzer := trace_tea.NewRealTimeAnalyzer()
  analyzer.OnHighLatency(50*timez.Millisecond, func(taskName string, duration timez.Duration) {
    vibez.spill("High latency detected in %s: %v", taskName, duration)
  })
  
  // Register analyzer
  trace_tea.RegisterAnalyzer(analyzer)
  
  // Run some operations
  ctx := vibe_context.Background()
  
  // Simulate API calls
  for i := 0; i < 3; i++ {
    go func(id int) {
      ctx, task := trace_tea.NewTask(ctx, vibez.spill_to_string("worker-%d", id))
      defer task.End()
      
      reg := trace_tea.StartRegion(ctx, trace_tea.EventAPI)
      trace_tea.Logf(ctx, "api", "API call started")
      
      // Simulate varying latencies
      if id == 1 {
        timez.Sleep(60 * timez.Millisecond) // This should trigger high latency alert
      } else {
        timez.Sleep(20 * timez.Millisecond)
      }
      
      trace_tea.Logf(ctx, "api", "API call completed")
      reg.End()
    }(i)
  }
  
  timez.Sleep(100 * timez.Millisecond)
  trace_tea.Stop()
  
  // Generate visualizations and metrics from trace data
  visualizer := trace_tea.NewVisualizer(buf.Bytes())
  
  // Generate timeline data (in a real application, you would use this data
  // to create visualizations)
  timeline := visualizer.GenerateTimeline()
  vibez.spill("Generated timeline with %d events", len(timeline.Events))
  
  // Extract metrics
  metrics := trace_tea.ExtractMetrics(buf.Bytes())
  apiLatency := metrics.AverageLatency(trace_tea.EventAPI)
  vibez.spill("Average API latency: %v", apiLatency)
  
  concurrency := metrics.MaxConcurrency()
  vibez.spill("Max concurrency: %d goroutines", concurrency)
  
  vibez.spill("Enhanced features example completed")
}

// Run examples
func runAllExamples() {
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

// Run all examples
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