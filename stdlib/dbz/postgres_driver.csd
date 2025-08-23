fr fr REAL PostgreSQL Database Driver Implementation
fr fr Implements PostgreSQL wire protocol for native database connectivity

yeet "stringz"
yeet "networkz"
yeet "vibez"
yeet "memoryz"
yeet "cryptz"

fr fr ===== PostgreSQL Protocol Constants =====

sus POSTGRES_DEFAULT_PORT drip = 5432
sus POSTGRES_PROTOCOL_VERSION drip = 196608  // 3.0 in network format

fr fr Message types
sus POSTGRES_AUTH_OK drip = 0
sus POSTGRES_AUTH_PASSWORD drip = 3
sus POSTGRES_AUTH_MD5 drip = 5
sus POSTGRES_AUTH_SASL drip = 10

sus POSTGRES_READY_FOR_QUERY rune = 'Z'
sus POSTGRES_QUERY rune = 'Q'
sus POSTGRES_ROW_DESCRIPTION rune = 'T'
sus POSTGRES_DATA_ROW rune = 'D'
sus POSTGRES_COMMAND_COMPLETE rune = 'C'
sus POSTGRES_ERROR_RESPONSE rune = 'E'
sus POSTGRES_NOTICE_RESPONSE rune = 'N'
sus POSTGRES_PARAMETER_STATUS rune = 'S'
sus POSTGRES_BACKEND_KEY_DATA rune = 'K'

fr fr ===== PostgreSQL Connection Structure =====

squad PostgresConnection {
    socket_fd drip
    host tea
    port drip
    database tea
    username tea
    password tea
    connected lit
    process_id drip
    secret_key drip
    transaction_status rune
    server_version tea
    server_encoding tea
}

fr fr ===== Connection Pool Management =====

sus MAX_POSTGRES_CONNECTIONS drip = 100
sus postgres_connection_pool []PostgresConnection = []
sus postgres_pool_initialized lit = cringe

slay init_postgres_pool() {
    ready (!postgres_pool_initialized) {
        postgres_connection_pool = make_array<PostgresConnection>(MAX_POSTGRES_CONNECTIONS)
        bestie (sus i drip = 0; i < MAX_POSTGRES_CONNECTIONS; i++) {
            postgres_connection_pool[i] = PostgresConnection{
                socket_fd: -1,
                connected: cringe,
                transaction_status: 'I'  // Idle
            }
        }
        postgres_pool_initialized = based
        vibez.spill("PostgreSQL connection pool initialized with", MAX_POSTGRES_CONNECTIONS, "slots")
    }
}

slay get_postgres_connection(host tea, port drip, database tea, username tea, password tea) *PostgresConnection {
    init_postgres_pool()
    
    fr fr Try to find existing connection
    bestie (sus i drip = 0; i < MAX_POSTGRES_CONNECTIONS; i++) {
        sus *conn *PostgresConnection = &postgres_connection_pool[i]
        ready (conn.connected && 
               conn.host == host && 
               conn.port == port &&
               conn.database == database &&
               conn.username == username) {
            damn conn
        }
    }
    
    fr fr Find empty slot for new connection
    bestie (sus i drip = 0; i < MAX_POSTGRES_CONNECTIONS; i++) {
        sus *conn *PostgresConnection = &postgres_connection_pool[i]
        ready (!conn.connected) {
            conn.host = host
            conn.port = port
            conn.database = database
            conn.username = username
            conn.password = password
            ready (postgres_connect(conn)) {
                damn conn
            } otherwise {
                damn nullable<PostgresConnection>
            }
        }
    }
    
    vibez.spill("ERROR: PostgreSQL connection pool exhausted")
    damn nullable<PostgresConnection>
}

fr fr ===== Binary Protocol Implementation =====

slay write_int32(data []rune, value drip) {
    data[0] = (value >> 24) & 0xFF
    data[1] = (value >> 16) & 0xFF  
    data[2] = (value >> 8) & 0xFF
    data[3] = value & 0xFF
}

slay read_int32(data []rune) drip {
    damn ((data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3])
}

slay write_int16(data []rune, value drip) {
    data[0] = (value >> 8) & 0xFF
    data[1] = value & 0xFF
}

slay read_int16(data []rune) drip {
    damn ((data[0] << 8) | data[1])
}

