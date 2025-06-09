/// Template Format Support - Various output formats for CURSED templates
use std::collections::HashMap;
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
}

/// Template format renderer
pub struct TemplateFormatRenderer {
    format: TemplateFormat,
}

impl TemplateFormatRenderer {
    /// Create a new format renderer
    pub fn new(format: TemplateFormat) -> Self {
        Self { format }
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
        }
    }

    /// Render as plain text
    fn render_text(&self, data: &CursedObject) -> Result<String, CursedError> {
        match data {
            CursedObject::String(s) => Ok(s.clone()),
            CursedObject::Integer(n) => Ok(n.to_string()),
            CursedObject::Float(n) => Ok(n.to_string()),
            CursedObject::Boolean(b) => Ok(b.to_string()),
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
}
