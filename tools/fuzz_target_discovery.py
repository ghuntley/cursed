#!/usr/bin/env python3
"""
CURSED Automated Fuzz Target Discovery System

This system analyzes the CURSED codebase to identify functions suitable for fuzzing,
focusing on functions that process external input and could be vulnerable to security issues.
"""

import os
import re
import json
import ast
import subprocess
from pathlib import Path
from typing import List, Dict, Set, Optional, NamedTuple
from dataclasses import dataclass
from enum import Enum

class InputType(Enum):
    """Types of external input that functions might process"""
    FILE_IO = "file_io"
    NETWORK = "network"
    PARSING = "parsing"
    USER_INPUT = "user_input"
    MEMORY_BUFFER = "memory_buffer"
    CONFIGURATION = "configuration"
    SERIALIZATION = "serialization"

class RiskLevel(Enum):
    """Risk levels for potential security vulnerabilities"""
    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"

@dataclass
class FunctionSignature:
    """Represents a function signature with metadata"""
    name: str
    file_path: str
    line_number: int
    parameters: List[str]
    return_type: Optional[str]
    language: str  # 'zig', 'rust', 'c', 'cursed'
    input_types: Set[InputType]
    risk_level: RiskLevel
    complexity_score: int
    is_public: bool
    has_error_handling: bool

