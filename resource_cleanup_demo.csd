fr fr Advanced resource cleanup demonstration using later/defer
yeet "testz"

fr fr Simulated resource management system
squad FileHandle {
    spill path tea
    spill is_open lit
}

squad DatabaseConnection {
    spill host tea
    spill is_connected lit
}

squad MemoryBuffer {
    spill size normie
    spill is_allocated lit
}

fr fr Global resource tracking
sus resources_opened []tea = []
sus resources_closed []tea = []

slay open_file(path tea) FileHandle {
    resources_opened.push("file:" + path)
    damn FileHandle{path: path, is_open: based}
}

slay close_file(handle FileHandle) {
    resources_closed.push("file:" + handle.path)
}

slay connect_database(host tea) DatabaseConnection {
    resources_opened.push("db:" + host)
    damn DatabaseConnection{host: host, is_connected: based}
}

slay disconnect_database(conn DatabaseConnection) {
    resources_closed.push("db:" + conn.host)
}

slay allocate_memory(size normie) MemoryBuffer {
    resources_opened.push("memory:" + tea(size))
    damn MemoryBuffer{size: size, is_allocated: based}
}

slay free_memory(buffer MemoryBuffer) {
    resources_closed.push("memory:" + tea(buffer.size))
}

test_start("Complex Resource Management with Later")

slay complex_operation() {
    fr fr Open multiple resources
    sus file1 = open_file("config.txt")
    later close_file(file1)
    
    sus file2 = open_file("data.log")
    later close_file(file2)
    
    sus db_conn = connect_database("localhost:5432")
    later disconnect_database(db_conn)
    
    sus memory = allocate_memory(1024)
    later free_memory(memory)
    
    fr fr Simulate some work
    vibez.spill("Processing data...")
    
    fr fr Resources will be cleaned up in reverse order when function exits
}

complex_operation()

fr fr Verify resources were opened and closed in correct order
assert_eq_int(resources_opened.len(), 4)
assert_eq_int(resources_closed.len(), 4)

fr fr Check opening order
assert_eq_string(resources_opened[0], "file:config.txt")
assert_eq_string(resources_opened[1], "file:data.log")
assert_eq_string(resources_opened[2], "db:localhost:5432")
assert_eq_string(resources_opened[3], "memory:1024")

fr fr Check closing order (reverse of opening due to LIFO defer execution)
assert_eq_string(resources_closed[0], "memory:1024")     fr fr Last opened, first closed
assert_eq_string(resources_closed[1], "db:localhost:5432")
assert_eq_string(resources_closed[2], "file:data.log")
assert_eq_string(resources_closed[3], "file:config.txt") fr fr First opened, last closed

test_start("Exception Safety with Later")

sus error_cleanup_called lit = cringe
sus successful_cleanup_called lit = cringe

slay risky_operation() {
    sus resource = open_file("risky.txt")
    later {
        close_file(resource)
        if resource.path == "risky.txt" {
            error_cleanup_called = based
        }
    }
    
    fr fr Simulate an error condition
    sus should_fail lit = based
    if should_fail {
        fr fr Even if we return early, defer will execute
        damn
    }
    
    fr fr This shouldn't execute
    successful_cleanup_called = based
}

risky_operation()

fr fr Verify cleanup happened despite early return
assert_true(error_cleanup_called)
assert_false(successful_cleanup_called)

test_start("Nested Function Later Scopes")

sus scope_log []tea = []

slay level_three() {
    scope_log.push("level3_start")
    later { scope_log.push("level3_defer") }
    scope_log.push("level3_end")
}

slay level_two() {
    scope_log.push("level2_start")
    later { scope_log.push("level2_defer") }
    level_three()
    scope_log.push("level2_middle")
    later { scope_log.push("level2_defer2") }
    scope_log.push("level2_end")
}

slay level_one() {
    scope_log.push("level1_start")
    later { scope_log.push("level1_defer") }
    level_two()
    scope_log.push("level1_end")
}

level_one()

fr fr Verify proper defer scope isolation
assert_eq_string(scope_log[0], "level1_start")
assert_eq_string(scope_log[1], "level2_start")
assert_eq_string(scope_log[2], "level3_start")
assert_eq_string(scope_log[3], "level3_end")
assert_eq_string(scope_log[4], "level3_defer")   fr fr level3 defer executes when level3 exits
assert_eq_string(scope_log[5], "level2_middle")
assert_eq_string(scope_log[6], "level2_end")
assert_eq_string(scope_log[7], "level2_defer2")  fr fr Last level2 defer first (LIFO)
assert_eq_string(scope_log[8], "level2_defer")   fr fr First level2 defer last (LIFO)
assert_eq_string(scope_log[9], "level1_end")
assert_eq_string(scope_log[10], "level1_defer")  fr fr level1 defer executes when level1 exits

print_test_summary()
