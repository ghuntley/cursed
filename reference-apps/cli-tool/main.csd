# CURSED CLI Tool - File Organization and Search Utility
# Demonstrates: CLI parsing, file operations, pattern matching, error handling

yeet "vibez"
yeet "filez" 
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "timez"

# CLI argument structure
squad CliArgs {
    command tea
    path tea
    pattern tea
    recursive lit
    show_help lit
}

# File information structure  
squad FileInfo {
    path tea
    size drip
    modified drip
    is_directory lit
}

# Parse command line arguments
slay parse_args(args []tea) CliArgs {
    sus cli_args CliArgs = {
        command: "",
        path: ".",
        pattern: "*",
        recursive: false,
        show_help: false
    }
    
    sus i drip = 0
    bestie (i < len(args)) {
        sus arg tea = args[i]
        
        ready (stringz.starts_with(arg, "--")) {
            sick (arg) {
                when "--help" -> cli_args.show_help = based
                when "--recursive" -> cli_args.recursive = based
                when "--pattern" -> {
                    ready (i + 1 < len(args)) {
                        i = i + 1
                        cli_args.pattern = args[i]
                    }
                }
                when "--path" -> {
                    ready (i + 1 < len(args)) {
                        i = i + 1
                        cli_args.path = args[i]
                    }
                }
            }
        } otherwise {
            ready (stringz.len(cli_args.command) == 0) {
                cli_args.command = arg
            }
        }
        
        i = i + 1
    }
    
    damn cli_args
}

# Display help information
slay show_help() {
    vibez.spill("CURSED File Tool v1.0")
    vibez.spill("Usage: cursed-file-tool <command> [options]")
    vibez.spill("")
    vibez.spill("Commands:")
    vibez.spill("  list      List files in directory")
    vibez.spill("  search    Search for files matching pattern")
    vibez.spill("  organize  Organize files by extension")
    vibez.spill("  stats     Show directory statistics")
    vibez.spill("")
    vibez.spill("Options:")
    vibez.spill("  --path <path>      Target directory (default: current)")
    vibez.spill("  --pattern <pat>    File pattern to match (default: *)")
    vibez.spill("  --recursive        Search subdirectories")
    vibez.spill("  --help            Show this help")
    vibez.spill("")
    vibez.spill("Examples:")
    vibez.spill("  cursed-file-tool list --path /home --recursive")
    vibez.spill("  cursed-file-tool search --pattern '*.csd'")
    vibez.spill("  cursed-file-tool organize --path ./downloads")
}

# Get file information
slay get_file_info(path tea) yikes<FileInfo> {
    ready (!filez.exists(path)) {
        yikes "file does not exist"
    }
    
    sus info FileInfo = {
        path: path,
        size: filez.size(path) fam { when _ -> damn 0 },
        modified: filez.modified_time(path) fam { when _ -> damn 0 },
        is_directory: filez.is_directory(path)
    }
    
    damn info
}

# List files in directory
slay list_files(path tea, recursive lit, pattern tea) {
    vibez.spill("Listing files in:", path)
    vibez.spill("Pattern:", pattern)
    vibez.spill("Recursive:", ready (recursive) { damn "yes" } otherwise { damn "no" })
    vibez.spill("")
    
    sus entries []tea = filez.list_directory(path) fam {
        when _ -> {
            vibez.spill("Error: Cannot access directory")
            damn
        }
    }
    
    sus total_files drip = 0
    sus total_size drip = 0
    
    bestie (entry in entries) {
        sus full_path tea = filez.join_path(path, entry)
        
        # Pattern matching for file names
        ready (stringz.matches_pattern(entry, pattern)) {
            sus info FileInfo = get_file_info(full_path) fam {
                when _ -> skip  # Skip files we can't access
            }
            
            # Format file size
            sus size_str tea = ready (info.size > 1024 * 1024) {
                damn stringz.format("%.1f MB", info.size / (1024.0 * 1024.0))
            } ready (info.size > 1024) {
                damn stringz.format("%.1f KB", info.size / 1024.0)
            } otherwise {
                damn stringz.format("%d B", info.size)
            }
            
            # Format modified time
            sus time_str tea = timez.format_time(info.modified, "2006-01-02 15:04")
            
            # Display file information
            ready (info.is_directory) {
                vibez.spill("[DIR ]", entry, size_str, time_str)
            } otherwise {
                vibez.spill("[FILE]", entry, size_str, time_str)
                total_files = total_files + 1
                total_size = total_size + info.size
            }
        }
        
        # Recursive directory processing
        ready (recursive && filez.is_directory(full_path) && entry != "." && entry != "..") {
            list_files(full_path, recursive, pattern)
        }
    }
    
    vibez.spill("")
    vibez.spill("Total files:", total_files)
    vibez.spill("Total size:", total_size, "bytes")
}

