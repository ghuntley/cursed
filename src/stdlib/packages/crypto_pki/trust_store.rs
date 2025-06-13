//! Trust Store Management
//! 
//! Manage trusted certificate authorities and certificate validation.

use std::collections::HashMap;
use crate::stdlib::packages::crypto_pki::types::TrustStore;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};

/// Trust store manager for organizing multiple trust stores
#[derive(Debug)]
pub struct TrustStoreManager {
    /// Named trust stores
    trust_stores: HashMap<String, TrustStore>,
    /// Default trust store name
    default_store: Option<String>,
}

impl TrustStoreManager {
    /// Create a new trust store manager
    pub fn new() -> Self {
        Self {
            trust_stores: HashMap::new(),
            default_store: None,
        }
    }
    
    /// Add a trust store
    pub fn add_trust_store(&mut self, name: String, trust_store: TrustStore) {
        if self.default_store.is_none() {
            self.default_store = Some(name.clone());
        }
        self.trust_stores.insert(name, trust_store);
    }
    
    /// Get a trust store by name
    pub fn get_trust_store(&self, name: &str) -> Option<&TrustStore> {
        self.trust_stores.get(name)
    }
    
    /// Get the default trust store
    pub fn get_default_trust_store(&self) -> Option<&TrustStore> {
        if let Some(default_name) = &self.default_store {
            self.trust_stores.get(default_name)
        } else {
            None
        }
    }
}
