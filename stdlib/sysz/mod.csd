fr fr sysz module - System calls, environment, process management
fr fr Essential system integration for CURSED runtime

yeet "memoryz"
yeet "debugz"

fr fr ===== SYSTEM INFORMATION STRUCTURES =====

squad SystemInfo {
    spill os_name tea
    spill arch tea
    spill version tea
    spill hostname tea
    spill username tea
    spill home_dir tea
    spill temp_dir tea
    spill cpu_count normie
    spill memory_total normie
    spill memory_available normie
}

squad ProcessInfo {
    spill pid normie
    spill ppid normie
    spill name tea
    spill status tea
    spill memory_usage normie
    spill cpu_percent meal
    spill start_time normie
    spill command_line tea
}

squad FileStats {
    spill path tea
    spill size normie
    spill mode normie
    spill uid normie
    spill gid normie
    spill access_time normie
    spill modify_time normie
    spill create_time normie
    spill is_file lit
    spill is_dir lit
    spill is_symlink lit
}

squad EnvironmentVar {
    spill name tea
    spill value tea
}

squad Signal {
    spill number normie
    spill name tea
    spill default_action tea
}

fr fr Process status constants
sus PROCESS_RUNNING normie = 1
sus PROCESS_SLEEPING normie = 2
sus PROCESS_STOPPED normie = 3
sus PROCESS_ZOMBIE normie = 4

fr fr File mode constants
sus FILE_MODE_READ normie = 4
sus FILE_MODE_WRITE normie = 2
sus FILE_MODE_EXECUTE normie = 1

fr fr Signal constants
sus SIGTERM normie = 15
sus SIGKILL normie = 9
sus SIGINT normie = 2
sus SIGUSR1 normie = 10
sus SIGUSR2 normie = 12

fr fr Global system state
sus current_system_info SystemInfo = SystemInfo{}
sus environment_vars []EnvironmentVar = []
sus signal_handlers []normie = []  fr fr Maps signal numbers to handler addresses

fr fr ===== SYSTEM INFORMATION FUNCTIONS =====

slay get_system_info() SystemInfo {
    fr fr Populate system information
    current_system_info.os_name = get_os_name()
    current_system_info.arch = get_arch()
    current_system_info.version = get_os_version()
    current_system_info.hostname = get_hostname()
    current_system_info.username = get_username()
    current_system_info.home_dir = get_home_dir()
    current_system_info.temp_dir = get_temp_dir()
    current_system_info.cpu_count = get_cpu_count()
    current_system_info.memory_total = get_total_memory()
    current_system_info.memory_available = get_available_memory()
    
    damn current_system_info
}

slay get_os_name() tea {
    sus os_info tea = syscall_get_os_info()
    fr fr Parse OS name from system call result
    lowkey os_info.contains("Linux") {
        damn "Linux"
    } highkey os_info.contains("Darwin") {
        damn "macOS"
    } highkey os_info.contains("Windows") {
        damn "Windows"
    } highkey {
        damn "Unknown"
    }
}

slay get_arch() tea {
    sus arch_info tea = syscall_get_arch_info()
    lowkey arch_info.contains("x86_64") || arch_info.contains("amd64") {
        damn "x86_64"
    } highkey arch_info.contains("aarch64") || arch_info.contains("arm64") {
        damn "arm64"
    } highkey arch_info.contains("i386") {
        damn "i386"
    } highkey {
        damn "unknown"
    }
}

slay get_os_version() tea {
    damn syscall_get_os_version()
}

slay get_hostname() tea {
    damn syscall_get_hostname()
}

slay get_username() tea {
    damn syscall_get_username()
}

slay get_home_dir() tea {
    sus home tea = get_env("HOME")
    lowkey home == "" {
        home = get_env("USERPROFILE")  fr fr Windows fallback
    }
    damn home
}

slay get_temp_dir() tea {
    sus temp tea = get_env("TMPDIR")
    lowkey temp == "" {
        temp = get_env("TMP")  fr fr Windows
    }
    lowkey temp == "" {
        temp = "/tmp"  fr fr Unix default
    }
    damn temp
}

