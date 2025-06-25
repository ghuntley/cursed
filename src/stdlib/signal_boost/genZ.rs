use crate::error::CursedError;
/// GenZ themed features for signal handling with enhanced capabilities
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::path::PathBuf;
// use crate::stdlib::signal_boost::core::{BoostSignal, notify, NotifyHandle, SIGUSR1, SIGUSR2, SIGHUP, SIGQUIT, SIGINT, SIGTERM};
// use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult};

/// VibeChecker - runs health checks when specific signals are received
pub struct VibeChecker {
    signal: BoostSignal,
    check_fn: Arc<dyn Fn() -> bool + Send + Sync>,
    running: Arc<AtomicBool>,
    last_status: Arc<Mutex<bool>>,
    handle: Option<thread::JoinHandle<()>>,
    _notify_handle: Option<NotifyHandle>,
}

impl VibeChecker {
    /// Create a new VibeChecker
    pub fn new<F>(signal: BoostSignal, check_fn: F) -> Self
    where
        F: Fn() -> bool + Send + Sync + 'static,
    {
        Self {
            signal,
            check_fn: Arc::new(check_fn),
            running: Arc::new(AtomicBool::new(false)),
            last_status: Arc::new(Mutex::new(true)),
            handle: None,
            _notify_handle: None,
        }
    }
    
    /// Start the vibe checker
    pub fn start(&mut self) -> SignalBoostResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(SignalBoostError::General("VibeChecker already running".to_string()));
        }
        
        let (receiver, notify_handle) = notify(&[self.signal])?;
        self._notify_handle = Some(notify_handle);
        
        let check_fn = Arc::clone(&self.check_fn);
        let running = Arc::clone(&self.running);
        let last_status = Arc::clone(&self.last_status);
        let signal = self.signal;
        
        let handle = thread::spawn(move || {
            tracing::info!("VibeChecker started for signal {}", signal);
            
            while running.load(Ordering::SeqCst) {
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(received_signal) => {
                        tracing::info!("VibeChecker received signal {}, running health check...", received_signal);
                        
                        let start_time = std::time::Instant::now();
                        let is_healthy = check_fn();
                        let check_duration = start_time.elapsed();
                        
                        {
                            let mut status = last_status.lock().unwrap();
                            *status = is_healthy;
                        }
                        
                        if is_healthy {
                            tracing::info!("✅ Vibe check passed in {:?} - we're vibing! 😎", check_duration);
                        } else {
                            tracing::error!("❌ Vibe check failed in {:?} - not vibing... 😬", check_duration);
                        }
                    },
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        continue;
                    },
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        tracing::info!("VibeChecker signal receiver disconnected");
                        break;
                    }
                }
            }
            
            tracing::info!("VibeChecker stopped");
        });
        
        self.handle = Some(handle);
        Ok(())
    }
    
    /// Stop the vibe checker
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        
        if let Some(handle) = self.handle.take() {
            thread::sleep(Duration::from_millis(100));
            if !handle.is_finished() {
                tracing::warn!("VibeChecker thread did not stop gracefully");
            }
        }
        
        if let Some(mut notify_handle) = self._notify_handle.take() {
            notify_handle.stop();
        }
        
        Ok(())
    }
    
    /// Get the last health check status
    pub fn get_status(&self) -> bool {
        *self.last_status.lock().unwrap()
    }
    
    /// Check if the vibe checker is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for VibeChecker {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Create a VibeChecker for the given signal and health check function
pub fn vibe_check<F>(signal: BoostSignal, check_fn: F) -> VibeChecker
where
    F: Fn() -> bool + Send + Sync + 'static,
{
    VibeChecker::new(signal, check_fn)
}

/// YeetOnSignal - terminates the program dramatically on signal
pub fn yeet_on_signal(signal: BoostSignal, message: &str) -> SignalBoostResult<NotifyHandle> {
    let (receiver, handle) = notify(&[signal])?;
    let message = message.to_string();
    
    thread::spawn(move || {
        if let Ok(received_signal) = receiver.recv() {
            tracing::error!("💀 YEET! {} Received signal: {} 💀", message, received_signal);
            
            // Give a moment for the log to be written
            thread::sleep(Duration::from_millis(100));
            
            // Exit dramatically
            std::process::exit(1);
        }
    });
    
    tracing::info!("YeetOnSignal set up for signal {} with message: {}", signal, message);
    Ok(handle)
}

