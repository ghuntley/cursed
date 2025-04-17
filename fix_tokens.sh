#!/bin/bash

# Find all lines with 'token: Token::new(' and fix them by adding '.token_literal()'
sed -i 's/token: Token::new(\(.*\)),/token: Token::new(\1).token_literal(),/g' tests/struct_field_type_inference_test.rs