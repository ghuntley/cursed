fr fr SignalBoost Demo - Enhanced signal handling for CURSED
fr fr This example demonstrates all the major features of the SignalBoost module

yeet "stdlib::signal_boost"

slay main_character() tea {
    vibez.spill("🚀 Starting SignalBoost demo...")
    
    fr fr Initialize the SignalBoost module
    signal_boost.initialize()
    
    fr fr Demo 1: Basic signal notification
    demo_basic_signals()
    
    fr fr Demo 2: Signal handler with custom handlers
    demo_signal_handler()
    
    fr fr Demo 3: Graceful shutdown
    demo_graceful_shutdown()
    
    fr fr Demo 4: Signal multiplexing
    demo_signal_multiplexing()
    
    fr fr Demo 5: Signal filtering and throttling
    demo_signal_filtering()
    
    fr fr Demo 6: GenZ themed features
    demo_genZ_features()
    
    vibez.spill("✨ SignalBoost demo completed!")
}

slay demo_basic_signals() tea {
    vibez.spill("\n📡 Demo 1: Basic Signal Notification")
    
    fr fr Set up basic signal notification
    facts signals = [signal_boost.SIGINT, signal_boost.SIGTERM, signal_boost.SIGUSR1]
    facts (receiver, handle) = signal_boost.notify(signals)
    
    vibez.spill("✅ Set up signal notification for: SIGINT, SIGTERM, SIGUSR1")
    vibez.spill("   - Handle ID: {}", handle.id())
    vibez.spill("   - Active: {}", handle.is_active())
    vibez.spill("   - Signals: {:?}", handle.signals())
    
    fr fr Demonstrate handle operations
    facts new_signals = [signal_boost.SIGTERM, signal_boost.SIGHUP]
    handle.reset(new_signals)
    vibez.spill("✅ Reset signals to: SIGTERM, SIGHUP")
    
    fr fr Clean up
    handle.stop()
    vibez.spill("✅ Stopped signal notification")
}

slay demo_signal_handler() tea {
    vibez.spill("\n🎯 Demo 2: Signal Handler with Custom Handlers")
    
    fr fr Create signal handler
    sus handler = signal_boost.SignalHandler.new()
    
    fr fr Register different types of handlers
    handler.register(signal_boost.SIGINT, slay(sig) {
        vibez.spill("🛑 Received SIGINT: {}", sig)
        vibez.spill("   Performing graceful interrupt handling...")
    })
    
    handler.register_func(signal_boost.SIGTERM, slay() {
        vibez.spill("💀 Received SIGTERM - initiating shutdown sequence")
    })
    
    fr fr Register handler with priority
    handler.register_with_priority(signal_boost.SIGUSR1, 10, "high_priority_usr1", slay(sig) {
        vibez.spill("👆 High priority SIGUSR1 handler: {}", sig)
    })
    
    handler.register_with_priority(signal_boost.SIGUSR1, 5, "low_priority_usr1", slay(sig) {
        vibez.spill("👆 Low priority SIGUSR1 handler: {}", sig)
    })
    
    fr fr Enable debug mode
    handler.enable_debug(based)
    
    fr fr Show handler statistics
    facts stats = handler.get_statistics()
    vibez.spill("✅ Signal handler configured:")
    vibez.spill("   - Total signals: {}", stats.total_signals)
    vibez.spill("   - Total handlers: {}", stats.total_handlers)
    vibez.spill("   - Running: {}", handler.is_running())
    
    fr fr Demonstrate priority adjustment
    handler.set_priority(signal_boost.SIGUSR1, "low_priority_usr1", 15)
    vibez.spill("✅ Adjusted priority for low_priority_usr1 to 15")
    
    fr fr The handler would start listening here in a real application
    fr fr handler.handle() // This would block and listen for signals
    
    vibez.spill("✅ Signal handler demo completed")
}

