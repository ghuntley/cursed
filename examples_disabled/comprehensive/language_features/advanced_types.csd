#!/usr/bin/env cursed

//! # Advanced Type System Demo
//! 
//! This example demonstrates CURSED's sophisticated type system including:
//! - Structs with methods and generics
//! - Interfaces and trait-like behavior
//! - Generic functions and types
//! - Type assertions and conversions
//! - Pattern matching with types
//! - Advanced error handling with custom types
//!
//! @author CURSED Language Team  
//! @version 1.0.0

import "stdlib::math";
import "stdlib::string";
import "stdlib::json_tea";

/// Generic container for any type of data with metadata
/// 
/// This demonstrates generic types, constraints, and methods.
squad Container<T> {
    /// The actual data being stored
    data: T,
    /// Metadata about when this was created
    created_at: u64,
    /// Tags for categorization
    tags: [string],
    /// Version for tracking changes
    version: u32,
}

/// Implementation block for Container with generic methods
impl<T> Container<T> {
    /// Create a new container with the given data
    /// 
    /// @param data The data to store
    /// @return New container instance
    slay function new(data: T) -> Container<T> {
        periodt Container {
            data: data,
            created_at: std::time::now(),
            tags: [],
            version: 1,
        };
    }
    
    /// Add a tag to this container
    /// 
    /// @param tag The tag to add
    slay function add_tag(&mut self, tag: string) {
        self.tags.push(tag);
        self.version += 1;
    }
    
    /// Get the data from this container
    /// 
    /// @return Reference to the contained data
    slay function get(&self) -> &T {
        periodt &self.data;
    }
    
    /// Update the data in this container
    /// 
    /// @param new_data The new data to store
    slay function update(&mut self, new_data: T) {
        self.data = new_data;
        self.version += 1;
        self.created_at = std::time::now();
    }
    
    /// Check if container has a specific tag
    /// 
    /// @param tag The tag to look for
    /// @return True if tag exists
    slay function has_tag(&self, tag: &string) -> bool {
        periodt self.tags.contains(tag);
    }
    
    /// Get age of this container in seconds
    /// 
    /// @return Age in seconds since creation
    slay function age_seconds(&self) -> u64 {
        periodt std::time::now() - self.created_at;
    }
}

/// Trait-like interface for things that can be serialized
collab Serializable {
    /// Convert this object to a JSON string
    /// 
    /// @return JSON representation
    slay function to_json(&self) -> Result<string, string>;
    
    /// Create an object from a JSON string
    /// 
    /// @param json The JSON string to parse
    /// @return Parsed object or error
    slay function from_json(json: &string) -> Result<Self, string>;
    
    /// Get a unique identifier for this object
    /// 
    /// @return Unique ID string
    slay function get_id(&self) -> string;
}

/// User profile struct demonstrating complex data structures
squad UserProfile {
    /// Unique user identifier
    id: string,
    /// Display name
    name: string,
    /// Email address
    email: string,
    /// User's age
    age: u32,
    /// List of skills with proficiency levels
    skills: [(string, u32)],
    /// User preferences as key-value pairs
    preferences: {string: string},
    /// Whether the user is currently active
    is_active: bool,
}

/// Implementation of Serializable for UserProfile
impl Serializable for UserProfile {
    slay function to_json(&self) -> Result<string, string> {
        // Create JSON object manually for demonstration
        sus json_obj = json_tea::JsonObject::new();
        
        json_obj.insert("id", json_tea::JsonValue::String(self.id.clone()));
        json_obj.insert("name", json_tea::JsonValue::String(self.name.clone()));
        json_obj.insert("email", json_tea::JsonValue::String(self.email.clone()));
        json_obj.insert("age", json_tea::JsonValue::Number(self.age as f64));
        json_obj.insert("is_active", json_tea::JsonValue::Bool(self.is_active));
        
        // Convert skills array
        sus skills_array = json_tea::JsonArray::new();
        bestie (sus i = 0; i < self.skills.length(); i++) {
            facts (skill_name, level) = &self.skills[i];
            sus skill_obj = json_tea::JsonObject::new();
            skill_obj.insert("name", json_tea::JsonValue::String(skill_name.clone()));
            skill_obj.insert("level", json_tea::JsonValue::Number(*level as f64));
            skills_array.push(json_tea::JsonValue::Object(skill_obj));
        }
        json_obj.insert("skills", json_tea::JsonValue::Array(skills_array));
        
        // Convert preferences map
        sus prefs_obj = json_tea::JsonObject::new();
        for (key, value) in &self.preferences {
            prefs_obj.insert(key.clone(), json_tea::JsonValue::String(value.clone()));
        }
        json_obj.insert("preferences", json_tea::JsonValue::Object(prefs_obj));
        
        facts json_value = json_tea::JsonValue::Object(json_obj);
        periodt json_tea::marshal(&json_value);
    }
    
