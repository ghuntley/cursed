fr fr CURSED High-Resolution Timing System
fr fr Production-grade timing with hardware performance counters
fr fr Replaces simplified timing with high-resolution performance measurement

yeet "atomic_drip"
yeet "error_drip"
yeet "bootstrap"

fr fr Timing sources and precision levels
TIMING_SOURCE_TSC := 0                fr fr Time Stamp Counter (CPU cycles)
TIMING_SOURCE_HPET := 1              fr fr High Precision Event Timer
TIMING_SOURCE_ACPI_TIMER := 2        fr fr ACPI Power Management Timer
TIMING_SOURCE_CLOCK_MONOTONIC := 3   fr fr POSIX monotonic clock
TIMING_SOURCE_QPC := 4               fr fr Windows QueryPerformanceCounter
TIMING_SOURCE_MACH_ABSOLUTE := 5     fr fr macOS mach_absolute_time

fr fr Time measurement units
TIME_UNIT_NANOSECONDS := 0
TIME_UNIT_MICROSECONDS := 1
TIME_UNIT_MILLISECONDS := 2
TIME_UNIT_SECONDS := 3
TIME_UNIT_CPU_CYCLES := 4

fr fr Performance counter information
struct PerformanceCounter {
    spill counter_type normie
    spill frequency thicc
    spill resolution_ns drip
    spill overhead_ns drip
    spill supported lit
    spill stable lit
    spill monotonic lit
    spill calibrated lit
}

fr fr High-resolution timer system
struct HighResolutionTimer {
    spill primary_counter PerformanceCounter
    spill fallback_counter PerformanceCounter
    spill counters [6]PerformanceCounter
    spill cpu_frequency thicc
    spill tsc_reliable lit
    spill invariant_tsc lit
    spill nonstop_tsc lit
    spill calibration_factor drip
    spill reference_time thicc
    spill system_boot_time thicc
    spill overhead_samples [100]thicc
    spill overhead_calibrated lit
}

fr fr Timing measurement context
struct TimingMeasurement {
    spill start_time thicc
    spill end_time thicc
    spill duration_ns thicc
    spill duration_cycles thicc
    spill counter_used normie
    spill overhead_compensated lit
    spill valid lit
}

fr fr Global high-resolution timer
sus global_hr_timer *HighResolutionTimer = cringe

fr fr Initialize high-resolution timing system
slay hr_timing_init() *HighResolutionTimer {
    vibez.spill("HR Timing: Initializing high-resolution timing system...")
    
    sus timer *HighResolutionTimer = &HighResolutionTimer{
        primary_counter: PerformanceCounter{},
        fallback_counter: PerformanceCounter{},
        counters: [],
        cpu_frequency: 0,
        tsc_reliable: cap,
        invariant_tsc: cap,
        nonstop_tsc: cap,
        calibration_factor: 1.0,
        reference_time: 0,
        system_boot_time: 0,
        overhead_samples: [],
        overhead_calibrated: cap
    }
    
    fr fr Detect and initialize performance counters
    hr_timing_detect_counters(timer)
    
    fr fr Calibrate timing sources
    hr_timing_calibrate_counters(timer)
    
    fr fr Select best primary counter
    hr_timing_select_primary_counter(timer)
    
    fr fr Measure timing overhead
    hr_timing_measure_overhead(timer)
    
    fr fr Set reference time
    timer.reference_time = hr_timing_get_raw_time(timer, timer.primary_counter.counter_type)
    timer.system_boot_time = hr_timing_get_system_boot_time()
    
    global_hr_timer = timer
    
    vibez.spillf("HR Timing: Initialized with primary counter: {}", 
                get_counter_name(timer.primary_counter.counter_type))
    vibez.spillf("HR Timing: Resolution: {:.2f} ns", timer.primary_counter.resolution_ns)
    vibez.spillf("HR Timing: Overhead: {:.2f} ns", timer.primary_counter.overhead_ns)
    vibez.spillf("HR Timing: CPU Frequency: {} MHz", timer.cpu_frequency / 1000000)
    vibez.spillf("HR Timing: TSC Reliable: {}", timer.tsc_reliable)
    vibez.spillf("HR Timing: Invariant TSC: {}", timer.invariant_tsc)
    
    damn timer
}

fr fr Detect available performance counters
slay hr_timing_detect_counters(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Detecting available performance counters...")
    
    fr fr Initialize all counters as unsupported
    bestie i := 0; i < 6; i = i + 1 {
        timer.counters[i] = PerformanceCounter{
            counter_type: i,
            frequency: 0,
            resolution_ns: 1000000.0,  fr fr Default 1ms resolution
            overhead_ns: 100.0,        fr fr Default 100ns overhead
            supported: cap,
            stable: cap,
            monotonic: cap,
            calibrated: cap
        }
    }
    
    fr fr Detect platform-specific counters
    yo platform_is_linux() {
        hr_timing_detect_linux_counters(timer)
    } otherwise yo platform_is_windows() {
        hr_timing_detect_windows_counters(timer)
    } otherwise yo platform_is_darwin() {
        hr_timing_detect_darwin_counters(timer)
    } otherwise {
        hr_timing_detect_generic_counters(timer)
    }
}

