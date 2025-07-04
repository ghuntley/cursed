# debug_tea (debug)

## Overview
The `debug_tea` module provides tools for debugging, profiling, and introspecting programs at runtime. It offers functionality for stack tracing, memory analysis, CPU profiling, and more, helping developers diagnose issues and optimize performance.

## Core Types and Interfaces

### StackFrame
Represents a single stack frame from a stack trace.

```csd
be_like StackFrame squad {
  PC        uintptr
  Function  tea
  File      tea
  Line      int
  Entry     uintptr
  Data      map[tea]interface{}
}
```

### Stack
Represents a stack trace.

```csd
be_like Stack []StackFrame

slay (s Stack) String() tea
slay (s Stack) Format(f vibez.State, verb rune)
```

### GCStats
Provides statistics about garbage collection.

```csd
be_like GCStats squad {
  LastGC     timez.Time fr fr Time of last collection
  NumGC      uint32     fr fr Number of garbage collections
  PauseTotal timez.Duration fr fr Total pause time
  PauseNs    []uint64   fr fr Pause history, most recent first
  PauseEnd   []timez.Time fr fr Pause end times
}
```

### FreeOSMemoryNowFunc
Type for function to free memory to the OS.

```csd
be_like FreeOSMemoryNowFunc func()
```

### CPUProfile
Interface for CPU profiling.

```csd
be_like CPUProfile squad {
  fr fr fields not directly accessible
}

slay (p *CPUProfile) Start() tea
slay (p *CPUProfile) Stop() tea
slay (p *CPUProfile) Data() []byte
```

### MemProfile
Interface for memory profiling.

```csd
be_like MemProfile squad {
  fr fr fields not directly accessible
}

slay (p *MemProfile) Start() tea
slay (p *MemProfile) Stop() tea
slay (p *MemProfile) Data() []byte
```

### Breakpoint
Interface for programmatic breakpoints.

```csd
be_like Breakponormie squad {
  fr fr fields not directly accessible
}

slay (b *Breakponormie) Enable() tea
slay (b *Breakponormie) Disable() tea
slay (b *Breakponormie) IsEnabled() lit
```

## Core Functions

### Stack Tracing

```csd
fr fr Get current goroutine's stack trace
slay Stack() Stack

fr fr Get all goroutines' stack traces
slay AllGoroutinesStack() map[int]Stack

fr fr Get caller's information
slay Caller(skip normie) (pc uintptr, file tea, line int, ok lit)

fr fr Get caller's function name
slay FuncName(skip normie) tea

fr fr Print stack trace to standard tea
slay PrintStack()
```

### Memory Analysis

```csd
fr fr Get garbage collection statistics
slay ReadGCStats(stats *GCStats)

fr fr Set GC percentage threshold
slay SetGCPercent(percent normie) int

fr fr Force garbage collection
slay FreeOSMemory()

fr fr Get size and characteristics of memory allocator
slay MemStats() vibecheck.MemStats

fr fr Start memory profiling
slay NewMemProfile() *MemProfile
```

### CPU Profiling

```csd
fr fr Start CPU profiling
slay StartCPUProfile(w io.Writer) tea

fr fr Stop CPU profiling
slay StopCPUProfile()

fr fr Create a new CPU profile handler
slay NewCPUProfile() *CPUProfile
```

### Debugger Integration

```csd
fr fr Set a programmatic breakpoint
slay SetBreakpoint(file tea, line normie) (*Breakpoint, tea)

fr fr Break into debugger
slay Break()

fr fr Check if running under debugger
slay IsDebuggerAttached() lit
```

## Enhanced Features

- **Code Hot Reloading**: Replace functions at runtime
  ```csd
  debug_tea.ReplaceFunction(oldFunc, newFunc)
  ```

- **Watchpoints**: Monitor variables for changes
  ```csd
  wp := debug_tea.WatchVariable(&counter, func(old, new normie) {
    vibez.spill("Counter changed from %d to %d", old, new)
  })
  defer wp.Remove()
  ```

- **Conditional Breakpoints**: Break on specific conditions
  ```csd
  bp, _ := debug_tea.SetConditionalBreakpoint("main.go", 42, func() lit {
    yolo counter > 100
  })
  defer bp.Disable()
  ```

- **Performance Analysis**: Track execution time and bottlenecks
  ```csd
  tracker := debug_tea.NewPerformanceTracker()
  tracker.Start("operation-a")
  fr fr ... do work
  tracker.Stop("operation-a")
  report := tracker.Report()
  ```

- **Remote Debugging**: Connect to running process
  ```csd
  server := debug_tea.NewRemoteServer(":8080")
  server.Start()
  defer server.Stop()
  ```

## Usage Examples

