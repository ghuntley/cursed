/// fr fr FFI bindings to libsqlite3 - low-level interface that slays periodt
/// 
/// This module provides safe Rust bindings to the SQLite C API.
/// All unsafe operations are wrapped in safe interfaces with proper
/// error handling and memory management.

use std::ffi::{CString, CStr, c_void, c_char, c_int, c_double};
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::{SqliteError, SqliteResult, SqliteType, SqliteVersion, SqliteFeatures};

/// fr fr SQLite result codes (subset of important ones)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteResultCode {
impl SqliteResultCode {
    /// slay Convert from raw SQLite result code
    pub fn from_raw(code: c_int) -> Self {
        match code {
        }
    }

    /// slay Check if result indicates success
    pub fn is_ok(self) -> bool {
        matches!(self, SqliteResultCode::Ok | SqliteResultCode::Row | SqliteResultCode::Done)
    /// slay Get error message for result code
    pub fn message(self) -> &'static str {
        match self {
            SqliteResultCode::IoErr => "Some kind of disk I/O error",
        }
    }
/// fr fr SQLite database handle (opaque pointer)
#[derive(Debug)]
pub struct SqliteHandle {
impl SqliteHandle {
    /// slay Create new handle from raw pointer
    pub unsafe fn from_raw(ptr: *mut c_void, path: String, open_flags: i32) -> SqliteResult<Self> {
        NonNull::new(ptr)
            .map(|ptr| Self { ptr, path, open_flags })
            .ok_or_else(|| SqliteError::null_pointer("Invalid SQLite database handle"))
    /// slay Get raw pointer
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    /// slay Get database path
    pub fn path(&self) -> &str {
        &self.path
    /// slay Get open flags
    pub fn open_flags(&self) -> i32 {
        self.open_flags
    }
}

unsafe impl Send for SqliteHandle {}
unsafe impl Sync for SqliteHandle {}

/// fr fr SQLite prepared statement handle
#[derive(Debug)]
pub struct SqliteStmtHandle {
impl SqliteStmtHandle {
    /// slay Create new statement handle from raw pointer
    pub unsafe fn from_raw(
    ) -> SqliteResult<Self> {
        NonNull::new(ptr)
            .map(|ptr| Self { ptr, sql, parameter_count, column_count })
            .ok_or_else(|| SqliteError::null_pointer("Invalid SQLite statement handle"))
    /// slay Get raw pointer
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    /// slay Get SQL text
    pub fn sql(&self) -> &str {
        &self.sql
    /// slay Get parameter count
    pub fn parameter_count(&self) -> i32 {
        self.parameter_count
    /// slay Get column count
    pub fn column_count(&self) -> i32 {
        self.column_count
    }
}

unsafe impl Send for SqliteStmtHandle {}
unsafe impl Sync for SqliteStmtHandle {}

/// fr fr SQLite backup handle
#[derive(Debug)]
pub struct SqliteBackupHandle {
impl SqliteBackupHandle {
    /// slay Create new backup handle from raw pointer
    pub unsafe fn from_raw(
    ) -> SqliteResult<Self> {
        NonNull::new(ptr)
            .map(|ptr| Self { ptr, source_name, dest_name })
            .ok_or_else(|| SqliteError::null_pointer("Invalid SQLite backup handle"))
    /// slay Get raw pointer
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    /// slay Get source database name
    pub fn source_name(&self) -> &str {
        &self.source_name
    /// slay Get destination database name
    pub fn dest_name(&self) -> &str {
        &self.dest_name
    }
}

unsafe impl Send for SqliteBackupHandle {}
unsafe impl Sync for SqliteBackupHandle {}

/// fr fr Main SQLite FFI interface
pub struct SqliteFFI;

impl SqliteFFI {
    /// slay Initialize SQLite library
    pub fn initialize() -> SqliteResult<()> {
        let result = unsafe { sqlite3_initialize() };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, "Failed to initialize SQLite"))
        }
    }

    /// slay Shutdown SQLite library
    pub fn shutdown() -> SqliteResult<()> {
        let result = unsafe { sqlite3_shutdown() };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, "Failed to shutdown SQLite"))
        }
    }

    /// slay Open database connection
    pub fn open(path: &str, flags: i32) -> SqliteResult<SqliteHandle> {
        let c_path = CString::new(path)
            .map_err(|_| SqliteError::invalid_parameter("Invalid database path"))?;
        
        let mut db_ptr: *mut c_void = ptr::null_mut();
        let result = unsafe {
            sqlite3_open_v2(
            )

        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            unsafe { SqliteHandle::from_raw(db_ptr, path.to_string(), flags) }
        } else {
            // Clean up on error
            if !db_ptr.is_null() {
                unsafe { sqlite3_close_v2(db_ptr) };
            }
            Err(SqliteError::ffi_error(code, &format!("Failed to open database: {}", path)))
        }
    }

    /// slay Close database connection
    pub fn close(handle: &SqliteHandle) -> SqliteResult<()> {
        let result = unsafe { sqlite3_close_v2(handle.as_ptr()) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, "Failed to close database"))
        }
    }

    /// slay Prepare SQL statement
    pub fn prepare(db: &SqliteHandle, sql: &str) -> SqliteResult<SqliteStmtHandle> {
        let c_sql = CString::new(sql)
            .map_err(|_| SqliteError::invalid_parameter("Invalid SQL statement"))?;
        
        let mut stmt_ptr: *mut c_void = ptr::null_mut();
        let mut tail_ptr: *const c_char = ptr::null();
        
        let result = unsafe {
            sqlite3_prepare_v2(
            )

        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            let parameter_count = unsafe { sqlite3_bind_parameter_count(stmt_ptr) };
            let column_count = unsafe { sqlite3_column_count(stmt_ptr) };
            
            unsafe { 
                SqliteStmtHandle::from_raw(stmt_ptr, sql.to_string(), parameter_count, column_count) 
            }
        } else {
            let error_msg = Self::get_error_message(db).unwrap_or_else(|_| "Unknown error".to_string());
            Err(SqliteError::ffi_error(code, &format!("Failed to prepare statement: {}", error_msg)))
        }
    }

    /// slay Finalize prepared statement
    pub fn finalize(stmt: &SqliteStmtHandle) -> SqliteResult<()> {
        let result = unsafe { sqlite3_finalize(stmt.as_ptr()) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, "Failed to finalize statement"))
        }
    }