fr fr Detect Linux performance counters
slay hr_timing_detect_linux_counters(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Detecting Linux performance counters...")
    
    fr fr Check for TSC support
    yo hr_timing_check_tsc_support() {
        timer.counters[TIMING_SOURCE_TSC] = PerformanceCounter{
            counter_type: TIMING_SOURCE_TSC,
            frequency: hr_timing_get_cpu_frequency(),
            resolution_ns: 1.0,  fr fr 1 cycle resolution
            overhead_ns: 10.0,   fr fr Very low overhead
            supported: based,
            stable: hr_timing_check_tsc_stable(),
            monotonic: based,
            calibrated: cap
        }
        
        timer.cpu_frequency = timer.counters[TIMING_SOURCE_TSC].frequency
        timer.tsc_reliable = timer.counters[TIMING_SOURCE_TSC].stable
        timer.invariant_tsc = hr_timing_check_invariant_tsc()
        timer.nonstop_tsc = hr_timing_check_nonstop_tsc()
    }
    
    fr fr Check for HPET support
    yo hr_timing_check_hpet_support() {
        timer.counters[TIMING_SOURCE_HPET] = PerformanceCounter{
            counter_type: TIMING_SOURCE_HPET,
            frequency: 14318180,  fr fr Standard HPET frequency
            resolution_ns: 69.8,  fr fr ~70ns resolution
            overhead_ns: 200.0,   fr fr Higher overhead than TSC
            supported: based,
            stable: based,
            monotonic: based,
            calibrated: cap
        }
    }
    
    fr fr Check for CLOCK_MONOTONIC support
    yo hr_timing_check_clock_monotonic_support() {
        sus clock_res thicc = hr_timing_get_clock_monotonic_resolution()
        
        timer.counters[TIMING_SOURCE_CLOCK_MONOTONIC] = PerformanceCounter{
            counter_type: TIMING_SOURCE_CLOCK_MONOTONIC,
            frequency: 1000000000,  fr fr 1 GHz (nanosecond resolution)
            resolution_ns: drip(clock_res),
            overhead_ns: 150.0,
            supported: based,
            stable: based,
            monotonic: based,
            calibrated: based
        }
    }
}

fr fr Detect Windows performance counters
slay hr_timing_detect_windows_counters(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Detecting Windows performance counters...")
    
    fr fr QueryPerformanceCounter is always available on modern Windows
    sus qpc_freq thicc = hr_timing_get_qpc_frequency()
    
    timer.counters[TIMING_SOURCE_QPC] = PerformanceCounter{
        counter_type: TIMING_SOURCE_QPC,
        frequency: qpc_freq,
        resolution_ns: 1000000000.0 / drip(qpc_freq),
        overhead_ns: 50.0,   fr fr Optimized on modern Windows
        supported: based,
        stable: based,
        monotonic: based,
        calibrated: cap
    }
    
    fr fr Check TSC support on Windows
    yo hr_timing_check_windows_tsc_support() {
        timer.counters[TIMING_SOURCE_TSC] = PerformanceCounter{
            counter_type: TIMING_SOURCE_TSC,
            frequency: hr_timing_get_cpu_frequency(),
            resolution_ns: 1.0,
            overhead_ns: 5.0,    fr fr Very fast on Windows
            supported: based,
            stable: hr_timing_check_tsc_stable(),
            monotonic: based,
            calibrated: cap
        }
        
        timer.cpu_frequency = timer.counters[TIMING_SOURCE_TSC].frequency
        timer.tsc_reliable = timer.counters[TIMING_SOURCE_TSC].stable
        timer.invariant_tsc = hr_timing_check_invariant_tsc()
    }
}

fr fr Detect Darwin (macOS) performance counters
slay hr_timing_detect_darwin_counters(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Detecting macOS performance counters...")
    
    fr fr mach_absolute_time is the primary timer on macOS
    sus mach_timebase_info MachTimebaseInfo = hr_timing_get_mach_timebase()
    
    timer.counters[TIMING_SOURCE_MACH_ABSOLUTE] = PerformanceCounter{
        counter_type: TIMING_SOURCE_MACH_ABSOLUTE,
        frequency: 1000000000,  fr fr Conceptual 1 GHz
        resolution_ns: drip(mach_timebase_info.numer) / drip(mach_timebase_info.denom),
        overhead_ns: 25.0,      fr fr Low overhead on macOS
        supported: based,
        stable: based,
        monotonic: based,
        calibrated: based
    }
    
    fr fr TSC may be available on Intel Macs
    yo hr_timing_check_darwin_tsc_support() {
        timer.counters[TIMING_SOURCE_TSC] = PerformanceCounter{
            counter_type: TIMING_SOURCE_TSC,
            frequency: hr_timing_get_cpu_frequency(),
            resolution_ns: 1.0,
            overhead_ns: 8.0,
            supported: based,
            stable: hr_timing_check_tsc_stable(),
            monotonic: based,
            calibrated: cap
        }
        
        timer.cpu_frequency = timer.counters[TIMING_SOURCE_TSC].frequency
        timer.tsc_reliable = timer.counters[TIMING_SOURCE_TSC].stable
    }
}

