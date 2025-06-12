/// Template Format Support - Various output formats for CURSED templates
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, instrument, warn};
use serde_json::{Map, Value as JsonValue};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;

/// Supported template output formats
#[derive(Debug, Clone)]
pub enum TemplateFormat {
    /// Plain text output
    Text,
    /// HTML output with escaping
    Html,
    /// JSON output
    Json,
    /// YAML output
    Yaml,
    /// XML output
    Xml,
    /// Markdown output
    Markdown,
    /// CSV output
    Csv,
    /// Email template (text + HTML)
    Email,
    /// Configuration file formats
    Config(ConfigFormat),
    /// Document templates
    Document(DocumentFormat),
    /// API specification formats
    Api(ApiFormat),
    /// Build system formats
    Build(BuildFormat),
}

/// Configuration file formats
#[derive(Debug, Clone)]
pub enum ConfigFormat {
    /// TOML configuration
    Toml,
    /// INI configuration
    Ini,
    /// Environment variables
    Env,
    /// Shell script
    Shell,
    /// Dockerfile
    Dockerfile,
    /// Nginx configuration
    Nginx,
    /// Apache configuration
    Apache,
    /// Kubernetes YAML
    Kubernetes,
    /// Docker Compose
    DockerCompose,
}

/// Document template formats
#[derive(Debug, Clone)]
pub enum DocumentFormat {
    /// README file
    Readme,
    /// License file
    License,
    /// Changelog
    Changelog,
    /// Code documentation
    CodeDoc,
    /// API documentation
    ApiDoc,
    /// Project documentation
    ProjectDoc,
    /// Release notes
    ReleaseNotes,
}

/// API specification formats
#[derive(Debug, Clone)]
pub enum ApiFormat {
    /// OpenAPI/Swagger specification
    OpenApi,
    /// GraphQL schema
    GraphQL,
    /// Protocol Buffers
    Protobuf,
    /// JSON Schema
    JsonSchema,
    /// WSDL
    Wsdl,
    /// AsyncAPI
    AsyncApi,
}

/// Build system formats
#[derive(Debug, Clone)]
pub enum BuildFormat {
    /// Makefile
    Makefile,
    /// Cargo build script
    BuildRs,
    /// CMake
    CMake,
    /// Gradle
    Gradle,
    /// Maven POM
    Maven,
    /// Package.json
    PackageJson,
    /// GitHub Actions
    GitHubActions,
    /// CI/CD configuration
    CiCd,
}

/// Template format renderer
pub struct TemplateFormatRenderer {
    format: TemplateFormat,
    options: FormatOptions,
}

/// Format rendering options
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// Pretty print output
    pub pretty: bool,
    /// Indent size for pretty printing
    pub indent_size: usize,
    /// Include format validation
    pub validate: bool,
    /// Auto-escape content
    pub auto_escape: bool,
    /// Custom format options
    pub custom: HashMap<String, String>,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            pretty: true,
            indent_size: 2,
            validate: true,
            auto_escape: true,
            custom: HashMap::new(),
        }
    }
}

impl TemplateFormatRenderer {
    /// Create a new format renderer
    pub fn new(format: TemplateFormat) -> Self {
        Self { 
            format,
            options: FormatOptions::default(),
        }
    }

    /// Create a new format renderer with options
    pub fn with_options(format: TemplateFormat, options: FormatOptions) -> Self {
        Self { format, options }
    }

    /// Render data in the specified format
    #[instrument(skip(self, data))]
    pub fn render(&self, data: &CursedObject) -> Result<String, CursedError> {
        debug!(format = ?self.format, "Rendering template in format");

        match &self.format {
            TemplateFormat::Text => self.render_text(data),
            TemplateFormat::Html => self.render_html(data),
            TemplateFormat::Json => self.render_json(data),
            TemplateFormat::Yaml => self.render_yaml(data),
            TemplateFormat::Xml => self.render_xml(data),
            TemplateFormat::Markdown => self.render_markdown(data),
            TemplateFormat::Csv => self.render_csv(data),
            TemplateFormat::Email => self.render_email(data),
            TemplateFormat::Config(config_format) => self.render_config(data, config_format),
            TemplateFormat::Document(doc_format) => self.render_document(data, doc_format),
            TemplateFormat::Api(api_format) => self.render_api(data, api_format),
            TemplateFormat::Build(build_format) => self.render_build(data, build_format),
        }
    }

