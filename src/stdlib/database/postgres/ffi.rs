/// FFI bindings to libpq for PostgreSQL connectivity
/// 
/// This module provides safe Rust bindings to the PostgreSQL libpq C library.
/// All memory management and error handling is done safely through these bindings.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

/// fr fr PostgreSQL connection handle (opaque pointer)
#[repr(C)]
pub struct PGconn {
    _private: [u8; 0],
}

/// fr fr PostgreSQL result handle (opaque pointer)  
#[repr(C)]
pub struct PGresult {
    _private: [u8; 0],
}

/// fr fr PostgreSQL connection status enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnStatusType {
    ConnectionOk = 0,
    ConnectionBad = 1,
    ConnectionStarted = 2,
    ConnectionMade = 3,
    ConnectionAwaitingResponse = 4,
    ConnectionAuthOk = 5,
    ConnectionSetenv = 6,
    ConnectionSslStartup = 7,
    ConnectionNeeded = 8,
    ConnectionCheckWritable = 9,
    ConnectionConsume = 10,
    ConnectionGssStartup = 11,
    ConnectionCheckTarget = 12,
    ConnectionCheckStandby = 13,
}

/// fr fr PostgreSQL result status enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecStatusType {
    PgresEmptyQuery = 0,
    PgresCommandOk = 1,
    PgresTuplesOk = 2,
    PgresCopyOut = 3,
    PgresCopyIn = 4,
    PgresBadResponse = 5,
    PgresNonfatalError = 6,
    PgresFatalError = 7,
    PgresCopyBoth = 8,
    PgresSingleTuple = 9,
    PgresPipelineSync = 10,
    PgresPipelineAborted = 11,
}

/// fr fr PostgreSQL data type OIDs
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgreSQLOid {
    Bool = 16,
    Bytea = 17,
    Char = 18,
    Name = 19,
    Int8 = 20,
    Int2 = 21,
    Int2Vector = 22,
    Int4 = 23,
    Regproc = 24,
    Text = 25,
    Oid = 26,
    Tid = 27,
    Xid = 28,
    Cid = 29,
    Varchar = 1043,
    Date = 1082,
    Time = 1083,
    Timestamp = 1114,
    Timestamptz = 1184,
    Interval = 1186,
    Numeric = 1700,
    Json = 114,
    Jsonb = 3802,
    Uuid = 2950,
    Inet = 869,
    Float4 = 700,
    Float8 = 701,
    // Array types (add 1000 to base type)
    BoolArray = 1000,
    ByteaArray = 1001,
    CharArray = 1002,
    NameArray = 1003,
    Int2Array = 1005,
    Int4Array = 1007,
    TextArray = 1009,
    VarcharArray = 1015,
    Float4Array = 1021,
    Float8Array = 1022,
}

/// fr fr Transaction isolation levels for PostgreSQL
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgreSQLIsolationLevel {
    ReadUncommitted = 1,
    ReadCommitted = 2,
    RepeatableRead = 3,
    Serializable = 4,
}

