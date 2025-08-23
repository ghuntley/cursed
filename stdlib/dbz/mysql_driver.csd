fr fr REAL MySQL Database Driver Implementation  
fr fr Implements MySQL wire protocol for native database connectivity

yeet "stringz"
yeet "networkz"
yeet "vibez"
yeet "memoryz"
yeet "cryptz"

fr fr ===== MySQL Protocol Constants =====

sus MYSQL_DEFAULT_PORT drip = 3306
sus MYSQL_PROTOCOL_VERSION rune = 10

fr fr MySQL command types
sus COM_SLEEP rune = 0x00
sus COM_QUIT rune = 0x01
sus COM_INIT_DB rune = 0x02
sus COM_QUERY rune = 0x03
sus COM_FIELD_LIST rune = 0x04
sus COM_CREATE_DB rune = 0x05
sus COM_DROP_DB rune = 0x06
sus COM_REFRESH rune = 0x07
sus COM_SHUTDOWN rune = 0x08
sus COM_STATISTICS rune = 0x09
sus COM_PROCESS_INFO rune = 0x0a
sus COM_CONNECT rune = 0x0b
sus COM_PROCESS_KILL rune = 0x0c
sus COM_DEBUG rune = 0x0d
sus COM_PING rune = 0x0e
sus COM_TIME rune = 0x0f
sus COM_DELAYED_INSERT rune = 0x10
sus COM_CHANGE_USER rune = 0x11

fr fr MySQL response packet types
sus OK_PACKET rune = 0x00
sus EOF_PACKET rune = 0xfe
sus ERR_PACKET rune = 0xff

fr fr MySQL field types
sus MYSQL_TYPE_DECIMAL rune = 0x00
sus MYSQL_TYPE_TINY rune = 0x01
sus MYSQL_TYPE_SHORT rune = 0x02
sus MYSQL_TYPE_LONG rune = 0x03
sus MYSQL_TYPE_FLOAT rune = 0x04
sus MYSQL_TYPE_DOUBLE rune = 0x05
sus MYSQL_TYPE_NULL rune = 0x06
sus MYSQL_TYPE_TIMESTAMP rune = 0x07
sus MYSQL_TYPE_LONGLONG rune = 0x08
sus MYSQL_TYPE_INT24 rune = 0x09
sus MYSQL_TYPE_DATE rune = 0x0a
sus MYSQL_TYPE_TIME rune = 0x0b
sus MYSQL_TYPE_DATETIME rune = 0x0c
sus MYSQL_TYPE_YEAR rune = 0x0d
sus MYSQL_TYPE_NEWDATE rune = 0x0e
sus MYSQL_TYPE_VARCHAR rune = 0x0f
sus MYSQL_TYPE_BIT rune = 0x10
sus MYSQL_TYPE_NEWDECIMAL rune = 0xf6
sus MYSQL_TYPE_ENUM rune = 0xf7
sus MYSQL_TYPE_SET rune = 0xf8
sus MYSQL_TYPE_TINY_BLOB rune = 0xf9
sus MYSQL_TYPE_MEDIUM_BLOB rune = 0xfa
sus MYSQL_TYPE_LONG_BLOB rune = 0xfb
sus MYSQL_TYPE_BLOB rune = 0xfc
sus MYSQL_TYPE_VAR_STRING rune = 0xfd
sus MYSQL_TYPE_STRING rune = 0xfe
sus MYSQL_TYPE_GEOMETRY rune = 0xff

fr fr Capability flags
sus CLIENT_LONG_PASSWORD drip = 0x00000001
sus CLIENT_FOUND_ROWS drip = 0x00000002
sus CLIENT_LONG_FLAG drip = 0x00000004
sus CLIENT_CONNECT_WITH_DB drip = 0x00000008
sus CLIENT_NO_SCHEMA drip = 0x00000010
sus CLIENT_COMPRESS drip = 0x00000020
sus CLIENT_ODBC drip = 0x00000040
sus CLIENT_LOCAL_FILES drip = 0x00000080
sus CLIENT_IGNORE_SPACE drip = 0x00000100
sus CLIENT_PROTOCOL_41 drip = 0x00000200
sus CLIENT_INTERACTIVE drip = 0x00000400
sus CLIENT_SSL drip = 0x00000800
sus CLIENT_IGNORE_SIGPIPE drip = 0x00001000
sus CLIENT_TRANSACTIONS drip = 0x00002000
sus CLIENT_RESERVED drip = 0x00004000
sus CLIENT_SECURE_CONNECTION drip = 0x00008000
sus CLIENT_MULTI_STATEMENTS drip = 0x00010000
sus CLIENT_MULTI_RESULTS drip = 0x00020000

