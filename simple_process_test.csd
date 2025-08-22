fr fr Simple Process Management Test
yeet "process_real"

fr fr Test real process spawning and management
sus args []tea = []tea{"Hello", "World"}
sus process Process = process_spawn("echo", args)

vibez.spill("Spawned process ID: %d", process.process_id)
vibez.spill("Process command: %s", process.command)
vibez.spill("Process is running: %s", process.is_running ? "true" : "false")

fr fr Test resource usage
sus memory thicc = get_memory_usage()
sus cpu meal = get_cpu_usage() 
vibez.spill("Memory usage: %d bytes", memory)
vibez.spill("CPU usage: %.2f%%", cpu)

fr fr Test environment
sus hostname tea = get_hostname()
sus user tea = get_current_user()
vibez.spill("Hostname: %s", hostname)  
vibez.spill("User: %s", user)

fr fr Test limits
sus mem_limit_result lit = set_memory_limit(50 * 1024 * 1024) fr fr 50MB
sus cpu_limit_result lit = set_cpu_limit(5.0) fr fr 5 seconds
vibez.spill("Memory limit set: %s", mem_limit_result ? "success" : "failed")
vibez.spill("CPU limit set: %s", cpu_limit_result ? "success" : "failed")

vibez.spill("Process management test complete!")