slay get_cpu_count() normie {
    damn syscall_get_cpu_count()
}

slay get_total_memory() normie {
    damn syscall_get_total_memory()
}

slay get_available_memory() normie {
    damn syscall_get_available_memory()
}

fr fr ===== ENVIRONMENT VARIABLE FUNCTIONS =====

slay get_env(name tea) tea {
    fr fr Get environment variable value
    bestie env_var in environment_vars {
        lowkey env_var.name == name {
            damn env_var.value
        }
    }
    
    fr fr Not found in cache, query system
    sus value tea = syscall_get_env(name)
    
    fr fr Cache the result
    lowkey value != "" {
        sus new_var EnvironmentVar = EnvironmentVar{name: name, value: value}
        environment_vars.push(new_var)
    }
    
    damn value
}

slay set_env(name tea, value tea) lit {
    fr fr Set environment variable
    sus success lit = syscall_set_env(name, value)
    
    lowkey success {
        fr fr Update cache
        bestie i := 0; i < environment_vars.len(); i++ {
            lowkey environment_vars[i].name == name {
                environment_vars[i].value = value
                damn based
            }
        }
        
        fr fr Add new variable to cache
        sus new_var EnvironmentVar = EnvironmentVar{name: name, value: value}
        environment_vars.push(new_var)
    }
    
    damn success
}

slay unset_env(name tea) lit {
    fr fr Unset environment variable
    sus success lit = syscall_unset_env(name)
    
    lowkey success {
        fr fr Remove from cache
        bestie i := 0; i < environment_vars.len(); i++ {
            lowkey environment_vars[i].name == name {
                environment_vars.remove(i)
                damn based
            }
        }
    }
    
    damn success
}

slay get_all_env() []EnvironmentVar {
    fr fr Get all environment variables
    sus all_vars []EnvironmentVar = syscall_get_all_env()
    environment_vars = all_vars  fr fr Update cache
    damn all_vars
}

slay clear_env_cache() lit {
    environment_vars = []
    damn based
}

fr fr ===== PROCESS MANAGEMENT FUNCTIONS =====

slay get_current_pid() normie {
    damn syscall_get_current_pid()
}

slay get_parent_pid() normie {
    damn syscall_get_parent_pid()
}

slay get_process_info(pid normie) ProcessInfo {
    damn syscall_get_process_info(pid)
}

slay get_current_process_info() ProcessInfo {
    sus pid normie = get_current_pid()
    damn get_process_info(pid)
}

slay spawn_process(command tea, args []tea, working_dir tea) normie {
    fr fr Spawn new process
    debugz.log_info("Spawning process: ", command, " in ", working_dir)
    sus pid normie = syscall_spawn_process(command, args, working_dir)
    lowkey pid > 0 {
        debugz.log_info("Process spawned with PID: ", pid)
    } highkey {
        debugz.log_error("Failed to spawn process: ", command)
    }
    damn pid
}

slay wait_for_process(pid normie) normie {
    fr fr Wait for process to complete and get exit code
    debugz.log_debug("Waiting for process: ", pid)
    sus exit_code normie = syscall_wait_for_process(pid)
    debugz.log_debug("Process ", pid, " exited with code: ", exit_code)
    damn exit_code
}

slay kill_process(pid normie, signal normie) lit {
    fr fr Send signal to process
    debugz.log_info("Killing process ", pid, " with signal ", signal)
    sus success lit = syscall_kill_process(pid, signal)
    lowkey success {
        debugz.log_info("Signal sent successfully")
    } highkey {
        debugz.log_error("Failed to send signal to process ", pid)
    }
    damn success
}

slay terminate_process(pid normie) lit {
    fr fr Gracefully terminate process
    damn kill_process(pid, SIGTERM)
}

slay force_kill_process(pid normie) lit {
    fr fr Forcefully kill process
    damn kill_process(pid, SIGKILL)
}

slay is_process_running(pid normie) lit {
    sus info ProcessInfo = get_process_info(pid)
    damn info.status == "running" || info.status == "sleeping"
}

