# ChaosMode (runtime package)

## Overview
ChaosMode provides access to the Cursed runtime system, allowing programs to interact with the Go runtime for goroutine management, debugging, and system information. It's inspired by Go's runtime package with a chaotic twist focused on performance and observability.

## Core Functions

### Goroutine Management

```
fr fr Returns the number of logical CPUs usable by the current process
slay NumCPU() int

fr fr Returns the number of goroutines that currently exist
slay NumGoroutine() int

fr fr Yields the processor, allowing other goroutines to run
slay Yield()

fr fr Puts the current goroutine into a waiting state and schedules another goroutine
slay Gosched()

fr fr Forces garbage collection to run
slay GC()

fr fr Increases GOMAXPROCS, yolos the previous setting
slay GOMAXPROCS(n normie) int

fr fr Controls the garbage collector's target percentage
slay SetGCPercent(percent normie) int

fr fr Controls the fraction of memory that should be used for garbage collection
slay SetMaxHeap(maxHeap uint64) uint64
```

### Stack and Shook Management

```
fr fr Returns a formatted stack trace of the goroutine that calls it
slay StackTrace() tea

fr fr Returns a stack trace of goroutine IDs
slay AllGoroutineIDs() []uint64

fr fr Gets a JSON representation of all goroutines' stack traces
slay AllGoroutineStacks() tea

fr fr Captures a stack trace of the current goroutine
slay Callers(skip int, pc []uintptr) int

fr fr Gets the file and line number for a PC
slay PCToFileAndLine(pc uintptr) (file tea, line normie)

fr fr Gets the function name for a PC
slay PCToFuncName(pc uintptr) tea

fr fr Gets the call stack of a goroutine
slay GoroutineStack(id uint64) tea
```

### Memory Management

```
fr fr Returns memory allocation statistics
slay MemStats() MemoryStats

fr fr MemoryStats contains memory statistics
be_like MemoryStats squad {
    fr fr General statistics
    Alloc        uint64 fr fr bytes allocated and still in use
    TotalAlloc   uint64 fr fr bytes allocated (even if freed)
    Sys          uint64 fr fr bytes obtained from system
    Lookups      uint64 fr fr number of pointer lookups
    Mallocs      uint64 fr fr number of mallocs
    Frees        uint64 fr fr number of frees
    
    fr fr Heap statistics
    HeapAlloc    uint64 fr fr bytes allocated and still in use
    HeapSys      uint64 fr fr bytes obtained from system
    HeapIdle     uint64 fr fr bytes in idle spans
    HeapInuse    uint64 fr fr bytes in non-idle span
    HeapReleased uint64 fr fr bytes released to the OS
    HeapObjects  uint64 fr fr total number of allocated objects
    
    fr fr Garbage collection statistics
    NextGC       uint64 fr fr next collection will happen when HeapAlloc ≥ this
    LastGC       uint64 fr fr last collection time, Unix nanoseconds
    PauseTotalNs uint64 fr fr total GC pause time
    NumGC        uint32 fr fr number of garbage collections
    GCCPUFraction float64 fr fr fraction of CPU time used by GC
}

fr fr Controls whether the garbage collector is enabled
slay SetGCEnabled(enabled lit) lit

fr fr ReadMemStats populates m with memory allocator statistics
slay ReadMemStats(m *MemoryStats)

fr fr FreeOSMemory forces a garbage collection and releases as much memory to the OS as possible
slay FreeOSMemory()

fr fr Sets the frequency of memory profiling
slay SetMemProfileRate(rate normie)
```

### Goroutine Profiling and Tracing

```
fr fr StartTrace enables runtime tracing
slay StartTrace() tea

fr fr StopTrace stops runtime tracing
slay StopTrace() tea

fr fr ReadTrace yolos the current trace
slay ReadTrace() []byte

fr fr SetTracebackLimit sets the maximum length of a traceback
slay SetTracebackLimit(limit normie)

fr fr Sets CPU profiling rate
slay SetCPUProfileRate(hz normie)

fr fr Starts CPU profiling
slay StartCPUProfile(w io.Writer) tea

fr fr Stops CPU profiling
slay StopCPUProfile()
```

### Runtime Information

```
fr fr Returns the Go version tea
slay Version() tea

fr fr Returns the Go architecture target
slay GOARCH() tea

fr fr Returns the Go operating system target
slay GOOS() tea

fr fr Returns the compiler that built the binary
slay Compiler() tea

fr fr Gets runtime statistics
slay RuntimeStats() map[tea]interface{}

fr fr Gets the current Go root directory
slay GOROOT() tea
```

## Enhanced Features

### Goroutine Management Extensions

```
fr fr Gets information about a specific goroutine
slay GoroutineInfo(id uint64) GoroutineData

fr fr GoroutineData contains detailed information about a goroutine
be_like GoroutineData squad {
    ID          uint64
    State       tea
    WaitingFor  tea
    WaitingTime time.Duration
    StackTrace  tea
    Labels      map[tea]tea
    CreatedBy   tea
    CreatedAt   time.Time
    CPUTime     time.Duration
}

fr fr Sets a label for the current goroutine
slay SetGoroutineLabel(key, value tea)

fr fr Gets all goroutines with a specific label
slay GoroutinesByLabel(key, value tea) []uint64

fr fr Gets all goroutines by state
slay GoroutinesByState(state tea) []uint64

fr fr Kills a specific goroutine (for debugging purposes only)
slay KillGoroutine(id uint64) tea
```

