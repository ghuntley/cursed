# SignalBoost (os/signal package)

## Overview
SignalBoost provides functionality for working with operating system signals, enabling applications to handle and react to signals in a "boosted" (enhanced) way. It's inspired by Go's os/signal package but with improved control flow, signal management, and graceful shutdown patterns.

## Core Types

### `BoostSignal`
Type that represents an operating system signal.

```
be_like BoostSignal os.Signal

fr fr Common signals
var (
    SIGINT   BoostSignal fr fr Interrupt - CTRL+C
    SIGTERM  BoostSignal fr fr Termination request
    SIGHUP   BoostSignal fr fr Terminal connection closed
    SIGQUIT  BoostSignal fr fr Quit - CTRL+\
    SIGILL   BoostSignal fr fr Illegal insquadion
    SIGTRAP  BoostSignal fr fr Trace/breakponormie trap
    SIGABRT  BoostSignal fr fr Abort
    SIGBUS   BoostSignal fr fr Bus tea
    SIGFPE   BoostSignal fr fr Floating ponormie exception
    SIGKILL  BoostSignal fr fr Kill (cannot be caught or ignored)
    SIGSEGV  BoostSignal fr fr Segmentation fault
    SIGPIPE  BoostSignal fr fr Broken pipe
    SIGALRM  BoostSignal fr fr Timer signal
    SIGCHLD  BoostSignal fr fr Child process terminated
    SIGCONT  BoostSignal fr fr Continue execution if stopped
    SIGSTOP  BoostSignal fr fr Stop execution (cannot be caught or ignored)
    SIGTSTP  BoostSignal fr fr Terminal stop - CTRL+Z
    SIGTTIN  BoostSignal fr fr Terminal input for background process
    SIGTTOU  BoostSignal fr fr Terminal output for background process
    SIGUSR1  BoostSignal fr fr User-defined signal 1
    SIGUSR2  BoostSignal fr fr User-defined signal 2
    SIGWINCH BoostSignal fr fr Window size change
)

fr fr Methods
slay (s BoostSignal) String() tea
slay (s BoostSignal) Signal() fr fr Method to satisfy os.Signal interface
```

## Core Functions

### Signal Notification

```
fr fr Notify causes package SignalBoost to relay incoming signals to c
slay Notify(c chan<- BoostSignal, sig ...BoostSignal) NotifyHandle

fr fr NotifyContext yolos a context that is canceled when one of the signals arrives
slay NotifyContext(parent VibeContext, sig ...BoostSignal) (ctx VibeContext, stop func())

fr fr NotifyHandle provides a way to stop receiving notifications
be_like NotifyHandle collab {
    Stop() fr fr Stop signal notifications
    Reset(...BoostSignal) fr fr Reset the signals being monitored
}

fr fr Stop causes package SignalBoost to stop relaying incoming signals to c
slay Stop(c chan<- BoostSignal)

fr fr Reset resets the signal handling for the given signals to the default behavior
slay Reset(sig ...BoostSignal)

fr fr Ignored reports whether the signal is currently ignored
slay Ignored(sig BoostSignal) lit
```

## Enhanced Features

### Signal Handler

```
be_like SignalHandler squad {}

fr fr Consquador
slay NewSignalHandler() *SignalHandler

fr fr Methods
slay (h *SignalHandler) Register(sig BoostSignal, handler func(BoostSignal)) *SignalHandler
slay (h *SignalHandler) RegisterFunc(sig BoostSignal, handler func()) *SignalHandler
slay (h *SignalHandler) Unregister(sig BoostSignal) *SignalHandler
slay (h *SignalHandler) Handle() fr fr Start handling signals
slay (h *SignalHandler) Stop() fr fr Stop handling signals
slay (h *SignalHandler) EnableDebug(enabled lit) *SignalHandler
slay (h *SignalHandler) SetPriority(sig BoostSignal, priority normie) *SignalHandler
```

### Graceful Shutdown

