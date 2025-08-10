/// Coverage reporting in multiple formats
/// 
/// Generates coverage reports in HTML, JSON, XML, and LCOV formats
/// with comprehensive visualizations and metrics.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use serde_json;

use super::{CoverageData, OutputFormat, CoverageConfig};

/// Generates coverage reports in multiple formats
pub struct CoverageReporter {
    config: CoverageConfig,
    templates_dir: PathBuf,
}

impl CoverageReporter {
    pub fn new(config: CoverageConfig) -> io::Result<Self> {
        let templates_dir = config.output_dir.join("templates");
        fs::create_dir_all(&templates_dir)?;
        
        let mut reporter = Self {
            config,
            templates_dir,
        };
        
        // Create HTML templates
        reporter.create_html_templates()?;
        
        Ok(reporter)
    }

    /// Generate a coverage report in the specified format
    pub async fn generate_report(&self, coverage_data: &CoverageData, format: &OutputFormat) -> io::Result<()> {
        match format {
            OutputFormat::Json => self.generate_json_report(coverage_data).await,
            OutputFormat::Html => self.generate_html_report(coverage_data).await,
            OutputFormat::Xml => self.generate_xml_report(coverage_data).await,
            OutputFormat::Lcov => self.generate_lcov_report(coverage_data).await,
            OutputFormat::Console => self.generate_console_report(coverage_data).await,
        }
    }

    /// Generate JSON coverage report
    async fn generate_json_report(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let output_file = self.config.output_dir.join("coverage.json");
        let json_data = serde_json::to_string_pretty(coverage_data)?;
        fs::write(output_file, json_data)?;
        println!("📄 JSON report generated: coverage/coverage.json");
        Ok(())
    }

    /// Generate HTML coverage report with interactive visualization
    async fn generate_html_report(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let html_dir = self.config.output_dir.join("html");
        fs::create_dir_all(&html_dir)?;
        
        // Generate main index page
        self.generate_html_index(coverage_data, &html_dir).await?;
        
        // Generate file detail pages
        for (file_path, file_coverage) in &coverage_data.files {
            self.generate_html_file_detail(file_path, file_coverage, &html_dir).await?;
        }
        
        // Copy assets
        self.copy_html_assets(&html_dir)?;
        
        println!("🌐 HTML report generated: coverage/html/index.html");
        Ok(())
    }