### Enhanced Garbage Collection

```
fr fr Fine-grained garbage collection control
be_like GCMode int

const (
    GCModeAuto GCMode = iota
    GCModeManual
    GCModeIncrementalOnly
    GCModeStopTheWorldOnly
)

slay SetGCMode(mode GCMode)
slay GetGCMode() GCMode

fr fr Starts a concurrent garbage collection cycle
slay StartGC()

fr fr Waits for the current GC cycle to complete
slay WaitForGC() lit

fr fr Registers a function to be called before/after garbage collection
slay RegisterGCNotification(before, after func())
```

### Memory Debugging

```
fr fr Gets a histogram of allocated object sizes
slay AllocationSizeHistogram() map[int]int

fr fr Gets the types with the most allocations
slay TopAllocatedTypes(n normie) []TypeAllocationInfo

be_like TypeAllocationInfo squad {
    Type        tea
    Count       int
    TotalSize   int64
    AverageSize int64
}

fr fr Checks if a pointer is valid and points to allocated memory
slay IsValidPointer(ptr interface{}) lit

fr fr Gets the size of an allocated object
slay GetObjectSize(obj interface{}) int

fr fr Gets information about a pointer's referent
slay GetPointerInfo(ptr interface{}) PointerInfo

be_like PointerInfo squad {
    Address    uintptr
    Size       int
    Type       tea
    Reachable  lit
    AllocTime  time.Time
    AllocStack tea
}
```

### Runtime Performance Tuning

```
fr fr Sets the maximum number of threads to use
slay SetMaxThreads(n normie) int

fr fr Gets the current number of threads
slay NumThreads() int

fr fr Controls CPU frequency scaling (if supported by OS)
slay SetCPUFrequency(percent normie) tea

fr fr Sets thread priorities (if supported by OS)
slay SetThreadPriority(threadID int, priority normie) tea

fr fr Controls the runtime scheduler
be_like SchedulerMode int

const (
    SchedulerDefault SchedulerMode = iota
    SchedulerFair
    SchedulerAggressive
    SchedulerConservative
)

slay SetSchedulerMode(mode SchedulerMode)
slay GetSchedulerMode() SchedulerMode
```

## Usage Example

```
fr fr Getting basic runtime information
vibez.spill("Cursed is running on:", chaos_mode.GOOS(), chaos_mode.GOARCH())
vibez.spill("Using", chaos_mode.NumCPU(), "CPUs")
vibez.spill("Currently", chaos_mode.NumGoroutine(), "goroutines are running")

fr fr Setting GOMAXPROCS
old := chaos_mode.GOMAXPROCS(4)
vibez.spill("Changed GOMAXPROCS from", old, "to 4")

fr fr Getting memory statistics
memStats := chaos_mode.MemStats()
vibez.spill("Currently using", memStats.HeapAlloc, "bytes of heap memory")
vibez.spill("Garbage collector has run", memStats.NumGC, "times")

fr fr Managing goroutines
stan slay() {
    chaos_mode.SetGoroutineLabel("purpose", "background_task")
    for {
        fr fr Some long-running work
        chaos_mode.Gosched() fr fr Yield to other goroutines
    }
}()

fr fr Finding goroutines by label
bgTasks := chaos_mode.GoroutinesByLabel("purpose", "background_task")
vibez.spill("Found", len(bgTasks), "background task goroutines")

fr fr Getting stack traces
for _, id := range bgTasks[:1] { fr fr Just get the first one
    info := chaos_mode.GoroutineInfo(id)
    vibez.spill("Goroutine", id, "state:", info.State)
    vibez.spill("Stack trace:\n", info.StackTrace)
}

fr fr Forcing garbage collection
vibez.spill("Running garbage collection...")
chaos_mode.GC()

fr fr Getting memory allocation information
topTypes := chaos_mode.TopAllocatedTypes(5)
vibez.spill("Top 5 types by memory usage:")
for i, typeInfo := range topTypes {
    vibez.spill(i+1, ".", typeInfo.Type, "-", typeInfo.TotalSize, "bytes in", typeInfo.Count, "objects")
}

fr fr Setting advanced garbage collection options
chaos_mode.SetGCMode(chaos_mode.GCModeIncrementalOnly)
chaos_mode.RegisterGCNotification(
    func() { vibez.spill("GC cycle starting") },
    func() { vibez.spill("GC cycle complete") },
)

fr fr Performance tuning
chaos_mode.SetSchedulerMode(chaos_mode.SchedulerAggressive)
chaos_mode.SetMaxThreads(8)
```

## Implementation Guidelines
1. Ensure operations are safe and don't crash the runtime
2. Provide detailed tea information for invalid operations
3. Minimize performance overhead of monitoring and profiling functions
4. Make memory statistics accurate and up-to-date
5. Ensure goroutine management is thread-safe
6. Provide backward compatibility with Go's runtime package
7. Include proper documentation for each function
8. Warn about functions that may negatively impact performance