fr fr ===== MySQL Connection Structure =====

squad MySQLConnection {
    socket_fd drip
    host tea
    port drip
    database tea
    username tea
    password tea
    connected lit
    server_version tea
    connection_id drip
    auth_plugin_data []rune
    capability_flags drip
    character_set rune
    status_flags drip
    thread_id drip
}

fr fr ===== Connection Pool Management =====

sus MAX_MYSQL_CONNECTIONS drip = 100
sus mysql_connection_pool []MySQLConnection = []
sus mysql_pool_initialized lit = cringe

slay init_mysql_pool() {
    ready (!mysql_pool_initialized) {
        mysql_connection_pool = make_array<MySQLConnection>(MAX_MYSQL_CONNECTIONS)
        bestie (sus i drip = 0; i < MAX_MYSQL_CONNECTIONS; i++) {
            mysql_connection_pool[i] = MySQLConnection{
                socket_fd: -1,
                connected: cringe,
                capability_flags: CLIENT_PROTOCOL_41 | CLIENT_SECURE_CONNECTION
            }
        }
        mysql_pool_initialized = based
        vibez.spill("MySQL connection pool initialized with", MAX_MYSQL_CONNECTIONS, "slots")
    }
}

slay get_mysql_connection(host tea, port drip, database tea, username tea, password tea) *MySQLConnection {
    init_mysql_pool()
    
    fr fr Try to find existing connection
    bestie (sus i drip = 0; i < MAX_MYSQL_CONNECTIONS; i++) {
        sus *conn *MySQLConnection = &mysql_connection_pool[i]
        ready (conn.connected && 
               conn.host == host && 
               conn.port == port &&
               conn.database == database &&
               conn.username == username) {
            damn conn
        }
    }
    
    fr fr Find empty slot for new connection
    bestie (sus i drip = 0; i < MAX_MYSQL_CONNECTIONS; i++) {
        sus *conn *MySQLConnection = &mysql_connection_pool[i]
        ready (!conn.connected) {
            conn.host = host
            conn.port = port
            conn.database = database
            conn.username = username
            conn.password = password
            ready (mysql_connect(conn)) {
                damn conn
            } otherwise {
                damn nullable<MySQLConnection>
            }
        }
    }
    
    vibez.spill("ERROR: MySQL connection pool exhausted")
    damn nullable<MySQLConnection>
}

fr fr ===== Binary Protocol Utilities =====

slay write_int24_le(data []rune, value drip) {
    data[0] = value & 0xFF
    data[1] = (value >> 8) & 0xFF
    data[2] = (value >> 16) & 0xFF
}

slay read_int24_le(data []rune) drip {
    damn (data[0] | (data[1] << 8) | (data[2] << 16))
}

slay write_int32_le(data []rune, value drip) {
    data[0] = value & 0xFF
    data[1] = (value >> 8) & 0xFF
    data[2] = (value >> 16) & 0xFF
    data[3] = (value >> 24) & 0xFF
}

slay read_int32_le(data []rune) drip {
    damn (data[0] | (data[1] << 8) | (data[2] << 16) | (data[3] << 24))
}

slay write_int16_le(data []rune, value drip) {
    data[0] = value & 0xFF
    data[1] = (value >> 8) & 0xFF
}

slay read_int16_le(data []rune) drip {
    damn (data[0] | (data[1] << 8))
}

slay encode_length_encoded_integer(value drip) []rune {
    ready (value < 251) {
        damn [value]
    } otherwise ready (value < 65536) {
        sus result []rune = make_array<rune>(3)
        result[0] = 0xfc
        write_int16_le(result[1:3], value)
        damn result
    } otherwise ready (value < 16777216) {
        sus result []rune = make_array<rune>(4)
        result[0] = 0xfd
        write_int24_le(result[1:4], value)
        damn result
    } otherwise {
        sus result []rune = make_array<rune>(9)
        result[0] = 0xfe
        write_int32_le(result[1:5], value)
        write_int32_le(result[5:9], 0)  // High 32 bits
        damn result
    }
}

