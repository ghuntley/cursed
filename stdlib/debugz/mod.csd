fr fr debugz module - Debug utilities, stack traces, profiling
fr fr Essential debugging tools for CURSED runtime and development

yeet "typez"
yeet "memoryz"

fr fr ===== DEBUG CONFIGURATION =====

squad DebugConfig {
    spill enabled lit
    spill log_level normie
    spill stack_trace_enabled lit
    spill memory_tracking lit
    spill performance_profiling lit
    spill breakpoints_enabled lit
    spill verbose_logging lit
}

squad StackFrame {
    spill function_name tea
    spill file_name tea
    spill line_number normie
    spill instruction_pointer normie
    spill local_variables []VariableInfo
}

squad VariableInfo {
    spill name tea
    spill type_info typez.TypeInfo
    spill value normie
    spill address normie
    spill scope tea
}

squad ProfilerResult {
    spill function_name tea
    spill call_count normie
    spill total_time_ns normie
    spill average_time_ns normie
    spill min_time_ns normie
    spill max_time_ns normie
    spill memory_allocated normie
}

squad Breakpoint {
    spill id normie
    spill file_name tea
    spill line_number normie
    spill condition tea
    spill hit_count normie
    spill enabled lit
}

fr fr Debug levels
sus DEBUG_LEVEL_ERROR normie = 1
sus DEBUG_LEVEL_WARN normie = 2
sus DEBUG_LEVEL_INFO normie = 3
sus DEBUG_LEVEL_DEBUG normie = 4
sus DEBUG_LEVEL_TRACE normie = 5

fr fr Global debug state
sus debug_config DebugConfig = DebugConfig{
    enabled: based,
    log_level: DEBUG_LEVEL_INFO,
    stack_trace_enabled: based,
    memory_tracking: based,
    performance_profiling: cap,
    breakpoints_enabled: cap,
    verbose_logging: cap
}

sus call_stack []StackFrame = []
sus profiler_data []ProfilerResult = []
sus active_breakpoints []Breakpoint = []
sus breakpoint_id_counter normie = 1
sus debug_log_buffer []tea = []

fr fr ===== CORE DEBUG FUNCTIONS =====

slay configure_debug(config DebugConfig) lit {
    debug_config = config
    lowkey config.enabled {
        log_info("Debug system enabled")
    }
    damn based
}

slay get_debug_config() DebugConfig {
    damn debug_config
}

slay is_debug_enabled() lit {
    damn debug_config.enabled
}

slay set_debug_level(level normie) lit {
    debug_config.log_level = level
    log_info("Debug level set to: ", level)
    damn based
}

fr fr ===== LOGGING FUNCTIONS =====

slay log_error(message tea, args ...tea) lit {
    lowkey debug_config.enabled && debug_config.log_level >= DEBUG_LEVEL_ERROR {
        sus formatted tea = format_log_message("ERROR", message, args...)
        output_log(formatted)
    }
    damn based
}

slay log_warn(message tea, args ...tea) lit {
    lowkey debug_config.enabled && debug_config.log_level >= DEBUG_LEVEL_WARN {
        sus formatted tea = format_log_message("WARN", message, args...)
        output_log(formatted)
    }
    damn based
}

slay log_info(message tea, args ...tea) lit {
    lowkey debug_config.enabled && debug_config.log_level >= DEBUG_LEVEL_INFO {
        sus formatted tea = format_log_message("INFO", message, args...)
        output_log(formatted)
    }
    damn based
}

slay log_debug(message tea, args ...tea) lit {
    lowkey debug_config.enabled && debug_config.log_level >= DEBUG_LEVEL_DEBUG {
        sus formatted tea = format_log_message("DEBUG", message, args...)
        output_log(formatted)
    }
    damn based
}

slay log_trace(message tea, args ...tea) lit {
    lowkey debug_config.enabled && debug_config.log_level >= DEBUG_LEVEL_TRACE {
        sus formatted tea = format_log_message("TRACE", message, args...)
        output_log(formatted)
    }
    damn based
}

