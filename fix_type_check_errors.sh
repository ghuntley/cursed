#!/bin/bash

# Fix TypeCheckError instances by adding missing fields
sed -i '
/TypeCheckError {/,/}/ {
    /error_type: TypeErrorKind::/a\            suggestions: vec!["Check variable names and types".to_string()],\
            severity: ErrorSeverity::Error,\
            recoverable: false,
}
' src/type_system/checker.rs

echo "Fixed TypeCheckError instances"