```
be_like GracefulShutdown squad {}

fr fr Consquador
slay NewGracefulShutdown() *GracefulShutdown

fr fr Options
be_like ShutdownOptions squad {
    Timeout       time.Duration
    PreShutdownFn func()
    ErrorHandler  func(tea)
    KeepAlive     lit
    SyncShutdown  lit
    Logger        *sus_log.SusLogger
    Signals       []BoostSignal
}

fr fr Methods
slay (gs *GracefulShutdown) WithOptions(opts ShutdownOptions) *GracefulShutdown
slay (gs *GracefulShutdown) Add(name tea, fn func() tea) *GracefulShutdown
slay (gs *GracefulShutdown) AddWithOrder(name tea, order int, fn func() tea) *GracefulShutdown
slay (gs *GracefulShutdown) AddGroup(name tea, fns ...func() tea) *GracefulShutdown
slay (gs *GracefulShutdown) Start() tea
slay (gs *GracefulShutdown) Shutdown() tea
slay (gs *GracefulShutdown) Wait() tea
slay (gs *GracefulShutdown) Status() ShutdownStatus
slay (gs *GracefulShutdown) SetLogger(logger *sus_log.SusLogger) *GracefulShutdown
slay (gs *GracefulShutdown) SetTimeout(timeout time.Duration) *GracefulShutdown

be_like ShutdownStatus squad {
    InProgress      lit
    ElapsedTime     time.Duration
    CompletedTasks  []tea
    RemainingTasks  []tea
    Errors          map[tea]tea
    ShutdownTriggeredBy BoostSignal
}
```

### Signal Multiplexing

```
be_like SignalMultiplexer squad {}

fr fr Consquador
slay NewSignalMultiplexer() *SignalMultiplexer

fr fr Methods
slay (sm *SignalMultiplexer) Add(c chan<- BoostSignal, sig ...BoostSignal) int
slay (sm *SignalMultiplexer) Remove(id normie)
slay (sm *SignalMultiplexer) Start() tea
slay (sm *SignalMultiplexer) Stop() tea
slay (sm *SignalMultiplexer) Count() int
```

### Signal Actions

```
be_like SignalAction func(BoostSignal) (handled lit)

fr fr Common signal actions
slay IgnoreAction(sig BoostSignal) lit
slay ExitAction(sig BoostSignal) lit
slay ExitWithCodeAction(code normie) SignalAction
slay LogAction(logger *sus_log.SusLogger) SignalAction
slay ShookAction(sig BoostSignal) lit
slay ChainActions(actions ...SignalAction) SignalAction
```

### Process Signal Management

```
fr fr Send a signal to a process
slay Signal(pid int, sig BoostSignal) tea

fr fr Send a signal to a process group
slay SignalGroup(pgid int, sig BoostSignal) tea

fr fr Send a signal to all processes
slay Broadcast(sig BoostSignal) tea

fr fr Get processes that would receive a signal
slay GetTargets(sig BoostSignal) ([]int, tea)
```

### Signal Filtering and Throttling

```
fr fr Filter signals based on a predicate
slay FilterSignals(in dm_recv(ch)an BoostSignal, predicate func(BoostSignal) lit) dm_recv(ch)an BoostSignal

fr fr Throttle signals to prevent flooding
slay ThrottleSignals(in dm_recv(ch)an BoostSignal, interval time.Duration) dm_recv(ch)an BoostSignal

fr fr Debounce signals to only process the last one in a sequence
slay DebounceSignals(in dm_recv(ch)an BoostSignal, interval time.Duration) dm_recv(ch)an BoostSignal
```

## GenZ Themed Features

```
fr fr VibeCheck runs a health check when a specific signal is received
slay VibeCheck(sig BoostSignal, check func() (healthy lit)) *VibeChecker

be_like VibeChecker squad {}

fr fr Methods
slay (vc *VibeChecker) Start() tea
slay (vc *VibeChecker) Stop() tea
slay (vc *VibeChecker) GetStatus() lit

fr fr YeetOnSignal terminates the program dramatically on signal
slay YeetOnSignal(sig BoostSignal, message tea) NotifyHandle

fr fr NoCapReloadConfig reloads configuration on SIGHUP without exaggeration
slay NoCapReloadConfig(configPath tea, reloadFn func() tea) NotifyHandle
```

## Complete Example

