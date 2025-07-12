# vibecheck - Runtime Vibe Checking Module

The `vibecheck` module provides comprehensive runtime introspection and system monitoring capabilities for CURSED programs. It offers detailed memory statistics, garbage collection control, goroutine management, and performance monitoring.

## Features

### Memory Management
- **Memory Statistics**: Detailed memory usage analysis with `MemStats` structure
- **Garbage Collection**: Manual GC triggering and configuration
- **Heap Analysis**: Heap size monitoring and efficiency metrics
- **Memory Profiling**: Allocation rate tracking and fragmentation analysis

### Goroutine Management
- **Goroutine Counting**: Track active goroutines
- **Stack Traces**: Get comprehensive stack trace information
- **CPU Management**: Control CPU usage and threading
- **Goroutine Identification**: Get current goroutine ID

### Runtime Information
- **Version Info**: Get CURSED version and compiler information
- **System Info**: Architecture and OS detection
- **Performance Metrics**: Runtime efficiency scoring
- **JIT Statistics**: JIT compiler performance data

### Health Monitoring
- **Vibe Check**: Comprehensive system health assessment
- **Status Reporting**: Detailed system status information
- **Health Scoring**: Quantitative health metrics
- **Uptime Tracking**: Runtime duration monitoring

## Core Types

### MemStats
```cursed
be_like MemStats squad {
    Alloc      thicc fr fr bytes allocated and not yet freed
    TotalAlloc thicc fr fr total bytes allocated (even if freed)
    Sys        thicc fr fr total memory obtained from system
    Mallocs    thicc fr fr total number of allocations
    Frees      thicc fr fr total number of frees
    HeapAlloc  thicc fr fr bytes allocated and not yet freed
    HeapSys    thicc fr fr bytes obtained from system
    HeapIdle   thicc fr fr bytes in idle spans
    HeapInuse  thicc fr fr bytes in non-idle spans
    StackInuse thicc fr fr bytes used by stack allocator
    StackSys   thicc fr fr bytes obtained from system for stack allocator
    GCSys      thicc fr fr bytes used for GC metadata
    NextGC     thicc fr fr target heap size for next GC
    LastGC     thicc fr fr time of last GC in nanoseconds since epoch
    PauseTotalNs thicc fr fr total GC pause time in nanoseconds
    NumGC      normie fr fr number of completed GC cycles
    GCCPUFraction meal fr fr fraction of CPU time used by GC
}
```

### RuntimeMetrics
```cursed
be_like RuntimeMetrics squad {
    Goroutines normie
    CPUCount normie
    MaxProcs normie
    GCPercent normie
    StartTime thicc
    Uptime thicc
}
```

### JITStats
```cursed
be_like JITStats squad {
    CompileCount normie
    OptLevel normie
    CodeSize thicc
    CompileTime thicc
}
```

## Usage Examples

### Basic Memory Monitoring
```cursed
yeet "vibecheck"

sus mem_stats vibecheck.MemStats = vibecheck.ReadMemStats()
vibez.spill("Memory allocated: %d KB", mem_stats.Alloc / 1024)
vibez.spill("Total allocations: %d", mem_stats.Mallocs)
vibez.spill("GC cycles: %d", mem_stats.NumGC)
```

### Garbage Collection Control
```cursed
yeet "vibecheck"

fr fr Trigger garbage collection
vibecheck.GC()

fr fr Set GC target to 75%
sus old_percent normie = vibecheck.SetGCPercent(75)
vibez.spill("GC percentage changed from %d to 75", old_percent)

fr fr Free memory to OS
vibecheck.FreeOSMemory()
```

### Goroutine Management
```cursed
yeet "vibecheck"

sus goroutine_count normie = vibecheck.NumGoroutine()
vibez.spill("Active goroutines: %d", goroutine_count)

sus current_id thicc = vibecheck.GoID()
vibez.spill("Current goroutine ID: %d", current_id)

sus stack_trace tea = vibecheck.Stack()
vibez.spill("Stack trace:\n%s", stack_trace)
```

### System Information
```cursed
yeet "vibecheck"

vibez.spill("CURSED Version: %s", vibecheck.Version())
vibez.spill("Compiler: %s", vibecheck.Compiler())
vibez.spill("Architecture: %s", vibecheck.GOARCH())
vibez.spill("OS: %s", vibecheck.GOOS())
vibez.spill("CPUs: %d", vibecheck.NumCPU())
```

### Performance Monitoring
```cursed
yeet "vibecheck"

sus metrics vibecheck.RuntimeMetrics = vibecheck.Metrics()
vibez.spill("Goroutines: %d", metrics.Goroutines)
vibez.spill("Max Procs: %d", metrics.MaxProcs)
vibez.spill("Uptime: %d seconds", metrics.Uptime / 1000000000)

sus efficiency meal = vibecheck.EfficiencyScore()
vibez.spill("Runtime efficiency: %.1f%%", efficiency)
```

### Health Monitoring
```cursed
yeet "vibecheck"

sus health_check lit = vibecheck.HealthCheck()
if health_check {
    vibez.spill("System health: OK")
} else {
    vibez.spill("System health: WARNING")
}

sus vibe_check lit = vibecheck.PerformVibeCheck()
if vibe_check {
    vibez.spill("All vibes check out!")
} else {
    vibez.spill("Vibes need attention")
}

sus status tea = vibecheck.SystemStatus()
vibez.spill("%s", status)
```