# Search for files matching pattern
slay search_files(path tea, pattern tea, recursive lit) {
    vibez.spill("Searching for pattern:", pattern)
    vibez.spill("In directory:", path)
    vibez.spill("")
    
    sus matches []tea = []
    sus search_recursive slay(current_path tea) {
        sus entries []tea = filez.list_directory(current_path) fam {
            when _ -> damn
        }
        
        bestie (entry in entries) {
            sus full_path tea = filez.join_path(current_path, entry)
            
            ready (stringz.matches_pattern(entry, pattern)) {
                matches = arrayz.append(matches, full_path)
                vibez.spill("Found:", full_path)
            }
            
            ready (recursive && filez.is_directory(full_path) && entry != "." && entry != "..") {
                search_recursive(full_path)
            }
        }
    }
    
    search_recursive(path)
    
    vibez.spill("")
    vibez.spill("Found", len(matches), "matching files")
}

# Organize files by extension
slay organize_files(path tea) {
    vibez.spill("Organizing files in:", path)
    
    sus extensions map<tea, drip> = {}
    sus entries []tea = filez.list_directory(path) fam {
        when _ -> {
            vibez.spill("Error: Cannot access directory")
            damn
        }
    }
    
    bestie (entry in entries) {
        sus full_path tea = filez.join_path(path, entry)
        
        ready (!filez.is_directory(full_path)) {
            sus ext tea = filez.get_extension(entry)
            ready (stringz.len(ext) == 0) {
                ext = "no_extension"
            }
            
            ready (ext in extensions) {
                extensions[ext] = extensions[ext] + 1
            } otherwise {
                extensions[ext] = 1
            }
        }
    }
    
    vibez.spill("")
    vibez.spill("File organization by extension:")
    bestie (ext, count in extensions) {
        vibez.spill("  ", ext, ":", count, "files")
    }
}

# Show directory statistics
slay show_stats(path tea) {
    vibez.spill("Directory statistics for:", path)
    vibez.spill("")
    
    sus total_files drip = 0
    sus total_dirs drip = 0
    sus total_size drip = 0
    sus largest_file tea = ""
    sus largest_size drip = 0
    
    sus entries []tea = filez.list_directory(path) fam {
        when _ -> {
            vibez.spill("Error: Cannot access directory")
            damn
        }
    }
    
    bestie (entry in entries) {
        sus full_path tea = filez.join_path(path, entry)
        sus info FileInfo = get_file_info(full_path) fam {
            when _ -> skip
        }
        
        ready (info.is_directory) {
            total_dirs = total_dirs + 1
        } otherwise {
            total_files = total_files + 1
            total_size = total_size + info.size
            
            ready (info.size > largest_size) {
                largest_size = info.size
                largest_file = entry
            }
        }
    }
    
    sus avg_size meal = ready (total_files > 0) {
        damn total_size / total_files
    } otherwise {
        damn 0.0
    }
    
    vibez.spill("Files:", total_files)
    vibez.spill("Directories:", total_dirs)
    vibez.spill("Total size:", total_size, "bytes")
    vibez.spill("Average file size:", avg_size, "bytes")
    vibez.spill("Largest file:", largest_file, "(", largest_size, "bytes)")
}

# Main entry point
slay main_character() {
    sus args []tea = vibez.get_args() fam {
        when _ -> damn []
    }
    
    sus cli_args CliArgs = parse_args(args)
    
    ready (cli_args.show_help || stringz.len(cli_args.command) == 0) {
        show_help()
        damn
    }
    
    sick (cli_args.command) {
        when "list" -> list_files(cli_args.path, cli_args.recursive, cli_args.pattern)
        when "search" -> search_files(cli_args.path, cli_args.pattern, cli_args.recursive)
        when "organize" -> organize_files(cli_args.path)
        when "stats" -> show_stats(cli_args.path)
        when _ -> {
            vibez.spill("Unknown command:", cli_args.command)
            vibez.spill("Use --help for usage information")
        }
    }
}

# Run the application
main()
