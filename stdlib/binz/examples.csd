fr fr BINZ EXAMPLES - Comprehensive Usage Examples
fr fr Demonstrates all major features of the binary serialization system

yeet "binz"
yeet "vibez"
yeet "stringz"
yeet "mathz"

fr fr ===== BASIC USAGE EXAMPLES =====

slay example_basic_types() lit {
    vibez.spill("🔧 Basic Types Example")
    vibez.spill("-" * 40)
    
    fr fr Create different basic types
    sus null_val BinzValue = binz_create_null()
    sus bool_val BinzValue = binz_create_bool(based)
    sus int_val BinzValue = binz_create_int(42)
    sus float_val BinzValue = binz_create_float(3.14159)
    sus string_val BinzValue = binz_create_string("Hello BINZ!")
    
    fr fr Encode each type
    sus null_encoded drip[value] = binz_encode(null_val)
    sus bool_encoded drip[value] = binz_encode(bool_val)
    sus int_encoded drip[value] = binz_encode(int_val)
    sus float_encoded drip[value] = binz_encode(float_val)
    sus string_encoded drip[value] = binz_encode(string_val)
    
    vibez.spill("Encoded sizes:")
    vibez.spill("  Null: " + int_to_string(array_length(null_encoded)) + " bytes")
    vibez.spill("  Bool: " + int_to_string(array_length(bool_encoded)) + " bytes")
    vibez.spill("  Int: " + int_to_string(array_length(int_encoded)) + " bytes")
    vibez.spill("  Float: " + int_to_string(array_length(float_encoded)) + " bytes")
    vibez.spill("  String: " + int_to_string(array_length(string_encoded)) + " bytes")
    
    fr fr Decode and verify
    sus null_decoded BinzValue = binz_decode(null_encoded)
    sus bool_decoded BinzValue = binz_decode(bool_encoded)
    sus int_decoded BinzValue = binz_decode(int_encoded)
    sus float_decoded BinzValue = binz_decode(float_encoded)
    sus string_decoded BinzValue = binz_decode(string_encoded)
    
    vibez.spill("Decoded values:")
    vibez.spill("  Bool: " + ready (bool_decoded.bool_value) { "true" } otherwise { "false" })
    vibez.spill("  Int: " + int_to_string(int_decoded.int_value))
    vibez.spill("  String: " + string_decoded.string_value)
    
    damn based
}

slay example_arrays_and_structs() lit {
    vibez.spill("📦 Arrays and Structs Example")
    vibez.spill("-" * 40)
    
    fr fr Create array with mixed types
    sus mixed_array BinzValue = binz_create_array()
    mixed_array.array_values[0] = binz_create_int(1)
    mixed_array.array_values[1] = binz_create_string("two")
    mixed_array.array_values[2] = binz_create_float(3.0)
    mixed_array.array_values[3] = binz_create_bool(based)
    mixed_array.array_values[4] = binz_create_null()
    
    fr fr Create person struct
    sus person BinzValue = binz_create_struct()
    person.struct_fields[0] = "name"
    person.struct_values[0] = binz_create_string("Alice Johnson")
    person.struct_fields[1] = "age"
    person.struct_values[1] = binz_create_int(28)
    person.struct_fields[2] = "active"
    person.struct_values[2] = binz_create_bool(based)
    person.struct_fields[3] = "score"
    person.struct_values[3] = binz_create_float(95.5)
    person.struct_fields[4] = "hobbies"
    
    sus hobbies BinzValue = binz_create_array()
    hobbies.array_values[0] = binz_create_string("reading")
    hobbies.array_values[1] = binz_create_string("hiking")
    hobbies.array_values[2] = binz_create_string("programming")
    person.struct_values[4] = hobbies
    
    fr fr Encode both
    sus array_encoded drip[value] = binz_encode(mixed_array)
    sus struct_encoded drip[value] = binz_encode(person)
    
    vibez.spill("Array encoded: " + int_to_string(array_length(array_encoded)) + " bytes")
    vibez.spill("Struct encoded: " + int_to_string(array_length(struct_encoded)) + " bytes")
    
    fr fr Decode and display
    sus array_decoded BinzValue = binz_decode(array_encoded)
    sus struct_decoded BinzValue = binz_decode(struct_encoded)
    
    vibez.spill("Array elements: " + int_to_string(array_length(array_decoded.array_values)))
    vibez.spill("Struct fields: " + int_to_string(array_length(struct_decoded.struct_fields)))
    
    fr fr Display struct contents
    sus field_count drip = array_length(struct_decoded.struct_fields)
    sus i drip = 0
    bestie (i < field_count) {
        sus field_name tea = struct_decoded.struct_fields[i]
        sus field_value BinzValue = struct_decoded.struct_values[i]
        
        ready (field_name == "name") {
            vibez.spill("  Name: " + field_value.string_value)
        } otherwise ready (field_name == "age") {
            vibez.spill("  Age: " + int_to_string(field_value.int_value))
        } otherwise ready (field_name == "active") {
            sus status tea = ready (field_value.bool_value) { "Active" } otherwise { "Inactive" }
            vibez.spill("  Status: " + status)
        } otherwise ready (field_name == "score") {
            vibez.spill("  Score: " + float_to_string(field_value.float_value))
        } otherwise ready (field_name == "hobbies") {
            vibez.spill("  Hobbies: " + int_to_string(array_length(field_value.array_values)) + " items")
        }
        
        i = i + 1
    }
    
    damn based
}

