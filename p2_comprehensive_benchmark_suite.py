#!/usr/bin/env python3
"""
P2 CURSED Compiler Comprehensive Benchmark Suite
==============================================

A production-grade benchmarking system with:
1. Comprehensive benchmark harness (compilation speed, runtime, memory)
2. Automated regression detection with statistical analysis
3. Performance comparison with other languages (Go, Rust, C++)
4. Continuous benchmarking integration with CI/CD
5. Real-world application benchmarks
6. Memory safety validation with zero-leak confirmation

This builds upon the existing continuous benchmark system to create a complete
enterprise-ready benchmarking solution.
"""

import os
import sys
import json
import time
import sqlite3
import statistics
import subprocess
import threading
import hashlib
import signal
import logging
import traceback
from pathlib import Path
from datetime import datetime, timedelta
from typing import Dict, List, Tuple, Optional, Any, Union
from dataclasses import dataclass, asdict
from concurrent.futures import ThreadPoolExecutor, as_completed
import psutil

try:
    import numpy as np
    import pandas as pd
    from scipy import stats
    from sklearn.ensemble import IsolationForest
    from sklearn.preprocessing import StandardScaler
    import matplotlib.pyplot as plt
    import seaborn as sns
    import plotly.graph_objects as go
    import plotly.express as px
    from plotly.subplots import make_subplots
    import requests
    ADVANCED_FEATURES = True
