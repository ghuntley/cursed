#!/usr/bin/env python3
"""
CURSED Fuzz Template Generator

Generates specific fuzz targets based on function analysis and creates
optimized fuzzing strategies for different types of input processing.
"""

import os
import json
from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass

@dataclass
class FuzzTemplate:
    """Template for generating fuzz targets"""
    name: str
    description: str
    languages: List[str]
    template_content: str
    build_script: str
    test_cases: List[str]

class FuzzTemplateGenerator:
    """Generates optimized fuzz templates for different function types"""
    
    def __init__(self):
        self.templates = self._create_templates()
    
    def _create_templates(self) -> Dict[str, FuzzTemplate]:
        """Create predefined templates for different fuzzing scenarios"""
        templates = {}
        
        # Parser fuzzing template
        templates["parser"] = FuzzTemplate(
            name="Parser Fuzzing",
            description="Fuzz targets for parsing functions (lexer, parser, AST)",
            languages=["zig", "rust", "c"],
            template_content=self._get_parser_template(),
            build_script=self._get_parser_build_script(),
            test_cases=self._get_parser_test_cases()
        )
        
        # File I/O fuzzing template
        templates["file_io"] = FuzzTemplate(
            name="File I/O Fuzzing", 
            description="Fuzz targets for file operations and path handling",
            languages=["zig", "rust", "c"],
            template_content=self._get_file_io_template(),
            build_script=self._get_file_io_build_script(),
            test_cases=self._get_file_io_test_cases()
        )
        
        # Network fuzzing template
        templates["network"] = FuzzTemplate(
            name="Network Fuzzing",
            description="Fuzz targets for network operations and protocol parsing",
            languages=["zig", "rust", "c"],
            template_content=self._get_network_template(),
            build_script=self._get_network_build_script(),
            test_cases=self._get_network_test_cases()
        )
        
        # Memory buffer fuzzing template
        templates["memory_buffer"] = FuzzTemplate(
            name="Memory Buffer Fuzzing",
            description="Fuzz targets for buffer operations and memory handling",
            languages=["zig", "rust", "c"],
            template_content=self._get_memory_buffer_template(),
            build_script=self._get_memory_buffer_build_script(),
            test_cases=self._get_memory_buffer_test_cases()
        )
        
        # CURSED language fuzzing template
        templates["cursed_lang"] = FuzzTemplate(
            name="CURSED Language Fuzzing",
            description="Fuzz targets specific to CURSED language features",
            languages=["cursed", "zig"],
            template_content=self._get_cursed_lang_template(),
            build_script=self._get_cursed_lang_build_script(),
            test_cases=self._get_cursed_lang_test_cases()
        )
        
        return templates
    
    def _get_parser_template(self) -> str:
        return """// CURSED Parser Fuzz Target Template
// Targets: {function_name} in {file_path}:{line_number}
// Risk Level: {risk_level}

{includes}

{language_specific_setup}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {{
    if (size == 0 || size > MAX_INPUT_SIZE) return 0;
    
    // Initialize allocator and error handling
    {allocator_setup}
    
    // Create null-terminated input for parsing
    char *input = malloc(size + 1);
    if (!input) return 0;
    memcpy(input, data, size);
    input[size] = '\\0';
    
    // Test various parser entry points
    {parser_test_calls}
    
    // Cleanup
    free(input);
    {cleanup_code}
    
    return 0;
}}

// Dictionary for parser fuzzing
const char *parser_dict[] = {{
    // CURSED keywords
    "sus", "drip", "slay", "damn", "vibez", "spill", "yeet", "based", "cringe",
    "bestie", "ready", "otherwise", "sick", "when", "squad", "spill", "collab",
    
    // Common tokens
    "=", "==", "!=", "&&", "||", "+", "-", "*", "/", "%",
    "(", ")", "{{", "}}", "[", "]", ";", ",", ".",
    
    // Common patterns
    "function", "if", "else", "while", "for", "return",
    "struct", "interface", "import", "export"
}};

{additional_test_functions}
"""

    def _get_parser_build_script(self) -> str:
        return """#!/bin/bash
# Build script for parser fuzz targets

set -e

echo "Building parser fuzz targets..."

# Compile with sanitizers
CFLAGS="-fsanitize=fuzzer,address,undefined -g -O1 -fno-omit-frame-pointer"
LDFLAGS="-fsanitize=fuzzer,address,undefined"

# Build each parser target
for target in fuzz_parser_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

# Build Zig targets
for target in fuzz_parser_*.zig; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        zig build-exe -fsanitize-c -lc "$target"
    fi
done

echo "Parser fuzz targets built successfully!"
"""

    def _get_parser_test_cases(self) -> List[str]:
        return [
            "sus x drip = 42;",
            "slay test() drip { damn 0; }",
            'ready (based) { vibez.spill("hello"); }',
            "bestie (x < 10) { x = x + 1; }",
            'yeet "modulename";',
            "",  # Empty input
            "a" * 10000,  # Large input
            "sus x drip = " + "(" * 1000 + "42" + ")" * 1000 + ";",  # Deep nesting
            "\\x00\\x01\\x02\\x03",  # Binary data
            "sus \\xff\\xfe\\xfd = 42;",  # Invalid UTF-8
        ]

    def _get_file_io_template(self) -> str:
        return """// CURSED File I/O Fuzz Target Template
// Targets: {function_name} in {file_path}:{line_number}

{includes}

#define MAX_PATH_SIZE 4096
#define MAX_FILE_SIZE (1024 * 1024)  // 1MB limit

{language_specific_setup}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {{
    if (size == 0 || size > MAX_FILE_SIZE) return 0;
    
    {allocator_setup}
    
    // Create temporary file with fuzzed content
    char temp_path[] = "/tmp/fuzz_file_XXXXXX";
    int fd = mkstemp(temp_path);
    if (fd == -1) return 0;
    
    // Write fuzzed data to temp file
    if (write(fd, data, size) != (ssize_t)size) {{
        close(fd);
        unlink(temp_path);
        return 0;
    }}
    close(fd);
    
    // Test file operations
    {file_io_test_calls}
    
    // Cleanup
    unlink(temp_path);
    {cleanup_code}
    
    return 0;
}}

// Test with various path manipulations
void test_path_operations(const uint8_t *data, size_t size) {{
    if (size == 0 || size > MAX_PATH_SIZE) return;
    
    char path[MAX_PATH_SIZE];
    size_t copy_size = size < MAX_PATH_SIZE - 1 ? size : MAX_PATH_SIZE - 1;
    memcpy(path, data, copy_size);
    path[copy_size] = '\\0';
    
    // Test path validation and manipulation
    // {function_name}(path);
}}

{additional_test_functions}
"""

    def _get_file_io_build_script(self) -> str:
        return """#!/bin/bash
# Build script for file I/O fuzz targets

set -e

echo "Building file I/O fuzz targets..."

CFLAGS="-fsanitize=fuzzer,address -g -O1"
LDFLAGS="-fsanitize=fuzzer,address"

for target in fuzz_file_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

echo "File I/O fuzz targets built successfully!"
"""

    def _get_file_io_test_cases(self) -> List[str]:
        return [
            "/etc/passwd",
            "../../../etc/passwd",
            "file.txt",
            "",
            "a" * 1000,
            "/dev/null",
            "/tmp/test.txt",
            "\\x00\\x01\\x02",
            "file\\nwith\\nnewlines",
            "file\\'with\\'quotes",
        ]

    def _get_network_template(self) -> str:
        return """// CURSED Network Fuzz Target Template
// Targets: {function_name} in {file_path}:{line_number}

{includes}
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

#define MAX_PACKET_SIZE 65535

{language_specific_setup}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {{
    if (size == 0 || size > MAX_PACKET_SIZE) return 0;
    
    {allocator_setup}
    
    // Test packet parsing
    {network_test_calls}
    
    // Test with socket operations (mock)
    test_socket_operations(data, size);
    
    {cleanup_code}
    return 0;
}}

void test_socket_operations(const uint8_t *data, size_t size) {{
    // Create mock socket operations
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return;
    
    // Test with fuzzed network data
    // {function_name}(sockfd, data, size);
    
    close(sockfd);
}}

{additional_test_functions}
"""

    def _get_network_build_script(self) -> str:
        return """#!/bin/bash
# Build script for network fuzz targets

set -e

echo "Building network fuzz targets..."

CFLAGS="-fsanitize=fuzzer,address -g -O1"
LDFLAGS="-fsanitize=fuzzer,address"

for target in fuzz_network_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

echo "Network fuzz targets built successfully!"
"""

    def _get_network_test_cases(self) -> List[str]:
        return [
            "GET / HTTP/1.1\\r\\nHost: localhost\\r\\n\\r\\n",
            "POST /data HTTP/1.1\\r\\nContent-Length: 4\\r\\n\\r\\ntest",
            "\\x00\\x01\\x02\\x03",  # Binary protocol data
            "a" * 8192,  # Large packet
            "",  # Empty packet
            "HTTP/1.1 200 OK\\r\\n\\r\\n",
            "\\xff\\xfe\\xfd\\xfc",  # Invalid headers
        ]

    def _get_memory_buffer_template(self) -> str:
        return """// CURSED Memory Buffer Fuzz Target Template
// Targets: {function_name} in {file_path}:{line_number}

{includes}

#define MAX_BUFFER_SIZE (1024 * 1024)

{language_specific_setup}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {{
    if (size == 0 || size > MAX_BUFFER_SIZE) return 0;
    
    {allocator_setup}
    
    // Test buffer operations with various sizes
    test_buffer_operations(data, size);
    test_string_operations(data, size);
    test_boundary_conditions(data, size);
    
    {cleanup_code}
    return 0;
}}

void test_buffer_operations(const uint8_t *data, size_t size) {{
    // Test buffer copying and manipulation
    uint8_t *buffer = malloc(size + 1);
    if (!buffer) return;
    
    // {function_name}(buffer, data, size);
    
    free(buffer);
}}

void test_string_operations(const uint8_t *data, size_t size) {{
    char *str = malloc(size + 1);
    if (!str) return;
    
    memcpy(str, data, size);
    str[size] = '\\0';
    
    // Test string functions
    // {function_name}(str);
    
    free(str);
}}

void test_boundary_conditions(const uint8_t *data, size_t size) {{
    // Test edge cases
    if (size > 0) {{
        // {function_name}(data, 0);        // Zero size
        // {function_name}(data, 1);        // Single byte
        // {function_name}(data, size);     // Full size
        // {function_name}(NULL, 0);        // NULL pointer
    }}
}}

{additional_test_functions}
"""

    def _get_memory_buffer_build_script(self) -> str:
        return """#!/bin/bash
# Build script for memory buffer fuzz targets

set -e

echo "Building memory buffer fuzz targets..."

CFLAGS="-fsanitize=fuzzer,address,undefined -g -O1"
LDFLAGS="-fsanitize=fuzzer,address,undefined"

for target in fuzz_buffer_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS
    fi
done

echo "Memory buffer fuzz targets built successfully!"
"""

    def _get_memory_buffer_test_cases(self) -> List[str]:
        return [
            "hello world",
            "",
            "\\x00\\x01\\x02",
            "a" * 1000,
            "\\xff" * 100,
            "hello\\x00world",
            "\\x80\\x81\\x82",  # High-bit characters
        ]

    def _get_cursed_lang_template(self) -> str:
        return """// CURSED Language-Specific Fuzz Target Template
// Targets: {function_name} in {file_path}:{line_number}

{includes}

{language_specific_setup}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {{
    if (size == 0 || size > 100000) return 0;
    
    {allocator_setup}
    
    // Test CURSED-specific language features
    test_cursed_syntax(data, size);
    test_cursed_stdlib(data, size);
    test_cursed_types(data, size);
    
    {cleanup_code}
    return 0;
}}

void test_cursed_syntax(const uint8_t *data, size_t size) {{
    char *cursed_code = malloc(size + 1);
    if (!cursed_code) return;
    
    memcpy(cursed_code, data, size);
    cursed_code[size] = '\\0';
    
    // Test CURSED parser with generated code
    // {function_name}(cursed_code);
    
    free(cursed_code);
}}

void test_cursed_stdlib(const uint8_t *data, size_t size) {{
    // Test CURSED standard library functions
    // Example: vibez.spill(), mathz functions, etc.
}}

void test_cursed_types(const uint8_t *data, size_t size) {{
    // Test CURSED type system
    // Example: drip, tea, lit types
}}

// CURSED-specific fuzzing dictionary
const char *cursed_dict[] = {{
    "sus", "drip", "slay", "damn", "vibez", "spill", "yeet",
    "based", "cringe", "bestie", "ready", "otherwise", "sick",
    "when", "squad", "collab", "tea", "lit", "normie",
    "mathz", "stringz", "arrayz", "testz", "cryptz"
}};

{additional_test_functions}
"""

    def _get_cursed_lang_build_script(self) -> str:
        return """#!/bin/bash
# Build script for CURSED language fuzz targets

set -e

echo "Building CURSED language fuzz targets..."

# First ensure CURSED compiler is built
if [ ! -f "../zig-out/bin/cursed-zig" ]; then
    echo "Building CURSED compiler first..."
    cd ..
    zig build
    cd fuzz_targets
fi

# Build C targets
CFLAGS="-fsanitize=fuzzer,address -g -O1"
LDFLAGS="-fsanitize=fuzzer,address"

for target in fuzz_cursed_*.c; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        clang $CFLAGS -o "${target%.c}" "$target" $LDFLAGS -I../src-zig
    fi
done

# Build Zig targets
for target in fuzz_cursed_*.zig; do
    if [ -f "$target" ]; then
        echo "Building $target..."
        zig build-exe -fsanitize-c -lc "$target" --main-pkg-path .. -I../src-zig
    fi
done

echo "CURSED language fuzz targets built successfully!"
"""

    def _get_cursed_lang_test_cases(self) -> List[str]:
        return [
            "sus x drip = 42; vibez.spill(x);",
            "slay test() drip { damn 42; }",
            'yeet "mathz"; sus result drip = abs_normie(-5);',
            'ready (based) { vibez.spill("true"); }',
            "bestie (x < 10) { x = x + 1; }",
            "squad Point { spill x drip; spill y drip; }",
            "sus arr []drip = [1, 2, 3];",
            "",
            "sus " + "a" * 1000 + " drip = 42;",
            "\\x00\\x01\\x02",
        ]

    def generate_specific_target(self, template_type: str, function_info: Dict, output_dir: Path):
        """Generate a specific fuzz target based on template and function info"""
        if template_type not in self.templates:
            raise ValueError(f"Unknown template type: {template_type}")
        
        template = self.templates[template_type]
        
        # Determine language-specific content
        language = function_info.get('language', 'c')
        
        includes = self._get_includes_for_language(language)
        language_setup = self._get_language_setup(language)
        allocator_setup = self._get_allocator_setup(language)
        cleanup_code = self._get_cleanup_code(language)
        test_calls = self._get_test_calls(template_type, function_info)
        additional_functions = self._get_additional_functions(template_type, function_info)
        
        # Fill template
        content = template.template_content.format(
            function_name=function_info.get('name', 'unknown_function'),
            file_path=function_info.get('file_path', 'unknown_file'),
            line_number=function_info.get('line_number', 0),
            risk_level=function_info.get('risk_level', 'unknown'),
            includes=includes,
            language_specific_setup=language_setup,
            allocator_setup=allocator_setup,
            cleanup_code=cleanup_code,
            parser_test_calls=test_calls if template_type == 'parser' else '',
            file_io_test_calls=test_calls if template_type == 'file_io' else '',
            network_test_calls=test_calls if template_type == 'network' else '',
            additional_test_functions=additional_functions
        )
        
        # Write target file
        file_ext = 'c' if language in ['c', 'zig'] else 'rs'
        target_file = output_dir / f"fuzz_{template_type}_{function_info['name']}.{file_ext}"
        
        with open(target_file, 'w') as f:
            f.write(content)
        
        # Write build script
        build_script_file = output_dir / f"build_{template_type}.sh"
        with open(build_script_file, 'w') as f:
            f.write(template.build_script)
        build_script_file.chmod(0o755)
        
        # Write test cases
        test_cases_file = output_dir / f"test_cases_{template_type}.txt"
        with open(test_cases_file, 'w') as f:
            for test_case in template.test_cases:
                f.write(repr(test_case) + '\\n')
        
        return target_file

    def _get_includes_for_language(self, language: str) -> str:
        """Get appropriate includes for language"""
        if language == 'c':
            return """#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>"""
        elif language == 'zig':
            return """const std = @import("std");
const testing = std.testing;
const c = @cImport({
    @cInclude("stdint.h");
    @cInclude("stdlib.h");
    @cInclude("string.h");
});"""
        else:
            return "#include <stdint.h>\\n#include <stddef.h>"

    def _get_language_setup(self, language: str) -> str:
        """Get language-specific setup code"""
        if language == 'zig':
            return """// Zig-specific setup
extern fn malloc(size: usize) ?*anyopaque;
extern fn free(ptr: ?*anyopaque) void;"""
        return "// C-specific setup"

    def _get_allocator_setup(self, language: str) -> str:
        """Get allocator setup for language"""
        if language == 'zig':
            return """var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();"""
        return "// C uses malloc/free directly"

    def _get_cleanup_code(self, language: str) -> str:
        """Get cleanup code for language"""
        if language == 'zig':
            return "// Zig GPA cleanup handled by defer"
        return "// C cleanup handled manually"

    def _get_test_calls(self, template_type: str, function_info: Dict) -> str:
        """Generate test calls for specific function"""
        func_name = function_info.get('name', 'unknown_function')
        
        if template_type == 'parser':
            return f"""// Test {func_name} with fuzzed input
    // Example: {func_name}(allocator, input);
    // Example: {func_name}(input, size);"""
        elif template_type == 'file_io':
            return f"""// Test {func_name} with temp file
    // Example: {func_name}(temp_path);
    // Example: {func_name}(temp_path, data, size);"""
        elif template_type == 'network':
            return f"""// Test {func_name} with network data
    // Example: {func_name}(data, size);
    // Example: {func_name}(sockfd, data, size);"""
        
        return f"// Test {func_name} with fuzzed data"

    def _get_additional_functions(self, template_type: str, function_info: Dict) -> str:
        """Generate additional test functions"""
        return f"""
// Additional test functions for {function_info.get('name', 'unknown')}
void test_edge_cases(const uint8_t *data, size_t size) {{
    // Test with edge cases specific to {template_type}
}}

void test_error_conditions(const uint8_t *data, size_t size) {{
    // Test error handling paths
}}
"""

