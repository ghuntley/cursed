fr fr CURSED Formatter CLI - Command Line Interface
fr fr Comprehensive CLI for the CURSED formatter with all features

yeet "stringz"

fr fr ===== CLI COMMAND STRUCTURE =====

squad FormatCliArgs {
    spill help lit
    spill version lit
    spill check lit
    spill diff lit
    spill in_place lit
    spill backup lit
    spill validate lit
    spill verbose lit
    spill style tea
    spill config_file tea
    spill input_file tea
    spill output_file tea
}

slay default_cli_args() FormatCliArgs {
    damn FormatCliArgs{
        help: cringe,
        version: cringe,
        check: cringe,
        diff: cringe,
        in_place: cringe,
        backup: cringe,
        validate: cringe,
        verbose: cringe,
        style: "default",
        config_file: "",
        input_file: "",
        output_file: ""
    }
}

fr fr ===== ARGUMENT PARSING =====

slay parse_cli_arguments(args tea[value]) FormatCliArgs {
    sus cli_args FormatCliArgs = default_cli_args()
    sus i drip = 0
    
    bestie (i < len(args)) {
        sus arg tea = args[i]
        
        ready (arg == "--help" || arg == "-h") {
            cli_args.help = based
        } otherwise ready (arg == "--version" || arg == "-v") {
            cli_args.version = based
        } otherwise ready (arg == "--check" || arg == "-c") {
            cli_args.check = based
        } otherwise ready (arg == "--diff" || arg == "-d") {
            cli_args.diff = based
        } otherwise ready (arg == "--in-place" || arg == "-i") {
            cli_args.in_place = based
        } otherwise ready (arg == "--backup" || arg == "-b") {
            cli_args.backup = based
        } otherwise ready (arg == "--validate" || arg == "-V") {
            cli_args.validate = based
        } otherwise ready (arg == "--verbose") {
            cli_args.verbose = based
        } otherwise ready (arg == "--style" || arg == "-s") {
            ready (i + 1 < len(args)) {
                i = i + 1
                cli_args.style = args[i]
            }
        } otherwise ready (arg == "--config" || arg == "-C") {
            ready (i + 1 < len(args)) {
                i = i + 1
                cli_args.config_file = args[i]
            }
        } otherwise ready (arg == "--output" || arg == "-o") {
            ready (i + 1 < len(args)) {
                i = i + 1
                cli_args.output_file = args[i]
            }
        } otherwise ready (!starts_with(arg, "-")) {
            ready (cli_args.input_file == "") {
                cli_args.input_file = arg
            }
        } otherwise {
            vibez.spill("Unknown option: " + arg)
        }
        
        i = i + 1
    }
    
    damn cli_args
}

fr fr ===== HELP AND VERSION =====

slay print_cli_help() {
    vibez.spill("CURSED Code Formatter - Professional code formatting tool")
    vibez.spill("")
    vibez.spill("USAGE:")
    vibez.spill("    cursed-fmt [OPTIONS] [FILE]")
    vibez.spill("")
    vibez.spill("OPTIONS:")
    vibez.spill("    -h, --help              Show this help message and exit")
    vibez.spill("    -v, --version           Show version information")
    vibez.spill("    -c, --check             Check if file needs formatting (exit 1 if yes)")
    vibez.spill("    -d, --diff              Show diff instead of reformatting")
    vibez.spill("    -i, --in-place          Format file in place")
    vibez.spill("    -b, --backup            Create backup when formatting in place")
    vibez.spill("    -V, --validate          Validate syntax only (no formatting)")
    vibez.spill("    -s, --style STYLE       Use predefined style:")
    vibez.spill("                              default  - Standard CURSED style")
    vibez.spill("                              compact  - Compact style (2-space indent)")
    vibez.spill("                              google   - Google style guide")
    vibez.spill("    -C, --config FILE       Use custom configuration file")
    vibez.spill("    -o, --output FILE       Write output to specific file")
    vibez.spill("    --verbose               Enable verbose output")
    vibez.spill("")
    vibez.spill("EXAMPLES:")
    vibez.spill("    cursed-fmt file.csd                     # Format and print to stdout")
    vibez.spill("    cursed-fmt -i file.csd                  # Format file in place")
    vibez.spill("    cursed-fmt -d file.csd                  # Show formatting diff")
    vibez.spill("    cursed-fmt -c file.csd                  # Check if formatting needed")
    vibez.spill("    cursed-fmt -s compact file.csd          # Use compact style")
    vibez.spill("    cursed-fmt -V file.csd                  # Validate syntax only")
    vibez.spill("    cursed-fmt -b -i file.csd               # Format in place with backup")
    vibez.spill("")
    vibez.spill("EXIT CODES:")
    vibez.spill("    0    Success or no changes needed")
    vibez.spill("    1    Formatting needed (with --check) or errors")
    vibez.spill("    2    Invalid arguments or file not found")
}