extern "C" {
    /// slay Connect to PostgreSQL database
    pub fn PQconnectdb(conninfo: *const c_char) -> *mut PGconn;
    
    /// slay Start asynchronous connection to PostgreSQL
    pub fn PQconnectStart(conninfo: *const c_char) -> *mut PGconn;
    
    /// slay Continue asynchronous connection
    pub fn PQconnectPoll(conn: *mut PGconn) -> i32;
    
    /// slay Close PostgreSQL connection
    pub fn PQfinish(conn: *mut PGconn);
    
    /// slay Check connection status
    pub fn PQstatus(conn: *mut PGconn) -> ConnStatusType;
    
    /// slay Get last error message
    pub fn PQerrorMessage(conn: *mut PGconn) -> *const c_char;
    
    /// slay Reset connection
    pub fn PQreset(conn: *mut PGconn);
    
    /// slay Execute SQL command
    pub fn PQexec(conn: *mut PGconn, command: *const c_char) -> *mut PGresult;
    
    /// slay Execute parameterized query
    pub fn PQexecParams(
        conn: *mut PGconn,
        command: *const c_char,
        nParams: c_int,
        paramTypes: *const c_uint,
        paramValues: *const *const c_char,
        paramLengths: *const c_int,
        paramFormats: *const c_int,
        resultFormat: c_int,
    ) -> *mut PGresult;
    
    /// slay Prepare a statement
    pub fn PQprepare(
        conn: *mut PGconn,
        stmtName: *const c_char,
        query: *const c_char,
        nParams: c_int,
        paramTypes: *const c_uint,
    ) -> *mut PGresult;
    
    /// slay Execute prepared statement
    pub fn PQexecPrepared(
        conn: *mut PGconn,
        stmtName: *const c_char,
        nParams: c_int,
        paramValues: *const *const c_char,
        paramLengths: *const c_int,
        paramFormats: *const c_int,
        resultFormat: c_int,
    ) -> *mut PGresult;
    
    /// slay Deallocate prepared statement
    pub fn PQdeallocate(conn: *mut PGconn, stmtName: *const c_char) -> *mut PGresult;
    
    /// slay Clear result set
    pub fn PQclear(result: *mut PGresult);
    
    /// slay Get result status
    pub fn PQresultStatus(result: *mut PGresult) -> ExecStatusType;
    
    /// slay Get result error message
    pub fn PQresultErrorMessage(result: *mut PGresult) -> *const c_char;
    
    /// slay Get result error field
    pub fn PQresultErrorField(result: *mut PGresult, fieldcode: c_int) -> *const c_char;
    
    /// slay Get number of tuples (rows)
    pub fn PQntuples(result: *mut PGresult) -> c_int;
    
    /// slay Get number of fields (columns)
    pub fn PQnfields(result: *mut PGresult) -> c_int;
    
    /// slay Get field name
    pub fn PQfname(result: *mut PGresult, column_number: c_int) -> *const c_char;
    
    /// slay Get field type OID
    pub fn PQftype(result: *mut PGresult, column_number: c_int) -> c_uint;
    
    /// slay Get field value
    pub fn PQgetvalue(result: *mut PGresult, row_number: c_int, column_number: c_int) -> *const c_char;
    
    /// slay Get field value length
    pub fn PQgetlength(result: *mut PGresult, row_number: c_int, column_number: c_int) -> c_int;
    
    /// slay Check if field value is null
    pub fn PQgetisnull(result: *mut PGresult, row_number: c_int, column_number: c_int) -> c_int;
    
    /// slay Get command status
    pub fn PQcmdStatus(result: *mut PGresult) -> *const c_char;
    
    /// slay Get affected rows
    pub fn PQcmdTuples(result: *mut PGresult) -> *const c_char;
    
    /// slay Enter copy mode
    pub fn PQputCopyData(conn: *mut PGconn, buffer: *const c_char, nbytes: c_int) -> c_int;
    
    /// slay End copy mode
    pub fn PQputCopyEnd(conn: *mut PGconn, errormsg: *const c_char) -> c_int;
    
    /// slay Get copy data
    pub fn PQgetCopyData(conn: *mut PGconn, buffer: *mut *mut c_char, async_: c_int) -> c_int;
    
    /// slay Escape string literal
    pub fn PQescapeLiteral(conn: *mut PGconn, str: *const c_char, length: usize) -> *mut c_char;
    
    /// slay Escape identifier
    pub fn PQescapeIdentifier(conn: *mut PGconn, str: *const c_char, length: usize) -> *mut c_char;
    
    /// slay Free escaped string
    pub fn PQfreemem(ptr: *mut c_void);
    
    /// slay Get PostgreSQL version
    pub fn PQserverVersion(conn: *mut PGconn) -> c_int;
    
    /// slay Get library version
    pub fn PQlibVersion() -> c_int;
    
    /// slay Set client encoding
    pub fn PQsetClientEncoding(conn: *mut PGconn, encoding: *const c_char) -> c_int;
    
    /// slay Get client encoding
    pub fn PQclientEncoding(conn: *mut PGconn) -> c_int;
    
    /// slay Begin transaction
    pub fn PQtransactionStatus(conn: *mut PGconn) -> c_int;
    
    /// slay Set notice receiver
    pub fn PQsetNoticeReceiver(
        conn: *mut PGconn,
        proc: extern "C" fn(*mut c_void, *const PGresult),
        arg: *mut c_void,
    );
    
    /// slay Get database name
    pub fn PQdb(conn: *mut PGconn) -> *const c_char;
    
    /// slay Get user name
    pub fn PQuser(conn: *mut PGconn) -> *const c_char;
    
    /// slay Get password
    pub fn PQpass(conn: *mut PGconn) -> *const c_char;
    
    /// slay Get host
    pub fn PQhost(conn: *mut PGconn) -> *const c_char;
    
    /// slay Get port
    pub fn PQport(conn: *mut PGconn) -> *const c_char;
    
    /// slay Get options
    pub fn PQoptions(conn: *mut PGconn) -> *const c_char;
}

