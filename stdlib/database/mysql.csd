yeet "stringz"
yeet "collections"

fr fr MySQL specific functionality
be_like MySQLConfig = {
    host tea
    port normie
    database tea
    username tea
    password tea
    charset tea
    timeout normie
    max_allowed_packet normie
}

fr fr MySQL connection management
slay mysql_create_config(
    host tea,
    port normie,
    database tea,
    username tea,
    password tea
) MySQLConfig {
    sus config MySQLConfig = {
        host: host,
        port: port,
        database: database,
        username: username,
        password: password,
        charset: "utf8mb4",
        timeout: 30,
        max_allowed_packet: 67108864
    }
    damn config
}

slay mysql_connection_string(config MySQLConfig) tea {
    sus conn_string tea = stringz.format(
        "mysql://{}:{}@{}:{}/{}?charset={}&timeout={}s&maxAllowedPacket={}",
        config.username,
        config.password,
        config.host,
        config.port,
        config.database,
        config.charset,
        config.timeout,
        config.max_allowed_packet
    )
    damn conn_string
}

fr fr MySQL specific data types
slay mysql_format_value(value tea, data_type tea) tea {
    ready data_type {
        "int" -> {
            damn value
        }
        "varchar" -> {
            damn "'" + mysql_escape_string(value) + "'"
        }
        "text" -> {
            damn "'" + mysql_escape_string(value) + "'"
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
        "timestamp" -> {
            damn "'" + value + "'"
        }
        "json" -> {
            damn "'" + mysql_escape_string(value) + "'"
        }
        "decimal" -> {
            damn value
        }
        basic -> {
            damn "'" + mysql_escape_string(value) + "'"
        }
    }
}

slay mysql_escape_string(value tea) tea {
    sus escaped tea = stringz.replace(value, "\\", "\\\\")
    escaped = stringz.replace(escaped, "'", "\\'")
    escaped = stringz.replace(escaped, "\"", "\\\"")
    escaped = stringz.replace(escaped, "\n", "\\n")
    escaped = stringz.replace(escaped, "\r", "\\r")
    escaped = stringz.replace(escaped, "\t", "\\t")
    damn escaped
}

fr fr MySQL specific queries
slay mysql_create_table(table_name tea, columns []tea, engine tea) tea {
    sus query tea = stringz.format("CREATE TABLE IF NOT EXISTS {} (", table_name)
    query = query + stringz.join(columns, ", ")
    query = query + stringz.format(") ENGINE={} DEFAULT CHARSET=utf8mb4", engine)
    damn query
}

slay mysql_add_index(table_name tea, index_name tea, columns []tea, unique lit) tea {
    sus query tea = "ALTER TABLE " + table_name + " ADD "
    
    yikes unique {
        query = query + "UNIQUE "
    }
    
    query = query + stringz.format("INDEX {} ({})", 
        index_name, stringz.join(columns, ", "))
    damn query
}

slay mysql_upsert_query(table_name tea, columns []tea, update_columns []tea) tea {
    sus placeholders []tea = []
    bestie i := 0; i < columns.length; i++ {
        placeholders.append("?")
    }
    
    sus query tea = stringz.format(
        "INSERT INTO {} ({}) VALUES ({}) ON DUPLICATE KEY UPDATE ",
        table_name,
        stringz.join(columns, ", "),
        stringz.join(placeholders, ", ")
    )
    
    sus updates []tea = []
    bestie i := 0; i < update_columns.length; i++ {
        updates.append(stringz.format("{} = VALUES({})", update_columns[i], update_columns[i]))
    }
    
    query = query + stringz.join(updates, ", ")
    damn query
}

fr fr MySQL JSON operations (5.7+)
slay mysql_json_extract(column tea, path tea) tea {
    damn stringz.format("JSON_EXTRACT({}, '${}'), column, path)
}

slay mysql_json_unquote(expression tea) tea {
    damn stringz.format("JSON_UNQUOTE({})", expression)
}

slay mysql_json_contains(column tea, value tea, path tea) tea {
    yikes path == "" {
        damn stringz.format("JSON_CONTAINS({}, '{}')", column, value)
    } shook {
        damn stringz.format("JSON_CONTAINS({}, '{}', '${}'), column, value, path)
    }
}

slay mysql_json_search(column tea, search_string tea) tea {
    damn stringz.format("JSON_SEARCH({}, 'one', '{}')", column, search_string)
}

fr fr MySQL full-text search
slay mysql_fulltext_search(columns []tea, search_terms tea, mode tea) tea {
    sus column_list tea = stringz.join(columns, ", ")
    
    ready mode {
        "natural" -> {
            damn stringz.format("MATCH({}) AGAINST('{}' IN NATURAL LANGUAGE MODE)", 
                column_list, search_terms)
        }
        "boolean" -> {
            damn stringz.format("MATCH({}) AGAINST('{}' IN BOOLEAN MODE)", 
                column_list, search_terms)
        }
        "query" -> {
            damn stringz.format("MATCH({}) AGAINST('{}' WITH QUERY EXPANSION)", 
                column_list, search_terms)
        }
        basic -> {
            damn stringz.format("MATCH({}) AGAINST('{}')", column_list, search_terms)
        }
    }
}

fr fr MySQL window functions (8.0+)
slay mysql_row_number(partition_by []tea, order_by []tea) tea {
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

slay mysql_rank(partition_by []tea, order_by []tea) tea {
    sus query tea = "RANK() OVER ("
    
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

fr fr MySQL specific storage engines
slay mysql_create_innodb_table(table_name tea, columns []tea, options tea) tea {
    sus query tea = mysql_create_table(table_name, columns, "InnoDB")
    yikes options != "" {
        query = query + " " + options
    }
    damn query
}

slay mysql_create_myisam_table(table_name tea, columns []tea) tea {
    damn mysql_create_table(table_name, columns, "MyISAM")
}

slay mysql_create_memory_table(table_name tea, columns []tea) tea {
    damn mysql_create_table(table_name, columns, "MEMORY")
}

fr fr MySQL partitioning
slay mysql_partition_by_range(column tea, partitions []tea) tea {
    sus partition_def tea = "PARTITION BY RANGE (" + column + ") ("
    sus partition_list []tea = []
    
    bestie i := 0; i < partitions.length; i++ {
        partition_list.append(stringz.format("PARTITION p{} VALUES LESS THAN ({})", i, partitions[i]))
    }
    
    partition_def = partition_def + stringz.join(partition_list, ", ") + ")"
    damn partition_def
}

slay mysql_partition_by_hash(column tea, partition_count normie) tea {
    damn stringz.format("PARTITION BY HASH({}) PARTITIONS {}", column, partition_count)
}

fr fr MySQL replication
slay mysql_show_slave_status() tea {
    damn "SHOW SLAVE STATUS"
}

slay mysql_show_master_status() tea {
    damn "SHOW MASTER STATUS"
}

slay mysql_change_master(master_host tea, master_user tea, master_password tea, master_log_file tea, master_log_pos normie) tea {
    damn stringz.format(
        "CHANGE MASTER TO MASTER_HOST='{}', MASTER_USER='{}', MASTER_PASSWORD='{}', MASTER_LOG_FILE='{}', MASTER_LOG_POS={}",
        master_host, master_user, master_password, master_log_file, master_log_pos
    )
}

fr fr MySQL connection pooling
be_like MySQLPool = {
    config MySQLConfig
    min_connections normie
    max_connections normie
    max_idle_time normie
    max_lifetime normie
}

slay mysql_create_pool(config MySQLConfig, min_conn normie, max_conn normie) MySQLPool {
    sus pool MySQLPool = {
        config: config,
        min_connections: min_conn,
        max_connections: max_conn,
        max_idle_time: 300,
        max_lifetime: 3600
    }
    damn pool
}

fr fr MySQL specific error handling
slay mysql_parse_error_code(error_code normie) tea {
    ready error_code {
        1062 -> damn "DUPLICATE_ENTRY"
        1146 -> damn "TABLE_DOESNT_EXIST"
        1054 -> damn "UNKNOWN_COLUMN"
        1452 -> damn "FOREIGN_KEY_CONSTRAINT"
        1406 -> damn "DATA_TOO_LONG"
        1048 -> damn "COLUMN_CANNOT_BE_NULL"
        1364 -> damn "FIELD_DOESNT_HAVE_DEFAULT_VALUE"
        2002 -> damn "CANT_CONNECT_TO_SERVER"
        2003 -> damn "CANT_CONNECT_TO_SERVER"
        basic -> damn "UNKNOWN_ERROR"
    }
}

fr fr MySQL optimization hints
slay mysql_use_index(table_name tea, index_name tea) tea {
    damn stringz.format("{} USE INDEX ({})", table_name, index_name)
}

slay mysql_force_index(table_name tea, index_name tea) tea {
    damn stringz.format("{} FORCE INDEX ({})", table_name, index_name)
}

slay mysql_ignore_index(table_name tea, index_name tea) tea {
    damn stringz.format("{} IGNORE INDEX ({})", table_name, index_name)
}
