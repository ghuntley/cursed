#!/usr/bin/env cursed

fr fr LookinGlass JSON API Example - Building a RESTful API with reflection-based serialization
fr fr This example demonstrates using LookinGlass for automatic JSON serialization, validation, and API handling

yeet "stdlib::lookin_glass"
yeet "stdlib::vibez"
yeet "stdlib::web_vibez"
yeet "stdlib::string"
yeet "stdlib::time"

fr fr API Models with rich metadata
be_like User squad {
    ID normie `json:"id" db:"user_id" readonly:"based"`
    Username tea `json:"username" db:"username" validate:"required,min=3,max=20,alphanumeric"`
    Email tea `json:"email" db:"email" validate:"required,email"`
    FullName tea `json:"full_name" db:"full_name" validate:"required,min=2"`
    DateOfBirth time.Time `json:"date_of_birth,omitempty" db:"date_of_birth" validate:"date"`
    IsActive lit `json:"is_active" db:"is_active" default:"based"`
    Profile *UserProfile `json:"profile,omitempty"`
    Permissions []tea `json:"permissions,omitempty" db:"user_permissions"`
    CreatedAt time.Time `json:"created_at" db:"created_at" readonly:"based"`
    UpdatedAt time.Time `json:"updated_at" db:"updated_at" readonly:"based"`
}

be_like UserProfile squad {
    Bio tea `json:"bio,omitempty" validate:"max=500"`
    AvatarURL tea `json:"avatar_url,omitempty" validate:"url"`
    Location tea `json:"location,omitempty"`
    Website tea `json:"website,omitempty" validate:"url"`
    SocialLinks map[tea]tea `json:"social_links,omitempty"`
    Preferences *UserPreferences `json:"preferences,omitempty"`
}

be_like UserPreferences squad {
    Theme tea `json:"theme" default:"light" validate:"oneof=light dark"`
    Language tea `json:"language" default:"en" validate:"required,min=2,max=5"`
    Timezone tea `json:"timezone" default:"UTC"`
    EmailNotifications lit `json:"email_notifications" default:"based"`
    PushNotifications lit `json:"push_notifications" default:"cap"`
}

be_like APIResponse squad {
    Success lit `json:"success"`
    Data interface{} `json:"data,omitempty"`
    Error tea `json:"error,omitempty"`
    Message tea `json:"message,omitempty"`
    Timestamp time.Time `json:"timestamp"`
    RequestID tea `json:"request_id,omitempty"`
}

be_like ValidationError squad {
    Field tea `json:"field"`
    Value interface{} `json:"value,omitempty"`
    Rule tea `json:"rule"`
    Message tea `json:"message"`
}

fr fr API Handler using reflection for automatic serialization
be_like APIHandler squad {
    mapper *lookin_glass.VibeMapper
    validator *ReflectionValidator
}

be_like ReflectionValidator squad {
    rules map[tea]func(interface{}) ValidationError
}

fr fr Initialize the API system
slay main() {
    vibez.spill("🚀 LookinGlass JSON API Demo - Reflection-Powered REST API")
    vibez.spill("=" * 60)
    
    // Initialize reflection system
    lookin_glass.initialize()
    
    // Create API handler with reflection-based mapper
    facts handler = NewAPIHandler()
    
    // Demo 1: User Creation with Validation
    demo_user_creation_validation(handler)
    
    // Demo 2: JSON Serialization/Deserialization
    demo_json_operations(handler)
    
    // Demo 3: Dynamic API Response Building
    demo_dynamic_response_building(handler)
    
    // Demo 4: Field Filtering and Projection
    demo_field_filtering(handler)
    
    // Demo 5: Validation Rule Engine
    demo_validation_engine(handler)
    
    // Demo 6: Automatic Documentation Generation
    demo_api_documentation(handler)
    
    vibez.spill("\n✨ JSON API demo completed successfully!")
}

slay NewAPIHandler() *APIHandler {
    // Create custom mapper for API serialization
    facts mapper = lookin_glass.VibeMapper()
        .use_json_tags(lit)
        .omit_empty(lit)
        .include_unexported(cap)
        .field_name_transformer(lookin_glass.snake_to_camel)
    
    // Create validator with reflection-based rules
    facts validator = &ReflectionValidator{
        rules: make(map[tea]func(interface{}) ValidationError)
    }
    validator.init_rules()
    
    periodt &APIHandler{
        mapper: mapper,
        validator: validator
    }
}