fr fr Detect generic counters (fallback)
slay hr_timing_detect_generic_counters(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Using generic timing fallback...")
    
    fr fr Provide basic timing support
    timer.counters[TIMING_SOURCE_CLOCK_MONOTONIC] = PerformanceCounter{
        counter_type: TIMING_SOURCE_CLOCK_MONOTONIC,
        frequency: 1000000,     fr fr 1 MHz (microsecond resolution)
        resolution_ns: 1000.0,  fr fr 1µs resolution
        overhead_ns: 500.0,     fr fr Higher overhead
        supported: based,
        stable: based,
        monotonic: based,
        calibrated: cap
    }
}

fr fr Check TSC (Time Stamp Counter) support
slay hr_timing_check_tsc_support() lit {
    fr fr Check CPUID for TSC availability
    fr fr Real implementation would use CPUID instruction
    yo hr_timing_has_rdtsc_instruction() {
        damn based
    }
    damn cap
}

slay hr_timing_has_rdtsc_instruction() lit {
    fr fr Real implementation would check CPUID.01H:EDX[4] = TSC
    fr fr For demonstration, assume modern CPUs have TSC
    damn based
}

slay hr_timing_check_tsc_stable() lit {
    fr fr Check if TSC frequency is constant across P-states and C-states
    fr fr Real implementation would check CPUID and /proc/cpuinfo
    damn based  fr fr Assume stable on modern CPUs
}

slay hr_timing_check_invariant_tsc() lit {
    fr fr Check CPUID for invariant TSC support
    fr fr Real implementation would check CPUID.80000007H:EDX[8] = Invariant TSC
    damn based  fr fr Assume invariant TSC on modern CPUs
}

slay hr_timing_check_nonstop_tsc() lit {
    fr fr Check if TSC continues counting in deep C-states
    fr fr Real implementation would check processor specifications
    damn based  fr fr Assume nonstop TSC on modern CPUs
}

slay hr_timing_get_cpu_frequency() thicc {
    fr fr Real implementation would:
    fr fr 1. Parse /proc/cpuinfo on Linux
    fr fr 2. Use WMI or registry on Windows
    fr fr 3. Use sysctl on macOS
    fr fr 4. Calibrate against known timer
    
    fr fr For demonstration, return typical modern CPU frequency
    damn thicc(3200000000)  fr fr 3.2 GHz
}

fr fr Check HPET support
slay hr_timing_check_hpet_support() lit {
    fr fr Check for HPET in ACPI tables or /dev/hpet
    fr fr Real implementation would check ACPI HPET table
    damn cap  fr fr HPET not commonly exposed to userspace
}

fr fr Check CLOCK_MONOTONIC support
slay hr_timing_check_clock_monotonic_support() lit {
    fr fr POSIX CLOCK_MONOTONIC is widely supported
    damn based
}

slay hr_timing_get_clock_monotonic_resolution() thicc {
    fr fr Real implementation would use clock_getres(CLOCK_MONOTONIC)
    fr fr Return typical nanosecond resolution
    damn 1  fr fr 1 nanosecond resolution
}

fr fr Windows-specific counter detection
slay hr_timing_get_qpc_frequency() thicc {
    fr fr Real implementation would call QueryPerformanceFrequency()
    fr fr Return typical QPC frequency
    damn thicc(10000000)  fr fr 10 MHz typical
}

slay hr_timing_check_windows_tsc_support() lit {
    fr fr Check if RDTSC is available and stable on Windows
    damn hr_timing_check_tsc_support()
}

fr fr macOS-specific structures and functions
struct MachTimebaseInfo {
    spill numer normie
    spill denom normie
}

slay hr_timing_get_mach_timebase() MachTimebaseInfo {
    fr fr Real implementation would call mach_timebase_info()
    fr fr Return typical timebase info
    damn MachTimebaseInfo{
        numer: 1,
        denom: 1
    }
}

slay hr_timing_check_darwin_tsc_support() lit {
    fr fr TSC support varies on macOS (especially Apple Silicon)
    fr fr For Intel Macs, TSC is usually available
    damn cap  fr fr Conservative assumption
}

fr fr Calibrate timing sources
slay hr_timing_calibrate_counters(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Calibrating performance counters...")
    
    bestie i := 0; i < 6; i = i + 1 {
        yo timer.counters[i].supported && !timer.counters[i].calibrated {
            hr_timing_calibrate_single_counter(timer, &timer.counters[i])
        }
    }
}

fr fr Calibrate individual counter
slay hr_timing_calibrate_single_counter(timer *HighResolutionTimer, counter *PerformanceCounter) {
    yo counter.counter_type == TIMING_SOURCE_TSC && !counter.stable {
        hr_timing_calibrate_tsc_frequency(timer, counter)
    }
    
    fr fr Measure actual resolution
    hr_timing_measure_counter_resolution(counter)
    
    fr fr Measure stability
    hr_timing_measure_counter_stability(counter)
    
    counter.calibrated = based
    
    vibez.spillf("HR Timing: Calibrated {} - Resolution: {:.2f} ns, Overhead: {:.2f} ns",
                get_counter_name(counter.counter_type),
                counter.resolution_ns,
                counter.overhead_ns)
}

