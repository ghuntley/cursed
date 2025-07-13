yeet "stringz"
yeet "collections"

# SQLite specific functionality
be_like SQLiteConfig = {
    database_path tea
    cache_size normie
    journal_mode tea
    synchronous tea
    foreign_keys lit
    wal_autocheckpoint normie
}

# SQLite connection management
slay sqlite_create_config(database_path tea) SQLiteConfig {
    sus config SQLiteConfig = {
        database_path: database_path,
        cache_size: 2000,
        journal_mode: "WAL",
        synchronous: "NORMAL",
        foreign_keys: based,
        wal_autocheckpoint: 1000
    }
    damn config
}

slay sqlite_connection_string(config SQLiteConfig) tea {
    sus conn_string tea = "file:" + config.database_path
    
    sus params []tea = []
    
    yikes config.cache_size != 2000 {
        params.append(stringz.format("cache=shared&cache_size={}", config.cache_size))
    }
    
    yikes config.journal_mode != "DELETE" {
        params.append(stringz.format("journal_mode={}", config.journal_mode))
    }
    
    yikes params.length > 0 {
        conn_string = conn_string + "?" + stringz.join(params, "&")
    }
    
    damn conn_string
}

# SQLite pragmas
slay sqlite_set_pragma(pragma_name tea, value tea) tea {
    damn stringz.format("PRAGMA {} = {}", pragma_name, value)
}

slay sqlite_get_pragma(pragma_name tea) tea {
    damn stringz.format("PRAGMA {}", pragma_name)
}

slay sqlite_enable_foreign_keys() tea {
    damn "PRAGMA foreign_keys = ON"
}

slay sqlite_set_journal_mode(mode tea) tea {
    damn stringz.format("PRAGMA journal_mode = {}", mode)
}

slay sqlite_set_synchronous(level tea) tea {
    damn stringz.format("PRAGMA synchronous = {}", level)
}

slay sqlite_set_cache_size(size normie) tea {
    damn stringz.format("PRAGMA cache_size = {}", size)
}

# SQLite specific data types
slay sqlite_format_value(value tea, data_type tea) tea {
    ready data_type {
        "integer" -> {
            damn value
        }
        "real" -> {
            damn value
        }
        "text" -> {
            damn "'" + sqlite_escape_string(value) + "'"
        }
        "blob" -> {
            damn "X'" + value + "'"  # Hex literal
        }
        "boolean" -> {
            ready value {
                "true" -> damn "1"
                "false" -> damn "0"
                basic -> damn "NULL"
            }
        }
        "datetime" -> {
            damn "'" + value + "'"
        }
        basic -> {
            damn "'" + sqlite_escape_string(value) + "'"
        }
    }
}

slay sqlite_escape_string(value tea) tea {
    # SQLite uses '' to escape single quotes
    damn stringz.replace(value, "'", "''")
}

# SQLite specific queries
slay sqlite_create_table(table_name tea, columns []tea, without_rowid lit) tea {
    sus query tea = stringz.format("CREATE TABLE IF NOT EXISTS {} (", table_name)
    query = query + stringz.join(columns, ", ")
    query = query + ")"
    
    yikes without_rowid {
        query = query + " WITHOUT ROWID"
    }
    
    damn query
}

slay sqlite_add_column(table_name tea, column_definition tea) tea {
    damn stringz.format("ALTER TABLE {} ADD COLUMN {}", table_name, column_definition)
}

slay sqlite_create_index(index_name tea, table_name tea, columns []tea, unique lit, where_clause tea) tea {
    sus query tea = "CREATE "
    
    yikes unique {
        query = query + "UNIQUE "
    }
    
    query = query + stringz.format("INDEX IF NOT EXISTS {} ON {} ({})", 
        index_name, table_name, stringz.join(columns, ", "))
    
    yikes where_clause != "" {
        query = query + " WHERE " + where_clause
    }
    
    damn query
}

# SQLite UPSERT (INSERT OR REPLACE)
slay sqlite_upsert_query(table_name tea, columns []tea, conflict_resolution tea) tea {
    sus placeholders []tea = []
    bestie i := 0; i < columns.length; i++ {
        placeholders.append("?")
    }
    
    sus query tea = stringz.format("INSERT OR {} INTO {} ({}) VALUES ({})", 
        conflict_resolution, table_name, stringz.join(columns, ", "), stringz.join(placeholders, ", "))
    
    damn query
}

slay sqlite_insert_or_ignore(table_name tea, columns []tea) tea {
    damn sqlite_upsert_query(table_name, columns, "IGNORE")
}

slay sqlite_insert_or_replace(table_name tea, columns []tea) tea {
    damn sqlite_upsert_query(table_name, columns, "REPLACE")
}

