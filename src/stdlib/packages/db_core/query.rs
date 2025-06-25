/// fr fr Database query types and management - where queries come to life periodt
///
/// This module defines query structures, parameter handling, caching,
/// and execution planning for database operations. Query optimization bestie!

use crate::stdlib::packages::db_core::error::{
    DatabaseError, ErrorKind, QueryError
};
use crate::error::Error;
use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};

/// fr fr Query types for different database operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryType {
    /// SELECT query
    Select,
    /// INSERT query
    Insert,
    /// UPDATE query
    Update,
    /// DELETE query
    Delete,
    /// CREATE query (DDL)
    Create,
    /// ALTER query (DDL)
    Alter,
    /// DROP query (DDL)
    Drop,
    /// STORED PROCEDURE call
    Call,
    /// BATCH operation
    Batch,
    /// UNKNOWN query type
    Unknown,
}

/// fr fr Main query structure
#[derive(Debug, Clone)]
pub struct Query {
    /// Unique query identifier
    pub id: String,
    /// Query type
    pub query_type: QueryType,
    /// SQL statement or command
    pub statement: String,
    /// Query parameters
    pub parameters: ParameterSet,
    /// Query metadata
    pub metadata: QueryMetadata,
    /// Query options
    pub options: QueryOptions,
}

/// fr fr Query builder for constructing queries
#[derive(Debug)]
pub struct QueryBuilder {
    query: Query,
}

/// fr fr Parameter handling for queries
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name (for named parameters)
    pub name: Option<String>,
    /// Parameter value as string (serialized)
    pub value: String,
    /// Parameter type hint
    pub type_hint: Option<String>,
    /// Parameter direction (IN/OUT/INOUT)
    pub direction: ParameterDirection,
}

/// fr fr Parameter direction for stored procedures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterDirection {
    /// Input parameter
    In,
    /// Output parameter
    Out,
    /// Input/Output parameter
    InOut,
}

/// fr fr Collection of parameters
#[derive(Debug, Clone)]
pub struct ParameterSet {
    /// Ordered list of parameters
    pub parameters: Vec<Parameter>,
    /// Named parameter mapping
    pub named_parameters: HashMap<String, usize>,
}

/// fr fr Query metadata for optimization and caching
#[derive(Debug, Clone)]
pub struct QueryMetadata {
    /// When query was created
    pub created_at: SystemTime,
    /// Last execution time
    pub last_executed: Option<SystemTime>,
    /// Execution count
    pub execution_count: u64,
    /// Average execution time
    pub avg_execution_time: Option<Duration>,
    /// Query complexity score (0-100)
    pub complexity_score: u8,
    /// Tables referenced by the query
    pub referenced_tables: Vec<String>,
    /// Columns referenced by the query
    pub referenced_columns: Vec<String>,
    /// Whether query modifies data
    pub is_modifying: bool,
    /// Whether query is cacheable
    pub is_cacheable: bool,
}

/// fr fr Query execution options
#[derive(Debug, Clone)]
pub struct QueryOptions {
    /// Query timeout
    pub timeout: Option<Duration>,
    /// Maximum number of rows to return
    pub max_rows: Option<usize>,
    /// Fetch size for large result sets
    pub fetch_size: Option<usize>,
    /// Whether to use prepared statements
    pub use_prepared: bool,
    /// Whether to cache the query plan
    pub cache_plan: bool,
    /// Whether to enable query optimization
    pub optimize: bool,
    /// Custom execution hints
    pub hints: HashMap<String, String>,
}

/// fr fr SQL-specific query structure
#[derive(Debug, Clone)]
pub struct SqlQuery {
    /// Base query
    pub base: Query,
    /// SQL dialect
    pub dialect: String,
    /// SQL-specific options
    pub sql_options: SqlQueryOptions,
}

/// fr fr NoSQL-specific query structure
#[derive(Debug, Clone)]
pub struct NoSqlQuery {
    /// Base query
    pub base: Query,
    /// NoSQL database type
    pub database_type: String,
    /// Collection/table name
    pub collection: String,
    /// Query document/filter
    pub filter: HashMap<String, serde_json::Value>,
    /// Projection fields
    pub projection: Option<Vec<String>>,
    /// Sort specification
    pub sort: Option<HashMap<String, i32>>,
    /// Skip/offset
    pub skip: Option<u64>,
    /// Limit
    pub limit: Option<u64>,
}

/// fr fr SQL-specific query options
#[derive(Debug, Clone)]
pub struct SqlQueryOptions {
    /// Transaction isolation level
    pub isolation_level: Option<String>,
    /// Whether to use read-only connection
    pub read_only: bool,
    /// Auto-commit setting
    pub auto_commit: Option<bool>,
    /// Query execution plan preference
    pub execution_plan: Option<String>,
}