    /// Render as plain text
    fn render_text(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::String(s) => Ok(s.clone()),
            CursedObject::Integer(n) => Ok(n.to_string()),
            CursedObject::Float(n) => Ok(n.to_string()),
            CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Char(c) => Ok(c.to_string()),
            CursedObject::Nil => Ok("".to_string()),
            CursedObject::Array(arr) => {
                let items: Vec<String> = arr.iter()
                    .map(|item| self.render_text(item))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(items.join("\n"))
            }
            CursedObject::Map(map) => {
                let mut output = String::new();
                for (key, value) in map {
                    let value_str = self.render_text(value)?;
                    output.push_str(&format!("{}: {}\n", key, value_str));
                }
                Ok(output)
            }
        }
    }

    /// Render as HTML
    fn render_html(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::String(s) => Ok(self.escape_html(s)),
            CursedObject::Integer(n) => Ok(n.to_string()),
            CursedObject::Float(n) => Ok(n.to_string()),
            CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Char(c) => Ok(c.to_string()),
            CursedObject::Nil => Ok("".to_string()),
            CursedObject::Array(arr) => {
                let mut html = String::from("<ul>\n");
                for item in arr {
                    let item_html = self.render_html(item)?;
                    html.push_str(&format!("  <li>{}</li>\n", item_html));
                }
                html.push_str("</ul>");
                Ok(html)
            }
            CursedObject::Map(map) => {
                let mut html = String::from("<dl>\n");
                for (key, value) in map {
                    let value_html = self.render_html(value)?;
                    html.push_str(&format!("  <dt>{}</dt>\n  <dd>{}</dd>\n", 
                        self.escape_html(key), value_html));
                }
                html.push_str("</dl>");
                Ok(html)
            }
        }
    }

    /// Render as JSON
    fn render_json(&self, data: &CursedObject) -> Result<String, CursedError> {
        let json_value = self.cursed_to_json(data)?;
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| CursedError::TemplateError {
                message: format!("JSON serialization error: {}", e),
                source_location: None,
            })
    }

    /// Render as YAML
    fn render_yaml(&self, data: &CursedObject) -> Result<String, CursedError> {
        let json_value = self.cursed_to_json(data)?;
        serde_yaml::to_string(&json_value)
            .map_err(|e| CursedError::TemplateError {
                message: format!("YAML serialization error: {}", e),
                source_location: None,
            })
    }

    /// Render as XML
    fn render_xml(&self, data: &CursedObject) -> Result<String, CursedError> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<root>\n");
        xml.push_str(&self.render_xml_element("data", data, 1)?);
        xml.push_str("</root>");
        Ok(xml)
    }

    /// Render XML element recursively
    fn render_xml_element(&self, tag: &str, data: &CursedObject, depth: usize) -> Result<String, CursedError> {
        let indent = "  ".repeat(depth);
        let safe_tag = self.sanitize_xml_tag(tag);

        match data {
            CursedObject::String(s) => {
                Ok(format!("{}<{}>{}</{}>\n", indent, safe_tag, self.escape_xml(s), safe_tag))
            }
            CursedObject::Integer(n) => {
                Ok(format!("{}<{}>{}</{}>\n", indent, safe_tag, n, safe_tag))
            }
            CursedObject::Float(n) => {
                Ok(format!("{}<{}>{}</{}>\n", indent, safe_tag, n, safe_tag))
            }
            CursedObject::Boolean(b) => {
                Ok(format!("{}<{}>{}</{}>\n", indent, safe_tag, b, safe_tag))
            }
            CursedObject::Char(c) => {
                Ok(format!("{}<{}>{}</{}>\n", indent, safe_tag, self.escape_xml(&c.to_string()), safe_tag))
            }
            CursedObject::Nil => {
                Ok(format!("{}<{} />\n", indent, safe_tag))
            }
            CursedObject::Array(arr) => {
                let mut xml = format!("{}<{}>\n", indent, safe_tag);
                for (i, item) in arr.iter().enumerate() {
                    xml.push_str(&self.render_xml_element(&format!("item_{}", i), item, depth + 1)?);
                }
                xml.push_str(&format!("{}</{}>\n", indent, safe_tag));
                Ok(xml)
            }
            CursedObject::Map(map) => {
                let mut xml = format!("{}<{}>\n", indent, safe_tag);
                for (key, value) in map {
                    xml.push_str(&self.render_xml_element(key, value, depth + 1)?);
                }
                xml.push_str(&format!("{}</{}>\n", indent, safe_tag));
                Ok(xml)
            }
        }
    }

    /// Render as Markdown
    fn render_markdown(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::String(s) => Ok(s.clone()),
            CursedObject::Integer(n) => Ok(n.to_string()),
            CursedObject::Float(n) => Ok(n.to_string()),
            CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Char(c) => Ok(c.to_string()),
            CursedObject::Nil => Ok("".to_string()),
            CursedObject::Array(arr) => {
                let mut md = String::new();
                for item in arr {
                    let item_md = self.render_markdown(item)?;
                    md.push_str(&format!("- {}\n", item_md));
                }
                Ok(md)
            }
            CursedObject::Map(map) => {
                let mut md = String::new();
                for (key, value) in map {
                    let value_md = self.render_markdown(value)?;
                    md.push_str(&format!("**{}**: {}\n\n", key, value_md));
                }
                Ok(md)
            }
        }
    }

    /// Render as CSV
    fn render_csv(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Array(arr) => {
                if arr.is_empty() {
                    return Ok(String::new());
                }

                // Check if all items are maps (rows)
                if arr.iter().all(|item| matches!(item, CursedObject::Map(_))) {
                    return self.render_csv_from_maps(arr);
                }

                // Otherwise, render as simple CSV
                let mut csv = String::new();
                for item in arr {
                    let value = self.cursed_to_csv_value(item)?;
                    csv.push_str(&value);
                    csv.push('\n');
                }
                Ok(csv)
            }
            CursedObject::Map(map) => {
                // Single row CSV
                let mut headers = Vec::new();
                let mut values = Vec::new();

                for (key, value) in map {
                    headers.push(self.escape_csv(key));
                    values.push(self.cursed_to_csv_value(value)?);
                }

                let mut csv = headers.join(",");
                csv.push('\n');
                csv.push_str(&values.join(","));
                csv.push('\n');
                Ok(csv)
            }
            _ => {
                let value = self.cursed_to_csv_value(data)?;
                Ok(format!("{}\n", value))
            }
        }
    }

    /// Render CSV from array of maps
    fn render_csv_from_maps(&self, arr: &[CursedObject]) -> Result<String, CursedError> {
        if arr.is_empty() {
            return Ok(String::new());
        }

        // Get all unique keys from all maps
        let mut all_keys = std::collections::HashSet::new();
        for item in arr {
            if let CursedObject::Map(map) = item {
                for key in map.keys() {
                    all_keys.insert(key.clone());
                }
            }
        }

        let mut sorted_keys: Vec<String> = all_keys.into_iter().collect();
        sorted_keys.sort();

        // Write header
        let mut csv = sorted_keys.iter()
            .map(|k| self.escape_csv(k))
            .collect::<Vec<_>>()
            .join(",");
        csv.push('\n');

        // Write data rows
        for item in arr {
            if let CursedObject::Map(map) = item {
                let row: Vec<String> = sorted_keys.iter()
                    .map(|key| {
                        map.get(key)
                            .map(|v| self.cursed_to_csv_value(v))
                            .unwrap_or_else(|| Ok("".to_string()))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                csv.push_str(&row.join(","));
                csv.push('\n');
            }
        }

        Ok(csv)
    }

    /// Render as email template
    fn render_email(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let subject = map.get("subject")
                    .map(|s| self.render_text(s))
                    .unwrap_or_else(|| Ok("No Subject".to_string()))?;

                let text_body = map.get("text")
                    .map(|s| self.render_text(s))
                    .unwrap_or_else(|| Ok("".to_string()))?;

                let html_body = map.get("html")
                    .map(|s| self.render_html(s))
                    .unwrap_or_else(|| Ok("".to_string()))?;

                let from = map.get("from")
                    .map(|s| self.render_text(s))
                    .unwrap_or_else(|| Ok("noreply@example.com".to_string()))?;

                let to = map.get("to")
                    .map(|s| self.render_text(s))
                    .unwrap_or_else(|| Ok("recipient@example.com".to_string()))?;

                // Simple email format
                let mut email = format!("From: {}\nTo: {}\nSubject: {}\n", from, to, subject);
                email.push_str("MIME-Version: 1.0\n");
                email.push_str("Content-Type: multipart/alternative; boundary=\"boundary\"\n\n");

                if !text_body.is_empty() {
                    email.push_str("--boundary\n");
                    email.push_str("Content-Type: text/plain; charset=UTF-8\n\n");
                    email.push_str(&text_body);
                    email.push_str("\n\n");
                }

                if !html_body.is_empty() {
                    email.push_str("--boundary\n");
                    email.push_str("Content-Type: text/html; charset=UTF-8\n\n");
                    email.push_str(&html_body);
                    email.push_str("\n\n");
                }

                email.push_str("--boundary--\n");
                Ok(email)
            }
            _ => self.render_text(data),
        }
    }

    /// Render configuration files
    fn render_config(&self, data: &CursedObject, format: &ConfigFormat) -> Result<String, CursedError> {
        match format {
            ConfigFormat::Toml => self.render_toml(data),
            ConfigFormat::Ini => self.render_ini(data),
            ConfigFormat::Env => self.render_env(data),
            ConfigFormat::Shell => self.render_shell(data),
            ConfigFormat::Dockerfile => self.render_dockerfile(data),
            ConfigFormat::Nginx => self.render_nginx(data),
            ConfigFormat::Apache => self.render_apache(data),
            ConfigFormat::Kubernetes => self.render_kubernetes(data),
            ConfigFormat::DockerCompose => self.render_docker_compose(data),
        }
    }

    /// Render as TOML
    fn render_toml(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut toml = String::new();
                for (key, value) in map {
                    match value {
                        CursedObject::String(s) => {
                            toml.push_str(&format!("{} = \"{}\"\n", key, s));
                        }
                        CursedObject::Integer(n) => {
                            toml.push_str(&format!("{} = {}\n", key, n));
                        }
                        CursedObject::Float(n) => {
                            toml.push_str(&format!("{} = {}\n", key, n));
                        }
                        CursedObject::Boolean(b) => {
                            toml.push_str(&format!("{} = {}\n", key, b));
                        }
                        CursedObject::Array(arr) => {
                            let values: Vec<String> = arr.iter()
                                .map(|item| match item {
                                    CursedObject::String(s) => format!("\"{}\"", s),
                                    CursedObject::Integer(n) => n.to_string(),
                                    CursedObject::Float(n) => n.to_string(),
                                    CursedObject::Boolean(b) => b.to_string(),
                                    _ => "\"\"".to_string(),
                                })
                                .collect();
                            toml.push_str(&format!("{} = [{}]\n", key, values.join(", ")));
                        }
                        CursedObject::Map(sub_map) => {
                            toml.push_str(&format!("[{}]\n", key));
                            for (sub_key, sub_value) in sub_map {
                                let sub_toml = self.render_toml(&CursedObject::Map(
                                    std::iter::once((sub_key.clone(), sub_value.clone())).collect()
                                ))?;
                                toml.push_str(&sub_toml);
                            }
                            toml.push('\n');
                        }
                        _ => {}
                    }
                }
                Ok(toml)
            }
            _ => Err(CursedError::TemplateError {
                message: "TOML format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    /// Render as INI
    fn render_ini(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut ini = String::new();
                for (key, value) in map {
                    match value {
                        CursedObject::Map(section) => {
                            ini.push_str(&format!("[{}]\n", key));
                            for (section_key, section_value) in section {
                                let value_str = self.render_text(section_value)?;
                                ini.push_str(&format!("{} = {}\n", section_key, value_str));
                            }
                            ini.push('\n');
                        }
                        _ => {
                            let value_str = self.render_text(value)?;
                            ini.push_str(&format!("{} = {}\n", key, value_str));
                        }
                    }
                }
                Ok(ini)
            }
            _ => Err(CursedError::TemplateError {
                message: "INI format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    /// Render as environment variables
    fn render_env(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut env = String::new();
                for (key, value) in map {
                    let value_str = self.render_text(value)?;
                    let env_key = key.to_uppercase().replace(' ', "_");
                    env.push_str(&format!("{}={}\n", env_key, self.escape_shell(&value_str)));
                }
                Ok(env)
            }
            _ => Err(CursedError::TemplateError {
                message: "Environment format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    /// Render as shell script
    fn render_shell(&self, data: &CursedObject) -> Result<String, CursedError> {
        let mut script = String::from("#!/bin/bash\n\n");
        match data {
            CursedObject::Map(map) => {
                for (key, value) in map {
                    let value_str = self.render_text(value)?;
                    let var_name = key.to_uppercase().replace(' ', "_");
                    script.push_str(&format!("{}={}\n", var_name, self.escape_shell(&value_str)));
                }
            }
            CursedObject::Array(commands) => {
                for command in commands {
                    let cmd_str = self.render_text(command)?;
                    script.push_str(&format!("{}\n", cmd_str));
                }
            }
            _ => {
                let cmd_str = self.render_text(data)?;
                script.push_str(&format!("{}\n", cmd_str));
            }
        }
        Ok(script)
    }

    /// Render as Dockerfile
    fn render_dockerfile(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut dockerfile = String::new();
                
                // Common Dockerfile instructions
                if let Some(from) = map.get("from") {
                    let from_str = self.render_text(from)?;
                    dockerfile.push_str(&format!("FROM {}\n", from_str));
                }

                if let Some(workdir) = map.get("workdir") {
                    let workdir_str = self.render_text(workdir)?;
                    dockerfile.push_str(&format!("WORKDIR {}\n", workdir_str));
                }

                if let Some(run) = map.get("run") {
                    match run {
                        CursedObject::Array(commands) => {
                            for cmd in commands {
                                let cmd_str = self.render_text(cmd)?;
                                dockerfile.push_str(&format!("RUN {}\n", cmd_str));
                            }
                        }
                        _ => {
                            let cmd_str = self.render_text(run)?;
                            dockerfile.push_str(&format!("RUN {}\n", cmd_str));
                        }
                    }
                }

                if let Some(copy) = map.get("copy") {
                    match copy {
                        CursedObject::Array(files) => {
                            for file in files {
                                let file_str = self.render_text(file)?;
                                dockerfile.push_str(&format!("COPY {}\n", file_str));
                            }
                        }
                        _ => {
                            let file_str = self.render_text(copy)?;
                            dockerfile.push_str(&format!("COPY {}\n", file_str));
                        }
                    }
                }

                if let Some(expose) = map.get("expose") {
                    let port_str = self.render_text(expose)?;
                    dockerfile.push_str(&format!("EXPOSE {}\n", port_str));
                }

                if let Some(cmd) = map.get("cmd") {
                    let cmd_str = self.render_text(cmd)?;
                    dockerfile.push_str(&format!("CMD {}\n", cmd_str));
                }

                Ok(dockerfile)
            }
            _ => Err(CursedError::TemplateError {
                message: "Dockerfile format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    /// Render as Nginx configuration
    fn render_nginx(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut nginx = String::new();

                for (directive, value) in map {
                    match directive.as_str() {
                        "server" => {
                            nginx.push_str("server {\n");
                            if let CursedObject::Map(server_config) = value {
                                for (server_directive, server_value) in server_config {
                                    let value_str = self.render_text(server_value)?;
                                    nginx.push_str(&format!("    {} {};\n", server_directive, value_str));
                                }
                            }
                            nginx.push_str("}\n\n");
                        }
                        "upstream" => {
                            nginx.push_str(&format!("upstream {} {{\n", self.render_text(value)?));
                            nginx.push_str("}\n\n");
                        }
                        _ => {
                            let value_str = self.render_text(value)?;
                            nginx.push_str(&format!("{} {};\n", directive, value_str));
                        }
                    }
                }

                Ok(nginx)
            }
            _ => Err(CursedError::TemplateError {
                message: "Nginx format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    /// Render as Apache configuration
    fn render_apache(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut apache = String::new();

                for (directive, value) in map {
                    match directive.as_str() {
                        "virtualhost" => {
                            if let CursedObject::Map(vhost_config) = value {
                                if let Some(addr) = vhost_config.get("address") {
                                    let addr_str = self.render_text(addr)?;
                                    apache.push_str(&format!("<VirtualHost {}>\n", addr_str));
                                    
                                    for (vhost_directive, vhost_value) in vhost_config {
                                        if vhost_directive != "address" {
                                            let value_str = self.render_text(vhost_value)?;
                                            apache.push_str(&format!("    {} {}\n", vhost_directive, value_str));
                                        }
                                    }
                                    apache.push_str("</VirtualHost>\n\n");
                                }
                            }
                        }
                        "directory" => {
                            if let CursedObject::Map(dir_config) = value {
                                if let Some(path) = dir_config.get("path") {
                                    let path_str = self.render_text(path)?;
                                    apache.push_str(&format!("<Directory {}>\n", path_str));
                                    
                                    for (dir_directive, dir_value) in dir_config {
                                        if dir_directive != "path" {
                                            let value_str = self.render_text(dir_value)?;
                                            apache.push_str(&format!("    {} {}\n", dir_directive, value_str));
                                        }
                                    }
                                    apache.push_str("</Directory>\n\n");
                                }
                            }
                        }
                        _ => {
                            let value_str = self.render_text(value)?;
                            apache.push_str(&format!("{} {}\n", directive, value_str));
                        }
                    }
                }

                Ok(apache)
            }
            _ => Err(CursedError::TemplateError {
                message: "Apache format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    /// Render as Kubernetes YAML
    fn render_kubernetes(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut k8s = String::from("apiVersion: v1\n");
                
                if let Some(kind) = map.get("kind") {
                    let kind_str = self.render_text(kind)?;
                    k8s.push_str(&format!("kind: {}\n", kind_str));
                }

                k8s.push_str("metadata:\n");
                if let Some(metadata) = map.get("metadata") {
                    let metadata_yaml = self.render_kubernetes_object(metadata, 1)?;
                    k8s.push_str(&metadata_yaml);
                }

                if let Some(spec) = map.get("spec") {
                    k8s.push_str("spec:\n");
                    let spec_yaml = self.render_kubernetes_object(spec, 1)?;
                    k8s.push_str(&spec_yaml);
                }

                Ok(k8s)
            }
            _ => self.render_yaml(data)
        }
    }

    /// Render Docker Compose
    fn render_docker_compose(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut compose = String::from("version: '3.8'\n\n");
                
                if let Some(services) = map.get("services") {
                    compose.push_str("services:\n");
                    if let CursedObject::Map(service_map) = services {
                        for (service_name, service_config) in service_map {
                            compose.push_str(&format!("  {}:\n", service_name));
                            let service_yaml = self.render_kubernetes_object(service_config, 2)?;
                            compose.push_str(&service_yaml);
                        }
                    }
                }

                if let Some(volumes) = map.get("volumes") {
                    compose.push_str("\nvolumes:\n");
                    let volumes_yaml = self.render_kubernetes_object(volumes, 1)?;
                    compose.push_str(&volumes_yaml);
                }

                if let Some(networks) = map.get("networks") {
                    compose.push_str("\nnetworks:\n");
                    let networks_yaml = self.render_kubernetes_object(networks, 1)?;
                    compose.push_str(&networks_yaml);
                }

                Ok(compose)
            }
            _ => self.render_yaml(data)
        }
    }

    /// Render document templates
    fn render_document(&self, data: &CursedObject, format: &DocumentFormat) -> Result<String, CursedError> {
        match format {
            DocumentFormat::Readme => self.render_readme(data),
            DocumentFormat::License => self.render_license(data),
            DocumentFormat::Changelog => self.render_changelog(data),
            DocumentFormat::CodeDoc => self.render_code_doc(data),
            DocumentFormat::ApiDoc => self.render_api_doc(data),
            DocumentFormat::ProjectDoc => self.render_project_doc(data),
            DocumentFormat::ReleaseNotes => self.render_release_notes(data),
        }
    }

    /// Render API specifications
    fn render_api(&self, data: &CursedObject, format: &ApiFormat) -> Result<String, CursedError> {
        match format {
            ApiFormat::OpenApi => self.render_openapi(data),
            ApiFormat::GraphQL => self.render_graphql(data),
            ApiFormat::Protobuf => self.render_protobuf(data),
            ApiFormat::JsonSchema => self.render_json_schema(data),
            ApiFormat::Wsdl => self.render_wsdl(data),
            ApiFormat::AsyncApi => self.render_asyncapi(data),
        }
    }

    /// Render build system files
    fn render_build(&self, data: &CursedObject, format: &BuildFormat) -> Result<String, CursedError> {
        match format {
            BuildFormat::Makefile => self.render_makefile(data),
            BuildFormat::BuildRs => self.render_build_rs(data),
            BuildFormat::CMake => self.render_cmake(data),
            BuildFormat::Gradle => self.render_gradle(data),
            BuildFormat::Maven => self.render_maven(data),
            BuildFormat::PackageJson => self.render_package_json(data),
            BuildFormat::GitHubActions => self.render_github_actions(data),
            BuildFormat::CiCd => self.render_ci_cd(data),
        }
    }

    /// Render README file
    fn render_readme(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut readme = String::new();
                
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    readme.push_str(&format!("# {}\n\n", title_str));
                }

                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    readme.push_str(&format!("{}\n\n", desc_str));
                }

                if let Some(installation) = map.get("installation") {
                    readme.push_str("## Installation\n\n");
                    let install_str = self.render_text(installation)?;
                    readme.push_str(&format!("{}\n\n", install_str));
                }

                if let Some(usage) = map.get("usage") {
                    readme.push_str("## Usage\n\n");
                    let usage_str = self.render_text(usage)?;
                    readme.push_str(&format!("{}\n\n", usage_str));
                }

                if let Some(license) = map.get("license") {
                    readme.push_str("## License\n\n");
                    let license_str = self.render_text(license)?;
                    readme.push_str(&format!("{}\n\n", license_str));
                }

                Ok(readme)
            }
            _ => self.render_markdown(data)
        }
    }

    /// Render OpenAPI specification
    fn render_openapi(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut openapi = String::from("openapi: 3.0.0\n");
                
                if let Some(info) = map.get("info") {
                    openapi.push_str("info:\n");
                    let info_yaml = self.render_kubernetes_object(info, 1)?;
                    openapi.push_str(&info_yaml);
                }

                if let Some(paths) = map.get("paths") {
                    openapi.push_str("paths:\n");
                    let paths_yaml = self.render_kubernetes_object(paths, 1)?;
                    openapi.push_str(&paths_yaml);
                }

                Ok(openapi)
            }
            _ => self.render_yaml(data)
        }
    }

    /// Render Makefile
    fn render_makefile(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::Map(map) => {
                let mut makefile = String::new();
                
                for (target, config) in map {
                    match config {
                        CursedObject::Map(target_config) => {
                            // Add dependencies if specified
                            if let Some(deps) = target_config.get("dependencies") {
                                let deps_str = self.render_text(deps)?;
                                makefile.push_str(&format!("{}: {}\n", target, deps_str));
                            } else {
                                makefile.push_str(&format!("{}:\n", target));
                            }

                            // Add commands
                            if let Some(commands) = target_config.get("commands") {
                                match commands {
                                    CursedObject::Array(cmd_array) => {
                                        for cmd in cmd_array {
                                            let cmd_str = self.render_text(cmd)?;
                                            makefile.push_str(&format!("\t{}\n", cmd_str));
                                        }
                                    }
                                    _ => {
                                        let cmd_str = self.render_text(commands)?;
                                        makefile.push_str(&format!("\t{}\n", cmd_str));
                                    }
                                }
                            }
                        }
                        _ => {
                            makefile.push_str(&format!("{}:\n", target));
                            let cmd_str = self.render_text(config)?;
                            makefile.push_str(&format!("\t{}\n", cmd_str));
                        }
                    }
                    makefile.push('\n');
                }

                Ok(makefile)
            }
            _ => Err(CursedError::TemplateError {
                message: "Makefile format requires a map".to_string(),
                source_location: None,
            }),
        }
    }

    // Utility methods for the new formats
    fn render_kubernetes_object(&self, obj: &CursedObject, indent_level: usize) -> Result<String, CursedError> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match obj {
            CursedObject::Map(map) => {
                for (key, value) in map {
                    match value {
                        CursedObject::Map(_) => {
                            result.push_str(&format!("{}{}:\n", indent, key));
                            result.push_str(&self.render_kubernetes_object(value, indent_level + 1)?);
                        }
                        CursedObject::Array(arr) => {
                            result.push_str(&format!("{}{}:\n", indent, key));
                            for item in arr {
                                result.push_str(&format!("{}- ", "  ".repeat(indent_level)));
                                match item {
                                    CursedObject::Map(_) => {
                                        result.push('\n');
                                        result.push_str(&self.render_kubernetes_object(item, indent_level + 1)?);
                                    }
                                    _ => {
                                        let item_str = self.render_text(item)?;
                                        result.push_str(&format!("{}\n", item_str));
                                    }
                                }
                            }
                        }
                        _ => {
                            let value_str = self.render_text(value)?;
                            result.push_str(&format!("{}{}: {}\n", indent, key, value_str));
                        }
                    }
                }
            }
            _ => {
                let value_str = self.render_text(obj)?;
                result.push_str(&format!("{}{}\n", indent, value_str));
            }
        }

        Ok(result)
    }

    // Placeholder implementations for the remaining formats
    fn render_license(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_text(data) }
    fn render_changelog(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_markdown(data) }
    fn render_code_doc(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_markdown(data) }
    fn render_api_doc(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_markdown(data) }
    fn render_project_doc(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_markdown(data) }
    fn render_release_notes(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_markdown(data) }
    fn render_graphql(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_text(data) }
    fn render_protobuf(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_text(data) }
    fn render_json_schema(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_json(data) }
    fn render_wsdl(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_xml(data) }
    fn render_asyncapi(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_yaml(data) }
    fn render_build_rs(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_text(data) }
    fn render_cmake(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_text(data) }
    fn render_gradle(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_text(data) }
    fn render_maven(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_xml(data) }
    fn render_package_json(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_json(data) }
    fn render_github_actions(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_yaml(data) }
    fn render_ci_cd(&self, data: &CursedObject) -> Result<String, CursedError> { self.render_yaml(data) }

    /// Helper methods for escaping and conversion
    fn escape_html(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
    }

    fn escape_xml(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
    }

    fn escape_csv(&self, s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }

    fn escape_shell(&self, s: &str) -> String {
        if s.contains(' ') || s.contains('"') || s.contains('\'') || s.contains('$') {
            format!("\"{}\"", s.replace('"', "\\\""))
        } else {
            s.to_string()
        }
    }

    fn sanitize_xml_tag(&self, tag: &str) -> String {
        tag.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect::<String>()
            .trim_matches(|c: char| c.is_ascii_digit())
            .to_string()
    }

    fn cursed_to_json(&self, obj: &CursedObject) -> Result<JsonValue, CursedError> {
        match obj {
            CursedObject::String(s) => Ok(JsonValue::String(s.clone())),
            CursedObject::Integer(n) => Ok(JsonValue::Number(serde_json::Number::from(*n))),
            CursedObject::Float(n) => {
                if let Some(num) = serde_json::Number::from_f64(*n) {
                    Ok(JsonValue::Number(num))
                } else {
                    Ok(JsonValue::Null)
                }
            }
            CursedObject::Boolean(b) => Ok(JsonValue::Bool(*b)),
            CursedObject::Char(c) => Ok(JsonValue::String(c.to_string())),
            CursedObject::Nil => Ok(JsonValue::Null),
            CursedObject::Array(arr) => {
                let json_arr: Result<Vec<JsonValue>, CursedError> = arr.iter()
                    .map(|item| self.cursed_to_json(item))
                    .collect();
                Ok(JsonValue::Array(json_arr?))
            }
            CursedObject::Map(map) => {
                let mut json_map = Map::new();
                for (key, value) in map {
                    json_map.insert(key.clone(), self.cursed_to_json(value)?);
                }
                Ok(JsonValue::Object(json_map))
            }
        }
    }

    fn cursed_to_csv_value(&self, obj: &CursedObject) -> Result<String, CursedError> {
        let text = self.render_text(obj)?;
        Ok(self.escape_csv(&text))
    }

    /// Get Content-Type header for the format
    pub fn content_type(&self) -> &'static str {
        match &self.format {
            TemplateFormat::Text => "text/plain",
            TemplateFormat::Html => "text/html",
            TemplateFormat::Json => "application/json",
            TemplateFormat::Yaml => "application/x-yaml",
            TemplateFormat::Xml => "application/xml",
            TemplateFormat::Markdown => "text/markdown",
            TemplateFormat::Csv => "text/csv",
            TemplateFormat::Email => "message/rfc822",
            TemplateFormat::Config(config_format) => match config_format {
                ConfigFormat::Toml => "application/toml",
                ConfigFormat::Ini => "text/plain",
                ConfigFormat::Env => "text/plain",
                ConfigFormat::Shell => "application/x-sh",
                ConfigFormat::Dockerfile => "text/plain",
                ConfigFormat::Nginx => "text/plain",
                ConfigFormat::Apache => "text/plain",
                ConfigFormat::Kubernetes => "application/x-yaml",
                ConfigFormat::DockerCompose => "application/x-yaml",
            },
            TemplateFormat::Document(_) => "text/markdown",
            TemplateFormat::Api(api_format) => match api_format {
                ApiFormat::OpenApi => "application/x-yaml",
                ApiFormat::GraphQL => "application/graphql",
                ApiFormat::Protobuf => "application/x-protobuf",
                ApiFormat::JsonSchema => "application/schema+json",
                ApiFormat::Wsdl => "application/wsdl+xml",
                ApiFormat::AsyncApi => "application/x-yaml",
            },
            TemplateFormat::Build(_) => "text/plain",
        }
    }

    /// Validate rendered output for the format
    pub fn validate(&self, content: &str) -> Result<(), CursedError> {
        if !self.options.validate {
            return Ok(());
        }

        match &self.format {
            TemplateFormat::Json => {
                serde_json::from_str::<JsonValue>(content)
                    .map_err(|e| CursedError::TemplateError {
                        message: format!("Invalid JSON: {}", e),
                        source_location: None,
                    })?;
            }
            TemplateFormat::Yaml => {
                serde_yaml::from_str::<JsonValue>(content)
                    .map_err(|e| CursedError::TemplateError {
                        message: format!("Invalid YAML: {}", e),
                        source_location: None,
                    })?;
            }
            TemplateFormat::Xml => {
                // Basic XML validation - just check for well-formed structure
                if !self.is_well_formed_xml(content) {
                    return Err(CursedError::TemplateError {
                        message: "Invalid XML structure".to_string(),
                        source_location: None,
                    });
                }
            }
            _ => {} // No validation for other formats yet
        }
        Ok(())
    }

    /// Basic XML well-formedness check
    fn is_well_formed_xml(&self, content: &str) -> bool {
        let mut stack = Vec::new();
        let mut in_tag = false;
        let mut tag_name = String::new();
        let mut is_closing = false;
        
        for ch in content.chars() {
            match ch {
                '<' => {
                    in_tag = true;
                    tag_name.clear();
                    is_closing = false;
                }
                '>' => {
                    if in_tag && !tag_name.is_empty() {
                        if is_closing {
                            if let Some(last_tag) = stack.pop() {
                                if last_tag != tag_name {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else if !tag_name.ends_with('/') {
                            stack.push(tag_name.clone());
                        }
                    }
                    in_tag = false;
                }
                '/' if in_tag && tag_name.is_empty() => {
                    is_closing = true;
                }
                _ if in_tag => {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        tag_name.push(ch);
                    }
                }
                _ => {}
            }
        }
        
        stack.is_empty()
    }
}

/// Format detection utilities
pub struct FormatDetector;

impl FormatDetector {
    /// Detect format from file extension
    pub fn from_extension(path: &str) -> Option<TemplateFormat> {
        let path = Path::new(path);
        let extension = path.extension()?.to_str()?.to_lowercase();
        
        match extension.as_str() {
            "txt" => Some(TemplateFormat::Text),
            "html" | "htm" => Some(TemplateFormat::Html),
            "json" => Some(TemplateFormat::Json),
            "yaml" | "yml" => Some(TemplateFormat::Yaml),
            "xml" => Some(TemplateFormat::Xml),
            "md" | "markdown" => Some(TemplateFormat::Markdown),
            "csv" => Some(TemplateFormat::Csv),
            "toml" => Some(TemplateFormat::Config(ConfigFormat::Toml)),
            "ini" => Some(TemplateFormat::Config(ConfigFormat::Ini)),
            "env" => Some(TemplateFormat::Config(ConfigFormat::Env)),
            "sh" | "bash" => Some(TemplateFormat::Config(ConfigFormat::Shell)),
            "dockerfile" => Some(TemplateFormat::Config(ConfigFormat::Dockerfile)),
            "conf" => Some(TemplateFormat::Config(ConfigFormat::Nginx)),
            "graphql" | "gql" => Some(TemplateFormat::Api(ApiFormat::GraphQL)),
            "proto" => Some(TemplateFormat::Api(ApiFormat::Protobuf)),
            "makefile" | "mk" => Some(TemplateFormat::Build(BuildFormat::Makefile)),
            "rs" if path.file_name().unwrap_or_default() == "build.rs" => {
                Some(TemplateFormat::Build(BuildFormat::BuildRs))
            },
            _ => {
                // Check by filename
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    match filename.to_lowercase().as_str() {
                        "makefile" => Some(TemplateFormat::Build(BuildFormat::Makefile)),
                        "dockerfile" => Some(TemplateFormat::Config(ConfigFormat::Dockerfile)),
                        "docker-compose.yml" | "docker-compose.yaml" => {
                            Some(TemplateFormat::Config(ConfigFormat::DockerCompose))
                        },
                        "readme.md" | "readme" => Some(TemplateFormat::Document(DocumentFormat::Readme)),
                        "license" | "license.txt" => Some(TemplateFormat::Document(DocumentFormat::License)),
                        "changelog.md" | "changelog" => Some(TemplateFormat::Document(DocumentFormat::Changelog)),
                        "package.json" => Some(TemplateFormat::Build(BuildFormat::PackageJson)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Detect format from content analysis
    pub fn from_content(content: &str) -> Option<TemplateFormat> {
        let trimmed = content.trim();
        
        // Check for JSON
        if (trimmed.starts_with('{') && trimmed.ends_with('}')) ||
           (trimmed.starts_with('[') && trimmed.ends_with(']')) {
            if serde_json::from_str::<JsonValue>(content).is_ok() {
                return Some(TemplateFormat::Json);
            }
        }
        
        // Check for XML
        if trimmed.starts_with("<?xml") || 
           (trimmed.starts_with('<') && trimmed.ends_with('>')) {
            return Some(TemplateFormat::Xml);
        }
        
        // Check for HTML
        if trimmed.contains("<!DOCTYPE html") || 
           trimmed.contains("<html") ||
           trimmed.contains("<head>") ||
           trimmed.contains("<body>") {
            return Some(TemplateFormat::Html);
        }
        
        // Check for YAML
        if content.lines().any(|line| {
            let line = line.trim();
            line.contains(':') && !line.starts_with('#') && 
            !line.contains('=') && !line.contains('<')
        }) {
            return Some(TemplateFormat::Yaml);
        }
        
        // Check for specific formats by content patterns
        if content.lines().any(|line| line.trim().starts_with("FROM ")) {
            return Some(TemplateFormat::Config(ConfigFormat::Dockerfile));
        }
        
        if content.lines().any(|line| line.trim().starts_with("server {")) {
            return Some(TemplateFormat::Config(ConfigFormat::Nginx));
        }
        
        if content.contains("openapi:") || content.contains("swagger:") {
            return Some(TemplateFormat::Api(ApiFormat::OpenApi));
        }
        
        if content.contains("type Query") || content.contains("type Mutation") {
            return Some(TemplateFormat::Api(ApiFormat::GraphQL));
        }
        
        None
    }
}

/// Format conversion utilities
pub struct FormatConverter;

impl FormatConverter {
    /// Convert between formats
    pub fn convert(
        content: &str,
        from: TemplateFormat,
        to: TemplateFormat,
        data: &CursedObject,
    ) -> Result<String, CursedError> {
        // For now, just re-render with the target format
        let renderer = TemplateFormatRenderer::new(to);
        renderer.render(data)
    }

    /// Compose multiple templates
    pub fn compose(
        templates: &[(TemplateFormat, &CursedObject)],
        separator: &str,
    ) -> Result<String, CursedError> {
        let mut result = String::new();
        
        for (i, (format, data)) in templates.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            
            let renderer = TemplateFormatRenderer::new(format.clone());
            let rendered = renderer.render(data)?;
            result.push_str(&rendered);
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_json_rendering() {
        let renderer = TemplateFormatRenderer::new(TemplateFormat::Json);
        
        let mut map = HashMap::new();
        map.insert("name".to_string(), CursedObject::String("Alice".to_string()));
        map.insert("age".to_string(), CursedObject::Integer(25));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("\"name\""));
        assert!(result.contains("\"Alice\""));
        assert!(result.contains("\"age\""));
        assert!(result.contains("25"));
    }

    #[test]
    fn test_html_rendering() {
        let renderer = TemplateFormatRenderer::new(TemplateFormat::Html);
        
        let arr = vec![
            CursedObject::String("Item 1".to_string()),
            CursedObject::String("Item 2".to_string()),
        ];
        let data = CursedObject::Array(arr);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>Item 1</li>"));
        assert!(result.contains("<li>Item 2</li>"));
        assert!(result.contains("</ul>"));
    }

    #[test]
    fn test_csv_rendering() {
        let renderer = TemplateFormatRenderer::new(TemplateFormat::Csv);
        
        let mut map1 = HashMap::new();
        map1.insert("name".to_string(), CursedObject::String("Alice".to_string()));
        map1.insert("age".to_string(), CursedObject::Integer(25));
        
        let mut map2 = HashMap::new();
        map2.insert("name".to_string(), CursedObject::String("Bob".to_string()));
        map2.insert("age".to_string(), CursedObject::Integer(30));
        
        let arr = vec![CursedObject::Map(map1), CursedObject::Map(map2)];
        let data = CursedObject::Array(arr);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("name,age"));
        assert!(result.contains("Alice,25"));
        assert!(result.contains("Bob,30"));
    }

    #[test]
    fn test_toml_rendering() {
        let renderer = TemplateFormatRenderer::new(
            TemplateFormat::Config(ConfigFormat::Toml)
        );
        
        let mut map = HashMap::new();
        map.insert("title".to_string(), CursedObject::String("My App".to_string()));
        map.insert("debug".to_string(), CursedObject::Boolean(true));
        map.insert("port".to_string(), CursedObject::Integer(8080));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("title = \"My App\""));
        assert!(result.contains("debug = true"));
        assert!(result.contains("port = 8080"));
    }

    #[test]
    fn test_xml_rendering() {
        let renderer = TemplateFormatRenderer::new(TemplateFormat::Xml);
        
        let mut map = HashMap::new();
        map.insert("name".to_string(), CursedObject::String("Alice".to_string()));
        map.insert("score".to_string(), CursedObject::Integer(95));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("<?xml version=\"1.0\""));
        assert!(result.contains("<root>"));
        assert!(result.contains("<name>Alice</name>"));
        assert!(result.contains("<score>95</score>"));
        assert!(result.contains("</root>"));
    }

    #[test]
    fn test_html_escaping() {
        let renderer = TemplateFormatRenderer::new(TemplateFormat::Html);
        
        let data = CursedObject::String("<script>alert('xss')</script>".to_string());
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("&lt;script&gt;"));
        assert!(!result.contains("<script>"));
    }

    #[test]
    fn test_readme_rendering() {
        let renderer = TemplateFormatRenderer::new(
            TemplateFormat::Document(DocumentFormat::Readme)
        );
        
        let mut map = HashMap::new();
        map.insert("title".to_string(), CursedObject::String("My Project".to_string()));
        map.insert("description".to_string(), CursedObject::String("A great project".to_string()));
        map.insert("installation".to_string(), CursedObject::String("npm install".to_string()));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("# My Project"));
        assert!(result.contains("A great project"));
        assert!(result.contains("## Installation"));
        assert!(result.contains("npm install"));
    }

    #[test]
    fn test_makefile_rendering() {
        let renderer = TemplateFormatRenderer::new(
            TemplateFormat::Build(BuildFormat::Makefile)
        );
        
        let mut build_config = HashMap::new();
        build_config.insert("dependencies".to_string(), CursedObject::String("clean".to_string()));
        
        let commands = vec![
            CursedObject::String("cargo build".to_string()),
            CursedObject::String("strip target/release/myapp".to_string()),
        ];
        build_config.insert("commands".to_string(), CursedObject::Array(commands));
        
        let mut map = HashMap::new();
        map.insert("build".to_string(), CursedObject::Map(build_config));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("build: clean"));
        assert!(result.contains("\tcargo build"));
        assert!(result.contains("\tstrip target/release/myapp"));
    }

    #[test]
    fn test_kubernetes_rendering() {
        let renderer = TemplateFormatRenderer::new(
            TemplateFormat::Config(ConfigFormat::Kubernetes)
        );
        
        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), CursedObject::String("my-app".to_string()));
        metadata.insert("namespace".to_string(), CursedObject::String("default".to_string()));
        
        let mut spec = HashMap::new();
        spec.insert("replicas".to_string(), CursedObject::Integer(3));
        
        let mut map = HashMap::new();
        map.insert("kind".to_string(), CursedObject::String("Deployment".to_string()));
        map.insert("metadata".to_string(), CursedObject::Map(metadata));
        map.insert("spec".to_string(), CursedObject::Map(spec));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("apiVersion: v1"));
        assert!(result.contains("kind: Deployment"));
        assert!(result.contains("name: my-app"));
        assert!(result.contains("replicas: 3"));
    }

    #[test]
    fn test_openapi_rendering() {
        let renderer = TemplateFormatRenderer::new(
            TemplateFormat::Api(ApiFormat::OpenApi)
        );
        
        let mut info = HashMap::new();
        info.insert("title".to_string(), CursedObject::String("My API".to_string()));
        info.insert("version".to_string(), CursedObject::String("1.0.0".to_string()));
        
        let mut paths = HashMap::new();
        let mut get_users = HashMap::new();
        get_users.insert("summary".to_string(), CursedObject::String("Get users".to_string()));
        paths.insert("/users".to_string(), CursedObject::Map(get_users));
        
        let mut map = HashMap::new();
        map.insert("info".to_string(), CursedObject::Map(info));
        map.insert("paths".to_string(), CursedObject::Map(paths));
        
        let data = CursedObject::Map(map);
        let result = renderer.render(&data).unwrap();
        
        assert!(result.contains("openapi: 3.0.0"));
        assert!(result.contains("title: My API"));
        assert!(result.contains("version: 1.0.0"));
    }

    #[test]
    fn test_format_detection_from_extension() {
        assert!(matches!(
            FormatDetector::from_extension("file.json"),
            Some(TemplateFormat::Json)
        ));
        
        assert!(matches!(
            FormatDetector::from_extension("file.yaml"),
            Some(TemplateFormat::Yaml)
        ));
        
        assert!(matches!(
            FormatDetector::from_extension("Makefile"),
            Some(TemplateFormat::Build(BuildFormat::Makefile))
        ));
        
        assert!(matches!(
            FormatDetector::from_extension("README.md"),
            Some(TemplateFormat::Document(DocumentFormat::Readme))
        ));
    }

    #[test]
    fn test_format_detection_from_content() {
        assert!(matches!(
            FormatDetector::from_content(r#"{"key": "value"}"#),
            Some(TemplateFormat::Json)
        ));
        
        assert!(matches!(
            FormatDetector::from_content("<?xml version=\"1.0\"?><root></root>"),
            Some(TemplateFormat::Xml)
        ));
        
        assert!(matches!(
            FormatDetector::from_content("FROM ubuntu:20.04\nRUN apt-get update"),
            Some(TemplateFormat::Config(ConfigFormat::Dockerfile))
        ));
        
        assert!(matches!(
            FormatDetector::from_content("server {\n  listen 80;\n}"),
            Some(TemplateFormat::Config(ConfigFormat::Nginx))
        ));
    }

    #[test]
    fn test_content_type_headers() {
        let json_renderer = TemplateFormatRenderer::new(TemplateFormat::Json);
        assert_eq!(json_renderer.content_type(), "application/json");
        
        let html_renderer = TemplateFormatRenderer::new(TemplateFormat::Html);
        assert_eq!(html_renderer.content_type(), "text/html");
        
        let yaml_renderer = TemplateFormatRenderer::new(TemplateFormat::Yaml);
        assert_eq!(yaml_renderer.content_type(), "application/x-yaml");
    }

    #[test]
    fn test_format_validation() {
        let json_renderer = TemplateFormatRenderer::new(TemplateFormat::Json);
        
        // Valid JSON should pass
        assert!(json_renderer.validate(r#"{"valid": true}"#).is_ok());
        
        // Invalid JSON should fail
        assert!(json_renderer.validate(r#"{"invalid": }"#).is_err());
    }

    #[test]
    fn test_template_composition() {
        let mut map1 = HashMap::new();
        map1.insert("section".to_string(), CursedObject::String("Header".to_string()));
        
        let mut map2 = HashMap::new();
        map2.insert("section".to_string(), CursedObject::String("Body".to_string()));
        
        let obj1 = CursedObject::Map(map1);
        let obj2 = CursedObject::Map(map2);
        
        let templates = vec![
            (TemplateFormat::Text, &obj1),
            (TemplateFormat::Text, &obj2),
        ];
        
        let result = FormatConverter::compose(&templates, "\n---\n").unwrap();
        assert!(result.contains("Header"));
        assert!(result.contains("Body"));
        assert!(result.contains("---"));
    }
}
