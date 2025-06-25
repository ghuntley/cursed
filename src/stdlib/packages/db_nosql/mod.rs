use crate::error::CursedError;
/// fr fr NoSQL database drivers - for when you need flexibility periodt
///
/// This package provides NoSQL database support for CURSED with drivers for
/// MongoDB, Redis, and other document/key-value stores. NoSQL vibes bestie!

// Core NoSQL modules
pub mod drivers;
pub mod document;
// pub mod mongodb;  // Temporarily disabled - mongodb crate not available
pub mod redis;

// Re-export important types
pub use drivers::{NoSqlDriver, NoSqlConnection};
pub use document::{Document, Collection};
// pub use mongodb::{
//     MongoDbDriver, MongoDbConnection, MongoDbDatabase, MongoDbCollection,
//     MongoDbConfig, MongoDbError, MongoDbQueryBuilder, MongoDbTransaction,
//     AggregationPipelineBuilder, WriteConcernConfig,
// };  // Temporarily disabled - mongodb crate not available
pub use redis::{RedisDriver, RedisConnection, RedisConfig, RedisConnectionPool};

/// slay Initialize the db_nosql package
// pub fn init_db_nosql() -> crate::stdlib::packages::db_core::error::DatabaseResult<()> {
    println!("📄 db_nosql package initialized - NoSQL drivers ready bestie!");
    Ok(())
