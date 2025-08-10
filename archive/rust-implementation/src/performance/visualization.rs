//! Performance visualization and reporting tools
//! 
//! Generates charts, graphs, and visual reports for performance monitoring data
//! including real-time dashboards and historical trend analysis.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::error::CursedError;
use crate::performance::{PerformanceConfig, PerformanceData, PerformanceMetrics};
use std::time::Instant;

/// Performance visualization system
pub struct PerformanceVisualizer {
    config: PerformanceConfig,
    chart_templates: HashMap<String, ChartTemplate>,
}

/// Chart template for different visualizations
#[derive(Debug, Clone)]
pub struct ChartTemplate {
    pub name: String,
    pub chart_type: ChartType,
    pub title: String,
    pub x_axis_label: String,
    pub y_axis_label: String,
    pub color_scheme: ColorScheme,
    pub dimensions: ChartDimensions,
}

/// Types of charts available
#[derive(Debug, Clone)]
pub enum ChartType {
    LineChart,
    BarChart,
    ScatterPlot,
    Histogram,
    HeatMap,
    PieChart,
    AreaChart,
    BoxPlot,
}

/// Color schemes for charts
#[derive(Debug, Clone)]
pub enum ColorScheme {
    Default,
    Performance,
    Regression,
    Memory,
    CPU,
    Custom(Vec<String>),
}

/// Chart dimensions
#[derive(Debug, Clone)]
pub struct ChartDimensions {
    pub width: u32,
    pub height: u32,
}

/// Dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    pub title: String,
    pub layout: DashboardLayout,
    pub charts: Vec<ChartConfig>,
    pub refresh_interval: u32,
    pub theme: DashboardTheme,
}

/// Dashboard layout options
#[derive(Debug, Clone)]
pub enum DashboardLayout {
    Grid,
    Vertical,
    Horizontal,
    Custom,
}

/// Chart configuration for dashboard
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub chart_id: String,
    pub chart_type: ChartType,
    pub data_source: String,
    pub position: ChartPosition,
    pub size: ChartSize,
}

/// Chart position in dashboard
#[derive(Debug, Clone)]
pub struct ChartPosition {
    pub row: u32,
    pub column: u32,
}

/// Chart size options
#[derive(Debug, Clone)]
pub enum ChartSize {
    Small,
    Medium,
    Large,
    Custom(u32, u32),
}

/// Dashboard theme
#[derive(Debug, Clone)]
pub enum DashboardTheme {
    Light,
    Dark,
    HighContrast,
    Custom(String),
}

impl PerformanceVisualizer {
    /// Create a new performance visualizer
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        let mut visualizer = Self {
            config,
            chart_templates: HashMap::new(),
        };
        
        visualizer.load_default_templates()?;
        
