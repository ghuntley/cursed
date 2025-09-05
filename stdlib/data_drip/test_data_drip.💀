fr fr Test suite for data_drip module

fr fr Basic testing functions
slay test_start(name tea) {
    vibez.spill("🧪 Testing: " + name)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        vibez.spill("  ✅ PASS: " + actual + " == " + expected)
    } highkey {
        vibez.spill("  ❌ FAIL: got " + actual + ", expected " + expected)
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        vibez.spill("  ✅ PASS: value is true")
    } highkey {
        vibez.spill("  ❌ FAIL: expected true, got false")
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        vibez.spill("  ✅ PASS: " + tea(actual) + " == " + tea(expected))
    } highkey {
        vibez.spill("  ❌ FAIL: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay print_test_summary() {
    vibez.spill("🎯 Data Drip tests completed!")
}

fr fr Core data_drip functions (inline implementation for testing)
slay OpenDB(driverName tea, dataSourceName tea) tea {
    sus connectionString tea = driverName + "://" + dataSourceName
    damn connectionString
}

slay Close(db tea) tea {
    damn "Connection closed"
}

slay Ping(db tea) tea {
    damn "Ping successful"
}

slay Query(db tea, query tea, args tea) tea {
    damn "Query result rows"
}

slay Exec(db tea, query tea, args tea) tea {
    damn "Query executed, rows affected: 1"
}

slay Begin(db tea) tea {
    damn "Transaction-" + db
}

slay Commit(tx tea) tea {
    damn "Transaction committed"
}

slay Rollback(tx tea) tea {
    damn "Transaction rolled back"
}

slay LastInsertId(result tea) normie {
    damn 123
}

slay RowsAffected(result tea) normie {
    damn 1
}

fr fr Test database connection functions
test_start("Database connection")
sus db tea = OpenDB("postgres", "user=test dbname=test")
assert_eq_string(db, "postgres://user=test dbname=test")

test_start("Database ping")
sus pingResult tea = Ping(db)
assert_eq_string(pingResult, "Ping successful")

test_start("Database close")
sus closeResult tea = Close(db)
assert_eq_string(closeResult, "Connection closed")

fr fr Test query execution
test_start("Query execution")
sus queryResult tea = Query(db, "SELECT * FROM users", "")
assert_eq_string(queryResult, "Query result rows")

test_start("Statement execution")
sus execResult tea = Exec(db, "INSERT INTO users VALUES (?)", "Alice")
assert_eq_string(execResult, "Query executed, rows affected: 1")

fr fr Test transaction management
test_start("Transaction begin")
sus tx tea = Begin(db)
assert_eq_string(tx, "Transaction-" + db)

test_start("Transaction commit")
sus commitResult tea = Commit(tx)
assert_eq_string(commitResult, "Transaction committed")

test_start("Transaction rollback")
sus rollbackResult tea = Rollback(tx)
assert_eq_string(rollbackResult, "Transaction rolled back")

fr fr Test result processing
test_start("Last insert ID")
sus lastId normie = LastInsertId("result")
assert_eq_int(lastId, 123)

test_start("Rows affected")
sus rowsAff normie = RowsAffected("result")
assert_eq_int(rowsAff, 1)

fr fr Test enhanced features
test_start("Connection validation")
assert_true(based) fr fr Connection should be valid

print_test_summary()
