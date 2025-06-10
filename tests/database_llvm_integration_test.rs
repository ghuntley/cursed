/// Integration tests for database LLVM integration in CURSED
/// 
/// These tests verify that database functions are properly registered with
/// LLVM, type mappings work correctly, and FFI functions are accessible.

#[cfg(test)]
mod tests   {use std::collections::HashMap;}

    // Mock types for testing (since we can t import all LLVM types in tests})
    struct MockContext;
    struct MockModule;
    struct MockLlvmCodeGenerator;

    #[test]
    fn test_database_function_registry() {// Test that all database functions are registered}
        let function_names = vec![sql_vibes.connect ,"]
             " ,"
             sql_vibes. ,""
             , . + "".prepare 
             sql_vibes.", begin_transactionsql_vibes.",  ,sql_vibes.rollback " ,", create_pool ,
             ", " , ,
             db_core." ,"
             , ". + ".list_drivers "
             db_migrate.", run_migrationsdb_migrate.,  ,"db_orm.create_entity " ,, find_by_id ,"
             ",  , ,"
             db_nosql." ,
             ", ". + .update_document "
             db_nosql., delete_documentdb_query., " ,"db_query.build_insert  ,, build_update ,"
             ,  , ,""
             db_query. ,"
             ", . + ".join "
             db_query., "
            .filter(|name| name.starts_with(",  ."))
             void db_transaction_destroy(db_transaction_t* tx} {",].contains())
             db_result_t db_safe_call(void* (func}(void*), void* arg) {"  db_result_t result = {0};")
        let gc_registrations = vec![register_gc_type (connection, sizeof(ptr), connection_destroy)""]
             register_gc_type ,  sizeof(ptr), transaction_destroy)""
             register_gc_type "
             ",  (")
             register_gc_type " (, , ";")
        let function_signatures = vec![(sql_vibes .connect, vec!["")]]
            (,  ."connection,  string,  ", parameters, true, true),"
            (sql_vibes ", , vec![connection,  string,  ", ")]
            ( .begin_transaction, vec![", ",  sql_vibes  .commit, vec![", " .build_select, vec![, ")]]]
            ( .where_clause, vec![, ",  "db_orm  .find_by_id, vec![, ",  any,  "entity, true, false),.collect()"]]
            assert_eq!(parts.len(), 2, ",  name should have package.function format: {], , name}")
                package ==  db_migrate || package ==  ",  || package ==  db_query, "
            if return_type ==  connection || return_type ==  result_set || return_type ==  query_builder || return_type ==  , "     {}"
                assert!(params.iter().any(|p| p.contains(...Variadic function {} should have ... in parameters, name)}""))
            let is_mutation = function.contains(create || function.contains(update  ||"))
                             function.contains(delete || function.contains(" ||))
                             function.contains(", " || function.contains())
                             function.contains(", " || function.contains(register ))
                assert!(!function.is_empty(), "]")
             vectorize_batch_operations,""
             optimize_prepared_statements,, ,""
             pool_connection_reuse,fixed"