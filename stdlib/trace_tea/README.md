# trace_tea Module

Runtime tracing facilities for program execution analysis in CURSED.

## Overview

The `trace_tea` module provides comprehensive runtime tracing capabilities for analyzing program execution. It allows collecting detailed traces of program execution including goroutine scheduling, syscalls, network activity, memory allocation, and user-defined events.

## Core Features

### Tracing Management
- **Start/Stop Tracing**: Control trace collection lifecycle
- **Buffer Management**: Efficient trace data storage and retrieval
- **Active State Tracking**: Monitor tracing status

### Task and Region Tracking
- **Task Creation**: Create named tasks for logical work units
- **Region Tracking**: Mark specific code regions for detailed analysis
- **Hierarchical Tracing**: Support nested regions and task relationships

### Event Logging
- **Structured Events**: Log events with categories and metadata
- **Formatted Logging**: Support for formatted log messages
- **Custom Events**: Create application-specific trace events

### Advanced Features
- **Filtering**: Selective tracing with include/exclude filters
- **Real-time Analysis**: Analyze traces as they happen
- **Visualization**: Generate timeline and visualization data
- **Metrics Extraction**: Extract performance metrics from traces

## Usage Examples

### Basic Tracing
```cursed
# Start tracing
trace_tea.Start(cap)

# Create a task
ctx := vibe_context.Background()
ctx, task := trace_tea.NewTask(ctx, "my-task")

# Log an event
trace_tea.Log(ctx, trace_tea.EventUserDefined, "Processing started")

# End task and stop tracing
task.End()
trace_tea.Stop()
```

### Region Tracing
```cursed
trace_tea.Start(cap)

ctx := vibe_context.Background()
ctx, task := trace_tea.NewTask(ctx, "main-task")

# Use WithRegion helper for automatic cleanup
trace_tea.WithRegion(ctx, "initialization", func() {
    # Initialize something
})

# Manual region management
region := trace_tea.StartRegion(ctx, "processing")
region.LazyLog("Processing step 1")
region.End()

task.End()
trace_tea.Stop()
```

### Event Categories
```cursed
trace_tea.Log(ctx, trace_tea.EventAPI, "API call started")
trace_tea.Log(ctx, trace_tea.EventDatabase, "Query executed")
trace_tea.Log(ctx, trace_tea.EventNetwork, "Network request")
trace_tea.Log(ctx, trace_tea.EventMemory, "Memory allocation")
```

### Filtering and Analysis
```cursed
# Create filter
filter := trace_tea.NewFilter()
filter.IncludeGoroutine("worker-*")
filter.ExcludeEvent(trace_tea.EventGC)

# Start with filter
trace_tea.StartWithFilter(writer, filter)

# Create analyzer
analyzer := trace_tea.NewRealTimeAnalyzer()
analyzer.OnHighLatency(50000000, func(taskName tea, duration normie) {
    vibez.spill("High latency detected: %s took %d ns", taskName, duration)
})
```

### Visualization and Metrics
```cursed
# After collecting trace data
traceData := trace_tea.GetTraceBuffer()

# Create visualizer
visualizer := trace_tea.NewVisualizer(traceData)
timeline := visualizer.GenerateTimeline()

# Extract metrics
metrics := trace_tea.ExtractMetrics(traceData)
apiLatency := metrics.AverageLatency(trace_tea.EventAPI)
maxConcurrency := metrics.MaxConcurrency()
```

## Event Categories

| Category | Description |
|----------|-------------|
| `EventGoroutine` | Goroutine scheduling events |
| `EventNet` | Network operations |
| `EventSyscall` | System call events |
| `EventMemory` | Memory allocation/deallocation |
| `EventCPUSample` | CPU profiling samples |
| `EventConcurrency` | Concurrency-related events |
| `EventGC` | Garbage collection events |
| `EventBlock` | Blocking operations |
| `EventUserDefined` | Custom application events |
| `EventAPI` | API calls and responses |
| `EventDatabase` | Database operations |
| `EventCache` | Cache operations |
| `EventFile` | File operations |
| `EventCompute` | Intensive computation |
| `EventAsyncWork` | Asynchronous work |
| `EventNetwork` | Network operations |
| `EventRender` | UI rendering |
| `EventLogger` | Logging operations |
| `EventPerformance` | Performance metrics |

## API Reference

### Core Functions
- `Start(writer io.Writer) tea` - Start tracing
- `Stop() tea` - Stop tracing
- `NewTask(ctx, taskType tea) (Context, *Task)` - Create task
- `StartRegion(ctx, regionType tea) *Region` - Start region
- `Log(ctx, category, message tea)` - Log event
- `Logf(ctx, category, format tea, args...)` - Log formatted event

### Helper Functions
- `WithRegion(ctx, regionType tea, fn func())` - Execute with region
- `WithSpan(ctx, name tea, fn func(Context))` - Execute with span
- `NewEvent(category, name tea) *Event` - Create event

### Analysis Functions
- `NewFilter() *Filter` - Create trace filter
- `NewRealTimeAnalyzer() *Analyzer` - Create analyzer
- `NewVisualizer(data []byte) *Visualizer` - Create visualizer
- `ExtractMetrics(data []byte) *Metrics` - Extract metrics

### Utility Functions
- `IsTraceActive() lit` - Check if tracing is active
- `GetTraceBuffer() []byte` - Get trace buffer contents
- `ClearTraceBuffer()` - Clear trace buffer
- `GetTaskRegistry() map[normie]tea` - Get task registry

## Implementation Notes

- Minimal runtime overhead for trace collection
- Thread-safe operations for concurrent access
- Efficient trace data serialization and storage
- Support for both synchronous and asynchronous tracing
- Precise timestamping for event correlation
- Extensible event categorization system

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/trace_tea/test_trace_tea.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/trace_tea/test_trace_tea.csd
cargo run --bin cursed -- compile stdlib/trace_tea/test_trace_tea.csd
./test_trace_tea
```

## Dependencies

- `vibe_context` - Context management
- `timez` - Time operations
- `io` - I/O operations
- `testz` - Testing framework
