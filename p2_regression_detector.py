#!/usr/bin/env python3
"""
P2 CURSED Compiler Automated Regression Detection System
=======================================================

Advanced statistical regression detection with:
- Real-time performance monitoring
- Automated alerting and bisection
- Machine learning-based anomaly detection
- Performance budget enforcement
- Trend analysis and forecasting
"""

import sqlite3
import numpy as np
import pandas as pd
import logging
from datetime import datetime, timedelta
from typing import Dict, List, Tuple, Optional, Any
from dataclasses import dataclass
import json
import subprocess
from pathlib import Path

try:
    from scipy import stats
    from sklearn.ensemble import IsolationForest
    from sklearn.preprocessing import StandardScaler
    from sklearn.linear_model import LinearRegression
    import matplotlib.pyplot as plt
    import seaborn as sns
    ADVANCED_FEATURES = True
except ImportError:
    ADVANCED_FEATURES = False

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class RegressionAlert:
    """Performance regression alert"""
    benchmark_name: str
    metric: str
    current_value: float
    baseline_value: float
    regression_percent: float
    confidence: float
    severity: str
    timestamp: datetime
    git_commit: str
    alert_id: str

@dataclass
class PerformanceBudget:
    """Performance budget configuration"""
    benchmark_name: str
    metric: str
    budget_value: float
    warning_threshold: float
    critical_threshold: float
    enabled: bool

@dataclass
class TrendAnalysis:
    """Performance trend analysis result"""
    benchmark_name: str
    metric: str
    trend_direction: str  # 'improving', 'degrading', 'stable'
    slope: float
    r_squared: float
    prediction_7d: float
    prediction_30d: float
    confidence_interval: Tuple[float, float]

class StatisticalRegressionDetector:
    """Advanced statistical regression detection"""
    
    def __init__(self, database_path: str, config: Dict[str, Any]):
        self.database_path = database_path
        self.config = config
        self.scaler = StandardScaler() if ADVANCED_FEATURES else None
        
    def detect_regressions(self, recent_results: List[Dict[str, Any]], 
                          lookback_days: int = 14) -> List[RegressionAlert]:
        """Detect performance regressions using multiple statistical methods"""
        alerts = []
        
        for result in recent_results:
            if not result.get('success', False):
                continue
                
            benchmark_name = result['benchmark_name']
            
            # Check each metric
            metrics = ['compilation_time_ms', 'execution_time_ms', 'memory_usage_kb']
            for metric in metrics:
                current_value = result.get(metric, 0)
                if current_value <= 0:
                    continue
                
                # Get historical data
                historical_data = self._get_historical_data(
                    benchmark_name, metric, lookback_days
                )
                
                if len(historical_data) < self.config.get('min_historical_samples', 10):
                    continue
                
                # Perform regression detection
                alert = self._analyze_regression(
                    benchmark_name, metric, current_value, 
                    historical_data, result.get('git_commit', 'unknown')
                )
                
                if alert:
                    alerts.append(alert)
        
        return alerts
    
    def _get_historical_data(self, benchmark_name: str, metric: str, 
                           days: int) -> List[float]:
        """Get historical performance data"""
        cutoff_date = datetime.now() - timedelta(days=days)
        
        with sqlite3.connect(self.database_path) as conn:
            cursor = conn.execute(f"""
                SELECT {metric} FROM cross_language_results
                WHERE benchmark_name = ? AND language = 'cursed' 
                AND timestamp > ? AND success = 1
                ORDER BY timestamp DESC
            """, (benchmark_name, cutoff_date.isoformat()))
            
            return [row[0] for row in cursor.fetchall()]
    
    def _analyze_regression(self, benchmark_name: str, metric: str, 
                          current_value: float, historical_data: List[float],
                          git_commit: str) -> Optional[RegressionAlert]:
        """Analyze for regression using multiple statistical tests"""
        
        # 1. Basic threshold check
        baseline_value = np.median(historical_data)
        regression_percent = ((current_value - baseline_value) / baseline_value) * 100
        
        min_regression = self.config.get('regression_thresholds', {}).get('minor', 5.0)
        if regression_percent < min_regression:
            return None
        
        # 2. Mann-Whitney U test for statistical significance
        try:
            statistic, p_value = stats.mannwhitneyu(
                historical_data, [current_value], alternative='less'
            )
            confidence = 1 - p_value
        except Exception:
            confidence = 0.0
        
        min_confidence = self.config.get('confidence_threshold', 0.95)
        if confidence < min_confidence:
            return None
        
        # 3. Effect size calculation (Cohen's d)
        try:
            historical_mean = np.mean(historical_data)
            historical_std = np.std(historical_data)
            effect_size = abs((current_value - historical_mean) / historical_std)
        except Exception:
            effect_size = 0.0
        
        # 4. Anomaly detection using Isolation Forest
        if ADVANCED_FEATURES and len(historical_data) >= 20:
            try:
                # Prepare data for anomaly detection
                data = np.array(historical_data + [current_value]).reshape(-1, 1)
                
                # Fit Isolation Forest
                isolation_forest = IsolationForest(contamination=0.1, random_state=42)
                anomaly_scores = isolation_forest.fit_predict(data)
                
                # Check if current value is an anomaly
                is_anomaly = anomaly_scores[-1] == -1
                if not is_anomaly:
                    logger.info(f"Current value not detected as anomaly by Isolation Forest")
                    return None
            except Exception as e:
                logger.warning(f"Anomaly detection failed: {e}")
        
        # Determine severity
        severity = self._determine_severity(regression_percent)
        
        # Generate unique alert ID
        alert_id = f"{benchmark_name}_{metric}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
        
        return RegressionAlert(
            benchmark_name=benchmark_name,
            metric=metric,
            current_value=current_value,
            baseline_value=baseline_value,
            regression_percent=regression_percent,
            confidence=confidence,
            severity=severity,
            timestamp=datetime.now(),
            git_commit=git_commit,
            alert_id=alert_id
        )
    
    def _determine_severity(self, regression_percent: float) -> str:
        """Determine alert severity based on regression percentage"""
        thresholds = self.config.get('regression_thresholds', {})
        
        if regression_percent >= thresholds.get('critical', 25.0):
            return 'critical'
        elif regression_percent >= thresholds.get('major', 15.0):
            return 'major'
        else:
            return 'minor'

