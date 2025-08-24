#!/usr/bin/env cursed-zig

yeet "filez"
yeet "stringz" 
yeet "vibez"
yeet "testz"

struct DocValidationResult {
    file_path tea,
    broken_links []tea,
    valid_links []tea,
    errors []tea
}

struct LinkCheck {
    link_text tea,
    target_path tea,
    exists lit,
    error_message tea
}

slay validate_documentation_links() {
    vibez.spill(vibez.color("🔍 CURSED Documentation Validation", "blue"))
    vibez.spill("=" + repeat("=", 50))
    
    sus doc_directories []tea = [
        "docs/",
        "stdlib/",
        "examples/",
        "specs/"
    ]
    
    sus total_files drip = 0
    sus total_links drip = 0
    sus broken_links drip = 0
    sus validation_results []DocValidationResult = []
    
    bestie (sus dir tea : doc_directories) {
        ready (filez.exists(dir)) {
            sus results []DocValidationResult = validate_directory(dir)
            validation_results = append(validation_results, results...)
            
            bestie (sus result DocValidationResult : results) {
                total_files++
                total_links += result.broken_links.length() + result.valid_links.length()
                broken_links += result.broken_links.length()
            }
        }
    }
    
    print_validation_summary(validation_results, total_files, total_links, broken_links)
    
    ready (broken_links > 0) {
        generate_fix_script(validation_results)
    }
}

slay validate_directory(dir_path tea) []DocValidationResult {
    sus results []DocValidationResult = []
    sus files []tea = filez.list_files_recursive(dir_path, "*.md")
    
    vibez.spill(vibez.color("📁 Validating directory: " + dir_path, "cyan"))
    
    bestie (sus file tea : files) {
        sus result DocValidationResult = validate_file(file)
        results = append(results, result)
        
        ready (result.broken_links.length() > 0) {
            vibez.spill("  " + vibez.color("❌ " + file, "red") + 
                       " (" + result.broken_links.length().(tea) + " broken links)")
        } otherwise {
            vibez.spill("  " + vibez.color("✅ " + file, "green"))
        }
    }
    
    damn results
}

slay validate_file(file_path tea) DocValidationResult {
    sus content tea = filez.read_file(file_path) fam {
        when _ -> damn DocValidationResult{
            file_path: file_path,
            errors: ["Failed to read file"]
        }
    }
    
    sus links []LinkCheck = extract_markdown_links(content, file_path)
    sus broken_links []tea = []
    sus valid_links []tea = []
    
    bestie (sus link LinkCheck : links) {
        ready (link.exists) {
            valid_links = append(valid_links, link.link_text + " -> " + link.target_path)
        } otherwise {
            broken_links = append(broken_links, link.link_text + " -> " + link.target_path + 
                                " (" + link.error_message + ")")
        }
    }
    
    damn DocValidationResult{
        file_path: file_path,
        broken_links: broken_links,
        valid_links: valid_links,
        errors: []
    }
}

slay extract_markdown_links(content tea, base_path tea) []LinkCheck {
    sus links []LinkCheck = []
    sus lines []tea = stringz.split(content, "\n")
    
    bestie (sus line tea : lines) {
        sus markdown_links []tea = find_markdown_links(line)
        
        bestie (sus link_match tea : markdown_links) {
            sus link_info LinkCheck = parse_markdown_link(link_match, base_path)
            links = append(links, link_info)
        }
    }
    
    damn links
}

slay find_markdown_links(line tea) []tea {
    sus links []tea = []
    sus pattern tea = "\\[([^\\]]+)\\]\\(([^\\)]+)\\)"  // [text](url)
    
    // Simple regex-like extraction (CURSED doesn't have regex yet)
    sus pos drip = 0
    bestie (pos < line.length()) {
        sus start drip = stringz.find(line, "[", pos)
        ready (start == -1) { break }
        
        sus end_bracket drip = stringz.find(line, "]", start)
        ready (end_bracket == -1) { break }
        
        sus paren_start drip = end_bracket + 1
        ready (paren_start >= line.length() || line[paren_start] != "(") {
            pos = start + 1
            continue
        }
        
        sus paren_end drip = stringz.find(line, ")", paren_start)
        ready (paren_end == -1) { break }
        
        sus link tea = line.substring(start, paren_end + 1)
        links = append(links, link)
        pos = paren_end + 1
    }
    
    damn links
}

slay parse_markdown_link(link_text tea, base_path tea) LinkCheck {
    // Extract [text](url) format
    sus bracket_end drip = stringz.find(link_text, "]")
    sus paren_start drip = bracket_end + 2  // Skip "]("
    sus paren_end drip = stringz.find(link_text, ")", paren_start)
    
    sus text tea = link_text.substring(1, bracket_end)
    sus url tea = link_text.substring(paren_start, paren_end)
    
    // Skip external URLs
    ready (stringz.starts_with(url, "http://") || stringz.starts_with(url, "https://")) {
        damn LinkCheck{
            link_text: text,
            target_path: url,
            exists: based,  // Assume external links are valid
            error_message: ""
        }
    }
    
    // Skip anchors and fragments
    ready (stringz.starts_with(url, "#")) {
        damn LinkCheck{
            link_text: text,
            target_path: url,
            exists: based,  // Assume anchors are valid
            error_message: ""
        }
    }
    
    // Resolve relative path
    sus base_dir tea = filez.get_directory(base_path)
    sus resolved_path tea = resolve_relative_path(base_dir, url)
    
    // Check if file exists
    sus exists lit = filez.exists(resolved_path)
    sus error_msg tea = ready (exists) { "" } otherwise { "File not found" }
    
    damn LinkCheck{
        link_text: text,
        target_path: resolved_path,
        exists: exists,
        error_message: error_msg
    }
}