fr fr ===== SCHEMA SYSTEM EXAMPLES =====

slay example_schema_definition() lit {
    vibez.spill("📐 Schema Definition Example")
    vibez.spill("-" * 40)
    
    fr fr Define user schema with validation
    sus user_schema BinzSchema = binz_create_schema(1001, 1, "User")
    user_schema = binz_schema_add_field(user_schema, "id", "uint32", cringe)        # required
    user_schema = binz_schema_add_field(user_schema, "username", "string", cringe)  # required
    user_schema = binz_schema_add_field(user_schema, "email", "string", based)     # optional
    user_schema = binz_schema_add_field(user_schema, "age", "uint32", based)       # optional
    user_schema = binz_schema_add_field(user_schema, "premium", "bool", based)     # optional
    user_schema.compatibility_mode = "forward"
    
    vibez.spill("Created schema: " + user_schema.name + " v" + int_to_string(user_schema.version))
    vibez.spill("Schema ID: " + int_to_string(user_schema.id))
    vibez.spill("Fields: " + int_to_string(array_length(user_schema.field_names)))
    vibez.spill("Compatibility: " + user_schema.compatibility_mode)
    
    fr fr Create user data conforming to schema
    sus user_data BinzValue = binz_create_struct()
    user_data.struct_fields[0] = "id"
    user_data.struct_values[0] = binz_create_uint(12345)
    user_data.struct_fields[1] = "username"
    user_data.struct_values[1] = binz_create_string("alice_dev")
    user_data.struct_fields[2] = "email"
    user_data.struct_values[2] = binz_create_string("alice@example.com")
    user_data.struct_fields[3] = "age"
    user_data.struct_values[3] = binz_create_uint(25)
    user_data.struct_fields[4] = "premium"
    user_data.struct_values[4] = binz_create_bool(based)
    
    fr fr Validate against schema
    sus validation_result lit = binz_validate_against_schema(user_data, user_schema)
    vibez.spill("Schema validation: " + ready (validation_result) { "PASSED" } otherwise { "FAILED" })
    
    fr fr Encode with schema
    sus schema_encoded drip[value] = binz_encode_with_schema(user_data, user_schema)
    sus normal_encoded drip[value] = binz_encode(user_data)
    
    vibez.spill("Normal encoding: " + int_to_string(array_length(normal_encoded)) + " bytes")
    vibez.spill("Schema encoding: " + int_to_string(array_length(schema_encoded)) + " bytes")
    
    fr fr Decode with schema
    sus schema_decoded BinzValue = binz_decode_with_schema(schema_encoded, user_schema)
    vibez.spill("Schema decode successful: " + ready (schema_decoded.type_tag == TAG_STRUCT) { "YES" } otherwise { "NO" })
    
    damn based
}

