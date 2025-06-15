//! CURSED Documentation Generator
//! 
//! Standalone binary for generating comprehensive documentation for CURSED projects.
//! Provides a command-line interface for documenting CURSED source code with
//! multiple output formats and advanced features.

use clap::{Arg, Command, ArgAction, ArgMatches};
use cursed::cli::documentation::{add_documentation_commands, handle_documentation_command};
use cursed::error::Error;
use std::process;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

#[tokio::main]
async fn main() {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Build the CLI application
    let app = build_cli();
    
    // Parse command line arguments
    let matches = app.get_matches();
    
    // Handle the documentation command
    match handle_doc_command(&matches).await {
        Ok(()) => {
            info!("Documentation generation completed successfully");
        }
        Err(e) => {
            error!("Documentation generation failed: {}", e);
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

/// Build the CLI application
fn build_cli() -> Command {
    let base_cmd = Command::new("cursed-doc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Geoffrey Huntley")
        .about("CURSED Documentation Generator")
        .long_about("Generate comprehensive API documentation for CURSED projects. \
                     Supports multiple output formats including HTML, Markdown, and JSON \
                     with features like cross-references, search indexing, and custom styling.")
        .arg(
            Arg::new("input")
                .help("Input directory or file to document")
                .value_name("INPUT")
                .default_value(".")
                .long_help("The source directory or file to generate documentation from. \
                           Can be a single .csd file or a directory containing CURSED source files.")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory for generated documentation")
                .default_value("docs")
                .long_help("Directory where the generated documentation will be written. \
                           The directory will be created if it doesn't exist.")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format(s): html, markdown, json")
                .action(ArgAction::Append)
                .value_parser(["html", "markdown", "md", "json", "xml", "latex"])
                .long_help("Output format for the documentation. Can be specified multiple times \
                           to generate documentation in multiple formats simultaneously.")
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file (TOML or JSON)")
                .long_help("Path to a configuration file containing documentation generation \
                           settings. Supports both TOML and JSON formats.")
        )
        .arg(
            Arg::new("title")
                .long("title")
                .value_name("TITLE")
                .help("Documentation title")
                .default_value("CURSED Documentation")
                .long_help("The title to use for the generated documentation. \
                           Appears in HTML titles and headers.")
        )
        .arg(
            Arg::new("description")
                .long("description")
                .value_name("DESC")
                .help("Project description")
                .long_help("A brief description of the project that will appear \
                           in the documentation overview.")
        )
        .arg(
            Arg::new("version")
                .long("version")
                .value_name("VERSION")
                .help("Project version")
                .long_help("Version string for the project (e.g., '1.0.0')")
        )
        .arg(
            Arg::new("author")
                .long("author")
                .value_name("AUTHOR")
                .action(ArgAction::Append)
                .help("Project authors (can be used multiple times)")
                .long_help("Author names for the project. Can be specified multiple times \
                           for projects with multiple authors.")
        )
        .arg(
            Arg::new("homepage")
                .long("homepage")
                .value_name("URL")
                .help("Project homepage URL")
        )
        .arg(
            Arg::new("repository")
                .long("repository")
                .value_name("URL")
                .help("Project repository URL")
        )
        .arg(
            Arg::new("license")
                .long("license")
                .value_name("LICENSE")
                .help("Project license")
        )
        .arg(
            Arg::new("include-private")
                .long("include-private")
                .action(ArgAction::SetTrue)
                .help("Include private items in documentation")
                .long_help("Include private functions, types, and other items that are \
                           normally not exported. Useful for internal documentation.")
        )
        .arg(
            Arg::new("no-source")
                .long("no-source")
                .action(ArgAction::SetTrue)
                .help("Exclude source code from documentation")
                .long_help("Don't include source code snippets in the generated documentation. \
                           This can reduce file size for large projects.")
        )
        .arg(
            Arg::new("no-cross-refs")
                .long("no-cross-refs")
                .action(ArgAction::SetTrue)
                .help("Disable cross-reference generation")
                .long_help("Skip generating cross-references between documentation items. \
                           This can speed up generation for large projects.")
        )
        .arg(
            Arg::new("no-search")
                .long("no-search")
                .action(ArgAction::SetTrue)
                .help("Disable search index generation")
                .long_help("Don't generate a search index. This disables search functionality \
                           in HTML output but can speed up generation.")
        )
        .arg(
            Arg::new("no-examples")
                .long("no-examples")
                .action(ArgAction::SetTrue)
                .help("Exclude code examples from documentation")
                .long_help("Don't extract and include code examples from documentation comments.")
        )
        .arg(
            Arg::new("include-dependencies")
                .long("include-dependencies")
                .action(ArgAction::SetTrue)
                .help("Include documentation for dependencies")
                .long_help("Generate documentation for external dependencies in addition \
                           to the main project code.")
        )
        .arg(
            Arg::new("max-depth")
                .long("max-depth")
                .value_name("DEPTH")
                .help("Maximum type recursion depth")
                .default_value("10")
                .value_parser(clap::value_parser!(usize))
                .long_help("Maximum depth for recursive type documentation to prevent \
                           infinite recursion in complex type hierarchies.")
        )
        .arg(
            Arg::new("theme")
                .long("theme")
                .value_name("THEME")
                .help("HTML theme: light, dark, auto")
                .default_value("auto")
                .value_parser(["light", "dark", "auto"])
                .long_help("Color theme for HTML output. 'auto' uses system preference.")
        )
        .arg(
            Arg::new("custom-css")
                .long("custom-css")
                .value_name("FILE")
                .action(ArgAction::Append)
                .help("Custom CSS files for HTML output")
                .long_help("Path to custom CSS files to include in HTML output. \
                           Can be specified multiple times.")
        )
        .arg(
            Arg::new("template-dir")
                .long("template-dir")
                .value_name("DIR")
                .help("Custom template directory")
                .long_help("Directory containing custom templates for HTML generation. \
                           Templates should follow the expected naming conventions.")
        )
        .arg(
            Arg::new("favicon")
                .long("favicon")
                .value_name("FILE")
                .help("Custom favicon for HTML output")
        )
        .arg(
            Arg::new("logo")
                .long("logo")
                .value_name("FILE")
                .help("Custom logo for documentation")
        )
        .arg(
            Arg::new("watch")
                .short('w')
                .long("watch")
                .action(ArgAction::SetTrue)
                .help("Watch files for changes and regenerate automatically")
                .long_help("Monitor source files for changes and automatically regenerate \
                           documentation when files are modified.")
        )
        .arg(
            Arg::new("serve")
                .long("serve")
                .value_name("PORT")
                .help("Serve documentation on local HTTP server")
                .value_parser(clap::value_parser!(u16))
                .long_help("Start a local HTTP server to serve the generated documentation. \
                           Useful for previewing HTML output.")
        )
        .arg(
            Arg::new("open")
                .long("open")
                .action(ArgAction::SetTrue)
                .help("Open documentation in browser after generation")
                .long_help("Automatically open the generated documentation in the default \
                           web browser after generation completes.")
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppress output except for errors")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbose output")
                .conflicts_with("quiet")
        )
        .arg(
            Arg::new("save-config")
                .long("save-config")
                .value_name("FILE")
                .help("Save current configuration to file")
                .long_help("Save the current configuration (including command-line options) \
                           to a file for future use.")
        )
        .arg(
            Arg::new("init-config")
                .long("init-config")
                .value_name("FILE")
                .help("Create a default configuration file")
                .long_help("Create a new configuration file with default settings \
                           that can be customized for the project.")
        );

    base_cmd
}

/// Handle the documentation command
async fn handle_doc_command(matches: &ArgMatches) -> Result<(), Error> {
    handle_documentation_command(matches).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_builds() {
        let app = build_cli();
        app.debug_assert();
    }

    #[test]
    fn test_default_args() {
        let app = build_cli();
        let matches = app.try_get_matches_from(&["cursed-doc"]).unwrap();
        
        assert_eq!(matches.get_one::<String>("input").unwrap(), ".");
        assert_eq!(matches.get_one::<String>("output").unwrap(), "docs");
        assert_eq!(matches.get_one::<String>("title").unwrap(), "CURSED Documentation");
        assert_eq!(matches.get_one::<usize>("max-depth").unwrap(), &10);
        assert_eq!(matches.get_one::<String>("theme").unwrap(), "auto");
    }

    #[test]
    fn test_multiple_formats() {
        let app = build_cli();
        let matches = app.try_get_matches_from(&[
            "cursed-doc",
            "--format", "html",
            "--format", "markdown",
            "--format", "json"
        ]).unwrap();
        
        let formats: Vec<&String> = matches.get_many::<String>("format").unwrap().collect();
        assert_eq!(formats.len(), 3);
        assert!(formats.contains(&&"html".to_string()));
        assert!(formats.contains(&&"markdown".to_string()));
        assert!(formats.contains(&&"json".to_string()));
    }

    #[test]
    fn test_boolean_flags() {
        let app = build_cli();
        let matches = app.try_get_matches_from(&[
            "cursed-doc",
            "--include-private",
            "--no-source",
            "--watch",
            "--verbose"
        ]).unwrap();
        
        assert!(matches.get_flag("include-private"));
        assert!(matches.get_flag("no-source"));
        assert!(matches.get_flag("watch"));
        assert!(matches.get_flag("verbose"));
        assert!(!matches.get_flag("quiet"));
    }
}
