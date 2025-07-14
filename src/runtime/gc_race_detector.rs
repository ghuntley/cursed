/// Race condition detector for GC operations
/// Provides runtime detection of race conditions in memory management

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc::{self, Sender, Receiver};

use crate::error::CursedError;

/// Memory access types for race detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessType {
    Read,
    Write,
    Allocate,
    Deallocate,
    MarkObject,
    SweepObject,
}

/// Memory access record for race detection
#[derive(Debug, Clone)]
pub struct MemoryAccess {
    /// Thread ID that performed the access
    pub thread_id: thread::ThreadId,
    /// Memory address accessed
    pub address: usize,
    /// Type of access
    pub access_type: AccessType,
    /// Timestamp of access
    pub timestamp: Instant,
    /// Stack trace (simplified as function name)
    pub function: String,
    /// Sequence number for ordering
    pub sequence: u64,
}

/// Race condition detector
pub struct GcRaceDetector {
    /// Access history for race detection
    access_history: Arc<RwLock<VecDeque<MemoryAccess>>>,
    /// Per-address access tracking
    address_tracking: Arc<RwLock<HashMap<usize, Vec<MemoryAccess>>>>,
    /// Race detection enabled flag
    enabled: AtomicBool,
    /// Maximum history size
    max_history_size: usize,
    /// Sequence counter
    sequence_counter: AtomicUsize,
    /// Detection statistics
    stats: Arc<RwLock<RaceDetectionStats>>,
    /// Background analyzer
    analyzer_handle: Option<thread::JoinHandle<()>>,
    /// Analysis channel
    analysis_sender: Option<Sender<MemoryAccess>>,
}

/// Race detection statistics
#[derive(Debug, Clone, Default)]
pub struct RaceDetectionStats {
    /// Total memory accesses tracked
    pub total_accesses: u64,
    /// Potential races detected
    pub races_detected: u64,
    /// Read-write races
    pub read_write_races: u64,
    /// Write-write races
    pub write_write_races: u64,
    /// GC races (allocation during sweep, etc.)
    pub gc_races: u64,
    /// False positives filtered
    pub false_positives_filtered: u64,
}

/// Race detection result
#[derive(Debug, Clone)]
pub struct RaceDetectionResult {
    /// Race type detected
    pub race_type: RaceType,
    /// First access involved in race
    pub access1: MemoryAccess,
    /// Second access involved in race
    pub access2: MemoryAccess,
    /// Severity of the race
    pub severity: RaceSeverity,
    /// Description of the race
    pub description: String,
}

/// Types of races detected
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RaceType {
    /// Read-write race
    ReadWrite,
    /// Write-write race
    WriteWrite,
    /// Allocation during sweep
    AllocDuringSweep,
    /// Deallocation during mark
    DeallocDuringMark,
    /// Concurrent modification
    ConcurrentModification,
}

/// Race severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum RaceSeverity {
    /// Low severity - unlikely to cause issues
    Low,
    /// Medium severity - could cause issues
    Medium,
    /// High severity - likely to cause corruption
    High,
    /// Critical severity - guaranteed corruption
    Critical,
}

impl GcRaceDetector {
    /// Create new race detector
    pub fn new(max_history_size: usize) -> Self {
        Self {
            access_history: Arc::new(RwLock::new(VecDeque::new())),
            address_tracking: Arc::new(RwLock::new(HashMap::new())),
            enabled: AtomicBool::new(false),
            max_history_size,
            sequence_counter: AtomicUsize::new(0),
            stats: Arc::new(RwLock::new(RaceDetectionStats::default())),
            analyzer_handle: None,
            analysis_sender: None,
        }
    }
    