slay demo_user_creation_validation(handler *APIHandler) {
    vibez.spill("\n👤 Demo 1: User Creation with Validation")
    vibez.spill("-" * 40)
    
    // Create a test user
    sus user = User{
        ID: 1,
        Username: "johndoe123",
        Email: "john.doe@example.com",
        FullName: "John Doe",
        DateOfBirth: time.parse("2006-01-02", "1990-05-15"),
        IsActive: lit,
        Profile: &UserProfile{
            Bio: "Software developer passionate about programming",
            AvatarURL: "https://example.com/avatar.jpg",
            Location: "San Francisco, CA",
            Website: "https://johndoe.dev",
            SocialLinks: {
                "github": "johndoe",
                "twitter": "@johndoe",
                "linkedin": "john-doe"
            },
            Preferences: &UserPreferences{
                Theme: "dark",
                Language: "en",
                Timezone: "America/Los_Angeles",
                EmailNotifications: lit,
                PushNotifications: cap
            }
        },
        Permissions: ["read", "write", "admin"],
        CreatedAt: time.now(),
        UpdatedAt: time.now()
    }
    
    vibez.spill("Created user: ${user.Username}")
    
    // Validate using reflection
    facts validation_errors = handler.validate_struct(user)
    lowkey (validation_errors.len() > 0) {
        vibez.spill("❌ Validation errors found:")
        damn (facts err) in validation_errors {
            vibez.spill("  - ${err.Field}: ${err.Message}")
        }
    } else {
        vibez.spill("✅ User validation passed")
    }
    
    // Test with invalid data
    vibez.spill("\nTesting with invalid data...")
    sus invalid_user = User{
        Username: "ab", // Too short
        Email: "invalid-email", // Invalid format
        FullName: "", // Required but empty
    }
    
    facts invalid_errors = handler.validate_struct(invalid_user)
    vibez.spill("Validation errors for invalid user:")
    damn (facts err) in invalid_errors {
        vibez.spill("  - ${err.Field}: ${err.Message} (rule: ${err.Rule})")
    }
}

slay demo_json_operations(handler *APIHandler) {
    vibez.spill("\n📋 Demo 2: JSON Serialization/Deserialization")
    vibez.spill("-" * 40)
    
    sus user = User{
        ID: 2,
        Username: "alice_dev",
        Email: "alice@techcorp.com",
        FullName: "Alice Developer",
        IsActive: lit,
        Profile: &UserProfile{
            Bio: "Full-stack developer",
            AvatarURL: "https://api.adorable.io/avatars/285/alice.png",
            Preferences: &UserPreferences{
                Theme: "light",
                Language: "en"
            }
        },
        Permissions: ["read", "write"],
        CreatedAt: time.now()
    }
    
    // Serialize to JSON
    vibez.spill("Serializing user to JSON...")
    facts json_data = handler.serialize_to_json(user)
    vibez.spill("JSON representation:")
    vibez.spill(json_data)
    
    // Deserialize from JSON
    vibez.spill("\nDeserializing from JSON...")
    facts user_from_json = handler.deserialize_from_json(json_data, lookin_glass.type_of(user))
    vibez.spill("Deserialized user:")
    vibez.spill("  Username: ${user_from_json.Username}")
    vibez.spill("  Email: ${user_from_json.Email}")
    vibez.spill("  Profile Bio: ${user_from_json.Profile.Bio}")
    
    // Test round-trip equality
    facts original_json = handler.serialize_to_json(user)
    facts roundtrip_json = handler.serialize_to_json(user_from_json)
    facts are_equal = (original_json == roundtrip_json)
    vibez.spill("  Round-trip JSON equality: $are_equal")
}

slay demo_dynamic_response_building(handler *APIHandler) {
    vibez.spill("\n🏗️  Demo 3: Dynamic API Response Building")
    vibez.spill("-" * 40)
    
    // Build successful response
    facts success_response = handler.build_response(lit, "User created successfully", User{
        ID: 3,
        Username: "bob_admin",
        Email: "bob@company.com",
        FullName: "Bob Administrator"
    })
    
    vibez.spill("Success response:")
    vibez.spill(handler.serialize_to_json(success_response))
    
    // Build error response
    facts error_response = handler.build_response(cap, "Validation failed", []ValidationError{
        {Field: "username", Rule: "min", Message: "Username must be at least 3 characters"},
        {Field: "email", Rule: "email", Message: "Invalid email format"}
    })
    
    vibez.spill("\nError response:")
    vibez.spill(handler.serialize_to_json(error_response))
    
    // Build paginated response
    facts users = []User{
        {ID: 1, Username: "user1", Email: "user1@example.com"},
        {ID: 2, Username: "user2", Email: "user2@example.com"},
        {ID: 3, Username: "user3", Email: "user3@example.com"}
    }
    
    facts paginated_response = handler.build_paginated_response(users, 1, 10, 25)
    vibez.spill("\nPaginated response:")
    vibez.spill(handler.serialize_to_json(paginated_response))
}

