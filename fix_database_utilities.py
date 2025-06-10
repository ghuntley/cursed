#!/usr/bin/env python3

import re

# Read the file
with open('tests/database_test_utilities.rs', 'r') as f:
    content = f.read()

# Fix parameter passing - convert Vec<SqlValue> to &[Parameter]
# Pattern 1: .query(sql, vec![...]).await
pattern1 = r'\.query\(([^,]+),\s*vec!\[([^\]]+)\]\)\.await'
def fix_query_params(match):
    sql = match.group(1)
    params = match.group(2)
    return f'.query({sql}, &sql_values_to_parameters(&vec![{params}])).await'

content = re.sub(pattern1, fix_query_params, content)

# Pattern 2: .execute(sql, vec![]).await
content = re.sub(r'\.execute\(([^,]+),\s*vec!\[\]\)\.await', r'.execute(\1, &[]).await', content)

# Pattern 3: .execute(sql, vec![...]).await  
pattern3 = r'\.execute\(([^,]+),\s*vec!\[([^\]]+)\]\)\.await'
def fix_execute_params(match):
    sql = match.group(1)
    params = match.group(2)
    return f'.execute({sql}, &sql_values_to_parameters(&vec![{params}])).await'

content = re.sub(pattern3, fix_execute_params, content)

# Fix row count comparison
content = re.sub(r'result\.row_count\(\)\s*>\s*(\d+)', r'result.row_count().unwrap_or(0) > \1', content)

# Fix rows access - replace result.rows()[0] with result iteration
pattern4 = r'result\.rows\(\)\[0\]\.get_(\w+)\("([^"]+)"\)\?'
def fix_rows_access(match):
    method = match.group(1)
    column = match.group(2)
    return f'{{ let mut result = result; if let Some(row) = result.next()? {{ row.get_{method}("{column}")? }} else {{ panic!("No rows returned") }} }}'

# This is complex pattern, let's handle it case by case in the specific contexts
# For now, let's fix the fake() method calls
content = re.sub(r'\((\d+)\.\.(\d+)\)\.fake\(\)', r'thread_rng().gen_range(\1..\2)', content)

# Fix Transaction::new() calls - it needs parameters
content = re.sub(r'Transaction::new\(\)', r'Transaction::new("mock_connection", TransactionOptions::default())', content)

# Fix missing trait implementations
missing_methods = '''
        async fn prepare(&mut self, _sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
            todo!("Mock prepare not implemented")
        }

        async fn ping(&mut self) -> DbResult<()> {
            Ok(())
        }

        fn connection_info(&self) -> ConnectionInfo {
            ConnectionInfo {
                driver_name: "mock".to_string(),
                database_name: Some("mock_db".to_string()),
                host: Some("localhost".to_string()),
                port: Some(5432),
                username: Some("mock_user".to_string()),
                ssl_mode: None,
                connection_timeout: Some(30),
                query_timeout: Some(30),
                max_connections: Some(1),
                is_pooled: false,
            }
        }'''

# Insert missing methods before the closing brace of impl DatabaseConnection
impl_end = content.find('    }\n\n    impl')
if impl_end != -1:
    content = content[:impl_end] + missing_methods + '\n' + content[impl_end:]

# Write the file back
with open('tests/database_test_utilities.rs', 'w') as f:
    f.write(content)

print("Fixed database test utilities")