slay example_schema_migration() lit {
    vibez.spill("🔄 Schema Migration Example")
    vibez.spill("-" * 40)
    
    fr fr Create version 1 schema
    sus product_v1 BinzSchema = binz_create_schema(2001, 1, "Product")
    product_v1 = binz_schema_add_field(product_v1, "id", "uint32", cringe)
    product_v1 = binz_schema_add_field(product_v1, "name", "string", cringe)
    product_v1 = binz_schema_add_field(product_v1, "price", "float64", cringe)
    product_v1.compatibility_mode = "backward"
    
    fr fr Create version 2 schema with changes
    sus product_v2 BinzSchema = binz_create_schema(2001, 2, "Product")
    product_v2 = binz_schema_add_field(product_v2, "id", "uint32", cringe)
    product_v2 = binz_schema_add_field(product_v2, "title", "string", cringe)      # renamed from 'name'
    product_v2 = binz_schema_add_field(product_v2, "price", "float64", cringe)
    product_v2 = binz_schema_add_field(product_v2, "category", "string", based)   # new optional field
    product_v2 = binz_schema_add_field(product_v2, "in_stock", "bool", based)    # new optional field
    product_v2.compatibility_mode = "full"
    
    fr fr Define migration rule
    sus migration BinzMigrationRule = BinzMigrationRule{}
    migration.from_version = 1
    migration.to_version = 2
    migration.field_mappings = []
    
    sus name_mapping BinzFieldMapping = BinzFieldMapping{}
    name_mapping.old_name = "name"
    name_mapping.new_name = "title"
    name_mapping.type_conversion = "none"
    migration.field_mappings[0] = name_mapping
    
    product_v1.migration_rules[0] = migration
    
    vibez.spill("Schema v1 fields: " + int_to_string(array_length(product_v1.field_names)))
    vibez.spill("Schema v2 fields: " + int_to_string(array_length(product_v2.field_names)))
    
    fr fr Create v1 data
    sus v1_product BinzValue = binz_create_struct()
    v1_product.struct_fields[0] = "id"
    v1_product.struct_values[0] = binz_create_uint(54321)
    v1_product.struct_fields[1] = "name"
    v1_product.struct_values[1] = binz_create_string("Laptop Pro")
    v1_product.struct_fields[2] = "price"
    v1_product.struct_values[2] = binz_create_float(1299.99)
    
    fr fr Test migration
    sus migrated_schema BinzSchema = binz_migrate_schema(product_v1, product_v2)
    vibez.spill("Migration successful: " + ready (migrated_schema.version == 2) { "YES" } otherwise { "NO" })
    
    damn based
}

fr fr ===== PERFORMANCE EXAMPLES =====

slay example_compression() lit {
    vibez.spill("🗜️ Compression Example")
    vibez.spill("-" * 40)
    
    fr fr Create repetitive data (good for compression)
    sus repetitive_array BinzValue = binz_create_array()
    
    fr fr Add 100 repeated values
    sus i drip = 0
    bestie (i < 100) {
        sus value drip = i / 10  fr fr Every 10 elements same value
        repetitive_array.array_values[i] = binz_create_int(value)
        i = i + 1
    }
    
    fr fr Normal encoding
    sus normal_encoded drip[value] = binz_encode(repetitive_array)
    
    fr fr Compressed encoding
    sus compressed_copy BinzValue = repetitive_array
    compressed_copy.type_tag = TAG_COMPRESSED
    sus compressed_encoded drip[value] = binz_encode(compressed_copy)
    
    sus normal_size drip = array_length(normal_encoded)
    sus compressed_size drip = array_length(compressed_encoded)
    sus compression_ratio normie = normie(compressed_size) / normie(normal_size) * 100.0
    
    vibez.spill("Original size: " + int_to_string(normal_size) + " bytes")
    vibez.spill("Compressed size: " + int_to_string(compressed_size) + " bytes")
    vibez.spill("Compression ratio: " + float_to_string(compression_ratio) + "%")
    
    fr fr Decode compressed data
    sus decompressed BinzValue = binz_decode(compressed_encoded)
    vibez.spill("Decompression successful: " + ready (decompressed.type_tag == TAG_ARRAY_MIXED) { "YES" } otherwise { "NO" })
    vibez.spill("Element count: " + int_to_string(array_length(decompressed.array_values)))
    
    damn based
}

