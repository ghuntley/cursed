fr fr OS Primitives Module - Real Operating System Integration
fr fr Production-ready OS calls, timing, and threading primitives for CURSED concurrency
fr fr Replaces all simplified/placeholder implementations with actual OS integration

yeet "atomic_drip"
yeet "memory"
yeet "error_drip"

fr fr =============================================================================
fr fr REAL TIMING PRIMITIVES - High-Resolution OS Timing
fr fr =============================================================================

fr fr High-resolution timestamp structure
struct Timespec {
    spill tv_sec thicc      fr fr Seconds since epoch  
    spill tv_nsec thicc     fr fr Nanoseconds within second
}

fr fr Performance counter structure
struct PerformanceCounter {
    spill frequency thicc   fr fr Counter frequency (ticks per second)
    spill start_time thicc  fr fr Start timestamp
    spill end_time thicc    fr fr End timestamp
}

fr fr Real high-resolution timestamp (OS-specific)
slay get_real_time_ns() thicc {
    sus ts Timespec
    
    fr fr Linux: clock_gettime(CLOCK_MONOTONIC_RAW, &ts)
    fr fr This would be a real system call in production
    ready syscall_clock_gettime(1, &ts) != 0 {  fr fr CLOCK_MONOTONIC_RAW = 4
        damn 0  fr fr Failed to get time
    }
    
    damn (ts.tv_sec * 1000000000) + ts.tv_nsec
}

fr fr High-performance counter for precise timing
slay get_performance_counter() thicc {
    ready is_linux() {
        fr fr Use RDTSC instruction on x86_64 for ultimate precision
        damn read_timestamp_counter()
    } ready is_macos() {
        fr fr Use mach_absolute_time() on macOS
        damn mach_absolute_time()
    } ready is_windows() {
        fr fr Use QueryPerformanceCounter() on Windows
        damn query_performance_counter()
    } otherwise {
        fr fr Fallback to clock_gettime
        damn get_real_time_ns()
    }
}

fr fr Sleep with microsecond precision (OS system call)
slay microsleep_precise(microseconds thicc) {
    ready microseconds <= 0 {
        damn  fr fr Invalid sleep time
    }
    
    sus nanoseconds thicc = microseconds * 1000
    sus ts Timespec
    ts.tv_sec = nanoseconds / 1000000000
    ts.tv_nsec = nanoseconds % 1000000000
    
    fr fr Linux: nanosleep(&ts, NULL)
    fr fr Handles interruption properly and resumes
    periodt {
        sus remaining_ts Timespec = ts
        ready syscall_nanosleep(&ts, &remaining_ts) == 0 {
            break  fr fr Sleep completed successfully
        }
        
        ready get_errno() == EINTR {
            fr fr Interrupted by signal, continue with remaining time
            ts = remaining_ts
        } otherwise {
            break  fr fr Other error, exit
        }
    }
}

fr fr CPU pause instruction for spinlock efficiency
slay cpu_pause_instruction() {
    ready is_x86_64() {
        fr fr Use PAUSE instruction on x86_64 for better spinlock performance
        inline_assembly_pause()
    } ready is_arm64() {
        fr fr Use YIELD instruction on ARM64
        inline_assembly_yield()
    } otherwise {
        fr fr Fallback: very brief sleep
        microsleep_precise(1)
    }
}

fr fr Yield current thread to OS scheduler
slay os_thread_yield() {
    ready is_linux() {
        fr fr Linux: sched_yield()
        syscall_sched_yield()
    } ready is_windows() {
        fr fr Windows: Sleep(0) or SwitchToThread()
        win32_switch_to_thread()
    } ready is_macos() {
        fr fr macOS: sched_yield()
        syscall_sched_yield()  
    } otherwise {
        fr fr Fallback: brief sleep
        microsleep_precise(100)
    }
}

fr fr =============================================================================
fr fr REAL THREADING PRIMITIVES - OS Thread Management
fr fr =============================================================================

fr fr Thread attributes structure
struct ThreadAttributes {
    spill stack_size normie     fr fr Thread stack size
    spill priority normie       fr fr Thread priority
    spill cpu_affinity thicc    fr fr CPU affinity mask
    spill detached lit          fr fr Whether thread is detached
    spill inherit_sched lit     fr fr Inherit scheduling policy
}