slay get_process_list() []ProcessInfo {
    fr fr Get list of all processes
    damn syscall_get_process_list()
}

fr fr ===== FILE SYSTEM OPERATIONS =====

slay get_file_stats(path tea) FileStats {
    fr fr Get file/directory statistics
    damn syscall_get_file_stats(path)
}

slay file_exists(path tea) lit {
    sus stats FileStats = get_file_stats(path)
    damn stats.size >= 0  fr fr Non-negative size indicates file exists
}

slay is_file(path tea) lit {
    sus stats FileStats = get_file_stats(path)
    damn stats.is_file
}

slay is_directory(path tea) lit {
    sus stats FileStats = get_file_stats(path)
    damn stats.is_dir
}

slay is_symlink(path tea) lit {
    sus stats FileStats = get_file_stats(path)
    damn stats.is_symlink
}

slay get_file_size(path tea) normie {
    sus stats FileStats = get_file_stats(path)
    damn stats.size
}

slay get_file_permissions(path tea) normie {
    sus stats FileStats = get_file_stats(path)
    damn stats.mode
}

slay set_file_permissions(path tea, mode normie) lit {
    debugz.log_debug("Setting permissions for ", path, " to ", mode)
    sus success lit = syscall_set_file_permissions(path, mode)
    lowkey success {
        debugz.log_debug("Permissions updated successfully")
    } highkey {
        debugz.log_error("Failed to set permissions for ", path)
    }
    damn success
}

slay create_directory(path tea, recursive lit) lit {
    debugz.log_info("Creating directory: ", path, " (recursive: ", recursive, ")")
    sus success lit = syscall_create_directory(path, recursive)
    lowkey success {
        debugz.log_info("Directory created successfully")
    } highkey {
        debugz.log_error("Failed to create directory: ", path)
    }
    damn success
}

slay remove_directory(path tea, recursive lit) lit {
    debugz.log_info("Removing directory: ", path, " (recursive: ", recursive, ")")
    sus success lit = syscall_remove_directory(path, recursive)
    lowkey success {
        debugz.log_info("Directory removed successfully")
    } highkey {
        debugz.log_error("Failed to remove directory: ", path)
    }
    damn success
}

slay copy_file(src tea, dest tea) lit {
    debugz.log_info("Copying file from ", src, " to ", dest)
    sus success lit = syscall_copy_file(src, dest)
    lowkey success {
        debugz.log_info("File copied successfully")
    } highkey {
        debugz.log_error("Failed to copy file")
    }
    damn success
}

slay move_file(src tea, dest tea) lit {
    debugz.log_info("Moving file from ", src, " to ", dest)
    sus success lit = syscall_move_file(src, dest)
    lowkey success {
        debugz.log_info("File moved successfully")
    } highkey {
        debugz.log_error("Failed to move file")
    }
    damn success
}

slay delete_file(path tea) lit {
    debugz.log_info("Deleting file: ", path)
    sus success lit = syscall_delete_file(path)
    lowkey success {
        debugz.log_info("File deleted successfully")
    } highkey {
        debugz.log_error("Failed to delete file: ", path)
    }
    damn success
}

slay list_directory(path tea) []tea {
    fr fr List directory contents
    debugz.log_debug("Listing directory: ", path)
    sus files []tea = syscall_list_directory(path)
    debugz.log_debug("Found ", files.len(), " entries")
    damn files
}

slay get_current_directory() tea {
    damn syscall_get_current_directory()
}

slay set_current_directory(path tea) lit {
    debugz.log_info("Changing directory to: ", path)
    sus success lit = syscall_set_current_directory(path)
    lowkey success {
        debugz.log_info("Directory changed successfully")
    } highkey {
        debugz.log_error("Failed to change directory to: ", path)
    }
    damn success
}

fr fr ===== SIGNAL HANDLING =====