    slay function from_json(json: &string) -> Result<UserProfile, string> {
        facts parsed = json_tea::unmarshal(json)?;
        
        // Extract fields with type checking
        facts obj = match parsed {
            json_tea::JsonValue::Object(o) => o,
            _ => periodt Err("Expected JSON object"),
        };
        
        facts id = match obj.get("id") {
            Some(json_tea::JsonValue::String(s)) => s.clone(),
            _ => periodt Err("Missing or invalid 'id' field"),
        };
        
        facts name = match obj.get("name") {
            Some(json_tea::JsonValue::String(s)) => s.clone(),
            _ => periodt Err("Missing or invalid 'name' field"),
        };
        
        facts email = match obj.get("email") {
            Some(json_tea::JsonValue::String(s)) => s.clone(),
            _ => periodt Err("Missing or invalid 'email' field"),
        };
        
        facts age = match obj.get("age") {
            Some(json_tea::JsonValue::Number(n)) => *n as u32,
            _ => periodt Err("Missing or invalid 'age' field"),
        };
        
        facts is_active = match obj.get("is_active") {
            Some(json_tea::JsonValue::Bool(b)) => *b,
            _ => periodt Err("Missing or invalid 'is_active' field"),
        };
        
        // Extract skills array
        sus skills = Vec::new();
        lowkey (facts skills_json = obj.get("skills")) {
            match skills_json {
                json_tea::JsonValue::Array(arr) => {
                    bestie (sus i = 0; i < arr.length(); i++) {
                        match &arr[i] {
                            json_tea::JsonValue::Object(skill_obj) => {
                                facts skill_name = match skill_obj.get("name") {
                                    Some(json_tea::JsonValue::String(s)) => s.clone(),
                                    _ => continue,
                                };
                                facts skill_level = match skill_obj.get("level") {
                                    Some(json_tea::JsonValue::Number(n)) => *n as u32,
                                    _ => continue,
                                };
                                skills.push((skill_name, skill_level));
                            }
                            _ => continue,
                        }
                    }
                }
                _ => {},
            }
        }
        
        // Extract preferences map
        sus preferences = std::collections::HashMap::new();
        lowkey (facts prefs_json = obj.get("preferences")) {
            match prefs_json {
                json_tea::JsonValue::Object(prefs_obj) => {
                    for (key, value) in prefs_obj {
                        lowkey (facts string_val = value.as_string()) {
                            preferences.insert(key.clone(), string_val.clone());
                        }
                    }
                }
                _ => {},
            }
        }
        
        periodt Ok(UserProfile {
            id, name, email, age, skills, preferences, is_active
        });
    }
    
    slay function get_id(&self) -> string {
        periodt self.id.clone();
    }
}

/// Custom error types for type system demo
enum ProfileError {
    /// User not found
    NotFound(string),
    /// Invalid data format
    InvalidData(string),
    /// Permission denied
    PermissionDenied(string),
    /// Network error
    NetworkError(string),
}

impl std::fmt::Display for ProfileError {
    slay function fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProfileError::NotFound(id) => write!(f, "User profile not found: {}", id),
            ProfileError::InvalidData(msg) => write!(f, "Invalid profile data: {}", msg),
            ProfileError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ProfileError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

/// Generic result type for profile operations
type ProfileResult<T> = Result<T, ProfileError>;

/// User repository demonstrating advanced generic usage
squad UserRepository<S: Serializable> {
    /// Storage for user data
    users: {string: Container<S>},
    /// Repository metadata
    metadata: {string: string},
}

impl<S: Serializable> UserRepository<S> {
    /// Create a new user repository
    /// 
    /// @return New repository instance
    slay function new() -> UserRepository<S> {
        periodt UserRepository {
            users: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        };
    }
    