slay demo_graceful_shutdown() tea {
    vibez.spill("\n🛑 Demo 3: Graceful Shutdown")
    
    fr fr Create graceful shutdown with custom options
    facts options = signal_boost.ShutdownOptions{
        timeout: time.seconds(30),
        pre_shutdown_fn: slay() {
            vibez.spill("🔄 Pre-shutdown: Preparing for graceful exit...")
        },
        error_handler: slay(err) {
            vibez.spill("❌ Shutdown error: {}", err)
        },
        keep_alive: cap,
        sync_shutdown: based,
        signals: [signal_boost.SIGINT, signal_boost.SIGTERM]
    }
    
    sus shutdown = signal_boost.GracefulShutdown.with_options(options)
    
    fr fr Add individual shutdown tasks
    shutdown.add("database", slay() tea {
        vibez.spill("💾 Closing database connections...")
        time.sleep_millis(100) fr fr Simulate database cleanup
        damn Ok(())
    })
    
    shutdown.add_with_order("http_server", 1, slay() tea {
        vibez.spill("🌐 Shutting down HTTP server...")
        time.sleep_millis(50) fr fr Simulate server shutdown
        damn Ok(())
    })
    
    shutdown.add_with_order("cache", 2, slay() tea {
        vibez.spill("🗄️ Flushing cache...")
        time.sleep_millis(25)
        damn Ok(())
    })
    
    fr fr Add a task group
    facts cleanup_group = signal_boost.ShutdownTaskGroup.new("cleanup", cap)
        .add_task("temp_files", slay() tea {
            vibez.spill("🗑️ Removing temporary files...")
            damn Ok(())
        })
        .add_task("logs", slay() tea {
            vibez.spill("📝 Flushing logs...")
            damn Ok(())
        })
    
    shutdown.add_group(cleanup_group)
    
    fr fr Show shutdown status
    facts status = shutdown.status()
    vibez.spill("✅ Graceful shutdown configured:")
    vibez.spill("   - In progress: {}", status.in_progress)
    vibez.spill("   - Completed tasks: {}", status.completed_tasks.len())
    vibez.spill("   - Errors: {}", status.errors.len())
    
    fr fr In a real application, you would:
    fr fr shutdown.start()    // Start listening for signals
    fr fr shutdown.wait()     // Wait for shutdown completion
    
    vibez.spill("✅ Graceful shutdown demo completed")
}

slay demo_signal_multiplexing() tea {
    vibez.spill("\n🔀 Demo 4: Signal Multiplexing")
    
    fr fr Create signal multiplexer
    sus multiplexer = signal_boost.SignalMultiplexer.new()
    
    fr fr Create channels for different purposes
    facts (ui_sender, ui_receiver) = channel()
    facts (logger_sender, logger_receiver) = channel()
    facts (metrics_sender, metrics_receiver) = channel()
    
    fr fr Add subscriptions
    facts ui_handle = multiplexer.add(ui_sender, [signal_boost.SIGINT, signal_boost.SIGTERM])
    facts logger_handle = multiplexer.add(logger_sender, [signal_boost.SIGUSR1, signal_boost.SIGUSR2])
    facts metrics_handle = multiplexer.add(metrics_sender, [signal_boost.SIGHUP, signal_boost.SIGQUIT])
    
    vibez.spill("✅ Signal multiplexer configured:")
    vibez.spill("   - Total subscriptions: {}", multiplexer.count())
    vibez.spill("   - UI handle ID: {}", ui_handle.id())
    vibez.spill("   - Logger handle ID: {}", logger_handle.id())
    vibez.spill("   - Metrics handle ID: {}", metrics_handle.id())
    
    fr fr Show monitored signals
    facts monitored = multiplexer.monitored_signals()
    vibez.spill("   - Monitored signals: {:?}", monitored)
    
    fr fr Get statistics
    facts stats = multiplexer.get_statistics()
    vibez.spill("   - Active subscriptions: {}", stats.active_subscriptions)
    vibez.spill("   - Total signals processed: {}", stats.signals_processed)
    
    fr fr In a real application, you would:
    fr fr multiplexer.start()  // Start distributing signals
    fr fr Handle signals in separate goroutines for each channel
    
    fr fr Clean up
    multiplexer.remove_handle(ui_handle)
    multiplexer.remove_handle(logger_handle)
    multiplexer.remove_handle(metrics_handle)
    
    vibez.spill("✅ Signal multiplexing demo completed")
}

