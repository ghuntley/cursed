#!/usr/bin/env python3
"""
Comprehensive Memory Analysis for CURSED Zig Implementation
"""

import subprocess
import os
import time
import json
from dataclasses import dataclass
from typing import List, Dict, Any

@dataclass
class MemoryProfile:
    program: str
    heap_usage: int
    allocs: int
    frees: int
    leaks: int
    peak_memory: int
    execution_time: float

class MemoryAnalyzer:
    def __init__(self, cursed_binary: str):
        self.cursed_binary = cursed_binary
        self.test_programs = []
        
    def create_test_programs(self) -> List[str]:
        """Create various test programs to stress memory usage"""
        programs = []
        
        # Simple program
        programs.append('''
fr fr Simple program
yeet "testz"
test_start("Simple Test")
vibez.spill("Hello World")
print_test_summary()
        ''')
        
        # Memory intensive program
        programs.append('''
fr fr Memory intensive program  
yeet "testz"
test_start("Memory Test")
sus arrays [][]normie = []
bestie i := 0; i < 100; i = i + 1 {
    sus inner []normie = []
    bestie j := 0; j < 100; j = j + 1 {
        inner.push(i * j)
    }
    arrays.push(inner)
}
print_test_summary()
        ''')
        
        # String manipulation program
        programs.append('''
fr fr String manipulation
yeet "testz"
test_start("String Test")
sus big_string tea = ""
bestie i := 0; i < 1000; i = i + 1 {
    big_string = big_string + "segment_" + i + "_"
}
assert_true(big_string.len() > 5000)
print_test_summary()
        ''')
        
        # Complex data structures
        programs.append('''
fr fr Complex structures
yeet "testz"

squad Point {
    spill x meal
    spill y meal
}

test_start("Complex Test")
sus points []Point = []
bestie i := 0; i < 200; i = i + 1 {
    sus p Point = Point{x: i * 1.5, y: i * 2.5}
    points.push(p)
}
print_test_summary()
        ''')
        
        return programs
    
    def run_valgrind_analysis(self, program_content: str) -> MemoryProfile:
        """Run valgrind analysis on a program"""
        # Write program to temp file
        program_file = f"/tmp/memory_test_{int(time.time())}.csd"
        with open(program_file, 'w') as f:
            f.write(program_content)
        
        try:
            start_time = time.time()
            
            # Run valgrind
            cmd = [
                'valgrind', 
                '--tool=memcheck',
                '--leak-check=full',
                '--show-leak-kinds=all',
                '--track-origins=yes',
                '--log-fd=2',
                self.cursed_binary,
                program_file
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            execution_time = time.time() - start_time
            
            # Parse valgrind output
            output = result.stderr
            heap_usage = 0
            allocs = 0
            frees = 0
            leaks = 0
            peak_memory = 0
            
            for line in output.split('\n'):
                if 'total heap usage:' in line:
                    parts = line.split()
                    for i, part in enumerate(parts):
                        if part == 'allocs,' and i > 0:
                            allocs = int(parts[i-1])
                        elif part == 'frees,' and i > 0:
                            frees = int(parts[i-1])
                        elif 'bytes' in part and i > 0:
                            heap_usage = int(parts[i-1].replace(',', ''))
                
                if 'in use at exit:' in line:
                    parts = line.split()
                    if len(parts) >= 6:
                        leaks = int(parts[5])
                        
                if 'peak heap size:' in line:
                    parts = line.split()
                    if len(parts) >= 4:
                        peak_memory = int(parts[3].replace(',', ''))
            
            return MemoryProfile(
                program=program_file,
                heap_usage=heap_usage, 
                allocs=allocs,
                frees=frees,
                leaks=leaks,
                peak_memory=peak_memory,
                execution_time=execution_time
            )
            
        except subprocess.TimeoutExpired:
            return MemoryProfile(
                program=program_file,
                heap_usage=-1,
                allocs=-1, 
                frees=-1,
                leaks=-1,
                peak_memory=-1,
                execution_time=30.0
            )
        finally:
            if os.path.exists(program_file):
                os.remove(program_file)
    
    def analyze_memory_patterns(self) -> Dict[str, Any]:
        """Analyze memory usage patterns across different program types"""
        programs = self.create_test_programs()
        profiles = []
        
        print("Running comprehensive memory analysis...")
        
        for i, program in enumerate(programs):
            print(f"Analyzing program {i+1}/{len(programs)}...")
            profile = self.run_valgrind_analysis(program)
            profiles.append(profile)
            
        # Calculate summary statistics
        total_allocs = sum(p.allocs for p in profiles if p.allocs >= 0)
        total_frees = sum(p.frees for p in profiles if p.frees >= 0)
        total_leaks = sum(p.leaks for p in profiles if p.leaks >= 0)
        avg_execution_time = sum(p.execution_time for p in profiles) / len(profiles)
        max_peak_memory = max((p.peak_memory for p in profiles if p.peak_memory >= 0), default=0)
        
        return {
            'summary': {
                'total_programs_tested': len(programs),
                'total_allocations': total_allocs,
                'total_frees': total_frees,
                'total_leaks': total_leaks,
                'avg_execution_time': avg_execution_time,
                'max_peak_memory': max_peak_memory,
                'memory_efficiency': (total_frees / total_allocs * 100) if total_allocs > 0 else 100
            },
            'profiles': [
                {
                    'heap_usage': p.heap_usage,
                    'allocs': p.allocs,
                    'frees': p.frees,
                    'leaks': p.leaks,
                    'peak_memory': p.peak_memory,
                    'execution_time': p.execution_time
                } for p in profiles
            ]
        }
    
    def generate_report(self) -> str:
        """Generate memory analysis report"""
        analysis = self.analyze_memory_patterns()
        
        report = """
# CURSED Zig Implementation Memory Analysis Report

## Summary Statistics
"""
        
        summary = analysis['summary']
        report += f"""
- **Programs Tested**: {summary['total_programs_tested']}
- **Total Allocations**: {summary['total_allocations']}
- **Total Frees**: {summary['total_frees']}
- **Memory Leaks**: {summary['total_leaks']} bytes
- **Average Execution Time**: {summary['avg_execution_time']:.3f}s
- **Peak Memory Usage**: {summary['max_peak_memory']} bytes
- **Memory Efficiency**: {summary['memory_efficiency']:.1f}%
"""
        
        # Memory leak analysis
        if summary['total_leaks'] == 0:
            report += "\n## ✅ Memory Leak Analysis: EXCELLENT\n"
            report += "No memory leaks detected across all test programs.\n"
        else:
            report += f"\n## ⚠️ Memory Leak Analysis: {summary['total_leaks']} bytes leaked\n"
            
        # Performance analysis
        if summary['avg_execution_time'] < 1.0:
            report += "\n## ✅ Performance Analysis: EXCELLENT\n"
            report += f"Average execution time of {summary['avg_execution_time']:.3f}s is excellent.\n"
        else:
            report += f"\n## ⚠️ Performance Analysis: {summary['avg_execution_time']:.3f}s execution time\n"
            
        # Memory efficiency analysis
        if summary['memory_efficiency'] >= 99:
            report += "\n## ✅ Memory Efficiency: EXCELLENT\n"
            report += f"Memory efficiency of {summary['memory_efficiency']:.1f}% indicates proper cleanup.\n"
        else:
            report += f"\n## ⚠️ Memory Efficiency: {summary['memory_efficiency']:.1f}%\n"
            
        return report

def main():
    analyzer = MemoryAnalyzer("./cursed-unified")
    
    if not os.path.exists("./cursed-unified"):
        print("Error: cursed-unified binary not found. Run: zig build-exe src-zig/main_unified.zig -lc --name cursed-unified")
        return
        
    report = analyzer.generate_report()
    print(report)
    
    # Save report
    with open('memory_analysis_report.md', 'w') as f:
        f.write(report)
    print("\nReport saved to memory_analysis_report.md")

if __name__ == "__main__":
    main()