    /// Generate main HTML index page
    async fn generate_html_index(&self, coverage_data: &CoverageData, html_dir: &Path) -> io::Result<()> {
        let template = self.load_html_template("index.html")?;
        
        // Generate file list HTML
        let mut file_rows = String::new();
        let mut files: Vec<_> = coverage_data.files.iter().collect();
        files.sort_by(|a, b| a.0.cmp(b.0));
        
        for (file_path, file_coverage) in files {
            let coverage_class = if file_coverage.coverage_percentage >= 90.0 {
                "high-coverage"
            } else if file_coverage.coverage_percentage >= 70.0 {
                "medium-coverage"
            } else {
                "low-coverage"
            };
            
            let relative_file_path = file_path.replace(&format!("{}/", std::env::current_dir().unwrap().to_string_lossy()), "");
            let html_file_name = format!("{}.html", file_path.replace("/", "_").replace(".", "_"));
            
            file_rows.push_str(&format!(
                r#"
                <tr class="{}">
                    <td><a href="{}">{}</a></td>
                    <td>{}/{}</td>
                    <td>{:.2}%</td>
                    <td>{}/{}</td>
                    <td>{:.2}%</td>
                    <td>{}/{}</td>
                    <td>{:.2}%</td>
                </tr>
                "#,
                coverage_class,
                html_file_name,
                relative_file_path,
                file_coverage.covered_lines,
                file_coverage.total_lines,
                file_coverage.coverage_percentage,
                file_coverage.functions.values().filter(|f| f.is_covered).count(),
                file_coverage.functions.len(),
                if file_coverage.functions.is_empty() { 100.0 } else {
                    (file_coverage.functions.values().filter(|f| f.is_covered).count() as f64 / file_coverage.functions.len() as f64) * 100.0
                },
                file_coverage.branches.values().filter(|b| b.is_covered).count(),
                file_coverage.branches.len(),
                if file_coverage.branches.is_empty() { 100.0 } else {
                    (file_coverage.branches.values().filter(|b| b.is_covered).count() as f64 / file_coverage.branches.len() as f64) * 100.0
                }
            ));
        }
        
        let html_content = template
            .replace("{{TIMESTAMP}}", &coverage_data.timestamp)
            .replace("{{TEST_RUN_ID}}", &coverage_data.test_run_id)
            .replace("{{TOTAL_FILES}}", &coverage_data.summary.total_files.to_string())
            .replace("{{TOTAL_LINES}}", &coverage_data.summary.total_lines.to_string())
            .replace("{{COVERED_LINES}}", &coverage_data.summary.covered_lines.to_string())
            .replace("{{LINE_COVERAGE}}", &format!("{:.2}", coverage_data.summary.line_coverage_percentage))
            .replace("{{TOTAL_FUNCTIONS}}", &coverage_data.summary.total_functions.to_string())
            .replace("{{COVERED_FUNCTIONS}}", &coverage_data.summary.covered_functions.to_string())
            .replace("{{FUNCTION_COVERAGE}}", &format!("{:.2}", coverage_data.summary.function_coverage_percentage))
            .replace("{{TOTAL_BRANCHES}}", &coverage_data.summary.total_branches.to_string())
            .replace("{{COVERED_BRANCHES}}", &coverage_data.summary.covered_branches.to_string())
            .replace("{{BRANCH_COVERAGE}}", &format!("{:.2}", coverage_data.summary.branch_coverage_percentage))
            .replace("{{FILE_ROWS}}", &file_rows);
        
        let index_file = html_dir.join("index.html");
        fs::write(index_file, html_content)?;
        
        Ok(())
    }

    /// Generate HTML file detail page
    async fn generate_html_file_detail(&self, file_path: &str, file_coverage: &super::FileCoverage, html_dir: &Path) -> io::Result<()> {
        let template = self.load_html_template("file.html")?;
        
        // Load source file content
        let source_content = fs::read_to_string(file_path).unwrap_or_else(|_| "Source file not found".to_string());
        
        // Generate source code HTML with coverage highlighting
        let mut source_html = String::new();
        for (line_num, line_content) in source_content.lines().enumerate() {
            let line_number = (line_num + 1) as u32;
            
            let (coverage_class, hit_count) = if let Some(line_cov) = file_coverage.lines.get(&line_number) {
                if line_cov.is_covered {
                    ("covered", line_cov.execution_count.to_string())
                } else {
                    ("uncovered", "0".to_string())
                }
            } else {
                ("not-executable", "".to_string())
            };
            
            source_html.push_str(&format!(
                r#"<tr class="{}"><td class="line-number">{}</td><td class="hit-count">{}</td><td class="source-line">{}</td></tr>"#,
                coverage_class,
                line_number,
                hit_count,
                html_escape(line_content)
            ));
        }
        
        let html_content = template
            .replace("{{FILE_PATH}}", file_path)
            .replace("{{TOTAL_LINES}}", &file_coverage.total_lines.to_string())
            .replace("{{COVERED_LINES}}", &file_coverage.covered_lines.to_string())
            .replace("{{COVERAGE_PERCENTAGE}}", &format!("{:.2}", file_coverage.coverage_percentage))
            .replace("{{SOURCE_LINES}}", &source_html);
        
        let html_file_name = format!("{}.html", file_path.replace("/", "_").replace(".", "_"));
        let output_file = html_dir.join(html_file_name);
        fs::write(output_file, html_content)?;
        
        Ok(())
    }

