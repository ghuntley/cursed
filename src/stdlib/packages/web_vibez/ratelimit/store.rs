use crate::error::Error;
/// fr fr Rate limit storage backends - persistent state management
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use async_trait::async_trait;

use super::{ClientState, RateLimitResult, RateLimitError, current_timestamp};

/// fr fr Rate limit store trait - storage abstraction
#[async_trait::async_trait]
pub trait RateLimitStore: Send + Sync {
    /// fr fr Get client state from store - retrieve current status
    async fn get_client_state(&self, client_id: &str) -> RateLimitResult<ClientState>;
    
    /// fr fr Update client state in store - persist changes
    async fn update_client_state(&self, client_id: &str, state: &ClientState) -> RateLimitResult<()>;
    
    /// fr fr Reset client state - administrative cleanup
    async fn reset_client(&self, client_id: &str) -> RateLimitResult<()>;
    
    /// fr fr Cleanup expired states - maintenance
    async fn cleanup_expired(&self, ttl: Duration) -> RateLimitResult<u64>;
    
    /// fr fr Get store statistics - monitoring
    async fn get_stats(&self) -> RateLimitResult<StoreStats>;
}

/// fr fr Store statistics - monitoring information
#[derive(Debug, Clone)]
pub struct StoreStats {
    pub total_clients: u64,
    pub active_clients: u64,
    pub memory_usage: u64,
    pub last_cleanup: u64,
}

/// fr fr In-memory rate limit store - fast local storage
pub struct InMemoryStore {
    clients: Arc<RwLock<HashMap<String, ClientState>>>,
    stats: Arc<RwLock<StoreStats>>,
}

impl InMemoryStore {
    /// fr fr Create new in-memory store - local setup
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(StoreStats {
                total_clients: 0,
                active_clients: 0,
                memory_usage: 0,
                last_cleanup: current_timestamp(),
            })),
        }
    }

    /// fr fr Get client count - monitoring
    pub fn client_count(&self) -> usize {
        self.clients.read().unwrap().len()
    }

    /// fr fr Clear all clients - reset store
    pub fn clear(&self) {
        self.clients.write().unwrap().clear();
        let mut stats = self.stats.write().unwrap();
        stats.total_clients = 0;
        stats.active_clients = 0;
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl RateLimitStore for InMemoryStore {
    async fn get_client_state(&self, client_id: &str) -> RateLimitResult<ClientState> {
        let clients = self.clients.read().unwrap();
        Ok(clients.get(client_id).cloned().unwrap_or_default())
    }

    async fn update_client_state(&self, client_id: &str, state: &ClientState) -> RateLimitResult<()> {
        let mut clients = self.clients.write().unwrap();
        let is_new = !clients.contains_key(client_id);
        clients.insert(client_id.to_string(), state.clone());
        
        if is_new {
            let mut stats = self.stats.write().unwrap();
            stats.total_clients += 1;
            stats.active_clients = clients.len() as u64;
        }
        
        Ok(())
    }

    async fn reset_client(&self, client_id: &str) -> RateLimitResult<()> {
        let mut clients = self.clients.write().unwrap();
        clients.remove(client_id);
        
        let mut stats = self.stats.write().unwrap();
        stats.active_clients = clients.len() as u64;
        
        Ok(())
    }

    async fn cleanup_expired(&self, ttl: Duration) -> RateLimitResult<u64> {
        let now = current_timestamp();
        let expiry_threshold = now.saturating_sub(ttl.as_secs());
        
        let mut clients = self.clients.write().unwrap();
        let initial_count = clients.len();
        
        clients.retain(|_, state| state.last_request > expiry_threshold);
        
        let removed_count = (initial_count - clients.len()) as u64;
        
        let mut stats = self.stats.write().unwrap();
        stats.active_clients = clients.len() as u64;
        stats.last_cleanup = now;
        
        Ok(removed_count)
    }

    async fn get_stats(&self) -> RateLimitResult<StoreStats> {
        Ok(self.stats.read().unwrap().clone())
    }
}

/// fr fr Redis-compatible store interface - distributed storage
pub struct RedisStore {
    // In a real implementation, this would contain Redis connection
    // For now, we'll use in-memory as fallback with Redis-like interface
    fallback: InMemoryStore,
    redis_url: String,
    connected: bool,
}

impl RedisStore {
    /// fr fr Create new Redis store - distributed setup
    pub fn new(redis_url: String) -> Self {
        Self {
            fallback: InMemoryStore::new(),
            redis_url,
            connected: false, // Placeholder - real implementation would connect
        }
    }

    /// fr fr Connect to Redis - establish connection
    pub async fn connect(&mut self) -> RateLimitResult<()> {
        // In real implementation, would establish Redis connection
        // For now, just mark as connected
        self.connected = true;
        Ok(())
    }

    /// fr fr Check if connected to Redis - status check
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// fr fr Get Redis URL - configuration
    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }
}

