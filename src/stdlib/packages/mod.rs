/// fr fr Packages module for CURSED stdlib - modular library organization

// Database packages - comprehensive database connectivity
pub mod db_core;
pub mod db_sql;
pub mod db_nosql;
pub mod db_pool;
pub mod db_query;
pub mod db_orm;
pub mod db_migrate;

// Cryptography packages - comprehensive security suite
pub mod crypto_advanced;
pub mod crypto_asymmetric;
pub mod crypto_signatures;
pub mod crypto_kdf;
pub mod crypto_hash_advanced;
pub mod crypto_random;
pub mod crypto_zk;
pub mod crypto_pqc;
pub mod crypto_pki;
pub mod crypto_protocols;

// Existing packages
pub mod web_vibez;
pub mod sql_vibes;
pub mod test_vibes;

// Re-export database packages for convenience
pub use db_core::*;
pub use db_sql::*;

// Re-export cryptography packages for convenience
pub use crypto_advanced::*;
pub use crypto_asymmetric::*;
pub use crypto_signatures::*;
pub use crypto_kdf::*;
pub use crypto_hash_advanced::*;
pub use crypto_random::*;
pub use crypto_zk::*;
pub use crypto_pqc::*;
pub use crypto_pki::*;
pub use crypto_protocols::*;

// Re-export existing packages for convenience
pub use web_vibez::*;
pub use sql_vibes::*;
pub use test_vibes::*;
