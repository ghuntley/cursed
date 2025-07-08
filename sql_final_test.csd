// Final SQL Slay test - inline module with simple tests
vibez.spill("🗄️  SQL Slay Module Test")
vibez.spill("=======================")

// Database connection state
sus db_active lit = cap
sus db_host tea = ""
sus db_port normie = 0

// Database functions
slay db_connect(host tea, port normie, db tea, user tea, pass tea) lit {
    db_active = based
    db_host = host
    db_port = port
    vibez.spill("Connected to " + host + ":" + port)
    damn based
}

slay db_is_connected() lit {
    damn db_active
}

slay sql_select(table tea, columns tea, where_clause tea) tea {
    sus query tea = "SELECT " + columns + " FROM " + table
    lowkey where_clause != "" {
        query = query + " WHERE " + where_clause
    }
    damn query
}

slay sql_execute(query tea) lit {
    lowkey db_active {
        vibez.spill("Executing: " + query)
        damn based
    }
    damn cap
}

// Test functions
vibez.spill("Testing database connection...")
sus connected lit = db_connect("localhost", 5432, "testdb", "user", "pass")
vibez.spill("Connection result: " + connected)

vibez.spill("Testing connection status...")
sus is_connected lit = db_is_connected()
vibez.spill("Is connected: " + is_connected)

vibez.spill("Testing SQL query building...")
sus query tea = sql_select("users", "*", "age > 18")
vibez.spill("Built query: " + query)

vibez.spill("Testing SQL execution...")
sus executed lit = sql_execute(query)
vibez.spill("Execution result: " + executed)

vibez.spill("✅ SQL Slay module test completed successfully!")