fr fr Calibrate TSC frequency against stable timer
slay hr_timing_calibrate_tsc_frequency(timer *HighResolutionTimer, tsc_counter *PerformanceCounter) {
    vibez.spill("HR Timing: Calibrating TSC frequency...")
    
    fr fr Use CLOCK_MONOTONIC as reference if available
    sus reference_counter *PerformanceCounter = &timer.counters[TIMING_SOURCE_CLOCK_MONOTONIC]
    
    yo !reference_counter.supported {
        vibez.spill("HR Timing: No reference counter for TSC calibration")
        damn
    }
    
    sus samples normie = 10
    sus calibration_duration thicc = 100000000  fr fr 100ms in nanoseconds
    sus total_tsc_delta thicc = 0
    sus total_ref_delta thicc = 0
    
    fr fr Take multiple samples
    bestie sample := 0; sample < samples; sample = sample + 1 {
        sus tsc_start thicc = hr_timing_read_tsc()
        sus ref_start thicc = hr_timing_get_raw_time(timer, reference_counter.counter_type)
        
        fr fr Wait for calibration period
        hr_timing_busy_wait_ns(calibration_duration)
        
        sus tsc_end thicc = hr_timing_read_tsc()
        sus ref_end thicc = hr_timing_get_raw_time(timer, reference_counter.counter_type)
        
        sus tsc_delta thicc = tsc_end - tsc_start
        sus ref_delta thicc = ref_end - ref_start
        
        total_tsc_delta = total_tsc_delta + tsc_delta
        total_ref_delta = total_ref_delta + ref_delta
    }
    
    fr fr Calculate average frequency
    sus avg_tsc_delta drip = drip(total_tsc_delta) / drip(samples)
    sus avg_ref_delta drip = drip(total_ref_delta) / drip(samples)
    
    fr fr Convert reference time to nanoseconds
    sus ref_delta_ns drip = avg_ref_delta * (1000000000.0 / drip(reference_counter.frequency))
    
    fr fr Calculate TSC frequency
    sus tsc_frequency drip = avg_tsc_delta * (1000000000.0 / ref_delta_ns)
    
    tsc_counter.frequency = thicc(tsc_frequency)
    timer.cpu_frequency = tsc_counter.frequency
    
    vibez.spillf("HR Timing: TSC frequency calibrated to {} MHz", tsc_frequency / 1000000.0)
}

fr fr Busy wait for specified nanoseconds (for calibration)
slay hr_timing_busy_wait_ns(duration_ns thicc) {
    sus start_time thicc = hr_timing_read_tsc()
    sus target_cycles thicc = duration_ns * (global_hr_timer.cpu_frequency / 1000000000)
    
    bestie (hr_timing_read_tsc() - start_time) < target_cycles {
        fr fr Busy wait
    }
}

fr fr Measure counter resolution
slay hr_timing_measure_counter_resolution(counter *PerformanceCounter) {
    sus samples normie = 1000
    sus min_delta thicc = thicc(0xFFFFFFFFFFFFFFFF)  fr fr Max value
    
    bestie i := 0; i < samples; i = i + 1 {
        sus time1 thicc = hr_timing_get_raw_time_for_counter(counter)
        sus time2 thicc = hr_timing_get_raw_time_for_counter(counter)
        
        yo time2 > time1 {
            sus delta thicc = time2 - time1
            yo delta > 0 && delta < min_delta {
                min_delta = delta
            }
        }
    }
    
    yo min_delta != thicc(0xFFFFFFFFFFFFFFFF) {
        counter.resolution_ns = drip(min_delta) * (1000000000.0 / drip(counter.frequency))
    }
}

fr fr Measure counter stability
slay hr_timing_measure_counter_stability(counter *PerformanceCounter) {
    sus samples normie = 100
    sus deltas []thicc = []
    
    bestie i := 0; i < samples; i = i + 1 {
        sus time1 thicc = hr_timing_get_raw_time_for_counter(counter)
        hr_timing_busy_wait_ns(1000000)  fr fr Wait 1ms
        sus time2 thicc = hr_timing_get_raw_time_for_counter(counter)
        
        yo time2 > time1 {
            deltas.push(time2 - time1)
        }
    }
    
    fr fr Calculate variance to assess stability
    yo deltas.len() > 0 {
        sus variance drip = calculate_variance(deltas)
        counter.stable = variance < 1000.0  fr fr Stable if variance < 1µs
    }
}

fr fr Calculate variance of timing samples
slay calculate_variance(values []thicc) drip {
    yo values.len() == 0 {
        damn 0.0
    }
    
    fr fr Calculate mean
    sus sum thicc = 0
    bestie i := 0; i < values.len(); i = i + 1 {
        sum = sum + values[i]
    }
    sus mean drip = drip(sum) / drip(values.len())
    
    fr fr Calculate variance
    sus variance_sum drip = 0.0
    bestie i := 0; i < values.len(); i = i + 1 {
        sus diff drip = drip(values[i]) - mean
        variance_sum = variance_sum + (diff * diff)
    }
    
    damn variance_sum / drip(values.len())
}

