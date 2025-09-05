#!/usr/bin/env python3
"""
Differential Bootstrap Testing Framework
Compares output between original and self-compiled CURSED compilers
"""

import os
import subprocess
import json
import tempfile
import difflib
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass
from datetime import datetime
import hashlib

@dataclass
class TestResult:
    test_name: str
    original_output: str
    original_exit_code: int
    stage2_output: str
    stage2_exit_code: int
    outputs_match: bool
    exit_codes_match: bool
    test_passed: bool
    diff: str
    execution_time_original: float
    execution_time_stage2: float

class DifferentialTester:
    def __init__(self, original_compiler: str, stage2_compiler: str):
        self.original_compiler = original_compiler
        self.stage2_compiler = stage2_compiler
        self.test_results: List[TestResult] = []
        
    def run_compiler_test(self, compiler: str, test_file: str, timeout: int = 30) -> Tuple[str, int, float]:
        """Run a compiler test and return (output, exit_code, execution_time)"""
        import time
        
        start_time = time.time()
        try:
            result = subprocess.run(
                [compiler, test_file],
                capture_output=True,
                text=True,
                timeout=timeout
            )
            end_time = time.time()
            
            execution_time = end_time - start_time
            return result.stdout + result.stderr, result.returncode, execution_time
            
        except subprocess.TimeoutExpired:
            return "TIMEOUT", 124, timeout
        except Exception as e:
            return f"ERROR: {str(e)}", 1, 0
    
    def create_test_programs(self) -> Dict[str, str]:
        """Create a comprehensive set of test programs"""
        tests = {
            "basic_print": '''vibez.spill("Hello, World!")''',
            
            "variables": '''
sus x normie = 42
sus y normie = 24
sus result normie = x + y
vibez.spill("Result: " + result.to_string())
''',
            
            "conditionals": '''
sus value normie = 15
lowkey (value > 10) {
    vibez.spill("Value is greater than 10")
} highkey {
    vibez.spill("Value is not greater than 10")
}
''',
            
            "loops": '''
sus count normie = 5
periodt (count > 0) {
    vibez.spill("Count: " + count.to_string())
    count = count - 1
}
''',
            
            "functions": '''
slay add(a normie, b normie) normie {
    damn a + b
}

sus result normie = add(10, 20)
vibez.spill("Function result: " + result.to_string())
''',
            
            "recursion": '''
slay factorial(n normie) normie {
    lowkey (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

sus result normie = factorial(5)
vibez.spill("Factorial(5) = " + result.to_string())
''',
            
            "tuples": '''
sus tuple := (42, "hello", based)
sus first normie = tuple.0
sus second tea = tuple.1
sus third lit = tuple.2
vibez.spill("Tuple: " + first.to_string() + " " + second + " " + third.to_string())
''',
            
            "type_assertions": '''
sus x normie = 42
sus y meal = x.(meal)
vibez.spill("Type assertion: " + y.to_string())
''',
            
            "short_declarations": '''
x := 100
y := 200
sum := x + y
vibez.spill("Short declaration sum: " + sum.to_string())
''',
            
            "increment_operators": '''
sus counter normie = 0
counter++
++counter
vibez.spill("Counter after increments: " + counter.to_string())
''',
            
            "mixed_arithmetic": '''
sus int_val normie = 5
sus float_val meal = 3.14
sus result meal = int_val.(meal) * float_val
vibez.spill("Mixed arithmetic: " + result.to_string())
''',
            
            "complex_control": '''
sus i normie = 1
periodt (i <= 3) {
    lowkey (i % 2 == 0) {
        vibez.spill("Even: " + i.to_string())
    } highkey {
        vibez.spill("Odd: " + i.to_string())
    }
    i++
}
''',
            
            "nested_functions": '''
slay outer(x normie) normie {
    slay inner(y normie) normie {
        damn x + y
    }
    damn inner(10)
}

sus result normie = outer(5)
vibez.spill("Nested function result: " + result.to_string())
''',
            
            "error_syntax": '''
# This should cause a syntax error
sus x normie = 42
vibez.spill("Missing quote)
''',
            
            "error_type": '''
# This should cause a type error  
sus x normie = 42
sus y tea = "hello"
sus result normie = x + y  # Type mismatch
''',
        }
        
        return tests
    
    def run_differential_test(self, test_name: str, program_content: str) -> TestResult:
        """Run a differential test between original and Stage 2 compilers"""
        print(f"🔍 Running differential test: {test_name}")
        
        # Create temporary test file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.💀', delete=False) as f:
            f.write(program_content)
            test_file = f.name
        
        try:
            # Run with original compiler
            original_output, original_exit, original_time = self.run_compiler_test(
                self.original_compiler, test_file
            )
            
            # Run with Stage 2 compiler
            stage2_output, stage2_exit, stage2_time = self.run_compiler_test(
                self.stage2_compiler, test_file
            )
            
            # Compare results
            outputs_match = original_output.strip() == stage2_output.strip()
            exit_codes_match = original_exit == stage2_exit
            
            # Generate diff if outputs don't match
            diff = ""
            if not outputs_match:
                diff_lines = list(difflib.unified_diff(
                    original_output.splitlines(keepends=True),
                    stage2_output.splitlines(keepends=True),
                    fromfile="original",
                    tofile="stage2",
                    lineterm=""
                ))
                diff = ''.join(diff_lines)
            
            # Determine if test passed
            # For error tests, we expect both to fail
            if test_name.startswith("error_"):
                test_passed = (original_exit != 0 and stage2_exit != 0)
            else:
                test_passed = outputs_match and exit_codes_match and original_exit == 0
            
            result = TestResult(
                test_name=test_name,
                original_output=original_output,
                original_exit_code=original_exit,
                stage2_output=stage2_output,
                stage2_exit_code=stage2_exit,
                outputs_match=outputs_match,
                exit_codes_match=exit_codes_match,
                test_passed=test_passed,
                diff=diff,
                execution_time_original=original_time,
                execution_time_stage2=stage2_time
            )
            
            status = "✅ PASS" if test_passed else "❌ FAIL"
            print(f"   {status} - Original: {original_exit}, Stage2: {stage2_exit}")
            
            return result
            
        finally:
            # Cleanup temporary file
            os.unlink(test_file)
    
    def run_all_tests(self) -> List[TestResult]:
        """Run all differential tests"""
        print("🚀 Starting comprehensive differential testing")
        
        test_programs = self.create_test_programs()
        results = []
        
        for test_name, program_content in test_programs.items():
            result = self.run_differential_test(test_name, program_content)
            results.append(result)
            self.test_results.append(result)
        
        return results
    
    def generate_report(self, output_file: str = "differential_test_report.json"):
        """Generate comprehensive test report"""
        # Calculate statistics
        total_tests = len(self.test_results)
        passed_tests = sum(1 for r in self.test_results if r.test_passed)
        failed_tests = total_tests - passed_tests
        
        output_matches = sum(1 for r in self.test_results if r.outputs_match)
        exit_code_matches = sum(1 for r in self.test_results if r.exit_codes_match)
        
        # Calculate performance metrics
        total_original_time = sum(r.execution_time_original for r in self.test_results)
        total_stage2_time = sum(r.execution_time_stage2 for r in self.test_results)
        
        performance_ratio = (total_stage2_time / total_original_time * 100) if total_original_time > 0 else 0
        
        # Create report
        report = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total_tests": total_tests,
                "passed_tests": passed_tests,
                "failed_tests": failed_tests,
                "pass_rate": (passed_tests / total_tests * 100) if total_tests > 0 else 0,
                "output_matches": output_matches,
                "exit_code_matches": exit_code_matches,
                "total_original_time": total_original_time,
                "total_stage2_time": total_stage2_time,
                "performance_ratio": performance_ratio
            },
            "test_results": [
                {
                    "test_name": r.test_name,
                    "test_passed": r.test_passed,
                    "outputs_match": r.outputs_match,
                    "exit_codes_match": r.exit_codes_match,
                    "original_exit_code": r.original_exit_code,
                    "stage2_exit_code": r.stage2_exit_code,
                    "execution_time_original": r.execution_time_original,
                    "execution_time_stage2": r.execution_time_stage2,
                    "has_diff": bool(r.diff),
                    "diff_preview": r.diff[:500] if r.diff else None
                }
                for r in self.test_results
            ]
        }
        
        # Save JSON report
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        # Generate markdown report
        md_file = output_file.replace('.json', '.md')
        self.generate_markdown_report(md_file, report)
        
        print(f"\n📊 Differential test report generated:")
        print(f"   JSON: {output_file}")
        print(f"   Markdown: {md_file}")
        
        return report
    
    def generate_markdown_report(self, filename: str, report: Dict):
        """Generate markdown report"""
        with open(filename, 'w') as f:
            f.write("# Differential Bootstrap Testing Report\n\n")
            f.write(f"**Generated:** {report['timestamp']}\n\n")
            
            # Summary
            summary = report['summary']
            f.write("## Summary\n\n")
            f.write(f"- **Total Tests:** {summary['total_tests']}\n")
            f.write(f"- **Passed:** {summary['passed_tests']}\n")
            f.write(f"- **Failed:** {summary['failed_tests']}\n")
            f.write(f"- **Pass Rate:** {summary['pass_rate']:.1f}%\n")
            f.write(f"- **Output Matches:** {summary['output_matches']}/{summary['total_tests']}\n")
            f.write(f"- **Exit Code Matches:** {summary['exit_code_matches']}/{summary['total_tests']}\n")
            f.write(f"- **Performance Ratio:** {summary['performance_ratio']:.1f}% (Stage2 vs Original)\n\n")
            
            # Status indicator
            if summary['pass_rate'] >= 90:
                f.write("🟢 **Status: EXCELLENT** - Self-hosting compiler is highly reliable\n\n")
            elif summary['pass_rate'] >= 75:
                f.write("🟡 **Status: GOOD** - Self-hosting compiler is mostly functional\n\n")
            elif summary['pass_rate'] >= 50:
                f.write("🟠 **Status: DEVELOPING** - Self-hosting compiler has significant issues\n\n")
            else:
                f.write("🔴 **Status: EARLY** - Self-hosting compiler needs major work\n\n")
            
            # Detailed results
            f.write("## Detailed Test Results\n\n")
            f.write("| Test | Status | Output Match | Exit Code Match | Time (Orig/Stage2) |\n")
            f.write("|------|--------|--------------|-----------------|-------------------|\n")
            
            for test in report['test_results']:
                status = "✅" if test['test_passed'] else "❌"
                output_match = "✅" if test['outputs_match'] else "❌"
                exit_match = "✅" if test['exit_codes_match'] else "❌"
                time_info = f"{test['execution_time_original']:.3f}s / {test['execution_time_stage2']:.3f}s"
                
                f.write(f"| {test['test_name']} | {status} | {output_match} | {exit_match} | {time_info} |\n")
            
            # Failed tests details
            failed_tests = [t for t in report['test_results'] if not t['test_passed']]
            if failed_tests:
                f.write("\n## Failed Tests Analysis\n\n")
                for test in failed_tests:
                    f.write(f"### {test['test_name']}\n\n")
                    f.write(f"- **Original Exit Code:** {test['original_exit_code']}\n")
                    f.write(f"- **Stage2 Exit Code:** {test['stage2_exit_code']}\n")
                    f.write(f"- **Output Match:** {'Yes' if test['outputs_match'] else 'No'}\n")
                    
                    if test['has_diff'] and test['diff_preview']:
                        f.write("\n**Output Diff Preview:**\n")
                        f.write("```diff\n")
                        f.write(test['diff_preview'])
                        f.write("\n```\n\n")
    
    def print_summary(self):
        """Print summary to console"""
        total = len(self.test_results)
        passed = sum(1 for r in self.test_results if r.test_passed)
        failed = total - passed
        
        print(f"\n🎯 DIFFERENTIAL TESTING SUMMARY")
        print(f"================================")
        print(f"Total Tests: {total}")
        print(f"Passed: {passed}")
        print(f"Failed: {failed}")
        print(f"Pass Rate: {(passed/total*100):.1f}%")
        
        if failed > 0:
            print(f"\n❌ Failed Tests:")
            for r in self.test_results:
                if not r.test_passed:
                    print(f"  - {r.test_name}")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Differential Bootstrap Testing")
    parser.add_argument("--original", default="./target/release/cursed", 
                       help="Path to original compiler")
    parser.add_argument("--stage2", default="./cursed_stage2",
                       help="Path to Stage 2 compiler")
    parser.add_argument("--output", default="differential_test_report.json",
                       help="Output report file")
    
    args = parser.parse_args()
    
    # Check if compilers exist
    if not os.path.exists(args.original):
        print(f"❌ Original compiler not found: {args.original}")
        return 1
    
    if not os.path.exists(args.stage2):
        print(f"❌ Stage 2 compiler not found: {args.stage2}")
        print("   Try building it first with:")
        print(f"   {args.original} -- compile src/bootstrap/stage2/main.💀 -o cursed_stage2")
        return 1
    
    # Run differential testing
    tester = DifferentialTester(args.original, args.stage2)
    tester.run_all_tests()
    
    # Generate report
    report = tester.generate_report(args.output)
    tester.print_summary()
    
    # Return appropriate exit code
    pass_rate = report['summary']['pass_rate']
    if pass_rate >= 75:
        print("\n✅ Differential testing completed successfully!")
        return 0
    else:
        print(f"\n⚠️ Differential testing completed with issues (pass rate: {pass_rate:.1f}%)")
        return 1

if __name__ == "__main__":
    exit(main())
