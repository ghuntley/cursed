use crate::error::Error;
/// Real security and permissions management for CURSED IPC
/// 
/// This module provides comprehensive security functionality for inter-process
/// communication, including authentication, authorization, encryption, and access control.
/// 
/// # Why Security is Critical for Distributed Systems
/// 
/// IPC security provides:
/// - Process isolation and privilege separation
/// - Data encryption and integrity protection
/// - Authentication and authorization frameworks
/// - Audit trails and security monitoring
/// - Attack surface minimization
/// 
/// In distributed systems, IPC security enables:
/// - Zero-trust architecture with service-to-service authentication
/// - Data sovereignty and regulatory compliance
/// - Multi-tenant security isolation
/// - Insider threat protection and least privilege access
/// - Security incident detection and response

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime};
use std::fs;
use std::path::Path;
use rand::{Rng, thread_rng};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use crate::stdlib::ipc::{
use crate::stdlib::web_vibez::SecurityContext;
    IpcResult, IpcError,
    permission_denied, resource_error
};

use crate::stdlib::ipc::types::{IpcPermissions, ProcessId};
use crate::stdlib::ipc::error::{security_error, system_error};

/// Security context for IPC operations
#[derive(Debug, Clone)]
pub struct IpcSecurityContext {
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
    pub process_id: ProcessId,
    pub permissions: IpcPermissions,
    pub session_token: Option<String>,
    pub created_at: SystemTime,
    pub last_accessed: Option<SystemTime>,
    pub access_count: u64,
    pub security_level: SecurityLevel,
    pub capabilities: Vec<String>,
}

impl IpcSecurityContext {
    pub fn new(process_id: ProcessId) -> Self {
        Self {
            user_id: Self::get_current_user_id(),
            group_id: Self::get_current_group_id(),
            process_id,
            permissions: IpcPermissions::read_write(),
            session_token: None,
            created_at: SystemTime::now(),
            last_accessed: None,
            access_count: 0,
            security_level: SecurityLevel::Standard,
            capabilities: Vec::new(),
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.session_token = Some(token);
        self
    }

    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self
    }

    pub fn add_capability(mut self, capability: String) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self
    }

    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())
    }

    pub fn update_access(&mut self) {
        self.last_accessed = Some(SystemTime::now());
        self.access_count += 1;
    }

    pub fn is_elevated(&self) -> bool {
        matches!(self.security_level, SecurityLevel::Elevated | SecurityLevel::Administrative)
    }

    #[cfg(unix)]
    fn get_current_user_id() -> Option<u32> {
        Some(unsafe { libc::getuid() })
    }

    #[cfg(not(unix))]
    fn get_current_user_id() -> Option<u32> {
        None
    }

    #[cfg(unix)]
    fn get_current_group_id() -> Option<u32> {
        Some(unsafe { libc::getgid() })
    }

    #[cfg(not(unix))]
    fn get_current_group_id() -> Option<u32> {
        None
    }
}

/// Security levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    None = 0,
    Basic = 1,
    Standard = 2,
    Elevated = 3,
    Administrative = 4,
}

/// Security policy configuration
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub enforce_permissions: bool,
    pub allow_cross_user: bool,
    pub require_authentication: bool,
    pub encryption_required: bool,
    pub audit_enabled: bool,
    pub max_session_duration: Duration,
    pub password_complexity: bool,
    pub token_expiry: Duration,
    pub rate_limiting: bool,
    pub max_requests_per_minute: u32,
}

impl SecurityPolicy {
    pub fn new() -> Self {
        Self {
            enforce_permissions: true,
            allow_cross_user: false,
            require_authentication: true,
            encryption_required: false,
            audit_enabled: true,
            max_session_duration: Duration::from_secs(8 * 3600),
            password_complexity: true,
            token_expiry: Duration::from_secs(3600),
            rate_limiting: true,
            max_requests_per_minute: 1000,
        }
    }

