yeet "testz"
yeet "stringz"
yeet "mathz"
yeet "dropz"
yeet "timez"
yeet "encode_mood"

# Coverage Analysis Module for CURSED
# Provides comprehensive code coverage tracking and analysis

# Coverage data structures
sus CoveragePoint struct {
    file_path tea
    line_number normie
    column_number normie
    function_name tea
    hit_count normie
    branch_taken lit
}

sus CoverageReport struct {
    total_lines normie
    covered_lines normie
    total_functions normie
    covered_functions normie
    total_branches normie
    covered_branches normie
    line_coverage_percent meal
    function_coverage_percent meal
    branch_coverage_percent meal
    file_reports [tea]
}

sus CoverageConfig struct {
    output_format tea  # "html", "json", "console", "xml"
    output_directory tea
    threshold_line meal
    threshold_function meal
    threshold_branch meal
    include_patterns [tea]
    exclude_patterns [tea]
    instrument_tests lit
}

# Global coverage data storage
sus coverage_data [CoveragePoint]
sus coverage_config CoverageConfig
sus instrumentation_enabled lit = cap

# Initialize coverage system
slay coverage_init(config CoverageConfig) lit {
    coverage_config = config
    coverage_data = []
    instrumentation_enabled = based
    
    # Create output directory if it doesn't exist
    create_directory(config.output_directory)
    
    vibez.spill("Coverage analysis initialized")
    damn based
}

# Instrument code for coverage tracking
slay instrument_code(file_path tea, source_code tea) tea {
    sus instrumented_code tea = ""
    sus lines [tea] = stringz.split(source_code, "\n")
    sus line_number normie = 1
    
    # Add coverage tracking imports
    instrumented_code = instrumented_code + "yeet \"coverage_analysis\"\n"
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = lines[i]
        
        # Skip empty lines and comments
        lowkey stringz.trim(line) == "" || stringz.starts_with(stringz.trim(line), "#") {
            instrumented_code = instrumented_code + line + "\n"
            line_number++
            simp
        }
        
        # Instrument function definitions
        lowkey stringz.contains(line, "slay ") {
            sus function_name tea = extract_function_name(line)
            instrumented_code = instrumented_code + line + "\n"
            instrumented_code = instrumented_code + "    coverage_track_function(\"" + file_path + "\", " + 
                              toString(line_number) + ", \"" + function_name + "\")\n"
        } else {
            # Instrument regular lines
            instrumented_code = instrumented_code + "coverage_track_line(\"" + file_path + "\", " + 
                              toString(line_number) + ")\n"
            instrumented_code = instrumented_code + line + "\n"
        }
        
        line_number++
    }
    
    damn instrumented_code
}

# Extract function name from function definition line
slay extract_function_name(line tea) tea {
    sus parts [tea] = stringz.split(line, " ")
    bestie i := 0; i < len(parts); i++ {
        lowkey parts[i] == "slay" && i + 1 < len(parts) {
            sus func_part tea = parts[i + 1]
            sus paren_index normie = stringz.index_of(func_part, "(")
            lowkey paren_index > 0 {
                damn stringz.substring(func_part, 0, paren_index)
            } else {
                damn func_part
            }
        }
    }
    damn "unknown_function"
}

# Track line coverage
slay coverage_track_line(file_path tea, line_number normie) {
    lowkey !instrumentation_enabled {
        damn
    }
    
    sus point CoveragePoint = {
        file_path: file_path,
        line_number: line_number,
        column_number: 0,
        function_name: "",
        hit_count: 1,
        branch_taken: cap
    }
    
    # Check if this line is already tracked
    bestie i := 0; i < len(coverage_data); i++ {
        lowkey coverage_data[i].file_path == file_path && 
              coverage_data[i].line_number == line_number {
            coverage_data[i].hit_count++
            damn
        }
    }
    
    # Add new coverage point
    coverage_data = append(coverage_data, point)
}

