# SignalBoost (os/signal package)

## Overview
SignalBoost provides functionality for working with operating system signals, enabling applications to handle and react to signals in a "boosted" (enhanced) way. It's inspired by Go's os/signal package but with improved control flow, signal management, and graceful shutdown patterns.

## Core Types

### `BoostSignal`
Type that represents an operating system signal.

```go
type BoostSignal os.Signal

// Common signals
var (
    SIGINT   BoostSignal // Interrupt - CTRL+C
    SIGTERM  BoostSignal // Termination request
    SIGHUP   BoostSignal // Terminal connection closed
    SIGQUIT  BoostSignal // Quit - CTRL+\
    SIGILL   BoostSignal // Illegal instruction
    SIGTRAP  BoostSignal // Trace/breakpoint trap
    SIGABRT  BoostSignal // Abort
    SIGBUS   BoostSignal // Bus error
    SIGFPE   BoostSignal // Floating point exception
    SIGKILL  BoostSignal // Kill (cannot be caught or ignored)
    SIGSEGV  BoostSignal // Segmentation fault
    SIGPIPE  BoostSignal // Broken pipe
    SIGALRM  BoostSignal // Timer signal
    SIGCHLD  BoostSignal // Child process terminated
    SIGCONT  BoostSignal // Continue execution if stopped
    SIGSTOP  BoostSignal // Stop execution (cannot be caught or ignored)
    SIGTSTP  BoostSignal // Terminal stop - CTRL+Z
    SIGTTIN  BoostSignal // Terminal input for background process
    SIGTTOU  BoostSignal // Terminal output for background process
    SIGUSR1  BoostSignal // User-defined signal 1
    SIGUSR2  BoostSignal // User-defined signal 2
    SIGWINCH BoostSignal // Window size change
)

// Methods
func (s BoostSignal) String() string
func (s BoostSignal) Signal() // Method to satisfy os.Signal interface
```

## Core Functions

### Signal Notification

```go
// Notify causes package SignalBoost to relay incoming signals to c
func Notify(c chan<- BoostSignal, sig ...BoostSignal) NotifyHandle

// NotifyContext returns a context that is canceled when one of the signals arrives
func NotifyContext(parent VibeContext, sig ...BoostSignal) (ctx VibeContext, stop func())

// NotifyHandle provides a way to stop receiving notifications
type NotifyHandle interface {
    Stop() // Stop signal notifications
    Reset(...BoostSignal) // Reset the signals being monitored
}

// Stop causes package SignalBoost to stop relaying incoming signals to c
func Stop(c chan<- BoostSignal)

// Reset resets the signal handling for the given signals to the default behavior
func Reset(sig ...BoostSignal)

// Ignored reports whether the signal is currently ignored
func Ignored(sig BoostSignal) bool
```

## Enhanced Features

### Signal Handler

```go
type SignalHandler struct {}

// Constructor
func NewSignalHandler() *SignalHandler

// Methods
func (h *SignalHandler) Register(sig BoostSignal, handler func(BoostSignal)) *SignalHandler
func (h *SignalHandler) RegisterFunc(sig BoostSignal, handler func()) *SignalHandler
func (h *SignalHandler) Unregister(sig BoostSignal) *SignalHandler
func (h *SignalHandler) Handle() // Start handling signals
func (h *SignalHandler) Stop() // Stop handling signals
func (h *SignalHandler) EnableDebug(enabled bool) *SignalHandler
func (h *SignalHandler) SetPriority(sig BoostSignal, priority int) *SignalHandler
```

### Graceful Shutdown