slay send_startup_message(conn *PostgresConnection) lit {
    sus message []rune = make_array<rune>(1024)
    sus pos drip = 0
    
    fr fr Protocol version (4 bytes)
    write_int32(message[pos:pos+4], POSTGRES_PROTOCOL_VERSION)
    pos += 4
    
    fr fr Parameters
    sus params []tea = [
        "user", conn.username,
        "database", conn.database,
        "application_name", "cursed-dbz",
        "client_encoding", "UTF8",
        ""  // Terminator
    ]
    
    bestie (sus param tea : params) {
        sus param_bytes []rune = stringz.to_bytes(param)
        bestie (sus b rune : param_bytes) {
            message[pos] = b
            pos++
        }
        message[pos] = 0  // Null terminator
        pos++
    }
    
    fr fr Message length at beginning
    sus length_bytes []rune = make_array<rune>(4) 
    write_int32(length_bytes, pos)
    
    fr fr Send length then message
    sus bytes_sent drip = networkz.socket_send(conn.socket_fd, length_bytes)
    ready (bytes_sent != 4) {
        damn cringe
    }
    
    bytes_sent = networkz.socket_send(conn.socket_fd, message[0:pos])
    damn bytes_sent == pos
}

slay receive_message(conn *PostgresConnection) (rune, []rune) {
    sus header []rune = make_array<rune>(5)
    sus bytes_received drip = networkz.socket_receive(conn.socket_fd, header)
    
    ready (bytes_received != 5) {
        damn 0, []
    }
    
    sus msg_type rune = header[0]
    sus msg_length drip = read_int32(header[1:5])
    
    sus payload []rune = make_array<rune>(msg_length - 4)
    bytes_received = networkz.socket_receive(conn.socket_fd, payload)
    
    ready (bytes_received != msg_length - 4) {
        damn 0, []
    }
    
    damn msg_type, payload
}

slay handle_authentication(conn *PostgresConnection, auth_type drip, auth_data []rune) lit {
    sick auth_type {
        when POSTGRES_AUTH_OK -> {
            vibez.spill("PostgreSQL authentication successful")
            damn based
        }
        when POSTGRES_AUTH_PASSWORD -> {
            fr fr Send plaintext password
            sus password_msg []rune = make_array<rune>(1024)
            sus pos drip = 0
            
            password_msg[pos] = 'p'  // Password message
            pos++
            
            sus password_bytes []rune = stringz.to_bytes(conn.password)
            write_int32(password_msg[pos:pos+4], password_bytes.len() + 5)
            pos += 4
            
            bestie (sus b rune : password_bytes) {
                password_msg[pos] = b
                pos++
            }
            password_msg[pos] = 0  // Null terminator
            pos++
            
            sus bytes_sent drip = networkz.socket_send(conn.socket_fd, password_msg[0:pos])
            damn bytes_sent == pos
        }
        when POSTGRES_AUTH_MD5 -> {
            fr fr MD5 authentication with salt
            ready (auth_data.len() < 4) {
                damn cringe
            }
            
            sus salt []rune = auth_data[0:4]
            
            fr fr Create MD5 hash: md5(md5(password + username) + salt)
            sus inner_hash tea = cryptz.md5_hex(conn.password + conn.username)
            sus outer_input []rune = stringz.to_bytes(inner_hash)
            bestie (sus s rune : salt) {
                outer_input = outer_input + [s]
            }
            sus final_hash tea = "md5" + cryptz.md5_hex(stringz.from_bytes(outer_input))
            
            sus auth_msg []rune = make_array<rune>(1024)
            sus pos drip = 0
            
            auth_msg[pos] = 'p'  // Password message
            pos++
            
            sus hash_bytes []rune = stringz.to_bytes(final_hash)
            write_int32(auth_msg[pos:pos+4], hash_bytes.len() + 5)
            pos += 4
            
            bestie (sus b rune : hash_bytes) {
                auth_msg[pos] = b
                pos++
            }
            auth_msg[pos] = 0
            pos++
            
            sus bytes_sent drip = networkz.socket_send(conn.socket_fd, auth_msg[0:pos])
            damn bytes_sent == pos
        }
        when _ -> {
            vibez.spill("ERROR: Unsupported PostgreSQL auth type:", auth_type)
            damn cringe
        }
    }
}