slay format_log_message(level tea, message tea, args ...tea) tea {
    sus timestamp tea = get_current_timestamp()
    sus thread_id normie = get_current_thread_id()
    sus formatted_args tea = format_args(args...)
    damn vibez.spillstr("[%s] [%s] [T:%d] %s %s", timestamp, level, thread_id, message, formatted_args)
}

slay format_args(args ...tea) tea {
    lowkey args.len() == 0 {
        damn ""
    }
    
    sus result tea = ""
    bestie i := 0; i < args.len(); i++ {
        lowkey i > 0 {
            result = result + " "
        }
        result = result + args[i]
    }
    damn result
}

slay output_log(message tea) lit {
    lowkey debug_config.verbose_logging {
        vibez.spill(message)
    }
    debug_log_buffer.push(message)
    
    fr fr Keep buffer size manageable
    lowkey debug_log_buffer.len() > 1000 {
        debug_log_buffer = debug_log_buffer[500:]  fr fr Keep last 500 entries
    }
    damn based
}

fr fr ===== STACK TRACE FUNCTIONS =====

slay push_stack_frame(function_name tea, file_name tea, line_number normie) lit {
    lowkey !debug_config.stack_trace_enabled {
        damn cap
    }
    
    sus frame StackFrame = StackFrame{
        function_name: function_name,
        file_name: file_name,
        line_number: line_number,
        instruction_pointer: get_instruction_pointer(),
        local_variables: []
    }
    
    call_stack.push(frame)
    log_trace("Entering function: ", function_name, " at ", file_name, ":", line_number)
    damn based
}

slay pop_stack_frame() lit {
    lowkey !debug_config.stack_trace_enabled || call_stack.len() == 0 {
        damn cap
    }
    
    sus frame StackFrame = call_stack[call_stack.len() - 1]
    call_stack.pop()
    log_trace("Exiting function: ", frame.function_name)
    damn based
}

slay get_stack_trace() []StackFrame {
    damn call_stack
}

slay print_stack_trace() lit {
    lowkey call_stack.len() == 0 {
        vibez.spill("No stack trace available")
        damn based
    }
    
    vibez.spill("🔍 Stack Trace")
    vibez.spill("═══════════════")
    
    bestie i := call_stack.len() - 1; i >= 0; i-- {
        sus frame StackFrame = call_stack[i]
        vibez.spill("  ", (call_stack.len() - i - 1), ": ", frame.function_name)
        vibez.spill("     at ", frame.file_name, ":", frame.line_number)
        
        lowkey frame.local_variables.len() > 0 {
            vibez.spill("     locals:")
            bestie variable in frame.local_variables {
                vibez.spill("       ", variable.name, ": ", variable.type_info.name, " = ", variable.value)
            }
        }
    }
    damn based
}

slay get_current_function() tea {
    lowkey call_stack.len() > 0 {
        damn call_stack[call_stack.len() - 1].function_name
    }
    damn "unknown"
}

slay get_call_depth() normie {
    damn call_stack.len()
}

fr fr ===== VARIABLE INSPECTION =====

slay add_local_variable(name tea, type_info typez.TypeInfo, value normie, address normie) lit {
    lowkey call_stack.len() == 0 {
        damn cap
    }
    
    sus variable VariableInfo = VariableInfo{
        name: name,
        type_info: type_info,
        value: value,
        address: address,
        scope: "local"
    }
    
    call_stack[call_stack.len() - 1].local_variables.push(variable)
    log_trace("Added local variable: ", name, " = ", value)
    damn based
}

slay inspect_variable(name tea) VariableInfo {
    fr fr Find variable in current scope
    lowkey call_stack.len() > 0 {
        sus current_frame StackFrame = call_stack[call_stack.len() - 1]
        bestie variable in current_frame.local_variables {
            lowkey variable.name == name {
                damn variable
            }
        }
    }
    
    fr fr Return empty variable if not found
    damn VariableInfo{
        name: "not_found",
        type_info: typez.get_type_by_id(0),
        value: 0,
        address: 0,
        scope: "unknown"
    }
}

