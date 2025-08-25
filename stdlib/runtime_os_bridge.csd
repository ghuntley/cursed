fr fr ============================================================================
fr fr CURSED Runtime OS Bridge - Real OS Integration Interface
fr fr ============================================================================
fr fr This module defines the interface between CURSED stdlib and the runtime
fr fr for direct OS integration. These functions are implemented in Zig/C
fr fr and provide actual system calls and OS services.
fr fr ============================================================================

fr fr System call constants
facts {
    SYS_READ = 0
    SYS_WRITE = 1
    SYS_OPEN = 2
    SYS_CLOSE = 3
    SYS_GETTID = 186
    SYS_SCHED_YIELD = 24
    SYS_CLOCK_GETTIME = 228
    SYS_NANOSLEEP = 35
    SYS_FUTEX = 202
    SYS_CLONE = 56
    SYS_FORK = 57
    SYS_EXECVE = 59
    SYS_WAIT4 = 61
    CLOCK_MONOTONIC = 1
}

fr fr =============================================================================
fr fr CORE SYSCALL INTERFACE
fr fr =============================================================================

fr fr Generic system call interface - maps directly to OS syscall mechanism
extern slay cursed_runtime_syscall(syscall_number normie, arg1 thicc, arg2 thicc, arg3 thicc, arg4 thicc, arg5 thicc, arg6 thicc) thicc

fr fr =============================================================================
fr fr THREAD AND PROCESS MANAGEMENT
fr fr =============================================================================

fr fr Get thread ID
extern slay cursed_runtime_gettid() thicc

fr fr Yield CPU to scheduler
extern slay cursed_runtime_sched_yield() normie

fr fr Clone/fork operations
extern slay cursed_runtime_clone(flags thicc, stack_ptr thicc, parent_tidptr thicc, child_tidptr thicc, thread_func thicc, thread_arg thicc) thicc
extern slay cursed_runtime_futex(uaddr thicc, futex_op normie, val normie, timeout thicc, uaddr2 thicc, val3 normie) normie

fr fr Get current process/thread information  
extern slay cursed_runtime_get_pid() normie
extern slay cursed_runtime_get_ppid() normie
extern slay cursed_runtime_sysconf(name normie) normie
extern slay cursed_runtime_get_errno() normie

fr fr =============================================================================
fr fr TIME AND CLOCK FUNCTIONS
fr fr =============================================================================

fr fr Get monotonic time in nanoseconds
extern slay cursed_runtime_clock_gettime_monotonic() thicc

fr fr Get current wall clock time in milliseconds
extern slay cursed_runtime_get_time_ms() thicc

fr fr Timespec structure management
extern slay cursed_runtime_alloc_timespec() thicc
extern slay cursed_runtime_free_timespec(ptr thicc)
extern slay cursed_runtime_timespec_to_ns(timespec_ptr thicc) thicc

fr fr Sleep functions
extern slay cursed_runtime_nanosleep(nanoseconds thicc) normie
extern slay cursed_runtime_sleep_ms(milliseconds normie) normie

fr fr =============================================================================
fr fr WINDOWS API INTEGRATION  
fr fr =============================================================================

extern slay cursed_runtime_win32_create_thread(security thicc, stack_size normie, start_addr thicc, parameter thicc, flags normie, thread_id_ptr thicc) thicc
extern slay cursed_runtime_win32_wait_for_single_object(handle thicc, timeout normie) normie
extern slay cursed_runtime_win32_close_handle(handle thicc) normie
extern slay cursed_runtime_win32_switch_to_thread()
extern slay cursed_runtime_win32_get_current_thread_id() thicc

fr fr Critical section operations
extern slay cursed_runtime_win32_init_critical_section(cs thicc)
extern slay cursed_runtime_win32_enter_critical_section(cs thicc)
extern slay cursed_runtime_win32_leave_critical_section(cs thicc)
extern slay cursed_runtime_win32_try_enter_critical_section(cs thicc) lit

fr fr Condition variable operations
extern slay cursed_runtime_win32_init_condition_variable(cv thicc)
extern slay cursed_runtime_win32_sleep_condition_variable_cs(cv thicc, cs thicc, timeout normie) lit
extern slay cursed_runtime_win32_wake_condition_variable(cv thicc)
extern slay cursed_runtime_win32_wake_all_condition_variable(cv thicc)

fr fr =============================================================================
fr fr AUDIO BUFFER OPERATIONS
fr fr =============================================================================

fr fr Audio buffer direct access
extern slay cursed_runtime_write_audio_buffer_byte(buffer tea, index normie, value normie)
extern slay cursed_runtime_read_audio_buffer_byte(buffer tea, index normie) normie
extern slay cursed_runtime_write_audio_buffer_float(buffer tea, index normie, value drip) 
extern slay cursed_runtime_read_audio_buffer_float(buffer tea, index normie) drip

