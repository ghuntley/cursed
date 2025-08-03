yeet "testz"

fr fr Advanced Defer Statement Implementation Test
fr fr Tests all enhanced defer features including:
fr fr - Proper LIFO execution order
fr fr - Resource cleanup with exceptions
fr fr - Nested scope handling
fr fr - Integration with panic recovery
fr fr - Performance optimizations

test_start("Advanced Defer Implementation - LIFO Order")

fr fr Test basic defer LIFO execution order
slay test_defer_lifo_order() tea {
    sus order := ""
    
    later {
        order = order + "3"
    }
    
    later {
        order = order + "2"
    }
    
    later {
        order = order + "1"
    }
    
    order = order + "0"
    
    damn order  fr fr Should be "0123" due to LIFO execution
}

assert_eq_string(test_defer_lifo_order(), "0123")

fr fr Test defer with resource management
struct File {
    name tea
    handle drip
    is_open lit
}

impl File {
    slay open(filename tea) File {
        damn File { name: filename, handle: 42, is_open: based }
    }
    
    slay close() {
        is_open = cringe
        vibez.spill(format("Closing file: {}", name))
    }
    
    slay write(data tea) {
        vibez.spill(format("Writing to {}: {}", name, data))
    }
}

slay test_defer_resource_cleanup() tea {
    sus result := ""
    
    {
        sus file := File::open("test.txt")
        
        later {
            file.close()
            result = result + "closed "
        }
        
        file.write("hello world")
        result = result + "written "
        
        fr fr File should be automatically closed when scope exits
    }
    
    result = result + "done"
    damn result
}

assert_eq_string(test_defer_resource_cleanup(), "written closed done")

fr fr Test defer with nested scopes
slay test_defer_nested_scopes() tea {
    sus sequence := ""
    
    later {
        sequence = sequence + "outer-end "
    }
    
    {
        later {
            sequence = sequence + "inner1-end "
        }
        
        sequence = sequence + "inner1-start "
        
        {
            later {
                sequence = sequence + "inner2-end "
            }
            
            sequence = sequence + "inner2-start "
        }
        
        sequence = sequence + "inner1-middle "
    }
    
    sequence = sequence + "outer-middle "
    
    damn sequence
}

fr fr Expected order: inner1-start inner2-start inner2-end inner1-middle inner1-end outer-middle outer-end
assert_eq_string(test_defer_nested_scopes(), 
    "inner1-start inner2-start inner2-end inner1-middle inner1-end outer-middle outer-end ")

fr fr Test defer with early returns
slay test_defer_early_return(should_return lit) tea {
    sus cleanup_called := ""
    
    later {
        cleanup_called = cleanup_called + "cleanup1 "
    }
    
    if should_return {
        later {
            cleanup_called = cleanup_called + "early-cleanup "
        }
        
        damn cleanup_called + "early-return"
    }
    
    later {
        cleanup_called = cleanup_called + "cleanup2 "
    }
    
    damn cleanup_called + "normal-return"
}

fr fr Test early return path
assert_eq_string(test_defer_early_return(based), "early-cleanup cleanup1 early-return")

fr fr Test normal return path  
assert_eq_string(test_defer_early_return(cringe), "cleanup2 cleanup1 normal-return")

fr fr Test defer with exception handling
slay test_defer_with_panics() tea {
    sus cleanup_status := ""
    
    sus result := try {
        later {
            cleanup_status = cleanup_status + "panic-cleanup "
        }
        
        later {
            cleanup_status = cleanup_status + "normal-cleanup "
        }
        
        fr fr Simulate an error condition
        if based {
            panic("test panic")
        }
        
        "success"
    } catch (e) {
        cleanup_status + "caught-panic"
    }
    
    damn result
}

assert_eq_string(test_defer_with_panics(), "normal-cleanup panic-cleanup caught-panic")

fr fr Test defer with multiple error paths
slay test_defer_multiple_paths(path drip) tea {
    sus trace := ""
    
    later {
        trace = trace + "final "
    }
    
    if path == 1 {
        later {
            trace = trace + "path1-cleanup "
        }
        damn trace + "path1"
    } else if path == 2 {
        later {
            trace = trace + "path2-cleanup "
        }
        damn trace + "path2"
    }
    
    later {
        trace = trace + "default-cleanup "
    }
    
    damn trace + "default"
}

assert_eq_string(test_defer_multiple_paths(1), "path1-cleanup final path1")
assert_eq_string(test_defer_multiple_paths(2), "path2-cleanup final path2")
assert_eq_string(test_defer_multiple_paths(3), "default-cleanup final default")

fr fr Test defer with complex resource hierarchies
struct Database {
    name tea
    connections drip
    is_connected lit
}

struct Transaction {
    db Database
    id drip
    is_active lit
}

impl Database {
    slay connect(name tea) Database {
        damn Database { name: name, connections: 1, is_connected: based }
    }
    
    slay disconnect() {
        is_connected = cringe
        connections = 0
    }
    
    slay begin_transaction() Transaction {
        damn Transaction { db: self, id: 123, is_active: based }
    }
}

impl Transaction {
    slay commit() {
        is_active = cringe
        vibez.spill("Transaction committed")
    }
    
    slay rollback() {
        is_active = cringe
        vibez.spill("Transaction rolled back")
    }
}

slay test_defer_resource_hierarchy() tea {
    sus status := ""
    
    {
        sus db := Database::connect("testdb")
        later {
            db.disconnect()
            status = status + "db-disconnected "
        }
        
        {
            sus tx := db.begin_transaction()
            later {
                if tx.is_active {
                    tx.rollback()
                    status = status + "tx-rollback "
                } else {
                    status = status + "tx-already-finished "
                }
            }
            
            fr fr Do some work
            status = status + "work-done "
            
            fr fr Commit transaction
            tx.commit()
            status = status + "tx-committed "
        }
        
        status = status + "tx-scope-ended "
    }
    
    status = status + "db-scope-ended"
    damn status
}

assert_eq_string(test_defer_resource_hierarchy(), 
    "work-done tx-committed tx-already-finished tx-scope-ended db-disconnected db-scope-ended")

fr fr Test defer performance with many defers
slay test_defer_performance() normie {
    sus counter := 0
    
    fr fr Create many defer statements to test performance
    for i in 0..100 {
        later {
            counter = counter + 1
        }
    }
    
    damn counter
}

assert_eq_int(test_defer_performance(), 100)

fr fr Test defer with async operations (if supported)
slay test_defer_async_cleanup() tea {
    sus status := ""
    
    later {
        status = status + "async-cleanup "
    }
    
    fr fr Simulate async work
    status = status + "async-work "
    
    damn status
}

assert_eq_string(test_defer_async_cleanup(), "async-work async-cleanup ")

print_test_summary()