slay demo_signal_filtering() tea {
    vibez.spill("\n🔧 Demo 5: Signal Filtering and Throttling")
    
    fr fr Create a signal source channel
    facts (source_sender, source_receiver) = channel()
    
    fr fr Demo filtering
    facts filtered = signal_boost.filter_signals(source_receiver, slay(signal) lit {
        damn signal == signal_boost.SIGINT || signal == signal_boost.SIGTERM
    })
    
    vibez.spill("✅ Set up signal filtering (SIGINT and SIGTERM only)")
    
    fr fr Demo throttling
    facts (throttle_sender, throttle_receiver) = channel()
    facts throttled = signal_boost.throttle_signals(throttle_receiver, time.milliseconds(100))
    
    vibez.spill("✅ Set up signal throttling (100ms interval)")
    
    fr fr Demo debouncing
    facts (debounce_sender, debounce_receiver) = channel()
    facts debounced = signal_boost.debounce_signals(debounce_receiver, time.milliseconds(50))
    
    vibez.spill("✅ Set up signal debouncing (50ms interval)")
    
    fr fr Demo filter chain
    facts (chain_sender, chain_receiver) = channel()
    facts chain = signal_boost.SignalFilterChain.new()
        .add_predicate_filter(slay(signal) lit { signal != signal_boost.SIGKILL })
        .add_deduplicate()
        .add_throttle(time.milliseconds(10))
        .add_rate_limit(5) fr fr 5 signals per second max
    
    facts chain_output = chain.apply(chain_receiver)
    
    vibez.spill("✅ Set up signal filter chain:")
    vibez.spill("   - Filter out SIGKILL")
    vibez.spill("   - Deduplicate consecutive signals")
    vibez.spill("   - Throttle to 10ms intervals")
    vibez.spill("   - Rate limit to 5 signals/second")
    
    fr fr In a real application, you would send signals to these channels
    fr fr and handle the filtered/processed signals
    
    vibez.spill("✅ Signal filtering demo completed")
}

