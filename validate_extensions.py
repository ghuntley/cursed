#!/usr/bin/env python3
"""
CURSED Extensions Validation Script

Validates that all CURSED language extensions are properly structured
and contain the required files and syntax definitions.
"""

import os
import sys
from pathlib import Path
import re

class ExtensionValidator:
    def __init__(self):
        self.project_root = Path(__file__).parent
        self.results = {}

    def validate_file_exists(self, path, description):
        """Check if a file exists"""
        full_path = self.project_root / path
        exists = full_path.exists()
        status = "✓" if exists else "✗"
        print(f"{status} {description}: {path}")
        return exists

    def validate_file_contains(self, path, patterns, description):
        """Check if a file contains required patterns"""
        full_path = self.project_root / path
        if not full_path.exists():
            print(f"✗ {description}: {path} (file not found)")
            return False

        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()

            missing = []
            for pattern in patterns:
                if not re.search(pattern, content, re.MULTILINE):
                    missing.append(pattern)

            if missing:
                print(f"✗ {description}: {path} (missing: {missing})")
                return False
            else:
                print(f"✓ {description}: {path}")
                return True
        except Exception as e:
            print(f"✗ {description}: {path} (error: {e})")
            return False

    def validate_tree_sitter(self):
        """Validate Tree-Sitter grammar"""
        print("\n🔍 Validating Tree-Sitter Grammar")
        print("-" * 40)

        ts_dir = "tree-sitter"
        checks = [
            (f"{ts_dir}/grammar.js", "Tree-Sitter grammar file"),
            (f"{ts_dir}/package.json", "Tree-Sitter package.json"),
            (f"{ts_dir}/test/corpus/basic.txt", "Tree-Sitter test corpus"),
        ]

        all_passed = True
        for path, desc in checks:
            if not self.validate_file_exists(path, desc):
                all_passed = False

        # Check grammar contains CURSED keywords
        keywords = ["vibe", "slay", "sus", "be_like", "ready", "based", "cringe"]
        patterns = [rf"'{kw}'" for kw in keywords]
        if not self.validate_file_contains(f"{ts_dir}/grammar.js", patterns,
                                         "Tree-Sitter grammar keywords"):
            all_passed = False

        self.results['tree_sitter'] = all_passed

    def validate_vscode_extension(self):
        """Validate VSCode extension"""
        print("\n🔍 Validating VSCode Extension")
        print("-" * 40)

        vscode_dir = "cursed-vscode-extension"
        checks = [
            (f"{vscode_dir}/package.json", "VSCode package.json"),
            (f"{vscode_dir}/syntaxes/cursed.tmLanguage.json", "VSCode syntax file"),
            (f"{vscode_dir}/language-configuration.json", "VSCode language config"),
        ]

        all_passed = True
        for path, desc in checks:
            if not self.validate_file_exists(path, desc):
                all_passed = False

        # Check package.json has required fields
        package_checks = [
            r'"name":\s*"cursed-language"',
            r'"languages":\s*\[',
            r'"grammars":\s*\[',
            r'"extensions":\s*\["\.💀"',
        ]
        if not self.validate_file_contains(f"{vscode_dir}/package.json", package_checks,
                                         "VSCode package.json structure"):
            all_passed = False

        # Check syntax file has CURSED keywords
        syntax_patterns = [
            r'"match":\s*".*\\bready\\b.*"',
            r'"match":\s*".*\\bslay\\b.*"',
            r'"match":\s*".*\\bbased\\b.*"',
        ]
        if not self.validate_file_contains(f"{vscode_dir}/syntaxes/cursed.tmLanguage.json",
                                         syntax_patterns, "VSCode syntax keywords"):
            all_passed = False

        self.results['vscode'] = all_passed

    def validate_vim_extensions(self):
        """Validate Vim extensions"""
        print("\n🔍 Validating Vim Extensions")
        print("-" * 40)

        vim_extensions = [
            ("cursed-vim-advanced", ["syntax/cursed.vim", "plugin/cursed.vim", "ftdetect/cursed.vim"]),
            ("vim-cursed", ["syntax/cursed.vim", "ftplugin/cursed.vim", "ftdetect/cursed.vim"])
        ]

        all_passed = True

        for ext_name, required_files in vim_extensions:
            print(f"\nValidating {ext_name}:")
            ext_passed = True

            for file_path in required_files:
                full_path = f"{ext_name}/{file_path}"
                if not self.validate_file_exists(full_path, f"{ext_name} {file_path}"):
                    ext_passed = False

            # Check syntax file has keywords
            syntax_file = f"{ext_name}/syntax/cursed.vim"
            if self.validate_file_exists(syntax_file, f"{ext_name} syntax file"):
                keyword_patterns = [
                    r'syn keyword.*\bvibe\b',
                    r'syn keyword.*\bslay\b',
                    r'syn keyword.*\bbased\b',
                    r'syn keyword.*\bcringe\b',
                ]
                if not self.validate_file_contains(syntax_file, keyword_patterns,
                                                 f"{ext_name} syntax keywords"):
                    ext_passed = False

            if not ext_passed:
                all_passed = False

        self.results['vim'] = all_passed

    def validate_intellij_plugin(self):
        """Validate IntelliJ plugin"""
        print("\n🔍 Validating IntelliJ Plugin")
        print("-" * 40)

        ij_dir = "cursed-intellij-plugin"
        checks = [
            (f"{ij_dir}/src/main/kotlin/org/cursed/CursedLexer.kt", "IntelliJ lexer"),
            (f"{ij_dir}/src/main/kotlin/org/cursed/CursedParserDefinition.kt", "IntelliJ parser"),
            (f"{ij_dir}/src/main/resources/META-INF/plugin.xml", "IntelliJ plugin.xml"),
            (f"{ij_dir}/build.gradle.kts", "IntelliJ build file"),
        ]

        all_passed = True
        for path, desc in checks:
            if not self.validate_file_exists(path, desc):
                all_passed = False

        # Check plugin.xml has required elements
        plugin_patterns = [
            r'<name>.*</name>',
            r'<id>.*</id>',
            r'<description>.*</description>',
        ]
        if not self.validate_file_contains(f"{ij_dir}/src/main/resources/META-INF/plugin.xml",
                                         plugin_patterns, "IntelliJ plugin.xml content"):
            all_passed = False

        self.results['intellij'] = all_passed

    def validate_syntax_consistency(self):
        """Validate syntax consistency across extensions"""
        print("\n🔍 Validating Syntax Consistency")
        print("-" * 40)

        # Check that all extensions define the same core keywords
        core_keywords = ["vibe", "slay", "sus", "be_like", "ready", "based", "cringe"]

        extensions_to_check = [
            ("tree-sitter/grammar.js", "Tree-Sitter"),
            ("cursed-vscode-extension/syntaxes/cursed.tmLanguage.json", "VSCode"),
            ("cursed-vim-advanced/syntax/cursed.vim", "Vim Advanced"),
            ("vim-cursed/syntax/cursed.vim", "Vim Cursed"),
        ]

        consistency_results = {}

        for file_path, ext_name in extensions_to_check:
            full_path = self.project_root / file_path
            if full_path.exists():
                with open(full_path, 'r', encoding='utf-8') as f:
                    content = f.read()

                found_keywords = []
                for keyword in core_keywords:
                    # Different patterns for different file types
                    if file_path.endswith('.js'):
                        pattern = rf"'{keyword}'"
                    elif file_path.endswith('.json'):
                        pattern = rf'"{keyword}"'
                    elif file_path.endswith('.vim'):
                        pattern = rf'\b{keyword}\b'
                    else:
                        pattern = rf'\b{keyword}\b'

                    if re.search(pattern, content):
                        found_keywords.append(keyword)

                missing = set(core_keywords) - set(found_keywords)
                if missing:
                    print(f"✗ {ext_name} missing keywords: {missing}")
                    consistency_results[ext_name] = False
                else:
                    print(f"✓ {ext_name} has all core keywords")
                    consistency_results[ext_name] = True
            else:
                print(f"✗ {ext_name} file not found: {file_path}")
                consistency_results[ext_name] = False

        self.results['syntax_consistency'] = all(consistency_results.values())

    def run_validation(self):
        """Run all validations"""
        print("🧪 CURSED Extensions Validation Suite")
        print("=" * 50)

        self.validate_tree_sitter()
        self.validate_vscode_extension()
        self.validate_vim_extensions()
        self.validate_intellij_plugin()
        self.validate_syntax_consistency()

        # Summary
        print("\n" + "=" * 50)
        print("📊 VALIDATION RESULTS SUMMARY")
        print("=" * 50)

        passed = sum(1 for result in self.results.values() if result)
        total = len(self.results)

        for test_name, result in self.results.items():
            status = "✓ PASS" if result else "✗ FAIL"
            test_display = test_name.replace('_', ' ').title()
            print("25")

        print(f"\n🎯 Overall: {passed}/{total} validations passed")

        if passed == total:
            print("🎉 All validations passed! CURSED extensions are properly structured.")
            return 0
        else:
            print("❌ Some validations failed. Please check the issues above.")
            return 1

if __name__ == "__main__":
    validator = ExtensionValidator()
    sys.exit(validator.run_validation())