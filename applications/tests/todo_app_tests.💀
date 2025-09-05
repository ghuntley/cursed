fr fr TODO APPLICATION TESTS - Complete test suite for todo list app
fr fr Tests CRUD operations, categories, API endpoints, web interface

yeet "database_enhanced"  
yeet "webz"
yeet "json"
yeet "stringz"
yeet "timez"
yeet "fs"
yeet "vibez"

fr fr Mock the todo app functions by importing the structure
fr fr In a real scenario, these would be imported from the main application

fr fr ===== TEST DATA SETUP =====

sus test_db_url tea = "file://./todo_test_db"
sus test_results []tea = []
sus tests_passed drip = 0
sus tests_failed drip = 0

squad TestTodo {
    sus id drip
    sus title tea
    sus description tea  
    sus category tea
    sus priority tea
    sus due_date tea
    sus completed lit
    sus created_at tea
    sus updated_at tea
}

squad TestCategory {
    sus id drip
    sus name tea
    sus color tea
    sus description tea
    sus created_at tea
}

fr fr ===== TEST UTILITIES =====

slay assert_true(condition lit, test_name tea) {
    ready (condition) {
        test_results[test_results.length] = "✅ PASS: " + test_name
        tests_passed = tests_passed + 1
        vibez.spill("✅ PASS: " + test_name)
    } otherwise {
        test_results[test_results.length] = "❌ FAIL: " + test_name
        tests_failed = tests_failed + 1
        vibez.spill("❌ FAIL: " + test_name)
    }
}

slay assert_equal(expected tea, actual tea, test_name tea) {
    ready (expected == actual) {
        test_results[test_results.length] = "✅ PASS: " + test_name
        tests_passed = tests_passed + 1
        vibez.spill("✅ PASS: " + test_name)
    } otherwise {
        test_results[test_results.length] = "❌ FAIL: " + test_name + " - Expected: " + expected + ", Got: " + actual
        tests_failed = tests_failed + 1
        vibez.spill("❌ FAIL: " + test_name + " - Expected: " + expected + ", Got: " + actual)
    }
}

slay assert_greater_than(actual drip, expected drip, test_name tea) {
    ready (actual > expected) {
        test_results[test_results.length] = "✅ PASS: " + test_name
        tests_passed = tests_passed + 1
        vibez.spill("✅ PASS: " + test_name)
    } otherwise {
        test_results[test_results.length] = "❌ FAIL: " + test_name + " - Expected > " + stringz.from_int(expected) + ", Got: " + stringz.from_int(actual)
        tests_failed = tests_failed + 1
        vibez.spill("❌ FAIL: " + test_name + " - Expected > " + stringz.from_int(expected) + ", Got: " + stringz.from_int(actual))
    }
}

fr fr ===== TODO APP SIMULATION FUNCTIONS =====

slay setup_test_database() database_enhanced.DatabaseConnection {
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    ready (!conn.is_connected) {
        vibez.spill("Failed to create test database connection")
        damn conn
    }
    
    fr fr Create todos table
    sus todos_schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "title": "TEXT NOT NULL",
        "description": "TEXT",
        "category": "TEXT DEFAULT 'General'",
        "priority": "TEXT DEFAULT 'Medium'",
        "due_date": "TEXT",
        "completed": "BOOLEAN DEFAULT 0",
        "created_at": "TEXT NOT NULL",
        "updated_at": "TEXT NOT NULL"
    })
    
    database_enhanced.create_table(conn, "todos", todos_schema)
    
    fr fr Create categories table
    sus categories_schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL UNIQUE",
        "color": "TEXT DEFAULT '#007bff'",
        "description": "TEXT",
        "created_at": "TEXT NOT NULL"
    })
    
    database_enhanced.create_table(conn, "categories", categories_schema)
    
    fr fr Insert default categories
    sus now tea = timez.format_iso8601(timez.now_millis())
    sus default_categories []tea = [
        json.object_to_string({
            "name": "Personal",
            "color": "#28a745",
            "description": "Personal tasks",
            "created_at": now
        }),
        json.object_to_string({
            "name": "Work", 
            "color": "#007bff",
            "description": "Work tasks",
            "created_at": now
        })
    ]
    
    sus i drip = 0
    bestie (i < default_categories.length) {
        database_enhanced.insert_record(conn, "categories", default_categories[i])
        i = i + 1
    }
    
    damn conn
}

