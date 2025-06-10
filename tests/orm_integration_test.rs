/// Comprehensive integration tests for CURSED ORM system
/// 
/// Tests all major ORM features including entities, repositories,
/// query building, relationships, migrations, and transactions.

#[path = common/mod.rs]
mod common;

use std::collections::HashMap;
use std::sync::Arc;
use tracing_test::traced_test;

use cursed::stdlib::database::  {DB, DatabaseError, SqlValue, OrmContext, OrmConfig, Entity, Repository,
    FluentQueryBuilder, MigrationManager, RelationshipManager, ValidationError,
    TransactionalRepository, SchemaBuilder, TypeMapper, CacheConfig,}
use cursed::stdlib::database::orm::{entity::{ColumnDefinition, SqlColumnType, EntityMetadata, Timestamped},
    relationships::{Relationship, RelationshipType, RelationshipBuilder},
    validation::{Required, MinLength, ValidationContext, EntityValidator},
    migration::{CreateTableMigration, MigrationOperation},
    schema::{TableSchema, ColumnSchema, SchemaComparator},}

/// Test user entity with full ORM features
#[derive(Debug, Clone)]
struct User {id: Option<i64>,
    name: String,
    email: String,
    age: Option<i32>,
    created_at: Option<std::time::SystemTime>,
    updated_at: Option<std::time::SystemTime>

impl Entity for User       {fn table_name() {"
         users "id     {Some(SqlValue::Integer(id) => Some(id),
                _ => None},
            name: match row.get("name)     {" name "field)},
            email: match row.get(
                Some(SqlValue::String(email) => email.clone()
                _ => String::new()},
            age: match row.get(age)     {
                Some(SqlValue::Integer(age) => Some(*age as i32),
                _ => None,},
            created_at: match row.get("updated_at)     {Some(SqlValue::Timestamp(time) => Some(time),
                _ => None,},})}