/// fr fr Query execution plan
#[derive(Debug, Clone)]
pub struct QueryPlan {
    /// Plan identifier
    pub id: String,
    /// Query this plan is for
    pub query_id: String,
    /// Estimated execution cost
    pub estimated_cost: f64,
    /// Estimated number of rows
    pub estimated_rows: Option<usize>,
    /// Execution steps
    pub steps: Vec<ExecutionStep>,
    /// Plan creation time
    pub created_at: SystemTime,
    /// Whether plan is optimal
    pub is_optimal: bool,
}

/// fr fr Query execution step
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    /// Step identifier
    pub id: String,
    /// Step type (scan, join, sort, etc.)
    pub step_type: String,
    /// Target table/collection
    pub target: Option<String>,
    /// Index used (if any)
    pub index_name: Option<String>,
    /// Estimated cost for this step
    pub estimated_cost: f64,
    /// Estimated rows for this step
    pub estimated_rows: Option<usize>,
    /// Step description
    pub description: String,
}

/// fr fr Query cache for performance optimization
#[derive(Debug)]
pub struct QueryCache {
    /// Cached queries
    cache: HashMap<String, CachedQuery>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: CacheStats,
}

/// fr fr Cached query entry
#[derive(Debug, Clone)]
pub struct CachedQuery {
    /// Original query
    pub query: Query,
    /// Cached execution plan
    pub plan: Option<QueryPlan>,
    /// Cache entry metadata
    pub metadata: CacheEntryMetadata,
}

/// fr fr Cache entry metadata
#[derive(Debug, Clone)]
pub struct CacheEntryMetadata {
    /// When entry was created
    pub created_at: SystemTime,
    /// When entry was last accessed
    pub last_accessed: SystemTime,
    /// Number of times accessed
    pub access_count: u64,
    /// Entry size in bytes
    pub size_bytes: usize,
    /// Whether entry is pinned
    pub is_pinned: bool,
}

/// fr fr Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of cached queries
    pub max_entries: usize,
    /// Maximum cache size in bytes
    pub max_size_bytes: usize,
    /// Entry time-to-live
    pub ttl: Duration,
    /// Whether to enable cache
    pub enabled: bool,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
}

/// fr fr Cache eviction policies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    /// Least Frequently Used
    LFU,
    /// First In, First Out
    FIFO,
    /// Time-based expiration
    TTL,
}

/// fr fr Cache statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Total cache evictions
    pub evictions: u64,
    /// Current cache size
    pub current_size: usize,
    /// Current entry count
    pub current_entries: usize,
}

impl Query {
    /// slay Create a new query
    pub fn new(statement: &str) -> Self {
        let query_type = Self::detect_query_type(statement);
        let id = Self::generate_id(statement);
        
        Self {
            id,
            query_type: query_type.clone(),
            statement: statement.to_string(),
            parameters: ParameterSet::new(),
            metadata: QueryMetadata::new(query_type == QueryType::Select),
            options: QueryOptions::default(),
        }
    }

    /// slay Detect query type from statement
    fn detect_query_type(statement: &str) -> QueryType {
        let trimmed = statement.trim_start().to_uppercase();
        
        if trimmed.starts_with("SELECT") {
            QueryType::Select
        } else if trimmed.starts_with("INSERT") {
            QueryType::Insert
        } else if trimmed.starts_with("UPDATE") {
            QueryType::Update
        } else if trimmed.starts_with("DELETE") {
            QueryType::Delete
        } else if trimmed.starts_with("CREATE") {
            QueryType::Create
        } else if trimmed.starts_with("ALTER") {
            QueryType::Alter
        } else if trimmed.starts_with("DROP") {
            QueryType::Drop
        } else if trimmed.starts_with("CALL") || trimmed.starts_with("EXEC") {
            QueryType::Call
        } else {
            QueryType::Unknown
        }
    }

    /// slay Generate query ID
    fn generate_id(statement: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        statement.hash(&mut hasher);
        format!("query_{:016x}", hasher.finish())
    }