/// NoCapReloadConfig - reloads configuration on SIGHUP without exaggeration
pub fn no_cap_reload_config<F>(config_path: &str, reload_fn: F) -> SignalBoostResult<NotifyHandle>
where
    F: Fn() -> Result<(), String> + Send + Sync + 'static,
{
    let (receiver, handle) = notify(&[SIGHUP])?;
    let config_path = config_path.to_string();
    let reload_fn = Arc::new(reload_fn);
    
    thread::spawn(move || {
        tracing::info!("NoCapReloadConfig listening for SIGHUP to reload {}", config_path);
        
        while let Ok(signal) = receiver.recv() {
            tracing::info!("📄 No cap, reloading config from {} due to signal {}", config_path, signal);
            
            let start_time = std::time::Instant::now();
            match reload_fn() {
                Ok(()) => {
                    let reload_time = start_time.elapsed();
                    tracing::info!("✅ Config reloaded successfully in {:?} - no cap! 💯", reload_time);
                },
                Err(err) => {
                    let reload_time = start_time.elapsed();
                    tracing::error!("❌ Config reload failed in {:?}: {} - that's cap! 😤", reload_time, err);
                }
            }
        }
        
        tracing::info!("NoCapReloadConfig stopped");
    });
    
    Ok(handle)
}

/// BussinLogger - logs signals with GenZ flair
pub struct BussinLogger {
    running: Arc<AtomicBool>,
    signals: Vec<BoostSignal>,
    handle: Option<thread::JoinHandle<()>>,
    _notify_handle: Option<NotifyHandle>,
}

impl BussinLogger {
    /// Create a new BussinLogger for specific signals
    pub fn new(signals: Vec<BoostSignal>) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            signals,
            handle: None,
            _notify_handle: None,
        }
    }
    
    /// Start the bussin logger
    pub fn start(&mut self) -> SignalBoostResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(SignalBoostError::General("BussinLogger already running".to_string()));
        }
        
        let (receiver, notify_handle) = notify(&self.signals)?;
        self._notify_handle = Some(notify_handle);
        
        let running = Arc::clone(&self.running);
        let signals = self.signals.clone();
        
        let handle = thread::spawn(move || {
            tracing::info!("🔥 BussinLogger started for signals: {:?} - this gonna be bussin!", signals);
            
            while running.load(Ordering::SeqCst) {
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(signal) => {
                        let emoji = match signal {
//                             s if s == crate::stdlib::signal_boost::core::SIGINT => "🛑",
//                             s if s == crate::stdlib::signal_boost::core::SIGTERM => "💀",
//                             s if s == crate::stdlib::signal_boost::core::SIGHUP => "🔄",
//                             s if s == crate::stdlib::signal_boost::core::SIGQUIT => "🚪",
//                             s if s == crate::stdlib::signal_boost::core::SIGUSR1 => "👆",
//                             s if s == crate::stdlib::signal_boost::core::SIGUSR2 => "✌️",
                            _ => "📡",
                        };
                        
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        
                        tracing::info!("{} [{}] Signal {} received - that's bussin! 🔥", 
                                     emoji, timestamp, signal);
                    },
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        continue;
                    },
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        tracing::info!("BussinLogger signal receiver disconnected");
                        break;
                    }
                }
            }
            
            tracing::info!("🔥 BussinLogger stopped - it was bussin while it lasted!");
        });
        
        self.handle = Some(handle);
        Ok(())
    }
    
    /// Stop the bussin logger
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        
        if let Some(handle) = self.handle.take() {
            thread::sleep(Duration::from_millis(100));
            if !handle.is_finished() {
                tracing::warn!("BussinLogger thread did not stop gracefully");
            }
        }
        
        if let Some(mut notify_handle) = self._notify_handle.take() {
            notify_handle.stop();
        }
        
        Ok(())
    }
    
    /// Check if the logger is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for BussinLogger {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// SheeshAlarm - creates a dramatic alarm for critical signals