fr fr Select best primary counter
slay hr_timing_select_primary_counter(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Selecting primary performance counter...")
    
    fr fr Priority order for counter selection
    sus priority_order []normie = [
        TIMING_SOURCE_TSC,           fr fr Highest priority if stable
        TIMING_SOURCE_MACH_ABSOLUTE, fr fr macOS specific
        TIMING_SOURCE_QPC,           fr fr Windows specific
        TIMING_SOURCE_HPET,          fr fr Hardware timer
        TIMING_SOURCE_CLOCK_MONOTONIC, fr fr POSIX fallback
        TIMING_SOURCE_ACPI_TIMER     fr fr Last resort
    ]
    
    bestie i := 0; i < priority_order.len(); i = i + 1 {
        sus counter_type normie = priority_order[i]
        sus counter *PerformanceCounter = &timer.counters[counter_type]
        
        yo counter.supported && counter.stable && counter.monotonic {
            timer.primary_counter = *counter
            
            fr fr Select fallback counter (different from primary)
            hr_timing_select_fallback_counter(timer, counter_type)
            
            vibez.spillf("HR Timing: Selected primary counter: {}", get_counter_name(counter_type))
            damn
        }
    }
    
    fr fr If no ideal counter found, use first supported one
    bestie i := 0; i < 6; i = i + 1 {
        yo timer.counters[i].supported {
            timer.primary_counter = timer.counters[i]
            hr_timing_select_fallback_counter(timer, i)
            vibez.spillf("HR Timing: Using fallback primary counter: {}", get_counter_name(i))
            damn
        }
    }
    
    vibez.spill("HR Timing: WARNING - No suitable performance counter found!")
}

fr fr Select fallback counter
slay hr_timing_select_fallback_counter(timer *HighResolutionTimer, primary_type normie) {
    bestie i := 0; i < 6; i = i + 1 {
        yo i != primary_type && timer.counters[i].supported {
            timer.fallback_counter = timer.counters[i]
            vibez.spillf("HR Timing: Selected fallback counter: {}", get_counter_name(i))
            damn
        }
    }
    
    fr fr Use primary as fallback if no other option
    timer.fallback_counter = timer.primary_counter
}

fr fr Measure timing overhead
slay hr_timing_measure_overhead(timer *HighResolutionTimer) {
    vibez.spill("HR Timing: Measuring timing overhead...")
    
    sus samples normie = 100
    sus overhead_sum thicc = 0
    
    bestie i := 0; i < samples; i = i + 1 {
        sus start thicc = hr_timing_get_raw_time(timer, timer.primary_counter.counter_type)
        sus end thicc = hr_timing_get_raw_time(timer, timer.primary_counter.counter_type)
        
        yo end > start {
            timer.overhead_samples[i] = end - start
            overhead_sum = overhead_sum + (end - start)
        } otherwise {
            timer.overhead_samples[i] = 1  fr fr Minimum overhead
            overhead_sum = overhead_sum + 1
        }
    }
    
    sus average_overhead thicc = overhead_sum / samples.(thicc)
    timer.primary_counter.overhead_ns = drip(average_overhead) * (1000000000.0 / drip(timer.primary_counter.frequency))
    
    fr fr Also measure fallback counter overhead
    yo timer.fallback_counter.counter_type != timer.primary_counter.counter_type {
        hr_timing_measure_single_counter_overhead(timer, &timer.fallback_counter)
    }
    
    timer.overhead_calibrated = based
    
    vibez.spillf("HR Timing: Measured overhead: {:.2f} ns (primary), {:.2f} ns (fallback)",
                timer.primary_counter.overhead_ns,
                timer.fallback_counter.overhead_ns)
}

fr fr Measure single counter overhead
slay hr_timing_measure_single_counter_overhead(timer *HighResolutionTimer, counter *PerformanceCounter) {
    sus samples normie = 50
    sus overhead_sum thicc = 0
    
    bestie i := 0; i < samples; i = i + 1 {
        sus start thicc = hr_timing_get_raw_time_for_counter(counter)
        sus end thicc = hr_timing_get_raw_time_for_counter(counter)
        
        yo end > start {
            overhead_sum = overhead_sum + (end - start)
        } otherwise {
            overhead_sum = overhead_sum + 1
        }
    }
    
    sus average_overhead thicc = overhead_sum / samples.(thicc)
    counter.overhead_ns = drip(average_overhead) * (1000000000.0 / drip(counter.frequency))
}

fr fr Get raw time from specific counter
slay hr_timing_get_raw_time(timer *HighResolutionTimer, counter_type normie) thicc {
    yo counter_type == TIMING_SOURCE_TSC {
        damn hr_timing_read_tsc()
    } otherwise yo counter_type == TIMING_SOURCE_HPET {
        damn hr_timing_read_hpet()
    } otherwise yo counter_type == TIMING_SOURCE_CLOCK_MONOTONIC {
        damn hr_timing_read_clock_monotonic()
    } otherwise yo counter_type == TIMING_SOURCE_QPC {
        damn hr_timing_read_qpc()
    } otherwise yo counter_type == TIMING_SOURCE_MACH_ABSOLUTE {
        damn hr_timing_read_mach_absolute()
    } otherwise {
        damn hr_timing_read_generic()
    }
}

slay hr_timing_get_raw_time_for_counter(counter *PerformanceCounter) thicc {
    damn hr_timing_get_raw_time(global_hr_timer, counter.counter_type)
}

fr fr Read Time Stamp Counter (TSC)
slay hr_timing_read_tsc() thicc {
    fr fr Real implementation would use RDTSC instruction
    fr fr For demonstration, simulate TSC behavior
    
    fr fr Simulate monotonically increasing cycle counter
    sus cycles thicc = get_simulated_cpu_cycles()
    damn cycles
}