slay demo_genZ_features() tea {
    vibez.spill("\n🔥 Demo 6: GenZ Themed Features")
    
    fr fr Demo 1: VibeChecker
    vibez.spill("✨ Setting up VibeChecker...")
    
    sus vibe_checker = signal_boost.vibe_check(signal_boost.SIGUSR1, slay() lit {
        fr fr Simulate health check
        facts random_health = math.random() > 0.3
        damn random_health
    })
    
    vibez.spill("✅ VibeChecker created for SIGUSR1")
    vibez.spill("   - Running: {}", vibe_checker.is_running())
    vibez.spill("   - Current status: {}", vibe_checker.get_status())
    
    fr fr Start vibe checker
    vibe_checker.start()
    vibez.spill("✅ VibeChecker started - will run health checks on SIGUSR1")
    
    fr fr Demo 2: YeetOnSignal (careful - this would exit!)
    vibez.spill("⚠️  Setting up YeetOnSignal (demo only - won't actually exit)")
    fr fr In real usage: signal_boost.yeet_on_signal(signal_boost.SIGQUIT, "Yeeting out, bruh!")
    vibez.spill("✅ YeetOnSignal would be set up for SIGQUIT")
    
    fr fr Demo 3: NoCapReloadConfig
    vibez.spill("📄 Setting up NoCapReloadConfig...")
    
    facts reload_handle = signal_boost.no_cap_reload_config("app_config.toml", slay() tea {
        vibez.spill("🔄 No cap, reloading configuration...")
        fr fr Simulate config reload
        time.sleep_millis(10)
        damn Ok(())
    })
    
    vibez.spill("✅ NoCapReloadConfig set up for SIGHUP")
    
    fr fr Demo 4: BussinLogger
    vibez.spill("🔥 Setting up BussinLogger...")
    
    sus bussin_logger = signal_boost.BussinLogger.new([
        signal_boost.SIGINT,
        signal_boost.SIGTERM,
        signal_boost.SIGUSR1,
        signal_boost.SIGUSR2
    ])
    
    bussin_logger.start()
    vibez.spill("✅ BussinLogger started - that's gonna be bussin!")
    vibez.spill("   - Running: {}", bussin_logger.is_running())
    
    fr fr Demo 5: SheeshAlarm
    vibez.spill("🚨 Setting up SheeshAlarm...")
    
    facts sheesh_handle = signal_boost.sheesh_alarm([signal_boost.SIGABRT, signal_boost.SIGSEGV])
    vibez.spill("✅ SheeshAlarm armed for critical signals - SHEESH!")
    
    fr fr Demo 6: FrFrReporter
    vibez.spill("📊 Setting up FrFrReporter...")
    
    sus fr_fr_reporter = signal_boost.FrFrReporter.new(
        [signal_boost.SIGINT, signal_boost.SIGTERM, signal_boost.SIGUSR1],
        time.seconds(5)
    )
    
    fr_fr_reporter.start([signal_boost.SIGINT, signal_boost.SIGTERM, signal_boost.SIGUSR1])
    vibez.spill("✅ FrFrReporter started - keeping it 100 with signal stats!")
    
    fr fr Demo 7: ChefKissHandler
    vibez.spill("👨‍🍳💋 Setting up ChefKissHandler...")
    
    facts chef_handle = signal_boost.chef_kiss_handler([signal_boost.SIGUSR2], slay(signal) {
        vibez.spill("👨‍🍳💋 Handling {} with absolute perfection!", signal)
    })
    
    vibez.spill("✅ ChefKissHandler set up - perfection incoming!")
    
    fr fr Show module statistics
    facts module_stats = signal_boost.get_statistics()
    vibez.spill("\n📊 SignalBoost Module Statistics:")
    vibez.spill("   - Active handlers: {}", module_stats.active_handlers)
    vibez.spill("   - Active multiplexers: {}", module_stats.active_multiplexers)
    vibez.spill("   - Graceful shutdowns: {}", module_stats.graceful_shutdowns)
    vibez.spill("   - Signals processed: {}", module_stats.signals_processed)
    
    fr fr Clean up GenZ features
    vibe_checker.stop()
    bussin_logger.stop()
    fr_fr_reporter.stop()
    
    vibez.spill("✅ GenZ features demo completed - it was fire! 🔥")
}

fr fr Utility function to demonstrate signal actions
slay demo_signal_actions() tea {
    vibez.spill("\n⚡ Bonus Demo: Signal Actions")
    
    fr fr Create different types of signal actions
    facts ignore = signal_boost.ignore_action()
    facts log_info = signal_boost.log_action(signal_boost.LogLevel.Info)
    facts shook = signal_boost.shook_action()
    
    fr fr Create custom action
    facts custom_action = signal_boost.create_action(slay(signal) lit {
        vibez.spill("🎯 Custom action triggered by signal: {}", signal)
        damn based
    })
    
    fr fr Create chained actions
    facts chained = signal_boost.chain_actions([ignore, log_info, shook, custom_action])
    
    fr fr Create conditional action
    facts conditional = signal_boost.conditional_action(
        slay(signal) lit { signal == signal_boost.SIGINT },
        custom_action
    )
    
    fr fr Create rate limited action
    facts rate_limited = signal_boost.rate_limited_action(custom_action, time.milliseconds(100))
    
    fr fr Create counting action
    facts (counting, counter) = signal_boost.counting_action(custom_action)
    
    vibez.spill("✅ Signal actions configured:")
    vibez.spill("   - Ignore action")
    vibez.spill("   - Log action (info level)")
    vibez.spill("   - Shook action")
    vibez.spill("   - Custom action")
    vibez.spill("   - Chained actions")
    vibez.spill("   - Conditional action (SIGINT only)")
    vibez.spill("   - Rate limited action (100ms)")
    vibez.spill("   - Counting action")
    
    fr fr Test actions with sample signal
    facts test_signal = signal_boost.SIGINT
    
    vibez.spill("\n🧪 Testing actions with {}:", test_signal)
    vibez.spill("   - Ignore: {}", ignore(test_signal))
    vibez.spill("   - Log: {}", log_info(test_signal))
    vibez.spill("   - Shook: {}", shook(test_signal))
    vibez.spill("   - Custom: {}", custom_action(test_signal))
    vibez.spill("   - Conditional: {}", conditional(test_signal))
    vibez.spill("   - Rate limited: {}", rate_limited(test_signal))
    vibez.spill("   - Counting: {}", counting(test_signal))
    vibez.spill("   - Counter value: {}", counter.load())
    
    vibez.spill("✅ Signal actions demo completed")
}

