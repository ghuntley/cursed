#!/usr/bin/env python3
"""
CURSED Compiler Benchmark Dashboard

A web-based dashboard for visualizing benchmark results and performance trends.
Provides real-time monitoring, historical analysis, and regression detection.
"""

import sqlite3
import json
from datetime import datetime, timedelta
from typing import List, Dict, Any
from pathlib import Path

try:
    from flask import Flask, render_template, jsonify, request
    import plotly.graph_objs as go
    import plotly.utils
    import pandas as pd
    FLASK_AVAILABLE = True
except ImportError:
    FLASK_AVAILABLE = False
    print("Flask and plotly not available. Install with: pip install flask plotly pandas")

class BenchmarkDashboard:
    """Web dashboard for benchmark visualization"""
    
    def __init__(self, database_path: str = "benchmark_results.db"):
        self.database_path = database_path
        if FLASK_AVAILABLE:
            self.app = Flask(__name__)
            self._setup_routes()
    
    def _setup_routes(self):
        """Setup Flask routes"""
        
        @self.app.route('/')
        def index():
            return self._render_dashboard()
        
        @self.app.route('/api/benchmarks')
        def api_benchmarks():
            days = request.args.get('days', 7, type=int)
            return jsonify(self._get_benchmark_data(days))
        
        @self.app.route('/api/alerts')
        def api_alerts():
            hours = request.args.get('hours', 24, type=int)
            return jsonify(self._get_alerts_data(hours))
        
        @self.app.route('/api/trends/<benchmark_name>')
        def api_trends(benchmark_name):
            days = request.args.get('days', 30, type=int)
            metric = request.args.get('metric', 'compilation_time_ms')
            return jsonify(self._get_trend_data(benchmark_name, metric, days))
        
        @self.app.route('/api/performance_chart')
        def api_performance_chart():
            return jsonify(self._generate_performance_chart())
    
    def _render_dashboard(self):
        """Render the main dashboard HTML"""
        return """
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Compiler Benchmark Dashboard</title>
    <script src="https://cdn.plot.ly/plotly-latest.min.js"></script>
    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            margin-bottom: 20px;
        }
        .dashboard-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            margin-bottom: 20px;
        }
        .widget {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 20px;
        }
        .full-width {
            grid-column: 1 / -1;
        }
        .metric-value {
            font-size: 2em;
            font-weight: bold;
            color: #667eea;
        }
        .metric-label {
            color: #666;
            font-size: 0.9em;
            text-transform: uppercase;
        }
        .alert {
            padding: 10px;
            margin: 5px 0;
            border-left: 4px solid;
            border-radius: 4px;
        }
        .alert-critical { border-color: #d32f2f; background: #ffebee; }
        .alert-high { border-color: #f57c00; background: #fff3e0; }
        .alert-medium { border-color: #fbc02d; background: #fffde7; }
        .alert-low { border-color: #388e3c; background: #e8f5e8; }
        .controls {
            margin: 20px 0;
        }
        select, button {
            padding: 8px 12px;
            margin: 0 5px;
            border: 1px solid #ddd;
            border-radius: 4px;
        }
        button {
            background: #667eea;
            color: white;
            cursor: pointer;
        }
        button:hover {
            background: #5a6fd8;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 CURSED Compiler Benchmark Dashboard</h1>
        <p>Real-time performance monitoring and regression detection</p>
        <p id="last-update">Last updated: Loading...</p>
    </div>
    
    <div class="controls">
        <label>Time Range:</label>
        <select id="timeRange" onchange="updateDashboard()">
            <option value="1">Last 24 hours</option>
            <option value="7" selected>Last 7 days</option>
            <option value="30">Last 30 days</option>
        </select>
        
        <label>Benchmark:</label>
        <select id="benchmarkSelect" onchange="updateTrends()">
            <option value="all">All Benchmarks</option>
        </select>
        
        <button onclick="refreshData()">Refresh Data</button>
    </div>
    
    <div class="dashboard-grid">
        <div class="widget">
            <div class="metric-label">Recent Benchmarks</div>
            <div class="metric-value" id="recentBenchmarks">-</div>
        </div>
        
        <div class="widget">
            <div class="metric-label">Active Alerts</div>
            <div class="metric-value" id="activeAlerts">-</div>
        </div>
        
        <div class="widget">
            <div class="metric-label">Avg Compilation Time</div>
            <div class="metric-value" id="avgCompileTime">-</div>
        </div>
        
        <div class="widget">
            <div class="metric-label">Performance Score</div>
            <div class="metric-value" id="performanceScore">-</div>
        </div>
        
        <div class="widget full-width">
            <h3>Performance Trends</h3>
            <div id="performanceChart" style="height: 400px;"></div>
        </div>
        
        <div class="widget">
            <h3>Recent Alerts</h3>
            <div id="alertsList" style="max-height: 300px; overflow-y: auto;">
                Loading alerts...
            </div>
        </div>
        
        <div class="widget">
            <h3>Benchmark Status</h3>
            <div id="benchmarkStatus">
                Loading benchmark status...
            </div>
        </div>
    </div>
    
    <script>
        let currentData = {};
        
        function updateDashboard() {
            const days = document.getElementById('timeRange').value;
            
            // Update metrics
            fetch(`/api/benchmarks?days=${days}`)
                .then(response => response.json())
                .then(data => {
                    currentData = data;
                    updateMetrics(data);
                    updateBenchmarkSelect(data);
                    updateBenchmarkStatus(data);
                });
            
            // Update alerts
            const hours = days * 24;
            fetch(`/api/alerts?hours=${hours}`)
                .then(response => response.json())
                .then(data => updateAlerts(data));
            
            // Update performance chart
            fetch('/api/performance_chart')
                .then(response => response.json())
                .then(data => updatePerformanceChart(data));
            
            document.getElementById('last-update').textContent = 
                'Last updated: ' + new Date().toLocaleString();
        }
        
        function updateMetrics(data) {
            document.getElementById('recentBenchmarks').textContent = data.total_benchmarks || 0;
            document.getElementById('avgCompileTime').textContent = 
                (data.avg_compilation_time || 0).toFixed(1) + 'ms';
            
            // Calculate performance score (inverse of average time, normalized)
            const score = data.avg_compilation_time ? 
                Math.max(0, 100 - (data.avg_compilation_time / 10)).toFixed(0) : '-';
            document.getElementById('performanceScore').textContent = score + '%';
        }
        
        function updateAlerts(data) {
            document.getElementById('activeAlerts').textContent = data.length;
            
            const alertsList = document.getElementById('alertsList');
            if (data.length === 0) {
                alertsList.innerHTML = '<p>No recent alerts. All systems performing well! ✅</p>';
                return;
            }
            
            let html = '';
            data.forEach(alert => {
                html += `
                    <div class="alert alert-${alert.severity}">
                        <strong>${alert.severity.toUpperCase()}</strong> - ${alert.benchmark_name}<br>
                        ${alert.metric}: ${alert.regression_percent.toFixed(1)}% regression<br>
                        <small>${new Date(alert.timestamp).toLocaleString()}</small>
                    </div>
                `;
            });
            alertsList.innerHTML = html;
        }
        
        function updateBenchmarkSelect(data) {
            const select = document.getElementById('benchmarkSelect');
            const currentValue = select.value;
            
            // Clear existing options except "All Benchmarks"
            select.innerHTML = '<option value="all">All Benchmarks</option>';
            
            // Add benchmark options
            if (data.benchmarks) {
                data.benchmarks.forEach(benchmark => {
                    const option = document.createElement('option');
                    option.value = benchmark;
                    option.textContent = benchmark;
                    select.appendChild(option);
                });
            }
            
            // Restore previous selection if still valid
            if (currentValue && Array.from(select.options).some(opt => opt.value === currentValue)) {
                select.value = currentValue;
            }
        }
        
        function updateBenchmarkStatus(data) {
            const statusDiv = document.getElementById('benchmarkStatus');
            let html = '';
            
            if (data.benchmark_status) {
                Object.entries(data.benchmark_status).forEach(([name, status]) => {
                    const statusIcon = status.success ? '✅' : '❌';
                    const timeColor = status.compilation_time > 2000 ? 'color: red' : 
                                    status.compilation_time > 1000 ? 'color: orange' : 'color: green';
                    
                    html += `
                        <div style="margin: 5px 0; padding: 5px; border: 1px solid #eee; border-radius: 4px;">
                            ${statusIcon} <strong>${name}</strong><br>
                            <small style="${timeColor}">Compile: ${status.compilation_time.toFixed(1)}ms</small>
                        </div>
                    `;
                });
            } else {
                html = '<p>No benchmark status available</p>';
            }
            
            statusDiv.innerHTML = html;
        }
        
        function updatePerformanceChart(data) {
            if (!data || !data.x || !data.y) {
                document.getElementById('performanceChart').innerHTML = 
                    '<p>No performance data available</p>';
                return;
            }
            
            const trace = {
                x: data.x,
                y: data.y,
                type: 'scatter',
                mode: 'lines+markers',
                name: 'Compilation Time',
                line: { color: '#667eea', width: 2 },
                marker: { size: 6 }
            };
            
            const layout = {
                title: 'Performance Over Time',
                xaxis: { title: 'Time' },
                yaxis: { title: 'Compilation Time (ms)' },
                margin: { l: 50, r: 50, t: 50, b: 50 }
            };
            
            Plotly.newPlot('performanceChart', [trace], layout, {responsive: true});
        }
        
        function updateTrends() {
            const benchmark = document.getElementById('benchmarkSelect').value;
            const days = document.getElementById('timeRange').value;
            
            if (benchmark === 'all') {
                // Show combined trends
                updateDashboard();
                return;
            }
            
            fetch(`/api/trends/${benchmark}?days=${days}&metric=compilation_time_ms`)
                .then(response => response.json())
                .then(data => updatePerformanceChart(data));
        }
        
        function refreshData() {
            updateDashboard();
        }
        
        // Initialize dashboard
        updateDashboard();
        
        // Auto-refresh every 30 seconds
        setInterval(updateDashboard, 30000);
    </script>
</body>
</html>
        """
    
    def _get_benchmark_data(self, days: int) -> Dict[str, Any]:
        """Get benchmark data for the dashboard"""
        try:
            with sqlite3.connect(self.database_path) as conn:
                cutoff_date = datetime.now() - timedelta(days=days)
                
                # Get recent benchmark results
                cursor = conn.execute("""
                    SELECT benchmark_name, compilation_time_ms, execution_time_ms, 
                           success, timestamp
                    FROM benchmark_results
                    WHERE timestamp > ? AND success = 1
                    ORDER BY timestamp DESC
                """, (cutoff_date.isoformat(),))
                
                results = cursor.fetchall()
                
                if not results:
                    return {
                        "total_benchmarks": 0,
                        "avg_compilation_time": 0,
                        "benchmarks": [],
                        "benchmark_status": {}
                    }
                
                # Calculate statistics
                compilation_times = [row[1] for row in results if row[1] > 0]
                avg_compilation_time = sum(compilation_times) / len(compilation_times) if compilation_times else 0
                
                # Get unique benchmarks
                benchmarks = list(set(row[0] for row in results))
                
                # Get status for each benchmark (most recent result)
                benchmark_status = {}
                for benchmark in benchmarks:
                    cursor = conn.execute("""
                        SELECT compilation_time_ms, execution_time_ms, success
                        FROM benchmark_results
                        WHERE benchmark_name = ?
                        ORDER BY timestamp DESC
                        LIMIT 1
                    """, (benchmark,))
                    
                    row = cursor.fetchone()
                    if row:
                        benchmark_status[benchmark] = {
                            "compilation_time": row[0],
                            "execution_time": row[1],
                            "success": bool(row[2])
                        }
                
                return {
                    "total_benchmarks": len(results),
                    "avg_compilation_time": avg_compilation_time,
                    "benchmarks": benchmarks,
                    "benchmark_status": benchmark_status
                }
                
        except Exception as e:
            print(f"Error getting benchmark data: {e}")
            return {"total_benchmarks": 0, "avg_compilation_time": 0, "benchmarks": []}
    
    def _get_alerts_data(self, hours: int) -> List[Dict[str, Any]]:
        """Get alerts data for the dashboard"""
        try:
            with sqlite3.connect(self.database_path) as conn:
                cutoff_date = datetime.now() - timedelta(hours=hours)
                
                cursor = conn.execute("""
                    SELECT timestamp, benchmark_name, metric, current_value, 
                           baseline_value, regression_percent, confidence, severity
                    FROM regression_alerts
                    WHERE timestamp > ?
                    ORDER BY timestamp DESC
                """, (cutoff_date.isoformat(),))
                
                alerts = []
                for row in cursor.fetchall():
                    alerts.append({
                        "timestamp": row[0],
                        "benchmark_name": row[1],
                        "metric": row[2],
                        "current_value": row[3],
                        "baseline_value": row[4],
                        "regression_percent": row[5],
                        "confidence": row[6],
                        "severity": row[7]
                    })
                
                return alerts
                
        except Exception as e:
            print(f"Error getting alerts data: {e}")
            return []
    
    def _get_trend_data(self, benchmark_name: str, metric: str, days: int) -> Dict[str, Any]:
        """Get trend data for a specific benchmark and metric"""
        try:
            with sqlite3.connect(self.database_path) as conn:
                cutoff_date = datetime.now() - timedelta(days=days)
                
                cursor = conn.execute(f"""
                    SELECT timestamp, {metric}
                    FROM benchmark_results
                    WHERE benchmark_name = ? AND timestamp > ? AND success = 1
                    ORDER BY timestamp
                """, (benchmark_name, cutoff_date.isoformat()))
                
                results = cursor.fetchall()
                
                if not results:
                    return {"x": [], "y": []}
                
                timestamps = [row[0] for row in results]
                values = [row[1] for row in results]
                
                return {"x": timestamps, "y": values}
                
        except Exception as e:
            print(f"Error getting trend data: {e}")
            return {"x": [], "y": []}
    
    def _generate_performance_chart(self) -> Dict[str, Any]:
        """Generate performance chart data"""
        try:
            with sqlite3.connect(self.database_path) as conn:
                # Get recent compilation times across all benchmarks
                cutoff_date = datetime.now() - timedelta(days=7)
                
                cursor = conn.execute("""
                    SELECT timestamp, AVG(compilation_time_ms) as avg_time
                    FROM benchmark_results
                    WHERE timestamp > ? AND success = 1
                    GROUP BY DATE(timestamp)
                    ORDER BY timestamp
                """, (cutoff_date.isoformat(),))
                
                results = cursor.fetchall()
                
                if not results:
                    return {"x": [], "y": []}
                
                timestamps = [row[0] for row in results]
                values = [row[1] for row in results]
                
                return {"x": timestamps, "y": values}
                
        except Exception as e:
            print(f"Error generating performance chart: {e}")
            return {"x": [], "y": []}
    
    def run(self, host: str = "localhost", port: int = 5000, debug: bool = False):
        """Run the dashboard server"""
        if not FLASK_AVAILABLE:
            print("Flask is not available. Install with: pip install flask plotly pandas")
            return
        
        print(f"Starting CURSED Benchmark Dashboard at http://{host}:{port}")
        self.app.run(host=host, port=port, debug=debug)

def main():
    """Main entry point for the dashboard"""
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Compiler Benchmark Dashboard")
    parser.add_argument("--database", default="benchmark_results.db", help="Path to benchmark database")
    parser.add_argument("--host", default="localhost", help="Host to bind to")
    parser.add_argument("--port", type=int, default=5000, help="Port to bind to")
    parser.add_argument("--debug", action="store_true", help="Enable debug mode")
    
    args = parser.parse_args()
    
    dashboard = BenchmarkDashboard(args.database)
    dashboard.run(args.host, args.port, args.debug)

if __name__ == "__main__":
    main()