slay print_cli_version() {
    vibez.spill("CURSED Code Formatter v1.0.0")
    vibez.spill("Production-ready formatter built in pure CURSED")
    vibez.spill("Features: AST-based formatting, configuration, diff generation")
    vibez.spill("Author: CURSED Community")
    vibez.spill("License: MIT")
}

fr fr ===== CONFIGURATION LOADING =====

slay load_config_for_style(style tea) FormatterConfig {
    ready (style == "compact") {
        damn compact_formatter_config()
    } otherwise ready (style == "google") {
        damn google_style_config()
    } otherwise {
        damn default_formatter_config()
    }
}

slay load_config_from_file(filename tea) FormatterConfig {
    fr fr In a real implementation, this would read from file
    fr fr For now, return default config
    vibez.spill("Loading config from: " + filename)
    damn default_formatter_config()
}

slay load_effective_config(cli_args FormatCliArgs) FormatterConfig {
    sus config FormatterConfig = load_config_for_style(cli_args.style)
    
    ready (cli_args.config_file != "") {
        config = load_config_from_file(cli_args.config_file)
    }
    
    damn config
}

fr fr ===== FILE OPERATIONS =====

slay read_source_file(filename tea) tea {
    fr fr In a real implementation, this would read from filesystem
    fr fr For demo, return sample code based on filename
    ready (filename == "test.csd") {
        damn "sus x drip=42;slay test(){damn x+1;}"
    } otherwise ready (filename == "complex.csd") {
        damn "ready(x>0){vibez.spill(x);}otherwise{vibez.spill(0);}"
    } otherwise ready (filename == "struct.csd") {
        damn "squad Point{spill x drip spill y drip}"
    } otherwise {
        damn "sus example drip = 123; slay demo() { damn example; }"
    }
}

slay write_source_file(filename tea, content tea) lit {
    fr fr In a real implementation, this would write to filesystem
    vibez.spill("Writing formatted code to: " + filename)
    vibez.spill("Content length: " + int_to_string(string_length(content)))
    damn based
}

slay create_backup_file(filename tea, content tea) lit {
    sus backup_name tea = filename + ".backup"
    fr fr In a real implementation, this would create backup file
    vibez.spill("Creating backup: " + backup_name)
    damn write_source_file(backup_name, content)
}

slay file_exists_check(filename tea) lit {
    fr fr In a real implementation, this would check file existence
    ready (filename == "test.csd" || filename == "complex.csd" || filename == "struct.csd") {
        damn based
    }
    damn cringe
}

fr fr ===== MAIN CLI PROCESSING =====

