use std::collections::HashMap;

/// Rate limit metrics
#[derive(Debug, Clone)]
pub struct RateLimitMetrics {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub denied_requests: u64,
    pub client_metrics: HashMap<String, ClientMetrics>,
}

impl RateLimitMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            allowed_requests: 0,
            denied_requests: 0,
            client_metrics: HashMap::new(),
        }
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.allowed_requests as f64 / self.total_requests as f64
        }
    }
    
    pub fn denial_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.denied_requests as f64 / self.total_requests as f64
        }
    }
}

impl Default for RateLimitMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Client-specific metrics
#[derive(Debug, Clone)]
pub struct ClientMetrics {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub denied_requests: u64,
    pub first_request_time: Option<u64>,
    pub last_request_time: Option<u64>,
}

impl ClientMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            allowed_requests: 0,
            denied_requests: 0,
            first_request_time: None,
            last_request_time: None,
        }
    }
    
    pub fn record_request(&mut self, timestamp: u64, allowed: bool) {
        self.total_requests += 1;
        if allowed {
            self.allowed_requests += 1;
        } else {
            self.denied_requests += 1;
        }
        
        if self.first_request_time.is_none() {
            self.first_request_time = Some(timestamp);
        }
        self.last_request_time = Some(timestamp);
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.allowed_requests as f64 / self.total_requests as f64
        }
    }
}

impl Default for ClientMetrics {
    fn default() -> Self {
        Self::new()
    }
}
