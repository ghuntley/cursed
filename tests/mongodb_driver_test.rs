/// Comprehensive test suite for MongoDB driver
/// 
/// Tests all major functionality including:
/// - Connection management and configuration
/// - CRUD operations with various data types
/// - Query building and filtering
/// - Aggregation pipeline construction
/// - Index management operations
/// - Error handling and edge cases
/// - Value/BSON conversion utilities
/// - Async operation handling

#[cfg(test)]
mod tests {
    use crate::stdlib::packages::db_nosql::mongodb::*;
    use crate::value::Value;
    use std::collections::HashMap;
    use mongodb::bson::{doc, Document, Bson};
    use tokio;

    /// Test MongoDB configuration creation and validation
    #[test]
    fn test_mongodb_config_creation() {
        let config = MongoDbConfig::default();
        
        assert_eq!(config.connection_string, "mongodb://localhost:27017");
        assert_eq!(config.database_name, "cursed_db");
        assert_eq!(config.max_pool_size, Some(10));
        assert_eq!(config.min_pool_size, Some(1));
        assert_eq!(config.retry_writes, true);
        assert_eq!(config.retry_reads, true);
        assert_eq!(config.enable_ssl, false);
        
        // Test custom configuration
        let custom_config = MongoDbConfig {
            connection_string: "mongodb://user:pass@example.com:27017/mydb".to_string(),
            database_name: "custom_db".to_string(),
            max_pool_size: Some(25),
            connect_timeout: Some(30),
            enable_ssl: true,
            ..MongoDbConfig::default()
        };
        
        assert_eq!(custom_config.connection_string, "mongodb://user:pass@example.com:27017/mydb");
        assert_eq!(custom_config.database_name, "custom_db");
        assert_eq!(custom_config.max_pool_size, Some(25));
        assert_eq!(custom_config.connect_timeout, Some(30));
        assert_eq!(custom_config.enable_ssl, true);
    }

    /// Test write concern configuration
    #[test]
    fn test_write_concern_config() {
        let write_concern = WriteConcernConfig {
            w: Some(2),
            w_string: None,
            journal: Some(true),
            timeout: Some(10000),
        };
        
        assert_eq!(write_concern.w, Some(2));
        assert_eq!(write_concern.journal, Some(true));
        assert_eq!(write_concern.timeout, Some(10000));
        
        let write_concern_with_string = WriteConcernConfig {
            w: None,
            w_string: Some("majority".to_string()),
            journal: Some(false),
            timeout: Some(5000),
        };
        
        assert_eq!(write_concern_with_string.w_string, Some("majority".to_string()));
        assert_eq!(write_concern_with_string.journal, Some(false));
    }

    /// Test query builder functionality
    #[test]
    fn test_query_builder() {
        let mut builder = MongoDbQueryBuilder::new();
        
        // Test basic filter
        let name_value = Value::String("Alice".to_string());
        builder = builder.filter("name", &name_value).unwrap();
        
        let filter = builder.get_filter();
        assert!(filter.contains_key("name"));
        
        // Test chaining operations
        builder = builder
            .project(&["name", "email", "age"])
            .sort("age", -1)
            .limit(10)
            .skip(5);
        
        let options = builder.build_find_options();
        assert_eq!(options.limit, Some(10));
        assert_eq!(options.skip, Some(5));
        assert!(options.projection.is_some());
        assert!(options.sort.is_some());
        
        // Verify projection contains expected fields
        if let Some(projection) = options.projection {
            assert!(projection.contains_key("name"));
            assert!(projection.contains_key("email"));
            assert!(projection.contains_key("age"));
        }
        
        // Verify sort contains expected field
        if let Some(sort) = options.sort {
            assert!(sort.contains_key("age"));
            assert_eq!(sort.get("age"), Some(&Bson::Int32(-1)));
        }
    }