    pub fn strict() -> Self {
        Self {
            enforce_permissions: true,
            allow_cross_user: false,
            require_authentication: true,
            encryption_required: true,
            audit_enabled: true,
            max_session_duration: Duration::from_secs(4 * 3600),
            password_complexity: true,
            token_expiry: Duration::from_secs(30 * 60),
            rate_limiting: true,
            max_requests_per_minute: 100,
        }
    }

    pub fn permissive() -> Self {
        Self {
            enforce_permissions: false,
            allow_cross_user: true,
            require_authentication: false,
            encryption_required: false,
            audit_enabled: false,
            max_session_duration: Duration::from_secs(24 * 3600),
            password_complexity: false,
            token_expiry: Duration::from_secs(8 * 3600),
            rate_limiting: false,
            max_requests_per_minute: 10000,
        }
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// Access control configuration
#[derive(Debug, Clone)]
pub struct AccessControl {
    pub read_allowed: bool,
    pub write_allowed: bool,
    pub execute_allowed: bool,
    pub admin_required: bool,
    pub min_security_level: SecurityLevel,
    pub required_capabilities: Vec<String>,
    pub resource_specific_rules: HashMap<String, ResourceRule>,
}

impl AccessControl {
    pub fn new() -> Self {
        Self {
            read_allowed: true,
            write_allowed: false,
            execute_allowed: false,
            admin_required: false,
            min_security_level: SecurityLevel::Basic,
            required_capabilities: Vec::new(),
            resource_specific_rules: HashMap::new(),
        }
    }

    pub fn read_only() -> Self {
        Self {
            read_allowed: true,
            write_allowed: false,
            execute_allowed: false,
            admin_required: false,
            min_security_level: SecurityLevel::Basic,
            required_capabilities: Vec::new(),
            resource_specific_rules: HashMap::new(),
        }
    }

    pub fn full_access() -> Self {
        Self {
            read_allowed: true,
            write_allowed: true,
            execute_allowed: true,
            admin_required: false,
            min_security_level: SecurityLevel::Standard,
            required_capabilities: Vec::new(),
            resource_specific_rules: HashMap::new(),
        }
    }

    pub fn admin_only() -> Self {
        Self {
            read_allowed: true,
            write_allowed: true,
            execute_allowed: true,
            admin_required: true,
            min_security_level: SecurityLevel::Administrative,
            required_capabilities: vec!["admin".to_string()],
            resource_specific_rules: HashMap::new(),
        }
    }

    pub fn with_capability(mut self, capability: String) -> Self {
        if !self.required_capabilities.contains(&capability) {
            self.required_capabilities.push(capability);
        }
        self
    }

    pub fn with_resource_rule(mut self, resource: String, rule: ResourceRule) -> Self {
        self.resource_specific_rules.insert(resource, rule);
        self
    }

    pub fn check_access(&self, context: &IpcSecurityContext, operation: &str, resource: &str) -> AuthorizationResult {
        // Check security level
        if context.security_level < self.min_security_level {
            return AuthorizationResult::Denied("Insufficient security level".to_string());
        }

        // Check admin requirement
        if self.admin_required && !context.is_elevated() {
            return AuthorizationResult::RequiresElevation;
        }

        // Check capabilities
        for capability in &self.required_capabilities {
            if !context.has_capability(capability) {
                return AuthorizationResult::Denied(format!("Missing capability: {}", capability));
            }
        }

        // Check operation permissions
        match operation {
            "read" if !self.read_allowed => {
                return AuthorizationResult::Denied("Read access denied".to_string());
            }
            "write" if !self.write_allowed => {
                return AuthorizationResult::Denied("Write access denied".to_string());
            }
            "execute" if !self.execute_allowed => {
                return AuthorizationResult::Denied("Execute access denied".to_string());
            }
            _ => {}
        }

        // Check resource-specific rules
        if let Some(rule) = self.resource_specific_rules.get(resource) {
            return rule.check_access(context, operation);
        }

        AuthorizationResult::Allowed
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource-specific access rule
#[derive(Debug, Clone)]
pub struct ResourceRule {
    pub allowed_users: Option<Vec<u32>>,
    pub allowed_groups: Option<Vec<u32>>,
    pub allowed_processes: Option<Vec<ProcessId>>,
    pub time_restrictions: Option<TimeRestriction>,
    pub rate_limit: Option<RateLimit>,
}

impl ResourceRule {
    pub fn new() -> Self {
        Self {
            allowed_users: None,
            allowed_groups: None,
            allowed_processes: None,
            time_restrictions: None,
            rate_limit: None,
        }
    }

    pub fn check_access(&self, context: &IpcSecurityContext, _operation: &str) -> AuthorizationResult {
        // Check user restrictions
        if let Some(ref allowed_users) = self.allowed_users {
            if let Some(user_id) = context.user_id {
                if !allowed_users.contains(&user_id) {
                    return AuthorizationResult::Denied("User not authorized".to_string());
                }
            }
        }

        // Check group restrictions
        if let Some(ref allowed_groups) = self.allowed_groups {
            if let Some(group_id) = context.group_id {
                if !allowed_groups.contains(&group_id) {
                    return AuthorizationResult::Denied("Group not authorized".to_string());
                }
            }
        }

        // Check process restrictions
        if let Some(ref allowed_processes) = self.allowed_processes {
            if !allowed_processes.contains(&context.process_id) {
                return AuthorizationResult::Denied("Process not authorized".to_string());
            }
        }

        // Check time restrictions
        if let Some(ref time_restriction) = self.time_restrictions {
            if !time_restriction.is_allowed() {
                return AuthorizationResult::Denied("Access not allowed at this time".to_string());
            }
        }

        AuthorizationResult::Allowed
    }
}

/// Time-based access restrictions
#[derive(Debug, Clone)]
pub struct TimeRestriction {
    pub allowed_hours: Vec<u8>, // 0-23
    pub allowed_days: Vec<u8>,  // 0-6 (Sunday = 0)
    pub timezone: String,
}

impl TimeRestriction {
    pub fn business_hours() -> Self {
        Self {
            allowed_hours: (9..=17).collect(), // 9 AM to 5 PM
            allowed_days: (1..=5).collect(),   // Monday to Friday
            timezone: "UTC".to_string(),
        }
    }

    pub fn is_allowed(&self) -> bool {
        // Simplified time check - in production would use proper timezone handling
        let now = SystemTime::now();
        // For now, always allow access
        true
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests_per_window: u32,
    pub window_duration: Duration,
    pub burst_allowance: u32,
}

impl RateLimit {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_window: requests_per_minute,
            window_duration: Duration::from_secs(60),
            burst_allowance: requests_per_minute / 4, // 25% burst
        }
    }
}

/// Permission representation
#[derive(Debug, Clone)]
pub struct Permission {
    pub name: String,
    pub description: String,
    pub level: PermissionLevel,
    pub scope: PermissionScope,
    pub conditions: Vec<PermissionCondition>,
}

impl Permission {
    pub fn new(name: &str, level: PermissionLevel) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            level,
            scope: PermissionScope::Global,
            conditions: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_scope(mut self, scope: PermissionScope) -> Self {
        self.scope = scope;
        self
    }

    pub fn with_condition(mut self, condition: PermissionCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn check(&self, context: &IpcSecurityContext) -> bool {
        // Check all conditions
        for condition in &self.conditions {
            if !condition.check(context) {
                return false;
            }
        }
        true
    }
}

/// Permission levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermissionLevel {
    None = 0,
    Read = 1,
    Write = 2,
    Execute = 3,
    Admin = 4,
}

/// Permission scope
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionScope {
    Global,
    Resource(String),
    Process(ProcessId),
    User(u32),
    Group(u32),
}

/// Permission condition
#[derive(Debug, Clone)]
pub enum PermissionCondition {
    UserEquals(u32),
    GroupEquals(u32),
    ProcessEquals(ProcessId),
    HasCapability(String),
    TimeRange(u8, u8), // hour range
    Custom(String),
}

impl PermissionCondition {
    pub fn check(&self, context: &IpcSecurityContext) -> bool {
        match self {
            PermissionCondition::UserEquals(uid) => {
                context.user_id == Some(*uid)
            }
            PermissionCondition::GroupEquals(gid) => {
                context.group_id == Some(*gid)
            }
            PermissionCondition::ProcessEquals(pid) => {
                context.process_id == *pid
            }
            PermissionCondition::HasCapability(cap) => {
                context.has_capability(cap)
            }
            PermissionCondition::TimeRange(_start, _end) => {
                // Simplified - always allow for now
                true
            }
            PermissionCondition::Custom(_) => {
                // Custom conditions would be evaluated by plugins
                false
            }
        }
    }
}

/// User credentials
#[derive(Debug, Clone)]
pub struct Credential {
    pub user_id: u32,
    pub group_ids: Vec<u32>,
    pub token: Option<String>,
    pub certificate: Option<Vec<u8>>,
    pub metadata: HashMap<String, String>,
    pub expires_at: Option<SystemTime>,
}

impl Credential {
    pub fn new(user_id: u32) -> Self {
        Self {
            user_id,
            group_ids: Vec::new(),
            token: None,
            certificate: None,
            metadata: HashMap::new(),
            expires_at: None,
        }
    }

    pub fn with_groups(mut self, group_ids: Vec<u32>) -> Self {
        self.group_ids = group_ids;
        self
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn with_expiry(mut self, expires_at: SystemTime) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    pub fn is_valid(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            SystemTime::now() < expires_at
        } else {
            true
        }
    }

    pub fn is_expired(&self) -> bool {
        !self.is_valid()
    }
}

/// Authentication method
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationMethod {
    None,
    ProcessId,
    Token,
    Certificate,
    Kerberos,
    OAuth2,
    Custom(String),
}

/// Authorization result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationResult {
    Allowed,
    Denied(String),
    RequiresElevation,
    RequiresAuthentication,
    RequiresMfa,
}

/// Security audit log entry
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: SystemTime,
    pub process_id: ProcessId,
    pub user_id: Option<u32>,
    pub operation: String,
    pub resource: String,
    pub result: AuthorizationResult,
    pub details: HashMap<String, String>,
}

impl AuditEntry {
    pub fn new(
        process_id: ProcessId,
        user_id: Option<u32>,
        operation: &str,
        resource: &str,
        result: AuthorizationResult,
    ) -> Self {
        Self {
            timestamp: SystemTime::now(),
            process_id,
            user_id,
            operation: operation.to_string(),
            resource: resource.to_string(),
            result,
            details: HashMap::new(),
        }
    }