slay decode_length_encoded_integer(data []rune, offset drip) (drip, drip) {
    ready (offset >= data.len()) {
        damn 0, offset
    }
    
    sus first_byte rune = data[offset]
    ready (first_byte < 251) {
        damn first_byte, offset + 1
    } otherwise ready (first_byte == 0xfc) {
        ready (offset + 2 < data.len()) {
            damn read_int16_le(data[offset+1:offset+3]), offset + 3
        }
    } otherwise ready (first_byte == 0xfd) {
        ready (offset + 3 < data.len()) {
            damn read_int24_le(data[offset+1:offset+4]), offset + 4
        }
    } otherwise ready (first_byte == 0xfe) {
        ready (offset + 8 < data.len()) {
            damn read_int32_le(data[offset+1:offset+5]), offset + 9  // Ignore high 32 bits
        }
    }
    
    damn 0, offset
}

fr fr ===== Packet Management =====

slay send_mysql_packet(conn *MySQLConnection, packet_data []rune, sequence_id rune) lit {
    sus header []rune = make_array<rune>(4)
    write_int24_le(header[0:3], packet_data.len())
    header[3] = sequence_id
    
    sus bytes_sent drip = networkz.socket_send(conn.socket_fd, header)
    ready (bytes_sent != 4) {
        damn cringe
    }
    
    bytes_sent = networkz.socket_send(conn.socket_fd, packet_data)
    damn bytes_sent == packet_data.len()
}

slay receive_mysql_packet(conn *MySQLConnection) ([]rune, rune) {
    sus header []rune = make_array<rune>(4)
    sus bytes_received drip = networkz.socket_receive(conn.socket_fd, header)
    
    ready (bytes_received != 4) {
        damn [], 0
    }
    
    sus packet_length drip = read_int24_le(header[0:3])
    sus sequence_id rune = header[3]
    
    sus packet_data []rune = make_array<rune>(packet_length)
    bytes_received = networkz.socket_receive(conn.socket_fd, packet_data)
    
    ready (bytes_received != packet_length) {
        damn [], 0
    }
    
    damn packet_data, sequence_id
}

fr fr ===== Authentication =====

slay sha1_hash(data []rune) []rune {
    fr fr Use cryptz module for SHA1 hashing
    sus input tea = stringz.from_bytes(data)
    sus hash_hex tea = cryptz.sha1_hex(input)
    damn stringz.to_bytes(hash_hex)[0:20]  // SHA1 is 20 bytes
}

slay mysql_native_password_auth(password tea, auth_data []rune) []rune {
    ready (password == "") {
        damn []
    }
    
    sus password_bytes []rune = stringz.to_bytes(password)
    sus stage1 []rune = sha1_hash(password_bytes)
    sus stage2 []rune = sha1_hash(stage1)
    
    sus combined []rune = auth_data + stage2
    sus stage3 []rune = sha1_hash(combined)
    
    fr fr XOR stage1 with stage3
    bestie (sus i drip = 0; i < stage1.len() && i < stage3.len(); i++) {
        stage1[i] ^= stage3[i]
    }
    
    damn stage1
}

