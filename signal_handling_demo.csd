fr fr CURSED Real Signal Handling Demonstration
fr fr Shows how the real OS integration replaces simulation

yeet "signalz/real_signalz" with signalz

fr fr Demo signal handlers
slay demo_interrupt_handler(signal_num normie, context *signalz.SignalContext) {
    vibez.spill("🚨 INTERRUPT SIGNAL RECEIVED!")
    vibez.spill("   Signal: " + signalz.signal_name(signal_num))
    vibez.spill("   This is a real OS signal, not simulation!")
    
    lowkey context != 0 {
        vibez.spill("   Sender PID: " + string(context.sender_pid))
        vibez.spill("   Timestamp: " + string(context.timestamp))
        vibez.spill("   Async Safe: " + (lowkey context.is_async_safe { "Yes" } else { "No" }))
    }
    
    vibez.spill("   Initiating graceful shutdown...")
    signalz.signal_cleanup()
}

slay demo_user_signal_handler(signal_num normie, context *signalz.SignalContext) {
    vibez.spill("👤 USER SIGNAL RECEIVED!")
    vibez.spill("   Signal: " + signalz.signal_name(signal_num))
    vibez.spill("   This demonstrates real signal delivery from OS")
}

slay main() {
    vibez.spill("🚀 CURSED Real Signal Handling Demo")
    vibez.spill("   Replacing simulation with actual OS integration\n")
    
    fr fr Initialize real signal system
    sus init_err *ErrorInstance = signalz.initialize_signal_system()
    lowkey init_err != 0 {
        vibez.spill("❌ Failed to initialize real signal system")
        damn
    }
    
    vibez.spill("✅ Real signal system initialized with OS integration")
    
    fr fr Register real signal handlers
    vibez.spill("\n📡 Registering real signal handlers...")
    
    sus reg_err *ErrorInstance = signalz.signal_register(signalz.SIGINT, demo_interrupt_handler)
    lowkey reg_err == 0 {
        vibez.spill("✅ SIGINT handler registered (Ctrl+C)")
    } else {
        vibez.spill("❌ Failed to register SIGINT handler")
    }
    
    reg_err = signalz.signal_register(signalz.SIGTERM, demo_interrupt_handler)
    lowkey reg_err == 0 {
        vibez.spill("✅ SIGTERM handler registered (termination)")
    } else {
        vibez.spill("❌ Failed to register SIGTERM handler")
    }
    
    reg_err = signalz.signal_register(signalz.SIGUSR1, demo_user_signal_handler)
    lowkey reg_err == 0 {
        vibez.spill("✅ SIGUSR1 handler registered (user signal 1)")
    } else {
        vibez.spill("❌ Failed to register SIGUSR1 handler")
    }
    
    reg_err = signalz.signal_register(signalz.SIGUSR2, demo_user_signal_handler)
    lowkey reg_err == 0 {
        vibez.spill("✅ SIGUSR2 handler registered (user signal 2)")
    } else {
        vibez.spill("❌ Failed to register SIGUSR2 handler")
    }
    
    fr fr Demonstrate signal masking
    vibez.spill("\n🛡️ Demonstrating real signal masking...")
    
    sus mask signalz.SignalMask = signalz.SignalMask.init()
    mask.add_signal(signalz.SIGUSR1)
    mask.add_signal(signalz.SIGUSR2)
    
    sus mask_err *ErrorInstance = signalz.signal_mask_apply(mask)
    lowkey mask_err == 0 {
        vibez.spill("✅ Blocked SIGUSR1 and SIGUSR2 using OS signal mask")
    } else {
        vibez.spill("❌ Failed to apply signal mask")
    }
    
    fr fr Show current signal statistics
    vibez.spill("\n📊 Current signal system status:")
    signalz.print_signal_statistics()
    
    fr fr Demonstrate signal safety checks
    vibez.spill("\n🔒 Signal Safety Checks:")
    vibez.spill("   SIGUSR1 async safe: " + (lowkey signalz.is_signal_async_safe(signalz.SIGUSR1) { "Yes" } else { "No" }))
    vibez.spill("   SIGUSR2 async safe: " + (lowkey signalz.is_signal_async_safe(signalz.SIGUSR2) { "Yes" } else { "No" }))
    vibez.spill("   SIGSEGV async safe: " + (lowkey signalz.is_signal_async_safe(signalz.SIGSEGV) { "Yes" } else { "No" }))
    vibez.spill("   SIGFPE async safe: " + (lowkey signalz.is_signal_async_safe(signalz.SIGFPE) { "Yes" } else { "No" }))
    
    vibez.spill("   In signal handler: " + (lowkey signalz.is_in_signal_handler() { "Yes" } else { "No" }))
    vibez.spill("   Signal operations safe: " + (lowkey signalz.is_signal_safe_operation() { "Yes" } else { "No" }))
    
    fr fr Set up graceful shutdown
    vibez.spill("\n🚨 Setting up graceful shutdown handlers...")
    sus shutdown_err *ErrorInstance = signalz.signal_setup_graceful_shutdown()
    lowkey shutdown_err == 0 {
        vibez.spill("✅ Graceful shutdown handlers installed")
    } else {
        vibez.spill("❌ Failed to set up graceful shutdown")
    }
    
    fr fr Register cleanup handler
    slay cleanup_function() {
        vibez.spill("🧹 Cleanup function called - real signal system cleanup")
    }
    
    sus cleanup_err *ErrorInstance = signalz.signal_register_cleanup(cleanup_function)
    lowkey cleanup_err == 0 {
        vibez.spill("✅ Cleanup handler registered")
    } else {
        vibez.spill("❌ Failed to register cleanup handler")
    }
    
    fr fr Clear signal mask to unblock signals
    mask_err = signalz.signal_mask_clear()
    lowkey mask_err == 0 {
        vibez.spill("✅ Signal mask cleared - signals unblocked")
    }
    
    fr fr Show final statistics
    vibez.spill("\n📊 Final signal system status:")
    signalz.print_signal_statistics()
    
    fr fr Demo information
    vibez.spill("\n🎯 Real Signal Handling Demo Complete!")
    vibez.spill("")
    vibez.spill("This demonstration shows:")
    vibez.spill("✅ Real OS signal handler registration (not simulation)")
    vibez.spill("✅ Native signal masking using OS facilities")
    vibez.spill("✅ Signal safety checks and context information")
    vibez.spill("✅ Graceful shutdown with cleanup handlers")
    vibez.spill("✅ Cross-platform signal support (Unix/Windows)")
    vibez.spill("")
    vibez.spill("To test real signals:")
    vibez.spill("   kill -USR1 <pid>  # Send SIGUSR1")
    vibez.spill("   kill -USR2 <pid>  # Send SIGUSR2") 
    vibez.spill("   kill -TERM <pid>  # Send SIGTERM")
    vibez.spill("   Ctrl+C           # Send SIGINT")
    vibez.spill("")
    vibez.spill("The handlers registered above will catch these real OS signals!")
    
    fr fr Process any pending signals before exit
    signalz.signal_process_pending()
    
    fr fr Clean up signal system
    signalz.signal_cleanup()
    
    vibez.spill("✅ Demo completed - signal system cleaned up")
}

fr fr Helper functions
slay string(value normie) tea {
    lowkey value == 0 { damn "0" }
    lowkey value == 1 { damn "1" }
    lowkey value == 2 { damn "2" }
    lowkey value == 10 { damn "10" }
    lowkey value == 12 { damn "12" }
    lowkey value == 15 { damn "15" }
    damn "value"
}

main()