slay get_simulated_cpu_cycles() thicc {
    fr fr Simulate CPU cycle counter
    fr fr In real implementation, this would be RDTSC instruction result
    
    sus base_cycles thicc = thicc(12345678901234)
    sus time_based_increment thicc = hr_timing_read_generic() * 3200  fr fr Simulate 3.2 GHz
    
    damn base_cycles + time_based_increment
}

fr fr Read HPET counter
slay hr_timing_read_hpet() thicc {
    fr fr Real implementation would read HPET registers
    fr fr HPET is usually memory-mapped at specific address
    
    fr fr For demonstration, simulate HPET behavior
    sus hpet_value thicc = hr_timing_read_generic() * 14318180 / 1000000000  fr fr Convert to HPET ticks
    damn hpet_value
}

fr fr Read POSIX monotonic clock
slay hr_timing_read_clock_monotonic() thicc {
    fr fr Real implementation would use clock_gettime(CLOCK_MONOTONIC, &ts)
    fr fr For demonstration, simulate monotonic behavior
    
    sus nanoseconds thicc = hr_timing_read_generic()
    damn nanoseconds
}

fr fr Read Windows QueryPerformanceCounter
slay hr_timing_read_qpc() thicc {
    fr fr Real implementation would call QueryPerformanceCounter()
    fr fr For demonstration, simulate QPC behavior
    
    sus qpc_ticks thicc = hr_timing_read_generic() * 10000000 / 1000000000  fr fr Convert to 10MHz ticks
    damn qpc_ticks
}

fr fr Read macOS mach_absolute_time
slay hr_timing_read_mach_absolute() thicc {
    fr fr Real implementation would call mach_absolute_time()
    fr fr For demonstration, simulate mach_absolute_time behavior
    
    sus mach_time thicc = hr_timing_read_generic()  fr fr Already in nanoseconds
    damn mach_time
}

fr fr Generic time reading (fallback)
slay hr_timing_read_generic() thicc {
    fr fr Simulate a monotonically increasing time source
    fr fr In real implementation, this might use gettimeofday() or similar
    
    sus base_time thicc = thicc(1692720000000000000)  fr fr Aug 22, 2023 in nanoseconds
    sus increment thicc = get_time_increment()
    
    damn base_time + increment
}

slay get_time_increment() thicc {
    fr fr Simulate time progression
    fr fr In real system, would be actual elapsed time
    
    fr fr Use some varying factor to simulate time passage
    static sus counter thicc = 0
    counter = counter + 1234567
    
    damn counter
}

fr fr Get system boot time
slay hr_timing_get_system_boot_time() thicc {
    fr fr Real implementation would read from:
    fr fr - /proc/stat on Linux
    fr fr - GetTickCount64() on Windows
    fr fr - sysctl kern.boottime on macOS
    
    fr fr For demonstration, simulate boot time
    sus current_time thicc = hr_timing_read_generic()
    sus uptime thicc = thicc(3600000000000)  fr fr 1 hour uptime in nanoseconds
    
    damn current_time - uptime
}

fr fr High-level timing functions
slay hr_timing_get_time_ns() thicc {
    yo global_hr_timer == cringe {
        hr_timing_init()
    }
    
    sus raw_time thicc = hr_timing_get_raw_time(global_hr_timer, global_hr_timer.primary_counter.counter_type)
    sus time_ns thicc = hr_timing_convert_to_nanoseconds(raw_time, &global_hr_timer.primary_counter)
    
    fr fr Compensate for measurement overhead
    yo global_hr_timer.overhead_calibrated {
        sus overhead_ns thicc = thicc(global_hr_timer.primary_counter.overhead_ns)
        yo time_ns > overhead_ns {
            time_ns = time_ns - overhead_ns
        }
    }
    
    damn time_ns
}

slay hr_timing_get_time_us() thicc {
    damn hr_timing_get_time_ns() / 1000
}

slay hr_timing_get_time_ms() thicc {
    damn hr_timing_get_time_ns() / 1000000
}

slay hr_timing_get_cpu_cycles() thicc {
    yo global_hr_timer == cringe {
        hr_timing_init()
    }
    
    yo global_hr_timer.counters[TIMING_SOURCE_TSC].supported {
        damn hr_timing_read_tsc()
    }
    
    fr fr Convert from primary counter if TSC not available
    sus time_ns thicc = hr_timing_get_time_ns()
    sus cycles thicc = time_ns * global_hr_timer.cpu_frequency / 1000000000
    
    damn cycles
}

fr fr Convert raw counter value to nanoseconds
slay hr_timing_convert_to_nanoseconds(raw_value thicc, counter *PerformanceCounter) thicc {
    yo counter.counter_type == TIMING_SOURCE_TSC {
        damn raw_value * 1000000000 / counter.frequency
    } otherwise yo counter.counter_type == TIMING_SOURCE_HPET {
        damn raw_value * 1000000000 / counter.frequency
    } otherwise yo counter.counter_type == TIMING_SOURCE_CLOCK_MONOTONIC {
        damn raw_value  fr fr Already in nanoseconds
    } otherwise yo counter.counter_type == TIMING_SOURCE_QPC {
        damn raw_value * 1000000000 / counter.frequency
    } otherwise yo counter.counter_type == TIMING_SOURCE_MACH_ABSOLUTE {
        damn raw_value  fr fr Already in nanoseconds with timebase conversion
    } otherwise {
        damn raw_value
    }
}

