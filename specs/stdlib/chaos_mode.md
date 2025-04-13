# ChaosMode (runtime package)

## Overview
ChaosMode provides access to the Cursed runtime system, allowing programs to interact with the Go runtime for goroutine management, debugging, and system information. It's inspired by Go's runtime package with a chaotic twist focused on performance and observability.

## Core Functions

### Goroutine Management

```go
// Returns the number of logical CPUs usable by the current process
func NumCPU() int

// Returns the number of goroutines that currently exist
func NumGoroutine() int

// Yields the processor, allowing other goroutines to run
func Yield()

// Puts the current goroutine into a waiting state and schedules another goroutine
func Gosched()

// Forces garbage collection to run
func GC()

// Increases GOMAXPROCS, returns the previous setting
func GOMAXPROCS(n int) int

// Controls the garbage collector's target percentage
func SetGCPercent(percent int) int

// Controls the fraction of memory that should be used for garbage collection
func SetMaxHeap(maxHeap uint64) uint64
```

### Stack and Panic Management

```go
// Returns a formatted stack trace of the goroutine that calls it
func StackTrace() string

// Returns a stack trace of goroutine IDs
func AllGoroutineIDs() []uint64

// Gets a JSON representation of all goroutines' stack traces
func AllGoroutineStacks() string

// Captures a stack trace of the current goroutine
func Callers(skip int, pc []uintptr) int

// Gets the file and line number for a PC
func PCToFileAndLine(pc uintptr) (file string, line int)

// Gets the function name for a PC
func PCToFuncName(pc uintptr) string

// Gets the call stack of a goroutine
func GoroutineStack(id uint64) string
```

### Memory Management

```go
// Returns memory allocation statistics
func MemStats() MemoryStats

// MemoryStats contains memory statistics
type MemoryStats struct {
    // General statistics
    Alloc        uint64 // bytes allocated and still in use
    TotalAlloc   uint64 // bytes allocated (even if freed)
    Sys          uint64 // bytes obtained from system
    Lookups      uint64 // number of pointer lookups
    Mallocs      uint64 // number of mallocs
    Frees        uint64 // number of frees
    
    // Heap statistics
    HeapAlloc    uint64 // bytes allocated and still in use
    HeapSys      uint64 // bytes obtained from system
    HeapIdle     uint64 // bytes in idle spans
    HeapInuse    uint64 // bytes in non-idle span
    HeapReleased uint64 // bytes released to the OS
    HeapObjects  uint64 // total number of allocated objects
    
    // Garbage collection statistics
    NextGC       uint64 // next collection will happen when HeapAlloc ≥ this
    LastGC       uint64 // last collection time, Unix nanoseconds
    PauseTotalNs uint64 // total GC pause time
    NumGC        uint32 // number of garbage collections
    GCCPUFraction float64 // fraction of CPU time used by GC
}

// Controls whether the garbage collector is enabled
func SetGCEnabled(enabled bool) bool

// ReadMemStats populates m with memory allocator statistics
func ReadMemStats(m *MemoryStats)

// FreeOSMemory forces a garbage collection and releases as much memory to the OS as possible
func FreeOSMemory()

// Sets the frequency of memory profiling
func SetMemProfileRate(rate int)
```

### Goroutine Profiling and Tracing

```go
// StartTrace enables runtime tracing
func StartTrace() error

// StopTrace stops runtime tracing
func StopTrace() error

// ReadTrace returns the current trace
func ReadTrace() []byte

// SetTracebackLimit sets the maximum length of a traceback
func SetTracebackLimit(limit int)

// Sets CPU profiling rate
func SetCPUProfileRate(hz int)

// Starts CPU profiling
func StartCPUProfile(w io.Writer) error

// Stops CPU profiling
func StopCPUProfile()
```

### Runtime Information

```go
// Returns the Go version string
func Version() string

// Returns the Go architecture target
func GOARCH() string

// Returns the Go operating system target
func GOOS() string

// Returns the compiler that built the binary
func Compiler() string

// Gets runtime statistics
func RuntimeStats() map[string]interface{}

// Gets the current Go root directory
func GOROOT() string
```

## Enhanced Features

### Goroutine Management Extensions

```go
// Gets information about a specific goroutine
func GoroutineInfo(id uint64) GoroutineData

// GoroutineData contains detailed information about a goroutine
type GoroutineData struct {
    ID          uint64
    State       string
    WaitingFor  string
    WaitingTime time.Duration
    StackTrace  string
    Labels      map[string]string
    CreatedBy   string
    CreatedAt   time.Time
    CPUTime     time.Duration
}

// Sets a label for the current goroutine
func SetGoroutineLabel(key, value string)

// Gets all goroutines with a specific label
func GoroutinesByLabel(key, value string) []uint64

// Gets all goroutines by state
func GoroutinesByState(state string) []uint64

// Kills a specific goroutine (for debugging purposes only)
func KillGoroutine(id uint64) error
```