slay postgres_connect(conn *PostgresConnection) lit {
    fr fr Create TCP connection
    conn.socket_fd = networkz.socket_create()
    ready (conn.socket_fd < 0) {
        vibez.spill("ERROR: Failed to create PostgreSQL socket")
        damn cringe
    }
    
    ready (!networkz.socket_connect(conn.socket_fd, conn.host, conn.port)) {
        vibez.spill("ERROR: Failed to connect to PostgreSQL server")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    fr fr Send startup message
    ready (!send_startup_message(conn)) {
        vibez.spill("ERROR: Failed to send PostgreSQL startup message")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    fr fr Handle authentication and setup
    sus authenticated lit = cringe
    bestie (!authenticated) {
        sus msg_type rune, sus payload []rune = receive_message(conn)
        
        sick msg_type {
            when 'R' -> {  // Authentication request
                ready (payload.len() < 4) {
                    vibez.spill("ERROR: Invalid PostgreSQL auth message")
                    damn cringe
                }
                
                sus auth_type drip = read_int32(payload[0:4])
                sus auth_data []rune = payload[4:]
                
                ready (!handle_authentication(conn, auth_type, auth_data)) {
                    vibez.spill("ERROR: PostgreSQL authentication failed")
                    damn cringe
                }
                
                ready (auth_type == POSTGRES_AUTH_OK) {
                    authenticated = based
                }
            }
            when POSTGRES_PARAMETER_STATUS -> {
                fr fr Server parameter notification
                sus param_name tea = stringz.from_null_terminated(payload, 0)
                sus param_value tea = stringz.from_null_terminated(payload, stringz.strlen(param_name) + 1)
                
                ready (param_name == "server_version") {
                    conn.server_version = param_value
                } otherwise ready (param_name == "server_encoding") {
                    conn.server_encoding = param_value
                }
            }
            when POSTGRES_BACKEND_KEY_DATA -> {
                ready (payload.len() >= 8) {
                    conn.process_id = read_int32(payload[0:4])
                    conn.secret_key = read_int32(payload[4:8])
                }
            }
            when POSTGRES_READY_FOR_QUERY -> {
                ready (payload.len() >= 1) {
                    conn.transaction_status = payload[0]
                }
                authenticated = based
                conn.connected = based
                vibez.spill("PostgreSQL connection established to", conn.host, "database", conn.database)
                damn based
            }
            when POSTGRES_ERROR_RESPONSE -> {
                sus error_msg tea = stringz.from_null_terminated(payload, 1)  // Skip severity
                vibez.spill("ERROR: PostgreSQL connection failed:", error_msg)
                damn cringe
            }
            when _ -> {
                vibez.spill("WARNING: Unknown PostgreSQL message type:", msg_type)
            }
        }
    }
    
    damn cringe
}

fr fr ===== Query Execution =====

slay postgres_execute_query(conn *PostgresConnection, sql tea) QueryResult {
    sus result QueryResult = QueryResult{}
    result.success = cringe
    
    ready (!conn.connected) {
        result.error_message = "PostgreSQL connection not established"
        damn result
    }
    
    fr fr Send query message
    sus query_msg []rune = make_array<rune>(1024)
    sus pos drip = 0
    
    query_msg[pos] = POSTGRES_QUERY
    pos++
    
    sus sql_bytes []rune = stringz.to_bytes(sql)
    write_int32(query_msg[pos:pos+4], sql_bytes.len() + 5)
    pos += 4
    
    bestie (sus b rune : sql_bytes) {
        query_msg[pos] = b
        pos++
    }
    query_msg[pos] = 0  // Null terminator
    pos++
    
    sus bytes_sent drip = networkz.socket_send(conn.socket_fd, query_msg[0:pos])
    ready (bytes_sent != pos) {
        result.error_message = "Failed to send PostgreSQL query"
        damn result
    }
    
    fr fr Process response messages
    sus query_complete lit = cringe
    result.rows = []
    result.column_names = []
    
    bestie (!query_complete) {
        sus msg_type rune, sus payload []rune = receive_message(conn)
        
        sick msg_type {
            when POSTGRES_ROW_DESCRIPTION -> {
                fr fr Column metadata
                ready (payload.len() < 2) {
                    result.error_message = "Invalid row description"
                    damn result
                }
                
                sus column_count drip = read_int16(payload[0:2])
                result.column_names = make_array<tea>(column_count)
                
                sus offset drip = 2
                bestie (sus i drip = 0; i < column_count; i++) {
                    sus col_name tea = stringz.from_null_terminated(payload, offset)
                    result.column_names[i] = col_name
                    offset += stringz.strlen(col_name) + 1
                    offset += 18  // Skip table OID, column attr, type OID, type size, type modifier, format code
                }
            }
            when POSTGRES_DATA_ROW -> {
                fr fr Row data
                ready (payload.len() < 2) {
                    continue
                }
                
                sus column_count drip = read_int16(payload[0:2])
                sus row []tea = make_array<tea>(column_count)
                
                sus offset drip = 2
                bestie (sus i drip = 0; i < column_count; i++) {
                    ready (offset + 4 > payload.len()) {
                        break
                    }
                    
                    sus field_length drip = read_int32(payload[offset:offset+4])
                    offset += 4
                    
                    ready (field_length == -1) {
                        row[i] = ""  // NULL value
                    } otherwise {
                        ready (offset + field_length <= payload.len()) {
                            row[i] = stringz.from_bytes(payload[offset:offset+field_length])
                            offset += field_length
                        }
                    }
                }
                
                result.rows = result.rows + [row]
            }
            when POSTGRES_COMMAND_COMPLETE -> {
                fr fr Extract rows affected from command tag
                sus command_tag tea = stringz.from_null_terminated(payload, 0)
                
                ready (stringz.contains(command_tag, "INSERT")) {
                    sus parts []tea = stringz.split(command_tag, " ")
                    ready (parts.len() >= 3) {
                        result.rows_affected = stringz.to_int(parts[2])
                        ready (parts.len() >= 2) {
                            result.last_insert_id = stringz.to_int(parts[1])
                        }
                    }
                } otherwise ready (stringz.contains(command_tag, "UPDATE") || stringz.contains(command_tag, "DELETE")) {
                    sus parts []tea = stringz.split(command_tag, " ")
                    ready (parts.len() >= 2) {
                        result.rows_affected = stringz.to_int(parts[1])
                    }
                }
                
                result.success = based
            }
            when POSTGRES_READY_FOR_QUERY -> {
                ready (payload.len() >= 1) {
                    conn.transaction_status = payload[0]
                }
                query_complete = based
            }
            when POSTGRES_ERROR_RESPONSE -> {
                sus error_msg tea = stringz.from_null_terminated(payload, 1)
                result.error_message = error_msg
                query_complete = based
            }
            when POSTGRES_NOTICE_RESPONSE -> {
                fr fr Ignore notices for now
            }
            when _ -> {
                vibez.spill("WARNING: Unknown PostgreSQL response message type:", msg_type)
            }
        }
    }
    
    damn result
}

fr fr ===== Public PostgreSQL Functions =====

slay postgres_real_query(host tea, port drip, database tea, username tea, password tea, sql tea) QueryResult {
    sus *conn *PostgresConnection = get_postgres_connection(host, port, database, username, password)
    ready (conn == nullable<PostgresConnection>) {
        sus result QueryResult = QueryResult{}
        result.success = cringe
        result.error_message = "Failed to get PostgreSQL connection"
        damn result
    }
    
    damn postgres_execute_query(conn, sql)
}

slay postgres_real_query_simple(connection_string tea, sql tea) QueryResult {
    fr fr Parse connection string: "host=localhost port=5432 dbname=test user=postgres password=secret"
    sus host tea = "localhost"
    sus port drip = POSTGRES_DEFAULT_PORT
    sus database tea = "postgres"
    sus username tea = "postgres"
    sus password tea = ""
    
    sus params []tea = stringz.split(connection_string, " ")
    bestie (sus param tea : params) {
        ready (stringz.starts_with(param, "host=")) {
            host = param[5:]
        } otherwise ready (stringz.starts_with(param, "port=")) {
            port = stringz.to_int(param[5:])
        } otherwise ready (stringz.starts_with(param, "dbname=")) {
            database = param[7:]
        } otherwise ready (stringz.starts_with(param, "user=")) {
            username = param[5:]
        } otherwise ready (stringz.starts_with(param, "password=")) {
            password = param[9:]
        }
    }
    
    damn postgres_real_query(host, port, database, username, password, sql)
}

fr fr ===== Transaction Management =====

slay postgres_begin_transaction(connection_string tea) lit {
    sus result QueryResult = postgres_real_query_simple(connection_string, "BEGIN")
    damn result.success
}

slay postgres_commit_transaction(connection_string tea) lit {
    sus result QueryResult = postgres_real_query_simple(connection_string, "COMMIT")
    damn result.success
}

slay postgres_rollback_transaction(connection_string tea) lit {
    sus result QueryResult = postgres_real_query_simple(connection_string, "ROLLBACK")
    damn result.success
}

fr fr ===== Connection Management =====

slay postgres_close_all_connections() {
    ready (postgres_pool_initialized) {
        bestie (sus i drip = 0; i < MAX_POSTGRES_CONNECTIONS; i++) {
            ready (postgres_connection_pool[i].connected) {
                networkz.socket_close(postgres_connection_pool[i].socket_fd)
                postgres_connection_pool[i].connected = cringe
            }
        }
        vibez.spill("All PostgreSQL connections closed")
    }
}

slay postgres_get_connection_stats() (drip, drip) {
    sus active_connections drip = 0
    sus total_connections drip = 0
    
    ready (postgres_pool_initialized) {
        bestie (sus conn PostgresConnection : postgres_connection_pool) {
            ready (conn.connected) {
                active_connections++
            }
            total_connections++
        }
    }
    
    damn active_connections, total_connections
}
