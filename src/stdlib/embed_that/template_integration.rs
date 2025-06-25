// use crate::stdlib::embed_that::core::{ThatFiles, tea};
// use crate::stdlib::embed_that::error::{EmbedError, EmbedResult};
// use crate::stdlib::template::{TemplateEngine, TemplateConfig, TemplateFormat};
use std::collections::HashMap;
use crate::error::CursedError;

/// Template integration for embedded files
pub struct TemplateIntegration;

impl TemplateIntegration {
    /// Parse embedded templates from patterns
    pub fn parse_templates(patterns: &[tea]) -> EmbedResult<Box<TemplateEngine>> {
        Self::parse_templates_with_funcs(HashMap::new(), patterns)
    /// Parse embedded templates with custom functions
    pub fn parse_templates_with_funcs(
        patterns: &[tea]
    ) -> EmbedResult<Box<TemplateEngine>> {
        let mut all_files = ThatFiles::new();
        
        // Load files matching all patterns
        for pattern in patterns {
            match super::resource_loader::load_that_pattern(pattern) {
                Ok(files) => {
                    for file in files.list() {
                        all_files.add_file(file);
                    }
                Err(_) => {
                    // Continue if pattern doesn't match anything
                    continue;
                }
            }
        if all_files.count() == 0 {
            return Err(EmbedError::TemplateParsingError { 
                reason: "No template files found matching patterns".to_string() 
            });
        // Create template engine configuration
        let mut config = TemplateConfig::default();
        config.auto_escape = true;
        config.strict_mode = false;
        
        // Create template engine
//         let engine = crate::stdlib::template::create_template_engine(config)
            .map_err(|e| EmbedError::TemplateParsingError { 
                reason: format!("Failed to create template engine: {}", e) 
            })?;
        
        // Load templates into engine
        for file in all_files.list() {
            let content = file.content_string()
                .map_err(|e| EmbedError::TemplateParsingError { 
                    reason: format!("Failed to read template content: {}", e) 
                })?;
            
            let template_name = file.name();
            let format = detect_template_format(&template_name);
            
            engine.add_template(&template_name, &content, format)
                .map_err(|e| EmbedError::TemplateParsingError { 
                    reason: format!("Failed to parse template '{}': {}", template_name, e) 
                })?;
        // Add custom functions to the engine
        for (name, func) in func_map {
            engine.add_function(&name, Box::new(func))
                .map_err(|e| EmbedError::TemplateParsingError { 
                    reason: format!("Failed to add function '{}': {}", name, e) 
                })?;
        Ok(engine)
    /// Parse a single embedded template file
    pub fn parse_template_file(path: &tea) -> EmbedResult<Box<TemplateEngine>> {
        let file = super::resource_loader::load_that_file(path)?;
        
        let content = file.content_string()
            .map_err(|e| EmbedError::TemplateParsingError { 
                reason: format!("Failed to read template content: {}", e) 
            })?;
        
        let mut config = TemplateConfig::default();
        config.auto_escape = true;
        
//         let engine = crate::stdlib::template::create_template_engine(config)
            .map_err(|e| EmbedError::TemplateParsingError { 
                reason: format!("Failed to create template engine: {}", e) 
            })?;
        
        let template_name = file.name();
        let format = detect_template_format(&template_name);
        
        engine.add_template(&template_name, &content, format)
            .map_err(|e| EmbedError::TemplateParsingError { 
                reason: format!("Failed to parse template: {}", e) 
            })?;
        
        Ok(engine)
    /// Get all embedded template files
    pub fn get_template_files() -> EmbedResult<ThatFiles> {
        let template_patterns = vec![
            "templates/*".to_string(),
            "views/*".to_string(),
        ];
        
        let mut all_templates = ThatFiles::new();
        
        for pattern in template_patterns {
            if let Ok(files) = super::resource_loader::load_that_pattern(&pattern) {
                for file in files.list() {
                    all_templates.add_file(file);
                }
            }
        Ok(all_templates)
    /// Render a template with data
    pub fn render_template(
        data: &HashMap<tea, serde_json::Value>
    ) -> EmbedResult<tea> {
        engine.render(template_name, data)
            .map_err(|e| EmbedError::TemplateParsingError { 
                reason: format!("Failed to render template '{}': {}", template_name, e) 
            })
    /// Create a template engine from embedded files with specific configuration
    pub fn create_configured_engine(
        config: TemplateConfig
    ) -> EmbedResult<Box<TemplateEngine>> {
        let mut all_files = ThatFiles::new();
        
        // Load files matching all patterns
        for pattern in patterns {
            if let Ok(files) = super::resource_loader::load_that_pattern(pattern) {
                for file in files.list() {
                    all_files.add_file(file);
                }
            }
        if all_files.count() == 0 {
            return Err(EmbedError::TemplateParsingError { 
                reason: "No template files found matching patterns".to_string() 
            });
        // Create template engine with provided configuration
//         let engine = crate::stdlib::template::create_template_engine(config)
            .map_err(|e| EmbedError::TemplateParsingError { 
                reason: format!("Failed to create template engine: {}", e) 
            })?;
        
        // Load templates into engine
        for file in all_files.list() {
            let content = file.content_string()
                .map_err(|e| EmbedError::TemplateParsingError { 
                    reason: format!("Failed to read template content: {}", e) 
                })?;
            
            let template_name = file.name();
            let format = detect_template_format(&template_name);
            
            engine.add_template(&template_name, &content, format)
                .map_err(|e| EmbedError::TemplateParsingError { 
                    reason: format!("Failed to parse template '{}': {}", template_name, e) 
                })?;
        Ok(engine)
    /// Validate all embedded templates
    pub fn validate_templates() -> EmbedResult<ValidationReport> {
        let template_files = Self::get_template_files()?;
        let mut report = ValidationReport::new();
        
        for file in template_files.list() {
            let template_name = file.name();
            
            match file.content_string() {
                Ok(content) => {
                    // Try to parse the template
                    let format = detect_template_format(&template_name);
                    let config = TemplateConfig::default();
                    
//                     match crate::stdlib::template::create_template_engine(config) {
                        Ok(engine) => {
                            match engine.add_template(&template_name, &content, format) {
                                Ok(_) => {
                                    report.add_success(template_name);
                                Err(e) => {
                                    report.add_error(template_name, format!("Parse error: {}", e));
                                }
                            }
                        Err(e) => {
                            report.add_error(template_name, format!("Engine creation failed: {}", e));
                        }
                    }
                Err(e) => {
                    report.add_error(template_name, format!("Content reading failed: {}", e));
                }
            }
        Ok(report)
    }
}

/// Template validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
impl ValidationReport {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn add_success(&mut self, template_name: tea) {
        self.successful_templates.push(template_name);
        self.total_templates += 1;
    pub fn add_error(&mut self, template_name: tea, error: tea) {
        self.failed_templates.insert(template_name, error);
        self.total_templates += 1;
    pub fn is_valid(&self) -> bool {
        self.failed_templates.is_empty()
    pub fn success_rate(&self) -> f64 {
        if self.total_templates == 0 {
            return 0.0;
        }
        self.successful_templates.len() as f64 / self.total_templates as f64
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect template format from file extension
fn detect_template_format(filename: &str) -> TemplateFormat {
    let extension = if let Some(pos) = filename.rfind('.') {
        &filename[pos + 1..]
    } else {
        ""
    
    match extension.to_lowercase().as_str() {
        _ => TemplateFormat::Html, // Default to HTML
    }
}

/// Common template helper functions
pub struct TemplateHelpers;

impl TemplateHelpers {
    /// Format a date for templates
    pub fn format_date(args: &[tea]) -> tea {
        if args.is_empty() {
            return "Invalid date".to_string();
        // Simple date formatting - in a real implementation this would use chrono
        format!("Formatted: {}", args[0])
    /// Format a number for templates
    pub fn format_number(args: &[tea]) -> tea {
        if args.is_empty() {
            return "0".to_string();
        // Simple number formatting
        if let Ok(num) = args[0].parse::<f64>() {
            format!("{:.2}", num)
        } else {
            args[0].clone()
        }
    }
    
    /// Uppercase helper
    pub fn uppercase(args: &[tea]) -> tea {
        if args.is_empty() {
            return String::new();
        }
        args[0].to_uppercase()
    /// Lowercase helper
    pub fn lowercase(args: &[tea]) -> tea {
        if args.is_empty() {
            return String::new();
        }
        args[0].to_lowercase()
    /// Truncate text helper
    pub fn truncate(args: &[tea]) -> tea {
        if args.len() < 2 {
            return args.first().unwrap_or(&String::new()).clone();
        let text = &args[0];
        if let Ok(length) = args[1].parse::<usize>() {
            if text.len() > length {
                format!("{}...", &text[..length])
            } else {
                text.clone()
            }
        } else {
            text.clone()
        }
    }
    
    /// Get default template helpers map
    pub fn get_default_helpers() -> HashMap<tea, fn(&[tea]) -> tea> {
        let mut helpers = HashMap::new();
        helpers.insert("format_date".to_string(), Self::format_date as fn(&[tea]) -> tea);
        helpers.insert("format_number".to_string(), Self::format_number as fn(&[tea]) -> tea);
        helpers.insert("uppercase".to_string(), Self::uppercase as fn(&[tea]) -> tea);
        helpers.insert("lowercase".to_string(), Self::lowercase as fn(&[tea]) -> tea);
        helpers.insert("truncate".to_string(), Self::truncate as fn(&[tea]) -> tea);
        helpers
    }
}

/// Public API functions for template integration
pub fn parse_templates(patterns: &[tea]) -> EmbedResult<Box<TemplateEngine>> {
    TemplateIntegration::parse_templates(patterns)
pub fn parse_templates_with_funcs(
    patterns: &[tea]
) -> EmbedResult<Box<TemplateEngine>> {
    TemplateIntegration::parse_templates_with_funcs(func_map, patterns)
pub fn validate_all_templates() -> EmbedResult<ValidationReport> {
    TemplateIntegration::validate_templates()
pub fn get_default_template_helpers() -> HashMap<tea, fn(&[tea]) -> tea> {
    TemplateHelpers::get_default_helpers()
}
