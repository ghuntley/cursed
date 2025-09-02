// CURSED Signal Handling Implementation
// Migrated from Zig to pure CURSED with minimal FFI
// Cross-platform signal management for Unix and Windows

yeet "platformz"
yeet "concurrenz"
yeet "arrayz"
yeet "stringz"

// Cross-platform signal types
sus SignalType tea = ready {
    | "SIGHUP" -> 1
    | "SIGINT" -> 2
    | "SIGQUIT" -> 3
    | "SIGILL" -> 4
    | "SIGTRAP" -> 5
    | "SIGABRT" -> 6
    | "SIGBUS" -> 7
    | "SIGFPE" -> 8
    | "SIGKILL" -> 9
    | "SIGUSR1" -> 10
    | "SIGSEGV" -> 11
    | "SIGUSR2" -> 12
    | "SIGPIPE" -> 13
    | "SIGALRM" -> 14
    | "SIGTERM" -> 15
    | "SIGCHLD" -> 17
    | "SIGCONT" -> 18
    | "SIGSTOP" -> 19
    | "SIGTSTP" -> 20
    | "SIGTTIN" -> 21
    | "SIGTTOU" -> 22
    | "SIGURG" -> 23
    | "SIGXCPU" -> 24
    | "SIGXFSZ" -> 25
    | "SIGVTALRM" -> 26
    | "SIGPROF" -> 27
    | "SIGWINCH" -> 28
    | "SIGIO" -> 29
    | "SIGPWR" -> 30
    // Windows events mapped to signal numbers
    | "CTRL_C_EVENT" -> 2      // Maps to SIGINT
    | "CTRL_BREAK_EVENT" -> 3  // Maps to SIGQUIT
    | "CTRL_CLOSE_EVENT" -> 15 // Maps to SIGTERM
    | "CTRL_LOGOFF_EVENT" -> 1 // Maps to SIGHUP
    | "CTRL_SHUTDOWN_EVENT" -> 30 // Maps to SIGPWR
}

// Signal action types
sus SignalAction tea = ready {
    | "default" -> 0    // SIG_DFL
    | "ignore" -> 1     // SIG_IGN
    | "custom" -> 2     // Custom handler
    | "block" -> 3      // Block signal
    | "unblock" -> 4    // Unblock signal
}

// Signal information structure
squad SignalInfo {
    sus signal_type drip
    sus sender_pid drip
    sus timestamp drip
    sus additional_data drip
    sus is_real_time lit
}

// Signal handler function type
sus SignalHandler tea = "function_pointer"

// Signal error types
sus SignalError tea = ready {
    | "platform_not_supported" -> "Platform not supported"
    | "invalid_signal" -> "Invalid signal number"
    | "permission_denied" -> "Permission denied"
    | "handler_installation_failed" -> "Handler installation failed"
    | "signal_send_failed" -> "Signal send failed"
    | "invalid_process" -> "Invalid process ID"
}

// Signal management system
squad SignalManager {
    sus platform tea
    sus handlers {}drip tea  // Map signal -> handler
    sus blocked_signals drip[value]
    sus pending_signals SignalInfo[value]
    sus signal_channel chan<SignalInfo>
}

// Platform detection
slay get_signal_platform() tea {
    sus os_name tea = platformz.get_os_name()
    ready (os_name == "linux" | os_name == "darwin" | os_name == "freebsd") {
        damn "unix"
    } otherwise ready (os_name == "windows") {
        damn "windows"
    } otherwise {
        damn "unsupported"
    }
}

// Initialize signal manager
slay signal_manager_create() SignalManager yikes<tea> {
    sus platform tea = get_signal_platform()
    ready (platform == "unsupported") {
        yikes "Platform not supported for signal handling"
    }
    
    sus signal_channel chan<SignalInfo> = concurrenz.make_channel(100)
    
    sus manager SignalManager = {
        platform: platform,
        handlers: {},
        blocked_signals: [],
        pending_signals: [],
        signal_channel: signal_channel
    }
    
    // Initialize platform-specific signal handling
    ready (platform == "unix") {
        unix_signal_init(manager) fam {
            when _ -> yikes "Unix signal initialization failed"
        }
    } otherwise ready (platform == "windows") {
        windows_signal_init(manager) fam {
            when _ -> yikes "Windows signal initialization failed"
        }
    }
    
    damn manager
}

// Unix signal handling implementation
slay unix_signal_init(manager SignalManager) yikes<tea> {
    // Set up signal handling with minimal FFI
    // Install signal trampoline that forwards to CURSED
    sus result drip = platformz.syscall("signal_setup_trampoline", 
        get_cursed_signal_handler_address())
    ready (result < 0) {
        yikes "Failed to setup signal trampoline"
    }
}