    /// Enable race detection
    pub fn enable(&mut self) -> Result<(), CursedError> {
        if self.enabled.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("Race detector already enabled"));
        }
        
        // Start background analyzer
        self.start_analyzer()?;
        
        Ok(())
    }
    
    /// Disable race detection
    pub fn disable(&mut self) -> Result<(), CursedError> {
        self.enabled.store(false, Ordering::Relaxed);
        
        // Stop background analyzer
        if let Some(sender) = self.analysis_sender.take() {
            drop(sender); // Close channel
        }
        
        if let Some(handle) = self.analyzer_handle.take() {
            handle.join()
                .map_err(|_| CursedError::runtime_error("Failed to join analyzer thread"))?;
        }
        
        Ok(())
    }
    
    /// Record memory access
    pub fn record_access(
        &self,
        address: usize,
        access_type: AccessType,
        function: &str,
    ) -> Result<(), CursedError> {
        if !self.enabled.load(Ordering::Relaxed) {
            return Ok(());
        }
        
        let sequence = self.sequence_counter.fetch_add(1, Ordering::Relaxed);
        
        let access = MemoryAccess {
            thread_id: thread::current().id(),
            address,
            access_type,
            timestamp: Instant::now(),
            function: function.to_string(),
            sequence: sequence as u64,
        };
        
        // Send to analyzer if channel available
        if let Some(ref sender) = self.analysis_sender {
            sender.send(access.clone())
                .map_err(|_| CursedError::runtime_error("Failed to send access to analyzer"))?;
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
            stats.total_accesses += 1;
        }
        
        Ok(())
    }
    
    /// Start background analyzer
    fn start_analyzer(&mut self) -> Result<(), CursedError> {
        let (sender, receiver) = mpsc::channel::<MemoryAccess>();
        let access_history = Arc::clone(&self.access_history);
        let address_tracking = Arc::clone(&self.address_tracking);
        let stats = Arc::clone(&self.stats);
        let max_history_size = self.max_history_size;
        
        let handle = thread::Builder::new()
            .name("gc-race-analyzer".to_string())
            .spawn(move || {
                let mut local_buffer = Vec::new();
                
                while let Ok(access) = receiver.recv() {
                    local_buffer.push(access.clone());
                    
                    // Process in batches for efficiency
                    if local_buffer.len() >= 100 {
                        Self::analyze_accesses(&local_buffer, &access_history, &address_tracking, &stats, max_history_size);
                        local_buffer.clear();
                    }
                }
                
                // Process remaining accesses
                if !local_buffer.is_empty() {
                    Self::analyze_accesses(&local_buffer, &access_history, &address_tracking, &stats, max_history_size);
                }
            })
            .map_err(|e| CursedError::runtime_error(&format!("Failed to start analyzer thread: {}", e)))?;
        
        self.analyzer_handle = Some(handle);
        self.analysis_sender = Some(sender);
        
        Ok(())
    }
    
    /// Analyze memory accesses for races
    fn analyze_accesses(
        accesses: &[MemoryAccess],
        access_history: &Arc<RwLock<VecDeque<MemoryAccess>>>,
        address_tracking: &Arc<RwLock<HashMap<usize, Vec<MemoryAccess>>>>,
        stats: &Arc<RwLock<RaceDetectionStats>>,
        max_history_size: usize,
    ) {
        for access in accesses {
            // Add to global history
            if let Ok(mut history) = access_history.write() {
                history.push_back(access.clone());
                
                // Limit history size
                while history.len() > max_history_size {
                    history.pop_front();
                }
            }
            
            // Add to per-address tracking
            if let Ok(mut tracking) = address_tracking.write() {
                let addr_accesses = tracking.entry(access.address).or_insert_with(Vec::new);
                addr_accesses.push(access.clone());
                
                // Check for races with recent accesses to same address
                Self::detect_races_for_address(addr_accesses, stats);
                
                // Limit per-address history
                if addr_accesses.len() > 50 {
                    addr_accesses.drain(0..25); // Remove oldest half
                }
            }
        }
    }
    
    /// Detect races for a specific address
    fn detect_races_for_address(
        accesses: &[MemoryAccess],
        stats: &Arc<RwLock<RaceDetectionStats>>,
    ) {
        if accesses.len() < 2 {
            return;
        }
        
        let recent_access = &accesses[accesses.len() - 1];
        
        // Check against recent accesses from different threads
        for prev_access in accesses.iter().rev().skip(1).take(10) {
            if prev_access.thread_id == recent_access.thread_id {
                continue; // Same thread, no race
            }
            
            // Check temporal proximity (within 100ms)
            if recent_access.timestamp.duration_since(prev_access.timestamp) > Duration::from_millis(100) {
                continue; // Too far apart
            }
            
            // Detect race conditions
            if let Some(race_type) = Self::classify_race(prev_access, recent_access) {
                if let Ok(mut stats_guard) = stats.write() {
                    stats_guard.races_detected += 1;
                    
                    match race_type {
                        RaceType::ReadWrite => stats_guard.read_write_races += 1,
                        RaceType::WriteWrite => stats_guard.write_write_races += 1,
                        RaceType::AllocDuringSweep | RaceType::DeallocDuringMark => {
                            stats_guard.gc_races += 1;
                        }
                        _ => {}
                    }
                }
                
                // Log critical races
                let severity = Self::assess_severity(&race_type, prev_access, recent_access);
                if severity >= RaceSeverity::High {
                    eprintln!("CRITICAL RACE DETECTED: {:?} at address 0x{:x} between threads {:?} and {:?}",
                        race_type, recent_access.address, prev_access.thread_id, recent_access.thread_id);
                }
            }
        }
    }
    
    /// Classify the type of race between two accesses
    fn classify_race(access1: &MemoryAccess, access2: &MemoryAccess) -> Option<RaceType> {
        use AccessType::*;
        
        match (access1.access_type, access2.access_type) {
            // Read-write races
            (Read, Write) | (Write, Read) => Some(RaceType::ReadWrite),
            
            // Write-write races
            (Write, Write) => Some(RaceType::WriteWrite),
            
            // GC-specific races
            (SweepObject, Allocate) | (Allocate, SweepObject) => Some(RaceType::AllocDuringSweep),
            (MarkObject, Deallocate) | (Deallocate, MarkObject) => Some(RaceType::DeallocDuringMark),
            
            // Concurrent modifications during GC
            (MarkObject, Write) | (Write, MarkObject) |
            (SweepObject, Write) | (Write, SweepObject) => Some(RaceType::ConcurrentModification),
            
            _ => None,
        }
    }
    
    /// Assess the severity of a race condition
    fn assess_severity(race_type: &RaceType, access1: &MemoryAccess, access2: &MemoryAccess) -> RaceSeverity {
        match race_type {
            RaceType::AllocDuringSweep | RaceType::DeallocDuringMark => RaceSeverity::Critical,
            RaceType::ConcurrentModification => RaceSeverity::High,
            RaceType::WriteWrite => RaceSeverity::High,
            RaceType::ReadWrite => {
                // Time-based severity
                let time_diff = access2.timestamp.duration_since(access1.timestamp);
                if time_diff < Duration::from_millis(1) {
                    RaceSeverity::High
                } else if time_diff < Duration::from_millis(10) {
                    RaceSeverity::Medium
                } else {
                    RaceSeverity::Low
                }
            }
        }
    }
    
    /// Get detection statistics
    pub fn get_stats(&self) -> Result<RaceDetectionStats, CursedError> {
        let stats = self.stats.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
        Ok(stats.clone())
    }
    
    /// Get recent races detected
    pub fn get_recent_races(&self) -> Result<Vec<RaceDetectionResult>, CursedError> {
        // This would return recent race detection results
        // For now, return empty vector as this is a simplified implementation
        Ok(Vec::new())
    }
    
    /// Clear all tracking data
    pub fn clear(&self) -> Result<(), CursedError> {
        {
            let mut history = self.access_history.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire history lock"))?;
            history.clear();
        }
        
        {
            let mut tracking = self.address_tracking.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire tracking lock"))?;
            tracking.clear();
        }
        
        {
            let mut stats = self.stats.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
            *stats = RaceDetectionStats::default();
        }
        
        self.sequence_counter.store(0, Ordering::Relaxed);
        
        Ok(())
    }
}