slay resolve_relative_path(base_dir tea, relative_path tea) tea {
    // Handle fragments (#section)
    sus fragment_pos drip = stringz.find(relative_path, "#")
    sus clean_path tea = ready (fragment_pos != -1) {
        relative_path.substring(0, fragment_pos)
    } otherwise {
        relative_path
    }
    
    // Handle parent directory references
    sus path_parts []tea = stringz.split(base_dir, "/")
    sus relative_parts []tea = stringz.split(clean_path, "/")
    
    bestie (sus part tea : relative_parts) {
        ready (part == "..") {
            ready (path_parts.length() > 0) {
                path_parts = path_parts[:path_parts.length()-1]
            }
        } otherwise ready (part != "." && part != "") {
            path_parts = append(path_parts, part)
        }
    }
    
    damn stringz.join(path_parts, "/")
}

slay print_validation_summary(results []DocValidationResult, total_files drip, 
                              total_links drip, broken_count drip) {
    vibez.spill("")
    vibez.spill(vibez.color("📊 Validation Summary", "blue"))
    vibez.spill("─" + repeat("─", 40))
    
    sus success_rate drip = ready (total_links > 0) {
        ((total_links - broken_count) * 100) / total_links
    } otherwise { 100 }
    
    vibez.spill("Files processed: " + total_files.(tea))
    vibez.spill("Total links: " + total_links.(tea))
    vibez.spill("Valid links: " + (total_links - broken_count).(tea))
    vibez.spill("Broken links: " + vibez.color(broken_count.(tea), "red"))
    vibez.spill("Success rate: " + vibez.color(success_rate.(tea) + "%", 
               ready (success_rate >= 90) { "green" } otherwise { "red" }))
    
    ready (broken_count > 0) {
        vibez.spill("")
        vibez.spill(vibez.color("🔧 Broken Links Details:", "yellow"))
        
        bestie (sus result DocValidationResult : results) {
            ready (result.broken_links.length() > 0) {
                vibez.spill("")
                vibez.spill("📄 " + vibez.bold(result.file_path))
                
                bestie (sus broken_link tea : result.broken_links) {
                    vibez.spill("  ❌ " + broken_link)
                }
            }
        }
    }
}

slay generate_fix_script(results []DocValidationResult) {
    vibez.spill("")
    vibez.spill(vibez.color("🛠️  Generating fix script...", "blue"))
    
    sus fix_script tea = "#!/bin/bash\n"
    fix_script += "# Auto-generated documentation fix script\n"
    fix_script += "echo \"Fixing broken documentation links...\"\n\n"
    
    sus fixes_available drip = 0
    
    bestie (sus result DocValidationResult : results) {
        bestie (sus broken_link tea : result.broken_links) {
            sus suggested_fix tea = suggest_fix(broken_link, result.file_path)
            ready (suggested_fix != "") {
                fix_script += "# Fix for " + result.file_path + "\n"
                fix_script += suggested_fix + "\n\n"
                fixes_available++
            }
        }
    }
    
    ready (fixes_available > 0) {
        filez.write_file("fix_docs.sh", fix_script)
        vibez.spill(vibez.color("✨ Generated fix_docs.sh with " + 
                               fixes_available.(tea) + " suggested fixes", "green"))
        vibez.spill("Run: chmod +x fix_docs.sh && ./fix_docs.sh")
    } otherwise {
        vibez.spill(vibez.color("ℹ️  No automated fixes available", "yellow"))
        vibez.spill("Manual intervention required for broken links")
    }
}

slay suggest_fix(broken_link tea, file_path tea) tea {
    // Extract target path from broken link description
    sus arrow_pos drip = stringz.find(broken_link, " -> ")
    ready (arrow_pos == -1) { damn "" }
    
    sus target tea = broken_link.substring(arrow_pos + 4)
    sus paren_pos drip = stringz.find(target, " (")
    ready (paren_pos != -1) {
        target = target.substring(0, paren_pos)
    }
    
    // Common fix patterns
    ready (stringz.contains(target, "BUILD_SYSTEM_README.md")) {
        damn "# BUILD_SYSTEM_README.md already created"
    }
    
    ready (stringz.contains(target, "CHANGELOG.md") && !stringz.starts_with(target, "/")) {
        damn "# CHANGELOG.md already created at root"
    }
    
    ready (stringz.ends_with(target, ".md") && !filez.exists(target)) {
        damn "mkdir -p \"$(dirname '" + target + "')\" && touch '" + target + "'"
    }
    
    damn ""
}

slay repeat(char tea, count drip) tea {
    sus result tea = ""
    bestie (sus i drip = 0; i < count; i++) {
        result += char
    }
    damn result
}

slay main() {
    testz.start_suite("Documentation Validation")
    validate_documentation_links()
    testz.print_summary()
}