slay example_memory_pools() lit {
    vibez.spill("🏊 Memory Pool Example")
    vibez.spill("-" * 40)
    
    fr fr Create large memory pool
    sus pool BinzMemoryPool = binz_create_memory_pool(8192)  # 8KB buffer
    
    fr fr Create test data
    sus test_struct BinzValue = binz_create_struct()
    test_struct.struct_fields[0] = "operation"
    test_struct.struct_values[0] = binz_create_string("user_login")
    test_struct.struct_fields[1] = "timestamp"
    test_struct.struct_values[1] = binz_create_uint(1693843200)  # Unix timestamp
    test_struct.struct_fields[2] = "user_id"
    test_struct.struct_values[2] = binz_create_uint(12345)
    test_struct.struct_fields[3] = "success"
    test_struct.struct_values[3] = binz_create_bool(based)
    
    fr fr Encode with pool (high performance)
    sus pooled_encoded drip[value] = binz_encode_with_pool(test_struct, pool)
    sus normal_encoded drip[value] = binz_encode(test_struct)
    
    vibez.spill("Pool buffer size: " + int_to_string(pool.size) + " bytes")
    vibez.spill("Pooled encoding: " + int_to_string(array_length(pooled_encoded)) + " bytes")
    vibez.spill("Normal encoding: " + int_to_string(array_length(normal_encoded)) + " bytes")
    vibez.spill("Results identical: " + ready (arrays_equal(pooled_encoded, normal_encoded)) { "YES" } otherwise { "NO" })
    
    fr fr Decode both
    sus pooled_decoded BinzValue = binz_decode(pooled_encoded)
    sus normal_decoded BinzValue = binz_decode(normal_encoded)
    
    vibez.spill("Both decoded successfully: " + 
                ready (pooled_decoded.type_tag == TAG_STRUCT && normal_decoded.type_tag == TAG_STRUCT) { "YES" } otherwise { "NO" })
    
    damn based
}

slay example_batch_operations() lit {
    vibez.spill("📦 Batch Operations Example")
    vibez.spill("-" * 40)
    
    fr fr Create batch of log entries
    sus log_entries BinzValue[value] = []
    
    sus i drip = 0
    bestie (i < 10) {
        sus log_entry BinzValue = binz_create_struct()
        log_entry.struct_fields[0] = "level"
        log_entry.struct_values[0] = binz_create_string(ready (i % 2 == 0) { "INFO" } otherwise { "ERROR" })
        
        log_entry.struct_fields[1] = "message"
        log_entry.struct_values[1] = binz_create_string("Log message " + int_to_string(i))
        
        log_entry.struct_fields[2] = "timestamp"
        log_entry.struct_values[2] = binz_create_uint(1693843200 + i)
        
        log_entries[i] = log_entry
        i = i + 1
    }
    
    fr fr Batch encode
    sus batch_encoded drip[value] = binz_encode_batch(log_entries)
    
    fr fr Individual encoding for comparison
    sus individual_total drip = 0
    sus j drip = 0
    bestie (j < array_length(log_entries)) {
        sus individual_encoded drip[value] = binz_encode(log_entries[j])
        individual_total = individual_total + array_length(individual_encoded)
        j = j + 1
    }
    
    sus batch_size drip = array_length(batch_encoded)
    
    vibez.spill("Batch entries: " + int_to_string(array_length(log_entries)))
    vibez.spill("Batch encoded: " + int_to_string(batch_size) + " bytes")
    vibez.spill("Individual total: " + int_to_string(individual_total) + " bytes")
    vibez.spill("Batch overhead: " + int_to_string(batch_size - individual_total) + " bytes")
    
    fr fr Decode batch
    sus decoded_entries BinzValue[value] = binz_decode_batch(batch_encoded)
    vibez.spill("Decoded entries: " + int_to_string(array_length(decoded_entries)))
    
    fr fr Verify first entry
    ready (array_length(decoded_entries) > 0) {
        sus first_entry BinzValue = decoded_entries[0]
        ready (first_entry.type_tag == TAG_STRUCT) {
            vibez.spill("First entry is valid struct: YES")
        }
    }
    
    damn based
}

