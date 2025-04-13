# vibecheck (runtime)

## Overview
The `vibecheck` module provides low-level runtime functionality and direct interaction with the Cursed runtime system. It offers capabilities like memory stats, garbage collection control, goroutine management, and runtime configuration.

## Core Types and Interfaces

### MemStats
Provides detailed memory statistics.

```csd
be_like MemStats squad {
  Alloc      uint64 fr fr bytes allocated and not yet freed
  TotalAlloc uint64 fr fr total bytes allocated (even if freed)
  Sys        uint64 fr fr total memory obtained from system
  Mallocs    uint64 fr fr total number of allocations
  Frees      uint64 fr fr total number of frees
  HeapAlloc  uint64 fr fr bytes allocated and not yet freed (same as Alloc)
  HeapSys    uint64 fr fr bytes obtained from system
  HeapIdle   uint64 fr fr bytes in idle spans
  HeapInuse  uint64 fr fr bytes in non-idle spans
  StackInuse uint64 fr fr bytes used by stack allocator
  StackSys   uint64 fr fr bytes obtained from system for stack allocator
  GCSys      uint64 fr fr bytes used for GC metadata
  NextGC     uint64 fr fr target heap size for next GC
  LastGC     uint64 fr fr time of last GC in nanoseconds since epoch
  PauseTotalNs uint64 fr fr total GC pause time in nanoseconds
  NumGC      uint32 fr fr number of completed GC cycles
  GCCPUFraction float64 fr fr fraction of CPU time used by GC
}
```

### Func
Contains information about a function.

```csd
be_like Func squad {
  fr fr fields not directly accessible
}

slay (f *Func) Name() tea
slay (f *Func) Entry() uintptr
slay (f *Func) FileLine(pc uintptr) (file tea, line normie)
```

### StackRecord
Represents a single entry in a stack trace.

```csd
be_like StackRecord squad {
  PC uintptr fr fr program counter
  Func *Func fr fr function information
  File tea fr fr file name
  Line normie    fr fr line number
}
```

## Core Functions

### Memory Management

```csd
fr fr Get detailed memory statistics
slay ReadMemStats(m *MemStats)

fr fr Run garbage collection synchronously
slay GC()

fr fr Set garbage collector target percentage
slay SetGCPercent(percent normie) int

fr fr Free memory to operating system
slay FreeOSMemory()
```

### Goroutine Management

```csd
fr fr Current number of goroutines
slay NumGoroutine() int

fr fr Get current goroutine ID
slay GoID() int64

fr fr Get stack trace for all goroutines
slay Stack() []byte

fr fr Get the number of logical CPUs usable by the current process
slay NumCPU() int

fr fr Set maximum number of CPUs that can be executing simultaneously
slay GOMAXPROCS(n normie) int
```

### Runtime Information

```csd
fr fr Get Cursed version information
slay Version() tea

fr fr Get compiler information
slay Compiler() tea

fr fr Get GOARCH equivalent
slay GOARCH() tea

fr fr Get GOOS equivalent
slay GOOS() tea

fr fr Get start time of the program
slay StartTime() int64

fr fr Get caller frame skipping n frames
slay Caller(skip normie) (pc uintptr, file tea, line int, ok lit)
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
  vibecheck.BlockProfile(based) fr fr Enable blocking profile
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
  vibecheck.SetJITOptLevel(2) fr fr Set optimization level
  ```

- **Runtime Metrics**: Real-time performance monitoring
  ```csd
  metrics := vibecheck.Metrics()
  ```

- **Resource Control**: Fine-grained control over system resources
  ```csd
  vibecheck.SetCPUProfileRate(100)  fr fr 100 samples per second
  vibecheck.SetMemoryLimit(1 << 30) fr fr 1GB memory limit
  ```

## Usage Examples

```csd
fr fr Memory statistics example
var m vibecheck.MemStats
vibecheck.ReadMemStats(&m)

vibez.spill("Allocated: %d KB", m.Alloc / 1024)
vibez.spill("Total Allocated: %d KB", m.TotalAlloc / 1024)
vibez.spill("System Memory: %d KB", m.Sys / 1024)
vibez.spill("Garbage Collections: %d", m.NumGC)

fr fr Goroutine information
vibez.spill("Number of goroutines: %d", vibecheck.NumGoroutine())
vibez.spill("Max PROCS: %d", vibecheck.GOMAXPROCS(0))
vibez.spill("Number of CPUs: %d", vibecheck.NumCPU())

fr fr Set maximum number of processors
old := vibecheck.GOMAXPROCS(4)
vibez.spill("Old GOMAXPROCS: %d, New: 4", old)

fr fr Trigger garbage collection
vibez.spill("Triggering garbage collection...")
vibecheck.GC()

fr fr Get current goroutine ID
vibez.spill("Current goroutine ID: %d", vibecheck.GoID())

fr fr Get stack trace of all goroutines
stack := vibecheck.Stack()
vibez.spill("\nStack Trace:\n%s", tea(stack))

fr fr Get caller information
pc, file, line, ok := vibecheck.Caller(0)
if ok {
  f := vibecheck.FuncForPC(pc)
  vibez.spill("Called from %s (%s:%d)", f.Name(), file, line)
}

fr fr Version information
vibez.spill("Cursed Version: %s", vibecheck.Version())
vibez.spill("Compiler: %s", vibecheck.Compiler())
vibez.spill("Architecture: %s", vibecheck.GOARCH())
vibez.spill("Operating System: %s", vibecheck.GOOS())

fr fr Memory profiling
vibez.spill("\nStarting memory allocation...")
data := make([]byte, 10*1024*1024) fr fr Allocate 10MB
_ = data fr fr Prevent compiler optimization

vibecheck.ReadMemStats(&m)
vibez.spill("After allocation: %d KB", m.Alloc / 1024)

fr fr Set GC percentage
old_percent := vibecheck.SetGCPercent(100) fr fr Set to 100%
vibez.spill("Old GC percent: %d, New: 100", old_percent)

fr fr Performance monitoring
vibecheck.SetCPUProfileRate(100) fr fr Sample 100 times per second
cpu_profile := vibecheck.CPUProfile()
defer cpu_profile.Stop()

fr fr Run a CPU-intensive task
for i := 0; i < 1000000; i++ {
  _ = i * i
}

fr fr Custom GC control
vibecheck.SetFinalizer(&data, func(obj interface{}) {
  vibez.spill("Object being finalized")
})
vibecheck.KeepAlive(&data) fr fr Prevent premature collection
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