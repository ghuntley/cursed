#!/usr/bin/env python3
"""
CURSED Compiler Benchmark Analysis Tools

Advanced statistical analysis and visualization tools for benchmark data.
Includes regression detection, performance forecasting, and anomaly detection.
"""

import sqlite3
import numpy as np
import pandas as pd
from datetime import datetime, timedelta
from typing import List, Dict, Tuple, Optional, Any
from dataclasses import dataclass
import json
from pathlib import Path

try:
    import matplotlib.pyplot as plt
    import seaborn as sns
    from scipy import stats
    from sklearn.linear_model import LinearRegression
    from sklearn.preprocessing import StandardScaler
    from sklearn.metrics import mean_squared_error, r2_score
    from sklearn.ensemble import IsolationForest
    ANALYSIS_AVAILABLE = True
except ImportError:
    ANALYSIS_AVAILABLE = False
    print("Advanced analysis libraries not available.")
    print("Install with: pip install matplotlib seaborn scikit-learn")

@dataclass
class PerformanceMetrics:
    """Performance metrics for a benchmark"""
    benchmark_name: str
    mean_compilation_time: float
    std_compilation_time: float
    mean_execution_time: float
    std_execution_time: float
    mean_memory_usage: float
    std_memory_usage: float
    success_rate: float
    total_runs: int
    trend_slope: float
    volatility: float
    performance_score: float

@dataclass
class RegressionAnalysis:
    """Regression analysis results"""
    benchmark_name: str
    metric: str
    is_regression: bool
    p_value: float
    effect_size: float
    confidence: float
    current_mean: float
    historical_mean: float
    regression_magnitude: float

@dataclass
class ForecastResult:
    """Performance forecast result"""
    benchmark_name: str
    metric: str
    forecast_dates: List[datetime]
    forecast_values: List[float]
    confidence_lower: List[float]
    confidence_upper: List[float]
    model_accuracy: float
    trend_direction: str