fr fr ===== REAL-WORLD USE CASES =====

slay example_configuration_file() lit {
    vibez.spill("⚙️ Configuration File Example")
    vibez.spill("-" * 40)
    
    fr fr Create application configuration
    sus config BinzValue = binz_create_struct()
    
    fr fr Database settings
    sus db_config BinzValue = binz_create_struct()
    db_config.struct_fields[0] = "host"
    db_config.struct_values[0] = binz_create_string("localhost")
    db_config.struct_fields[1] = "port"
    db_config.struct_values[1] = binz_create_uint(5432)
    db_config.struct_fields[2] = "database"
    db_config.struct_values[2] = binz_create_string("myapp")
    db_config.struct_fields[3] = "ssl_enabled"
    db_config.struct_values[3] = binz_create_bool(based)
    
    fr fr Server settings
    sus server_config BinzValue = binz_create_struct()
    server_config.struct_fields[0] = "port"
    server_config.struct_values[0] = binz_create_uint(8080)
    server_config.struct_fields[1] = "workers"
    server_config.struct_values[1] = binz_create_uint(4)
    server_config.struct_fields[2] = "debug"
    server_config.struct_values[2] = binz_create_bool(cringe)
    
    fr fr Feature flags
    sus features BinzValue = binz_create_array()
    features.array_values[0] = binz_create_string("user_auth")
    features.array_values[1] = binz_create_string("file_upload")
    features.array_values[2] = binz_create_string("email_notifications")
    
    fr fr Assemble main config
    config.struct_fields[0] = "database"
    config.struct_values[0] = db_config
    config.struct_fields[1] = "server"
    config.struct_values[1] = server_config
    config.struct_fields[2] = "enabled_features"
    config.struct_values[2] = features
    config.struct_fields[3] = "version"
    config.struct_values[3] = binz_create_string("1.2.3")
    
    fr fr Encode configuration
    sus config_encoded drip[value] = binz_encode(config)
    vibez.spill("Configuration encoded: " + int_to_string(array_length(config_encoded)) + " bytes")
    
    fr fr Simulate saving to file and loading
    sus loaded_config BinzValue = binz_decode(config_encoded)
    
    ready (loaded_config.type_tag == TAG_STRUCT) {
        vibez.spill("Configuration loaded successfully")
        vibez.spill("Root fields: " + int_to_string(array_length(loaded_config.struct_fields)))
    }
    
    damn based
}