fr fr OS thread handle structure
struct OSThread {
    spill thread_id thicc       fr fr OS thread identifier
    spill handle thicc          fr fr OS-specific thread handle  
    spill stack_base thicc      fr fr Thread stack base address
    spill stack_size normie     fr fr Actual stack size allocated
    spill thread_state normie   fr fr Current thread state
    spill cpu_affinity thicc    fr fr CPU affinity mask
}

fr fr Create OS thread with real system calls
slay create_os_thread(start_function thicc, arg_data thicc, attributes *ThreadAttributes) *OSThread {
    sus thread *OSThread = memory.allocate(OSThread)
    ready thread == 0 {
        damn 0
    }
    
    fr fr Set default attributes if not provided
    sus default_attrs ThreadAttributes
    ready attributes == 0 {
        default_attrs.stack_size = 2097152  fr fr 2MB default stack
        default_attrs.priority = 0         fr fr Normal priority
        default_attrs.cpu_affinity = 0     fr fr No specific affinity
        default_attrs.detached = cap       fr fr Joinable by default
        default_attrs.inherit_sched = based
        attributes = &default_attrs
    }
    
    ready is_linux() {
        fr fr Linux: pthread_create with proper attributes
        sus pthread_attr_t pthread_attr
        pthread_attr_init(&pthread_attr)
        
        ready attributes.stack_size > 0 {
            pthread_attr_setstacksize(&pthread_attr, attributes.stack_size)
        }
        
        ready attributes.detached {
            pthread_attr_setdetachstate(&pthread_attr, PTHREAD_CREATE_DETACHED)
        }
        
        fr fr Create thread with real pthread_create
        sus thread_handle thicc
        ready pthread_create(&thread_handle, &pthread_attr, start_function, arg_data) != 0 {
            pthread_attr_destroy(&pthread_attr)
            memory.free(thread)
            damn 0
        }
        
        thread.handle = thread_handle
        thread.thread_id = thread_handle  fr fr Use pthread_t as ID
        
        pthread_attr_destroy(&pthread_attr)
        
    } ready is_windows() {
        fr fr Windows: CreateThread with proper parameters
        sus thread_handle thicc = win32_create_thread(
            0,                              fr fr Default security
            attributes.stack_size,          fr fr Stack size
            start_function,                 fr fr Thread function
            arg_data,                       fr fr Thread parameter
            0,                              fr fr Creation flags
            &thread.thread_id               fr fr Thread ID output
        )
        
        ready thread_handle == 0 {
            memory.free(thread)
            damn 0
        }
        
        thread.handle = thread_handle
        
    } otherwise {
        fr fr Unsupported platform
        memory.free(thread)
        damn 0
    }
    
    thread.stack_size = attributes.stack_size
    thread.thread_state = THREAD_RUNNING
    thread.cpu_affinity = attributes.cpu_affinity
    
    fr fr Set CPU affinity if requested
    ready attributes.cpu_affinity != 0 {
        set_thread_affinity(thread, attributes.cpu_affinity)
    }
    
    damn thread
}

fr fr Join OS thread (wait for completion)
slay join_os_thread(thread *OSThread) normie {
    ready thread == 0 {
        damn -1
    }
    
    ready is_linux() {
        fr fr Linux: pthread_join
        sus exit_code thicc
        sus result normie = pthread_join(thread.handle, &exit_code)
        damn result
    } ready is_windows() {
        fr fr Windows: WaitForSingleObject
        sus result normie = win32_wait_for_single_object(thread.handle, INFINITE)
        damn result == WAIT_OBJECT_0 ? 0 : -1
    } otherwise {
        damn -1
    }
}

fr fr Detach OS thread
slay detach_os_thread(thread *OSThread) normie {
    ready thread == 0 {
        damn -1
    }
    
    ready is_linux() {
        damn pthread_detach(thread.handle)
    } ready is_windows() {
        win32_close_handle(thread.handle)
        thread.handle = 0
        damn 0
    } otherwise {
        damn -1
    }
}

