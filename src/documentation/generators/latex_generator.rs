//! LaTeX Documentation Generator
//! 
//! A comprehensive LaTeX documentation generator for the CURSED programming language
//! that produces professional academic-style documentation with syntax highlighting,
//! mathematical notation, cross-references, and support for multiple document classes.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn};

/// LaTeX document classes supported by the generator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentClass {
    /// Article class for smaller documentation
    Article,
    /// Report class for comprehensive documentation  
    Report,
    /// Book class for complete language documentation
    Book,
    /// Beamer class for presentation slides
    Beamer,
}

impl DocumentClass {
    /// Get the LaTeX document class string
    pub fn to_latex(&self) -> &'static str {
        match self {
            DocumentClass::Article => "article",
            DocumentClass::Report => "report", 
            DocumentClass::Book => "book",
            DocumentClass::Beamer => "beamer",
        }
    }

    /// Get default options for the document class
    pub fn default_options(&self) -> Vec<&'static str> {
        match self {
            DocumentClass::Article => vec!["11pt", "a4paper"],
            DocumentClass::Report => vec!["11pt", "a4paper", "twoside"],
            DocumentClass::Book => vec!["11pt", "a4paper", "twoside", "openright"],
            DocumentClass::Beamer => vec!["11pt", "aspectratio=169"],
        }
    }
}

/// Syntax highlighting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxHighlighting {
    /// Whether to use minted (requires Python pygments) or listings package
    pub use_minted: bool,
    /// Color scheme for syntax highlighting
    pub color_scheme: ColorScheme,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Line number style (left, right, none)
    pub line_number_style: String,
    /// Tab size for code formatting
    pub tab_size: usize,
    /// Whether to break long lines
    pub break_lines: bool,
}

impl Default for SyntaxHighlighting {
    fn default() -> Self {
        Self {
            use_minted: false, // Default to listings for better compatibility
            color_scheme: ColorScheme::default(),
            show_line_numbers: true,
            line_number_style: "left".to_string(),
            tab_size: 4,
            break_lines: true,
        }
    }
}

/// Color scheme for syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    /// Background color for code blocks
    pub background: String,
    /// Comment color
    pub comment: String,
    /// Keyword color
    pub keyword: String,
    /// String literal color
    pub string: String,
    /// Number color
    pub number: String,
    /// Function name color
    pub function: String,
    /// Type name color
    pub type_name: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            background: "backcolour".to_string(),
            comment: "codegreen".to_string(),
            keyword: "magenta".to_string(),
            string: "codepurple".to_string(),
            number: "orange".to_string(),
            function: "blue".to_string(),
            type_name: "violet".to_string(),
        }
    }
}

/// Package configuration for LaTeX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    /// Standard packages to include
    pub standard_packages: Vec<String>,
    /// Additional packages with options
    pub additional_packages: HashMap<String, Vec<String>>,
    /// Custom package definitions
    pub custom_definitions: Vec<String>,
}

impl Default for PackageConfig {
    fn default() -> Self {
        let mut standard_packages = vec![
            "inputenc".to_string(),
            "fontenc".to_string(), 
            "lmodern".to_string(),
            "geometry".to_string(),
            "fancyhdr".to_string(),
            "listings".to_string(),
            "xcolor".to_string(),
            "hyperref".to_string(),
            "graphicx".to_string(),
            "amsmath".to_string(),
            "amsfonts".to_string(),
            "amssymb".to_string(),
            "booktabs".to_string(),
            "longtable".to_string(),
            "makeidx".to_string(),
        ];

        let mut additional_packages = HashMap::new();
        additional_packages.insert("inputenc".to_string(), vec!["utf8".to_string()]);
        additional_packages.insert("fontenc".to_string(), vec!["T1".to_string()]);
        additional_packages.insert("geometry".to_string(), vec!["margin=1in".to_string()]);

        Self {
            standard_packages,
            additional_packages,
            custom_definitions: Vec::new(),
        }
    }
}

/// LaTeX generator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaTeXConfig {
    /// Document class to use
    pub document_class: DocumentClass,
    /// Custom document class options
    pub document_options: Vec<String>,
    /// Package configuration
    pub packages: PackageConfig,
    /// Syntax highlighting configuration
    pub syntax_highlighting: SyntaxHighlighting,
    /// Paper size (a4paper, letterpaper, etc.)
    pub paper_size: String,
    /// Font size (10pt, 11pt, 12pt)
    pub font_size: String,
    /// Margin settings
    pub margins: String,
    /// Whether to generate table of contents
    pub generate_toc: bool,
    /// Whether to generate list of figures
    pub generate_lof: bool,
    /// Whether to generate list of tables
    pub generate_lot: bool,
    /// Whether to generate index
    pub generate_index: bool,
    /// Whether to generate bibliography
    pub generate_bibliography: bool,
    /// Bibliography style (plain, alpha, etc.)
    pub bibliography_style: String,
    /// Maximum TOC depth
    pub toc_depth: usize,
    /// Whether to use Unicode support
    pub unicode_support: bool,
    /// Custom header/footer
    pub custom_headers: bool,
    /// Include source code listings
    pub include_code_listings: bool,
    /// Cross-reference generation
    pub generate_cross_refs: bool,
    /// Mathematical notation support
    pub math_support: bool,
}

impl Default for LaTeXConfig {
    fn default() -> Self {
        Self {
            document_class: DocumentClass::Article,
            document_options: Vec::new(),
            packages: PackageConfig::default(),
            syntax_highlighting: SyntaxHighlighting::default(),
            paper_size: "a4paper".to_string(),
            font_size: "11pt".to_string(),
            margins: "margin=1in".to_string(),
            generate_toc: true,
            generate_lof: false,
            generate_lot: false,
            generate_index: true,
            generate_bibliography: true,
            bibliography_style: "plain".to_string(),
            toc_depth: 3,
            unicode_support: true,
            custom_headers: true,
            include_code_listings: true,
            generate_cross_refs: true,
            math_support: true,
        }
    }
}

/// Main LaTeX documentation generator
pub struct LaTeXGenerator {
    /// Generator configuration
    config: LaTeXConfig,
    /// Cross-reference map for generating \label and \ref commands
    cross_refs: HashMap<String, String>,
    /// Index entries for generating index
    index_entries: HashMap<String, Vec<String>>,
    /// Bibliography entries
    bibliography_entries: Vec<String>,
}

impl LaTeXGenerator {
    /// Create a new LaTeX generator with the given configuration
    #[instrument(skip(config))]
    pub fn new(config: LaTeXConfig) -> Self {
        info!("Initializing LaTeX generator with document class: {:?}", config.document_class);
        
        Self {
            config,
            cross_refs: HashMap::new(),
            index_entries: HashMap::new(),
            bibliography_entries: Vec::new(),
        }
    }