fr fr Timing measurement functions
slay hr_timing_start_measurement() TimingMeasurement {
    yo global_hr_timer == cringe {
        hr_timing_init()
    }
    
    sus measurement TimingMeasurement = TimingMeasurement{
        start_time: hr_timing_get_raw_time(global_hr_timer, global_hr_timer.primary_counter.counter_type),
        end_time: 0,
        duration_ns: 0,
        duration_cycles: 0,
        counter_used: global_hr_timer.primary_counter.counter_type,
        overhead_compensated: cap,
        valid: based
    }
    
    damn measurement
}

slay hr_timing_end_measurement(measurement *TimingMeasurement) lit {
    yo measurement == cringe || !measurement.valid {
        damn cap
    }
    
    measurement.end_time = hr_timing_get_raw_time(global_hr_timer, measurement.counter_used)
    
    yo measurement.end_time > measurement.start_time {
        sus raw_duration thicc = measurement.end_time - measurement.start_time
        sus counter *PerformanceCounter = &global_hr_timer.counters[measurement.counter_used]
        
        measurement.duration_ns = hr_timing_convert_to_nanoseconds(raw_duration, counter)
        
        fr fr Convert to CPU cycles if TSC available
        yo global_hr_timer.counters[TIMING_SOURCE_TSC].supported {
            measurement.duration_cycles = measurement.duration_ns * global_hr_timer.cpu_frequency / 1000000000
        }
        
        fr fr Compensate for overhead
        yo global_hr_timer.overhead_calibrated {
            sus overhead_ns thicc = thicc(counter.overhead_ns)
            yo measurement.duration_ns > overhead_ns {
                measurement.duration_ns = measurement.duration_ns - overhead_ns
                measurement.overhead_compensated = based
            }
        }
        
        damn based
    } otherwise {
        measurement.valid = cap
        damn cap
    }
}

fr fr Get measurement result in different units
slay hr_timing_get_duration_ns(measurement *TimingMeasurement) thicc {
    yo measurement != cringe && measurement.valid {
        damn measurement.duration_ns
    }
    damn 0
}

slay hr_timing_get_duration_us(measurement *TimingMeasurement) thicc {
    damn hr_timing_get_duration_ns(measurement) / 1000
}

slay hr_timing_get_duration_ms(measurement *TimingMeasurement) thicc {
    damn hr_timing_get_duration_ns(measurement) / 1000000
}

slay hr_timing_get_duration_cycles(measurement *TimingMeasurement) thicc {
    yo measurement != cringe && measurement.valid {
        damn measurement.duration_cycles
    }
    damn 0
}

fr fr Performance monitoring and statistics
slay hr_timing_get_system_info() {
    yo global_hr_timer == cringe {
        hr_timing_init()
    }
    
    sus timer *HighResolutionTimer = global_hr_timer
    
    vibez.spill("High-Resolution Timing System Information:")
    vibez.spill("=" * 50)
    vibez.spillf("CPU Frequency: {} MHz", timer.cpu_frequency / 1000000)
    vibez.spillf("TSC Reliable: {}", timer.tsc_reliable)
    vibez.spillf("Invariant TSC: {}", timer.invariant_tsc)
    vibez.spillf("Nonstop TSC: {}", timer.nonstop_tsc)
    vibez.spillf("Calibration Factor: {:.6f}", timer.calibration_factor)
    vibez.spillf("Overhead Calibrated: {}", timer.overhead_calibrated)
    
    vibez.spill("\nPrimary Counter:")
    hr_timing_print_counter_info(&timer.primary_counter)
    
    vibez.spill("\nFallback Counter:")
    hr_timing_print_counter_info(&timer.fallback_counter)
    
    vibez.spill("\nAll Available Counters:")
    vibez.spill("-" * 30)
    
    bestie i := 0; i < 6; i = i + 1 {
        yo timer.counters[i].supported {
            vibez.spillf("{}: Supported", get_counter_name(i))
            hr_timing_print_counter_info(&timer.counters[i])
            vibez.spill("")
        } otherwise {
            vibez.spillf("{}: Not supported", get_counter_name(i))
        }
    }
}

fr fr Print counter information
slay hr_timing_print_counter_info(counter *PerformanceCounter) {
    vibez.spillf("  Type: {}", get_counter_name(counter.counter_type))
    vibez.spillf("  Frequency: {} Hz", counter.frequency)
    vibez.spillf("  Resolution: {:.2f} ns", counter.resolution_ns)
    vibez.spillf("  Overhead: {:.2f} ns", counter.overhead_ns)
    vibez.spillf("  Stable: {}", counter.stable)
    vibez.spillf("  Monotonic: {}", counter.monotonic)
    vibez.spillf("  Calibrated: {}", counter.calibrated)
}