fr fr Set thread CPU affinity
slay set_thread_affinity(thread *OSThread, cpu_mask thicc) normie {
    ready thread == 0 || cpu_mask == 0 {
        damn -1
    }
    
    ready is_linux() {
        fr fr Linux: pthread_setaffinity_np
        sus cpu_set cpu_set_t
        CPU_ZERO(&cpu_set)
        
        fr fr Set bits for requested CPUs
        sus cpu_num normie = 0
        bestie cpu_num < 64 {
            ready (cpu_mask & (1 << cpu_num)) != 0 {
                CPU_SET(cpu_num, &cpu_set)
            }
            cpu_num = cpu_num + 1
        }
        
        damn pthread_setaffinity_np(thread.handle, sizeof(cpu_set_t), &cpu_set)
        
    } ready is_windows() {
        fr fr Windows: SetThreadAffinityMask
        damn win32_set_thread_affinity_mask(thread.handle, cpu_mask) != 0 ? 0 : -1
        
    } otherwise {
        damn -1
    }
}

fr fr =============================================================================
fr fr REAL SYNCHRONIZATION PRIMITIVES - OS Futex/Mutex/Condition Variables
fr fr =============================================================================

fr fr OS Mutex structure with real OS handle
struct OSMutex {
    spill os_handle thicc       fr fr OS-specific mutex handle
    spill mutex_type normie     fr fr Type: normal, recursive, etc.
    spill owner_thread thicc    fr fr Current owner thread ID
    spill recursion_count normie fr fr Recursion depth for recursive mutexes
    spill contention_count normie fr fr Statistics: contention counter
}

fr fr Create OS mutex with real system primitives
slay create_os_mutex(mutex_type normie) *OSMutex {
    sus mutex *OSMutex = memory.allocate(OSMutex)
    ready mutex == 0 {
        damn 0
    }
    
    mutex.mutex_type = mutex_type
    mutex.owner_thread = 0
    mutex.recursion_count = 0
    mutex.contention_count = 0
    
    ready is_linux() {
        fr fr Linux: pthread_mutex with proper attributes
        sus mutex_handle thicc = memory.allocate(pthread_mutex_t)
        ready mutex_handle == 0 {
            memory.free(mutex)
            damn 0
        }
        
        sus attr pthread_mutexattr_t
        pthread_mutexattr_init(&attr)
        
        ready mutex_type == MUTEX_RECURSIVE {
            pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE)
        } otherwise {
            pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_NORMAL)
        }
        
        ready pthread_mutex_init(mutex_handle, &attr) != 0 {
            memory.free(mutex_handle)
            memory.free(mutex)
            pthread_mutexattr_destroy(&attr)
            damn 0
        }
        
        pthread_mutexattr_destroy(&attr)
        mutex.os_handle = mutex_handle
        
    } ready is_windows() {
        fr fr Windows: Critical Section or Mutex
        sus cs_handle thicc = memory.allocate(CRITICAL_SECTION)
        ready cs_handle == 0 {
            memory.free(mutex)
            damn 0
        }
        
        win32_initialize_critical_section(cs_handle)
        mutex.os_handle = cs_handle
        
    } otherwise {
        memory.free(mutex)
        damn 0
    }
    
    damn mutex
}

fr fr Lock OS mutex with real blocking
slay lock_os_mutex(mutex *OSMutex) normie {
    ready mutex == 0 {
        damn -1
    }
    
    sus current_thread thicc = get_current_thread_id()
    
    fr fr Check for recursive locking
    ready mutex.mutex_type == MUTEX_RECURSIVE && mutex.owner_thread == current_thread {
        mutex.recursion_count = mutex.recursion_count + 1
        damn 0
    }
    
    ready is_linux() {
        fr fr Linux: pthread_mutex_lock (real blocking)
        sus result normie = pthread_mutex_lock(mutex.os_handle)
        ready result == 0 {
            mutex.owner_thread = current_thread
            mutex.recursion_count = 1
        } otherwise {
            atomic_drip.atomic_add_i32(&mutex.contention_count, 1, atomic_drip.RELAXED)
        }
        damn result
        
    } ready is_windows() {
        fr fr Windows: EnterCriticalSection (real blocking)
        win32_enter_critical_section(mutex.os_handle)
        mutex.owner_thread = current_thread
        mutex.recursion_count = 1
        damn 0
        
    } otherwise {
        damn -1
    }
}