### Enhanced Garbage Collection

```go
// Fine-grained garbage collection control
type GCMode int

const (
    GCModeAuto GCMode = iota
    GCModeManual
    GCModeIncrementalOnly
    GCModeStopTheWorldOnly
)

func SetGCMode(mode GCMode)
func GetGCMode() GCMode

// Starts a concurrent garbage collection cycle
func StartGC()

// Waits for the current GC cycle to complete
func WaitForGC() bool

// Registers a function to be called before/after garbage collection
func RegisterGCNotification(before, after func())
```

### Memory Debugging

```go
// Gets a histogram of allocated object sizes
func AllocationSizeHistogram() map[int]int

// Gets the types with the most allocations
func TopAllocatedTypes(n int) []TypeAllocationInfo

type TypeAllocationInfo struct {
    Type        string
    Count       int
    TotalSize   int64
    AverageSize int64
}

// Checks if a pointer is valid and points to allocated memory
func IsValidPointer(ptr interface{}) bool

// Gets the size of an allocated object
func GetObjectSize(obj interface{}) int

// Gets information about a pointer's referent
func GetPointerInfo(ptr interface{}) PointerInfo

type PointerInfo struct {
    Address    uintptr
    Size       int
    Type       string
    Reachable  bool
    AllocTime  time.Time
    AllocStack string
}
```

### Runtime Performance Tuning

```go
// Sets the maximum number of threads to use
func SetMaxThreads(n int) int

// Gets the current number of threads
func NumThreads() int

// Controls CPU frequency scaling (if supported by OS)
func SetCPUFrequency(percent int) error

// Sets thread priorities (if supported by OS)
func SetThreadPriority(threadID int, priority int) error

// Controls the runtime scheduler
type SchedulerMode int

const (
    SchedulerDefault SchedulerMode = iota
    SchedulerFair
    SchedulerAggressive
    SchedulerConservative
)

func SetSchedulerMode(mode SchedulerMode)
func GetSchedulerMode() SchedulerMode
```

## Usage Example

```go
// Getting basic runtime information
vibez.spill("Cursed is running on:", chaos_mode.GOOS(), chaos_mode.GOARCH())
vibez.spill("Using", chaos_mode.NumCPU(), "CPUs")
vibez.spill("Currently", chaos_mode.NumGoroutine(), "goroutines are running")

// Setting GOMAXPROCS
old := chaos_mode.GOMAXPROCS(4)
vibez.spill("Changed GOMAXPROCS from", old, "to 4")

// Getting memory statistics
memStats := chaos_mode.MemStats()
vibez.spill("Currently using", memStats.HeapAlloc, "bytes of heap memory")
vibez.spill("Garbage collector has run", memStats.NumGC, "times")

// Managing goroutines
go func() {
    chaos_mode.SetGoroutineLabel("purpose", "background_task")
    for {
        // Some long-running work
        chaos_mode.Gosched() // Yield to other goroutines
    }
}()

// Finding goroutines by label
bgTasks := chaos_mode.GoroutinesByLabel("purpose", "background_task")
vibez.spill("Found", len(bgTasks), "background task goroutines")

// Getting stack traces
for _, id := range bgTasks[:1] { // Just get the first one
    info := chaos_mode.GoroutineInfo(id)
    vibez.spill("Goroutine", id, "state:", info.State)
    vibez.spill("Stack trace:\n", info.StackTrace)
}

// Forcing garbage collection
vibez.spill("Running garbage collection...")
chaos_mode.GC()

// Getting memory allocation information
topTypes := chaos_mode.TopAllocatedTypes(5)
vibez.spill("Top 5 types by memory usage:")
for i, typeInfo := range topTypes {
    vibez.spill(i+1, ".", typeInfo.Type, "-", typeInfo.TotalSize, "bytes in", typeInfo.Count, "objects")
}

// Setting advanced garbage collection options
chaos_mode.SetGCMode(chaos_mode.GCModeIncrementalOnly)
chaos_mode.RegisterGCNotification(
    func() { vibez.spill("GC cycle starting") },
    func() { vibez.spill("GC cycle complete") },
)

// Performance tuning
chaos_mode.SetSchedulerMode(chaos_mode.SchedulerAggressive)
chaos_mode.SetMaxThreads(8)
```

## Implementation Guidelines
1. Ensure operations are safe and don't crash the runtime
2. Provide detailed error information for invalid operations
3. Minimize performance overhead of monitoring and profiling functions
4. Make memory statistics accurate and up-to-date
5. Ensure goroutine management is thread-safe
6. Provide backward compatibility with Go's runtime package
7. Include proper documentation for each function
8. Warn about functions that may negatively impact performance