# Track function coverage
slay coverage_track_function(file_path tea, line_number normie, function_name tea) {
    lowkey !instrumentation_enabled {
        damn
    }
    
    sus point CoveragePoint = {
        file_path: file_path,
        line_number: line_number,
        column_number: 0,
        function_name: function_name,
        hit_count: 1,
        branch_taken: cap
    }
    
    # Check if this function is already tracked
    bestie i := 0; i < len(coverage_data); i++ {
        lowkey coverage_data[i].file_path == file_path && 
              coverage_data[i].function_name == function_name {
            coverage_data[i].hit_count++
            damn
        }
    }
    
    coverage_data = append(coverage_data, point)
}

# Track branch coverage
slay coverage_track_branch(file_path tea, line_number normie, branch_taken lit) {
    lowkey !instrumentation_enabled {
        damn
    }
    
    sus point CoveragePoint = {
        file_path: file_path,
        line_number: line_number,
        column_number: 0,
        function_name: "",
        hit_count: 1,
        branch_taken: branch_taken
    }
    
    coverage_data = append(coverage_data, point)
}

# Generate coverage report
slay generate_coverage_report() CoverageReport {
    sus report CoverageReport = {
        total_lines: 0,
        covered_lines: 0,
        total_functions: 0,
        covered_functions: 0,
        total_branches: 0,
        covered_branches: 0,
        line_coverage_percent: 0.0,
        function_coverage_percent: 0.0,
        branch_coverage_percent: 0.0,
        file_reports: []
    }
    
    # Calculate line coverage
    sus line_counts map[tea]normie = {}
    sus covered_line_counts map[tea]normie = {}
    
    bestie i := 0; i < len(coverage_data); i++ {
        sus point CoveragePoint = coverage_data[i]
        lowkey point.function_name == "" {  # Line coverage
            line_counts[point.file_path]++
            lowkey point.hit_count > 0 {
                covered_line_counts[point.file_path]++
            }
        }
    }
    
    # Calculate function coverage
    sus function_counts map[tea]normie = {}
    sus covered_function_counts map[tea]normie = {}
    
    bestie i := 0; i < len(coverage_data); i++ {
        sus point CoveragePoint = coverage_data[i]
        lowkey point.function_name != "" {  # Function coverage
            function_counts[point.file_path]++
            lowkey point.hit_count > 0 {
                covered_function_counts[point.file_path]++
            }
        }
    }
    
    # Calculate totals
    bestie file, count := range line_counts {
        report.total_lines += count
        report.covered_lines += covered_line_counts[file]
    }
    
    bestie file, count := range function_counts {
        report.total_functions += count
        report.covered_functions += covered_function_counts[file]
    }
    
    # Calculate percentages
    lowkey report.total_lines > 0 {
        report.line_coverage_percent = (report.covered_lines * 100.0) / report.total_lines
    }
    
    lowkey report.total_functions > 0 {
        report.function_coverage_percent = (report.covered_functions * 100.0) / report.total_functions
    }
    
    lowkey report.total_branches > 0 {
        report.branch_coverage_percent = (report.covered_branches * 100.0) / report.total_branches
    }
    
    damn report
}

# Generate HTML coverage report
slay generate_html_report(report CoverageReport, output_path tea) lit {
    sus html_content tea = generate_html_template(report)
    
    lowkey !write_file(output_path + "/coverage.html", html_content) {
        vibez.spill("Error: Could not write HTML coverage report")
        damn cap
    }
    
    # Generate individual file reports
    generate_file_html_reports(output_path)
    
    vibez.spill("HTML coverage report generated: " + output_path + "/coverage.html")
    damn based
}