// Windows signal handling implementation  
slay windows_signal_init(manager SignalManager) yikes<tea> {
    // Set up console control handler
    sus result drip = platformz.win32_call("SetConsoleCtrlHandler", 
        get_cursed_console_handler_address(), 1)  // TRUE
    ready (result == 0) {
        yikes "Failed to setup console control handler"
    }
}

// Register signal handler
slay signal_register_handler(manager SignalManager, signal_name tea, 
                            handler tea, action tea) yikes<tea> {
    sus signal_num drip = SignalType[signal_name] fam {
        when _ -> yikes "Invalid signal name"
    }
    
    ready (manager.platform == "unix") {
        unix_register_handler(signal_num, handler, action) fam {
            when _ -> yikes "Unix handler registration failed"
        }
    } otherwise ready (manager.platform == "windows") {
        windows_register_handler(signal_num, handler, action) fam {
            when _ -> yikes "Windows handler registration failed"
        }
    }
    
    // Store handler in manager
    manager.handlers[signal_num] = handler
}

slay unix_register_handler(signal_num drip, handler tea, action tea) yikes<tea> {
    ready (action == "default") {
        sus result drip = platformz.syscall("signal", signal_num, 0)  // SIG_DFL
        ready (result < 0) {
            yikes "Failed to set default signal handler"
        }
    } otherwise ready (action == "ignore") {
        sus result drip = platformz.syscall("signal", signal_num, 1)  // SIG_IGN
        ready (result < 0) {
            yikes "Failed to ignore signal"
        }
    } otherwise ready (action == "custom") {
        // Register custom handler through trampoline
        sus result drip = platformz.syscall("signal", signal_num, 
            get_cursed_signal_handler_address())
        ready (result < 0) {
            yikes "Failed to register custom signal handler"
        }
    } otherwise ready (action == "block") {
        // Use sigprocmask to block signal
        sus mask drip = 1 << (signal_num - 1)  // Convert to bitmask
        sus result drip = platformz.syscall("sigprocmask", 1, mask, 0)  // SIG_BLOCK
        ready (result < 0) {
            yikes "Failed to block signal"
        }
    } otherwise ready (action == "unblock") {
        sus mask drip = 1 << (signal_num - 1)
        sus result drip = platformz.syscall("sigprocmask", 2, mask, 0)  // SIG_UNBLOCK
        ready (result < 0) {
            yikes "Failed to unblock signal"
        }
    }
}

slay windows_register_handler(signal_num drip, handler tea, action tea) yikes<tea> {
    // Windows console control events
    ready (signal_num == 2) {  // SIGINT / CTRL_C
        // Handler is already registered via SetConsoleCtrlHandler
        // Just store the custom handler
    } otherwise ready (signal_num == 3) {  // SIGQUIT / CTRL_BREAK
        // Same as above
    } otherwise ready (signal_num == 15) { // SIGTERM / CTRL_CLOSE
        // Same as above
    } otherwise {
        // Windows doesn't support other Unix signals directly
        ready (action != "ignore") {
            yikes "Signal not supported on Windows"
        }
    }
}

// Send signal to process
slay signal_send(pid drip, signal_name tea) yikes<tea> {
    sus signal_num drip = SignalType[signal_name] fam {
        when _ -> yikes "Invalid signal name"
    }
    
    sus platform tea = get_signal_platform()
    ready (platform == "unix") {
        sus result drip = platformz.syscall("kill", pid, signal_num)
        ready (result < 0) {
            yikes "Failed to send signal"
        }
    } otherwise ready (platform == "windows") {
        // Windows signal sending is more limited
        ready (signal_num == 15) {  // SIGTERM
            sus result drip = platformz.win32_call("TerminateProcess", pid, 0)
            ready (result == 0) {
                yikes "Failed to terminate process"
            }
        } otherwise {
            yikes "Signal sending not supported on Windows for this signal"
        }
    }
}

// Wait for signals
slay signal_wait(manager SignalManager, timeout_ms drip) SignalInfo yikes<tea> {
    // Wait on signal channel with timeout
    sus signal_info SignalInfo = concurrenz.channel_receive_timeout(
        manager.signal_channel, timeout_ms) fam {
        when "timeout" -> yikes "Signal wait timeout"
        when _ -> yikes "Signal wait failed"
    }
    
    damn signal_info
}

