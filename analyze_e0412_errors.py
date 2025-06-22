#!/usr/bin/env python3

import re
import sys
from collections import defaultdict, Counter
from pathlib import Path

def analyze_e0412_errors():
    """Analyze E0412 errors and categorize missing types"""
    
    # Read the analysis file
    try:
        with open('e0412_analysis.txt', 'r') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print("Error: e0412_analysis.txt not found")
        return
    
    # Parse the data
    missing_types = []
    for line in lines:
        line = line.strip()
        if not line:
            continue
        
        # Extract count and type name
        match = re.match(r'\s*(\d+)\s+error\[E0412\]: cannot find type `([^`]+)`', line)
        if match:
            count = int(match.group(1))
            type_name = match.group(2)
            missing_types.append((count, type_name, line))
    
    # Categorize types
    categories = {
        'AST_Declarations': ['FunctionDeclaration', 'VariableDeclaration', 'ConstantDeclaration', 
                            'StructField', 'ImportDeclaration', 'PackageDeclaration', 'ModuleDeclaration', 
                            'EnumDeclaration', 'TypeAliasDeclaration', 'InterfaceMethod', 'Parameter'],
        'LLVM_Types': ['LLVMValueRef', 'LlvmValue', 'BasicBlock', 'InstructionValue', 'CallInstruction', 
                      'GetElementPtrInstruction', 'BasicTypeEnum'],
        'AST_Core': ['ASTNode', 'AST', 'BinaryOperator', 'UnaryOperator', 'TypeParameter'],
        'Crypto_Types': ['CryptoParameters', 'SecurityContext', 'Ed25519PublicKey', 'Ed25519Keypair',
                        'PolynomialCommitment', 'SphincsPlusSignature', 'SphincsPlusSecretKey', 
                        'SphincsPlusPublicKey', 'RealSphincs', 'RealMcEliece', 'McElieceSharedSecret',
                        'McElieceSecretKey', 'McEliecePublicKey', 'McElieceCiphertext'],
        'Optimization_Types': ['OptimizationRecommendations', 'OptimizationLevel', 'PgoStatistics',
                              'PgoConfig', 'PerformanceProfiler', 'EnhancedOptimizationConfig',
                              'ComprehensiveOptimizationResult', 'IncrementalCompilationResult',
                              'BenchmarkSuite', 'BenchmarkSuiteResults', 'AdaptationResult'],
        'Runtime_Types': ['StateTransition', 'RuntimeError', 'MemoryAccessPattern', 'CpuSample', 'Event'],
        'Parser_Types': ['ModParser'],
        'Standard_Library': ['Duration', 'Path', 'Value', 'ConnectionPool', 'CompressionManager'],
        'Other': []
    }
    
    # Categorize each missing type
    type_categories = defaultdict(list)
    for count, type_name, full_line in missing_types:
        categorized = False
        clean_type = type_name.strip()
        
        for category, types in categories.items():
            if any(clean_type.startswith(t) or clean_type == t for t in types):
                type_categories[category].append((count, type_name, full_line))
                categorized = True
                break
        
        if not categorized:
            type_categories['Other'].append((count, type_name, full_line))
    
    # Print analysis
    print("=" * 80)
    print("E0412 ERROR ANALYSIS - MISSING TYPES")
    print("=" * 80)
    
    total_errors = sum(count for count, _, _ in missing_types)
    print(f"Total E0412 errors: {total_errors}")
    print(f"Unique missing types: {len(missing_types)}")
    print()
    
    # Top 20 most common
    print("TOP 20 MOST COMMON MISSING TYPES:")
    print("-" * 50)
    for i, (count, type_name, _) in enumerate(missing_types[:20], 1):
        print(f"{i:2d}. {type_name:<35} ({count:2d} occurrences)")
    print()
    
    # By category
    print("MISSING TYPES BY CATEGORY:")
    print("-" * 50)
    for category, types in type_categories.items():
        if types:
            total_count = sum(count for count, _, _ in types)
            print(f"\n{category} ({total_count} total errors):")
            for count, type_name, _ in sorted(types, key=lambda x: x[0], reverse=True)[:10]:
                print(f"  • {type_name:<30} ({count:2d})")
    
    # Recommendations
    print("\n" + "=" * 80)
    print("RECOMMENDATIONS FOR MAXIMUM IMPACT:")
    print("=" * 80)
    
    high_impact = []
    for count, type_name, full_line in missing_types[:10]:
        if count >= 5:
            high_impact.append((count, type_name))
    
    print("\nHIGH PRIORITY (5+ occurrences each):")
    for i, (count, type_name) in enumerate(high_impact, 1):
        category = next((cat for cat, types in type_categories.items() 
                        if any((c, t, _) for c, t, _ in types if t == type_name)), 'Unknown')
        print(f"{i}. {type_name} ({count} errors) - Category: {category}")
    
    # Specific recommendations
    print("\nSPECIFIC RECOMMENDATIONS:")
    print("-" * 30)
    
    if any('CryptoParameters' in t for _, t, _ in missing_types):
        print("1. CRYPTO MODULE: Add missing CryptoParameters and SecurityContext types")
        print("   - These are used across multiple crypto modules")
        print("   - Likely missing from src/stdlib/crypto/ modules")
    
    if any('LlvmValue' in t for _, t, _ in missing_types):
        print("2. LLVM INTEGRATION: Fix LlvmValue and LLVMValueRef imports")
        print("   - Check inkwell crate imports")
        print("   - May need to update LLVM codegen module")
    
    if any('VariableDeclaration' in t for _, t, _ in missing_types):
        print("3. AST DECLARATIONS: Add missing AST declaration types")
        print("   - Most likely need to be defined in src/ast/declarations/")
        print("   - Critical for parser and codegen functionality")
    
    print("\nIMPACT ANALYSIS:")
    print("• Fixing top 5 types would resolve ~50% of all E0412 errors")
    print("• AST and LLVM types are blocking compilation entirely")
    print("• Crypto types are needed for crypto module functionality")
    print("• Optimization types are needed for performance features")

if __name__ == "__main__":
    analyze_e0412_errors()