/// fr fr Error field codes for detailed error information
pub mod error_field_codes {
    pub const SEVERITY: i32 = b'S' as i32;
    pub const SEVERITY_NONLOCALIZED: i32 = b'V' as i32;
    pub const SQLSTATE: i32 = b'C' as i32;
    pub const MESSAGE_PRIMARY: i32 = b'M' as i32;
    pub const MESSAGE_DETAIL: i32 = b'D' as i32;
    pub const MESSAGE_HINT: i32 = b'H' as i32;
    pub const STATEMENT_POSITION: i32 = b'P' as i32;
    pub const INTERNAL_POSITION: i32 = b'p' as i32;
    pub const INTERNAL_QUERY: i32 = b'q' as i32;
    pub const CONTEXT: i32 = b'W' as i32;
    pub const SCHEMA_NAME: i32 = b's' as i32;
    pub const TABLE_NAME: i32 = b't' as i32;
    pub const COLUMN_NAME: i32 = b'c' as i32;
    pub const DATATYPE_NAME: i32 = b'd' as i32;
    pub const CONSTRAINT_NAME: i32 = b'n' as i32;
    pub const SOURCE_FILE: i32 = b'F' as i32;
    pub const SOURCE_LINE: i32 = b'L' as i32;
    pub const SOURCE_FUNCTION: i32 = b'R' as i32;
}

/// fr fr Safe wrapper for PostgreSQL connection
pub struct SafePGconn {
    conn: *mut PGconn,
}

impl SafePGconn {
    /// slay Create a new connection wrapper
    pub fn new(conn: *mut PGconn) -> Option<Self> {
        if conn.is_null() {
            None
        } else {
            Some(Self { conn })
        }
    }
    
    /// slay Get raw connection pointer
    pub fn as_ptr(&self) -> *mut PGconn {
        self.conn
    }
    
    /// slay Check if connection is valid
    pub fn is_valid(&self) -> bool {
        !self.conn.is_null() && unsafe { PQstatus(self.conn) } == ConnStatusType::ConnectionOk
    }
}

impl Drop for SafePGconn {
    fn drop(&mut self) {
        if !self.conn.is_null() {
            unsafe {
                PQfinish(self.conn);
            }
        }
    }
}

unsafe impl Send for SafePGconn {}
unsafe impl Sync for SafePGconn {}

/// fr fr Safe wrapper for PostgreSQL result
pub struct SafePGresult {
    result: *mut PGresult,
}

impl SafePGresult {
    /// slay Create a new result wrapper
    pub fn new(result: *mut PGresult) -> Option<Self> {
        if result.is_null() {
            None
        } else {
            Some(Self { result })
        }
    }
    
    /// slay Get raw result pointer
    pub fn as_ptr(&self) -> *mut PGresult {
        self.result
    }
}

impl Drop for SafePGresult {
    fn drop(&mut self) {
        if !self.result.is_null() {
            unsafe {
                PQclear(self.result);
            }
        }
    }
}

unsafe impl Send for SafePGresult {}
unsafe impl Sync for SafePGresult {}

/// fr fr Helper functions for FFI operations
impl SafePGconn {
    /// slay Connect to database using connection string
    pub fn connect(conninfo: &str) -> Result<Self, String> {
        let c_conninfo = CString::new(conninfo).map_err(|e| format!("Invalid connection string: {}", e))?;
        
        let conn = unsafe { PQconnectdb(c_conninfo.as_ptr()) };
        
        if let Some(safe_conn) = Self::new(conn) {
            if safe_conn.is_valid() {
                Ok(safe_conn)
            } else {
                let error_msg = unsafe {
                    let ptr = PQerrorMessage(conn);
                    if ptr.is_null() {
                        "Unknown connection error".to_string()
                    } else {
                        CStr::from_ptr(ptr).to_string_lossy().to_string()
                    }
                };
                Err(error_msg)
            }
        } else {
            Err("Failed to create connection".to_string())
        }
    }
    
    /// slay Execute SQL query
    pub fn exec(&self, query: &str) -> Result<SafePGresult, String> {
        let c_query = CString::new(query).map_err(|e| format!("Invalid query: {}", e))?;
        
        let result = unsafe { PQexec(self.conn, c_query.as_ptr()) };
        
        if let Some(safe_result) = SafePGresult::new(result) {
            let status = unsafe { PQresultStatus(safe_result.as_ptr()) };
            
            match status {
                ExecStatusType::PgresCommandOk | ExecStatusType::PgresTuplesOk => {
                    Ok(safe_result)
                }
                _ => {
                    let error_msg = unsafe {
                        let ptr = PQresultErrorMessage(safe_result.as_ptr());
                        if ptr.is_null() {
                            "Unknown query error".to_string()
                        } else {
                            CStr::from_ptr(ptr).to_string_lossy().to_string()
                        }
                    };
                    Err(error_msg)
                }
            }
        } else {
            Err("Failed to execute query".to_string())
        }
    }
    