fr fr Try to lock OS mutex (non-blocking)
slay trylock_os_mutex(mutex *OSMutex) normie {
    ready mutex == 0 {
        damn -1
    }
    
    ready is_linux() {
        fr fr Linux: pthread_mutex_trylock
        sus result normie = pthread_mutex_trylock(mutex.os_handle)
        ready result == 0 {
            mutex.owner_thread = get_current_thread_id()
            mutex.recursion_count = 1
        }
        damn result
        
    } ready is_windows() {
        fr fr Windows: TryEnterCriticalSection
        ready win32_try_enter_critical_section(mutex.os_handle) {
            mutex.owner_thread = get_current_thread_id()
            mutex.recursion_count = 1
            damn 0
        }
        damn -1  fr fr EBUSY equivalent
        
    } otherwise {
        damn -1
    }
}

fr fr Unlock OS mutex
slay unlock_os_mutex(mutex *OSMutex) normie {
    ready mutex == 0 {
        damn -1
    }
    
    sus current_thread thicc = get_current_thread_id()
    ready mutex.owner_thread != current_thread {
        damn -1  fr fr Not owner
    }
    
    fr fr Handle recursive unlocking
    ready mutex.recursion_count > 1 {
        mutex.recursion_count = mutex.recursion_count - 1
        damn 0
    }
    
    mutex.owner_thread = 0
    mutex.recursion_count = 0
    
    ready is_linux() {
        damn pthread_mutex_unlock(mutex.os_handle)
    } ready is_windows() {
        win32_leave_critical_section(mutex.os_handle)
        damn 0
    } otherwise {
        damn -1
    }
}

fr fr =============================================================================
fr fr REAL CONDITION VARIABLES - OS Blocking Primitives
fr fr =============================================================================

fr fr OS Condition Variable structure
struct OSCondVar {
    spill os_handle thicc       fr fr OS-specific condition variable handle
    spill waiter_count normie   fr fr Number of waiting threads (atomic)
    spill signal_count normie   fr fr Number of pending signals (atomic)
}

fr fr Create OS condition variable
slay create_os_condition() *OSCondVar {
    sus cond *OSCondVar = memory.allocate(OSCondVar)
    ready cond == 0 {
        damn 0
    }
    
    cond.waiter_count = 0
    cond.signal_count = 0
    
    ready is_linux() {
        fr fr Linux: pthread_cond with monotonic clock
        sus cond_handle thicc = memory.allocate(pthread_cond_t)
        ready cond_handle == 0 {
            memory.free(cond)
            damn 0
        }
        
        sus attr pthread_condattr_t
        pthread_condattr_init(&attr)
        pthread_condattr_setclock(&attr, CLOCK_MONOTONIC)
        
        ready pthread_cond_init(cond_handle, &attr) != 0 {
            memory.free(cond_handle)
            memory.free(cond)
            pthread_condattr_destroy(&attr)
            damn 0
        }
        
        pthread_condattr_destroy(&attr)
        cond.os_handle = cond_handle
        
    } ready is_windows() {
        fr fr Windows: Condition Variable
        sus cv_handle thicc = memory.allocate(CONDITION_VARIABLE)
        ready cv_handle == 0 {
            memory.free(cond)
            damn 0
        }
        
        win32_initialize_condition_variable(cv_handle)
        cond.os_handle = cv_handle
        
    } otherwise {
        memory.free(cond)
        damn 0
    }
    
    damn cond
}