# Generate HTML template
slay generate_html_template(report CoverageReport) tea {
    sus html tea = "<!DOCTYPE html>\n"
    html += "<html><head><title>CURSED Coverage Report</title>\n"
    html += "<style>\n"
    html += "body { font-family: Arial, sans-serif; margin: 20px; }\n"
    html += ".header { background-color: #f0f0f0; padding: 10px; border-radius: 5px; }\n"
    html += ".metric { display: inline-block; margin: 10px; padding: 10px; border-radius: 5px; }\n"
    html += ".high-coverage { background-color: #d4edda; }\n"
    html += ".medium-coverage { background-color: #fff3cd; }\n"
    html += ".low-coverage { background-color: #f8d7da; }\n"
    html += "table { border-collapse: collapse; width: 100%; margin-top: 20px; }\n"
    html += "th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n"
    html += "th { background-color: #f2f2f2; }\n"
    html += "</style></head><body>\n"
    
    html += "<div class='header'>\n"
    html += "<h1>CURSED Code Coverage Report</h1>\n"
    html += "<p>Generated on: " + timez.format_now() + "</p>\n"
    html += "</div>\n"
    
    # Coverage metrics
    html += "<div class='metrics'>\n"
    html += generate_metric_div("Line Coverage", report.line_coverage_percent)
    html += generate_metric_div("Function Coverage", report.function_coverage_percent)
    html += generate_metric_div("Branch Coverage", report.branch_coverage_percent)
    html += "</div>\n"
    
    # Summary table
    html += "<h2>Coverage Summary</h2>\n"
    html += "<table>\n"
    html += "<tr><th>Metric</th><th>Covered</th><th>Total</th><th>Percentage</th></tr>\n"
    html += "<tr><td>Lines</td><td>" + toString(report.covered_lines) + "</td><td>" + 
            toString(report.total_lines) + "</td><td>" + 
            toString(report.line_coverage_percent) + "%</td></tr>\n"
    html += "<tr><td>Functions</td><td>" + toString(report.covered_functions) + "</td><td>" + 
            toString(report.total_functions) + "</td><td>" + 
            toString(report.function_coverage_percent) + "%</td></tr>\n"
    html += "<tr><td>Branches</td><td>" + toString(report.covered_branches) + "</td><td>" + 
            toString(report.total_branches) + "</td><td>" + 
            toString(report.branch_coverage_percent) + "%</td></tr>\n"
    html += "</table>\n"
    
    html += "</body></html>\n"
    
    damn html
}

# Generate metric div with color coding
slay generate_metric_div(label tea, percentage meal) tea {
    sus class_name tea = "low-coverage"
    lowkey percentage >= 90.0 {
        class_name = "high-coverage"
    } else lowkey percentage >= 70.0 {
        class_name = "medium-coverage"
    }
    
    damn "<div class='metric " + class_name + "'>\n" +
         "<h3>" + label + "</h3>\n" +
         "<p>" + toString(percentage) + "%</p>\n" +
         "</div>\n"
}

# Generate file-specific HTML reports
slay generate_file_html_reports(output_path tea) {
    sus file_coverage map[tea][CoveragePoint] = {}
    
    # Group coverage data by file
    bestie i := 0; i < len(coverage_data); i++ {
        sus point CoveragePoint = coverage_data[i]
        file_coverage[point.file_path] = append(file_coverage[point.file_path], point)
    }
    
    # Generate report for each file
    bestie file_path, points := range file_coverage {
        generate_single_file_html_report(file_path, points, output_path)
    }
}

# Generate single file HTML report
slay generate_single_file_html_report(file_path tea, points [CoveragePoint], output_path tea) {
    sus file_name tea = extract_filename(file_path)
    sus html_file tea = output_path + "/" + file_name + ".html"
    
    sus html tea = generate_file_html_content(file_path, points)
    write_file(html_file, html)
}

# Generate JSON coverage report
slay generate_json_report(report CoverageReport, output_path tea) lit {
    sus json_content tea = coverage_report_to_json(report)
    
    lowkey !write_file(output_path + "/coverage.json", json_content) {
        vibez.spill("Error: Could not write JSON coverage report")
        damn cap
    }
    
    vibez.spill("JSON coverage report generated: " + output_path + "/coverage.json")
    damn based
}