slay register_signal_handler(signal_num normie, handler_address normie) lit {
    fr fr Register signal handler
    debugz.log_info("Registering signal handler for signal ", signal_num)
    
    fr fr Extend signal handlers array if needed
    bestie signal_handlers.len() <= signal_num {
        signal_handlers.push(0)
    }
    
    signal_handlers[signal_num] = handler_address
    sus success lit = syscall_register_signal_handler(signal_num, handler_address)
    
    lowkey success {
        debugz.log_info("Signal handler registered successfully")
    } highkey {
        debugz.log_error("Failed to register signal handler for signal ", signal_num)
    }
    
    damn success
}

slay send_signal(pid normie, signal_num normie) lit {
    fr fr Send signal to process
    debugz.log_info("Sending signal ", signal_num, " to process ", pid)
    damn syscall_send_signal(pid, signal_num)
}

slay ignore_signal(signal_num normie) lit {
    fr fr Ignore a signal
    debugz.log_info("Ignoring signal ", signal_num)
    damn syscall_ignore_signal(signal_num)
}

slay default_signal_handler(signal_num normie) lit {
    fr fr Restore default signal handler
    debugz.log_info("Restoring default handler for signal ", signal_num)
    damn syscall_default_signal_handler(signal_num)
}

slay get_signal_name(signal_num normie) tea {
    fr fr Get signal name from number
    lowkey signal_num == SIGTERM {
        damn "SIGTERM"
    } highkey signal_num == SIGKILL {
        damn "SIGKILL"
    } highkey signal_num == SIGINT {
        damn "SIGINT"
    } highkey signal_num == SIGUSR1 {
        damn "SIGUSR1"
    } highkey signal_num == SIGUSR2 {
        damn "SIGUSR2"
    } highkey {
        damn "UNKNOWN"
    }
}

fr fr ===== TIME AND SLEEP FUNCTIONS =====

slay get_current_time_seconds() normie {
    damn syscall_get_current_time_seconds()
}

slay get_current_time_millis() normie {
    damn syscall_get_current_time_millis()
}

slay get_current_time_micros() normie {
    damn syscall_get_current_time_micros()
}

slay get_current_time_nanos() normie {
    damn syscall_get_current_time_nanos()
}

slay sleep_seconds(seconds normie) lit {
    debugz.log_trace("Sleeping for ", seconds, " seconds")
    syscall_sleep_seconds(seconds)
    damn based
}

slay sleep_millis(millis normie) lit {
    debugz.log_trace("Sleeping for ", millis, " milliseconds")
    syscall_sleep_millis(millis)
    damn based
}

slay sleep_micros(micros normie) lit {
    debugz.log_trace("Sleeping for ", micros, " microseconds")
    syscall_sleep_micros(micros)
    damn based
}

slay sleep_nanos(nanos normie) lit {
    debugz.log_trace("Sleeping for ", nanos, " nanoseconds")
    syscall_sleep_nanos(nanos)
    damn based
}

fr fr ===== NETWORK FUNCTIONS =====

slay get_network_interfaces() []tea {
    fr fr Get list of network interfaces
    damn syscall_get_network_interfaces()
}

slay get_ip_address(interface_name tea) tea {
    fr fr Get IP address for interface
    damn syscall_get_ip_address(interface_name)
}

slay get_mac_address(interface_name tea) tea {
    fr fr Get MAC address for interface
    damn syscall_get_mac_address(interface_name)
}

slay is_network_available() lit {
    fr fr Check if network is available
    damn syscall_is_network_available()
}

fr fr ===== SYSTEM RESOURCE MONITORING =====

slay get_cpu_usage() meal {
    fr fr Get CPU usage percentage
    damn syscall_get_cpu_usage()
}

slay get_memory_usage() normie {
    fr fr Get current memory usage in bytes
    damn syscall_get_memory_usage()
}

slay get_disk_usage(path tea) (normie, normie) {
    fr fr Get disk usage for path (used, total)
    sus used normie = syscall_get_disk_used(path)
    sus total normie = syscall_get_disk_total(path)
    damn (used, total)
}

slay get_load_average() (meal, meal, meal) {
    fr fr Get 1, 5, 15 minute load averages
    sus load1 meal = syscall_get_load_1min()
    sus load5 meal = syscall_get_load_5min()
    sus load15 meal = syscall_get_load_15min()
    damn (load1, load5, load15)
}