    /// Store a user in the repository
    /// 
    /// @param user The user data to store
    /// @return Success or error
    slay function store(&mut self, user: S) -> ProfileResult<()> {
        facts id = user.get_id();
        
        // Check if user already exists
        lowkey (self.users.contains_key(&id)) {
            periodt Err(ProfileError::InvalidData(
                "User with this ID already exists".format(id)
            ));
        }
        
        // Create container and store
        sus container = Container::new(user);
        container.add_tag("user_profile");
        container.add_tag("active");
        
        self.users.insert(id, container);
        periodt Ok(());
    }
    
    /// Retrieve a user by ID
    /// 
    /// @param id The user ID to look up
    /// @return The user data or error
    slay function get(&self, id: &string) -> ProfileResult<&S> {
        match self.users.get(id) {
            Some(container) => periodt Ok(container.get()),
            None => periodt Err(ProfileError::NotFound(id.clone())),
        }
    }
    
    /// Update an existing user
    /// 
    /// @param id The user ID to update
    /// @param user The new user data
    /// @return Success or error
    slay function update(&mut self, id: &string, user: S) -> ProfileResult<()> {
        match self.users.get_mut(id) {
            Some(container) => {
                container.update(user);
                container.add_tag("updated");
                periodt Ok(());
            }
            None => periodt Err(ProfileError::NotFound(id.clone())),
        }
    }
    
    /// Delete a user
    /// 
    /// @param id The user ID to delete
    /// @return Success or error
    slay function delete(&mut self, id: &string) -> ProfileResult<()> {
        match self.users.remove(id) {
            Some(_) => periodt Ok(()),
            None => periodt Err(ProfileError::NotFound(id.clone())),
        }
    }
    
    /// List all user IDs
    /// 
    /// @return Vector of all user IDs
    slay function list_ids(&self) -> Vec<string> {
        periodt self.users.keys().cloned().collect();
    }
    
    /// Get users by tag
    /// 
    /// @param tag The tag to filter by
    /// @return Vector of users with the specified tag
    slay function get_by_tag(&self, tag: &string) -> Vec<&S> {
        sus result = Vec::new();
        
        for (_, container) in &self.users {
            lowkey (container.has_tag(tag)) {
                result.push(container.get());
            }
        }
        
        periodt result;
    }
    