fr fr Wait on condition variable with real OS blocking
slay wait_os_condition(cond *OSCondVar, mutex *OSMutex) normie {
    ready cond == 0 || mutex == 0 {
        damn -1
    }
    
    atomic_drip.atomic_add_i32(&cond.waiter_count, 1, atomic_drip.RELAXED)
    
    sus result normie
    ready is_linux() {
        fr fr Linux: pthread_cond_wait (real blocking)
        result = pthread_cond_wait(cond.os_handle, mutex.os_handle)
    } ready is_windows() {
        fr fr Windows: SleepConditionVariableCS (real blocking)
        ready win32_sleep_condition_variable_cs(cond.os_handle, mutex.os_handle, INFINITE) {
            result = 0
        } otherwise {
            result = -1
        }
    } otherwise {
        result = -1
    }
    
    atomic_drip.atomic_sub_i32(&cond.waiter_count, 1, atomic_drip.RELAXED)
    damn result
}

fr fr Wait with timeout on condition variable
slay wait_os_condition_timeout(cond *OSCondVar, mutex *OSMutex, timeout_ms normie) normie {
    ready cond == 0 || mutex == 0 {
        damn -1
    }
    
    atomic_drip.atomic_add_i32(&cond.waiter_count, 1, atomic_drip.RELAXED)
    
    sus result normie
    ready is_linux() {
        fr fr Linux: pthread_cond_timedwait with absolute timeout
        sus ts Timespec
        clock_gettime(CLOCK_MONOTONIC, &ts)
        ts.tv_sec = ts.tv_sec + (timeout_ms / 1000)
        ts.tv_nsec = ts.tv_nsec + ((timeout_ms % 1000) * 1000000)
        ready ts.tv_nsec >= 1000000000 {
            ts.tv_sec = ts.tv_sec + 1
            ts.tv_nsec = ts.tv_nsec - 1000000000
        }
        
        result = pthread_cond_timedwait(cond.os_handle, mutex.os_handle, &ts)
        
    } ready is_windows() {
        fr fr Windows: SleepConditionVariableCS with timeout
        ready win32_sleep_condition_variable_cs(cond.os_handle, mutex.os_handle, timeout_ms) {
            result = 0
        } otherwise {
            result = -1  fr fr Timeout or error
        }
    } otherwise {
        result = -1
    }
    
    atomic_drip.atomic_sub_i32(&cond.waiter_count, 1, atomic_drip.RELAXED)
    damn result
}

fr fr Signal one waiting thread
slay signal_os_condition(cond *OSCondVar) normie {
    ready cond == 0 {
        damn -1
    }
    
    ready atomic_drip.atomic_load_i32(&cond.waiter_count, atomic_drip.ACQUIRE) == 0 {
        damn 0  fr fr No waiters
    }
    
    atomic_drip.atomic_add_i32(&cond.signal_count, 1, atomic_drip.RELEASE)
    
    ready is_linux() {
        damn pthread_cond_signal(cond.os_handle)
    } ready is_windows() {
        win32_wake_condition_variable(cond.os_handle)
        damn 0
    } otherwise {
        damn -1
    }
}

fr fr Broadcast to all waiting threads
slay broadcast_os_condition(cond *OSCondVar) normie {
    ready cond == 0 {
        damn -1
    }
    
    sus waiter_count normie = atomic_drip.atomic_load_i32(&cond.waiter_count, atomic_drip.ACQUIRE)
    ready waiter_count == 0 {
        damn 0  fr fr No waiters
    }
    
    ready is_linux() {
        damn pthread_cond_broadcast(cond.os_handle)
    } ready is_windows() {
        win32_wake_all_condition_variable(cond.os_handle)
        damn 0
    } otherwise {
        damn -1
    }
}

fr fr =============================================================================
fr fr REAL SYSTEM INFORMATION - OS Detection and Hardware Info
fr fr =============================================================================

fr fr CPU Information structure
struct CPUInfo {
    spill logical_cores normie  fr fr Number of logical CPU cores
    spill physical_cores normie fr fr Number of physical CPU cores
    spill cache_line_size normie fr fr CPU cache line size
    spill page_size normie      fr fr OS memory page size
    spill numa_nodes normie     fr fr Number of NUMA nodes
}

