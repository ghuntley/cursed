# SignalBoost Module

Enhanced signal handling for operating system signals with "boosted" capabilities for better control flow and graceful shutdown patterns.

## Features

- Enhanced signal types and constants
- Signal handler registration and management
- Graceful shutdown orchestration
- Signal multiplexing and filtering
- GenZ themed signal operations

## Key Components

- **BoostSignal**: Enhanced signal type with string representation
- **SignalHandler**: Register multiple handlers for different signals
- **GracefulShutdown**: Coordinate shutdown tasks with timeout and error handling
- **SignalMultiplexer**: Distribute signals to multiple channels
- **VibeChecker**: Health checks triggered by signals

## Usage Examples

```cursed
// Basic signal handling
sus handler := signal_boost.NewSignalHandler()
handler.Register(signal_boost.SIGINT, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received:", sig.String())
})
handler.Handle()

// Graceful shutdown
sus shutdown := signal_boost.NewGracefulShutdown()
shutdown.Add("database", func() tea {
    vibez.spill("Closing DB")
    damn cringe
})
shutdown.Start()

// GenZ features
signal_boost.YeetOnSignal(signal_boost.SIGQUIT, "Yeeting out!")
signal_boost.NoCapReloadConfig("config.json", reloadFunc)
```