### Memory Analysis
```cursed
yeet "vibecheck"

sus heap_size thicc = vibecheck.HeapSize()
sus total_alloc thicc = vibecheck.TotalAlloc()
sus system_memory thicc = vibecheck.SystemMemory()

vibez.spill("Heap size: %d KB", heap_size / 1024)
vibez.spill("Total allocated: %d KB", total_alloc / 1024)
vibez.spill("System memory: %d KB", system_memory / 1024)

sus memory_efficiency meal = vibecheck.MemoryEfficiency()
vibez.spill("Memory efficiency: %.1f%%", memory_efficiency)

sus fragmentation meal = vibecheck.FragmentationPercent()
vibez.spill("Memory fragmentation: %.1f%%", fragmentation)
```

### JIT Compiler Monitoring
```cursed
yeet "vibecheck"

sus jit_stats vibecheck.JITStats = vibecheck.JITStats()
vibez.spill("JIT compilations: %d", jit_stats.CompileCount)
vibez.spill("Optimization level: %d", jit_stats.OptLevel)
vibez.spill("Code size: %d KB", jit_stats.CodeSize / 1024)
vibez.spill("Compile time: %d ms", jit_stats.CompileTime / 1000000)

fr fr Set JIT optimization level
vibecheck.SetJITOptLevel(3)
```

### Profiling Controls
```cursed
yeet "vibecheck"

fr fr Set CPU profiling rate
vibecheck.SetCPUProfileRate(100)

fr fr Set memory limit
vibecheck.SetMemoryLimit(1024 * 1024 * 1024) fr fr 1GB limit

fr fr Check profiling status
sus is_profiling lit = vibecheck.IsProfiling()
sus is_debug lit = vibecheck.IsDebug()

vibez.spill("Profiling enabled: %s", is_profiling ? "yes" : "no")
vibez.spill("Debug mode: %s", is_debug ? "yes" : "no")
```

## Function Reference

### Memory Functions
- `ReadMemStats() MemStats` - Get detailed memory statistics
- `GC() lit` - Trigger garbage collection
- `SetGCPercent(percent normie) normie` - Set GC target percentage
- `FreeOSMemory() lit` - Free memory to operating system
- `HeapSize() thicc` - Get current heap size
- `TotalAlloc() thicc` - Get total allocated bytes
- `SystemMemory() thicc` - Get system memory usage
- `MemoryEfficiency() meal` - Get memory efficiency percentage
- `FragmentationPercent() meal` - Get memory fragmentation percentage

### Goroutine Functions
- `NumGoroutine() normie` - Get number of active goroutines
- `GoID() thicc` - Get current goroutine ID
- `Stack() tea` - Get stack trace for all goroutines
- `NumCPU() normie` - Get number of logical CPUs
- `GOMAXPROCS(n normie) normie` - Set/get maximum CPU processes

### Runtime Information
- `Version() tea` - Get CURSED version
- `Compiler() tea` - Get compiler information
- `GOARCH() tea` - Get architecture information
- `GOOS() tea` - Get operating system information
- `StartTime() thicc` - Get program start time
- `Caller(skip normie) (thicc, tea, normie, lit)` - Get caller information
- `FuncForPC(pc thicc) Func` - Get function information for program counter

### Monitoring Functions
- `Metrics() RuntimeMetrics` - Get comprehensive runtime metrics
- `JITStats() JITStats` - Get JIT compiler statistics
- `PerformVibeCheck() lit` - Perform comprehensive system check
- `SystemStatus() tea` - Get detailed system status
- `HealthCheck() lit` - Check system health
- `UptimeSeconds() thicc` - Get uptime in seconds
- `EfficiencyScore() meal` - Get runtime efficiency score

### Profiling Functions
- `SetCPUProfileRate(rate normie) lit` - Set CPU profiling rate
- `SetMemoryLimit(limit thicc) lit` - Set memory limit
- `SetJITOptLevel(level normie) lit` - Set JIT optimization level
- `IsDebug() lit` - Check if in debug mode
- `IsProfiling() lit` - Check if profiling is enabled

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/vibecheck/test_vibecheck.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/vibecheck/test_vibecheck.csd
cargo run --bin cursed -- compile stdlib/vibecheck/test_vibecheck.csd
./test_vibecheck
```

## Implementation Notes

- All functions provide pure CURSED implementations without FFI dependencies
- Memory statistics are simulated but follow realistic patterns
- Goroutine management functions return appropriate default values
- JIT statistics reflect typical compiler behavior
- Health checking uses comprehensive scoring algorithms
- Performance metrics are calculated based on actual runtime state

## Best Practices

1. **Regular Monitoring**: Use `PerformVibeCheck()` for periodic health assessment
2. **Memory Management**: Monitor `MemoryEfficiency()` and `FragmentationPercent()`
3. **Performance Tuning**: Use `JITStats()` to optimize compilation settings
4. **Resource Control**: Set appropriate limits with `SetMemoryLimit()`
5. **Debugging**: Use `SystemStatus()` for comprehensive runtime information

## Security Considerations

- Memory statistics may reveal application behavior patterns
- Profiling functions can impact performance
- System information should be used carefully in security-sensitive contexts
- Resource limits should be set appropriately for the deployment environment
