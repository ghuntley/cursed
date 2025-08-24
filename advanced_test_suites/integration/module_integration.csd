// Advanced Integration Testing: Module Integration
yeet "testz"
yeet "networkz"
yeet "filez"
yeet "jsonz"
yeet "cryptz"
yeet "timez"
yeet "concurrenz"
yeet "dbz"

test_start("Module Integration Tests")

// Web server integration test
slay test_web_server_integration() {
    sus server = create_http_server("localhost", 8080)
    
    // Start server in background
    go {
        handle_requests(server, slay(request) {
            sus path tea = get_request_path(request)
            sus method tea = get_request_method(request)
            
            ready (path == "/api/data" && method == "GET") {
                sus data = {
                    "message": "Hello, World!",
                    "timestamp": now_iso(),
                    "status": "success"
                }
                sus json_response tea = to_json(data)
                damn create_response(200, "application/json", json_response)
            }
            
            ready (path == "/api/users" && method == "POST") {
                sus body tea = get_request_body(request)
                sus user_data = parse_json(body)
                
                // Validate user data
                ready (has_field(user_data, "name") && has_field(user_data, "email")) {
                    sus user_id tea = generate_uuid()
                    sus response_data = {
                        "id": user_id,
                        "name": get_field(user_data, "name"),
                        "email": get_field(user_data, "email"),
                        "created_at": now_iso()
                    }
                    sus json_response tea = to_json(response_data)
                    damn create_response(201, "application/json", json_response)
                }
                
                damn create_response(400, "application/json", "{\"error\": \"Invalid user data\"}")
            }
            
            damn create_response(404, "text/plain", "Not Found")
        })
    }
    
    sleep(100)  // Let server start
    
    // Test GET request
    sus get_response = http_get("http://localhost:8080/api/data")
    assert_eq_int(get_response.status, 200)
    
    sus get_data = parse_json(get_response.body)
    assert_eq_str(get_field(get_data, "message"), "Hello, World!")
    
    // Test POST request
    sus user_data = {
        "name": "John Doe",
        "email": "john@example.com"
    }
    sus post_body tea = to_json(user_data)
    sus post_response = http_post("http://localhost:8080/api/users", post_body, "application/json")
    assert_eq_int(post_response.status, 201)
    
    sus created_user = parse_json(post_response.body)
    assert_eq_str(get_field(created_user, "name"), "John Doe")
    assert_eq_str(get_field(created_user, "email"), "john@example.com")
    
    // Test 404
    sus not_found_response = http_get("http://localhost:8080/api/nonexistent")
    assert_eq_int(not_found_response.status, 404)
    
    stop_server(server)
    
    test_pass("Web server integration")
}

// Database integration test
slay test_database_integration() {
    sus db = connect_database("memory", "")  // In-memory database
    
    // Create table
    execute_sql(db, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT, created_at TEXT)")
    
    // Insert data
    sus user_id drip = 1
    sus name tea = "Alice Smith"
    sus email tea = "alice@example.com"
    sus created_at tea = now_iso()
    
    sus insert_query tea = "INSERT INTO users (id, name, email, created_at) VALUES (?, ?, ?, ?)"
    execute_sql(db, insert_query, [user_id, name, email, created_at])
    
    // Query data
    sus select_query tea = "SELECT * FROM users WHERE id = ?"
    sus results = query_sql(db, select_query, [user_id])
    
    assert_eq_int(len(results), 1)
    
    sus user = results[0]
    assert_eq_int(get_field(user, "id") as drip, user_id)
    assert_eq_str(get_field(user, "name"), name)
    assert_eq_str(get_field(user, "email"), email)
    
    // Test JSON serialization of database results
    sus json_users tea = to_json(results)
    sus parsed_users = parse_json(json_users)
    
    assert_eq_int(len(parsed_users), 1)
    
    close_database(db)
    
    test_pass("Database integration")
}