def main():
    """Main entry point for template generator"""
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Fuzz Template Generator")
    parser.add_argument("--report", required=True, help="JSON report from fuzz_target_discovery.py")
    parser.add_argument("--output-dir", default="generated_fuzz_targets", help="Output directory")
    parser.add_argument("--template-type", help="Specific template type to generate")
    
    args = parser.parse_args()
    
    # Load function report
    with open(args.report, 'r') as f:
        report = json.load(f)
    
    # Create output directory
    output_dir = Path(args.output_dir)
    output_dir.mkdir(exist_ok=True)
    
    # Initialize generator
    generator = FuzzTemplateGenerator()
    
    print(f"🎯 Generating specialized fuzz targets...")
    
    # Map input types to template types
    input_type_mapping = {
        'parsing': 'parser',
        'file_io': 'file_io',
        'network': 'network',
        'memory_buffer': 'memory_buffer',
        'user_input': 'cursed_lang',
        'serialization': 'parser'
    }
    
    generated_count = 0
    
    for func_info in report['functions']:
        # Determine appropriate template(s)
        input_types = func_info.get('input_types', [])
        
        templates_to_generate = set()
        for input_type in input_types:
            if input_type in input_type_mapping:
                templates_to_generate.add(input_type_mapping[input_type])
        
        # If no specific input types, use general template based on language
        if not templates_to_generate:
            if func_info.get('language') == 'cursed':
                templates_to_generate.add('cursed_lang')
            else:
                templates_to_generate.add('parser')  # Default to parser
        
        # Generate targets for each template type
        for template_type in templates_to_generate:
            if args.template_type and template_type != args.template_type:
                continue
                
            try:
                target_file = generator.generate_specific_target(
                    template_type, func_info, output_dir
                )
                print(f"  ✅ Generated {target_file}")
                generated_count += 1
            except Exception as e:
                print(f"  ❌ Failed to generate target for {func_info['name']}: {e}")
    
    print(f"\\n🎉 Generated {generated_count} specialized fuzz targets in {output_dir}")
    
    # Generate master build script
    master_build = output_dir / "build_all_targets.sh"
    with open(master_build, 'w') as f:
        f.write("""#!/bin/bash
# Master build script for all fuzz targets

set -e

echo "🚀 Building all CURSED fuzz targets..."

# Run all build scripts
for build_script in build_*.sh; do
    if [ -f "$build_script" ] && [ "$build_script" != "build_all_targets.sh" ]; then
        echo "Running $build_script..."
        bash "$build_script"
    fi
done

echo "✅ All fuzz targets built successfully!"
""")
    master_build.chmod(0o755)
    
    print(f"📋 Master build script: {master_build}")

if __name__ == "__main__":
    main()
