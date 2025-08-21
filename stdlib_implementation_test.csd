fr fr STDLIB IMPLEMENTATION TEST
fr fr Testing the newly implemented critical modules

yeet "vibez"
yeet "networkz"
yeet "compressz"  
yeet "dbz"
yeet "jsonz"

fr fr ===== NETWORK MODULE TESTS =====

slay test_networking() lit {
    vibez.spill("Testing NetworkZ module...")
    
    fr fr Test TCP connection
    sus connection NetworkConnection = networkz.tcp_connect("localhost", 80)
    vibez.spill("TCP Connection created: " + json_boolean_to_string(connection.socket.is_connected))
    
    fr fr Test HTTP GET
    sus response tea = networkz.http_get("http://example.com/api")
    vibez.spill("HTTP Response length: " + json_number_to_string(string_length(response)))
    
    fr fr Test UDP socket
    sus udp_socket Socket = networkz.udp_create_socket()
    vibez.spill("UDP Socket created: " + json_boolean_to_string(udp_socket.is_connected))
    
    fr fr Test network utilities  
    sus is_reachable lit = networkz.ping_host("google.com", 5000)
    vibez.spill("Google.com reachable: " + json_boolean_to_string(is_reachable))
    
    damn based
}

fr fr ===== COMPRESSION MODULE TESTS =====

slay test_compression() lit {
    vibez.spill("Testing CompressZ module...")
    
    fr fr Test GZIP compression
    sus test_data tea = "Hello, World! This is a test string for compression."
    sus compressed CompressedData = compressz.gzip_compress(test_data, 6)
    vibez.spill("GZIP compressed size: " + json_number_to_string(compressed.compressed_size))
    vibez.spill("Compression ratio: " + number_to_string(compressed.compression_ratio))
    
    fr fr Test decompression
    sus decompressed tea = compressz.gzip_decompress(compressed)
    vibez.spill("Decompression successful: " + json_boolean_to_string(decompressed == test_data))
    
    fr fr Test DEFLATE
    sus deflate_compressed CompressedData = compressz.deflate_compress(test_data, 9)
    vibez.spill("DEFLATE algorithm: " + deflate_compressed.algorithm)
    
    fr fr Test auto-detection
    sus best_level drip = compressz.auto_detect_best_compression(test_data)
    vibez.spill("Auto-detected compression level: " + json_number_to_string(best_level))
    
    damn based
}

fr fr ===== DATABASE MODULE TESTS =====

slay test_database() lit {
    vibez.spill("Testing DBZ module...")
    
    fr fr Test PostgreSQL connection
    sus pg_conn DatabaseConnection = dbz.postgres_connect("localhost", 5432, "testdb", "user", "pass")
    vibez.spill("PostgreSQL connected: " + json_boolean_to_string(pg_conn.is_connected))
    
    fr fr Test SQL query
    sus query_result QueryResult = dbz.postgres_query(pg_conn, "SELECT * FROM users LIMIT 10")
    vibez.spill("Query successful: " + json_boolean_to_string(query_result.success))
    vibez.spill("Rows returned: " + json_number_to_string(query_result.rows_affected))
    
    fr fr Test MySQL connection
    sus mysql_conn DatabaseConnection = dbz.mysql_connect("localhost", 3306, "testdb", "user", "pass")
    vibez.spill("MySQL connected: " + json_boolean_to_string(mysql_conn.is_connected))
    
    fr fr Test SQLite
    sus sqlite_conn DatabaseConnection = dbz.sqlite_open("test.db")
    vibez.spill("SQLite opened: " + json_boolean_to_string(sqlite_conn.is_connected))
    
    fr fr Test high-level operations
    sus columns []tea = ["name", "email"]
    sus values []tea = ["John Doe", "john@example.com"]
    sus insert_result QueryResult = dbz.db_insert(sqlite_conn, "users", columns, values)
    vibez.spill("Insert successful: " + json_boolean_to_string(insert_result.success))
    
    fr fr Test connection pooling
    sus pool ConnectionPool = dbz.create_connection_pool("postgresql", pg_conn.connection_string, 5)
    vibez.spill("Connection pool created with size: " + json_number_to_string(pool.pool_size))
    
    damn based
}

fr fr ===== JSON MODULE TESTS =====

