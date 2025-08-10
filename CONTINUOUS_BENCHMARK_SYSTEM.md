# CURSED Compiler Continuous Benchmark System

A comprehensive continuous benchmarking system for the CURSED compiler that provides real-time performance monitoring, statistical analysis, regression detection, and automated reporting.

## 🚀 Features

### Core Capabilities
- **Continuous Monitoring**: Automated benchmark execution at configurable intervals
- **Statistical Analysis**: Advanced regression detection using Mann-Whitney U tests and effect size analysis
- **Performance Forecasting**: Time series analysis and trend prediction
- **Anomaly Detection**: Isolation Forest-based outlier detection
- **Real-time Dashboard**: Web-based visualization and monitoring interface
- **Automated Alerts**: Configurable performance regression notifications
- **Historical Analysis**: SQLite-based data storage with comprehensive querying

### Metrics Tracked
- **Compilation Time**: Average, standard deviation, trends
- **Execution Time**: Runtime performance analysis
- **Memory Usage**: Peak memory consumption tracking
- **Binary Size**: Compiled output size monitoring
- **Success Rate**: Compilation and execution reliability
- **Performance Score**: Normalized performance rating

## 📁 System Components

```
continuous_benchmark_harness.py    # Main harness system
benchmark_dashboard.py             # Web-based dashboard
benchmark_analysis_tools.py       # Advanced analysis tools
run_continuous_benchmarks.sh      # Control script
benchmark_config.json             # Configuration file
requirements.txt                   # Python dependencies
```

## 🛠️ Installation & Setup

### 1. Initial Setup
```bash
# Make the control script executable
chmod +x run_continuous_benchmarks.sh

# Run initial setup (installs dependencies, builds compiler)
./run_continuous_benchmarks.sh setup
```

This will:
- Create a Python virtual environment
- Install required dependencies (numpy, scipy, matplotlib, plotly, pandas, etc.)
- Build the CURSED compiler
- Set up benchmark test files
- Create necessary directories

### 2. Python Dependencies
The system requires several Python packages for advanced analysis:
```bash
pip install numpy scipy matplotlib plotly pandas requests flask scikit-learn psutil
```

### 3. Directory Structure
After setup, the following will be created:
```
benchmark_venv/           # Python virtual environment
benchmark_reports/        # Generated reports
benchmark_results.db      # SQLite database
benchmark_harness.log     # System logs
dashboard.log             # Dashboard logs
```

## 🚀 Usage

### Starting the Complete System
```bash
# Start both monitoring and dashboard
./run_continuous_benchmarks.sh start
```

This launches:
- Continuous benchmark monitoring (background process)
- Web dashboard at http://localhost:5000
- Automatic benchmark execution every 30 minutes (configurable)

### Individual Components

#### Monitor Only
```bash
# Start only the benchmark monitoring
./run_continuous_benchmarks.sh monitor
```

#### Dashboard Only
```bash
# Start only the web dashboard
./run_continuous_benchmarks.sh dashboard
```

#### Single Benchmark Run
```bash
# Run one benchmark cycle for testing
./run_continuous_benchmarks.sh single
```

### System Management

#### Check Status
```bash
./run_continuous_benchmarks.sh status
```

Shows:
- Process status (running/stopped)
- Database size and location
- Log file information
- Recent log entries

#### View Logs
```bash
./run_continuous_benchmarks.sh logs
```

#### Generate Reports
```bash
# Generate 7-day performance report
./run_continuous_benchmarks.sh report

# Generate 30-day performance report
./run_continuous_benchmarks.sh report 30
```

#### Stop Services
```bash
./run_continuous_benchmarks.sh stop
```

## ⚙️ Configuration

### Main Configuration (`benchmark_config.json`)

```json
{
  "benchmark_interval_minutes": 30,
  "benchmark_files": [
    "benchmarks/cursed/fasta.csd",
    "benchmarks/cursed/mandelbrot.csd",
    "comprehensive_stdlib_test.csd"
  ],
  "optimization_levels": ["Debug", "ReleaseFast", "ReleaseSmall"],
  "regression_threshold_percent": 10.0,
  "confidence_threshold": 0.95,
  "alert_severity_thresholds": {
    "low": 5.0,
    "medium": 15.0,
    "high": 25.0,
    "critical": 50.0
  }
}
```

### Key Configuration Options

- **benchmark_interval_minutes**: How often to run benchmarks (default: 30)
- **regression_threshold_percent**: Minimum regression to trigger alerts (default: 10%)
- **confidence_threshold**: Statistical confidence level (default: 95%)
- **max_workers**: Parallel benchmark execution threads (default: 4)

### Performance Targets
```json
"performance_targets": {
  "compilation_time_ms": {
    "target": 1000,
    "warning_threshold": 1500,
    "critical_threshold": 3000
  }
}
```

## 📊 Dashboard Features

Access the dashboard at http://localhost:5000

### Real-time Metrics
- **Recent Benchmarks**: Count of benchmarks run
- **Active Alerts**: Current regression alerts
- **Average Compilation Time**: Real-time performance
- **Performance Score**: Normalized performance rating

### Visualizations
- **Performance Trends**: Time series charts
- **Benchmark Status**: Success/failure indicators
- **Alert Timeline**: Recent regression notifications

### Interactive Controls
- **Time Range Selection**: 24 hours, 7 days, 30 days
- **Benchmark Filtering**: View specific benchmarks
- **Auto-refresh**: Updates every 30 seconds

## 🔍 Advanced Analysis

### Statistical Analysis Tools
```bash
# Generate comprehensive analysis report
python3 benchmark_analysis_tools.py --mode report --days 30

# Check for regressions only
python3 benchmark_analysis_tools.py --mode regressions --days 7

# Detect anomalies
python3 benchmark_analysis_tools.py --mode anomalies --days 14

# Performance forecasting
python3 benchmark_analysis_tools.py --mode forecast --days 30
```