        Ok(visualizer)
    }

    /// Start the visualization system
    pub fn start(&self) -> Result<(), CursedError> {
        // Create output directory
        fs::create_dir_all(&self.config.output_dir)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create output directory: {}", e)))?;
        
        println!("Performance visualizer started");
        Ok(())
    }

    /// Stop the visualization system
    pub fn stop(&self) -> Result<(), CursedError> {
        println!("Performance visualizer stopped");
        Ok(())
    }

    /// Load default chart templates
    fn load_default_templates(&mut self) -> Result<(), CursedError> {
        // Performance over time chart
        let performance_template = ChartTemplate {
            name: "performance_over_time".to_string(),
            chart_type: ChartType::LineChart,
            title: "Performance Over Time".to_string(),
            x_axis_label: "Time".to_string(),
            y_axis_label: "Performance (ms)".to_string(),
            color_scheme: ColorScheme::Performance,
            dimensions: ChartDimensions { width: 800, height: 400 },
        };
        
        // Memory usage chart
        let memory_template = ChartTemplate {
            name: "memory_usage".to_string(),
            chart_type: ChartType::AreaChart,
            title: "Memory Usage".to_string(),
            x_axis_label: "Time".to_string(),
            y_axis_label: "Memory (MB)".to_string(),
            color_scheme: ColorScheme::Memory,
            dimensions: ChartDimensions { width: 800, height: 400 },
        };
        
        // CPU usage chart
        let cpu_template = ChartTemplate {
            name: "cpu_usage".to_string(),
            chart_type: ChartType::LineChart,
            title: "CPU Usage".to_string(),
            x_axis_label: "Time".to_string(),
            y_axis_label: "CPU (%)".to_string(),
            color_scheme: ColorScheme::CPU,
            dimensions: ChartDimensions { width: 800, height: 400 },
        };
        
        // Throughput histogram
        let throughput_template = ChartTemplate {
            name: "throughput_histogram".to_string(),
            chart_type: ChartType::Histogram,
            title: "Throughput Distribution".to_string(),
            x_axis_label: "Throughput (ops/sec)".to_string(),
            y_axis_label: "Frequency".to_string(),
            color_scheme: ColorScheme::Default,
            dimensions: ChartDimensions { width: 600, height: 400 },
        };
        
        // Latency box plot
        let latency_template = ChartTemplate {
            name: "latency_boxplot".to_string(),
            chart_type: ChartType::BoxPlot,
            title: "Latency Distribution".to_string(),
            x_axis_label: "Operation".to_string(),
            y_axis_label: "Latency (ms)".to_string(),
            color_scheme: ColorScheme::Default,
            dimensions: ChartDimensions { width: 600, height: 400 },
        };
        
        self.chart_templates.insert("performance_over_time".to_string(), performance_template);
        self.chart_templates.insert("memory_usage".to_string(), memory_template);
        self.chart_templates.insert("cpu_usage".to_string(), cpu_template);
        self.chart_templates.insert("throughput_histogram".to_string(), throughput_template);
        self.chart_templates.insert("latency_boxplot".to_string(), latency_template);
        
        Ok(())
    }

    /// Create visualization for performance data
    pub fn create_visualization(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let output_file = format!("{}/performance_visualization.html", self.config.output_dir);
        
        let html = self.generate_html_dashboard(data)?;
        
        fs::write(&output_file, html)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write visualization: {}", e)))?;
        
        println!("Visualization created: {}", output_file);
        Ok(output_file)
    }

    /// Generate HTML dashboard
    fn generate_html_dashboard(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let mut html = String::new();
        
        // HTML head
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<title>CURSED Compiler Performance Dashboard</title>\n");
        html.push_str("<script src=\"https://cdn.plot.ly/plotly-latest.min.js\"></script>\n");
        html.push_str("<style>\n");
        html.push_str(&self.generate_dashboard_css());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        // Header
        html.push_str("<header>\n");
        html.push_str("<h1>CURSED Compiler Performance Dashboard</h1>\n");
        html.push_str(&format!("<p>Generated: {}</p>\n", chrono::Utc::now().to_rfc3339()));
        html.push_str("</header>\n");
        
        // Main content
        html.push_str("<main>\n");
        
        // Performance metrics summary
        html.push_str("<section class=\"summary\">\n");
        html.push_str("<h2>Performance Summary</h2>\n");
        html.push_str("<div class=\"metrics-grid\">\n");
        
        if let Some(latest_metrics) = data.metrics.last() {
            html.push_str(&format!("<div class=\"metric-card\">\n"));
            html.push_str(&format!("<h3>Compilation Time</h3>\n"));
            html.push_str(&format!("<p class=\"metric-value\">{:?}</p>\n", latest_metrics.compilation_time));
            html.push_str("</div>\n");
            
            html.push_str(&format!("<div class=\"metric-card\">\n"));
            html.push_str(&format!("<h3>Execution Time</h3>\n"));
            html.push_str(&format!("<p class=\"metric-value\">{:?}</p>\n", latest_metrics.execution_time));
            html.push_str("</div>\n");
            
            html.push_str(&format!("<div class=\"metric-card\">\n"));
            html.push_str(&format!("<h3>Memory Usage</h3>\n"));
            html.push_str(&format!("<p class=\"metric-value\">{:.2} MB</p>\n", latest_metrics.memory_usage as f64 / 1024.0 / 1024.0));
            html.push_str("</div>\n");
            
            html.push_str(&format!("<div class=\"metric-card\">\n"));
            html.push_str(&format!("<h3>CPU Usage</h3>\n"));
            html.push_str(&format!("<p class=\"metric-value\">{:.1}%</p>\n", latest_metrics.cpu_usage));
            html.push_str("</div>\n");
            
            html.push_str(&format!("<div class=\"metric-card\">\n"));
            html.push_str(&format!("<h3>Throughput</h3>\n"));
            html.push_str(&format!("<p class=\"metric-value\">{:.2} ops/sec</p>\n", latest_metrics.throughput));
            html.push_str("</div>\n");
            
            html.push_str(&format!("<div class=\"metric-card\">\n"));
            html.push_str(&format!("<h3>Error Rate</h3>\n"));
            html.push_str(&format!("<p class=\"metric-value\">{:.3}%</p>\n", latest_metrics.error_rate * 100.0));
            html.push_str("</div>\n");
        }
        
        html.push_str("</div>\n");
        html.push_str("</section>\n");
        
        // Charts section
        html.push_str("<section class=\"charts\">\n");
        html.push_str("<h2>Performance Charts</h2>\n");
        html.push_str("<div class=\"charts-grid\">\n");
        
        // Performance over time chart
        html.push_str("<div class=\"chart-container\">\n");
        html.push_str("<div id=\"performance-chart\"></div>\n");
        html.push_str("</div>\n");
        
        // Memory usage chart
        html.push_str("<div class=\"chart-container\">\n");
        html.push_str("<div id=\"memory-chart\"></div>\n");
        html.push_str("</div>\n");
        
        // CPU usage chart
        html.push_str("<div class=\"chart-container\">\n");
        html.push_str("<div id=\"cpu-chart\"></div>\n");
        html.push_str("</div>\n");
        
        // Throughput chart
        html.push_str("<div class=\"chart-container\">\n");
        html.push_str("<div id=\"throughput-chart\"></div>\n");
        html.push_str("</div>\n");
        
        html.push_str("</div>\n");
        html.push_str("</section>\n");
        
        html.push_str("</main>\n");
        
        // JavaScript for charts
        html.push_str("<script>\n");
        html.push_str(&self.generate_chart_javascript(data)?);
        html.push_str("</script>\n");
        
        html.push_str("</body>\n</html>\n");
        
        Ok(html)
    }

    /// Generate CSS for dashboard
    fn generate_dashboard_css(&self) -> String {
        r#"
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: #f5f5f5;
            color: #333;
            line-height: 1.6;
        }
        
        header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 2rem 0;
            text-align: center;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        
        header h1 {
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
        }
        
        main {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }
        
        .summary {
            margin-bottom: 3rem;
        }
        
        .summary h2 {
            color: #333;
            margin-bottom: 1rem;
            font-size: 1.8rem;
        }
        
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }
        
        .metric-card {
            background: white;
            padding: 1.5rem;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            text-align: center;
            transition: transform 0.2s;
        }
        
        .metric-card:hover {
            transform: translateY(-2px);
        }
        
        .metric-card h3 {
            color: #666;
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 0.5rem;
        }
        
        .metric-value {
            font-size: 2rem;
            font-weight: bold;
            color: #667eea;
        }
        
        .charts h2 {
            color: #333;
            margin-bottom: 1rem;
            font-size: 1.8rem;
        }
        
        .charts-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
            gap: 2rem;
        }
        
        .chart-container {
            background: white;
            padding: 1rem;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        
        .chart-container div {
            min-height: 300px;
        }
        
        @media (max-width: 768px) {
            .metrics-grid {
                grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            }
            
            .charts-grid {
                grid-template-columns: 1fr;
            }
            
            header h1 {
                font-size: 2rem;
            }
        }
        "#.to_string()
    }

    /// Generate JavaScript for charts
    fn generate_chart_javascript(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let mut js = String::new();
        
        // Prepare data for charts
        let timestamps: Vec<String> = data.timestamps.iter()
            .map(|t| format!("{:?}", t))
            .collect();
        
        let compilation_times: Vec<f64> = data.metrics.iter()
            .map(|m| m.compilation_time.as_millis() as f64)
            .collect();
        
        let execution_times: Vec<f64> = data.metrics.iter()
            .map(|m| m.execution_time.as_millis() as f64)
            .collect();
        
        let memory_usage: Vec<f64> = data.metrics.iter()
            .map(|m| m.memory_usage as f64 / 1024.0 / 1024.0)
            .collect();
        
        let cpu_usage: Vec<f64> = data.metrics.iter()
            .map(|m| m.cpu_usage)
            .collect();
        
        let throughput: Vec<f64> = data.metrics.iter()
            .map(|m| m.throughput)
            .collect();
        
        // Performance over time chart
        js.push_str(&format!(r#"
        var performanceData = [
            {{
                x: {},
                y: {},
                type: 'scatter',
                mode: 'lines+markers',
                name: 'Compilation Time',
                line: {{color: '#667eea'}},
                marker: {{size: 6}}
            }},
            {{
                x: {},
                y: {},
                type: 'scatter',
                mode: 'lines+markers',
                name: 'Execution Time',
                line: {{color: '#764ba2'}},
                marker: {{size: 6}}
            }}
        ];
        
        var performanceLayout = {{
            title: 'Performance Over Time',
            xaxis: {{title: 'Time'}},
            yaxis: {{title: 'Time (ms)'}},
            showlegend: true,
            hovermode: 'closest'
        }};
        
        Plotly.newPlot('performance-chart', performanceData, performanceLayout);
        "#, 
        format!("{:?}", timestamps),
        format!("{:?}", compilation_times),
        format!("{:?}", timestamps),
        format!("{:?}", execution_times)
        ));
        
        // Memory usage chart
        js.push_str(&format!(r#"
        var memoryData = [
            {{
                x: {},
                y: {},
                type: 'scatter',
                mode: 'lines',
                fill: 'tozeroy',
                name: 'Memory Usage',
                line: {{color: '#ff6b6b'}},
                fillcolor: 'rgba(255, 107, 107, 0.3)'
            }}
        ];
        
        var memoryLayout = {{
            title: 'Memory Usage',
            xaxis: {{title: 'Time'}},
            yaxis: {{title: 'Memory (MB)'}},
            showlegend: false
        }};
        
        Plotly.newPlot('memory-chart', memoryData, memoryLayout);
        "#, 
        format!("{:?}", timestamps),
        format!("{:?}", memory_usage)
        ));
        
        // CPU usage chart
        js.push_str(&format!(r#"
        var cpuData = [
            {{
                x: {},
                y: {},
                type: 'scatter',
                mode: 'lines+markers',
                name: 'CPU Usage',
                line: {{color: '#4ecdc4'}},
                marker: {{size: 4}}
            }}
        ];
        
        var cpuLayout = {{
            title: 'CPU Usage',
            xaxis: {{title: 'Time'}},
            yaxis: {{title: 'CPU (%)'}},
            showlegend: false
        }};
        
        Plotly.newPlot('cpu-chart', cpuData, cpuLayout);
        "#, 
        format!("{:?}", timestamps),
        format!("{:?}", cpu_usage)
        ));
        
        // Throughput chart
        js.push_str(&format!(r#"
        var throughputData = [
            {{
                x: {},
                y: {},
                type: 'bar',
                name: 'Throughput',
                marker: {{color: '#45b7d1'}}
            }}
        ];
        
        var throughputLayout = {{
            title: 'Throughput',
            xaxis: {{title: 'Time'}},
            yaxis: {{title: 'Throughput (ops/sec)'}},
            showlegend: false
        }};
        
        Plotly.newPlot('throughput-chart', throughputData, throughputLayout);
        "#, 
        format!("{:?}", timestamps),
        format!("{:?}", throughput)
        ));
        
        // Auto-refresh functionality
        js.push_str(r#"
        // Auto-refresh every 30 seconds
        setInterval(function() {
            location.reload();
        }, 30000);
        
        // Add responsive behavior
        window.addEventListener('resize', function() {
            Plotly.Plots.resize('performance-chart');
            Plotly.Plots.resize('memory-chart');
            Plotly.Plots.resize('cpu-chart');
            Plotly.Plots.resize('throughput-chart');
        });
        "#);
        
        Ok(js)
    }

    /// Create real-time dashboard
    pub fn create_realtime_dashboard(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let output_file = format!("{}/realtime_dashboard.html", self.config.output_dir);
        
        let html = self.generate_realtime_dashboard_html(data)?;
        
        fs::write(&output_file, html)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write real-time dashboard: {}", e)))?;
        
        println!("Real-time dashboard created: {}", output_file);
        Ok(output_file)
    }

    /// Generate real-time dashboard HTML
    fn generate_realtime_dashboard_html(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let mut html = String::new();
        
        // Add WebSocket support for real-time updates
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<title>CURSED Compiler Real-Time Performance Dashboard</title>\n");
        html.push_str("<script src=\"https://cdn.plot.ly/plotly-latest.min.js\"></script>\n");
        html.push_str("<style>\n");
        html.push_str(&self.generate_dashboard_css());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        // Header with live indicator
        html.push_str("<header>\n");
        html.push_str("<h1>CURSED Compiler Real-Time Performance Dashboard</h1>\n");
        html.push_str("<div class=\"live-indicator\">\n");
        html.push_str("<span class=\"live-dot\"></span>\n");
        html.push_str("<span>LIVE</span>\n");
        html.push_str("</div>\n");
        html.push_str("</header>\n");
        
        // Main content (similar to regular dashboard)
        html.push_str("<main>\n");
        html.push_str(&self.generate_dashboard_content(data)?);
        html.push_str("</main>\n");
        
        // JavaScript for real-time updates
        html.push_str("<script>\n");
        html.push_str(&self.generate_realtime_javascript(data)?);
        html.push_str("</script>\n");
        
        html.push_str("</body>\n</html>\n");
        
        Ok(html)
    }

    /// Generate dashboard content
    fn generate_dashboard_content(&self, data: &PerformanceData) -> Result<String, CursedError> {
        // This would include the main dashboard sections
        // For brevity, returning a placeholder
        Ok("<p>Dashboard content would be generated here</p>".to_string())
    }

    /// Generate real-time JavaScript
    fn generate_realtime_javascript(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let mut js = String::new();
        
        // Add real-time update logic
        js.push_str(r#"
        // Real-time update interval
        const UPDATE_INTERVAL = 5000; // 5 seconds
        let updateCounter = 0;
        
        // Initialize charts
        initializeCharts();
        
        // Start real-time updates
        setInterval(updateCharts, UPDATE_INTERVAL);
        
        function initializeCharts() {
            // Initialize all charts with current data
            console.log('Initializing charts...');
        }
        
        function updateCharts() {
            // Simulate real-time data updates
            updateCounter++;
            console.log('Updating charts... #' + updateCounter);
            
            // In a real implementation, this would fetch new data from the server
            // For now, we'll simulate updates
            addRandomDataPoint();
        }
        
        function addRandomDataPoint() {
            // Add random data point to demonstrate real-time updates
            const timestamp = new Date().toISOString();
            const randomValue = Math.random() * 100;
            
            // This would update the actual charts with new data
            console.log('New data point:', timestamp, randomValue);
        }
        
        // Add status indicator updates
        function updateStatus() {
            const statusElement = document.querySelector('.live-indicator');
            if (statusElement) {
                statusElement.classList.add('pulse');
                setTimeout(() => {
                    statusElement.classList.remove('pulse');
                }, 500);
            }
        }
        
        // Update status every time we refresh data
        setInterval(updateStatus, UPDATE_INTERVAL);
        "#);
        
        Ok(js)
    }

    /// Export visualization data
    pub fn export_data(&self, data: &PerformanceData, format: &str) -> Result<String, CursedError> {
        let output_file = format!("{}/performance_data.{}", self.config.output_dir, format);
        
        let exported_data = match format {
            "json" => format!("{:#?}", data),
            "csv" => self.export_to_csv(data)?,
            _ => return Err(CursedError::runtime_error(&format!("Unsupported export format: {}", format))),
        };
        
        fs::write(&output_file, exported_data)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to export data: {}", e)))?;
        
        println!("Data exported to: {}", output_file);
        Ok(output_file)
    }

    /// Export data to CSV format
    fn export_to_csv(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let mut csv = String::new();
        
        // CSV header
        csv.push_str("timestamp,compilation_time_ms,execution_time_ms,memory_usage_mb,cpu_usage_percent,throughput_ops_per_sec,latency_ms,error_rate_percent,gc_pressure_percent\n");
        
        // CSV data rows
        for (i, metrics) in data.metrics.iter().enumerate() {
            let timestamp = if i < data.timestamps.len() {
                format!("{:?}", data.timestamps[i])
            } else {
                "unknown".to_string()
            };
            
            csv.push_str(&format!(
                "{},{},{},{:.2},{:.2},{:.2},{},{:.4},{:.2}\n",
                timestamp,
                metrics.compilation_time.as_millis(),
                metrics.execution_time.as_millis(),
                metrics.memory_usage as f64 / 1024.0 / 1024.0,
                metrics.cpu_usage,
                metrics.throughput,
                metrics.latency.as_millis(),
                metrics.error_rate * 100.0,
                metrics.gc_pressure * 100.0
            ));
        }
        
        Ok(csv)
    }

    /// Get available chart templates
    pub fn get_chart_templates(&self) -> &HashMap<String, ChartTemplate> {
        &self.chart_templates
    }

    /// Add custom chart template
    pub fn add_chart_template(&mut self, template: ChartTemplate) {
        self.chart_templates.insert(template.name.clone(), template);
    }

    /// Generate performance report
    pub fn generate_performance_report(&self, data: &PerformanceData) -> Result<String, CursedError> {
        let output_file = format!("{}/performance_report.html", self.config.output_dir);
        
        let html = self.generate_html_dashboard(data)?;
        
        fs::write(&output_file, html)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write performance report: {}", e)))?;
        
        println!("Performance report generated: {}", output_file);
        Ok(output_file)
    }
}