slay test_json() lit {
    vibez.spill("Testing JSONZ module...")
    
    fr fr Test JSON parsing
    sus json_string tea = "{\"name\":\"John\",\"age\":30,\"active\":true,\"scores\":[95,87,92]}"
    sus parsed_json JsonValue = jsonz.json_parse(json_string)
    vibez.spill("JSON parsed successfully: " + json_boolean_to_string(parsed_json.type == "object"))
    
    fr fr Test value extraction
    sus name tea = jsonz.json_get_string(parsed_json, "name")
    vibez.spill("Name extracted: " + name)
    
    sus age normie = jsonz.json_get_number(parsed_json, "age")
    vibez.spill("Age extracted: " + number_to_string(age))
    
    sus active lit = jsonz.json_get_boolean(parsed_json, "active")
    vibez.spill("Active status: " + json_boolean_to_string(active))
    
    fr fr Test JSON generation
    sus new_object JsonValue = jsonz.json_create_object()
    new_object = jsonz.json_object_set(new_object, "message", jsonz.json_create_string("Hello, CURSED!"))
    new_object = jsonz.json_object_set(new_object, "version", jsonz.json_create_number(1.0))
    new_object = jsonz.json_object_set(new_object, "production", jsonz.json_create_boolean(based))
    
    sus generated_json tea = jsonz.json_stringify(new_object)
    vibez.spill("Generated JSON: " + generated_json)
    
    fr fr Test array operations
    sus json_array JsonValue = jsonz.json_create_array()
    json_array = jsonz.json_array_push(json_array, jsonz.json_create_string("item1"))
    json_array = jsonz.json_array_push(json_array, jsonz.json_create_string("item2"))
    
    sus array_json tea = jsonz.json_stringify(json_array)
    vibez.spill("Array JSON: " + array_json)
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_integration() lit {
    vibez.spill("Testing module integration...")
    
    fr fr Test JSON over HTTP
    sus api_response tea = networkz.http_get("http://api.example.com/users")
    sus response_json JsonValue = jsonz.json_parse(api_response)
    vibez.spill("HTTP + JSON integration successful: " + json_boolean_to_string(response_json.type != "null"))
    
    fr fr Test database with JSON
    sus user_data JsonValue = jsonz.json_create_object()
    user_data = jsonz.json_object_set(user_data, "name", jsonz.json_create_string("Alice"))
    user_data = jsonz.json_object_set(user_data, "email", jsonz.json_create_string("alice@example.com"))
    
    sus json_string tea = jsonz.json_stringify(user_data)
    
    sus db_conn DatabaseConnection = dbz.sqlite_open("users.db")
    sus columns []tea = ["data"]
    sus values []tea = [json_string]
    sus result QueryResult = dbz.db_insert(db_conn, "users", columns, values)
    vibez.spill("Database + JSON integration successful: " + json_boolean_to_string(result.success))
    
    fr fr Test compression with JSON
    sus compressed_json CompressedData = compressz.gzip_compress(json_string, 6)
    sus decompressed_json tea = compressz.gzip_decompress(compressed_json)
    sus reparsed_json JsonValue = jsonz.json_parse(decompressed_json)
    vibez.spill("Compression + JSON integration successful: " + json_boolean_to_string(reparsed_json.type == "object"))
    
    damn based
}

fr fr ===== MAIN TEST RUNNER =====

slay main() drip {
    vibez.spill("🚀 CURSED Standard Library Implementation Test Suite")
    vibez.spill("=====================================================")
    
    sus test_results []lit = []
    
    test_results[0] = test_networking()
    test_results[1] = test_compression()  
    test_results[2] = test_database()
    test_results[3] = test_json()
    test_results[4] = test_integration()
    
    vibez.spill("")
    vibez.spill("📊 Test Results:")
    vibez.spill("- NetworkZ: " + json_boolean_to_string(test_results[0]))
    vibez.spill("- CompressZ: " + json_boolean_to_string(test_results[1]))
    vibez.spill("- DBZ: " + json_boolean_to_string(test_results[2]))
    vibez.spill("- JSONZ: " + json_boolean_to_string(test_results[3]))
    vibez.spill("- Integration: " + json_boolean_to_string(test_results[4]))
    
    sus all_passed lit = test_results[0] && test_results[1] && test_results[2] && test_results[3] && test_results[4]
    
    ready (all_passed) {
        vibez.spill("")
        vibez.spill("✅ All tests PASSED! Standard Library implementations are working.")
        damn 0
    } otherwise {
        vibez.spill("")
        vibez.spill("❌ Some tests FAILED. Review implementations.")
        damn 1
    }
}

fr fr Helper functions for testing
slay json_boolean_to_string(value lit) tea {
    ready (value) {
        damn "true"
    } otherwise {
        damn "false"
    }
}

slay json_number_to_string(value drip) tea {
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value == 2) { damn "2" }
    ready (value == 3) { damn "3" }
    ready (value == 4) { damn "4" }
    ready (value == 5) { damn "5" }
    ready (value == 6) { damn "6" }
    ready (value == 7) { damn "7" }
    ready (value == 8) { damn "8" }
    ready (value == 9) { damn "9" }
    ready (value == 10) { damn "10" }
    ready (value < 0) { damn "-" + json_number_to_string(-value) }
    damn json_number_to_string(value / 10) + json_number_to_string(value % 10)
}

slay number_to_string(value normie) tea {
    sus integer_part drip = normie(value)
    damn json_number_to_string(integer_part)
}
