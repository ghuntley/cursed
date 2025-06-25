use crate::error::CursedError;
/// Template Format Support - Various output formats for CURSED templates
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, instrument, warn};
use serde_json::{Map, Value as JsonValue};

use crate::object::Object as CursedObject;

/// Supported template output formats
#[derive(Debug, Clone)]
pub enum TemplateFormat {
    /// Plain text output
    /// HTML output with escaping
    /// JSON output
    /// YAML output
    /// XML output
    /// Markdown output
    /// CSV output
    /// Email template (text + HTML)
    /// Configuration file formats
    /// Document templates
    /// API specification formats
    /// Build system formats
/// Configuration file formats
#[derive(Debug, Clone)]
pub enum ConfigFormat {
    /// TOML configuration
    /// INI configuration
    /// Environment variables
    /// Shell script
    /// Dockerfile
    /// Nginx configuration
    /// Apache configuration
    /// Kubernetes YAML
    /// Docker Compose
/// Document template formats
#[derive(Debug, Clone)]
pub enum DocumentFormat {
    /// README file
    /// License file
    /// Changelog
    /// Code documentation
    /// API documentation
    /// Project documentation
    /// Release notes
/// API specification formats
#[derive(Debug, Clone)]
pub enum ApiFormat {
    /// OpenAPI/Swagger specification
    /// GraphQL schema
    /// Protocol Buffers
    /// JSON Schema
    /// WSDL
    /// AsyncAPI
/// Build system formats
#[derive(Debug, Clone)]
pub enum BuildFormat {
    /// Makefile
    /// Cargo build script
    /// CMake
    /// Gradle
    /// Maven POM
    /// Package.json
    /// GitHub Actions
    /// CI/CD configuration
/// Template format renderer
pub struct TemplateFormatRenderer {
/// Format rendering options
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// Pretty print output
    /// Indent size for pretty printing
    /// Include format validation
    /// Auto-escape content
    /// Custom format options
impl Default for FormatOptions {
    fn default() -> Self {
        Self {
        }
    }
impl TemplateFormatRenderer {
    /// Create a new format renderer
    pub fn new(format: TemplateFormat) -> Self {
        Self { 
        }
    }

    /// Create a new format renderer with options
    pub fn with_options(format: TemplateFormat, options: FormatOptions) -> Self {
        Self { format, options }
    }

    /// Render data in the specified format
    #[instrument(skip(self, data))]
    pub fn render(&self, data: &CursedObject) -> crate::error::Result<()> {
        debug!(format = ?self.format, "Rendering template in format");

        match &self.format {
        }
    }

