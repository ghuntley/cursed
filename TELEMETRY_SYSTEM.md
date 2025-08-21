# CURSED Opt-In Telemetry System for Bug Bash

## Overview
Anonymous crash reporting and performance metrics to improve CURSED stability during the v1.0.0-rc2 Bug Bash period.

## Privacy-First Design

### Data Collection Principles
1. **Explicit Opt-In**: No telemetry without user consent
2. **Anonymous Only**: No personally identifiable information
3. **Minimal Data**: Only crash fingerprints and performance metrics
4. **Local Control**: Users can inspect and delete data anytime
5. **Transparent**: All collected data visible to users

### User Consent Flow
```bash
# First run telemetry prompt
CURSED v1.0.0-rc2 Bug Bash Telemetry

Help improve CURSED by sharing anonymous crash reports and performance data.

What we collect:
  ✓ Crash stack traces (code locations only)
  ✓ Performance metrics (compile/runtime timing)  
  ✓ Platform info (OS, architecture)
  ✓ Language feature usage (anonymized)

What we DON'T collect:
  ✗ Your source code
  ✗ File names or paths
  ✗ Personal information
  ✗ Network activity

Enable telemetry? [y/N]: _
```

### Configuration
```bash
# Enable telemetry
cursed-zig --telemetry enable

# Disable telemetry  
cursed-zig --telemetry disable

# View collected data
cursed-zig --telemetry show

# Clear local telemetry data
cursed-zig --telemetry clear
```

## Crash Fingerprinting

### Stack Trace Anonymization
```zig
// Original stack trace (NOT collected)
panic: index out of bounds
  at src/main.zig:45:12 (MyProject.parseUserInput)
  at /home/user/project/parser.zig:123:8 (MyProject.run)

// Anonymized fingerprint (collected)
{
  "crash_id": "index_out_of_bounds_hash_af2b1",
  "stack": [
    {"file": "main.zig", "line": 45, "function": "parseUserInput"},
    {"file": "parser.zig", "line": 123, "function": "run"}
  ],
  "platform": "linux-x86_64",
  "version": "1.0.0-rc2",
  "timestamp": "2025-08-21T10:30:00Z"
}
```

### Crash Categories
```json
{
  "compiler_crashes": {
    "parser_panic": {"count": 12, "files": ["complex_generics.csd"]},
    "codegen_error": {"count": 5, "files": ["concurrency_test.csd"]},
    "llvm_assertion": {"count": 3, "files": ["cross_compile.csd"]}
  },
  
  "runtime_crashes": {
    "memory_corruption": {"count": 2, "modules": ["stringz"]},
    "channel_deadlock": {"count": 7, "modules": ["concurrenz"]},
    "stack_overflow": {"count": 1, "modules": ["recursive_test"]}
  }
}
```

## Performance Metrics

### Compilation Metrics
```json
{
  "compilation": {
    "avg_time_ms": 120,
    "peak_memory_mb": 85,
    "file_size_bytes": 2048,
    "feature_usage": {
      "generics": 0.23,
      "concurrency": 0.15,
      "error_handling": 0.67
    }
  }
}
```

### Runtime Metrics  
```json
{
  "runtime": {
    "startup_time_ms": 12,
    "goroutines_created": 45,
    "channels_created": 8,
    "gc_pause_times_ms": [0.8, 1.2, 0.5],
    "memory_peak_mb": 128
  }
}
```

### Platform Distribution
```json
{
  "platforms": {
    "linux-x86_64": 0.45,
    "linux-aarch64": 0.12, 
    "macos-x86_64": 0.18,
    "macos-aarch64": 0.15,
    "windows-x86_64": 0.08,
    "wasm32-wasi": 0.02
  }
}
```

## Implementation

### Local Storage
```bash
# Telemetry data stored locally
~/.cursed/telemetry/
├── config.json          # User preferences
├── crashes/             # Anonymized crash dumps
│   ├── 2025-08-21-001.json
│   └── 2025-08-21-002.json
├── performance/         # Performance metrics
│   ├── compile-2025-08-21.json
│   └── runtime-2025-08-21.json
└── usage/              # Feature usage stats
    └── features-2025-08-21.json
```

### Data Upload
```bash
# Batch upload (daily, user can review before sending)
cursed-zig --telemetry upload --review

# Review shows:
Uploading 3 crash reports and 15 performance samples to:
https://telemetry.cursedlang.org/v1/batch

Crash fingerprints:
  - compiler_panic_generics (occurred 2 times)
  - runtime_channel_deadlock (occurred 1 time)

Performance summary:
  - Avg compile time: 156ms (15 samples)
  - Avg memory usage: 78MB (15 samples)

Continue? [y/N]: _
```