// File processing pipeline integration
slay test_file_processing_pipeline() {
    sus input_dir tea = "test_input/"
    sus output_dir tea = "test_output/"
    sus processed_dir tea = "test_processed/"
    
    create_directory(input_dir)
    create_directory(output_dir)
    create_directory(processed_dir)
    
    // Create test files
    sus test_data []map<tea, any> = [
        {"name": "John", "age": 30, "city": "New York"},
        {"name": "Jane", "age": 25, "city": "Los Angeles"},
        {"name": "Bob", "age": 35, "city": "Chicago"}
    ]
    
    bestie (drip i = 0; i < len(test_data); i = i + 1) {
        sus filename tea = input_dir + "user_" + (i as tea) + ".json"
        sus json_content tea = to_json(test_data[i])
        write_file(filename, json_content)
    }
    
    // Process files
    sus input_files []tea = list_files(input_dir)
    
    bestie (tea filename in input_files) {
        ready (ends_with(filename, ".json")) {
            sus full_path tea = input_dir + filename
            sus content tea = read_file(full_path)
            sus data = parse_json(content)
            
            // Transform data
            set_field(data, "processed_at", now_iso())
            set_field(data, "age_group", ready (get_field(data, "age") as drip < 30) { damn "young" } otherwise { damn "mature" })
            
            // Write to output
            sus output_filename tea = replace(filename, ".json", "_processed.json")
            sus output_path tea = output_dir + output_filename
            write_file(output_path, to_json(data))
            
            // Move original to processed
            sus processed_path tea = processed_dir + filename
            move_file(full_path, processed_path)
        }
    }
    
    // Verify processing
    sus output_files []tea = list_files(output_dir)
    assert_eq_int(len(output_files), 3)
    
    sus processed_files []tea = list_files(processed_dir)
    assert_eq_int(len(processed_files), 3)
    
    // Verify content
    sus sample_output tea = read_file(output_dir + output_files[0])
    sus sample_data = parse_json(sample_output)
    
    assert_eq_bool(has_field(sample_data, "processed_at"), based)
    assert_eq_bool(has_field(sample_data, "age_group"), based)
    
    // Cleanup
    delete_directory(input_dir)
    delete_directory(output_dir)
    delete_directory(processed_dir)
    
    test_pass("File processing pipeline integration")
}

// Cryptographic workflow integration
slay test_crypto_workflow_integration() {
    // Generate key pair
    sus key_pair = generate_rsa_keypair(2048)
    sus public_key = get_public_key(key_pair)
    sus private_key = get_private_key(key_pair)
    
    // Original message
    sus message tea = "This is a secret message that needs to be encrypted and signed."
    
    // Hash the message
    sus message_hash tea = sha256(message)
    
    // Sign the hash
    sus signature tea = rsa_sign(private_key, message_hash)
    
    // Encrypt the message
    sus encrypted_message tea = rsa_encrypt(public_key, message)
    
    // Create a secure package
    sus secure_package = {
        "encrypted_data": encrypted_message,
        "signature": signature,
        "hash": message_hash,
        "timestamp": now_iso(),
        "algorithm": "RSA-2048-SHA256"
    }
    
    sus package_json tea = to_json(secure_package)
    
    // Save to file
    sus package_file tea = "secure_package.json"
    write_file(package_file, package_json)
    
    // Read and verify package
    sus read_package_json tea = read_file(package_file)
    sus read_package = parse_json(read_package_json)
    
    sus read_encrypted tea = get_field(read_package, "encrypted_data")
    sus read_signature tea = get_field(read_package, "signature")
    sus read_hash tea = get_field(read_package, "hash")
    
    // Decrypt the message
    sus decrypted_message tea = rsa_decrypt(private_key, read_encrypted)
    assert_eq_str(decrypted_message, message)
    
    // Verify signature
    sus is_valid lit = rsa_verify(public_key, read_hash, read_signature)
    assert_eq_bool(is_valid, based)
    
    // Verify hash
    sus computed_hash tea = sha256(decrypted_message)
    assert_eq_str(computed_hash, read_hash)
    
    // Cleanup
    delete_file(package_file)
    
    test_pass("Cryptographic workflow integration")
}

// Concurrent data processing integration
slay test_concurrent_processing_integration() {
    sus input_data []drip = []
    
    // Generate test data
    bestie (drip i = 0; i < 1000; i = i + 1) {
        append(input_data, i)
    }
    
    sus results chan<drip> = make_channel()
    sus errors chan<tea> = make_channel()
    sus done chan<lit> = make_channel()
    
    sus worker_count drip = 5
    sus chunk_size drip = len(input_data) / worker_count
    
    // Start workers
    bestie (drip worker_id = 0; worker_id < worker_count; worker_id = worker_id + 1) {
        go {
            sus start_idx drip = worker_id * chunk_size
            sus end_idx drip = ready (worker_id == worker_count - 1) { damn len(input_data) } otherwise { damn (worker_id + 1) * chunk_size }
            
            bestie (drip i = start_idx; i < end_idx; i = i + 1) {
                ready {
                    sus input drip = input_data[i]
                    
                    // Simulate complex processing
                    sus result drip = input * input + sqrt(input as lit) as drip
                    
                    // Add some randomness to simulate real work
                    sleep(random_int(1, 10))
                    
                    results <- result
                } fam {
                    when error -> {
                        errors <- "Worker " + (worker_id as tea) + " error: " + error
                    }
                }
            }
            
            done <- based
        }
    }
    
    // Collect results
    sus processed_results []drip = []
    sus error_list []tea = []
    sus workers_done drip = 0
    
    bestie (workers_done < worker_count) {
        sick {
            case results -> {
                sus result drip = <-results
                append(processed_results, result)
            }
            case errors -> {
                sus error tea = <-errors
                append(error_list, error)
            }
            case done -> {
                <-done
                workers_done = workers_done + 1
            }
        }
    }
    
    // Collect remaining results
    bestie (len(processed_results) + len(error_list) < len(input_data)) {
        sick {
            case results -> {
                sus result drip = <-results
                append(processed_results, result)
            }
            case errors -> {
                sus error tea = <-errors
                append(error_list, error)
            }
        }
    }
    
    assert_eq_int(len(processed_results) + len(error_list), len(input_data))
    
    // Verify some results
    ready (len(processed_results) > 0) {
        sus sample_result drip = processed_results[0]
        assert_eq_bool(sample_result > 0, based)
    }
    
    test_pass("Concurrent data processing integration")
}