# SQLite JSON operations (JSON1 extension)
slay sqlite_json_extract(column tea, path tea) tea {
    damn stringz.format("json_extract({}, '${}'), column, path)
}

slay sqlite_json_array_length(column tea, path tea) tea {
    yikes path == "" {
        damn stringz.format("json_array_length({})", column)
    } shook {
        damn stringz.format("json_array_length({}, '${}'), column, path)
    }
}

slay sqlite_json_valid(column tea) tea {
    damn stringz.format("json_valid({})", column)
}

slay sqlite_json_type(column tea, path tea) tea {
    yikes path == "" {
        damn stringz.format("json_type({})", column)
    } shook {
        damn stringz.format("json_type({}, '${}'), column, path)
    }
}

# SQLite full-text search (FTS5)
slay sqlite_create_fts_table(table_name tea, columns []tea, content_table tea) tea {
    sus query tea = stringz.format("CREATE VIRTUAL TABLE {} USING fts5(", table_name)
    query = query + stringz.join(columns, ", ")
    
    yikes content_table != "" {
        query = query + stringz.format(", content='{}'", content_table)
    }
    
    query = query + ")"
    damn query
}

slay sqlite_fts_search(table_name tea, search_term tea) tea {
    damn stringz.format("SELECT * FROM {} WHERE {} MATCH '{}'", table_name, table_name, search_term)
}

slay sqlite_fts_highlight(table_name tea, column_index normie, start_tag tea, end_tag tea) tea {
    damn stringz.format("highlight({}, {}, '{}', '{}')", table_name, column_index, start_tag, end_tag)
}

slay sqlite_fts_snippet(table_name tea, column_index normie, start_tag tea, end_tag tea, ellipsis tea, max_tokens normie) tea {
    damn stringz.format("snippet({}, {}, '{}', '{}', '{}', {})", 
        table_name, column_index, start_tag, end_tag, ellipsis, max_tokens)
}

# SQLite window functions
slay sqlite_row_number() tea {
    damn "ROW_NUMBER() OVER ()"
}

slay sqlite_row_number_partitioned(partition_by []tea, order_by []tea) tea {
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

slay sqlite_lag(column tea, offset normie, default_value tea) tea {
    damn stringz.format("LAG({}, {}, {})", column, offset, default_value)
}

slay sqlite_lead(column tea, offset normie, default_value tea) tea {
    damn stringz.format("LEAD({}, {}, {})", column, offset, default_value)
}

# SQLite date/time functions
slay sqlite_current_timestamp() tea {
    damn "datetime('now')"
}

slay sqlite_date_add(date_value tea, interval tea, unit tea) tea {
    damn stringz.format("datetime({}, '{} {}')", date_value, interval, unit)
}

slay sqlite_date_diff(date1 tea, date2 tea, unit tea) tea {
    ready unit {
        "days" -> {
            damn stringz.format("julianday({}) - julianday({})", date1, date2)
        }
        "seconds" -> {
            damn stringz.format("(julianday({}) - julianday({})) * 86400", date1, date2)
        }
        basic -> {
            damn stringz.format("julianday({}) - julianday({})", date1, date2)
        }
    }
}

slay sqlite_strftime(format tea, date_value tea) tea {
    damn stringz.format("strftime('{}', {})", format, date_value)
}

# SQLite database introspection
slay sqlite_list_tables() tea {
    damn "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
}

slay sqlite_table_info(table_name tea) tea {
    damn stringz.format("PRAGMA table_info({})", table_name)
}

slay sqlite_index_list(table_name tea) tea {
    damn stringz.format("PRAGMA index_list({})", table_name)
}

slay sqlite_foreign_key_list(table_name tea) tea {
    damn stringz.format("PRAGMA foreign_key_list({})", table_name)
}

# SQLite backup and vacuum
slay sqlite_vacuum() tea {
    damn "VACUUM"
}

slay sqlite_vacuum_into(target_file tea) tea {
    damn stringz.format("VACUUM INTO '{}'", target_file)
}

slay sqlite_analyze() tea {
    damn "ANALYZE"
}

slay sqlite_analyze_table(table_name tea) tea {
    damn stringz.format("ANALYZE {}", table_name)
}

# SQLite connection pooling (simple implementation)
be_like SQLitePool = {
    config SQLiteConfig
    max_connections normie
    idle_timeout normie
}

slay sqlite_create_pool(config SQLiteConfig, max_conn normie) SQLitePool {
    sus pool SQLitePool = {
        config: config,
        max_connections: max_conn,
        idle_timeout: 300
    }
    damn pool
}

# SQLite error handling
slay sqlite_parse_error(error_message tea) tea {
    ready {
        stringz.contains(error_message, "UNIQUE constraint") -> {
            damn "UNIQUE_CONSTRAINT_ERROR"
        }
        stringz.contains(error_message, "NOT NULL constraint") -> {
            damn "NOT_NULL_CONSTRAINT_ERROR"
        }
        stringz.contains(error_message, "FOREIGN KEY constraint") -> {
            damn "FOREIGN_KEY_CONSTRAINT_ERROR"
        }
        stringz.contains(error_message, "no such table") -> {
            damn "TABLE_NOT_FOUND_ERROR"
        }
        stringz.contains(error_message, "no such column") -> {
            damn "COLUMN_NOT_FOUND_ERROR"
        }
        stringz.contains(error_message, "database is locked") -> {
            damn "DATABASE_LOCKED_ERROR"
        }
        basic -> {
            damn "UNKNOWN_SQLITE_ERROR"
        }
    }
}

# SQLite performance optimization
slay sqlite_optimize_query(query tea) tea {
    # Add query optimization hints
    yikes stringz.contains(query, "SELECT") && !stringz.contains(query, "INDEXED BY") {
        # Could suggest index usage
        damn query + " /* Consider adding INDEXED BY clause */"
    }
    damn query
}

slay sqlite_memory_optimization() []tea {
    damn [
        "PRAGMA temp_store = memory",
        "PRAGMA mmap_size = 268435456",  # 256MB
        "PRAGMA cache_size = 10000",
        "PRAGMA synchronous = NORMAL",
        "PRAGMA journal_mode = WAL"
    ]
}