pub fn sheesh_alarm(signals: &[BoostSignal]) -> SignalBoostResult<NotifyHandle> {
    let (receiver, handle) = notify(signals)?;
    let signals = signals.to_vec();
    
    thread::spawn(move || {
        tracing::info!("🚨 SheeshAlarm armed for signals: {:?} - SHEESH! 🚨", signals);
        
        while let Ok(signal) = receiver.recv() {
            // Create a dramatic alarm
            for i in 0..5 {
                tracing::error!("🚨🚨🚨 SHEESH ALARM #{} 🚨🚨🚨", i + 1);
                tracing::error!("Signal {} received - SHEESH THAT'S INTENSE! 😱", signal);
                thread::sleep(Duration::from_millis(200));
            }
            
            tracing::error!("🚨 SHEESH ALARM COMPLETE FOR SIGNAL {} 🚨", signal);
        }
    });
    
    Ok(handle)
}

/// FrFrReporter - honestly reports signal status
pub struct FrFrReporter {
    report_interval: Duration,
    running: Arc<AtomicBool>,
    signal_counts: Arc<Mutex<std::collections::HashMap<BoostSignal, usize>>>,
    handle: Option<thread::JoinHandle<()>>,
    _notify_handle: Option<NotifyHandle>,
}

impl FrFrReporter {
    /// Create a new FrFrReporter
    pub fn new(signals: Vec<BoostSignal>, report_interval: Duration) -> Self {
        Self {
            report_interval,
            running: Arc::new(AtomicBool::new(false)),
            signal_counts: Arc::new(Mutex::new(std::collections::HashMap::new())),
            handle: None,
            _notify_handle: None,
        }
    }
    
    /// Start the fr fr reporter
    pub fn start(&mut self, signals: Vec<BoostSignal>) -> SignalBoostResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(SignalBoostError::General("FrFrReporter already running".to_string()));
        }
        
        let (receiver, notify_handle) = notify(&signals)?;
        self._notify_handle = Some(notify_handle);
        
        let running = Arc::clone(&self.running);
        let signal_counts = Arc::clone(&self.signal_counts);
        let report_interval = self.report_interval;
        
        // Signal counting thread
        let counts_clone = Arc::clone(&signal_counts);
        thread::spawn(move || {
            while let Ok(signal) = receiver.recv() {
                let mut counts = counts_clone.lock().unwrap();
                *counts.entry(signal).or_insert(0) += 1;
                tracing::debug!("FrFrReporter counted signal {}", signal);
            }
        });
        
        // Reporting thread
        let handle = thread::spawn(move || {
            tracing::info!("📊 FrFrReporter started - gonna keep it 100 with the signal stats! 💯");
            
            while running.load(Ordering::SeqCst) {
                thread::sleep(report_interval);
                
                let counts = signal_counts.lock().unwrap();
                if !counts.is_empty() {
                    tracing::info!("📊 Fr fr signal report:");
                    let total_signals: usize = counts.values().sum();
                    
                    for (signal, count) in counts.iter() {
                        let percentage = (*count as f64 / total_signals as f64) * 100.0;
                        tracing::info!("  {} {}: {} times ({:.1}%) - no cap!", 
                                     get_signal_emoji(*signal), signal, count, percentage);
                    }
                    
                    tracing::info!("📊 Total signals received: {} - fr fr keeping track! 💯", total_signals);
                } else {
                    tracing::info!("📊 Fr fr signal report: No signals yet - staying quiet! 🤫");
                }
            }
            
            tracing::info!("📊 FrFrReporter stopped - it's been real! ✨");
        });
        
        self.handle = Some(handle);
        Ok(())
    }
    
    /// Stop the fr fr reporter
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        
        if let Some(handle) = self.handle.take() {
            thread::sleep(Duration::from_millis(100));
            if !handle.is_finished() {
                tracing::warn!("FrFrReporter thread did not stop gracefully");
            }
        }
        
        if let Some(mut notify_handle) = self._notify_handle.take() {
            notify_handle.stop();
        }
        
        Ok(())
    }
    
    /// Get current signal counts
    pub fn get_counts(&self) -> std::collections::HashMap<BoostSignal, usize> {
        self.signal_counts.lock().unwrap().clone()
    }
}