    /// Generate complete LaTeX documentation from extracted documentation
    #[instrument(skip(self, docs))]
    pub fn generate_documentation(
        &mut self,
        docs: &[crate::documentation::ExtractedDocumentation],
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, Error> {
        info!("Generating LaTeX documentation for {} modules", docs.len());
        
        std::fs::create_dir_all(output_dir)
            .map_err(|e| Error::FileWriteError(output_dir.to_path_buf(), e.to_string()))?;

        let mut output_files = Vec::new();

        // Generate main document
        let main_content = self.generate_main_document(docs)?;
        let main_file = output_dir.join("documentation.tex");
        std::fs::write(&main_file, main_content)
            .map_err(|e| Error::FileWriteError(main_file.clone(), e.to_string()))?;
        output_files.push(main_file);

        // Generate individual module files if using report or book class
        if matches!(self.config.document_class, DocumentClass::Report | DocumentClass::Book) {
            for doc in docs {
                let module_content = self.generate_module_document(doc)?;
                let module_name = doc.source_file.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .replace(' ', "_");
                let module_file = output_dir.join(format!("{}.tex", module_name));
                std::fs::write(&module_file, module_content)
                    .map_err(|e| Error::FileWriteError(module_file.clone(), e.to_string()))?;
                output_files.push(module_file);
            }
        }

        // Generate bibliography file if enabled
        if self.config.generate_bibliography && !self.bibliography_entries.is_empty() {
            let bib_content = self.generate_bibliography()?;
            let bib_file = output_dir.join("references.bib");
            std::fs::write(&bib_file, bib_content)
                .map_err(|e| Error::FileWriteError(bib_file.clone(), e.to_string()))?;
            output_files.push(bib_file);
        }

        // Generate Makefile for compilation
        let makefile_content = self.generate_makefile()?;
        let makefile = output_dir.join("Makefile");
        std::fs::write(&makefile, makefile_content)
            .map_err(|e| Error::FileWriteError(makefile.clone(), e.to_string()))?;
        output_files.push(makefile);

        // Generate compilation script
        let compile_script = self.generate_compile_script()?;
        let script_file = output_dir.join("compile.sh");
        std::fs::write(&script_file, compile_script)
            .map_err(|e| Error::FileWriteError(script_file.clone(), e.to_string()))?;
        
        // Make script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&script_file)
                .map_err(|e| Error::FileWriteError(script_file.clone(), e.to_string()))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_file, perms)
                .map_err(|e| Error::FileWriteError(script_file.clone(), e.to_string()))?;
        }
        
        output_files.push(script_file);