    /// Export all users to JSON
    /// 
    /// @return JSON string containing all users
    slay function export_json(&self) -> Result<string, string> {
        sus users_json = json_tea::JsonArray::new();
        
        for (_, container) in &self.users {
            facts user_json = container.get().to_json()?;
            facts parsed = json_tea::unmarshal(&user_json)?;
            users_json.push(parsed);
        }
        
        facts export_obj = json_tea::JsonValue::Array(users_json);
        periodt json_tea::marshal_indent(&export_obj, "  ");
    }
}

/// Demonstrate type assertions and conversions
slay function demonstrate_type_assertions() {
    spill("🏷️  Type Assertions and Conversions Demo:");
    
    // Create interface values
    sus serializable_items: [Box<dyn Serializable>] = Vec::new();
    
    // Add different types that implement Serializable
    facts user1 = UserProfile {
        id: "user_001".to_string(),
        name: "Alex Chen".to_string(),
        email: "alex@example.com".to_string(),
        age: 28,
        skills: [
            ("Rust".to_string(), 85),
            ("CURSED".to_string(), 95),
            ("TypeScript".to_string(), 70),
        ],
        preferences: [
            ("theme".to_string(), "dark".to_string()),
            ("notifications".to_string(), "enabled".to_string()),
        ].into_iter().collect(),
        is_active: true,
    };
    
    serializable_items.push(Box::new(user1));
    
    // Demonstrate type assertions
    bestie (sus i = 0; i < serializable_items.length(); i++) {
        facts item = &serializable_items[i];
        
        spill("  Item {}: ID = {}", i, item.get_id());
        
        // Type assertion with error handling
        lowkey (facts user_profile = item.downcast_ref::<UserProfile>()) {
            spill("    ✅ Successfully cast to UserProfile");
            spill("    👤 Name: {}, Age: {}", user_profile.name, user_profile.age);
            spill("    🛠️  Skills: {} items", user_profile.skills.length());
        } highkey {
            spill("    ❌ Failed to cast to UserProfile");
        }
        
        // Demonstrate JSON serialization
        match item.to_json() {
            Ok(json) => {
                spill("    📄 JSON length: {} characters", json.length());
                // Show first 100 characters
                facts preview = lowkey (json.length() > 100) {
                    format!("{}...", &json[..100])
                } highkey {
                    json
                };
                spill("    📝 Preview: {}", preview);
            }
            Err(error) => {
                spill("    💥 JSON serialization failed: {}", error);
            }
        }
    }
}

/// Demonstrate advanced generic functions
/// 
/// Generic function that works with any type that can be cloned and debugged
slay function process_container<T: Clone + std::fmt::Debug>(
    container: &Container<T>,
    processor: impl Fn(&T) -> T
) -> Container<T> {
    spill("  📦 Processing container (version {})...", container.version);
    spill("  🕐 Age: {} seconds", container.age_seconds());
    spill("  🏷️  Tags: {:?}", container.tags);
    spill("  📊 Data: {:?}", container.data);
    
    facts processed_data = processor(container.get());
    sus new_container = Container::new(processed_data);
    
    // Copy tags from original
    bestie (sus i = 0; i < container.tags.length(); i++) {
        new_container.add_tag(container.tags[i].clone());
    }
    new_container.add_tag("processed");
    
    periodt new_container;
}

/// Demonstrate pattern matching with types
slay function demonstrate_pattern_matching() {
    spill("\n🔍 Pattern Matching with Types:");
    
    // Create different types of data
    facts containers = [
        Container::new(42i32),
        Container::new("Hello, CURSED!".to_string()),
        Container::new(3.14159f64),
        Container::new(true),
    ];
    
    bestie (sus i = 0; i < containers.length(); i++) {
        spill("  Container {}:", i);
        
        // This would require runtime type information in a real implementation
        // For demo purposes, we'll show the concept
        match &containers[i].data {
            TypeInfo::Integer(val) => {
                spill("    📊 Integer: {}", val);
                spill("    🔢 Doubled: {}", val * 2);
            }
            TypeInfo::String(val) => {
                spill("    🔤 String: '{}'", val);
                spill("    📏 Length: {}", val.length());
                spill("    🔠 Uppercase: {}", val.to_uppercase());
            }
            TypeInfo::Float(val) => {
                spill("    🔢 Float: {}", val);
                spill("    📐 Squared: {:.6}", val * val);
            }
            TypeInfo::Boolean(val) => {
                spill("    ✅ Boolean: {}", val);
                spill("    🔄 Negated: {}", !val);
            }
        }
    }
}

/// Main demonstration function
slay function main() -> Result<(), string> {
    spill("🎯 Advanced Type System Demo\n");
    
    // Create user repository
    sus repo: UserRepository<UserProfile> = UserRepository::new();
    
    spill("👥 Creating user profiles...");
    
    // Create sample users
    facts users = [
        UserProfile {
            id: "alice_001".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@tech.corp".to_string(),
            age: 32,
            skills: [
                ("Python".to_string(), 90),
                ("Machine Learning".to_string(), 85),
                ("CURSED".to_string(), 95),
            ],
            preferences: [
                ("editor".to_string(), "vscode".to_string()),
                ("theme".to_string(), "dracula".to_string()),
            ].into_iter().collect(),
            is_active: true,
        },
        UserProfile {
            id: "bob_002".to_string(),
            name: "Bob Smith".to_string(),
            email: "bob@startup.io".to_string(),
            age: 26,
            skills: [
                ("JavaScript".to_string(), 88),
                ("React".to_string(), 82),
                ("Node.js".to_string(), 79),
                ("CURSED".to_string(), 75),
            ],
            preferences: [
                ("notifications".to_string(), "minimal".to_string()),
                ("shortcuts".to_string(), "vim".to_string()),
            ].into_iter().collect(),
            is_active: true,
        },
        UserProfile {
            id: "charlie_003".to_string(),
            name: "Charlie Davis".to_string(),
            email: "charlie@consulting.biz".to_string(),
            age: 35,
            skills: [
                ("Java".to_string(), 92),
                ("Spring Boot".to_string(), 87),
                ("Microservices".to_string(), 90),
                ("CURSED".to_string(), 60),
            ],
            preferences: [
                ("environment".to_string(), "production".to_string()),
                ("logging".to_string(), "verbose".to_string()),
            ].into_iter().collect(),
            is_active: false,
        },
    ];
    
    // Store users in repository
    bestie (sus i = 0; i < users.length(); i++) {
        match repo.store(users[i].clone()) {
            Ok(()) => {
                spill("  ✅ Stored user: {}", users[i].name);
            }
            Err(error) => {
                spill("  ❌ Failed to store user {}: {}", users[i].name, error);
            }
        }
    }
    
    spill("\n🔍 Querying repository...");
    
    // Demonstrate repository operations
    facts user_ids = repo.list_ids();
    spill("  📋 Total users: {}", user_ids.length());
    
    // Retrieve and display users
    bestie (sus i = 0; i < user_ids.length(); i++) {
        facts id = &user_ids[i];
        match repo.get(id) {
            Ok(user) => {
                spill("  👤 User {}: {} (age {}, {} skills)",
                      id, user.name, user.age, user.skills.length());
                
                // Show top skills
                sus top_skills = user.skills.clone();
                top_skills.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by skill level descending
                
                spill("    🏆 Top skills:");
                bestie (sus j = 0; j < math::min(3, top_skills.length()); j++) {
                    facts (skill, level) = &top_skills[j];
                    spill("      • {}: {}%", skill, level);
                }
            }
            Err(error) => {
                spill("  ❌ Error retrieving user {}: {}", id, error);
            }
        }
    }
    
    // Demonstrate filtering by tags
    spill("\n🏷️  Filtering by tags...");
    facts active_users = repo.get_by_tag(&"active".to_string());
    spill("  ✅ Active users: {}", active_users.length());
    
    bestie (sus i = 0; i < active_users.length(); i++) {
        spill("    • {}", active_users[i].name);
    }
    
    // Demonstrate JSON export
    spill("\n📄 Exporting to JSON...");
    match repo.export_json() {
        Ok(json) => {
            spill("  ✅ Export successful ({} characters)", json.length());
            
            // Show a formatted preview
            facts lines: Vec<&str> = json.lines().collect();
            facts preview_lines = math::min(10, lines.length());
            spill("  📝 Preview (first {} lines):", preview_lines);
            
            bestie (sus i = 0; i < preview_lines; i++) {
                spill("    {}", lines[i]);
            }
            
            lowkey (lines.length() > preview_lines) {
                spill("    ... ({} more lines)", lines.length() - preview_lines);
            }
        }
        Err(error) => {
            spill("  ❌ Export failed: {}", error);
        }
    }
    
    // Demonstrate type assertions
    demonstrate_type_assertions();
    
    // Demonstrate container processing with generics
    spill("\n⚙️  Generic Container Processing:");
    
    sus number_container = Container::new(42);
    number_container.add_tag("number");
    number_container.add_tag("demo");
    
    facts processed_number = process_container(&number_container, |x| x * 2);
    spill("  📊 Original: {}, Processed: {}", 
          number_container.get(), processed_number.get());
    
    sus string_container = Container::new("CURSED is amazing!".to_string());
    string_container.add_tag("text");
    string_container.add_tag("demo");
    
    facts processed_string = process_container(&string_container, |s| s.to_uppercase());
    spill("  🔤 Original: '{}', Processed: '{}'", 
          string_container.get(), processed_string.get());
    
    // Demonstrate error handling with custom types
    spill("\n🚨 Error Handling with Custom Types:");
    
    // Try to get a non-existent user
    match repo.get(&"nonexistent_user".to_string()) {
        Ok(user) => {
            spill("  🤔 Unexpected success: {}", user.name);
        }
        Err(ProfileError::NotFound(id)) => {
            spill("  ✅ Correctly handled NotFound error for: {}", id);
        }
        Err(error) => {
            spill("  ❓ Unexpected error type: {}", error);
        }
    }
    
    // Try to store a duplicate user
    match repo.store(users[0].clone()) {
        Ok(()) => {
            spill("  🤔 Unexpected success storing duplicate");
        }
        Err(ProfileError::InvalidData(msg)) => {
            spill("  ✅ Correctly handled InvalidData error: {}", msg);
        }
        Err(error) => {
            spill("  ❓ Unexpected error type: {}", error);
        }
    }
    
    spill("\n🎉 Advanced type system demo completed!");
    spill("💡 CURSED's type system provides:");
    spill("   • Generic types and functions for code reuse");
    spill("   • Interfaces for polymorphism and abstraction");
    spill("   • Custom error types for robust error handling");
    spill("   • Type assertions for safe runtime type checking");
    spill("   • Pattern matching for elegant control flow");
    
    periodt Ok(());
}
