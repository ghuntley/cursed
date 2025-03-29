#!/usr/bin/env python3
"""
A simple wrapper script that processes CURSED code files
without requiring the full VM implementation.
"""

import sys
import re


def process_cursed_file(filename):
    """Process a CURSED file by extracting variables and print statements."""
    with open(filename, 'r') as f:
        content = f.read()

    # Extract variable declarations
    variables = {}
    var_pattern = r'sus\s+(\w+)\s*=\s*([^;]+);'
    for match in re.finditer(var_pattern, content):
        var_name = match.group(1)
        var_value = match.group(2).strip()

        # Evaluate numeric values
        if var_value.isdigit():
            variables[var_name] = int(var_value)
        elif '+' in var_value:
            # Handle basic addition
            parts = var_value.split('+')
            if len(parts) == 2:
                left = parts[0].strip()
                right = parts[1].strip()
                if left in variables and right in variables:
                    variables[var_name] = variables[left] + variables[right]
                elif left in variables and right.isdigit():
                    variables[var_name] = variables[left] + int(right)
                elif left.isdigit() and right in variables:
                    variables[var_name] = int(left) + variables[right]
                elif left.isdigit() and right.isdigit():
                    variables[var_name] = int(left) + int(right)
        elif var_value.startswith('"') and var_value.endswith('"'):
            # Handle string values
            variables[var_name] = var_value[1:-1]  # Remove quotes
        else:
            variables[var_name] = var_value

    # Find and execute vibez.spill statements 
    spill_pattern = r'vibez\.spill\(([^)]+)\);'
    for match in re.finditer(spill_pattern, content):
        arg = match.group(1).strip()

        # Check if it's a variable
        if arg in variables:
            print(variables[arg])
        # Check if it's an addition expression
        elif '+' in arg:
            parts = arg.split('+')
            if len(parts) == 2:
                left = parts[0].strip()
                right = parts[1].strip()
                if left in variables and right in variables:
                    print(variables[left] + variables[right])
                elif left in variables and right.isdigit():
                    print(variables[left] + int(right))
                elif left.isdigit() and right in variables:
                    print(int(left) + variables[right])
                elif left.isdigit() and right.isdigit():
                    print(int(left) + int(right))
        # Check if it's a string literal
        elif arg.startswith('"') and arg.endswith('"'):
            print(arg[1:-1])  # Remove quotes
        else:
            print(f"Unknown argument to vibez.spill: {arg}")


def main():
    if len(sys.argv) < 2:
        print("Usage: python cursed_wrapper.py <cursed_file>")
        sys.exit(1)

    filename = sys.argv[1]
    process_cursed_file(filename)


if __name__ == "__main__":
    main() 