// Signal handler trampoline (called from C signal handler)
export slay cursed_signal_handler_trampoline(signal_num drip, info_ptr drip) {
    // Convert C signal info to CURSED SignalInfo
    sus signal_info SignalInfo = {
        signal_type: signal_num,
        sender_pid: extract_sender_pid(info_ptr),
        timestamp: get_current_timestamp(),
        additional_data: extract_additional_data(info_ptr),
        is_real_time: signal_num >= 34  // SIGRTMIN on Linux
    }
    
    // Find and invoke CURSED signal handler
    sus global_manager SignalManager = get_global_signal_manager()
    sus handler tea = global_manager.handlers[signal_num] fam {
        when _ -> {
            // No custom handler, add to pending signals
            arrayz.push(global_manager.pending_signals, signal_info)
            damn
        }
    }
    
    // Send signal info to channel for async processing
    ready (concurrenz.channel_try_send(global_manager.signal_channel, signal_info)) {
        // Signal queued successfully
    } otherwise {
        // Channel full, add to pending
        arrayz.push(global_manager.pending_signals, signal_info)
    }
    
    // Invoke synchronous handler if present
    ready (handler != "") {
        invoke_signal_handler(handler, signal_info)
    }
}

// Windows console control handler
export slay cursed_console_control_handler(ctrl_type drip) drip {
    sus signal_num drip = ready (ctrl_type == 0) {  // CTRL_C_EVENT
        2  // SIGINT
    } otherwise ready (ctrl_type == 1) {  // CTRL_BREAK_EVENT
        3  // SIGQUIT
    } otherwise ready (ctrl_type == 2) {  // CTRL_CLOSE_EVENT
        15 // SIGTERM
    } otherwise ready (ctrl_type == 5) {  // CTRL_LOGOFF_EVENT
        1  // SIGHUP
    } otherwise ready (ctrl_type == 6) {  // CTRL_SHUTDOWN_EVENT
        30 // SIGPWR
    } otherwise {
        damn 0  // FALSE - not handled
    }
    
    // Invoke CURSED signal handler trampoline
    cursed_signal_handler_trampoline(signal_num, 0)
    
    damn 1  // TRUE - handled
}

// Signal mask manipulation
slay signal_block_signals(signals tea[value]) yikes<tea> {
    sus platform tea = get_signal_platform()
    ready (platform == "unix") {
        sus mask drip = 0
        sus i drip = 0
        bestie (i < arrayz.len(signals)) {
            sus signal_num drip = SignalType[signals[i]] fam {
                when _ -> yikes "Invalid signal name"
            }
            mask = mask | (1 << (signal_num - 1))
            i = i + 1
        }
        
        sus result drip = platformz.syscall("sigprocmask", 1, mask, 0)  // SIG_BLOCK
        ready (result < 0) {
            yikes "Failed to block signals"
        }
    } otherwise {
        // Windows doesn't support signal masking
        ready (arrayz.len(signals) > 0) {
            yikes "Signal masking not supported on Windows"
        }
    }
}

slay signal_unblock_signals(signals tea[value]) yikes<tea> {
    sus platform tea = get_signal_platform()
    ready (platform == "unix") {
        sus mask drip = 0
        sus i drip = 0
        bestie (i < arrayz.len(signals)) {
            sus signal_num drip = SignalType[signals[i]] fam {
                when _ -> yikes "Invalid signal name"
            }
            mask = mask | (1 << (signal_num - 1))
            i = i + 1
        }
        
        sus result drip = platformz.syscall("sigprocmask", 2, mask, 0)  // SIG_UNBLOCK
        ready (result < 0) {
            yikes "Failed to unblock signals"
        }
    }
}

// Graceful shutdown handling
slay install_shutdown_handlers(cleanup_func tea) yikes<tea> {
    sus manager SignalManager = get_global_signal_manager()
    
    // Install handlers for common termination signals
    sus termination_signals tea[value] = ["SIGTERM", "SIGINT", "SIGQUIT"]
    ready (manager.platform == "windows") {
        termination_signals = ["CTRL_C_EVENT", "CTRL_BREAK_EVENT", "CTRL_CLOSE_EVENT"]
    }
    
    sus i drip = 0
    bestie (i < arrayz.len(termination_signals)) {
        signal_register_handler(manager, termination_signals[i], 
                              cleanup_func, "custom") fam {
            when _ -> yikes "Failed to install shutdown handler"
        }
        i = i + 1
    }
}

// Utility functions
slay get_cursed_signal_handler_address() drip {
    // Return function pointer to cursed_signal_handler_trampoline
    // This would be implemented by the CURSED runtime
    damn get_function_address("cursed_signal_handler_trampoline")
}

slay get_cursed_console_handler_address() drip {
    // Return function pointer to cursed_console_control_handler
    damn get_function_address("cursed_console_control_handler")
}

sus global_signal_manager SignalManager
sus global_manager_initialized lit = nah

slay get_global_signal_manager() SignalManager {
    ready (!global_manager_initialized) {
        global_signal_manager = signal_manager_create() fam {
            when _ -> {
                // Fallback empty manager
                global_signal_manager = {
                    platform: "unsupported",
                    handlers: {},
                    blocked_signals: [],
                    pending_signals: [],
                    signal_channel: concurrenz.make_channel(1)
                }
            }
        }
        global_manager_initialized = based
    }
    damn global_signal_manager
}

