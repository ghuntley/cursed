/// Redis monitoring and performance tracking
/// 
/// Provides comprehensive monitoring capabilities including command tracking,
/// performance metrics, health monitoring, and alerting functionality.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

use super::DatabaseError;

/// Redis monitoring system
#[derive(Debug)]
pub struct RedisMonitor {
    stats: Arc<Mutex<MonitoringStats>>,
    command_history: Arc<Mutex<Vec<CommandRecord>>>,
    alerts: Arc<Mutex<Vec<Alert>>>,
    started_at: Instant,
}

/// Monitoring statistics
#[derive(Debug, Default)]
pub struct MonitoringStats {
    /// Total commands executed
    pub total_commands: u64,
    /// Successful commands
    pub successful_commands: u64,
    /// Failed commands
    pub failed_commands: u64,
    /// Average response time
    pub avg_response_time: Duration,
    /// Peak response time
    pub peak_response_time: Duration,
    /// Commands by type
    pub commands_by_type: HashMap<String, u64>,
    /// Error counts by type
    pub errors_by_type: HashMap<String, u64>,
    /// Slow commands count
    pub slow_commands: u64,
    /// Connection events
    pub connection_events: ConnectionEvents,
}

/// Connection event counters
#[derive(Debug, Default)]
pub struct ConnectionEvents {
    pub connections_created: u64,
    pub connections_closed: u64,
    pub connection_errors: u64,
    pub connection_timeouts: u64,
    pub reconnection_attempts: u64,
}

/// Command execution record
#[derive(Debug, Clone)]
pub struct CommandRecord {
    pub timestamp: Instant,
    pub command: String,
    pub duration: Duration,
    pub success: bool,
    pub error_message: Option<String>,
    pub connection_id: Option<u64>,
}

/// Monitoring alert
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: u64,
    pub timestamp: Instant,
    pub alert_type: AlertType,
    pub message: String,
    pub severity: AlertSeverity,
    pub acknowledged: bool,
}

/// Alert types
#[derive(Debug, Clone, PartialEq)]
pub enum AlertType {
    HighErrorRate,
    SlowCommands,
    ConnectionIssues,
    MemoryUsage,
    Custom(String),
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl RedisMonitor {
    /// Create new monitoring instance
    #[instrument]
    pub fn new() -> Result<(), Error> {
        info!("Creating Redis monitor");
        
        Ok(Self {
            stats: Arc::new(Mutex::new(MonitoringStats::default())),
            command_history: Arc::new(Mutex::new(Vec::new())),
            alerts: Arc::new(Mutex::new(Vec::new())),
            started_at: Instant::now(),
        })
    }
    
    /// Record command execution
    #[instrument(skip(self))]
    pub async fn record_command(&self, command: &str, duration: Duration, success: bool) {
        debug!(command = command, duration = ?duration, success = success, "Recording command execution");
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_commands += 1;
            
            if success {
                stats.successful_commands += 1;
            } else {
                stats.failed_commands += 1;
            }
            
            // Update command type counters
            *stats.commands_by_type.entry(command.to_string()).or_insert(0) += 1;
            
            // Update response time statistics
            self.update_response_times(&mut stats, duration);
            
            // Check for slow commands
            if duration > Duration::from_millis(100) {
                stats.slow_commands += 1;
                self.create_alert(
                    AlertType::SlowCommands,
                    format!("Slow command detected: {} took {:?}", command, duration),
                    AlertSeverity::Warning,
                ).await;
            }
        }
        
        // Add to command history (keep last 1000 commands)
        {
            let mut history = self.command_history.lock().unwrap();
            let record = CommandRecord {
                timestamp: Instant::now(),
                command: command.to_string(),
                duration,
                success,
                error_message: if success { None } else { Some("Command failed".to_string()) },
                connection_id: None,
            };
            
            history.push(record);
            
            // Keep only last 1000 records
            if history.len() > 1000 {
                history.remove(0);
            }
        }
        
        // Check for alerts
        self.check_error_rate().await;
    }
    
    /// Record connection event
    #[instrument(skip(self))]
    pub async fn record_connection_event(&self, event_type: &str, connection_id: Option<u64>) {
        debug!(event_type = event_type, connection_id = ?connection_id, "Recording connection event");
        
        let mut stats = self.stats.lock().unwrap();
        match event_type {
            "created" => stats.connection_events.connections_created += 1,
            "closed" => stats.connection_events.connections_closed += 1,
            "error" => {
                stats.connection_events.connection_errors += 1;
                drop(stats);
                self.create_alert(
                    AlertType::ConnectionIssues,
                    "Connection error occurred".to_string(),
                    AlertSeverity::Warning,
                ).await;
            }
            "timeout" => {
                stats.connection_events.connection_timeouts += 1;
                drop(stats);
                self.create_alert(
                    AlertType::ConnectionIssues,
                    "Connection timeout occurred".to_string(),
                    AlertSeverity::Critical,
                ).await;
            }
            "reconnect" => stats.connection_events.reconnection_attempts += 1,
            _ => {}
        }
    }
    
