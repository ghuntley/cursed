#!/usr/bin/env python3
"""
CURSED Parser Grammar Validation Script

This script validates the CURSED parser implementation against the grammar specification
by examining the source code and checking for proper keyword handling, AST structure,
and grammar rule implementation.
"""

import os
import re
from pathlib import Path

def read_file(path):
    """Read file content safely"""
    try:
        with open(path, 'r', encoding='utf-8') as f:
            return f.read()
    except Exception as e:
        print(f"Error reading {path}: {e}")
        return ""

def find_token_definitions():
    """Extract token definitions from lexer"""
    lexer_path = "src/lexer.rs"
    content = read_file(lexer_path)
    
    if not content:
        return {}
    
    # Extract enum TokenType variants
    token_enum_match = re.search(r'pub enum TokenType \{(.*?)\}', content, re.DOTALL)
    if not token_enum_match:
        return {}
    
    enum_content = token_enum_match.group(1)
    
    # Find Gen Z slang keywords with their comments
    tokens = {}
    for line in enum_content.split('\n'):
        line = line.strip()
        if '//' in line and ',' in line:
            parts = line.split('//')
            if len(parts) >= 2:
                token_name = parts[0].strip().rstrip(',')
                comment = parts[1].strip()
                tokens[token_name] = comment
    
    return tokens

def check_parser_support():
    """Check parser support for grammar rules"""
    parser_mod_path = "src/parser/mod.rs"
    parser_statements_path = "src/parser/mod_parser_statements.rs"
    parser_expressions_path = "src/parser/mod_parser_expressions.rs"
    
    parser_content = read_file(parser_mod_path)
    statements_content = read_file(parser_statements_path)
    expressions_content = read_file(parser_expressions_path)
    
    # Check for grammar rule implementations
    grammar_support = {}
    
    # Package declarations (vibe)
    if 'parse_package_declaration' in parser_content and 'TokenType::Vibe' in parser_content:
        grammar_support['package_declaration'] = True
    else:
        grammar_support['package_declaration'] = False
    
    # Import statements (yeet)
    if 'parse_import_statement' in parser_content and 'TokenType::Yeet' in parser_content:
        grammar_support['import_statement'] = True
    else:
        grammar_support['import_statement'] = False
    
    # Variable declarations (sus/facts)
    if 'parse_variable_declaration' in statements_content and 'TokenType::Sus' in statements_content:
        grammar_support['variable_declaration'] = True
    else:
        grammar_support['variable_declaration'] = False
    
    # Function declarations (slay)
    if 'parse_function_declaration' in statements_content and 'TokenType::Slay' in statements_content:
        grammar_support['function_declaration'] = True
    else:
        grammar_support['function_declaration'] = False
    
    # If statements (lowkey/highkey)
    if 'parse_if_statement' in statements_content and 'TokenType::Lowkey' in statements_content:
        grammar_support['if_statement'] = True
    else:
        grammar_support['if_statement'] = False
    
    # Switch statements (vibe_check/mood/basic)
    if 'parse_switch_statement' in statements_content and 'TokenType::VibeCheck' in statements_content:
        grammar_support['switch_statement'] = True
    else:
        grammar_support['switch_statement'] = False
    
    # For loops (bestie)
    if 'parse_for_statement' in statements_content and 'TokenType::Bestie' in statements_content:
        grammar_support['for_statement'] = True
    else:
        grammar_support['for_statement'] = False
    
    # While loops (periodt)
    if 'parse_while_statement' in statements_content and 'TokenType::Periodt' in statements_content:
        grammar_support['while_statement'] = True
    else:
        grammar_support['while_statement'] = False
    
    # Return statements (yolo)
    if 'parse_return_statement' in statements_content and 'TokenType::Yolo' in statements_content:
        grammar_support['return_statement'] = True
    else:
        grammar_support['return_statement'] = False
    
    # Break/continue (ghosted/simp)
    if 'parse_break_statement' in statements_content and 'TokenType::Ghosted' in statements_content:
        grammar_support['break_statement'] = True
    else:
        grammar_support['break_statement'] = False
    
    if 'parse_continue_statement' in statements_content and 'TokenType::Simp' in statements_content:
        grammar_support['continue_statement'] = True
    else:
        grammar_support['continue_statement'] = False
    
    return grammar_support

def check_ast_support():
    """Check AST node support for grammar constructs"""
    ast_files = [
        "src/ast/statements.rs",
        "src/ast/conditionals.rs", 
        "src/ast/declarations.rs",
        "src/ast/expressions.rs",
    ]
    
    ast_support = {}
    
    for ast_file in ast_files:
        content = read_file(ast_file)
        
        # Check for statement types
        if 'IfStatement' in content:
            ast_support['if_statement_ast'] = True
        if 'SwitchStatement' in content:
            ast_support['switch_statement_ast'] = True
        if 'ForStatement' in content:
            ast_support['for_statement_ast'] = True
        if 'WhileStatement' in content:
            ast_support['while_statement_ast'] = True
        if 'ReturnStatement' in content:
            ast_support['return_statement_ast'] = True
        if 'BreakStatement' in content:
            ast_support['break_statement_ast'] = True
        if 'ContinueStatement' in content:
            ast_support['continue_statement_ast'] = True
        if 'FunctionStatement' in content:
            ast_support['function_statement_ast'] = True
        if 'VariableStatement' in content:
            ast_support['variable_statement_ast'] = True
    
    return ast_support