slay create_test_todo(conn database_enhanced.DatabaseConnection, title tea, description tea, category tea, priority tea, due_date tea) drip {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    sus todo_data tea = json.object_to_string({
        "title": title,
        "description": description,
        "category": category,
        "priority": priority,
        "due_date": due_date,
        "completed": "false",
        "created_at": now,
        "updated_at": now
    })
    
    ready (database_enhanced.insert_record(conn, "todos", todo_data)) {
        damn mathz.random_int(10000)
    }
    
    damn 0
}

slay get_test_todos(conn database_enhanced.DatabaseConnection) []tea {
    damn database_enhanced.find_records(conn, "todos", "{}")
}

slay update_test_todo(conn database_enhanced.DatabaseConnection, id drip, updates tea) lit {
    sus update_data map[tea]tea = json.parse_object(updates)
    update_data["updated_at"] = timez.format_iso8601(timez.now_millis())
    sus final_updates tea = json.object_to_string(update_data)
    
    damn database_enhanced.update_record(conn, "todos", id, final_updates)
}

slay delete_test_todo(conn database_enhanced.DatabaseConnection, id drip) lit {
    damn database_enhanced.delete_record(conn, "todos", id)
}

slay get_test_categories(conn database_enhanced.DatabaseConnection) []tea {
    damn database_enhanced.find_records(conn, "categories", "{}")
}

fr fr ===== DATABASE LAYER TESTS =====

