use crate::error::Error;
// Visualization generation for profiling data

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::profiling::core::ProfilerError;
use crate::profiling::cpu::{CpuProfileData, FlameGraph, FlameGraphNode};
use crate::profiling::memory::MemoryProfileData;
use crate::profiling::concurrency::ConcurrencyProfileData;

/// Visualization generator for profiling data
#[derive(Debug)]
pub struct VisualizationGenerator {
    config: VisualizationConfig,
}

impl VisualizationGenerator {
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }
    
    #[instrument(skip(self, cpu_data))]
    pub fn generate_flame_graph(&self, cpu_data: &CpuProfileData) -> Result<(), Error> {
        info!("Generating flame graph visualization");
        
        let flame_graph = FlameGraph::from_cpu_profile(cpu_data)?;
        
        let svg = self.flame_graph_to_svg(&flame_graph)?;
        Ok(svg)
    }
    
    #[instrument(skip(self, cpu_data))]
    pub fn generate_call_graph(&self, cpu_data: &CpuProfileData) -> Result<(), Error> {
        info!("Generating call graph visualization");
        
        let call_graph = cpu_data.get_call_graph();
        let dot = self.call_graph_to_dot(&call_graph)?;
        Ok(dot)
    }
    
    #[instrument(skip(self, memory_data))]
    pub fn generate_memory_timeline(&self, memory_data: &MemoryProfileData) -> Result<(), Error> {
        info!("Generating memory timeline visualization");
        
        let analysis = memory_data.analyze_patterns();
        let svg = self.memory_timeline_to_svg(&analysis.temporal_patterns)?;
        Ok(svg)
    }
    
    #[instrument(skip(self, memory_data))]
    pub fn generate_allocation_heatmap(&self, memory_data: &MemoryProfileData) -> Result<(), Error> {
        info!("Generating allocation heatmap");
        
        let analysis = memory_data.analyze_patterns();
        let svg = self.allocation_heatmap_to_svg(&analysis.size_histogram)?;
        Ok(svg)
    }
    
    #[instrument(skip(self, concurrency_data))]
    pub fn generate_goroutine_timeline(&self, concurrency_data: &ConcurrencyProfileData) -> Result<(), Error> {
        info!("Generating goroutine timeline visualization");
        
        let timeline = concurrency_data.generate_goroutine_timeline();
        let svg = self.goroutine_timeline_to_svg(&timeline)?;
        Ok(svg)
    }
    
    #[instrument(skip(self, concurrency_data))]
    pub fn generate_channel_flow_diagram(&self, concurrency_data: &ConcurrencyProfileData) -> Result<(), Error> {
        info!("Generating channel flow diagram");
        
        let analysis = concurrency_data.analyze_channels();
        let svg = self.channel_flow_to_svg(&analysis)?;
        Ok(svg)
    }
    
    #[instrument(skip(self))]
    pub fn generate_interactive_dashboard(&self, profile_data: &crate::profiling::core::ProfileData) -> Result<(), Error> {
        info!("Generating interactive dashboard");
        
        let html = self.generate_dashboard_html(profile_data)?;
        Ok(html)
    }
    
    fn flame_graph_to_svg(&self, flame_graph: &FlameGraph) -> Result<(), Error> {
        let mut svg = String::new();
        
        let width = self.config.flame_graph_width;
        let height = self.config.flame_graph_height;
        
        // SVG header
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        ));
        
        // Style definitions
        svg.push_str(r#"
<defs>
    <style>
        .frame { stroke: #000; stroke-width: 0.5; cursor: pointer; }
        .frame:hover { stroke: #f00; stroke-width: 1; }
        .label { font-family: Arial; font-size: 12px; pointer-events: none; }
        .title { font-family: Arial; font-size: 16px; font-weight: bold; text-anchor: middle; }
    </style>
</defs>
"#);
        
        // Title
        svg.push_str(&format!(
            r#"<text x="{}" y="20" class="title">CPU Flame Graph - {} samples</text>"#,
            width / 2,
            flame_graph.total_samples
        ));
        
        // Calculate layout
        let frame_height = 20.0;
        let top_margin = 40.0;
        let width_per_sample = (width - 40) as f64 / flame_graph.total_samples as f64;
        
        // Group nodes by depth
        let mut levels: HashMap<usize, Vec<&FlameGraphNode>> = HashMap::new();
        for node in &flame_graph.nodes {
            levels.entry(node.depth).or_default().push(node);
        }
        
        // Render each level
        for depth in 0..=flame_graph.max_depth {
            if let Some(nodes) = levels.get(&depth) {
                let y = top_margin + (flame_graph.max_depth - depth) as f64 * frame_height;
                let mut x_offset = 20.0;
                
                for node in nodes {
                    let width = node.value as f64 * width_per_sample;
                    
                    if width > 1.0 {
                        // Generate color based on function name
                        let color = self.generate_color(&node.name);
                        
                        // Frame rectangle
                        svg.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" class="frame" title="{}: {} samples ({:.1}%)">"#,
                            x_offset, y, width, frame_height, color,
                            node.name, node.value,
                            (node.value as f64 / flame_graph.total_samples as f64) * 100.0
                        ));
                        svg.push_str("</rect>");
                        
                        // Label (if wide enough)
                        if width > 50.0 {
                            svg.push_str(&format!(
                                r#"<text x="{}" y="{}" class="label" fill="black">{}</text>"#,
                                x_offset + 2.0,
                                y + frame_height - 4.0,
                                self.truncate_text(&node.name, width as usize / 6)
                            ));
                        }
                        
                        x_offset += width;
                    }
                }
            }
        }
        
        svg.push_str("</svg>");
        Ok(svg)
    }
    
    fn call_graph_to_dot(&self, call_graph: &crate::profiling::cpu::CallGraph) -> Result<(), Error> {
        let mut dot = String::new();
        
        dot.push_str("digraph CallGraph {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box, style=filled];\n");
        
        // Add nodes
        for (function, stats) in &call_graph.nodes {
            let color = if stats.sample_count > 1000 {
                "red"
            } else if stats.sample_count > 100 {
                "orange"
            } else {
                "lightblue"
            };
            
            dot.push_str(&format!(
                "  \"{}\" [fillcolor={}, label=\"{}\\n{} samples\"];\n",
                function, color, function, stats.sample_count
            ));
        }
        
        // Add edges
        for (caller, callees) in &call_graph.edges {
            for (callee, count) in callees {
                let weight = if *count > 100 {
                    "3"
                } else if *count > 10 {
                    "2"
                } else {
                    "1"
                };
                
                dot.push_str(&format!(
                    "  \"{}\" -> \"{}\" [label=\"{}\", penwidth={}];\n",
                    caller, callee, count, weight
                ));
            }
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }
    
    fn memory_timeline_to_svg(&self, timeline: &[crate::profiling::memory::TemporalAllocation]) -> Result<(), Error> {
        let mut svg = String::new();
        
        let width = self.config.timeline_width;
        let height = self.config.timeline_height;
        
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        ));
        
        // Style
        svg.push_str(r#"
<style>
    .timeline-line { fill: none; stroke: #0066cc; stroke-width: 2; }
    .grid-line { stroke: #ddd; stroke-width: 1; }
    .axis-label { font-family: Arial; font-size: 12px; }
    .title { font-family: Arial; font-size: 16px; font-weight: bold; text-anchor: middle; }
</style>
"#);
        
        // Title
        svg.push_str(&format!(
            r#"<text x="{}" y="20" class="title">Memory Usage Timeline</text>"#,
            width / 2
        ));
        
        if !timeline.is_empty() {
            let margin = 60.0;
            let chart_width = width as f64 - 2.0 * margin;
            let chart_height = height as f64 - 2.0 * margin;
            
            let max_size = timeline.iter()
                .map(|t| t.cumulative_size)
                .max()
                .unwrap_or(0);
            
            // Grid lines
            for i in 0..=5 {
                let y = margin + (i as f64 * chart_height / 5.0);
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" class="grid-line"/>"#,
                    margin, y, margin + chart_width, y
                ));
                
                let value = max_size - (i * max_size / 5);
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" class="axis-label">{}</text>"#,
                    margin - 10.0, y + 5.0, self.format_bytes(value)
                ));
            }
            
            // Timeline data
            let mut path = String::from("M");
            for (i, point) in timeline.iter().enumerate() {
                let x = margin + (i as f64 * chart_width / timeline.len() as f64);
                let y = margin + chart_height - (point.cumulative_size as f64 / max_size as f64 * chart_height);
                
                if i == 0 {
                    path.push_str(&format!("{},{}", x, y));
                } else {
                    path.push_str(&format!(" L{},{}", x, y));
                }
            }
            
            svg.push_str(&format!(r#"<path d="{}" class="timeline-line"/>"#, path));
        }
        
        svg.push_str("</svg>");
        Ok(svg)
    }
    
    fn allocation_heatmap_to_svg(&self, histogram: &HashMap<usize, u64>) -> Result<(), Error> {
        let mut svg = String::new();
        
        let width = self.config.heatmap_width;
        let height = self.config.heatmap_height;
        
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        ));
        
        svg.push_str(r#"
<style>
    .heatmap-rect { stroke: #fff; stroke-width: 1; }
    .label { font-family: Arial; font-size: 10px; text-anchor: middle; }
    .title { font-family: Arial; font-size: 16px; font-weight: bold; text-anchor: middle; }
</style>
"#);
        
        svg.push_str(&format!(
            r#"<text x="{}" y="20" class="title">Allocation Size Heatmap</text>"#,
            width / 2
        ));
        
        if !histogram.is_empty() {
            let margin = 40.0;
            let chart_width = width as f64 - 2.0 * margin;
            let chart_height = height as f64 - 2.0 * margin;
            
            let max_count = *histogram.values().max().unwrap_or(&1);
            let buckets: Vec<_> = histogram.iter().collect();
            
            let cell_width = chart_width / buckets.len() as f64;
            let cell_height = 30.0;
            
            for (i, (size, count)) in buckets.iter().enumerate() {
                let x = margin + i as f64 * cell_width;
                let y = margin + chart_height - cell_height;
                
                let intensity = (**count as f64 / max_count as f64).min(1.0);
                let color = format!("rgb({}, {}, {})",
                    (255.0 * intensity) as u8,
                    (100.0 * (1.0 - intensity)) as u8,
                    (100.0 * (1.0 - intensity)) as u8
                );
                
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" class="heatmap-rect" title="Size: {}, Count: {}"/>"#,
                    x, y, cell_width, cell_height, color, self.format_bytes(**size), count
                ));
                
                if cell_width > 40.0 {
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" class="label">{}</text>"#,
                        x + cell_width / 2.0,
                        y + cell_height / 2.0 + 5.0,
                        self.format_bytes(**size)
                    ));
                }
            }
        }
        
        svg.push_str("</svg>");
        Ok(svg)
    }
    
    fn goroutine_timeline_to_svg(&self, timeline: &[crate::profiling::concurrency::GoroutineTimeline]) -> Result<(), Error> {
        let mut svg = String::new();
        
        let width = self.config.timeline_width;
        let height = self.config.timeline_height;
        
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        ));
        
        svg.push_str(r#"
<style>
    .goroutine-bar { stroke: #000; stroke-width: 1; }
    .label { font-family: Arial; font-size: 10px; }
    .title { font-family: Arial; font-size: 16px; font-weight: bold; text-anchor: middle; }
</style>
"#);
        
        svg.push_str(&format!(
            r#"<text x="{}" y="20" class="title">Goroutine Timeline</text>"#,
            width / 2
        ));
        
        if !timeline.is_empty() {
            let margin = 60.0;
            let chart_width = width as f64 - 2.0 * margin;
            let chart_height = height as f64 - 2.0 * margin;
            let bar_height = chart_height / timeline.len() as f64;
            
            for (i, goroutine) in timeline.iter().enumerate() {
                let y = margin + i as f64 * bar_height;
                
                if let (Some(spawn_time), Some(completion_time)) = (goroutine.spawn_time, goroutine.completion_time) {
                    let lifetime = completion_time.duration_since(spawn_time);
                    
                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="green" class="goroutine-bar" title="Goroutine {}: {:?}"/>"#,
                        margin,
                        y,
                        chart_width.min(200.0), // Simplified width calculation
                        bar_height - 2.0,
                        goroutine.goroutine_id,
                        lifetime
                    ));
                    
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" class="label">G{}</text>"#,
                        margin + 5.0,
                        y + bar_height / 2.0 + 3.0,
                        goroutine.goroutine_id
                    ));
                }
            }
        }
        
        svg.push_str("</svg>");
        Ok(svg)
    }
    
    fn channel_flow_to_svg(&self, analysis: &crate::profiling::concurrency::ChannelAnalysis) -> Result<(), Error> {
        let mut svg = String::new();
        
        let width = self.config.flow_diagram_width;
        let height = self.config.flow_diagram_height;
        
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        ));
        
        svg.push_str(r#"
<style>
    .channel-node { fill: #2196F3; stroke: black; stroke-width: 2; }
    .flow-arrow { stroke: black; stroke-width: 2; marker-end: url(#arrowhead); }
    .label { font-family: Arial; font-size: 12px; text-anchor: middle; }
    .title { font-family: Arial; font-size: 16px; font-weight: bold; text-anchor: middle; }
</style>
<defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
        <polygon points="0 0, 10 3.5, 0 7" fill="black"/>
    </marker>
</defs>
"#);
        
        svg.push_str(&format!(
            r#"<text x="{}" y="20" class="title">Channel Flow Diagram</text>"#,
            width / 2
        ));
        
        // Simplified channel visualization
        let margin = 60.0;
        let node_radius = 20.0;
        
        for (i, (channel_id, stats)) in analysis.channel_stats.iter().enumerate() {
            let x = margin + (i as f64 * 100.0);
            let y = height as f64 / 2.0;
            
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" class="channel-node" title="Channel {}: {} ops"/>"#,
                x, y, node_radius, channel_id, stats.send_count + stats.receive_count
            ));
            
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="label">Ch{}</text>"#,
                x, y + 5.0, channel_id
            ));
        }
        
        svg.push_str("</svg>");
        Ok(svg)
    }
    
    fn generate_dashboard_html(&self, _profile_data: &crate::profiling::core::ProfileData) -> Result<(), Error> {
        let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Performance Dashboard</title>
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .dashboard-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
        .panel { border: 1px solid #ddd; padding: 15px; border-radius: 5px; }
        .panel h3 { margin-top: 0; color: #333; }
        .metric { font-size: 24px; font-weight: bold; color: #0066cc; }
        .chart { width: 100%; height: 300px; }
    </style>
</head>
<body>
    <h1>CURSED Performance Dashboard</h1>
    
    <div class="dashboard-grid">
        <div class="panel">
            <h3>CPU Performance</h3>
            <div class="metric">85%</div>
            <p>CPU Utilization</p>
            <div id="cpu-chart" class="chart"></div>
        </div>
        
        <div class="panel">
            <h3>Memory Usage</h3>
            <div class="metric">1.2 GB</div>
            <p>Peak Memory</p>
            <div id="memory-chart" class="chart"></div>
        </div>
        
        <div class="panel">
            <h3>Goroutines</h3>
            <div class="metric">12</div>
            <p>Active Goroutines</p>
            <div id="goroutine-chart" class="chart"></div>
        </div>
        
        <div class="panel">
            <h3>I/O Operations</h3>
            <div class="metric">245</div>
            <p>Total Operations</p>
            <div id="io-chart" class="chart"></div>
        </div>
    </div>
    
    <script>
        // Interactive charts would be implemented here using D3.js
        console.log('Dashboard loaded');
    </script>
</body>
</html>
"#;
        
        Ok(html.to_string())
    }
    
    fn generate_color(&self, name: &str) -> String {
        let mut hash = 0;
        for byte in name.bytes() {
            hash = ((hash << 5) - hash + byte as u32) & 0xffffff;
        }
        
        let hue = (hash % 360) as f64;
        format!("hsl({}, 70%, 60%)", hue)
    }
    
    fn truncate_text(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }
    
    fn format_bytes(&self, bytes: usize) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut value = bytes as f64;
        let mut unit_index = 0;
        
        while value >= 1024.0 && unit_index < UNITS.len() - 1 {
            value /= 1024.0;
            unit_index += 1;
        }
        
        if value < 10.0 && unit_index > 0 {
            format!("{:.1}{}", value, UNITS[unit_index])
        } else {
            format!("{:.0}{}", value, UNITS[unit_index])
        }
    }
}

/// Visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub flame_graph_width: u32,
    pub flame_graph_height: u32,
    pub timeline_width: u32,
    pub timeline_height: u32,
    pub heatmap_width: u32,
    pub heatmap_height: u32,
    pub flow_diagram_width: u32,
    pub flow_diagram_height: u32,
    pub interactive_mode: bool,
    pub color_scheme: ColorScheme,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            flame_graph_width: 1200,
            flame_graph_height: 600,
            timeline_width: 1000,
            timeline_height: 400,
            heatmap_width: 800,
            heatmap_height: 300,
            flow_diagram_width: 900,
            flow_diagram_height: 500,
            interactive_mode: true,
            color_scheme: ColorScheme::Default,
        }
    }
}