# Convert coverage report to JSON
slay coverage_report_to_json(report CoverageReport) tea {
    sus json tea = "{\n"
    json += "  \"total_lines\": " + toString(report.total_lines) + ",\n"
    json += "  \"covered_lines\": " + toString(report.covered_lines) + ",\n"
    json += "  \"total_functions\": " + toString(report.total_functions) + ",\n"
    json += "  \"covered_functions\": " + toString(report.covered_functions) + ",\n"
    json += "  \"total_branches\": " + toString(report.total_branches) + ",\n"
    json += "  \"covered_branches\": " + toString(report.covered_branches) + ",\n"
    json += "  \"line_coverage_percent\": " + toString(report.line_coverage_percent) + ",\n"
    json += "  \"function_coverage_percent\": " + toString(report.function_coverage_percent) + ",\n"
    json += "  \"branch_coverage_percent\": " + toString(report.branch_coverage_percent) + ",\n"
    json += "  \"timestamp\": \"" + timez.format_now() + "\",\n"
    json += "  \"coverage_data\": [\n"
    
    bestie i := 0; i < len(coverage_data); i++ {
        sus point CoveragePoint = coverage_data[i]
        json += "    {\n"
        json += "      \"file_path\": \"" + point.file_path + "\",\n"
        json += "      \"line_number\": " + toString(point.line_number) + ",\n"
        json += "      \"function_name\": \"" + point.function_name + "\",\n"
        json += "      \"hit_count\": " + toString(point.hit_count) + ",\n"
        json += "      \"branch_taken\": " + toString(point.branch_taken) + "\n"
        json += "    }"
        lowkey i < len(coverage_data) - 1 {
            json += ","
        }
        json += "\n"
    }
    
    json += "  ]\n"
    json += "}\n"
    
    damn json
}

# Generate console coverage report
slay generate_console_report(report CoverageReport) {
    vibez.spill("=== CURSED Coverage Report ===")
    vibez.spill("")
    vibez.spill("Coverage Summary:")
    vibez.spill("  Lines:     " + toString(report.covered_lines) + "/" + 
               toString(report.total_lines) + " (" + 
               toString(report.line_coverage_percent) + "%)")
    vibez.spill("  Functions: " + toString(report.covered_functions) + "/" + 
               toString(report.total_functions) + " (" + 
               toString(report.function_coverage_percent) + "%)")
    vibez.spill("  Branches:  " + toString(report.covered_branches) + "/" + 
               toString(report.total_branches) + " (" + 
               toString(report.branch_coverage_percent) + "%)")
    vibez.spill("")
    
    # Coverage thresholds
    check_coverage_thresholds(report)
}

# Check coverage against configured thresholds
slay check_coverage_thresholds(report CoverageReport) lit {
    sus all_thresholds_met lit = based
    
    lowkey report.line_coverage_percent < coverage_config.threshold_line {
        vibez.spill("⚠️  Line coverage (" + toString(report.line_coverage_percent) + 
                   "%) below threshold (" + toString(coverage_config.threshold_line) + "%)")
        all_thresholds_met = cap
    }
    
    lowkey report.function_coverage_percent < coverage_config.threshold_function {
        vibez.spill("⚠️  Function coverage (" + toString(report.function_coverage_percent) + 
                   "%) below threshold (" + toString(coverage_config.threshold_function) + "%)")
        all_thresholds_met = cap
    }
    
    lowkey report.branch_coverage_percent < coverage_config.threshold_branch {
        vibez.spill("⚠️  Branch coverage (" + toString(report.branch_coverage_percent) + 
                   "%) below threshold (" + toString(coverage_config.threshold_branch) + "%)")
        all_thresholds_met = cap
    }
    
    lowkey all_thresholds_met {
        vibez.spill("✅ All coverage thresholds met!")
    }
    
    damn all_thresholds_met
}