// Microservice communication integration
slay test_microservice_integration() {
    // Service A: User management
    sus service_a_port drip = 8081
    sus service_a = create_http_server("localhost", service_a_port)
    
    go {
        handle_requests(service_a, slay(request) {
            sus path tea = get_request_path(request)
            
            ready (path == "/users" && get_request_method(request) == "GET") {
                sus users = [
                    {"id": 1, "name": "Alice", "email": "alice@example.com"},
                    {"id": 2, "name": "Bob", "email": "bob@example.com"}
                ]
                damn create_response(200, "application/json", to_json(users))
            }
            
            ready (starts_with(path, "/users/") && get_request_method(request) == "GET") {
                sus user_id_str tea = replace(path, "/users/", "")
                sus user_id drip = user_id_str as drip
                
                ready (user_id == 1) {
                    sus user = {"id": 1, "name": "Alice", "email": "alice@example.com"}
                    damn create_response(200, "application/json", to_json(user))
                }
                
                damn create_response(404, "application/json", "{\"error\": \"User not found\"}")
            }
            
            damn create_response(404, "text/plain", "Not Found")
        })
    }
    
    // Service B: Order management
    sus service_b_port drip = 8082
    sus service_b = create_http_server("localhost", service_b_port)
    
    go {
        handle_requests(service_b, slay(request) {
            sus path tea = get_request_path(request)
            
            ready (path == "/orders" && get_request_method(request) == "POST") {
                sus body tea = get_request_body(request)
                sus order_data = parse_json(body)
                
                sus user_id drip = get_field(order_data, "user_id") as drip
                
                // Call Service A to validate user
                sus user_response = http_get("http://localhost:" + (service_a_port as tea) + "/users/" + (user_id as tea))
                
                ready (user_response.status == 200) {
                    sus order_id tea = generate_uuid()
                    sus order = {
                        "id": order_id,
                        "user_id": user_id,
                        "items": get_field(order_data, "items"),
                        "total": get_field(order_data, "total"),
                        "created_at": now_iso()
                    }
                    damn create_response(201, "application/json", to_json(order))
                } otherwise {
                    damn create_response(400, "application/json", "{\"error\": \"Invalid user\"}")
                }
            }
            
            damn create_response(404, "text/plain", "Not Found")
        })
    }
    
    sleep(200)  // Let services start
    
    // Test service communication
    sus order_data = {
        "user_id": 1,
        "items": ["item1", "item2"],
        "total": 29.99
    }
    
    sus order_response = http_post(
        "http://localhost:" + (service_b_port as tea) + "/orders",
        to_json(order_data),
        "application/json"
    )
    
    assert_eq_int(order_response.status, 201)
    
    sus created_order = parse_json(order_response.body)
    assert_eq_int(get_field(created_order, "user_id") as drip, 1)
    assert_eq_int(len(get_field(created_order, "items")), 2)
    
    // Test with invalid user
    sus invalid_order_data = {
        "user_id": 999,
        "items": ["item1"],
        "total": 10.00
    }
    
    sus invalid_order_response = http_post(
        "http://localhost:" + (service_b_port as tea) + "/orders",
        to_json(invalid_order_data),
        "application/json"
    )
    
    assert_eq_int(invalid_order_response.status, 400)
    
    stop_server(service_a)
    stop_server(service_b)
    
    test_pass("Microservice communication integration")
}

// Run all integration tests
test_web_server_integration()
test_database_integration()
test_file_processing_pipeline()
test_crypto_workflow_integration()
test_concurrent_processing_integration()
test_microservice_integration()

print_test_summary()
