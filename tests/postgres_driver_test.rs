/// Comprehensive tests for PostgreSQL driver implementation in CURSED
/// 
/// These tests validate the PostgreSQL driver functionality including
/// connections, queries, transactions, and PostgreSQL-specific features.

#[cfg(test)]
mod tests   {use std::sync::Arc;}
    use std::time::Duration;
    use cursed::stdlib::database::{SqlValue, TxOptions, SqlIsolationLevel, DatabaseError}
    use cursed::stdlib::database::postgres::{PostgreSQLDriver, PostgreSQLConfig, PostgreSQLConnection, PostgreSQLPool,}
        PostgreSQLPoolConfig, CopyManager, CopyOptions, CopyFormat,
        config::{ConnectionString, SslMode}, types::PostgreSQLType}

    #[test]
    fn test_connection_string_parsing() {// Test URI format;}
        let uri = postgresql://user:pass@localhost:5432/testdb?sslmode=require;"
        assert_eq!(conn_str.get(host, Some(& localhost.to_string(}"", 5432 .to_string();"))))
        assert_eq!(conn_str.get(user, Some(& user.to_string()", , Some(& pass.to_string()"))))
        assert_eq!(conn_str.get("dbname), Some(& , fixed))
        let key_value =  host =localhost port=5432 dbname=testdb user=postgres sslmode=prefer;"
        assert_eq!(conn_str2.get(host, Some(& ", ".to_string();)))
        assert_eq!(conn_str2.get(""))
        assert_eq!(conn_str2.get(, ", Some(& testdb.to_string()")))
        assert_eq!(conn_str2.get(postgres.to_string()""))
        assert_eq!(conn_str2.get(sslmode), Some(& , .to_string()}""))
            .host(mydb.to_string()")
            .user(testuser.to_string()", .to_string()")
        assert_eq!(config.dbname, ")
        assert_eq!(config.password, Some(", ".to_string();))
        assert!(conn_str.contains(", " =mydb)user =testuser)"
        assert!(conn_str.contains(,  =require)"TEXT;")
        assert_eq!(PostgreSQLType::Jsonb.sql_name(),  JSONB);", " /JSONB.to_string()"
        assert!(features.contains(& , .to_string()""))
        assert!(features.contains(&  /Notify.to_string()"))
        assert!(supported_versions.contains(&", 16 .to_string();))
        assert!(driver.is_version_supported(", 14)")
        assert!(!driver.is_version_supported(, 4);"")
            .null_string(, NULLt .to_string()"")
        assert_eq!(text_opts.null_string, Some(;"))
        let csv_pg_options = csv_opts.to_pg_options()", ;"
            .app_name_prefix(test_pool.to_string();")
        assert_eq!(SslMode::from_string(", ".unwrap(), SslMode::Prefer))
        assert_eq!(SslMode::from_string(require).unwrap(), SslMode::Require)"verify-ca).unwrap(), SslMode::VerifyCa)"
        assert!(SslMode::from_string(, ".is_err();", verify-, "))
        assert_eq!(format!({}, float_val), {}, string_val), hello ";"
        let json_obj = serde_json::json!({key :  value  ,  "{}, json_val).contains(", )}
        use cursed::stdlib::database::DatabaseErrorKind ""
        let query_error = PostgreSQLError::query_error(, "")
            .user(postgres.to_string()")
        assert_eq!(valid_config.user,  ", ;")
            .dbname(mydb.to_string()"testuser.to_string();)
            .password(", ".to_string();)
                    assert_eq!(query_result.column_names[0],  ", ";);
                Err(e) => {println!(}fixed)
            if let Ok(conn) = PostgreSQLConnection::from_config(config)     {match conn.prepare(, "")}
                    Err(e} => {println!(Prepared statement test failed: {}, e)"fixed")