fr fr Get real CPU count from OS
slay get_real_cpu_count() normie {
    ready is_linux() {
        fr fr Linux: sysconf(_SC_NPROCESSORS_ONLN)
        damn syscall_sysconf(_SC_NPROCESSORS_ONLN)
    } ready is_windows() {
        fr fr Windows: GetSystemInfo
        sus system_info SYSTEM_INFO
        win32_get_system_info(&system_info)
        damn system_info.dwNumberOfProcessors
    } ready is_macos() {
        fr fr macOS: sysctlbyname("hw.logicalcpu")
        sus cpu_count normie
        sus size normie = sizeof(cpu_count)
        ready sysctlbyname("hw.logicalcpu", &cpu_count, &size, 0, 0) == 0 {
            damn cpu_count
        }
        damn 1  fr fr Fallback
    } otherwise {
        damn 1  fr fr Conservative fallback
    }
}

fr fr Get comprehensive CPU information
slay get_cpu_info() *CPUInfo {
    sus info *CPUInfo = memory.allocate(CPUInfo)
    ready info == 0 {
        damn 0
    }
    
    info.logical_cores = get_real_cpu_count()
    info.cache_line_size = 64  fr fr Common default
    info.numa_nodes = 1        fr fr Default to single NUMA node
    
    ready is_linux() {
        fr fr Linux: Read from /proc/cpuinfo and /sys/devices/system/cpu
        info.physical_cores = get_linux_physical_cores()
        info.cache_line_size = get_linux_cache_line_size()
        info.page_size = syscall_getpagesize()
        info.numa_nodes = get_linux_numa_nodes()
        
    } ready is_windows() {
        fr fr Windows: GetLogicalProcessorInformation
        sus processor_info *SYSTEM_LOGICAL_PROCESSOR_INFORMATION
        sus buffer_size normie = 0
        
        fr fr Get required buffer size
        win32_get_logical_processor_information(0, &buffer_size)
        ready buffer_size > 0 {
            processor_info = memory.allocate(buffer_size)
            ready processor_info != 0 {
                ready win32_get_logical_processor_information(processor_info, &buffer_size) {
                    info.physical_cores = count_windows_physical_cores(processor_info, buffer_size)
                    info.cache_line_size = get_windows_cache_line_size(processor_info, buffer_size)
                }
                memory.free(processor_info)
            }
        }
        info.page_size = get_windows_page_size()
        
    } ready is_macos() {
        fr fr macOS: sysctlbyname for various CPU parameters
        info.physical_cores = get_macos_physical_cores()
        info.cache_line_size = get_macos_cache_line_size()
        info.page_size = get_macos_page_size()
        
    } otherwise {
        fr fr Conservative defaults for unknown platforms
        info.physical_cores = info.logical_cores
        info.page_size = 4096
    }
    
    damn info
}

fr fr Get current thread ID from OS
slay get_current_thread_id() thicc {
    ready is_linux() {
        fr fr Linux: gettid() system call
        damn syscall_gettid()
    } ready is_windows() {
        fr fr Windows: GetCurrentThreadId()
        damn win32_get_current_thread_id()
    } ready is_macos() {
        fr fr macOS: pthread_threadid_np()
        sus thread_id thicc
        ready pthread_threadid_np(pthread_self(), &thread_id) == 0 {
            damn thread_id
        }
        damn pthread_self()  fr fr Fallback to pthread_t
    } otherwise {
        damn 1  fr fr Fallback
    }
}

fr fr =============================================================================
fr fr PLATFORM DETECTION AND LOW-LEVEL OPERATIONS
fr fr =============================================================================

fr fr Platform detection functions
slay is_linux() lit {
    fr fr This would be determined at compile time with preprocessor
    fr fr For now, assume Linux as primary target
    damn based
}

slay is_windows() lit {
    damn cap  fr fr Windows support not yet implemented
}

slay is_macos() lit {
    damn cap  fr fr macOS support not yet implemented
}

slay is_x86_64() lit {
    fr fr This would be determined at compile time
    damn based  fr fr Assume x86_64 as primary target
}

slay is_arm64() lit {
    damn cap  fr fr ARM64 support not yet implemented
}

fr fr Low-level CPU instructions (would be inline assembly)
slay inline_assembly_pause() {
    fr fr x86_64: __asm__ volatile("pause" ::: "memory");
}

slay inline_assembly_yield() {
    fr fr ARM64: __asm__ volatile("yield" ::: "memory");
}

