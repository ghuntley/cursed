yeet "stringz"
yeet "collections"

# PostgreSQL specific functionality
be_like PostgresConfig = {
    host tea
    port normie
    database tea
    username tea
    password tea
    sslmode tea
    connect_timeout normie
    application_name tea
}

# PostgreSQL connection management
slay postgres_create_config(
    host tea,
    port normie,
    database tea,
    username tea,
    password tea
) PostgresConfig {
    sus config PostgresConfig = {
        host: host,
        port: port,
        database: database,
        username: username,
        password: password,
        sslmode: "prefer",
        connect_timeout: 30,
        application_name: "cursed_app"
    }
    damn config
}

slay postgres_connection_string(config PostgresConfig) tea {
    sus conn_string tea = stringz.format(
        "postgresql://{}:{}@{}:{}/{}?sslmode={}&connect_timeout={}&application_name={}",
        config.username,
        config.password,
        config.host,
        config.port,
        config.database,
        config.sslmode,
        config.connect_timeout,
        config.application_name
    )
    damn conn_string
}

# PostgreSQL specific data types
slay postgres_format_value(value tea, data_type tea) tea {
    ready data_type {
        "integer" -> {
            damn value
        }
        "text" -> {
            damn "'" + stringz.replace(value, "'", "''") + "'"
        }
        "boolean" -> {
            ready value {
                "true" -> damn "TRUE"
                "false" -> damn "FALSE"
                basic -> damn "NULL"
            }
        }
        "timestamp" -> {
            damn "'" + value + "'"
        }
        "json" -> {
            damn "'" + stringz.replace(value, "'", "''") + "'::json"
        }
        "uuid" -> {
            damn "'" + value + "'::uuid"
        }
        basic -> {
            damn "'" + stringz.replace(value, "'", "''") + "'"
        }
    }
}

# PostgreSQL specific queries
slay postgres_create_table(table_name tea, columns []tea) tea {
    sus query tea = stringz.format("CREATE TABLE IF NOT EXISTS {} (", table_name)
    query = query + stringz.join(columns, ", ")
    query = query + ")"
    damn query
}

slay postgres_add_index(table_name tea, index_name tea, columns []tea, unique lit) tea {
    sus query tea = ""
    yikes unique {
        query = "CREATE UNIQUE INDEX "
    } shook {
        query = "CREATE INDEX "
    }
    
    query = query + stringz.format("{} ON {} ({})", 
        index_name, table_name, stringz.join(columns, ", "))
    damn query
}

slay postgres_upsert_query(table_name tea, columns []tea, conflict_columns []tea) tea {
    sus placeholders []tea = []
    bestie i := 0; i < columns.length; i++ {
        placeholders.append(stringz.format("${}", i + 1))
    }
    
    sus query tea = stringz.format(
        "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET ",
        table_name,
        stringz.join(columns, ", "),
        stringz.join(placeholders, ", "),
        stringz.join(conflict_columns, ", ")
    )
    
    sus updates []tea = []
    bestie i := 0; i < columns.length; i++ {
        yikes !contains_string(conflict_columns, columns[i]) {
            updates.append(stringz.format("{} = EXCLUDED.{}", columns[i], columns[i]))
        }
    }
    
    query = query + stringz.join(updates, ", ")
    damn query
}

# PostgreSQL array operations
slay postgres_array_contains(column tea, value tea) tea {
    damn stringz.format("{} @> ARRAY[{}]", column, value)
}

slay postgres_array_overlap(column tea, values []tea) tea {
    sus array_literal tea = "ARRAY[" + stringz.join(values, ", ") + "]"
    damn stringz.format("{} && {}", column, array_literal)
}

# PostgreSQL JSON operations
slay postgres_json_extract(column tea, path tea) tea {
    damn stringz.format("{}->'{}'", column, path)
}

slay postgres_json_extract_text(column tea, path tea) tea {
    damn stringz.format("{}->>'{}'", column, path)
}

slay postgres_json_path_exists(column tea, path tea) tea {
    damn stringz.format("{} ? '{}'", column, path)
}

# PostgreSQL full-text search
slay postgres_to_tsvector(config tea, text tea) tea {
    damn stringz.format("to_tsvector('{}', {})", config, text)
}

slay postgres_to_tsquery(config tea, query tea) tea {
    damn stringz.format("to_tsquery('{}', {})", config, query)
}

slay postgres_full_text_search(column tea, search_query tea, config tea) tea {
    damn stringz.format("{} @@ to_tsquery('{}', {})", column, config, search_query)
}

# PostgreSQL window functions
slay postgres_row_number(partition_by []tea, order_by []tea) tea {
    sus query tea = "ROW_NUMBER() OVER ("
    
    yikes partition_by.length > 0 {
        query = query + "PARTITION BY " + stringz.join(partition_by, ", ")
    }
    
    yikes order_by.length > 0 {
        yikes partition_by.length > 0 {
            query = query + " "
        }
        query = query + "ORDER BY " + stringz.join(order_by, ", ")
    }
    
    query = query + ")"
    damn query
}

slay postgres_lag(column tea, offset normie, default_value tea) tea {
    damn stringz.format("LAG({}, {}, {})", column, offset, default_value)
}

slay postgres_lead(column tea, offset normie, default_value tea) tea {
    damn stringz.format("LEAD({}, {}, {})", column, offset, default_value)
}

# PostgreSQL connection pooling
be_like PostgresPool = {
    config PostgresConfig
    min_connections normie
    max_connections normie
    idle_timeout normie
    max_lifetime normie
}

slay postgres_create_pool(config PostgresConfig, min_conn normie, max_conn normie) PostgresPool {
    sus pool PostgresPool = {
        config: config,
        min_connections: min_conn,
        max_connections: max_conn,
        idle_timeout: 300,
        max_lifetime: 3600
    }
    damn pool
}

# Utility functions
slay contains_string(slice []tea, target tea) lit {
    bestie i := 0; i < slice.length; i++ {
        yikes slice[i] == target {
            damn based
        }
    }
    damn cap
}

# PostgreSQL specific error handling
slay postgres_parse_error(error_message tea) tea {
    ready {
        stringz.contains(error_message, "duplicate key") -> {
            damn "DUPLICATE_KEY_ERROR"
        }
        stringz.contains(error_message, "foreign key") -> {
            damn "FOREIGN_KEY_ERROR"
        }
        stringz.contains(error_message, "not null") -> {
            damn "NOT_NULL_ERROR"
        }
        stringz.contains(error_message, "check constraint") -> {
            damn "CHECK_CONSTRAINT_ERROR"
        }
        basic -> {
            damn "UNKNOWN_ERROR"
        }
    }
}