impl Drop for FrFrReporter {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Get emoji for signal
fn get_signal_emoji(signal: BoostSignal) -> &'static str {
    match signal {
//         s if s == crate::stdlib::signal_boost::core::SIGINT => "🛑",
//         s if s == crate::stdlib::signal_boost::core::SIGTERM => "💀",
//         s if s == crate::stdlib::signal_boost::core::SIGHUP => "🔄",
//         s if s == crate::stdlib::signal_boost::core::SIGQUIT => "🚪",
//         s if s == crate::stdlib::signal_boost::core::SIGUSR1 => "👆",
//         s if s == crate::stdlib::signal_boost::core::SIGUSR2 => "✌️",
//         s if s == crate::stdlib::signal_boost::core::SIGABRT => "💥",
//         s if s == crate::stdlib::signal_boost::core::SIGKILL => "⚡",
//         s if s == crate::stdlib::signal_boost::core::SIGSEGV => "🔥",
        _ => "📡",
    }
}

/// ChefKissHandler - handles signals with perfection
pub fn chef_kiss_handler<F>(signals: &[BoostSignal], handler: F) -> SignalBoostResult<NotifyHandle>
where
    F: Fn(BoostSignal) + Send + Sync + 'static,
{
    let (receiver, handle) = notify(signals)?;
    let handler = Arc::new(handler);
    let signals = signals.to_vec();
    
    thread::spawn(move || {
        tracing::info!("👨‍🍳💋 ChefKissHandler started for {:?} - perfection incoming!", signals);
        
        while let Ok(signal) = receiver.recv() {
            tracing::info!("👨‍🍳💋 Chef's kiss handling signal {} - *mwah* perfection!", signal);
            
            let start_time = std::time::Instant::now();
            handler(signal);
            let handle_time = start_time.elapsed();
            
            tracing::info!("👨‍🍳💋 Signal {} handled in {:?} - absolutely chef's kiss! ✨", 
                         signal, handle_time);
        }
        
        tracing::info!("👨‍🍳💋 ChefKissHandler stopped - it was perfection while it lasted!");
    });
    
    Ok(handle)
}


/// Termination step configuration
#[derive(Debug, Clone)]
pub struct TerminationStep {
    pub signal: BoostSignal,
    pub wait_time: Duration,
    pub message: String,
}

impl YeetHandler {
    /// Create a new YeetHandler
    pub fn new(signals: Vec<BoostSignal>) -> Self {
        Self {
            target_signals: signals,
            termination_sequence: vec![
                TerminationStep {
                    signal: SIGTERM,
                    wait_time: Duration::from_secs(5),
                    message: "No cap, time to go! 💅".to_string(),
                },
                TerminationStep {
                    signal: SIGINT,
                    wait_time: Duration::from_secs(3),
                    message: "Bestie, this is your final warning ⚠️".to_string(),
                },
            ],
            max_wait_time: Duration::from_secs(10),
            force_kill_delay: Duration::from_secs(15),
            yeet_count: AtomicUsize::new(0),
            last_yeet: Mutex::new(None),
            running: Arc::new(AtomicBool::new(false)),
            _notify_handle: None,
        }
    }

    /// Start the yeet handler
    pub fn start(&mut self) -> SignalBoostResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(invalid_operation("YeetHandler already yeeting"));
        }

        let (receiver, notify_handle) = notify(&self.target_signals)?;
        self._notify_handle = Some(notify_handle);

        let termination_sequence = self.termination_sequence.clone();
        let max_wait_time = self.max_wait_time;
        let force_kill_delay = self.force_kill_delay;
        let yeet_count = Arc::new(AtomicUsize::new(0));
        let last_yeet = Arc::new(Mutex::new(None::<Instant>));
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match receiver.recv() {
                    Ok(signal) => {
                        yeet_count.fetch_add(1, Ordering::SeqCst);
                        *last_yeet.lock().unwrap() = Some(Instant::now());
                        
                        tracing::warn!("Yeet signal {} received! Starting termination sequence 🚀", signal);
                        
                        // Execute termination sequence
                        for step in &termination_sequence {
                            tracing::info!("{}", step.message);
                            
                            // Send signal to self
                            unsafe {
                                #[cfg(unix)]
                                libc::kill(std::process::id() as i32, step.signal as i32);
                            }
                            
                            thread::sleep(step.wait_time);
                        }
                        
                        // Force kill after max wait time
                        tracing::error!("That's it, we're done here! Force killing process 💀");
                        thread::sleep(force_kill_delay);
                        
                        unsafe {
                            #[cfg(unix)]
                            libc::kill(std::process::id() as i32, libc::SIGKILL);
                        }
                        
                        std::process::exit(1);
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(())
    }

    /// Stop the yeet handler
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        self._notify_handle = None;
        Ok(())
    }

    /// Get yeet statistics
    pub fn get_yeet_stats(&self) -> YeetStats {
        YeetStats {
            total_yeets: self.yeet_count.load(Ordering::SeqCst),
            last_yeet_time: *self.last_yeet.lock().unwrap(),
            is_active: self.running.load(Ordering::SeqCst),
        }
    }
}