    /// Generate XML coverage report (Cobertura format)
    async fn generate_xml_report(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let output_file = self.config.output_dir.join("coverage.xml");
        
        let mut xml_content = String::new();
        xml_content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml_content.push_str("\n");
        xml_content.push_str(&format!(
            r#"<coverage timestamp="{}" version="cursed-coverage-1.0">"#,
            coverage_data.timestamp
        ));
        xml_content.push_str("\n");
        
        // Add summary
        xml_content.push_str(&format!(
            r#"  <summary lines-total="{}" lines-covered="{}" line-rate="{:.4}" functions-total="{}" functions-covered="{}" function-rate="{:.4}" branches-total="{}" branches-covered="{}" branch-rate="{:.4}"/>"#,
            coverage_data.summary.total_lines,
            coverage_data.summary.covered_lines,
            coverage_data.summary.line_coverage_percentage / 100.0,
            coverage_data.summary.total_functions,
            coverage_data.summary.covered_functions,
            coverage_data.summary.function_coverage_percentage / 100.0,
            coverage_data.summary.total_branches,
            coverage_data.summary.covered_branches,
            coverage_data.summary.branch_coverage_percentage / 100.0
        ));
        xml_content.push_str("\n");
        
        // Add packages and files
        xml_content.push_str("  <packages>\n");
        
        // Group files by directory (package)
        let mut packages: HashMap<String, Vec<&super::FileCoverage>> = HashMap::new();
        for file_coverage in coverage_data.files.values() {
            let package_name = Path::new(&file_coverage.path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "root".to_string());
            packages.entry(package_name).or_insert_with(Vec::new).push(file_coverage);
        }
        
        for (package_name, files) in packages {
            xml_content.push_str(&format!(r#"    <package name="{}">"#, xml_escape(&package_name)));
            xml_content.push_str("\n");
            xml_content.push_str("      <classes>\n");
            
            for file_coverage in files {
                let file_name = Path::new(&file_coverage.path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                
                xml_content.push_str(&format!(
                    r#"        <class name="{}" filename="{}" line-rate="{:.4}">"#,
                    xml_escape(&file_name),
                    xml_escape(&file_coverage.path),
                    file_coverage.coverage_percentage / 100.0
                ));
                xml_content.push_str("\n");
                
                // Add methods (functions)
                if !file_coverage.functions.is_empty() {
                    xml_content.push_str("          <methods>\n");
                    for function in file_coverage.functions.values() {
                        xml_content.push_str(&format!(
                            r#"            <method name="{}" hits="{}" signature="()"/>"#,
                            xml_escape(&function.name),
                            function.execution_count
                        ));
                        xml_content.push_str("\n");
                    }
                    xml_content.push_str("          </methods>\n");
                }
                
                // Add lines
                xml_content.push_str("          <lines>\n");
                for line in file_coverage.lines.values() {
                    xml_content.push_str(&format!(
                        r#"            <line number="{}" hits="{}"/>"#,
                        line.line_number,
                        line.execution_count
                    ));
                    xml_content.push_str("\n");
                }
                xml_content.push_str("          </lines>\n");
                
                xml_content.push_str("        </class>\n");
            }
            
            xml_content.push_str("      </classes>\n");
            xml_content.push_str("    </package>\n");
        }
        
        xml_content.push_str("  </packages>\n");
        xml_content.push_str("</coverage>\n");
        
        fs::write(output_file, xml_content)?;
        println!("📊 XML report generated: coverage/coverage.xml");
        Ok(())
    }

    /// Generate LCOV coverage report
    async fn generate_lcov_report(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let output_file = self.config.output_dir.join("coverage.lcov");
        let mut lcov_content = String::new();
        
        for file_coverage in coverage_data.files.values() {
            lcov_content.push_str(&format!("SF:{}\n", file_coverage.path));
            
            // Add function information
            for function in file_coverage.functions.values() {
                lcov_content.push_str(&format!("FN:{},{}\n", function.start_line, function.name));
            }
            
            for function in file_coverage.functions.values() {
                lcov_content.push_str(&format!("FNDA:{},{}\n", function.execution_count, function.name));
            }
            
            lcov_content.push_str(&format!("FNF:{}\n", file_coverage.functions.len()));
            lcov_content.push_str(&format!("FNH:{}\n", file_coverage.functions.values().filter(|f| f.is_covered).count()));
            
            // Add line information
            for line in file_coverage.lines.values() {
                lcov_content.push_str(&format!("DA:{},{}\n", line.line_number, line.execution_count));
            }
            
            lcov_content.push_str(&format!("LF:{}\n", file_coverage.total_lines));
            lcov_content.push_str(&format!("LH:{}\n", file_coverage.covered_lines));
            
            // Add branch information
            for branch in file_coverage.branches.values() {
                lcov_content.push_str(&format!("BDA:{},0,{}\n", branch.line_number, branch.false_count));
                lcov_content.push_str(&format!("BDA:{},1,{}\n", branch.line_number, branch.true_count));
            }
            
            lcov_content.push_str(&format!("BRF:{}\n", file_coverage.branches.len() * 2));
            lcov_content.push_str(&format!("BRH:{}\n", file_coverage.branches.values().filter(|b| b.is_covered).count() * 2));
            
            lcov_content.push_str("end_of_record\n");
        }
        
        fs::write(output_file, lcov_content)?;
        println!("📈 LCOV report generated: coverage/coverage.lcov");
        Ok(())
    }

    /// Generate console coverage report
    async fn generate_console_report(&self, coverage_data: &CoverageData) -> io::Result<()> {
        println!("\n{}", "=".repeat(80));
        println!("📊 CURSED Coverage Report");
        println!("{}", "=".repeat(80));
        println!("Test Run ID: {}", coverage_data.test_run_id);
        println!("Timestamp: {}", coverage_data.timestamp);
        println!();
        
        // Summary table
        println!("📈 Coverage Summary:");
        println!("┌────────────────┬─────────┬─────────┬─────────────┐");
        println!("│ Type           │ Covered │ Total   │ Percentage  │");
        println!("├────────────────┼─────────┼─────────┼─────────────┤");
        println!("│ Lines          │ {:7} │ {:7} │ {:10.2}% │", 
                 coverage_data.summary.covered_lines, 
                 coverage_data.summary.total_lines, 
                 coverage_data.summary.line_coverage_percentage);
        println!("│ Functions      │ {:7} │ {:7} │ {:10.2}% │", 
                 coverage_data.summary.covered_functions, 
                 coverage_data.summary.total_functions, 
                 coverage_data.summary.function_coverage_percentage);
        println!("│ Branches       │ {:7} │ {:7} │ {:10.2}% │", 
                 coverage_data.summary.covered_branches, 
                 coverage_data.summary.total_branches, 
                 coverage_data.summary.branch_coverage_percentage);
        println!("└────────────────┴─────────┴─────────┴─────────────┘");
        println!();
        
        // File breakdown
        println!("📁 File Coverage Breakdown:");
        println!("┌─────────────────────────────────────────────────┬─────────────┐");
        println!("│ File                                            │ Coverage    │");
        println!("├─────────────────────────────────────────────────┼─────────────┤");
        
        let mut files: Vec<_> = coverage_data.files.iter().collect();
        files.sort_by(|a, b| b.1.coverage_percentage.partial_cmp(&a.1.coverage_percentage).unwrap());
        
        for (file_path, file_coverage) in files.iter().take(20) {
            let short_path = if file_path.len() > 47 {
                format!("...{}", &file_path[file_path.len() - 44..])
            } else {
                file_path.to_string()
            };
            
            let coverage_indicator = if file_coverage.coverage_percentage >= 90.0 {
                "🟢"
            } else if file_coverage.coverage_percentage >= 70.0 {
                "🟡"
            } else {
                "🔴"
            };
            
            println!("│ {:47} │ {} {:8.2}% │", 
                     short_path, 
                     coverage_indicator,
                     file_coverage.coverage_percentage);
        }
        
        if files.len() > 20 {
            println!("│ ... and {} more files                           │             │", files.len() - 20);
        }
        
        println!("└─────────────────────────────────────────────────┴─────────────┘");
        println!();
        
        // Show files with low coverage
        let low_coverage_files: Vec<_> = files.iter()
            .filter(|(_, cov)| cov.coverage_percentage < 70.0)
            .collect();
        
        if !low_coverage_files.is_empty() {
            println!("⚠️  Files with low coverage (< 70%):");
            for (file_path, file_coverage) in low_coverage_files.iter().take(10) {
                println!("   🔴 {} ({:.2}%)", file_path, file_coverage.coverage_percentage);
            }
            println!();
        }
        
        Ok(())
    }

    /// Create HTML templates
    fn create_html_templates(&self) -> io::Result<()> {
        // Create index.html template
        let index_template = include_str!("templates/index.html");
        fs::write(self.templates_dir.join("index.html"), index_template)?;
        
        // Create file.html template
        let file_template = include_str!("templates/file.html");
        fs::write(self.templates_dir.join("file.html"), file_template)?;
        
        Ok(())
    }

    /// Load HTML template
    fn load_html_template(&self, template_name: &str) -> io::Result<String> {
        fs::read_to_string(self.templates_dir.join(template_name))
    }

    /// Copy HTML assets (CSS, JS)
    fn copy_html_assets(&self, html_dir: &Path) -> io::Result<()> {
        let css_content = include_str!("templates/style.css");
        fs::write(html_dir.join("style.css"), css_content)?;
        
        let js_content = include_str!("templates/coverage.js");
        fs::write(html_dir.join("coverage.js"), js_content)?;
        
        Ok(())
    }
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Escape XML special characters
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Include template files as string literals
mod templates {
    pub const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED Coverage Report</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>🎯 CURSED Coverage Report</h1>
        <p>Generated: {{TIMESTAMP}} | Test Run: {{TEST_RUN_ID}}</p>
    </header>
    
    <main>
        <section class="summary">
            <h2>📊 Coverage Summary</h2>
            <div class="metrics">
                <div class="metric">
                    <div class="metric-value">{{LINE_COVERAGE}}%</div>
                    <div class="metric-label">Line Coverage</div>
                    <div class="metric-detail">{{COVERED_LINES}} / {{TOTAL_LINES}} lines</div>
                </div>
                <div class="metric">
                    <div class="metric-value">{{FUNCTION_COVERAGE}}%</div>
                    <div class="metric-label">Function Coverage</div>
                    <div class="metric-detail">{{COVERED_FUNCTIONS}} / {{TOTAL_FUNCTIONS}} functions</div>
                </div>
                <div class="metric">
                    <div class="metric-value">{{BRANCH_COVERAGE}}%</div>
                    <div class="metric-label">Branch Coverage</div>
                    <div class="metric-detail">{{COVERED_BRANCHES}} / {{TOTAL_BRANCHES}} branches</div>
                </div>
            </div>
        </section>
        
        <section class="file-list">
            <h2>📁 File Coverage</h2>
            <table>
                <thead>
                    <tr>
                        <th>File</th>
                        <th>Lines</th>
                        <th>Line %</th>
                        <th>Functions</th>
                        <th>Function %</th>
                        <th>Branches</th>
                        <th>Branch %</th>
                    </tr>
                </thead>
                <tbody>
                    {{FILE_ROWS}}
                </tbody>
            </table>
        </section>
    </main>
    
    <script src="coverage.js"></script>
</body>
</html>
"#;

    pub const FILE_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{FILE_PATH}} - CURSED Coverage</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>📄 {{FILE_PATH}}</h1>
        <p>Coverage: {{COVERAGE_PERCENTAGE}}% ({{COVERED_LINES}} / {{TOTAL_LINES}} lines)</p>
        <a href="index.html">← Back to Summary</a>
    </header>
    
    <main>
        <section class="source-code">
            <table class="source-table">
                <thead>
                    <tr>
                        <th>Line</th>
                        <th>Hits</th>
                        <th>Source</th>
                    </tr>
                </thead>
                <tbody>
                    {{SOURCE_LINES}}
                </tbody>
            </table>
        </section>
    </main>
    
    <script src="coverage.js"></script>
</body>
</html>
"#;
}