fr fr Audio device integration
extern slay cursed_runtime_open_audio_device(sample_rate normie, channels normie, buffer_size normie) thicc
extern slay cursed_runtime_close_audio_device(device_handle thicc) normie
extern slay cursed_runtime_write_audio_samples(device_handle thicc, buffer tea, sample_count normie) normie

fr fr =============================================================================
fr fr FILESYSTEM INTEGRATION
fr fr =============================================================================

extern slay cursed_runtime_open_file(path tea, flags normie, mode normie) normie
extern slay cursed_runtime_close_file(fd normie) normie  
extern slay cursed_runtime_read_file(fd normie, buffer tea, size normie) normie
extern slay cursed_runtime_write_file(fd normie, data tea, size normie) normie
extern slay cursed_runtime_seek_file(fd normie, offset thicc, whence normie) thicc
extern slay cursed_runtime_stat_file(path tea, stat_buffer thicc) normie

fr fr =============================================================================
fr fr NETWORK INTEGRATION
fr fr =============================================================================

extern slay cursed_runtime_socket_create(domain normie, type normie, protocol normie) normie
extern slay cursed_runtime_socket_bind(socket_fd normie, addr thicc, addr_len normie) normie
extern slay cursed_runtime_socket_listen(socket_fd normie, backlog normie) normie
extern slay cursed_runtime_socket_accept(socket_fd normie, addr thicc, addr_len thicc) normie
extern slay cursed_runtime_socket_connect(socket_fd normie, addr thicc, addr_len normie) normie
extern slay cursed_runtime_socket_send(socket_fd normie, data tea, size normie, flags normie) normie
extern slay cursed_runtime_socket_recv(socket_fd normie, buffer tea, size normie, flags normie) normie
extern slay cursed_runtime_socket_close(socket_fd normie) normie

fr fr =============================================================================
fr fr MEMORY MANAGEMENT INTEGRATION
fr fr =============================================================================

extern slay cursed_runtime_alloc_memory(size normie) thicc
extern slay cursed_runtime_free_memory(ptr thicc)
extern slay cursed_runtime_realloc_memory(ptr thicc, new_size normie) thicc
extern slay cursed_runtime_get_memory_usage() thicc

fr fr =============================================================================
fr fr ERROR HANDLING AND DEBUGGING
fr fr =============================================================================

extern slay cursed_runtime_get_stack_trace() tea
extern slay cursed_runtime_set_error_handler(handler thicc)
extern slay cursed_runtime_abort_program(exit_code normie)
extern slay cursed_runtime_print_debug(message tea)

fr fr =============================================================================
fr fr DYNAMIC LOADING AND FFI 
fr fr =============================================================================

extern slay cursed_runtime_dlopen(filename tea, flags normie) thicc
extern slay cursed_runtime_dlsym(handle thicc, symbol tea) thicc
extern slay cursed_runtime_dlclose(handle thicc) normie

fr fr =============================================================================
fr fr UTILITY FUNCTIONS
fr fr =============================================================================

fr fr UUID/unique ID generation
extern slay cursed_runtime_generate_uuid() tea
extern slay cursed_runtime_generate_random_bytes(buffer tea, size normie)

fr fr String/buffer utilities
extern slay cursed_runtime_memcpy(dest thicc, src thicc, size normie)
extern slay cursed_runtime_memset(ptr thicc, value normie, size normie)
extern slay cursed_runtime_strcmp(str1 tea, str2 tea) normie

fr fr Environment variables
extern slay cursed_runtime_getenv(name tea) tea
extern slay cursed_runtime_setenv(name tea, value tea, overwrite normie) normie

fr fr =============================================================================
fr fr PERFORMANCE AND MONITORING
fr fr =============================================================================

extern slay cursed_runtime_get_cpu_count() normie
extern slay cursed_runtime_get_cpu_usage() drip
extern slay cursed_runtime_get_memory_stats(total thicc, used thicc, available thicc)
extern slay cursed_runtime_get_thread_count() normie

fr fr =============================================================================
fr fr CONSTANTS AND ERROR CODES
fr fr =============================================================================

facts {
    fr fr File operations
    O_RDONLY = 0
    O_WRONLY = 1
    O_RDWR = 2
    O_CREAT = 64
    O_TRUNC = 512
    O_APPEND = 1024
    
    fr fr Network operations
    AF_INET = 2
    AF_INET6 = 10
    SOCK_STREAM = 1
    SOCK_DGRAM = 2
    
    fr fr Error codes
    EINVAL = 22
    ENOENT = 2
    EACCES = 13
    ENOMEM = 12
    
    fr fr Wait results
    WAIT_OBJECT_0 = 0
    WAIT_TIMEOUT = 258
    WAIT_FAILED = 4294967295
}