slay example_api_message_protocol() lit {
    vibez.spill("🌐 API Message Protocol Example")
    vibez.spill("-" * 40)
    
    fr fr Create API request message
    sus request BinzValue = binz_create_struct()
    request.struct_fields[0] = "method"
    request.struct_values[0] = binz_create_string("GET")
    request.struct_fields[1] = "path"
    request.struct_values[1] = binz_create_string("/api/users/12345")
    request.struct_fields[2] = "headers"
    
    sus headers BinzValue = binz_create_struct()
    headers.struct_fields[0] = "Authorization"
    headers.struct_values[0] = binz_create_string("Bearer jwt_token_here")
    headers.struct_fields[1] = "Content-Type"
    headers.struct_values[1] = binz_create_string("application/x-binz")
    headers.struct_fields[2] = "User-Agent"
    headers.struct_values[2] = binz_create_string("CURSED/1.0")
    request.struct_values[2] = headers
    
    request.struct_fields[3] = "timestamp"
    request.struct_values[3] = binz_create_uint(1693843200)
    
    fr fr Create API response message
    sus response BinzValue = binz_create_struct()
    response.struct_fields[0] = "status"
    response.struct_values[0] = binz_create_uint(200)
    response.struct_fields[1] = "message"
    response.struct_values[1] = binz_create_string("OK")
    response.struct_fields[2] = "data"
    
    sus user_data BinzValue = binz_create_struct()
    user_data.struct_fields[0] = "id"
    user_data.struct_values[0] = binz_create_uint(12345)
    user_data.struct_fields[1] = "username"
    user_data.struct_values[1] = binz_create_string("john_doe")
    user_data.struct_fields[2] = "email"
    user_data.struct_values[2] = binz_create_string("john@example.com")
    user_data.struct_fields[3] = "created_at"
    user_data.struct_values[3] = binz_create_uint(1693000000)
    response.struct_values[2] = user_data
    
    response.struct_fields[3] = "response_time_ms"
    response.struct_values[3] = binz_create_uint(47)
    
    fr fr Encode both messages
    sus request_encoded drip[value] = binz_encode(request)
    sus response_encoded drip[value] = binz_encode(response)
    
    vibez.spill("Request size: " + int_to_string(array_length(request_encoded)) + " bytes")
    vibez.spill("Response size: " + int_to_string(array_length(response_encoded)) + " bytes")
    vibez.spill("Total payload: " + int_to_string(array_length(request_encoded) + array_length(response_encoded)) + " bytes")
    
    fr fr Decode and verify
    sus request_decoded BinzValue = binz_decode(request_encoded)
    sus response_decoded BinzValue = binz_decode(response_encoded)
    
    vibez.spill("Request decode: " + ready (request_decoded.type_tag == TAG_STRUCT) { "SUCCESS" } otherwise { "FAILED" })
    vibez.spill("Response decode: " + ready (response_decoded.type_tag == TAG_STRUCT) { "SUCCESS" } otherwise { "FAILED" })
    
    damn based
}

fr fr ===== HELPER FUNCTIONS =====

slay float_to_string(val normie) tea {
    fr fr Simple float to string conversion
    sus integer_part drip = normie(val)
    sus decimal_part normie = val - normie(integer_part)
    
    ready (decimal_part == 0.0) {
        damn int_to_string(integer_part) + ".0"
    } otherwise {
        sus scaled drip = normie(decimal_part * 100.0)  # 2 decimal places
        damn int_to_string(integer_part) + "." + int_to_string(scaled)
    }
}

slay arrays_equal(a drip[value], b drip[value]) lit {
    sus len_a drip = array_length(a)
    sus len_b drip = array_length(b)
    
    ready (len_a != len_b) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < len_a) {
        ready (a[i] != b[i]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

fr fr ===== MAIN EXAMPLE RUNNER =====

slay run_all_binz_examples() lit {
    vibez.spill("🚀 BINZ Binary Serialization Examples")
    vibez.spill("=" * 60)
    vibez.spill("")
    
    example_basic_types()
    vibez.spill("")
    
    example_arrays_and_structs()
    vibez.spill("")
    
    example_schema_definition()
    vibez.spill("")
    
    example_schema_migration()
    vibez.spill("")
    
    example_compression()
    vibez.spill("")
    
    example_memory_pools()
    vibez.spill("")
    
    example_batch_operations()
    vibez.spill("")
    
    example_configuration_file()
    vibez.spill("")
    
    example_api_message_protocol()
    vibez.spill("")
    
    vibez.spill("=" * 60)
    vibez.spill("✨ All BINZ examples completed successfully!")
    
    damn based
}

fr fr Run examples if this file is executed directly
run_all_binz_examples()