fr fr Real-world usage example
slay real_world_example() tea {
    vibez.spill("\n🌍 Real-world Example: HTTP Server with Signal Handling")
    
    fr fr Set up comprehensive signal handling for an HTTP server
    
    fr fr 1. Graceful shutdown
    sus shutdown = signal_boost.GracefulShutdown.with_options(signal_boost.ShutdownOptions{
        timeout: time.seconds(30),
        pre_shutdown_fn: slay() {
            vibez.spill("🔄 Starting graceful shutdown...")
        },
        error_handler: slay(err) {
            vibez.spill("❌ Shutdown error: {}", err)
        },
        keep_alive: cap,
        sync_shutdown: based,
        signals: [signal_boost.SIGINT, signal_boost.SIGTERM]
    })
    
    fr fr Add shutdown tasks
    shutdown.add_with_order("stop_accepting_connections", 0, slay() tea {
        vibez.spill("🚫 Stopped accepting new connections")
        damn Ok(())
    })
    
    shutdown.add_with_order("finish_requests", 1, slay() tea {
        vibez.spill("⏳ Waiting for active requests to complete...")
        time.sleep_millis(100) fr fr Simulate request completion
        damn Ok(())
    })
    
    shutdown.add_with_order("close_database", 2, slay() tea {
        vibez.spill("💾 Closing database connections...")
        damn Ok(())
    })
    
    fr fr 2. Configuration reload on SIGHUP
    facts reload_handle = signal_boost.no_cap_reload_config("server.toml", slay() tea {
        vibez.spill("📄 Reloading server configuration...")
        damn Ok(())
    })
    
    fr fr 3. Health checks on SIGUSR1
    sus health_checker = signal_boost.vibe_check(signal_boost.SIGUSR1, slay() lit {
        fr fr Simulate health check
        vibez.spill("🏥 Running health check...")
        damn based fr fr Server is healthy
    })
    health_checker.start()
    
    fr fr 4. Statistics reporting on SIGUSR2
    facts stats_handle = signal_boost.chef_kiss_handler([signal_boost.SIGUSR2], slay(signal) {
        vibez.spill("📊 Server Statistics Report:")
        vibez.spill("   - Uptime: 42 minutes")
        vibez.spill("   - Active connections: 127")
        vibez.spill("   - Requests served: 15,432")
        vibez.spill("   - Memory usage: 245 MB")
    })
    
    fr fr 5. Emergency exit on SIGQUIT (commented out for demo)
    fr fr signal_boost.yeet_on_signal(signal_boost.SIGQUIT, "Emergency exit requested!")
    
    vibez.spill("✅ HTTP Server signal handling configured:")
    vibez.spill("   - SIGINT/SIGTERM: Graceful shutdown")
    vibez.spill("   - SIGHUP: Configuration reload")
    vibez.spill("   - SIGUSR1: Health check")
    vibez.spill("   - SIGUSR2: Statistics report")
    vibez.spill("   - SIGQUIT: Emergency exit (disabled in demo)")
    
    fr fr In a real server, you would:
    fr fr shutdown.start()       // Start listening for shutdown signals
    fr fr start_http_server()    // Start the actual HTTP server
    fr fr shutdown.wait()        // Wait for graceful shutdown
    
    fr fr Clean up
    health_checker.stop()
    
    vibez.spill("✅ Real-world example completed")
}