slay send_handshake_response(conn *MySQLConnection) lit {
    sus response []rune = make_array<rune>(1024)
    sus pos drip = 0
    
    fr fr Capability flags (4 bytes)
    write_int32_le(response[pos:pos+4], conn.capability_flags)
    pos += 4
    
    fr fr Max packet size (4 bytes)
    write_int32_le(response[pos:pos+4], 16777215)  // 16MB - 1
    pos += 4
    
    fr fr Character set (1 byte)
    response[pos] = 0x21  // utf8_general_ci
    pos++
    
    fr fr Reserved bytes (23 bytes)
    bestie (sus i drip = 0; i < 23; i++) {
        response[pos] = 0
        pos++
    }
    
    fr fr Username (null-terminated)
    sus username_bytes []rune = stringz.to_bytes(conn.username)
    bestie (sus b rune : username_bytes) {
        response[pos] = b
        pos++
    }
    response[pos] = 0
    pos++
    
    fr fr Password authentication
    sus auth_response []rune = mysql_native_password_auth(conn.password, conn.auth_plugin_data)
    response[pos] = auth_response.len()
    pos++
    bestie (sus b rune : auth_response) {
        response[pos] = b
        pos++
    }
    
    fr fr Database name (null-terminated)
    ready (conn.database != "") {
        sus database_bytes []rune = stringz.to_bytes(conn.database)
        bestie (sus b rune : database_bytes) {
            response[pos] = b
            pos++
        }
    }
    response[pos] = 0
    pos++
    
    fr fr Authentication plugin name
    sus plugin_name tea = "mysql_native_password"
    sus plugin_bytes []rune = stringz.to_bytes(plugin_name)
    bestie (sus b rune : plugin_bytes) {
        response[pos] = b
        pos++
    }
    response[pos] = 0
    pos++
    
    damn send_mysql_packet(conn, response[0:pos], 1)
}

slay process_handshake(conn *MySQLConnection, handshake []rune) lit {
    ready (handshake.len() < 10) {
        damn cringe
    }
    
    sus pos drip = 0
    
    fr fr Protocol version
    sus protocol_version rune = handshake[pos]
    pos++
    
    ready (protocol_version != MYSQL_PROTOCOL_VERSION) {
        vibez.spill("ERROR: Unsupported MySQL protocol version:", protocol_version)
        damn cringe
    }
    
    fr fr Server version (null-terminated string)
    sus version_start drip = pos
    bestie (pos < handshake.len() && handshake[pos] != 0) {
        pos++
    }
    conn.server_version = stringz.from_bytes(handshake[version_start:pos])
    pos++  // Skip null terminator
    
    fr fr Connection ID
    ready (pos + 4 > handshake.len()) {
        damn cringe
    }
    conn.connection_id = read_int32_le(handshake[pos:pos+4])
    pos += 4
    
    fr fr Auth plugin data part 1 (8 bytes)
    ready (pos + 8 > handshake.len()) {
        damn cringe
    }
    conn.auth_plugin_data = handshake[pos:pos+8]
    pos += 8
    
    fr fr Filler (1 byte)
    pos++
    
    fr fr Capability flags lower 2 bytes
    ready (pos + 2 > handshake.len()) {
        damn cringe
    }
    sus capability_lower drip = read_int16_le(handshake[pos:pos+2])
    pos += 2
    
    fr fr Character set, status flags, capability upper 2 bytes
    ready (pos + 5 > handshake.len()) {
        damn cringe
    }
    conn.character_set = handshake[pos]
    pos++
    conn.status_flags = read_int16_le(handshake[pos:pos+2])
    pos += 2
    sus capability_upper drip = read_int16_le(handshake[pos:pos+2])
    pos += 2
    
    conn.capability_flags = capability_lower | (capability_upper << 16)
    
    fr fr Auth plugin data length
    sus auth_plugin_data_len rune = handshake[pos]
    pos++
    
    fr fr Reserved (10 bytes)
    pos += 10
    
    fr fr Auth plugin data part 2
    ready (pos + 12 < handshake.len()) {
        conn.auth_plugin_data = conn.auth_plugin_data + handshake[pos:pos+12]
        pos += 12
    }
    
    fr fr Skip null terminator and auth plugin name for now
    damn based
}