class PerformanceBudgetMonitor:
    """Monitor performance budgets and enforce limits"""
    
    def __init__(self, database_path: str, budgets: List[PerformanceBudget]):
        self.database_path = database_path
        self.budgets = {(b.benchmark_name, b.metric): b for b in budgets}
    
    def check_budgets(self, results: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Check if results violate performance budgets"""
        violations = []
        
        for result in results:
            if not result.get('success', False):
                continue
            
            benchmark_name = result['benchmark_name']
            
            for metric in ['compilation_time_ms', 'execution_time_ms', 'memory_usage_kb']:
                budget_key = (benchmark_name, metric)
                if budget_key not in self.budgets:
                    continue
                
                budget = self.budgets[budget_key]
                if not budget.enabled:
                    continue
                
                current_value = result.get(metric, 0)
                if current_value <= 0:
                    continue
                
                # Check against budget
                budget_exceeded = current_value > budget.budget_value
                warning_exceeded = current_value > budget.warning_threshold
                critical_exceeded = current_value > budget.critical_threshold
                
                if budget_exceeded:
                    violation_type = 'critical' if critical_exceeded else (
                        'warning' if warning_exceeded else 'budget'
                    )
                    
                    violations.append({
                        'benchmark_name': benchmark_name,
                        'metric': metric,
                        'current_value': current_value,
                        'budget_value': budget.budget_value,
                        'violation_type': violation_type,
                        'excess_percent': ((current_value - budget.budget_value) / budget.budget_value) * 100,
                        'timestamp': datetime.now().isoformat()
                    })
        
        return violations

class TrendAnalyzer:
    """Analyze performance trends and forecast future performance"""
    
    def __init__(self, database_path: str):
        self.database_path = database_path
    
    def analyze_trends(self, days: int = 30) -> List[TrendAnalysis]:
        """Analyze performance trends over time"""
        if not ADVANCED_FEATURES:
            logger.warning("Advanced features not available for trend analysis")
            return []
        
        trends = []
        
        # Get unique benchmark/metric combinations
        with sqlite3.connect(self.database_path) as conn:
            cursor = conn.execute("""
                SELECT DISTINCT benchmark_name FROM cross_language_results
                WHERE language = 'cursed' AND success = 1
            """)
            benchmarks = [row[0] for row in cursor.fetchall()]
        
        metrics = ['compilation_time_ms', 'execution_time_ms', 'memory_usage_kb']
        
        for benchmark in benchmarks:
            for metric in metrics:
                trend = self._analyze_single_trend(benchmark, metric, days)
                if trend:
                    trends.append(trend)
        
        return trends
    
    def _analyze_single_trend(self, benchmark_name: str, metric: str, 
                            days: int) -> Optional[TrendAnalysis]:
        """Analyze trend for a single benchmark/metric combination"""
        cutoff_date = datetime.now() - timedelta(days=days)
        
        with sqlite3.connect(self.database_path) as conn:
            df = pd.read_sql_query(f"""
                SELECT timestamp, {metric} 
                FROM cross_language_results
                WHERE benchmark_name = ? AND language = 'cursed' 
                AND timestamp > ? AND success = 1
                ORDER BY timestamp
            """, conn, params=(benchmark_name, cutoff_date.isoformat()))
        
        if len(df) < 10:  # Need minimum data points
            return None
        
        try:
            # Convert timestamp to numeric for regression
            df['timestamp'] = pd.to_datetime(df['timestamp'])
            df['days'] = (df['timestamp'] - df['timestamp'].min()).dt.total_seconds() / 86400
            
            # Perform linear regression
            X = df['days'].values.reshape(-1, 1)
            y = df[metric].values
            
            model = LinearRegression()
            model.fit(X, y)
            
            # Calculate trend metrics
            slope = model.coef_[0]
            r_squared = model.score(X, y)
            
            # Determine trend direction
            if abs(slope) < np.std(y) * 0.01:  # Less than 1% of std dev
                trend_direction = 'stable'
            elif slope > 0:
                trend_direction = 'degrading'  # Higher values = worse performance
            else:
                trend_direction = 'improving'
            
            # Make predictions
            current_days = df['days'].max()
            prediction_7d = model.predict([[current_days + 7]])[0]
            prediction_30d = model.predict([[current_days + 30]])[0]
            
            # Calculate confidence interval (simplified)
            residuals = y - model.predict(X)
            mse = np.mean(residuals ** 2)
            confidence_interval = (prediction_30d - 2 * np.sqrt(mse), 
                                 prediction_30d + 2 * np.sqrt(mse))
            
            return TrendAnalysis(
                benchmark_name=benchmark_name,
                metric=metric,
                trend_direction=trend_direction,
                slope=slope,
                r_squared=r_squared,
                prediction_7d=prediction_7d,
                prediction_30d=prediction_30d,
                confidence_interval=confidence_interval
            )
            
        except Exception as e:
            logger.warning(f"Trend analysis failed for {benchmark_name}/{metric}: {e}")
            return None

class AutomatedBisector:
    """Automated bisection to find regression-causing commits"""
    
    def __init__(self, workspace_path: str, database_path: str):
        self.workspace_path = Path(workspace_path)
        self.database_path = database_path
    
    def bisect_regression(self, alert: RegressionAlert, 
                         known_good_commit: str) -> Optional[str]:
        """Automatically bisect to find the commit that caused regression"""
        logger.info(f"Starting automatic bisection for {alert.alert_id}")
        
        try:
            # Start git bisect
            subprocess.run(['git', 'bisect', 'start'], 
                         cwd=self.workspace_path, check=True)
            subprocess.run(['git', 'bisect', 'bad', alert.git_commit], 
                         cwd=self.workspace_path, check=True)
            subprocess.run(['git', 'bisect', 'good', known_good_commit], 
                         cwd=self.workspace_path, check=True)
            
            # Bisect loop
            max_iterations = 20
            for i in range(max_iterations):
                # Get current commit
                result = subprocess.run(['git', 'rev-parse', 'HEAD'],
                                      cwd=self.workspace_path, 
                                      capture_output=True, text=True, check=True)
                current_commit = result.stdout.strip()
                
                # Build and test current commit
                is_good = self._test_commit(alert.benchmark_name, alert.metric, 
                                          alert.baseline_value)
                
                # Mark commit as good or bad
                mark = 'good' if is_good else 'bad'
                result = subprocess.run(['git', 'bisect', mark],
                                      cwd=self.workspace_path,
                                      capture_output=True, text=True)
                
                # Check if bisect is complete
                if 'is the first bad commit' in result.stdout:
                    logger.info(f"Bisection complete: {current_commit} is the regression commit")
                    return current_commit
            
            logger.warning("Bisection did not complete within maximum iterations")
            return None
            
        except subprocess.CalledProcessError as e:
            logger.error(f"Git bisect failed: {e}")
            return None
        finally:
            # Clean up bisect state
            try:
                subprocess.run(['git', 'bisect', 'reset'], 
                             cwd=self.workspace_path, check=False)
            except Exception:
                pass
    
    def _test_commit(self, benchmark_name: str, metric: str, 
                    baseline_value: float) -> bool:
        """Test if a commit is good (no regression) or bad (has regression)"""
        try:
            # Build compiler
            subprocess.run(['zig', 'build', '-Doptimize=ReleaseFast'],
                         cwd=self.workspace_path, check=True, 
                         capture_output=True, timeout=120)
            
            # Run specific benchmark
            # This would need to be implemented based on benchmark infrastructure
            # For now, return a simplified test
            
            return True  # Placeholder - would implement actual test
            
        except Exception as e:
            logger.warning(f"Commit test failed: {e}")
            return False

class P2RegressionMonitor:
    """Main P2 regression monitoring system"""
    
    def __init__(self, workspace_path: str, config_file: str):
        self.workspace_path = workspace_path
        self.config = self._load_config(config_file)
        self.database_path = self.config.get('database_path', 'p2_benchmark_results.db')
        
        # Initialize components
        self.regression_detector = StatisticalRegressionDetector(
            self.database_path, self.config
        )
        self.trend_analyzer = TrendAnalyzer(self.database_path)
        self.bisector = AutomatedBisector(workspace_path, self.database_path)
        
        # Load performance budgets
        budgets = self._load_performance_budgets()
        self.budget_monitor = PerformanceBudgetMonitor(self.database_path, budgets)
    
    def _load_config(self, config_file: str) -> Dict[str, Any]:
        """Load regression monitoring configuration"""
        try:
            with open(config_file, 'r') as f:
                return json.load(f)
        except FileNotFoundError:
            return {}
    
    def _load_performance_budgets(self) -> List[PerformanceBudget]:
        """Load performance budgets from configuration"""
        budgets = []
        budget_config = self.config.get('performance_budgets', {})
        
        for benchmark_name, metrics in budget_config.items():
            for metric, values in metrics.items():
                if isinstance(values, dict):
                    budget = PerformanceBudget(
                        benchmark_name=benchmark_name,
                        metric=metric,
                        budget_value=values.get('budget', 1000),
                        warning_threshold=values.get('warning', 1500),
                        critical_threshold=values.get('critical', 2000),
                        enabled=values.get('enabled', True)
                    )
                    budgets.append(budget)
        
        return budgets
    
    def monitor_latest_results(self) -> Dict[str, Any]:
        """Monitor latest benchmark results for regressions"""
        # Get recent results
        recent_results = self._get_recent_results(hours=1)
        
        if not recent_results:
            logger.info("No recent results to analyze")
            return {'alerts': [], 'budget_violations': [], 'trends': []}
        
        # Detect regressions
        alerts = self.regression_detector.detect_regressions(recent_results)
        
        # Check performance budgets
        budget_violations = self.budget_monitor.check_budgets(recent_results)
        
        # Analyze trends
        trends = self.trend_analyzer.analyze_trends()
        
        # Store alerts
        self._store_alerts(alerts)
        
        # Auto-bisect critical regressions
        for alert in alerts:
            if alert.severity == 'critical':
                self._handle_critical_regression(alert)
        
        return {
            'alerts': [alert.__dict__ for alert in alerts],
            'budget_violations': budget_violations,
            'trends': [trend.__dict__ for trend in trends],
            'monitoring_timestamp': datetime.now().isoformat()
        }
    
    def _get_recent_results(self, hours: int = 1) -> List[Dict[str, Any]]:
        """Get recent benchmark results"""
        cutoff_time = datetime.now() - timedelta(hours=hours)
        
        with sqlite3.connect(self.database_path) as conn:
            cursor = conn.execute("""
                SELECT benchmark_name, language, compilation_time_ms, 
                       execution_time_ms, memory_usage_kb, success, git_commit
                FROM cross_language_results
                WHERE timestamp > ? AND language = 'cursed'
                ORDER BY timestamp DESC
            """, (cutoff_time.isoformat(),))
            
            results = []
            for row in cursor.fetchall():
                results.append({
                    'benchmark_name': row[0],
                    'language': row[1],
                    'compilation_time_ms': row[2],
                    'execution_time_ms': row[3],
                    'memory_usage_kb': row[4],
                    'success': bool(row[5]),
                    'git_commit': row[6]
                })
            
            return results
    
    def _store_alerts(self, alerts: List[RegressionAlert]):
        """Store regression alerts in database"""
        if not alerts:
            return
        
        with sqlite3.connect(self.database_path) as conn:
            # Create alerts table if it doesn't exist
            conn.execute("""
                CREATE TABLE IF NOT EXISTS regression_alerts (
                    id INTEGER PRIMARY KEY,
                    alert_id TEXT UNIQUE,
                    benchmark_name TEXT,
                    metric TEXT,
                    current_value REAL,
                    baseline_value REAL,
                    regression_percent REAL,
                    confidence REAL,
                    severity TEXT,
                    timestamp TEXT,
                    git_commit TEXT,
                    resolved BOOLEAN DEFAULT 0
                )
            """)
            
            for alert in alerts:
                conn.execute("""
                    INSERT OR REPLACE INTO regression_alerts
                    (alert_id, benchmark_name, metric, current_value, baseline_value,
                     regression_percent, confidence, severity, timestamp, git_commit)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                """, (
                    alert.alert_id, alert.benchmark_name, alert.metric,
                    alert.current_value, alert.baseline_value,
                    alert.regression_percent, alert.confidence,
                    alert.severity, alert.timestamp.isoformat(), alert.git_commit
                ))
            
            conn.commit()
            logger.info(f"Stored {len(alerts)} regression alerts")
    
    def _handle_critical_regression(self, alert: RegressionAlert):
        """Handle critical regression with automated bisection"""
        logger.warning(f"Critical regression detected: {alert.alert_id}")
        
        # Find a known good commit (simplified - would need more sophisticated logic)
        good_commit = self._find_known_good_commit(alert)
        
        if good_commit:
            regression_commit = self.bisector.bisect_regression(alert, good_commit)
            if regression_commit:
                logger.error(f"Regression caused by commit: {regression_commit}")
                # Here you would typically:
                # - Send notifications
                # - Create GitHub issues
                # - Update alerts database
                # - Potentially revert the commit
    
    def _find_known_good_commit(self, alert: RegressionAlert) -> Optional[str]:
        """Find a known good commit for bisection"""
        # This is a simplified implementation
        # In practice, you'd want more sophisticated logic
        try:
            result = subprocess.run(['git', 'log', '--oneline', '-10'],
                                  cwd=self.workspace_path, capture_output=True, text=True)
            commits = result.stdout.strip().split('\n')
            if len(commits) >= 5:
                return commits[5].split()[0]  # 5 commits back
        except Exception:
            pass
        return None

def main():
    """Main entry point for P2 regression monitoring"""
    import argparse
    
    parser = argparse.ArgumentParser(description="P2 CURSED Regression Monitor")
    parser.add_argument("--workspace", default=".", help="Workspace path")
    parser.add_argument("--config", default="p2_benchmark_config.json", help="Config file")
    parser.add_argument("--mode", choices=["monitor", "analyze-trends", "check-budgets"],
                       default="monitor", help="Operation mode")
    
    args = parser.parse_args()
    
    monitor = P2RegressionMonitor(args.workspace, args.config)
    
    if args.mode == "monitor":
        results = monitor.monitor_latest_results()
        print(json.dumps(results, indent=2, default=str))
    elif args.mode == "analyze-trends":
        trends = monitor.trend_analyzer.analyze_trends()
        for trend in trends:
            print(f"{trend.benchmark_name}/{trend.metric}: {trend.trend_direction}")
    elif args.mode == "check-budgets":
        # Would implement budget checking
        print("Budget checking not yet implemented")

if __name__ == "__main__":
    main()
