#!/usr/bin/env python3

import os
import re
import sys

# Files and struct names that need Debug derive
expressions_to_fix = [
    ("src/ast/control_flow/range.rs", "RangeClause"),
    ("src/ast/control_flow/channel_range.rs", "ChannelRangeClause"),
    ("src/ast/control_flow/channel_range.rs", "ChannelClosureDetection"),
    ("src/ast/expressions/calls.rs", "CallExpression"),
    ("src/ast/expressions/calls.rs", "GenericCallExpression"),
    ("src/ast/expressions/channel.rs", "ChannelExpression"),
    ("src/ast/expressions/channel.rs", "SendExpression"),
    ("src/ast/expressions/channel.rs", "ReceiveExpression"),
    ("src/ast/expressions/collections.rs", "ArrayLiteral"),
    ("src/ast/expressions/collections.rs", "HashLiteral"),
    ("src/ast/expressions/collections.rs", "MapLiteral"),
    ("src/ast/expressions/collections.rs", "IndexExpression"),
    ("src/ast/expressions/concurrency.rs", "StanExpression"),
    ("src/ast/expressions/constraint.rs", "TypeConstraint"),
    ("src/ast/expressions/dot_expression.rs", "DotExpression"),
    ("src/ast/expressions/generics.rs", "TypeReference"),
    ("src/ast/expressions/generics.rs", "GenericCallExpression"),
    ("src/ast/expressions/generics.rs", "BeLikeExpression"),
    ("src/ast/expressions/if_expression.rs", "IfExpression"),
    ("src/ast/expressions/literals.rs", "IntegerLiteral"),
    ("src/ast/expressions/literals.rs", "FloatLiteral"),
    ("src/ast/expressions/literals.rs", "BooleanLiteral"),
    ("src/ast/expressions/literals.rs", "ByteLiteral"),
    ("src/ast/expressions/literals.rs", "RuneLiteral"),
    ("src/ast/expressions/operators.rs", "PrefixExpression"),
    ("src/ast/expressions/operators.rs", "InfixExpression"),
    ("src/ast/expressions/range_expression.rs", "RangeExpression"),
    ("src/ast/expressions/special.rs", "AssignmentExpression"),
    ("src/ast/expressions/special.rs", "BeLikeExpression"),
    ("src/ast/expressions/special.rs", "DefaultCase"),
    ("src/ast/expressions/struct_expr.rs", "StructLiteral"),
    ("src/ast/expressions/struct_expr.rs", "StructFieldAccess"),
    ("src/ast/expressions/types.rs", "TypeConversionExpression"),
    ("src/ast/pointer/operations.rs", "PointerDereference"),
    ("src/ast/pointer/types.rs", "PointerType"),
    ("parser/range_expression_error_recovery.rs", "RecoverableRangeExpression"),
    ("src/ast/statements/fields.rs", "FieldStatement"),
    ("src/ast/declarations/struct_interface.rs", "MethodSignature"),
]

def add_debug_derive(filepath, struct_name):
    """Add #[derive(Debug)] to a struct if it doesn't already have it."""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Check if the struct already has Debug derive
        struct_pattern = re.compile(rf'^\s*(?:#\[derive\([^)]*Debug[^)]*\)\]\s*)?(?:pub\s+)?(?:struct|enum)\s+{re.escape(struct_name)}\s*[{{]', re.MULTILINE)
        
        # Find the struct definition
        match = struct_pattern.search(content)
        if not match:
            print(f"Could not find struct {struct_name} in {filepath}")
            return False
            
        # Check if it already has Debug
        if "Debug" in match.group(0):
            print(f"Struct {struct_name} already has Debug derive in {filepath}")
            return True
            
        # Find the line where the struct is defined
        lines = content.split('\n')
        struct_line_idx = None
        
        for i, line in enumerate(lines):
            if re.search(rf'(?:pub\s+)?(?:struct|enum)\s+{re.escape(struct_name)}\s*[{{]', line):
                struct_line_idx = i
                break
                
        if struct_line_idx is None:
            print(f"Could not find struct definition line for {struct_name} in {filepath}")
            return False
            
        # Check if there's already a derive on the line above
        derive_line_idx = struct_line_idx - 1
        if derive_line_idx >= 0 and "#[derive(" in lines[derive_line_idx]:
            # Add Debug to existing derive
            lines[derive_line_idx] = lines[derive_line_idx].replace("#[derive(", "#[derive(Debug, ")
        else:
            # Add new derive line
            indent = len(lines[struct_line_idx]) - len(lines[struct_line_idx].lstrip())
            derive_line = " " * indent + "#[derive(Debug)]"
            lines.insert(struct_line_idx, derive_line)
            
        # Write back to file
        with open(filepath, 'w') as f:
            f.write('\n'.join(lines))
            
        print(f"Added Debug derive to {struct_name} in {filepath}")
        return True
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    success_count = 0
    for filepath, struct_name in expressions_to_fix:
        if add_debug_derive(filepath, struct_name):
            success_count += 1
            
    print(f"\nProcessed {success_count}/{len(expressions_to_fix)} files successfully")

if __name__ == "__main__":
    main()