class BenchmarkAnalyzer:
    """Advanced benchmark analysis and visualization"""
    
    def __init__(self, database_path: str = "benchmark_results.db"):
        self.database_path = database_path
        self.scaler = StandardScaler() if ANALYSIS_AVAILABLE else None
    
    def load_data(self, days: int = 30) -> pd.DataFrame:
        """Load benchmark data into pandas DataFrame"""
        if not ANALYSIS_AVAILABLE:
            raise ImportError("pandas not available")
        
        cutoff_date = datetime.now() - timedelta(days=days)
        
        with sqlite3.connect(self.database_path) as conn:
            query = """
                SELECT timestamp, git_commit, benchmark_name, compilation_time_ms,
                       execution_time_ms, memory_usage_kb, binary_size_bytes,
                       success, compiler_version, optimization_level
                FROM benchmark_results
                WHERE timestamp > ?
                ORDER BY timestamp
            """
            
            df = pd.read_sql_query(query, conn, params=(cutoff_date.isoformat(),))
            df['timestamp'] = pd.to_datetime(df['timestamp'])
            
            return df
    
    def calculate_performance_metrics(self, df: pd.DataFrame) -> List[PerformanceMetrics]:
        """Calculate comprehensive performance metrics for each benchmark"""
        metrics = []
        
        for benchmark_name in df['benchmark_name'].unique():
            benchmark_data = df[df['benchmark_name'] == benchmark_name]
            successful_runs = benchmark_data[benchmark_data['success'] == 1]
            
            if len(successful_runs) < 5:
                continue  # Not enough data
            
            # Calculate basic statistics
            mean_compilation = successful_runs['compilation_time_ms'].mean()
            std_compilation = successful_runs['compilation_time_ms'].std()
            mean_execution = successful_runs['execution_time_ms'].mean()
            std_execution = successful_runs['execution_time_ms'].std()
            mean_memory = successful_runs['memory_usage_kb'].mean()
            std_memory = successful_runs['memory_usage_kb'].std()
            
            # Calculate success rate
            success_rate = len(successful_runs) / len(benchmark_data)
            
            # Calculate trend
            if len(successful_runs) >= 10:
                x = np.arange(len(successful_runs))
                y = successful_runs['compilation_time_ms'].values
                slope, _, r_value, _, _ = stats.linregress(x, y)
                trend_slope = slope
            else:
                trend_slope = 0.0
            
            # Calculate volatility (coefficient of variation)
            volatility = std_compilation / mean_compilation if mean_compilation > 0 else 0
            
            # Calculate performance score (lower is better)
            baseline_time = 1000  # 1 second baseline
            performance_score = max(0, 100 - (mean_compilation / baseline_time * 50))
            
            metrics.append(PerformanceMetrics(
                benchmark_name=benchmark_name,
                mean_compilation_time=mean_compilation,
                std_compilation_time=std_compilation,
                mean_execution_time=mean_execution,
                std_execution_time=std_execution,
                mean_memory_usage=mean_memory,
                std_memory_usage=std_memory,
                success_rate=success_rate,
                total_runs=len(benchmark_data),
                trend_slope=trend_slope,
                volatility=volatility,
                performance_score=performance_score
            ))
        
        return metrics
    
    def detect_regressions(self, df: pd.DataFrame, 
                          window_size: int = 10,
                          confidence_threshold: float = 0.95) -> List[RegressionAnalysis]:
        """Detect performance regressions using statistical methods"""
        regressions = []
        
        for benchmark_name in df['benchmark_name'].unique():
            benchmark_data = df[df['benchmark_name'] == benchmark_name]
            successful_runs = benchmark_data[benchmark_data['success'] == 1]
            
            if len(successful_runs) < window_size * 2:
                continue
            
            # Analyze each metric
            metrics = ['compilation_time_ms', 'execution_time_ms', 'memory_usage_kb']
            
            for metric in metrics:
                values = successful_runs[metric].values
                
                if len(values) < window_size * 2:
                    continue
                
                # Split into historical and recent data
                split_point = len(values) - window_size
                historical_values = values[:split_point]
                recent_values = values[split_point:]
                
                # Perform statistical test
                is_regression, p_value, effect_size = self._statistical_regression_test(
                    historical_values, recent_values, confidence_threshold
                )
                
                if is_regression:
                    current_mean = np.mean(recent_values)
                    historical_mean = np.mean(historical_values)
                    regression_magnitude = ((current_mean - historical_mean) / historical_mean) * 100
                    
                    regressions.append(RegressionAnalysis(
                        benchmark_name=benchmark_name,
                        metric=metric,
                        is_regression=True,
                        p_value=p_value,
                        effect_size=effect_size,
                        confidence=1 - p_value,
                        current_mean=current_mean,
                        historical_mean=historical_mean,
                        regression_magnitude=regression_magnitude
                    ))
        
        return regressions
    
    def _statistical_regression_test(self, historical: np.ndarray, 
                                   recent: np.ndarray,
                                   confidence_threshold: float) -> Tuple[bool, float, float]:
        """Perform statistical test for regression detection"""
        try:
            # Use Mann-Whitney U test for non-parametric comparison
            statistic, p_value = stats.mannwhitneyu(
                historical, recent, alternative='less'
            )
            
            # Calculate effect size (Cohen's d)
            historical_mean = np.mean(historical)
            recent_mean = np.mean(recent)
            pooled_std = np.sqrt((np.var(historical) + np.var(recent)) / 2)
            
            if pooled_std > 0:
                effect_size = abs((recent_mean - historical_mean) / pooled_std)
            else:
                effect_size = 0.0
            
            # Check for regression (recent values significantly higher)
            is_regression = (p_value < (1 - confidence_threshold) and 
                           recent_mean > historical_mean)
            
            return is_regression, p_value, effect_size
            
        except Exception:
            return False, 1.0, 0.0
    
    def detect_anomalies(self, df: pd.DataFrame, 
                        contamination: float = 0.1) -> pd.DataFrame:
        """Detect anomalous benchmark results using Isolation Forest"""
        if not ANALYSIS_AVAILABLE:
            raise ImportError("scikit-learn not available")
        
        anomalies_list = []
        
        for benchmark_name in df['benchmark_name'].unique():
            benchmark_data = df[df['benchmark_name'] == benchmark_name]
            successful_runs = benchmark_data[benchmark_data['success'] == 1]
            
            if len(successful_runs) < 20:
                continue  # Need enough data for anomaly detection
            
            # Select features for anomaly detection
            features = ['compilation_time_ms', 'execution_time_ms', 'memory_usage_kb']
            X = successful_runs[features].values
            
            # Handle missing values
            X = np.nan_to_num(X)
            
            # Normalize features
            X_scaled = self.scaler.fit_transform(X)
            
            # Detect anomalies
            iso_forest = IsolationForest(contamination=contamination, random_state=42)
            anomaly_predictions = iso_forest.fit_predict(X_scaled)
            
            # Mark anomalies
            anomaly_indices = successful_runs.index[anomaly_predictions == -1]
            anomalous_data = successful_runs.loc[anomaly_indices].copy()
            anomalous_data['anomaly_score'] = iso_forest.score_samples(X_scaled)[anomaly_predictions == -1]
            anomalous_data['benchmark_name'] = benchmark_name
            
            anomalies_list.append(anomalous_data)
        
        if anomalies_list:
            return pd.concat(anomalies_list, ignore_index=True)
        else:
            return pd.DataFrame()
    
    def forecast_performance(self, df: pd.DataFrame, 
                           forecast_days: int = 7) -> List[ForecastResult]:
        """Forecast future performance using time series analysis"""
        if not ANALYSIS_AVAILABLE:
            raise ImportError("scikit-learn not available")
        
        forecasts = []
        
        for benchmark_name in df['benchmark_name'].unique():
            benchmark_data = df[df['benchmark_name'] == benchmark_name]
            successful_runs = benchmark_data[benchmark_data['success'] == 1]
            
            if len(successful_runs) < 30:
                continue  # Need enough historical data
            
            # Analyze compilation time trends
            time_data = successful_runs.sort_values('timestamp')
            
            # Convert timestamps to numeric values (days since first measurement)
            first_timestamp = time_data['timestamp'].iloc[0]
            time_data = time_data.copy()
            time_data['days_since_start'] = (time_data['timestamp'] - first_timestamp).dt.total_seconds() / 86400
            
            X = time_data['days_since_start'].values.reshape(-1, 1)
            y = time_data['compilation_time_ms'].values
            
            # Fit linear regression model
            model = LinearRegression()
            model.fit(X, y)
            
            # Calculate model accuracy
            y_pred = model.predict(X)
            r2 = r2_score(y, y_pred)
            
            # Generate future time points
            last_day = X[-1, 0]
            future_days = np.arange(last_day + 1, last_day + forecast_days + 1).reshape(-1, 1)
            
            # Make predictions
            forecast_values = model.predict(future_days)
            
            # Calculate confidence intervals (simple approximation)
            residuals = y - y_pred
            std_residual = np.std(residuals)
            confidence_margin = 1.96 * std_residual  # 95% confidence interval
            
            confidence_lower = forecast_values - confidence_margin
            confidence_upper = forecast_values + confidence_margin
            
            # Convert future days back to timestamps
            forecast_dates = [
                first_timestamp + timedelta(days=float(day))
                for day in future_days.flatten()
            ]
            
            # Determine trend direction
            slope = model.coef_[0]
            if abs(slope) < 1:
                trend_direction = "stable"
            elif slope > 0:
                trend_direction = "degrading"
            else:
                trend_direction = "improving"
            
            forecasts.append(ForecastResult(
                benchmark_name=benchmark_name,
                metric="compilation_time_ms",
                forecast_dates=forecast_dates,
                forecast_values=forecast_values.tolist(),
                confidence_lower=confidence_lower.tolist(),
                confidence_upper=confidence_upper.tolist(),
                model_accuracy=r2,
                trend_direction=trend_direction
            ))
        
        return forecasts
    
    def generate_performance_report(self, output_path: str = "performance_analysis.html",
                                  days: int = 30) -> str:
        """Generate comprehensive performance analysis report"""
        if not ANALYSIS_AVAILABLE:
            raise ImportError("Analysis libraries not available")
        
        # Load data
        df = self.load_data(days)
        
        if df.empty:
            return "No data available for analysis"
        
        # Perform analyses
        metrics = self.calculate_performance_metrics(df)
        regressions = self.detect_regressions(df)
        anomalies = self.detect_anomalies(df)
        forecasts = self.forecast_performance(df)
        
        # Generate visualizations
        self._create_performance_plots(df, metrics, forecasts)
        
        # Generate HTML report
        html_content = self._generate_analysis_html(metrics, regressions, anomalies, forecasts, days)
        
        with open(output_path, 'w') as f:
            f.write(html_content)
        
        return output_path
    
    def _create_performance_plots(self, df: pd.DataFrame, 
                                metrics: List[PerformanceMetrics],
                                forecasts: List[ForecastResult]):
        """Create performance visualization plots"""
        if not ANALYSIS_AVAILABLE:
            return
        
        # Set up the plotting style
        plt.style.use('seaborn-v0_8')
        fig, axes = plt.subplots(2, 2, figsize=(15, 12))
        
        # Plot 1: Compilation time trends
        ax1 = axes[0, 0]
        for benchmark_name in df['benchmark_name'].unique()[:5]:  # Limit to top 5
            benchmark_data = df[df['benchmark_name'] == benchmark_name]
            successful_runs = benchmark_data[benchmark_data['success'] == 1]
            
            if len(successful_runs) > 0:
                ax1.plot(successful_runs['timestamp'], 
                        successful_runs['compilation_time_ms'],
                        label=benchmark_name, alpha=0.7)
        
        ax1.set_title('Compilation Time Trends')
        ax1.set_xlabel('Time')
        ax1.set_ylabel('Compilation Time (ms)')
        ax1.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
        ax1.tick_params(axis='x', rotation=45)
        
        # Plot 2: Performance distribution
        ax2 = axes[0, 1]
        compilation_times = []
        benchmark_names = []
        
        for metric in metrics:
            compilation_times.append(metric.mean_compilation_time)
            benchmark_names.append(metric.benchmark_name)
        
        ax2.barh(benchmark_names, compilation_times)
        ax2.set_title('Average Compilation Times')
        ax2.set_xlabel('Compilation Time (ms)')
        
        # Plot 3: Success rate analysis
        ax3 = axes[1, 0]
        success_rates = [metric.success_rate * 100 for metric in metrics]
        benchmark_names = [metric.benchmark_name for metric in metrics]
        
        colors = ['green' if rate >= 95 else 'orange' if rate >= 90 else 'red' 
                 for rate in success_rates]
        ax3.bar(benchmark_names, success_rates, color=colors)
        ax3.set_title('Benchmark Success Rates')
        ax3.set_ylabel('Success Rate (%)')
        ax3.tick_params(axis='x', rotation=45)
        ax3.axhline(y=95, color='red', linestyle='--', alpha=0.7, label='Target: 95%')
        ax3.legend()
        
        # Plot 4: Performance volatility
        ax4 = axes[1, 1]
        volatilities = [metric.volatility for metric in metrics]
        benchmark_names = [metric.benchmark_name for metric in metrics]
        
        ax4.scatter(compilation_times, volatilities, s=100, alpha=0.7)
        for i, name in enumerate(benchmark_names):
            ax4.annotate(name, (compilation_times[i], volatilities[i]), 
                        xytext=(5, 5), textcoords='offset points', fontsize=8)
        
        ax4.set_title('Performance vs Volatility')
        ax4.set_xlabel('Mean Compilation Time (ms)')
        ax4.set_ylabel('Volatility (CV)')
        
        plt.tight_layout()
        plt.savefig('performance_analysis_plots.png', dpi=300, bbox_inches='tight')
        plt.close()
    
    def _generate_analysis_html(self, metrics: List[PerformanceMetrics],
                               regressions: List[RegressionAnalysis],
                               anomalies: pd.DataFrame,
                               forecasts: List[ForecastResult],
                               days: int) -> str:
        """Generate HTML report with analysis results"""
        
        # Calculate summary statistics
        total_benchmarks = len(metrics)
        avg_success_rate = np.mean([m.success_rate for m in metrics]) * 100
        critical_regressions = len([r for r in regressions if r.regression_magnitude > 25])
        total_anomalies = len(anomalies)
        
        html = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Compiler Performance Analysis Report</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; line-height: 1.6; }}
                .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); 
                          color: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; }}
                .summary {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); 
                           gap: 20px; margin: 20px 0; }}
                .metric-card {{ background: #f8f9fa; padding: 15px; border-radius: 8px; text-align: center; }}
                .metric-value {{ font-size: 2em; font-weight: bold; color: #667eea; }}
                .section {{ margin: 30px 0; }}
                .regression-alert {{ padding: 10px; margin: 10px 0; border-left: 4px solid #dc3545; 
                                   background: #f8d7da; border-radius: 4px; }}
                .anomaly-alert {{ padding: 10px; margin: 10px 0; border-left: 4px solid #ffc107; 
                                background: #fff3cd; border-radius: 4px; }}
                table {{ width: 100%; border-collapse: collapse; margin: 15px 0; }}
                th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
                th {{ background-color: #f2f2f2; }}
                .forecast {{ background: #e7f3ff; padding: 15px; border-radius: 8px; margin: 10px 0; }}
                .trend-improving {{ color: #28a745; }}
                .trend-degrading {{ color: #dc3545; }}
                .trend-stable {{ color: #6c757d; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>📊 CURSED Compiler Performance Analysis</h1>
                <p>Comprehensive analysis for the last {days} days</p>
                <p>Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
            </div>
            
            <div class="summary">
                <div class="metric-card">
                    <div class="metric-value">{total_benchmarks}</div>
                    <div>Benchmarks Analyzed</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{avg_success_rate:.1f}%</div>
                    <div>Average Success Rate</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{critical_regressions}</div>
                    <div>Critical Regressions</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{total_anomalies}</div>
                    <div>Anomalies Detected</div>
                </div>
            </div>
            
            <div class="section">
                <h2>🔍 Performance Metrics Summary</h2>
                <table>
                    <tr>
                        <th>Benchmark</th>
                        <th>Avg Compile Time (ms)</th>
                        <th>Success Rate (%)</th>
                        <th>Volatility</th>
                        <th>Performance Score</th>
                        <th>Trend</th>
                    </tr>
        """
        
        # Add metrics table rows
        for metric in sorted(metrics, key=lambda x: x.mean_compilation_time):
            trend_icon = "📈" if metric.trend_slope > 1 else "📉" if metric.trend_slope < -1 else "➡️"
            html += f"""
                    <tr>
                        <td>{metric.benchmark_name}</td>
                        <td>{metric.mean_compilation_time:.1f} ± {metric.std_compilation_time:.1f}</td>
                        <td>{metric.success_rate * 100:.1f}%</td>
                        <td>{metric.volatility:.3f}</td>
                        <td>{metric.performance_score:.1f}</td>
                        <td>{trend_icon} {metric.trend_slope:.2f}</td>
                    </tr>
            """
        
        html += """
                </table>
            </div>
        """
        
        # Add regressions section
        if regressions:
            html += """
            <div class="section">
                <h2>⚠️ Performance Regressions Detected</h2>
            """
            
            for regression in regressions:
                severity = "CRITICAL" if regression.regression_magnitude > 25 else "HIGH"
                html += f"""
                <div class="regression-alert">
                    <strong>{severity}</strong> - {regression.benchmark_name} ({regression.metric})<br>
                    Regression: {regression.regression_magnitude:.1f}% increase<br>
                    Confidence: {regression.confidence * 100:.1f}% (p-value: {regression.p_value:.4f})<br>
                    From {regression.historical_mean:.1f} to {regression.current_mean:.1f}
                </div>
                """
            
            html += "</div>"
        
        # Add forecasts section
        if forecasts:
            html += """
            <div class="section">
                <h2>🔮 Performance Forecasts</h2>
            """
            
            for forecast in forecasts:
                trend_class = f"trend-{forecast.trend_direction}"
                html += f"""
                <div class="forecast">
                    <strong>{forecast.benchmark_name}</strong><br>
                    <span class="{trend_class}">Trend: {forecast.trend_direction.upper()}</span><br>
                    Model Accuracy: {forecast.model_accuracy:.3f}<br>
                    7-day forecast: {forecast.forecast_values[-1]:.1f}ms 
                    (±{(forecast.confidence_upper[-1] - forecast.confidence_lower[-1])/2:.1f}ms)
                </div>
                """
            
            html += "</div>"
        
        # Add anomalies section
        if not anomalies.empty:
            html += f"""
            <div class="section">
                <h2>🚨 Anomalies Detected ({len(anomalies)} total)</h2>
            """
            
            for _, anomaly in anomalies.head(10).iterrows():
                html += f"""
                <div class="anomaly-alert">
                    <strong>{anomaly['benchmark_name']}</strong><br>
                    Time: {anomaly['timestamp']}<br>
                    Compile: {anomaly['compilation_time_ms']:.1f}ms, 
                    Execute: {anomaly['execution_time_ms']:.1f}ms<br>
                    Anomaly Score: {anomaly.get('anomaly_score', 'N/A')}
                </div>
                """
            
            if len(anomalies) > 10:
                html += f"<p>... and {len(anomalies) - 10} more anomalies</p>"
            
            html += "</div>"
        
        html += """
            <div class="section">
                <h2>📈 Performance Visualizations</h2>
                <img src="performance_analysis_plots.png" alt="Performance Analysis Plots" 
                     style="max-width: 100%; height: auto;">
            </div>
            
            <div class="section">
                <h2>💡 Recommendations</h2>
        """
        
        # Generate recommendations
        recommendations = []
        
        if critical_regressions > 0:
            recommendations.append("🚨 Address critical performance regressions immediately")
        
        if avg_success_rate < 95:
            recommendations.append("🔧 Improve benchmark reliability - success rate below 95%")
        
        high_volatility_benchmarks = [m for m in metrics if m.volatility > 0.2]
        if high_volatility_benchmarks:
            recommendations.append(f"📊 Investigate high volatility in {len(high_volatility_benchmarks)} benchmarks")
        
        slow_benchmarks = [m for m in metrics if m.mean_compilation_time > 2000]
        if slow_benchmarks:
            recommendations.append(f"⚡ Optimize {len(slow_benchmarks)} slow benchmarks (>2s compilation)")
        
        if not recommendations:
            recommendations.append("✅ All metrics within acceptable ranges - maintain current performance")
        
        for rec in recommendations:
            html += f"<p>{rec}</p>"
        
        html += """
            </div>
        </body>
        </html>
        """
        
        return html

def main():
    """Main entry point for analysis tools"""
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Benchmark Analysis Tools")
    parser.add_argument("--database", default="benchmark_results.db", help="Path to benchmark database")
    parser.add_argument("--days", type=int, default=30, help="Days of data to analyze")
    parser.add_argument("--output", default="performance_analysis.html", help="Output report path")
    parser.add_argument("--mode", choices=["report", "metrics", "regressions", "anomalies", "forecast"],
                       default="report", help="Analysis mode")
    
    args = parser.parse_args()
    
    if not ANALYSIS_AVAILABLE:
        print("Error: Required analysis libraries not available")
        print("Install with: pip install matplotlib seaborn scikit-learn pandas")
        return
    
    analyzer = BenchmarkAnalyzer(args.database)
    
    if args.mode == "report":
        report_path = analyzer.generate_performance_report(args.output, args.days)
        print(f"Performance analysis report generated: {report_path}")
    
    elif args.mode == "metrics":
        df = analyzer.load_data(args.days)
        metrics = analyzer.calculate_performance_metrics(df)
        for metric in metrics:
            print(f"{metric.benchmark_name}: {metric.mean_compilation_time:.1f}ms "
                  f"(success: {metric.success_rate*100:.1f}%)")
    
    elif args.mode == "regressions":
        df = analyzer.load_data(args.days)
        regressions = analyzer.detect_regressions(df)
        for regression in regressions:
            print(f"REGRESSION: {regression.benchmark_name} {regression.metric} "
                  f"({regression.regression_magnitude:.1f}% increase)")
    
    elif args.mode == "anomalies":
        df = analyzer.load_data(args.days)
        anomalies = analyzer.detect_anomalies(df)
        print(f"Detected {len(anomalies)} anomalies")
        for _, anomaly in anomalies.head(10).iterrows():
            print(f"  {anomaly['benchmark_name']}: {anomaly['compilation_time_ms']:.1f}ms")
    
    elif args.mode == "forecast":
        df = analyzer.load_data(args.days)
        forecasts = analyzer.forecast_performance(df)
        for forecast in forecasts:
            print(f"{forecast.benchmark_name}: {forecast.trend_direction} trend, "
                  f"7-day forecast: {forecast.forecast_values[-1]:.1f}ms")

if __name__ == "__main__":
    main()