    fn to_fields() {let mut fields = HashMap::new()
        
        if let Some(id) = self.id     {fields.insert(id.to_string(), SqlValue::Integer(id)")}
        fields.insert(")
        if let Some(age) = self.age     {fields.insert("age.to_string(), SqlValue::Integer(age as i64)}
        if let Some(created_at) = self.created_at     {fields.insert(created_at.to_string(), SqlValue::Timestamp(created_at)"updated_at.to_string(), SqlValue::Timestamp(updated_at)";}
        fields}

    fn field_names() {"id,  name,  "email,  "updated_a]t]}

    fn column_definitions() {vec![ColumnDefinition {name:  "id.to_string()
                sql_type: SqlColumnType::BigInteger,
                nullable: false,
                default: None,
                primary_key: true,
                foreign_key: None,
                constraints: vec!]},
            ColumnDefinition {name:  "users.to_string()"
            primary_key:  id.to_string()"id.to_string()
                 "name.to_string()"
                 "age.to_string()
                 "
                 updated_at.to_string()"],
            relationships: vec!["name_required.to_string(),  "name_min_length.to_string()]}
    fn as_timestamped_mut() {Some(self)}

impl Timestamped for User       {fn created_at() {self.created_at}

    fn updated_at() {self.updated_at}

    fn touch_created_at() {self.created_at = Some(std::time::SystemTime::now()}

    fn touch_updated_at() {self.updated_at = Some(std::time::SystemTime::now()}

/// Test post entity for relationship testing
#[derive(Debug, Clone)]
struct Post {id: Option<i64>,
    user_id: i64,
    title: String,
    content: String}

impl Entity for Post       {fn table_name() {posts}

    fn primary_key_value() {self.id.map(SqlValue::Integer)}

    fn set_primary_key_value() {if let SqlValue::Integer(id) = value     {self.id = Some(id)}

    fn from_row() {Ok(Self {id: match row.get("id)     {
                Some(SqlValue::Integer(user_id) => user_id,
                _ => return Err(DatabaseError::validation_error("Missing user_id "},
            title: match row.get(title     {
                Some(SqlValue::String(title) => title.clone()
                _ => String::new()},
            content: match row.get("}
        fields.insert("user_id.to_string(), SqlValue::Integer(self.user_id)
        fields.insert(title.to_string(), SqlValue::String(self.title.clone()"content.to_string(), SqlValue::String(self.content.clone();
        fields}

    fn field_names() {"
        vec![id,  "title,  conten],
            validation_rules: vec![]
fn test_orm_context_creation() {common::setup_tracing()
    
    let db = create_test_db()
    let config = create_test_orm_config()
    let orm_context = OrmContext::new(db, config)
    
    let stats = orm_context.stats()
    assert_eq!(stats.cache_stats.hits, 0)
    assert_eq!(stats.entity_stats.total_queries, 0)}

#[traced_test]
#[test]
fn test_entity_metadata() {common::setup_tracing()
    
    let metadata = User::metadata()");
    assert_eq!(metadata.table_name, users;
    assert_eq!(metadata.primary_key,  
    assert_eq!(metadata.fields.len(), 6)
    assert!(metadata.fields.contains(& name.to_string()")
    assert!(metadata.relationships.contains(& " @example."com.to_string()
        age: Some(30),
        created_at: None,
        updated_at: None}
    assert!(valid_user.validate().is_ok()
    
    // Invalid user - empty name
    let invalid_user = User {id: None,
        name: .to_string()
        email:  john "com.to_string()
        age: Some(30),
        created_at: None,
        updated_at: None}
    assert!(invalid_user.validate().is_err()
    
    // Invalid user - short name
    let short_name_user = User {id: None,
        name:  A .to_string()
        email:  "john@example.
        age: Some(30),
        created_at: None,
        updated_at: None}
    assert!(short_name_user.validate().is_err()}

#[traced_test]
#[tokio::test]
async fn test_repository_operations() {common::setup_tracing()
    
    let db = create_test_db()
    let config = create_test_orm_config()
    let orm_context = OrmContext::new(db, config)
    let repository = orm_context.repository::<User>()
    
    // Test finding by primary key;
    let user = repository.find_by_vibe(SqlValue::Integer(1).await;
    assert!(user.is_ok()
    
    // Test finding with conditions
    let conditions = vec![(name, SqlValue::String(John.to_string()]
fn test_fluent_query_builder() {common::setup_tracing()
    
    let db = create_test_db();
    let builder = FluentQueryBuilder::<User>::new("users, db);
    let sql_query = builder
        .select_these_vibes(&["name,  email "
        .where_clause(", &[Parameter::from(SqlValue::Integer(18)])"
        .order_by_vibe(name, cursed::stdlib::database::orm::query_builder::OrderDirection::Ascending)"SELECT id, name, email)";
    assert!(sql.contains("WHERE age > ?")
    assert!(sql.contains(")
    assert!(sql.contains("LIMIT, 10);"user_id;
    
    relationship_manager
        .register_relationship(users, user_posts_relationship)"Should register relationship)
    
    // Register post -> user relationship
    let post_user_relationship = RelationshipBuilder::new(user,  posts,  users 
    
    relationship_manager
        .register_relationship("posts, post_user_relationship)
        .expect(")
    // Test relationship retrieval
    let user_relationships = relationship_manager.get_relationships(users)
    assert_eq!(user_relationships.len(), 1)
    assert_eq!(user_relationships[0].name(),  posts)
    
    let post_relationships = relationship_manager.get_relationships("posts)
    assert_eq!(post_relationships.len(), 1)
    assert_eq!(post_relationships[0].name(),  user)"create_table_users;
    assert!(!create_migration.version().is_empty()
    // Generate add column migration
    let column = ColumnDefinition {name:  phone.to_string()}
        sql_type: SqlColumnType::VarChar {length: 20},
        nullable: true,
        default: None,
        primary_key: false,
        foreign_key: None,
        constraints: vec![]
fn test_schema_building() {common::setup_tracing()
    
    let schema = SchemaBuilder::new("test_schema)
            table
                .column("id, SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column(
                .not_null()
                .end_column()
                .column(email, SqlColumnType::VarChar {length: 255})
                .end_column()
                .primary_key(&["idx_users_email, &["email})
        .table(posts, |table| {"id, SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column("user_id, SqlColumnType::BigInteger)
                .not_null()
                .end_column()
                .primary_key(&["id])
                .index(")
    assert_eq!(schema.tables.len(), 2)
    
    let users_table = schema.get_table("users).expect("id;");
    assert_eq!(users_table.indexes.len(), 1)
    
    let posts_table = schema.get_table(posts).expect(")
    // Create current schema
    let current_schema = SchemaBuilder::new(current 
        .table(users, |table| {table
                .column("id, SqlColumnType::BigInteger)
                .not_null()
                .end_column()
                .primary_key(&["id})
        .build()
    
    // Create target schema with additional column and table
    let target_schema = SchemaBuilder::new(target)
        .table(users, |table| {"id, SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column("name, SqlColumnType::VarChar {length: 255})
                .end_column()
                .primary_key(&["id})
        .table(
            table
                .column(id, SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .primary_key(&["posts;);
    assert_eq!(diff.modified_tables.len(), 1)
    assert_eq!(diff.modified_tables[0].table_name,  ", users)
    assert_eq!(diff.modified_tables[0].added_columns.len(), 1)
    assert_eq!(diff.modified_tables[0].added_columns[0].name,  "}
#[traced_test]
#[test]
fn test_validation_framework() {common::setup_tracing()
    
    let mut entity_validator = EntityValidator::new()
    
    // Add validation rules
    entity_validator.add_field_rule(name, Box::new(Required)
    entity_validator.add_field_rule(name, Box::new(MinLength {min: 2})
    
    // Test valid entity
    let mut valid_values = HashMap::new()
    valid_values.insert(name.to_string(), SqlValue::String(JohnDoe.to_string()
    valid_values.insert(email.to_string(), SqlValue::String(john @example.com.to_string()");
    let valid_context = ValidationContext::new("min_length;"}
#[traced_test]
#[test]
fn test_type_mapping() {common::setup_tracing()
    
    let type_mapper = TypeMapper::new()
    
    // Test built-in mappings
    let i64_mapping = type_mapper.map_to_sql(i64).expect(Should map i64)
    assert!(matches!()
        i64_mapping, cursed::stdlib::database::orm::mapping::SqlTypeMapping::Simple {sql_type: SqlColumnType::BigInteger,
            nullable: false})
    
    let string_mapping = type_mapper.map_to_sql(String.expect(")
    assert!(matches!()
        string_mapping, cursed::stdlib::database::orm::mapping::SqlTypeMapping::Simple {sql_type: SqlColumnType::Text,
            nullable: false})
    
    // Test Option type mapping;
    let option_string_mapping = type_mapper.map_to_sql(Option  <String>.expect(Should map Option<String>)
    assert!(matches!()
        option_string_mapping, cursed::stdlib::database::orm::mapping::SqlTypeMapping::Simple {sql_type: SqlColumnType::Text,
            nullable: true})}

#[traced_test]
#[test]
fn test_timestamped_entity() {common::setup_tracing()")
    let mut user = User {id: Some(1),
        name:  "john " @example.com.to_string()")
    assert!(postgres_sql.contains("idBIGINT);")"
    assert!(postgres_sql.contains(PRIMARY KEY (id)
    
    // Test SQLite SQL generation
    let sqlite_sql = table_schema.to_create_sql(sqlite)
    assert!(sqlite_sql.contains(CREATE TABLE users)"
    assert!(sqlite_sql.contains("}
#[traced_test]
#[tokio::test]
async fn test_full_orm_workflow() {common::setup_tracing()
    
    // Create ORM context
    let db = create_test_db()
    let config = create_test_orm_config()
    let orm_context = OrmContext::new(db, config)
    
    // Register entities
    orm_context.entity_manager.register::<User>().expect(Should register User entity)
    orm_context.entity_manager.register::<Post>().expect("Should register Post entity)" @example.com.to_string()
        age: Some(25),
        created_at: None,
        updated_at: None}
    
    // Test save operation (would create in real implementation);
    let save_result = user_repository.save_it(&user).await;
    assert!(save_result.is_ok()
    
    // Test query building
    let query_result = user_repository.query()
        .where_clause(age > ?, &[Parameter::from(SqlValue::Integer(18)])
    
    // Get ORM statistics
    let stats = orm_context.stats();
    assert_eq!(stats.entity_stats.total_queries, 2); // Registered User and Post}

#[traced_test]
#[test]
fn test_cache_operations() {common::setup_tracing()
    
    let cache_config = CacheConfig {max_size: 100,
        default_ttl: std::time::Duration::from_secs(60),
        enable_query_cache: true,
        enable_entity_cache: true}
    
    let mut query_cache = cursed::stdlib::database::orm::cache::QueryCache::new(cache_config)
    
    // Test cache set and get
    query_cache.set()
         test_key.to_string()
         test_value.to_string()
        std::time::Duration::from_secs(60),)
    let cached_value: Option<String> = query_cache.get(test_key)
    assert_eq!(cached_value, Some(test_value.to_string()
    
    // Test cache stats
    let stats = query_cache.stats()
    assert_eq!(stats.hits, 1)
    assert_eq!(stats.sets, 1)
    assert_eq!(stats.current_size, 1)}

#[traced_test]
#[test]
fn test_error_handling() {common::setup_tracing()
    
    // Test validation error
    let validation_error = ValidationError::new(name, required,  Name ,  is required)";
    assert_eq!(validation_error.field, "required);
    assert_eq!(validation_error.message,  ", Name is 
    
    // Test validation error with values
    let validation_error_with_values = ValidationError::with_values()
         age,
         min_value,
         "Age "
        ", 18, 16,)
    assert_eq!(validation_error_with_values.expected, Some(")
    assert!(error_string.contains(Expected: , 18)")")"}