fr fr ===== UTILITY FUNCTIONS =====

slay exit_process(exit_code normie) cringe {
    fr fr Exit current process with code
    debugz.log_info("Exiting process with code: ", exit_code)
    syscall_exit_process(exit_code)
    damn cringe  fr fr Never reached
}

slay restart_process() cringe {
    fr fr Restart current process
    debugz.log_info("Restarting process")
    syscall_restart_process()
    damn cringe  fr fr Never reached
}

slay get_command_line_args() []tea {
    fr fr Get command line arguments
    damn syscall_get_command_line_args()
}

slay get_working_directory() tea {
    damn get_current_directory()
}

slay set_working_directory(path tea) lit {
    damn set_current_directory(path)
}

slay get_executable_path() tea {
    fr fr Get path to current executable
    damn syscall_get_executable_path()
}

slay get_library_path() tea {
    fr fr Get system library path
    damn get_env("LD_LIBRARY_PATH")
}

fr fr ===== SYSTEM CALLS INTERFACE =====

fr fr These functions interface with the actual system calls
fr fr In a real implementation, these would use assembly or FFI

slay syscall_get_os_info() tea {
    damn core.system_call(1, 0, 0, 0)  fr fr SYS_UNAME
}

slay syscall_get_arch_info() tea {
    damn core.system_call(2, 0, 0, 0)  fr fr SYS_ARCH
}

slay syscall_get_os_version() tea {
    damn core.system_call(3, 0, 0, 0)  fr fr SYS_VERSION
}

slay syscall_get_hostname() tea {
    damn core.system_call(4, 0, 0, 0)  fr fr SYS_HOSTNAME
}

slay syscall_get_username() tea {
    damn core.system_call(5, 0, 0, 0)  fr fr SYS_USERNAME
}

slay syscall_get_cpu_count() normie {
    damn core.system_call(6, 0, 0, 0)  fr fr SYS_CPU_COUNT
}

slay syscall_get_total_memory() normie {
    damn core.system_call(7, 0, 0, 0)  fr fr SYS_TOTAL_MEMORY
}

slay syscall_get_available_memory() normie {
    damn core.system_call(8, 0, 0, 0)  fr fr SYS_AVAILABLE_MEMORY
}

slay syscall_get_env(name tea) tea {
    damn core.system_call(10, name, 0, 0)  fr fr SYS_GETENV
}

slay syscall_set_env(name tea, value tea) lit {
    sus result normie = core.system_call(11, name, value, 0)  fr fr SYS_SETENV
    damn result == 0
}

slay syscall_unset_env(name tea) lit {
    sus result normie = core.system_call(12, name, 0, 0)  fr fr SYS_UNSETENV
    damn result == 0
}

slay syscall_get_all_env() []EnvironmentVar {
    damn core.system_call(13, 0, 0, 0)  fr fr SYS_ENVIRON
}

slay syscall_get_current_pid() normie {
    damn core.system_call(20, 0, 0, 0)  fr fr SYS_GETPID
}

slay syscall_get_parent_pid() normie {
    damn core.system_call(21, 0, 0, 0)  fr fr SYS_GETPPID
}

slay syscall_get_process_info(pid normie) ProcessInfo {
    damn core.system_call(22, pid, 0, 0)  fr fr SYS_PROCESS_INFO
}

slay syscall_spawn_process(command tea, args []tea, working_dir tea) normie {
    damn core.system_call(23, command, args, working_dir)  fr fr SYS_SPAWN
}

slay syscall_wait_for_process(pid normie) normie {
    damn core.system_call(24, pid, 0, 0)  fr fr SYS_WAIT
}

slay syscall_kill_process(pid normie, signal normie) lit {
    sus result normie = core.system_call(25, pid, signal, 0)  fr fr SYS_KILL
    damn result == 0
}

slay syscall_get_process_list() []ProcessInfo {
    damn core.system_call(26, 0, 0, 0)  fr fr SYS_PROCESS_LIST
}