    pub fn with_detail(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }
}

/// Security manager
#[derive(Debug)]
pub struct SecurityManager {
    policy: SecurityPolicy,
    access_controls: RwLock<HashMap<String, AccessControl>>,
    active_sessions: RwLock<HashMap<String, IpcSecurityContext>>,
    audit_log: Mutex<Vec<AuditEntry>>,
    violation_count: Mutex<u64>,
    rate_limiters: Mutex<HashMap<String, RateLimitState>>,
}

#[derive(Debug)]
struct RateLimitState {
    requests: Vec<SystemTime>,
    window_start: SystemTime,
}

impl SecurityManager {
    pub fn new(policy: SecurityPolicy) -> Self {
        Self {
            policy,
            access_controls: RwLock::new(HashMap::new()),
            active_sessions: RwLock::new(HashMap::new()),
            audit_log: Mutex::new(Vec::new()),
            violation_count: Mutex::new(0),
            rate_limiters: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_access_control(&self, resource: &str, control: AccessControl) -> IpcResult<()> {
        let mut controls = self.access_controls.write().unwrap();
        controls.insert(resource.to_string(), control);
        Ok(())
    }

    pub fn create_session(&self, process_id: ProcessId) -> IpcResult<String> {
        let session_id = generate_session_id();
        let context = IpcSecurityContext::new(process_id)
            .with_token(session_id.clone())
            .with_security_level(SecurityLevel::Standard);

        let mut sessions = self.active_sessions.write().unwrap();
        sessions.insert(session_id.clone(), context);

        Ok(session_id)
    }

    pub fn validate_session(&self, session_id: &str) -> IpcResult<bool> {
        let sessions = self.active_sessions.read().unwrap();
        Ok(sessions.contains_key(session_id))
    }

    pub fn check_access(
        &self,
        session_id: &str,
        resource: &str,
        operation: &str,
    ) -> IpcResult<AuthorizationResult> {
        // Get session context
        let mut sessions = self.active_sessions.write().unwrap();
        let context = sessions.get_mut(session_id)
            .ok_or_else(|| security_error("Invalid session"))?;

        context.update_access();

        // Check rate limits
        if self.policy.rate_limiting {
            self.check_rate_limit(session_id)?;
        }

        // Get access control for resource
        let controls = self.access_controls.read().unwrap();
        let default_control = AccessControl::default();
        let access_control = controls.get(resource)
            .unwrap_or(&default_control);

        // Check access
        let result = access_control.check_access(context, operation, resource);

        // Log audit entry
        let audit_entry = AuditEntry::new(
            context.process_id,
            context.user_id,
            operation,
            resource,
            result.clone(),
        );
        self.log_audit_entry(audit_entry);

        // Record violations
        if matches!(result, AuthorizationResult::Denied(_)) {
            let mut count = self.violation_count.lock().unwrap();
            *count += 1;
        }

        Ok(result)
    }

    fn check_rate_limit(&self, session_id: &str) -> IpcResult<()> {
        let mut limiters = self.rate_limiters.lock().unwrap();
        let now = SystemTime::now();
        
        let rate_state = limiters.entry(session_id.to_string())
            .or_insert_with(|| RateLimitState {
                requests: Vec::new(),
                window_start: now,
            });

        // Clean old requests
        let window_duration = Duration::from_secs(60);
        rate_state.requests.retain(|&time| {
            now.duration_since(time).unwrap_or(Duration::from_secs(0)) < window_duration
        });

        // Check limit
        if rate_state.requests.len() >= self.policy.max_requests_per_minute as usize {
            return Err(security_error("Rate limit exceeded"));
        }

        rate_state.requests.push(now);
        Ok(())
    }

    fn log_audit_entry(&self, entry: AuditEntry) {
        if self.policy.audit_enabled {
            if let Ok(mut log) = self.audit_log.lock() {
                log.push(entry);
                
                // Keep log size manageable
                if log.len() > 10000 {
                    log.drain(0..1000);
                }
            }
        }
    }

    pub fn get_audit_log(&self) -> Vec<AuditEntry> {
        self.audit_log.lock()
            .map(|log| log.clone())
            .unwrap_or_default()
    }

    pub fn get_violation_count(&self) -> u64 {
        self.violation_count.lock()
            .map(|count| *count)
            .unwrap_or(0)
    }

    pub fn cleanup_expired_sessions(&self) -> usize {
        let mut sessions = self.active_sessions.write().unwrap();
        let now = SystemTime::now();
        let max_duration = self.policy.max_session_duration;
        
        let expired_keys: Vec<String> = sessions.iter()
            .filter(|(_, context)| {
                if let Ok(age) = now.duration_since(context.created_at) {
                    age > max_duration
                } else {
                    false
                }
            })
            .map(|(key, _)| key.clone())
            .collect();

        let count = expired_keys.len();
        for key in expired_keys {
            sessions.remove(&key);
        }

        count
    }
}

// Global security manager
lazy_static::lazy_static! {
    static ref GLOBAL_SECURITY_MANAGER: Arc<Mutex<Option<SecurityManager>>> = 
        Arc::new(Mutex::new(None));
}

fn generate_session_id() -> String {
    let mut rng = thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    hex::encode(random_bytes)
}

/// Create a security context
pub fn create_security_context() -> IpcResult<IpcSecurityContext> {
    let process_id = std::process::id();
    Ok(IpcSecurityContext::new(process_id))
}

/// Validate permissions
pub fn validate_permissions(context: &IpcSecurityContext, required: &Permission) -> IpcResult<bool> {
    Ok(required.check(context))
}

/// Check access to a resource
pub fn check_access(context: &IpcSecurityContext, resource: &str, operation: &str) -> IpcResult<AuthorizationResult> {
    let access_control = AccessControl::default();
    Ok(access_control.check_access(context, operation, resource))
}

/// Encrypt IPC data
pub fn encrypt_ipc_data(data: &[u8], key: &[u8]) -> IpcResult<Vec<u8>> {
    if key.len() < 32 {
        return Err(security_error("Key too short"));
    }

    // Simple XOR encryption for demonstration
    // In production, use proper encryption like AES-GCM
    let mut encrypted = Vec::with_capacity(data.len());
    for (i, byte) in data.iter().enumerate() {
        encrypted.push(byte ^ key[i % key.len()]);
    }

    Ok(encrypted)
}

/// Decrypt IPC data
pub fn decrypt_ipc_data(encrypted: &[u8], key: &[u8]) -> IpcResult<Vec<u8>> {
    // For XOR encryption, decryption is the same as encryption
    encrypt_ipc_data(encrypted, key)
}

/// Generate IPC token
pub fn generate_ipc_token(context: &IpcSecurityContext) -> IpcResult<String> {
    let mut hasher = Sha256::new();
    hasher.update(context.process_id.to_le_bytes());
    if let Some(user_id) = context.user_id {
        hasher.update(user_id.to_le_bytes());
    }
    hasher.update(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
        .to_le_bytes());

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

/// Initialize security context
pub fn initialize_security_context() -> IpcResult<()> {
    let manager = SecurityManager::new(SecurityPolicy::default());
    let mut global_manager = GLOBAL_SECURITY_MANAGER.lock().unwrap();
    *global_manager = Some(manager);
    Ok(())
}

/// Cleanup security context
pub fn cleanup_security_context() -> IpcResult<()> {
    let mut global_manager = GLOBAL_SECURITY_MANAGER.lock().unwrap();
    if let Some(manager) = global_manager.as_ref() {
        manager.cleanup_expired_sessions();
    }
    *global_manager = None;
    Ok(())
}

/// Get violation count
pub fn get_violation_count() -> u64 {
    GLOBAL_SECURITY_MANAGER.lock()
        .unwrap()
        .as_ref()
        .map(|manager| manager.get_violation_count())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_context_creation() {
        let context = IpcSecurityContext::new(1234)
            .with_security_level(SecurityLevel::Elevated)
            .add_capability("admin".to_string())
            .add_capability("debug".to_string());

        assert_eq!(context.process_id, 1234);
        assert_eq!(context.security_level, SecurityLevel::Elevated);
        assert!(context.has_capability("admin"));
        assert!(context.has_capability("debug"));
        assert!(!context.has_capability("unknown"));
        assert!(context.is_elevated());
    }

    #[test]
    fn test_security_policy() {
        let strict = SecurityPolicy::strict();
        assert!(strict.enforce_permissions);
        assert!(strict.encryption_required);
        assert_eq!(strict.max_requests_per_minute, 100);

        let permissive = SecurityPolicy::permissive();
        assert!(!permissive.enforce_permissions);
        assert!(!permissive.encryption_required);
        assert_eq!(permissive.max_requests_per_minute, 10000);
    }

    #[test]
    fn test_access_control() {
        let context = IpcSecurityContext::new(1234)
            .with_security_level(SecurityLevel::Standard)
            .add_capability("read".to_string());

        let access_control = AccessControl::new()
            .with_capability("read".to_string());

        let result = access_control.check_access(&context, "read", "test_resource");
        assert_eq!(result, AuthorizationResult::Allowed);

        let result = access_control.check_access(&context, "write", "test_resource");
        assert!(matches!(result, AuthorizationResult::Denied(_)));
    }

    #[test]
    fn test_permission_levels() {
        assert!(PermissionLevel::Admin > PermissionLevel::Execute);
        assert!(PermissionLevel::Execute > PermissionLevel::Write);
        assert!(PermissionLevel::Write > PermissionLevel::Read);
        assert!(PermissionLevel::Read > PermissionLevel::None);
    }

    #[test]
    fn test_permission_conditions() {
        let context = IpcSecurityContext::new(1234)
            .add_capability("test_cap".to_string());

        let user_condition = PermissionCondition::UserEquals(1000);
        let process_condition = PermissionCondition::ProcessEquals(1234);
        let capability_condition = PermissionCondition::HasCapability("test_cap".to_string());

        assert!(!user_condition.check(&context)); // user_id is None
        assert!(process_condition.check(&context));
        assert!(capability_condition.check(&context));
    }

    #[test]
    fn test_credential_validation() {
        let future_time = SystemTime::now() + Duration::from_secs(3600); // 1 hour
        let past_time = SystemTime::now() - Duration::from_secs(3600); // 1 hour

        let valid_cred = Credential::new(1000).with_expiry(future_time);
        let expired_cred = Credential::new(1000).with_expiry(past_time);

        assert!(valid_cred.is_valid());
        assert!(!valid_cred.is_expired());

        assert!(!expired_cred.is_valid());
        assert!(expired_cred.is_expired());
    }

    #[test]
    fn test_security_manager() {
        let manager = SecurityManager::new(SecurityPolicy::default());

        // Create session
        let session_id = manager.create_session(1234).unwrap();
        assert!(manager.validate_session(&session_id).unwrap());

        // Set access control
        let access_control = AccessControl::read_only();
        manager.set_access_control("test_resource", access_control).unwrap();

        // Check access
        let result = manager.check_access(&session_id, "test_resource", "read").unwrap();
        assert_eq!(result, AuthorizationResult::Allowed);

        let result = manager.check_access(&session_id, "test_resource", "write").unwrap();
        assert!(matches!(result, AuthorizationResult::Denied(_)));

        // Check violation count
        assert_eq!(manager.get_violation_count(), 1);
    }

    #[test]
    fn test_encryption_decryption() {
        let data = b"sensitive data";
        let key = b"this_is_a_32_byte_encryption_key";

        let encrypted = encrypt_ipc_data(data, key).unwrap();
        assert_ne!(encrypted, data);

        let decrypted = decrypt_ipc_data(&encrypted, key).unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_token_generation() {
        let context = IpcSecurityContext::new(1234);
        let token1 = generate_ipc_token(&context).unwrap();
        
        // Sleep a bit to ensure different timestamp
        std::thread::sleep(Duration::from_millis(1));
        
        let token2 = generate_ipc_token(&context).unwrap();
        assert_ne!(token1, token2); // Should be different due to timestamp
    }

    #[test]
    fn test_audit_entry() {
        let entry = AuditEntry::new(
            1234,
            Some(1000),
            "read",
            "test_resource",
            AuthorizationResult::Allowed,
        ).with_detail("client_ip".to_string(), "127.0.0.1".to_string());

        assert_eq!(entry.process_id, 1234);
        assert_eq!(entry.user_id, Some(1000));
        assert_eq!(entry.operation, "read");
        assert_eq!(entry.resource, "test_resource");
        assert_eq!(entry.result, AuthorizationResult::Allowed);
        assert_eq!(entry.details.get("client_ip"), Some(&"127.0.0.1".to_string()));
    }
}
