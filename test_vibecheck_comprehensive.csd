yeet "vibecheck"

vibez.spill("=== CURSED Runtime Vibe Check ===")

fr fr Test memory statistics
sus mem_stats vibecheck.MemStats = vibecheck.ReadMemStats()
vibez.spill("Memory allocated: %d KB", mem_stats.Alloc / 1024)
vibez.spill("Total allocations: %d", mem_stats.Mallocs)
vibez.spill("GC cycles: %d", mem_stats.NumGC)

fr fr Test system information
vibez.spill("CURSED Version: %s", vibecheck.Version())
vibez.spill("Compiler: %s", vibecheck.Compiler())
vibez.spill("Architecture: %s", vibecheck.GOARCH())
vibez.spill("OS: %s", vibecheck.GOOS())

fr fr Test goroutine information
sus goroutine_count normie = vibecheck.NumGoroutine()
vibez.spill("Active goroutines: %d", goroutine_count)

sus current_id thicc = vibecheck.GoID()
vibez.spill("Current goroutine ID: %d", current_id)

fr fr Test performance metrics
sus metrics vibecheck.RuntimeMetrics = vibecheck.Metrics()
vibez.spill("CPUs: %d", metrics.CPUCount)
vibez.spill("Max Procs: %d", metrics.MaxProcs)

fr fr Test health check
sus health_check lit = vibecheck.HealthCheck()
if health_check {
    vibez.spill("✅ System health: OK")
} else {
    vibez.spill("⚠️ System health: WARNING")
}

fr fr Test comprehensive vibe check
sus vibe_check lit = vibecheck.PerformVibeCheck()
if vibe_check {
    vibez.spill("✅ All vibes check out!")
} else {
    vibez.spill("⚠️ Vibes need attention")
}

fr fr Test efficiency metrics
sus efficiency meal = vibecheck.EfficiencyScore()
vibez.spill("Runtime efficiency: %.1f%%", efficiency)

sus memory_efficiency meal = vibecheck.MemoryEfficiency()
vibez.spill("Memory efficiency: %.1f%%", memory_efficiency)

fr fr Test system status
sus status tea = vibecheck.SystemStatus()
vibez.spill("\n%s", status)

vibez.spill("=== Vibe Check Complete ===")