        info!("Generated {} LaTeX files", output_files.len());
        Ok(output_files)
    }

    /// Generate the main LaTeX document
    #[instrument(skip(self, docs))]
    fn generate_main_document(
        &mut self,
        docs: &[crate::documentation::ExtractedDocumentation],
    ) -> Result<String, Error> {
        let mut latex = String::new();

        // Document class and options
        let options = if self.config.document_options.is_empty() {
            self.config.document_class.default_options().join(",")
        } else {
            self.config.document_options.join(",")
        };
        
        latex.push_str(&format!(
            "\\documentclass[{}]{{{}}}\n\n",
            options,
            self.config.document_class.to_latex()
        ));

        // Preamble
        latex.push_str(&self.generate_preamble()?);

        // Document beginning
        latex.push_str("\\begin{document}\n\n");

        // Title page
        latex.push_str(&self.generate_title_page(docs)?);

        // Table of contents
        if self.config.generate_toc {
            latex.push_str(&self.generate_table_of_contents()?);
        }

        // Main content
        latex.push_str(&self.generate_main_content(docs)?);

        // Bibliography
        if self.config.generate_bibliography {
            latex.push_str(&self.generate_bibliography_section()?);
        }

        // Index
        if self.config.generate_index {
            latex.push_str(&self.generate_index_section()?);
        }

        // Document end
        latex.push_str("\\end{document}\n");

        Ok(latex)
    }

    /// Generate LaTeX preamble with packages and configurations
    #[instrument(skip(self))]
    fn generate_preamble(&self) -> Result<String, Error> {
        let mut preamble = String::new();

        // Unicode support
        if self.config.unicode_support {
            preamble.push_str("% Unicode and font encoding\n");
            preamble.push_str("\\usepackage[utf8]{inputenc}\n");
            preamble.push_str("\\usepackage[T1]{fontenc}\n");
            preamble.push_str("\\usepackage{lmodern}\n\n");
        }

        // Geometry and layout
        preamble.push_str("% Page layout and geometry\n");
        preamble.push_str(&format!("\\usepackage[{}]{{geometry}}\n", self.config.margins));
        
        if self.config.custom_headers {
            preamble.push_str("\\usepackage{fancyhdr}\n");
            preamble.push_str("\\pagestyle{fancy}\n");
        }
        preamble.push_str("\n");

        // Colors and graphics
        preamble.push_str("% Colors and graphics\n");
        preamble.push_str("\\usepackage{xcolor}\n");
        preamble.push_str("\\usepackage{graphicx}\n\n");

        // Code highlighting setup
        if self.config.include_code_listings {
            preamble.push_str(&self.generate_code_highlighting_setup()?);
        }

        // Mathematical notation
        if self.config.math_support {
            preamble.push_str("% Mathematical notation\n");
            preamble.push_str("\\usepackage{amsmath}\n");
            preamble.push_str("\\usepackage{amsfonts}\n");
            preamble.push_str("\\usepackage{amssymb}\n");
            preamble.push_str("\\usepackage{mathtools}\n\n");
        }

        // Tables
        preamble.push_str("% Tables\n");
        preamble.push_str("\\usepackage{booktabs}\n");
        preamble.push_str("\\usepackage{longtable}\n");
        preamble.push_str("\\usepackage{array}\n\n");

        // Cross-references and hyperlinks
        if self.config.generate_cross_refs {
            preamble.push_str("% Cross-references and hyperlinks\n");
            preamble.push_str("\\usepackage{hyperref}\n");
            preamble.push_str("\\hypersetup{\n");
            preamble.push_str("    colorlinks=true,\n");
            preamble.push_str("    linkcolor=blue,\n");
            preamble.push_str("    filecolor=magenta,\n");
            preamble.push_str("    urlcolor=cyan,\n");
            preamble.push_str("    citecolor=red\n");
            preamble.push_str("}\n\n");
        }

        // Index generation
        if self.config.generate_index {
            preamble.push_str("% Index generation\n");
            preamble.push_str("\\usepackage{makeidx}\n");
            preamble.push_str("\\makeindex\n\n");
        }

        // Additional packages
        for (package, options) in &self.config.packages.additional_packages {
            if !options.is_empty() {
                preamble.push_str(&format!("\\usepackage[{}]{{{}}}\n", options.join(","), package));
            } else {
                preamble.push_str(&format!("\\usepackage{{{}}}\n", package));
            }
        }

        // Custom definitions
        if !self.config.packages.custom_definitions.is_empty() {
            preamble.push_str("\n% Custom definitions\n");
            for definition in &self.config.packages.custom_definitions {
                preamble.push_str(&format!("{}\n", definition));
            }
        }

        preamble.push_str("\n");
        Ok(preamble)
    }

    /// Generate code highlighting setup
    fn generate_code_highlighting_setup(&self) -> Result<String, Error> {
        let mut setup = String::new();

        if self.config.syntax_highlighting.use_minted {
            // Minted setup (requires Python pygments)
            setup.push_str("% Code highlighting with minted\n");
            setup.push_str("\\usepackage{minted}\n");
            setup.push_str("\\usemintedstyle{default}\n");
            setup.push_str("\\setminted{\n");
            setup.push_str(&format!("    tabsize={},\n", self.config.syntax_highlighting.tab_size));
            setup.push_str(&format!("    linenos={},\n", self.config.syntax_highlighting.show_line_numbers));
            setup.push_str(&format!("    breaklines={},\n", self.config.syntax_highlighting.break_lines));
            setup.push_str("    bgcolor=codebg\n");
            setup.push_str("}\n\n");
        } else {
            // Listings setup (more compatible)
            setup.push_str("% Code highlighting with listings\n");
            setup.push_str("\\usepackage{listings}\n\n");
            
            // Define colors
            let colors = &self.config.syntax_highlighting.color_scheme;
            setup.push_str("% Define colors for code highlighting\n");
            setup.push_str("\\definecolor{codegreen}{rgb}{0,0.6,0}\n");
            setup.push_str("\\definecolor{codegray}{rgb}{0.5,0.5,0.5}\n");
            setup.push_str("\\definecolor{codepurple}{rgb}{0.58,0,0.82}\n");
            setup.push_str("\\definecolor{backcolour}{rgb}{0.95,0.95,0.92}\n");
            setup.push_str("\\definecolor{codeblue}{rgb}{0,0,0.8}\n");
            setup.push_str("\\definecolor{codeviolet}{rgb}{0.5,0,0.5}\n");
            setup.push_str("\\definecolor{codeorange}{rgb}{1,0.5,0}\n\n");

            // Define CURSED language
            setup.push_str("% Define CURSED language for listings\n");
            setup.push_str("\\lstdefinelanguage{CURSED}{\n");
            setup.push_str("    keywords={slay,yolo,sus,facts,lowkey,highkey,periodt,bestie,flex,stan,\n");
            setup.push_str("              vibe_check,mood,basic,squad,collab,cap,no_cap,bet,fr,\n");
            setup.push_str("              async,await,if,else,for,while,fn,struct,interface,enum,\n");
            setup.push_str("              import,export,let,const,return,break,continue,match,\n");
            setup.push_str("              true,false,nil,self,type,impl,trait,where,pub,priv},\n");
            setup.push_str("    sensitive=true,\n");
            setup.push_str("    comment=[l]{//},\n");
            setup.push_str("    morecomment=[s]{/*}{*/},\n");
            setup.push_str("    string=[b]\",\n");
            setup.push_str("    morestring=[b]',\n");
            setup.push_str("    numbers=left,\n");
            setup.push_str("    numbersep=5pt,\n");
            setup.push_str("    showstringspaces=false\n");
            setup.push_str("}\n\n");

            // Define style
            setup.push_str("% Define code style\n");
            setup.push_str("\\lstdefinestyle{cursedstyle}{\n");
            setup.push_str(&format!("    backgroundcolor=\\color{{{}}},\n", colors.background));
            setup.push_str(&format!("    commentstyle=\\color{{{}}},\n", colors.comment));
            setup.push_str(&format!("    keywordstyle=\\color{{{}}},\n", colors.keyword));
            setup.push_str(&format!("    stringstyle=\\color{{{}}},\n", colors.string));
            setup.push_str("    basicstyle=\\ttfamily\\footnotesize,\n");
            setup.push_str("    numberstyle=\\tiny\\color{codegray},\n");
            setup.push_str(&format!("    breaklines={},\n", self.config.syntax_highlighting.break_lines));
            setup.push_str("    breakatwhitespace=false,\n");
            setup.push_str("    captionpos=b,\n");
            setup.push_str("    keepspaces=true,\n");
            setup.push_str(&format!("    numbers={},\n", if self.config.syntax_highlighting.show_line_numbers { "left" } else { "none" }));
            setup.push_str(&format!("    numbersep=5pt,\n"));
            setup.push_str("    showspaces=false,\n");
            setup.push_str("    showstringspaces=false,\n");
            setup.push_str("    showtabs=false,\n");
            setup.push_str(&format!("    tabsize={}\n", self.config.syntax_highlighting.tab_size));
            setup.push_str("}\n\n");

            setup.push_str("\\lstset{style=cursedstyle}\n\n");
        }

        Ok(setup)
    }

    /// Generate title page
    fn generate_title_page(&self, docs: &[crate::documentation::ExtractedDocumentation]) -> Result<String, Error> {
        let mut title = String::new();

        match self.config.document_class {
            DocumentClass::Beamer => {
                // Beamer title frame
                title.push_str("\\title{CURSED Programming Language Documentation}\n");
                title.push_str("\\author{CURSED Development Team}\n");
                title.push_str("\\date{\\today}\n\n");
                title.push_str("\\frame{\\titlepage}\n\n");
            }
            _ => {
                // Standard title page
                title.push_str("\\title{\\textbf{CURSED Programming Language}\\\\Documentation}\n");
                title.push_str("\\author{CURSED Development Team}\n");
                title.push_str("\\date{\\today}\n\n");
                title.push_str("\\maketitle\n\n");
                
                // Abstract section for non-book classes
                if !matches!(self.config.document_class, DocumentClass::Book) {
                    title.push_str("\\begin{abstract}\n");
                    title.push_str("This document provides comprehensive documentation for the CURSED programming language, ");
                    title.push_str(&format!("covering {} modules with detailed API reference, examples, and usage guidelines. ", docs.len()));
                    title.push_str("CURSED is a modern programming language that embraces Gen Z slang while providing ");
                    title.push_str("powerful features for systems programming, web development, and more.\n");
                    title.push_str("\\end{abstract}\n\n");
                }
                
                title.push_str("\\newpage\n\n");
            }
        }

        Ok(title)
    }

    /// Generate table of contents
    fn generate_table_of_contents(&self) -> Result<String, Error> {
        let mut toc = String::new();

        match self.config.document_class {
            DocumentClass::Beamer => {
                // Beamer outline frame
                toc.push_str("\\begin{frame}\n");
                toc.push_str("\\frametitle{Outline}\n");
                toc.push_str("\\tableofcontents\n");
                toc.push_str("\\end{frame}\n\n");
            }
            _ => {
                // Standard table of contents
                toc.push_str(&format!("\\setcounter{{tocdepth}}{{{}}}\n", self.config.toc_depth));
                toc.push_str("\\tableofcontents\n");
                
                if self.config.generate_lof {
                    toc.push_str("\\listoffigures\n");
                }
                
                if self.config.generate_lot {
                    toc.push_str("\\listoftables\n");
                }
                
                toc.push_str("\\newpage\n\n");
            }
        }

        Ok(toc)
    }

    /// Generate main content sections
    #[instrument(skip(self, docs))]
    fn generate_main_content(
        &mut self,
        docs: &[crate::documentation::ExtractedDocumentation],
    ) -> Result<String, Error> {
        let mut content = String::new();

        // Introduction section
        content.push_str(&self.generate_introduction_section(docs)?);

        // Quick reference section
        content.push_str(&self.generate_quick_reference_section(docs)?);

        // Module documentation
        for (i, doc) in docs.iter().enumerate() {
            content.push_str(&self.generate_module_section(doc, i)?);
        }

        // API reference appendix
        content.push_str(&self.generate_api_reference_appendix(docs)?);

        Ok(content)
    }

    /// Generate introduction section
    fn generate_introduction_section(&self, docs: &[crate::documentation::ExtractedDocumentation]) -> Result<String, Error> {
        let mut intro = String::new();

        let section_cmd = match self.config.document_class {
            DocumentClass::Book => "\\chapter",
            DocumentClass::Beamer => "\\section",
            _ => "\\section",
        };

        intro.push_str(&format!("{}{{Introduction}}\n", section_cmd));
        if self.config.generate_index {
            intro.push_str("\\index{Introduction}\n");
        }
        intro.push_str("\\label{sec:introduction}\n\n");

        intro.push_str("The CURSED programming language is a modern, expressive language that combines ");
        intro.push_str("the familiarity of Gen Z slang with powerful programming constructs. This documentation ");
        intro.push_str("provides comprehensive coverage of the language features, standard library, and best practices.\n\n");

        // Language overview
        intro.push_str("\\subsection{Language Overview}\n");
        intro.push_str("\\label{subsec:overview}\n\n");
        
        intro.push_str("CURSED features:\n");
        intro.push_str("\\begin{itemize}\n");
        intro.push_str("\\item \\textbf{Expressive Syntax}: Uses Gen Z slang for keywords (\\texttt{slay}, \\texttt{yolo}, \\texttt{sus})\n");
        intro.push_str("\\item \\textbf{Type Safety}: Strong static typing with type inference\n");
        intro.push_str("\\item \\textbf{Memory Safety}: Automatic memory management with garbage collection\n");
        intro.push_str("\\item \\textbf{Concurrency}: Built-in support for goroutines and channels\n");
        intro.push_str("\\item \\textbf{Interoperability}: Seamless integration with existing systems\n");
        intro.push_str("\\end{itemize}\n\n");

        // Documentation statistics
        intro.push_str("\\subsection{Documentation Statistics}\n");
        intro.push_str("\\label{subsec:stats}\n\n");
        
        let total_functions = docs.iter().map(|d| d.functions.len()).sum::<usize>();
        let total_types = docs.iter().map(|d| d.types.len()).sum::<usize>();
        let total_constants = docs.iter().map(|d| d.constants.len()).sum::<usize>();
        
        intro.push_str("\\begin{table}[h]\n");
        intro.push_str("\\centering\n");
        intro.push_str("\\begin{tabular}{lr}\n");
        intro.push_str("\\toprule\n");
        intro.push_str("\\textbf{Component} & \\textbf{Count} \\\\\n");
        intro.push_str("\\midrule\n");
        intro.push_str(&format!("Modules & {} \\\\\n", docs.len()));
        intro.push_str(&format!("Functions & {} \\\\\n", total_functions));
        intro.push_str(&format!("Types & {} \\\\\n", total_types));
        intro.push_str(&format!("Constants & {} \\\\\n", total_constants));
        intro.push_str("\\bottomrule\n");
        intro.push_str("\\end{tabular}\n");
        intro.push_str("\\caption{Documentation Statistics}\n");
        intro.push_str("\\label{tab:stats}\n");
        intro.push_str("\\end{table}\n\n");

        Ok(intro)
    }

    /// Generate quick reference section
    fn generate_quick_reference_section(&self, docs: &[crate::documentation::ExtractedDocumentation]) -> Result<String, Error> {
        let mut reference = String::new();

        let section_cmd = match self.config.document_class {
            DocumentClass::Book => "\\chapter",
            DocumentClass::Beamer => "\\section",
            _ => "\\section",
        };

        reference.push_str(&format!("{}{{Quick Reference}}\n", section_cmd));
        if self.config.generate_index {
            reference.push_str("\\index{Quick Reference}\n");
        }
        reference.push_str("\\label{sec:quick-reference}\n\n");

        // Keyword reference
        reference.push_str("\\subsection{Keywords}\n");
        reference.push_str("\\label{subsec:keywords}\n\n");
        
        reference.push_str("\\begin{longtable}{p{0.2\\textwidth}p{0.3\\textwidth}p{0.4\\textwidth}}\n");
        reference.push_str("\\toprule\n");
        reference.push_str("\\textbf{Keyword} & \\textbf{Standard Equivalent} & \\textbf{Description} \\\\\n");
        reference.push_str("\\midrule\n");
        reference.push_str("\\endhead\n");
        
        let keywords = vec![
            ("slay", "fn", "Function declaration"),
            ("yolo", "yield", "Yield control in loops/goroutines"),
            ("sus", "let", "Variable declaration"),
            ("facts", "const", "Constant declaration"),
            ("lowkey", "if", "Conditional statement"),
            ("highkey", "else", "Alternative branch"),
            ("periodt", "while", "While loop"),
            ("bestie", "for", "For loop"),
            ("flex", "break", "Break from loop"),
            ("stan", "spawn", "Spawn goroutine"),
            ("vibe_check", "switch", "Switch statement"),
            ("mood", "case", "Switch case"),
            ("basic", "default", "Default case"),
            ("squad", "struct", "Structure definition"),
            ("collab", "interface", "Interface definition"),
            ("cap", "true", "Boolean true"),
            ("no_cap", "false", "Boolean false"),
            ("bet", "return", "Return statement"),
            ("fr", "assert", "Assertion"),
        ];

        for (cursed, standard, description) in keywords {
            reference.push_str(&format!(
                "\\texttt{{{}}} & \\texttt{{{}}} & {} \\\\\n",
                self.escape_latex(cursed),
                self.escape_latex(standard),
                self.escape_latex(description)
            ));
        }
        
        reference.push_str("\\bottomrule\n");
        reference.push_str("\\end{longtable}\n\n");

        // Common patterns
        reference.push_str("\\subsection{Common Patterns}\n");
        reference.push_str("\\label{subsec:patterns}\n\n");

        if self.config.include_code_listings {
            reference.push_str("\\subsubsection{Function Definition}\n");
            reference.push_str("\\begin{lstlisting}[language=CURSED,caption={Function Definition Pattern}]\n");
            reference.push_str("/// Calculate the sum of two numbers\n");
            reference.push_str("slay add(a: i32, b: i32) -> i32 {\n");
            reference.push_str("    bet a + b\n");
            reference.push_str("}\n");
            reference.push_str("\\end{lstlisting}\n\n");

            reference.push_str("\\subsubsection{Struct Definition}\n");
            reference.push_str("\\begin{lstlisting}[language=CURSED,caption={Struct Definition Pattern}]\n");
            reference.push_str("/// A person with name and age\n");
            reference.push_str("squad Person {\n");
            reference.push_str("    name: String,\n");
            reference.push_str("    age: u32,\n");
            reference.push_str("}\n");
            reference.push_str("\\end{lstlisting}\n\n");
        }

        Ok(reference)
    }

    /// Generate module section
    #[instrument(skip(self, doc))]
    fn generate_module_section(
        &mut self,
        doc: &crate::documentation::ExtractedDocumentation,
        index: usize,
    ) -> Result<String, Error> {
        let mut module = String::new();

        let module_name = doc.source_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        let section_cmd = match self.config.document_class {
            DocumentClass::Book => "\\chapter",
            DocumentClass::Beamer => "\\section",
            _ => "\\section",
        };

        module.push_str(&format!("{}{{Module: {}}}\n", section_cmd, self.escape_latex(&module_name)));
        if self.config.generate_index {
            module.push_str(&format!("\\index{{{}!Module}}\n", self.escape_latex(&module_name)));
        }
        module.push_str(&format!("\\label{{sec:module-{}}}\n\n", index));

        // Module description
        if let Some(ref module_doc) = doc.module_doc {
            if let Some(ref description) = module_doc.description {
                module.push_str(&format!("{}\n\n", self.escape_latex(description)));
            }
        }

        // Module information table
        module.push_str("\\begin{table}[h]\n");
        module.push_str("\\centering\n");
        module.push_str("\\begin{tabular}{lr}\n");
        module.push_str("\\toprule\n");
        module.push_str("\\textbf{Component} & \\textbf{Count} \\\\\n");
        module.push_str("\\midrule\n");
        module.push_str(&format!("Functions & {} \\\\\n", doc.functions.len()));
        module.push_str(&format!("Types & {} \\\\\n", doc.types.len()));
        module.push_str(&format!("Constants & {} \\\\\n", doc.constants.len()));
        module.push_str(&format!("Variables & {} \\\\\n", doc.variables.len()));
        module.push_str("\\bottomrule\n");
        module.push_str("\\end{tabular}\n");
        module.push_str(&format!("\\caption{{Module {} Statistics}}\n", self.escape_latex(&module_name)));
        module.push_str(&format!("\\label{{tab:module-{}-stats}}\n", index));
        module.push_str("\\end{table}\n\n");

        // Functions section
        if !doc.functions.is_empty() {
            module.push_str("\\subsection{Functions}\n");
            module.push_str(&format!("\\label{{subsec:module-{}-functions}}\n\n", index));
            
            for (i, func) in doc.functions.iter().enumerate() {
                module.push_str(&self.generate_function_documentation(func, &module_name, i)?);
            }
        }

        // Types section  
        if !doc.types.is_empty() {
            module.push_str("\\subsection{Types}\n");
            module.push_str(&format!("\\label{{subsec:module-{}-types}}\n\n", index));
            
            for (i, type_doc) in doc.types.iter().enumerate() {
                module.push_str(&self.generate_type_documentation(type_doc, &module_name, i)?);
            }
        }

        // Constants section
        if !doc.constants.is_empty() {
            module.push_str("\\subsection{Constants}\n");
            module.push_str(&format!("\\label{{subsec:module-{}-constants}}\n\n", index));
            
            for (i, constant) in doc.constants.iter().enumerate() {
                module.push_str(&self.generate_constant_documentation(constant, &module_name, i)?);
            }
        }

        Ok(module)
    }

    /// Generate function documentation
    fn generate_function_documentation(
        &mut self,
        func: &crate::documentation::FunctionDoc,
        module_name: &str,
        index: usize,
    ) -> Result<String, Error> {
        let mut doc = String::new();

        // Function header
        doc.push_str(&format!("\\subsubsection{{{}}}\n", self.escape_latex(&func.name)));
        if self.config.generate_index {
            doc.push_str(&format!("\\index{{{}!{}}}\n", self.escape_latex(module_name), self.escape_latex(&func.name)));
            doc.push_str(&format!("\\index{{Functions!{}}}\n", self.escape_latex(&func.name)));
        }
        doc.push_str(&format!("\\label{{func:{}:{}}}\n\n", module_name.replace(' ', "_"), func.name.replace(' ', "_")));

        // Function signature in math mode for better formatting
        if self.config.math_support {
            doc.push_str("\\textbf{Signature:}\n");
            doc.push_str("\\begin{equation*}\n");
            doc.push_str(&format!("\\text{{{}}}(", self.escape_latex(&func.name)));
            
            let params: Vec<String> = func.parameters.iter()
                .map(|p| format!("\\text{{{}}} : \\text{{{}}}", 
                    self.escape_latex(&p.name),
                    self.escape_latex(&p.param_type)))
                .collect();
            doc.push_str(&params.join(", "));
            
            if let Some(ref return_type) = func.return_type {
                doc.push_str(&format!(") \\rightarrow \\text{{{}}}", self.escape_latex(&return_type.name)));
            } else {
                doc.push_str(")");
            }
            
            doc.push_str("\\end{equation*}\n\n");
        }

        // Description
        if let Some(ref description) = func.description {
            doc.push_str(&format!("{}\n\n", self.escape_latex(description)));
        }

        // Parameters table
        if !func.parameters.is_empty() {
            doc.push_str("\\textbf{Parameters:}\n\n");
            doc.push_str("\\begin{longtable}{p{0.2\\textwidth}p{0.25\\textwidth}p{0.45\\textwidth}}\n");
            doc.push_str("\\toprule\n");
            doc.push_str("\\textbf{Name} & \\textbf{Type} & \\textbf{Description} \\\\\n");
            doc.push_str("\\midrule\n");
            doc.push_str("\\endhead\n");
            
            for param in &func.parameters {
                let param_desc = param.description.as_ref()
                    .map(|d| self.escape_latex(d))
                    .unwrap_or_else(|| "No description".to_string());
                    
                doc.push_str(&format!(
                    "\\texttt{{{}}} & \\texttt{{{}}} & {} \\\\\n",
                    self.escape_latex(&param.name),
                    self.escape_latex(&param.param_type),
                    param_desc
                ));
            }
            
            doc.push_str("\\bottomrule\n");
            doc.push_str("\\end{longtable}\n\n");
        }

        // Return type
        if let Some(ref return_type) = func.return_type {
            doc.push_str(&format!("\\textbf{{Return Type:}} \\texttt{{{}}}\n\n", self.escape_latex(&return_type.name)));
        }

        // Examples
        if !func.examples.is_empty() && self.config.include_code_listings {
            doc.push_str("\\textbf{Examples:}\n\n");
            
            for (i, example) in func.examples.iter().enumerate() {
                if let Some(ref title) = example.title {
                    doc.push_str(&format!("\\paragraph{{{}}}\n\n", self.escape_latex(title)));
                }
                
                doc.push_str(&format!(
                    "\\begin{{lstlisting}}[language=CURSED,caption={{Example {} for {}}}]\n",
                    i + 1,
                    self.escape_latex(&func.name)
                ));
                doc.push_str(&example.code);
                doc.push_str("\\end{lstlisting}\n\n");
                
                if let Some(ref desc) = example.description {
                    doc.push_str(&format!("{}\n\n", self.escape_latex(desc)));
                }
            }
        }

        // Source code
        if let Some(ref source) = func.source_code {
            if self.config.include_code_listings {
                doc.push_str("\\textbf{Source Code:}\n\n");
                doc.push_str(&format!(
                    "\\begin{{lstlisting}}[language=CURSED,caption={{Source code for {}}}]\n",
                    self.escape_latex(&func.name)
                ));
                doc.push_str(source);
                doc.push_str("\\end{lstlisting}\n\n");
            }
        }

        Ok(doc)
    }

    /// Generate type documentation
    fn generate_type_documentation(
        &mut self,
        type_doc: &crate::documentation::TypeDoc,
        module_name: &str,
        index: usize,
    ) -> Result<String, Error> {
        let mut doc = String::new();

        // Type header
        doc.push_str(&format!("\\subsubsection{{{} ({})}}\n", 
            self.escape_latex(&type_doc.name),
            self.escape_latex(&type_doc.type_def)
        ));
        if self.config.generate_index {
            doc.push_str(&format!("\\index{{{}!{}}}\n", self.escape_latex(module_name), self.escape_latex(&type_doc.name)));
            doc.push_str(&format!("\\index{{Types!{}}}\n", self.escape_latex(&type_doc.name)));
        }
        doc.push_str(&format!("\\label{{type:{}:{}}}\n\n", module_name.replace(' ', "_"), type_doc.name.replace(' ', "_")));

        // Description
        if let Some(ref description) = type_doc.description {
            doc.push_str(&format!("{}\n\n", self.escape_latex(description)));
        }

        // Fields table
        if !type_doc.fields.is_empty() {
            doc.push_str("\\textbf{Fields:}\n\n");
            doc.push_str("\\begin{longtable}{p{0.2\\textwidth}p{0.25\\textwidth}p{0.1\\textwidth}p{0.35\\textwidth}}\n");
            doc.push_str("\\toprule\n");
            doc.push_str("\\textbf{Name} & \\textbf{Type} & \\textbf{Visibility} & \\textbf{Description} \\\\\n");
            doc.push_str("\\midrule\n");
            doc.push_str("\\endhead\n");
            
            for field in &type_doc.fields {
                let field_desc = field.description.as_ref()
                    .map(|d| self.escape_latex(d))
                    .unwrap_or_else(|| "No description".to_string());
                    
                doc.push_str(&format!(
                    "\\texttt{{{}}} & \\texttt{{{}}} & {} & {} \\\\\n",
                    self.escape_latex(&field.name),
                    self.escape_latex(&field.field_type),
                    self.escape_latex(&field.visibility),
                    field_desc
                ));
            }
            
            doc.push_str("\\bottomrule\n");
            doc.push_str("\\end{longtable}\n\n");
        }

        // Methods
        if !type_doc.methods.is_empty() {
            doc.push_str("\\textbf{Methods:}\n\n");
            
            for (i, method) in type_doc.methods.iter().enumerate() {
                doc.push_str(&format!("\\paragraph{{{}}}\n", self.escape_latex(&method.name)));
                if self.config.generate_index {
                    doc.push_str(&format!("\\index{{{}!{}!{}}}\n", 
                        self.escape_latex(module_name), 
                        self.escape_latex(&type_doc.name),
                        self.escape_latex(&method.name)
                    ));
                }
                doc.push_str(&format!("\\label{{method:{}:{}:{}}}\n\n", 
                    module_name.replace(' ', "_"), 
                    type_doc.name.replace(' ', "_"),
                    method.name.replace(' ', "_")
                ));
                
                if let Some(ref description) = method.description {
                    doc.push_str(&format!("{}\n\n", self.escape_latex(description)));
                }
                
                // Method signature
                if self.config.math_support {
                    doc.push_str("\\begin{equation*}\n");
                    doc.push_str(&format!("\\text{{{}}}(", self.escape_latex(&method.name)));
                    
                    let params: Vec<String> = method.parameters.iter()
                        .map(|p| format!("\\text{{{}}} : \\text{{{}}}", 
                            self.escape_latex(&p.name),
                            self.escape_latex(&p.param_type)))
                        .collect();
                    doc.push_str(&params.join(", "));
                    
                    if let Some(ref return_type) = method.return_type {
                        doc.push_str(&format!(") \\rightarrow \\text{{{}}}", self.escape_latex(&return_type.name)));
                    } else {
                        doc.push_str(")");
                    }
                    
                    doc.push_str("\\end{equation*}\n\n");
                }
            }
        }

        Ok(doc)
    }

    /// Generate constant documentation  
    fn generate_constant_documentation(
        &mut self,
        constant: &crate::documentation::DocumentationItem,
        module_name: &str,
        index: usize,
    ) -> Result<String, Error> {
        let mut doc = String::new();

        // Constant header
        doc.push_str(&format!("\\paragraph{{{}}}\n", self.escape_latex(&constant.name)));
        if self.config.generate_index {
            doc.push_str(&format!("\\index{{{}!{}}}\n", self.escape_latex(module_name), self.escape_latex(&constant.name)));
            doc.push_str(&format!("\\index{{Constants!{}}}\n", self.escape_latex(&constant.name)));
        }
        doc.push_str(&format!("\\label{{const:{}:{}}}\n\n", module_name.replace(' ', "_"), constant.name.replace(' ', "_")));

        // Description
        if let Some(ref description) = constant.description {
            doc.push_str(&format!("{}\n\n", self.escape_latex(description)));
        }

        // Type information
        if let Some(const_type) = constant.metadata.get("type") {
            doc.push_str(&format!("\\textbf{{Type:}} \\texttt{{{}}}\n\n", self.escape_latex(const_type)));
        }

        // Source code if available
        if let Some(ref source) = constant.source_code {
            if self.config.include_code_listings {
                doc.push_str("\\begin{lstlisting}[language=CURSED]\n");
                doc.push_str(source);
                doc.push_str("\\end{lstlisting}\n\n");
            }
        }

        Ok(doc)
    }

    /// Generate API reference appendix
    fn generate_api_reference_appendix(&self, docs: &[crate::documentation::ExtractedDocumentation]) -> Result<String, Error> {
        let mut appendix = String::new();

        let section_cmd = match self.config.document_class {
            DocumentClass::Book => "\\appendix\n\\chapter",
            DocumentClass::Beamer => "\\section",
            _ => "\\appendix\n\\section",
        };

        appendix.push_str(&format!("{}{{API Reference}}\n", section_cmd));
        if self.config.generate_index {
            appendix.push_str("\\index{API Reference}\n");
        }
        appendix.push_str("\\label{app:api-reference}\n\n");

        // Function index
        appendix.push_str("\\subsection{Function Index}\n");
        appendix.push_str("\\label{app:function-index}\n\n");
        
        appendix.push_str("\\begin{longtable}{p{0.3\\textwidth}p{0.3\\textwidth}p{0.3\\textwidth}}\n");
        appendix.push_str("\\toprule\n");
        appendix.push_str("\\textbf{Function} & \\textbf{Module} & \\textbf{Return Type} \\\\\n");
        appendix.push_str("\\midrule\n");
        appendix.push_str("\\endhead\n");
        
        for doc in docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
                
            for func in &doc.functions {
                let return_type = func.return_type.as_ref()
                    .map(|rt| rt.name.as_str())
                    .unwrap_or("void");
                    
                appendix.push_str(&format!(
                    "\\texttt{{{}}} & {} & \\texttt{{{}}} \\\\\n",
                    self.escape_latex(&func.name),
                    self.escape_latex(&module_name),
                    self.escape_latex(return_type)
                ));
            }
        }
        
        appendix.push_str("\\bottomrule\n");
        appendix.push_str("\\end{longtable}\n\n");

        // Type index
        appendix.push_str("\\subsection{Type Index}\n");
        appendix.push_str("\\label{app:type-index}\n\n");
        
        appendix.push_str("\\begin{longtable}{p{0.3\\textwidth}p{0.3\\textwidth}p{0.3\\textwidth}}\n");
        appendix.push_str("\\toprule\n");
        appendix.push_str("\\textbf{Type} & \\textbf{Module} & \\textbf{Kind} \\\\\n");
        appendix.push_str("\\midrule\n");
        appendix.push_str("\\endhead\n");
        
        for doc in docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
                
            for type_doc in &doc.types {
                appendix.push_str(&format!(
                    "\\texttt{{{}}} & {} & {} \\\\\n",
                    self.escape_latex(&type_doc.name),
                    self.escape_latex(&module_name),
                    self.escape_latex(&type_doc.type_def)
                ));
            }
        }
        
        appendix.push_str("\\bottomrule\n");
        appendix.push_str("\\end{longtable}\n\n");

        Ok(appendix)
    }

    /// Generate bibliography section
    fn generate_bibliography_section(&self) -> Result<String, Error> {
        let mut bib = String::new();

        match self.config.document_class {
            DocumentClass::Beamer => {
                // Beamer bibliography frame
                bib.push_str("\\begin{frame}\n");
                bib.push_str("\\frametitle{References}\n");
                bib.push_str(&format!("\\bibliographystyle{{{}}}\n", self.config.bibliography_style));
                bib.push_str("\\bibliography{references}\n");
                bib.push_str("\\end{frame}\n\n");
            }
            _ => {
                // Standard bibliography
                bib.push_str(&format!("\\bibliographystyle{{{}}}\n", self.config.bibliography_style));
                bib.push_str("\\bibliography{references}\n\n");
            }
        }

        Ok(bib)
    }

    /// Generate index section
    fn generate_index_section(&self) -> Result<String, Error> {
        let mut index = String::new();

        if !matches!(self.config.document_class, DocumentClass::Beamer) {
            index.push_str("\\printindex\n\n");
        }

        Ok(index)
    }

    /// Generate module document (for separate compilation)
    fn generate_module_document(&mut self, doc: &crate::documentation::ExtractedDocumentation) -> Result<String, Error> {
        let module_name = doc.source_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        let mut content = format!(
            "% Module: {}\n% This file is included in the main documentation\n\n",
            module_name
        );

        content.push_str(&self.generate_module_section(doc, 0)?);

        Ok(content)
    }

    /// Generate bibliography file
    fn generate_bibliography(&self) -> Result<String, Error> {
        let mut bib = String::new();

        // Add standard CURSED language reference
        bib.push_str("@misc{cursed_lang,\n");
        bib.push_str("    title={CURSED Programming Language},\n");
        bib.push_str("    author={CURSED Development Team},\n");
        bib.push_str("    year={2024},\n");
        bib.push_str("    url={https://github.com/cursed-lang/cursed},\n");
        bib.push_str("    note={Modern programming language with Gen Z syntax}\n");
        bib.push_str("}\n\n");

        // Add documentation generation reference
        bib.push_str("@misc{cursed_docs,\n");
        bib.push_str("    title={CURSED Programming Language Documentation},\n");
        bib.push_str("    author={CURSED Development Team},\n");
        bib.push_str("    year={2024},\n");
        bib.push_str("    note={Generated documentation},\n");
        bib.push_str("    howpublished={\\texttt{cursed-doc} tool}\n");
        bib.push_str("}\n\n");

        // Add any custom bibliography entries
        for entry in &self.bibliography_entries {
            bib.push_str(entry);
            bib.push_str("\n\n");
        }

        Ok(bib)
    }

    /// Generate Makefile for LaTeX compilation
    fn generate_makefile(&self) -> Result<String, Error> {
        let mut makefile = String::new();

        makefile.push_str("# LaTeX Documentation Makefile for CURSED\n");
        makefile.push_str("# Generated by cursed-doc LaTeX generator\n\n");

        makefile.push_str("MAIN = documentation\n");
        makefile.push_str("LATEX = pdflatex\n");
        makefile.push_str("BIBTEX = bibtex\n");
        makefile.push_str("MAKEINDEX = makeindex\n");
        
        if self.config.syntax_highlighting.use_minted {
            makefile.push_str("LATEXFLAGS = -shell-escape\n");
        } else {
            makefile.push_str("LATEXFLAGS = \n");
        }
        
        makefile.push_str("\n.PHONY: all clean cleanall view help\n\n");

        makefile.push_str("all: $(MAIN).pdf\n\n");

        // Main PDF target
        makefile.push_str("$(MAIN).pdf: $(MAIN).tex");
        if self.config.generate_bibliography {
            makefile.push_str(" references.bib");
        }
        makefile.push_str("\n");
        makefile.push_str("\t$(LATEX) $(LATEXFLAGS) $(MAIN).tex\n");
        
        if self.config.generate_index {
            makefile.push_str("\t$(MAKEINDEX) $(MAIN).idx\n");
        }
        
        if self.config.generate_bibliography {
            makefile.push_str("\t$(BIBTEX) $(MAIN)\n");
        }
        
        makefile.push_str("\t$(LATEX) $(LATEXFLAGS) $(MAIN).tex\n");
        makefile.push_str("\t$(LATEX) $(LATEXFLAGS) $(MAIN).tex\n\n");

        // Clean targets
        makefile.push_str("clean:\n");
        makefile.push_str("\trm -f *.aux *.bbl *.blg *.log *.out *.toc *.lof *.lot\n");
        makefile.push_str("\trm -f *.idx *.ilg *.ind *.fls *.fdb_latexmk\n");
        if self.config.syntax_highlighting.use_minted {
            makefile.push_str("\trm -rf _minted-*\n");
        }
        makefile.push_str("\n");

        makefile.push_str("cleanall: clean\n");
        makefile.push_str("\trm -f $(MAIN).pdf\n\n");

        makefile.push_str("view: $(MAIN).pdf\n");
        makefile.push_str("\topen $(MAIN).pdf || xdg-open $(MAIN).pdf\n\n");

        makefile.push_str("help:\n");
        makefile.push_str("\t@echo \"Available targets:\"\n");
        makefile.push_str("\t@echo \"  all      - Build PDF documentation\"\n");
        makefile.push_str("\t@echo \"  clean    - Remove auxiliary files\"\n");
        makefile.push_str("\t@echo \"  cleanall - Remove all generated files\"\n");
        makefile.push_str("\t@echo \"  view     - Open PDF documentation\"\n");
        makefile.push_str("\t@echo \"  help     - Show this help\"\n");

        Ok(makefile)
    }

    /// Generate compilation script
    fn generate_compile_script(&self) -> Result<String, Error> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# LaTeX Compilation Script for CURSED Documentation\n");
        script.push_str("# Generated by cursed-doc LaTeX generator\n\n");

        script.push_str("set -e\n\n");

        script.push_str("echo \"Compiling CURSED LaTeX documentation...\"\n\n");

        // Check for required tools
        script.push_str("# Check for required tools\n");
        script.push_str("command -v pdflatex >/dev/null 2>&1 || { echo >&2 \"pdflatex is required but not installed. Aborting.\"; exit 1; }\n");
        
        if self.config.generate_bibliography {
            script.push_str("command -v bibtex >/dev/null 2>&1 || { echo >&2 \"bibtex is required but not installed. Aborting.\"; exit 1; }\n");
        }
        
        if self.config.generate_index {
            script.push_str("command -v makeindex >/dev/null 2>&1 || { echo >&2 \"makeindex is required but not installed. Aborting.\"; exit 1; }\n");
        }
        
        if self.config.syntax_highlighting.use_minted {
            script.push_str("command -v pygmentize >/dev/null 2>&1 || { echo >&2 \"pygmentize (Python pygments) is required for minted but not installed. Aborting.\"; exit 1; }\n");
        }
        
        script.push_str("\n");

        // Compilation steps
        script.push_str("echo \"Running first LaTeX pass...\"\n");
        if self.config.syntax_highlighting.use_minted {
            script.push_str("pdflatex -shell-escape documentation.tex\n\n");
        } else {
            script.push_str("pdflatex documentation.tex\n\n");
        }

        if self.config.generate_index {
            script.push_str("if [ -f documentation.idx ]; then\n");
            script.push_str("    echo \"Generating index...\"\n");
            script.push_str("    makeindex documentation.idx\n");
            script.push_str("fi\n\n");
        }

        if self.config.generate_bibliography {
            script.push_str("if [ -f references.bib ]; then\n");
            script.push_str("    echo \"Processing bibliography...\"\n");
            script.push_str("    bibtex documentation\n");
            script.push_str("fi\n\n");
        }

        script.push_str("echo \"Running second LaTeX pass...\"\n");
        if self.config.syntax_highlighting.use_minted {
            script.push_str("pdflatex -shell-escape documentation.tex\n\n");
        } else {
            script.push_str("pdflatex documentation.tex\n\n");
        }

        script.push_str("echo \"Running final LaTeX pass...\"\n");
        if self.config.syntax_highlighting.use_minted {
            script.push_str("pdflatex -shell-escape documentation.tex\n\n");
        } else {
            script.push_str("pdflatex documentation.tex\n\n");
        }

        script.push_str("echo \"Documentation compiled successfully: documentation.pdf\"\n");
        script.push_str("echo \"Clean up auxiliary files with: make clean\"\n");

        Ok(script)
    }

    /// Escape LaTeX special characters
    fn escape_latex(&self, text: &str) -> String {
        text.replace('\\', r#"\textbackslash{}"#)
            .replace('{', r#"\{"#)
            .replace('}', r#"\}"#)
            .replace('$', r#"\$"#)
            .replace('&', r#"\&"#)
            .replace('%', r#"\%"#)
            .replace('#', r#"\#"#)
            .replace('^', r#"\textasciicircum{}"#)
            .replace('_', r#"\_"#)
            .replace('~', r#"\textasciitilde{}"#)
    }

    /// Add bibliography entry
    pub fn add_bibliography_entry(&mut self, entry: String) {
        self.bibliography_entries.push(entry);
    }

    /// Add cross-reference
    pub fn add_cross_reference(&mut self, label: String, reference: String) {
        self.cross_refs.insert(label, reference);
    }

    /// Add index entry
    pub fn add_index_entry(&mut self, term: String, page_ref: String) {
        self.index_entries.entry(term).or_insert_with(Vec::new).push(page_ref);
    }
}
