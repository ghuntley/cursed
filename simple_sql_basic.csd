// Simple test without imports to verify sql_slay module
vibez.spill("Testing SQL Slay module...")

// Test basic connection
sus connected lit = db_connect("localhost", 5432, "testdb", "user", "pass")
vibez.spill("Connected: " + connected)

// Test is_connected
sus is_conn lit = db_is_connected()
vibez.spill("Is connected: " + is_conn)

// Test query building
sus query tea = sql_select("users", "*", "age > 18")
vibez.spill("Query: " + query)

// Test disconnect
sus disconnected lit = db_disconnect()
vibez.spill("Disconnected: " + disconnected)

vibez.spill("SQL Slay module basic test completed!")