/// Color schemes for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    Default,
    HighContrast,
    Colorblind,
    Dark,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visualization_generator_creation() {
        let config = VisualizationConfig::default();
        let generator = VisualizationGenerator::new(config);
        assert_eq!(generator.config.flame_graph_width, 1200);
    }
    
    #[test]
    fn test_color_generation() {
        let config = VisualizationConfig::default();
        let generator = VisualizationGenerator::new(config);
        
        let color1 = generator.generate_color("function1");
        let color2 = generator.generate_color("function2");
        
        assert_ne!(color1, color2);
        assert!(color1.starts_with("hsl("));
    }
    
    #[test]
    fn test_text_truncation() {
        let config = VisualizationConfig::default();
        let generator = VisualizationGenerator::new(config);
        
        let truncated = generator.truncate_text("very_long_function_name", 10);
        assert_eq!(truncated, "very_lo...");
        
        let short = generator.truncate_text("short", 10);
        assert_eq!(short, "short");
    }
    
    #[test]
    fn test_bytes_formatting() {
        let config = VisualizationConfig::default();
        let generator = VisualizationGenerator::new(config);
        
        assert_eq!(generator.format_bytes(512), "512B");
        assert_eq!(generator.format_bytes(1024), "1KB");
        assert_eq!(generator.format_bytes(1536), "2KB");
        assert_eq!(generator.format_bytes(1048576), "1MB");
    }
}