impl Drop for GcRaceDetector {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}

/// Global race detector instance
use std::sync::OnceLock;
static GLOBAL_RACE_DETECTOR: OnceLock<Arc<Mutex<GcRaceDetector>>> = OnceLock::new();

/// Initialize global race detector
pub fn initialize_race_detector(max_history_size: usize) -> Result<(), CursedError> {
    let detector = GcRaceDetector::new(max_history_size);
    
    GLOBAL_RACE_DETECTOR.set(Arc::new(Mutex::new(detector)))
        .map_err(|_| CursedError::runtime_error("Race detector already initialized"))?;
    
    Ok(())
}

/// Get global race detector
pub fn get_race_detector() -> Option<Arc<Mutex<GcRaceDetector>>> {
    GLOBAL_RACE_DETECTOR.get().cloned()
}

/// Enable global race detection
pub fn enable_race_detection() -> Result<(), CursedError> {
    if let Some(detector_arc) = get_race_detector() {
        let mut detector = detector_arc.lock()
            .map_err(|_| CursedError::runtime_error("Failed to acquire race detector lock"))?;
        detector.enable()?;
    }
    Ok(())
}

/// Disable global race detection
pub fn disable_race_detection() -> Result<(), CursedError> {
    if let Some(detector_arc) = get_race_detector() {
        let mut detector = detector_arc.lock()
            .map_err(|_| CursedError::runtime_error("Failed to acquire race detector lock"))?;
        detector.disable()?;
    }
    Ok(())
}