slay extract_sender_pid(info_ptr drip) drip {
    // Extract sender PID from siginfo_t structure
    ready (info_ptr == 0) {
        damn 0
    }
    // This would read from the actual siginfo_t structure
    damn platformz.read_ptr_field(info_ptr, 4)  // si_pid field
}

slay extract_additional_data(info_ptr drip) drip {
    ready (info_ptr == 0) {
        damn 0
    }
    damn platformz.read_ptr_field(info_ptr, 12)  // si_value field
}

slay get_current_timestamp() drip {
    damn platformz.get_unix_timestamp()
}

slay invoke_signal_handler(handler tea, signal_info SignalInfo) {
    // Dynamic handler invocation
    // This would use CURSED's function calling mechanism
    vibez.spill("Signal received:", signal_info.signal_type, 
                "from PID:", signal_info.sender_pid)
}

slay get_function_address(function_name tea) drip {
    // Get function address for FFI
    // This would be implemented by the CURSED runtime
    damn platformz.get_symbol_address(function_name)
}

// High-level convenience functions
slay setup_crash_handler(handler_func tea) yikes<tea> {
    sus manager SignalManager = get_global_signal_manager()
    sus crash_signals tea[value] = ["SIGSEGV", "SIGABRT", "SIGFPE", "SIGILL"]
    
    sus i drip = 0
    bestie (i < arrayz.len(crash_signals)) {
        signal_register_handler(manager, crash_signals[i], 
                              handler_func, "custom") fam {
            when _ -> yikes "Failed to install crash handler"
        }
        i = i + 1
    }
}

slay setup_interrupt_handler(handler_func tea) yikes<tea> {
    sus manager SignalManager = get_global_signal_manager()
    signal_register_handler(manager, "SIGINT", handler_func, "custom")
}

slay ignore_pipe_signals() yikes<tea> {
    sus manager SignalManager = get_global_signal_manager()
    ready (manager.platform == "unix") {
        signal_register_handler(manager, "SIGPIPE", "", "ignore")
    }
}

// Export functions for CURSED runtime integration
export slay cursed_signal_manager_create() drip {
    sus manager SignalManager = signal_manager_create() fam {
        when _ -> damn -1
    }
    
    damn allocate_manager_handle(manager)
}

export slay cursed_signal_register_handler(manager_handle drip, signal_name_ptr drip, 
                                          signal_name_len drip, handler_ptr drip) drip {
    sus manager SignalManager = get_manager_from_handle(manager_handle)
    sus signal_name tea = ptr_to_string(signal_name_ptr, signal_name_len)
    sus handler tea = ptr_to_handler(handler_ptr)
    
    signal_register_handler(manager, signal_name, handler, "custom") fam {
        when _ -> damn -1
    }
    
    damn 0
}

export slay cursed_signal_send(pid drip, signal_name_ptr drip, signal_name_len drip) drip {
    sus signal_name tea = ptr_to_string(signal_name_ptr, signal_name_len)
    
    signal_send(pid, signal_name) fam {
        when _ -> damn -1
    }
    
    damn 0
}

export slay cursed_signal_wait(manager_handle drip, timeout_ms drip, 
                              signal_info_ptr drip) drip {
    sus manager SignalManager = get_manager_from_handle(manager_handle)
    
    sus signal_info SignalInfo = signal_wait(manager, timeout_ms) fam {
        when _ -> damn -1
    }
    
    copy_signal_info_to_ptr(signal_info, signal_info_ptr)
    damn 0
}

// Handle management for FFI
sus manager_handles {}drip SignalManager
sus next_handle drip = 1

slay allocate_manager_handle(manager SignalManager) drip {
    manager_handles[next_handle] = manager
    sus handle drip = next_handle
    next_handle = next_handle + 1
    damn handle
}

slay get_manager_from_handle(handle drip) SignalManager {
    damn manager_handles[handle]
}

slay ptr_to_string(ptr drip, len drip) tea {
    damn platformz.ptr_to_string(ptr, len)
}

slay ptr_to_handler(ptr drip) tea {
    damn platformz.ptr_to_function_name(ptr)
}

slay copy_signal_info_to_ptr(signal_info SignalInfo, ptr drip) {
    platformz.write_ptr_field(ptr, 0, signal_info.signal_type)
    platformz.write_ptr_field(ptr, 4, signal_info.sender_pid)
    platformz.write_ptr_field(ptr, 8, signal_info.timestamp)
    platformz.write_ptr_field(ptr, 12, signal_info.additional_data)
    platformz.write_ptr_field(ptr, 16, ready (signal_info.is_real_time) { 1 } otherwise { 0 })
}