class FuzzTargetDiscovery:
    """Main class for discovering fuzz targets in the CURSED codebase"""
    
    def __init__(self, project_root: str):
        self.project_root = Path(project_root)
        self.functions: List[FunctionSignature] = []
        
        # Patterns for identifying risky functions
        self.risky_patterns = {
            InputType.PARSING: [
                r'parse\w*', r'decode\w*', r'unmarshal\w*', r'deserialize\w*',
                r'tokenize\w*', r'lex\w*', r'compile\w*', r'interpret\w*'
            ],
            InputType.FILE_IO: [
                r'read_?file\w*', r'write_?file\w*', r'open\w*', r'load\w*',
                r'save\w*', r'import\w*', r'export\w*'
            ],
            InputType.NETWORK: [
                r'tcp_\w*', r'udp_\w*', r'http_\w*', r'socket\w*', r'connect\w*',
                r'send\w*', r'recv\w*', r'accept\w*'
            ],
            InputType.USER_INPUT: [
                r'input\w*', r'readline\w*', r'getline\w*', r'scanf\w*',
                r'validate\w*', r'sanitize\w*'
            ],
            InputType.MEMORY_BUFFER: [
                r'memcpy\w*', r'strcpy\w*', r'strcat\w*', r'sprintf\w*',
                r'buffer\w*', r'copy\w*'
            ],
            InputType.SERIALIZATION: [
                r'json\w*', r'xml\w*', r'yaml\w*', r'marshal\w*', r'encode\w*'
            ]
        }
        
        # Critical function names that should always be fuzzed
        self.critical_functions = {
            'parseProgram', 'parseStatement', 'parseExpression', 'tokenize',
            'read_file', 'write_file', 'parse_file', 'parse_content',
            'handleRequest', 'handleMessage', 'evaluate', 'validate_string',
            'parse_advanced_function_signature', 'parse_dwarf_info'
        }

    def discover_targets(self) -> List[FunctionSignature]:
        """Main entry point to discover all fuzz targets"""
        print("🔍 Discovering fuzz targets in CURSED codebase...")
        
        # Analyze different language files
        self._analyze_zig_files()
        self._analyze_rust_files()
        self._analyze_c_files()
        self._analyze_cursed_files()
        
        # Sort by risk level and complexity
        self.functions.sort(key=lambda f: (f.risk_level.value, -f.complexity_score))
        
        print(f"✅ Discovered {len(self.functions)} potential fuzz targets")
        return self.functions

    def _analyze_zig_files(self):
        """Analyze Zig source files for fuzz targets"""
        zig_files = list(self.project_root.glob("**/*.zig"))
        for file_path in zig_files:
            if file_path.name.startswith('.'):
                continue
            self._parse_zig_file(file_path)

    def _parse_zig_file(self, file_path: Path):
        """Parse a Zig file to extract function signatures"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"⚠️  Could not read {file_path}: {e}")
            return

        # Zig function pattern: pub fn functionName(...) return_type { ... }
        zig_function_pattern = r'(?:pub\s+)?fn\s+(\w+)\s*\([^)]*\)(?:\s*(\w+|\w+\.\w+|\[\]\w+))?\s*\{'
        
        for match in re.finditer(zig_function_pattern, content, re.MULTILINE):
            func_name = match.group(1)
            return_type = match.group(2)
            line_number = content[:match.start()].count('\n') + 1
            
            # Extract parameters
            param_match = re.search(r'\(([^)]*)\)', match.group(0))
            parameters = []
            if param_match:
                param_str = param_match.group(1).strip()
                if param_str:
                    parameters = [p.strip() for p in param_str.split(',')]
            
            # Analyze function for risk factors
            input_types = self._identify_input_types(func_name, content, match.start(), match.end())
            risk_level = self._calculate_risk_level(func_name, input_types, parameters)
            complexity_score = self._calculate_complexity(content, match.start(), match.end())
            has_error_handling = self._has_error_handling(content, match.start(), match.end())
            is_public = 'pub fn' in match.group(0)
            
            if input_types or func_name in self.critical_functions:
                function_sig = FunctionSignature(
                    name=func_name,
                    file_path=str(file_path.relative_to(self.project_root)),
                    line_number=line_number,
                    parameters=parameters,
                    return_type=return_type,
                    language='zig',
                    input_types=input_types,
                    risk_level=risk_level,
                    complexity_score=complexity_score,
                    is_public=is_public,
                    has_error_handling=has_error_handling
                )
                self.functions.append(function_sig)

    def _analyze_rust_files(self):
        """Analyze Rust source files for fuzz targets"""
        rust_files = list(self.project_root.glob("**/*.rs"))
        for file_path in rust_files:
            if file_path.name.startswith('.'):
                continue
            self._parse_rust_file(file_path)

    def _parse_rust_file(self, file_path: Path):
        """Parse a Rust file to extract function signatures"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"⚠️  Could not read {file_path}: {e}")
            return

        # Rust function pattern: pub fn function_name(...) -> return_type { ... }
        rust_function_pattern = r'(?:pub\s+)?fn\s+(\w+)\s*\([^)]*\)(?:\s*->\s*([^{]+))?\s*\{'
        
        for match in re.finditer(rust_function_pattern, content, re.MULTILINE):
            func_name = match.group(1)
            return_type = match.group(2).strip() if match.group(2) else None
            line_number = content[:match.start()].count('\n') + 1
            
            # Extract parameters
            param_match = re.search(r'\(([^)]*)\)', match.group(0))
            parameters = []
            if param_match:
                param_str = param_match.group(1).strip()
                if param_str:
                    parameters = [p.strip() for p in param_str.split(',')]
            
            # Analyze function for risk factors
            input_types = self._identify_input_types(func_name, content, match.start(), match.end())
            risk_level = self._calculate_risk_level(func_name, input_types, parameters)
            complexity_score = self._calculate_complexity(content, match.start(), match.end())
            has_error_handling = self._has_error_handling(content, match.start(), match.end())
            is_public = 'pub fn' in match.group(0)
            
            if input_types or func_name in self.critical_functions:
                function_sig = FunctionSignature(
                    name=func_name,
                    file_path=str(file_path.relative_to(self.project_root)),
                    line_number=line_number,
                    parameters=parameters,
                    return_type=return_type,
                    language='rust',
                    input_types=input_types,
                    risk_level=risk_level,
                    complexity_score=complexity_score,
                    is_public=is_public,
                    has_error_handling=has_error_handling
                )
                self.functions.append(function_sig)

    def _analyze_c_files(self):
        """Analyze C source files for fuzz targets"""
        c_files = list(self.project_root.glob("**/*.c")) + list(self.project_root.glob("**/*.h"))
        for file_path in c_files:
            if file_path.name.startswith('.'):
                continue
            self._parse_c_file(file_path)

    def _parse_c_file(self, file_path: Path):
        """Parse a C file to extract function signatures"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"⚠️  Could not read {file_path}: {e}")
            return

        # C function pattern: return_type function_name(...) { ... }
        c_function_pattern = r'(\w+\s*\*?\s+)(\w+)\s*\([^)]*\)\s*\{'
        
        for match in re.finditer(c_function_pattern, content, re.MULTILINE):
            return_type = match.group(1).strip()
            func_name = match.group(2)
            line_number = content[:match.start()].count('\n') + 1
            
            # Skip common C keywords that aren't functions
            if return_type in ['if', 'while', 'for', 'switch', 'struct', 'enum']:
                continue
            
            # Extract parameters
            param_match = re.search(r'\(([^)]*)\)', match.group(0))
            parameters = []
            if param_match:
                param_str = param_match.group(1).strip()
                if param_str and param_str != 'void':
                    parameters = [p.strip() for p in param_str.split(',')]
            
            # Analyze function for risk factors
            input_types = self._identify_input_types(func_name, content, match.start(), match.end())
            risk_level = self._calculate_risk_level(func_name, input_types, parameters)
            complexity_score = self._calculate_complexity(content, match.start(), match.end())
            has_error_handling = self._has_error_handling(content, match.start(), match.end())
            
            if input_types or func_name in self.critical_functions:
                function_sig = FunctionSignature(
                    name=func_name,
                    file_path=str(file_path.relative_to(self.project_root)),
                    line_number=line_number,
                    parameters=parameters,
                    return_type=return_type,
                    language='c',
                    input_types=input_types,
                    risk_level=risk_level,
                    complexity_score=complexity_score,
                    is_public=True,  # C functions are generally public
                    has_error_handling=has_error_handling
                )
                self.functions.append(function_sig)

    def _analyze_cursed_files(self):
        """Analyze CURSED source files for fuzz targets"""
        cursed_files = list(self.project_root.glob("**/*.💀"))
        for file_path in cursed_files:
            if file_path.name.startswith('.'):
                continue
            self._parse_cursed_file(file_path)

    def _parse_cursed_file(self, file_path: Path):
        """Parse a CURSED file to extract function signatures"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"⚠️  Could not read {file_path}: {e}")
            return

        # CURSED function pattern: slay functionName(...) return_type { ... }
        cursed_function_pattern = r'slay\s+(\w+)\s*\([^)]*\)(?:\s+(\w+))?\s*\{'
        
        for match in re.finditer(cursed_function_pattern, content, re.MULTILINE):
            func_name = match.group(1)
            return_type = match.group(2)
            line_number = content[:match.start()].count('\n') + 1
            
            # Extract parameters
            param_match = re.search(r'\(([^)]*)\)', match.group(0))
            parameters = []
            if param_match:
                param_str = param_match.group(1).strip()
                if param_str:
                    parameters = [p.strip() for p in param_str.split(',')]
            
            # Analyze function for risk factors
            input_types = self._identify_input_types(func_name, content, match.start(), match.end())
            risk_level = self._calculate_risk_level(func_name, input_types, parameters)
            complexity_score = self._calculate_complexity(content, match.start(), match.end())
            has_error_handling = self._has_error_handling(content, match.start(), match.end())
            
            if input_types or func_name in self.critical_functions:
                function_sig = FunctionSignature(
                    name=func_name,
                    file_path=str(file_path.relative_to(self.project_root)),
                    line_number=line_number,
                    parameters=parameters,
                    return_type=return_type,
                    language='cursed',
                    input_types=input_types,
                    risk_level=risk_level,
                    complexity_score=complexity_score,
                    is_public=True,  # CURSED functions are generally public
                    has_error_handling=has_error_handling
                )
                self.functions.append(function_sig)

    def _identify_input_types(self, func_name: str, content: str, start: int, end: int) -> Set[InputType]:
        """Identify what types of external input a function processes"""
        input_types = set()
        
        # Check function name against patterns
        for input_type, patterns in self.risky_patterns.items():
            for pattern in patterns:
                if re.search(pattern, func_name, re.IGNORECASE):
                    input_types.add(input_type)
        
        # Analyze function body for additional clues
        function_body = content[start:end]
        
        # Look for file operations
        if any(keyword in function_body for keyword in ['open', 'read', 'write', 'file', 'path']):
            input_types.add(InputType.FILE_IO)
        
        # Look for network operations
        if any(keyword in function_body for keyword in ['socket', 'tcp', 'udp', 'http', 'connect']):
            input_types.add(InputType.NETWORK)
        
        # Look for parsing operations
        if any(keyword in function_body for keyword in ['parse', 'token', 'ast', 'lexer']):
            input_types.add(InputType.PARSING)
        
        # Look for buffer operations
        if any(keyword in function_body for keyword in ['buffer', 'copy', 'mem', 'str']):
            input_types.add(InputType.MEMORY_BUFFER)
        
        return input_types

    def _calculate_risk_level(self, func_name: str, input_types: Set[InputType], parameters: List[str]) -> RiskLevel:
        """Calculate risk level based on function characteristics"""
        risk_score = 0
        
        # Critical functions are always high risk
        if func_name in self.critical_functions:
            risk_score += 3
        
        # Input type risk scoring
        risk_weights = {
            InputType.PARSING: 3,
            InputType.MEMORY_BUFFER: 3,
            InputType.NETWORK: 2,
            InputType.FILE_IO: 2,
            InputType.SERIALIZATION: 2,
            InputType.USER_INPUT: 2,
            InputType.CONFIGURATION: 1
        }
        
        for input_type in input_types:
            risk_score += risk_weights.get(input_type, 1)
        
        # Parameter risk factors
        for param in parameters:
            if any(risky in param.lower() for risky in ['char*', 'void*', 'buffer', 'data', 'input']):
                risk_score += 1
        
        # Convert score to risk level
        if risk_score >= 6:
            return RiskLevel.CRITICAL
        elif risk_score >= 4:
            return RiskLevel.HIGH
        elif risk_score >= 2:
            return RiskLevel.MEDIUM
        else:
            return RiskLevel.LOW

    def _calculate_complexity(self, content: str, start: int, end: int) -> int:
        """Calculate complexity score based on function characteristics"""
        function_body = content[start:end]
        complexity = 0
        
        # Count control flow statements
        complexity += len(re.findall(r'\b(if|while|for|switch|case)\b', function_body))
        
        # Count function calls
        complexity += len(re.findall(r'\w+\s*\(', function_body))
        
        # Count loops
        complexity += len(re.findall(r'\b(for|while|loop)\b', function_body))
        
        # Count error handling
        complexity += len(re.findall(r'\b(try|catch|error|result)\b', function_body))
        
        return complexity

    def _has_error_handling(self, content: str, start: int, end: int) -> bool:
        """Check if function has error handling"""
        function_body = content[start:end]
        error_patterns = [
            r'\berror\b', r'\btry\b', r'\bcatch\b', r'\bresult\b',
            r'\.unwrap\(\)', r'\.expect\(', r'return.*Error',
            r'ParserError', r'yikes\b', r'fam\b'
        ]
        
        return any(re.search(pattern, function_body, re.IGNORECASE) for pattern in error_patterns)

    def generate_fuzz_targets(self, output_dir: str = "fuzz_targets"):
        """Generate actual fuzz target files for the discovered functions"""
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)
        
        print(f"🎯 Generating fuzz targets in {output_path}")
        
        # Generate libFuzzer targets for C functions
        self._generate_libfuzzer_targets(output_path)
        
        # Generate cargo-fuzz targets for Rust functions
        self._generate_cargo_fuzz_targets(output_path)
        
        # Generate Zig fuzz targets
        self._generate_zig_fuzz_targets(output_path)
        
        # Generate test harness
        self._generate_fuzz_harness(output_path)
        
        print("✅ Fuzz target generation complete")

    def _generate_libfuzzer_targets(self, output_path: Path):
        """Generate libFuzzer targets for C functions"""
        c_functions = [f for f in self.functions if f.language == 'c']
        
        for func in c_functions:
            if func.risk_level in [RiskLevel.CRITICAL, RiskLevel.HIGH]:
                target_file = output_path / f"fuzz_{func.name}.c"
                self._write_libfuzzer_target(target_file, func)

    def _write_libfuzzer_target(self, target_file: Path, func: FunctionSignature):
        """Write a libFuzzer target file"""
        content = f"""// Fuzz target for {func.name} in {func.file_path}:{func.line_number}
// Risk Level: {func.risk_level.value.upper()}
// Input Types: {', '.join(t.value for t in func.input_types)}

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

// Include the header containing {func.name}
// #include "{func.file_path.replace('.c', '.h')}"

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {{
    if (size == 0) return 0;
    
    // Null-terminate data for string functions
    char *input = malloc(size + 1);
    if (!input) return 0;
    memcpy(input, data, size);
    input[size] = '\\0';
    
    // TODO: Call {func.name} with fuzzed input
    // Example: {func.name}(input);
    
    free(input);
    return 0;
}}
"""
        with open(target_file, 'w') as f:
            f.write(content)

    def _generate_cargo_fuzz_targets(self, output_path: Path):
        """Generate cargo-fuzz targets for Rust functions"""
        rust_functions = [f for f in self.functions if f.language == 'rust']
        
        fuzz_dir = output_path / "cargo_fuzz"
        fuzz_dir.mkdir(exist_ok=True)
        
        # Generate Cargo.toml for fuzz targets
        cargo_toml = fuzz_dir / "Cargo.toml"
        with open(cargo_toml, 'w') as f:
            f.write("""[package]
name = "cursed-fuzz"
version = "0.1.0"
edition = "2021"

[dependencies]
libfuzzer-sys = "0.4"

[[bin]]
name = "fuzz_targets"
path = "src/main.rs"
""")
        
        src_dir = fuzz_dir / "src"
        src_dir.mkdir(exist_ok=True)
        
        for func in rust_functions:
            if func.risk_level in [RiskLevel.CRITICAL, RiskLevel.HIGH]:
                target_file = src_dir / f"fuzz_{func.name}.rs"
                self._write_cargo_fuzz_target(target_file, func)

    def _write_cargo_fuzz_target(self, target_file: Path, func: FunctionSignature):
        """Write a cargo-fuzz target file"""
        content = f"""// Fuzz target for {func.name} in {func.file_path}:{func.line_number}
// Risk Level: {func.risk_level.value.upper()}
// Input Types: {', '.join(t.value for t in func.input_types)}

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {{
    if data.is_empty() {{
        return;
    }}
    
    // Convert bytes to string for parsing functions
    if let Ok(input_str) = std::str::from_utf8(data) {{
        // TODO: Call {func.name} with fuzzed input
        // Example: {func.name}(input_str);
    }}
}});
"""
        with open(target_file, 'w') as f:
            f.write(content)

    def _generate_zig_fuzz_targets(self, output_path: Path):
        """Generate Zig fuzz targets"""
        zig_functions = [f for f in self.functions if f.language == 'zig']
        
        for func in zig_functions:
            if func.risk_level in [RiskLevel.CRITICAL, RiskLevel.HIGH]:
                target_file = output_path / f"fuzz_{func.name}.zig"
                self._write_zig_fuzz_target(target_file, func)

    def _write_zig_fuzz_target(self, target_file: Path, func: FunctionSignature):
        """Write a Zig fuzz target file"""
        gpa_init = "var gpa = std.heap.GeneralPurposeAllocator(.{}){};"
        content = f"""// Fuzz target for {func.name} in {func.file_path}:{func.line_number}
// Risk Level: {func.risk_level.value.upper()}
// Input Types: {', '.join(t.value for t in func.input_types)}

const std = @import("std");
const testing = std.testing;

// Import the module containing {func.name}
// const target_module = @import("../{func.file_path}");

export fn LLVMFuzzerTestOneInput(data: [*]const u8, size: usize) c_int {{
    if (size == 0) return 0;
    
    {gpa_init}
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const input = data[0..size];
    
    // TODO: Call {func.name} with fuzzed input
    // Example: _ = target_module.{func.name}(allocator, input) catch return 0;
    
    return 0;
}}

test "fuzz target basic test" {{
    const test_data = "test input";
    _ = LLVMFuzzerTestOneInput(test_data.ptr, test_data.len);
}}
"""
        with open(target_file, 'w') as f:
            f.write(content)

    def _generate_fuzz_harness(self, output_path: Path):
        """Generate a comprehensive fuzz test harness"""
        harness_file = output_path / "run_all_fuzz_tests.sh"
        
        content = """#!/bin/bash
# CURSED Automated Fuzz Testing Harness

set -e

echo "🚀 Starting CURSED fuzz testing..."

# Compile and run libFuzzer targets
echo "📦 Building libFuzzer targets..."
for fuzz_target in fuzz_*.c; do
    if [ -f "$fuzz_target" ]; then
        echo "Building $fuzz_target..."
        clang -fsanitize=fuzzer,address -g "$fuzz_target" -o "${fuzz_target%.c}"
        echo "Running $fuzz_target for 60 seconds..."
        timeout 60 "./${fuzz_target%.c}" || true
    fi
done

# Run cargo-fuzz targets
if [ -d "cargo_fuzz" ]; then
    echo "🦀 Running Rust fuzz targets..."
    cd cargo_fuzz
    cargo install cargo-fuzz
    for fuzz_target in src/fuzz_*.rs; do
        if [ -f "$fuzz_target" ]; then
            target_name=$(basename "$fuzz_target" .rs)
            echo "Running $target_name for 60 seconds..."
            timeout 60 cargo fuzz run "$target_name" || true
        fi
    done
    cd ..
fi

# Run Zig fuzz targets
echo "⚡ Building Zig fuzz targets..."
for fuzz_target in fuzz_*.zig; do
    if [ -f "$fuzz_target" ]; then
        echo "Building $fuzz_target..."
        zig build-exe -fsanitize-c -lc "$fuzz_target"
        echo "Running basic test for $fuzz_target..."
        "./${fuzz_target%.zig}" || true
    fi
done

echo "✅ Fuzz testing complete!"
"""
        
        with open(harness_file, 'w') as f:
            f.write(content)
        
        # Make executable
        harness_file.chmod(0o755)

    def export_report(self, output_file: str = "fuzz_target_report.json"):
        """Export a detailed report of discovered fuzz targets"""
        report_data = {
            "summary": {
                "total_functions": len(self.functions),
                "critical_risk": len([f for f in self.functions if f.risk_level == RiskLevel.CRITICAL]),
                "high_risk": len([f for f in self.functions if f.risk_level == RiskLevel.HIGH]),
                "medium_risk": len([f for f in self.functions if f.risk_level == RiskLevel.MEDIUM]),
                "low_risk": len([f for f in self.functions if f.risk_level == RiskLevel.LOW]),
                "languages": {}
            },
            "functions": []
        }
        
        # Count by language
        for func in self.functions:
            if func.language not in report_data["summary"]["languages"]:
                report_data["summary"]["languages"][func.language] = 0
            report_data["summary"]["languages"][func.language] += 1
        
        # Export function details
        for func in self.functions:
            func_data = {
                "name": func.name,
                "file_path": func.file_path,
                "line_number": func.line_number,
                "language": func.language,
                "risk_level": func.risk_level.value,
                "complexity_score": func.complexity_score,
                "input_types": [t.value for t in func.input_types],
                "parameters": func.parameters,
                "return_type": func.return_type,
                "is_public": func.is_public,
                "has_error_handling": func.has_error_handling
            }
            report_data["functions"].append(func_data)
        
        with open(output_file, 'w') as f:
            json.dump(report_data, f, indent=2)
        
        print(f"📊 Report exported to {output_file}")