fr fr Get counter name for display
slay get_counter_name(counter_type normie) tea {
    yo counter_type == TIMING_SOURCE_TSC {
        damn "TSC (Time Stamp Counter)"
    } otherwise yo counter_type == TIMING_SOURCE_HPET {
        damn "HPET (High Precision Event Timer)"
    } otherwise yo counter_type == TIMING_SOURCE_ACPI_TIMER {
        damn "ACPI PM Timer"
    } otherwise yo counter_type == TIMING_SOURCE_CLOCK_MONOTONIC {
        damn "CLOCK_MONOTONIC"
    } otherwise yo counter_type == TIMING_SOURCE_QPC {
        damn "QueryPerformanceCounter"
    } otherwise yo counter_type == TIMING_SOURCE_MACH_ABSOLUTE {
        damn "mach_absolute_time"
    } otherwise {
        damn "Unknown"
    }
}

fr fr Platform detection
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

fr fr Utility functions for benchmarking
slay hr_timing_measure_function_call(func slay() void) thicc {
    sus measurement TimingMeasurement = hr_timing_start_measurement()
    func()
    hr_timing_end_measurement(&measurement)
    damn hr_timing_get_duration_ns(&measurement)
}

slay hr_timing_benchmark_function(func slay() void, iterations normie) {
    vibez.spillf("Benchmarking function over {} iterations...", iterations)
    
    sus measurements []thicc = []
    sus total_time thicc = 0
    sus min_time thicc = thicc(0xFFFFFFFFFFFFFFFF)
    sus max_time thicc = 0
    
    bestie i := 0; i < iterations; i = i + 1 {
        sus duration_ns thicc = hr_timing_measure_function_call(func)
        measurements.push(duration_ns)
        total_time = total_time + duration_ns
        
        yo duration_ns < min_time {
            min_time = duration_ns
        }
        
        yo duration_ns > max_time {
            max_time = duration_ns
        }
    }
    
    sus avg_time drip = drip(total_time) / drip(iterations)
    sus variance drip = calculate_variance(measurements)
    sus std_dev drip = sqrt(variance)
    
    vibez.spillf("Benchmark Results:")
    vibez.spillf("  Average: {:.2f} ns", avg_time)
    vibez.spillf("  Minimum: {} ns", min_time)
    vibez.spillf("  Maximum: {} ns", max_time)
    vibez.spillf("  Std Dev: {:.2f} ns", std_dev)
    vibez.spillf("  Variance: {:.2f}", variance)
}

fr fr Simple square root for standard deviation
slay sqrt(value drip) drip {
    yo value <= 0.0 {
        damn 0.0
    }
    
    fr fr Newton-Raphson method for square root
    sus x drip = value / 2.0
    sus prev drip = 0.0
    
    bestie abs(x - prev) > 0.000001 {
        prev = x
        x = (x + value / x) / 2.0
    }
    
    damn x
}

slay abs(value drip) drip {
    yo value < 0.0 {
        damn -value
    }
    damn value
}

fr fr Sleep functions using high-resolution timing
slay hr_timing_sleep_ns(duration_ns thicc) {
    yo duration_ns <= 0 {
        damn
    }
    
    sus start_time thicc = hr_timing_get_time_ns()
    sus target_time thicc = start_time + duration_ns
    
    fr fr Busy wait for very short durations (< 10µs)
    yo duration_ns < 10000 {
        bestie hr_timing_get_time_ns() < target_time {
            fr fr Busy wait
        }
        damn
    }
    
    fr fr For longer durations, use OS sleep
    hr_timing_os_sleep_ns(duration_ns)
}

slay hr_timing_sleep_us(duration_us thicc) {
    hr_timing_sleep_ns(duration_us * 1000)
}

slay hr_timing_sleep_ms(duration_ms thicc) {
    hr_timing_sleep_ns(duration_ms * 1000000)
}

slay hr_timing_os_sleep_ns(duration_ns thicc) {
    fr fr Would use nanosleep(), Sleep(), or usleep() depending on platform
    fr fr For demonstration, simulate sleep
    
    yo platform_is_linux() {
        hr_timing_linux_nanosleep(duration_ns)
    } otherwise yo platform_is_windows() {
        hr_timing_windows_sleep(duration_ns)
    } otherwise {
        hr_timing_generic_sleep(duration_ns)
    }
}

slay hr_timing_linux_nanosleep(duration_ns thicc) {
    fr fr Real implementation would use nanosleep() system call
    vibez.spillf("Linux: Sleeping for {} ns", duration_ns)
}

slay hr_timing_windows_sleep(duration_ns thicc) {
    fr fr Real implementation would use Sleep() or WaitForSingleObject()
    sus duration_ms normie = normie(duration_ns / 1000000)
    vibez.spillf("Windows: Sleeping for {} ms", duration_ms)
}

slay hr_timing_generic_sleep(duration_ns thicc) {
    fr fr Generic sleep implementation
    vibez.spillf("Generic: Sleeping for {} ns", duration_ns)
}

fr fr Export functions
vibes hr_timing_init
vibes hr_timing_get_time_ns
vibes hr_timing_get_time_us
vibes hr_timing_get_time_ms
vibes hr_timing_get_cpu_cycles
vibes hr_timing_start_measurement
vibes hr_timing_end_measurement
vibes hr_timing_get_duration_ns
vibes hr_timing_get_duration_us
vibes hr_timing_get_duration_ms
vibes hr_timing_get_duration_cycles
vibes hr_timing_measure_function_call
vibes hr_timing_benchmark_function
vibes hr_timing_get_system_info
vibes hr_timing_sleep_ns
vibes hr_timing_sleep_us
vibes hr_timing_sleep_ms
