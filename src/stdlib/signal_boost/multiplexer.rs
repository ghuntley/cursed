use crate::error::CursedError;
/// Signal multiplexer for distributing signals to multiple channels
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;
// use crate::stdlib::signal_boost::core::{BoostSignal, notify};
// use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult};

/// Handle for managing a multiplexer subscription
pub struct MultiplexerHandle {
impl MultiplexerHandle {
    fn new(id: usize, signals: Vec<BoostSignal>) -> Self {
        Self {
        }
    }
    
    /// Get the subscription ID
    pub fn id(&self) -> usize {
        self.id
    /// Get the signals this handle is subscribed to
    pub fn signals(&self) -> &[BoostSignal] {
        &self.signals
    /// Check if this handle is still active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    /// Deactivate this handle
    pub fn deactivate(&self) {
        self.active.store(false, Ordering::SeqCst);
    }
}

/// Entry for a signal subscription
struct SubscriptionEntry {
/// Signal multiplexer for distributing signals to multiple channels
pub struct SignalMultiplexer {
impl SignalMultiplexer {
    /// Create a new SignalMultiplexer
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add a channel to receive specific signals
    pub fn add(&mut self, sender: Sender<BoostSignal>, signals: &[BoostSignal]) -> SignalBoostResult<MultiplexerHandle> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let handle = Arc::new(AtomicBool::new(true));
        
        let entry = SubscriptionEntry {
        
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.insert(id, entry);
            
            let mut stats = self.stats.lock().unwrap();
            stats.total_subscriptions += 1;
            stats.active_subscriptions += 1;
            
            for signal in signals {
                *stats.signal_counts.entry(*signal).or_insert(0) += 1;
            }
        }
        
        let multiplexer_handle = MultiplexerHandle {
        
        tracing::debug!("Added subscription {} for signals: {:?}", id, signals);
        Ok(multiplexer_handle)
    /// Remove a subscription by ID
    pub fn remove(&mut self, id: usize) -> SignalBoostResult<()> {
        let mut subscriptions = self.subscriptions.lock().unwrap();
        
        if let Some(entry) = subscriptions.remove(&id) {
            entry.handle.store(false, Ordering::SeqCst);
            
            let mut stats = self.stats.lock().unwrap();
            stats.active_subscriptions = stats.active_subscriptions.saturating_sub(1);
            
            for signal in &entry.signals {
                if let Some(count) = stats.signal_counts.get_mut(signal) {
                    *count = count.saturating_sub(1);
                    if *count == 0 {
                        stats.signal_counts.remove(signal);
                    }
                }
            tracing::debug!("Removed subscription {}", id);
            Ok(())
        } else {
            Err(SignalBoostError::General(format!("Subscription {} not found", id)))
        }
    }
    
    /// Remove a subscription using its handle
    pub fn remove_handle(&mut self, handle: &MultiplexerHandle) -> SignalBoostResult<()> {
        handle.deactivate();
        self.remove(handle.id())
    /// Start the multiplexer
    pub fn start(&mut self) -> SignalBoostResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(SignalBoostError::General("Multiplexer already running".to_string()));
        // Get all unique signals from subscriptions
        let all_signals: Vec<BoostSignal> = {
            let subscriptions = self.subscriptions.lock().unwrap();
            let mut signals = Vec::new();
            for entry in subscriptions.values() {
                for signal in &entry.signals {
                    if !signals.contains(signal) {
                        signals.push(*signal);
                    }
                }
            }
            signals
        
        if all_signals.is_empty() {
            self.running.store(false, Ordering::SeqCst);
            return Err(SignalBoostError::ConfigError("No signals to multiplex".to_string()));
        // Set up signal notification
        let (receiver, _signal_handle) = notify(&all_signals)?;
        
        let subscriptions = Arc::clone(&self.subscriptions);
        let running = Arc::clone(&self.running);
        let stats = Arc::clone(&self.stats);
        
        let handle = thread::spawn(move || {
            tracing::info!("Signal multiplexer started for {} signals", all_signals.len());
            
            while running.load(Ordering::SeqCst) {
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(signal) => {
                        tracing::debug!("Multiplexer received signal: {}", signal);
                        
                        // Distribute signal to all interested subscribers
                        let mut distributed = 0;
                        let mut failed = 0;
                        
                        {
                            let mut subscriptions = subscriptions.lock().unwrap();
                            
                            // Remove inactive subscriptions
                            subscriptions.retain(|_, entry| entry.handle.load(Ordering::SeqCst));
                            
                            for (id, entry) in subscriptions.iter() {
                                if entry.signals.contains(&signal) && entry.handle.load(Ordering::SeqCst) {
                                    match entry.sender.send(signal) {
                                        Ok(()) => {
                                            distributed += 1;
                                            tracing::debug!("Distributed signal {} to subscription {}", signal, id);
                                        Err(_) => {
                                            failed += 1;
                                            entry.handle.store(false, Ordering::SeqCst);
                                            tracing::warn!("Failed to send signal {} to subscription {} (receiver dropped)", signal, id);
                                        }
                                    }
                                }
                            }
                        // Update statistics
                        {
                            let mut stats = stats.lock().unwrap();
                            stats.signals_processed += 1;
                            stats.signals_distributed += distributed;
                            stats.failed_distributions += failed;
                            
                            let signal_stats = stats.signal_stats.entry(signal).or_insert_with(SignalStats::default);
                            signal_stats.received += 1;
                            signal_stats.distributed += distributed;
                            signal_stats.failed += failed;
                        tracing::info!("Signal {} distributed to {} subscribers ({} failed)", signal, distributed, failed);
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Normal timeout, continue
                        continue;
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        tracing::info!("Signal receiver disconnected, stopping multiplexer");
                        break;
                    }
                }
            tracing::info!("Signal multiplexer stopped");
        });
        
        self.handle = Some(handle);
        
        tracing::info!("Signal multiplexer started with {} subscriptions", self.count());
        Ok(())
    /// Stop the multiplexer
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        self.running.store(false, Ordering::SeqCst);
        
        if let Some(handle) = self.handle.take() {
            // Give the thread a moment to stop gracefully
            thread::sleep(Duration::from_millis(100));
            
            if !handle.is_finished() {
                tracing::warn!("Multiplexer thread did not stop gracefully");
            }
        }
        
        tracing::info!("Signal multiplexer stopped");
        Ok(())
    /// Get the number of active subscriptions
    pub fn count(&self) -> usize {
        let subscriptions = self.subscriptions.lock().unwrap();
        subscriptions.len()
    /// Check if the multiplexer is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    /// Get multiplexer statistics
    pub fn get_statistics(&self) -> MultiplexerStats {
        let mut stats = self.stats.lock().unwrap();
        
        // Update active subscription count
        let subscriptions = self.subscriptions.lock().unwrap();
        stats.active_subscriptions = subscriptions.len();
        
        stats.clone()
    /// Clear all subscriptions
    pub fn clear(&mut self) -> SignalBoostResult<()> {
        let mut subscriptions = self.subscriptions.lock().unwrap();
        
        // Deactivate all handles
        for entry in subscriptions.values() {
            entry.handle.store(false, Ordering::SeqCst);
        subscriptions.clear();
        
        let mut stats = self.stats.lock().unwrap();
        stats.active_subscriptions = 0;
        stats.signal_counts.clear();
        
        tracing::info!("Cleared all subscriptions");
        Ok(())
    /// Get all signals being monitored
    pub fn monitored_signals(&self) -> Vec<BoostSignal> {
        let subscriptions = self.subscriptions.lock().unwrap();
        let mut signals = Vec::new();
        
        for entry in subscriptions.values() {
            for signal in &entry.signals {
                if !signals.contains(signal) {
                    signals.push(*signal);
                }
            }
        signals
    }
}

impl Drop for SignalMultiplexer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Statistics for the signal multiplexer
#[derive(Debug, Clone, Default)]
pub struct MultiplexerStats {
/// Statistics for individual signals
#[derive(Debug, Clone, Default)]
pub struct SignalStats {
// Global statistics
static ACTIVE_MULTIPLEXERS: AtomicUsize = AtomicUsize::new(0);

pub fn get_active_count() -> usize {
    ACTIVE_MULTIPLEXERS.load(Ordering::SeqCst)