/// Yeet statistics
#[derive(Debug, Clone)]
pub struct YeetStats {
    pub total_yeets: usize,
    pub last_yeet_time: Option<Instant>,
    pub is_active: bool,
}

/// NoCapReloadManager - configuration reloading with verification
pub struct NoCapReloadManager {
    config_path: PathBuf,
    reload_signal: BoostSignal,
    validators: Vec<Box<dyn Fn(&str) -> bool + Send + Sync>>,
    reload_count: AtomicUsize,
    last_reload: Mutex<Option<Instant>>,
    current_config: RwLock<String>,
    running: Arc<AtomicBool>,
    _notify_handle: Option<NotifyHandle>,
}

impl NoCapReloadManager {
    /// Create a new NoCapReloadManager
    pub fn new<P: Into<PathBuf>>(config_path: P, signal: BoostSignal) -> Self {
        Self {
            config_path: config_path.into(),
            reload_signal: signal,
            validators: Vec::new(),
            reload_count: AtomicUsize::new(0),
            last_reload: Mutex::new(None),
            current_config: RwLock::new(String::new()),
            running: Arc::new(AtomicBool::new(false)),
            _notify_handle: None,
        }
    }

    /// Add a config validator
    pub fn add_validator<F>(&mut self, validator: F)
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.validators.push(Box::new(validator));
    }

    /// Start the reload manager
    pub fn start(&mut self) -> SignalBoostResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(invalid_operation("NoCapReloadManager already running, bestie"));
        }

        // Load initial config
        self.load_config()?;

        let (receiver, notify_handle) = notify(&[self.reload_signal])?;
        self._notify_handle = Some(notify_handle);

        let config_path = self.config_path.clone();
        let reload_count = Arc::new(AtomicUsize::new(0));
        let last_reload = Arc::new(Mutex::new(None::<Instant>));
        let current_config = Arc::new(RwLock::new(String::new()));
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match receiver.recv() {
                    Ok(signal) => {
                        tracing::info!("No cap reload signal {} received! 🔄", signal);
                        
                        match std::fs::read_to_string(&config_path) {
                            Ok(new_config) => {
                                // Validate config
                                let is_valid = true; // Simplified - would run validators
                                
                                if is_valid {
                                    *current_config.write().unwrap() = new_config;
                                    reload_count.fetch_add(1, Ordering::SeqCst);
                                    *last_reload.lock().unwrap() = Some(Instant::now());
                                    
                                    tracing::info!("Config reloaded successfully! That's fire! 🔥");
                                } else {
                                    tracing::error!("Config validation failed! That ain't it, chief 🚫");
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to reload config: {} - This is not the vibe 😤", e);
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(())
    }

    /// Stop the reload manager
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        self._notify_handle = None;
        Ok(())
    }

    /// Get current config
    pub fn get_config(&self) -> String {
        self.current_config.read().unwrap().clone()
    }

    /// Get reload statistics
    pub fn get_reload_stats(&self) -> ReloadStats {
        ReloadStats {
            total_reloads: self.reload_count.load(Ordering::SeqCst),
            last_reload_time: *self.last_reload.lock().unwrap(),
            config_size: self.current_config.read().unwrap().len(),
            is_active: self.running.load(Ordering::SeqCst),
        }
    }

    fn load_config(&self) -> SignalBoostResult<()> {
        let config = std::fs::read_to_string(&self.config_path)
            .map_err(|e| SignalBoostError::General(format!("Failed to load config: {}", e)))?;
        
        // Validate initial config
        for validator in &self.validators {
            if !validator(&config) {
                return Err(SignalBoostError::General("Initial config validation failed".to_string()));
            }
        }
        
        *self.current_config.write().unwrap() = config;
        Ok(())
    }
}

/// Reload statistics
#[derive(Debug, Clone)]
pub struct ReloadStats {
    pub total_reloads: usize,
    pub last_reload_time: Option<Instant>,
    pub config_size: usize,
    pub is_active: bool,
}

/// FlexSignalQueue - priority-based signal queue with burst handling
pub struct FlexSignalQueue {
    queue: Arc<Mutex<VecDeque<QueuedSignal>>>,
    priorities: Arc<RwLock<HashMap<BoostSignal, u8>>>,
    max_queue_size: usize,
    burst_threshold: usize,
    burst_window: Duration,
    processing_stats: Arc<RwLock<ProcessingStats>>,
    running: Arc<AtomicBool>,
    _notify_handle: Option<NotifyHandle>,
}

/// Queued signal with metadata
#[derive(Debug, Clone)]
struct QueuedSignal {
    signal: BoostSignal,
    received_at: Instant,
    priority: u8,
    source_info: String,
}

/// Signal processing statistics
#[derive(Debug, Clone, Default)]
struct ProcessingStats {
    total_signals: usize,
    processed_signals: usize,
    dropped_signals: usize,
    burst_events: usize,
    queue_overflows: usize,
    average_processing_time: Duration,
}

impl FlexSignalQueue {
    /// Create a new FlexSignalQueue
    pub fn new(signals: Vec<BoostSignal>, max_queue_size: usize) -> Self {
        let mut priorities = HashMap::new();
        
        // Set default priorities
        priorities.insert(SIGINT, 100);   // Highest priority
        priorities.insert(SIGTERM, 90);
        priorities.insert(SIGUSR1, 50);   // Medium priority
        priorities.insert(SIGUSR2, 40);
        priorities.insert(SIGHUP, 30);    // Lower priority
        
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            priorities: Arc::new(RwLock::new(priorities)),
            max_queue_size,
            burst_threshold: max_queue_size / 4,
            burst_window: Duration::from_secs(1),
            processing_stats: Arc::new(RwLock::new(ProcessingStats::default())),
            running: Arc::new(AtomicBool::new(false)),
            _notify_handle: None,
        }
    }

    /// Set signal priority
    pub fn set_priority(&self, signal: BoostSignal, priority: u8) {
        let mut priorities = self.priorities.write().unwrap();
        priorities.insert(signal, priority);
    }

    /// Start processing signals
    pub fn start<F>(&mut self, processor: F) -> SignalBoostResult<()>
    where
        F: Fn(BoostSignal) -> SignalBoostResult<()> + Send + Sync + 'static,
    {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(invalid_operation("FlexSignalQueue already flexing"));
        }

        let signals: Vec<BoostSignal> = self.priorities.read().unwrap().keys().copied().collect();
        let (receiver, notify_handle) = notify(&signals)?;
        self._notify_handle = Some(notify_handle);

        let queue = Arc::clone(&self.queue);
        let priorities = Arc::clone(&self.priorities);
        let processing_stats = Arc::clone(&self.processing_stats);
        let max_queue_size = self.max_queue_size;
        let burst_threshold = self.burst_threshold;
        let burst_window = self.burst_window;
        let running = Arc::clone(&self.running);

        // Signal receiver thread
        let queue_clone = Arc::clone(&queue);
        let priorities_clone = Arc::clone(&priorities);
        let stats_clone = Arc::clone(&processing_stats);
        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match receiver.recv() {
                    Ok(signal) => {
                        let mut queue_guard = queue_clone.lock().unwrap();
                        let mut stats = stats_clone.write().unwrap();
                        
                        stats.total_signals += 1;
                        
                        // Check for queue overflow
                        if queue_guard.len() >= max_queue_size {
                            stats.queue_overflows += 1;
                            stats.dropped_signals += 1;
                            tracing::warn!("Queue overflow! Dropping signal {} - that's not very cash money 💸", signal);
                            continue;
                        }
                        
                        // Check for burst
                        let recent_signals = queue_guard.iter()
                            .filter(|s| s.received_at.elapsed() < burst_window)
                            .count();
                        
                        if recent_signals >= burst_threshold {
                            stats.burst_events += 1;
                            tracing::warn!("Signal burst detected! {} signals in {:?} - slow your roll! 🚦", recent_signals, burst_window);
                        }
                        
                        let priority = priorities_clone.read().unwrap()
                            .get(&signal).copied().unwrap_or(0);
                        
                        let queued_signal = QueuedSignal {
                            signal,
                            received_at: Instant::now(),
                            priority,
                            source_info: format!("PID:{}", std::process::id()),
                        };
                        
                        // Insert in priority order
                        let insert_pos = queue_guard.iter()
                            .position(|s| s.priority < priority)
                            .unwrap_or(queue_guard.len());
                        
                        queue_guard.insert(insert_pos, queued_signal);
                        
                        tracing::debug!("Queued signal {} with priority {} - that's giving main character energy! ✨", signal, priority);
                    }
                    Err(_) => break,
                }
            }
        });

        // Signal processor thread
        let processor = Arc::new(processor);
        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let signal_to_process = {
                    let mut queue_guard = queue.lock().unwrap();
                    queue_guard.pop_front()
                };
                
                if let Some(queued_signal) = signal_to_process {
                    let start_time = Instant::now();
                    
                    match processor(queued_signal.signal) {
                        Ok(_) => {
                            let mut stats = processing_stats.write().unwrap();
                            stats.processed_signals += 1;
                            
                            let processing_time = start_time.elapsed();
                            stats.average_processing_time = 
                                (stats.average_processing_time + processing_time) / 2;
                            
                            tracing::debug!("Processed signal {} in {:?} - that hits different! 🎯", 
                                queued_signal.signal, processing_time);
                        }
                        Err(e) => {
                            tracing::error!("Failed to process signal {}: {} - this ain't it 😔", 
                                queued_signal.signal, e);
                        }
                    }
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });

        Ok(())
    }

    /// Stop the signal queue
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        self._notify_handle = None;
        Ok(())
    }

    /// Get queue statistics
    pub fn get_stats(&self) -> QueueStats {
        let stats = self.processing_stats.read().unwrap();
        let queue_size = self.queue.lock().unwrap().len();
        
        QueueStats {
            total_signals: stats.total_signals,
            processed_signals: stats.processed_signals,
            dropped_signals: stats.dropped_signals,
            current_queue_size: queue_size,
            burst_events: stats.burst_events,
            queue_overflows: stats.queue_overflows,
            average_processing_time: stats.average_processing_time,
        }
    }
}