    /// slay Execute statement step
    pub fn step(stmt: &SqliteStmtHandle) -> SqliteResult<SqliteResultCode> {
        let result = unsafe { sqlite3_step(stmt.as_ptr()) };
        Ok(SqliteResultCode::from_raw(result))
    /// slay Reset prepared statement
    pub fn reset(stmt: &SqliteStmtHandle) -> SqliteResult<()> {
        let result = unsafe { sqlite3_reset(stmt.as_ptr()) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, "Failed to reset statement"))
        }
    }

    /// slay Clear bindings on prepared statement
    pub fn clear_bindings(stmt: &SqliteStmtHandle) -> SqliteResult<()> {
        let result = unsafe { sqlite3_clear_bindings(stmt.as_ptr()) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, "Failed to clear bindings"))
        }
    }

    /// slay Bind NULL value
    pub fn bind_null(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<()> {
        let result = unsafe { sqlite3_bind_null(stmt.as_ptr(), index) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, &format!("Failed to bind NULL at index {}", index)))
        }
    }

    /// slay Bind integer value
    pub fn bind_int64(stmt: &SqliteStmtHandle, index: i32, value: i64) -> SqliteResult<()> {
        let result = unsafe { sqlite3_bind_int64(stmt.as_ptr(), index, value) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, &format!("Failed to bind int64 at index {}", index)))
        }
    }

    /// slay Bind double value
    pub fn bind_double(stmt: &SqliteStmtHandle, index: i32, value: f64) -> SqliteResult<()> {
        let result = unsafe { sqlite3_bind_double(stmt.as_ptr(), index, value) };
        let code = SqliteResultCode::from_raw(result);
        
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, &format!("Failed to bind double at index {}", index)))
        }
    }

    /// slay Bind text value
    pub fn bind_text(stmt: &SqliteStmtHandle, index: i32, value: &str) -> SqliteResult<()> {
        let c_value = CString::new(value)
            .map_err(|_| SqliteError::invalid_parameter("Invalid text value"))?;
        
        let result = unsafe {
            sqlite3_bind_text(
            )
        
        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, &format!("Failed to bind text at index {}", index)))
        }
    }

    /// slay Bind blob value
    pub fn bind_blob(stmt: &SqliteStmtHandle, index: i32, value: &[u8]) -> SqliteResult<()> {
        let result = unsafe {
            sqlite3_bind_blob(
            )
        
        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            Ok(())
        } else {
            Err(SqliteError::ffi_error(code, &format!("Failed to bind blob at index {}", index)))
        }
    }

    /// slay Get column type
    pub fn column_type(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<SqliteType> {
        let type_code = unsafe { sqlite3_column_type(stmt.as_ptr(), index) };
        Ok(SqliteType::from_code(type_code))
    /// slay Get column name
    pub fn column_name(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<String> {
        let name_ptr = unsafe { sqlite3_column_name(stmt.as_ptr(), index) };
        if name_ptr.is_null() {
            return Err(SqliteError::null_pointer("Column name is null"));
        let c_str = unsafe { CStr::from_ptr(name_ptr) };
        c_str.to_str()
            .map(|s| s.to_string())
            .map_err(|_| SqliteError::encoding_error("Invalid column name encoding"))
    /// slay Get column as int64
    pub fn column_int64(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<i64> {
        let value = unsafe { sqlite3_column_int64(stmt.as_ptr(), index) };
        Ok(value)
    /// slay Get column as double
    pub fn column_double(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<f64> {
        let value = unsafe { sqlite3_column_double(stmt.as_ptr(), index) };
        Ok(value)
    /// slay Get column as text
    pub fn column_text(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<String> {
        let text_ptr = unsafe { sqlite3_column_text(stmt.as_ptr(), index) };
        if text_ptr.is_null() {
            return Ok(String::new());
        let c_str = unsafe { CStr::from_ptr(text_ptr as *const c_char) };
        c_str.to_str()
            .map(|s| s.to_string())
            .map_err(|_| SqliteError::encoding_error("Invalid text encoding"))
    /// slay Get column as blob
    pub fn column_blob(stmt: &SqliteStmtHandle, index: i32) -> SqliteResult<Vec<u8>> {
        let blob_ptr = unsafe { sqlite3_column_blob(stmt.as_ptr(), index) };
        let blob_size = unsafe { sqlite3_column_bytes(stmt.as_ptr(), index) };
        
        if blob_ptr.is_null() || blob_size == 0 {
            return Ok(Vec::new());
        let blob_slice = unsafe {
            std::slice::from_raw_parts(blob_ptr as *const u8, blob_size as usize)
        
        Ok(blob_slice.to_vec())
    /// slay Get last insert row ID
    pub fn last_insert_rowid(db: &SqliteHandle) -> SqliteResult<i64> {
        let rowid = unsafe { sqlite3_last_insert_rowid(db.as_ptr()) };
        Ok(rowid)
    /// slay Get number of changes from last statement
    pub fn changes(db: &SqliteHandle) -> SqliteResult<i32> {
        let changes = unsafe { sqlite3_changes(db.as_ptr()) };
        Ok(changes)
    /// slay Get error message from database
    pub fn get_error_message(db: &SqliteHandle) -> SqliteResult<String> {
        let error_ptr = unsafe { sqlite3_errmsg(db.as_ptr()) };
        if error_ptr.is_null() {
            return Ok("Unknown error".to_string());
        let c_str = unsafe { CStr::from_ptr(error_ptr) };
        c_str.to_str()
            .map(|s| s.to_string())
            .map_err(|_| SqliteError::encoding_error("Invalid error message encoding"))
    /// slay Begin transaction
    pub fn begin_transaction(db: &SqliteHandle, transaction_type: &str) -> SqliteResult<()> {
        let sql = format!("BEGIN {}", transaction_type);
        let c_sql = CString::new(sql)
            .map_err(|_| SqliteError::invalid_parameter("Invalid transaction type"))?;
        
        let result = unsafe {
            sqlite3_exec(
            )
        
        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            Ok(())
        } else {
            let error_msg = Self::get_error_message(db).unwrap_or_else(|_| "Unknown error".to_string());
            Err(SqliteError::ffi_error(code, &format!("Failed to begin transaction: {}", error_msg)))
        }
    }

    /// slay Commit transaction
    pub fn commit_transaction(db: &SqliteHandle) -> SqliteResult<()> {
        let result = unsafe {
            sqlite3_exec(
            )
        
        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            Ok(())
        } else {
            let error_msg = Self::get_error_message(db).unwrap_or_else(|_| "Unknown error".to_string());
            Err(SqliteError::ffi_error(code, &format!("Failed to commit transaction: {}", error_msg)))
        }
    }

    /// slay Rollback transaction
    pub fn rollback_transaction(db: &SqliteHandle) -> SqliteResult<()> {
        let result = unsafe {
            sqlite3_exec(
            )
        
        let code = SqliteResultCode::from_raw(result);
        if code.is_ok() {
            Ok(())
        } else {
            let error_msg = Self::get_error_message(db).unwrap_or_else(|_| "Unknown error".to_string());
            Err(SqliteError::ffi_error(code, &format!("Failed to rollback transaction: {}", error_msg)))
        }
    }

    /// slay Get SQLite version
    pub fn get_version() -> SqliteResult<SqliteVersion> {
        let version_ptr = unsafe { sqlite3_libversion() };
        if version_ptr.is_null() {
            return Err(SqliteError::null_pointer("Version string is null"));
        let c_str = unsafe { CStr::from_ptr(version_ptr) };
        let version_str = c_str.to_str()
            .map_err(|_| SqliteError::encoding_error("Invalid version encoding"))?;
        
        let version_number = unsafe { sqlite3_libversion_number() };
        
        Ok(SqliteVersion {
        })
    /// slay Get SQLite source ID
    pub fn get_source_id() -> SqliteResult<String> {
        let source_ptr = unsafe { sqlite3_sourceid() };
        if source_ptr.is_null() {
            return Ok("Unknown".to_string());
        let c_str = unsafe { CStr::from_ptr(source_ptr) };
        c_str.to_str()
            .map(|s| s.to_string())
            .map_err(|_| SqliteError::encoding_error("Invalid source ID encoding"))
    /// slay Check if feature is compiled in
    pub fn is_feature_compiled(feature: &str) -> SqliteResult<bool> {
        let c_feature = CString::new(feature)
            .map_err(|_| SqliteError::invalid_parameter("Invalid feature name"))?;
        
        let result = unsafe { sqlite3_compileoption_used(c_feature.as_ptr()) };
        Ok(result != 0)
    /// slay Get compile options
    pub fn get_compile_options() -> SqliteResult<Vec<String>> {
        let mut options = Vec::new();
        let mut index = 0;
        
        loop {
            let option_ptr = unsafe { sqlite3_compileoption_get(index) };
            if option_ptr.is_null() {
                break;
            let c_str = unsafe { CStr::from_ptr(option_ptr) };
            let option_str = c_str.to_str()
                .map_err(|_| SqliteError::encoding_error("Invalid compile option encoding"))?;
            
            options.push(option_str.to_string());
            index += 1;
        Ok(options)
    }
}

// SQLite constants
const SQLITE_TRANSIENT: unsafe extern "C" fn(*mut c_void) = {
    unsafe extern "C" fn transient(_: *mut c_void) {}
    transient
// };

// Alternative destructor that indicates SQLite should make its own copy
const SQLITE_TRANSIENT_DESTRUCTOR: *const c_void = -1isize as *const c_void;

// SQLite C API function declarations
extern "C" {
    fn sqlite3_initialize() -> c_int;
    fn sqlite3_shutdown() -> c_int;
    fn sqlite3_open_v2(
    ) -> c_int;
    fn sqlite3_close_v2(db: *mut c_void) -> c_int;
    fn sqlite3_prepare_v2(
    ) -> c_int;
    fn sqlite3_finalize(pstmt: *mut c_void) -> c_int;
    fn sqlite3_step(pstmt: *mut c_void) -> c_int;
    fn sqlite3_reset(pstmt: *mut c_void) -> c_int;
    fn sqlite3_clear_bindings(pstmt: *mut c_void) -> c_int;
    fn sqlite3_bind_parameter_count(pstmt: *mut c_void) -> c_int;
    fn sqlite3_column_count(pstmt: *mut c_void) -> c_int;
    fn sqlite3_bind_null(pstmt: *mut c_void, index: c_int) -> c_int;
    fn sqlite3_bind_int64(pstmt: *mut c_void, index: c_int, value: i64) -> c_int;
    fn sqlite3_bind_double(pstmt: *mut c_void, index: c_int, value: c_double) -> c_int;
    fn sqlite3_bind_text(
    ) -> c_int;
    fn sqlite3_bind_blob(
    ) -> c_int;
    fn sqlite3_column_type(pstmt: *mut c_void, icol: c_int) -> c_int;
    fn sqlite3_column_name(pstmt: *mut c_void, n: c_int) -> *const c_char;
    fn sqlite3_column_int64(pstmt: *mut c_void, icol: c_int) -> i64;
    fn sqlite3_column_double(pstmt: *mut c_void, icol: c_int) -> c_double;
    fn sqlite3_column_text(pstmt: *mut c_void, icol: c_int) -> *const u8;
    fn sqlite3_column_blob(pstmt: *mut c_void, icol: c_int) -> *const c_void;
    fn sqlite3_column_bytes(pstmt: *mut c_void, icol: c_int) -> c_int;
    fn sqlite3_last_insert_rowid(db: *mut c_void) -> i64;
    fn sqlite3_changes(db: *mut c_void) -> c_int;
    fn sqlite3_errmsg(db: *mut c_void) -> *const c_char;
    fn sqlite3_exec(
        callback: Option<unsafe extern "C" fn(
    ) -> c_int;
    fn sqlite3_libversion() -> *const c_char;
    fn sqlite3_libversion_number() -> c_int;
    fn sqlite3_sourceid() -> *const c_char;
    fn sqlite3_compileoption_used(zoptname: *const c_char) -> c_int;
    fn sqlite3_compileoption_get(n: c_int) -> *const c_char;