#[async_trait::async_trait]
impl RateLimitStore for RedisStore {
    async fn get_client_state(&self, client_id: &str) -> RateLimitResult<ClientState> {
        if self.connected {
            // In real implementation, would query Redis
            // For now, fallback to in-memory
            self.fallback.get_client_state(client_id).await
        } else {
            Err(RateLimitError::StoreError("Redis not connected".to_string()))
        }
    }

    async fn update_client_state(&self, client_id: &str, state: &ClientState) -> RateLimitResult<()> {
        if self.connected {
            // In real implementation, would update Redis
            // For now, fallback to in-memory
            self.fallback.update_client_state(client_id, state).await
        } else {
            Err(RateLimitError::StoreError("Redis not connected".to_string()))
        }
    }

    async fn reset_client(&self, client_id: &str) -> RateLimitResult<()> {
        if self.connected {
            // In real implementation, would delete from Redis
            self.fallback.reset_client(client_id).await
        } else {
            Err(RateLimitError::StoreError("Redis not connected".to_string()))
        }
    }

    async fn cleanup_expired(&self, ttl: Duration) -> RateLimitResult<u64> {
        if self.connected {
            // In real implementation, would use Redis TTL/EXPIRE
            self.fallback.cleanup_expired(ttl).await
        } else {
            Err(RateLimitError::StoreError("Redis not connected".to_string()))
        }
    }

    async fn get_stats(&self) -> RateLimitResult<StoreStats> {
        if self.connected {
            // In real implementation, would query Redis INFO
            self.fallback.get_stats().await
        } else {
            Err(RateLimitError::StoreError("Redis not connected".to_string()))
        }
    }
}

/// fr fr Store backend enum - concrete implementations
#[derive(Clone)]
pub enum StoreBackend {
    InMemory(Arc<InMemoryStore>),
    Redis(Arc<RedisStore>),
}

impl StoreBackend {
    /// fr fr Get client state from store - retrieve current status
    pub async fn get_client_state(&self, client_id: &str) -> RateLimitResult<ClientState> {
        match self {
            StoreBackend::InMemory(store) => store.get_client_state(client_id).await,
            StoreBackend::Redis(store) => store.get_client_state(client_id).await,
        }
    }
    
    /// fr fr Update client state in store - persist changes
    pub async fn update_client_state(&self, client_id: &str, state: &ClientState) -> RateLimitResult<()> {
        match self {
            StoreBackend::InMemory(store) => store.update_client_state(client_id, state).await,
            StoreBackend::Redis(store) => store.update_client_state(client_id, state).await,
        }
    }
    
    /// fr fr Reset client state - administrative cleanup
    pub async fn reset_client(&self, client_id: &str) -> RateLimitResult<()> {
        match self {
            StoreBackend::InMemory(store) => store.reset_client(client_id).await,
            StoreBackend::Redis(store) => store.reset_client(client_id).await,
        }
    }
    
    /// fr fr Cleanup expired states - maintenance
    pub async fn cleanup_expired(&self, ttl: Duration) -> RateLimitResult<u64> {
        match self {
            StoreBackend::InMemory(store) => store.cleanup_expired(ttl).await,
            StoreBackend::Redis(store) => store.cleanup_expired(ttl).await,
        }
    }
    
    /// fr fr Get store statistics - monitoring
    pub async fn get_stats(&self) -> RateLimitResult<StoreStats> {
        match self {
            StoreBackend::InMemory(store) => store.get_stats().await,
            StoreBackend::Redis(store) => store.get_stats().await,
        }
    }
}

/// fr fr Distributed store - multiple backend coordination
pub struct DistributedStore {
    primary: StoreBackend,
    replica: Option<StoreBackend>,
    write_to_replica: bool,
}

impl DistributedStore {
    /// fr fr Create new distributed store - multiple backends
    pub fn new(primary: StoreBackend) -> Self {
        Self {
            primary,
            replica: None,
            write_to_replica: false,
        }
    }

    /// fr fr Add replica store - backup storage
    pub fn with_replica(mut self, replica: StoreBackend, write_to_replica: bool) -> Self {
        self.replica = Some(replica);
        self.write_to_replica = write_to_replica;
        self
    }
}