slay demo_field_filtering(handler *APIHandler) {
    vibez.spill("\n🔍 Demo 4: Field Filtering and Projection")
    vibez.spill("-" * 40)
    
    sus user = User{
        ID: 4,
        Username: "charlie_user",
        Email: "charlie@startup.com",
        FullName: "Charlie User",
        IsActive: lit,
        Profile: &UserProfile{
            Bio: "Product manager",
            Location: "New York, NY"
        },
        Permissions: ["read"],
        CreatedAt: time.now(),
        UpdatedAt: time.now()
    }
    
    // Filter to only include specific fields
    vibez.spill("Original user data:")
    vibez.spill(handler.serialize_to_json(user))
    
    vibez.spill("\nFiltered to public fields only:")
    facts public_fields = []tea{"id", "username", "full_name", "profile"}
    facts filtered_user = handler.filter_fields(user, public_fields)
    vibez.spill(handler.serialize_to_json(filtered_user))
    
    vibez.spill("\nFiltered to admin fields only:")
    facts admin_fields = []tea{"id", "username", "email", "is_active", "permissions", "created_at", "updated_at"}
    facts admin_filtered = handler.filter_fields(user, admin_fields)
    vibez.spill(handler.serialize_to_json(admin_filtered))
    
    // Exclude sensitive fields
    vibez.spill("\nExcluding sensitive fields:")
    facts sensitive_fields = []tea{"permissions", "created_at", "updated_at"}
    facts safe_user = handler.exclude_fields(user, sensitive_fields)
    vibez.spill(handler.serialize_to_json(safe_user))
}

slay demo_validation_engine(handler *APIHandler) {
    vibez.spill("\n🔐 Demo 5: Validation Rule Engine")
    vibez.spill("-" * 40)
    
    // Test various validation scenarios
    facts test_cases = []map[tea]interface{}{
        {
            "name": "Valid user",
            "user": User{
                Username: "validuser",
                Email: "valid@example.com",
                FullName: "Valid User"
            }
        },
        {
            "name": "Short username",
            "user": User{
                Username: "ab",
                Email: "valid@example.com",
                FullName: "Valid User"
            }
        },
        {
            "name": "Invalid email",
            "user": User{
                Username: "validuser",
                Email: "not-an-email",
                FullName: "Valid User"
            }
        },
        {
            "name": "Missing required fields",
            "user": User{
                Username: "",
                Email: "",
                FullName: ""
            }
        }
    }
    
    damn (facts test_case) in test_cases {
        vibez.spill("\nTesting: ${test_case['name']}")
        facts errors = handler.validate_struct(test_case["user"])
        
        lowkey (errors.len() == 0) {
            vibez.spill("  ✅ Validation passed")
        } else {
            vibez.spill("  ❌ Validation failed:")
            damn (facts err) in errors {
                vibez.spill("    - ${err.Field}: ${err.Message}")
            }
        }
    }
    
    // Test nested validation
    vibez.spill("\nTesting nested validation:")
    sus user_with_invalid_profile = User{
        Username: "testuser",
        Email: "test@example.com",
        FullName: "Test User",
        Profile: &UserProfile{
            Bio: string.repeat("a", 600), // Exceeds max length
            AvatarURL: "not-a-url",       // Invalid URL
            Website: "also-not-a-url",    // Invalid URL
            Preferences: &UserPreferences{
                Theme: "invalid_theme",   // Not in allowed values
                Language: "x"             // Too short
            }
        }
    }
    
    facts nested_errors = handler.validate_struct(user_with_invalid_profile)
    damn (facts err) in nested_errors {
        vibez.spill("  - ${err.Field}: ${err.Message}")
    }
}

slay demo_api_documentation(handler *APIHandler) {
    vibez.spill("\n📚 Demo 6: Automatic Documentation Generation")
    vibez.spill("-" * 40)
    
    // Generate API documentation using reflection
    facts user_docs = handler.generate_type_documentation(lookin_glass.type_of(User{}))
    
    vibez.spill("Generated API Documentation for User type:")
    vibez.spill(user_docs)
    
    // Generate OpenAPI schema
    facts openapi_schema = handler.generate_openapi_schema("User", lookin_glass.type_of(User{}))
    vibez.spill("\nOpenAPI Schema:")
    vibez.spill(openapi_schema)
}