slay syscall_get_file_stats(path tea) FileStats {
    damn core.system_call(30, path, 0, 0)  fr fr SYS_STAT
}

slay syscall_set_file_permissions(path tea, mode normie) lit {
    sus result normie = core.system_call(31, path, mode, 0)  fr fr SYS_CHMOD
    damn result == 0
}

slay syscall_create_directory(path tea, recursive lit) lit {
    sus result normie = core.system_call(32, path, recursive, 0)  fr fr SYS_MKDIR
    damn result == 0
}

slay syscall_remove_directory(path tea, recursive lit) lit {
    sus result normie = core.system_call(33, path, recursive, 0)  fr fr SYS_RMDIR
    damn result == 0
}

slay syscall_copy_file(src tea, dest tea) lit {
    sus result normie = core.system_call(34, src, dest, 0)  fr fr SYS_COPY
    damn result == 0
}

slay syscall_move_file(src tea, dest tea) lit {
    sus result normie = core.system_call(35, src, dest, 0)  fr fr SYS_MOVE
    damn result == 0
}

slay syscall_delete_file(path tea) lit {
    sus result normie = core.system_call(36, path, 0, 0)  fr fr SYS_UNLINK
    damn result == 0
}

slay syscall_list_directory(path tea) []tea {
    damn core.system_call(37, path, 0, 0)  fr fr SYS_READDIR
}

slay syscall_get_current_directory() tea {
    damn core.system_call(38, 0, 0, 0)  fr fr SYS_GETCWD
}

slay syscall_set_current_directory(path tea) lit {
    sus result normie = core.system_call(39, path, 0, 0)  fr fr SYS_CHDIR
    damn result == 0
}

slay syscall_register_signal_handler(signal_num normie, handler_address normie) lit {
    sus result normie = core.system_call(40, signal_num, handler_address, 0)  fr fr SYS_SIGNAL
    damn result == 0
}

slay syscall_send_signal(pid normie, signal_num normie) lit {
    sus result normie = core.system_call(41, pid, signal_num, 0)  fr fr SYS_KILL
    damn result == 0
}

slay syscall_ignore_signal(signal_num normie) lit {
    sus result normie = core.system_call(42, signal_num, 1, 0)  fr fr SYS_SIGNAL_IGNORE
    damn result == 0
}

slay syscall_default_signal_handler(signal_num normie) lit {
    sus result normie = core.system_call(43, signal_num, 0, 0)  fr fr SYS_SIGNAL_DEFAULT
    damn result == 0
}

slay syscall_get_current_time_seconds() normie {
    damn core.system_call(50, 0, 0, 0)  fr fr SYS_TIME
}

slay syscall_get_current_time_millis() normie {
    damn core.system_call(51, 0, 0, 0)  fr fr SYS_TIME_MILLIS
}

slay syscall_get_current_time_micros() normie {
    damn core.system_call(52, 0, 0, 0)  fr fr SYS_TIME_MICROS
}

slay syscall_get_current_time_nanos() normie {
    damn core.system_call(53, 0, 0, 0)  fr fr SYS_TIME_NANOS
}

slay syscall_sleep_seconds(seconds normie) cringe {
    core.system_call(54, seconds, 0, 0)  fr fr SYS_SLEEP
    damn cringe
}

slay syscall_sleep_millis(millis normie) cringe {
    core.system_call(55, millis, 0, 0)  fr fr SYS_SLEEP_MILLIS
    damn cringe
}

slay syscall_sleep_micros(micros normie) cringe {
    core.system_call(56, micros, 0, 0)  fr fr SYS_SLEEP_MICROS
    damn cringe
}

slay syscall_sleep_nanos(nanos normie) cringe {
    core.system_call(57, nanos, 0, 0)  fr fr SYS_SLEEP_NANOS
    damn cringe
}

slay syscall_get_network_interfaces() []tea {
    damn core.system_call(60, 0, 0, 0)  fr fr SYS_NET_INTERFACES
}

slay syscall_get_ip_address(interface_name tea) tea {
    damn core.system_call(61, interface_name, 0, 0)  fr fr SYS_NET_IP
}