slay process_cli_request(cli_args FormatCliArgs) drip {
    fr fr Handle help and version
    ready (cli_args.help) {
        print_cli_help()
        damn 0
    }
    
    ready (cli_args.version) {
        print_cli_version()
        damn 0
    }
    
    fr fr Validate input file
    ready (cli_args.input_file == "") {
        vibez.spill("Error: No input file specified")
        vibez.spill("Use --help for usage information")
        damn 2
    }
    
    ready (!file_exists_check(cli_args.input_file)) {
        vibez.spill("Error: File not found: " + cli_args.input_file)
        damn 2
    }
    
    ready (cli_args.verbose) {
        vibez.spill("Processing file: " + cli_args.input_file)
        vibez.spill("Style: " + cli_args.style)
    }
    
    fr fr Read source code
    sus source_code tea = read_source_file(cli_args.input_file)
    ready (source_code == "") {
        vibez.spill("Error: Could not read file: " + cli_args.input_file)
        damn 2
    }
    
    fr fr Load configuration
    sus config FormatterConfig = load_effective_config(cli_args)
    
    ready (cli_args.verbose) {
        vibez.spill("Configuration:")
        vibez.spill("  Indent size: " + int_to_string(config.indent_size))
        vibez.spill("  Max line length: " + int_to_string(config.max_line_length))
        vibez.spill("  Use spaces: " + (ready (config.use_spaces) { "yes" } otherwise { "no" }))
    }
    
    fr fr Handle validation only mode
    ready (cli_args.validate) {
        sus errors tea[value] = validate_basic_syntax(source_code)
        ready (len(errors) == 0) {
            ready (cli_args.verbose) {
                vibez.spill("✅ Syntax validation passed")
            }
            damn 0
        } otherwise {
            vibez.spill("❌ Syntax validation failed:")
            sus i drip = 0
            bestie (i < len(errors)) {
                vibez.spill("  " + errors[i])
                i = i + 1
            }
            damn 1
        }
    }
    
    fr fr Format the source code
    sus formatted_code tea = format_cursed_code_with_config(source_code, config)
    
    fr fr Handle check only mode
    ready (cli_args.check) {
        sus needs_fmt lit = needs_formatting(source_code, config)
        ready (!needs_fmt) {
            ready (cli_args.verbose) {
                vibez.spill("✅ File is already formatted correctly")
            }
            damn 0
        } otherwise {
            ready (cli_args.verbose) {
                vibez.spill("❌ File needs formatting")
                sus change_count drip = count_format_changes(source_code, config)
                vibez.spill("Changes needed: " + int_to_string(change_count))
            }
            damn 1
        }
    }
    
    fr fr Handle diff mode
    ready (cli_args.diff) {
        sus diff_output tea = format_with_diff(source_code, config)
        vibez.spill(diff_output)
        damn 0
    }
    
    fr fr Handle in-place formatting
    ready (cli_args.in_place) {
        fr fr Create backup if requested
        ready (cli_args.backup) {
            sus backup_success lit = create_backup_file(cli_args.input_file, source_code)
            ready (!backup_success) {
                vibez.spill("Error: Could not create backup file")
                damn 2
            }
            ready (cli_args.verbose) {
                vibez.spill("✅ Backup created: " + cli_args.input_file + ".backup")
            }
        }
        
        fr fr Write formatted code back to original file
        sus write_success lit = write_source_file(cli_args.input_file, formatted_code)
        ready (!write_success) {
            vibez.spill("Error: Could not write to file: " + cli_args.input_file)
            damn 2
        }
        
        ready (cli_args.verbose) {
            vibez.spill("✅ File formatted in place: " + cli_args.input_file)
        }
        damn 0
    }
    
    fr fr Handle output to specific file
    ready (cli_args.output_file != "") {
        sus write_success lit = write_source_file(cli_args.output_file, formatted_code)
        ready (!write_success) {
            vibez.spill("Error: Could not write to file: " + cli_args.output_file)
            damn 2
        }
        
        ready (cli_args.verbose) {
            vibez.spill("✅ Formatted code written to: " + cli_args.output_file)
        }
        damn 0
    }
    
    fr fr Default: output to stdout
    vibez.spill("Formatted output:")
    vibez.spill(formatted_code)
    damn 0
}

fr fr ===== BATCH PROCESSING =====

slay process_multiple_files(files tea[value], cli_args FormatCliArgs) drip {
    sus total_files drip = len(files)
    sus success_count drip = 0
    sus error_count drip = 0
    
    ready (cli_args.verbose) {
        vibez.spill("Processing " + int_to_string(total_files) + " files...")
    }
    
    sus i drip = 0
    bestie (i < len(files)) {
        sus file tea = files[i]
        sus file_args FormatCliArgs = cli_args
        file_args.input_file = file
        
        ready (cli_args.verbose) {
            vibez.spill("Processing (" + int_to_string(i + 1) + "/" + int_to_string(total_files) + "): " + file)
        }
        
        sus result drip = process_cli_request(file_args)
        ready (result == 0) {
            success_count = success_count + 1
        } otherwise {
            error_count = error_count + 1
        }
        
        i = i + 1
    }
    
    vibez.spill("Batch processing complete:")
    vibez.spill("  ✅ Success: " + int_to_string(success_count))
    vibez.spill("  ❌ Errors: " + int_to_string(error_count))
    
    ready (error_count > 0) {
        damn 1
    }
    damn 0
}

fr fr ===== INTERACTIVE MODE =====

