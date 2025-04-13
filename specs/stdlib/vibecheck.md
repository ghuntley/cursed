# vibecheck (runtime)

## Overview
The `vibecheck` module provides low-level runtime functionality and direct interaction with the Cursed runtime system. It offers capabilities like memory stats, garbage collection control, goroutine management, and runtime configuration.

## Core Types and Interfaces

### MemStats
Provides detailed memory statistics.

```csd
type MemStats struct {
  Alloc      uint64 // bytes allocated and not yet freed
  TotalAlloc uint64 // total bytes allocated (even if freed)
  Sys        uint64 // total memory obtained from system
  Mallocs    uint64 // total number of allocations
  Frees      uint64 // total number of frees
  HeapAlloc  uint64 // bytes allocated and not yet freed (same as Alloc)
  HeapSys    uint64 // bytes obtained from system
  HeapIdle   uint64 // bytes in idle spans
  HeapInuse  uint64 // bytes in non-idle spans
  StackInuse uint64 // bytes used by stack allocator
  StackSys   uint64 // bytes obtained from system for stack allocator
  GCSys      uint64 // bytes used for GC metadata
  NextGC     uint64 // target heap size for next GC
  LastGC     uint64 // time of last GC in nanoseconds since epoch
  PauseTotalNs uint64 // total GC pause time in nanoseconds
  NumGC      uint32 // number of completed GC cycles
  GCCPUFraction float64 // fraction of CPU time used by GC
}
```

### Func
Contains information about a function.

```csd
type Func struct {
  // fields not directly accessible
}

func (f *Func) Name() string
func (f *Func) Entry() uintptr
func (f *Func) FileLine(pc uintptr) (file string, line int)
```

### StackRecord
Represents a single entry in a stack trace.

```csd
type StackRecord struct {
  PC uintptr // program counter
  Func *Func // function information
  File string // file name
  Line int    // line number
}
```

## Core Functions

### Memory Management

```csd
// Get detailed memory statistics
func ReadMemStats(m *MemStats)

// Run garbage collection synchronously
func GC()

// Set garbage collector target percentage
func SetGCPercent(percent int) int

// Free memory to operating system
func FreeOSMemory()
```

### Goroutine Management

```csd
// Current number of goroutines
func NumGoroutine() int

// Get current goroutine ID
func GoID() int64

// Get stack trace for all goroutines
func Stack() []byte

// Get the number of logical CPUs usable by the current process
func NumCPU() int

// Set maximum number of CPUs that can be executing simultaneously
func GOMAXPROCS(n int) int
```

### Runtime Information

```csd
// Get Cursed version information
func Version() string

// Get compiler information
func Compiler() string

// Get GOARCH equivalent
func GOARCH() string

// Get GOOS equivalent
func GOOS() string

// Get start time of the program
func StartTime() int64

// Get caller frame skipping n frames
func Caller(skip int) (pc uintptr, file string, line int, ok bool)
```

## Enhanced Features

- **Precise Memory Profiling**: Detailed memory usage analysis
  ```csd
  profile := vibecheck.MemoryProfile()
  vibecheck.WriteProfile(profile, file)
  ```

- **Goroutine Debugging**: Advanced goroutine inspection and control
  ```csd
  info := vibecheck.GoroutineInfo(goroutineID)
  vibecheck.BlockProfile(true) // Enable blocking profile
  ```

- **Runtime Hooks**: Register callbacks for runtime events
  ```csd
  vibecheck.SetGCNotifier(func() {
    vibez.spill("GC cycle completed")
  })
  ```

- **JIT Compiler Introspection**: Access to JIT compilation details
  ```csd
  stats := vibecheck.JITStats()
  vibecheck.SetJITOptLevel(2) // Set optimization level
  ```

- **Runtime Metrics**: Real-time performance monitoring
  ```csd
  metrics := vibecheck.Metrics()
  ```

- **Resource Control**: Fine-grained control over system resources
  ```csd
  vibecheck.SetCPUProfileRate(100)  // 100 samples per second
  vibecheck.SetMemoryLimit(1 << 30) // 1GB memory limit
  ```

## Usage Examples

```csd
// Memory statistics example
var m vibecheck.MemStats
vibecheck.ReadMemStats(&m)

vibez.spill("Allocated: %d KB", m.Alloc / 1024)
vibez.spill("Total Allocated: %d KB", m.TotalAlloc / 1024)
vibez.spill("System Memory: %d KB", m.Sys / 1024)
vibez.spill("Garbage Collections: %d", m.NumGC)

// Goroutine information
vibez.spill("Number of goroutines: %d", vibecheck.NumGoroutine())
vibez.spill("Max PROCS: %d", vibecheck.GOMAXPROCS(0))
vibez.spill("Number of CPUs: %d", vibecheck.NumCPU())

// Set maximum number of processors
old := vibecheck.GOMAXPROCS(4)
vibez.spill("Old GOMAXPROCS: %d, New: 4", old)

// Trigger garbage collection
vibez.spill("Triggering garbage collection...")
vibecheck.GC()

// Get current goroutine ID
vibez.spill("Current goroutine ID: %d", vibecheck.GoID())

// Get stack trace of all goroutines
stack := vibecheck.Stack()
vibez.spill("\nStack Trace:\n%s", string(stack))

// Get caller information
pc, file, line, ok := vibecheck.Caller(0)
if ok {
  f := vibecheck.FuncForPC(pc)
  vibez.spill("Called from %s (%s:%d)", f.Name(), file, line)
}

// Version information
vibez.spill("Cursed Version: %s", vibecheck.Version())
vibez.spill("Compiler: %s", vibecheck.Compiler())
vibez.spill("Architecture: %s", vibecheck.GOARCH())
vibez.spill("Operating System: %s", vibecheck.GOOS())

// Memory profiling
vibez.spill("\nStarting memory allocation...")
data := make([]byte, 10*1024*1024) // Allocate 10MB
_ = data // Prevent compiler optimization

vibecheck.ReadMemStats(&m)
vibez.spill("After allocation: %d KB", m.Alloc / 1024)

// Set GC percentage
old_percent := vibecheck.SetGCPercent(100) // Set to 100%
vibez.spill("Old GC percent: %d, New: 100", old_percent)

// Performance monitoring
vibecheck.SetCPUProfileRate(100) // Sample 100 times per second
cpu_profile := vibecheck.CPUProfile()
defer cpu_profile.Stop()

// Run a CPU-intensive task
for i := 0; i < 1000000; i++ {
  _ = i * i
}

// Custom GC control
vibecheck.SetFinalizer(&data, func(obj interface{}) {
  vibez.spill("Object being finalized")
})
vibecheck.KeepAlive(&data) // Prevent premature collection
```

## Implementation Guidelines

- Memory statistics should be accurate and comprehensive
- Garbage collection control should be thread-safe
- Goroutine management should have minimal overhead
- Runtime information should be cached when appropriate
- Error handling should be robust for low-level operations
- Performance impact should be minimized for production use
- Memory profiling should be detailed but efficient
- System calls should be abstracted appropriately
- Resource limits should be enforced consistently