```go
type GracefulShutdown struct {}

// Constructor
func NewGracefulShutdown() *GracefulShutdown

// Options
type ShutdownOptions struct {
    Timeout       time.Duration
    PreShutdownFn func()
    ErrorHandler  func(error)
    KeepAlive     bool
    SyncShutdown  bool
    Logger        *sus_log.SusLogger
    Signals       []BoostSignal
}

// Methods
func (gs *GracefulShutdown) WithOptions(opts ShutdownOptions) *GracefulShutdown
func (gs *GracefulShutdown) Add(name string, fn func() error) *GracefulShutdown
func (gs *GracefulShutdown) AddWithOrder(name string, order int, fn func() error) *GracefulShutdown
func (gs *GracefulShutdown) AddGroup(name string, fns ...func() error) *GracefulShutdown
func (gs *GracefulShutdown) Start() error
func (gs *GracefulShutdown) Shutdown() error
func (gs *GracefulShutdown) Wait() error
func (gs *GracefulShutdown) Status() ShutdownStatus
func (gs *GracefulShutdown) SetLogger(logger *sus_log.SusLogger) *GracefulShutdown
func (gs *GracefulShutdown) SetTimeout(timeout time.Duration) *GracefulShutdown

type ShutdownStatus struct {
    InProgress      bool
    ElapsedTime     time.Duration
    CompletedTasks  []string
    RemainingTasks  []string
    Errors          map[string]error
    ShutdownTriggeredBy BoostSignal
}
```

### Signal Multiplexing

```go
type SignalMultiplexer struct {}

// Constructor
func NewSignalMultiplexer() *SignalMultiplexer

// Methods
func (sm *SignalMultiplexer) Add(c chan<- BoostSignal, sig ...BoostSignal) int
func (sm *SignalMultiplexer) Remove(id int)
func (sm *SignalMultiplexer) Start() error
func (sm *SignalMultiplexer) Stop() error
func (sm *SignalMultiplexer) Count() int
```

### Signal Actions

```go
type SignalAction func(BoostSignal) (handled bool)

// Common signal actions
func IgnoreAction(sig BoostSignal) bool
func ExitAction(sig BoostSignal) bool
func ExitWithCodeAction(code int) SignalAction
func LogAction(logger *sus_log.SusLogger) SignalAction
func PanicAction(sig BoostSignal) bool
func ChainActions(actions ...SignalAction) SignalAction
```

### Process Signal Management

```go
// Send a signal to a process
func Signal(pid int, sig BoostSignal) error

// Send a signal to a process group
func SignalGroup(pgid int, sig BoostSignal) error

// Send a signal to all processes
func Broadcast(sig BoostSignal) error

// Get processes that would receive a signal
func GetTargets(sig BoostSignal) ([]int, error)
```

### Signal Filtering and Throttling

```go
// Filter signals based on a predicate
func FilterSignals(in <-chan BoostSignal, predicate func(BoostSignal) bool) <-chan BoostSignal

// Throttle signals to prevent flooding
func ThrottleSignals(in <-chan BoostSignal, interval time.Duration) <-chan BoostSignal

// Debounce signals to only process the last one in a sequence
func DebounceSignals(in <-chan BoostSignal, interval time.Duration) <-chan BoostSignal
```

## GenZ Themed Features

```go
// VibeCheck runs a health check when a specific signal is received
func VibeCheck(sig BoostSignal, check func() (healthy bool)) *VibeChecker

type VibeChecker struct {}

// Methods
func (vc *VibeChecker) Start() error
func (vc *VibeChecker) Stop() error
func (vc *VibeChecker) GetStatus() bool

// YeetOnSignal terminates the program dramatically on signal
func YeetOnSignal(sig BoostSignal, message string) NotifyHandle

// NoCapReloadConfig reloads configuration on SIGHUP without exaggeration
func NoCapReloadConfig(configPath string, reloadFn func() error) NotifyHandle
```

## Complete Example