slay print_local_variables() lit {
    lowkey call_stack.len() == 0 {
        vibez.spill("No active function")
        damn based
    }
    
    sus current_frame StackFrame = call_stack[call_stack.len() - 1]
    vibez.spill("📝 Local Variables in ", current_frame.function_name)
    vibez.spill("══════════════════════════════════")
    
    lowkey current_frame.local_variables.len() == 0 {
        vibez.spill("No local variables")
        damn based
    }
    
    bestie variable in current_frame.local_variables {
        vibez.spill("  ", variable.name, ": ", variable.type_info.name)
        vibez.spill("    Value: ", variable.value)
        vibez.spill("    Address: 0x", format_hex(variable.address))
        vibez.spill("    Scope: ", variable.scope)
        vibez.spill("")
    }
    damn based
}

fr fr ===== PROFILING FUNCTIONS =====

slay start_profiling(function_name tea) lit {
    lowkey !debug_config.performance_profiling {
        damn cap
    }
    
    fr fr Find or create profiler entry
    bestie i := 0; i < profiler_data.len(); i++ {
        lowkey profiler_data[i].function_name == function_name {
            profiler_data[i].call_count = profiler_data[i].call_count + 1
            damn based
        }
    }
    
    fr fr Create new profiler entry
    sus result ProfilerResult = ProfilerResult{
        function_name: function_name,
        call_count: 1,
        total_time_ns: 0,
        average_time_ns: 0,
        min_time_ns: 999999999,
        max_time_ns: 0,
        memory_allocated: 0
    }
    
    profiler_data.push(result)
    log_trace("Started profiling: ", function_name)
    damn based
}

slay end_profiling(function_name tea, execution_time_ns normie) lit {
    lowkey !debug_config.performance_profiling {
        damn cap
    }
    
    bestie i := 0; i < profiler_data.len(); i++ {
        lowkey profiler_data[i].function_name == function_name {
            sus old_result ProfilerResult = profiler_data[i]
            
            old_result.total_time_ns = old_result.total_time_ns + execution_time_ns
            old_result.average_time_ns = old_result.total_time_ns / old_result.call_count
            
            lowkey execution_time_ns < old_result.min_time_ns {
                old_result.min_time_ns = execution_time_ns
            }
            
            lowkey execution_time_ns > old_result.max_time_ns {
                old_result.max_time_ns = execution_time_ns
            }
            
            profiler_data[i] = old_result
            log_trace("Ended profiling: ", function_name, " (", execution_time_ns, "ns)")
            damn based
        }
    }
    
    damn cap
}

slay get_profiler_results() []ProfilerResult {
    damn profiler_data
}

slay print_profiler_report() lit {
    lowkey profiler_data.len() == 0 {
        vibez.spill("No profiling data available")
        damn based
    }
    
    vibez.spill("⏱️ Performance Profile Report")
    vibez.spill("═══════════════════════════════")
    vibez.spill("Function                 | Calls | Total (ms) | Avg (µs) | Min (µs) | Max (µs)")
    vibez.spill("────────────────────────────────────────────────────────────────────────────")
    
    bestie result in profiler_data {
        sus total_ms meal = result.total_time_ns / 1000000.0
        sus avg_us meal = result.average_time_ns / 1000.0
        sus min_us meal = result.min_time_ns / 1000.0
        sus max_us meal = result.max_time_ns / 1000.0
        
        vibez.spillf("%-24s | %5d | %10.2f | %8.2f | %8.2f | %8.2f",
                     result.function_name, result.call_count, total_ms, avg_us, min_us, max_us)
    }
    damn based
}

slay reset_profiler() lit {
    profiler_data = []
    log_info("Profiler data reset")
    damn based
}

fr fr ===== BREAKPOINT FUNCTIONS =====

slay add_breakpoint(file_name tea, line_number normie, condition tea) normie {
    sus breakpoint Breakpoint = Breakpoint{
        id: breakpoint_id_counter,
        file_name: file_name,
        line_number: line_number,
        condition: condition,
        hit_count: 0,
        enabled: based
    }
    
    active_breakpoints.push(breakpoint)
    breakpoint_id_counter = breakpoint_id_counter + 1
    
    log_info("Breakpoint added: ", file_name, ":", line_number)
    damn breakpoint.id
}

