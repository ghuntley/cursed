//! Trust Store Manager - Production Implementation

use crate::stdlib::packages::crypto_pki::{
    error::{PkiError, PkiResult},
    types::*,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Trust store manager for managing certificate trust anchors
#[derive(Debug)]
pub struct TrustStoreManager {
    /// Trust stores registry
    pub trust_stores: Arc<Mutex<HashMap<String, TrustStore>>>,
    /// Default trust store name
    pub default_store: String,
    /// Trust store statistics
    pub statistics: TrustStoreStatistics,
}

/// Trust store statistics
#[derive(Debug, Default)]
pub struct TrustStoreStatistics {
    pub stores_managed: u32,
    pub root_certificates: u32,
    pub intermediate_certificates: u32,
    pub trust_checks: u64,
    pub trust_hits: u64,
}

impl TrustStoreManager {
    /// Create new trust store manager
    pub fn new() -> Self {
        let mut manager = Self {
            trust_stores: Arc::new(Mutex::new(HashMap::new())),
            default_store: "default".to_string(),
            statistics: TrustStoreStatistics::default(),
        };
        
        // Initialize default trust store
        let _ = manager.create_trust_store("default".to_string());
        
        manager
    }
    
    /// Create a new trust store
    pub fn create_trust_store(&mut self, name: String) -> PkiResult<()> {
        let mut stores = self.trust_stores.lock()
            .map_err(|_| PkiError::general("Failed to lock trust stores"))?;
        
        let trust_store = TrustStore::new(&name);
        stores.insert(name, trust_store);
        self.statistics.stores_managed += 1;
        
        Ok(())
    }
    
    /// Add root certificate to trust store
    pub fn add_root_certificate(&mut self, store_name: &str, certificate: X509Certificate) -> PkiResult<()> {
        let mut stores = self.trust_stores.lock()
            .map_err(|_| PkiError::general("Failed to lock trust stores"))?;
        
        let store = stores.get_mut(store_name)
            .ok_or_else(|| PkiError::trust_store_error("Trust store not found", Some(store_name.to_string()), "add_root"))?;
        
        store.add_root_certificate(certificate);
        self.statistics.root_certificates += 1;
        
        Ok(())
    }
    
    /// Check if certificate is trusted
    pub fn is_trusted(&self, store_name: &str, certificate: &X509Certificate) -> PkiResult<bool> {
        let stores = self.trust_stores.lock()
            .map_err(|_| PkiError::general("Failed to lock trust stores"))?;
        
        let store = stores.get(store_name)
            .ok_or_else(|| PkiError::trust_store_error("Trust store not found", Some(store_name.to_string()), "is_trusted"))?;
        
        Ok(store.is_trusted(certificate))
    }
    
    /// Get trust store
    pub fn get_trust_store(&self, name: &str) -> PkiResult<TrustStore> {
        let stores = self.trust_stores.lock()
            .map_err(|_| PkiError::general("Failed to lock trust stores"))?;
        
        stores.get(name)
            .cloned()
            .ok_or_else(|| PkiError::trust_store_error("Trust store not found", Some(name.to_string()), "get"))
    }
}