    /// Get current statistics
    pub fn get_stats(&self) -> MonitoringStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get recent command history
    pub fn get_command_history(&self, limit: usize) -> Vec<CommandRecord> {
        let history = self.command_history.lock().unwrap();
        let start = history.len().saturating_sub(limit);
        history[start..].to_vec()
    }
    
    /// Get active alerts
    pub fn get_alerts(&self, unacknowledged_only: bool) -> Vec<Alert> {
        let alerts = self.alerts.lock().unwrap();
        if unacknowledged_only {
            alerts.iter().filter(|a| !a.acknowledged).cloned().collect()
        } else {
            alerts.clone()
        }
    }
    
    /// Acknowledge alert
    #[instrument(skip(self))]
    pub async fn acknowledge_alert(&self, alert_id: u64) -> Result<(), Error> {
        debug!(alert_id = alert_id, "Acknowledging alert");
        
        let mut alerts = self.alerts.lock().unwrap();
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledged = true;
            info!(alert_id = alert_id, "Alert acknowledged");
            Ok(())
        } else {
            Err(DatabaseError::General("Alert not found".to_string()))
        }
    }
    
    /// Get uptime
    pub fn get_uptime(&self) -> Duration {
        self.started_at.elapsed()
    }
    
    /// Get health status
    pub fn get_health_status(&self) -> HealthStatus {
        let stats = self.stats.lock().unwrap();
        let alerts = self.alerts.lock().unwrap();
        
        // Calculate error rate
        let error_rate = if stats.total_commands > 0 {
            (stats.failed_commands as f64 / stats.total_commands as f64) * 100.0
        } else {
            0.0
        };
        
        // Count critical alerts
        let critical_alerts = alerts.iter().filter(|a| !a.acknowledged && a.severity == AlertSeverity::Critical).count();
        
        if critical_alerts > 0 || error_rate > 10.0 {
            HealthStatus::Critical
        } else if error_rate > 5.0 || stats.slow_commands > 10 {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
    
    /// Update response time statistics
    fn update_response_times(&self, stats: &mut MonitoringStats, duration: Duration) {
        // Simple moving average calculation
        let total_time = stats.avg_response_time * stats.total_commands.saturating_sub(1) as u32 + duration;
        stats.avg_response_time = total_time / stats.total_commands as u32;
        
        // Update peak time
        if duration > stats.peak_response_time {
            stats.peak_response_time = duration;
        }
    }
    
    /// Check error rate and create alerts if needed
    async fn check_error_rate(&self) {
        let stats = self.stats.lock().unwrap();
        
        if stats.total_commands >= 100 {
            let error_rate = (stats.failed_commands as f64 / stats.total_commands as f64) * 100.0;
            
            if error_rate > 10.0 {
                drop(stats);
                self.create_alert(
                    AlertType::HighErrorRate,
                    format!("High error rate detected: {:.2}%", error_rate),
                    AlertSeverity::Critical,
                ).await;
            } else if error_rate > 5.0 {
                drop(stats);
                self.create_alert(
                    AlertType::HighErrorRate,
                    format!("Elevated error rate: {:.2}%", error_rate),
                    AlertSeverity::Warning,
                ).await;
            }
        }
    }
    
    /// Create new alert
    async fn create_alert(&self, alert_type: AlertType, message: String, severity: AlertSeverity) {
        let alert = Alert {
            id: rand::random::<u64>(),
            timestamp: Instant::now(),
            alert_type,
            message,
            severity,
            acknowledged: false,
        };
        
        warn!(alert_id = alert.id, message = %alert.message, severity = ?alert.severity, "Alert created");
        
        let mut alerts = self.alerts.lock().unwrap();
        alerts.push(alert);
        
        // Keep only last 100 alerts
        if alerts.len() > 100 {
            alerts.remove(0);
        }
    }
}

/// Health status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

impl Clone for MonitoringStats {
    fn clone(&self) -> Self {
        Self {
            total_commands: self.total_commands,
            successful_commands: self.successful_commands,
            failed_commands: self.failed_commands,
            avg_response_time: self.avg_response_time,
            peak_response_time: self.peak_response_time,
            commands_by_type: self.commands_by_type.clone(),
            errors_by_type: self.errors_by_type.clone(),
            slow_commands: self.slow_commands,
            connection_events: self.connection_events.clone(),
        }
    }
}

impl Clone for ConnectionEvents {
    fn clone(&self) -> Self {
        Self {
            connections_created: self.connections_created,
            connections_closed: self.connections_closed,
            connection_errors: self.connection_errors,
            connection_timeouts: self.connection_timeouts,
            reconnection_attempts: self.reconnection_attempts,
        }
    }
}