# Run coverage analysis on a project
slay run_coverage_analysis(project_path tea, config CoverageConfig) lit {
    coverage_init(config)
    
    vibez.spill("Starting coverage analysis for: " + project_path)
    
    # Find all .csd files in project
    sus files [tea] = find_cursed_files(project_path)
    
    vibez.spill("Found " + toString(len(files)) + " CURSED files")
    
    # Instrument and run tests
    bestie i := 0; i < len(files); i++ {
        sus file_path tea = files[i]
        lowkey should_include_file(file_path, config) {
            instrument_and_execute_file(file_path)
        }
    }
    
    # Generate reports
    sus report CoverageReport = generate_coverage_report()
    
    lowkey config.output_format == "html" || config.output_format == "all" {
        generate_html_report(report, config.output_directory)
    }
    
    lowkey config.output_format == "json" || config.output_format == "all" {
        generate_json_report(report, config.output_directory)
    }
    
    lowkey config.output_format == "console" || config.output_format == "all" {
        generate_console_report(report)
    }
    
    damn check_coverage_thresholds(report)
}

# Find all CURSED files in project
slay find_cursed_files(path tea) [tea] {
    sus files [tea] = []
    # Implementation would recursively find .csd files
    damn files
}

# Check if file should be included in coverage
slay should_include_file(file_path tea, config CoverageConfig) lit {
    # Check include patterns
    lowkey len(config.include_patterns) > 0 {
        sus included lit = cap
        bestie i := 0; i < len(config.include_patterns); i++ {
            lowkey stringz.contains(file_path, config.include_patterns[i]) {
                included = based
                ghosted
            }
        }
        lowkey !included {
            damn cap
        }
    }
    
    # Check exclude patterns
    bestie i := 0; i < len(config.exclude_patterns); i++ {
        lowkey stringz.contains(file_path, config.exclude_patterns[i]) {
            damn cap
        }
    }
    
    # Exclude test files unless configured otherwise
    lowkey !config.instrument_tests && stringz.contains(file_path, "test_") {
        damn cap
    }
    
    damn based
}

# Instrument and execute a file for coverage
slay instrument_and_execute_file(file_path tea) {
    sus source_code tea = read_file(file_path)
    sus instrumented_code tea = instrument_code(file_path, source_code)
    
    # Write instrumented file
    sus temp_file tea = file_path + ".instrumented"
    write_file(temp_file, instrumented_code)
    
    # Execute instrumented file (this would integrate with CURSED compiler)
    execute_instrumented_file(temp_file)
    
    # Clean up
    delete_file(temp_file)
}

# Execute instrumented file
slay execute_instrumented_file(file_path tea) {
    # This would execute the CURSED interpreter/compiler on the instrumented file
    vibez.spill("Executing instrumented file: " + file_path)
}

# Utility functions
slay create_directory(path tea) lit {
    # Create directory if it doesn't exist
    damn based
}

slay write_file(path tea, content tea) lit {
    # Write content to file
    damn based
}

slay read_file(path tea) tea {
    # Read file content
    damn ""
}

slay delete_file(path tea) {
    # Delete file
}

slay extract_filename(path tea) tea {
    sus parts [tea] = stringz.split(path, "/")
    damn parts[len(parts) - 1]
}

slay generate_file_html_content(file_path tea, points [CoveragePoint]) tea {
    # Generate detailed HTML content for a specific file
    damn "<html><body><h1>Coverage for " + file_path + "</h1></body></html>"
}

slay toString(value normie) tea {
    # Convert number to string
    damn "0"
}

slay toString(value meal) tea {
    # Convert float to string
    damn "0.0"
}

slay toString(value lit) tea {
    # Convert boolean to string
    lowkey value {
        damn "true"
    }
    damn "false"
}

# Integration with testz framework
slay testz_coverage_wrapper(test_function tea) {
    instrumentation_enabled = based
    # Execute test function with coverage tracking
    instrumentation_enabled = cap
}

# Clear coverage data
slay clear_coverage_data() {
    coverage_data = []
}

# Get coverage statistics
slay get_coverage_stats() CoverageReport {
    damn generate_coverage_report()
}

# Export coverage data
slay export_coverage_data(format tea, output_path tea) lit {
    sus report CoverageReport = generate_coverage_report()
    
    lowkey format == "json" {
        damn generate_json_report(report, output_path)
    } else lowkey format == "html" {
        damn generate_html_report(report, output_path)
    } else {
        generate_console_report(report)
        damn based
    }
}