/// Record memory access globally
pub fn record_memory_access(address: usize, access_type: AccessType, function: &str) -> Result<(), CursedError> {
    if let Some(detector_arc) = get_race_detector() {
        let detector = detector_arc.lock()
            .map_err(|_| CursedError::runtime_error("Failed to acquire race detector lock"))?;
        detector.record_access(address, access_type, function)?;
    }
    Ok(())
}

/// Convenience macros for recording accesses
#[macro_export]
macro_rules! record_read {
    ($addr:expr) => {
        let _ = $crate::runtime::gc_race_detector::record_memory_access(
            $addr,
            $crate::runtime::gc_race_detector::AccessType::Read,
            module_path!(),
        );
    };
}

#[macro_export]
macro_rules! record_write {
    ($addr:expr) => {
        let _ = $crate::runtime::gc_race_detector::record_memory_access(
            $addr,
            $crate::runtime::gc_race_detector::AccessType::Write,
            module_path!(),
        );
    };
}

#[macro_export]
macro_rules! record_alloc {
    ($addr:expr) => {
        let _ = $crate::runtime::gc_race_detector::record_memory_access(
            $addr,
            $crate::runtime::gc_race_detector::AccessType::Allocate,
            module_path!(),
        );
    };
}

#[macro_export]
macro_rules! record_dealloc {
    ($addr:expr) => {
        let _ = $crate::runtime::gc_race_detector::record_memory_access(
            $addr,
            $crate::runtime::gc_race_detector::AccessType::Deallocate,
            module_path!(),
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_detector_creation() {
        let detector = GcRaceDetector::new(1000);
        assert!(!detector.enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_race_detector_enable_disable() {
        let mut detector = GcRaceDetector::new(1000);
        
        detector.enable().unwrap();
        assert!(detector.enabled.load(Ordering::Relaxed));
        
        detector.disable().unwrap();
        assert!(!detector.enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_access_recording() {
        let mut detector = GcRaceDetector::new(1000);
        detector.enable().unwrap();
        
        detector.record_access(0x1000, AccessType::Read, "test_function").unwrap();
        detector.record_access(0x1000, AccessType::Write, "test_function").unwrap();
        
        let stats = detector.get_stats().unwrap();
        assert_eq!(stats.total_accesses, 2);
        
        detector.disable().unwrap();
    }
    
    #[test]
    fn test_race_classification() {
        let access1 = MemoryAccess {
            thread_id: thread::current().id(),
            address: 0x1000,
            access_type: AccessType::Read,
            timestamp: Instant::now(),
            function: "test".to_string(),
            sequence: 1,
        };
        
        let access2 = MemoryAccess {
            thread_id: thread::current().id(),
            address: 0x1000,
            access_type: AccessType::Write,
            timestamp: Instant::now(),
            function: "test".to_string(),
            sequence: 2,
        };
        
        let race_type = GcRaceDetector::classify_race(&access1, &access2);
        assert_eq!(race_type, Some(RaceType::ReadWrite));
    }
    
    #[test]
    fn test_severity_assessment() {
        let access1 = MemoryAccess {
            thread_id: thread::current().id(),
            address: 0x1000,
            access_type: AccessType::Allocate,
            timestamp: Instant::now(),
            function: "test".to_string(),
            sequence: 1,
        };
        
        let access2 = MemoryAccess {
            thread_id: thread::current().id(),
            address: 0x1000,
            access_type: AccessType::SweepObject,
            timestamp: Instant::now(),
            function: "test".to_string(),
            sequence: 2,
        };
        
        let severity = GcRaceDetector::assess_severity(&RaceType::AllocDuringSweep, &access1, &access2);
        assert_eq!(severity, RaceSeverity::Critical);
    }
    
    #[test]
    fn test_global_race_detector() {
        initialize_race_detector(1000).unwrap();
        
        enable_race_detection().unwrap();
        
        record_memory_access(0x1000, AccessType::Read, "test").unwrap();
        record_memory_access(0x1000, AccessType::Write, "test").unwrap();
        
        disable_race_detection().unwrap();
    }
    
    #[test]
    fn test_concurrent_access_detection() {
        use std::sync::Barrier;
        
        initialize_race_detector(1000).unwrap();
        enable_race_detection().unwrap();
        
        let barrier = Arc::new(Barrier::new(2));
        let address = 0x1000;
        
        let handles = (0..2).map(|i| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                
                let access_type = if i == 0 { AccessType::Read } else { AccessType::Write };
                record_memory_access(address, access_type, "concurrent_test").unwrap();
            })
        }).collect::<Vec<_>>();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Allow time for analysis
        thread::sleep(Duration::from_millis(100));
        
        disable_race_detection().unwrap();
    }
}