slay remove_breakpoint(breakpoint_id normie) lit {
    bestie i := 0; i < active_breakpoints.len(); i++ {
        lowkey active_breakpoints[i].id == breakpoint_id {
            sus bp Breakpoint = active_breakpoints[i]
            active_breakpoints.remove(i)
            log_info("Breakpoint removed: ", bp.file_name, ":", bp.line_number)
            damn based
        }
    }
    damn cap
}

slay enable_breakpoint(breakpoint_id normie) lit {
    bestie i := 0; i < active_breakpoints.len(); i++ {
        lowkey active_breakpoints[i].id == breakpoint_id {
            active_breakpoints[i].enabled = based
            log_info("Breakpoint enabled: ", breakpoint_id)
            damn based
        }
    }
    damn cap
}

slay disable_breakpoint(breakpoint_id normie) lit {
    bestie i := 0; i < active_breakpoints.len(); i++ {
        lowkey active_breakpoints[i].id == breakpoint_id {
            active_breakpoints[i].enabled = cap
            log_info("Breakpoint disabled: ", breakpoint_id)
            damn based
        }
    }
    damn cap
}

slay check_breakpoint(file_name tea, line_number normie) lit {
    lowkey !debug_config.breakpoints_enabled {
        damn cap
    }
    
    bestie breakpoint in active_breakpoints {
        lowkey breakpoint.enabled && 
              breakpoint.file_name == file_name && 
              breakpoint.line_number == line_number {
            
            breakpoint.hit_count = breakpoint.hit_count + 1
            log_info("Breakpoint hit: ", file_name, ":", line_number, " (count: ", breakpoint.hit_count, ")")
            
            fr fr Evaluate condition if present
            lowkey breakpoint.condition != "" {
                lowkey !evaluate_breakpoint_condition(breakpoint.condition) {
                    damn cap  fr fr Condition not met
                }
            }
            
            damn based  fr fr Breakpoint triggered
        }
    }
    
    damn cap
}

slay evaluate_breakpoint_condition(condition tea) lit {
    fr fr Simplified condition evaluation
    fr fr Real implementation would parse and evaluate expressions
    lowkey condition == "true" || condition == "based" {
        damn based
    }
    lowkey condition == "false" || condition == "cap" {
        damn cap
    }
    damn based  fr fr Default to true for unknown conditions
}

slay list_breakpoints() lit {
    vibez.spill("🎯 Active Breakpoints")
    vibez.spill("════════════════════")
    
    lowkey active_breakpoints.len() == 0 {
        vibez.spill("No breakpoints set")
        damn based
    }
    
    bestie breakpoint in active_breakpoints {
        sus status tea = "enabled"
        lowkey !breakpoint.enabled {
            status = "disabled"
        }
        
        vibez.spill("  ", breakpoint.id, ": ", breakpoint.file_name, ":", breakpoint.line_number)
        vibez.spill("     Status: ", status, ", Hits: ", breakpoint.hit_count)
        lowkey breakpoint.condition != "" {
            vibez.spill("     Condition: ", breakpoint.condition)
        }
        vibez.spill("")
    }
    damn based
}

fr fr ===== ASSERTION FUNCTIONS =====

slay debug_assert(condition lit, message tea) lit {
    lowkey !condition {
        log_error("ASSERTION FAILED: ", message)
        print_stack_trace()
        
        lowkey debug_config.breakpoints_enabled {
            vibez.spill("💥 Assertion failure - entering debug mode")
            enter_debug_mode()
        }
        
        damn cap
    }
    damn based
}

slay debug_assert_eq(actual normie, expected normie, message tea) lit {
    lowkey actual != expected {
        log_error("ASSERTION FAILED: ", message, " - Expected: ", expected, ", Got: ", actual)
        print_stack_trace()
        damn cap
    }
    damn based
}

slay debug_assert_not_null(ptr normie, message tea) lit {
    lowkey ptr == 0 {
        log_error("ASSERTION FAILED: ", message, " - Pointer is null")
        print_stack_trace()
        damn cap
    }
    damn based
}

fr fr ===== MEMORY DEBUGGING =====