def validate_grammar_keywords():
    """Validate that Gen Z slang keywords match grammar spec"""
    
    # Expected keywords from grammar spec
    expected_keywords = {
        'vibe': 'package',
        'yeet': 'import', 
        'facts': 'constants',
        'sus': 'variables',
        'be_like': 'type alias',
        'slay': 'function',
        'lowkey': 'if',
        'highkey': 'else',
        'vibe_check': 'switch',
        'mood': 'case',
        'basic': 'default',
        'bestie': 'for',
        'periodt': 'while',
        'yolo': 'return',
        'ghosted': 'break',
        'simp': 'continue',
        'stan': 'goroutine',
        'flex': 'range',
        'squad': 'struct',
        'collab': 'interface'
    }
    
    tokens = find_token_definitions()
    
    validation_results = {}
    
    for keyword, description in expected_keywords.items():
        # Convert to token name format
        token_names = [
            keyword.title().replace('_', ''),  # vibe_check -> VibeCheck
            keyword.upper().replace('_', ''),  # facts -> FACTS  
            keyword.capitalize(),              # slay -> Slay
        ]
        
        found = False
        for token_name in token_names:
            if token_name in tokens:
                validation_results[keyword] = {
                    'found': True,
                    'token': token_name,
                    'description': tokens[token_name]
                }
                found = True
                break
        
        if not found:
            validation_results[keyword] = {
                'found': False,
                'token': None,
                'description': f"Expected: {description}"
            }
    
    return validation_results

def main():
    print("=== CURSED Parser Grammar Validation ===\n")
    
    # 1. Check token definitions
    print("1. Checking Gen Z Slang Keywords:")
    print("-" * 40)
    
    keyword_validation = validate_grammar_keywords()
    passed_keywords = 0
    total_keywords = len(keyword_validation)
    
    for keyword, result in keyword_validation.items():
        if result['found']:
            print(f"✓ {keyword:12} -> {result['token']:12} ({result['description']})")
            passed_keywords += 1
        else:
            print(f"✗ {keyword:12} -> MISSING       ({result['description']})")
    
    print(f"\nKeywords: {passed_keywords}/{total_keywords} found")
    
    # 2. Check parser support
    print("\n2. Checking Parser Grammar Support:")
    print("-" * 40)
    
    parser_support = check_parser_support()
    passed_parser = 0
    total_parser = len(parser_support)
    
    for rule, supported in parser_support.items():
        status = "✓" if supported else "✗"
        print(f"{status} {rule}")
        if supported:
            passed_parser += 1
    
    print(f"\nParser Rules: {passed_parser}/{total_parser} implemented")
    
    # 3. Check AST support
    print("\n3. Checking AST Node Support:")
    print("-" * 40)
    
    ast_support = check_ast_support()
    passed_ast = 0
    total_ast = len(ast_support)
    
    for node, supported in ast_support.items():
        status = "✓" if supported else "✗"
        print(f"{status} {node}")
        if supported:
            passed_ast += 1
    
    print(f"\nAST Nodes: {passed_ast}/{total_ast} found")
    
    # 4. Overall assessment
    print("\n=== Grammar Validation Summary ===")
    total_checks = passed_keywords + passed_parser + passed_ast
    max_checks = total_keywords + total_parser + total_ast
    percentage = (total_checks / max_checks) * 100 if max_checks > 0 else 0
    
    print(f"Overall Coverage: {total_checks}/{max_checks} ({percentage:.1f}%)")
    
    if percentage >= 80:
        print("✅ GOOD: Parser implementation covers most grammar requirements")
    elif percentage >= 60:
        print("⚠️  FAIR: Parser implementation covers basic grammar requirements")
    else:
        print("❌ POOR: Parser implementation missing significant grammar support")
    
    # 5. Recommendations
    print("\n=== Recommendations ===")
    
    missing_keywords = [k for k, v in keyword_validation.items() if not v['found']]
    if missing_keywords:
        print(f"Missing keywords: {', '.join(missing_keywords)}")
    
    missing_parser = [k for k, v in parser_support.items() if not v]
    if missing_parser:
        print(f"Missing parser rules: {', '.join(missing_parser)}")
    
    missing_ast = [k for k, v in ast_support.items() if not v]
    if missing_ast:
        print(f"Missing AST nodes: {', '.join(missing_ast)}")
    
    if percentage >= 90:
        print("✨ Parser implementation is very comprehensive!")
    elif not missing_keywords and not missing_parser:
        print("🔧 Focus on completing AST node implementations")
    elif not missing_keywords:
        print("🔧 Focus on completing parser rule implementations")
    else:
        print("🔧 Start by ensuring all Gen Z keywords are properly tokenized")

if __name__ == "__main__":
    main()
