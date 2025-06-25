use crate::error::CursedError;
/// # Enhanced Networking Utilities
/// 
/// This module provides advanced networking features including load balancing,
/// retry mechanisms with exponential backoff, network quality assessment,
/// and advanced protocol support for the CURSED vibe_net package.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use std::io::{Read, Write};
// use crate::stdlib::vibe_net::{NetResult, VibeContext, ConnVibe};

/// Enhanced connection with retry capabilities and advanced features
pub struct EnhancedConnVibe {
/// Configuration for retry mechanisms with exponential backoff
#[derive(Debug, Clone)]
pub struct RetryConfig {
impl Default for RetryConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Network quality assessment and tracking
#[derive(Debug, Clone)]
pub struct NetworkQualityTracker {
impl NetworkQualityTracker {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Record a successful operation with latency
    pub fn record_success(&mut self, latency: Duration, throughput_bytes: u64) {
        self.success_count += 1;
        self.latency_samples.push(latency);
        self.throughput_samples.push(throughput_bytes);
        
        // Keep only recent samples (last 100)
        if self.latency_samples.len() > 100 {
            self.latency_samples.remove(0);
        }
        if self.throughput_samples.len() > 100 {
            self.throughput_samples.remove(0);
        self.update_quality_score();
    /// Record a failed operation
    pub fn record_error(&mut self) {
        self.error_count += 1;
        self.update_quality_score();
    /// Calculate current network quality score (0.0 to 1.0)
    pub fn quality_score(&self) -> f64 {
        self.quality_score
    /// Get average latency from recent samples
    pub fn average_latency(&self) -> Option<Duration> {
        if self.latency_samples.is_empty() {
            return None;
        let sum: Duration = self.latency_samples.iter().sum();
        Some(sum / self.latency_samples.len() as u32)
    /// Get average throughput from recent samples
    pub fn average_throughput(&self) -> Option<u64> {
        if self.throughput_samples.is_empty() {
            return None;
        let sum: u64 = self.throughput_samples.iter().sum();
        Some(sum / self.throughput_samples.len() as u64)
    /// Update the quality score based on recent metrics
    fn update_quality_score(&mut self) {
        let total_operations = self.success_count + self.error_count;
        if total_operations == 0 {
            self.quality_score = 1.0;
            return;
        // Base score on success rate
        let success_rate = self.success_count as f64 / total_operations as f64;
        
        // Adjust based on latency (penalty for high latency)
        let latency_factor = if let Some(avg_latency) = self.average_latency() {
            let latency_ms = avg_latency.as_millis() as f64;
            (1.0 - (latency_ms / 1000.0).min(1.0)).max(0.0)
        } else {
            1.0

        self.quality_score = (success_rate * 0.7 + latency_factor * 0.3).min(1.0);
        self.last_assessment = Some(Instant::now());
    }
}

/// Load balancer for distributing connections across multiple endpoints
pub struct LoadBalancer {
#[derive(Debug, Clone)]
pub struct Endpoint {
#[derive(Debug, Clone)]
pub enum LoadBalanceStrategy {
/// Health checker for monitoring endpoint availability
pub struct HealthChecker {
impl LoadBalancer {
    pub fn new(endpoints: Vec<String>, strategy: LoadBalanceStrategy) -> Self {
        let endpoints = endpoints.into_iter().map(|addr| Endpoint {
        }).collect();

        Self {
            health_checker: HealthChecker {
        }
    }

    /// Select the next endpoint based on the load balancing strategy
    pub fn select_endpoint(&self) -> NetResult<String> {
        let healthy_endpoints: Vec<&Endpoint> = self.endpoints.iter()
            .filter(|e| e.is_healthy)
            .collect();

        if healthy_endpoints.is_empty() {
            return Err(CursedError::new("No healthy endpoints available"));
        match self.strategy {
            LoadBalanceStrategy::RoundRobin => {
                let mut index = self.current_index.lock().unwrap();
                let selected = &healthy_endpoints[*index % healthy_endpoints.len()];
                *index += 1;
                Ok(selected.address.clone())
            }
            LoadBalanceStrategy::WeightedRoundRobin => {
                // Implement weighted round robin
                self.select_weighted_endpoint(healthy_endpoints)
            }
            LoadBalanceStrategy::LeastConnections => {
                // For simplicity, using last_used as a proxy for connections
                let selected = healthy_endpoints.iter()
                    .min_by_key(|e| e.last_used.unwrap_or(Instant::now()))
                    .unwrap();
                Ok(selected.address.clone())
            }
            LoadBalanceStrategy::LatencyBased => {
                let selected = healthy_endpoints.iter()
                    .min_by_key(|e| e.quality_tracker.average_latency().unwrap_or(Duration::MAX))
                    .unwrap();
                Ok(selected.address.clone())
            }
            LoadBalanceStrategy::QualityBased => {
                let selected = healthy_endpoints.iter()
                    .max_by(|a, b| a.quality_tracker.quality_score()
                        .partial_cmp(&b.quality_tracker.quality_score())
                        .unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap();
                Ok(selected.address.clone())
            }
        }
    fn select_weighted_endpoint(&self, endpoints: Vec<&Endpoint>) -> NetResult<String> {
        let total_weight: u32 = endpoints.iter().map(|e| e.weight).sum();
        if total_weight == 0 {
            return Err(CursedError::new("All endpoints have zero weight"));
        // Simple weighted selection (could be improved with more efficient algorithms)
        let mut current_weight = 0u32;
        let target = (self.current_index.lock().unwrap().wrapping_add(1) as u32) % total_weight;
        
        for endpoint in endpoints {
            current_weight += endpoint.weight;
            if current_weight > target {
                return Ok(endpoint.address.clone());
            }
        }

        Ok(endpoints[0].address.clone())
    /// Update endpoint health status
    pub fn update_endpoint_health(&mut self, address: &str, is_healthy: bool) {
        if let Some(endpoint) = self.endpoints.iter_mut().find(|e| e.address == address) {
            endpoint.is_healthy = is_healthy;
        }
    }

    /// Record operation result for an endpoint
    pub fn record_operation(&mut self, address: &str, success: bool, latency: Option<Duration>, throughput: Option<u64>) {
        if let Some(endpoint) = self.endpoints.iter_mut().find(|e| e.address == address) {
            endpoint.last_used = Some(Instant::now());
            
            if success {
                endpoint.quality_tracker.record_success(
                    throughput.unwrap_or(0)
                );
            } else {
                endpoint.quality_tracker.record_error();
            }
        }
    }
}

impl EnhancedConnVibe {
    pub fn new(conn: Box<dyn ConnVibe>) -> Self {
        Self {
        }
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    pub fn with_load_balancer(mut self, load_balancer: Arc<LoadBalancer>) -> Self {
        self.load_balancer = Some(load_balancer);
        self
    /// Enhanced read with retry and quality tracking
    pub fn read_with_retry(&mut self, buf: &mut [u8]) -> NetResult<usize> {
        let start_time = Instant::now();
        let mut last_error = None;
        
        for attempt in 0..=self.retry_config.max_retries {
            match self.inner.read(buf) {
                Ok(bytes_read) => {
                    let latency = start_time.elapsed();
                    self.quality_tracker.record_success(latency, bytes_read as u64);
                    return Ok(bytes_read);
                }
                Err(err) => {
                    last_error = Some(err);
                    
                    if attempt < self.retry_config.max_retries {
                        let delay = self.calculate_backoff_delay(attempt);
                        std::thread::sleep(delay);
                    }
                }
            }
        }

        self.quality_tracker.record_error();
        Err(last_error.unwrap_or_else(|| CursedError::new("Read failed after all retries")))
    /// Enhanced write with retry and quality tracking
    pub fn write_with_retry(&mut self, buf: &[u8]) -> NetResult<usize> {
        let start_time = Instant::now();
        let mut last_error = None;
        
        for attempt in 0..=self.retry_config.max_retries {
            match self.inner.write(buf) {
                Ok(bytes_written) => {
                    let latency = start_time.elapsed();
                    self.quality_tracker.record_success(latency, bytes_written as u64);
                    return Ok(bytes_written);
                }
                Err(err) => {
                    last_error = Some(err);
                    
                    if attempt < self.retry_config.max_retries {
                        let delay = self.calculate_backoff_delay(attempt);
                        std::thread::sleep(delay);
                    }
                }
            }
        }

        self.quality_tracker.record_error();
        Err(last_error.unwrap_or_else(|| CursedError::new("Write failed after all retries")))
    /// Calculate exponential backoff delay with optional jitter
    fn calculate_backoff_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.retry_config.initial_delay.as_millis() as f64;
        let multiplier = self.retry_config.backoff_multiplier.powi(attempt as i32);
        let delay_ms = (base_delay * multiplier) as u64;
        
        let mut delay = Duration::from_millis(delay_ms.min(self.retry_config.max_delay.as_millis() as u64));
        
        if self.retry_config.jitter {
            // Add random jitter (±25%)
            let jitter_factor = 0.75 + (rand::random::<f64>() * 0.5); // 0.75 to 1.25
            delay = Duration::from_millis((delay.as_millis() as f64 * jitter_factor) as u64);
        delay
    /// Get current network quality metrics
    pub fn quality_metrics(&self) -> NetworkQualityMetrics {
        NetworkQualityMetrics {
        }
    }
/// Network quality metrics for reporting
#[derive(Debug, Clone)]
pub struct NetworkQualityMetrics {
/// Advanced protocol negotiation and support
pub struct ProtocolNegotiator {
impl ProtocolNegotiator {
    pub fn new() -> Self {
        Self {
            supported_protocols: vec![
                "http/1.1".to_string(),
                "http/2".to_string(),
        }
    }

    /// Add support for a protocol
    pub fn add_protocol(&mut self, protocol: &str) {
        if !self.supported_protocols.contains(&protocol.to_string()) {
            self.supported_protocols.push(protocol.to_string());
        }
    }

    /// Set preferred protocol
    pub fn set_preferred(&mut self, protocol: &str) {
        self.preferred_protocol = Some(protocol.to_string());
    /// Configure ALPN protocols for TLS negotiation
    pub fn set_alpn_protocols(&mut self, protocols: Vec<String>) {
        self.alpn_protocols = protocols;
    /// Negotiate protocol with peer
    pub fn negotiate(&self, peer_protocols: &[String]) -> Option<String> {
        // First try preferred protocol if supported by peer
        if let Some(ref preferred) = self.preferred_protocol {
            if peer_protocols.contains(preferred) && self.supported_protocols.contains(preferred) {
                return Some(preferred.clone());
            }
        }

        // Find first common protocol
        for protocol in &self.supported_protocols {
            if peer_protocols.contains(protocol) {
                return Some(protocol.clone());
            }
        }

        None
    }
}

/// Connection multiplexer for managing multiple streams over a single connection
pub struct ConnectionMultiplexer {
impl ConnectionMultiplexer {
    pub fn new(max_streams: u32) -> Self {
        Self {
        }
    }

    /// Open a new stream
    pub fn open_stream(&mut self, conn: Box<dyn ConnVibe>) -> NetResult<u32> {
        if self.streams.len() >= self.max_streams as usize {
            return Err(CursedError::new("Maximum number of streams reached"));
        let stream_id = self.next_stream_id;
        self.next_stream_id += 2; // Increment by 2 for client-initiated streams
        
        self.streams.insert(stream_id, conn);
        Ok(stream_id)
    /// Close a stream
    pub fn close_stream(&mut self, stream_id: u32) -> NetResult<()> {
        if self.streams.remove(&stream_id).is_some() {
            Ok(())
        } else {
            Err(CursedError::new(&format!("Stream {} not found", stream_id)))
        }
    }

    /// Get a stream by ID
    pub fn get_stream(&mut self, stream_id: u32) -> Option<&mut Box<dyn ConnVibe>> {
        self.streams.get_mut(&stream_id)
    /// Get active stream count
    pub fn active_streams(&self) -> usize {
        self.streams.len()
    }
}