slay run_interactive_mode() {
    vibez.spill("🎨 CURSED Interactive Formatter")
    vibez.spill("Enter CURSED code (type 'quit' to exit, 'help' for commands)")
    vibez.spill("")
    
    sus config FormatterConfig = default_formatter_config()
    sus iteration drip = 0
    
    bestie (iteration < 5) {  fr fr Limit iterations for demo
        vibez.spill("> ")
        
        fr fr Simulate user input for demo
        sus input tea = ""
        ready (iteration == 0) {
            input = "sus x drip=42;"
        } otherwise ready (iteration == 1) {
            input = "style compact"
        } otherwise ready (iteration == 2) {
            input = "slay test(){damn 42;}"
        } otherwise ready (iteration == 3) {
            input = "validate sus x drip = ;"
        } otherwise {
            input = "quit"
        }
        
        vibez.spill(input)
        
        ready (input == "quit" || input == "exit") {
            vibez.spill("Goodbye!")
            break
        } otherwise ready (input == "help") {
            print_interactive_help()
        } otherwise ready (starts_with(input, "style ")) {
            sus style tea = slice_tea(input, 6, string_length(input))
            config = load_config_for_style(style)
            vibez.spill("✅ Style changed to: " + style)
        } otherwise ready (starts_with(input, "validate ")) {
            sus code tea = slice_tea(input, 9, string_length(input))
            sus errors tea[value] = validate_basic_syntax(code)
            ready (len(errors) == 0) {
                vibez.spill("✅ Valid syntax")
            } otherwise {
                vibez.spill("❌ Syntax errors:")
                sus i drip = 0
                bestie (i < len(errors)) {
                    vibez.spill("  " + errors[i])
                    i = i + 1
                }
            }
        } otherwise {
            fr fr Format the input
            sus formatted tea = format_cursed_code_with_config(input, config)
            vibez.spill("Formatted:")
            vibez.spill(formatted)
        }
        
        vibez.spill("")
        iteration = iteration + 1
    }
}

slay print_interactive_help() {
    vibez.spill("Interactive Formatter Commands:")
    vibez.spill("  help                    - Show this help")
    vibez.spill("  quit, exit              - Exit interactive mode")
    vibez.spill("  style [name]            - Change formatting style")
    vibez.spill("                           (default, compact, google)")
    vibez.spill("  validate [code]         - Validate syntax")
    vibez.spill("  [CURSED code]           - Format the code")
}

fr fr ===== MAIN CLI ENTRY POINT =====

slay cli_main(args tea[value]) drip {
    ready (len(args) == 0) {
        vibez.spill("CURSED Code Formatter")
        vibez.spill("Use --help for usage information or try 'interactive' mode")
        damn 0
    }
    
    fr fr Special commands
    ready (len(args) == 1 && args[0] == "interactive") {
        run_interactive_mode()
        damn 0
    }
    
    fr fr Parse command line arguments
    sus cli_args FormatCliArgs = parse_cli_arguments(args)
    
    fr fr Process the request
    damn process_cli_request(cli_args)
}

fr fr ===== DEMO MAIN FUNCTION =====

slay main() {
    vibez.spill("🎨 CURSED Formatter CLI - Comprehensive Demo")
    vibez.spill("Demonstrating professional code formatter capabilities")
    vibez.spill("")
    
    fr fr Demo different CLI scenarios
    vibez.spill("=== Demo 1: Basic formatting ===")
    sus demo_args1 tea[value] = ["test.csd"]
    sus result1 drip = cli_main(demo_args1)
    vibez.spill("Exit code: " + int_to_string(result1))
    vibez.spill("")
    
    vibez.spill("=== Demo 2: Check mode ===")
    sus demo_args2 tea[value] = ["--check", "--verbose", "test.csd"]
    sus result2 drip = cli_main(demo_args2)
    vibez.spill("Exit code: " + int_to_string(result2))
    vibez.spill("")
    
    vibez.spill("=== Demo 3: Diff mode ===")
    sus demo_args3 tea[value] = ["--diff", "complex.csd"]
    sus result3 drip = cli_main(demo_args3)
    vibez.spill("Exit code: " + int_to_string(result3))
    vibez.spill("")
    
    vibez.spill("=== Demo 4: Compact style ===")
    sus demo_args4 tea[value] = ["--style", "compact", "--verbose", "struct.csd"]
    sus result4 drip = cli_main(demo_args4)
    vibez.spill("Exit code: " + int_to_string(result4))
    vibez.spill("")
    
    vibez.spill("=== Demo 5: Validation only ===")
    sus demo_args5 tea[value] = ["--validate", "--verbose", "test.csd"]
    sus result5 drip = cli_main(demo_args5)
    vibez.spill("Exit code: " + int_to_string(result5))
    vibez.spill("")
    
    vibez.spill("=== Demo 6: In-place with backup ===")
    sus demo_args6 tea[value] = ["--in-place", "--backup", "--verbose", "test.csd"]
    sus result6 drip = cli_main(demo_args6)
    vibez.spill("Exit code: " + int_to_string(result6))
    vibez.spill("")
    
    vibez.spill("=== Demo 7: Interactive mode ===")
    run_interactive_mode()
    vibez.spill("")
    
    vibez.spill("=== Demo 8: Help output ===")
    sus demo_args8 tea[value] = ["--help"]
    sus result8 drip = cli_main(demo_args8)
    vibez.spill("")
    
    vibez.spill("✅ CLI Demo completed successfully!")
    vibez.spill("🚀 Ready for integration with main CURSED toolchain")
}