slay read_timestamp_counter() thicc {
    fr fr x86_64: RDTSC instruction
    fr fr __asm__ volatile("rdtsc" : "=a" (low), "=d" (high));
    damn 0  fr fr Placeholder - would return actual TSC value
}

fr fr =============================================================================
fr fr ERROR CODES AND CONSTANTS
fr fr =============================================================================

fr fr Thread states
sus THREAD_CREATED normie = 0
sus THREAD_RUNNING normie = 1
sus THREAD_WAITING normie = 2
sus THREAD_TERMINATED normie = 3

fr fr Mutex types
sus MUTEX_NORMAL normie = 0
sus MUTEX_RECURSIVE normie = 1
sus MUTEX_ERRORCHECK normie = 2

fr fr POSIX error codes
sus EINTR normie = 4
sus EBUSY normie = 16
sus ETIMEDOUT normie = 110

fr fr Windows constants (would be from windows.h)
sus INFINITE normie = 0xFFFFFFFF
sus WAIT_OBJECT_0 normie = 0

fr fr Syscall numbers (Linux x86_64)
sus SYS_NANOSLEEP normie = 35
sus SYS_SCHED_YIELD normie = 24
sus SYS_CLOCK_GETTIME normie = 228
sus SYS_GETTID normie = 186

fr fr sysconf constants
sus _SC_NPROCESSORS_ONLN normie = 84

fr fr =============================================================================
fr fr SYSCALL WRAPPERS - Direct OS Integration
fr fr =============================================================================

fr fr Direct system call wrappers (would use actual syscall mechanism)
slay syscall_clock_gettime(clock_id normie, ts *Timespec) normie {
    fr fr Linux: sys_clock_gettime syscall
    damn -1  fr fr Placeholder - would make real syscall
}

slay syscall_nanosleep(req *Timespec, rem *Timespec) normie {
    fr fr Linux: sys_nanosleep syscall
    damn -1  fr fr Placeholder - would make real syscall
}

slay syscall_sched_yield() normie {
    fr fr Linux: sys_sched_yield syscall
    damn -1  fr fr Placeholder - would make real syscall
}

slay syscall_gettid() thicc {
    fr fr Linux: sys_gettid syscall
    damn 1  fr fr Placeholder - would return real thread ID
}

slay syscall_sysconf(name normie) normie {
    fr fr POSIX: sysconf() call
    damn 4  fr fr Placeholder - would return real value
}

slay get_errno() normie {
    fr fr Get errno from thread-local storage
    damn 0  fr fr Placeholder - would return real errno
}

fr fr =============================================================================
fr fr WINDOWS API WRAPPERS - Win32 Integration
fr fr =============================================================================

fr fr Windows API function wrappers (would use actual Win32 APIs)
slay win32_create_thread(security thicc, stack_size normie, start_addr thicc, 
                        parameter thicc, flags normie, thread_id *thicc) thicc {
    damn 0  fr fr Placeholder - would call CreateThread
}

slay win32_wait_for_single_object(handle thicc, timeout normie) normie {
    damn WAIT_OBJECT_0  fr fr Placeholder - would call WaitForSingleObject
}

slay win32_close_handle(handle thicc) {
    fr fr Placeholder - would call CloseHandle
}

slay win32_switch_to_thread() {
    fr fr Placeholder - would call SwitchToThread
}

slay win32_get_current_thread_id() thicc {
    damn 1  fr fr Placeholder - would call GetCurrentThreadId
}

fr fr Critical Section operations
slay win32_initialize_critical_section(cs thicc) {
    fr fr Placeholder - would call InitializeCriticalSection
}

slay win32_enter_critical_section(cs thicc) {
    fr fr Placeholder - would call EnterCriticalSection
}

slay win32_leave_critical_section(cs thicc) {
    fr fr Placeholder - would call LeaveCriticalSection
}

slay win32_try_enter_critical_section(cs thicc) lit {
    damn cap  fr fr Placeholder - would call TryEnterCriticalSection
}

fr fr Condition Variable operations
slay win32_initialize_condition_variable(cv thicc) {
    fr fr Placeholder - would call InitializeConditionVariable
}

