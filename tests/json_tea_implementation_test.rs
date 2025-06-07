use std::sync::Arc;
use cursed::error::Error;
use cursed::object::Object;
use cursed::stdlib::json_tea;

#[test]
fn test_json_unmarshal_primitive_values() {
    // Test null
    let args = vec![Arc::new(Object::String("null".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    assert!(matches!(*result, Object::Null));

    // Test boolean true
    let args = vec![Arc::new(Object::String("true".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    assert!(matches!(*result, Object::Boolean(true)));

    // Test boolean false
    let args = vec![Arc::new(Object::String("false".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    assert!(matches!(*result, Object::Boolean(false)));

    // Test integer
    let args = vec![Arc::new(Object::String("42".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    assert!(matches!(*result, Object::Integer(42)));

    // Test float
    let args = vec![Arc::new(Object::String("3.14".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::Float(val) = &*result {
        assert!((*val - 3.14).abs() < f64::EPSILON);
    } else {
        panic!("Expected Float, got {:?}", result);
    }

    // Test string
    let args = vec![Arc::new(Object::String("\"hello\"".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::String(val) = &*result {
        assert_eq!(val, "hello");
    } else {
        panic!("Expected String, got {:?}", result);
    }
}

#[test]
fn test_json_unmarshal_arrays() {
    // Test empty array
    let args = vec![Arc::new(Object::String("[]".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::Array(val) = &*result {
        assert!(val.is_empty());
    } else {
        panic!("Expected Array, got {:?}", result);
    }

    // Test array with primitive values
    let args = vec![Arc::new(Object::String("[1, true, \"hello\"]".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::Array(val) = &*result {
        assert_eq!(val.len(), 3);
        assert!(matches!(val[0], Object::Integer(1)));
        assert!(matches!(val[1], Object::Boolean(true)));
        if let Object::String(s) = &val[2] {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected String, got {:?}", val[2]);
        }
    } else {
        panic!("Expected Array, got {:?}", result);
    }

    // Test nested array
    let args = vec![Arc::new(Object::String("[1, [2, 3], 4]".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::Array(val) = &*result {
        assert_eq!(val.len(), 3);
        assert!(matches!(val[0], Object::Integer(1)));
        if let Object::Array(inner) = &val[1] {
            assert_eq!(inner.len(), 2);
            assert!(matches!(inner[0], Object::Integer(2)));
            assert!(matches!(inner[1], Object::Integer(3)));
        } else {
            panic!("Expected inner Array, got {:?}", val[1]);
        }
        assert!(matches!(val[2], Object::Integer(4)));
    } else {
        panic!("Expected Array, got {:?}", result);
    }
}

#[test]
fn test_json_unmarshal_objects() {
    // Test empty object
    let args = vec![Arc::new(Object::String("{}".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::HashTable(val) = &*result {
        assert!(val.is_empty());
    } else {
        panic!("Expected HashTable, got {:?}", result);
    }

    // Test object with primitive values
    let args = vec![Arc::new(Object::String("{\"name\":\"zoomer\",\"age\":21,\"cool\":true}".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::HashTable(val) = &*result {
        assert_eq!(val.len(), 3);
        if let Some(Object::String(name)) = val.get("name") {
            assert_eq!(name, "zoomer");
        } else {
            panic!("Expected String for name, got {:?}", val.get("name"));
        }
        if let Some(Object::Integer(age)) = val.get("age") {
            assert_eq!(*age, 21);
        } else {
            panic!("Expected Integer for age, got {:?}", val.get("age"));
        }
        if let Some(Object::Boolean(cool)) = val.get("cool") {
            assert!(*cool);
        } else {
            panic!("Expected Boolean for cool, got {:?}", val.get("cool"));
        }
    } else {
        panic!("Expected HashTable, got {:?}", result);
    }

    // Test nested object
    let args = vec![Arc::new(Object::String("{\"user\":{\"name\":\"zoomer\",\"age\":21},\"active\":true}".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::HashTable(val) = &*result {
        assert_eq!(val.len(), 2);
        if let Some(Object::Boolean(active)) = val.get("active") {
            assert!(*active);
        } else {
            panic!("Expected Boolean for active, got {:?}", val.get("active"));
        }
        if let Some(Object::HashTable(user)) = val.get("user") {
            assert_eq!(user.len(), 2);
            if let Some(Object::String(name)) = user.get("name") {
                assert_eq!(name, "zoomer");
            } else {
                panic!("Expected String for name, got {:?}", user.get("name"));
            }
            if let Some(Object::Integer(age)) = user.get("age") {
                assert_eq!(*age, 21);
            } else {
                panic!("Expected Integer for age, got {:?}", user.get("age"));
            }
        } else {
            panic!("Expected HashTable for user, got {:?}", val.get("user"));
        }
    } else {
        panic!("Expected HashTable, got {:?}", result);
    }
}

#[test]
fn test_json_unmarshal_complex() {
    // Test complex nested structure
    let json = r#"{
        "users": [
            {"name": "zoomer", "skills": ["coding", "tiktok"]},
            {"name": "boomer", "skills": ["email", "facebook"]}
        ],
        "stats": {
            "average_age": 30.5,
            "active_users": 2
        },
        "settings": null
    }"#;

    let args = vec![Arc::new(Object::String(json.to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args).unwrap();
    
    if let Object::HashTable(val) = &*result {
        assert_eq!(val.len(), 3);
        
        // Check users array
        if let Some(Object::Array(users)) = val.get("users") {
            assert_eq!(users.len(), 2);
            
            // Check first user
            if let Object::HashTable(user1) = &users[0] {
                if let Some(Object::String(name)) = user1.get("name") {
                    assert_eq!(name, "zoomer");
                } else {
                    panic!("Expected String for name, got {:?}", user1.get("name"));
                }
                
                if let Some(Object::Array(skills)) = user1.get("skills") {
                    assert_eq!(skills.len(), 2);
                    if let Object::String(skill1) = &skills[0] {
                        assert_eq!(skill1, "coding");
                    } else {
                        panic!("Expected String for skill1, got {:?}", skills[0]);
                    }
                    if let Object::String(skill2) = &skills[1] {
                        assert_eq!(skill2, "tiktok");
                    } else {
                        panic!("Expected String for skill2, got {:?}", skills[1]);
                    }
                } else {
                    panic!("Expected Array for skills, got {:?}", user1.get("skills"));
                }
            } else {
                panic!("Expected HashTable for user1, got {:?}", users[0]);
            }
            
            // Check second user
            if let Object::HashTable(user2) = &users[1] {
                if let Some(Object::String(name)) = user2.get("name") {
                    assert_eq!(name, "boomer");
                } else {
                    panic!("Expected String for name, got {:?}", user2.get("name"));
                }
            } else {
                panic!("Expected HashTable for user2, got {:?}", users[1]);
            }
        } else {
            panic!("Expected Array for users, got {:?}", val.get("users"));
        }
        
        // Check stats object
        if let Some(Object::HashTable(stats)) = val.get("stats") {
            if let Some(Object::Float(avg_age)) = stats.get("average_age") {
                assert!((*avg_age - 30.5).abs() < f64::EPSILON);
            } else {
                panic!("Expected Float for average_age, got {:?}", stats.get("average_age"));
            }
            
            if let Some(Object::Integer(active_users)) = stats.get("active_users") {
                assert_eq!(*active_users, 2);
            } else {
                panic!("Expected Integer for active_users, got {:?}", stats.get("active_users"));
            }
        } else {
            panic!("Expected HashTable for stats, got {:?}", val.get("stats"));
        }
        
        // Check settings null
        if let Some(obj) = val.get("settings") {
            assert!(matches!(obj, Object::Null));
        } else {
            panic!("Expected Null for settings, got None");
        }
    } else {
        panic!("Expected HashTable, got {:?}", result);
    }
}

#[test]
fn test_json_unmarshal_error_cases() {
    // Test invalid JSON
    let args = vec![Arc::new(Object::String("{".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args);
    assert!(result.is_err());

    // Test missing second argument
    let args = vec![Arc::new(Object::String("{}".to_string()))];
    let result = json_tea::unmarshal(&args);
    assert!(result.is_err());

    // Test non-string first argument
    let args = vec![Arc::new(Object::Integer(42)), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args);
    assert!(result.is_err());

    // Test malformed JSON object
    let args = vec![Arc::new(Object::String("{\"key\":}".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args);
    assert!(result.is_err());

    // Test malformed JSON array
    let args = vec![Arc::new(Object::String("[1,]".to_string())), Arc::new(Object::Null)];
    let result = json_tea::unmarshal(&args);
    assert!(result.is_err());
}