```go
// Basic signal handling
c := make(chan signal_boost.BoostSignal, 1)
signal_boost.Notify(c, signal_boost.SIGINT, signal_boost.SIGTERM)

// Wait for signal or context cancellation
select {
case sig := <-c:
    vibez.spill("Received signal:", sig)
    // Handle graceful shutdown
    return
case <-ctx.Done():
    vibez.spill("Context canceled")
    return
}

// Using NotifyContext
ctx, stop := signal_boost.NotifyContext(context.Background(), signal_boost.SIGINT, signal_boost.SIGTERM)
defer stop()

// Do some work that respects context
select {
case <-ctx.Done():
    if ctx.Err() == context.Canceled {
        vibez.spill("Received shutdown signal")
    }
    return
case <-time.After(5 * time.Second):
    vibez.spill("Work completed normally")
}

// Using the SignalHandler
handler := signal_boost.NewSignalHandler()

// Register different handlers for different signals
handler.Register(signal_boost.SIGINT, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received interrupt signal")
    // Perform clean shutdown
})

handler.Register(signal_boost.SIGTERM, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received termination signal")
    // Perform clean shutdown
})

// Register handler for custom signal
handler.Register(signal_boost.SIGUSR1, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received SIGUSR1")
    // Reload configuration
    reloadConfig()
})

// Start handling signals
handler.Handle()
defer handler.Stop()

// Using GracefulShutdown
shutdown := signal_boost.NewGracefulShutdown().WithOptions(signal_boost.ShutdownOptions{
    Timeout: 30 * time.Second,
    Signals: []signal_boost.BoostSignal{signal_boost.SIGINT, signal_boost.SIGTERM},
    PreShutdownFn: func() {
        vibez.spill("Starting graceful shutdown...")
    },
    ErrorHandler: func(err error) {
        vibez.spill("Error during shutdown:", err)
    },
})

// Add shutdown tasks
shutdown.Add("database", func() error {
    vibez.spill("Closing database connections...")
    return dbConn.Close()
})

shutdown.Add("http-server", func() error {
    vibez.spill("Shutting down HTTP server...")
    return server.Shutdown(context.Background())
})

shutdown.AddGroup("cleanup", 
    func() error {
        vibez.spill("Removing temporary files...")
        return os.RemoveAll(tempDir)
    },
    func() error {
        vibez.spill("Flushing logs...")
        return logger.Flush()
    },
)

// Start handling shutdown
shutdown.Start()

// Wait for shutdown to complete in another goroutine
go func() {
    if err := shutdown.Wait(); err != nil {
        vibez.spill("Shutdown completed with error:", err)
    } else {
        vibez.spill("Shutdown completed successfully")
    }
}()

// Continue with application logic
// ...

// Manually trigger shutdown
// shutdown.Shutdown()

// Using GenZ themed features
// Run a vibe check when receiving SIGUSR1
vibe := signal_boost.VibeCheck(signal_boost.SIGUSR1, func() bool {
    // Check if application is healthy
    return checkHealth()
})
vibe.Start()
defer vibe.Stop()

// Reload configuration on SIGHUP
signal_boost.NoCapReloadConfig("config.json", func() error {
    return loadConfig("config.json")
})

// Exit dramatically on SIGQUIT
signal_boost.YeetOnSignal(signal_boost.SIGQUIT, "Yeeting out, bruh!")

// Using a multiplexer to distribute signals
mux := signal_boost.NewSignalMultiplexer()

chan1 := make(chan signal_boost.BoostSignal, 1)
chan2 := make(chan signal_boost.BoostSignal, 1)

// Add channels for different signals
id1 := mux.Add(chan1, signal_boost.SIGINT)
id2 := mux.Add(chan2, signal_boost.SIGTERM, signal_boost.SIGHUP)

// Start distributing signals
mux.Start()
defer mux.Stop()

// Use different channels for different purposes
go func() {
    for sig := range chan1 {
        vibez.spill("Chan1 received:", sig)
    }
}()

go func() {
    for sig := range chan2 {
        vibez.spill("Chan2 received:", sig)
    }
}()

// Signal filtering
// Only pass through SIGINT and SIGTERM
filtered := signal_boost.FilterSignals(c, func(sig signal_boost.BoostSignal) bool {
    return sig == signal_boost.SIGINT || sig == signal_boost.SIGTERM
})

// Throttle signals to once per second
throttled := signal_boost.ThrottleSignals(filtered, 1*time.Second)

// Use the filtered signals
for sig := range throttled {
    vibez.spill("Received throttled signal:", sig)
}
```

## Implementation Guidelines
1. Ensure signals are handled reliably across different operating systems
2. Implement graceful shutdown patterns that work in complex applications
3. Provide clear documentation for each signal and its default behavior
4. Design for minimal overhead in signal processing
5. Support both synchronous and asynchronous signal handling models
6. Include safeguards against signal storms and recursive signal handling
7. Implement proper resource cleanup during signal handling
8. Maintain compatibility with Go's os/signal package