#[async_trait::async_trait]
impl RateLimitStore for DistributedStore {
    async fn get_client_state(&self, client_id: &str) -> RateLimitResult<ClientState> {
        // Try primary first
        match self.primary.get_client_state(client_id).await {
            Ok(state) => Ok(state),
            Err(_) => {
                // Fallback to replica if available
                if let Some(replica) = &self.replica {
                    replica.get_client_state(client_id).await
                } else {
                    Ok(ClientState::default())
                }
            }
        }
    }

    async fn update_client_state(&self, client_id: &str, state: &ClientState) -> RateLimitResult<()> {
        // Always write to primary
        let primary_result = self.primary.update_client_state(client_id, state).await;
        
        // Write to replica if configured
        if self.write_to_replica {
            if let Some(replica) = &self.replica {
                let _ = replica.update_client_state(client_id, state).await;
            }
        }
        
        primary_result
    }

    async fn reset_client(&self, client_id: &str) -> RateLimitResult<()> {
        let primary_result = self.primary.reset_client(client_id).await;
        
        if let Some(replica) = &self.replica {
            let _ = replica.reset_client(client_id).await;
        }
        
        primary_result
    }

    async fn cleanup_expired(&self, ttl: Duration) -> RateLimitResult<u64> {
        let primary_result = self.primary.cleanup_expired(ttl).await;
        
        if let Some(replica) = &self.replica {
            let _ = replica.cleanup_expired(ttl).await;
        }
        
        primary_result
    }

    async fn get_stats(&self) -> RateLimitResult<StoreStats> {
        self.primary.get_stats().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_in_memory_store_basic() {
        let store = InMemoryStore::new();
        
        // Get non-existent client
        let state = store.get_client_state("test").await.unwrap();
        assert_eq!(state.count, 0);
        
        // Update client state
        let mut new_state = ClientState::new();
        new_state.count = 5;
        store.update_client_state("test", &new_state).await.unwrap();
        
        // Verify state persisted
        let retrieved_state = store.get_client_state("test").await.unwrap();
        assert_eq!(retrieved_state.count, 5);
        
        // Check client count
        assert_eq!(store.client_count(), 1);
    }

    #[tokio::test]
    async fn test_in_memory_store_cleanup() {
        let store = InMemoryStore::new();
        
        // Add some states with old timestamps
        let mut old_state = ClientState::new();
        old_state.last_request = current_timestamp() - 3600; // 1 hour ago
        store.update_client_state("old_client", &old_state).await.unwrap();
        
        let mut new_state = ClientState::new();
        new_state.last_request = current_timestamp(); // Now
        store.update_client_state("new_client", &new_state).await.unwrap();
        
        // Cleanup with 30 minute TTL
        let removed = store.cleanup_expired(Duration::from_secs(1800)).await.unwrap();
        assert_eq!(removed, 1);
        assert_eq!(store.client_count(), 1);
        
        // Verify correct client remains
        let remaining_state = store.get_client_state("new_client").await.unwrap();
        assert!(remaining_state.last_request > 0);
    }

    #[tokio::test]
    async fn test_redis_store_creation() {
        let mut store = RedisStore::new("redis://localhost:6379".to_string());
        assert!(!store.is_connected());
        assert_eq!(store.redis_url(), "redis://localhost:6379");
        
        // Test connection (placeholder)
        store.connect().await.unwrap();
        assert!(store.is_connected());
    }

    #[tokio::test]
    async fn test_distributed_store() {
        let primary = Arc::new(InMemoryStore::new());
        let replica = Arc::new(InMemoryStore::new());
        
        let distributed = DistributedStore::new(primary.clone())
            .with_replica(replica.clone(), true);
        
        // Update state
        let mut state = ClientState::new();
        state.count = 10;
        distributed.update_client_state("test", &state).await.unwrap();
        
        // Verify both stores have the data
        let primary_state = primary.get_client_state("test").await.unwrap();
        let replica_state = replica.get_client_state("test").await.unwrap();
        
        assert_eq!(primary_state.count, 10);
        assert_eq!(replica_state.count, 10);
    }

    #[tokio::test]
    async fn test_store_stats() {
        let store = InMemoryStore::new();
        
        // Initial stats
        let stats = store.get_stats().await.unwrap();
        assert_eq!(stats.active_clients, 0);
        
        // Add some clients
        for i in 0..5 {
            let state = ClientState::new();
            store.update_client_state(&format!("client_{}", i), &state).await.unwrap();
        }
        
        let updated_stats = store.get_stats().await.unwrap();
        assert_eq!(updated_stats.active_clients, 5);
        assert_eq!(updated_stats.total_clients, 5);
    }
}