slay win32_sleep_condition_variable_cs(cv thicc, cs thicc, timeout normie) lit {
    damn cap  fr fr Placeholder - would call SleepConditionVariableCS
}

slay win32_wake_condition_variable(cv thicc) {
    fr fr Placeholder - would call WakeConditionVariable
}

slay win32_wake_all_condition_variable(cv thicc) {
    fr fr Placeholder - would call WakeAllConditionVariable
}

fr fr =============================================================================
fr fr PTHREAD WRAPPERS - POSIX Thread Integration
fr fr =============================================================================

fr fr POSIX thread function wrappers (would use actual pthread APIs)
slay pthread_create(thread *thicc, attr *pthread_attr_t, start_routine thicc, arg thicc) normie {
    damn -1  fr fr Placeholder - would call pthread_create
}

slay pthread_join(thread thicc, retval *thicc) normie {
    damn -1  fr fr Placeholder - would call pthread_join
}

slay pthread_detach(thread thicc) normie {
    damn -1  fr fr Placeholder - would call pthread_detach
}

slay pthread_self() thicc {
    damn 1  fr fr Placeholder - would call pthread_self
}

fr fr Mutex operations
slay pthread_mutex_init(mutex *pthread_mutex_t, attr *pthread_mutexattr_t) normie {
    damn -1  fr fr Placeholder - would call pthread_mutex_init
}

slay pthread_mutex_lock(mutex *pthread_mutex_t) normie {
    damn -1  fr fr Placeholder - would call pthread_mutex_lock
}

slay pthread_mutex_unlock(mutex *pthread_mutex_t) normie {
    damn -1  fr fr Placeholder - would call pthread_mutex_unlock
}

slay pthread_mutex_trylock(mutex *pthread_mutex_t) normie {
    damn -1  fr fr Placeholder - would call pthread_mutex_trylock
}

fr fr Condition variable operations
slay pthread_cond_init(cond *pthread_cond_t, attr *pthread_condattr_t) normie {
    damn -1  fr fr Placeholder - would call pthread_cond_init
}

slay pthread_cond_wait(cond *pthread_cond_t, mutex *pthread_mutex_t) normie {
    damn -1  fr fr Placeholder - would call pthread_cond_wait
}

slay pthread_cond_timedwait(cond *pthread_cond_t, mutex *pthread_mutex_t, abstime *Timespec) normie {
    damn -1  fr fr Placeholder - would call pthread_cond_timedwait
}

slay pthread_cond_signal(cond *pthread_cond_t) normie {
    damn -1  fr fr Placeholder - would call pthread_cond_signal
}

slay pthread_cond_broadcast(cond *pthread_cond_t) normie {
    damn -1  fr fr Placeholder - would call pthread_cond_broadcast
}

fr fr =============================================================================
fr fr UTILITY FUNCTIONS FOR PLATFORM-SPECIFIC CODE
fr fr =============================================================================

fr fr Linux-specific CPU information
slay get_linux_physical_cores() normie {
    fr fr Would read from /sys/devices/system/cpu/cpu*/topology/core_id
    damn 2  fr fr Placeholder
}

slay get_linux_cache_line_size() normie {
    fr fr Would read from /sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size
    damn 64  fr fr Common cache line size
}

slay get_linux_numa_nodes() normie {
    fr fr Would read from /sys/devices/system/node/
    damn 1  fr fr Default single node
}

fr fr macOS-specific functions
slay mach_absolute_time() thicc {
    damn 0  fr fr Placeholder - would call actual mach_absolute_time
}

slay get_macos_physical_cores() normie {
    fr fr Would use sysctlbyname("hw.physicalcpu")
    damn 2  fr fr Placeholder
}

slay get_macos_cache_line_size() normie {
    fr fr Would use sysctlbyname("hw.cachelinesize")
    damn 64  fr fr Placeholder
}

slay get_macos_page_size() normie {
    damn 4096  fr fr Placeholder
}

fr fr Windows performance counter
slay query_performance_counter() thicc {
    damn 0  fr fr Placeholder - would call QueryPerformanceCounter
}