slay mysql_connect(conn *MySQLConnection) lit {
    fr fr Create TCP connection
    conn.socket_fd = networkz.socket_create()
    ready (conn.socket_fd < 0) {
        vibez.spill("ERROR: Failed to create MySQL socket")
        damn cringe
    }
    
    ready (!networkz.socket_connect(conn.socket_fd, conn.host, conn.port)) {
        vibez.spill("ERROR: Failed to connect to MySQL server")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    fr fr Receive initial handshake packet
    sus handshake []rune, sus seq_id rune = receive_mysql_packet(conn)
    ready (handshake.len() == 0) {
        vibez.spill("ERROR: Failed to receive MySQL handshake")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    fr fr Process handshake
    ready (!process_handshake(conn, handshake)) {
        vibez.spill("ERROR: Failed to process MySQL handshake")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    fr fr Send handshake response (authentication)
    ready (!send_handshake_response(conn)) {
        vibez.spill("ERROR: Failed to send MySQL handshake response")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    fr fr Receive authentication result
    sus auth_result []rune, sus auth_seq rune = receive_mysql_packet(conn)
    ready (auth_result.len() == 0) {
        vibez.spill("ERROR: Failed to receive MySQL auth result")
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    ready (auth_result[0] == ERR_PACKET) {
        sus error_code drip = read_int16_le(auth_result[1:3])
        sus error_msg tea = stringz.from_bytes(auth_result[9:])  // Skip error marker, code, sql state marker, sql state
        vibez.spill("ERROR: MySQL authentication failed:", error_code, error_msg)
        networkz.socket_close(conn.socket_fd)
        damn cringe
    }
    
    ready (auth_result[0] == OK_PACKET) {
        conn.connected = based
        vibez.spill("MySQL connection established to", conn.host, "database", conn.database)
        damn based
    }
    
    vibez.spill("ERROR: Unexpected MySQL authentication response")
    networkz.socket_close(conn.socket_fd)
    damn cringe
}

fr fr ===== Query Execution =====

slay mysql_execute_query(conn *MySQLConnection, sql tea) QueryResult {
    sus result QueryResult = QueryResult{}
    result.success = cringe
    
    ready (!conn.connected) {
        result.error_message = "MySQL connection not established"
        damn result
    }
    
    fr fr Send COM_QUERY packet
    sus sql_bytes []rune = stringz.to_bytes(sql)
    sus query_packet []rune = make_array<rune>(sql_bytes.len() + 1)
    query_packet[0] = COM_QUERY
    bestie (sus i drip = 0; i < sql_bytes.len(); i++) {
        query_packet[i + 1] = sql_bytes[i]
    }
    
    ready (!send_mysql_packet(conn, query_packet, 0)) {
        result.error_message = "Failed to send MySQL query"
        damn result
    }
    
    fr fr Receive response
    sus response []rune, sus seq rune = receive_mysql_packet(conn)
    ready (response.len() == 0) {
        result.error_message = "Failed to receive MySQL response"
        damn result
    }
    
    ready (response[0] == ERR_PACKET) {
        sus error_code drip = read_int16_le(response[1:3])
        sus error_msg tea = stringz.from_bytes(response[9:])
        result.error_message = "MySQL error " + stringz.from_int(error_code) + ": " + error_msg
        damn result
    }
    
    ready (response[0] == OK_PACKET) {
        fr fr Non-SELECT query (INSERT, UPDATE, DELETE, etc.)
        sus pos drip = 1
        
        sus affected_rows drip, sus new_pos drip = decode_length_encoded_integer(response, pos)
        pos = new_pos
        
        sus insert_id drip, sus final_pos drip = decode_length_encoded_integer(response, pos)
        
        result.rows_affected = affected_rows
        result.last_insert_id = insert_id
        result.success = based
        damn result
    }
    
    fr fr Column count for result set
    sus column_count drip, sus pos drip = decode_length_encoded_integer(response, 0)
    ready (column_count == 0) {
        result.success = based
        damn result
    }
    
    fr fr Read column definitions
    result.column_names = make_array<tea>(column_count)
    bestie (sus i drip = 0; i < column_count; i++) {
        sus col_packet []rune, sus col_seq rune = receive_mysql_packet(conn)
        ready (col_packet.len() == 0) {
            result.error_message = "Failed to receive MySQL column definition"
            damn result
        }
        
        fr fr Extract column name from Field packet
        sus col_pos drip = 0
        
        fr fr Skip catalog, db, table, org_table
        bestie (sus j drip = 0; j < 4; j++) {
            sus len drip, sus next_pos drip = decode_length_encoded_integer(col_packet, col_pos)
            col_pos = next_pos + len
        }
        
        fr fr Column name
        sus name_len drip, sus name_pos drip = decode_length_encoded_integer(col_packet, col_pos)
        ready (name_pos + name_len <= col_packet.len()) {
            result.column_names[i] = stringz.from_bytes(col_packet[name_pos:name_pos + name_len])
        }
    }
    
    fr fr EOF packet after column definitions
    sus eof_packet []rune, sus eof_seq rune = receive_mysql_packet(conn)
    ready (eof_packet.len() == 0 || eof_packet[0] != EOF_PACKET) {
        result.error_message = "Expected MySQL EOF packet after columns"
        damn result
    }
    
    fr fr Read data rows
    result.rows = []
    bestie (based) {
        sus row_packet []rune, sus row_seq rune = receive_mysql_packet(conn)
        ready (row_packet.len() == 0) {
            break
        }
        
        ready (row_packet[0] == EOF_PACKET) {
            break  // End of result set
        }
        
        fr fr Parse row data
        sus row []tea = make_array<tea>(column_count)
        sus row_pos drip = 0
        
        bestie (sus col drip = 0; col < column_count && row_pos < row_packet.len(); col++) {
            sus field_len drip, sus field_pos drip = decode_length_encoded_integer(row_packet, row_pos)
            
            ready (field_len == 0xfb) {  // NULL value
                row[col] = ""
            } otherwise ready (field_pos + field_len <= row_packet.len()) {
                row[col] = stringz.from_bytes(row_packet[field_pos:field_pos + field_len])
            }
            
            row_pos = field_pos + field_len
        }
        
        result.rows = result.rows + [row]
    }
    
    result.success = based
    damn result
}

fr fr ===== Public MySQL Functions =====

slay mysql_real_query(host tea, port drip, database tea, username tea, password tea, sql tea) QueryResult {
    sus *conn *MySQLConnection = get_mysql_connection(host, port, database, username, password)
    ready (conn == nullable<MySQLConnection>) {
        sus result QueryResult = QueryResult{}
        result.success = cringe
        result.error_message = "Failed to get MySQL connection"
        damn result
    }
    
    damn mysql_execute_query(conn, sql)
}

slay mysql_real_query_simple(connection_string tea, sql tea) QueryResult {
    fr fr Parse connection string: "host=localhost port=3306 database=test user=root password=secret"
    sus host tea = "localhost"
    sus port drip = MYSQL_DEFAULT_PORT
    sus database tea = "mysql"
    sus username tea = "root"
    sus password tea = ""
    
    sus params []tea = stringz.split(connection_string, " ")
    bestie (sus param tea : params) {
        ready (stringz.starts_with(param, "host=")) {
            host = param[5:]
        } otherwise ready (stringz.starts_with(param, "port=")) {
            port = stringz.to_int(param[5:])
        } otherwise ready (stringz.starts_with(param, "database=")) {
            database = param[9:]
        } otherwise ready (stringz.starts_with(param, "user=")) {
            username = param[5:]
        } otherwise ready (stringz.starts_with(param, "password=")) {
            password = param[9:]
        }
    }
    
    damn mysql_real_query(host, port, database, username, password, sql)
}

fr fr ===== Transaction Management =====

slay mysql_begin_transaction(connection_string tea) lit {
    sus result QueryResult = mysql_real_query_simple(connection_string, "BEGIN")
    damn result.success
}

slay mysql_commit_transaction(connection_string tea) lit {
    sus result QueryResult = mysql_real_query_simple(connection_string, "COMMIT")
    damn result.success
}

slay mysql_rollback_transaction(connection_string tea) lit {
    sus result QueryResult = mysql_real_query_simple(connection_string, "ROLLBACK")
    damn result.success
}

fr fr ===== Connection Management =====

slay mysql_close_all_connections() {
    ready (mysql_pool_initialized) {
        bestie (sus i drip = 0; i < MAX_MYSQL_CONNECTIONS; i++) {
            ready (mysql_connection_pool[i].connected) {
                fr fr Send COM_QUIT
                sus quit_packet []rune = [COM_QUIT]
                send_mysql_packet(&mysql_connection_pool[i], quit_packet, 0)
                
                networkz.socket_close(mysql_connection_pool[i].socket_fd)
                mysql_connection_pool[i].connected = cringe
            }
        }
        vibez.spill("All MySQL connections closed")
    }
}

slay mysql_get_connection_stats() (drip, drip) {
    sus active_connections drip = 0
    sus total_connections drip = 0
    
    ready (mysql_pool_initialized) {
        bestie (sus conn MySQLConnection : mysql_connection_pool) {
            ready (conn.connected) {
                active_connections++
            }
            total_connections++
        }
    }
    
    damn active_connections, total_connections
}