    /// Render as plain text
    fn render_text(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
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
    /// Render as HTML
    fn render_html(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
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
    /// Render as JSON
    fn render_json(&self, data: &CursedObject) -> crate::error::Result<()> {
        let json_value = self.cursed_to_json(data)?;
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| CursedError::TemplateError {
            })
    /// Render as YAML
    fn render_yaml(&self, data: &CursedObject) -> crate::error::Result<()> {
        let json_value = self.cursed_to_json(data)?;
        serde_yaml::to_string(&json_value)
            .map_err(|e| CursedError::TemplateError {
            })
    /// Render as XML
    fn render_xml(&self, data: &CursedObject) -> crate::error::Result<()> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<root>\n");
        xml.push_str(&self.render_xml_element("data", data, 1)?);
        xml.push_str("</root>");
        Ok(xml)
    /// Render XML element recursively
    fn render_xml_element(&self, tag: &str, data: &CursedObject, depth: usize) -> crate::error::Result<()> {
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
    /// Render as Markdown
    fn render_markdown(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
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
    /// Render as CSV
    fn render_csv(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Array(arr) => {
                if arr.is_empty() {
                    return Ok(String::new());
                // Check if all items are maps (rows)
                if arr.iter().all(|item| matches!(item, CursedObject::Map(_))) {
                    return self.render_csv_from_maps(arr);
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
    /// Render CSV from array of maps
    fn render_csv_from_maps(&self, arr: &[CursedObject]) -> crate::error::Result<()> {
        if arr.is_empty() {
            return Ok(String::new());
        // Get all unique keys from all maps
        let mut all_keys = std::collections::HashSet::new();
        for item in arr {
            if let CursedObject::Map(map) = item {
                for key in map.keys() {
                    all_keys.insert(key.clone());
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
    /// Render as email template
    fn render_email(&self, data: &CursedObject) -> crate::error::Result<()> {
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
                if !html_body.is_empty() {
                    email.push_str("--boundary\n");
                    email.push_str("Content-Type: text/html; charset=UTF-8\n\n");
                    email.push_str(&html_body);
                    email.push_str("\n\n");
                email.push_str("--boundary--\n");
                Ok(email)
            }
        }
    }

    /// Render configuration files
    fn render_config(&self, data: &CursedObject, format: &ConfigFormat) -> crate::error::Result<()> {
        match format {
        }
    }

    /// Render as TOML
    fn render_toml(&self, data: &CursedObject) -> crate::error::Result<()> {
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
        }
    }

    /// Render as INI
    fn render_ini(&self, data: &CursedObject) -> crate::error::Result<()> {
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
        }
    }

    /// Render as environment variables
    fn render_env(&self, data: &CursedObject) -> crate::error::Result<()> {
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
        }
    }

    /// Render as shell script
    fn render_shell(&self, data: &CursedObject) -> crate::error::Result<()> {
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
    /// Render as Dockerfile
    fn render_dockerfile(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut dockerfile = String::new();
                
                // Common Dockerfile instructions
                if let Some(from) = map.get("from") {
                    let from_str = self.render_text(from)?;
                    dockerfile.push_str(&format!("FROM {}\n", from_str));
                if let Some(workdir) = map.get("workdir") {
                    let workdir_str = self.render_text(workdir)?;
                    dockerfile.push_str(&format!("WORKDIR {}\n", workdir_str));
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
                if let Some(expose) = map.get("expose") {
                    let port_str = self.render_text(expose)?;
                    dockerfile.push_str(&format!("EXPOSE {}\n", port_str));
                if let Some(cmd) = map.get("cmd") {
                    let cmd_str = self.render_text(cmd)?;
                    dockerfile.push_str(&format!("CMD {}\n", cmd_str));
                Ok(dockerfile)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render as Nginx configuration
    fn render_nginx(&self, data: &CursedObject) -> crate::error::Result<()> {
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
                Ok(nginx)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render as Apache configuration
    fn render_apache(&self, data: &CursedObject) -> crate::error::Result<()> {
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
                Ok(apache)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render as Kubernetes YAML
    fn render_kubernetes(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut k8s = String::from("apiVersion: v1\n");
                
                if let Some(kind) = map.get("kind") {
                    let kind_str = self.render_text(kind)?;
                    k8s.push_str(&format!("kind: {}\n", kind_str));
                k8s.push_str("metadata:\n");
                if let Some(metadata) = map.get("metadata") {
                    let metadata_yaml = self.render_kubernetes_object(metadata, 1)?;
                    k8s.push_str(&metadata_yaml);
                if let Some(spec) = map.get("spec") {
                    k8s.push_str("spec:\n");
                    let spec_yaml = self.render_kubernetes_object(spec, 1)?;
                    k8s.push_str(&spec_yaml);
                Ok(k8s)
            }
            _ => self.render_yaml(data)
        }
    }

    /// Render Docker Compose
    fn render_docker_compose(&self, data: &CursedObject) -> crate::error::Result<()> {
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
                if let Some(volumes) = map.get("volumes") {
                    compose.push_str("\nvolumes:\n");
                    let volumes_yaml = self.render_kubernetes_object(volumes, 1)?;
                    compose.push_str(&volumes_yaml);
                if let Some(networks) = map.get("networks") {
                    compose.push_str("\nnetworks:\n");
                    let networks_yaml = self.render_kubernetes_object(networks, 1)?;
                    compose.push_str(&networks_yaml);
                Ok(compose)
            }
            _ => self.render_yaml(data)
        }
    }

    /// Render document templates
    fn render_document(&self, data: &CursedObject, format: &DocumentFormat) -> crate::error::Result<()> {
        match format {
        }
    }

    /// Render API specifications
    fn render_api(&self, data: &CursedObject, format: &ApiFormat) -> crate::error::Result<()> {
        match format {
        }
    }

    /// Render build system files
    fn render_build(&self, data: &CursedObject, format: &BuildFormat) -> crate::error::Result<()> {
        match format {
        }
    }

    /// Render README file
    fn render_readme(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut readme = String::new();
                
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    readme.push_str(&format!("# {}\n\n", title_str));
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    readme.push_str(&format!("{}\n\n", desc_str));
                if let Some(installation) = map.get("installation") {
                    readme.push_str("## Installation\n\n");
                    let install_str = self.render_text(installation)?;
                    readme.push_str(&format!("{}\n\n", install_str));
                if let Some(usage) = map.get("usage") {
                    readme.push_str("## Usage\n\n");
                    let usage_str = self.render_text(usage)?;
                    readme.push_str(&format!("{}\n\n", usage_str));
                if let Some(license) = map.get("license") {
                    readme.push_str("## License\n\n");
                    let license_str = self.render_text(license)?;
                    readme.push_str(&format!("{}\n\n", license_str));
                Ok(readme)
            }
            _ => self.render_markdown(data)
        }
    }

    /// Render OpenAPI specification
    fn render_openapi(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut openapi = String::from("openapi: 3.0.0\n");
                
                if let Some(info) = map.get("info") {
                    openapi.push_str("info:\n");
                    let info_yaml = self.render_kubernetes_object(info, 1)?;
                    openapi.push_str(&info_yaml);
                if let Some(paths) = map.get("paths") {
                    openapi.push_str("paths:\n");
                    let paths_yaml = self.render_kubernetes_object(paths, 1)?;
                    openapi.push_str(&paths_yaml);
                Ok(openapi)
            }
            _ => self.render_yaml(data)
        }
    }

    /// Render Makefile
    fn render_makefile(&self, data: &CursedObject) -> crate::error::Result<()> {
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
                Ok(makefile)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    // Utility methods for the new formats
    fn render_kubernetes_object(&self, obj: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
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
    /// Render license file
    fn render_license(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut license = String::new();
                
                // Get license type or default to MIT
                let license_type = map.get("type")
                    .map(|t| self.render_text(t))
                    .unwrap_or_else(|| Ok("MIT".to_string()))?;
                
                // Get copyright year or default to current year
                let year = map.get("year")
                    .map(|y| self.render_text(y))
                    .unwrap_or_else(|| Ok("2024".to_string()))?;
                
                // Get copyright holder
                let holder = map.get("holder")
                    .or_else(|| map.get("author"))
                    .or_else(|| map.get("name"))
                    .map(|h| self.render_text(h))
                    .unwrap_or_else(|| Ok("Copyright Holder".to_string()))?;
                
                // Get project name if available
                let project_name = map.get("project")
                    .or_else(|| map.get("project_name"))
                    .map(|p| self.render_text(p))
                    .unwrap_or_else(|| Ok("".to_string()))?;
                
                // Render based on license type
                match license_type.to_lowercase().as_str() {
                    "mit" => {
                        if !project_name.is_empty() {
                            license.push_str(&format!("MIT License\n\n"));
                            license.push_str(&format!("Copyright (c) {} {}\n\n", year, holder));
                        } else {
                            license.push_str(&format!("MIT License\n\n"));
                            license.push_str(&format!("Copyright (c) {} {}\n\n", year, holder));
                        license.push_str("Permission is hereby granted, free of charge, to any person obtaining a copy\n");
                        license.push_str("of this software and associated documentation files (the \"Software\"), to deal\n");
                        license.push_str("in the Software without restriction, including without limitation the rights\n");
                        license.push_str("to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n");
                        license.push_str("copies of the Software, and to permit persons to whom the Software is\n");
                        license.push_str("furnished to do so, subject to the following conditions:\n\n");
                        
                        license.push_str("The above copyright notice and this permission notice shall be included in all\n");
                        license.push_str("copies or substantial portions of the Software.\n\n");
                        
                        license.push_str("THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n");
                        license.push_str("IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n");
                        license.push_str("FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n");
                        license.push_str("AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n");
                        license.push_str("LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n");
                        license.push_str("OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n");
                        license.push_str("SOFTWARE.\n");
                    }
                    "apache" | "apache-2.0" => {
                        license.push_str(&format!("Apache License\nVersion 2.0, January 2004\n"));
                        license.push_str("http://www.apache.org/licenses/\n\n");
                        license.push_str("TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION\n\n");
                        
                        license.push_str("1. Definitions.\n\n");
                        license.push_str("\"License\" shall mean the terms and conditions for use, reproduction,\n");
                        license.push_str("and distribution as defined by Sections 1 through 9 of this document.\n\n");
                        
                        // Add more Apache license content...
                        license.push_str(&format!("Copyright {} {}\n\n", year, holder));
                        license.push_str("Licensed under the Apache License, Version 2.0 (the \"License\");\n");
                        license.push_str("you may not use this file except in compliance with the License.\n");
                        license.push_str("You may obtain a copy of the License at\n\n");
                        license.push_str("    http://www.apache.org/licenses/LICENSE-2.0\n\n");
                        license.push_str("Unless required by applicable law or agreed to in writing, software\n");
                        license.push_str("distributed under the License is distributed on an \"AS IS\" BASIS,\n");
                        license.push_str("WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n");
                        license.push_str("See the License for the specific language governing permissions and\n");
                        license.push_str("limitations under the License.\n");
                    }
                    "gpl" | "gpl-3.0" => {
                        license.push_str("GNU GENERAL PUBLIC LICENSE\nVersion 3, 29 June 2007\n\n");
                        license.push_str(&format!("Copyright (C) {} {}\n\n", year, holder));
                        license.push_str("This program is free software: you can redistribute it and/or modify\n");
                        license.push_str("it under the terms of the GNU General Public License as published by\n");
                        license.push_str("the Free Software Foundation, either version 3 of the License, or\n");
                        license.push_str("(at your option) any later version.\n\n");
                        license.push_str("This program is distributed in the hope that it will be useful,\n");
                        license.push_str("but WITHOUT ANY WARRANTY; without even the implied warranty of\n");
                        license.push_str("MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n");
                        license.push_str("GNU General Public License for more details.\n\n");
                        license.push_str("You should have received a copy of the GNU General Public License\n");
                        license.push_str("along with this program.  If not, see <https://www.gnu.org/licenses/>.\n");
                    }
                    "bsd" | "bsd-3-clause" => {
                        license.push_str("BSD 3-Clause License\n\n");
                        license.push_str(&format!("Copyright (c) {} {}\n\n", year, holder));
                        license.push_str("Redistribution and use in source and binary forms, with or without\n");
                        license.push_str("modification, are permitted provided that the following conditions are met:\n\n");
                        license.push_str("1. Redistributions of source code must retain the above copyright notice, this\n");
                        license.push_str("   list of conditions and the following disclaimer.\n\n");
                        license.push_str("2. Redistributions in binary form must reproduce the above copyright notice,\n");
                        license.push_str("   this list of conditions and the following disclaimer in the documentation\n");
                        license.push_str("   and/or other materials provided with the distribution.\n\n");
                        license.push_str("3. Neither the name of the copyright holder nor the names of its\n");
                        license.push_str("   contributors may be used to endorse or promote products derived from\n");
                        license.push_str("   this software without specific prior written permission.\n\n");
                        license.push_str("THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\"\n");
                        license.push_str("AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE\n");
                        license.push_str("IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE\n");
                        license.push_str("DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE\n");
                        license.push_str("FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL\n");
                        license.push_str("DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR\n");
                        license.push_str("SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER\n");
                        license.push_str("CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,\n");
                        license.push_str("OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE\n");
                        license.push_str("OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.\n");
                    }
                    "unlicense" => {
                        license.push_str("This is free and unencumbered software released into the public domain.\n\n");
                        license.push_str("Anyone is free to copy, modify, publish, use, compile, sell, or\n");
                        license.push_str("distribute this software, either in source code form or as a compiled\n");
                        license.push_str("binary, for any purpose, commercial or non-commercial, and by any\n");
                        license.push_str("means.\n\n");
                        license.push_str("In jurisdictions that recognize copyright laws, the author or authors\n");
                        license.push_str("of this software dedicate any and all copyright interest in the\n");
                        license.push_str("software to the public domain. We make this dedication for the benefit\n");
                        license.push_str("of the public at large and to the detriment of our heirs and\n");
                        license.push_str("successors. We intend this dedication to be an overt act of\n");
                        license.push_str("relinquishment in perpetuity of all present and future rights to this\n");
                        license.push_str("software under copyright law.\n\n");
                        license.push_str("THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND,\n");
                        license.push_str("EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF\n");
                        license.push_str("MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.\n");
                        license.push_str("IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR\n");
                        license.push_str("OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,\n");
                        license.push_str("ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR\n");
                        license.push_str("OTHER DEALINGS IN THE SOFTWARE.\n\n");
                        license.push_str("For more information, please refer to <https://unlicense.org>\n");
                    }
                    _ => {
                        // Custom license or fallback
                        if let Some(content) = map.get("content") {
                            license.push_str(&self.render_text(content)?);
                        } else {
                            license.push_str(&format!("Copyright (c) {} {}\n\n", year, holder));
                            license.push_str("All rights reserved.\n\n");
                            license.push_str("This software and associated documentation files (the \"Software\") are\n");
                            license.push_str("proprietary and confidential. Unauthorized copying, distribution, or use\n");
                            license.push_str("of this Software is strictly prohibited.\n");
                        }
                    }
                Ok(license)
            }
            _ => {
                // Simple string license content
                self.render_text(data)
            }
        }
    }
    fn render_changelog(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut changelog = String::new();
                
                // Title and description
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    changelog.push_str(&format!("# {}\n\n", title_str));
                } else {
                    changelog.push_str("# Changelog\n\n");
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    changelog.push_str(&format!("{}\n\n", desc_str));
                } else {
                    changelog.push_str("All notable changes to this project will be documented in this file.\n\n");
                    changelog.push_str("The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\n");
                    changelog.push_str("and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n");
                // Handle versions
                if let Some(versions) = map.get("versions") {
                    match versions {
                        CursedObject::Array(version_array) => {
                            for version_obj in version_array {
                                changelog.push_str(&self.render_changelog_version(version_obj)?);
                            }
                        }
                        _ => {
                            warn!("Versions should be an array");
                            return Err(CursedError::TemplateError {
                            });
                        }
                    }
                } else {
                    // If no versions provided, treat the entire map as a single version
                    changelog.push_str(&self.render_changelog_version(data)?);
                Ok(changelog)
            }
            _ => {
                // Fallback to simple text rendering
                self.render_markdown(data)
            }
        }
    /// Render a single changelog version entry
    fn render_changelog_version(&self, version_obj: &CursedObject) -> crate::error::Result<()> {
        match version_obj {
            CursedObject::Map(version_map) => {
                let mut version_text = String::new();

                // Version header
                let version = version_map.get("version")
                    .map(|v| self.render_text(v))
                    .unwrap_or_else(|| Ok("Unreleased".to_string()))?;

                let date = version_map.get("date")
                    .map(|d| self.render_text(d))
                    .unwrap_or_else(|| Ok("TBD".to_string()))?;

                version_text.push_str(&format!("## [{}] - {}\n\n", version, date));

                // Render change sections in standard order
                let sections = vec![
                ];

                for (section_key, section_header) in sections {
                    if let Some(section_data) = version_map.get(section_key) {
                        version_text.push_str(&format!("{}\n", section_header));
                        version_text.push_str(&self.render_changelog_section(section_data)?);
                        version_text.push('\n');
                    }
                }

                // Handle any other sections not in the standard list
                for (key, value) in version_map {
                    if !matches!(key.as_str(), "version" | "date" | "added" | "changed" | "deprecated" | "removed" | "fixed" | "security") {
                        let section_header = format!("### {}", self.capitalize_first(key));
                        version_text.push_str(&format!("{}\n", section_header));
                        version_text.push_str(&self.render_changelog_section(value)?);
                        version_text.push('\n');
                    }
                }

                Ok(version_text)
            }
            _ => {
                // Handle simple string or other formats
                let text = self.render_text(version_obj)?;
                Ok(format!("## {}\n\n", text))
            }
        }
    /// Render a changelog section (list of changes)
    fn render_changelog_section(&self, section_data: &CursedObject) -> crate::error::Result<()> {
        match section_data {
            CursedObject::Array(changes) => {
                let mut section_text = String::new();
                for change in changes {
                    let change_text = self.render_text(change)?;
                    if !change_text.is_empty() {
                        section_text.push_str(&format!("- {}\n", change_text));
                    }
                }
                Ok(section_text)
            }
            CursedObject::String(text) => {
                if text.is_empty() {
                    Ok(String::new())
                } else {
                    Ok(format!("- {}\n", text))
                }
            }
            _ => {
                let text = self.render_text(section_data)?;
                if text.is_empty() {
                    Ok(String::new())
                } else {
                    Ok(format!("- {}\n", text))
                }
            }
        }
    }

    /// Capitalize the first letter of a string
    fn capitalize_first(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
        }
    }
    /// Render code documentation
    fn render_code_doc(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut doc = String::new();
                
                // Document title
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    doc.push_str(&format!("# {}\n\n", title_str));
                // Language detection for syntax highlighting
                let language = map.get("language")
                    .map(|l| self.render_text(l))
                    .unwrap_or_else(|| Ok("text".to_string()))?;
                
                // Table of contents
                if let Some(modules) = map.get("modules") {
                    if let CursedObject::Array(module_array) = modules {
                        if !module_array.is_empty() {
                            doc.push_str("## Table of Contents\n\n");
                            for module in module_array {
                                if let CursedObject::Map(module_map) = module {
                                    if let Some(module_name) = module_map.get("name") {
                                        let name_str = self.render_text(module_name)?;
                                        let anchor = self.generate_anchor(&name_str);
                                        doc.push_str(&format!("- [{}](#{})\n", name_str, anchor));
                                        
                                        // Add function links
                                        if let Some(functions) = module_map.get("functions") {
                                            if let CursedObject::Array(func_array) = functions {
                                                for function in func_array {
                                                    if let CursedObject::Map(func_map) = function {
                                                        if let Some(func_name) = func_map.get("name") {
                                                            let func_name_str = self.render_text(func_name)?;
                                                            let func_anchor = self.generate_anchor(&format!("{}.{}", name_str, func_name_str));
                                                            doc.push_str(&format!("  - [{}](#{})\n", func_name_str, func_anchor));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            doc.push_str("\n");
                        }
                    }
                // Overview section
                if let Some(overview) = map.get("overview") {
                    doc.push_str("## Overview\n\n");
                    let overview_str = self.render_text(overview)?;
                    doc.push_str(&format!("{}\n\n", overview_str));
                // Installation section
                if let Some(installation) = map.get("installation") {
                    doc.push_str("## Installation\n\n");
                    let install_str = self.render_text(installation)?;
                        self.get_install_language(&language), install_str));
                // Quick start section
                if let Some(quick_start) = map.get("quick_start") {
                    doc.push_str("## Quick Start\n\n");
                    let quick_start_str = self.render_text(quick_start)?;
                    doc.push_str(&format!("```{}\n{}\n```\n\n", language, quick_start_str));
                // Render modules
                if let Some(modules) = map.get("modules") {
                    doc.push_str(&self.render_modules(modules, &language)?);
                // Global functions (functions not in modules)
                if let Some(functions) = map.get("functions") {
                    doc.push_str("## Functions\n\n");
                    doc.push_str(&self.render_functions(functions, &language, None)?);
                // Global classes/types
                if let Some(classes) = map.get("classes") {
                    doc.push_str("## Classes\n\n");
                    doc.push_str(&self.render_classes(classes, &language)?);
                // Global constants
                if let Some(constants) = map.get("constants") {
                    doc.push_str("## Constants\n\n");
                    doc.push_str(&self.render_constants(constants, &language)?);
                // Examples section
                if let Some(examples) = map.get("examples") {
                    doc.push_str("## Examples\n\n");
                    doc.push_str(&self.render_examples(examples, &language)?);
                // License section
                if let Some(license) = map.get("license") {
                    doc.push_str("## License\n\n");
                    let license_str = self.render_text(license)?;
                    doc.push_str(&format!("{}\n\n", license_str));
                Ok(doc)
            }
            _ => self.render_markdown(data)
        }
    }
    
    /// Render modules section
    fn render_modules(&self, modules: &CursedObject, language: &str) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(module_array) = modules {
            for module in module_array {
                if let CursedObject::Map(module_map) = module {
                    let module_name = module_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("Unnamed Module".to_string()))?;
                    
                    let anchor = self.generate_anchor(&module_name);
                    doc.push_str(&format!("## {} {{#{}}}\n\n", module_name, anchor));
                    
                    // Module description
                    if let Some(description) = module_map.get("description") {
                        let desc_str = self.render_text(description)?;
                        doc.push_str(&format!("{}\n\n", desc_str));
                    // Module import/usage
                    if let Some(import) = module_map.get("import") {
                        let import_str = self.render_text(import)?;
                        doc.push_str(&format!("```{}\n{}\n```\n\n", language, import_str));
                    // Module functions
                    if let Some(functions) = module_map.get("functions") {
                        doc.push_str("### Functions\n\n");
                        doc.push_str(&self.render_functions(functions, language, Some(&module_name))?);
                    // Module classes
                    if let Some(classes) = module_map.get("classes") {
                        doc.push_str("### Classes\n\n");
                        doc.push_str(&self.render_classes(classes, language)?);
                    // Module constants
                    if let Some(constants) = module_map.get("constants") {
                        doc.push_str("### Constants\n\n");
                        doc.push_str(&self.render_constants(constants, language)?);
                    // Module types/interfaces
                    if let Some(types) = module_map.get("types") {
                        doc.push_str("### Types\n\n");
                        doc.push_str(&self.render_types(types, language)?);
                    }
                }
            }
        }
        
        Ok(doc)
    /// Render functions section
    fn render_functions(&self, functions: &CursedObject, language: &str, module_name: Option<&str>) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(func_array) = functions {
            for function in func_array {
                if let CursedObject::Map(func_map) = function {
                    let func_name = func_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("unnamed_function".to_string()))?;
                    
                    let full_name = if let Some(module) = module_name {
                        format!("{}.{}", module, func_name)
                    } else {
                        func_name.clone()
                    
                    let anchor = self.generate_anchor(&full_name);
                    doc.push_str(&format!("#### `{}` {{#{}}}\n\n", func_name, anchor));
                    
                    // Function description
                    if let Some(description) = func_map.get("description") {
                        let desc_str = self.render_text(description)?;
                        doc.push_str(&format!("{}\n\n", desc_str));
                    // Function signature
                    let signature = self.generate_function_signature(func_map, language)?;
                    doc.push_str(&format!("```{}\n{}\n```\n\n", language, signature));
                    
                    // Parameters
                    if let Some(parameters) = func_map.get("parameters") {
                        doc.push_str("**Parameters:**\n\n");
                        doc.push_str(&self.render_parameters(parameters)?);
                    // Returns
                    if let Some(returns) = func_map.get("returns") {
                        doc.push_str("**Returns:**\n\n");
                        doc.push_str(&self.render_return_type(returns)?);
                    // Throws/Errors
                    if let Some(throws) = func_map.get("throws") {
                        doc.push_str("**Throws:**\n\n");
                        doc.push_str(&self.render_errors(throws)?);
                    // Examples
                    if let Some(examples) = func_map.get("examples") {
                        doc.push_str("**Examples:**\n\n");
                        doc.push_str(&self.render_function_examples(examples, language)?);
                    // Notes/See Also
                    if let Some(notes) = func_map.get("notes") {
                        doc.push_str("**Notes:**\n\n");
                        let notes_str = self.render_text(notes)?;
                        doc.push_str(&format!("{}\n\n", notes_str));
                    doc.push_str("---\n\n");
                }
            }
        Ok(doc)
    /// Render classes section
    fn render_classes(&self, classes: &CursedObject, language: &str) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(class_array) = classes {
            for class in class_array {
                if let CursedObject::Map(class_map) = class {
                    let class_name = class_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("UnnamedClass".to_string()))?;
                    
                    let anchor = self.generate_anchor(&class_name);
                    doc.push_str(&format!("#### `{}` {{#{}}}\n\n", class_name, anchor));
                    
                    // Class description
                    if let Some(description) = class_map.get("description") {
                        let desc_str = self.render_text(description)?;
                        doc.push_str(&format!("{}\n\n", desc_str));
                    // Class definition
                    if let Some(definition) = class_map.get("definition") {
                        let def_str = self.render_text(definition)?;
                        doc.push_str(&format!("```{}\n{}\n```\n\n", language, def_str));
                    // Properties
                    if let Some(properties) = class_map.get("properties") {
                        doc.push_str("**Properties:**\n\n");
                        doc.push_str(&self.render_properties(properties)?);
                    // Methods
                    if let Some(methods) = class_map.get("methods") {
                        doc.push_str("**Methods:**\n\n");
                        doc.push_str(&self.render_functions(methods, language, Some(&class_name))?);
                    doc.push_str("---\n\n");
                }
            }
        Ok(doc)
    /// Render constants section
    fn render_constants(&self, constants: &CursedObject, language: &str) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(const_array) = constants {
            doc.push_str("| Name | Type | Value | Description |\n");
            doc.push_str("|------|------|-------|-------------|\n");
            
            for constant in const_array {
                if let CursedObject::Map(const_map) = constant {
                    let name = const_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("UNNAMED".to_string()))?;
                    
                    let const_type = const_map.get("type")
                        .map(|t| self.render_text(t))
                        .unwrap_or_else(|| Ok("unknown".to_string()))?;
                    
                    let value = const_map.get("value")
                        .map(|v| self.render_text(v))
                        .unwrap_or_else(|| Ok("".to_string()))?;
                    
                    let description = const_map.get("description")
                        .map(|d| self.render_text(d))
                        .unwrap_or_else(|| Ok("".to_string()))?;
                    
                        name, const_type, value, description));
                }
            }
            doc.push_str("\n");
        Ok(doc)
    /// Render types/interfaces section
    fn render_types(&self, types: &CursedObject, language: &str) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(type_array) = types {
            for type_obj in type_array {
                if let CursedObject::Map(type_map) = type_obj {
                    let type_name = type_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("UnnamedType".to_string()))?;
                    
                    let anchor = self.generate_anchor(&type_name);
                    doc.push_str(&format!("#### `{}` {{#{}}}\n\n", type_name, anchor));
                    
                    // Type description
                    if let Some(description) = type_map.get("description") {
                        let desc_str = self.render_text(description)?;
                        doc.push_str(&format!("{}\n\n", desc_str));
                    // Type definition
                    if let Some(definition) = type_map.get("definition") {
                        let def_str = self.render_text(definition)?;
                        doc.push_str(&format!("```{}\n{}\n```\n\n", language, def_str));
                    // Type fields/properties
                    if let Some(fields) = type_map.get("fields") {
                        doc.push_str("**Fields:**\n\n");
                        doc.push_str(&self.render_properties(fields)?);
                    doc.push_str("---\n\n");
                }
            }
        Ok(doc)
    /// Render examples section
    fn render_examples(&self, examples: &CursedObject, language: &str) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        match examples {
            CursedObject::Array(example_array) => {
                for (i, example) in example_array.iter().enumerate() {
                    match example {
                        CursedObject::Map(example_map) => {
                            if let Some(title) = example_map.get("title") {
                                let title_str = self.render_text(title)?;
                                doc.push_str(&format!("### {}\n\n", title_str));
                            } else {
                                doc.push_str(&format!("### Example {}\n\n", i + 1));
                            if let Some(description) = example_map.get("description") {
                                let desc_str = self.render_text(description)?;
                                doc.push_str(&format!("{}\n\n", desc_str));
                            if let Some(code) = example_map.get("code") {
                                let code_str = self.render_text(code)?;
                                doc.push_str(&format!("```{}\n{}\n```\n\n", language, code_str));
                            if let Some(output) = example_map.get("output") {
                                let output_str = self.render_text(output)?;
                                doc.push_str("**Output:**\n\n");
                                doc.push_str(&format!("```\n{}\n```\n\n", output_str));
                            }
                        }
                        _ => {
                            let example_str = self.render_text(example)?;
                            doc.push_str(&format!("### Example {}\n\n", i + 1));
                            doc.push_str(&format!("```{}\n{}\n```\n\n", language, example_str));
                        }
                    }
                }
            }
            _ => {
                let example_str = self.render_text(examples)?;
                doc.push_str(&format!("```{}\n{}\n```\n\n", language, example_str));
            }
        }
        
        Ok(doc)
    /// Render function parameters
    fn render_parameters(&self, parameters: &CursedObject) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(param_array) = parameters {
            for param in param_array {
                if let CursedObject::Map(param_map) = param {
                    let name = param_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("param".to_string()))?;
                    
                    let param_type = param_map.get("type")
                        .map(|t| self.render_text(t))
                        .unwrap_or_else(|| Ok("unknown".to_string()))?;
                    
                    let description = param_map.get("description")
                        .map(|d| self.render_text(d))
                        .unwrap_or_else(|| Ok("".to_string()))?;
                    
                    let optional = param_map.get("optional")
                        .and_then(|o| match o {
                        })
                        .unwrap_or(false);
                    
                    let default_value = param_map.get("default")
                        .map(|d| self.render_text(d));
                    
                    doc.push_str(&format!("- **{}**", name));
                    
                    if optional {
                        doc.push_str(" *(optional)*");
                    doc.push_str(&format!(" (`{}`)", param_type));
                    
                    if let Some(Ok(default)) = default_value {
                        doc.push_str(&format!(" - Default: `{}`", default));
                    if !description.is_empty() {
                        doc.push_str(&format!(" - {}", description));
                    doc.push_str("\n");
                }
            }
            doc.push_str("\n");
        Ok(doc)
    /// Render return type information
    fn render_return_type(&self, returns: &CursedObject) -> crate::error::Result<()> {
        match returns {
            CursedObject::Map(return_map) => {
                let return_type = return_map.get("type")
                    .map(|t| self.render_text(t))
                    .unwrap_or_else(|| Ok("void".to_string()))?;
                
                let description = return_map.get("description")
                    .map(|d| self.render_text(d))
                    .unwrap_or_else(|| Ok("".to_string()))?;
                
                let mut doc = format!("`{}` - {}\n\n", return_type, description);
                
                // Handle complex return types
                if let Some(properties) = return_map.get("properties") {
                    doc.push_str("**Properties:**\n\n");
                    doc.push_str(&self.render_properties(properties)?);
                Ok(doc)
            }
            _ => {
                let return_str = self.render_text(returns)?;
                Ok(format!("`{}` - Return value\n\n", return_str))
            }
        }
    /// Render error/exception information
    fn render_errors(&self, throws: &CursedObject) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(error_array) = throws {
            for error in error_array {
                if let CursedObject::Map(error_map) = error {
                    let error_type = error_map.get("type")
                        .map(|t| self.render_text(t))
                        .unwrap_or_else(|| Ok("CursedError".to_string()))?;
                    
                    let description = error_map.get("description")
                        .map(|d| self.render_text(d))
                        .unwrap_or_else(|| Ok("".to_string()))?;
                    
                    doc.push_str(&format!("- `{}` - {}\n", error_type, description));
                } else {
                    let error_str = self.render_text(error)?;
                    doc.push_str(&format!("- `{}`\n", error_str));
                }
            }
            doc.push_str("\n");
        Ok(doc)
    /// Render properties/fields table
    fn render_properties(&self, properties: &CursedObject) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(prop_array) = properties {
            doc.push_str("| Name | Type | Description |\n");
            doc.push_str("|------|------|-------------|\n");
            
            for prop in prop_array {
                if let CursedObject::Map(prop_map) = prop {
                    let name = prop_map.get("name")
                        .map(|n| self.render_text(n))
                        .unwrap_or_else(|| Ok("property".to_string()))?;
                    
                    let prop_type = prop_map.get("type")
                        .map(|t| self.render_text(t))
                        .unwrap_or_else(|| Ok("unknown".to_string()))?;
                    
                    let description = prop_map.get("description")
                        .map(|d| self.render_text(d))
                        .unwrap_or_else(|| Ok("".to_string()))?;
                    
                    doc.push_str(&format!("| `{}` | `{}` | {} |\n", name, prop_type, description));
                }
            }
            doc.push_str("\n");
        Ok(doc)
    /// Render function examples with proper formatting
    fn render_function_examples(&self, examples: &CursedObject, language: &str) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        if let CursedObject::Array(example_array) = examples {
            for (i, example) in example_array.iter().enumerate() {
                match example {
                    CursedObject::Map(example_map) => {
                        if let Some(description) = example_map.get("description") {
                            let desc_str = self.render_text(description)?;
                            doc.push_str(&format!("{}\n\n", desc_str));
                        if let Some(code) = example_map.get("code") {
                            let code_str = self.render_text(code)?;
                            doc.push_str(&format!("```{}\n{}\n```\n\n", language, code_str));
                        if let Some(output) = example_map.get("output") {
                            let output_str = self.render_text(output)?;
                            doc.push_str(&format!("Output:\n```\n{}\n```\n\n", output_str));
                        }
                    }
                    _ => {
                        let example_str = self.render_text(example)?;
                        doc.push_str(&format!("```{}\n{}\n```\n\n", language, example_str));
                    }
                }
                
                if i < example_array.len() - 1 {
                    doc.push_str("---\n\n");
                }
            }
        } else {
            let example_str = self.render_text(examples)?;
            doc.push_str(&format!("```{}\n{}\n```\n\n", language, example_str));
        Ok(doc)
    /// Generate function signature based on language
    fn generate_function_signature(&self, func_map: &std::collections::HashMap<String, CursedObject>, language: &str) -> crate::error::Result<()> {
        let func_name = func_map.get("name")
            .map(|n| self.render_text(n))
            .unwrap_or_else(|| Ok("function".to_string()))?;
        
        let mut signature = String::new();
        
        match language {
            "rust" => {
                signature.push_str("fn ");
                signature.push_str(&func_name);
                signature.push('(');
                
                if let Some(parameters) = func_map.get("parameters") {
                    signature.push_str(&self.format_rust_parameters(parameters)?);
                signature.push(')');
                
                if let Some(returns) = func_map.get("returns") {
                    if let CursedObject::Map(return_map) = returns {
                        if let Some(return_type) = return_map.get("type") {
                            let type_str = self.render_text(return_type)?;
                            if type_str != "void" && type_str != "()" {
                                signature.push_str(" -> ");
                                signature.push_str(&type_str);
                            }
                        }
                    }
                }
            }
            "python" => {
                signature.push_str("def ");
                signature.push_str(&func_name);
                signature.push('(');
                
                if let Some(parameters) = func_map.get("parameters") {
                    signature.push_str(&self.format_python_parameters(parameters)?);
                signature.push(')');
                
                if let Some(returns) = func_map.get("returns") {
                    if let CursedObject::Map(return_map) = returns {
                        if let Some(return_type) = return_map.get("type") {
                            let type_str = self.render_text(return_type)?;
                            if type_str != "None" {
                                signature.push_str(" -> ");
                                signature.push_str(&type_str);
                            }
                        }
                    }
                }
                
                signature.push(':');
            }
            "javascript" | "typescript" => {
                signature.push_str("function ");
                signature.push_str(&func_name);
                signature.push('(');
                
                if let Some(parameters) = func_map.get("parameters") {
                    signature.push_str(&self.format_js_parameters(parameters, language == "typescript")?);
                signature.push(')');
                
                if language == "typescript" {
                    if let Some(returns) = func_map.get("returns") {
                        if let CursedObject::Map(return_map) = returns {
                            if let Some(return_type) = return_map.get("type") {
                                let type_str = self.render_text(return_type)?;
                                if type_str != "void" {
                                    signature.push_str(": ");
                                    signature.push_str(&type_str);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                // Generic signature
                signature.push_str(&func_name);
                signature.push('(');
                
                if let Some(parameters) = func_map.get("parameters") {
                    signature.push_str(&self.format_generic_parameters(parameters)?);
                signature.push(')');
            }
        }
        
        Ok(signature)
    /// Format parameters for Rust
    fn format_rust_parameters(&self, parameters: &CursedObject) -> crate::error::Result<()> {
        if let CursedObject::Array(param_array) = parameters {
            let params: crate::error::Result<()> = param_array.iter()
                .map(|param| {
                    if let CursedObject::Map(param_map) = param {
                        let name = param_map.get("name")
                            .map(|n| self.render_text(n))
                            .unwrap_or_else(|| Ok("param".to_string()))?;
                        let param_type = param_map.get("type")
                            .map(|t| self.render_text(t))
                            .unwrap_or_else(|| Ok("()".to_string()))?;
                        Ok(format!("{}: {}", name, param_type))
                    } else {
                        Ok("param".to_string())
                    }
                })
                .collect();
            Ok(params?.join(", "))
        } else {
            Ok(String::new())
        }
    }
    
    /// Format parameters for Python
    fn format_python_parameters(&self, parameters: &CursedObject) -> crate::error::Result<()> {
        if let CursedObject::Array(param_array) = parameters {
            let params: crate::error::Result<()> = param_array.iter()
                .map(|param| {
                    if let CursedObject::Map(param_map) = param {
                        let name = param_map.get("name")
                            .map(|n| self.render_text(n))
                            .unwrap_or_else(|| Ok("param".to_string()))?;
                        
                        let mut param_str = name;
                        
                        if let Some(param_type) = param_map.get("type") {
                            let type_str = self.render_text(param_type)?;
                            param_str = format!("{}: {}", param_str, type_str);
                        if let Some(default) = param_map.get("default") {
                            let default_str = self.render_text(default)?;
                            param_str = format!("{} = {}", param_str, default_str);
                        Ok(param_str)
                    } else {
                        Ok("param".to_string())
                    }
                })
                .collect();
            Ok(params?.join(", "))
        } else {
            Ok(String::new())
        }
    }
    
    /// Format parameters for JavaScript/TypeScript
    fn format_js_parameters(&self, parameters: &CursedObject, include_types: bool) -> crate::error::Result<()> {
        if let CursedObject::Array(param_array) = parameters {
            let params: crate::error::Result<()> = param_array.iter()
                .map(|param| {
                    if let CursedObject::Map(param_map) = param {
                        let name = param_map.get("name")
                            .map(|n| self.render_text(n))
                            .unwrap_or_else(|| Ok("param".to_string()))?;
                        
                        let mut param_str = name;
                        
                        if include_types {
                            if let Some(param_type) = param_map.get("type") {
                                let type_str = self.render_text(param_type)?;
                                param_str = format!("{}: {}", param_str, type_str);
                            }
                        }
                        
                        if let Some(default) = param_map.get("default") {
                            let default_str = self.render_text(default)?;
                            param_str = format!("{} = {}", param_str, default_str);
                        Ok(param_str)
                    } else {
                        Ok("param".to_string())
                    }
                })
                .collect();
            Ok(params?.join(", "))
        } else {
            Ok(String::new())
        }
    }
    
    /// Format parameters generically
    fn format_generic_parameters(&self, parameters: &CursedObject) -> crate::error::Result<()> {
        if let CursedObject::Array(param_array) = parameters {
            let params: crate::error::Result<()> = param_array.iter()
                .map(|param| {
                    if let CursedObject::Map(param_map) = param {
                        let name = param_map.get("name")
                            .map(|n| self.render_text(n))
                            .unwrap_or_else(|| Ok("param".to_string()))?;
                        let param_type = param_map.get("type")
                            .map(|t| self.render_text(t))
                            .unwrap_or_else(|| Ok("".to_string()))?;
                        
                        if param_type.is_empty() {
                            Ok(name)
                        } else {
                            Ok(format!("{}: {}", name, param_type))
                        }
                    } else {
                        let name = self.render_text(param)?;
                        Ok(name)
                    }
                })
                .collect();
            Ok(params?.join(", "))
        } else {
            Ok(String::new())
        }
    }
    
    /// Generate anchor for table of contents
    fn generate_anchor(&self, text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    /// Get appropriate language for installation commands
    fn get_install_language(&self, language: &str) -> &str {
        match language {
        }
    }
    /// Render API documentation
    fn render_api_doc(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut doc = String::new();
                
                // API title
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    doc.push_str(&format!("# {} API Documentation\n\n", title_str));
                } else {
                    doc.push_str("# API Documentation\n\n");
                // API description/overview
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    doc.push_str(&format!("{}\n\n", desc_str));
                // Base URL
                if let Some(base_url) = map.get("base_url") {
                    let url_str = self.render_text(base_url)?;
                    doc.push_str(&format!("**Base URL:** `{}`\n\n", url_str));
                // API version
                if let Some(version) = map.get("version") {
                    let version_str = self.render_text(version)?;
                    doc.push_str(&format!("**Version:** {}\n\n", version_str));
                // Authentication
                if let Some(auth) = map.get("authentication") {
                    doc.push_str("## Authentication\n\n");
                    match auth {
                        CursedObject::Map(auth_map) => {
                            if let Some(auth_type) = auth_map.get("type") {
                                let type_str = self.render_text(auth_type)?;
                                doc.push_str(&format!("**Type:** {}\n\n", type_str));
                            }
                            if let Some(description) = auth_map.get("description") {
                                let desc_str = self.render_text(description)?;
                                doc.push_str(&format!("{}\n\n", desc_str));
                            }
                            if let Some(example) = auth_map.get("example") {
                                let example_str = self.render_text(example)?;
                                doc.push_str(&format!("**Example:**\n```\n{}\n```\n\n", example_str));
                            }
                        }
                        _ => {
                            let auth_str = self.render_text(auth)?;
                            doc.push_str(&format!("{}\n\n", auth_str));
                        }
                    }
                // Rate limiting
                if let Some(rate_limit) = map.get("rate_limit") {
                    doc.push_str("## Rate Limiting\n\n");
                    let rate_str = self.render_text(rate_limit)?;
                    doc.push_str(&format!("{}\n\n", rate_str));
                // Endpoints
                if let Some(endpoints) = map.get("endpoints") {
                    doc.push_str("## Endpoints\n\n");
                    doc.push_str(&self.render_api_endpoints(endpoints)?);
                // CursedError codes
                if let Some(errors) = map.get("errors") {
                    doc.push_str("## CursedError Codes\n\n");
                    doc.push_str(&self.render_api_errors(errors)?);
                // Response formats
                if let Some(formats) = map.get("response_formats") {
                    doc.push_str("## Response Formats\n\n");
                    let formats_str = self.render_text(formats)?;
                    doc.push_str(&format!("{}\n\n", formats_str));
                // SDKs and libraries
                if let Some(sdks) = map.get("sdks") {
                    doc.push_str("## SDKs and Libraries\n\n");
                    if let CursedObject::Array(sdk_array) = sdks {
                        for sdk in sdk_array {
                            if let CursedObject::Map(sdk_map) = sdk {
                                if let (Some(name), Some(url)) = (sdk_map.get("name"), sdk_map.get("url")) {
                                    let name_str = self.render_text(name)?;
                                    let url_str = self.render_text(url)?;
                                    doc.push_str(&format!("- [{}]({})\n", name_str, url_str));
                                }
                            }
                        }
                        doc.push_str("\n");
                    }
                }
                
                // Examples
                if let Some(examples) = map.get("examples") {
                    doc.push_str("## Examples\n\n");
                    doc.push_str(&self.render_api_examples(examples)?);
                Ok(doc)
            }
            _ => self.render_markdown(data)
        }
    }
    
    /// Render project documentation
    fn render_project_doc(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut doc = String::new();
                
                // Project title
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    doc.push_str(&format!("# {}\n\n", title_str));
                // Project description
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    doc.push_str(&format!("{}\n\n", desc_str));
                // Table of contents
                doc.push_str("## Table of Contents\n\n");
                let sections = vec![
                ];
                
                for (key, title) in &sections {
                    if map.contains_key(*key) {
                        let anchor = key.replace('_', "-");
                        doc.push_str(&format!("- [{}](#{})\n", title, anchor));
                    }
                }
                doc.push_str("\n");
                
                // Render each section
                for (key, title) in &sections {
                    if let Some(content) = map.get(*key) {
                        doc.push_str(&format!("## {}\n\n", title));
                        match content {
                            CursedObject::Map(section_map) => {
                                // Complex section with subsections
                                for (sub_key, sub_content) in section_map {
                                    let sub_title = self.capitalize_first(sub_key);
                                    doc.push_str(&format!("### {}\n\n", sub_title));
                                    let sub_text = self.render_text(sub_content)?;
                                    doc.push_str(&format!("{}\n\n", sub_text));
                                }
                            }
                            _ => {
                                // Simple section content
                                let content_str = self.render_text(content)?;
                                doc.push_str(&format!("{}\n\n", content_str));
                            }
                        }
                    }
                }
                
                // Project metadata
                if let Some(metadata) = map.get("metadata") {
                    doc.push_str("## Project Information\n\n");
                    if let CursedObject::Map(meta_map) = metadata {
                        for (key, value) in meta_map {
                            let value_str = self.render_text(value)?;
                            doc.push_str(&format!("**{}**: {}\n", self.capitalize_first(key), value_str));
                        }
                        doc.push_str("\n");
                    }
                }
                
                Ok(doc)
            }
            _ => self.render_markdown(data)
        }
    }
    
    /// Render release notes
    fn render_release_notes(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut notes = String::new();
                
                // Release title
                if let Some(title) = map.get("title") {
                    let title_str = self.render_text(title)?;
                    notes.push_str(&format!("# {}\n\n", title_str));
                } else {
                    notes.push_str("# Release Notes\n\n");
                // Release summary
                if let Some(summary) = map.get("summary") {
                    let summary_str = self.render_text(summary)?;
                    notes.push_str(&format!("{}\n\n", summary_str));
                // Release information
                if let Some(version) = map.get("version") {
                    let version_str = self.render_text(version)?;
                    notes.push_str(&format!("**Version:** {}\n", version_str));
                if let Some(date) = map.get("date") {
                    let date_str = self.render_text(date)?;
                    notes.push_str(&format!("**Release Date:** {}\n", date_str));
                if let Some(tag) = map.get("tag") {
                    let tag_str = self.render_text(tag)?;
                    notes.push_str(&format!("**Git Tag:** {}\n", tag_str));
                notes.push_str("\n");
                
                // Highlights
                if let Some(highlights) = map.get("highlights") {
                    notes.push_str("## ✨ Highlights\n\n");
                    match highlights {
                        CursedObject::Array(highlight_array) => {
                            for highlight in highlight_array {
                                let highlight_str = self.render_text(highlight)?;
                                notes.push_str(&format!("- {}\n", highlight_str));
                            }
                        }
                        _ => {
                            let highlight_str = self.render_text(highlights)?;
                            notes.push_str(&format!("{}\n", highlight_str));
                        }
                    }
                    notes.push_str("\n");
                // Changes sections
                let change_sections = vec![
                ];
                
                for (key, title) in change_sections {
                    if let Some(changes) = map.get(key) {
                        notes.push_str(&format!("## {}\n\n", title));
                        match changes {
                            CursedObject::Array(change_array) => {
                                for change in change_array {
                                    match change {
                                        CursedObject::Map(change_map) => {
                                            let description = change_map.get("description")
                                                .map(|d| self.render_text(d))
                                                .unwrap_or_else(|| Ok("".to_string()))?;
                                            
                                            let pr = change_map.get("pr")
                                                .map(|p| self.render_text(p))
                                                .unwrap_or_else(|| Ok("".to_string()))?;
                                            
                                            let author = change_map.get("author")
                                                .map(|a| self.render_text(a))
                                                .unwrap_or_else(|| Ok("".to_string()))?;
                                            
                                            let mut change_line = format!("- {}", description);
                                            if !pr.is_empty() {
                                                change_line.push_str(&format!(" (#{}", pr));
                                                if !author.is_empty() {
                                                    change_line.push_str(&format!(" by @{}", author));
                                                }
                                                change_line.push(')');
                                            } else if !author.is_empty() {
                                                change_line.push_str(&format!(" by @{}", author));
                                            }
                                            notes.push_str(&format!("{}\n", change_line));
                                        }
                                        _ => {
                                            let change_str = self.render_text(change)?;
                                            notes.push_str(&format!("- {}\n", change_str));
                                        }
                                    }
                                }
                            }
                            _ => {
                                let changes_str = self.render_text(changes)?;
                                notes.push_str(&format!("{}\n", changes_str));
                            }
                        }
                        notes.push_str("\n");
                    }
                }
                
                // Contributors
                if let Some(contributors) = map.get("contributors") {
                    notes.push_str("## 👥 Contributors\n\n");
                    notes.push_str("Thanks to all the contributors who made this release possible:\n\n");
                    if let CursedObject::Array(contributor_array) = contributors {
                        for contributor in contributor_array {
                            let contributor_str = self.render_text(contributor)?;
                            notes.push_str(&format!("- @{}\n", contributor_str));
                        }
                    }
                    notes.push_str("\n");
                // Migration notes
                if let Some(migration) = map.get("migration") {
                    notes.push_str("## 📝 Migration Guide\n\n");
                    let migration_str = self.render_text(migration)?;
                    notes.push_str(&format!("{}\n\n", migration_str));
                // Installation/upgrade instructions
                if let Some(installation) = map.get("installation") {
                    notes.push_str("## 📥 Installation\n\n");
                    let install_str = self.render_text(installation)?;
                    notes.push_str(&format!("{}\n\n", install_str));
                // Links
                if let Some(links) = map.get("links") {
                    notes.push_str("## 🔗 Links\n\n");
                    if let CursedObject::Map(link_map) = links {
                        for (name, url) in link_map {
                            let url_str = self.render_text(url)?;
                            notes.push_str(&format!("- [{}]({})\n", name, url_str));
                        }
                    }
                    notes.push_str("\n");
                Ok(notes)
            }
            _ => self.render_markdown(data)
        }
    }
    /// Render as GraphQL schema
    fn render_graphql(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut schema = String::new();
                
                // Add schema description if present
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    schema.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", desc_str));
                // Render schema definition
                if let Some(schema_def) = map.get("schema") {
                    schema.push_str(&self.render_graphql_schema_definition(schema_def)?);
                    schema.push('\n');
                // Render custom scalars
                if let Some(scalars) = map.get("scalars") {
                    schema.push_str(&self.render_graphql_scalars(scalars)?);
                // Render directives
                if let Some(directives) = map.get("directives") {
                    schema.push_str(&self.render_graphql_directives(directives)?);
                // Render types
                if let Some(types) = map.get("types") {
                    schema.push_str(&self.render_graphql_types(types)?);
                Ok(schema.trim().to_string())
            }
            _ => self.render_text(data)
        }
    }
    
    /// Render GraphQL schema definition
    fn render_graphql_schema_definition(&self, schema_def: &CursedObject) -> crate::error::Result<()> {
        match schema_def {
            CursedObject::Map(map) => {
                let mut schema = String::from("schema {\n");
                
                if let Some(query) = map.get("query") {
                    let query_str = self.render_text(query)?;
                    schema.push_str(&format!("  query: {}\n", query_str));
                if let Some(mutation) = map.get("mutation") {
                    let mutation_str = self.render_text(mutation)?;
                    schema.push_str(&format!("  mutation: {}\n", mutation_str));
                if let Some(subscription) = map.get("subscription") {
                    let subscription_str = self.render_text(subscription)?;
                    schema.push_str(&format!("  subscription: {}\n", subscription_str));
                schema.push_str("}\n");
                Ok(schema)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render GraphQL custom scalars
    fn render_graphql_scalars(&self, scalars: &CursedObject) -> crate::error::Result<()> {
        match scalars {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for scalar in arr {
                    if let CursedObject::Map(scalar_map) = scalar {
                        if let Some(name) = scalar_map.get("name") {
                            let name_str = self.render_text(name)?;
                            
                            // Add description if present
                            if let Some(description) = scalar_map.get("description") {
                                let desc_str = self.render_text(description)?;
                                result.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", desc_str));
                            result.push_str(&format!("scalar {}", name_str));
                            
                            // Add directives if present
                            if let Some(directives) = scalar_map.get("directives") {
                                result.push_str(&self.render_graphql_field_directives(directives)?);
                            result.push_str("\n\n");
                        }
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render GraphQL directive definitions
    fn render_graphql_directives(&self, directives: &CursedObject) -> crate::error::Result<()> {
        match directives {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for directive in arr {
                    if let CursedObject::Map(dir_map) = directive {
                        if let Some(name) = dir_map.get("name") {
                            let name_str = self.render_text(name)?;
                            
                            // Add description if present
                            if let Some(description) = dir_map.get("description") {
                                let desc_str = self.render_text(description)?;
                                result.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", desc_str));
                            result.push_str(&format!("directive @{}", name_str));
                            
                            // Add arguments if present
                            if let Some(arguments) = dir_map.get("arguments") {
                                result.push_str(&self.render_graphql_arguments(arguments, false)?);
                            // Add locations
                            if let Some(locations) = dir_map.get("locations") {
                                result.push_str(" on ");
                                match locations {
                                    CursedObject::Array(loc_arr) => {
                                        let locs: crate::error::Result<()> = loc_arr.iter()
                                            .map(|loc| self.render_text(loc))
                                            .collect();
                                        result.push_str(&locs?.join(" | "));
                                    }
                                    _ => {
                                        let loc_str = self.render_text(locations)?;
                                        result.push_str(&loc_str);
                                    }
                                }
                            result.push_str("\n\n");
                        }
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render GraphQL types
    fn render_graphql_types(&self, types: &CursedObject) -> crate::error::Result<()> {
        match types {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for type_def in arr {
                    if let CursedObject::Map(type_map) = type_def {
                        result.push_str(&self.render_graphql_type_definition(type_map)?);
                        result.push_str("\n\n");
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render individual GraphQL type definition
    fn render_graphql_type_definition(&self, type_map: &std::collections::HashMap<String, CursedObject>) -> crate::error::Result<()> {
        let kind = type_map.get("kind")
            .map(|k| self.render_text(k))
            .unwrap_or_else(|| Ok("object".to_string()))?;
        
        let name = type_map.get("name")
            .map(|n| self.render_text(n))
            .unwrap_or_else(|| Ok("UnknownType".to_string()))?;
        
        let mut type_def = String::new();
        
        // Add description if present
        if let Some(description) = type_map.get("description") {
            let desc_str = self.render_text(description)?;
            type_def.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", desc_str));
        match kind.as_str() {
            "object" => {
                type_def.push_str(&format!("type {}", name));
                
                // Add implements if present
                if let Some(implements) = type_map.get("implements") {
                    type_def.push_str(" implements ");
                    match implements {
                        CursedObject::Array(impl_arr) => {
                            let interfaces: crate::error::Result<()> = impl_arr.iter()
                                .map(|iface| self.render_text(iface))
                                .collect();
                            type_def.push_str(&interfaces?.join(" & "));
                        }
                        _ => {
                            let impl_str = self.render_text(implements)?;
                            type_def.push_str(&impl_str);
                        }
                    }
                // Add directives if present
                if let Some(directives) = type_map.get("directives") {
                    type_def.push_str(&self.render_graphql_field_directives(directives)?);
                type_def.push_str(" {\n");
                
                // Add fields
                if let Some(fields) = type_map.get("fields") {
                    type_def.push_str(&self.render_graphql_fields(fields)?);
                type_def.push('}');
            }
            "interface" => {
                type_def.push_str(&format!("interface {}", name));
                
                // Add implements if present (for interface extending interface)
                if let Some(implements) = type_map.get("implements") {
                    type_def.push_str(" implements ");
                    match implements {
                        CursedObject::Array(impl_arr) => {
                            let interfaces: crate::error::Result<()> = impl_arr.iter()
                                .map(|iface| self.render_text(iface))
                                .collect();
                            type_def.push_str(&interfaces?.join(" & "));
                        }
                        _ => {
                            let impl_str = self.render_text(implements)?;
                            type_def.push_str(&impl_str);
                        }
                    }
                // Add directives if present
                if let Some(directives) = type_map.get("directives") {
                    type_def.push_str(&self.render_graphql_field_directives(directives)?);
                type_def.push_str(" {\n");
                
                // Add fields
                if let Some(fields) = type_map.get("fields") {
                    type_def.push_str(&self.render_graphql_fields(fields)?);
                type_def.push('}');
            }
            "union" => {
                type_def.push_str(&format!("union {}", name));
                
                // Add directives if present
                if let Some(directives) = type_map.get("directives") {
                    type_def.push_str(&self.render_graphql_field_directives(directives)?);
                // Add union members
                if let Some(types) = type_map.get("types") {
                    type_def.push_str(" = ");
                    match types {
                        CursedObject::Array(type_arr) => {
                            let union_types: crate::error::Result<()> = type_arr.iter()
                                .map(|t| self.render_text(t))
                                .collect();
                            type_def.push_str(&union_types?.join(" | "));
                        }
                        _ => {
                            let types_str = self.render_text(types)?;
                            type_def.push_str(&types_str);
                        }
                    }
                }
            }
            "enum" => {
                type_def.push_str(&format!("enum {}", name));
                
                // Add directives if present
                if let Some(directives) = type_map.get("directives") {
                    type_def.push_str(&self.render_graphql_field_directives(directives)?);
                type_def.push_str(" {\n");
                
                // Add enum values
                if let Some(values) = type_map.get("values") {
                    type_def.push_str(&self.render_graphql_enum_values(values)?);
                type_def.push('}');
            }
            "input" => {
                type_def.push_str(&format!("input {}", name));
                
                // Add directives if present
                if let Some(directives) = type_map.get("directives") {
                    type_def.push_str(&self.render_graphql_field_directives(directives)?);
                type_def.push_str(" {\n");
                
                // Add input fields
                if let Some(fields) = type_map.get("fields") {
                    type_def.push_str(&self.render_graphql_input_fields(fields)?);
                type_def.push('}');
            }
            "scalar" => {
                type_def.push_str(&format!("scalar {}", name));
                
                // Add directives if present
                if let Some(directives) = type_map.get("directives") {
                    type_def.push_str(&self.render_graphql_field_directives(directives)?);
                }
            }
            _ => {
                // Default to object type
                type_def.push_str(&format!("type {}", name));
                type_def.push_str(" {\n");
                
                if let Some(fields) = type_map.get("fields") {
                    type_def.push_str(&self.render_graphql_fields(fields)?);
                type_def.push('}');
            }
        }
        
        Ok(type_def)
    /// Render GraphQL fields
    fn render_graphql_fields(&self, fields: &CursedObject) -> crate::error::Result<()> {
        match fields {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for field in arr {
                    if let CursedObject::Map(field_map) = field {
                        result.push_str(&self.render_graphql_field(field_map)?);
                        result.push('\n');
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render individual GraphQL field
    fn render_graphql_field(&self, field_map: &std::collections::HashMap<String, CursedObject>) -> crate::error::Result<()> {
        let mut field_def = String::new();
        
        // Add description if present
        if let Some(description) = field_map.get("description") {
            let desc_str = self.render_text(description)?;
            field_def.push_str(&format!("  \"\"\"\n  {}\n  \"\"\"\n", desc_str));
        field_def.push_str("  ");
        
        let name = field_map.get("name")
            .map(|n| self.render_text(n))
            .unwrap_or_else(|| Ok("unknownField".to_string()))?;
        
        field_def.push_str(&name);
        
        // Add arguments if present
        if let Some(arguments) = field_map.get("arguments") {
            field_def.push_str(&self.render_graphql_arguments(arguments, true)?);
        // Add type
        if let Some(field_type) = field_map.get("type") {
            let type_str = self.render_text(field_type)?;
            field_def.push_str(&format!(": {}", type_str));
        // Add directives if present
        if let Some(directives) = field_map.get("directives") {
            field_def.push_str(&self.render_graphql_field_directives(directives)?);
        Ok(field_def)
    /// Render GraphQL input fields
    fn render_graphql_input_fields(&self, fields: &CursedObject) -> crate::error::Result<()> {
        match fields {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for field in arr {
                    if let CursedObject::Map(field_map) = field {
                        result.push_str(&self.render_graphql_input_field(field_map)?);
                        result.push('\n');
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render individual GraphQL input field
    fn render_graphql_input_field(&self, field_map: &std::collections::HashMap<String, CursedObject>) -> crate::error::Result<()> {
        let mut field_def = String::new();
        
        // Add description if present
        if let Some(description) = field_map.get("description") {
            let desc_str = self.render_text(description)?;
            field_def.push_str(&format!("  \"\"\"\n  {}\n  \"\"\"\n", desc_str));
        field_def.push_str("  ");
        
        let name = field_map.get("name")
            .map(|n| self.render_text(n))
            .unwrap_or_else(|| Ok("unknownField".to_string()))?;
        
        field_def.push_str(&name);
        
        // Add type
        if let Some(field_type) = field_map.get("type") {
            let type_str = self.render_text(field_type)?;
            field_def.push_str(&format!(": {}", type_str));
        // Add default value if present
        if let Some(default_value) = field_map.get("defaultValue") {
            let default_str = self.render_graphql_value(default_value)?;
            field_def.push_str(&format!(" = {}", default_str));
        // Add directives if present
        if let Some(directives) = field_map.get("directives") {
            field_def.push_str(&self.render_graphql_field_directives(directives)?);
        Ok(field_def)
    /// Render GraphQL enum values
    fn render_graphql_enum_values(&self, values: &CursedObject) -> crate::error::Result<()> {
        match values {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for value in arr {
                    match value {
                        CursedObject::Map(value_map) => {
                            // Add description if present
                            if let Some(description) = value_map.get("description") {
                                let desc_str = self.render_text(description)?;
                                result.push_str(&format!("  \"\"\"\n  {}\n  \"\"\"\n", desc_str));
                            result.push_str("  ");
                            
                            let name = value_map.get("name")
                                .map(|n| self.render_text(n))
                                .unwrap_or_else(|| Ok("UNKNOWN_VALUE".to_string()))?;
                            
                            result.push_str(&name);
                            
                            // Add directives if present
                            if let Some(directives) = value_map.get("directives") {
                                result.push_str(&self.render_graphql_field_directives(directives)?);
                            result.push('\n');
                        }
                        _ => {
                            // Simple string value
                            let value_str = self.render_text(value)?;
                            result.push_str(&format!("  {}\n", value_str));
                        }
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render GraphQL arguments
    fn render_graphql_arguments(&self, arguments: &CursedObject, add_spacing: bool) -> crate::error::Result<()> {
        match arguments {
            CursedObject::Array(arr) => {
                if arr.is_empty() {
                    return Ok(String::new());
                let mut result = String::from("(");
                
                for (i, arg) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    if let CursedObject::Map(arg_map) = arg {
                        let name = arg_map.get("name")
                            .map(|n| self.render_text(n))
                            .unwrap_or_else(|| Ok("unknownArg".to_string()))?;
                        
                        result.push_str(&name);
                        
                        if let Some(arg_type) = arg_map.get("type") {
                            let type_str = self.render_text(arg_type)?;
                            result.push_str(&format!(": {}", type_str));
                        // Add default value if present
                        if let Some(default_value) = arg_map.get("defaultValue") {
                            let default_str = self.render_graphql_value(default_value)?;
                            result.push_str(&format!(" = {}", default_str));
                        // Add directives if present
                        if let Some(directives) = arg_map.get("directives") {
                            result.push_str(&self.render_graphql_field_directives(directives)?);
                        }
                    }
                result.push(')');
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render GraphQL field directives
    fn render_graphql_field_directives(&self, directives: &CursedObject) -> crate::error::Result<()> {
        match directives {
            CursedObject::Array(arr) => {
                let mut result = String::new();
                for directive in arr {
                    if let CursedObject::Map(dir_map) = directive {
                        if let Some(name) = dir_map.get("name") {
                            let name_str = self.render_text(name)?;
                            result.push_str(&format!(" @{}", name_str));
                            
                            // Add directive arguments if present
                            if let Some(arguments) = dir_map.get("arguments") {
                                if let CursedObject::Map(arg_map) = arguments {
                                    if !arg_map.is_empty() {
                                        result.push('(');
                                        let mut first = true;
                                        for (key, value) in arg_map {
                                            if !first {
                                                result.push_str(", ");
                                            }
                                            first = false;
                                            let value_str = self.render_graphql_value(value)?;
                                            result.push_str(&format!("{}: {}", key, value_str));
                                        }
                                        result.push(')');
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(result)
            }
            _ => Ok(String::new())
        }
    }
    
    /// Render GraphQL value (for default values, directive arguments, etc.)
    fn render_graphql_value(&self, value: &CursedObject) -> crate::error::Result<()> {
        match value {
            CursedObject::Array(arr) => {
                let values: crate::error::Result<()> = arr.iter()
                    .map(|item| self.render_graphql_value(item))
                    .collect();
                Ok(format!("[{}]", values?.join(", ")))
            }
            CursedObject::Map(map) => {
                let mut result = String::from("{");
                let mut first = true;
                for (key, val) in map {
                    if !first {
                        result.push_str(", ");
                    }
                    first = false;
                    let val_str = self.render_graphql_value(val)?;
                    result.push_str(&format!("{}: {}", key, val_str));
                }
                result.push('}');
                Ok(result)
            }
        }
    }
    fn render_protobuf(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut proto = String::new();
                
                // Syntax declaration (default to proto3)
                let syntax = map.get("syntax")
                    .map(|s| self.render_text(s))
                    .unwrap_or_else(|| Ok("proto3".to_string()))?;
                proto.push_str(&format!("syntax = \"{}\";\n\n", syntax));
                
                // Package declaration
                if let Some(package) = map.get("package") {
                    let pkg_str = self.render_text(package)?;
                    proto.push_str(&format!("package {};\n\n", pkg_str));
                // Imports
                if let Some(imports) = map.get("imports") {
                    match imports {
                        CursedObject::Array(import_list) => {
                            for import in import_list {
                                let import_str = self.render_text(import)?;
                                proto.push_str(&format!("import \"{}\";\n", import_str));
                            }
                            proto.push('\n');
                        }
                        _ => {
                            let import_str = self.render_text(imports)?;
                            proto.push_str(&format!("import \"{}\";\n\n", import_str));
                        }
                    }
                // Options
                if let Some(options) = map.get("options") {
                    if let CursedObject::Map(option_map) = options {
                        for (key, value) in option_map {
                            let value_str = self.render_protobuf_option_value(value)?;
                            proto.push_str(&format!("option {} = {};\n", key, value_str));
                        }
                        proto.push('\n');
                    }
                }
                
                // Enums
                if let Some(enums) = map.get("enums") {
                    if let CursedObject::Array(enum_list) = enums {
                        for enum_def in enum_list {
                            proto.push_str(&self.render_protobuf_enum(enum_def)?);
                            proto.push('\n');
                        }
                    }
                // Messages
                if let Some(messages) = map.get("messages") {
                    if let CursedObject::Array(message_list) = messages {
                        for message in message_list {
                            proto.push_str(&self.render_protobuf_message(message, 0)?);
                            proto.push('\n');
                        }
                    }
                // Services
                if let Some(services) = map.get("services") {
                    if let CursedObject::Array(service_list) = services {
                        for service in service_list {
                            proto.push_str(&self.render_protobuf_service(service)?);
                            proto.push('\n');
                        }
                    }
                Ok(proto.trim_end().to_string())
            }
            _ => Err(CursedError::TemplateError {
        }
    }
    
    /// Render protobuf enum definition
    fn render_protobuf_enum(&self, enum_def: &CursedObject) -> crate::error::Result<()> {
        if let CursedObject::Map(enum_map) = enum_def {
            let name = enum_map.get("name")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let name_str = self.render_text(name)?;
            
            let mut enum_proto = format!("enum {} {{\n", name_str);
            
            // Enum options
            if let Some(options) = enum_map.get("options") {
                if let CursedObject::Map(option_map) = options {
                    for (key, value) in option_map {
                        let value_str = self.render_protobuf_option_value(value)?;
                        enum_proto.push_str(&format!("  option {} = {};\n", key, value_str));
                    }
                    enum_proto.push('\n');
                }
            }
            
            // Enum values
            if let Some(values) = enum_map.get("values") {
                if let CursedObject::Array(value_list) = values {
                    for value in value_list {
                        if let CursedObject::Map(value_map) = value {
                            let value_name = value_map.get("name")
                                .map(|n| self.render_text(n))
                                .unwrap_or_else(|| Ok("UNKNOWN".to_string()))?;
                            let value_number = value_map.get("number")
                                .map(|n| self.render_text(n))
                                .unwrap_or_else(|| Ok("0".to_string()))?;
                            
                            enum_proto.push_str(&format!("  {} = {};\n", value_name, value_number));
                        }
                    }
                }
            }
            
            enum_proto.push('}');
            Ok(enum_proto)
        } else {
            Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render protobuf message definition
    fn render_protobuf_message(&self, message: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        if let CursedObject::Map(message_map) = message {
            let name = message_map.get("name")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let name_str = self.render_text(name)?;
            
            let indent = "  ".repeat(indent_level);
            let mut message_proto = format!("{}message {} {{\n", indent, name_str);
            
            // Message options
            if let Some(options) = message_map.get("options") {
                if let CursedObject::Map(option_map) = options {
                    for (key, value) in option_map {
                        let value_str = self.render_protobuf_option_value(value)?;
                        message_proto.push_str(&format!("{}  option {} = {};\n", indent, key, value_str));
                    }
                    message_proto.push('\n');
                }
            }
            
            // Nested enums
            if let Some(nested_enums) = message_map.get("enums") {
                if let CursedObject::Array(enum_list) = nested_enums {
                    for enum_def in enum_list {
                        let enum_str = self.render_protobuf_enum(enum_def)?;
                        let indented_enum = enum_str.split("\n")
                            .map(|line| format!("{}  {}", indent, line))
                            .collect::<Vec<_>>()
                            .join("\n");
                        message_proto.push_str(&indented_enum);
                        message_proto.push_str("\n\n");
                    }
                }
            // Nested messages
            if let Some(nested_messages) = message_map.get("messages") {
                if let CursedObject::Array(nested_list) = nested_messages {
                    for nested_message in nested_list {
                        message_proto.push_str(&self.render_protobuf_message(nested_message, indent_level + 1)?);
                        message_proto.push_str("\n\n");
                    }
                }
            // Oneofs
            if let Some(oneofs) = message_map.get("oneofs") {
                if let CursedObject::Array(oneof_list) = oneofs {
                    for oneof in oneof_list {
                        message_proto.push_str(&self.render_protobuf_oneof(oneof, indent_level + 1)?);
                        message_proto.push_str("\n\n");
                    }
                }
            // Fields
            if let Some(fields) = message_map.get("fields") {
                if let CursedObject::Array(field_list) = fields {
                    for field in field_list {
                        message_proto.push_str(&self.render_protobuf_field(field, indent_level + 1)?);
                        message_proto.push('\n');
                    }
                }
            message_proto.push_str(&format!("{}}}", indent));
            Ok(message_proto)
        } else {
            Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render protobuf oneof definition
    fn render_protobuf_oneof(&self, oneof: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        if let CursedObject::Map(oneof_map) = oneof {
            let name = oneof_map.get("name")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let name_str = self.render_text(name)?;
            
            let indent = "  ".repeat(indent_level);
            let mut oneof_proto = format!("{}oneof {} {{\n", indent, name_str);
            
            if let Some(fields) = oneof_map.get("fields") {
                if let CursedObject::Array(field_list) = fields {
                    for field in field_list {
                        oneof_proto.push_str(&self.render_protobuf_field(field, indent_level + 1)?);
                        oneof_proto.push('\n');
                    }
                }
            oneof_proto.push_str(&format!("{}}}", indent));
            Ok(oneof_proto)
        } else {
            Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render protobuf field definition
    fn render_protobuf_field(&self, field: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        if let CursedObject::Map(field_map) = field {
            let field_type = field_map.get("type")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let type_str = self.render_text(field_type)?;
            
            let name = field_map.get("name")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let name_str = self.render_text(name)?;
            
            let number = field_map.get("number")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let number_str = self.render_text(number)?;
            
            let indent = "  ".repeat(indent_level);
            let mut field_proto = String::new();
            
            // Check if field is repeated
            let is_repeated = field_map.get("repeated")
                .map(|r| match r {
                })
                .unwrap_or(false);
            
            // Check if field is optional (proto3)
            let is_optional = field_map.get("optional")
                .map(|o| match o {
                })
                .unwrap_or(false);
            
            // Build field declaration
            if is_repeated {
                field_proto.push_str(&format!("{}repeated {} {} = {}", indent, type_str, name_str, number_str));
            } else if is_optional {
                field_proto.push_str(&format!("{}optional {} {} = {}", indent, type_str, name_str, number_str));
            } else {
                field_proto.push_str(&format!("{}{} {} = {}", indent, type_str, name_str, number_str));
            // Field options
            if let Some(options) = field_map.get("options") {
                if let CursedObject::Map(option_map) = options {
                    if !option_map.is_empty() {
                        let mut option_strs = Vec::new();
                        for (key, value) in option_map {
                            let value_str = self.render_protobuf_option_value(value)?;
                            option_strs.push(format!("{} = {}", key, value_str));
                        }
                        field_proto.push_str(&format!(" [{}]", option_strs.join(", ")));
                    }
                }
            field_proto.push(';');
            Ok(field_proto)
        } else {
            Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render protobuf service definition
    fn render_protobuf_service(&self, service: &CursedObject) -> crate::error::Result<()> {
        if let CursedObject::Map(service_map) = service {
            let name = service_map.get("name")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let name_str = self.render_text(name)?;
            
            let mut service_proto = format!("service {} {{\n", name_str);
            
            // Service options
            if let Some(options) = service_map.get("options") {
                if let CursedObject::Map(option_map) = options {
                    for (key, value) in option_map {
                        let value_str = self.render_protobuf_option_value(value)?;
                        service_proto.push_str(&format!("  option {} = {};\n", key, value_str));
                    }
                    service_proto.push('\n');
                }
            }
            
            // RPC methods
            if let Some(methods) = service_map.get("methods") {
                if let CursedObject::Array(method_list) = methods {
                    for method in method_list {
                        service_proto.push_str(&self.render_protobuf_rpc_method(method)?);
                        service_proto.push('\n');
                    }
                }
            service_proto.push('}');
            Ok(service_proto)
        } else {
            Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render protobuf RPC method definition
    fn render_protobuf_rpc_method(&self, method: &CursedObject) -> crate::error::Result<()> {
        if let CursedObject::Map(method_map) = method {
            let name = method_map.get("name")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let name_str = self.render_text(name)?;
            
            let input_type = method_map.get("input_type")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let input_str = self.render_text(input_type)?;
            
            let output_type = method_map.get("output_type")
                .ok_or_else(|| CursedError::TemplateError {
                })?;
            let output_str = self.render_text(output_type)?;
            
            // Check for streaming
            let client_streaming = method_map.get("client_streaming")
                .map(|s| match s {
                })
                .unwrap_or(false);
            
            let server_streaming = method_map.get("server_streaming")
                .map(|s| match s {
                })
                .unwrap_or(false);
            
            let input_prefix = if client_streaming { "stream " } else { "" };
            let output_prefix = if server_streaming { "stream " } else { "" };
            
                name_str, input_prefix, input_str, output_prefix, output_str);
            
            // Method options
            if let Some(options) = method_map.get("options") {
                if let CursedObject::Map(option_map) = options {
                    if !option_map.is_empty() {
                        rpc_proto.push_str(" {\n");
                        for (key, value) in option_map {
                            let value_str = self.render_protobuf_option_value(value)?;
                            rpc_proto.push_str(&format!("    option {} = {};\n", key, value_str));
                        }
                        rpc_proto.push_str("  }");
                    }
                }
            Ok(rpc_proto)
        } else {
            Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render protobuf option value with proper formatting
    fn render_protobuf_option_value(&self, value: &CursedObject) -> crate::error::Result<()> {
        match value {
            _ => {
                let text = self.render_text(value)?;
                if text.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                    Ok(text)
                } else {
                    Ok(format!("\"{}\"", text))
                }
            }
        }
    }
    fn render_json_schema(&self, data: &CursedObject) -> crate::error::Result<()> {
        let schema = self.build_json_schema(data)?;
        serde_json::to_string_pretty(&schema)
            .map_err(|e| CursedError::TemplateError {
            })
    /// Build JSON Schema from CursedObject
    fn build_json_schema(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut schema = Map::new();
                
                // Set default schema version if not specified
                if !map.contains_key("$schema") {
                    schema.insert(
                        JsonValue::String("https://json-schema.org/draft/2020-12/schema".to_string())
                    );
                // Process all schema fields
                for (key, value) in map {
                    match key.as_str() {
                        "$schema" | "$id" | "title" | "description" | "type" => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                        "properties" => {
                            schema.insert(key.clone(), self.build_schema_properties(value)?);
                        }
                        "items" => {
                            schema.insert(key.clone(), self.build_schema_items(value)?);
                        }
                        "required" => {
                            schema.insert(key.clone(), self.build_required_array(value)?);
                        }
                        "enum" => {
                            schema.insert(key.clone(), self.build_enum_array(value)?);
                        }
                        "definitions" | "$defs" => {
                            schema.insert(key.clone(), self.build_schema_definitions(value)?);
                        }
                        "allOf" | "anyOf" | "oneOf" => {
                            schema.insert(key.clone(), self.build_schema_array(value)?);
                        }
                        "not" => {
                            schema.insert(key.clone(), self.build_json_schema(value)?);
                        }
                        "if" | "then" | "else" => {
                            schema.insert(key.clone(), self.build_json_schema(value)?);
                        }
                        // Validation keywords
                        "minimum" | "maximum" | "exclusiveMinimum" | "exclusiveMaximum" |
                        "multipleOf" | "minLength" | "maxLength" | "minItems" | "maxItems" |
                        "minProperties" | "maxProperties" | "uniqueItems" | "additionalProperties" |
                        "additionalItems" | "patternProperties" => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                        "pattern" | "format" | "contentMediaType" | "contentEncoding" => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                        "const" | "default" | "examples" => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                        "$ref" | "$anchor" | "$dynamicRef" | "$dynamicAnchor" => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                        "readOnly" | "writeOnly" | "deprecated" => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                        // Custom fields are passed through as-is
                        _ => {
                            schema.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                    }
                Ok(JsonValue::Object(schema))
            }
            _ => {
                // If not a map, try to infer schema from the data type
                self.infer_schema_from_value(data)
            }
        }
    /// Build properties object for JSON Schema
    fn build_schema_properties(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(props) => {
                let mut properties = Map::new();
                
                for (prop_name, prop_schema) in props {
                    properties.insert(prop_name.clone(), self.build_json_schema(prop_schema)?);
                Ok(JsonValue::Object(properties))
            }
            _ => Err(CursedError::TemplateError {
                message: "Properties must be an object/map".to_string(),
            })
        }
    }

    /// Build items schema for arrays
    fn build_schema_items(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Array(items) => {
                // Array of schemas (tuple validation)
                let schemas: crate::error::Result<()> = items.iter()
                    .map(|item| self.build_json_schema(item))
                    .collect();
                Ok(JsonValue::Array(schemas?))
            }
            _ => {
                // Single schema for all items
                self.build_json_schema(data)
            }
        }
    /// Build required array
    fn build_required_array(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Array(arr) => {
                let required: crate::error::Result<()> = arr.iter()
                    .map(|item| match item {
                        _ => Err(CursedError::TemplateError {
                        })
                    })
                    .collect();
                Ok(JsonValue::Array(required?))
            }
            _ => Err(CursedError::TemplateError {
            })
        }
    }

    /// Build enum array
    fn build_enum_array(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Array(arr) => {
                let enum_values: crate::error::Result<()> = arr.iter()
                    .map(|item| self.cursed_to_json(item))
                    .collect();
                Ok(JsonValue::Array(enum_values?))
            }
            _ => Err(CursedError::TemplateError {
            })
        }
    }

    /// Build schema definitions
    fn build_schema_definitions(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(defs) => {
                let mut definitions = Map::new();
                
                for (def_name, def_schema) in defs {
                    definitions.insert(def_name.clone(), self.build_json_schema(def_schema)?);
                Ok(JsonValue::Object(definitions))
            }
            _ => Err(CursedError::TemplateError {
                message: "Definitions must be an object/map".to_string(),
            })
        }
    }

    /// Build schema array for allOf, anyOf, oneOf
    fn build_schema_array(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Array(schemas) => {
                let schema_array: crate::error::Result<()> = schemas.iter()
                    .map(|schema| self.build_json_schema(schema))
                    .collect();
                Ok(JsonValue::Array(schema_array?))
            }
            _ => Err(CursedError::TemplateError {
            })
        }
    }

    /// Infer schema from a value
    fn infer_schema_from_value(&self, data: &CursedObject) -> crate::error::Result<()> {
        let mut schema = Map::new();
        
        // Set default schema version
        schema.insert(
            JsonValue::String("https://json-schema.org/draft/2020-12/schema".to_string())
        );
        
        match data {
            CursedObject::String(_) => {
                schema.insert("type".to_string(), JsonValue::String("string".to_string()));
            }
            CursedObject::Integer(_) => {
                schema.insert("type".to_string(), JsonValue::String("integer".to_string()));
            }
            CursedObject::Float(_) => {
                schema.insert("type".to_string(), JsonValue::String("number".to_string()));
            }
            CursedObject::Boolean(_) => {
                schema.insert("type".to_string(), JsonValue::String("boolean".to_string()));
            }
            CursedObject::Char(_) => {
                schema.insert("type".to_string(), JsonValue::String("string".to_string()));
                schema.insert("minLength".to_string(), JsonValue::Number(serde_json::Number::from(1)));
                schema.insert("maxLength".to_string(), JsonValue::Number(serde_json::Number::from(1)));
            }
            CursedObject::Nil => {
                schema.insert("type".to_string(), JsonValue::String("null".to_string()));
            }
            CursedObject::Array(arr) => {
                schema.insert("type".to_string(), JsonValue::String("array".to_string()));
                
                // Try to infer item schema from first element
                if let Some(first_item) = arr.first() {
                    schema.insert("items".to_string(), self.infer_schema_from_value(first_item)?);
                }
            }
            CursedObject::Map(map) => {
                schema.insert("type".to_string(), JsonValue::String("object".to_string()));
                
                // Infer properties from map keys
                let mut properties = Map::new();
                let mut required = Vec::new();
                
                for (key, value) in map {
                    properties.insert(key.clone(), self.infer_schema_from_value(value)?);
                    required.push(JsonValue::String(key.clone()));
                if !properties.is_empty() {
                    schema.insert("properties".to_string(), JsonValue::Object(properties));
                if !required.is_empty() {
                    schema.insert("required".to_string(), JsonValue::Array(required));
                schema.insert("additionalProperties".to_string(), JsonValue::Bool(false));
            }
        }
        
        Ok(JsonValue::Object(schema))
    }
    /// Render WSDL (Web Services Description Language)
    fn render_wsdl(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut wsdl = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                
                // WSDL root element with namespaces
                wsdl.push_str("<definitions xmlns=\"http://schemas.xmlsoap.org/wsdl/\"\n");
                wsdl.push_str("             xmlns:soap=\"http://schemas.xmlsoap.org/wsdl/soap/\"\n");
                wsdl.push_str("             xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\"\n");
                wsdl.push_str("             xmlns:tns=\"http://example.com/\"\n");
                wsdl.push_str("             targetNamespace=\"http://example.com/\">\n\n");
                
                // Types section
                if let Some(types) = map.get("types") {
                    wsdl.push_str("  <types>\n");
                    wsdl.push_str("    <xsd:schema targetNamespace=\"http://example.com/\">\n");
                    if let CursedObject::Array(type_array) = types {
                        for type_def in type_array {
                            if let CursedObject::Map(type_map) = type_def {
                                if let Some(name) = type_map.get("name") {
                                    let name_str = self.render_text(name)?;
                                    wsdl.push_str(&format!("      <xsd:complexType name=\"{}\">\n", name_str));
                                    
                                    if let Some(elements) = type_map.get("elements") {
                                        wsdl.push_str("        <xsd:sequence>\n");
                                        if let CursedObject::Array(elem_array) = elements {
                                            for element in elem_array {
                                                if let CursedObject::Map(elem_map) = element {
                                                    let elem_name = elem_map.get("name")
                                                        .map(|n| self.render_text(n))
                                                        .unwrap_or_else(|| Ok("element".to_string()))?;
                                                    let elem_type = elem_map.get("type")
                                                        .map(|t| self.render_text(t))
                                                        .unwrap_or_else(|| Ok("xsd:string".to_string()))?;
                                                    wsdl.push_str(&format!("          <xsd:element name=\"{}\" type=\"{}\"/>\n", elem_name, elem_type));
                                                }
                                            }
                                        }
                                        wsdl.push_str("        </xsd:sequence>\n");
                                    }
                                    wsdl.push_str("      </xsd:complexType>\n");
                                }
                            }
                        }
                    }
                    wsdl.push_str("    </xsd:schema>\n");
                    wsdl.push_str("  </types>\n\n");
                // Messages
                if let Some(messages) = map.get("messages") {
                    if let CursedObject::Array(msg_array) = messages {
                        for message in msg_array {
                            if let CursedObject::Map(msg_map) = message {
                                if let Some(name) = msg_map.get("name") {
                                    let name_str = self.render_text(name)?;
                                    wsdl.push_str(&format!("  <message name=\"{}\">\n", name_str));
                                    
                                    if let Some(parts) = msg_map.get("parts") {
                                        if let CursedObject::Array(part_array) = parts {
                                            for part in part_array {
                                                if let CursedObject::Map(part_map) = part {
                                                    let part_name = part_map.get("name")
                                                        .map(|n| self.render_text(n))
                                                        .unwrap_or_else(|| Ok("part".to_string()))?;
                                                    let part_type = part_map.get("type")
                                                        .map(|t| self.render_text(t))
                                                        .unwrap_or_else(|| Ok("xsd:string".to_string()))?;
                                                    wsdl.push_str(&format!("    <part name=\"{}\" type=\"{}\"/>\n", part_name, part_type));
                                                }
                                            }
                                        }
                                    }
                                    wsdl.push_str("  </message>\n\n");
                                }
                            }
                        }
                    }
                // Port types
                if let Some(port_types) = map.get("port_types") {
                    if let CursedObject::Array(pt_array) = port_types {
                        for port_type in pt_array {
                            if let CursedObject::Map(pt_map) = port_type {
                                if let Some(name) = pt_map.get("name") {
                                    let name_str = self.render_text(name)?;
                                    wsdl.push_str(&format!("  <portType name=\"{}\">\n", name_str));
                                    
                                    if let Some(operations) = pt_map.get("operations") {
                                        if let CursedObject::Array(op_array) = operations {
                                            for operation in op_array {
                                                if let CursedObject::Map(op_map) = operation {
                                                    let op_name = op_map.get("name")
                                                        .map(|n| self.render_text(n))
                                                        .unwrap_or_else(|| Ok("operation".to_string()))?;
                                                    wsdl.push_str(&format!("    <operation name=\"{}\">\n", op_name));
                                                    
                                                    if let Some(input) = op_map.get("input") {
                                                        let input_str = self.render_text(input)?;
                                                        wsdl.push_str(&format!("      <input message=\"tns:{}\"/>\n", input_str));
                                                    if let Some(output) = op_map.get("output") {
                                                        let output_str = self.render_text(output)?;
                                                        wsdl.push_str(&format!("      <output message=\"tns:{}\"/>\n", output_str));
                                                    wsdl.push_str("    </operation>\n");
                                                }
                                            }
                                        }
                                    }
                                    wsdl.push_str("  </portType>\n\n");
                                }
                            }
                        }
                    }
                // Bindings
                if let Some(bindings) = map.get("bindings") {
                    if let CursedObject::Array(binding_array) = bindings {
                        for binding in binding_array {
                            if let CursedObject::Map(binding_map) = binding {
                                if let Some(name) = binding_map.get("name") {
                                    let name_str = self.render_text(name)?;
                                    let port_type = binding_map.get("port_type")
                                        .map(|pt| self.render_text(pt))
                                        .unwrap_or_else(|| Ok("tns:ServicePortType".to_string()))?;
                                    
                                    wsdl.push_str(&format!("  <binding name=\"{}\" type=\"{}\">\n", name_str, port_type));
                                    wsdl.push_str("    <soap:binding style=\"rpc\" transport=\"http://schemas.xmlsoap.org/soap/http\"/>\n");
                                    wsdl.push_str("  </binding>\n\n");
                                }
                            }
                        }
                    }
                // Service
                if let Some(service) = map.get("service") {
                    if let CursedObject::Map(service_map) = service {
                        let name = service_map.get("name")
                            .map(|n| self.render_text(n))
                            .unwrap_or_else(|| Ok("Service".to_string()))?;
                        
                        wsdl.push_str(&format!("  <service name=\"{}\">\n", name));
                        
                        if let Some(ports) = service_map.get("ports") {
                            if let CursedObject::Array(port_array) = ports {
                                for port in port_array {
                                    if let CursedObject::Map(port_map) = port {
                                        let port_name = port_map.get("name")
                                            .map(|n| self.render_text(n))
                                            .unwrap_or_else(|| Ok("ServicePort".to_string()))?;
                                        let binding = port_map.get("binding")
                                            .map(|b| self.render_text(b))
                                            .unwrap_or_else(|| Ok("tns:ServiceBinding".to_string()))?;
                                        let location = port_map.get("location")
                                            .map(|l| self.render_text(l))
                                            .unwrap_or_else(|| Ok("http://example.com/service".to_string()))?;
                                        
                                        wsdl.push_str(&format!("    <port name=\"{}\" binding=\"{}\">\n", port_name, binding));
                                        wsdl.push_str(&format!("      <soap:address location=\"{}\"/>\n", location));
                                        wsdl.push_str("    </port>\n");
                                    }
                                }
                            }
                        }
                        wsdl.push_str("  </service>\n\n");
                    }
                }
                
                wsdl.push_str("</definitions>\n");
                Ok(wsdl)
            }
            _ => self.render_xml(data)
        }
    }
    
    /// Render AsyncAPI specification  
    fn render_asyncapi(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut asyncapi = String::new();
                
                // AsyncAPI version
                let version = map.get("asyncapi")
                    .map(|v| self.render_text(v))
                    .unwrap_or_else(|| Ok("2.6.0".to_string()))?;
                asyncapi.push_str(&format!("asyncapi: {}\n\n", version));
                
                // Info section
                if let Some(info) = map.get("info") {
                    asyncapi.push_str("info:\n");
                    if let CursedObject::Map(info_map) = info {
                        for (key, value) in info_map {
                            let value_str = self.render_text(value)?;
                            asyncapi.push_str(&format!("  {}: {}\n", key, value_str));
                        }
                    }
                    asyncapi.push_str("\n");
                // Servers
                if let Some(servers) = map.get("servers") {
                    asyncapi.push_str("servers:\n");
                    if let CursedObject::Map(server_map) = servers {
                        for (server_name, server_config) in server_map {
                            asyncapi.push_str(&format!("  {}:\n", server_name));
                            if let CursedObject::Map(config) = server_config {
                                for (key, value) in config {
                                    let value_str = self.render_text(value)?;
                                    asyncapi.push_str(&format!("    {}: {}\n", key, value_str));
                                }
                            }
                        }
                    }
                    asyncapi.push_str("\n");
                // Channels
                if let Some(channels) = map.get("channels") {
                    asyncapi.push_str("channels:\n");
                    if let CursedObject::Map(channel_map) = channels {
                        for (channel_name, channel_config) in channel_map {
                            asyncapi.push_str(&format!("  {}:\n", channel_name));
                            if let CursedObject::Map(config) = channel_config {
                                // Description
                                if let Some(description) = config.get("description") {
                                    let desc_str = self.render_text(description)?;
                                    asyncapi.push_str(&format!("    description: {}\n", desc_str));
                                // Subscribe operation
                                if let Some(subscribe) = config.get("subscribe") {
                                    asyncapi.push_str("    subscribe:\n");
                                    if let CursedObject::Map(sub_map) = subscribe {
                                        for (key, value) in sub_map {
                                            match key.as_str() {
                                                "message" => {
                                                    asyncapi.push_str("      message:\n");
                                                    if let CursedObject::Map(msg_map) = value {
                                                        for (msg_key, msg_value) in msg_map {
                                                            let msg_value_str = self.render_text(msg_value)?;
                                                            asyncapi.push_str(&format!("        {}: {}\n", msg_key, msg_value_str));
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let value_str = self.render_text(value)?;
                                                    asyncapi.push_str(&format!("      {}: {}\n", key, value_str));
                                                }
                                            }
                                        }
                                    }
                                // Publish operation
                                if let Some(publish) = config.get("publish") {
                                    asyncapi.push_str("    publish:\n");
                                    if let CursedObject::Map(pub_map) = publish {
                                        for (key, value) in pub_map {
                                            match key.as_str() {
                                                "message" => {
                                                    asyncapi.push_str("      message:\n");
                                                    if let CursedObject::Map(msg_map) = value {
                                                        for (msg_key, msg_value) in msg_map {
                                                            let msg_value_str = self.render_text(msg_value)?;
                                                            asyncapi.push_str(&format!("        {}: {}\n", msg_key, msg_value_str));
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let value_str = self.render_text(value)?;
                                                    asyncapi.push_str(&format!("      {}: {}\n", key, value_str));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    asyncapi.push_str("\n");
                // Components
                if let Some(components) = map.get("components") {
                    asyncapi.push_str("components:\n");
                    if let CursedObject::Map(comp_map) = components {
                        // Messages
                        if let Some(messages) = comp_map.get("messages") {
                            asyncapi.push_str("  messages:\n");
                            if let CursedObject::Map(msg_map) = messages {
                                for (msg_name, msg_config) in msg_map {
                                    asyncapi.push_str(&format!("    {}:\n", msg_name));
                                    asyncapi.push_str(&self.render_kubernetes_object(msg_config, 3)?);
                                }
                            }
                        // Schemas
                        if let Some(schemas) = comp_map.get("schemas") {
                            asyncapi.push_str("  schemas:\n");
                            if let CursedObject::Map(schema_map) = schemas {
                                for (schema_name, schema_config) in schema_map {
                                    asyncapi.push_str(&format!("    {}:\n", schema_name));
                                    asyncapi.push_str(&self.render_kubernetes_object(schema_config, 3)?);
                                }
                            }
                        // Security schemes
                        if let Some(security_schemes) = comp_map.get("securitySchemes") {
                            asyncapi.push_str("  securitySchemes:\n");
                            asyncapi.push_str(&self.render_kubernetes_object(security_schemes, 2)?);
                        }
                    }
                Ok(asyncapi)
            }
            _ => self.render_yaml(data)
        }
    }
    /// Render Rust build.rs file
    fn render_build_rs(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut build_rs = String::from("// Build script generated by CURSED template\n");
                build_rs.push_str("use std::env;\n\n");
                build_rs.push_str("fn main() {
        // TODO: implement
    }\n");

                // Handle linking libraries
                if let Some(link_libs) = map.get("link_libs") {
                    match link_libs {
                        CursedObject::Array(libs) => {
                            for lib in libs {
                                let lib_str = self.render_text(lib)?;
                                build_rs.push_str(&format!("    println!(\"cargo:rustc-link-lib={}\");\n", lib_str));
                            }
                        }
                        _ => {
                            let lib_str = self.render_text(link_libs)?;
                            build_rs.push_str(&format!("    println!(\"cargo:rustc-link-lib={}\");\n", lib_str));
                        }
                    }
                    build_rs.push('\n');
                // Handle link search paths
                if let Some(link_search) = map.get("link_search") {
                    match link_search {
                        CursedObject::Array(paths) => {
                            for path in paths {
                                let path_str = self.render_text(path)?;
                                build_rs.push_str(&format!("    println!(\"cargo:rustc-link-search=native={}\");\n", path_str));
                            }
                        }
                        _ => {
                            let path_str = self.render_text(link_search)?;
                            build_rs.push_str(&format!("    println!(\"cargo:rustc-link-search=native={}\");\n", path_str));
                        }
                    }
                    build_rs.push('\n');
                // Handle environment variables
                if let Some(env_vars) = map.get("env_vars") {
                    if let CursedObject::Map(env_map) = env_vars {
                        for (key, value) in env_map {
                            let value_str = self.render_text(value)?;
                            build_rs.push_str(&format!("    println!(\"cargo:rustc-env={}={}\");\n", key, value_str));
                        }
                        build_rs.push('\n');
                    }
                }

                // Handle re-run conditions
                if let Some(rerun_if) = map.get("rerun_if") {
                    match rerun_if {
                        CursedObject::Array(conditions) => {
                            for condition in conditions {
                                if let CursedObject::Map(cond_map) = condition {
                                    if let Some(changed) = cond_map.get("changed") {
                                        let file_str = self.render_text(changed)?;
                                        build_rs.push_str(&format!("    println!(\"cargo:rerun-if-changed={}\");\n", file_str));
                                    }
                                    if let Some(env_changed) = cond_map.get("env_changed") {
                                        let env_str = self.render_text(env_changed)?;
                                        build_rs.push_str(&format!("    println!(\"cargo:rerun-if-env-changed={}\");\n", env_str));
                                    }
                                }
                            }
                        }
                        _ => {
                            let rerun_str = self.render_text(rerun_if)?;
                            build_rs.push_str(&format!("    println!(\"cargo:rerun-if-changed={}\");\n", rerun_str));
                        }
                    }
                    build_rs.push('\n');
                // Handle custom code
                if let Some(custom) = map.get("custom_code") {
                    build_rs.push_str("    // Custom build logic\n");
                    let custom_str = self.render_text(custom)?;
                    // Add proper indentation to custom code
                    for line in custom_str.split("\n") {
                        if !line.trim().is_empty() {
                            build_rs.push_str(&format!("    {}\n", line));
                        } else {
                            build_rs.push('\n');
                        }
                    }
                    build_rs.push('\n');
                // Handle target-specific configuration
                if let Some(targets) = map.get("targets") {
                    if let CursedObject::Map(target_map) = targets {
                        build_rs.push_str("    // Target-specific configuration\n");
                        build_rs.push_str("    let target = env::var(\"TARGET\").unwrap();\n");
                        for (target_name, config) in target_map {
                            build_rs.push_str(&format!("    if target.contains(\"{}\") {{\n", target_name));
                            if let CursedObject::Map(target_config) = config {
                                for (key, value) in target_config {
                                    let value_str = self.render_text(value)?;
                                    match key.as_str() {
                                    }
                                }
                            }
                            build_rs.push_str("    }\n");
                        }
                        build_rs.push('\n');
                    }
                }

                build_rs.push_str("}\n");
                Ok(build_rs)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render CMakeLists.txt file
    fn render_cmake(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut cmake = String::new();

                // CMake minimum version
                let min_version = map.get("cmake_minimum_required")
                    .map(|v| self.render_text(v))
                    .unwrap_or_else(|| Ok("3.16".to_string()))?;
                cmake.push_str(&format!("cmake_minimum_required(VERSION {})\n\n", min_version));

                // Project declaration
                if let Some(project) = map.get("project") {
                    match project {
                        CursedObject::Map(proj_map) => {
                            let name = proj_map.get("name")
                                .map(|n| self.render_text(n))
                                .unwrap_or_else(|| Ok("MyProject".to_string()))?;
                            cmake.push_str(&format!("project({}", name));

                            if let Some(version) = proj_map.get("version") {
                                let version_str = self.render_text(version)?;
                                cmake.push_str(&format!(" VERSION {}", version_str));
                            if let Some(languages) = proj_map.get("languages") {
                                match languages {
                                    CursedObject::Array(langs) => {
                                        cmake.push_str(" LANGUAGES");
                                        for lang in langs {
                                            let lang_str = self.render_text(lang)?;
                                            cmake.push_str(&format!(" {}", lang_str));
                                        }
                                    }
                                    _ => {
                                        let lang_str = self.render_text(languages)?;
                                        cmake.push_str(&format!(" LANGUAGES {}", lang_str));
                                    }
                                }
                            }
                            cmake.push_str(")\n\n");
                        }
                        _ => {
                            let name = self.render_text(project)?;
                            cmake.push_str(&format!("project({})\n\n", name));
                        }
                    }
                // Set C++ standard
                if let Some(cxx_standard) = map.get("cxx_standard") {
                    let std_str = self.render_text(cxx_standard)?;
                    cmake.push_str(&format!("set(CMAKE_CXX_STANDARD {})\n", std_str));
                    cmake.push_str("set(CMAKE_CXX_STANDARD_REQUIRED ON)\n\n");
                // Find packages
                if let Some(packages) = map.get("find_package") {
                    match packages {
                        CursedObject::Array(pkg_array) => {
                            for pkg in pkg_array {
                                match pkg {
                                    CursedObject::Map(pkg_map) => {
                                        let name = pkg_map.get("name")
                                            .map(|n| self.render_text(n))
                                            .unwrap_or_else(|| Ok("Package".to_string()))?;
                                        cmake.push_str(&format!("find_package({}", name));

                                        if let Some(version) = pkg_map.get("version") {
                                            let version_str = self.render_text(version)?;
                                            cmake.push_str(&format!(" {}", version_str));
                                        if let Some(required) = pkg_map.get("required") {
                                            if let CursedObject::Boolean(true) = required {
                                                cmake.push_str(" REQUIRED");
                                            }
                                        }

                                        cmake.push_str(")\n");
                                    }
                                    _ => {
                                        let pkg_str = self.render_text(pkg)?;
                                        cmake.push_str(&format!("find_package({})\n", pkg_str));
                                    }
                                }
                            }
                            cmake.push('\n');
                        }
                        _ => {
                            let pkg_str = self.render_text(packages)?;
                            cmake.push_str(&format!("find_package({})\n\n", pkg_str));
                        }
                    }
                // Add subdirectories
                if let Some(subdirs) = map.get("add_subdirectory") {
                    match subdirs {
                        CursedObject::Array(dirs) => {
                            for dir in dirs {
                                let dir_str = self.render_text(dir)?;
                                cmake.push_str(&format!("add_subdirectory({})\n", dir_str));
                            }
                            cmake.push('\n');
                        }
                        _ => {
                            let dir_str = self.render_text(subdirs)?;
                            cmake.push_str(&format!("add_subdirectory({})\n\n", dir_str));
                        }
                    }
                // Targets (executables and libraries)
                if let Some(targets) = map.get("targets") {
                    if let CursedObject::Map(target_map) = targets {
                        for (target_name, target_config) in target_map {
                            if let CursedObject::Map(config) = target_config {
                                let target_type = config.get("type")
                                    .map(|t| self.render_text(t))
                                    .unwrap_or_else(|| Ok("executable".to_string()))?;

                                // Add target
                                match target_type.as_str() {
                                    "executable" => {
                                        cmake.push_str(&format!("add_executable({}", target_name));
                                    }
                                    "library" | "static" => {
                                        cmake.push_str(&format!("add_library({} STATIC", target_name));
                                    }
                                    "shared" => {
                                        cmake.push_str(&format!("add_library({} SHARED", target_name));
                                    }
                                    _ => {
                                        cmake.push_str(&format!("add_executable({}", target_name));
                                    }
                                }

                                // Add sources
                                if let Some(sources) = config.get("sources") {
                                    match sources {
                                        CursedObject::Array(src_array) => {
                                            for src in src_array {
                                                let src_str = self.render_text(src)?;
                                                cmake.push_str(&format!(" {}", src_str));
                                            }
                                        }
                                        _ => {
                                            let src_str = self.render_text(sources)?;
                                            cmake.push_str(&format!(" {}", src_str));
                                        }
                                    }
                                }
                                cmake.push_str(")\n");

                                // Target properties
                                if let Some(include_dirs) = config.get("include_directories") {
                                    cmake.push_str(&format!("target_include_directories({} PRIVATE", target_name));
                                    match include_dirs {
                                        CursedObject::Array(dirs) => {
                                            for dir in dirs {
                                                let dir_str = self.render_text(dir)?;
                                                cmake.push_str(&format!(" {}", dir_str));
                                            }
                                        }
                                        _ => {
                                            let dir_str = self.render_text(include_dirs)?;
                                            cmake.push_str(&format!(" {}", dir_str));
                                        }
                                    }
                                    cmake.push_str(")\n");
                                // Link libraries
                                if let Some(link_libs) = config.get("link_libraries") {
                                    cmake.push_str(&format!("target_link_libraries({}", target_name));
                                    match link_libs {
                                        CursedObject::Array(libs) => {
                                            for lib in libs {
                                                let lib_str = self.render_text(lib)?;
                                                cmake.push_str(&format!(" {}", lib_str));
                                            }
                                        }
                                        _ => {
                                            let lib_str = self.render_text(link_libs)?;
                                            cmake.push_str(&format!(" {}", lib_str));
                                        }
                                    }
                                    cmake.push_str(")\n");
                                // Compile definitions
                                if let Some(definitions) = config.get("compile_definitions") {
                                    cmake.push_str(&format!("target_compile_definitions({} PRIVATE", target_name));
                                    match definitions {
                                        CursedObject::Array(defs) => {
                                            for def in defs {
                                                let def_str = self.render_text(def)?;
                                                cmake.push_str(&format!(" {}", def_str));
                                            }
                                        }
                                        _ => {
                                            let def_str = self.render_text(definitions)?;
                                            cmake.push_str(&format!(" {}", def_str));
                                        }
                                    }
                                    cmake.push_str(")\n");
                                cmake.push('\n');
                            }
                        }
                    }
                }

                // Installation rules
                if let Some(install) = map.get("install") {
                    if let CursedObject::Map(install_map) = install {
                        for (install_type, install_config) in install_map {
                            cmake.push_str(&format!("install({}", install_type.to_uppercase()));
                            if let CursedObject::Map(config) = install_config {
                                for (key, value) in config {
                                    let value_str = self.render_text(value)?;
                                    cmake.push_str(&format!(" {} {}", key.to_uppercase(), value_str));
                                }
                            }
                            cmake.push_str(")\n");
                        }
                        cmake.push('\n');
                    }
                }

                Ok(cmake)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render Gradle build.gradle file (Kotlin DSL preferred)
    fn render_gradle(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut gradle = String::new();

                // Plugins
                if let Some(plugins) = map.get("plugins") {
                    gradle.push_str("plugins {\n");
                    match plugins {
                        CursedObject::Array(plugin_array) => {
                            for plugin in plugin_array {
                                match plugin {
                                    CursedObject::Map(plugin_map) => {
                                        if let Some(id) = plugin_map.get("id") {
                                            let id_str = self.render_text(id)?;
                                            gradle.push_str(&format!("    id(\"{}\") ", id_str));
                                            if let Some(version) = plugin_map.get("version") {
                                                let version_str = self.render_text(version)?;
                                                gradle.push_str(&format!("version \"{}\" ", version_str));
                                            }
                                            if let Some(apply) = plugin_map.get("apply") {
                                                if let CursedObject::Boolean(false) = apply {
                                                    gradle.push_str("apply false ");
                                                }
                                            }
                                            gradle.push_str("\n");
                                        }
                                    }
                                    _ => {
                                        let plugin_str = self.render_text(plugin)?;
                                        gradle.push_str(&format!("    id(\"{}\")\n", plugin_str));
                                    }
                                }
                            }
                        }
                        _ => {
                            let plugin_str = self.render_text(plugins)?;
                            gradle.push_str(&format!("    id(\"{}\")\n", plugin_str));
                        }
                    }
                    gradle.push_str("}\n\n");
                // Project properties
                if let Some(group) = map.get("group") {
                    let group_str = self.render_text(group)?;
                    gradle.push_str(&format!("group = \"{}\"\n", group_str));
                if let Some(version) = map.get("version") {
                    let version_str = self.render_text(version)?;
                    gradle.push_str(&format!("version = \"{}\"\n", version_str));
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    gradle.push_str(&format!("description = \"{}\"\n", desc_str));
                gradle.push('\n');

                // Java configuration
                if let Some(java) = map.get("java") {
                    gradle.push_str("java {\n");
                    if let CursedObject::Map(java_map) = java {
                        if let Some(source_compatibility) = java_map.get("sourceCompatibility") {
                            let compat_str = self.render_text(source_compatibility)?;
                            gradle.push_str(&format!("    sourceCompatibility = JavaVersion.VERSION_{}\n", compat_str.replace('.', "_")));
                        }
                        if let Some(target_compatibility) = java_map.get("targetCompatibility") {
                            let compat_str = self.render_text(target_compatibility)?;
                            gradle.push_str(&format!("    targetCompatibility = JavaVersion.VERSION_{}\n", compat_str.replace('.', "_")));
                        }
                    }
                    gradle.push_str("}\n\n");
                // Repositories
                if let Some(repositories) = map.get("repositories") {
                    gradle.push_str("repositories {\n");
                    match repositories {
                        CursedObject::Array(repo_array) => {
                            for repo in repo_array {
                                let repo_str = self.render_text(repo)?;
                                match repo_str.as_str() {
                                    "mavenCentral" | "gradlePluginPortal" | "google" => {
                                        gradle.push_str(&format!("    {}()\n", repo_str));
                                    }
                                    _ => {
                                        gradle.push_str(&format!("    maven {{ url = uri(\"{}\") }}\n", repo_str));
                                    }
                                }
                            }
                        }
                        _ => {
                            let repo_str = self.render_text(repositories)?;
                            gradle.push_str(&format!("    {}()\n", repo_str));
                        }
                    }
                    gradle.push_str("}\n\n");
                // Dependencies
                if let Some(dependencies) = map.get("dependencies") {
                    gradle.push_str("dependencies {\n");
                    if let CursedObject::Map(dep_map) = dependencies {
                        for (config, deps) in dep_map {
                            match deps {
                                CursedObject::Array(dep_array) => {
                                    for dep in dep_array {
                                        match dep {
                                            CursedObject::Map(dep_details) => {
                                                if let (Some(group), Some(name), Some(version)) = 
                                                    (dep_details.get("group"), dep_details.get("name"), dep_details.get("version")) {
                                                    let group_str = self.render_text(group)?;
                                                    let name_str = self.render_text(name)?;
                                                    let version_str = self.render_text(version)?;
                                                    gradle.push_str(&format!("    {}(\"{}:{}:{}\")\n", config, group_str, name_str, version_str));
                                                }
                                            }
                                            _ => {
                                                let dep_str = self.render_text(dep)?;
                                                gradle.push_str(&format!("    {}(\"{}\")\n", config, dep_str));
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    let dep_str = self.render_text(deps)?;
                                    gradle.push_str(&format!("    {}(\"{}\")\n", config, dep_str));
                                }
                            }
                        }
                    }
                    gradle.push_str("}\n\n");
                // Tasks
                if let Some(tasks) = map.get("tasks") {
                    if let CursedObject::Map(task_map) = tasks {
                        for (task_name, task_config) in task_map {
                            if let CursedObject::Map(config) = task_config {
                                if let Some(task_type) = config.get("type") {
                                    let type_str = self.render_text(task_type)?;
                                    gradle.push_str(&format!("tasks.register<{}>(\"{}\") {{\n", type_str, task_name));
                                } else {
                                    gradle.push_str(&format!("tasks.register(\"{}\") {{\n", task_name));
                                for (key, value) in config {
                                    if key != "type" {
                                        let value_str = self.render_text(value)?;
                                        match key.as_str() {
                                        }
                                    }
                                }
                                gradle.push_str("}\n\n");
                            }
                        }
                    }
                }

                // Publishing configuration
                if let Some(publishing) = map.get("publishing") {
                    gradle.push_str("publishing {\n");
                    if let CursedObject::Map(pub_map) = publishing {
                        if let Some(publications) = pub_map.get("publications") {
                            gradle.push_str("    publications {\n");
                            if let CursedObject::Map(pub_map_inner) = publications {
                                for (pub_name, pub_config) in pub_map_inner {
                                    gradle.push_str(&format!("        create<MavenPublication>(\"{}\") {{\n", pub_name));
                                    if let CursedObject::Map(config) = pub_config {
                                        for (key, value) in config {
                                            let value_str = self.render_text(value)?;
                                            gradle.push_str(&format!("            {} = \"{}\"\n", key, value_str));
                                        }
                                    }
                                    gradle.push_str("        }\n");
                                }
                            }
                            gradle.push_str("    }\n");
                        }
                    }
                    gradle.push_str("}\n\n");
                Ok(gradle)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render Maven pom.xml file
    fn render_maven(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut pom = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                pom.push_str("<project xmlns=\"http://maven.apache.org/POM/4.0.0\"\n");
                pom.push_str("         xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n");
                pom.push_str("         xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0\n");
                pom.push_str("         http://maven.apache.org/xsd/maven-4.0.0.xsd\">\n");
                pom.push_str("    <modelVersion>4.0.0</modelVersion>\n\n");

                // Project coordinates
                if let Some(group_id) = map.get("groupId") {
                    let group_str = self.render_text(group_id)?;
                    pom.push_str(&format!("    <groupId>{}</groupId>\n", self.escape_xml(&group_str)));
                if let Some(artifact_id) = map.get("artifactId") {
                    let artifact_str = self.render_text(artifact_id)?;
                    pom.push_str(&format!("    <artifactId>{}</artifactId>\n", self.escape_xml(&artifact_str)));
                if let Some(version) = map.get("version") {
                    let version_str = self.render_text(version)?;
                    pom.push_str(&format!("    <version>{}</version>\n", self.escape_xml(&version_str)));
                if let Some(packaging) = map.get("packaging") {
                    let packaging_str = self.render_text(packaging)?;
                    pom.push_str(&format!("    <packaging>{}</packaging>\n", self.escape_xml(&packaging_str)));
                if let Some(name) = map.get("name") {
                    let name_str = self.render_text(name)?;
                    pom.push_str(&format!("    <name>{}</name>\n", self.escape_xml(&name_str)));
                if let Some(description) = map.get("description") {
                    let desc_str = self.render_text(description)?;
                    pom.push_str(&format!("    <description>{}</description>\n", self.escape_xml(&desc_str)));
                pom.push('\n');

                // Parent
                if let Some(parent) = map.get("parent") {
                    pom.push_str("    <parent>\n");
                    if let CursedObject::Map(parent_map) = parent {
                        if let Some(group_id) = parent_map.get("groupId") {
                            let group_str = self.render_text(group_id)?;
                            pom.push_str(&format!("        <groupId>{}</groupId>\n", self.escape_xml(&group_str)));
                        }
                        if let Some(artifact_id) = parent_map.get("artifactId") {
                            let artifact_str = self.render_text(artifact_id)?;
                            pom.push_str(&format!("        <artifactId>{}</artifactId>\n", self.escape_xml(&artifact_str)));
                        }
                        if let Some(version) = parent_map.get("version") {
                            let version_str = self.render_text(version)?;
                            pom.push_str(&format!("        <version>{}</version>\n", self.escape_xml(&version_str)));
                        }
                    }
                    pom.push_str("    </parent>\n\n");
                // Properties
                if let Some(properties) = map.get("properties") {
                    pom.push_str("    <properties>\n");
                    if let CursedObject::Map(prop_map) = properties {
                        for (key, value) in prop_map {
                            let value_str = self.render_text(value)?;
                            pom.push_str(&format!("        <{}>{}</{}>\n", 
                                self.sanitize_xml_tag(key)));
                        }
                    }
                    pom.push_str("    </properties>\n\n");
                // Dependencies
                if let Some(dependencies) = map.get("dependencies") {
                    pom.push_str("    <dependencies>\n");
                    if let CursedObject::Array(dep_array) = dependencies {
                        for dep in dep_array {
                            if let CursedObject::Map(dep_map) = dep {
                                pom.push_str("        <dependency>\n");
                                
                                if let Some(group_id) = dep_map.get("groupId") {
                                    let group_str = self.render_text(group_id)?;
                                    pom.push_str(&format!("            <groupId>{}</groupId>\n", self.escape_xml(&group_str)));
                                if let Some(artifact_id) = dep_map.get("artifactId") {
                                    let artifact_str = self.render_text(artifact_id)?;
                                    pom.push_str(&format!("            <artifactId>{}</artifactId>\n", self.escape_xml(&artifact_str)));
                                if let Some(version) = dep_map.get("version") {
                                    let version_str = self.render_text(version)?;
                                    pom.push_str(&format!("            <version>{}</version>\n", self.escape_xml(&version_str)));
                                if let Some(scope) = dep_map.get("scope") {
                                    let scope_str = self.render_text(scope)?;
                                    pom.push_str(&format!("            <scope>{}</scope>\n", self.escape_xml(&scope_str)));
                                if let Some(optional) = dep_map.get("optional") {
                                    if let CursedObject::Boolean(true) = optional {
                                        pom.push_str("            <optional>true</optional>\n");
                                    }
                                }
                                
                                pom.push_str("        </dependency>\n");
                            }
                        }
                    }
                    pom.push_str("    </dependencies>\n\n");
                // Build section
                if let Some(build) = map.get("build") {
                    pom.push_str("    <build>\n");
                    if let CursedObject::Map(build_map) = build {
                        // Source directory
                        if let Some(source_dir) = build_map.get("sourceDirectory") {
                            let source_str = self.render_text(source_dir)?;
                            pom.push_str(&format!("        <sourceDirectory>{}</sourceDirectory>\n", self.escape_xml(&source_str)));
                        // Plugins
                        if let Some(plugins) = build_map.get("plugins") {
                            pom.push_str("        <plugins>\n");
                            if let CursedObject::Array(plugin_array) = plugins {
                                for plugin in plugin_array {
                                    if let CursedObject::Map(plugin_map) = plugin {
                                        pom.push_str("            <plugin>\n");
                                        
                                        if let Some(group_id) = plugin_map.get("groupId") {
                                            let group_str = self.render_text(group_id)?;
                                            pom.push_str(&format!("                <groupId>{}</groupId>\n", self.escape_xml(&group_str)));
                                        if let Some(artifact_id) = plugin_map.get("artifactId") {
                                            let artifact_str = self.render_text(artifact_id)?;
                                            pom.push_str(&format!("                <artifactId>{}</artifactId>\n", self.escape_xml(&artifact_str)));
                                        if let Some(version) = plugin_map.get("version") {
                                            let version_str = self.render_text(version)?;
                                            pom.push_str(&format!("                <version>{}</version>\n", self.escape_xml(&version_str)));
                                        // Configuration
                                        if let Some(configuration) = plugin_map.get("configuration") {
                                            pom.push_str("                <configuration>\n");
                                            pom.push_str(&self.render_maven_configuration(configuration, 5)?);
                                            pom.push_str("                </configuration>\n");
                                        pom.push_str("            </plugin>\n");
                                    }
                                }
                            }
                            pom.push_str("        </plugins>\n");
                        }
                    }
                    pom.push_str("    </build>\n\n");
                // Repositories
                if let Some(repositories) = map.get("repositories") {
                    pom.push_str("    <repositories>\n");
                    if let CursedObject::Array(repo_array) = repositories {
                        for repo in repo_array {
                            if let CursedObject::Map(repo_map) = repo {
                                pom.push_str("        <repository>\n");
                                
                                if let Some(id) = repo_map.get("id") {
                                    let id_str = self.render_text(id)?;
                                    pom.push_str(&format!("            <id>{}</id>\n", self.escape_xml(&id_str)));
                                if let Some(url) = repo_map.get("url") {
                                    let url_str = self.render_text(url)?;
                                    pom.push_str(&format!("            <url>{}</url>\n", self.escape_xml(&url_str)));
                                pom.push_str("        </repository>\n");
                            }
                        }
                    }
                    pom.push_str("    </repositories>\n\n");
                // Profiles
                if let Some(profiles) = map.get("profiles") {
                    pom.push_str("    <profiles>\n");
                    if let CursedObject::Array(profile_array) = profiles {
                        for profile in profile_array {
                            if let CursedObject::Map(profile_map) = profile {
                                pom.push_str("        <profile>\n");
                                
                                if let Some(id) = profile_map.get("id") {
                                    let id_str = self.render_text(id)?;
                                    pom.push_str(&format!("            <id>{}</id>\n", self.escape_xml(&id_str)));
                                // Add other profile elements as needed
                                pom.push_str("        </profile>\n");
                            }
                        }
                    }
                    pom.push_str("    </profiles>\n\n");
                pom.push_str("</project>\n");
                Ok(pom)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Helper method to render Maven plugin configuration
    fn render_maven_configuration(&self, config: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "    ".repeat(indent_level);
        let mut result = String::new();

        match config {
            CursedObject::Map(map) => {
                for (key, value) in map {
                    let safe_key = self.sanitize_xml_tag(key);
                    match value {
                        CursedObject::Map(_) => {
                            result.push_str(&format!("{}<{}>\n", indent, safe_key));
                            result.push_str(&self.render_maven_configuration(value, indent_level + 1)?);
                            result.push_str(&format!("{}</{}>\n", indent, safe_key));
                        }
                        CursedObject::Array(arr) => {
                            result.push_str(&format!("{}<{}>\n", indent, safe_key));
                            for item in arr {
                                result.push_str(&self.render_maven_configuration(item, indent_level + 1)?);
                            }
                            result.push_str(&format!("{}</{}>\n", indent, safe_key));
                        }
                        _ => {
                            let value_str = self.render_text(value)?;
                            result.push_str(&format!("{}<{}>{}</{}>\n", 
                                indent, safe_key, self.escape_xml(&value_str), safe_key));
                        }
                    }
                }
            }
            _ => {
                let value_str = self.render_text(config)?;
                result.push_str(&format!("{}{}\n", indent, self.escape_xml(&value_str)));
            }
        }

        Ok(result)
    }
    fn render_package_json(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut package_json = serde_json::Map::new();
                
                // Define field order for proper package.json structure
                let field_order = [
                    "sideEffects"
                ];

                // Process fields in the defined order
                for field in field_order.iter() {
                    if let Some(value) = map.get(*field) {
                        match field {
                            &"scripts" | &"dependencies" | &"devDependencies" | &"peerDependencies" | 
                            &"bundleDependencies" | &"optionalDependencies" | &"engines" | &"config" |
                            &"publishConfig" | &"overrides" | &"peerDependenciesMeta" => {
                                package_json.insert(field.to_string(), self.render_package_object(value)?);
                            }
                            &"keywords" | &"files" | &"contributors" | &"maintainers" | &"funding" => {
                                package_json.insert(field.to_string(), self.render_package_array(value)?);
                            }
                            &"workspaces" => {
                                package_json.insert(field.to_string(), self.render_package_workspaces(value)?);
                            }
                            &"exports" | &"imports" => {
                                package_json.insert(field.to_string(), self.render_package_exports(value)?);
                            }
                            &"author" => {
                                package_json.insert(field.to_string(), self.render_package_person(value)?);
                            }
                            &"repository" | &"bugs" => {
                                package_json.insert(field.to_string(), self.render_package_repository(value)?);
                            }
                            &"bin" => {
                                package_json.insert(field.to_string(), self.render_package_bin(value)?);
                            }
                            _ => {
                                // Standard fields (strings, booleans, etc.)
                                package_json.insert(field.to_string(), self.cursed_to_json(value)?);
                            }
                        }
                    }
                }

                // Add any remaining fields not in the predefined order
                for (key, value) in map {
                    if !field_order.contains(&key.as_str()) {
                        package_json.insert(key.clone(), self.cursed_to_json(value)?);
                    }
                }

                // Validate required fields
                if !package_json.contains_key("name") {
                    warn!("package.json missing required 'name' field");
                }
                if !package_json.contains_key("version") {
                    warn!("package.json missing required 'version' field");
                // Format with proper indentation
                let json_value = JsonValue::Object(package_json);
                serde_json::to_string_pretty(&json_value)
                    .map_err(|e| CursedError::TemplateError {
                    })
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render package.json object fields (dependencies, scripts, etc.)
    fn render_package_object(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Map(map) => {
                let mut json_map = serde_json::Map::new();
                for (key, value) in map {
                    match value {
                        CursedObject::String(s) => {
                            json_map.insert(key.clone(), JsonValue::String(s.clone()));
                        }
                        CursedObject::Integer(n) => {
                            json_map.insert(key.clone(), JsonValue::String(n.to_string()));
                        }
                        CursedObject::Float(n) => {
                            json_map.insert(key.clone(), JsonValue::String(n.to_string()));
                        }
                        _ => {
                            json_map.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                    }
                }
                Ok(JsonValue::Object(json_map))
            }
            _ => self.cursed_to_json(obj)
        }
    }

    /// Render package.json array fields (keywords, files, etc.)
    fn render_package_array(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Array(arr) => {
                let json_arr: crate::error::Result<()> = arr.iter()
                    .map(|item| self.cursed_to_json(item))
                    .collect();
                Ok(JsonValue::Array(json_arr?))
            }
            CursedObject::String(s) => {
                // Single string converted to array
                Ok(JsonValue::Array(vec![JsonValue::String(s.clone())]))
            }
            _ => self.cursed_to_json(obj)
        }
    }

    /// Render package.json workspaces field
    fn render_package_workspaces(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Map(map) => {
                // Workspaces object with packages and nohoist
                let mut json_map = serde_json::Map::new();
                
                if let Some(packages) = map.get("packages") {
                    json_map.insert("packages".to_string(), self.render_package_array(packages)?);
                if let Some(nohoist) = map.get("nohoist") {
                    json_map.insert("nohoist".to_string(), self.render_package_array(nohoist)?);
                for (key, value) in map {
                    if key != "packages" && key != "nohoist" {
                        json_map.insert(key.clone(), self.cursed_to_json(value)?);
                    }
                }
                
                Ok(JsonValue::Object(json_map))
            }
            _ => self.cursed_to_json(obj)
        }
    }

    /// Render package.json exports/imports field
    fn render_package_exports(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Map(map) => {
                let mut json_map = serde_json::Map::new();
                for (key, value) in map {
                    match value {
                        CursedObject::Map(_) => {
                            // Nested conditional exports
                            json_map.insert(key.clone(), self.render_package_exports(value)?);
                        }
                        _ => {
                            json_map.insert(key.clone(), self.cursed_to_json(value)?);
                        }
                    }
                }
                Ok(JsonValue::Object(json_map))
            }
            _ => self.cursed_to_json(obj)
        }
    }

    /// Render package.json person field (author, contributors, etc.)
    fn render_package_person(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Map(map) => {
                let mut json_map = serde_json::Map::new();
                
                // Standard person fields
                for field in ["name", "email", "url"] {
                    if let Some(value) = map.get(field) {
                        if let CursedObject::String(s) = value {
                            json_map.insert(field.to_string(), JsonValue::String(s.clone()));
                        }
                    }
                Ok(JsonValue::Object(json_map))
            }
            _ => self.cursed_to_json(obj)
        }
    }

    /// Render package.json repository/bugs field
    fn render_package_repository(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Map(map) => {
                let mut json_map = serde_json::Map::new();
                
                // Standard repository fields
                for field in ["type", "url", "directory"] {
                    if let Some(value) = map.get(field) {
                        json_map.insert(field.to_string(), self.cursed_to_json(value)?);
                    }
                }
                
                Ok(JsonValue::Object(json_map))
            }
            _ => self.cursed_to_json(obj)
        }
    }

    /// Render package.json bin field
    fn render_package_bin(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            _ => self.cursed_to_json(obj)
        }
    }
    /// Render as GitHub Actions workflow
    fn render_github_actions(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut workflow = String::new();
                
                // Workflow name
                if let Some(name) = map.get("name") {
                    let name_str = self.render_text(name)?;
                    workflow.push_str(&format!("name: {}\n\n", name_str));
                // Workflow triggers (on)
                if let Some(on_triggers) = map.get("on") {
                    workflow.push_str("on:\n");
                    workflow.push_str(&self.render_github_actions_triggers(on_triggers, 1)?);
                    workflow.push('\n');
                // Global environment variables
                if let Some(env) = map.get("env") {
                    workflow.push_str("env:\n");
                    workflow.push_str(&self.render_github_actions_env(env, 1)?);
                    workflow.push('\n');
                // Global defaults
                if let Some(defaults) = map.get("defaults") {
                    workflow.push_str("defaults:\n");
                    workflow.push_str(&self.render_kubernetes_object(defaults, 1)?);
                    workflow.push('\n');
                // Concurrency settings
                if let Some(concurrency) = map.get("concurrency") {
                    workflow.push_str("concurrency:\n");
                    workflow.push_str(&self.render_kubernetes_object(concurrency, 1)?);
                    workflow.push('\n');
                // Permissions
                if let Some(permissions) = map.get("permissions") {
                    workflow.push_str("permissions:\n");
                    workflow.push_str(&self.render_kubernetes_object(permissions, 1)?);
                    workflow.push('\n');
                // Jobs
                if let Some(jobs) = map.get("jobs") {
                    workflow.push_str("jobs:\n");
                    workflow.push_str(&self.render_github_actions_jobs(jobs, 1)?);
                Ok(workflow)
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Render GitHub Actions workflow triggers
    fn render_github_actions_triggers(&self, triggers: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match triggers {
            CursedObject::Map(trigger_map) => {
                for (trigger_name, trigger_config) in trigger_map {
                    match trigger_config {
                        CursedObject::Map(config) => {
                            result.push_str(&format!("{}{}:\n", indent, trigger_name));
                            
                            // Handle common trigger configurations
                            for (key, value) in config {
                                match key.as_str() {
                                    "branches" | "tags" | "paths" | "paths-ignore" => {
                                        result.push_str(&format!("{}  {}:\n", indent, key));
                                        if let CursedObject::Array(arr) = value {
                                            for item in arr {
                                                let item_str = self.render_text(item)?;
                                                result.push_str(&format!("{}    - {}\n", indent, item_str));
                                            }
                                        } else {
                                            let value_str = self.render_text(value)?;
                                            result.push_str(&format!("{}    - {}\n", indent, value_str));
                                        }
                                    }
                                    "types" => {
                                        result.push_str(&format!("{}  types:\n", indent));
                                        if let CursedObject::Array(arr) = value {
                                            for item in arr {
                                                let item_str = self.render_text(item)?;
                                                result.push_str(&format!("{}    - {}\n", indent, item_str));
                                            }
                                        }
                                    }
                                    _ => {
                                        let value_str = self.render_text(value)?;
                                        result.push_str(&format!("{}  {}: {}\n", indent, key, value_str));
                                    }
                                }
                            }
                        }
                        CursedObject::Array(arr) => {
                            result.push_str(&format!("{}{}:\n", indent, trigger_name));
                            for item in arr {
                                let item_str = self.render_text(item)?;
                                result.push_str(&format!("{}  - {}\n", indent, item_str));
                            }
                        }
                        _ => {
                            let value_str = self.render_text(trigger_config)?;
                            result.push_str(&format!("{}{}: {}\n", indent, trigger_name, value_str));
                        }
                    }
                }
            }
            CursedObject::Array(trigger_array) => {
                // Simple array of trigger names
                for trigger in trigger_array {
                    let trigger_str = self.render_text(trigger)?;
                    result.push_str(&format!("{}- {}\n", indent, trigger_str));
                }
            }
            _ => {
                let trigger_str = self.render_text(triggers)?;
                result.push_str(&format!("{}{}\n", indent, trigger_str));
            }
        }

        Ok(result)
    /// Render GitHub Actions environment variables
    fn render_github_actions_env(&self, env: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match env {
            CursedObject::Map(env_map) => {
                for (key, value) in env_map {
                    let value_str = self.render_text(value)?;
                    // Check if value needs quoting (contains spaces, special chars, or expressions)
                    if value_str.contains(' ') || value_str.contains('$') || value_str.contains('{') {
                        result.push_str(&format!("{}{}: \"{}\"\n", indent, key, value_str));
                    } else {
                        result.push_str(&format!("{}{}: {}\n", indent, key, value_str));
                    }
                }
            }
            _ => return Err(CursedError::TemplateError {
        Ok(result)
    /// Render GitHub Actions jobs
    fn render_github_actions_jobs(&self, jobs: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match jobs {
            CursedObject::Map(jobs_map) => {
                for (job_name, job_config) in jobs_map {
                    result.push_str(&format!("{}{}:\n", indent, job_name));
                    result.push_str(&self.render_github_actions_job(job_config, indent_level + 1)?);
                    result.push('\n');
                }
            }
            _ => return Err(CursedError::TemplateError {
        Ok(result)
    /// Render a single GitHub Actions job
    fn render_github_actions_job(&self, job: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match job {
            CursedObject::Map(job_map) => {
                // Job configuration order: name, runs-on, needs, if, strategy, env, outputs, steps
                
                // Job name
                if let Some(name) = job_map.get("name") {
                    let name_str = self.render_text(name)?;
                    result.push_str(&format!("{}name: {}\n", indent, name_str));
                // runs-on (required)
                if let Some(runs_on) = job_map.get("runs-on") {
                    match runs_on {
                        CursedObject::Array(arr) => {
                            result.push_str(&format!("{}runs-on:\n", indent));
                            for item in arr {
                                let item_str = self.render_text(item)?;
                                result.push_str(&format!("{}  - {}\n", indent, item_str));
                            }
                        }
                        _ => {
                            let runs_on_str = self.render_text(runs_on)?;
                            result.push_str(&format!("{}runs-on: {}\n", indent, runs_on_str));
                        }
                    }
                // Job dependencies
                if let Some(needs) = job_map.get("needs") {
                    match needs {
                        CursedObject::Array(arr) => {
                            result.push_str(&format!("{}needs:\n", indent));
                            for item in arr {
                                let item_str = self.render_text(item)?;
                                result.push_str(&format!("{}  - {}\n", indent, item_str));
                            }
                        }
                        _ => {
                            let needs_str = self.render_text(needs)?;
                            result.push_str(&format!("{}needs: {}\n", indent, needs_str));
                        }
                    }
                // Conditional execution
                if let Some(condition) = job_map.get("if") {
                    let condition_str = self.render_text(condition)?;
                    result.push_str(&format!("{}if: {}\n", indent, condition_str));
                // Strategy (matrix builds)
                if let Some(strategy) = job_map.get("strategy") {
                    result.push_str(&format!("{}strategy:\n", indent));
                    result.push_str(&self.render_github_actions_strategy(strategy, indent_level + 1)?);
                // Job environment variables
                if let Some(env) = job_map.get("env") {
                    result.push_str(&format!("{}env:\n", indent));
                    result.push_str(&self.render_github_actions_env(env, indent_level + 1)?);
                // Job outputs
                if let Some(outputs) = job_map.get("outputs") {
                    result.push_str(&format!("{}outputs:\n", indent));
                    result.push_str(&self.render_kubernetes_object(outputs, indent_level + 1)?);
                // Job defaults
                if let Some(defaults) = job_map.get("defaults") {
                    result.push_str(&format!("{}defaults:\n", indent));
                    result.push_str(&self.render_kubernetes_object(defaults, indent_level + 1)?);
                // Timeout
                if let Some(timeout_minutes) = job_map.get("timeout-minutes") {
                    let timeout_str = self.render_text(timeout_minutes)?;
                    result.push_str(&format!("{}timeout-minutes: {}\n", indent, timeout_str));
                // Continue on error
                if let Some(continue_on_error) = job_map.get("continue-on-error") {
                    let continue_str = self.render_text(continue_on_error)?;
                    result.push_str(&format!("{}continue-on-error: {}\n", indent, continue_str));
                // Container
                if let Some(container) = job_map.get("container") {
                    result.push_str(&format!("{}container:\n", indent));
                    result.push_str(&self.render_kubernetes_object(container, indent_level + 1)?);
                // Services
                if let Some(services) = job_map.get("services") {
                    result.push_str(&format!("{}services:\n", indent));
                    result.push_str(&self.render_kubernetes_object(services, indent_level + 1)?);
                // Steps (most important part)
                if let Some(steps) = job_map.get("steps") {
                    result.push_str(&format!("{}steps:\n", indent));
                    result.push_str(&self.render_github_actions_steps(steps, indent_level + 1)?);
                }
            }
            _ => return Err(CursedError::TemplateError {
        Ok(result)
    /// Render GitHub Actions strategy (matrix builds)
    fn render_github_actions_strategy(&self, strategy: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match strategy {
            CursedObject::Map(strategy_map) => {
                // Matrix strategy
                if let Some(matrix) = strategy_map.get("matrix") {
                    result.push_str(&format!("{}matrix:\n", indent));
                    match matrix {
                        CursedObject::Map(matrix_map) => {
                            for (key, value) in matrix_map {
                                match value {
                                    CursedObject::Array(arr) => {
                                        result.push_str(&format!("{}  {}:\n", indent, key));
                                        for item in arr {
                                            let item_str = self.render_text(item)?;
                                            result.push_str(&format!("{}    - {}\n", indent, item_str));
                                        }
                                    }
                                    _ => {
                                        let value_str = self.render_text(value)?;
                                        result.push_str(&format!("{}  {}: {}\n", indent, key, value_str));
                                    }
                                }
                            }
                        }
                        _ => {
                            result.push_str(&self.render_kubernetes_object(matrix, indent_level + 1)?);
                        }
                    }
                // Fail-fast
                if let Some(fail_fast) = strategy_map.get("fail-fast") {
                    let fail_fast_str = self.render_text(fail_fast)?;
                    result.push_str(&format!("{}fail-fast: {}\n", indent, fail_fast_str));
                // Max-parallel
                if let Some(max_parallel) = strategy_map.get("max-parallel") {
                    let max_parallel_str = self.render_text(max_parallel)?;
                    result.push_str(&format!("{}max-parallel: {}\n", indent, max_parallel_str));
                }
            }
            _ => {
                result.push_str(&self.render_kubernetes_object(strategy, indent_level)?);
            }
        }

        Ok(result)
    /// Render GitHub Actions steps
    fn render_github_actions_steps(&self, steps: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match steps {
            CursedObject::Array(steps_array) => {
                for step in steps_array {
                    result.push_str(&format!("{}- ", indent));
                    result.push_str(&self.render_github_actions_step(step, indent_level)?);
                }
            }
            _ => return Err(CursedError::TemplateError {
        Ok(result)
    /// Render a single GitHub Actions step
    fn render_github_actions_step(&self, step: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match step {
            CursedObject::Map(step_map) => {
                let mut has_content = false;

                // Step name
                if let Some(name) = step_map.get("name") {
                    let name_str = self.render_text(name)?;
                    if has_content {
                        result.push_str(&format!("{}  name: {}\n", indent, name_str));
                    } else {
                        result.push_str(&format!("name: {}\n", name_str));
                        has_content = true;
                    }
                }

                // Step ID
                if let Some(id) = step_map.get("id") {
                    let id_str = self.render_text(id)?;
                    if has_content {
                        result.push_str(&format!("{}  id: {}\n", indent, id_str));
                    } else {
                        result.push_str(&format!("id: {}\n", id_str));
                        has_content = true;
                    }
                }

                // Conditional execution
                if let Some(condition) = step_map.get("if") {
                    let condition_str = self.render_text(condition)?;
                    if has_content {
                        result.push_str(&format!("{}  if: {}\n", indent, condition_str));
                    } else {
                        result.push_str(&format!("if: {}\n", condition_str));
                        has_content = true;
                    }
                }

                // Uses (action)
                if let Some(uses) = step_map.get("uses") {
                    let uses_str = self.render_text(uses)?;
                    if has_content {
                        result.push_str(&format!("{}  uses: {}\n", indent, uses_str));
                    } else {
                        result.push_str(&format!("uses: {}\n", uses_str));
                        has_content = true;
                    }
                }

                // With (action inputs)
                if let Some(with) = step_map.get("with") {
                    if has_content {
                        result.push_str(&format!("{}  with:\n", indent));
                        result.push_str(&self.render_github_actions_with(with, indent_level + 2)?);
                    } else {
                        result.push_str("with:\n");
                        result.push_str(&self.render_github_actions_with(with, indent_level + 1)?);
                        has_content = true;
                    }
                }

                // Run (shell command)
                if let Some(run) = step_map.get("run") {
                    let run_str = self.render_text(run)?;
                    // For multi-line commands, use block scalar
                    if run_str.contains('\n') {
                        if has_content {
                            result.push_str(&format!("{}  run: |\n", indent));
                            for line in run_str.split("\n") {
                                result.push_str(&format!("{}    {}\n", indent, line));
                            }
                        } else {
                            result.push_str("run: |\n");
                            for line in run_str.split("\n") {
                                result.push_str(&format!("{}  {}\n", indent, line));
                            }
                            has_content = true;
                        }
                    } else {
                        if has_content {
                            result.push_str(&format!("{}  run: {}\n", indent, run_str));
                        } else {
                            result.push_str(&format!("run: {}\n", run_str));
                            has_content = true;
                        }
                    }
                // Shell
                if let Some(shell) = step_map.get("shell") {
                    let shell_str = self.render_text(shell)?;
                    if has_content {
                        result.push_str(&format!("{}  shell: {}\n", indent, shell_str));
                    } else {
                        result.push_str(&format!("shell: {}\n", shell_str));
                        has_content = true;
                    }
                }

                // Working directory
                if let Some(working_directory) = step_map.get("working-directory") {
                    let wd_str = self.render_text(working_directory)?;
                    if has_content {
                        result.push_str(&format!("{}  working-directory: {}\n", indent, wd_str));
                    } else {
                        result.push_str(&format!("working-directory: {}\n", wd_str));
                        has_content = true;
                    }
                }

                // Environment variables
                if let Some(env) = step_map.get("env") {
                    if has_content {
                        result.push_str(&format!("{}  env:\n", indent));
                        result.push_str(&self.render_github_actions_env(env, indent_level + 2)?);
                    } else {
                        result.push_str("env:\n");
                        result.push_str(&self.render_github_actions_env(env, indent_level + 1)?);
                        has_content = true;
                    }
                }

                // Continue on error
                if let Some(continue_on_error) = step_map.get("continue-on-error") {
                    let continue_str = self.render_text(continue_on_error)?;
                    if has_content {
                        result.push_str(&format!("{}  continue-on-error: {}\n", indent, continue_str));
                    } else {
                        result.push_str(&format!("continue-on-error: {}\n", continue_str));
                        has_content = true;
                    }
                }

                // Timeout
                if let Some(timeout_minutes) = step_map.get("timeout-minutes") {
                    let timeout_str = self.render_text(timeout_minutes)?;
                    if has_content {
                        result.push_str(&format!("{}  timeout-minutes: {}\n", indent, timeout_str));
                    } else {
                        result.push_str(&format!("timeout-minutes: {}\n", timeout_str));
                        has_content = true;
                    }
                }
            }
            _ => return Err(CursedError::TemplateError {
        Ok(result)
    /// Render GitHub Actions 'with' parameters
    fn render_github_actions_with(&self, with: &CursedObject, indent_level: usize) -> crate::error::Result<()> {
        let indent = "  ".repeat(indent_level);
        let mut result = String::new();

        match with {
            CursedObject::Map(with_map) => {
                for (key, value) in with_map {
                    let value_str = self.render_text(value)?;
                    // Check if value needs quoting (contains expressions or special characters)
                    if value_str.contains('$') || value_str.contains('{') || value_str.contains('\n') || value_str.contains(':') {
                        if value_str.contains('\n') {
                            result.push_str(&format!("{}{}: |\n", indent, key));
                            for line in value_str.split("\n") {
                                result.push_str(&format!("{}  {}\n", indent, line));
                            }
                        } else {
                            result.push_str(&format!("{}{}: \"{}\"\n", indent, key, value_str));
                        }
                    } else {
                        result.push_str(&format!("{}{}: {}\n", indent, key, value_str));
                    }
                }
            }
            _ => return Err(CursedError::TemplateError {
        Ok(result)
    }
    /// Render CI/CD configuration
    fn render_ci_cd(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                // Detect CI/CD platform from data or default to generic
                let platform = map.get("platform")
                    .map(|p| self.render_text(p))
                    .unwrap_or_else(|| Ok("gitlab".to_string()))?;
                
                match platform.to_lowercase().as_str() {
                    _ => self.render_gitlab_ci(data), // Default to GitLab CI
                }
            }
            _ => self.render_yaml(data)
        }
    }
    
    /// Render GitLab CI configuration
    fn render_gitlab_ci(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut gitlab_ci = String::new();
                
                // Stages
                if let Some(stages) = map.get("stages") {
                    gitlab_ci.push_str("stages:\n");
                    if let CursedObject::Array(stage_array) = stages {
                        for stage in stage_array {
                            let stage_str = self.render_text(stage)?;
                            gitlab_ci.push_str(&format!("  - {}\n", stage_str));
                        }
                    }
                    gitlab_ci.push_str("\n");
                // Global variables
                if let Some(variables) = map.get("variables") {
                    gitlab_ci.push_str("variables:\n");
                    if let CursedObject::Map(var_map) = variables {
                        for (key, value) in var_map {
                            let value_str = self.render_text(value)?;
                            gitlab_ci.push_str(&format!("  {}: {}\n", key, value_str));
                        }
                    }
                    gitlab_ci.push_str("\n");
                // Before script
                if let Some(before_script) = map.get("before_script") {
                    gitlab_ci.push_str("before_script:\n");
                    if let CursedObject::Array(script_array) = before_script {
                        for script in script_array {
                            let script_str = self.render_text(script)?;
                            gitlab_ci.push_str(&format!("  - {}\n", script_str));
                        }
                    }
                    gitlab_ci.push_str("\n");
                // After script
                if let Some(after_script) = map.get("after_script") {
                    gitlab_ci.push_str("after_script:\n");
                    if let CursedObject::Array(script_array) = after_script {
                        for script in script_array {
                            let script_str = self.render_text(script)?;
                            gitlab_ci.push_str(&format!("  - {}\n", script_str));
                        }
                    }
                    gitlab_ci.push_str("\n");
                // Jobs
                if let Some(jobs) = map.get("jobs") {
                    if let CursedObject::Map(job_map) = jobs {
                        for (job_name, job_config) in job_map {
                            gitlab_ci.push_str(&format!("{}:\n", job_name));
                            if let CursedObject::Map(config) = job_config {
                                for (key, value) in config {
                                    match key.as_str() {
                                        "script" => {
                                            gitlab_ci.push_str("  script:\n");
                                            if let CursedObject::Array(script_array) = value {
                                                for script in script_array {
                                                    let script_str = self.render_text(script)?;
                                                    gitlab_ci.push_str(&format!("    - {}\n", script_str));
                                                }
                                            }
                                        }
                                        "only" | "except" | "rules" => {
                                            gitlab_ci.push_str(&format!("  {}:\n", key));
                                            if let CursedObject::Array(rule_array) = value {
                                                for rule in rule_array {
                                                    let rule_str = self.render_text(rule)?;
                                                    gitlab_ci.push_str(&format!("    - {}\n", rule_str));
                                                }
                                            }
                                        }
                                        "artifacts" => {
                                            gitlab_ci.push_str("  artifacts:\n");
                                            if let CursedObject::Map(art_map) = value {
                                                for (art_key, art_value) in art_map {
                                                    match art_key.as_str() {
                                                        "paths" => {
                                                            gitlab_ci.push_str("    paths:\n");
                                                            if let CursedObject::Array(path_array) = art_value {
                                                                for path in path_array {
                                                                    let path_str = self.render_text(path)?;
                                                                    gitlab_ci.push_str(&format!("      - {}\n", path_str));
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            let art_value_str = self.render_text(art_value)?;
                                                            gitlab_ci.push_str(&format!("    {}: {}\n", art_key, art_value_str));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            let value_str = self.render_text(value)?;
                                            gitlab_ci.push_str(&format!("  {}: {}\n", key, value_str));
                                        }
                                    }
                                }
                            }
                            gitlab_ci.push_str("\n");
                        }
                    }
                Ok(gitlab_ci)
            }
            _ => self.render_yaml(data)
        }
    }
    
    /// Render Jenkins pipeline
    fn render_jenkins(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut jenkins = String::from("pipeline {\n");
                
                // Agent
                if let Some(agent) = map.get("agent") {
                    let agent_str = self.render_text(agent)?;
                    jenkins.push_str(&format!("    agent {}\n\n", agent_str));
                } else {
                    jenkins.push_str("    agent any\n\n");
                // Environment
                if let Some(environment) = map.get("environment") {
                    jenkins.push_str("    environment {\n");
                    if let CursedObject::Map(env_map) = environment {
                        for (key, value) in env_map {
                            let value_str = self.render_text(value)?;
                            jenkins.push_str(&format!("        {} = '{}'\n", key, value_str));
                        }
                    }
                    jenkins.push_str("    }\n\n");
                // Stages
                if let Some(stages) = map.get("stages") {
                    jenkins.push_str("    stages {\n");
                    if let CursedObject::Map(stage_map) = stages {
                        for (stage_name, stage_config) in stage_map {
                            jenkins.push_str(&format!("        stage('{}') {{\n", stage_name));
                            jenkins.push_str("            steps {\n");
                            
                            if let CursedObject::Map(config) = stage_config {
                                if let Some(steps) = config.get("steps") {
                                    if let CursedObject::Array(step_array) = steps {
                                        for step in step_array {
                                            let step_str = self.render_text(step)?;
                                            jenkins.push_str(&format!("                {}\n", step_str));
                                        }
                                    }
                                }
                            }
                            
                            jenkins.push_str("            }\n");
                            jenkins.push_str("        }\n");
                        }
                    }
                    jenkins.push_str("    }\n\n");
                // Post actions
                if let Some(post) = map.get("post") {
                    jenkins.push_str("    post {\n");
                    if let CursedObject::Map(post_map) = post {
                        for (condition, actions) in post_map {
                            jenkins.push_str(&format!("        {} {{\n", condition));
                            if let CursedObject::Array(action_array) = actions {
                                for action in action_array {
                                    let action_str = self.render_text(action)?;
                                    jenkins.push_str(&format!("            {}\n", action_str));
                                }
                            }
                            jenkins.push_str("        }\n");
                        }
                    }
                    jenkins.push_str("    }\n");
                jenkins.push_str("}\n");
                Ok(jenkins)
            }
            _ => Err(CursedError::TemplateError {
            })
        }
    }
    
    /// Render Travis CI configuration
    fn render_travis_ci(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut travis = String::new();
                
                // Language
                if let Some(language) = map.get("language") {
                    let lang_str = self.render_text(language)?;
                    travis.push_str(&format!("language: {}\n", lang_str));
                // Language versions
                if let Some(versions) = map.get("versions") {
                    match map.get("language").map(|l| self.render_text(l)).unwrap_or_else(|| Ok("generic".to_string()))?.as_str() {
                    if let CursedObject::Array(version_array) = versions {
                        for version in version_array {
                            let version_str = self.render_text(version)?;
                            travis.push_str(&format!("  - {}\n", version_str));
                        }
                    }
                    travis.push_str("\n");
                // Environment variables
                if let Some(env) = map.get("env") {
                    travis.push_str("env:\n");
                    if let CursedObject::Array(env_array) = env {
                        for env_var in env_array {
                            let env_str = self.render_text(env_var)?;
                            travis.push_str(&format!("  - {}\n", env_str));
                        }
                    }
                    travis.push_str("\n");
                // Script phases
                let phases = vec![
                ];
                
                for (key, section) in phases {
                    if let Some(commands) = map.get(key) {
                        travis.push_str(&format!("{}:\n", section));
                        if let CursedObject::Array(cmd_array) = commands {
                            for cmd in cmd_array {
                                let cmd_str = self.render_text(cmd)?;
                                travis.push_str(&format!("  - {}\n", cmd_str));
                            }
                        } else {
                            let cmd_str = self.render_text(commands)?;
                            travis.push_str(&format!("  - {}\n", cmd_str));
                        }
                        travis.push_str("\n");
                    }
                }
                
                Ok(travis)
            }
            _ => self.render_yaml(data)
        }
    }
    
    /// Render CircleCI configuration
    fn render_circle_ci(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut circle = String::from("version: 2.1\n\n");
                
                // Orbs
                if let Some(orbs) = map.get("orbs") {
                    circle.push_str("orbs:\n");
                    if let CursedObject::Map(orb_map) = orbs {
                        for (orb_name, orb_version) in orb_map {
                            let version_str = self.render_text(orb_version)?;
                            circle.push_str(&format!("  {}: {}\n", orb_name, version_str));
                        }
                    }
                    circle.push_str("\n");
                // Jobs
                if let Some(jobs) = map.get("jobs") {
                    circle.push_str("jobs:\n");
                    if let CursedObject::Map(job_map) = jobs {
                        for (job_name, job_config) in job_map {
                            circle.push_str(&format!("  {}:\n", job_name));
                            circle.push_str(&self.render_kubernetes_object(job_config, 2)?);
                        }
                    }
                    circle.push_str("\n");
                // Workflows
                if let Some(workflows) = map.get("workflows") {
                    circle.push_str("workflows:\n");
                    circle.push_str(&self.render_kubernetes_object(workflows, 1)?);
                Ok(circle)
            }
            _ => self.render_yaml(data)
        }
    }
    
    /// Render Azure Pipelines configuration
    fn render_azure_pipelines(&self, data: &CursedObject) -> crate::error::Result<()> {
        match data {
            CursedObject::Map(map) => {
                let mut azure = String::new();
                
                // Trigger
                if let Some(trigger) = map.get("trigger") {
                    azure.push_str("trigger:\n");
                    if let CursedObject::Array(trigger_array) = trigger {
                        for branch in trigger_array {
                            let branch_str = self.render_text(branch)?;
                            azure.push_str(&format!("  - {}\n", branch_str));
                        }
                    }
                    azure.push_str("\n");
                // Pool
                if let Some(pool) = map.get("pool") {
                    azure.push_str("pool:\n");
                    if let CursedObject::Map(pool_map) = pool {
                        for (key, value) in pool_map {
                            let value_str = self.render_text(value)?;
                            azure.push_str(&format!("  {}: {}\n", key, value_str));
                        }
                    } else {
                        let pool_str = self.render_text(pool)?;
                        azure.push_str(&format!("  vmImage: {}\n", pool_str));
                    }
                    azure.push_str("\n");
                // Variables
                if let Some(variables) = map.get("variables") {
                    azure.push_str("variables:\n");
                    if let CursedObject::Map(var_map) = variables {
                        for (key, value) in var_map {
                            let value_str = self.render_text(value)?;
                            azure.push_str(&format!("  {}: {}\n", key, value_str));
                        }
                    }
                    azure.push_str("\n");
                // Steps
                if let Some(steps) = map.get("steps") {
                    azure.push_str("steps:\n");
                    if let CursedObject::Array(step_array) = steps {
                        for step in step_array {
                            azure.push_str("- ");
                            azure.push_str(&self.render_azure_step(step)?);
                        }
                    }
                Ok(azure)
            }
            _ => self.render_yaml(data)
        }
    }
    
    /// Render Azure Pipeline step
    fn render_azure_step(&self, step: &CursedObject) -> crate::error::Result<()> {
        match step {
            CursedObject::Map(step_map) => {
                let mut step_yaml = String::new();
                
                // Step type (task, script, etc.)
                if let Some(task) = step_map.get("task") {
                    let task_str = self.render_text(task)?;
                    step_yaml.push_str(&format!("task: {}\n", task_str));
                } else if let Some(script) = step_map.get("script") {
                    let script_str = self.render_text(script)?;
                    step_yaml.push_str(&format!("script: {}\n", script_str));
                // Other step properties
                for (key, value) in step_map {
                    if key != "task" && key != "script" {
                        match key.as_str() {
                            "inputs" => {
                                step_yaml.push_str("  inputs:\n");
                                step_yaml.push_str(&self.render_kubernetes_object(value, 2)?);
                            }
                            _ => {
                                let value_str = self.render_text(value)?;
                                step_yaml.push_str(&format!("  {}: {}\n", key, value_str));
                            }
                        }
                    }
                }
                
                Ok(step_yaml)
            }
            _ => {
                let step_str = self.render_text(step)?;
                Ok(format!("script: {}\n", step_str))
            }
        }
    /// Render API endpoints
    fn render_api_endpoints(&self, endpoints: &CursedObject) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        match endpoints {
            CursedObject::Array(endpoint_array) => {
                for endpoint in endpoint_array {
                    if let CursedObject::Map(endpoint_map) = endpoint {
                        let method = endpoint_map.get("method")
                            .map(|m| self.render_text(m))
                            .unwrap_or_else(|| Ok("GET".to_string()))?;
                        let path = endpoint_map.get("path")
                            .map(|p| self.render_text(p))
                            .unwrap_or_else(|| Ok("/".to_string()))?;
                        let description = endpoint_map.get("description")
                            .map(|d| self.render_text(d))
                            .unwrap_or_else(|| Ok("".to_string()))?;
                        
                        doc.push_str(&format!("### {} {}\n\n", method.to_uppercase(), path));
                        if !description.is_empty() {
                            doc.push_str(&format!("{}\n\n", description));
                        // Parameters
                        if let Some(parameters) = endpoint_map.get("parameters") {
                            doc.push_str("**Parameters:**\n\n");
                            doc.push_str(&self.render_parameters(parameters)?);
                        // Request body
                        if let Some(request_body) = endpoint_map.get("request_body") {
                            doc.push_str("**Request Body:**\n\n");
                            let body_str = self.render_text(request_body)?;
                            doc.push_str(&format!("```json\n{}\n```\n\n", body_str));
                        // Responses
                        if let Some(responses) = endpoint_map.get("responses") {
                            doc.push_str("**Responses:**\n\n");
                            if let CursedObject::Map(response_map) = responses {
                                for (status_code, response) in response_map {
                                    doc.push_str(&format!("- **{}**: ", status_code));
                                    if let CursedObject::Map(resp_details) = response {
                                        if let Some(desc) = resp_details.get("description") {
                                            let desc_str = self.render_text(desc)?;
                                            doc.push_str(&format!("{}\n", desc_str));
                                        }
                                        if let Some(example) = resp_details.get("example") {
                                            let example_str = self.render_text(example)?;
                                            doc.push_str(&format!("  ```json\n  {}\n  ```\n", example_str));
                                        }
                                    } else {
                                        let resp_str = self.render_text(response)?;
                                        doc.push_str(&format!("{}\n", resp_str));
                                    }
                                }
                            }
                            doc.push_str("\n");
                        // Example
                        if let Some(example) = endpoint_map.get("example") {
                            doc.push_str("**Example:**\n\n");
                            let example_str = self.render_text(example)?;
                            doc.push_str(&format!("```bash\n{}\n```\n\n", example_str));
                        doc.push_str("---\n\n");
                    }
                }
            }
            _ => {
                let endpoints_str = self.render_text(endpoints)?;
                doc.push_str(&format!("{}\n\n", endpoints_str));
            }
        }
        
        Ok(doc)
    /// Render API errors
    fn render_api_errors(&self, errors: &CursedObject) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        match errors {
            CursedObject::Array(error_array) => {
                doc.push_str("| Code | Message | Description |\n");
                doc.push_str("|------|---------|-------------|\n");
                
                for error in error_array {
                    if let CursedObject::Map(error_map) = error {
                        let code = error_map.get("code")
                            .map(|c| self.render_text(c))
                            .unwrap_or_else(|| Ok("500".to_string()))?;
                        let message = error_map.get("message")
                            .map(|m| self.render_text(m))
                            .unwrap_or_else(|| Ok("Internal Server CursedError".to_string()))?;
                        let description = error_map.get("description")
                            .map(|d| self.render_text(d))
                            .unwrap_or_else(|| Ok("".to_string()))?;
                        
                        doc.push_str(&format!("| {} | {} | {} |\n", code, message, description));
                    }
                }
                doc.push_str("\n");
            }
            CursedObject::Map(error_map) => {
                doc.push_str("| Code | Message | Description |\n");
                doc.push_str("|------|---------|-------------|\n");
                
                for (code, error_info) in error_map {
                    if let CursedObject::Map(info_map) = error_info {
                        let message = info_map.get("message")
                            .map(|m| self.render_text(m))
                            .unwrap_or_else(|| Ok("CursedError".to_string()))?;
                        let description = info_map.get("description")
                            .map(|d| self.render_text(d))
                            .unwrap_or_else(|| Ok("".to_string()))?;
                        
                        doc.push_str(&format!("| {} | {} | {} |\n", code, message, description));
                    } else {
                        let message = self.render_text(error_info)?;
                        doc.push_str(&format!("| {} | {} | |\n", code, message));
                    }
                }
                doc.push_str("\n");
            }
            _ => {
                let errors_str = self.render_text(errors)?;
                doc.push_str(&format!("{}\n\n", errors_str));
            }
        }
        
        Ok(doc)
    /// Render API examples
    fn render_api_examples(&self, examples: &CursedObject) -> crate::error::Result<()> {
        let mut doc = String::new();
        
        match examples {
            CursedObject::Array(example_array) => {
                for (i, example) in example_array.iter().enumerate() {
                    if let CursedObject::Map(example_map) = example {
                        let title = example_map.get("title")
                            .map(|t| self.render_text(t))
                            .unwrap_or_else(|| Ok(format!("Example {}", i + 1)))?;
                        
                        doc.push_str(&format!("### {}\n\n", title));
                        
                        if let Some(description) = example_map.get("description") {
                            let desc_str = self.render_text(description)?;
                            doc.push_str(&format!("{}\n\n", desc_str));
                        if let Some(request) = example_map.get("request") {
                            doc.push_str("**Request:**\n\n");
                            let request_str = self.render_text(request)?;
                            doc.push_str(&format!("```bash\n{}\n```\n\n", request_str));
                        if let Some(response) = example_map.get("response") {
                            doc.push_str("**Response:**\n\n");
                            let response_str = self.render_text(response)?;
                            doc.push_str(&format!("```json\n{}\n```\n\n", response_str));
                        }
                    } else {
                        let example_str = self.render_text(example)?;
                        doc.push_str(&format!("### Example {}\n\n", i + 1));
                        doc.push_str(&format!("```\n{}\n```\n\n", example_str));
                    }
                }
            }
            _ => {
                let examples_str = self.render_text(examples)?;
                doc.push_str(&format!("```\n{}\n```\n\n", examples_str));
            }
        }
        
        Ok(doc)
    /// Helper methods for escaping and conversion
    fn escape_html(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
    fn escape_xml(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
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
    fn cursed_to_json(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Float(n) => {
                if let Some(num) = serde_json::Number::from_f64(*n) {
                    Ok(JsonValue::Number(num))
                } else {
                    Ok(JsonValue::Null)
                }
            }
            CursedObject::Array(arr) => {
                let json_arr: crate::error::Result<()> = arr.iter()
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
    fn cursed_to_csv_value(&self, obj: &CursedObject) -> crate::error::Result<()> {
        let text = self.render_text(obj)?;
        Ok(self.escape_csv(&text))
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
            TemplateFormat::Document(_) => "text/markdown",
            TemplateFormat::Api(api_format) => match api_format {
                ApiFormat::OpenApi => "application/x-yaml",
                ApiFormat::GraphQL => "application/graphql",
                ApiFormat::Protobuf => "application/x-protobuf",
                ApiFormat::JsonSchema => "application/schema+json",
                ApiFormat::Wsdl => "application/wsdl+xml",
                ApiFormat::AsyncApi => "application/x-yaml",
            TemplateFormat::Build(_) => "text/plain",
        }
    }

    /// Validate rendered output for the format
    pub fn validate(&self, content: &str) -> crate::error::Result<()> {
        if !self.options.validate {
            return Ok(());
        match &self.format {
            TemplateFormat::Json => {
                serde_json::from_str::<JsonValue>(content)
                    .map_err(|e| CursedError::TemplateError {
                    })?;
            }
            TemplateFormat::Yaml => {
                serde_yaml::from_str::<JsonValue>(content)
                    .map_err(|e| CursedError::TemplateError {
                    })?;
            }
            TemplateFormat::Xml => {
                // Basic XML validation - just check for well-formed structure
                if !self.is_well_formed_xml(content) {
                    return Err(CursedError::TemplateError {
                    });
                }
            }
            _ => {} // No validation for other formats yet
        }
        Ok(())
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
            "rs" if path.file_name().unwrap_or_default() == "build.rs" => {
                Some(TemplateFormat::Build(BuildFormat::BuildRs))
            _ => {
                // Check by filename
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    match filename.to_lowercase().as_str() {
                        "docker-compose.yml" | "docker-compose.yaml" => {
                            Some(TemplateFormat::Config(ConfigFormat::DockerCompose))
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
        // Check for HTML
        if trimmed.contains("<!DOCTYPE html") || 
           trimmed.contains("<html") ||
           trimmed.contains("<head>") ||
           trimmed.contains("<body>") {
            return Some(TemplateFormat::Html);
        // Check for YAML
        if content.split("\n").any(|line| {
            let line = line.trim();
            line.contains(':') && !line.starts_with('#') && 
            !line.contains('=') && !line.contains('<')
        }) {
            return Some(TemplateFormat::Yaml);
        // Check for specific formats by content patterns
        if content.split("\n").any(|line| line.trim().starts_with("FROM ")) {
            return Some(TemplateFormat::Config(ConfigFormat::Dockerfile));
        if content.split("\n").any(|line| line.trim().starts_with("server {")) {
            return Some(TemplateFormat::Config(ConfigFormat::Nginx));
        if content.contains("openapi:") || content.contains("swagger:") {
            return Some(TemplateFormat::Api(ApiFormat::OpenApi));
        if content.contains("type Query") || content.contains("type Mutation") {
            return Some(TemplateFormat::Api(ApiFormat::GraphQL));
        None
    }
}

/// Format conversion utilities
pub struct FormatConverter;

impl FormatConverter {
    /// Convert between formats
    pub fn convert(
    ) -> crate::error::Result<()> {
        // For now, just re-render with the target format
        let renderer = TemplateFormatRenderer::new(to);
        renderer.render(data)
    /// Compose multiple templates
    pub fn compose(
    ) -> crate::error::Result<()> {
        let mut result = String::new();
        
        for (i, (format, data)) in templates.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            let renderer = TemplateFormatRenderer::new(format.clone());
            let rendered = renderer.render(data)?;
            result.push_str(&rendered);
        Ok(result)
    }
}

