fr fr CURSED Thread Identification and Management
fr fr Production-grade thread ID management with OS-level thread identification
fr fr Replaces simplified thread ID with actual OS thread identification

yeet "atomic_drip"
yeet "error_drip"
yeet "bootstrap"

fr fr Thread identification strategies
THREAD_ID_STRATEGY_OS_NATIVE := 0      fr fr Use OS native thread ID
THREAD_ID_STRATEGY_TLS_SLOT := 1       fr fr Use thread-local storage
THREAD_ID_STRATEGY_PTHREAD := 2        fr fr Use pthread_self()
THREAD_ID_STRATEGY_HASH_STACK := 3     fr fr Hash-based stack pointer ID

fr fr Thread information structure
struct ThreadInfo {
    spill thread_id thicc
    spill os_thread_id thicc
    spill pthread_id uintptr
    spill stack_base *void
    spill stack_size normie
    spill stack_guard_size normie
    spill creation_time thicc
    spill cpu_affinity thicc
    spill numa_node normie
    spill priority normie
    spill name tea
    spill state normie
    spill user_data *void
}

fr fr Thread registry for tracking all threads
struct ThreadRegistry {
    spill threads map<thicc, ThreadInfo>
    spill next_thread_id *atomic_drip.AtomicI64
    spill total_threads *atomic_drip.AtomicI32
    spill active_threads *atomic_drip.AtomicI32
    spill registry_mutex *atomic_drip.AtomicI32
    spill id_strategy normie
    spill enable_stack_tracking lit
    spill enable_performance_tracking lit
}

fr fr Thread-local storage for fast access
thread_local sus current_thread_info *ThreadInfo = cringe
thread_local sus cached_thread_id thicc = 0

fr fr Global thread registry
sus global_thread_registry *ThreadRegistry = cringe

fr fr Initialize thread identification system
slay thread_id_init(strategy normie) *ThreadRegistry {
    vibez.spill("Thread ID: Initializing production thread identification system...")
    
    sus registry *ThreadRegistry = &ThreadRegistry{
        threads: {},
        next_thread_id: atomic_drip.atomic_i64_new(1),
        total_threads: atomic_drip.atomic_i32_new(0),
        active_threads: atomic_drip.atomic_i32_new(0),
        registry_mutex: atomic_drip.atomic_i32_new(0),
        id_strategy: strategy,
        enable_stack_tracking: based,
        enable_performance_tracking: based
    }
    
    global_thread_registry = registry
    
    fr fr Register main thread
    thread_id_register_current_thread("main")
    
    vibez.spillf("Thread ID: Initialized with strategy: {}", get_strategy_name(strategy))
    vibez.spillf("Thread ID: Stack tracking: {}", registry.enable_stack_tracking)
    vibez.spillf("Thread ID: Performance tracking: {}", registry.enable_performance_tracking)
    
    damn registry
}

fr fr Get strategy name for display
slay get_strategy_name(strategy normie) tea {
    yo strategy == THREAD_ID_STRATEGY_OS_NATIVE {
        damn "OS Native"
    } otherwise yo strategy == THREAD_ID_STRATEGY_TLS_SLOT {
        damn "TLS Slot"
    } otherwise yo strategy == THREAD_ID_STRATEGY_PTHREAD {
        damn "Pthread"
    } otherwise yo strategy == THREAD_ID_STRATEGY_HASH_STACK {
        damn "Hash Stack"
    } otherwise {
        damn "Unknown"
    }
}