slay track_memory_allocation(ptr normie, size normie, type_name tea) lit {
    lowkey debug_config.memory_tracking {
        log_debug("Memory allocated: ", size, " bytes at 0x", format_hex(ptr), " for ", type_name)
    }
    damn based
}

slay track_memory_deallocation(ptr normie, size normie) lit {
    lowkey debug_config.memory_tracking {
        log_debug("Memory freed: ", size, " bytes at 0x", format_hex(ptr))
    }
    damn based
}

slay check_memory_corruption(ptr normie, size normie) lit {
    fr fr Basic memory corruption check
    lowkey ptr == 0 {
        log_error("Null pointer access detected")
        damn cap
    }
    
    fr fr Check if pointer is in valid range (simplified)
    lowkey !memoryz.is_valid_pointer(ptr) {
        log_error("Invalid pointer access: 0x", format_hex(ptr))
        damn cap
    }
    
    damn based
}

fr fr ===== DEBUG MODE INTERACTION =====

slay enter_debug_mode() lit {
    vibez.spill("🐛 Entering debug mode")
    vibez.spill("Available commands: help, stack, vars, continue, quit")
    
    bestie based {
        vibez.spill("(debug) ", "")
        sus command tea = vibez.scanln()
        
        lowkey command == "help" {
            print_debug_help()
        } highkey command == "stack" {
            print_stack_trace()
        } highkey command == "vars" {
            print_local_variables()
        } highkey command == "continue" {
            vibez.spill("Continuing execution...")
            ghosted
        } highkey command == "quit" {
            vibez.spill("Exiting debug mode")
            damn based
        } highkey {
            vibez.spill("Unknown command. Type 'help' for available commands.")
        }
    }
    
    damn based
}

slay print_debug_help() lit {
    vibez.spill("Debug Commands:")
    vibez.spill("  help     - Show this help")
    vibez.spill("  stack    - Print stack trace")
    vibez.spill("  vars     - Print local variables")
    vibez.spill("  continue - Continue execution")
    vibez.spill("  quit     - Exit debug mode")
    damn based
}

fr fr ===== UTILITY FUNCTIONS =====

slay get_current_timestamp() tea {
    fr fr Get current timestamp for logging
    damn "2025-07-22T10:30:00.000Z"  fr fr Simplified
}

slay get_current_thread_id() normie {
    fr fr Get current thread ID
    damn 1  fr fr Simplified
}

slay get_instruction_pointer() normie {
    fr fr Get current instruction pointer
    damn core.get_instruction_pointer()
}

slay format_hex(value normie) tea {
    fr fr Format number as hexadecimal
    fr fr Simplified hex formatting
    lowkey value == 0 {
        damn "0"
    }
    damn vibez.spillstr("%X", value)
}

slay get_debug_log() []tea {
    damn debug_log_buffer
}

slay clear_debug_log() lit {
    debug_log_buffer = []
    log_info("Debug log cleared")
    damn based
}

slay dump_debug_state() lit {
    vibez.spill("🔧 Debug System State")
    vibez.spill("════════════════════")
    vibez.spill("Enabled: ", format_bool(debug_config.enabled))
    vibez.spill("Log Level: ", debug_config.log_level)
    vibez.spill("Stack Trace: ", format_bool(debug_config.stack_trace_enabled))
    vibez.spill("Memory Tracking: ", format_bool(debug_config.memory_tracking))
    vibez.spill("Profiling: ", format_bool(debug_config.performance_profiling))
    vibez.spill("Breakpoints: ", format_bool(debug_config.breakpoints_enabled))
    vibez.spill("Call Stack Depth: ", call_stack.len())
    vibez.spill("Active Breakpoints: ", active_breakpoints.len())
    vibez.spill("Profiler Entries: ", profiler_data.len())
    vibez.spill("Log Buffer Size: ", debug_log_buffer.len())
    damn based
}

slay format_bool(value lit) tea {
    lowkey value {
        damn "true"
    }
    damn "false"
}

fr fr Initialize debug system
slay init_debug_system() lit {
    log_info("Debug system initialized")
    damn based
}
