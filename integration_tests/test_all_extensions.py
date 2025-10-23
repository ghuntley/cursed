#!/usr/bin/env python3
"""
CURSED Extensions Integration Test Suite

This script tests all CURSED language extensions to ensure they work correctly
and provide consistent behavior across different editors and environments.
"""

import os
import sys
import subprocess
import tempfile
import shutil
from pathlib import Path

class ExtensionTester:
    def __init__(self):
        self.test_dir = Path(__file__).parent
        self.project_root = self.test_dir.parent
        self.results = {}

    def run_command(self, cmd, cwd=None, timeout=30):
        """Run a command and return (success, output, error)"""
        try:
            result = subprocess.run(
                cmd,
                shell=True,
                cwd=cwd or self.project_root,
                capture_output=True,
                text=True,
                timeout=timeout
            )
            return result.returncode == 0, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return False, "", "Command timed out"
        except Exception as e:
            return False, "", str(e)

    def test_tree_sitter(self):
        """Test Tree-Sitter grammar parsing"""
        print("Testing Tree-Sitter grammar...")

        os.chdir(self.project_root / "tree-sitter")

        # Test parsing
        success, stdout, stderr = self.run_command("npm test")
        if success:
            print("✓ Tree-Sitter tests passed")
            self.results['tree_sitter'] = True
        else:
            print("✗ Tree-Sitter tests failed")
            print(stderr)
            self.results['tree_sitter'] = False

        # Test parsing our test file
        test_file = self.project_root / "test_syntax.💀"
        if test_file.exists():
            success, stdout, stderr = self.run_command(f"npx tree-sitter parse {test_file}")
            if success and "ERROR" not in stdout:
                print("✓ Tree-Sitter can parse CURSED files")
            else:
                print("✗ Tree-Sitter parsing failed")
                print(stderr)

    def test_vscode_extension(self):
        """Test VSCode extension"""
        print("Testing VSCode extension...")

        vscode_dir = self.project_root / "cursed-vscode-extension"

        if not vscode_dir.exists():
            print("✗ VSCode extension directory not found")
            self.results['vscode'] = False
            return

        os.chdir(vscode_dir)

        # Install dependencies
        success, _, stderr = self.run_command("npm install")
        if not success:
            print("✗ Failed to install VSCode extension dependencies")
            print(stderr)
            self.results['vscode'] = False
            return

        # Compile
        success, _, stderr = self.run_command("npm run compile")
        if not success:
            print("✗ Failed to compile VSCode extension")
            print(stderr)
            self.results['vscode'] = False
            return

        # Check package.json structure
        import json
        try:
            with open("package.json", "r") as f:
                package = json.load(f)

            # Check required fields
            required_fields = [
                "name", "displayName", "version", "engines",
                "contributes.languages", "contributes.grammars"
            ]

            missing_fields = []
            for field in required_fields:
                keys = field.split(".")
                obj = package
                for key in keys:
                    if key not in obj:
                        missing_fields.append(field)
                        break
                    obj = obj[key]

            if missing_fields:
                print(f"✗ Missing required fields in package.json: {missing_fields}")
                self.results['vscode'] = False
            else:
                print("✓ VSCode extension package.json is valid")
                self.results['vscode'] = True

        except Exception as e:
            print(f"✗ Failed to validate VSCode extension: {e}")
            self.results['vscode'] = False

    def test_vim_extensions(self):
        """Test Vim extensions"""
        print("Testing Vim extensions...")

        vim_dirs = [
            self.project_root / "cursed-vim-advanced",
            self.project_root / "vim-cursed"
        ]

        all_passed = True

        for vim_dir in vim_dirs:
            if not vim_dir.exists():
                print(f"✗ Vim extension directory not found: {vim_dir.name}")
                all_passed = False
                continue

            print(f"Testing {vim_dir.name}...")

            # Check for required files
            required_files = ["syntax/cursed.vim", "ftdetect/cursed.vim"]
            if vim_dir.name == "cursed-vim-advanced":
                required_files.extend(["plugin/cursed.vim"])

            missing_files = []
            for file_path in required_files:
                if not (vim_dir / file_path).exists():
                    missing_files.append(file_path)

            if missing_files:
                print(f"✗ Missing required files in {vim_dir.name}: {missing_files}")
                all_passed = False
            else:
                print(f"✓ {vim_dir.name} has all required files")

                # Test syntax file
                syntax_file = vim_dir / "syntax/cursed.vim"
                with open(syntax_file, "r") as f:
                    content = f.read()

                # Check for CURSED-specific keywords
                required_keywords = ["vibe", "slay", "sus", "be_like", "ready", "based", "cringe"]
                missing_keywords = []

                for keyword in required_keywords:
                    if f"syn keyword.*\\b{keyword}\\b" not in content:
                        missing_keywords.append(keyword)

                if missing_keywords:
                    print(f"✗ Missing keywords in {vim_dir.name} syntax: {missing_keywords}")
                    all_passed = False
                else:
                    print(f"✓ {vim_dir.name} syntax contains required keywords")

        self.results['vim'] = all_passed

    def test_intellij_plugin(self):
        """Test IntelliJ plugin"""
        print("Testing IntelliJ plugin...")

        intellij_dir = self.project_root / "cursed-intellij-plugin"

        if not intellij_dir.exists():
            print("✗ IntelliJ plugin directory not found")
            self.results['intellij'] = False
            return

        # Check for required files
        required_files = [
            "src/main/kotlin/org/cursed/CursedLexer.kt",
            "src/main/kotlin/org/cursed/CursedParserDefinition.kt",
            "src/main/resources/META-INF/plugin.xml",
            "build.gradle.kts"
        ]

        missing_files = []
        for file_path in required_files:
            if not (intellij_dir / file_path).exists():
                missing_files.append(file_path)

        if missing_files:
            print(f"✗ Missing required files in IntelliJ plugin: {missing_files}")
            self.results['intellij'] = False
        else:
            print("✓ IntelliJ plugin has all required files")

            # Check plugin.xml
            plugin_xml = intellij_dir / "src/main/resources/META-INF/plugin.xml"
            with open(plugin_xml, "r") as f:
                content = f.read()

            if "<name>" in content and "<id>" in content:
                print("✓ IntelliJ plugin.xml is valid")
                self.results['intellij'] = True
            else:
                print("✗ IntelliJ plugin.xml is missing required elements")
                self.results['intellij'] = False

    def test_syntax_consistency(self):
        """Test that all extensions handle syntax consistently"""
        print("Testing syntax consistency across extensions...")

        # Test file with various CURSED constructs
        test_content = '''vibe main

yeet "vibez"

fr fr line comment
no cap
block comment
on god

be_like Person squad {
    name tea
    age normie
}

slay main() {
    sus p Person = Person{name: "Alice", age: 30}
    ready p.age > 18 {
        vibez.spill("Adult")
    } otherwise {
        vibez.spill("Minor")
    }

    bestie i = 0; i < 10; i++ {
        vibez.spill(i)
    }

    sus ch dm<tea> = make(dm<tea>, 10)
    stan func() {
        dm_send(ch, "hello")
    }()

    sus msg, ok = dm_recv(ch)
    ready ok {
        vibez.spill("Received:", msg)
    }

    dm_close(ch)

    fam {
        risky_operation()
    } sus err {
        vibez.spill("Error:", err.message())
    }
}'''

        # Write test file
        test_file = self.project_root / "syntax_consistency_test.💀"
        with open(test_file, "w") as f:
            f.write(test_content)

        print("✓ Created syntax consistency test file")

        # Test with Tree-Sitter
        os.chdir(self.project_root / "tree-sitter")
        success, stdout, stderr = self.run_command(f"npx tree-sitter parse {test_file}")

        if success and "ERROR" not in stdout:
            print("✓ Tree-Sitter can parse comprehensive CURSED syntax")
            syntax_consistent = True
        else:
            print("✗ Tree-Sitter failed to parse comprehensive syntax")
            syntax_consistent = False

        # Clean up
        test_file.unlink(missing_ok=True)

        self.results['syntax_consistency'] = syntax_consistent

    def test_file_extensions(self):
        """Test that all extensions recognize CURSED file extensions"""
        print("Testing file extension recognition...")

        extensions_to_test = [".💀", ".cursed"]

        results = {}
        for ext in extensions_to_test:
            test_file = self.project_root / f"test{ext}"
            test_file.write_text("vibe main\nslay main() {}\n")

            # Test Tree-Sitter recognition
            os.chdir(self.project_root / "tree-sitter")
            success, _, _ = self.run_command(f"npx tree-sitter parse {test_file}")

            results[ext] = success
            test_file.unlink()

        all_recognized = all(results.values())
        if all_recognized:
            print("✓ All file extensions are properly recognized")
        else:
            unrecognized = [ext for ext, recognized in results.items() if not recognized]
            print(f"✗ Some file extensions not recognized: {unrecognized}")

        self.results['file_extensions'] = all_recognized

    def run_all_tests(self):
        """Run all tests"""
        print("🧪 Running CURSED Extensions Integration Test Suite")
        print("=" * 60)

        self.test_tree_sitter()
        print()

        self.test_vscode_extension()
        print()

        self.test_vim_extensions()
        print()

        self.test_intellij_plugin()
        print()

        self.test_syntax_consistency()
        print()

        self.test_file_extensions()
        print()

        # Summary
        print("=" * 60)
        print("📊 TEST RESULTS SUMMARY")
        print("=" * 60)

        passed = 0
        total = len(self.results)

        for test_name, result in self.results.items():
            status = "✓ PASS" if result else "✗ FAIL"
            print("25")
            if result:
                passed += 1

        print(f"\n🎯 Overall: {passed}/{total} tests passed")

        if passed == total:
            print("🎉 All tests passed! CURSED extensions are working correctly.")
            return 0
        else:
            print("❌ Some tests failed. Please check the output above.")
            return 1

if __name__ == "__main__":
    tester = ExtensionTester()
    sys.exit(tester.run_all_tests())