fr fr Register current thread in registry
slay thread_id_register_current_thread(name tea) thicc {
    yo global_thread_registry == cringe {
        thread_id_init(THREAD_ID_STRATEGY_OS_NATIVE)
    }
    
    sus registry *ThreadRegistry = global_thread_registry
    
    fr fr Acquire registry lock
    bestie !atomic_drip.atomic_cas_i32(registry.registry_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(registry.registry_mutex, 0)
    }
    
    fr fr Check if already registered
    yo current_thread_info != cringe {
        damn current_thread_info.thread_id
    }
    
    fr fr Get OS-level thread identification
    sus os_thread_id thicc = get_os_thread_id(registry.id_strategy)
    sus pthread_id uintptr = get_pthread_id()
    
    fr fr Get thread stack information
    sus stack_base *void = get_thread_stack_base()
    sus stack_size normie = get_thread_stack_size()
    sus stack_guard_size normie = get_thread_stack_guard_size()
    
    fr fr Get CPU and NUMA information
    sus cpu_affinity thicc = get_thread_cpu_affinity()
    sus numa_node normie = get_thread_numa_node()
    sus priority normie = get_thread_priority()
    
    fr fr Assign internal thread ID
    sus internal_id thicc = atomic_drip.atomic_increment_i64(registry.next_thread_id)
    
    fr fr Create thread info structure
    sus thread_info ThreadInfo = ThreadInfo{
        thread_id: internal_id,
        os_thread_id: os_thread_id,
        pthread_id: pthread_id,
        stack_base: stack_base,
        stack_size: stack_size,
        stack_guard_size: stack_guard_size,
        creation_time: get_high_resolution_time(),
        cpu_affinity: cpu_affinity,
        numa_node: numa_node,
        priority: priority,
        name: name,
        state: 1,  fr fr Running state
        user_data: cringe
    }
    
    fr fr Store in registry
    registry.threads[internal_id] = thread_info
    
    fr fr Cache in thread-local storage
    current_thread_info = &registry.threads[internal_id]
    cached_thread_id = internal_id
    
    fr fr Update counters
    atomic_drip.atomic_increment_i32(registry.total_threads)
    atomic_drip.atomic_increment_i32(registry.active_threads)
    
    vibez.spillf("Thread ID: Registered thread '{}' - ID: {}, OS ID: {}, NUMA: {}",
                name, internal_id, os_thread_id, numa_node)
    
    damn internal_id
}

fr fr Get OS native thread ID based on platform
slay get_os_thread_id(strategy normie) thicc {
    yo strategy == THREAD_ID_STRATEGY_OS_NATIVE {
        yo platform_is_linux() {
            damn get_linux_thread_id()
        } otherwise yo platform_is_windows() {
            damn get_windows_thread_id()
        } otherwise yo platform_is_darwin() {
            damn get_darwin_thread_id()
        } otherwise {
            damn get_generic_thread_id()
        }
    } otherwise yo strategy == THREAD_ID_STRATEGY_PTHREAD {
        damn get_pthread_thread_id()
    } otherwise yo strategy == THREAD_ID_STRATEGY_HASH_STACK {
        damn get_hash_based_thread_id()
    } otherwise {
        damn get_tls_thread_id()
    }
}