### Server-Side Processing
```yaml
# Telemetry aggregation pipeline
ingestion:
  - validate_schema
  - deduplicate_fingerprints
  - anonymize_further
  
analysis:
  - crash_clustering
  - performance_regression_detection  
  - platform_specific_issues
  
reporting:
  - daily_stability_dashboard
  - weekly_bug_bash_insights
  - real_time_critical_alerts
```

## Bug Bash Integration

### Real-Time Crash Detection
```json
{
  "alert": "critical_crash_spike",
  "description": "Parser panic increased 300% in last hour",
  "crash_signature": "complex_generics_parser_overflow",
  "affected_platforms": ["linux-x86_64", "macos-aarch64"],
  "sample_count": 47,
  "first_seen": "2025-08-21T14:30:00Z",
  "action": "investigate_immediately"
}
```

### Community Feedback Loop
```markdown
# Weekly Bug Bash Telemetry Report

## Stability Trends
- **Crash Rate**: 2.3% (improved from 3.1% last week)
- **Top Crashes**: Parser generics (34%), Channel deadlocks (28%)  
- **Platform Stability**: Linux > macOS > Windows

## Performance Insights  
- **Compile Time**: 15% improvement with latest optimizations
- **Memory Usage**: Stable across all platforms
- **Feature Adoption**: Error handling usage up 40%

## Community Impact
- **Data Contributors**: 234 users opted in (67% participation)
- **Crash Reports**: 1,247 unique fingerprints collected
- **Performance Samples**: 15,678 compilation measurements

## Action Items
- [ ] Fix parser panic with nested generics (P0)
- [ ] Investigate channel implementation race condition (P1)
- [ ] Optimize memory allocation in string module (P2)
```

### Privacy Dashboard
```bash
cursed-zig --telemetry dashboard

CURSED Telemetry Dashboard
========================

Status: ✅ ENABLED (opted in 5 days ago)

Local Data:
  Crashes: 3 reports (2.1 KB)
  Performance: 47 samples (15.3 KB)  
  Usage: 12 days (4.2 KB)
  Total: 21.6 KB

Uploads:
  Last Upload: 2 days ago (18 items)
  Next Upload: in 22 hours
  Total Uploaded: 156 items
  
Privacy:
  ✅ All data anonymized
  ✅ No source code collected  
  ✅ No personal info collected
  ✅ Can disable anytime

Commands:
  View data: cursed-zig --telemetry show
  Clear data: cursed-zig --telemetry clear  
  Disable: cursed-zig --telemetry disable
```

## Security Considerations

### Data Security
- **Transport**: TLS 1.3 encryption for all uploads
- **Storage**: Encrypted at rest with automatic key rotation
- **Access**: Strict team access controls with audit logging
- **Retention**: 90 days maximum, deleted automatically

### Attack Prevention
- **Rate Limiting**: Max 1000 reports per IP per day
- **Validation**: Strict schema validation, size limits
- **Monitoring**: Anomaly detection for spam/abuse
- **Isolation**: Telemetry system isolated from main infrastructure

### Privacy Compliance
- **GDPR Ready**: Right to deletion, data portability
- **No Tracking**: No user identification or tracking
- **Consent**: Clear opt-in process with easy opt-out
- **Transparency**: Open source telemetry client code

## Implementation Code

### Telemetry Client (src-zig/telemetry.zig)
```zig
const std = @import("std");
const json = std.json;

pub const TelemetryConfig = struct {
    enabled: bool = false,
    last_upload: i64 = 0,
    data_retention_days: u32 = 30,
};

pub const CrashReport = struct {
    crash_id: []const u8,
    stack_trace: []StackFrame,
    platform: []const u8,
    version: []const u8,
    timestamp: i64,
    
    const StackFrame = struct {
        file: []const u8,
        line: u32,
        function: []const u8,
    };
};

pub fn recordCrash(allocator: std.mem.Allocator, stack: []StackFrame) !void {
    const config = try loadConfig(allocator);
    if (!config.enabled) return;
    
    const crash = CrashReport{
        .crash_id = try generateFingerprint(allocator, stack),
        .stack_trace = try anonymizeStack(allocator, stack),
        .platform = try getPlatform(allocator),
        .version = "1.0.0-rc2",
        .timestamp = std.time.timestamp(),
    };
    
    try saveCrashReport(allocator, crash);
}
```

This telemetry system provides valuable crash and performance insights while maintaining strict privacy standards and user control.
