#!/usr/bin/env python3
"""
CURSED Compiler Continuous Benchmark Harness System

A comprehensive benchmarking system that provides:
- Continuous benchmark execution and monitoring
- Statistical analysis and regression detection
- Performance trend analysis over time
- Automated reporting and alerting
- Historical data storage and comparison
"""

import os
import sys
import json
import time
import sqlite3
import statistics
import subprocess
import threading
from pathlib import Path
from datetime import datetime, timedelta
from typing import Dict, List, Tuple, Optional, Any
from dataclasses import dataclass, asdict
from concurrent.futures import ThreadPoolExecutor
import hashlib
import signal
import logging
import numpy as np
from scipy import stats

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('benchmark_harness.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

@dataclass
class BenchmarkResult:
    """Single benchmark execution result"""
    timestamp: datetime
    git_commit: str
    benchmark_name: str
    compilation_time_ms: float
    execution_time_ms: float
    memory_usage_kb: int
    binary_size_bytes: int
    success: bool
    error_message: Optional[str] = None
    compiler_version: str = ""
    optimization_level: str = ""
    metadata: Dict[str, Any] = None

    def __post_init__(self):
        if self.metadata is None:
            self.metadata = {}

@dataclass
class RegressionAlert:
    """Performance regression alert"""
    timestamp: datetime
    benchmark_name: str
    metric: str
    current_value: float
    baseline_value: float
    regression_percent: float
    confidence: float
    severity: str  # 'low', 'medium', 'high', 'critical'

class StatisticalAnalyzer:
    """Statistical analysis for performance data"""
    
    @staticmethod
    def detect_regression(current_results: List[float], 
                         historical_results: List[float],
                         confidence_threshold: float = 0.95) -> Tuple[bool, float, float]:
        """
        Detect performance regression using statistical analysis
        Returns: (is_regression, p_value, effect_size)
        """
        if len(current_results) < 3 or len(historical_results) < 10:
            return False, 1.0, 0.0
        
        # Use Mann-Whitney U test for non-parametric comparison
        try:
            statistic, p_value = stats.mannwhitneyu(
                historical_results, current_results, alternative='less'
            )
            
            # Calculate effect size (Cohen's d)
            current_mean = np.mean(current_results)
            historical_mean = np.mean(historical_results)
            pooled_std = np.sqrt(
                (np.var(current_results) + np.var(historical_results)) / 2
            )
            
            if pooled_std > 0:
                effect_size = abs((current_mean - historical_mean) / pooled_std)
            else:
                effect_size = 0.0
            
            is_regression = p_value < (1 - confidence_threshold) and current_mean > historical_mean
            return is_regression, p_value, effect_size
            
        except Exception as e:
            logger.warning(f"Statistical analysis failed: {e}")
            return False, 1.0, 0.0
    
    @staticmethod
    def calculate_trend(values: List[float], timestamps: List[datetime]) -> Dict[str, float]:
        """Calculate performance trend over time"""
        if len(values) < 5:
            return {"slope": 0.0, "r_squared": 0.0, "trend": "insufficient_data"}
        
        try:
            # Convert timestamps to numeric values (days since first timestamp)
            first_time = min(timestamps)
            x = [(t - first_time).total_seconds() / 86400 for t in timestamps]
            y = values
            
            # Linear regression
            slope, intercept, r_value, p_value, std_err = stats.linregress(x, y)
            r_squared = r_value ** 2
            
            # Determine trend direction
            if abs(slope) < std_err:
                trend = "stable"
            elif slope > 0:
                trend = "degrading"
            else:
                trend = "improving"
            
            return {
                "slope": slope,
                "r_squared": r_squared,
                "trend": trend,
                "p_value": p_value
            }
            
        except Exception as e:
            logger.warning(f"Trend calculation failed: {e}")
            return {"slope": 0.0, "r_squared": 0.0, "trend": "error"}

class BenchmarkDatabase:
    """SQLite database for storing benchmark results"""
    
    def __init__(self, db_path: str = "benchmark_results.db"):
        self.db_path = db_path
        self.init_database()
    
    def init_database(self):
        """Initialize database schema"""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS benchmark_results (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    git_commit TEXT NOT NULL,
                    benchmark_name TEXT NOT NULL,
                    compilation_time_ms REAL NOT NULL,
                    execution_time_ms REAL NOT NULL,
                    memory_usage_kb INTEGER NOT NULL,
                    binary_size_bytes INTEGER NOT NULL,
                    success BOOLEAN NOT NULL,
                    error_message TEXT,
                    compiler_version TEXT,
                    optimization_level TEXT,
                    metadata TEXT
                )
            """)
            
            conn.execute("""
                CREATE TABLE IF NOT EXISTS regression_alerts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    benchmark_name TEXT NOT NULL,
                    metric TEXT NOT NULL,
                    current_value REAL NOT NULL,
                    baseline_value REAL NOT NULL,
                    regression_percent REAL NOT NULL,
                    confidence REAL NOT NULL,
                    severity TEXT NOT NULL
                )
            """)
            
            # Create indexes for performance
            conn.execute("CREATE INDEX IF NOT EXISTS idx_benchmark_timestamp ON benchmark_results(benchmark_name, timestamp)")
            conn.execute("CREATE INDEX IF NOT EXISTS idx_git_commit ON benchmark_results(git_commit)")
    
    def store_result(self, result: BenchmarkResult):
        """Store benchmark result in database"""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                INSERT INTO benchmark_results 
                (timestamp, git_commit, benchmark_name, compilation_time_ms, 
                 execution_time_ms, memory_usage_kb, binary_size_bytes, success,
                 error_message, compiler_version, optimization_level, metadata)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                result.timestamp.isoformat(),
                result.git_commit,
                result.benchmark_name,
                result.compilation_time_ms,
                result.execution_time_ms,
                result.memory_usage_kb,
                result.binary_size_bytes,
                result.success,
                result.error_message,
                result.compiler_version,
                result.optimization_level,
                json.dumps(result.metadata)
            ))
    
    def store_alert(self, alert: RegressionAlert):
        """Store regression alert in database"""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                INSERT INTO regression_alerts
                (timestamp, benchmark_name, metric, current_value, baseline_value,
                 regression_percent, confidence, severity)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                alert.timestamp.isoformat(),
                alert.benchmark_name,
                alert.metric,
                alert.current_value,
                alert.baseline_value,
                alert.regression_percent,
                alert.confidence,
                alert.severity
            ))
    
    def get_historical_results(self, benchmark_name: str, 
                             metric: str, 
                             days: int = 30) -> List[Tuple[datetime, float]]:
        """Get historical results for a benchmark metric"""
        cutoff_date = datetime.now() - timedelta(days=days)
        
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                SELECT timestamp, {} FROM benchmark_results
                WHERE benchmark_name = ? AND timestamp > ? AND success = 1
                ORDER BY timestamp DESC
            """.format(metric), (benchmark_name, cutoff_date.isoformat()))
            
            return [(datetime.fromisoformat(row[0]), row[1]) for row in cursor.fetchall()]
    
    def get_recent_alerts(self, hours: int = 24) -> List[RegressionAlert]:
        """Get recent regression alerts"""
        cutoff_date = datetime.now() - timedelta(hours=hours)
        
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                SELECT timestamp, benchmark_name, metric, current_value, baseline_value,
                       regression_percent, confidence, severity
                FROM regression_alerts
                WHERE timestamp > ?
                ORDER BY timestamp DESC
            """, (cutoff_date.isoformat(),))
            
            return [
                RegressionAlert(
                    timestamp=datetime.fromisoformat(row[0]),
                    benchmark_name=row[1],
                    metric=row[2],
                    current_value=row[3],
                    baseline_value=row[4],
                    regression_percent=row[5],
                    confidence=row[6],
                    severity=row[7]
                ) for row in cursor.fetchall()
            ]

class BenchmarkRunner:
    """Execute individual benchmarks and collect metrics"""
    
    def __init__(self, workspace_path: str):
        self.workspace_path = Path(workspace_path)
        self.git_commit = self._get_git_commit()
        
    def _get_git_commit(self) -> str:
        """Get current git commit hash"""
        try:
            result = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=self.workspace_path,
                capture_output=True,
                text=True,
                timeout=10
            )
            return result.stdout.strip() if result.returncode == 0 else "unknown"
        except Exception:
            return "unknown"
    
    def _build_compiler(self, optimization_level: str = "ReleaseFast") -> bool:
        """Build the CURSED compiler"""
        try:
            cmd = ["zig", "build"]
            if optimization_level != "Debug":
                cmd.extend([f"-Doptimize={optimization_level}"])
                
            result = subprocess.run(
                cmd,
                cwd=self.workspace_path,
                capture_output=True,
                text=True,
                timeout=300
            )
            return result.returncode == 0
        except Exception as e:
            logger.error(f"Compiler build failed: {e}")
            return False
    
    def _measure_compilation(self, benchmark_file: Path, 
                           compiler_path: Path,
                           iterations: int = 5) -> Tuple[float, bool, Optional[str]]:
        """Measure compilation time"""
        times = []
        error_message = None
        
        for i in range(iterations):
            # Clean previous outputs
            output_file = benchmark_file.with_suffix('')
            if output_file.exists():
                output_file.unlink()
            
            start_time = time.perf_counter()
            
            try:
                result = subprocess.run(
                    [str(compiler_path), str(benchmark_file), "--compile"],
                    cwd=self.workspace_path,
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                end_time = time.perf_counter()
                compilation_time = (end_time - start_time) * 1000  # Convert to milliseconds
                
                if result.returncode == 0:
                    times.append(compilation_time)
                else:
                    error_message = result.stderr or result.stdout
                    
            except subprocess.TimeoutExpired:
                error_message = "Compilation timeout"
                break
            except Exception as e:
                error_message = str(e)
                break
        
        if times:
            avg_time = statistics.mean(times)
            return avg_time, True, None
        else:
            return 0.0, False, error_message
    
    def _measure_execution(self, executable_path: Path,
                          iterations: int = 3) -> Tuple[float, int]:
        """Measure execution time and peak memory usage"""
        if not executable_path.exists():
            return 0.0, 0
        
        times = []
        max_memory = 0
        
        for i in range(iterations):
            try:
                # Use time command to measure execution and memory
                result = subprocess.run(
                    ["/usr/bin/time", "-f", "%e %M", str(executable_path)],
                    capture_output=True,
                    text=True,
                    timeout=30
                )
                
                if result.returncode == 0:
                    # Parse time output: elapsed_time max_memory_kb
                    stderr_lines = result.stderr.strip().split('\n')
                    time_line = stderr_lines[-1]  # Last line contains timing info
                    parts = time_line.split()
                    if len(parts) >= 2:
                        exec_time = float(parts[0]) * 1000  # Convert to milliseconds
                        memory_kb = int(parts[1])
                        times.append(exec_time)
                        max_memory = max(max_memory, memory_kb)
                        
            except Exception as e:
                logger.warning(f"Execution measurement failed: {e}")
                continue
        
        avg_time = statistics.mean(times) if times else 0.0
        return avg_time, max_memory
    
    def _get_binary_size(self, executable_path: Path) -> int:
        """Get compiled binary size in bytes"""
        try:
            return executable_path.stat().st_size if executable_path.exists() else 0
        except Exception:
            return 0
    
    def run_benchmark(self, benchmark_file: Path,
                     compiler_path: Path,
                     optimization_level: str = "ReleaseFast") -> BenchmarkResult:
        """Run a single benchmark and collect all metrics"""
        logger.info(f"Running benchmark: {benchmark_file.name}")
        
        # Measure compilation
        compilation_time, compilation_success, error_msg = self._measure_compilation(
            benchmark_file, compiler_path
        )
        
        executable_path = benchmark_file.with_suffix('')
        execution_time = 0.0
        memory_usage = 0
        binary_size = 0
        
        if compilation_success:
            # Measure execution and memory
            execution_time, memory_usage = self._measure_execution(executable_path)
            binary_size = self._get_binary_size(executable_path)
        
        result = BenchmarkResult(
            timestamp=datetime.now(),
            git_commit=self.git_commit,
            benchmark_name=benchmark_file.name,
            compilation_time_ms=compilation_time,
            execution_time_ms=execution_time,
            memory_usage_kb=memory_usage,
            binary_size_bytes=binary_size,
            success=compilation_success,
            error_message=error_msg,
            compiler_version=self._get_compiler_version(),
            optimization_level=optimization_level,
            metadata={
                "iterations_compilation": 5,
                "iterations_execution": 3,
                "environment": self._get_environment_info()
            }
        )
        
        return result
    
    def _get_compiler_version(self) -> str:
        """Get compiler version information"""
        try:
            compiler_path = self.workspace_path / "zig-out" / "bin" / "cursed-zig"
            if compiler_path.exists():
                result = subprocess.run(
                    [str(compiler_path), "--version"],
                    capture_output=True,
                    text=True,
                    timeout=10
                )
                return result.stdout.strip() if result.returncode == 0 else "unknown"
        except Exception:
            pass
        return "unknown"
    
    def _get_environment_info(self) -> Dict[str, str]:
        """Get environment information for context"""
        return {
            "os": os.name,
            "python_version": sys.version,
            "cpu_count": str(os.cpu_count()),
            "zig_version": self._get_zig_version()
        }
    
    def _get_zig_version(self) -> str:
        """Get Zig compiler version"""
        try:
            result = subprocess.run(
                ["zig", "version"],
                capture_output=True,
                text=True,
                timeout=10
            )
            return result.stdout.strip() if result.returncode == 0 else "unknown"
        except Exception:
            return "unknown"

class ContinuousBenchmarkHarness:
    """Main harness for continuous benchmarking"""
    
    def __init__(self, workspace_path: str, config_file: str = "benchmark_config.json"):
        self.workspace_path = Path(workspace_path)
        self.config = self._load_config(config_file)
        self.database = BenchmarkDatabase(self.config.get("database_path", "benchmark_results.db"))
        self.runner = BenchmarkRunner(workspace_path)
        self.analyzer = StatisticalAnalyzer()
        self.running = False
        self.executor = ThreadPoolExecutor(max_workers=self.config.get("max_workers", 4))
        
    def _load_config(self, config_file: str) -> Dict[str, Any]:
        """Load configuration from JSON file"""
        default_config = {
            "benchmark_interval_minutes": 60,
            "benchmark_files": [
                "benchmarks/cursed/fasta.csd",
                "benchmarks/cursed/mandelbrot.csd",
                "benchmarks/cursed/binary_trees.csd",
                "benchmarks/cursed/n_bodies.csd",
                "benchmarks/cursed/string_processing.csd",
                "comprehensive_stdlib_test.csd",
                "advanced_features_test.csd"
            ],
            "optimization_levels": ["Debug", "ReleaseFast", "ReleaseSmall"],
            "regression_threshold_percent": 10.0,
            "confidence_threshold": 0.95,
            "alert_severity_thresholds": {
                "low": 5.0,
                "medium": 15.0, 
                "high": 25.0,
                "critical": 50.0
            },
            "max_workers": 4,
            "database_path": "benchmark_results.db",
            "report_path": "benchmark_reports",
            "git_monitoring": True,
            "email_alerts": False,
            "webhook_url": None
        }
        
        try:
            with open(config_file, 'r') as f:
                user_config = json.load(f)
                default_config.update(user_config)
        except FileNotFoundError:
            logger.info(f"Config file {config_file} not found, using defaults")
            # Save default config
            with open(config_file, 'w') as f:
                json.dump(default_config, f, indent=2)
        except Exception as e:
            logger.warning(f"Error loading config: {e}, using defaults")
        
        return default_config
    
    def start_monitoring(self):
        """Start continuous benchmark monitoring"""
        logger.info("Starting continuous benchmark monitoring")
        self.running = True
        
        # Set up signal handlers for graceful shutdown
        signal.signal(signal.SIGINT, self._signal_handler)
        signal.signal(signal.SIGTERM, self._signal_handler)
        
        try:
            while self.running:
                self._run_benchmark_cycle()
                self._wait_for_next_cycle()
        except KeyboardInterrupt:
            logger.info("Received interrupt signal")
        finally:
            self.stop_monitoring()
    
    def _signal_handler(self, signum, frame):
        """Handle shutdown signals"""
        logger.info(f"Received signal {signum}, shutting down gracefully")
        self.running = False
    
    def stop_monitoring(self):
        """Stop continuous monitoring"""
        logger.info("Stopping continuous benchmark monitoring")
        self.running = False
        self.executor.shutdown(wait=True)
    
    def _run_benchmark_cycle(self):
        """Run a complete benchmark cycle"""
        logger.info("Starting benchmark cycle")
        cycle_start = datetime.now()
        
        # Build compiler if needed
        if not self._ensure_compiler_built():
            logger.error("Failed to build compiler, skipping cycle")
            return
        
        # Run benchmarks in parallel
        futures = []
        for benchmark_file in self.config["benchmark_files"]:
            benchmark_path = self.workspace_path / benchmark_file
            if not benchmark_path.exists():
                logger.warning(f"Benchmark file not found: {benchmark_path}")
                continue
            
            for opt_level in self.config["optimization_levels"]:
                future = self.executor.submit(
                    self._run_single_benchmark,
                    benchmark_path,
                    opt_level
                )
                futures.append(future)
        
        # Collect results
        results = []
        for future in futures:
            try:
                result = future.result(timeout=300)  # 5 minute timeout
                if result:
                    results.append(result)
                    self.database.store_result(result)
            except Exception as e:
                logger.error(f"Benchmark execution failed: {e}")
        
        # Analyze results for regressions
        self._analyze_for_regressions(results)
        
        cycle_duration = (datetime.now() - cycle_start).total_seconds()
        logger.info(f"Benchmark cycle completed in {cycle_duration:.1f}s, {len(results)} results collected")
    
    def _ensure_compiler_built(self) -> bool:
        """Ensure compiler is built and ready"""
        compiler_path = self.workspace_path / "zig-out" / "bin" / "cursed-zig"
        if not compiler_path.exists():
            logger.info("Compiler not found, building...")
            return self.runner._build_compiler()
        return True
    
    def _run_single_benchmark(self, benchmark_path: Path, optimization_level: str) -> Optional[BenchmarkResult]:
        """Run a single benchmark with given optimization level"""
        try:
            # Build compiler with specific optimization
            if not self.runner._build_compiler(optimization_level):
                logger.error(f"Failed to build compiler with {optimization_level}")
                return None
            
            compiler_path = self.workspace_path / "zig-out" / "bin" / "cursed-zig"
            result = self.runner.run_benchmark(benchmark_path, compiler_path, optimization_level)
            
            logger.info(f"Benchmark {benchmark_path.name} ({optimization_level}): "
                       f"compile={result.compilation_time_ms:.1f}ms, "
                       f"execute={result.execution_time_ms:.1f}ms")
            
            return result
            
        except Exception as e:
            logger.error(f"Error running benchmark {benchmark_path.name}: {e}")
            return None
    
    def _analyze_for_regressions(self, current_results: List[BenchmarkResult]):
        """Analyze current results for performance regressions"""
        for result in current_results:
            if not result.success:
                continue
            
            # Check each metric for regressions
            metrics = [
                ("compilation_time_ms", result.compilation_time_ms),
                ("execution_time_ms", result.execution_time_ms),
                ("memory_usage_kb", result.memory_usage_kb),
                ("binary_size_bytes", result.binary_size_bytes)
            ]
            
            for metric_name, current_value in metrics:
                if current_value <= 0:
                    continue
                    
                # Get historical data
                historical_data = self.database.get_historical_results(
                    result.benchmark_name, metric_name, days=7
                )
                
                if len(historical_data) < 10:
                    continue  # Not enough historical data
                
                historical_values = [value for _, value in historical_data]
                baseline_value = statistics.median(historical_values)
                
                # Calculate regression percentage
                regression_percent = ((current_value - baseline_value) / baseline_value) * 100
                
                # Check if this is a significant regression
                if regression_percent > self.config["regression_threshold_percent"]:
                    # Perform statistical analysis
                    is_regression, p_value, effect_size = self.analyzer.detect_regression(
                        [current_value], historical_values, self.config["confidence_threshold"]
                    )
                    
                    if is_regression:
                        severity = self._determine_severity(regression_percent)
                        alert = RegressionAlert(
                            timestamp=datetime.now(),
                            benchmark_name=result.benchmark_name,
                            metric=metric_name,
                            current_value=current_value,
                            baseline_value=baseline_value,
                            regression_percent=regression_percent,
                            confidence=1 - p_value,
                            severity=severity
                        )
                        
                        self.database.store_alert(alert)
                        self._send_alert(alert)
                        
                        logger.warning(f"REGRESSION DETECTED: {result.benchmark_name} {metric_name} "
                                     f"increased by {regression_percent:.1f}% "
                                     f"(confidence: {(1-p_value)*100:.1f}%)")
    
    def _determine_severity(self, regression_percent: float) -> str:
        """Determine alert severity based on regression percentage"""
        thresholds = self.config["alert_severity_thresholds"]
        if regression_percent >= thresholds["critical"]:
            return "critical"
        elif regression_percent >= thresholds["high"]:
            return "high"
        elif regression_percent >= thresholds["medium"]:
            return "medium"
        else:
            return "low"
    
    def _send_alert(self, alert: RegressionAlert):
        """Send regression alert via configured channels"""
        logger.warning(f"ALERT [{alert.severity.upper()}]: Performance regression in {alert.benchmark_name}")
        
        # TODO: Implement email alerts if configured
        if self.config.get("email_alerts"):
            self._send_email_alert(alert)
        
        # TODO: Implement webhook alerts if configured
        if self.config.get("webhook_url"):
            self._send_webhook_alert(alert)
    
    def _send_email_alert(self, alert: RegressionAlert):
        """Send email alert (placeholder)"""
        # Implementation would go here
        pass
    
    def _send_webhook_alert(self, alert: RegressionAlert):
        """Send webhook alert (placeholder)"""
        # Implementation would go here
        pass
    
    def _wait_for_next_cycle(self):
        """Wait for the next benchmark cycle"""
        interval_seconds = self.config["benchmark_interval_minutes"] * 60
        sleep_time = 0
        
        while sleep_time < interval_seconds and self.running:
            time.sleep(min(10, interval_seconds - sleep_time))  # Check every 10s
            sleep_time += 10
    
    def generate_report(self, days: int = 7) -> str:
        """Generate performance report for the last N days"""
        report_path = Path(self.config["report_path"])
        report_path.mkdir(exist_ok=True)
        
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        report_file = report_path / f"performance_report_{timestamp}.html"
        
        # Generate HTML report
        html_content = self._generate_html_report(days)
        
        with open(report_file, 'w') as f:
            f.write(html_content)
        
        logger.info(f"Performance report generated: {report_file}")
        return str(report_file)
    
    def _generate_html_report(self, days: int) -> str:
        """Generate HTML performance report"""
        # This would generate a comprehensive HTML report with charts
        # For now, returning a simple template
        return f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Compiler Performance Report</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; }}
                .header {{ background: #f0f0f0; padding: 20px; }}
                .metric {{ margin: 20px 0; }}
                .alert {{ padding: 10px; margin: 10px 0; border-left: 4px solid #ff0000; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>CURSED Compiler Performance Report</h1>
                <p>Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
                <p>Period: Last {days} days</p>
            </div>
            
            <div class="metric">
                <h2>Recent Alerts</h2>
                {self._format_recent_alerts()}
            </div>
            
            <div class="metric">
                <h2>Performance Trends</h2>
                <p>Detailed trend analysis would be shown here with charts</p>
            </div>
        </body>
        </html>
        """
    
    def _format_recent_alerts(self) -> str:
        """Format recent alerts for HTML report"""
        alerts = self.database.get_recent_alerts(hours=168)  # Last 7 days
        if not alerts:
            return "<p>No alerts in the last 7 days.</p>"
        
        html = ""
        for alert in alerts:
            html += f"""
            <div class="alert">
                <strong>{alert.severity.upper()}</strong> - {alert.benchmark_name} {alert.metric}<br>
                Regression: {alert.regression_percent:.1f}% 
                (from {alert.baseline_value:.1f} to {alert.current_value:.1f})<br>
                Time: {alert.timestamp.strftime('%Y-%m-%d %H:%M:%S')}
            </div>
            """
        return html

def main():
    """Main entry point for the continuous benchmark harness"""
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Compiler Continuous Benchmark Harness")
    parser.add_argument("--workspace", default=".", help="Path to CURSED workspace")
    parser.add_argument("--config", default="benchmark_config.json", help="Configuration file")
    parser.add_argument("--mode", choices=["monitor", "single", "report"], default="monitor",
                       help="Operation mode")
    parser.add_argument("--days", type=int, default=7, help="Days for report generation")
    
    args = parser.parse_args()
    
    harness = ContinuousBenchmarkHarness(args.workspace, args.config)
    
    if args.mode == "monitor":
        harness.start_monitoring()
    elif args.mode == "single":
        harness._run_benchmark_cycle()
    elif args.mode == "report":
        report_path = harness.generate_report(args.days)
        print(f"Report generated: {report_path}")

if __name__ == "__main__":
    main()