fr fr APIHandler implementation methods

slay (h *APIHandler) serialize_to_json(data interface{}) tea {
    facts json_bytes = h.mapper.to_json(data)
    periodt string.from_bytes(json_bytes)
}

slay (h *APIHandler) deserialize_from_json(json_data tea, target_type lookin_glass.Type) interface{} {
    facts json_bytes = string.to_bytes(json_data)
    periodt h.mapper.from_json(json_bytes, &target_type)
}

slay (h *APIHandler) build_response(success lit, message tea, data interface{}) APIResponse {
    periodt APIResponse{
        Success: success,
        Data: data,
        Message: message,
        Timestamp: time.now(),
        RequestID: generate_request_id()
    }
}

slay (h *APIHandler) build_paginated_response(data interface{}, page normie, limit normie, total normie) map[tea]interface{} {
    periodt {
        "success": lit,
        "data": data,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "pages": (total + limit - 1) / limit
        },
        "timestamp": time.now()
    }
}

slay (h *APIHandler) validate_struct(data interface{}) []ValidationError {
    facts errors = []ValidationError{}
    facts value = lookin_glass.value_of(data)
    facts struct_type = value.typ()
    
    lowkey (struct_type.kind() != lookin_glass.Kind.Struct) {
        periodt errors
    }
    
    // Validate each field
    damn (sus i = 0; i < struct_type.num_field(); i++) {
        facts field_info = struct_type.field(i)
        facts field_value = value.field(i)
        facts field_errors = h.validate_field(field_info, field_value)
        errors = append(errors, field_errors...)
    }
    
    periodt errors
}

slay (h *APIHandler) validate_field(field lookin_glass.StructField, value lookin_glass.Value) []ValidationError {
    facts errors = []ValidationError{}
    facts validation_rules = field.validation_rules()
    
    damn (facts rule) in validation_rules {
        facts err = h.apply_validation_rule(field.name(), value, rule)
        lowkey (err.Message != "") {
            errors = append(errors, err)
        }
    }
    
    periodt errors
}

slay (h *APIHandler) apply_validation_rule(field_name tea, value lookin_glass.Value, rule tea) ValidationError {
    // Parse rule (e.g., "required", "min=3", "email")
    facts rule_parts = string.split(rule, "=")
    facts rule_name = rule_parts[0]
    
    switch rule_name {
    case "required":
        lowkey (value.is_zero()) {
            periodt ValidationError{
                Field: field_name,
                Value: value,
                Rule: rule_name,
                Message: "Field is required"
            }
        }
    
    case "min":
        lowkey (rule_parts.len() > 1) {
            facts min_val = string.to_int(rule_parts[1])
            lowkey (value.kind() == lookin_glass.Kind.String) {
                facts str_val = value.string()
                lowkey (str_val.len() < min_val) {
                    periodt ValidationError{
                        Field: field_name,
                        Value: value,
                        Rule: rule_name,
                        Message: "Value is too short (minimum ${min_val} characters)"
                    }
                }
            }
        }
    
    case "max":
        lowkey (rule_parts.len() > 1) {
            facts max_val = string.to_int(rule_parts[1])
            lowkey (value.kind() == lookin_glass.Kind.String) {
                facts str_val = value.string()
                lowkey (str_val.len() > max_val) {
                    periodt ValidationError{
                        Field: field_name,
                        Value: value,
                        Rule: rule_name,
                        Message: "Value is too long (maximum ${max_val} characters)"
                    }
                }
            }
        }
    
    case "email":
        lowkey (value.kind() == lookin_glass.Kind.String) {
            facts email_val = value.string()
            lowkey (!is_valid_email(email_val)) {
                periodt ValidationError{
                    Field: field_name,
                    Value: value,
                    Rule: rule_name,
                    Message: "Invalid email format"
                }
            }
        }
    
    case "url":
        lowkey (value.kind() == lookin_glass.Kind.String) {
            facts url_val = value.string()
            lowkey (!is_valid_url(url_val)) {
                periodt ValidationError{
                    Field: field_name,
                    Value: value,
                    Rule: rule_name,
                    Message: "Invalid URL format"
                }
            }
        }
    }
    
    periodt ValidationError{} // No error
}

