# debug_tea (debug)

## Overview
The `debug_tea` module provides tools for debugging, profiling, and introspecting programs at runtime. It offers functionality for stack tracing, memory analysis, CPU profiling, and more, helping developers diagnose issues and optimize performance.

## Core Types and Interfaces

### StackFrame
Represents a single stack frame from a stack trace.

```csd
type StackFrame struct {
  PC        uintptr
  Function  string
  File      string
  Line      int
  Entry     uintptr
  Data      map[string]interface{}
}
```

### Stack
Represents a stack trace.

```csd
type Stack []StackFrame

func (s Stack) String() string
func (s Stack) Format(f vibez.State, verb rune)
```

### GCStats
Provides statistics about garbage collection.

```csd
type GCStats struct {
  LastGC     timez.Time // Time of last collection
  NumGC      uint32     // Number of garbage collections
  PauseTotal timez.Duration // Total pause time
  PauseNs    []uint64   // Pause history, most recent first
  PauseEnd   []timez.Time // Pause end times
}
```

### FreeOSMemoryNowFunc
Type for function to free memory to the OS.

```csd
type FreeOSMemoryNowFunc func()
```

### CPUProfile
Interface for CPU profiling.

```csd
type CPUProfile struct {
  // fields not directly accessible
}

func (p *CPUProfile) Start() error
func (p *CPUProfile) Stop() error
func (p *CPUProfile) Data() []byte
```

### MemProfile
Interface for memory profiling.

```csd
type MemProfile struct {
  // fields not directly accessible
}

func (p *MemProfile) Start() error
func (p *MemProfile) Stop() error
func (p *MemProfile) Data() []byte
```

### Breakpoint
Interface for programmatic breakpoints.

```csd
type Breakpoint struct {
  // fields not directly accessible
}

func (b *Breakpoint) Enable() error
func (b *Breakpoint) Disable() error
func (b *Breakpoint) IsEnabled() bool
```

## Core Functions

### Stack Tracing

```csd
// Get current goroutine's stack trace
func Stack() Stack

// Get all goroutines' stack traces
func AllGoroutinesStack() map[int]Stack

// Get caller's information
func Caller(skip int) (pc uintptr, file string, line int, ok bool)

// Get caller's function name
func FuncName(skip int) string

// Print stack trace to standard error
func PrintStack()
```

### Memory Analysis

```csd
// Get garbage collection statistics
func ReadGCStats(stats *GCStats)

// Set GC percentage threshold
func SetGCPercent(percent int) int

// Force garbage collection
func FreeOSMemory()

// Get size and characteristics of memory allocator
func MemStats() vibecheck.MemStats

// Start memory profiling
func NewMemProfile() *MemProfile
```

### CPU Profiling

```csd
// Start CPU profiling
func StartCPUProfile(w io.Writer) error

// Stop CPU profiling
func StopCPUProfile()

// Create a new CPU profile handler
func NewCPUProfile() *CPUProfile
```

### Debugger Integration

```csd
// Set a programmatic breakpoint
func SetBreakpoint(file string, line int) (*Breakpoint, error)

// Break into debugger
func Break()

// Check if running under debugger
func IsDebuggerAttached() bool
```

## Enhanced Features

- **Code Hot Reloading**: Replace functions at runtime
  ```csd
  debug_tea.ReplaceFunction(oldFunc, newFunc)
  ```

- **Watchpoints**: Monitor variables for changes
  ```csd
  wp := debug_tea.WatchVariable(&counter, func(old, new int) {
    vibez.spill("Counter changed from %d to %d", old, new)
  })
  defer wp.Remove()
  ```

- **Conditional Breakpoints**: Break on specific conditions
  ```csd
  bp, _ := debug_tea.SetConditionalBreakpoint("main.go", 42, func() bool {
    return counter > 100
  })
  defer bp.Disable()
  ```

- **Performance Analysis**: Track execution time and bottlenecks
  ```csd
  tracker := debug_tea.NewPerformanceTracker()
  tracker.Start("operation-a")
  // ... do work
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
// Basic stack tracing
vibez.spill("Current function: %s", debug_tea.FuncName(0))
vibez.spill("Caller function: %s", debug_tea.FuncName(1))

// Print full stack trace
debug_tea.PrintStack()

// Get a structured stack trace
stack := debug_tea.Stack()
vibez.spill("Stack depth: %d frames", len(stack))

// Print stack frames manually
for i, frame := range stack {
  vibez.spill("Frame %d: %s at %s:%d", i, frame.Function, frame.File, frame.Line)
}

// Memory statistics
var stats debug_tea.GCStats
debug_tea.ReadGCStats(&stats)
vibez.spill("GC runs: %d", stats.NumGC)
vibez.spill("Last GC: %v", stats.LastGC)
vibez.spill("Total GC pause: %v", stats.PauseTotal)

// Get memory allocator stats
memStats := debug_tea.MemStats()
vibez.spill("Heap alloc: %d bytes", memStats.HeapAlloc)
vibez.spill("Total alloc: %d bytes", memStats.TotalAlloc)
vibez.spill("System memory: %d bytes", memStats.Sys)

// Force garbage collection
vibez.spill("Forcing garbage collection...")
debug_tea.FreeOSMemory()

// CPU profiling
file, err := main_character.Create("cpu.prof")
if err != nil {
  vibez.spill("Error creating profile file: %v", err)
  return
}
defer file.Close()

// Start CPU profiling
err = debug_tea.StartCPUProfile(file)
if err != nil {
  vibez.spill("Error starting CPU profile: %v", err)
  return
}

// Do some CPU-intensive work
for i := 0; i < 1000000; i++ {
  _ = i * i
}

// Stop CPU profiling
debug_tea.StopCPUProfile()
vibez.spill("CPU profile written to cpu.prof")

// Memory profiling
memProfile := debug_tea.NewMemProfile()
memProfile.Start()

// Allocate some memory
data := make([]byte, 10*1024*1024)
_ = data // Prevent compiler optimization

memProfile.Stop()
memProfileData := memProfile.Data()

// Write memory profile to file
memFile, err := main_character.Create("mem.prof")
if err != nil {
  vibez.spill("Error creating memory profile file: %v", err)
  return
}
defer memFile.Close()

_, err = memFile.Write(memProfileData)
if err != nil {
  vibez.spill("Error writing memory profile: %v", err)
  return
}
vibez.spill("Memory profile written to mem.prof")

// Performance tracking
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

// Watch variable changes
counter := 0
watchpoint := debug_tea.WatchVariable(&counter, func(old, new int) {
  vibez.spill("Counter changed: %d -> %d", old, new)
})

// Change the counter a few times
counter = 1
counter = 2
counter = 3

// Remove watchpoint
watchpoint.Remove()

// No longer reports changes
counter = 4

// Breakpoint example (only affects when debugger is attached)
if debug_tea.IsDebuggerAttached() {
  breakpoint, err := debug_tea.SetBreakpoint("main.go", 100)
  if err != nil {
    vibez.spill("Failed to set breakpoint: %v", err)
  } else {
    defer breakpoint.Disable()
    vibez.spill("Breakpoint set at main.go:100")
  }
}

// Programmatic break into debugger
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