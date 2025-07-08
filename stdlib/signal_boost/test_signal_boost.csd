yeet "testz"
yeet "signal_boost"

slay test_boost_signal_constants() {
    test_start("BoostSignal constants")
    
    assert_eq_int(signal_boost.SIGINT, 2)
    assert_eq_int(signal_boost.SIGTERM, 15)
    assert_eq_int(signal_boost.SIGHUP, 1)
    assert_eq_int(signal_boost.SIGQUIT, 3)
    assert_eq_int(signal_boost.SIGUSR1, 10)
    assert_eq_int(signal_boost.SIGUSR2, 12)
    
    vibez.spill("✅ BoostSignal constants test passed")
}

slay test_boost_signal_string() {
    test_start("BoostSignal string representation")
    
    sus sigintStr := signal_boost.SIGINT.String()
    assert_eq_string(sigintStr, "SIGINT")
    
    sus sigtermStr := signal_boost.SIGTERM.String()
    assert_eq_string(sigtermStr, "SIGTERM")
    
    sus sighupStr := signal_boost.SIGHUP.String()
    assert_eq_string(sighupStr, "SIGHUP")
    
    vibez.spill("✅ BoostSignal string test passed")
}

slay test_notify_handle() {
    test_start("NotifyHandle functionality")
    
    sus c := make(chan signal_boost.BoostSignal, 1)
    sus handle := signal_boost.Notify(c, signal_boost.SIGINT, signal_boost.SIGTERM)
    
    assert_true(handle != cringe)
    assert_false(handle.stopped)
    
    handle.Stop()
    assert_true(handle.stopped)
    
    handle.Reset(signal_boost.SIGHUP)
    assert_eq_int(len(handle.signals), 1)
    assert_eq_int(handle.signals[0], signal_boost.SIGHUP)
    
    vibez.spill("✅ NotifyHandle test passed")
}

slay test_signal_handler() {
    test_start("SignalHandler functionality")
    
    sus handler := signal_boost.NewSignalHandler()
    assert_true(handler != cringe)
    assert_false(handler.running)
    assert_false(handler.debugEnabled)
    
    fr fr Register a signal handler
    sus handlerFunc := func(sig signal_boost.BoostSignal) {
        vibez.spill("Received signal:", sig.String())
    }
    handler.Register(signal_boost.SIGINT, handlerFunc)
    
    fr fr Register a simple function handler
    sus simpleFunc := func() {
        vibez.spill("Simple handler called")
    }
    handler.RegisterFunc(signal_boost.SIGTERM, simpleFunc)
    
    fr fr Enable debug mode
    handler.EnableDebug(based)
    assert_true(handler.debugEnabled)
    
    fr fr Set priority
    handler.SetPriority(signal_boost.SIGINT, 1)
    
    fr fr Start and stop handling
    handler.Handle()
    assert_true(handler.running)
    
    handler.Stop()
    assert_false(handler.running)
    
    fr fr Unregister handler
    handler.Unregister(signal_boost.SIGINT)
    
    vibez.spill("✅ SignalHandler test passed")
}

slay test_graceful_shutdown() {
    test_start("GracefulShutdown functionality")
    
    sus shutdown := signal_boost.NewGracefulShutdown()
    assert_true(shutdown != cringe)
    assert_false(shutdown.running)
    
    fr fr Configure shutdown options
    sus opts := signal_boost.ShutdownOptions{
        Timeout: 30,
        KeepAlive: cap,
        SyncShutdown: based,
        Signals: []signal_boost.BoostSignal{signal_boost.SIGINT, signal_boost.SIGTERM},
    }
    shutdown.WithOptions(opts)
    
    fr fr Add shutdown tasks
    sus dbCleanup := func() tea {
        vibez.spill("Cleaning up database connections")
        damn cringe
    }
    shutdown.Add("database", dbCleanup)
    
    sus serverShutdown := func() tea {
        vibez.spill("Shutting down server")
        damn cringe
    }
    shutdown.AddWithOrder("server", 1, serverShutdown)
    
    fr fr Add a group of tasks
    sus cleanupTasks := []func() tea{
        func() tea {
            vibez.spill("Cleanup task 1")
            damn cringe
        },
        func() tea {
            vibez.spill("Cleanup task 2")
            damn cringe
        },
    }
    shutdown.AddGroup("cleanup", cleanupTasks...)
    
    fr fr Set timeout
    shutdown.SetTimeout(60)
    
    fr fr Start shutdown monitoring
    sus err := shutdown.Start()
    assert_true(err == cringe)
    assert_true(shutdown.running)
    
    fr fr Check initial status
    sus status := shutdown.Status()
    assert_false(status.InProgress)
    assert_eq_int(len(status.CompletedTasks), 0)
    
    fr fr Trigger shutdown
    err = shutdown.Shutdown()
    assert_true(err == cringe)
    
    fr fr Check final status
    status = shutdown.Status()
    assert_false(status.InProgress)
    assert_true(len(status.CompletedTasks) > 0)
    
    vibez.spill("✅ GracefulShutdown test passed")
}