slay syscall_get_mac_address(interface_name tea) tea {
    damn core.system_call(62, interface_name, 0, 0)  fr fr SYS_NET_MAC
}

slay syscall_is_network_available() lit {
    sus result normie = core.system_call(63, 0, 0, 0)  fr fr SYS_NET_AVAILABLE
    damn result == 1
}

slay syscall_get_cpu_usage() meal {
    damn core.system_call(70, 0, 0, 0)  fr fr SYS_CPU_USAGE
}

slay syscall_get_memory_usage() normie {
    damn core.system_call(71, 0, 0, 0)  fr fr SYS_MEMORY_USAGE
}

slay syscall_get_disk_used(path tea) normie {
    damn core.system_call(72, path, 0, 0)  fr fr SYS_DISK_USED
}

slay syscall_get_disk_total(path tea) normie {
    damn core.system_call(73, path, 0, 0)  fr fr SYS_DISK_TOTAL
}

slay syscall_get_load_1min() meal {
    damn core.system_call(74, 0, 0, 0)  fr fr SYS_LOAD_1MIN
}

slay syscall_get_load_5min() meal {
    damn core.system_call(75, 0, 0, 0)  fr fr SYS_LOAD_5MIN
}

slay syscall_get_load_15min() meal {
    damn core.system_call(76, 0, 0, 0)  fr fr SYS_LOAD_15MIN
}

slay syscall_exit_process(exit_code normie) cringe {
    core.system_call(80, exit_code, 0, 0)  fr fr SYS_EXIT
    damn cringe
}

slay syscall_restart_process() cringe {
    core.system_call(81, 0, 0, 0)  fr fr SYS_RESTART
    damn cringe
}

slay syscall_get_command_line_args() []tea {
    damn core.system_call(82, 0, 0, 0)  fr fr SYS_ARGS
}

slay syscall_get_executable_path() tea {
    damn core.system_call(83, 0, 0, 0)  fr fr SYS_EXECUTABLE_PATH
}

fr fr ===== SYSTEM REPORTING FUNCTIONS =====

slay print_system_info() lit {
    sus info SystemInfo = get_system_info()
    
    vibez.spill("💻 System Information")
    vibez.spill("═══════════════════")
    vibez.spill("OS: ", info.os_name, " ", info.version)
    vibez.spill("Architecture: ", info.arch)
    vibez.spill("Hostname: ", info.hostname)
    vibez.spill("Username: ", info.username)
    vibez.spill("Home Directory: ", info.home_dir)
    vibez.spill("Temp Directory: ", info.temp_dir)
    vibez.spill("CPU Count: ", info.cpu_count)
    vibez.spill("Total Memory: ", memoryz.format_bytes(info.memory_total))
    vibez.spill("Available Memory: ", memoryz.format_bytes(info.memory_available))
    
    damn based
}

slay print_process_info(pid normie) lit {
    sus info ProcessInfo = get_process_info(pid)
    
    vibez.spill("🔄 Process Information")
    vibez.spill("═════════════════════")
    vibez.spill("PID: ", info.pid)
    vibez.spill("Parent PID: ", info.ppid)
    vibez.spill("Name: ", info.name)
    vibez.spill("Status: ", info.status)
    vibez.spill("Memory Usage: ", memoryz.format_bytes(info.memory_usage))
    vibez.spill("CPU Percent: ", info.cpu_percent, "%")
    vibez.spill("Start Time: ", info.start_time)
    vibez.spill("Command Line: ", info.command_line)
    
    damn based
}

slay print_environment_vars() lit {
    sus all_vars []EnvironmentVar = get_all_env()
    
    vibez.spill("🌍 Environment Variables")
    vibez.spill("═══════════════════════")
    
    bestie env_var in all_vars {
        vibez.spill(env_var.name, "=", env_var.value)
    }
    
    damn based
}

fr fr Initialize system module
slay init_system() lit {
    debugz.log_info("System module initialized")
    fr fr Pre-populate system info cache
    get_system_info()
    damn based
}