    /// slay Add parameter to query
    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.parameters.add_parameter(parameter);
    }

    /// slay Add named parameter
    pub fn add_named_parameter(&mut self, name: &str, value: &str, type_hint: Option<String>) {
        let parameter = Parameter {
            name: Some(name.to_string()),
            value: value.to_string(),
            type_hint,
            direction: ParameterDirection::In,
        };
        self.parameters.add_parameter(parameter);
    }

    /// slay Validate query
    pub fn validate(&self) -> DbResult<()> {
        if self.statement.trim().is_empty() {
            return Err(DatabaseError::query(
                QueryError::SyntaxError,
                "Query statement cannot be empty"
            ));
        }

        // Basic SQL injection check (very basic)
        if self.statement.contains("--") || self.statement.contains("/*") {
            return Err(DatabaseError::query(
                QueryError::SyntaxError,
                "Query contains potentially unsafe comments"
            ));
        }

        Ok(())
    }

    /// slay Get query hash for caching
    pub fn get_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.statement.hash(&mut hasher);
        self.parameters.hash(&mut hasher);
        hasher.finish()
    }
}

impl QueryBuilder {
    /// slay Create a new query builder
    pub fn new() -> Self {
        Self {
            query: Query::new(""),
        }
    }

    /// slay Set query statement
    pub fn statement(mut self, statement: &str) -> Self {
        self.query = Query::new(statement);
        self
    }

    /// slay Add parameter
    pub fn parameter(mut self, name: &str, value: &str) -> Self {
        self.query.add_named_parameter(name, value, None);
        self
    }

    /// slay Set timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.query.options.timeout = Some(timeout);
        self
    }

    /// slay Set max rows
    pub fn max_rows(mut self, max_rows: usize) -> Self {
        self.query.options.max_rows = Some(max_rows);
        self
    }

    /// slay Enable prepared statements
    pub fn use_prepared(mut self) -> Self {
        self.query.options.use_prepared = true;
        self
    }

    /// slay Build the query
    pub fn build(self) -> DbResult<Query> {
        self.query.validate()?;
        Ok(self.query)
    }
}

impl Parameter {
    /// slay Create a new input parameter
    pub fn input(value: &str) -> Self {
        Self {
            name: None,
            value: value.to_string(),
            type_hint: None,
            direction: ParameterDirection::In,
        }
    }

    /// slay Create a new named input parameter
    pub fn named_input(name: &str, value: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            value: value.to_string(),
            type_hint: None,
            direction: ParameterDirection::In,
        }
    }

    /// slay Create a new output parameter
    pub fn output(type_hint: &str) -> Self {
        Self {
            name: None,
            value: String::new(),
            type_hint: Some(type_hint.to_string()),
            direction: ParameterDirection::Out,
        }
    }

    /// slay Create a new named output parameter
    pub fn named_output(name: &str, type_hint: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            value: String::new(),
            type_hint: Some(type_hint.to_string()),
            direction: ParameterDirection::Out,
        }
    }
}

impl ParameterSet {
    /// slay Create a new parameter set
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            named_parameters: HashMap::new(),
        }
    }

    /// slay Add a parameter
    pub fn add_parameter(&mut self, parameter: Parameter) {
        let index = self.parameters.len();
        
        if let Some(name) = &parameter.name {
            self.named_parameters.insert(name.clone(), index);
        }
        
        self.parameters.push(parameter);
    }

    /// slay Get parameter by index
    pub fn get(&self, index: usize) -> Option<&Parameter> {
        self.parameters.get(index)
    }

    /// slay Get parameter by name
    pub fn get_by_name(&self, name: &str) -> Option<&Parameter> {
        self.named_parameters.get(name)
            .and_then(|&index| self.parameters.get(index))
    }

    /// slay Get parameter count
    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    /// slay Check if parameter set is empty
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }
}

impl Hash for ParameterSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for param in &self.parameters {
            param.name.hash(state);
            param.value.hash(state);
            param.type_hint.hash(state);
        }
    }
}

impl QueryMetadata {
    /// slay Create new metadata
    pub fn new(is_cacheable: bool) -> Self {
        Self {
            created_at: SystemTime::now(),
            last_executed: None,
            execution_count: 0,
            avg_execution_time: None,
            complexity_score: 0,
            referenced_tables: Vec::new(),
            referenced_columns: Vec::new(),
            is_modifying: false,
            is_cacheable,
        }
    }

    /// slay Record query execution
    pub fn record_execution(&mut self, execution_time: Duration) {
        self.last_executed = Some(SystemTime::now());
        self.execution_count += 1;
        
        if let Some(avg) = self.avg_execution_time {
            // Calculate running average
            let total = avg.as_nanos() * (self.execution_count - 1) as u128;
            let new_avg = (total + execution_time.as_nanos()) / self.execution_count as u128;
            self.avg_execution_time = Some(Duration::from_nanos(new_avg as u64));
        } else {
            self.avg_execution_time = Some(execution_time);
        }
    }
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            timeout: Some(Duration::from_secs(30)),
            max_rows: None,
            fetch_size: Some(1000),
            use_prepared: false,
            cache_plan: true,
            optimize: true,
            hints: HashMap::new(),
        }
    }
}