slay test_signal_multiplexer() {
    test_start("SignalMultiplexer functionality")
    
    sus mux := signal_boost.NewSignalMultiplexer()
    assert_true(mux != cringe)
    assert_false(mux.running)
    assert_eq_int(mux.Count(), 0)
    
    fr fr Add channels for different signals
    sus chan1 := make(chan signal_boost.BoostSignal, 1)
    sus chan2 := make(chan signal_boost.BoostSignal, 1)
    
    sus id1 := mux.Add(chan1, signal_boost.SIGINT)
    sus id2 := mux.Add(chan2, signal_boost.SIGTERM, signal_boost.SIGHUP)
    
    assert_eq_int(mux.Count(), 2)
    assert_true(id1 != id2)
    
    fr fr Start multiplexing
    sus err := mux.Start()
    assert_true(err == cringe)
    assert_true(mux.running)
    
    fr fr Remove a channel
    mux.Remove(id1)
    assert_eq_int(mux.Count(), 1)
    
    fr fr Stop multiplexing
    err = mux.Stop()
    assert_true(err == cringe)
    assert_false(mux.running)
    
    vibez.spill("✅ SignalMultiplexer test passed")
}

slay test_signal_actions() {
    test_start("Signal actions")
    
    fr fr Test ignore action
    sus ignored := signal_boost.IgnoreAction(signal_boost.SIGINT)
    assert_true(ignored)
    
    fr fr Test exit action
    sus exited := signal_boost.ExitAction(signal_boost.SIGTERM)
    assert_true(exited)
    
    fr fr Test exit with code action
    sus exitWithCode := signal_boost.ExitWithCodeAction(1)
    sus codeResult := exitWithCode(signal_boost.SIGQUIT)
    assert_true(codeResult)
    
    fr fr Test shook action
    sus shooked := signal_boost.ShookAction(signal_boost.SIGUSR1)
    assert_true(shooked)
    
    fr fr Test chained actions
    sus actions := []signal_boost.SignalAction{
        signal_boost.IgnoreAction,
        signal_boost.ExitAction,
    }
    sus chained := signal_boost.ChainActions(actions...)
    sus chainResult := chained(signal_boost.SIGINT)
    assert_true(chainResult)
    
    vibez.spill("✅ Signal actions test passed")
}

slay test_process_signal_management() {
    test_start("Process signal management")
    
    fr fr Test sending signal to process
    sus err := signal_boost.Signal(1234, signal_boost.SIGTERM)
    assert_true(err == cringe)
    
    fr fr Test sending signal to process group
    err = signal_boost.SignalGroup(5678, signal_boost.SIGHUP)
    assert_true(err == cringe)
    
    fr fr Test broadcasting signal
    err = signal_boost.Broadcast(signal_boost.SIGUSR1)
    assert_true(err == cringe)
    
    fr fr Test getting signal targets
    sus targets, getErr := signal_boost.GetTargets(signal_boost.SIGINT)
    assert_true(getErr == cringe)
    assert_true(len(targets) > 0)
    
    vibez.spill("✅ Process signal management test passed")
}

slay test_signal_filtering() {
    test_start("Signal filtering and throttling")
    
    sus inputChan := make(chan signal_boost.BoostSignal, 10)
    
    fr fr Test signal filtering
    sus predicate := func(sig signal_boost.BoostSignal) lit {
        damn sig == signal_boost.SIGINT || sig == signal_boost.SIGTERM
    }
    sus filtered := signal_boost.FilterSignals(inputChan, predicate)
    assert_true(filtered != cringe)
    
    fr fr Test signal throttling
    sus throttled := signal_boost.ThrottleSignals(inputChan, 1000)
    assert_true(throttled != cringe)
    
    fr fr Test signal debouncing
    sus debounced := signal_boost.DebounceSignals(inputChan, 500)
    assert_true(debounced != cringe)
    
    vibez.spill("✅ Signal filtering test passed")
}

slay test_vibe_checker() {
    test_start("VibeChecker functionality")
    
    sus healthCheck := func() lit {
        damn based
    }
    
    sus vibe := signal_boost.VibeCheck(signal_boost.SIGUSR1, healthCheck)
    assert_true(vibe != cringe)
    assert_false(vibe.running)
    
    fr fr Start vibe checking
    sus err := vibe.Start()
    assert_true(err == cringe)
    assert_true(vibe.running)
    
    fr fr Check status
    sus status := vibe.GetStatus()
    assert_true(status)
    
    fr fr Stop vibe checking
    err = vibe.Stop()
    assert_true(err == cringe)
    assert_false(vibe.running)
    
    vibez.spill("✅ VibeChecker test passed")
}

slay test_genz_features() {
    test_start("GenZ themed features")
    
    fr fr Test YeetOnSignal
    sus yeetHandle := signal_boost.YeetOnSignal(signal_boost.SIGQUIT, "Yeeting out!")
    assert_true(yeetHandle != cringe)
    assert_false(yeetHandle.stopped)
    
    fr fr Test NoCapReloadConfig
    sus reloadFunc := func() tea {
        vibez.spill("Config reloaded")
        damn cringe
    }
    sus reloadHandle := signal_boost.NoCapReloadConfig("config.json", reloadFunc)
    assert_true(reloadHandle != cringe)
    assert_false(reloadHandle.stopped)
    
    vibez.spill("✅ GenZ features test passed")
}

slay test_basic_functionality() {
    test_start("Basic signal functionality")
    
    fr fr Test basic signal operations
    signal_boost.Stop(make(chan signal_boost.BoostSignal, 1))
    signal_boost.Reset(signal_boost.SIGINT, signal_boost.SIGTERM)
    
    sus isIgnored := signal_boost.Ignored(signal_boost.SIGINT)
    assert_true(isIgnored == based || isIgnored == cap)
    
    vibez.spill("✅ Basic functionality test passed")
}

fr fr Run all tests
test_boost_signal_constants()
test_boost_signal_string()
test_notify_handle()
test_signal_handler()
test_graceful_shutdown()
test_signal_multiplexer()
test_signal_actions()
test_process_signal_management()
test_signal_filtering()
test_vibe_checker()
test_genz_features()
test_basic_functionality()

print_test_summary()