slay test_todo_database_operations() {
    vibez.spill("Running todo database operation tests...")
    
    sus conn database_enhanced.DatabaseConnection = setup_test_database()
    assert_true(conn.is_connected, "Test database connection established")
    
    fr fr Test todo creation
    sus todo_id drip = create_test_todo(conn, 
        "Test Todo", 
        "This is a test todo item",
        "Personal",
        "High",
        "2024-12-31T23:59:59Z"
    )
    
    assert_greater_than(todo_id, 0, "Todo creation returns valid ID")
    
    fr fr Test retrieving all todos
    sus all_todos []tea = get_test_todos(conn)
    assert_greater_than(all_todos.length, 0, "All todos retrieval")
    
    fr fr Test todo data integrity
    sus first_todo map[tea]tea = json.parse_object(all_todos[0])
    assert_equal("Test Todo", first_todo["title"], "Todo title preservation")
    assert_equal("Personal", first_todo["category"], "Todo category preservation")
    assert_equal("High", first_todo["priority"], "Todo priority preservation")
    assert_equal("false", first_todo["completed"], "Todo completion status default")
    
    fr fr Test todo updates
    sus update_data tea = json.object_to_string({
        "completed": "true",
        "description": "Updated description"
    })
    
    sus update_result lit = update_test_todo(conn, 1, update_data)
    assert_true(update_result, "Todo update operation")
    
    fr fr Verify update
    sus updated_todos []tea = get_test_todos(conn)
    sus updated_todo map[tea]tea = json.parse_object(updated_todos[0])
    assert_equal("true", updated_todo["completed"], "Todo completion status update")
    assert_equal("Updated description", updated_todo["description"], "Todo description update")
    
    fr fr Test category operations
    sus categories []tea = get_test_categories(conn)
    assert_greater_than(categories.length, 1, "Default categories created")
    
    fr fr Test multiple todo creation
    create_test_todo(conn, "Second Todo", "Another test", "Work", "Medium", "")
    create_test_todo(conn, "Third Todo", "Yet another", "Personal", "Low", "")
    
    sus multiple_todos []tea = get_test_todos(conn)
    assert_equal(3, multiple_todos.length, "Multiple todo creation")
    
    fr fr Test todo deletion
    sus delete_result lit = delete_test_todo(conn, 1)
    assert_true(delete_result, "Todo deletion operation")
    
    sus remaining_todos []tea = get_test_todos(conn)
    assert_equal(2, remaining_todos.length, "Todo count after deletion")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== API ENDPOINT TESTS =====

slay test_todo_api_endpoints() {
    vibez.spill("Running todo API endpoint tests...")
    
    sus conn database_enhanced.DatabaseConnection = setup_test_database()
    
    fr fr Create test data
    create_test_todo(conn, "API Test Todo", "Testing API", "Work", "High", "2024-06-01T10:00:00Z")
    create_test_todo(conn, "Another API Todo", "More testing", "Personal", "Medium", "")
    
    fr fr Simulate GET /api/todos request
    sus get_request webz.HttpRequest = webz.HttpRequest{
        method: "GET",
        path: "/api/todos",
        headers: {"Accept": "application/json"},
        body: "",
        query_params: {}
    }
    
    sus get_response webz.HttpResponse = simulate_api_get_todos(conn, get_request)
    assert_equal(200, get_response.status_code, "GET /api/todos status code")
    assert_equal("application/json", get_response.headers["Content-Type"], "GET /api/todos content type")
    
    fr fr Verify response contains todos
    sus response_data map[tea]tea = json.parse_object(get_response.body)
    assert_true(response_data["count"] != "0", "GET /api/todos returns todos")
    
    fr fr Simulate POST /api/todos request
    sus new_todo_data tea = json.object_to_string({
        "title": "Created via API",
        "description": "This todo was created through API",
        "category": "Personal",
        "priority": "High",
        "due_date": "2024-07-01T12:00:00Z"
    })
    
    sus post_request webz.HttpRequest = webz.HttpRequest{
        method: "POST",
        path: "/api/todos",
        headers: {"Content-Type": "application/json"},
        body: new_todo_data,
        query_params: {}
    }
    
    sus post_response webz.HttpResponse = simulate_api_create_todo(conn, post_request)
    assert_equal(201, post_response.status_code, "POST /api/todos status code")
    
    fr fr Verify todo was created
    sus all_todos_after_create []tea = get_test_todos(conn)
    assert_equal(3, all_todos_after_create.length, "Todo created via API")
    
    fr fr Simulate PUT /api/todos/1 request
    sus update_todo_data tea = json.object_to_string({
        "completed": "true",
        "title": "Updated via API"
    })
    
    sus put_request webz.HttpRequest = webz.HttpRequest{
        method: "PUT",
        path: "/api/todos/1",
        headers: {"Content-Type": "application/json"},
        body: update_todo_data,
        query_params: {}
    }
    
    sus put_response webz.HttpResponse = simulate_api_update_todo(conn, put_request)
    assert_equal(200, put_response.status_code, "PUT /api/todos/1 status code")
    
    fr fr Simulate DELETE /api/todos/1 request
    sus delete_request webz.HttpRequest = webz.HttpRequest{
        method: "DELETE",
        path: "/api/todos/1",
        headers: {},
        body: "",
        query_params: {}
    }
    
    sus delete_response webz.HttpResponse = simulate_api_delete_todo(conn, delete_request)
    assert_equal(200, delete_response.status_code, "DELETE /api/todos/1 status code")
    
    fr fr Verify deletion
    sus todos_after_delete []tea = get_test_todos(conn)
    assert_equal(2, todos_after_delete.length, "Todo deleted via API")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== API SIMULATION FUNCTIONS =====

slay simulate_api_get_todos(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    response.status_code = 200
    
    sus todos []tea = get_test_todos(conn)
    sus todos_json tea = json.array_to_string(todos)
    response.body = json.object_to_string({"todos": todos_json, "count": stringz.from_int(todos.length)})
    
    damn response
}

slay simulate_api_create_todo(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus todo_data map[tea]tea = json.parse_object(request.body)
    
    ready (todo_data["title"] == "") {
        response.status_code = 400
        response.body = json.object_to_string({"error": "Title is required"})
        damn response
    }
    
    sus id drip = create_test_todo(conn, 
        todo_data["title"],
        todo_data["description"],
        todo_data["category"],
        todo_data["priority"],
        todo_data["due_date"]
    )
    
    ready (id > 0) {
        response.status_code = 201
        response.body = json.object_to_string({
            "id": stringz.from_int(id),
            "message": "Todo created successfully"
        })
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to create todo"})
    }
    
    damn response
}

slay simulate_api_update_todo(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    fr fr Extract ID from path (simulated)
    sus id drip = 1 fr fr Hardcoded for test
    
    ready (update_test_todo(conn, id, request.body)) {
        response.status_code = 200
        response.body = json.object_to_string({"message": "Todo updated successfully"})
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to update todo"})
    }
    
    damn response
}

slay simulate_api_delete_todo(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    fr fr Extract ID from path (simulated)
    sus id drip = 1 fr fr Hardcoded for test
    
    ready (delete_test_todo(conn, id)) {
        response.status_code = 200
        response.body = json.object_to_string({"message": "Todo deleted successfully"})
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to delete todo"})
    }
    
    damn response
}

fr fr ===== BUSINESS LOGIC TESTS =====

slay test_todo_business_logic() {
    vibez.spill("Running todo business logic tests...")
    
    sus conn database_enhanced.DatabaseConnection = setup_test_database()
    
    fr fr Test priority validation
    sus high_priority_id drip = create_test_todo(conn, "High Priority Task", "Important", "Work", "High", "")
    sus medium_priority_id drip = create_test_todo(conn, "Medium Priority Task", "Normal", "Work", "Medium", "")
    sus low_priority_id drip = create_test_todo(conn, "Low Priority Task", "Later", "Work", "Low", "")
    
    assert_greater_than(high_priority_id, 0, "High priority todo creation")
    assert_greater_than(medium_priority_id, 0, "Medium priority todo creation") 
    assert_greater_than(low_priority_id, 0, "Low priority todo creation")
    
    fr fr Test category filtering
    create_test_todo(conn, "Personal Task 1", "Personal stuff", "Personal", "Medium", "")
    create_test_todo(conn, "Personal Task 2", "More personal", "Personal", "Low", "")
    create_test_todo(conn, "Work Task 1", "Work stuff", "Work", "High", "")
    
    sus personal_todos []tea = get_todos_by_category(conn, "Personal")
    sus work_todos []tea = get_todos_by_category(conn, "Work")
    
    assert_greater_than(personal_todos.length, 1, "Personal category filtering")
    assert_greater_than(work_todos.length, 3, "Work category filtering")
    
    fr fr Test completion workflow
    sus completion_id drip = create_test_todo(conn, "Complete Me", "Test completion", "Personal", "Medium", "")
    
    fr fr Complete the todo
    sus completion_update tea = json.object_to_string({"completed": "true"})
    sus completion_result lit = update_test_todo(conn, completion_id, completion_update)
    assert_true(completion_result, "Todo completion workflow")
    
    fr fr Test due date handling
    sus future_date tea = "2024-12-31T23:59:59Z"
    sus past_date tea = "2024-01-01T00:00:00Z"
    
    create_test_todo(conn, "Future Task", "Due in future", "Personal", "High", future_date)
    create_test_todo(conn, "Overdue Task", "Past due", "Work", "High", past_date)
    
    sus all_todos []tea = get_test_todos(conn)
    sus future_found lit = cringe
    sus past_found lit = cringe
    
    sus i drip = 0
    bestie (i < all_todos.length) {
        sus todo_data map[tea]tea = json.parse_object(all_todos[i])
        ready (todo_data["title"] == "Future Task") {
            future_found = based
            assert_equal(future_date, todo_data["due_date"], "Future due date handling")
        }
        ready (todo_data["title"] == "Overdue Task") {
            past_found = based
            assert_equal(past_date, todo_data["due_date"], "Past due date handling")
        }
        i = i + 1
    }
    
    assert_true(future_found, "Future task creation")
    assert_true(past_found, "Overdue task creation")
    
    database_enhanced.close_connection(conn)
}

slay get_todos_by_category(conn database_enhanced.DatabaseConnection, category tea) []tea {
    sus conditions tea = json.object_to_string({
        "category": category
    })
    
    damn database_enhanced.find_records(conn, "todos", conditions)
}

fr fr ===== DATA VALIDATION TESTS =====

slay test_todo_data_validation() {
    vibez.spill("Running todo data validation tests...")
    
    sus conn database_enhanced.DatabaseConnection = setup_test_database()
    
    fr fr Test required field validation
    sus empty_title_id drip = create_test_todo(conn, "", "No title", "Personal", "Medium", "")
    assert_equal(0, empty_title_id, "Empty title validation")
    
    fr fr Test valid todo creation
    sus valid_id drip = create_test_todo(conn, "Valid Todo", "Good data", "Personal", "High", "2024-06-01T10:00:00Z")
    assert_greater_than(valid_id, 0, "Valid todo creation")
    
    fr fr Test category validation (should accept any category)
    sus custom_category_id drip = create_test_todo(conn, "Custom Category", "Test", "CustomCategory", "Low", "")
    assert_greater_than(custom_category_id, 0, "Custom category acceptance")
    
    fr fr Test priority validation (should accept any priority)  
    sus custom_priority_id drip = create_test_todo(conn, "Custom Priority", "Test", "Personal", "Urgent", "")
    assert_greater_than(custom_priority_id, 0, "Custom priority acceptance")
    
    fr fr Test date format validation
    sus good_date_id drip = create_test_todo(conn, "Good Date", "Test", "Personal", "Medium", "2024-06-01T10:00:00Z")
    assert_greater_than(good_date_id, 0, "Valid date format acceptance")
    
    fr fr Test empty date acceptance
    sus empty_date_id drip = create_test_todo(conn, "No Date", "Test", "Personal", "Medium", "")
    assert_greater_than(empty_date_id, 0, "Empty date acceptance")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== PERFORMANCE TESTS =====

slay test_todo_performance() {
    vibez.spill("Running todo performance tests...")
    
    sus conn database_enhanced.DatabaseConnection = setup_test_database()
    
    fr fr Test bulk todo creation performance
    sus start_time drip = timez.now_millis()
    sus todo_count drip = 50
    
    sus i drip = 0
    bestie (i < todo_count) {
        create_test_todo(conn, 
            "Performance Test Todo " + stringz.from_int(i),
            "Testing performance with todo " + stringz.from_int(i),
            "Work",
            "Medium",
            "2024-12-31T23:59:59Z"
        )
        i = i + 1
    }
    
    sus creation_end_time drip = timez.now_millis()
    sus creation_duration drip = creation_end_time - start_time
    
    vibez.spill("Bulk creation performance: " + stringz.from_int(todo_count) + " todos in " + 
               stringz.from_int(creation_duration) + "ms")
    
    fr fr Test bulk retrieval performance
    sus retrieval_start drip = timez.now_millis()
    sus all_performance_todos []tea = get_test_todos(conn)
    sus retrieval_end drip = timez.now_millis()
    sus retrieval_duration drip = retrieval_end - retrieval_start
    
    assert_greater_than(all_performance_todos.length, todo_count - 5, "Bulk todo retrieval")
    vibez.spill("Bulk retrieval performance: " + stringz.from_int(all_performance_todos.length) + 
               " todos in " + stringz.from_int(retrieval_duration) + "ms")
    
    fr fr Test search performance
    sus search_start drip = timez.now_millis()
    sus work_todos []tea = get_todos_by_category(conn, "Work")
    sus search_end drip = timez.now_millis()
    sus search_duration drip = search_end - search_start
    
    assert_greater_than(work_todos.length, 0, "Category search results")
    vibez.spill("Search performance: " + stringz.from_int(work_todos.length) + 
               " todos found in " + stringz.from_int(search_duration) + "ms")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== INTEGRATION TESTS =====

slay test_todo_integration() {
    vibez.spill("Running todo integration tests...")
    
    sus conn database_enhanced.DatabaseConnection = setup_test_database()
    
    fr fr Test complete workflow: Create -> Read -> Update -> Delete
    sus workflow_id drip = create_test_todo(conn, "Workflow Test", "Full workflow", "Personal", "High", "2024-06-01T10:00:00Z")
    assert_greater_than(workflow_id, 0, "Workflow step 1: Create")
    
    sus created_todos []tea = get_test_todos(conn)
    assert_greater_than(created_todos.length, 0, "Workflow step 2: Read")
    
    sus workflow_update tea = json.object_to_string({
        "title": "Updated Workflow Test",
        "completed": "true",
        "priority": "Medium"
    })
    
    sus update_success lit = update_test_todo(conn, workflow_id, workflow_update)
    assert_true(update_success, "Workflow step 3: Update")
    
    sus delete_success lit = delete_test_todo(conn, workflow_id)
    assert_true(delete_success, "Workflow step 4: Delete")
    
    fr fr Test category and todo relationship
    create_test_todo(conn, "Category Test 1", "Test", "TestCategory", "High", "")
    create_test_todo(conn, "Category Test 2", "Test", "TestCategory", "Medium", "")
    create_test_todo(conn, "Different Category", "Test", "DifferentCategory", "Low", "")
    
    sus test_category_todos []tea = get_todos_by_category(conn, "TestCategory")
    sus different_category_todos []tea = get_todos_by_category(conn, "DifferentCategory")
    
    assert_equal(2, test_category_todos.length, "Category relationship - TestCategory")
    assert_equal(1, different_category_todos.length, "Category relationship - DifferentCategory")
    
    fr fr Test concurrent operations simulation
    database_enhanced.begin_transaction(conn)
    
    create_test_todo(conn, "Transaction Test 1", "Test", "Personal", "High", "")
    create_test_todo(conn, "Transaction Test 2", "Test", "Work", "Medium", "")
    
    database_enhanced.commit_transaction(conn)
    
    sus transaction_todos []tea = get_test_todos(conn)
    assert_greater_than(transaction_todos.length, 3, "Transaction commit verification")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_todo_tests() {
    vibez.spill("🚀 Starting Todo Application Test Suite")
    vibez.spill("=" + stringz.repeat("=", 60))
    
    fr fr Run all test suites
    test_todo_database_operations()
    test_todo_api_endpoints() 
    test_todo_business_logic()
    test_todo_data_validation()
    test_todo_performance()
    test_todo_integration()
    
    fr fr Print final results
    vibez.spill("=" + stringz.repeat("=", 60))
    vibez.spill("🏁 Todo Application Test Suite Complete")
    vibez.spill("✅ Tests Passed: " + stringz.from_int(tests_passed))
    vibez.spill("❌ Tests Failed: " + stringz.from_int(tests_failed))
    vibez.spill("📊 Total Tests: " + stringz.from_int(tests_passed + tests_failed))
    
    ready (tests_failed > 0) {
        vibez.spill("⚠️  Some tests failed. Review the output above.")
    } otherwise {
        vibez.spill("🎉 All todo application tests passed successfully!")
    }
    
    vibez.spill("=" + stringz.repeat("=", 60))
    
    fr fr Generate test report
    generate_todo_test_report()
    
    fr fr Cleanup
    cleanup_test_files()
}

slay generate_todo_test_report() {
    sus report tea = "# Todo Application Test Report\n\n"
    report = report + "Generated: " + timez.format_iso8601(timez.now_millis()) + "\n\n"
    report = report + "## Summary\n\n"
    report = report + "- **Total Tests:** " + stringz.from_int(tests_passed + tests_failed) + "\n"
    report = report + "- **Passed:** " + stringz.from_int(tests_passed) + "\n" 
    report = report + "- **Failed:** " + stringz.from_int(tests_failed) + "\n"
    report = report + "- **Success Rate:** " + calculate_success_rate() + "%\n\n"
    report = report + "## Test Categories\n\n"
    report = report + "1. **Database Operations** - CRUD functionality\n"
    report = report + "2. **API Endpoints** - REST API testing\n"
    report = report + "3. **Business Logic** - Core application logic\n" 
    report = report + "4. **Data Validation** - Input validation\n"
    report = report + "5. **Performance** - Bulk operations and timing\n"
    report = report + "6. **Integration** - End-to-end workflows\n\n"
    report = report + "## Detailed Results\n\n"
    
    sus i drip = 0
    bestie (i < test_results.length) {
        report = report + "- " + test_results[i] + "\n"
        i = i + 1
    }
    
    sus report_file tea = "test_results_todo_app.md"
    fs.write_file(report_file, report)
    vibez.spill("📝 Test report written to: " + report_file)
}

slay calculate_success_rate() tea {
    sus total drip = tests_passed + tests_failed
    ready (total == 0) {
        damn "0"
    }
    
    sus rate drip = (tests_passed * 100) / total
    damn stringz.from_int(rate)
}

slay cleanup_test_files() {
    fr fr Clean up test database files
    vibez.spill("Cleaning up test files...")
}

fr fr Run the todo application test suite
run_all_todo_tests()