impl QueryCache {
    /// slay Create a new query cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: HashMap::new(),
            config,
            stats: CacheStats::default(),
        }
    }

    /// slay Get query from cache
    pub fn get(&mut self, query_hash: u64) -> Option<&CachedQuery> {
        let key = query_hash.to_string();
        
        if let Some(cached) = self.cache.get_mut(&key) {
            // Update access metadata
            cached.metadata.last_accessed = SystemTime::now();
            cached.metadata.access_count += 1;
            
            self.stats.hits += 1;
            Some(cached)
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// slay Put query in cache
    pub fn put(&mut self, query: Query, plan: Option<QueryPlan>) {
        if !self.config.enabled || !query.metadata.is_cacheable {
            return;
        }

        let key = query.get_hash().to_string();
        let size = self.estimate_size(&query);
        
        // Check if we need to evict entries
        self.maybe_evict(size);
        
        let cached_query = CachedQuery {
            query,
            plan,
            metadata: CacheEntryMetadata {
                created_at: SystemTime::now(),
                last_accessed: SystemTime::now(),
                access_count: 0,
                size_bytes: size,
                is_pinned: false,
            },
        };
        
        self.cache.insert(key, cached_query);
        self.stats.current_entries += 1;
        self.stats.current_size += size;
    }

    /// slay Estimate size of query in cache
    fn estimate_size(&self, query: &Query) -> usize {
        query.statement.len() + 
        query.parameters.parameters.len() * 64 + // Rough estimate
        256 // Metadata overhead
    }

    /// slay Maybe evict entries based on policy
    fn maybe_evict(&mut self, needed_size: usize) {
        if self.stats.current_entries >= self.config.max_entries ||
           self.stats.current_size + needed_size > self.config.max_size_bytes {
            
            match self.config.eviction_policy {
                EvictionPolicy::LRU => self.evict_lru(),
                EvictionPolicy::LFU => self.evict_lfu(),
                EvictionPolicy::FIFO => self.evict_fifo(),
                EvictionPolicy::TTL => self.evict_expired(),
            }
        }
    }

    /// slay Evict least recently used entry
    fn evict_lru(&mut self) {
        let mut oldest_key = None;
        let mut oldest_time = SystemTime::now();
        
        for (key, cached) in &self.cache {
            if !cached.metadata.is_pinned && cached.metadata.last_accessed < oldest_time {
                oldest_time = cached.metadata.last_accessed;
                oldest_key = Some(key.clone());
            }
        }
        
        if let Some(key) = oldest_key {
            if let Some(cached) = self.cache.remove(&key) {
                self.stats.evictions += 1;
                self.stats.current_entries -= 1;
                self.stats.current_size -= cached.metadata.size_bytes;
            }
        }
    }

    /// slay Evict least frequently used entry
    fn evict_lfu(&mut self) {
        let mut lfu_key = None;
        let mut min_count = u64::MAX;
        
        for (key, cached) in &self.cache {
            if !cached.metadata.is_pinned && cached.metadata.access_count < min_count {
                min_count = cached.metadata.access_count;
                lfu_key = Some(key.clone());
            }
        }
        
        if let Some(key) = lfu_key {
            if let Some(cached) = self.cache.remove(&key) {
                self.stats.evictions += 1;
                self.stats.current_entries -= 1;
                self.stats.current_size -= cached.metadata.size_bytes;
            }
        }
    }

    /// slay Evict first in, first out
    fn evict_fifo(&mut self) {
        let mut oldest_key = None;
        let mut oldest_time = SystemTime::now();
        
        for (key, cached) in &self.cache {
            if !cached.metadata.is_pinned && cached.metadata.created_at < oldest_time {
                oldest_time = cached.metadata.created_at;
                oldest_key = Some(key.clone());
            }
        }
        
        if let Some(key) = oldest_key {
            if let Some(cached) = self.cache.remove(&key) {
                self.stats.evictions += 1;
                self.stats.current_entries -= 1;
                self.stats.current_size -= cached.metadata.size_bytes;
            }
        }
    }

    /// slay Evict expired entries
    fn evict_expired(&mut self) {
        let now = SystemTime::now();
        let mut expired_keys = Vec::new();
        
        for (key, cached) in &self.cache {
            if !cached.metadata.is_pinned {
                if let Ok(age) = now.duration_since(cached.metadata.created_at) {
                    if age > self.config.ttl {
                        expired_keys.push(key.clone());
                    }
                }
            }
        }
        
        for key in expired_keys {
            if let Some(cached) = self.cache.remove(&key) {
                self.stats.evictions += 1;
                self.stats.current_entries -= 1;
                self.stats.current_size -= cached.metadata.size_bytes;
            }
        }
    }

    /// slay Clear all cache entries
    pub fn clear(&mut self) {
        self.cache.clear();
        self.stats.current_entries = 0;
        self.stats.current_size = 0;
    }

    /// slay Get cache hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let total = self.stats.hits + self.stats.misses;
        if total > 0 {
            self.stats.hits as f64 / total as f64
        } else {
            0.0
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            max_size_bytes: 10 * 1024 * 1024, // 10MB
            ttl: Duration::from_secs(3600), // 1 hour
            enabled: true,
            eviction_policy: EvictionPolicy::LRU,
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ParameterSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_creation() {
        let query = Query::new("SELECT * FROM users WHERE id = ?");
        assert_eq!(query.query_type, QueryType::Select);
        assert!(!query.id.is_empty());
    }

    #[test]
    fn test_query_type_detection() {
        assert_eq!(Query::detect_query_type("SELECT * FROM users"), QueryType::Select);
        assert_eq!(Query::detect_query_type("INSERT INTO users"), QueryType::Insert);
        assert_eq!(Query::detect_query_type("UPDATE users SET"), QueryType::Update);
        assert_eq!(Query::detect_query_type("DELETE FROM users"), QueryType::Delete);
        assert_eq!(Query::detect_query_type("CREATE TABLE users"), QueryType::Create);
    }

    #[test]
    fn test_parameter_handling() {
        let mut param_set = ParameterSet::new();
        param_set.add_parameter(Parameter::named_input("id", "123"));
        param_set.add_parameter(Parameter::named_input("name", "Alice"));

        assert_eq!(param_set.len(), 2);
        assert!(param_set.get_by_name("id").is_some());
        assert!(param_set.get_by_name("name").is_some());
        assert!(param_set.get_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .statement("SELECT * FROM users WHERE id = :id")
            .parameter("id", "123")
            .timeout(Duration::from_secs(30))
            .max_rows(100)
            .use_prepared()
            .build()
            .unwrap();

        assert_eq!(query.query_type, QueryType::Select);
        assert_eq!(query.parameters.len(), 1);
        assert_eq!(query.options.timeout, Some(Duration::from_secs(30)));
        assert_eq!(query.options.max_rows, Some(100));
        assert!(query.options.use_prepared);
    }

    #[test]
    fn test_query_validation() {
        let empty_query = Query::new("");
        assert!(empty_query.validate().is_err());

        let valid_query = Query::new("SELECT 1");
        assert!(valid_query.validate().is_ok());

        let suspicious_query = Query::new("SELECT * FROM users -- DROP TABLE users");
        assert!(suspicious_query.validate().is_err());
    }

    #[test]
    fn test_query_cache() {
        let config = CacheConfig::default();
        let mut cache = QueryCache::new(config);

        let query = Query::new("SELECT * FROM users");
        let query_hash = query.get_hash();
        
        // Miss first time
        assert!(cache.get(query_hash).is_none());
        assert_eq!(cache.stats.misses, 1);

        // Put in cache
        cache.put(query, None);
        assert_eq!(cache.stats.current_entries, 1);

        // Hit second time
        assert!(cache.get(query_hash).is_some());
        assert_eq!(cache.stats.hits, 1);
    }

    #[test]
    fn test_query_metadata() {
        let mut metadata = QueryMetadata::new(true);
        assert_eq!(metadata.execution_count, 0);
        assert!(metadata.avg_execution_time.is_none());

        metadata.record_execution(Duration::from_millis(100));
        assert_eq!(metadata.execution_count, 1);
        assert_eq!(metadata.avg_execution_time, Some(Duration::from_millis(100)));

        metadata.record_execution(Duration::from_millis(200));
        assert_eq!(metadata.execution_count, 2);
        // Average should be 150ms
        assert!(metadata.avg_execution_time.unwrap().as_millis() == 150);
    }

    #[test]
    fn test_parameter_directions() {
        let input = Parameter::input("value");
        let output = Parameter::output("varchar");
        let named_input = Parameter::named_input("param", "value");
        let named_output = Parameter::named_output("result", "int");

        assert_eq!(input.direction, ParameterDirection::In);
        assert_eq!(output.direction, ParameterDirection::Out);
        assert_eq!(named_input.direction, ParameterDirection::In);
        assert_eq!(named_output.direction, ParameterDirection::Out);
    }
}