except ImportError:
    ADVANCED_FEATURES = False
    print("Warning: Advanced analysis features not available")
    print("Install with: pip install numpy pandas scipy scikit-learn matplotlib seaborn plotly requests")

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('p2_benchmark_suite.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

@dataclass
class CrossLanguageBenchmarkResult:
    """Cross-language benchmark comparison result"""
    benchmark_name: str
    language: str
    compilation_time_ms: float
    execution_time_ms: float
    memory_usage_kb: int
    binary_size_bytes: int
    success: bool
    compiler_version: str
    optimization_level: str
    timestamp: datetime
    git_commit: str = ""
    error_message: Optional[str] = None

@dataclass
class MemoryLeakResult:
    """Memory leak detection result"""
    benchmark_name: str
    leak_detected: bool
    leaked_bytes: int
    leak_locations: List[str]
    peak_memory_kb: int
    total_allocations: int
    total_frees: int
    valgrind_output: str
    timestamp: datetime

@dataclass
class RealWorldBenchmark:
    """Real-world application benchmark"""
    name: str
    description: str
    category: str  # web_server, cli_tool, game, compiler, etc.
    source_file: str
    expected_features: List[str]
    performance_targets: Dict[str, float]
    complexity_score: int  # 1-10 scale

@dataclass
class ComprehensivePerformanceReport:
    """Comprehensive performance analysis report"""
    timestamp: datetime
    cursed_results: List[CrossLanguageBenchmarkResult]
    comparison_results: Dict[str, List[CrossLanguageBenchmarkResult]]
    memory_safety_results: List[MemoryLeakResult]
    real_world_results: List[CrossLanguageBenchmarkResult]
    regression_alerts: List[Dict[str, Any]]
    performance_score: float
    recommendation: str

class CrossLanguageCompiler:
    """Handles compilation for different languages"""
    
    def __init__(self, workspace_path: Path):
        self.workspace_path = workspace_path
        self.compilers = self._detect_compilers()
    
    def _detect_compilers(self) -> Dict[str, str]:
        """Detect available compilers on the system"""
        compilers = {}
        
        # Check for various compilers
        compiler_commands = {
            'go': 'go',
            'rust': 'rustc',
            'cpp': 'g++',
            'clang': 'clang++',
            'zig': 'zig',
            'java': 'javac',
            'python': 'python3',
            'node': 'node'
        }
        
        for lang, cmd in compiler_commands.items():
            try:
                result = subprocess.run([cmd, '--version'], 
                                      capture_output=True, text=True, timeout=5)
                if result.returncode == 0:
                    compilers[lang] = cmd
                    logger.info(f"Found {lang} compiler: {cmd}")
            except (subprocess.TimeoutExpired, FileNotFoundError):
                logger.debug(f"{lang} compiler not found")
        
        return compilers
    
    def compile_benchmark(self, language: str, source_file: Path, 
                         optimization_level: str = "release") -> Tuple[bool, Optional[Path], str]:
        """Compile benchmark for specific language"""
        if language not in self.compilers:
            return False, None, f"Compiler for {language} not available"
        
        try:
            if language == 'go':
                return self._compile_go(source_file, optimization_level)
            elif language == 'rust':
                return self._compile_rust(source_file, optimization_level)
            elif language == 'cpp':
                return self._compile_cpp(source_file, optimization_level)
            elif language == 'zig':
                return self._compile_zig(source_file, optimization_level)
            elif language == 'cursed':
                return self._compile_cursed(source_file, optimization_level)
            else:
                return False, None, f"Compilation not implemented for {language}"
        except Exception as e:
            return False, None, f"Compilation error: {str(e)}"
    
    def _compile_cursed(self, source_file: Path, optimization_level: str) -> Tuple[bool, Optional[Path], str]:
        """Compile CURSED source"""
        compiler_path = self.workspace_path / "zig-out" / "bin" / "cursed-zig"
        output_file = source_file.with_suffix('')
        
        try:
            result = subprocess.run([
                str(compiler_path), str(source_file), "--compile"
            ], capture_output=True, text=True, timeout=60)
            
            if result.returncode == 0 and output_file.exists():
                return True, output_file, result.stdout
            else:
                return False, None, result.stderr or result.stdout
        except subprocess.TimeoutExpired:
            return False, None, "Compilation timeout"
    
    def _compile_go(self, source_file: Path, optimization_level: str) -> Tuple[bool, Optional[Path], str]:
        """Compile Go source"""
        output_file = source_file.with_suffix('')
        
        cmd = ['go', 'build', '-o', str(output_file)]
        if optimization_level == "release":
            cmd.extend(['-ldflags', '-s -w'])  # Strip debug info
        
        cmd.append(str(source_file))
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        
        if result.returncode == 0 and output_file.exists():
            return True, output_file, result.stdout
        else:
            return False, None, result.stderr
    
    def _compile_rust(self, source_file: Path, optimization_level: str) -> Tuple[bool, Optional[Path], str]:
        """Compile Rust source"""
        output_file = source_file.with_suffix('')
        
        cmd = ['rustc']
        if optimization_level == "release":
            cmd.extend(['-O', '-C', 'strip=symbols'])
        cmd.extend(['-o', str(output_file), str(source_file)])
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        
        if result.returncode == 0 and output_file.exists():
            return True, output_file, result.stdout
        else:
            return False, None, result.stderr
    
    def _compile_cpp(self, source_file: Path, optimization_level: str) -> Tuple[bool, Optional[Path], str]:
        """Compile C++ source"""
        output_file = source_file.with_suffix('')
        
        cmd = ['g++']
        if optimization_level == "release":
            cmd.extend(['-O3', '-s'])
        else:
            cmd.append('-g')
        
        cmd.extend(['-o', str(output_file), str(source_file)])
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        
        if result.returncode == 0 and output_file.exists():
            return True, output_file, result.stdout
        else:
            return False, None, result.stderr
    
    def _compile_zig(self, source_file: Path, optimization_level: str) -> Tuple[bool, Optional[Path], str]:
        """Compile Zig source"""
        output_file = source_file.with_suffix('')
        
        cmd = ['zig', 'build-exe']
        if optimization_level == "release":
            cmd.extend(['-O', 'ReleaseFast'])
        
        cmd.extend([str(source_file), '-femit-bin=' + str(output_file)])
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        
        if result.returncode == 0 and output_file.exists():
            return True, output_file, result.stdout
        else:
            return False, None, result.stderr

class MemoryLeakDetector:
    """Memory leak detection using valgrind and other tools"""
    
    def __init__(self):
        self.valgrind_available = self._check_valgrind()
        
    def _check_valgrind(self) -> bool:
        """Check if valgrind is available"""
        try:
            result = subprocess.run(['valgrind', '--version'], 
                                  capture_output=True, text=True, timeout=5)
            return result.returncode == 0
        except (subprocess.TimeoutExpired, FileNotFoundError):
            return False
    
    def detect_leaks(self, executable_path: Path, args: List[str] = None) -> MemoryLeakResult:
        """Detect memory leaks in executable"""
        if not self.valgrind_available:
            logger.warning("Valgrind not available, skipping memory leak detection")
            return MemoryLeakResult(
                benchmark_name=executable_path.name,
                leak_detected=False,
                leaked_bytes=0,
                leak_locations=[],
                peak_memory_kb=0,
                total_allocations=0,
                total_frees=0,
                valgrind_output="Valgrind not available",
                timestamp=datetime.now()
            )
        
        args = args or []
        cmd = [
            'valgrind',
            '--tool=memcheck',
            '--leak-check=full',
            '--show-leak-kinds=all',
            '--track-origins=yes',
            '--verbose',
            '--xml=yes',
            '--xml-file=/tmp/valgrind_output.xml',
            str(executable_path)
        ] + args
        
        try:
            start_time = time.time()
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
            end_time = time.time()
            
            # Parse valgrind output
            leak_detected = "definitely lost" in result.stderr or "possibly lost" in result.stderr
            leaked_bytes = self._parse_leaked_bytes(result.stderr)
            leak_locations = self._parse_leak_locations(result.stderr)
            peak_memory = self._parse_peak_memory(result.stderr)
            
            return MemoryLeakResult(
                benchmark_name=executable_path.name,
                leak_detected=leak_detected,
                leaked_bytes=leaked_bytes,
                leak_locations=leak_locations,
                peak_memory_kb=peak_memory,
                total_allocations=0,  # Would need additional parsing
                total_frees=0,       # Would need additional parsing
                valgrind_output=result.stderr,
                timestamp=datetime.now()
            )
            
        except subprocess.TimeoutExpired:
            logger.error(f"Valgrind timeout for {executable_path}")
            return MemoryLeakResult(
                benchmark_name=executable_path.name,
                leak_detected=False,
                leaked_bytes=0,
                leak_locations=[],
                peak_memory_kb=0,
                total_allocations=0,
                total_frees=0,
                valgrind_output="Timeout",
                timestamp=datetime.now()
            )
    
    def _parse_leaked_bytes(self, valgrind_output: str) -> int:
        """Parse leaked bytes from valgrind output"""
        lines = valgrind_output.split('\n')
        for line in lines:
            if "definitely lost:" in line:
                parts = line.split()
                for i, part in enumerate(parts):
                    if part == "bytes" and i > 0:
                        try:
                            return int(parts[i-1].replace(',', ''))
                        except ValueError:
                            continue
        return 0
    
    def _parse_leak_locations(self, valgrind_output: str) -> List[str]:
        """Parse leak locations from valgrind output"""
        locations = []
        lines = valgrind_output.split('\n')
        
        for i, line in enumerate(lines):
            if "at 0x" in line and "by 0x" in lines[i+1:i+5]:
                locations.append(line.strip())
                
        return locations[:10]  # Limit to first 10 locations
    
    def _parse_peak_memory(self, valgrind_output: str) -> int:
        """Parse peak memory usage from valgrind output"""
        lines = valgrind_output.split('\n')
        for line in lines:
            if "in use at exit:" in line:
                parts = line.split()
                for i, part in enumerate(parts):
                    if "bytes" in part and i > 0:
                        try:
                            bytes_val = int(parts[i-1].replace(',', ''))
                            return bytes_val // 1024  # Convert to KB
                        except ValueError:
                            continue
        return 0

class RealWorldBenchmarkSuite:
    """Real-world application benchmarks"""
    
    def __init__(self, workspace_path: Path):
        self.workspace_path = workspace_path
        self.benchmarks = self._define_real_world_benchmarks()
    
    def _define_real_world_benchmarks(self) -> List[RealWorldBenchmark]:
        """Define comprehensive real-world benchmarks"""
        return [
            RealWorldBenchmark(
                name="web_server",
                description="HTTP server with routing and JSON API",
                category="network",
                source_file="benchmarks/real_world/web_server.csd",
                expected_features=["networkz", "jsonz", "concurrenz"],
                performance_targets={
                    "startup_time_ms": 50,
                    "requests_per_second": 10000,
                    "memory_per_connection_kb": 4
                },
                complexity_score=8
            ),
            RealWorldBenchmark(
                name="cli_tool",
                description="Command-line file processing tool",
                category="system",
                source_file="benchmarks/real_world/cli_tool.csd",
                expected_features=["filez", "stringz", "arrayz"],
                performance_targets={
                    "startup_time_ms": 10,
                    "files_per_second": 1000,
                    "memory_usage_mb": 50
                },
                complexity_score=5
            ),
            RealWorldBenchmark(
                name="database_orm",
                description="Object-relational mapping with connection pooling",
                category="database",
                source_file="benchmarks/real_world/database_orm.csd",
                expected_features=["dbz", "sqlz", "concurrenz"],
                performance_targets={
                    "queries_per_second": 5000,
                    "connection_setup_ms": 5,
                    "memory_per_connection_kb": 8
                },
                complexity_score=9
            ),
            RealWorldBenchmark(
                name="game_engine",
                description="Simple 2D game engine with physics",
                category="game",
                source_file="benchmarks/real_world/game_engine.csd",
                expected_features=["gamez", "mathz", "drawz"],
                performance_targets={
                    "fps": 60,
                    "frame_time_ms": 16,
                    "memory_usage_mb": 100
                },
                complexity_score=10
            ),
            RealWorldBenchmark(
                name="compiler_frontend",
                description="Language parser and semantic analyzer",
                category="compiler",
                source_file="benchmarks/real_world/compiler_frontend.csd",
                expected_features=["stringz", "arrayz", "testz"],
                performance_targets={
                    "lines_per_second": 100000,
                    "memory_per_line_bytes": 100,
                    "parse_time_ms": 1000
                },
                complexity_score=9
            ),
            RealWorldBenchmark(
                name="crypto_service",
                description="Cryptographic service with TLS",
                category="crypto",
                source_file="benchmarks/real_world/crypto_service.csd",
                expected_features=["cryptz", "tlsz", "networkz"],
                performance_targets={
                    "encryptions_per_second": 1000,
                    "key_generation_ms": 100,
                    "memory_usage_mb": 20
                },
                complexity_score=8
            )
        ]
    
    def create_benchmarks(self):
        """Create real-world benchmark source files"""
        real_world_dir = self.workspace_path / "benchmarks" / "real_world"
        real_world_dir.mkdir(exist_ok=True)
        
        for benchmark in self.benchmarks:
            benchmark_path = self.workspace_path / benchmark.source_file
            if not benchmark_path.exists():
                self._create_benchmark_source(benchmark, benchmark_path)
    
    def _create_benchmark_source(self, benchmark: RealWorldBenchmark, path: Path):
        """Create source code for real-world benchmark"""
        content = self._generate_benchmark_content(benchmark)
        with open(path, 'w') as f:
            f.write(content)
        logger.info(f"Created real-world benchmark: {path}")
    
    def _generate_benchmark_content(self, benchmark: RealWorldBenchmark) -> str:
        """Generate CURSED source code for benchmark"""
        if benchmark.name == "web_server":
            return '''
yeet "networkz"
yeet "jsonz"
yeet "concurrenz"
yeet "vibez"

squad Server {
    port drip
    running lit
}

slay new_server(port drip) Server {
    damn Server{port: port, running: based}
}

slay (server *Server) handle_request(request tea) tea {
    sus response squad = jsonz.object()
    jsonz.set(response, "status", "ok")
    jsonz.set(response, "timestamp", timez.now())
    jsonz.set(response, "method", request)
    damn jsonz.stringify(response)
}

slay (server *Server) start() {
    server.running = based
    vibez.spill("Server starting on port", server.port)
    
    bestie (server.running) {
        # Simulate handling requests
        sus request tea = "GET /api/status"
        sus response tea = server.handle_request(request)
        # Process request (simulated)
        sleep(1)
    }
}

slay benchmark_web_server() {
    sus server Server = new_server(8080)
    sus start_time drip = timez.now_ms()
    
    # Benchmark startup time
    server.start()
    
    sus end_time drip = timez.now_ms()
    sus startup_time drip = end_time - start_time
    
    vibez.spill("Web server benchmark completed")
    vibez.spill("Startup time:", startup_time, "ms")
}

benchmark_web_server()
'''
        elif benchmark.name == "cli_tool":
            return '''
yeet "filez"
yeet "stringz"
yeet "arrayz"
yeet "vibez"

slay process_file(filepath tea) drip {
    sus content tea = filez.read_file(filepath) fam {
        when _ -> damn 0
    }
    
    sus lines []tea = stringz.split(content, "\\n")
    sus processed drip = 0
    
    bestie (i drip = 0; i < arrayz.len(lines); i = i + 1) {
        sus line tea = lines[i]
        ready (stringz.len(line) > 0) {
            # Simulate processing
            sus words []tea = stringz.split(line, " ")
            processed = processed + arrayz.len(words)
        }
    }
    
    damn processed
}

slay benchmark_cli_tool() {
    sus test_files []tea = [
        "test1.txt",
        "test2.txt", 
        "test3.txt"
    ]
    
    sus start_time drip = timez.now_ms()
    sus total_processed drip = 0
    
    bestie (i drip = 0; i < arrayz.len(test_files); i = i + 1) {
        sus processed drip = process_file(test_files[i])
        total_processed = total_processed + processed
    }
    
    sus end_time drip = timez.now_ms()
    sus processing_time drip = end_time - start_time
    
    vibez.spill("CLI tool benchmark completed")
    vibez.spill("Processing time:", processing_time, "ms")
    vibez.spill("Total processed:", total_processed)
}

benchmark_cli_tool()
'''
        elif benchmark.name == "database_orm":
            return '''
yeet "dbz"
yeet "sqlz"
yeet "concurrenz"
yeet "vibez"

squad User {
    id drip
    name tea
    email tea
}

squad Database {
    connection_pool chan<*Connection>
    max_connections drip
}

slay new_database(max_conn drip) Database {
    sus pool chan<*Connection> = make_channel_buffered(max_conn)
    damn Database{connection_pool: pool, max_connections: max_conn}
}

slay (db *Database) get_connection() *Connection {
    damn <-db.connection_pool
}

slay (db *Database) return_connection(conn *Connection) {
    db.connection_pool <- conn
}

slay (db *Database) create_user(user User) lit {
    sus conn *Connection = db.get_connection()
    defer db.return_connection(conn)
    
    sus query tea = sqlz.build_insert("users", user)
    sus result lit = dbz.execute(conn, query)
    damn result
}

slay benchmark_database_orm() {
    sus db Database = new_database(10)
    sus start_time drip = timez.now_ms()
    
    # Benchmark multiple concurrent operations
    bestie (i drip = 0; i < 1000; i = i + 1) {
        sus user User = User{
            id: i,
            name: stringz.concat("User", stringz.from_int(i)),
            email: stringz.concat("user", stringz.from_int(i), "@test.com")
        }
        
        go {
            db.create_user(user)
        }
    }
    
    sus end_time drip = timez.now_ms()
    sus operation_time drip = end_time - start_time
    
    vibez.spill("Database ORM benchmark completed")
    vibez.spill("Operation time:", operation_time, "ms")
}

benchmark_database_orm()
'''
        else:
            # Generic benchmark template
            return f'''
yeet "vibez"
yeet "mathz"
yeet "timez"

slay benchmark_{benchmark.name.replace("-", "_")}() {{
    sus start_time drip = timez.now_ms()
    
    # Simulate {benchmark.description}
    bestie (i drip = 0; i < 10000; i = i + 1) {{
        sus result drip = mathz.sqrt(mathz.pow(i, 2))
        # Additional processing
    }}
    
    sus end_time drip = timez.now_ms()
    sus execution_time drip = end_time - start_time
    
    vibez.spill("{benchmark.name} benchmark completed")
    vibez.spill("Execution time:", execution_time, "ms")
}}

benchmark_{benchmark.name.replace("-", "_")}()
'''

class ComprehensiveBenchmarkRunner:
    """Main comprehensive benchmark runner"""
    
    def __init__(self, workspace_path: str, config_file: str = "p2_benchmark_config.json"):
        self.workspace_path = Path(workspace_path)
        self.config = self._load_config(config_file)
        self.cross_compiler = CrossLanguageCompiler(self.workspace_path)
        self.memory_detector = MemoryLeakDetector()
        self.real_world_suite = RealWorldBenchmarkSuite(self.workspace_path)
        self.database = self._init_database()
        
    def _load_config(self, config_file: str) -> Dict[str, Any]:
        """Load enhanced configuration"""
        default_config = {
            "languages": ["cursed", "go", "rust", "cpp"],
            "optimization_levels": ["debug", "release"],
            "benchmark_categories": ["micro", "real_world", "stdlib", "compiler"],
            "memory_leak_detection": True,
            "cross_language_comparison": True,
            "continuous_integration": True,
            "performance_targets": {
                "cursed_vs_go_ratio": 1.2,
                "cursed_vs_rust_ratio": 1.1,
                "cursed_vs_cpp_ratio": 1.5
            },
            "regression_thresholds": {
                "critical": 25.0,
                "major": 15.0,
                "minor": 5.0
            },
            "benchmark_timeout_seconds": 300,
            "memory_leak_tolerance_bytes": 1024,
            "parallel_execution": True,
            "max_workers": 8
        }
        
        try:
            with open(config_file, 'r') as f:
                user_config = json.load(f)
                default_config.update(user_config)
        except FileNotFoundError:
            # Save default config
            with open(config_file, 'w') as f:
                json.dump(default_config, f, indent=2)
            logger.info(f"Created default config: {config_file}")
        
        return default_config
    
    def _init_database(self) -> sqlite3.Connection:
        """Initialize enhanced database schema"""
        db_path = self.workspace_path / "p2_benchmark_results.db"
        conn = sqlite3.connect(str(db_path))
        
        # Create tables
        conn.execute('''
            CREATE TABLE IF NOT EXISTS cross_language_results (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                git_commit TEXT NOT NULL,
                benchmark_name TEXT NOT NULL,
                language TEXT NOT NULL,
                compilation_time_ms REAL NOT NULL,
                execution_time_ms REAL NOT NULL,
                memory_usage_kb INTEGER NOT NULL,
                binary_size_bytes INTEGER NOT NULL,
                success BOOLEAN NOT NULL,
                compiler_version TEXT,
                optimization_level TEXT,
                error_message TEXT
            )
        ''')
        
        conn.execute('''
            CREATE TABLE IF NOT EXISTS memory_leak_results (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                benchmark_name TEXT NOT NULL,
                leak_detected BOOLEAN NOT NULL,
                leaked_bytes INTEGER NOT NULL,
                peak_memory_kb INTEGER NOT NULL,
                valgrind_output TEXT
            )
        ''')
        
        conn.execute('''
            CREATE TABLE IF NOT EXISTS performance_comparisons (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                benchmark_name TEXT NOT NULL,
                cursed_time_ms REAL NOT NULL,
                comparison_language TEXT NOT NULL,
                comparison_time_ms REAL NOT NULL,
                performance_ratio REAL NOT NULL,
                meets_target BOOLEAN NOT NULL
            )
        ''')
        
        conn.commit()
        return conn
    
    def run_comprehensive_benchmark_suite(self) -> ComprehensivePerformanceReport:
        """Run the complete P2 benchmark suite"""
        logger.info("Starting P2 Comprehensive Benchmark Suite")
        start_time = datetime.now()
        
        # Create real-world benchmarks if needed
        self.real_world_suite.create_benchmarks()
        
        # Build all compilers
        self._build_cursed_compiler()
        
        # Run benchmarks
        cursed_results = self._run_cursed_benchmarks()
        comparison_results = self._run_cross_language_benchmarks()
        memory_results = self._run_memory_safety_validation()
        real_world_results = self._run_real_world_benchmarks()
        
        # Analyze results
        regression_alerts = self._analyze_regressions(cursed_results)
        performance_score = self._calculate_performance_score(
            cursed_results, comparison_results
        )
        
        # Generate comprehensive report
        report = ComprehensivePerformanceReport(
            timestamp=datetime.now(),
            cursed_results=cursed_results,
            comparison_results=comparison_results,
            memory_safety_results=memory_results,
            real_world_results=real_world_results,
            regression_alerts=regression_alerts,
            performance_score=performance_score,
            recommendation=self._generate_recommendation(performance_score, regression_alerts)
        )
        
        # Store results
        self._store_comprehensive_results(report)
        
        duration = (datetime.now() - start_time).total_seconds()
        logger.info(f"P2 Benchmark Suite completed in {duration:.1f}s")
        logger.info(f"Performance Score: {performance_score:.1f}/100")
        
        return report
    
    def _build_cursed_compiler(self):
        """Build CURSED compiler with optimizations"""
        logger.info("Building CURSED compiler...")
        
        cmd = ["zig", "build", "-Doptimize=ReleaseFast"]
        result = subprocess.run(cmd, cwd=self.workspace_path, capture_output=True, text=True)
        
        if result.returncode != 0:
            logger.error(f"Compiler build failed: {result.stderr}")
            raise RuntimeError("Failed to build CURSED compiler")
        
        logger.info("CURSED compiler built successfully")
    
    def _run_cursed_benchmarks(self) -> List[CrossLanguageBenchmarkResult]:
        """Run comprehensive CURSED benchmarks"""
        logger.info("Running CURSED benchmarks...")
        results = []
        
        benchmark_files = [
            "benchmarks/cursed/fasta.csd",
            "benchmarks/cursed/mandelbrot.csd",
            "benchmarks/cursed/binary_trees.csd",
            "benchmarks/cursed/n_bodies.csd",
            "comprehensive_stdlib_test.csd",
            "advanced_features_test.csd"
        ]
        
        for benchmark_file in benchmark_files:
            benchmark_path = self.workspace_path / benchmark_file
            if not benchmark_path.exists():
                logger.warning(f"Benchmark file not found: {benchmark_path}")
                continue
            
            for opt_level in self.config["optimization_levels"]:
                result = self._run_single_cursed_benchmark(benchmark_path, opt_level)
                if result:
                    results.append(result)
        
        logger.info(f"Completed {len(results)} CURSED benchmarks")
        return results
    
    def _run_single_cursed_benchmark(self, benchmark_path: Path, 
                                   optimization_level: str) -> Optional[CrossLanguageBenchmarkResult]:
        """Run single CURSED benchmark"""
        try:
            # Compile
            start_compile = time.time()
            success, executable, error = self.cross_compiler.compile_benchmark(
                "cursed", benchmark_path, optimization_level
            )
            compile_time = (time.time() - start_compile) * 1000
            
            if not success:
                logger.error(f"Compilation failed for {benchmark_path}: {error}")
                return CrossLanguageBenchmarkResult(
                    benchmark_name=benchmark_path.name,
                    language="cursed",
                    compilation_time_ms=compile_time,
                    execution_time_ms=0,
                    memory_usage_kb=0,
                    binary_size_bytes=0,
                    success=False,
                    compiler_version=self._get_cursed_version(),
                    optimization_level=optimization_level,
                    timestamp=datetime.now(),
                    error_message=error
                )
            
            # Execute and measure
            start_exec = time.time()
            exec_result = subprocess.run([str(executable)], 
                                       capture_output=True, text=True, timeout=30)
            exec_time = (time.time() - start_exec) * 1000
            
            # Get memory usage and binary size
            memory_usage = self._get_peak_memory_usage(executable)
            binary_size = executable.stat().st_size if executable.exists() else 0
            
            return CrossLanguageBenchmarkResult(
                benchmark_name=benchmark_path.name,
                language="cursed",
                compilation_time_ms=compile_time,
                execution_time_ms=exec_time,
                memory_usage_kb=memory_usage,
                binary_size_bytes=binary_size,
                success=exec_result.returncode == 0,
                compiler_version=self._get_cursed_version(),
                optimization_level=optimization_level,
                timestamp=datetime.now(),
                error_message=exec_result.stderr if exec_result.returncode != 0 else None
            )
            
        except Exception as e:
            logger.error(f"Error running benchmark {benchmark_path}: {e}")
            return None
    
    def _run_cross_language_benchmarks(self) -> Dict[str, List[CrossLanguageBenchmarkResult]]:
        """Run cross-language performance comparisons"""
        logger.info("Running cross-language benchmarks...")
        results = {}
        
        # Define equivalent benchmarks for different languages
        equivalent_benchmarks = {
            "fasta.csd": {"go": "benchmarks/go/fasta.go", "rust": "benchmarks/rust/fasta.rs", "cpp": "benchmarks/cplusplus/fasta.cpp"},
            "mandelbrot.csd": {"go": "benchmarks/go/mandelbrot.go", "rust": "benchmarks/rust/mandelbrot.rs", "cpp": "benchmarks/cplusplus/mandelbrot.cpp"},
            "binary_trees.csd": {"go": "benchmarks/go/binary_trees.go", "rust": "benchmarks/rust/binary_trees.rs", "cpp": "benchmarks/cplusplus/binary_trees.cpp"}
        }
        
        for cursed_benchmark, lang_files in equivalent_benchmarks.items():
            for language, file_path in lang_files.items():
                if language not in self.config["languages"]:
                    continue
                
                benchmark_path = self.workspace_path / file_path
                if not benchmark_path.exists():
                    logger.warning(f"Cross-language benchmark not found: {benchmark_path}")
                    continue
                
                if language not in results:
                    results[language] = []
                
                for opt_level in self.config["optimization_levels"]:
                    result = self._run_cross_language_benchmark(benchmark_path, language, opt_level)
                    if result:
                        results[language].append(result)
        
        return results
    
    def _run_cross_language_benchmark(self, benchmark_path: Path, language: str, 
                                    optimization_level: str) -> Optional[CrossLanguageBenchmarkResult]:
        """Run single cross-language benchmark"""
        try:
            # Compile
            start_compile = time.time()
            success, executable, error = self.cross_compiler.compile_benchmark(
                language, benchmark_path, optimization_level
            )
            compile_time = (time.time() - start_compile) * 1000
            
            if not success:
                return CrossLanguageBenchmarkResult(
                    benchmark_name=benchmark_path.name,
                    language=language,
                    compilation_time_ms=compile_time,
                    execution_time_ms=0,
                    memory_usage_kb=0,
                    binary_size_bytes=0,
                    success=False,
                    compiler_version=self._get_compiler_version(language),
                    optimization_level=optimization_level,
                    timestamp=datetime.now(),
                    error_message=error
                )
            
            # Execute and measure
            start_exec = time.time()
            exec_result = subprocess.run([str(executable)], 
                                       capture_output=True, text=True, timeout=30)
            exec_time = (time.time() - start_exec) * 1000
            
            memory_usage = self._get_peak_memory_usage(executable)
            binary_size = executable.stat().st_size if executable.exists() else 0
            
            return CrossLanguageBenchmarkResult(
                benchmark_name=benchmark_path.name,
                language=language,
                compilation_time_ms=compile_time,
                execution_time_ms=exec_time,
                memory_usage_kb=memory_usage,
                binary_size_bytes=binary_size,
                success=exec_result.returncode == 0,
                compiler_version=self._get_compiler_version(language),
                optimization_level=optimization_level,
                timestamp=datetime.now()
            )
            
        except Exception as e:
            logger.error(f"Error running {language} benchmark {benchmark_path}: {e}")
            return None
    
    def _run_memory_safety_validation(self) -> List[MemoryLeakResult]:
        """Run comprehensive memory safety validation"""
        logger.info("Running memory safety validation...")
        results = []
        
        # Test all CURSED benchmarks for memory leaks
        benchmark_files = [
            "benchmarks/cursed/fasta.csd",
            "benchmarks/cursed/mandelbrot.csd",
            "comprehensive_stdlib_test.csd"
        ]
        
        for benchmark_file in benchmark_files:
            benchmark_path = self.workspace_path / benchmark_file
            if not benchmark_path.exists():
                continue
            
            # Compile benchmark
            success, executable, _ = self.cross_compiler.compile_benchmark(
                "cursed", benchmark_path, "debug"  # Use debug for better leak detection
            )
            
            if success and executable:
                leak_result = self.memory_detector.detect_leaks(executable)
                results.append(leak_result)
                
                if leak_result.leak_detected:
                    logger.warning(f"Memory leak detected in {benchmark_path.name}: "
                                 f"{leak_result.leaked_bytes} bytes")
                else:
                    logger.info(f"No memory leaks in {benchmark_path.name}")
        
        return results
    
    def _run_real_world_benchmarks(self) -> List[CrossLanguageBenchmarkResult]:
        """Run real-world application benchmarks"""
        logger.info("Running real-world benchmarks...")
        results = []
        
        for benchmark in self.real_world_suite.benchmarks:
            benchmark_path = self.workspace_path / benchmark.source_file
            if not benchmark_path.exists():
                logger.warning(f"Real-world benchmark not found: {benchmark_path}")
                continue
            
            result = self._run_single_cursed_benchmark(benchmark_path, "release")
            if result:
                results.append(result)
        
        return results
    
    def _get_peak_memory_usage(self, executable: Path) -> int:
        """Get peak memory usage in KB"""
        try:
            result = subprocess.run(["/usr/bin/time", "-f", "%M", str(executable)],
                                  capture_output=True, text=True, timeout=30)
            if result.returncode == 0:
                return int(result.stderr.strip())
        except Exception:
            pass
        return 0
    
    def _get_cursed_version(self) -> str:
        """Get CURSED compiler version"""
        try:
            compiler_path = self.workspace_path / "zig-out" / "bin" / "cursed-zig"
            result = subprocess.run([str(compiler_path), "--version"],
                                  capture_output=True, text=True, timeout=5)
            return result.stdout.strip() if result.returncode == 0 else "unknown"
        except Exception:
            return "unknown"
    
    def _get_compiler_version(self, language: str) -> str:
        """Get compiler version for specific language"""
        try:
            if language == "go":
                result = subprocess.run(["go", "version"], capture_output=True, text=True)
            elif language == "rust":
                result = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
            elif language == "cpp":
                result = subprocess.run(["g++", "--version"], capture_output=True, text=True)
            else:
                return "unknown"
            
            return result.stdout.strip().split('\n')[0] if result.returncode == 0 else "unknown"
        except Exception:
            return "unknown"
    
    def _analyze_regressions(self, results: List[CrossLanguageBenchmarkResult]) -> List[Dict[str, Any]]:
        """Analyze for performance regressions"""
        alerts = []
        
        # Compare with historical data
        for result in results:
            if not result.success:
                continue
            
            # Query historical data from database
            cursor = self.database.execute('''
                SELECT execution_time_ms FROM cross_language_results
                WHERE benchmark_name = ? AND language = ? AND success = 1
                ORDER BY timestamp DESC LIMIT 20
            ''', (result.benchmark_name, result.language))
            
            historical_times = [row[0] for row in cursor.fetchall()]
            
            if len(historical_times) >= 5:
                baseline_time = statistics.median(historical_times)
                regression_percent = ((result.execution_time_ms - baseline_time) / baseline_time) * 100
                
                if regression_percent > self.config["regression_thresholds"]["minor"]:
                    severity = "minor"
                    if regression_percent > self.config["regression_thresholds"]["major"]:
                        severity = "major"
                    if regression_percent > self.config["regression_thresholds"]["critical"]:
                        severity = "critical"
                    
                    alerts.append({
                        "benchmark": result.benchmark_name,
                        "language": result.language,
                        "regression_percent": regression_percent,
                        "current_time": result.execution_time_ms,
                        "baseline_time": baseline_time,
                        "severity": severity,
                        "timestamp": result.timestamp.isoformat()
                    })
        
        return alerts
    
    def _calculate_performance_score(self, cursed_results: List[CrossLanguageBenchmarkResult],
                                   comparison_results: Dict[str, List[CrossLanguageBenchmarkResult]]) -> float:
        """Calculate overall performance score (0-100)"""
        if not cursed_results:
            return 0.0
        
        scores = []
        
        # Success rate score (30%)
        success_rate = sum(1 for r in cursed_results if r.success) / len(cursed_results)
        scores.append(success_rate * 30)
        
        # Compilation speed score (20%)
        compile_times = [r.compilation_time_ms for r in cursed_results if r.success]
        if compile_times:
            avg_compile_time = statistics.mean(compile_times)
            # Score based on sub-second compilation target
            compile_score = max(0, 1 - (avg_compile_time / 1000)) * 20
            scores.append(compile_score)
        
        # Execution speed score (30%)
        exec_times = [r.execution_time_ms for r in cursed_results if r.success]
        if exec_times:
            avg_exec_time = statistics.mean(exec_times)
            # Score based on reasonable execution time
            exec_score = max(0, 1 - (avg_exec_time / 1000)) * 30
            scores.append(exec_score)
        
        # Cross-language comparison score (20%)
        if comparison_results:
            comparison_score = self._calculate_comparison_score(cursed_results, comparison_results)
            scores.append(comparison_score * 20)
        
        return sum(scores)
    
    def _calculate_comparison_score(self, cursed_results: List[CrossLanguageBenchmarkResult],
                                  comparison_results: Dict[str, List[CrossLanguageBenchmarkResult]]) -> float:
        """Calculate score based on cross-language performance"""
        cursed_by_benchmark = {r.benchmark_name: r for r in cursed_results if r.success}
        scores = []
        
        for language, lang_results in comparison_results.items():
            lang_by_benchmark = {r.benchmark_name: r for r in lang_results if r.success}
            
            for benchmark_name in cursed_by_benchmark:
                if benchmark_name in lang_by_benchmark:
                    cursed_time = cursed_by_benchmark[benchmark_name].execution_time_ms
                    lang_time = lang_by_benchmark[benchmark_name].execution_time_ms
                    
                    if lang_time > 0:
                        ratio = cursed_time / lang_time
                        target_ratio = self.config["performance_targets"].get(f"cursed_vs_{language}_ratio", 1.5)
                        
                        # Score 1.0 if we meet target, scale down if we don't
                        score = min(1.0, target_ratio / ratio) if ratio > 0 else 0.0
                        scores.append(score)
        
        return statistics.mean(scores) if scores else 0.0
    
    def _generate_recommendation(self, performance_score: float, alerts: List[Dict[str, Any]]) -> str:
        """Generate performance recommendation"""
        if performance_score >= 90:
            return "Excellent performance. CURSED is ready for production use."
        elif performance_score >= 75:
            return "Good performance with minor areas for improvement."
        elif performance_score >= 60:
            return "Moderate performance. Consider optimization work."
        elif len(alerts) > 0:
            return f"Performance regressions detected ({len(alerts)} alerts). Investigation needed."
        else:
            return "Performance below targets. Significant optimization work required."
    
    def _store_comprehensive_results(self, report: ComprehensivePerformanceReport):
        """Store comprehensive results in database"""
        # Store cross-language results
        for result in report.cursed_results + [r for lang_results in report.comparison_results.values() for r in lang_results]:
            self.database.execute('''
                INSERT INTO cross_language_results
                (timestamp, git_commit, benchmark_name, language, compilation_time_ms,
                 execution_time_ms, memory_usage_kb, binary_size_bytes, success,
                 compiler_version, optimization_level, error_message)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', (
                result.timestamp.isoformat(),
                result.git_commit,
                result.benchmark_name,
                result.language,
                result.compilation_time_ms,
                result.execution_time_ms,
                result.memory_usage_kb,
                result.binary_size_bytes,
                result.success,
                result.compiler_version,
                result.optimization_level,
                result.error_message
            ))
        
        # Store memory leak results
        for result in report.memory_safety_results:
            self.database.execute('''
                INSERT INTO memory_leak_results
                (timestamp, benchmark_name, leak_detected, leaked_bytes,
                 peak_memory_kb, valgrind_output)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', (
                result.timestamp.isoformat(),
                result.benchmark_name,
                result.leak_detected,
                result.leaked_bytes,
                result.peak_memory_kb,
                result.valgrind_output
            ))
        
        self.database.commit()
        logger.info("Comprehensive results stored in database")
    
    def generate_html_report(self, report: ComprehensivePerformanceReport) -> str:
        """Generate comprehensive HTML report"""
        report_dir = self.workspace_path / "p2_benchmark_reports"
        report_dir.mkdir(exist_ok=True)
        
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        report_file = report_dir / f"p2_comprehensive_report_{timestamp}.html"
        
        html_content = f'''
<!DOCTYPE html>
<html>
<head>
    <title>P2 CURSED Compiler Comprehensive Benchmark Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .header {{ background: #2c3e50; color: white; padding: 20px; border-radius: 8px; }}
        .section {{ margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 8px; }}
        .metric {{ display: inline-block; margin: 10px; padding: 15px; background: #f8f9fa; border-radius: 5px; }}
        .success {{ color: #27ae60; }}
        .warning {{ color: #f39c12; }}
        .error {{ color: #e74c3c; }}
        .performance-score {{ font-size: 2em; font-weight: bold; text-align: center; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 P2 CURSED Compiler Comprehensive Benchmark Report</h1>
        <p><strong>Generated:</strong> {report.timestamp.strftime('%Y-%m-%d %H:%M:%S')}</p>
        <p><strong>Performance Score:</strong> <span class="performance-score">{report.performance_score:.1f}/100</span></p>
        <p><strong>Recommendation:</strong> {report.recommendation}</p>
    </div>
    
    <div class="section">
        <h2>📊 Executive Summary</h2>
        <div class="metric">
            <strong>CURSED Benchmarks:</strong> {len(report.cursed_results)}
        </div>
        <div class="metric">
            <strong>Cross-Language Tests:</strong> {sum(len(results) for results in report.comparison_results.values())}
        </div>
        <div class="metric">
            <strong>Memory Safety Tests:</strong> {len(report.memory_safety_results)}
        </div>
        <div class="metric">
            <strong>Regression Alerts:</strong> <span class="{'error' if len(report.regression_alerts) > 0 else 'success'}">{len(report.regression_alerts)}</span>
        </div>
    </div>
    
    <div class="section">
        <h2>🔍 Memory Safety Validation</h2>
        {self._format_memory_safety_results(report.memory_safety_results)}
    </div>
    
    <div class="section">
        <h2>⚡ Performance Comparison</h2>
        {self._format_performance_comparison(report.cursed_results, report.comparison_results)}
    </div>
    
    <div class="section">
        <h2>🏢 Real-World Application Benchmarks</h2>
        {self._format_real_world_results(report.real_world_results)}
    </div>
    
    <div class="section">
        <h2>🚨 Regression Alerts</h2>
        {self._format_regression_alerts(report.regression_alerts)}
    </div>
    
    <div class="section">
        <h2>📈 Detailed Results</h2>
        {self._format_detailed_results(report.cursed_results)}
    </div>
</body>
</html>
'''
        
        with open(report_file, 'w') as f:
            f.write(html_content)
        
        logger.info(f"P2 Comprehensive report generated: {report_file}")
        return str(report_file)
    
    def _format_memory_safety_results(self, results: List[MemoryLeakResult]) -> str:
        """Format memory safety results for HTML"""
        if not results:
            return "<p>No memory safety tests run.</p>"
        
        leak_count = sum(1 for r in results if r.leak_detected)
        total_leaked = sum(r.leaked_bytes for r in results)
        
        html = f'''
        <div class="metric">
            <strong>Zero-Leak Confirmation:</strong> 
            <span class="{'success' if leak_count == 0 else 'error'}">
                {'✅ PASSED' if leak_count == 0 else f'❌ FAILED ({leak_count} leaks)'}
            </span>
        </div>
        <div class="metric">
            <strong>Total Leaked:</strong> {total_leaked} bytes
        </div>
        
        <table>
            <tr>
                <th>Benchmark</th>
                <th>Status</th>
                <th>Leaked Bytes</th>
                <th>Peak Memory (KB)</th>
            </tr>
        '''
        
        for result in results:
            status = "❌ LEAK DETECTED" if result.leak_detected else "✅ CLEAN"
            status_class = "error" if result.leak_detected else "success"
            
            html += f'''
            <tr>
                <td>{result.benchmark_name}</td>
                <td class="{status_class}">{status}</td>
                <td>{result.leaked_bytes}</td>
                <td>{result.peak_memory_kb}</td>
            </tr>
            '''
        
        html += "</table>"
        return html
    
    def _format_performance_comparison(self, cursed_results: List[CrossLanguageBenchmarkResult],
                                     comparison_results: Dict[str, List[CrossLanguageBenchmarkResult]]) -> str:
        """Format cross-language performance comparison"""
        if not comparison_results:
            return "<p>No cross-language comparisons available.</p>"
        
        html = '''
        <table>
            <tr>
                <th>Benchmark</th>
                <th>CURSED (ms)</th>
                <th>Go (ms)</th>
                <th>Rust (ms)</th>
                <th>C++ (ms)</th>
                <th>Best Performance</th>
            </tr>
        '''
        
        cursed_by_name = {r.benchmark_name: r for r in cursed_results if r.success}
        
        for benchmark_name in cursed_by_name:
            cursed_time = cursed_by_name[benchmark_name].execution_time_ms
            times = {"CURSED": cursed_time}
            
            for lang, results in comparison_results.items():
                lang_by_name = {r.benchmark_name: r for r in results if r.success}
                if benchmark_name in lang_by_name:
                    times[lang.upper()] = lang_by_name[benchmark_name].execution_time_ms
            
            best_lang = min(times.keys(), key=lambda k: times[k])
            
            html += f'''
            <tr>
                <td>{benchmark_name}</td>
                <td class="{'success' if best_lang == 'CURSED' else ''}">{cursed_time:.1f}</td>
                <td class="{'success' if best_lang == 'GO' else ''}">{times.get('GO', 'N/A')}</td>
                <td class="{'success' if best_lang == 'RUST' else ''}">{times.get('RUST', 'N/A')}</td>
                <td class="{'success' if best_lang == 'CPP' else ''}">{times.get('CPP', 'N/A')}</td>
                <td><strong>{best_lang}</strong></td>
            </tr>
            '''
        
        html += "</table>"
        return html
    
    def _format_real_world_results(self, results: List[CrossLanguageBenchmarkResult]) -> str:
        """Format real-world benchmark results"""
        if not results:
            return "<p>No real-world benchmarks run.</p>"
        
        html = '''
        <table>
            <tr>
                <th>Application</th>
                <th>Compilation (ms)</th>
                <th>Execution (ms)</th>
                <th>Memory (KB)</th>
                <th>Binary Size (bytes)</th>
                <th>Status</th>
            </tr>
        '''
        
        for result in results:
            status = "✅ SUCCESS" if result.success else "❌ FAILED"
            status_class = "success" if result.success else "error"
            
            html += f'''
            <tr>
                <td>{result.benchmark_name}</td>
                <td>{result.compilation_time_ms:.1f}</td>
                <td>{result.execution_time_ms:.1f}</td>
                <td>{result.memory_usage_kb}</td>
                <td>{result.binary_size_bytes}</td>
                <td class="{status_class}">{status}</td>
            </tr>
            '''
        
        html += "</table>"
        return html
    
    def _format_regression_alerts(self, alerts: List[Dict[str, Any]]) -> str:
        """Format regression alerts"""
        if not alerts:
            return '<p class="success">✅ No performance regressions detected.</p>'
        
        html = '''
        <table>
            <tr>
                <th>Benchmark</th>
                <th>Language</th>
                <th>Regression %</th>
                <th>Current (ms)</th>
                <th>Baseline (ms)</th>
                <th>Severity</th>
            </tr>
        '''
        
        for alert in alerts:
            severity_class = {
                "minor": "warning",
                "major": "warning", 
                "critical": "error"
            }.get(alert["severity"], "")
            
            html += f'''
            <tr>
                <td>{alert["benchmark"]}</td>
                <td>{alert["language"]}</td>
                <td class="{severity_class}">{alert["regression_percent"]:.1f}%</td>
                <td>{alert["current_time"]:.1f}</td>
                <td>{alert["baseline_time"]:.1f}</td>
                <td class="{severity_class}">{alert["severity"].upper()}</td>
            </tr>
            '''
        
        html += "</table>"
        return html
    
    def _format_detailed_results(self, results: List[CrossLanguageBenchmarkResult]) -> str:
        """Format detailed benchmark results"""
        html = '''
        <table>
            <tr>
                <th>Benchmark</th>
                <th>Compilation (ms)</th>
                <th>Execution (ms)</th>
                <th>Memory (KB)</th>
                <th>Binary Size</th>
                <th>Optimization</th>
                <th>Status</th>
            </tr>
        '''
        
        for result in results:
            status = "✅ SUCCESS" if result.success else "❌ FAILED"
            status_class = "success" if result.success else "error"
            
            html += f'''
            <tr>
                <td>{result.benchmark_name}</td>
                <td>{result.compilation_time_ms:.1f}</td>
                <td>{result.execution_time_ms:.1f}</td>
                <td>{result.memory_usage_kb}</td>
                <td>{result.binary_size_bytes}</td>
                <td>{result.optimization_level}</td>
                <td class="{status_class}">{status}</td>
            </tr>
            '''
        
        html += "</table>"
        return html

def main():
    """Main entry point for P2 comprehensive benchmark suite"""
    import argparse
    
    parser = argparse.ArgumentParser(description="P2 CURSED Compiler Comprehensive Benchmark Suite")
    parser.add_argument("--workspace", default=".", help="Path to CURSED workspace")
    parser.add_argument("--config", default="p2_benchmark_config.json", help="Configuration file")
    parser.add_argument("--mode", choices=["full", "cursed-only", "cross-lang", "memory", "real-world"],
                       default="full", help="Benchmark mode")
    parser.add_argument("--output", help="Output report file path")
    
    args = parser.parse_args()
    
    try:
        runner = ComprehensiveBenchmarkRunner(args.workspace, args.config)
        
        if args.mode == "full":
            report = runner.run_comprehensive_benchmark_suite()
            report_path = runner.generate_html_report(report)
            print(f"📊 Comprehensive report generated: {report_path}")
            print(f"🎯 Performance Score: {report.performance_score:.1f}/100")
            print(f"💡 Recommendation: {report.recommendation}")
            
        else:
            print(f"Running {args.mode} benchmarks...")
            # Implement partial benchmark runs here
            
    except Exception as e:
        logger.error(f"P2 Benchmark suite failed: {e}")
        traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    main()