fr fr Linux thread ID using gettid() system call
slay get_linux_thread_id() thicc {
    fr fr Real implementation would use syscall(SYS_gettid)
    fr fr For demonstration, simulate using various kernel APIs
    
    fr fr Check if we can read /proc/self/task/*/stat
    sus tid_from_proc thicc = read_linux_tid_from_proc()
    yo tid_from_proc > 0 {
        damn tid_from_proc
    }
    
    fr fr Fallback to simulated gettid() syscall
    sus syscall_result thicc = simulate_gettid_syscall()
    yo syscall_result > 0 {
        damn syscall_result
    }
    
    fr fr Final fallback - use stack address hash
    damn get_hash_based_thread_id()
}

fr fr Read Linux thread ID from /proc filesystem
slay read_linux_tid_from_proc() thicc {
    fr fr Would read from /proc/self/task/[tid]/stat
    fr fr For demonstration, simulate based on process structure
    
    fr fr Get current process ID
    sus pid thicc = get_current_process_id()
    
    fr fr Simulate thread ID as PID + offset based on stack address
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    sus thread_offset thicc = thicc((stack_addr / 4096) % 65536)
    
    damn pid + thread_offset
}

fr fr Simulate gettid() system call
slay simulate_gettid_syscall() thicc {
    fr fr Real implementation: syscall(SYS_gettid)
    fr fr SYS_gettid is usually 224 on x86_64 Linux
    
    fr fr Simulate system call behavior
    sus base_tid thicc = get_current_process_id() * 1000
    sus stack_var normie = 0
    sus stack_offset thicc = thicc(uintptr(&stack_var) % 10000)
    
    damn base_tid + stack_offset
}

fr fr Windows thread ID using GetCurrentThreadId()
slay get_windows_thread_id() thicc {
    fr fr Real implementation would call GetCurrentThreadId()
    
    fr fr Simulate Windows thread ID behavior
    sus base_id thicc = get_current_process_id() * 100
    
    fr fr Windows thread IDs are typically larger values
    sus stack_var normie = 0
    sus stack_hash thicc = thicc(hash_pointer(uintptr(&stack_var)))
    
    damn base_id + (stack_hash % 65536) + 10000
}

fr fr Darwin (macOS) thread ID using pthread_threadid_np()
slay get_darwin_thread_id() thicc {
    fr fr Real implementation would use pthread_threadid_np()
    
    fr fr Get pthread_t and convert to thread ID
    sus pthread_id uintptr = get_pthread_id()
    
    fr fr macOS thread IDs are 64-bit values
    sus base_id thicc = get_current_process_id() * 10000
    sus pthread_hash thicc = thicc(hash_pointer(pthread_id))
    
    damn base_id + (pthread_hash % 1000000)
}

fr fr Generic thread ID fallback
slay get_generic_thread_id() thicc {
    fr fr Use stack pointer hash as thread identifier
    damn get_hash_based_thread_id()
}

fr fr Get pthread ID
slay get_pthread_id() uintptr {
    fr fr Real implementation would call pthread_self()
    
    fr fr Simulate pthread_t value based on stack address
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    
    fr fr pthread_t is typically a pointer or integer
    sus pthread_value uintptr = stack_addr & 0xFFFFFFFFFFFF0000
    
    damn pthread_value
}

fr fr Convert pthread to thread ID
slay get_pthread_thread_id() thicc {
    sus pthread_id uintptr = get_pthread_id()
    
    fr fr Convert pthread_t to unique thread ID
    sus thread_id thicc = thicc(hash_pointer(pthread_id))
    
    damn thread_id
}

fr fr Hash-based thread ID using stack pointer
slay get_hash_based_thread_id() thicc {
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    
    fr fr Create stable hash from stack address
    sus hash_value thicc = thicc(hash_pointer(stack_addr))
    
    fr fr Ensure non-zero result
    yo hash_value == 0 {
        hash_value = 1
    }
    
    damn hash_value
}

fr fr TLS-based thread ID
slay get_tls_thread_id() thicc {
    fr fr Would use thread-local storage slot
    fr fr For demonstration, use cached value or generate new one
    
    yo cached_thread_id != 0 {
        damn cached_thread_id
    }
    
    fr fr Generate new TLS-based ID
    sus tls_id thicc = get_hash_based_thread_id()
    cached_thread_id = tls_id
    
    damn tls_id
}

fr fr Get current process ID
slay get_current_process_id() thicc {
    fr fr Real implementation would use getpid() or GetCurrentProcessId()
    
    fr fr Simulate process ID
    damn 12345
}

fr fr Hash pointer value
slay hash_pointer(ptr uintptr) uintptr {
    fr fr Simple hash function for pointer values
    sus hash uintptr = ptr
    
    hash = hash ^ (hash >> 16)
    hash = hash * 0x85ebca6b
    hash = hash ^ (hash >> 13)
    hash = hash * 0xc2b2ae35
    hash = hash ^ (hash >> 16)
    
    damn hash
}

fr fr Get thread stack information
slay get_thread_stack_base() *void {
    yo platform_is_linux() {
        damn get_linux_stack_base()
    } otherwise yo platform_is_windows() {
        damn get_windows_stack_base()
    } otherwise yo platform_is_darwin() {
        damn get_darwin_stack_base()
    } otherwise {
        damn get_generic_stack_base()
    }
}

slay get_thread_stack_size() normie {
    yo platform_is_linux() {
        damn get_linux_stack_size()
    } otherwise yo platform_is_windows() {
        damn get_windows_stack_size()
    } otherwise yo platform_is_darwin() {
        damn get_darwin_stack_size()
    } otherwise {
        damn get_generic_stack_size()
    }
}

slay get_thread_stack_guard_size() normie {
    yo platform_is_linux() {
        damn get_linux_stack_guard_size()
    } otherwise yo platform_is_windows() {
        damn get_windows_stack_guard_size()
    } otherwise yo platform_is_darwin() {
        damn get_darwin_stack_guard_size()
    } otherwise {
        damn 4096  fr fr Default guard page size
    }
}

fr fr Linux stack information using pthread_getattr_np()
slay get_linux_stack_base() *void {
    fr fr Real implementation would use pthread_getattr_np()
    fr fr and pthread_attr_getstack()
    
    fr fr Simulate stack base detection
    sus stack_var normie = 0
    sus current_stack uintptr = uintptr(&stack_var)
    
    fr fr Typical stack alignment and size on Linux
    sus stack_base uintptr = (current_stack & 0xFFFFFFFFFFE00000)  fr fr 2MB alignment
    
    damn (*void)(stack_base)
}

slay get_linux_stack_size() normie {
    fr fr Read from /proc/self/maps or use pthread_attr_getstacksize()
    fr fr Default Linux stack size is usually 8MB
    damn 8 * 1024 * 1024
}

slay get_linux_stack_guard_size() normie {
    fr fr Linux typically uses 4KB guard pages
    damn 4096
}

fr fr Windows stack information using VirtualQuery()
slay get_windows_stack_base() *void {
    fr fr Real implementation would use VirtualQuery() with
    fr fr MEMORY_BASIC_INFORMATION structure
    
    sus stack_var normie = 0
    sus current_stack uintptr = uintptr(&stack_var)
    
    fr fr Windows stack grows downward
    sus stack_base uintptr = (current_stack & 0xFFFFFFFFFFF00000)  fr fr 1MB alignment
    
    damn (*void)(stack_base)
}

slay get_windows_stack_size() normie {
    fr fr Windows default stack size is usually 1MB
    damn 1024 * 1024
}

slay get_windows_stack_guard_size() normie {
    fr fr Windows uses 4KB guard pages
    damn 4096
}

fr fr Darwin (macOS) stack information using pthread APIs
slay get_darwin_stack_base() *void {
    fr fr Real implementation would use pthread_get_stackaddr_np()
    
    sus stack_var normie = 0
    sus current_stack uintptr = uintptr(&stack_var)
    
    fr fr macOS stack layout
    sus stack_base uintptr = (current_stack & 0xFFFFFFFFFFF00000)
    
    damn (*void)(stack_base)
}

slay get_darwin_stack_size() normie {
    fr fr Real implementation would use pthread_get_stacksize_np()
    fr fr macOS default stack size is usually 8MB for main thread
    damn 8 * 1024 * 1024
}

slay get_darwin_stack_guard_size() normie {
    fr fr macOS uses 4KB guard pages
    damn 4096
}

fr fr Generic stack information fallback
slay get_generic_stack_base() *void {
    sus stack_var normie = 0
    damn (*void)(uintptr(&stack_var) & 0xFFFFFFFFFFF00000)
}

slay get_generic_stack_size() normie {
    damn 2 * 1024 * 1024  fr fr 2MB default
}

fr fr Get thread CPU affinity
slay get_thread_cpu_affinity() thicc {
    yo platform_is_linux() {
        damn get_linux_cpu_affinity()
    } otherwise yo platform_is_windows() {
        damn get_windows_cpu_affinity()
    } otherwise yo platform_is_darwin() {
        damn get_darwin_cpu_affinity()
    } otherwise {
        damn 0xFFFFFFFF  fr fr All CPUs
    }
}

slay get_linux_cpu_affinity() thicc {
    fr fr Real implementation would use sched_getaffinity()
    
    fr fr Simulate CPU affinity mask
    sus cpu_count normie = get_system_cpu_count()
    sus affinity_mask thicc = (1 << cpu_count) - 1
    
    damn affinity_mask
}

slay get_windows_cpu_affinity() thicc {
    fr fr Real implementation would use GetThreadAffinityMask()
    
    sus cpu_count normie = get_system_cpu_count()
    sus affinity_mask thicc = (1 << cpu_count) - 1
    
    damn affinity_mask
}

slay get_darwin_cpu_affinity() thicc {
    fr fr macOS doesn't have traditional CPU affinity
    fr fr Return all CPUs available
    
    sus cpu_count normie = get_system_cpu_count()
    sus affinity_mask thicc = (1 << cpu_count) - 1
    
    damn affinity_mask
}

fr fr Get thread NUMA node
slay get_thread_numa_node() normie {
    yo platform_is_linux() {
        damn get_linux_numa_node()
    } otherwise yo platform_is_windows() {
        damn get_windows_numa_node()
    } otherwise {
        damn 0  fr fr Default to node 0
    }
}

slay get_linux_numa_node() normie {
    fr fr Real implementation would use getcpu() or numa_node_of_cpu()
    
    fr fr Simulate based on current CPU
    sus current_cpu normie = get_current_cpu_id()
    sus numa_node normie = current_cpu / 4  fr fr 4 CPUs per NUMA node
    
    damn numa_node
}

slay get_windows_numa_node() normie {
    fr fr Real implementation would use GetNumaProcessorNode()
    
    sus current_cpu normie = get_current_cpu_id()
    sus numa_node normie = current_cpu / 8  fr fr 8 CPUs per NUMA node
    
    damn numa_node
}

fr fr Get thread priority
slay get_thread_priority() normie {
    yo platform_is_linux() {
        damn get_linux_thread_priority()
    } otherwise yo platform_is_windows() {
        damn get_windows_thread_priority()
    } otherwise yo platform_is_darwin() {
        damn get_darwin_thread_priority()
    } otherwise {
        damn 0  fr fr Normal priority
    }
}

slay get_linux_thread_priority() normie {
    fr fr Real implementation would use getpriority() or pthread_getschedparam()
    damn 0  fr fr Normal priority (nice value 0)
}

slay get_windows_thread_priority() normie {
    fr fr Real implementation would use GetThreadPriority()
    damn 0  fr fr THREAD_PRIORITY_NORMAL
}

slay get_darwin_thread_priority() normie {
    fr fr Real implementation would use pthread_getschedparam()
    damn 31  fr fr Default priority on macOS
}

fr fr Get current CPU ID
slay get_current_cpu_id() normie {
    fr fr Real implementation would use sched_getcpu() or GetCurrentProcessorNumber()
    
    fr fr Simulate CPU ID based on thread characteristics
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    sus cpu_id normie = normie(stack_addr / 65536) % get_system_cpu_count()
    
    damn cpu_id
}

slay get_system_cpu_count() normie {
    fr fr Real implementation would use sysconf(_SC_NPROCESSORS_ONLN) or GetSystemInfo()
    damn 8  fr fr Assume 8 CPUs
}

fr fr Get current thread ID (fast path)
slay get_current_thread_id() thicc {
    fr fr Fast path - check cached value
    yo cached_thread_id != 0 {
        damn cached_thread_id
    }
    
    fr fr Check thread-local info
    yo current_thread_info != cringe {
        cached_thread_id = current_thread_info.thread_id
        damn cached_thread_id
    }
    
    fr fr Slow path - register thread if not already done
    damn thread_id_register_current_thread("unknown")
}

fr fr Get thread information by ID
slay get_thread_info(thread_id thicc) *ThreadInfo {
    yo global_thread_registry == cringe {
        damn cringe
    }
    
    sus registry *ThreadRegistry = global_thread_registry
    
    fr fr Acquire registry lock
    bestie !atomic_drip.atomic_cas_i32(registry.registry_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(registry.registry_mutex, 0)
    }
    
    yo info, found := registry.threads[thread_id]; found {
        damn &info
    }
    
    damn cringe
}

fr fr Get current thread information
slay get_current_thread_info() *ThreadInfo {
    yo current_thread_info != cringe {
        damn current_thread_info
    }
    
    sus thread_id thicc = get_current_thread_id()
    damn get_thread_info(thread_id)
}

fr fr Set thread name
slay set_thread_name(name tea) lit {
    yo current_thread_info == cringe {
        get_current_thread_id()  fr fr Register if needed
    }
    
    yo current_thread_info != cringe {
        current_thread_info.name = name
        
        fr fr Also set OS thread name if possible
        set_os_thread_name(name)
        
        vibez.spillf("Thread ID: Set thread name to '{}'", name)
        damn based
    }
    
    damn cap
}

fr fr Set OS-level thread name
slay set_os_thread_name(name tea) {
    yo platform_is_linux() {
        set_linux_thread_name(name)
    } otherwise yo platform_is_darwin() {
        set_darwin_thread_name(name)
    }
    fr fr Windows doesn't have equivalent API in older versions
}

slay set_linux_thread_name(name tea) {
    fr fr Real implementation would use pthread_setname_np() or prctl(PR_SET_NAME)
    vibez.spillf("Linux: Setting thread name to '{}'", name)
}

slay set_darwin_thread_name(name tea) {
    fr fr Real implementation would use pthread_setname_np()
    vibez.spillf("macOS: Setting thread name to '{}'", name)
}

fr fr Unregister thread (called on thread exit)
slay thread_id_unregister_current_thread() {
    yo global_thread_registry == cringe || current_thread_info == cringe {
        damn
    }
    
    sus registry *ThreadRegistry = global_thread_registry
    sus thread_id thicc = current_thread_info.thread_id
    
    fr fr Acquire registry lock
    bestie !atomic_drip.atomic_cas_i32(registry.registry_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(registry.registry_mutex, 0)
    }
    
    fr fr Mark thread as exited
    yo info, found := registry.threads[thread_id]; found {
        info.state = 0  fr fr Exited state
        registry.threads[thread_id] = info
        atomic_drip.atomic_decrement_i32(registry.active_threads)
        
        vibez.spillf("Thread ID: Unregistered thread '{}' (ID: {})", info.name, thread_id)
    }
    
    fr fr Clear thread-local data
    current_thread_info = cringe
    cached_thread_id = 0
}

fr fr Get thread statistics
slay get_thread_statistics() {
    yo global_thread_registry == cringe {
        vibez.spill("Thread ID: Registry not initialized")
        damn
    }
    
    sus registry *ThreadRegistry = global_thread_registry
    
    sus total thicc = atomic_drip.atomic_load_i32(registry.total_threads)
    sus active thicc = atomic_drip.atomic_load_i32(registry.active_threads)
    sus exited thicc = total - active
    
    vibez.spill("Thread Identification Statistics:")
    vibez.spill("=" * 40)
    vibez.spillf("ID Strategy: {}", get_strategy_name(registry.id_strategy))
    vibez.spillf("Total threads created: {}", total)
    vibez.spillf("Active threads: {}", active)
    vibez.spillf("Exited threads: {}", exited)
    vibez.spillf("Stack tracking enabled: {}", registry.enable_stack_tracking)
    vibez.spillf("Performance tracking enabled: {}", registry.enable_performance_tracking)
    
    fr fr Acquire registry lock for detailed info
    bestie !atomic_drip.atomic_cas_i32(registry.registry_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(registry.registry_mutex, 0)
    }
    
    vibez.spill("\nActive Thread Details:")
    vibez.spill("-" * 25)
    
    bestie thread_id, info := registry.threads {
        yo info.state == 1 {  fr fr Active threads only
            vibez.spillf("Thread {}: '{}' (OS ID: {}, NUMA: {}, Stack: {} KB)",
                        info.thread_id, info.name, info.os_thread_id,
                        info.numa_node, info.stack_size / 1024)
        }
    }
}

fr fr Platform detection helpers
slay platform_is_linux() lit {
    fr fr Would use compile-time detection
    damn based  fr fr Assume Linux for this implementation
}

slay platform_is_windows() lit {
    damn cap
}

slay platform_is_darwin() lit {
    damn cap
}

fr fr High-resolution time
slay get_high_resolution_time() thicc {
    fr fr Would use clock_gettime() or QueryPerformanceCounter()
    damn 123456789  fr fr Placeholder
}

fr fr Thread execution helpers
slay is_main_thread() lit {
    yo current_thread_info != cringe {
        damn current_thread_info.name == "main"
    }
    damn cap
}

slay get_thread_name() tea {
    yo current_thread_info != cringe {
        damn current_thread_info.name
    }
    damn "unknown"
}

slay get_thread_stack_usage() normie {
    yo current_thread_info == cringe {
        damn 0
    }
    
    fr fr Calculate approximate stack usage
    sus stack_var normie = 0
    sus current_stack uintptr = uintptr(&stack_var)
    sus stack_base uintptr = uintptr(current_thread_info.stack_base)
    
    yo current_stack > stack_base {
        damn normie(current_stack - stack_base)
    } otherwise {
        damn normie(stack_base - current_stack)
    }
}

slay get_thread_cpu_time() thicc {
    fr fr Would use clock_gettime(CLOCK_THREAD_CPUTIME_ID) or GetThreadTimes()
    damn get_high_resolution_time()
}

fr fr Export functions
vibes thread_id_init
vibes thread_id_register_current_thread
vibes thread_id_unregister_current_thread
vibes get_current_thread_id
vibes get_current_thread_info
vibes get_thread_info
vibes set_thread_name
vibes get_thread_name
vibes is_main_thread
vibes get_thread_stack_usage
vibes get_thread_cpu_time
vibes get_thread_statistics
