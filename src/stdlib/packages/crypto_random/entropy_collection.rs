/// Entropy collection system for cryptographically secure random number generation
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::thread;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::entropy_sources::{EntropySource, EntropySourceManager};
use super::entropy_mixing::EntropyMixer;

/// Entropy collection configuration
#[derive(Debug, Clone)]
pub struct EntropyCollectionConfig {
    pub min_entropy_bits: usize,        // Minimum entropy bits to maintain
    pub max_entropy_buffer: usize,      // Maximum entropy buffer size
    pub collection_interval: Duration,  // How often to collect entropy
    pub fast_reseed_threshold: usize,   // Threshold for fast reseeding
    pub slow_reseed_interval: Duration, // Interval for slow reseeding
    pub max_collection_failures: usize, // Max failures before marking source unhealthy
    pub entropy_sources: Vec<EntropySource>, // Sources to use for collection
}

impl Default for EntropyCollectionConfig {
    fn default() -> Self {
        Self {
            min_entropy_bits: 256,
            max_entropy_buffer: 4096,
            collection_interval: Duration::from_millis(100),
            fast_reseed_threshold: 1024,
            slow_reseed_interval: Duration::from_secs(10),
            max_collection_failures: 5,
            entropy_sources: vec![
                EntropySource::SystemRandom,
                EntropySource::HardwareRng,
                EntropySource::TimingJitter,
                EntropySource::MemoryLayout,
            ],
        }
    }
}

/// Entropy collection statistics
#[derive(Debug, Clone, Default)]
pub struct EntropyCollectionStats {
    pub total_entropy_collected: u64,
    pub collection_rounds: u64,
    pub fast_reseeds: u64,
    pub slow_reseeds: u64,
    pub collection_failures: u64,
    pub healthy_sources: usize,
    pub unhealthy_sources: usize,
    pub current_entropy_bits: usize,
    pub last_collection: Option<SystemTime>,
    pub last_fast_reseed: Option<SystemTime>,
    pub last_slow_reseed: Option<SystemTime>,
}

/// Entropy sample with metadata
#[derive(Debug, Clone)]
pub struct EntropySample {
    pub data: Vec<u8>,
    pub source: EntropySource,
    pub timestamp: SystemTime,
    pub estimated_entropy_bits: f64,
    pub collection_time: Duration,
}

/// Entropy collector that continuously gathers entropy from multiple sources
pub struct EntropyCollector {
    config: EntropyCollectionConfig,
    source_manager: Arc<EntropySourceManager>,
    entropy_mixer: Arc<Mutex<EntropyMixer>>,
    entropy_buffer: Arc<Mutex<Vec<u8>>>,
    entropy_samples: Arc<Mutex<VecDeque<EntropySample>>>,
    stats: Arc<Mutex<EntropyCollectionStats>>,
    source_health: Arc<Mutex<std::collections::HashMap<EntropySource, usize>>>,
    running: Arc<Mutex<bool>>,
    collection_thread: Option<thread::JoinHandle<()>>,
}