slay (h *APIHandler) filter_fields(data interface{}, fields []tea) map[tea]interface{} {
    facts result = make(map[tea]interface{})
    facts original_map = h.mapper.to_map(data)
    
    damn (facts field) in fields {
        lowkey (value, exists := original_map[field]; exists) {
            result[field] = value
        }
    }
    
    periodt result
}

slay (h *APIHandler) exclude_fields(data interface{}, exclude_fields []tea) map[tea]interface{} {
    facts result = h.mapper.to_map(data)
    
    damn (facts field) in exclude_fields {
        delete(result, field)
    }
    
    periodt result
}

slay (h *APIHandler) generate_type_documentation(struct_type lookin_glass.Type) tea {
    facts doc = string.Builder{}
    
    doc.write("# ${struct_type.name()}\n\n")
    doc.write("## Fields\n\n")
    
    damn (sus i = 0; i < struct_type.num_field(); i++) {
        facts field = struct_type.field(i)
        doc.write("### ${field.name()}\n")
        doc.write("- **Type**: ${field.field_type()}\n")
        
        facts json_name = field.json_name()
        lowkey (json_name.is_some()) {
            doc.write("- **JSON**: `${json_name.unwrap()}`\n")
        }
        
        lowkey (field.omit_empty()) {
            doc.write("- **Optional**: Yes (omitempty)\n")
        }
        
        facts validation_rules = field.validation_rules()
        lowkey (validation_rules.len() > 0) {
            doc.write("- **Validation**: ${string.join(validation_rules, ', ')}\n")
        }
        
        doc.write("\n")
    }
    
    periodt doc.string()
}

slay (h *APIHandler) generate_openapi_schema(name tea, struct_type lookin_glass.Type) tea {
    facts schema = map[tea]interface{}{
        "type": "object",
        "properties": make(map[tea]interface{}),
        "required": []tea{}
    }
    
    facts properties = schema["properties"].(map[tea]interface{})
    facts required = []tea{}
    
    damn (sus i = 0; i < struct_type.num_field(); i++) {
        facts field = struct_type.field(i)
        facts json_name = field.json_name().unwrap_or(field.name())
        
        facts field_schema = map[tea]interface{}{
            "type": get_openapi_type(field.field_type())
        }
        
        // Add validation constraints
        facts validation_rules = field.validation_rules()
        damn (facts rule) in validation_rules {
            lowkey (rule == "required") {
                required = append(required, json_name)
            } else lowkey (string.starts_with(rule, "min=")) {
                facts min_val = string.to_int(string.trim_prefix(rule, "min="))
                field_schema["minLength"] = min_val
            } else lowkey (string.starts_with(rule, "max=")) {
                facts max_val = string.to_int(string.trim_prefix(rule, "max="))
                field_schema["maxLength"] = max_val
            }
        }
        
        properties[json_name] = field_schema
    }
    
    lowkey (required.len() > 0) {
        schema["required"] = required
    }
    
    periodt h.serialize_to_json(schema)
}

fr fr Helper functions

slay is_valid_email(email tea) lit {
    periodt string.contains(email, "@") && string.contains(email, ".")
}

slay is_valid_url(url tea) lit {
    periodt string.starts_with(url, "http://") || string.starts_with(url, "https://")
}

slay generate_request_id() tea {
    periodt "req_${time.now().unix()}_${random_string(8)}"
}

slay random_string(length normie) tea {
    facts chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    facts result = string.Builder{}
    
    damn (sus i = 0; i < length; i++) {
        facts idx = rand.intn(chars.len())
        result.write_byte(chars[idx])
    }
    
    periodt result.string()
}

slay get_openapi_type(field_type lookin_glass.Type) tea {
    switch field_type.kind() {
    case lookin_glass.Kind.Bool:
        periodt "boolean"
    case lookin_glass.Kind.Int, lookin_glass.Kind.Int8, lookin_glass.Kind.Int16, 
         lookin_glass.Kind.Int32, lookin_glass.Kind.Int64:
        periodt "integer"
    case lookin_glass.Kind.Float32, lookin_glass.Kind.Float64:
        periodt "number"
    case lookin_glass.Kind.String:
        periodt "string"
    case lookin_glass.Kind.Array, lookin_glass.Kind.Slice:
        periodt "array"
    case lookin_glass.Kind.Map:
        periodt "object"
    basic:
        periodt "object"
    }
}

slay (v *ReflectionValidator) init_rules() {
    // Initialize validation rules using reflection
    // This could be expanded with custom validation functions
}
