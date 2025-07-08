# StatFlexin Module

Standardized interface for exposing runtime variables and metrics in a "flexing" (showoff-worthy) way with enhanced monitoring capabilities.

## Features

- Flexible variable types (Int, Float, String, Counter)
- Global registry for metric management
- Thread-safe metric operations
- Performance counters and gauges

## Core Types

- **FlexInt**: 64-bit integer metrics with atomic operations
- **FlexFloat**: 64-bit float metrics with arithmetic operations  
- **FlexString**: String metrics with get/set operations
- **FlexCounter**: Monotonic counter with increment/reset
- **Registry**: Global registry for managing all metrics

## Usage Examples

```cursed
// Create metrics
sus hitCounter := stat_flexin.NewFlexCounter("http.hits")
sus activeUsers := stat_flexin.NewFlexInt("users.active")
sus appVersion := stat_flexin.NewFlexString("app.version")

// Register globally
stat_flexin.Register("hits", hitCounter)
stat_flexin.Register("users", activeUsers)

// Use metrics
hitCounter.Inc()
activeUsers.Add(5)
appVersion.Set("1.0.0")

// Access from registry
sus hits := stat_flexin.Get("hits")
vibez.spill("Total hits:", hits.String())
```