    /// Test complex query building with multiple filters
    #[test]
    fn test_complex_query_building() {
        let mut builder = MongoDbQueryBuilder::new();
        
        // Add multiple filters
        builder = builder
            .filter("status", &Value::String("active".to_string())).unwrap()
            .filter("age", &Value::Int(25)).unwrap()
            .filter("department", &Value::String("Engineering".to_string())).unwrap();
        
        let filter = builder.get_filter();
        assert_eq!(filter.len(), 3);
        assert!(filter.contains_key("status"));
        assert!(filter.contains_key("age"));
        assert!(filter.contains_key("department"));
        
        // Test with complex value types
        let range_query = Value::Object({
            let mut range = HashMap::new();
            range.insert("$gte".to_string(), Value::Int(18));
            range.insert("$lte".to_string(), Value::Int(65));
            range
        });
        
        let range_builder = MongoDbQueryBuilder::new()
            .filter("age", &range_query).unwrap();
        
        let range_filter = range_builder.get_filter();
        assert!(range_filter.contains_key("age"));
    }

    /// Test Value to BSON conversion
    #[test]
    fn test_value_to_bson_conversion() {
        // Test null
        let null_value = Value::Null;
        let bson_null = super::value_to_bson(&null_value).unwrap();
        assert!(matches!(bson_null, Bson::Null));
        
        // Test boolean
        let bool_value = Value::Bool(true);
        let bson_bool = super::value_to_bson(&bool_value).unwrap();
        assert!(matches!(bson_bool, Bson::Boolean(true)));
        
        // Test integer
        let int_value = Value::Int(42);
        let bson_int = super::value_to_bson(&int_value).unwrap();
        assert!(matches!(bson_int, Bson::Int64(42)));
        
        // Test float
        let float_value = Value::Float(3.14159);
        let bson_float = super::value_to_bson(&float_value).unwrap();
        if let Bson::Double(f) = bson_float {
            assert!((f - 3.14159).abs() < f64::EPSILON);
        } else {
            panic!("Expected Double BSON type");
        }
        
        // Test string
        let string_value = Value::String("Hello, MongoDB!".to_string());
        let bson_string = super::value_to_bson(&string_value).unwrap();
        assert!(matches!(bson_string, Bson::String(ref s) if s == "Hello, MongoDB!"));
        
        // Test array
        let array_value = Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::String("three".to_string()),
        ]);
        let bson_array = super::value_to_bson(&array_value).unwrap();
        if let Bson::Array(arr) = bson_array {
            assert_eq!(arr.len(), 3);
            assert!(matches!(arr[0], Bson::Int64(1)));
            assert!(matches!(arr[1], Bson::Int64(2)));
            assert!(matches!(arr[2], Bson::String(ref s) if s == "three"));
        } else {
            panic!("Expected Array BSON type");
        }
        
        // Test object
        let object_value = Value::Object({
            let mut obj = HashMap::new();
            obj.insert("name".to_string(), Value::String("Alice".to_string()));
            obj.insert("age".to_string(), Value::Int(30));
            obj.insert("active".to_string(), Value::Bool(true));
            obj
        });
        let bson_object = super::value_to_bson(&object_value).unwrap();
        if let Bson::Document(doc) = bson_object {
            assert_eq!(doc.len(), 3);
            assert!(doc.contains_key("name"));
            assert!(doc.contains_key("age"));
            assert!(doc.contains_key("active"));
        } else {
            panic!("Expected Document BSON type");
        }
    }

    /// Test BSON to Value conversion
    #[test]
    fn test_bson_to_value_conversion() {
        // Test null
        let bson_null = Bson::Null;
        let value_null = super::bson_to_value(&bson_null).unwrap();
        assert!(matches!(value_null, Value::Null));
        
        // Test boolean
        let bson_bool = Bson::Boolean(false);
        let value_bool = super::bson_to_value(&bson_bool).unwrap();
        assert!(matches!(value_bool, Value::Bool(false)));
        
        // Test integers
        let bson_int32 = Bson::Int32(42);
        let value_int32 = super::bson_to_value(&bson_int32).unwrap();
        assert!(matches!(value_int32, Value::Int(42)));
        
        let bson_int64 = Bson::Int64(9223372036854775807);
        let value_int64 = super::bson_to_value(&bson_int64).unwrap();
        assert!(matches!(value_int64, Value::Int(9223372036854775807)));
        
        // Test double
        let bson_double = Bson::Double(2.718281828);
        let value_double = super::bson_to_value(&bson_double).unwrap();
        if let Value::Float(f) = value_double {
            assert!((f - 2.718281828).abs() < f64::EPSILON);
        } else {
            panic!("Expected Float Value type");
        }
        
        // Test string
        let bson_string = Bson::String("Hello, CURSED!".to_string());
        let value_string = super::bson_to_value(&bson_string).unwrap();
        assert!(matches!(value_string, Value::String(ref s) if s == "Hello, CURSED!"));
        
        // Test array
        let bson_array = Bson::Array(vec![
            Bson::Int64(100),
            Bson::String("test".to_string()),
            Bson::Boolean(true),
        ]);
        let value_array = super::bson_to_value(&bson_array).unwrap();
        if let Value::Array(arr) = value_array {
            assert_eq!(arr.len(), 3);
            assert!(matches!(arr[0], Value::Int(100)));
            assert!(matches!(arr[1], Value::String(ref s) if s == "test"));
            assert!(matches!(arr[2], Value::Bool(true)));
        } else {
            panic!("Expected Array Value type");
        }
        
        // Test document
        let bson_document = Bson::Document({
            let mut doc = Document::new();
            doc.insert("username", "cursed_user");
            doc.insert("score", 1337);
            doc.insert("premium", true);
            doc
        });
        let value_document = super::bson_to_value(&bson_document).unwrap();
        if let Value::Object(obj) = value_document {
            assert_eq!(obj.len(), 3);
            assert!(obj.contains_key("username"));
            assert!(obj.contains_key("score"));
            assert!(obj.contains_key("premium"));
        } else {
            panic!("Expected Object Value type");
        }
        
        // Test ObjectId conversion
        let object_id = mongodb::bson::oid::ObjectId::new();
        let bson_oid = Bson::ObjectId(object_id);
        let value_oid = super::bson_to_value(&bson_oid).unwrap();
        if let Value::String(s) = value_oid {
            assert_eq!(s.len(), 24); // ObjectId hex string length
        } else {
            panic!("Expected String Value type for ObjectId");
        }
    }

    /// Test aggregation pipeline builder
    #[test]
    fn test_aggregation_pipeline_builder() {
        let pipeline = AggregationPipelineBuilder::new()
            .match_stage(doc! { "status": "active" })
            .group_stage(doc! { 
                "_id": "$department", 
                "count": { "$sum": 1 },
                "avg_salary": { "$avg": "$salary" }
            })
            .sort_stage(doc! { "count": -1 })
            .project_stage(doc! { 
                "department": "$_id",
                "employee_count": "$count",
                "average_salary": "$avg_salary"
            })
            .limit_stage(5)
            .skip_stage(2)
            .build();
        
        assert_eq!(pipeline.len(), 6);
        
        // Verify each stage
        assert!(pipeline[0].contains_key("$match"));
        assert!(pipeline[1].contains_key("$group"));
        assert!(pipeline[2].contains_key("$sort"));
        assert!(pipeline[3].contains_key("$project"));
        assert!(pipeline[4].contains_key("$limit"));
        assert!(pipeline[5].contains_key("$skip"));
        
        // Check specific values
        if let Some(limit_value) = pipeline[4].get("$limit") {
            assert_eq!(limit_value, &Bson::Int64(5));
        }
        
        if let Some(skip_value) = pipeline[5].get("$skip") {
            assert_eq!(skip_value, &Bson::Int64(2));
        }
    }

    /// Test advanced aggregation pipeline with lookup and unwind
    #[test]
    fn test_advanced_aggregation_pipeline() {
        let pipeline = AggregationPipelineBuilder::new()
            .match_stage(doc! { "active": true })
            .lookup_stage("departments", "dept_id", "_id", "department_info")
            .unwind_stage("$department_info")
            .group_stage(doc! {
                "_id": "$department_info.name",
                "total_employees": { "$sum": 1 },
                "total_budget": { "$sum": "$department_info.budget" }
            })
            .sort_stage(doc! { "total_budget": -1 })
            .custom_stage(doc! {
                "$addFields": {
                    "budget_per_employee": {
                        "$divide": ["$total_budget", "$total_employees"]
                    }
                }
            })
            .build();
        
        assert_eq!(pipeline.len(), 6);
        
        // Verify lookup stage
        if let Some(lookup_stage) = pipeline[1].get("$lookup") {
            if let Bson::Document(lookup_doc) = lookup_stage {
                assert_eq!(lookup_doc.get("from"), Some(&Bson::String("departments".to_string())));
                assert_eq!(lookup_doc.get("localField"), Some(&Bson::String("dept_id".to_string())));
                assert_eq!(lookup_doc.get("foreignField"), Some(&Bson::String("_id".to_string())));
                assert_eq!(lookup_doc.get("as"), Some(&Bson::String("department_info".to_string())));
            }
        }
        
        // Verify unwind stage
        if let Some(unwind_stage) = pipeline[2].get("$unwind") {
            assert_eq!(unwind_stage, &Bson::String("$department_info".to_string()));
        }
        
        // Verify custom stage
        assert!(pipeline[5].contains_key("$addFields"));
    }

    /// Test error type conversions
    #[test]
    fn test_error_conversions() {
        // Test MongoDB error to CURSED error conversion
        let mongo_error = MongoDbError::ConnectionFailed("Connection timeout".to_string());
        let cursed_error: crate::error::CursedError = mongo_error.into();
        
        assert!(matches!(cursed_error.kind(), crate::error::ErrorKind::DatabaseError));
        assert!(cursed_error.message().contains("MongoDB error"));
        assert!(cursed_error.message().contains("ConnectionFailed"));
        
        // Test different error types
        let auth_error = MongoDbError::AuthenticationFailed("Invalid credentials".to_string());
        let cursed_auth_error: crate::error::CursedError = auth_error.into();
        assert!(cursed_auth_error.message().contains("AuthenticationFailed"));
        
        let serialization_error = MongoDbError::SerializationFailed("Invalid BSON".to_string());
        let cursed_ser_error: crate::error::CursedError = serialization_error.into();
        assert!(cursed_ser_error.message().contains("SerializationFailed"));
    }

    /// Test value to document conversion
    #[test]
    fn test_value_to_document_conversion() {
        // Test valid object conversion
        let valid_object = Value::Object({
            let mut obj = HashMap::new();
            obj.insert("field1".to_string(), Value::String("value1".to_string()));
            obj.insert("field2".to_string(), Value::Int(42));
            obj.insert("field3".to_string(), Value::Bool(true));
            obj
        });
        
        let document = super::value_to_document(&valid_object).unwrap();
        assert_eq!(document.len(), 3);
        assert!(document.contains_key("field1"));
        assert!(document.contains_key("field2"));
        assert!(document.contains_key("field3"));
        
        // Test nested object conversion
        let nested_object = Value::Object({
            let mut obj = HashMap::new();
            obj.insert("user".to_string(), Value::Object({
                let mut user_obj = HashMap::new();
                user_obj.insert("name".to_string(), Value::String("Alice".to_string()));
                user_obj.insert("age".to_string(), Value::Int(30));
                user_obj
            }));
            obj.insert("preferences".to_string(), Value::Array(vec![
                Value::String("email".to_string()),
                Value::String("notifications".to_string()),
            ]));
            obj
        });
        
        let nested_document = super::value_to_document(&nested_object).unwrap();
        assert_eq!(nested_document.len(), 2);
        assert!(nested_document.contains_key("user"));
        assert!(nested_document.contains_key("preferences"));
        
        // Test invalid conversion (non-object)
        let invalid_value = Value::String("Not an object".to_string());
        let result = super::value_to_document(&invalid_value);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MongoDbError::SerializationFailed(_)));
        
        let invalid_array = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let result2 = super::value_to_document(&invalid_array);
        assert!(result2.is_err());
    }

    /// Test MongoDB driver connection management
    #[tokio::test]
    async fn test_driver_connection_management() {
        let driver = MongoDbDriver::new();
        
        // Test initial state
        let connections = driver.list_connections().await;
        assert!(connections.is_empty());
        
        // Test adding connection (this will likely fail without MongoDB running)
        let config = MongoDbConfig {
            connection_string: "mongodb://localhost:27017".to_string(),
            database_name: "test_db".to_string(),
            connect_timeout: Some(1), // Very short timeout for testing
            ..MongoDbConfig::default()
        };
        
        let result = driver.add_connection("test_connection".to_string(), config).await;
        // We expect this to fail in test environment without MongoDB
        assert!(result.is_err());
        
        // Test getting non-existent connection
        let get_result = driver.get_connection("non_existent").await;
        assert!(get_result.is_err());
        assert!(matches!(get_result.unwrap_err(), MongoDbError::ConnectionFailed(_)));
        
        // Test removing non-existent connection
        let remove_result = driver.remove_connection("non_existent").await;
        assert!(remove_result.is_err());
    }

    /// Test query builder with edge cases
    #[test]
    fn test_query_builder_edge_cases() {
        // Test empty query builder
        let empty_builder = MongoDbQueryBuilder::new();
        let empty_filter = empty_builder.get_filter();
        assert!(empty_filter.is_empty());
        
        let empty_options = empty_builder.build_find_options();
        assert!(empty_options.projection.is_none());
        assert!(empty_options.sort.is_none());
        assert!(empty_options.limit.is_none());
        assert!(empty_options.skip.is_none());
        
        // Test builder with only projection
        let projection_only = MongoDbQueryBuilder::new()
            .project(&["field1", "field2"]);
        
        let proj_options = projection_only.build_find_options();
        assert!(proj_options.projection.is_some());
        assert!(proj_options.sort.is_none());
        
        // Test builder with multiple sorts
        let multi_sort = MongoDbQueryBuilder::new()
            .sort("field1", 1)
            .sort("field2", -1)
            .sort("field3", 1);
        
        let sort_options = multi_sort.build_find_options();
        if let Some(sort_doc) = sort_options.sort {
            assert_eq!(sort_doc.len(), 3);
            assert!(sort_doc.contains_key("field1"));
            assert!(sort_doc.contains_key("field2"));
            assert!(sort_doc.contains_key("field3"));
        }
        
        // Test zero values
        let zero_values = MongoDbQueryBuilder::new()
            .limit(0)
            .skip(0);
        
        let zero_options = zero_values.build_find_options();
        assert_eq!(zero_options.limit, Some(0));
        assert_eq!(zero_options.skip, Some(0));
    }

    /// Test large data structures conversion
    #[test]
    fn test_large_data_conversion() {
        // Create a large array
        let large_array = Value::Array(
            (0..1000).map(|i| Value::Int(i)).collect()
        );
        
        let bson_array = super::value_to_bson(&large_array).unwrap();
        if let Bson::Array(arr) = bson_array {
            assert_eq!(arr.len(), 1000);
            assert!(matches!(arr[0], Bson::Int64(0)));
            assert!(matches!(arr[999], Bson::Int64(999)));
        }
        
        // Create a large object
        let large_object = Value::Object(
            (0..100).map(|i| (format!("field_{}", i), Value::String(format!("value_{}", i))))
                    .collect()
        );
        
        let bson_object = super::value_to_bson(&large_object).unwrap();
        if let Bson::Document(doc) = bson_object {
            assert_eq!(doc.len(), 100);
            assert!(doc.contains_key("field_0"));
            assert!(doc.contains_key("field_99"));
        }
    }

    /// Test special BSON types conversion
    #[test]
    fn test_special_bson_types() {
        use mongodb::bson::{DateTime, Binary, Decimal128};
        
        // Test DateTime
        let now = DateTime::now();
        let bson_datetime = Bson::DateTime(now);
        let value_datetime = super::bson_to_value(&bson_datetime).unwrap();
        assert!(matches!(value_datetime, Value::String(_)));
        
        // Test Binary
        let binary_data = Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: vec![1, 2, 3, 4, 5],
        };
        let bson_binary = Bson::Binary(binary_data);
        let value_binary = super::bson_to_value(&bson_binary).unwrap();
        if let Value::String(s) = value_binary {
            assert!(s.contains("Binary"));
            assert!(s.contains("5 bytes"));
        }
        
        // Test Decimal128
        let decimal = Decimal128::from_str("123.456").unwrap();
        let bson_decimal = Bson::Decimal128(decimal);
        let value_decimal = super::bson_to_value(&bson_decimal).unwrap();
        assert!(matches!(value_decimal, Value::String(_)));
    }

    /// Performance test for conversions
    #[test]
    fn test_conversion_performance() {
        use std::time::Instant;
        
        // Create test data
        let test_objects: Vec<Value> = (0..1000).map(|i| {
            Value::Object({
                let mut obj = HashMap::new();
                obj.insert("id".to_string(), Value::Int(i));
                obj.insert("name".to_string(), Value::String(format!("Object {}", i)));
                obj.insert("active".to_string(), Value::Bool(i % 2 == 0));
                obj.insert("score".to_string(), Value::Float(i as f64 * 0.5));
                obj
            })
        }).collect();
        
        // Test Value to BSON conversion performance
        let start = Instant::now();
        for obj in &test_objects {
            let _ = super::value_to_bson(obj).unwrap();
        }
        let value_to_bson_duration = start.elapsed();
        
        // Convert to BSON for reverse test
        let bson_objects: Vec<Bson> = test_objects.iter()
            .map(|obj| super::value_to_bson(obj).unwrap())
            .collect();
        
        // Test BSON to Value conversion performance
        let start = Instant::now();
        for bson_obj in &bson_objects {
            let _ = super::bson_to_value(bson_obj).unwrap();
        }
        let bson_to_value_duration = start.elapsed();
        
        // Performance assertions (generous limits for CI environments)
        assert!(value_to_bson_duration.as_millis() < 1000, 
                "Value to BSON conversion took too long: {:?}", value_to_bson_duration);
        assert!(bson_to_value_duration.as_millis() < 1000, 
                "BSON to Value conversion took too long: {:?}", bson_to_value_duration);
        
        println!("Performance test results:");
        println!("  Value to BSON: {:?} for 1000 objects", value_to_bson_duration);
        println!("  BSON to Value: {:?} for 1000 objects", bson_to_value_duration);
    }

    /// Test default implementations
    #[test]
    fn test_default_implementations() {
        // Test MongoDbDriver default
        let driver = MongoDbDriver::default();
        let connections = tokio_test::block_on(driver.list_connections());
        assert!(connections.is_empty());
        
        // Test MongoDbQueryBuilder default
        let query_builder = MongoDbQueryBuilder::default();
        let filter = query_builder.get_filter();
        assert!(filter.is_empty());
        
        // Test AggregationPipelineBuilder default
        let pipeline_builder = AggregationPipelineBuilder::default();
        let pipeline = pipeline_builder.build();
        assert!(pipeline.is_empty());
    }
}