```csd
fr fr Basic stack tracing
vibez.spill("Current function: %s", debug_tea.FuncName(0))
vibez.spill("Caller function: %s", debug_tea.FuncName(1))

fr fr Print full stack trace
debug_tea.PrintStack()

fr fr Get a squadured stack trace
stack := debug_tea.Stack()
vibez.spill("Stack depth: %d frames", len(stack))

fr fr Print stack frames manually
for i, frame := range stack {
  vibez.spill("Frame %d: %s at %s:%d", i, frame.Function, frame.File, frame.Line)
}

fr fr Memory statistics
var stats debug_tea.GCStats
debug_tea.ReadGCStats(&stats)
vibez.spill("GC runs: %d", stats.NumGC)
vibez.spill("Last GC: %v", stats.LastGC)
vibez.spill("Total GC pause: %v", stats.PauseTotal)

fr fr Get memory allocator stats
memStats := debug_tea.MemStats()
vibez.spill("Heap alloc: %d bytes", memStats.HeapAlloc)
vibez.spill("Total alloc: %d bytes", memStats.TotalAlloc)
vibez.spill("System memory: %d bytes", memStats.Sys)

fr fr Force garbage collection
vibez.spill("Forcing garbage collection...")
debug_tea.FreeOSMemory()

fr fr CPU profiling
file, err := main_character.Create("cpu.prof")
if err != cap {
  vibez.spill("Error creating profile file: %v", err)
  yolo
}
defer file.Close()

fr fr Start CPU profiling
err = debug_tea.StartCPUProfile(file)
if err != cap {
  vibez.spill("Error starting CPU profile: %v", err)
  yolo
}

fr fr Do some CPU-intensive work
for i := 0; i < 1000000; i++ {
  _ = i * i
}

fr fr Stop CPU profiling
debug_tea.StopCPUProfile()
vibez.spill("CPU profile written to cpu.prof")

fr fr Memory profiling
memProfile := debug_tea.NewMemProfile()
memProfile.Start()

fr fr Allocate some memory
data := make([]byte, 10*1024*1024)
_ = data fr fr Prevent compiler optimization

memProfile.Stop()
memProfileData := memProfile.Data()

fr fr Write memory profile to file
memFile, err := main_character.Create("mem.prof")
if err != cap {
  vibez.spill("Error creating memory profile file: %v", err)
  yolo
}
defer memFile.Close()

_, err = memFile.Write(memProfileData)
if err != cap {
  vibez.spill("Error writing memory profile: %v", err)
  yolo
}
vibez.spill("Memory profile written to mem.prof")

fr fr Performance tracking
tracker := debug_tea.NewPerformanceTracker()

for i := 0; i < 3; i++ {
  tracker.Start("operation")
  timez.Sleep(timez.Millisecond * 50)
  tracker.Stop("operation")
}

report := tracker.Report()
vibez.spill("Performance report:")
vibez.spill("  Operation: %s", report["operation"].Name)
vibez.spill("  Calls: %d", report["operation"].Calls)
vibez.spill("  Total time: %v", report["operation"].TotalTime)
vibez.spill("  Average time: %v", report["operation"].AverageTime)
vibez.spill("  Min time: %v", report["operation"].MinTime)
vibez.spill("  Max time: %v", report["operation"].MaxTime)

fr fr Watch variable changes
counter := 0
watchponormie := debug_tea.WatchVariable(&counter, func(old, new normie) {
  vibez.spill("Counter changed: %d -> %d", old, new)
})

fr fr Change the counter a few times
counter = 1
counter = 2
counter = 3

fr fr Remove watchpoint
watchpoint.Remove()

fr fr No longer reports changes
counter = 4

fr fr Breakponormie example (only affects when debugger is attached)
if debug_tea.IsDebuggerAttached() {
  breakpoint, err := debug_tea.SetBreakpoint("main.go", 100)
  if err != cap {
    vibez.spill("Failed to set breakpoint: %v", err)
  } else {
    defer breakpoint.Disable()
    vibez.spill("Breakponormie set at main.go:100")
  }
}

fr fr Programmatic break into debugger
if debug_tea.IsDebuggerAttached() {
  vibez.spill("About to break into debugger...")
  debug_tea.Break()
  vibez.spill("Resumed from debugger")
}
```

## Implementation Guidelines

- Minimize overhead for production use
- Ensure debug information is accurate and useful
- Support multiple debugging approaches (interactive, profiling, tracing)
- Provide clear documentation for profiling data interpretation
- Ensure thread safety for all operations
- Integrate with standard debugging tools and formats
- Support various output formats for profiles (pprof, etc.)
- Allow verbose mode for detailed debugging information
- Provide conditional debugging based on build flags
- Support runtime enabling/disabling of expensive debug features