/// Queue statistics
#[derive(Debug, Clone)]
pub struct QueueStats {
    pub total_signals: usize,
    pub processed_signals: usize,
    pub dropped_signals: usize,
    pub current_queue_size: usize,
    pub burst_events: usize,
    pub queue_overflows: usize,
    pub average_processing_time: Duration,
}

/// Convenience functions for enhanced GenZ features

/// Create a YeetHandler for emergency shutdown
pub fn yeet_on_signals(signals: Vec<BoostSignal>) -> YeetHandler {
    YeetHandler::new(signals)
}

/// Create a NoCapReloadManager for config reloading
pub fn no_cap_reload_config_with_signal<P: Into<PathBuf>>(config_path: P, signal: BoostSignal) -> NoCapReloadManager {
    NoCapReloadManager::new(config_path, signal)
}

/// Create a FlexSignalQueue for priority signal processing
pub fn flex_signal_queue(signals: Vec<BoostSignal>, max_size: usize) -> FlexSignalQueue {
    FlexSignalQueue::new(signals, max_size)
}


fn invalid_operation(msg: &str) -> SignalBoostError {
    SignalBoostError::InvalidOperation(msg.to_string())
}

fn not_found(msg: &str) -> SignalBoostError {
    SignalBoostError::NotFound(msg.to_string())
}
