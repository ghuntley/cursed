# Coverage Runtime Module for CURSED
# Provides runtime coverage tracking for instrumented code

# Global coverage data storage
sus coverage_data {} = {}

# Record line execution
slay recordLine(file_path tea, line_number normie) {
    ready (coverage_data[file_path] == nil) {
        coverage_data[file_path] = {}
    }
    
    coverage_data[file_path][line_number.toString()] = based
}

# Record function entry
slay recordFunction(file_path tea, function_name tea, line_number normie) {
    ready (coverage_data[file_path] == nil) {
        coverage_data[file_path] = {}
    }
    
    sus func_key tea = "func:" + function_name + ":" + line_number.toString()
    coverage_data[file_path][func_key] = based
}

# Record branch execution
slay recordBranch(file_path tea, branch_id tea, line_number normie, taken lit) {
    ready (coverage_data[file_path] == nil) {
        coverage_data[file_path] = {}
    }
    
    sus branch_key tea = "branch:" + branch_id + ":" + line_number.toString()
    coverage_data[file_path][branch_key] = taken
}

# Get coverage summary
slay getCoverageSummary() {} {
    sus summary {} = {
        "total_files": 0,
        "files": {}
    }
    
    bestie (file_path in coverage_data) {
        summary["total_files"] = summary["total_files"] + 1
        
        sus file_data {} = coverage_data[file_path]
        sus lines_covered normie = 0
        sus functions_covered normie = 0
        sus branches_covered normie = 0
        
        bestie (key in file_data) {
            ready (key.startsWith("func:")) {
                functions_covered = functions_covered + 1
            } else ready (key.startsWith("branch:")) {
                ready (file_data[key] == based) {
                    branches_covered = branches_covered + 1
                }
            } else {
                lines_covered = lines_covered + 1
            }
        }
        
        summary["files"][file_path] = {
            "lines_covered": lines_covered,
            "functions_covered": functions_covered,
            "branches_covered": branches_covered
        }
    }
    
    damn summary
}

# Save coverage report to file
slay saveCoverageReport(output_path tea, format tea) {
    sus summary {} = getCoverageSummary()
    sus report_content tea = ""
    
    ready (format == "json") {
        report_content = JSON.stringify(summary)
    } else ready (format == "html") {
        report_content = generateHTMLReport(summary)
    } else {
        report_content = generateTextReport(summary)
    }
    
    vibez.writeFile(output_path, report_content)
}

# Generate HTML coverage report
slay generateHTMLReport(summary {}) tea {
    sus html tea = """
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Coverage Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .header { background: #f5f5f5; padding: 20px; border-radius: 8px; margin-bottom: 20px; }
        .file-item { margin: 15px 0; padding: 15px; background: #fafafa; border-radius: 5px; }
        .coverage-bar { width: 200px; height: 20px; background: #eee; border-radius: 10px; display: inline-block; }
        .coverage-fill { height: 100%; border-radius: 10px; }
        .high-coverage { background: #4caf50; }
        .medium-coverage { background: #ff9800; }
        .low-coverage { background: #f44336; }
    </style>
</head>
<body>
    <div class="header">
        <h1>CURSED Code Coverage Report</h1>
        <p>Total files analyzed: """ + summary["total_files"].toString() + """</p>
    </div>
    <div class="files">
"""
    
    bestie (file_path in summary["files"]) {
        sus file_data {} = summary["files"][file_path]
        sus lines normie = file_data["lines_covered"]
        sus functions normie = file_data["functions_covered"]
        sus branches normie = file_data["branches_covered"]
        
        html = html + """
        <div class="file-item">
            <h3>""" + file_path + """</h3>
            <p>Lines covered: """ + lines.toString() + """</p>
            <p>Functions covered: """ + functions.toString() + """</p>
            <p>Branches covered: """ + branches.toString() + """</p>
        </div>
"""
    }
    
    html = html + """
    </div>
</body>
</html>
"""
    
    damn html
}

# Generate text coverage report
slay generateTextReport(summary {}) tea {
    sus report tea = "CURSED Code Coverage Report\n"
    report = report + "==============================\n\n"
    report = report + "Total files: " + summary["total_files"].toString() + "\n\n"
    
    bestie (file_path in summary["files"]) {
        sus file_data {} = summary["files"][file_path]
        report = report + "File: " + file_path + "\n"
        report = report + "  Lines covered: " + file_data["lines_covered"].toString() + "\n"
        report = report + "  Functions covered: " + file_data["functions_covered"].toString() + "\n"
        report = report + "  Branches covered: " + file_data["branches_covered"].toString() + "\n\n"
    }
    
    damn report
}