### Regression Detection
The system uses advanced statistical methods:
- **Mann-Whitney U Test**: Non-parametric comparison
- **Effect Size Analysis**: Cohen's d for regression magnitude
- **Confidence Intervals**: Configurable statistical confidence
- **Trend Analysis**: Linear regression on time series data

### Anomaly Detection
- **Isolation Forest**: Machine learning-based outlier detection
- **Multi-dimensional Analysis**: Considers compilation time, execution time, and memory usage
- **Adaptive Thresholds**: Self-adjusting based on historical data

## 📈 Metrics & Alerts

### Performance Metrics
1. **Compilation Time**: Mean, standard deviation, trend slope
2. **Execution Time**: Runtime performance analysis
3. **Memory Usage**: Peak memory consumption
4. **Binary Size**: Compiled output size
5. **Success Rate**: Reliability percentage
6. **Volatility**: Coefficient of variation
7. **Performance Score**: Normalized rating (0-100)

### Alert Severity Levels
- **Low (5-15% regression)**: Minor performance degradation
- **Medium (15-25% regression)**: Moderate performance impact
- **High (25-50% regression)**: Significant performance loss
- **Critical (>50% regression)**: Severe performance regression

### Alert Conditions
Alerts are triggered when:
1. Performance regression exceeds threshold (default: 10%)
2. Statistical confidence is above threshold (default: 95%)
3. Sufficient historical data exists (minimum 10 samples)
4. Effect size indicates meaningful change

## 🔧 Troubleshooting

### Common Issues

#### Benchmark Harness Won't Start
```bash
# Check Python environment
source benchmark_venv/bin/activate
python3 --version

# Check dependencies
pip list | grep numpy

# Rebuild compiler
zig build

# Check logs
tail -20 benchmark_harness.log
```

#### Dashboard Not Accessible
```bash
# Check if dashboard is running
pgrep -f benchmark_dashboard.py

# Check port availability
netstat -tuln | grep 5000

# View dashboard logs
tail -20 dashboard.log
```

#### No Benchmark Data
```bash
# Run single benchmark manually
./run_continuous_benchmarks.sh single

# Check benchmark files exist
ls -la benchmarks/cursed/

# Check database
sqlite3 benchmark_results.db "SELECT COUNT(*) FROM benchmark_results;"
```

#### Memory Issues
```bash
# Check system resources
free -h
ps aux | grep cursed

# Reduce max_workers in config
# Increase benchmark_interval_minutes
```

### Log Files
- **benchmark_harness.log**: Main system events and errors
- **dashboard.log**: Web dashboard events
- **benchmark_results.db**: SQLite database with all data

## 🧪 Testing

### Verify Installation
```bash
# Test single benchmark cycle
./run_continuous_benchmarks.sh single

# Check database was created
ls -la benchmark_results.db

# Test dashboard (should show data)
./run_continuous_benchmarks.sh dashboard
```

### Manual Testing
```bash
# Run specific analysis
python3 benchmark_analysis_tools.py --mode metrics --days 1

# Test compiler build
zig build && ./zig-out/bin/cursed-zig --version

# Test benchmark file
echo 'vibez.spill("test")' > test.csd
./zig-out/bin/cursed-zig test.csd
```

## 📚 API Reference

### Database Schema
```sql
-- Benchmark results table
CREATE TABLE benchmark_results (
    id INTEGER PRIMARY KEY,
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
);

-- Regression alerts table
CREATE TABLE regression_alerts (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    benchmark_name TEXT NOT NULL,
    metric TEXT NOT NULL,
    current_value REAL NOT NULL,
    baseline_value REAL NOT NULL,
    regression_percent REAL NOT NULL,
    confidence REAL NOT NULL,
    severity TEXT NOT NULL
);
```

### REST API Endpoints
- `GET /api/benchmarks?days=N`: Get benchmark data
- `GET /api/alerts?hours=N`: Get recent alerts
- `GET /api/trends/<benchmark>?days=N&metric=X`: Get trend data
- `GET /api/performance_chart`: Get performance chart data

## 🔮 Future Enhancements

### Planned Features
1. **Email/Slack Notifications**: Automated alert delivery
2. **Multi-platform Testing**: Cross-compilation benchmark support
3. **Performance Budgets**: Configurable performance limits
4. **A/B Testing**: Compare different compiler versions
5. **Machine Learning**: Predictive performance modeling
6. **Integration**: CI/CD pipeline integration
7. **Mobile Dashboard**: Responsive mobile interface

### Extensibility
The system is designed for easy extension:
- **Custom Metrics**: Add new performance measurements
- **Alert Channels**: Implement additional notification methods
- **Analysis Algorithms**: Add new statistical methods
- **Visualization**: Create custom charts and reports

## 🤝 Contributing

### Adding New Benchmarks
1. Place `.csd` files in `benchmarks/cursed/` directory
2. Add to `benchmark_files` list in `benchmark_config.json`
3. Restart the monitoring system

### Custom Analysis
Extend `BenchmarkAnalyzer` class in `benchmark_analysis_tools.py`:
```python
def custom_analysis(self, df: pd.DataFrame) -> Any:
    # Your custom analysis logic
    pass
```

### Dashboard Customization
Modify `benchmark_dashboard.py` to add new visualizations or metrics.

## 📄 License

This continuous benchmark system is part of the CURSED compiler project and follows the same licensing terms.

---

**🚀 Happy Benchmarking!**

For issues, questions, or contributions, please refer to the main CURSED compiler documentation or create an issue in the project repository.