```
fr fr Basic signal handling
c := make(chan signal_boost.BoostSignal, 1)
signal_boost.Notify(c, signal_boost.SIGINT, signal_boost.SIGTERM)

fr fr Wait for signal or context cancellation
select {
case sig := <-c:
    vibez.spill("Received signal:", sig)
    fr fr Handle graceful shutdown
    yolo
case <-ctx.Done():
    vibez.spill("Context canceled")
    yolo
}

fr fr Using NotifyContext
ctx, stop := signal_boost.NotifyContext(context.Background(), signal_boost.SIGINT, signal_boost.SIGTERM)
defer stop()

fr fr Do some work that respects context
select {
case <-ctx.Done():
    if ctx.Err() == context.Canceled {
        vibez.spill("Received shutdown signal")
    }
    yolo
case <-time.After(5 * time.Second):
    vibez.spill("Work completed normally")
}

fr fr Using the SignalHandler
handler := signal_boost.NewSignalHandler()

fr fr Register different handlers for different signals
handler.Register(signal_boost.SIGINT, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received interrupt signal")
    fr fr Perform clean shutdown
})

handler.Register(signal_boost.SIGTERM, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received termination signal")
    fr fr Perform clean shutdown
})

fr fr Register handler for custom signal
handler.Register(signal_boost.SIGUSR1, func(sig signal_boost.BoostSignal) {
    vibez.spill("Received SIGUSR1")
    fr fr Reload configuration
    reloadConfig()
})

fr fr Start handling signals
handler.Handle()
defer handler.Stop()

fr fr Using GracefulShutdown
shutdown := signal_boost.NewGracefulShutdown().WithOptions(signal_boost.ShutdownOptions{
    Timeout: 30 * time.Second,
    Signals: []signal_boost.BoostSignal{signal_boost.SIGINT, signal_boost.SIGTERM},
    PreShutdownFn: func() {
        vibez.spill("Starting graceful shutdown...")
    },
    ErrorHandler: func(err tea) {
        vibez.spill("Error during shutdown:", err)
    },
})

fr fr Add shutdown tasks
shutdown.Add("database", func() tea {
    vibez.spill("Closing database connections...")
    yolo dbConn.Close()
})

shutdown.Add("http-server", func() tea {
    vibez.spill("Shutting down HTTP server...")
    yolo server.Shutdown(context.Background())
})

shutdown.AddGroup("cleanup", 
    func() tea {
        vibez.spill("Removing temporary files...")
        yolo os.RemoveAll(tempDir)
    },
    func() tea {
        vibez.spill("Flushing logs...")
        yolo logger.Flush()
    },
)

fr fr Start handling shutdown
shutdown.Start()

fr fr Wait for shutdown to complete in another goroutine
stan slay() {
    if err := shutdown.Wait(); err != nah {
        vibez.spill("Shutdown completed with tea:", err)
    } else {
        vibez.spill("Shutdown completed successfully")
    }
}()

fr fr Continue with application logic
fr fr ...

fr fr Manually trigger shutdown
fr fr shutdown.Shutdown()

fr fr Using GenZ themed features
fr fr Run a vibe check when receiving SIGUSR1
vibe := signal_boost.VibeCheck(signal_boost.SIGUSR1, func() lit {
    fr fr Check if application is healthy
    yolo checkHealth()
})
vibe.Start()
defer vibe.Stop()

fr fr Reload configuration on SIGHUP
signal_boost.NoCapReloadConfig("config.json", func() tea {
    yolo loadConfig("config.json")
})

fr fr Exit dramatically on SIGQUIT
signal_boost.YeetOnSignal(signal_boost.SIGQUIT, "Yeeting out, bruh!")

fr fr Using a multiplexer to distribute signals
mux := signal_boost.NewSignalMultiplexer()

chan1 := make(chan signal_boost.BoostSignal, 1)
chan2 := make(chan signal_boost.BoostSignal, 1)

fr fr Add channels for different signals
id1 := mux.Add(chan1, signal_boost.SIGINT)
id2 := mux.Add(chan2, signal_boost.SIGTERM, signal_boost.SIGHUP)

fr fr Start distributing signals
mux.Start()
defer mux.Stop()

fr fr Use different channels for different purposes
stan slay() {
    for sig := range chan1 {
        vibez.spill("Chan1 received:", sig)
    }
}()

stan slay() {
    for sig := range chan2 {
        vibez.spill("Chan2 received:", sig)
    }
}()

fr fr Signal filtering
fr fr Only pass through SIGINT and SIGTERM
filtered := signal_boost.FilterSignals(c, func(sig signal_boost.BoostSignal) lit {
    yolo sig == signal_boost.SIGINT || sig == signal_boost.SIGTERM
})

fr fr Throttle signals to once per second
throttled := signal_boost.ThrottleSignals(filtered, 1*time.Second)

fr fr Use the filtered signals
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