    /// slay Get connection error message
    pub fn error_message(&self) -> String {
        unsafe {
            let ptr = PQerrorMessage(self.conn);
            if ptr.is_null() {
                "No error message available".to_string()
            } else {
                CStr::from_ptr(ptr).to_string_lossy().to_string()
            }
        }
    }
    
    /// slay Get database information
    pub fn database_info(&self) -> DatabaseInfo {
        unsafe {
            let db = PQdb(self.conn);
            let user = PQuser(self.conn);
            let host = PQhost(self.conn);
            let port = PQport(self.conn);
            
            DatabaseInfo {
                database: if db.is_null() { "unknown".to_string() } else { CStr::from_ptr(db).to_string_lossy().to_string() },
                user: if user.is_null() { "unknown".to_string() } else { CStr::from_ptr(user).to_string_lossy().to_string() },
                host: if host.is_null() { "localhost".to_string() } else { CStr::from_ptr(host).to_string_lossy().to_string() },
                port: if port.is_null() { 5432 } else { CStr::from_ptr(port).to_string_lossy().parse().unwrap_or(5432) },
            }
        }
    }
}

/// fr fr Database connection information
#[derive(Debug, Clone)]
pub struct DatabaseInfo {
    pub database: String,
    pub user: String,
    pub host: String,
    pub port: u16,
}

/// fr fr Helper functions for result processing
impl SafePGresult {
    /// slay Get number of rows
    pub fn ntuples(&self) -> i32 {
        unsafe { PQntuples(self.result) }
    }
    
    /// slay Get number of columns
    pub fn nfields(&self) -> i32 {
        unsafe { PQnfields(self.result) }
    }
    
    /// slay Get field name
    pub fn field_name(&self, column: i32) -> String {
        unsafe {
            let ptr = PQfname(self.result, column);
            if ptr.is_null() {
                format!("column_{}", column)
            } else {
                CStr::from_ptr(ptr).to_string_lossy().to_string()
            }
        }
    }
    
    /// slay Get field type OID
    pub fn field_type(&self, column: i32) -> u32 {
        unsafe { PQftype(self.result, column) }
    }
    
    /// slay Get field value
    pub fn get_value(&self, row: i32, column: i32) -> Option<Vec<u8>> {
        unsafe {
            if PQgetisnull(self.result, row, column) != 0 {
                None
            } else {
                let ptr = PQgetvalue(self.result, row, column);
                let len = PQgetlength(self.result, row, column);
                
                if ptr.is_null() || len < 0 {
                    None
                } else {
                    let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
                    Some(slice.to_vec())
                }
            }
        }
    }
    
    /// slay Get command status
    pub fn command_status(&self) -> String {
        unsafe {
            let ptr = PQcmdStatus(self.result);
            if ptr.is_null() {
                "UNKNOWN".to_string()
            } else {
                CStr::from_ptr(ptr).to_string_lossy().to_string()
            }
        }
    }
    
    /// slay Get affected row count
    pub fn affected_rows(&self) -> i64 {
        unsafe {
            let ptr = PQcmdTuples(self.result);
            if ptr.is_null() {
                0
            } else {
                CStr::from_ptr(ptr).to_string_lossy().parse().unwrap_or(0)
            }
        }
    }
    
    /// slay Get detailed error information
    pub fn error_details(&self) -> ErrorDetails {
        unsafe {
            ErrorDetails {
                severity: self.get_error_field(error_field_codes::SEVERITY),
                sqlstate: self.get_error_field(error_field_codes::SQLSTATE),
                message: self.get_error_field(error_field_codes::MESSAGE_PRIMARY),
                detail: self.get_error_field(error_field_codes::MESSAGE_DETAIL),
                hint: self.get_error_field(error_field_codes::MESSAGE_HINT),
                position: self.get_error_field(error_field_codes::STATEMENT_POSITION),
                context: self.get_error_field(error_field_codes::CONTEXT),
                schema_name: self.get_error_field(error_field_codes::SCHEMA_NAME),
                table_name: self.get_error_field(error_field_codes::TABLE_NAME),
                column_name: self.get_error_field(error_field_codes::COLUMN_NAME),
                constraint_name: self.get_error_field(error_field_codes::CONSTRAINT_NAME),
            }
        }
    }
    
    fn get_error_field(&self, field_code: i32) -> Option<String> {
        unsafe {
            let ptr = PQresultErrorField(self.result, field_code);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
            }
        }
    }
}

/// fr fr Detailed error information from PostgreSQL
#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub severity: Option<String>,
    pub sqlstate: Option<String>,
    pub message: Option<String>,
    pub detail: Option<String>,
    pub hint: Option<String>,
    pub position: Option<String>,
    pub context: Option<String>,
    pub schema_name: Option<String>,
    pub table_name: Option<String>,
    pub column_name: Option<String>,
    pub constraint_name: Option<String>,
}