impl EntropyCollector {
    /// Create new entropy collector
    pub fn new(config: EntropyCollectionConfig) -> Self {
        let source_manager = Arc::new(EntropySourceManager::new());
        let entropy_mixer = Arc::new(Mutex::new(EntropyMixer::new()));
        
        Self {
            config,
            source_manager,
            entropy_mixer,
            entropy_buffer: Arc::new(Mutex::new(Vec::new())),
            entropy_samples: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(EntropyCollectionStats::default())),
            source_health: Arc::new(Mutex::new(std::collections::HashMap::new())),
            running: Arc::new(Mutex::new(false)),
            collection_thread: None,
        }
    }
    
    /// Start continuous entropy collection
    pub fn start(&mut self) -> AdvancedCryptoResult<()> {
        let mut running = self.running.lock().unwrap();
        if *running {
            return Ok(()); // Already running
        }
        *running = true;
        drop(running);
        
        // Initialize source health tracking
        {
            let mut health = self.source_health.lock().unwrap();
            for source in &self.config.entropy_sources {
                health.insert(source.clone(), 0);
            }
        }
        
        // Start collection thread
        let config = self.config.clone();
        let source_manager = Arc::clone(&self.source_manager);
        let entropy_mixer = Arc::clone(&self.entropy_mixer);
        let entropy_buffer = Arc::clone(&self.entropy_buffer);
        let entropy_samples = Arc::clone(&self.entropy_samples);
        let stats = Arc::clone(&self.stats);
        let source_health = Arc::clone(&self.source_health);
        let running = Arc::clone(&self.running);
        
        let handle = thread::spawn(move || {
            Self::collection_loop(
                config,
                source_manager,
                entropy_mixer,
                entropy_buffer,
                entropy_samples,
                stats,
                source_health,
                running,
            );
        });
        
        self.collection_thread = Some(handle);
        
        // Perform initial collection
        self.collect_initial_entropy()?;
        
        Ok(())
    }
    
    /// Stop entropy collection
    pub fn stop(&mut self) -> AdvancedCryptoResult<()> {
        {
            let mut running = self.running.lock().unwrap();
            *running = false;
        }
        
        if let Some(handle) = self.collection_thread.take() {
            handle.join().map_err(|_| "Failed to join collection thread")?;
        }
        
        Ok(())
    }
    
    /// Collection loop for background thread
    fn collection_loop(
        config: EntropyCollectionConfig,
        source_manager: Arc<EntropySourceManager>,
        entropy_mixer: Arc<Mutex<EntropyMixer>>,
        entropy_buffer: Arc<Mutex<Vec<u8>>>,
        entropy_samples: Arc<Mutex<VecDeque<EntropySample>>>,
        stats: Arc<Mutex<EntropyCollectionStats>>,
        source_health: Arc<Mutex<std::collections::HashMap<EntropySource, usize>>>,
        running: Arc<Mutex<bool>>,
    ) {
        let mut last_slow_reseed = SystemTime::now();
        
        while *running.lock().unwrap() {
            let start_time = SystemTime::now();
            
            // Collect from all healthy sources
            let mut collected_samples = Vec::new();
            
            for source in &config.entropy_sources {
                if Self::is_source_healthy(&source_health, source, config.max_collection_failures) {
                    match Self::collect_from_source(&source_manager, source, 32) {
                        Ok(sample) => {
                            collected_samples.push(sample);
                            Self::update_source_health(&source_health, source, true);
                        }
                        Err(_) => {
                            Self::update_source_health(&source_health, source, false);
                            Self::update_stats_failure(&stats);
                        }
                    }
                }
            }
            
            // Process collected samples
            if !collected_samples.empty() {
                Self::process_collected_samples(
                    collected_samples,
                    &entropy_mixer,
                    &entropy_buffer,
                    &entropy_samples,
                    &stats,
                    &config,
                );
            }
            
            // Check for slow reseed
            if start_time.duration_since(last_slow_reseed)
                .unwrap_or(Duration::from_secs(0)) >= config.slow_reseed_interval {
                Self::perform_slow_reseed(
                    &source_manager,
                    &entropy_mixer,
                    &entropy_buffer,
                    &stats,
                    &config,
                );
                last_slow_reseed = start_time;
            }
            
            // Update collection stats
            {
                let mut stats = stats.lock().unwrap();
                stats.collection_rounds += 1;
                stats.last_collection = Some(SystemTime::now());
            }
            
            // Sleep until next collection
            thread::sleep(config.collection_interval);
        }
    }
    
    /// Collect initial entropy for immediate use
    fn collect_initial_entropy(&self) -> AdvancedCryptoResult<()> {
        let mut collected_bytes = 0;
        let target_bytes = self.config.min_entropy_bits / 8;
        
        for source in &self.config.entropy_sources {
            if collected_bytes >= target_bytes {
                break;
            }
            
            match self.source_manager.collect_entropy(source, 64) {
                Ok(data) => {
                    let sample = EntropySample {
                        estimated_entropy_bits: self.estimate_entropy_bits(&data),
                        data,
                        source: source.clone(),
                        timestamp: SystemTime::now(),
                        collection_time: Duration::from_millis(1),
                    };
                    
                    // Mix into entropy buffer
                    let mixed_data = {
                        let mut mixer = self.entropy_mixer.lock().unwrap();
                        mixer.mix_entropy(&[sample.data.clone()])?
                    };
                    
                    {
                        let mut buffer = self.entropy_buffer.lock().unwrap();
                        buffer.extend_from_slice(&mixed_data);
                        collected_bytes += mixed_data.len();
                    }
                    
                    // Store sample
                    {
                        let mut samples = self.entropy_samples.lock().unwrap();
                        samples.push_back(sample);
                        
                        // Limit sample history
                        while samples.len() > 100 {
                            samples.pop_front();
                        }
                    }
                }
                Err(_) => {
                    Self::update_source_health(&self.source_health, source, false);
                }
            }
        }
        
        if collected_bytes < target_bytes / 2 {
            return Err("Failed to collect sufficient initial entropy".into());
        }
        
        Ok(())
    }
    
    /// Collect entropy from a specific source
    fn collect_from_source(
        source_manager: &Arc<EntropySourceManager>,
        source: &EntropySource,
        size: usize,
    ) -> AdvancedCryptoResult<EntropySample> {
        let start_time = SystemTime::now();
        
        let data = source_manager.collect_entropy(source, size)?;
        let collection_time = start_time.elapsed().unwrap_or(Duration::from_millis(1));
        
        let estimated_entropy_bits = Self::estimate_entropy_bits_simple(&data);
        
        Ok(EntropySample {
            data,
            source: source.clone(),
            timestamp: SystemTime::now(),
            estimated_entropy_bits,
            collection_time,
        })
    }
    
    /// Process collected entropy samples
    fn process_collected_samples(
        samples: Vec<EntropySample>,
        entropy_mixer: &Arc<Mutex<EntropyMixer>>,
        entropy_buffer: &Arc<Mutex<Vec<u8>>>,
        entropy_samples: &Arc<Mutex<VecDeque<EntropySample>>>,
        stats: &Arc<Mutex<EntropyCollectionStats>>,
        config: &EntropyCollectionConfig,
    ) {
        let sample_data: Vec<Vec<u8>> = samples.iter().map(|s| s.data.clone()).collect();
        
        // Mix entropy from all samples
        if let Ok(mixed_data) = {
            let mut mixer = entropy_mixer.lock().unwrap();
            mixer.mix_entropy(&sample_data)
        } {
            // Add to entropy buffer
            {
                let mut buffer = entropy_buffer.lock().unwrap();
                buffer.extend_from_slice(&mixed_data);
                
                // Limit buffer size
                if buffer.len() > config.max_entropy_buffer {
                    let excess = buffer.len() - config.max_entropy_buffer;
                    buffer.drain(0..excess);
                }
            }
            
            // Store samples
            {
                let mut sample_queue = entropy_samples.lock().unwrap();
                for sample in samples {
                    sample_queue.push_back(sample);
                }
                
                // Limit sample history
                while sample_queue.len() > 100 {
                    sample_queue.pop_front();
                }
            }
            
            // Update stats
            {
                let mut stats = stats.lock().unwrap();
                stats.total_entropy_collected += mixed_data.len() as u64;
                stats.current_entropy_bits = mixed_data.len() * 8; // Conservative estimate
            }
        }
    }
    
    /// Perform slow reseed with high-entropy sources
    fn perform_slow_reseed(
        source_manager: &Arc<EntropySourceManager>,
        entropy_mixer: &Arc<Mutex<EntropyMixer>>,
        entropy_buffer: &Arc<Mutex<Vec<u8>>>,
        stats: &Arc<Mutex<EntropyCollectionStats>>,
        config: &EntropyCollectionConfig,
    ) {
        let mut reseed_data = Vec::new();
        
        // Collect from best sources
        let best_sources = source_manager.get_best_sources(3);
        for source in best_sources {
            if let Ok(data) = source_manager.collect_entropy(&source, 64) {
                reseed_data.push(data);
            }
        }
        
        if !reseed_data.is_empty() {
            if let Ok(mixed_data) = {
                let mut mixer = entropy_mixer.lock().unwrap();
                mixer.mix_entropy(&reseed_data)
            } {
                // Add to front of entropy buffer (highest priority)
                {
                    let mut buffer = entropy_buffer.lock().unwrap();
                    let mut new_buffer = mixed_data;
                    new_buffer.extend_from_slice(&buffer);
                    *buffer = new_buffer;
                    
                    // Limit buffer size
                    if buffer.len() > config.max_entropy_buffer {
                        buffer.truncate(config.max_entropy_buffer);
                    }
                }
                
                // Update stats
                {
                    let mut stats = stats.lock().unwrap();
                    stats.slow_reseeds += 1;
                    stats.last_slow_reseed = Some(SystemTime::now());
                }
            }
        }
    }
    
    /// Check if entropy source is healthy
    fn is_source_healthy(
        source_health: &Arc<Mutex<std::collections::HashMap<EntropySource, usize>>>,
        source: &EntropySource,
        max_failures: usize,
    ) -> bool {
        let health = source_health.lock().unwrap();
        health.get(source).map_or(true, |&failures| failures < max_failures)
    }
    
    /// Update source health tracking
    fn update_source_health(
        source_health: &Arc<Mutex<std::collections::HashMap<EntropySource, usize>>>,
        source: &EntropySource,
        success: bool,
    ) {
        let mut health = source_health.lock().unwrap();
        let failures = health.entry(source.clone()).or_insert(0);
        
        if success {
            *failures = (*failures).saturating_sub(1);
        } else {
            *failures += 1;
        }
    }
    
    /// Update stats for collection failure
    fn update_stats_failure(stats: &Arc<Mutex<EntropyCollectionStats>>) {
        let mut stats = stats.lock().unwrap();
        stats.collection_failures += 1;
    }
    
    /// Simple entropy estimation
    fn estimate_entropy_bits_simple(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        // Count byte frequencies
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        // Calculate Shannon entropy
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for &freq in &frequencies {
            if freq > 0 {
                let p = freq as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        entropy * len
    }
    
    /// Estimate entropy bits (placeholder for more sophisticated estimation)
    fn estimate_entropy_bits(&self, data: &[u8]) -> f64 {
        Self::estimate_entropy_bits_simple(data)
    }
    
    /// Get entropy from buffer
    pub fn get_entropy(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        let mut buffer = self.entropy_buffer.lock().unwrap();
        
        if buffer.len() < size {
            return Err("Insufficient entropy available".into());
        }
        
        let entropy = buffer.drain(0..size).collect();
        
        // Trigger fast reseed if buffer is low
        if buffer.len() < self.config.fast_reseed_threshold {
            self.trigger_fast_reseed()?;
        }
        
        Ok(entropy)
    }
    
    /// Trigger fast reseed
    fn trigger_fast_reseed(&self) -> AdvancedCryptoResult<()> {
        let mut collected_data = Vec::new();
        
        // Quickly collect from available sources
        for source in &self.config.entropy_sources {
            if Self::is_source_healthy(&self.source_health, source, self.config.max_collection_failures) {
                if let Ok(data) = self.source_manager.collect_entropy(source, 32) {
                    collected_data.push(data);
                }
            }
        }
        
        if !collected_data.is_empty() {
            let mixed_data = {
                let mut mixer = self.entropy_mixer.lock().unwrap();
                mixer.mix_entropy(&collected_data)?
            };
            
            {
                let mut buffer = self.entropy_buffer.lock().unwrap();
                buffer.extend_from_slice(&mixed_data);
            }
            
            {
                let mut stats = self.stats.lock().unwrap();
                stats.fast_reseeds += 1;
                stats.last_fast_reseed = Some(SystemTime::now());
            }
        }
        
        Ok(())
    }
    
    /// Get current statistics
    pub fn get_stats(&self) -> EntropyCollectionStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get available entropy amount
    pub fn available_entropy(&self) -> usize {
        self.entropy_buffer.lock().unwrap().len()
    }
    
    /// Check if collector is running
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
    
    /// Get recent entropy samples
    pub fn get_recent_samples(&self, count: usize) -> Vec<EntropySample> {
        let samples = self.entropy_samples.lock().unwrap();
        samples.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }
}

impl Drop for EntropyCollector {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