def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Fuzz Target Discovery System")
    parser.add_argument("--project-root", default=".", help="Path to CURSED project root")
    parser.add_argument("--output-dir", default="fuzz_targets", help="Output directory for fuzz targets")
    parser.add_argument("--report", default="fuzz_target_report.json", help="Output file for JSON report")
    parser.add_argument("--generate", action="store_true", help="Generate fuzz target files")
    parser.add_argument("--min-risk", choices=["low", "medium", "high", "critical"], default="medium", help="Minimum risk level to include")
    
    args = parser.parse_args()
    
    # Initialize discovery system
    discovery = FuzzTargetDiscovery(args.project_root)
    
    # Discover targets
    functions = discovery.discover_targets()
    
    # Filter by minimum risk level
    risk_order = {"low": 0, "medium": 1, "high": 2, "critical": 3}
    min_risk_level = risk_order[args.min_risk]
    
    filtered_functions = [
        f for f in functions 
        if risk_order[f.risk_level.value] >= min_risk_level
    ]
    
    print(f"\n📈 Fuzz Target Discovery Results:")
    print(f"   Total functions analyzed: {len(functions)}")
    print(f"   Functions meeting risk threshold: {len(filtered_functions)}")
    print(f"   Critical risk: {len([f for f in filtered_functions if f.risk_level == RiskLevel.CRITICAL])}")
    print(f"   High risk: {len([f for f in filtered_functions if f.risk_level == RiskLevel.HIGH])}")
    print(f"   Medium risk: {len([f for f in filtered_functions if f.risk_level == RiskLevel.MEDIUM])}")
    
    # Show top targets
    print(f"\n🎯 Top 10 Fuzz Targets:")
    for i, func in enumerate(filtered_functions[:10]):
        input_types_str = ", ".join(t.value for t in func.input_types)
        print(f"   {i+1}. {func.name} ({func.language}) - {func.risk_level.value.upper()} risk")
        print(f"      File: {func.file_path}:{func.line_number}")
        print(f"      Input types: {input_types_str}")
        print()
    
    # Generate fuzz targets if requested
    if args.generate:
        discovery.functions = filtered_functions
        discovery.generate_fuzz_targets(args.output_dir)
    
    # Export report
    discovery.functions = filtered_functions
    discovery.export_report(args.report)

if __name__ == "__main__":
    main()
